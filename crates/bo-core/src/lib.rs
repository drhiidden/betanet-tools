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
    use super::*;
    use hkdf::Hkdf;
    use sha2::Sha256;

    /// Implementa HKDF-Expand-Label con la construcción TLS1.3:
    /// info = length(2) || label_len(1) || "tls13 " + label || context_len(1) || context
    pub fn hkdf_expand_label(secret: &[u8], label: &str, context: &[u8], length: usize) -> Result<Vec<u8>> {
        let full_label = format!("tls13 {}", label);
        let mut info: Vec<u8> = Vec::with_capacity(2 + 1 + full_label.len() + 1 + context.len());
        let l = (length as u16).to_be_bytes();
        info.extend_from_slice(&l);
        info.push(full_label.len() as u8);
        info.extend_from_slice(full_label.as_bytes());
        info.push(context.len() as u8);
        info.extend_from_slice(context);

        let hk = Hkdf::<Sha256>::from_prk(secret).map_err(|e| CoreError::Io(format!("hkdf prk: {:?}", e)))?;
        let mut okm = vec![0u8; length];
        hk.expand(&info, &mut okm).map_err(|e| CoreError::Io(format!("hkdf expand: {:?}", e)))?;
        Ok(okm)
    }

    /// Deriva k0 (64 bytes) y split usando la misma convención del C++
    pub fn derive_k0_and_split(tls_exporter_32: &[u8]) -> (Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>) {
        let k0 = hkdf_expand_label(tls_exporter_32, "htx inner v1", &[], 64).expect("derive k0 failed");
        let k0c = k0[0..32].to_vec();
        let k0s = k0[32..64].to_vec();
        let nsc = hkdf_expand_label(&k0c, "ns", &[], 12).expect("derive nsc failed");
        let nss = hkdf_expand_label(&k0s, "ns", &[], 12).expect("derive nss failed");
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}


