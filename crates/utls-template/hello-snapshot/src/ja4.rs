use md5::{Md5, Digest};

#[derive(Debug)]
pub struct H2Settings {
    pub settings: Vec<(u16, u32)>, // (id, value)
    pub pseudo_headers_order: Vec<String>,
}

#[derive(Debug)]
pub struct H3Settings {
    pub settings: Vec<(u64, u64)>, // (id, value)
    pub transport_params: Vec<(u64, Vec<u8>)>,
    pub qpack: Option<(u64,u64)>,
}

// Heuristic: busca el PREFACE de HTTP/2 y extrae el primer frame SETTINGS que encuentre
pub fn extract_h2_settings_from_bytes(raw: &[u8]) -> Option<H2Settings> {
    let preface = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";
    if let Some(pos) = raw.windows(preface.len()).position(|w| w == preface) {
        let mut idx = pos + preface.len();
        // parse frames
        while idx + 9 <= raw.len() {
            let len = ((raw[idx] as usize) << 16) | ((raw[idx+1] as usize) << 8) | (raw[idx+2] as usize);
            let typ = raw[idx+3];
            // let flags = raw[idx+4];
            // stream id 4 bytes
            // If SETTINGS (type 0x4)
            if typ == 0x4 {
                let payload_start = idx + 9;
                if payload_start + len <= raw.len() {
                    let mut settings = Vec::new();
                    let mut p = payload_start;
                    while p + 6 <= payload_start + len {
                        let id = ((raw[p] as u16) << 8) | raw[p+1] as u16;
                        let val = ((raw[p+2] as u32) << 24) | ((raw[p+3] as u32) << 16) | ((raw[p+4] as u32) << 8) | (raw[p+5] as u32);
                        settings.push((id, val));
                        p += 6;
                    }
                    return Some(H2Settings { settings, pseudo_headers_order: Vec::new() });
                }
            }
            idx += 9 + len;
        }
    }
    None
}

// Heuristic for H3: search for byte 0x04 (SETTINGS) in a stream and read following length as varint (not robust)
fn read_varint_at(raw: &[u8], pos: usize) -> Option<(u64, usize)> {
    if pos >= raw.len() { return None; }
    let b = raw[pos];
    let prefix = b >> 6;
    let len = match prefix { 0 => 1, 1 => 2, 2 => 4, 3 => 8, _ => 1 };
    if pos + len > raw.len() { return None; }
    let mut value: u64 = (b & 0x3f) as u64;
    for i in 1..len {
        value = (value << 8) | raw[pos+i] as u64;
    }
    Some((value, len))
}

pub fn extract_h3_settings_from_bytes(raw: &[u8]) -> Option<H3Settings> {
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == 0x04 {
            // read varint length at i+1
            if let Some((len, lbytes)) = read_varint_at(raw, i+1) {
                let start = i+1+lbytes;
                if start + (len as usize) <= raw.len() {
                    // parse simple key/value pairs as varint-id,varint-value sequence (best-effort)
                    let mut p = start;
                    let mut settings = Vec::new();
                    while p < start + (len as usize) {
                        if let Some((id, idlen)) = read_varint_at(raw, p) {
                            p += idlen;
                            if let Some((v, vlen)) = read_varint_at(raw, p) {
                                p += vlen;
                                settings.push((id, v));
                            } else { break; }
                        } else { break; }
                    }
                    return Some(H3Settings { settings, transport_params: Vec::new(), qpack: None });
                }
            }
        }
        i += 1;
    }
    None
}

pub fn ja4_h2(h2: &H2Settings) -> String {
    // canonical string: ids:values joined by '-'
    let s = h2.settings.iter().map(|(id,val)| format!("{}:{}", id, val)).collect::<Vec<_>>().join("-");
    // return md5 hex of that string for compactness
    let mut hasher = Md5::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn ja4_h3(h3: &H3Settings) -> String {
    let s = h3.settings.iter().map(|(id,val)| format!("{}:{}", id, val)).collect::<Vec<_>>().join("-");
    let mut hasher = Md5::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}
