pub mod tls_capture;
pub mod h2_h3_capture;
pub mod import_pcap;

use md5::{Md5, Digest};
use crate::import_pcap::TlsSnapshot;

pub fn snapshot_to_ja3(snap: &mut TlsSnapshot) {
    // Very simplified JA3: MD5 of raw bytes for now (placeholder)
    let mut hasher = Md5::new();
    hasher.update(&snap.raw_client_hello);
    let res = hasher.finalize();
    snap.ja3 = Some(format!("{:x}", res));
}
