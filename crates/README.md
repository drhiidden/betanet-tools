# Estructura y convención para `crates/`

Este directorio agrupa los _crates_ Rust del monorepo `betanet-tools`. Cada subdirectorio aquí es (o debe ser) un crate independiente con su propio `Cargo.toml` y código en `src/`.

- **`utls-template/`**: generador principal (Chrome‑Stable uTLS). Contiene la implementación principal y componentes relacionados: CLI, captura de snapshots, encoder de plantilla y exportadores. Ejemplo de contenidos internos:
  - `hello-template/` — DSL y encoder
  - `hello-snapshot/` — captura y parsers
  - `chrome-probe/` — runner de Chrome for Testing
  - `cli/` — binario integrador (opcional)

- **`bn-lint/`**: linter CLI para comprobar cumplimiento de la especificación (spec‑compliance).

- **`htx-lab/`**: prototipos HTX (Noise XK, AEAD, framing). Aquí van fuzzers y tests específicos (`fuzz/`, `tests/`).

- **`sbom/`**: utilidades para generar/validar SBOMs (por ejemplo, wrappers para `syft`, generación CycloneDX/SPDX).

Convenciones y comandos rápidos

- Workspace: mantener un `Cargo.toml` en la raíz del repo que liste los miembros del workspace. Ejemplo para añadir un nuevo crate:
  1. Crear carpeta `crates/<mi-crate>/`
  2. Añadir `Cargo.toml` y `src/lib.rs` (o `src/main.rs` para binarios).
  3. Añadir el path `"crates/<mi-crate>"` en `workspace.members` del `Cargo.toml` de la raíz.

- Comandos habituales (desde la raíz del repo):
  - Compilar un crate: `cargo build -p <crate-name>`
  - Ejecutar tests de un crate: `cargo test -p <crate-name>`
  - Ejecutar binario/CLI: `cargo run -p <crate-name> -- <args>`
  - Ejecutar todos los tests del workspace: `cargo test`

- Golden tests / reproducibilidad:
  - Guarda *goldens* por versión en `crates/utls-template/templates/chrome-<ver>-n-2/` o en `crates/utls-template/goldens/`.
  - Documenta la `versions.lock.json` y las flags de `Chrome for Testing` usadas para capturar snapshots.

- PCAP / depuración:
  - Si un crate emite PCAP/PCAPNG, añadir el artefacto en `crates/<crate>/artifacts/pcap/` y un test que valide que el PCAP puede abrirse en Wireshark (smoke test).

Buenas prácticas para nuevos crates

- Escribe un `README.md` mínimo en el nuevo crate explicando su propósito y cómo ejecutarlo.
- Añade `CI` y `lint` (github actions) si el crate aporta funcionalidad crítica.
- Mantén las interfaces públicas bien documentadas (`pub` API) y añade tests unitarios y E2E si aplica.

Dónde poner cosas relacionadas con `utls-template`

- Lógica de parsing/capture: `crates/utls-template/hello-snapshot/`
- DSL & encoder: `crates/utls-template/hello-template/`
- Chrome runner / probe: `crates/utls-template/chrome-probe/`
- CLI integrador: `crates/utls-template/cli/`

Estrategia recomendada

- Recomiendo la **Opción B (multi-crate)**: mantener `hello-template`, `hello-snapshot`, `chrome-probe`, `utls-cli` como crates separados.
  - Ventajas: builds paralelos, CI granular, límites de responsabilidad claros.
  - Desventajas: más `Cargo.toml` y sincronización entre crates (resolver con workspace y documentación).

---

Si quieres, puedo añadir plantillas de `README.md` con checklist y GWT (Given/When/Then) para las historias críticas (capture, gen, selftest, export).
