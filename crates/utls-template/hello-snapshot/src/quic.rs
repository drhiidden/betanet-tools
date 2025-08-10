// Heuristic QUIC parser to extract CRYPTO frame payloads from Initial packets

pub fn read_varint(buf: &[u8], pos: usize) -> Option<(u64, usize)> {
    if pos >= buf.len() { return None; }
    let b = buf[pos];
    let prefix = b >> 6;
    let len = match prefix { 0 => 1usize, 1 => 2usize, 2 => 4usize, 3 => 8usize, _ => 1usize };
    if pos + len > buf.len() { return None; }
    let mut value: u64 = (b & 0x3f) as u64;
    for i in 1..len {
        value = (value << 8) | (buf[pos + i] as u64);
    }
    Some((value, len))
}

fn is_long_header(first: u8) -> bool {
    (first & 0x80) != 0
}

// Try to extract CRYPTO frame payloads from a QUIC packet payload (after UDP/IP headers)
pub fn extract_crypto_from_quic_packet(payload: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    if payload.len() < 6 { return out; }
    let first = payload[0];
    if !is_long_header(first) { return out; }
    // version at 1..5
    if payload.len() < 6 { return out; }
    let version = u32::from_be_bytes([payload[1], payload[2], payload[3], payload[4]]);
    if version == 0 { return out; }
    // parse DCID/SCID lengths at pos 5
    let mut idx = 5;
    if idx >= payload.len() { return out; }
    let dcid_len = payload[idx] as usize; idx += 1;
    if idx + dcid_len > payload.len() { return out; }
    idx += dcid_len;
    if idx >= payload.len() { return out; }
    let scid_len = payload[idx] as usize; idx += 1;
    if idx + scid_len > payload.len() { return out; }
    idx += scid_len;
    // token length: varint
    if let Some((toklen, toklen_bytes)) = read_varint(payload, idx) {
        idx += toklen_bytes;
        if idx + (toklen as usize) > payload.len() { return out; }
        idx += toklen as usize;
    } else {
        return out;
    }
    // length of packet (varint) - skip
    if let Some((_plen, plen_bytes)) = read_varint(payload, idx) {
        idx += plen_bytes;
    } else {
        return out;
    }
    if idx >= payload.len() { return out; }
    // Now idx points to frames area. Iterate frames and collect CRYPTO frames (type 0x06)
    while idx < payload.len() {
        // read frame type varint
        if let Some((ftype, ftype_len)) = read_varint(payload, idx) {
            idx += ftype_len;
            // CRYPTO frame has type 0x06
            if ftype == 0x06 {
                if let Some((flen, flen_len)) = read_varint(payload, idx) {
                    idx += flen_len;
                    let flen_usize = flen as usize;
                    if idx + flen_usize <= payload.len() {
                        out.extend_from_slice(&payload[idx..idx+flen_usize]);
                        idx += flen_usize;
                        continue;
                    } else { break; }
                } else { break; }
            } else {
                // skip known frame types minimally: for simplicity, try to skip a length varint if applicable
                // many frames have varint length (e.g., STREAM). We'll attempt to skip if next bytes look like varint.
                if ftype == 0x10 || ftype == 0x1d || ftype == 0x1e { // STREAM types (heuristic)
                    // STREAM: stream id varint + offset varint + length varint + data
                    if let Some((_sid, sid_len)) = read_varint(payload, idx) {
                        idx += sid_len;
                        if let Some((_off, off_len)) = read_varint(payload, idx) {
                            idx += off_len;
                            if let Some((dlen, dlen_len)) = read_varint(payload, idx) {
                                idx += dlen_len + dlen as usize;
                                continue;
                            } else { break; }
                        } else { break; }
                    } else { break; }
                }
                // fallback: break to avoid infinite loop
                break;
            }
        } else { break; }
    }
    out
}

// Given a buffer that is concatenation of UDP payloads for a flow, collect CRYPTO bytes across packets
pub fn extract_crypto_from_flow(buf: &[u8]) -> Vec<u8> {
    let mut acc: Vec<u8> = Vec::new();
    // try to scan for QUIC long headers and extract
    let mut pos = 0usize;
    while pos + 6 <= buf.len() {
        if (buf[pos] & 0x80) != 0 && pos + 5 < buf.len() {
            // try to extract from pos
            let slice = &buf[pos..];
            let extracted = extract_crypto_from_quic_packet(slice);
            if !extracted.is_empty() {
                acc.extend_from_slice(&extracted);
            }
        }
        pos += 1;
    }
    acc
}
