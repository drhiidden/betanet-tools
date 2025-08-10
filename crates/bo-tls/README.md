
bo-tls
======

Rustls wrapper to obtain `export_keying_material` and provide an `OuterTransport` implementation.

Quickstart
- Build: `cargo build -p bo-tls`
- Example: call `export_keying_material_rustls("example.com:443", "example.com", "EXPORTER-betanet", 32)`

Notes
- The current POC may skip certificate verification in some branches; replace with `rustls-native-certs` or a proper CA store for production use.


