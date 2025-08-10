# chrome-probe

Propósito

- Lanzador y controlador para Chrome for Testing usado por `hello-snapshot`.
- Aplica flags deterministas (desactivar field trials, forzar QUIC en localhost, etc.) y gestiona perfil temporal.

Uso

- Compilar: `cargo build -p chrome-probe`
- Ejemplo (lib): `chrome_probe::launch_chrome(9222, &["--no-first-run","--origin-to-force-quic-on=127.0.0.1:443"])`

Notas

- Documentar las flags exactas y la versión de `Chrome for Testing` en `versions.lock.json`.
- Asegurar cleanup del `user-data-dir` temporal tras la captura.
