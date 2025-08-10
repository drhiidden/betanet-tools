# Estructura y convención para `crates/`

Este directorio agrupa los _crates_ Rust del monorepo `betanet-tools`. Cada subdirectorio aquí es (o debe ser) un crate independiente con su propio `Cargo.toml` y código en `src/`.

- **`utls-template/`**: generador principal (Chrome‑Stable uTLS). Contiene la implementación principal y componentes relacionados: CLI, captura de snapshots, encoder de plantilla y exportadores. Ejemplo de contenidos internos:
  - `src/` — código del crate (o subcrates si se desea submodularizar)
  - `cli/` — binario CLI (opcional)
  - `tests/`, `templates/`, `goldens/` — pruebas y artefactos dorados

- **`bn-lint/`**: linter CLI para comprobar cumplimiento de la especificación (spec‑compliance).

- **`htx-lab/`**: prototipos HTX (Noise XK, AEAD, framing). Aquí van fuzzers y tests específicos (`fuzz/`, `tests/`).

- **`sbom/`**: utilidades para generar/validar SBOMs (por ejemplo, wrappers para `syft`, generación CycloneDX/SPDX).

Convenciones y comandos rápidos

- Workspace: mantener un `Cargo.toml` en la raíz del repo que liste los miembros del workspace. Ejemplo para añadir un nuevo crate:
  1. Crear carpeta `crates/<mi-crate>/`
  2. Añadir `Cargo.toml` y `src/lib.rs` (o `src/main.rs` para binarios).
  3. Añadir `
