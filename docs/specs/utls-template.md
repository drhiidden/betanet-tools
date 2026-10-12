# utls-template — Specification & Requirements

Purpose
-------
This document specifies what the `utls-template` collection must provide so it is a stable, testable foundation for ClientHello/Client fingerprint tooling used by the project.

Scope
-----
- `crates/utls-template/*`: encoder, snapshot importer, CLI, and helper utilities for ClientHello, JA3/JA4, and PCAP handling.
- Related fixtures and goldens used for reproducible tests.

Acceptance Criteria
-------------------
- Public API documented (how to encode a template, how to import a pcap, CLI usage examples).
- Unit tests covering: encoder correctness, ClientHello structure, snapshot import (TCP+QUIC heuristics), and the `hello-template` example test.
- CI runs: `cargo build --all-features`, `cargo test`, `cargo clippy -D warnings`, `cargo audit`.
- Goldens: at least one PCAP and corresponding snapshot used as a golden test.

API surface (minimum)
---------------------
- Library functions:
  - `Encoder::encode_client_hello(template: &HelloTemplate, emit_pcap: bool) -> Result<EncodedClientHello>`
  - `import_from_pcap(path: &str) -> Result<TlsSnapshot>`
  - `snapshot_to_ja3(snap: &mut TlsSnapshot)`
- CLI commands:
  - `utls-cli gen --template <file> --out <file> [--emit-pcap]`
  - `utls-cli selftest --pcap <file> --template <file>`

File layout expectations
------------------------
- `crates/utls-template/hello-template/*` — encoding logic and template formats.
- `crates/utls-template/hello-snapshot/*` — import and JA3/JA4 helpers.
- `crates/utls-template/cli` — CLI entrypoint.
- `crates/utls-template/fixtures/goldens/` — example PCAPs and snapshots.

Tests
-----
- Unit tests for each crate in `crates/utls-template`.
- Integration tests: the CLI `selftest` should run against at least one golden pcap.
- Provide small test vectors that can run quickly in CI; larger goldens may be in an artifacts bucket or separate workflow.

CI Requirements
---------------
- `ubuntu-latest` runner with `rustup` toolchain; matrix may add `windows` and `macos` later.
- Steps: checkout, cache cargo, `cargo build`, `cargo test`, `cargo clippy -D warnings`, `cargo audit`.
- Optional: add `cargo tarpaulin` for coverage calculation on Linux.

Examples & Usage
----------------
- Add a short `examples/` entry showing how to create a `HelloTemplate`, call the encoder and write the pcap output.

Security & Privacy Notes
------------------------
- PCAP goldens must be sanitized (no PII, no secrets). Include a sanitizer script if large goldens are collected from live captures.
- Document that importing arbitrary PCAPs may include untrusted data; parsers must be defensive.

Contribution guidelines
-----------------------
- New features must include tests and documentation; small PRs preferred.
- Use the `runtime-rust-v0.1` milestone for runtime-related tasks.