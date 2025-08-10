// bo-tls: wrapper mínimo para rustls que exporta keying material
use bo_core::prelude::*;
use std::sync::Arc;
use std::net::TcpStream;
use std::io::{Read, Write};

use rustls::client::{ServerCertVerified, ServerCertVerifier, ServerName};
use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use tokio::net::TcpStream as TokioTcpStream;
use tokio_rustls::TlsConnector;
use tokio_rustls::rustls::client::HandshakeSignatureValid;
use tokio::io::{AsyncRead, AsyncWrite};
use std::pin::Pin;
use std::task::{Context, Poll};

struct NoCertificateVerification;

impl ServerCertVerifier for NoCertificateVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }
}

/// Conecta a `addr` ("host:port") y devuelve `export_keying_material(label, None, len)`.
/// Nota: esta función omite la verificación de certificado (POC solamente).
pub async fn export_keying_material_rustls(addr: &str, server_name: &str, label: &str, len: usize) -> Result<Vec<u8>> {
    let addr = addr.to_string();
    let server_name = server_name.to_string();
    let label = label.to_string();

    tokio::task::spawn_blocking(move || {
        // Crear configuración TLS sin verificación para POC
        let mut root_store = RootCertStore::empty();
        let config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_custom_certificate_verifier(Arc::new(NoCertificateVerification))
            .with_no_client_auth();

        let server_name = ServerName::try_from(server_name.as_str())
            .map_err(|e| CoreError::Io(format!("invalid server name: {:?}", e)))?;

        let mut conn = ClientConnection::new(Arc::new(config), server_name)
            .map_err(|e| CoreError::Io(format!("tls conn: {:?}", e)))?;

        // Usar tokio + tokio-rustls para la versión async
        let rt_tcp = TokioTcpStream::connect(addr).map_err(|e| CoreError::Io(format!("tcp connect: {:?}", e)))?;
        let connector = TlsConnector::from(Arc::new(config));
        let dnsname = ServerName::try_from(server_name.as_str()).map_err(|e| CoreError::Io(format!("invalid dnsname: {:?}", e)))?;
        let mut tls_stream = connector.connect(dnsname.clone(), rt_tcp).await.map_err(|e| CoreError::Io(format!("tls connect: {:?}", e)))?;

        // Forzar handshake ya hecho por connect; ahora extraer exporter desde la conexión interna
        // tokio_rustls::client::TlsStream wraps an inner rustls::ClientConnection we can access
        let conn_ref = tls_stream.get_ref().1;
        let mut out = vec![0u8; len];
        conn_ref.export_keying_material(label.as_bytes(), None, &mut out).map_err(|e| CoreError::Io(format!("exporter err: {:?}", e)))?;
        Ok(out)
    })
    .await
    .map_err(|e| CoreError::Io(format!("task join error: {:?}", e)))?
}


