# sbom

Propósito

- Utilidades para generar y validar SBOMs (CycloneDX / SPDX) y wrappers para herramientas como `syft`.
- Produce artefactos SBOM para releases y CI (SLSA-related metadata).

Comandos

- Compilar: `cargo build -p sbom`
- Generar SBOM (ejemplo wrapper): `cargo run -p sbom -- generate --target ./crates/utls-template` 

Notas

- Incluir validación de SBOM en pipelines de release y almacenar artefactos en releases.
