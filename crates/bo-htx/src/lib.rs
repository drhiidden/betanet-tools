/// bo-htx: SANS-IO HTX tunnel primitives

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_header() {
        let h = encode_tunnel_header(1, 0, 5);
        assert_eq!(decode_tunnel_length(&h), Some(5));
    }
}


