pub mod tls_capture;
pub mod h2_h3_capture;
pub mod import_pcap;
pub mod ja3;
pub mod ja4;

use md5::{Md5, Digest};
use crate::import_pcap::TlsSnapshot;

pub fn snapshot_to_ja3(snap: &mut TlsSnapshot) {
    if let Ok(j) = ja3::ja3_from_raw(&snap.raw_client_hello) {
        snap.ja3 = Some(j);
    } else {
        // fallback to md5
        let mut hasher = md5::Md5::new();
        hasher.update(&snap.raw_client_hello);
        let res = hasher.finalize();
        snap.ja3 = Some(format!("{:x}", res));
    }
}

pub fn ja4_h2_from_raw(raw: &[u8]) -> Option<String> {
    if let Some(h2) = ja4::extract_h2_settings_from_bytes(raw) {
        Some(ja4::ja4_h2(&h2))
    } else { None }
}

pub fn ja4_h3_from_raw(raw: &[u8]) -> Option<String> {
    if let Some(h3) = ja4::extract_h3_settings_from_bytes(raw) {
        Some(ja4::ja4_h3(&h3))
    } else { None }
}
