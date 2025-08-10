pub mod export_utls;

use serde::{Serialize, Deserialize};
use std::time::SystemTime;

mod grease;
use grease::GreaseMode;

#[derive(Serialize, Deserialize)]
pub struct HelloTemplate {
    pub tls_version: u16,                // e.g., TLS1_3 (0x0304)
    pub client_random: Option<[u8; 32]>,
    pub session_id: Option<Vec<u8>>,
    pub cipher_suites: Vec<u16>,                // orden exacto
    pub compression_methods: Vec<u8>,
    pub extensions: Vec<Extension>,             // orden exacto
    pub grease_mode: GreaseMode,
    pub psk_key_exchange_modes: Option<Vec<u8>>, // Para 0-RTT/PSK modes
}

#[derive(Serialize, Deserialize)]
pub enum Extension {
    ServerName { host: String },
    SupportedVersions { versions: Vec<u16> },
    SupportedGroups { groups: Vec<u16>, grease_slots: Vec<u16> },
    SignatureAlgorithms { algs: Vec<u16> },
    KeyShare { shares: Vec<(u16 /*group*/, Vec<u8> /*key*/)> },
    Alpn { protocols: Vec<String> },
    Padding { len: usize }, // Ahora determinista
    EchOuterStub { config_id: Vec<u8> }, // ECH outer / grease
    ApplicationSettings { protocols: Vec<String>, data: Vec<u8> }, // ALPS
    PskKeyExchangeModes { modes: Vec<u8> }, // PSK modes
    Unknown { typ: u16, bytes: Vec<u8> },
}

pub struct EncodedClientHello {
    pub raw_bytes: Vec<u8>,
    pub pcap_bytes: Option<Vec<u8>>,
}

fn write_u16_be(buf: &mut Vec<u8>, v: u16) {
    buf.push((v >> 8) as u8);
    buf.push((v & 0xff) as u8);
}
fn write_u24_be(buf: &mut Vec<u8>, v: usize) {
    buf.push(((v >> 16) & 0xff) as u8);
    buf.push(((v >> 8) & 0xff) as u8);
    buf.push((v & 0xff) as u8);
}
fn write_u32_le(buf: &mut Vec<u8>, v: u32) {
    buf.extend_from_slice(&v.to_le_bytes());
}
fn write_u16_le(buf: &mut Vec<u8>, v: u16) {
    buf.extend_from_slice(&v.to_le_bytes());
}

