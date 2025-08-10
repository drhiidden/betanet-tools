pub struct H2Snapshot {
    pub settings: Vec<(u16, u32)>, // En orden visto
    pub pseudo_headers_order: Vec<String>, // En orden visto
}
pub struct H3Snapshot {
    pub settings: Vec<(u64, u64)>, // HTTP/3 SETTINGS, en orden visto
    pub tp: Vec<(u64, Vec<u8>)>,   // Transport Parameters, en orden visto
    pub qpack: (u64, u64),         // QPACK specific settings
}

pub fn capture_h2_h3(/* ... stream, etc. ... */) -> (H2Snapshot, H3Snapshot) {
    // Se realiza un primer request minimal GET / con headers canónicos
    // Se recogen SETTINGS y TPs exactamente en orden de aparición
    todo!()
}
