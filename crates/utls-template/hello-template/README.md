# hello-template

Propósito

- Implementa el DSL y el encoder para generar `ClientHello` binarios reproducibles a partir de snapshots (Chrome N-2).
- Provee la lógica para GREASE, orden de extensiones, ALPS, ECH, PSK modes y la exportación a formatos compatibles con uTLS (Go/JSON).

Artefactos y rutas relevantes

- Plantillas: `templates/chrome-<ver>-n-2/`
- Golden tests: `tests/goldens/`

Comandos rápidos

- Compilar: `cargo build -p hello-template`
- Tests: `cargo test -p hello-template`
- Generar ClientHello (CLI futuro): `cargo run -p utls-cli -- gen --template ...`

Notas

- Mantener la API pública (`pub`) limitada y documentada. Añadir tests unitarios y golden tests por versión.
