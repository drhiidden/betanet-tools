
# Betanet Tools

Collection of utilities and crates for the Betanet ecosystem. This repository contains components for TLS/HTX experimentation, uTLS templates, and tooling to analyse and reproduce client fingerprints.

**License**: core is dual-licensed `MIT OR Apache-2.0`. See `LICENSE-MIT` and `LICENSE-APACHE`.

Before contributing, please read `CONTRIBUTING.md` and `CODE_OF_CONDUCT.md`.

## Overview

Key crates (open the crate README for details):

- `crates/bo-core` — core types and KDF utilities used across the runtime. See `crates/bo-core/README.md`.
- `crates/bo-tls` — rustls wrapper and outer-transport POC that extracts exporter bytes. See `crates/bo-tls/README.md`.
- `crates/bo-aead` — AEAD helpers (ChaCha20-Poly1305) and nonce logic. See `crates/bo-aead/README.md`.
- `crates/bo-htx` — HTX framing, streams and `Connection` API. See `crates/bo-htx/README.md`.
- `crates/utls-template` — uTLS template generator, snapshot capture and CLI tools. See `crates/utls-template/README.md`.

Other tools and experimental crates live under `crates/` (e.g. `bn-lint`, `htx-lab`, `sbom`).

## Quickstart

- Build the whole workspace: `cargo build --release`
- Build a single crate: `cargo build -p <crate-name>`
- Run the uTLS CLI: `cargo run -p utls-cli -- --help`

## Functional summary

This repo provides:

- TLS outer-handshake proof-of-concepts and exporters (interop reference with `libbetanet-cpp`).
- HTX runtime skeleton (key derivation, AEAD helpers and framing stubs) aimed to be a Rust-native runtime for inner transport.
- uTLS template tooling: capture and reproduce ClientHello fingerprints and generate test vectors.

## Raven Team & Community

This project is maintained by the Raven team and contributors. For community discussion and support:

- [Reddit presentation thread](https://www.reddit.com/r/rust/comments/1mlfnxz/betanet_a_new_hope_to_defeat_mass_internet/)

Replace the placeholders above with the canonical links. If you want, I can add the real links now.

## Project policies

- Toolchain: this repo includes `rust-toolchain.toml` set to `stable` and provides `rustfmt` and `clippy` in CI.
- Workspace: use `cargo build -p <crate>` to speed development. Tests live per-crate.
- Security: do not commit PCAPs containing PII — use sanitized fixtures under `crates/utls-template/fixtures/`.

## Where to start

- If you want to implement or extend the runtime, start in `crates/bo-core` and `crates/bo-htx`.
- If your goal is fingerprint generation or test vectors, use `crates/utls-template` and its CLI.

Thanks for contributing!
