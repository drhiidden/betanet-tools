# Design: L4 Mixnet (Nym-style) Integration and Node Requirements

Status: draft (skeleton)

Objective
--------
Design an optional integration of a Nym-style mixnet as an L4 layer for HTX. This document will cover the API between the HTX overlay and mixnodes, node throughput requirements, route selection, VRF/beacon mechanisms, and anti-congestion rules.

Acceptance criteria
-------------------
- `docs/l4-mixnet.md` describing: node API, VRF selection algorithm, beacon sources, expected latency/throughput, and anti-congestion rules.
- Prototype in `crates/htx-lab::mixnet_sim` that simulates path selection and collects latency samples.
- Initial placeholder lint rules in `bn-lint` for node configuration limits.

Proposed sections
-----------------
1. Introduction and scope (optional L4 layer)
2. Node requirements (CPU, memory, throughput, concurrent connections)
3. API between HTX overlay and mixnode
   - Endpoints/frames for sending/receiving packets
   - Required metadata (e.g., route_id, expiry, anonymity_set_id)
4. Route selection algorithm
   - VRF-based selection (proposed scheme)
   - Beacon sources (NTP, blockchain oracle, other oracles)
5. Expected latency and throughput
   - SLAs per hop and per path
6. Anti-congestion rules
   - Rate limiting, backoff and retry strategies
7. Instrumentation and metrics
8. Prototyping plan
   - `crates/htx-lab::mixnet_sim` (toy) and metrics to collect

TODO
----
- Add VRF selection pseudocode
- Add API and message examples
- Implement prototype in `crates/htx-lab`
