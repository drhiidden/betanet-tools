# Betanet Tools

Colección de utilidades y crates para el ecosistema Betanet.

**Licencia**: core is dual-licensed `MIT OR Apache-2.0`. Consulte `LICENSE-MIT` y `LICENSE-APACHE`.

**Contribuciones**: antes de contribuir, lea `CONTRIBUTING.md` y `CODE_OF_CONDUCT.md`.

## Documentación

Toda la documentación está centralizada en `docs/`. Empezad por `docs/index.md` que contiene índice y enlaces a las áreas principales (management, communication, specs, utls-template, spec-linter).

- Índice de docs: `docs/index.md`
- Bounties y CLA: plantillas y guía en `docs/management/BOUNTY.md` y `docs/management/CLA.md`.
  - Si queréis crear un bounty directamente en GitHub, usamos la plantilla de issue: `.github/ISSUE_TEMPLATE/bounty.md` (también disponible en `docs/management/ISSUE_TEMPLATE/bounty.md`).

## Quick start

- Compilar todo el workspace: `cargo build --release`
- Compilar un crate: `cargo build -p <crate-name>`
- Ejecutar el CLI uTLS: `cargo run -p utls-cli -- --help`

## Project layout

- `crates/utls-template/` — generador, snapshot, CLI.
- `crates/bn-lint/` — spec linter (planificado).

## Security & Privacy

- No publicar PCAPs con PII. Use fixtures sanitizados en `crates/utls-template/fixtures/`.
- Para reportes de seguridad, vea `SECURITY.md`.

Gracias por contribuir!
