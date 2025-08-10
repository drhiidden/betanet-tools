use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::quic;

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

pub fn import_pcap(path: &str) -> Result<TlsSnapshot, String> {
    // Use pcap crate to read packets
    let mut cap = pcap::Capture::from_file(path).map_err(|e| format!("open pcap failed: {}", e))?;

    // Map for UDP flows keyed by (src_ip,dst_ip,src_port,dst_port)
    let mut udp_flows: HashMap<(u32,u32,u16,u16), Vec<u8>> = HashMap::new();
    let mut tcp_payloads: Vec<u8> = Vec::new();

    loop {
        match cap.next_packet() {
            Ok(packet) => {
                let data = packet.data;
                if data.len() < 14 { continue; }
                // EtherType
                let ethertype = u16::from_be_bytes([data[12], data[13]]);
                if ethertype != 0x0800 { continue; } // only IPv4 for now
                if data.len() < 14 + 20 { continue; }
                // IPv4
                let ip_off = 14;
                let ihl = (data[ip_off] & 0x0f) as usize * 4;
                if data.len() < ip_off + ihl { continue; }
                let proto = data[ip_off+9];
                let src_ip = u32::from_be_bytes([data[ip_off+12], data[ip_off+13], data[ip_off+14], data[ip_off+15]]);
                let dst_ip = u32::from_be_bytes([data[ip_off+16], data[ip_off+17], data[ip_off+18], data[ip_off+19]]);

                if proto == 6u8 { // TCP
                    if data.len() < ip_off + ihl + 20 { continue; }
                    let tcp_off = ip_off + ihl;
                    let src_port = u16::from_be_bytes([data[tcp_off], data[tcp_off+1]]);
                    let dst_port = u16::from_be_bytes([data[tcp_off+2], data[tcp_off+3]]);
                    let data_offset = ((data[tcp_off+12] >> 4) as usize) * 4;
                    let payload_off = tcp_off + data_offset;
                    if payload_off >= data.len() { continue; }
                    let payload = &data[payload_off..];
                    // If port 443, collect payloads
                    if src_port == 443 || dst_port == 443 {
                        tcp_payloads.extend_from_slice(payload);
                    }
                } else if proto == 17u8 { // UDP
                    let udp_off = ip_off + ihl;
                    if data.len() < udp_off + 8 { continue; }
                    let src_port = u16::from_be_bytes([data[udp_off], data[udp_off+1]]);
                    let dst_port = u16::from_be_bytes([data[udp_off+2], data[udp_off+3]]);
                    let payload_off = udp_off + 8;
                    if payload_off >= data.len() { continue; }
                    let payload = &data[payload_off..];
                    if src_port == 443 || dst_port == 443 {
                        let key = (src_ip, dst_ip, src_port, dst_port);
                        udp_flows.entry(key).or_insert_with(Vec::new).extend_from_slice(payload);
                    }
                }
            }
            Err(pcap::Error::NoMorePackets) => break,
            Err(e) => return Err(format!("pcap read error: {}", e)),
        }
    }

    // First try to find ClientHello in TCP payloads
    if let Some(ch) = find_client_hello_in_buf(&tcp_payloads) {
        return Ok(TlsSnapshot { raw_client_hello: ch, ja3: None });
    }

    // Next, for each UDP flow, try to extract crypto from QUIC and find ClientHello
    for (_k, buf) in udp_flows.iter() {
        let crypto = quic::extract_crypto_from_flow(buf);
        if crypto.len() > 0 {
            if let Some(ch) = find_client_hello_in_buf(&crypto) {
                return Ok(TlsSnapshot { raw_client_hello: ch, ja3: None });
            }
        }
        // fallback: direct search in raw flow
        if let Some(ch) = find_client_hello_in_buf(buf) {
            return Ok(TlsSnapshot { raw_client_hello: ch, ja3: None });
        }
    }

    Err("ClientHello not found in pcap (tcp or udp 443)".to_string())
}
