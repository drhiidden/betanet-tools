/// bo-htx: SANS-IO HTX tunnel primitives (canonical library)

/// Encode a simple tunnel header (magic + version + flags + length)
pub fn encode_tunnel_header(version: u8, flags: u8, payload_len: u16) -> [u8;8] {
    let mut h = [0u8;8];
    // magic 4 bytes
    h[0..4].copy_from_slice(&[0x42, 0x4f, 0x48, 0x54]);
    h[4] = version;
    h[5] = flags;
    h[6] = (payload_len >> 8) as u8;
    h[7] = (payload_len & 0xff) as u8;
    h
}

pub fn decode_tunnel_length(header: &[u8]) -> Option<u16> {
    if header.len() < 8 { return None; }
    let len = ((header[6] as u16) << 8) | header[7] as u16;
    Some(len)
}

/// Encode a simple length-prefixed frame
pub fn encode_frame(payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(2 + payload.len());
    let len = payload.len() as u16;
    out.push((len >> 8) as u8);
    out.push((len & 0xff) as u8);
    out.extend_from_slice(payload);
    out
}

/// Decode a single length-prefixed frame if present
pub fn decode_frame(buf: &[u8]) -> Option<Vec<u8>> {
    if buf.len() < 2 { return None; }
    let len = ((buf[0] as usize) << 8) | (buf[1] as usize);
    if buf.len() < 2 + len { return None; }
    Some(buf[2..2+len].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_header() {
        let h = encode_tunnel_header(1, 0, 5);
        assert_eq!(decode_tunnel_length(&h), Some(5));
    }

    #[test]
    fn smoke_tunnel_frame() {
        let payload = b"hello";
        let enc = encode_frame(payload);
        let dec = decode_frame(&enc).unwrap();
        assert_eq!(dec, payload);
    }
}


