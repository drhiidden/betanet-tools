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


