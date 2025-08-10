# hello-snapshot

Propósito

- Captura `ClientHello` y QUIC Initial (CRYPTO Initial) desde Chrome for Testing y servidores de prueba.
- Produce snapshots estructurados: `snapshot_tls.json`, `snapshot_quic.json`, `snapshot_h2.json`, `snapshot_h3.json`.
- Soporta opciones: `--allow-hrr`, `--emit-pcap`.

Artefactos

- Snapshots: `artifacts/snapshots/`
- PCAPs: `artifacts/pcap/`

Comandos

- Compilar: `cargo build -p hello-snapshot`
- Tests: `cargo test -p hello-snapshot`
- Capturar (ejemplo manual): `cargo run -p hello-snapshot -- capture --allow-hrr=false --emit-pcap --out ./artifacts/` 

Notas

- Usa flags deterministas al lanzar Chrome en `chrome-probe` para reproducibilidad.
