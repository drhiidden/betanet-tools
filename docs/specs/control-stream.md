# Control Stream (CBOR)

This document describes the CBOR schema and security rules for the gateway control stream required by spec 1.1 L1-4.2.

Schema
-------
The reference schema is `docs/specs/control-stream.cbor.schema` (JSON Schema format for readability).

Key fields
-----------
- `version`: schema version.
- `nonce`: unique value per message to prevent replays.
- `ts`: monotonic timestamp in ms to tolerate reordering; use together with `nonce` for security.
- `mac`: message authentication code computed over critical fields (e.g., HMAC-SHA256 over `version||nonce||ts||payload`).
- `rate_limit`: token-bucket parameters exposed for traffic control: `rate` and `burst`.

Replay safety
-------------
- Verify `nonce` and `ts`. Reject messages whose `ts` is outside the configured window or whose `nonce` has already been used.
- Alternative: include a monotonic signed counter if topology allows.

Rate limiting
-------------
- The node applies a token-bucket according to `rate_limit`. The control stream only suggests parameters; local implementation may apply stricter limits.

Tests and validators
--------------------
- Provide positive and negative test vectors with CBOR blobs and metadata (timing, nonce reuse) to validate parsers.
- `bn-lint` should include a rule that validates CBOR conformance to this schema and detects simple replay patterns.

Example usage
-------------
- `crates/htx-lab` contains example generators and parsers for control stream messages (placeholders). Use `htx-lab` to create test vectors for `bn-lint`.
