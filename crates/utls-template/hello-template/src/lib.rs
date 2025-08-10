use serde::{Serialize, Deserialize};
use std::io::Cursor;
use std::time::{Duration, SystemTime};
use pcap::{self, PacketHeader, Linktype, Writer};

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

pub struct Encoder;
impl Encoder {
    pub fn encode_client_hello(t: &HelloTemplate, emit_pcap: bool) -> Result<EncodedClientHello, String> {
        // Construye Record TLS Handshake with ClientHello preservando orden.
        // Inserta GREASE (0x0a0a, 0x1a1a, ...) según GreaseMode y en slots específicos.
        // Asegura el padding determinista.
        let encoded_bytes = vec![0x16, 0x03, 0x01, 0x00, 0x01]; // Placeholder: bytes codificados del ClientHello

        let mut pcap_bytes: Option<Vec<u8>> = None;

        if emit_pcap {
            let mut buffer = Cursor::new(Vec::new());
            let mut writer = Writer::new(&mut buffer, Linktype::ETHERNET)
                .map_err(|e| format!("Failed to create pcap writer: {}", e))?;

            let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| format!("Failed to get timestamp: {}", e))?;

            let packet_header = PacketHeader {
                ts: libc::timeval {
                    tv_sec: timestamp.as_secs() as i64,
                    tv_usec: timestamp.subsec_micros() as i32,
                },
                caplen: encoded_bytes.len() as u32,
                len: encoded_bytes.len() as u32,
            };
            writer.write_packet(&packet_header, &encoded_bytes)
                .map_err(|e| format!("Failed to write ClientHello to pcap: {}", e))?;

            pcap_bytes = Some(buffer.into_inner());
        }

        Ok(EncodedClientHello {
            raw_bytes: encoded_bytes,
            pcap_bytes,
        })
    }
}
