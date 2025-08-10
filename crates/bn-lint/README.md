# bn-lint

Propósito

- CLI para verificar el cumplimiento de implementaciones con la especificación Betanet/utls-template.
- Ejecuta reglas que comprueban: orden de extensiones, presencia de campos obligatorios, límites de GREASE, y políticas de templates.

Comandos

- Compilar: `cargo build -p bn-lint`
- Ejecutar linter: `cargo run -p bn-lint -- lint --input snapshot_tls.json`

Notas

- Los fallos del linter deben producir códigos de salida claros y mensajes legibles para CI.
