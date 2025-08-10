// bo-aead: wrappers mínimos para AEAD
use bo_core::prelude::*;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use chacha20poly1305::aead::{Aead, KeyInit};

fn le64_to_bytes(v: u64) -> [u8;8] {
    v.to_le_bytes()
}

fn ns_and_counter_to_nonce(ns: &[u8], counter: u64) -> [u8;12] {
    if ns.len() != 12 { panic!("ns must be 12 bytes") }
    let mut nonce = [0u8;12];
    let cbytes = le64_to_bytes(counter);
    for i in 0..8 { nonce[i] = ns[i] ^ cbytes[i]; }
    for i in 8..12 { nonce[i] = ns[i]; }
    nonce
}

pub fn seal_chacha20poly1305(key: &[u8], ns: &[u8], counter_le64: u64, aad: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    if key.len() != 32 { return Err(CoreError::Io("invalid key len".into())); }
    let key = Key::from_slice(key);
    let aead = ChaCha20Poly1305::new(key);
    let nonce_bytes = ns_and_counter_to_nonce(ns, counter_le64);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ct = aead.encrypt(nonce, chacha20poly1305::aead::Payload { msg: plaintext, aad }).map_err(|_| CoreError::Io("encrypt fail".into()))?;
    Ok(ct)
}

pub fn open_chacha20poly1305(key: &[u8], ns: &[u8], counter_le64: u64, aad: &[u8], ct_with_tag: &[u8]) -> Result<Vec<u8>> {
    if key.len() != 32 { return Err(CoreError::Io("invalid key len".into())); }
    let key = Key::from_slice(key);
    let aead = ChaCha20Poly1305::new(key);
    let nonce_bytes = ns_and_counter_to_nonce(ns, counter_le64);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let pt = aead.decrypt(nonce, chacha20poly1305::aead::Payload { msg: ct_with_tag, aad }).map_err(|_| CoreError::Io("decrypt fail".into()))?;
    Ok(pt)
}


