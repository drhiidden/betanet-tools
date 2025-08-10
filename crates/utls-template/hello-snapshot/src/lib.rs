pub mod tls_capture;
pub mod h2_h3_capture;
pub mod import_pcap;
pub mod ja3;

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
