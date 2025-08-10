
bo-aead
======

AEAD wrappers (ChaCha20-Poly1305) and nonce utilities.

Quickstart
- Build: `cargo build -p bo-aead`

Public API
- `seal_chacha20poly1305(key, ns, counter, aad, plaintext)` — returns ciphertext+tag.
- `open_chacha20poly1305(key, ns, counter, aad, ct_with_tag)` — returns plaintext.


