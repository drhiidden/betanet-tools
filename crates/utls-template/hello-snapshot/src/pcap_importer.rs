use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::File;
use pnet_packet::Packet;
use pnet_packet::ethernet::{EthernetPacket, EtherTypes};
use pnet_packet::ipv4::Ipv4Packet;
use pnet_packet::tcp::TcpPacket;
use pnet_packet::udp::UdpPacket;
use pnet_packet::ip::IpNextHeaderProtocols;
use pcap_parser::LegacyPcapReader;
use pcap_parser::traits::PcapReaderIterator;

use crate::quic;
use crate::quic_decrypt;

#[derive(Serialize, Deserialize, Clone)]
pub struct TlsSnapshot {
    pub raw_client_hello: Vec<u8>,
    pub ja3: Option<String>,
}

fn find_client_hello_in_buf(buf: &[u8]) -> Option<Vec<u8>> {
    // Search for TLS record 0x16 and handshake type 0x01
    let n = buf.len();
    for i in 0..n.saturating_sub(9) {
        if buf[i] == 0x16 && i + 5 < n && buf[i+5] == 0x01 {
            // record length at i+3..i+4
            if i + 5 + 4 <= n {
                let len = ((buf[i+3] as usize) << 8) | (buf[i+4] as usize);
                let end = i + 5 + len;
                if end <= n {
                    return Some(buf[i..end].to_vec());
                }
            }
        }
    }
    None
}

pub fn import_from_pcap(path: &str) -> Result<TlsSnapshot, String> {
    let file = File::open(path).map_err(|e| format!("open pcap failed: {}", e))?;
    let mut reader = LegacyPcapReader::new(65536, file).map_err(|_| "Failed to build pcap reader")?;

    // Map for UDP flows keyed by (src_ip,dst_ip,src_port,dst_port)
    let mut udp_flows: HashMap<(u32,u32,u16,u16), Vec<u8>> = HashMap::new();
    let mut tcp_payloads: Vec<u8> = Vec::new();

    loop {
        match reader.next() {
            Ok((offset, block)) => {
                match block {
                    pcap_parser::PcapBlockOwned::Legacy(pkt) => {
                        let data = pkt.data;
                        if let Some(ethernet_packet) = EthernetPacket::new(&data) {
                            match ethernet_packet.get_ethertype() {
                                EtherTypes::Ipv4 => {
                                    if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                                        let src_ip = u32::from(ipv4_packet.get_source());
                                        let dst_ip = u32::from(ipv4_packet.get_destination());

                                        match ipv4_packet.get_next_level_protocol() {
                                            IpNextHeaderProtocols::Tcp => {
                                                if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()) {
                                                    if tcp_packet.get_source() == 443 || tcp_packet.get_destination() == 443 {
                                                        tcp_payloads.extend_from_slice(tcp_packet.payload());
                                                    }
                                                }
                                            },
                                            IpNextHeaderProtocols::Udp => {
                                                if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
                                                    if udp_packet.get_source() == 443 || udp_packet.get_destination() == 443 {
                                                        let key = (src_ip, dst_ip, udp_packet.get_source(), udp_packet.get_destination());
                                                        udp_flows.entry(key).or_insert_with(Vec::new).extend_from_slice(udp_packet.payload());
                                                    }
                                                }
                                            },
                                            _ => {}
                                        }
                                    }
                                },
                                _ => {} // Ignoring non-IPv4 packets for now
                            }
                        }
                    },
                    _ => {}
                }
                reader.consume(offset);
            },
            Err(pcap_parser::PcapError::Eof) => break,
            Err(e) => return Err(format!("pcap read error: {:?}", e)),
        }
    }

    // First try to find ClientHello in TCP payloads
    if let Some(ch) = find_client_hello_in_buf(&tcp_payloads) {
        return Ok(TlsSnapshot { raw_client_hello: ch, ja3: None });
    }

    // Next, for each UDP flow, try to extract crypto from QUIC and find ClientHello
    for (k, buf) in udp_flows.iter() {
        let crypto = quic::extract_crypto_from_flow(buf);
        if crypto.len() > 0 {
            if let Some(ch) = find_client_hello_in_buf(&crypto) {
                return Ok(TlsSnapshot { raw_client_hello: ch, ja3: None });
            }
            // Attempt decryption using heuristics (requires DCID)
            // k is (src_ip,dst_ip,src_port,dst_port) - we might use parts as DCID placeholder
            // This is a best-effort call; in real flow we need DCID bytes from packet; here we try a fallback
            if let Ok(pt) = quic_decrypt::decrypt_initial_crypto(&[0u8,1,2,3], 16, &[], &crypto, &[0u8;12], &[]) {
                if let Some(ch) = find_client_hello_in_buf(&pt) {
                    return Ok(TlsSnapshot { raw_client_hello: ch, ja3: None });
                }
            }
        }
        // fallback: direct search in raw flow
        if let Some(ch) = find_client_hello_in_buf(buf) {
            return Ok(TlsSnapshot { raw_client_hello: ch, ja3: None });
        }
    }

    Err("ClientHello not found in pcap (tcp or udp 443)".to_string())
}
