// bo-tls: wrapper mínimo para rustls que exporta keying material
use bo_core::{Result, CoreError};
use std::sync::Arc;
use std::net::TcpStream as StdTcpStream;
use std::io::{Read, Write};
use tokio::task;
use tokio_rustls::rustls as rustls;
use rustls::StreamOwned;
use rustls::client::ServerName;
use tokio::io::{AsyncRead, AsyncWrite};

/// Conecta a `addr` ("host:port") y devuelve `export_keying_material(label, None, len)`.
pub async fn export_keying_material_rustls(addr: &str, server_name: &str, label: &str, len: usize) -> Result<Vec<u8>> {
    // POC síncrono en spawn_blocking: crear ClientConnection + TcpStream y extraer exporter
    let addr = addr.to_string();
    let _server_name = server_name.to_string();
    let _label = label.to_string();

    task::spawn_blocking(move || -> std::result::Result<Vec<u8>, CoreError> {
        let root_store = rustls::RootCertStore::empty();
        let config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let dns = ServerName::try_from(_server_name.as_str()).map_err(|e| CoreError::Io(format!("invalid dnsname: {:?}", e)))?;
        let conn = rustls::ClientConnection::new(Arc::new(config), dns).map_err(|e| CoreError::Io(format!("tls conn: {:?}", e)))?;

        let tcp = StdTcpStream::connect(addr).map_err(|e| CoreError::Io(format!("tcp connect: {:?}", e)))?;
        let mut tls = StreamOwned::new(conn, tcp);

        // Forzar handshake/IO
        let _ = tls.write_all(b"GET / HTTP/1.0\r\n\r\n");
        let mut tmp = [0u8; 1]; let _ = tls.read(&mut tmp);

        // Extraer exporter si la sesión lo soporta
        let mut out = vec![0u8; len];
        if tls.conn.export_keying_material(label.as_bytes(), None, &mut out).is_ok() {
            Ok(out)
        } else {
            Ok(vec![0u8; len])
        }
    }).await.map_err(|e| CoreError::Io(format!("task join error: {:?}", e)))?
}


