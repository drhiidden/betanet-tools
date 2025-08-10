# htx-lab

Propósito

- Laboratorio para prototipos HTX: diseño de frames, especificación Noise XK, AEAD, y pruebas de interoperabilidad.
- Contiene objetivos de fuzzing y tests de integración.

Estructura

- `src/` — implementaciones y utilidades
- `fuzz/` — targets de `cargo-fuzz` para `framing`, `parser`, `state`
- `tests/` — tests de integración y golden vectors

Comandos

- Compilar: `cargo build -p htx-lab`
- Tests: `cargo test -p htx-lab`
- Ejecutar fuzz (local): `cargo fuzz run framing`

Notas

- Mantener fuzz targets pequeños y bien documentados; añadir CI específico para fuzz en paralelo si es crítico.
