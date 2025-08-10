use std::fs::File;
use std::io::Read;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TlsSnapshot {
    pub raw_client_hello: Vec<u8>,
    pub ja3: Option<String>,
}

pub fn import_pcap(path: &str) -> Result<TlsSnapshot, String> {
    let mut f = File::open(path).map_err(|e| format!("open pcap failed: {}", e))?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).map_err(|e| format!("read pcap failed: {}", e))?;
    // naive search for TLS handshake type 0x16 then ClientHello (handshake 0x01)
    for i in 0..buf.len()-5 {
        if buf[i] == 0x16 && buf[i+5] == 0x01 {
            // crude: extract from i until end of record length
            // record length at i+3..i+4 (2 bytes)
            if i+5+4 < buf.len() {
                let len_hi = buf[i+3] as usize;
                let len_lo = buf[i+4] as usize;
                let rlen = (len_hi<<8) | len_lo;
                let end = i + 5 + rlen;
                if end <= buf.len() {
                    let clienthello = buf[i..end].to_vec();
                    return Ok(TlsSnapshot { raw_client_hello: clienthello, ja3: None });
                }
            }
        }
    }
    Err("ClientHello not found in pcap (naive search)".to_string())
}
