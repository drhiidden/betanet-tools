bo-core
=======

Core crate containing common types and utilities for the HTX runtime.

Quickstart
- Build: `cargo build -p bo-core`
- Test: `cargo test -p bo-core`

Public API
- `kdf::hkdf_expand_label(secret, label, context, length)` — HKDF-TLS expand-label implementation.
- `transport::OuterTransport` trait — abstraction for dialing/accepting outer transports.

Notes
- This crate should avoid depending on high-level crates to prevent dependency cycles. Keep the public API minimal.


