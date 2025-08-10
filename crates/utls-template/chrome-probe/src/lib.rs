use std::process::{Command, Child};

pub fn launch_chrome(port: u16, extra_args: &[&str]) -> Result<Child, String> {
    let mut cmd = Command::new("chrome"); // Asume 'chrome' está en el PATH o usa ruta completa
    cmd.arg(format!("--remote-debugging-port={}", port));
    cmd.arg("--headless=new");
    cmd.arg("--user-data-dir=/tmp/chrome_profile"); // Directorio temporal
    cmd.arg("--no-first-run");
    cmd.arg("--no-default-browser-check");
    cmd.arg("--disable-field-trial-config");
    cmd.arg("--disable-features=OptimizationHints,NetworkQualityEstimator");
    cmd.arg("--disable-background-networking");
    cmd.arg("--enable-quic");
    cmd.arg("--origin-to-force-quic-on=127.0.0.1:443");

    // Añadir argumentos extra
    for arg in extra_args {
        cmd.arg(arg);
    }

    match cmd.spawn() {
        Ok(child) => Ok(child),
        Err(e) => Err(format!("Failed to launch Chrome: {}", e)),
    }
}
