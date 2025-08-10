use hkdf::Hkdf;
use sha2::Sha256;
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes128Gcm, Aes256Gcm, Nonce};
use sha2::Digest;

// Simplified QUIC initial decryption flow (IETF QUIC v1)
// - Derive initial secrets from DCID and version using Hkdf with salt (per QUIC v1)
// - Use derived key/iv to decrypt AEAD (CRYPTO) payload

// These constants/salts are as per drafts (example for v1):
const INITIAL_SALT_V1: &[u8] = &[
    0x38, 0x76, 0x2c, 0xf7, 0xf5, 0x59, 0x34, 0xb3,
    0x4d, 0x17, 0x9a, 0xe6, 0xa4, 0xc8, 0x0c, 0xad,
    0xcc, 0xbb, 0x7f, 0x0a,
];

fn hkdf_extract_expand(salt: &[u8], ikm: &[u8], info: &[u8], out_len: usize) -> Result<Vec<u8>, String> {
    let hk = Hkdf::<Sha256>::new(Some(salt), ikm);
    let mut okm = vec![0u8; out_len];
    hk.expand(info, &mut okm).map_err(|_| "hkdf expand error".to_string())?;
    Ok(okm)
}

// Derive initial secret using salt and DCID
fn initial_secret(dcid: &[u8]) -> Result<Vec<u8>, String> {
    // per spec: initial_secret = HKDF-Extract(salt, client_dst_connection_id)
    let hk = Hkdf::<Sha256>::new(Some(INITIAL_SALT_V1), dcid);
    let mut prk = [0u8; 32];
    // expand with empty info to get 32 bytes
    hk.expand(&[], &mut prk).map_err(|_| "hkdf extract expand failed".to_string())?;
    Ok(prk.to_vec())
}

// derive key/iv using HKDF-Expand-Label syntax
fn hkdf_expand_label(secret: &[u8], label: &str, out_len: usize) -> Result<Vec<u8>, String> {
    // info = length || "tls13 " + label || 0
    let mut info: Vec<u8> = Vec::new();
    info.push(((out_len >> 8) & 0xff) as u8);
    info.push((out_len & 0xff) as u8);
    let full_label = format!("tls13 {}", label);
    info.push(full_label.len() as u8);
    info.extend_from_slice(full_label.as_bytes());
    info.push(0u8);
    hkdf_extract_expand(&[], secret, &info, out_len)
}

// High-level function: given DCID and CRYPTO ciphertext+aad+nonce info, try to decrypt using derived key
// For simplification, we assume AEAD AES-128-GCM.

pub fn decrypt_initial_crypto(dcid: &[u8], aead_key_len: usize, key_info: &[u8], ciphertext: &[u8], nonce: &[u8], aad: &[u8]) -> Result<Vec<u8>, String> {
    // Derive initial secret (PRK)
    let prk = initial_secret(dcid)?;
    // Derive key using hkdf_expand_label
    let key = hkdf_expand_label(&prk, "quic key", aead_key_len)?;
    let iv = hkdf_expand_label(&prk, "quic iv", 12)?; // 96-bit nonce

    // XOR nonce with iv to produce actual AEAD nonce -> here we receive nonce from packet header and XOR
    if nonce.len() != iv.len() { return Err("nonce/iv len mismatch".into()); }
    let mut real_nonce = vec![0u8; iv.len()];
    for i in 0..iv.len() { real_nonce[i] = iv[i] ^ nonce[i]; }

    match aead_key_len {
        16 => {
            let cipher = Aes128Gcm::new_from_slice(&key).map_err(|e| format!("cipher init: {:?}", e))?;
            let nonce = Nonce::from_slice(&real_nonce);
            let pt = cipher.decrypt(nonce, aes_gcm::aead::Payload { msg: ciphertext, aad }).map_err(|_| "decrypt failed".to_string())?;
            Ok(pt)
        }
        32 => {
            let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| format!("cipher init: {:?}", e))?;
            let nonce = Nonce::from_slice(&real_nonce);
            let pt = cipher.decrypt(nonce, aes_gcm::aead::Payload { msg: ciphertext, aad }).map_err(|_| "decrypt failed".to_string())?;
            Ok(pt)
        }
        _ => Err("unsupported key length".into())
    }
}
