/// tunnel_mock: encapsulación / decapsulación simple (length-prefixed)

pub fn encode_frame(payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(2 + payload.len());
    let len = payload.len() as u16;
    out.push((len >> 8) as u8);
    out.push((len & 0xff) as u8);
    out.extend_from_slice(payload);
    out
}

pub fn decode_frame(mut buf: &[u8]) -> Option<Vec<u8>> {
    if buf.len() < 2 { return None; }
    let len = ((buf[0] as usize) << 8) | (buf[1] as usize);
    if buf.len() < 2 + len { return None; }
    Some(buf[2..2+len].to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_tunnel() {
        let p = b"hello";
        let enc = encode_frame(p);
        let dec = decode_frame(&enc).unwrap();
        assert_eq!(dec, p);
    }
}
