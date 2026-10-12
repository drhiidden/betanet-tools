# L1 Design: HTX Tunnel for SCION/HTX Routing

Status: draft

Objective
--------
Define a minimal wireformat and tunnel behavior for HTX to support SCION routes and non-SCION segments (encapsulation over TCP/QUIC-443).

Acceptance criteria
-------------------
- `docs/l1-htx.md` with wireformat, control messages, and examples.
- Mock implementation in `crates/htx-lab::tunnel_mock` that encapsulates/decapsulates frames.
- Unit tests for encapsulation/decapsulation and timeouts.

Minimal wireformat
------------------
- Tunnel header (fixed):
  - `magic` (4 bytes)
  - `version` (1 byte)
  - `flags` (1 byte)
  - `length` (2 bytes, payload length)
- Payload: concatenated HTX frames

Control messages
----------------
- `PING` / `PONG` for keepalive
- `CLOSE` with code and reason
- `MTU_ADJUST` for negotiation

Timeouts
--------
- Keepalive interval: configurable (e.g., 15s)
- Idle timeout: configurable (e.g., 120s)

Examples
--------
- Pseudocode for encapsulating an HTX frame within a TCP/QUIC tunnel.

TODO
----
- Add binary examples and tests in `crates/htx-lab/tests`.
