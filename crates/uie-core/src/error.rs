use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid shape: {0}")]
    InvalidShape(String),

    #[error("dtype mismatch: expected {expected:?}, got {got:?}")]
    DtypeMismatch {
        expected: super::tensor::DataType,
        got: super::tensor::DataType,
    },

    #[error("backend error ({backend}): {message}")]
    Backend {
        backend: super::backend::BackendId,
        message: String,
    },

    #[error("model load failed: {0}")]
    ModelLoad(String),

    #[error("unsupported operation: {0}")]
    UnsupportedOp(String),

    #[error("runtime: {0}")]
    Runtime(String),

    #[error("io: {0}")]
    Io(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn backend<B: fmt::Display>(backend: B, message: impl Into<String>) -> Self {
        Self::Backend {
            backend: super::backend::BackendId::new(backend.to_string()),
            message: message.into(),
        }
    }
}
