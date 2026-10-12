use bo_htx::{encode_frame, decode_frame};

pub fn encode_frame_example(payload: &[u8]) -> Vec<u8> {
    encode_frame(payload)
}

pub fn decode_frame_example(buf: &[u8]) -> Option<Vec<u8>> {
    decode_frame(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_tunnel() {
        let p = b"hello";
        let enc = encode_frame_example(p);
        let dec = decode_frame_example(&enc).unwrap();
        assert_eq!(dec, p);
    }
}
