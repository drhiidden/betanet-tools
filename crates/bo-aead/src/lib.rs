// bo-aead: wrappers mínimos para AEAD
use bo_core::prelude::*;
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::XChaCha20Poly1305;

pub fn aead_encrypt_example(_key: &[u8], _nonce: &[u8], _plaintext: &[u8]) -> Result<Vec<u8>> {
    // Placeholder: implementar en detalle
    Ok(vec![])
}


