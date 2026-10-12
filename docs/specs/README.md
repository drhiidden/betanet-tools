# Specs index & running htx-lab smoke tests

This folder contains spec documents and test helpers for HTX-related features.

Files of interest
- `docs/l1-htx.md` — HTX tunnel design (wireformat, control messages).
- `docs/l4-mixnet.md` — L4 mixnet integration design.
- `docs/specs/control-stream.md` — CBOR schema and security notes for gateway control stream.

Running htx-lab tests
---------------------
From the repository root:

```bash
# Run the unit and integration tests for htx-lab
cargo test -p htx-lab
```

Smoke tests are located in `crates/htx-lab/tests/`.

Where to add fixtures
---------------------
Add sanitized PCAPs and other large test artifacts under `crates/htx-lab/fixtures/` and document them in `crates/htx-lab/fixtures/README.md`.