pub struct Encoder;
impl Encoder {
    pub fn encode_client_hello(t: &HelloTemplate, emit_pcap: bool) -> Result<EncodedClientHello, String> {
        // Build handshake body (ClientHello)
        let mut body: Vec<u8> = Vec::new();

        // legacy_version (2 bytes) - use 0x0303 for compatibility
        write_u16_be(&mut body, 0x0303);

        // random (32 bytes)
        if let Some(r) = &t.client_random {
            body.extend_from_slice(r);
        } else {
            body.extend_from_slice(&[0u8; 32]);
        }

        // session_id
        if let Some(sid) = &t.session_id {
            body.push(sid.len() as u8);
            body.extend_from_slice(sid);
        } else {
            body.push(0u8);
        }

        // cipher_suites (2 bytes len + suites)
        let cs_len = t.cipher_suites.len() * 2;
        write_u16_be(&mut body, cs_len as u16);
        for cs in &t.cipher_suites {
            write_u16_be(&mut body, *cs);
        }

        // compression_methods
        body.push(t.compression_methods.len() as u8);
        body.extend_from_slice(&t.compression_methods);

        // extensions - prepare extensions payload first
        let mut exts: Vec<u8> = Vec::new();
        for ext in &t.extensions {
            match ext {
                Extension::ServerName { host } => {
                    // ext type 0x0000
                    write_u16_be(&mut exts, 0x0000);
                    // build server name list
                    let mut data: Vec<u8> = Vec::new();
                    // name type 0, length, host
                    let host_bytes = host.as_bytes();
                    let mut names: Vec<u8> = Vec::new();
                    names.push(0u8);
                    write_u16_be(&mut names, host_bytes.len() as u16);
                    names.extend_from_slice(host_bytes);
                    // overall list length
                    write_u16_be(&mut data, names.len() as u16);
                    data.extend_from_slice(&names);
                    write_u16_be(&mut exts, data.len() as u16);
                    exts.extend_from_slice(&data);
                }
                Extension::SupportedVersions { versions } => {
                    write_u16_be(&mut exts, 0x002b);
                    // data: uint8 length then list of uint16
                    let mut data: Vec<u8> = Vec::new();
                    data.push((versions.len() * 2) as u8);
                    for v in versions { write_u16_be(&mut data, *v); }
                    write_u16_be(&mut exts, data.len() as u16);
                    exts.extend_from_slice(&data);
                }
                Extension::SupportedGroups { groups, .. } => {
                    write_u16_be(&mut exts, 0x000a);
                    let mut data: Vec<u8> = Vec::new();
                    write_u16_be(&mut data, (groups.len() * 2) as u16);
                    for g in groups { write_u16_be(&mut data, *g); }
                    write_u16_be(&mut exts, data.len() as u16);
                    exts.extend_from_slice(&data);
                }
                Extension::SignatureAlgorithms { algs } => {
                    write_u16_be(&mut exts, 0x000d);
                    let mut data: Vec<u8> = Vec::new();
                    write_u16_be(&mut data, (algs.len() * 2) as u16);
                    for a in algs { write_u16_be(&mut data, *a); }
                    write_u16_be(&mut exts, data.len() as u16);
                    exts.extend_from_slice(&data);
                }
                Extension::KeyShare { shares } => {
                    write_u16_be(&mut exts, 0x0033);
                    let mut data: Vec<u8> = Vec::new();
                    // list length placeholder
                    let mut list: Vec<u8> = Vec::new();
                    for (group, key) in shares {
                        write_u16_be(&mut list, *group);
                        write_u16_be(&mut list, key.len() as u16);
                        list.extend_from_slice(key);
                    }
                    write_u16_be(&mut data, list.len() as u16);
                    data.extend_from_slice(&list);
                    write_u16_be(&mut exts, data.len() as u16);
                    exts.extend_from_slice(&data);
                }
                Extension::Alpn { protocols } => {
                    write_u16_be(&mut exts, 0x0010);
                    let mut data: Vec<u8> = Vec::new();
                    // alpn protocols: length-prefixed vector
                    let mut protos: Vec<u8> = Vec::new();
                    for p in protocols {
                        let pb = p.as_bytes();
                        protos.push(pb.len() as u8);
                        protos.extend_from_slice(pb);
                    }
                    write_u16_be(&mut data, protos.len() as u16);
                    data.extend_from_slice(&protos);
                    write_u16_be(&mut exts, data.len() as u16);
                    exts.extend_from_slice(&data);
                }
                Extension::Padding { len } => {
                    write_u16_be(&mut exts, 0x0015);
                    let mut data: Vec<u8> = vec![0u8; *len];
                    write_u16_be(&mut exts, data.len() as u16);
                    exts.extend_from_slice(&data);
                }
                Extension::EchOuterStub { config_id } => {
                    write_u16_be(&mut exts, 0xfe0d);
                    let mut data: Vec<u8> = Vec::new();
                    data.extend_from_slice(config_id);
                    write_u16_be(&mut exts, data.len() as u16);
                    exts.extend_from_slice(&data);
                }
                Extension::ApplicationSettings { protocols, data } => {
                    // Use extension type 0xff01 as placeholder for ALPS (implementation specific)
                    write_u16_be(&mut exts, 0xff01);
                    let mut d: Vec<u8> = Vec::new();
                    // encode protocols as simple list
                    let mut protos: Vec<u8> = Vec::new();
                    for p in protocols { let pb=p.as_bytes(); protos.push(pb.len() as u8); protos.extend_from_slice(pb); }
                    write_u16_be(&mut d, protos.len() as u16);
                    d.extend_from_slice(&protos);
                    d.extend_from_slice(data);
                    write_u16_be(&mut exts, d.len() as u16);
                    exts.extend_from_slice(&d);
                }
                Extension::PskKeyExchangeModes { modes } => {
                    write_u16_be(&mut exts, 0x002d);
                    let mut data: Vec<u8> = Vec::new();
                    data.push(modes.len() as u8);
                    data.extend_from_slice(modes);
                    write_u16_be(&mut exts, data.len() as u16);
                    exts.extend_from_slice(&data);
                }
                Extension::Unknown { typ, bytes } => {
                    write_u16_be(&mut exts, *typ);
                    write_u16_be(&mut exts, bytes.len() as u16);
                    exts.extend_from_slice(bytes);
                }
            }
        }

        // write extensions length
        write_u16_be(&mut body, exts.len() as u16);
        body.extend_from_slice(&exts);

        // Now wrap body into Handshake record
        let mut handshake: Vec<u8> = Vec::new();
        // Handshake header: type (1 = ClientHello)
        handshake.push(0x01u8);
        // length 3 bytes
        write_u24_be(&mut handshake, body.len());
        handshake.extend_from_slice(&body);

        // TLS record header
        let mut record: Vec<u8> = Vec::new();
        record.push(22u8); // Handshake
        write_u16_be(&mut record, 0x0303); // version
        write_u16_be(&mut record, handshake.len() as u16);
        record.extend_from_slice(&handshake);

        let encoded_bytes = record;

        // optionally produce pcap manually (little-endian pcap global header + packets)
        let mut pcap_bytes: Option<Vec<u8>> = None;
        if emit_pcap {
            let mut pcap: Vec<u8> = Vec::new();
            // pcap global header (little-endian: 0xd4c3b2a1)
            write_u32_le(&mut pcap, 0xd4c3b2a1);
            write_u16_le(&mut pcap, 2); // version major
            write_u16_le(&mut pcap, 4); // version minor
            write_u32_le(&mut pcap, 0); // thiszone
            write_u32_le(&mut pcap, 0); // sigfigs
            write_u32_le(&mut pcap, 65535); // snaplen
            write_u32_le(&mut pcap, 1); // network (LINKTYPE_ETHERNET)

            let now = SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|e| format!("ts failed: {}", e))?;
            let sec = now.as_secs() as u32;
            let usec = now.subsec_micros() as u32;
            write_u32_le(&mut pcap, sec);
            write_u32_le(&mut pcap, usec);
            write_u32_le(&mut pcap, encoded_bytes.len() as u32);
            write_u32_le(&mut pcap, encoded_bytes.len() as u32);
            pcap.extend_from_slice(&encoded_bytes);

            pcap_bytes = Some(pcap);
        }

        Ok(EncodedClientHello { raw_bytes: encoded_bytes, pcap_bytes })
    }
}
