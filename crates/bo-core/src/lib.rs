// bo-core: tipos y traits centrales para HTX runtime
pub mod prelude {
    pub use bytes::Bytes;
    pub use thiserror::Error;
}

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("io error: {0}")]
    Io(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;

pub mod kdf {
    use hkdf::Hkdf;
    use sha2::Sha256;

    /// Implementación de HKDF-Expand-Label (TLS1.3 style)
    pub fn hkdf_expand_label(secret: &[u8], label: &str, context: &[u8], length: usize) -> Vec<u8> {
        let full = format!("tls13 {}", label);
        let mut info: Vec<u8> = Vec::new();
        let l_be = (length as u16).to_be_bytes();
        info.extend_from_slice(&l_be);
        info.push(full.len() as u8);
        info.extend_from_slice(full.as_bytes());
        info.push(context.len() as u8);
        info.extend_from_slice(context);

        let hk = Hkdf::<Sha256>::from_prk(secret).expect("invalid prk");
        let mut okm = vec![0u8; length];
        hk.expand(&info, &mut okm).expect("hkdf expand failed");
        okm
    }

    /// Deriva k0 (64 bytes) y split usando la misma convención del C++
    pub fn derive_k0_and_split(tls_exporter_32: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
        let k0 = hkdf_expand_label(tls_exporter_32, "htx inner v1", &[], 64);
        let k0c = k0[0..32].to_vec();
        let k0s = k0[32..64].to_vec();
        let nsc = hkdf_expand_label(&k0c, "ns", &[], 12);
        let nss = hkdf_expand_label(&k0s, "ns", &[], 12);
        (k0c, k0s, nsc, nss)
    }
}

pub mod transport {
    use super::Result;
    use async_trait::async_trait;
    use tokio::io::{AsyncRead, AsyncWrite};

    pub struct DialOptions {
        pub host: String,
        pub port: u16,
    }

    #[async_trait]
    pub trait AsyncStream: AsyncRead + AsyncWrite + Unpin {}
    impl<T: AsyncRead + AsyncWrite + Unpin> AsyncStream for T {}

    #[async_trait]
    pub trait OuterTransport: Send + Sync {
        async fn dial(&self, opts: DialOptions) -> Result<(Vec<u8>, Box<dyn AsyncStream + Send>)>;
        async fn accept(&self, port: u16) -> Result<(Vec<u8>, Box<dyn AsyncStream + Send>)>;
    }
}


