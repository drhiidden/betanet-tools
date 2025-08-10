use md5::{Md5, Digest};

fn is_grease(u: u16) -> bool {
    // Standard GREASE values
    matches!(u, 0x0a0a | 0x1a1a | 0x2a2a | 0x3a3a | 0x4a4a | 0x5a5a | 0x6a6a | 0x7a7a |
                0x8a8a | 0x9a9a | 0xaaaa | 0xbaba | 0xcaca | 0xdada | 0xeaea | 0xfafa)
}

pub struct ParsedClientHello {
    pub version: u16,
    pub cipher_suites: Vec<u16>,
    pub extensions: Vec<u16>,
    pub supported_groups: Vec<u16>,
    pub ec_point_formats: Vec<u8>,
}

pub fn parse_client_hello(raw: &[u8]) -> Result<ParsedClientHello, String> {
    // Expect record header
    if raw.len() < 5 { return Err("too short for record".into()); }
    if raw[0] != 22 { return Err("not a handshake record".into()); }
    let _record_len = ((raw[3] as usize) << 8) | (raw[4] as usize);
    // Handshake
    if raw.len() < 9 { return Err("too short for handshake".into()); }
    if raw[5] != 0x01 { return Err("not a ClientHello handshake".into()); }
    // handshake length 3 bytes at 6..9
    let body_offset = 9;
    if raw.len() < body_offset + 2 { return Err("no legacy_version".into()); }
    let version = ((raw[body_offset] as u16) << 8) | raw[body_offset+1] as u16;
    let mut idx = body_offset + 2 + 32; // skip random
    if raw.len() <= idx { return Err("unexpected end after random".into()); }
    // session id
    let sid_len = raw[idx] as usize; idx += 1 + sid_len;
    if raw.len() < idx+2 { return Err("no cipher suites length".into()); }
    let cs_len = ((raw[idx] as usize) << 8) | raw[idx+1] as usize; idx += 2;
    if raw.len() < idx + cs_len { return Err("cipher suites truncated".into()); }
    let mut cipher_suites = Vec::new();
    let mut cs_idx = idx;
    while cs_idx < idx + cs_len {
        let cs = ((raw[cs_idx] as u16) << 8) | raw[cs_idx+1] as u16;
        if !is_grease(cs) { cipher_suites.push(cs); }
        cs_idx += 2;
    }
    idx += cs_len;
    // compression methods
    if raw.len() <= idx { return Err("no compression methods".into()); }
    let cm_len = raw[idx] as usize; idx += 1 + cm_len;
    if raw.len() < idx { return Err("compression methods truncated".into()); }
    // extensions
    if raw.len() < idx + 2 { return Ok(ParsedClientHello { version, cipher_suites, extensions: vec![], supported_groups: vec![], ec_point_formats: vec![] }); }
    let exts_len = ((raw[idx] as usize) << 8) | raw[idx+1] as usize; idx += 2;
    if raw.len() < idx + exts_len { return Err("extensions truncated".into()); }
    let mut exts_idx = idx;
    let end_exts = idx + exts_len;
    let mut extensions = Vec::new();
    let mut supported_groups = Vec::new();
    let mut ec_point_formats = Vec::new();
    while exts_idx + 4 <= end_exts {
        let ext_type = ((raw[exts_idx] as u16) << 8) | raw[exts_idx+1] as u16;
        let ext_len = ((raw[exts_idx+2] as usize) << 8) | raw[exts_idx+3] as usize;
        let data_start = exts_idx + 4;
        let data_end = data_start + ext_len;
        if data_end > end_exts { break; }
        if !is_grease(ext_type) { extensions.push(ext_type); }
        match ext_type {
            0x000a => { // supported_groups (named groups)
                if ext_len >= 2 {
                    let list_len = ((raw[data_start] as usize) << 8) | raw[data_start+1] as usize;
                    let mut gi = data_start + 2;
                    while gi + 1 < data_start + 2 + list_len && gi + 1 < data_end {
                        let g = ((raw[gi] as u16) << 8) | raw[gi+1] as u16;
                        if !is_grease(g) { supported_groups.push(g); }
                        gi += 2;
                    }
                }
            }
            0x000b => { // ec point formats
                if ext_len >= 1 {
                    let list_len = raw[data_start] as usize;
                    let mut pi = data_start + 1;
                    for _ in 0..list_len {
                        if pi >= data_end { break; }
                        ec_point_formats.push(raw[pi]); pi += 1;
                    }
                }
            }
            _ => {}
        }
        exts_idx = data_end;
    }
    Ok(ParsedClientHello { version, cipher_suites, extensions, supported_groups, ec_point_formats })
}

pub fn ja3_from_raw(raw: &[u8]) -> Result<String, String> {
    let parsed = parse_client_hello(raw)?;
    // build JA3 string
    let mut s = String::new();
    s.push_str(&format!("{}", parsed.version));
    s.push(',');
    // ciphers
    let cs_str = parsed.cipher_suites.iter().map(|c| c.to_string()).collect::<Vec<_>>().join("-");
    s.push_str(&cs_str);
    s.push(',');
    let ex_str = parsed.extensions.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("-");
    s.push_str(&ex_str);
    s.push(',');
    let sg_str = parsed.supported_groups.iter().map(|g| g.to_string()).collect::<Vec<_>>().join("-");
    s.push_str(&sg_str);
    s.push(',');
    let pf_str = parsed.ec_point_formats.iter().map(|p| p.to_string()).collect::<Vec<_>>().join("-");
    s.push_str(&pf_str);

    // MD5
    let mut hasher = Md5::new();
    hasher.update(s.as_bytes());
    let res = hasher.finalize();
    Ok(format!("{:x}", res))
}
