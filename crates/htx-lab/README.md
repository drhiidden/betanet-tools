# htx-lab

Toy prototypes and experiments for HTX features used by Betanet tools.

Modules included
- `mixnet_sim` — toy simulator for route selection latency.
- `control_stream_examples` — placeholder CBOR generator/parser.
- `tunnel_mock` — simple encapsulation/decapsulation utility.

Run tests
---------

```bash
# Run only htx-lab tests
cargo test -p htx-lab
```

Add fixtures
------------
Large fixtures belong under `crates/htx-lab/fixtures/` with a `README.md` describing origin and sanitisation.
