# BOUNTY — Proceso y términos básicos

Este documento detalla el flujo y reglas mínimas para proponer y aceptar *bounties* sobre este repositorio.

Principios

- El código público en este repo se publica bajo **MIT OR Apache‑2.0** (ver `LICENSE-*`).
- Los bounties (trabajos pagados) son acuerdos adicionales y se rigen por un contrato/CLA específico que se negociará antes del pago.
- Transparencia: todo bounty debe abrirse como *issue público* con etiqueta `bounty` y un alcance claro.

Flujo recomendado para un bounty

1. **Abrir issue**: crear un issue en `https://github.com/drhiidden/betanet-tools/issues` con:
   - Título: `bounty: breve-descripción`
   - Descripción: alcance, criterios de aceptación (Given/When/Then), entregables y timeline.
   - Presupuesto (USD) y método de pago (invoice/contract)

2. **Discusión y aceptación**: confirmar alcance y condiciones en el issue. Hasta aquí el trabajo es exploratorio.

3. **CLA / Contrato**: antes de empezar el trabajo pagado, firmar un acuerdo (plantilla en `docs/communication/CLA.md`). El acuerdo especificará licencia comercial otorgada a la parte pagadora (no necesariamente cesión de copyright).

4. **Desarrollo**: trabajar en rama con commits firmados y abrir PRs vinculadas al issue. Marcar en PR si el trabajo forma parte del bounty.

5. **Entrega y validación**: validar contra criterios de aceptación; reproducir tests y selftests (JA3/JA4) en la matriz de CI si aplica.

6. **Pago**: tras aceptación final, emitir factura/ejecutar contrato y procesar pago según lo acordado.

Notas legales y prácticas

- **Propiedad intelectual**: por defecto el contribuyente conserva el copyright; el contrato/CLA puede otorgar a la parte pagadora una licencia amplia (no exclusiva o exclusiva por tiempo limitado) para uso comercial.
- **PCAPs y datos**: no subir PCAPs con PII. Si el bounty requiere muestras, deben ser anonimizado/sanitizados antes de publicarse.
- **Transparencia**: todos los contratos asociados a bounties deben ser referenciados en el issue (sin publicar detalles privados) para auditoría.

Contacto y seguimiento

- Issues: `https://github.com/drhiidden/betanet-tools/issues` (use la etiqueta `bounty`).
- Para cualquier duda legal/procedimental, abrir un issue con la etiqueta `meta`.
