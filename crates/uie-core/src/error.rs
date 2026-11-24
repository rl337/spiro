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

    /// Model admission refused by a [memory budget](crate::resource::MemoryBudget) (US-025).
    #[error(
        "admission rejected: need {estimate_bytes} bytes, remaining {remaining_bytes} ({budget_source}): {reason}"
    )]
    AdmissionRejected {
        estimate_bytes: u64,
        remaining_bytes: u64,
        budget_source: String,
        reason: String,
    },

    /// Context / KV state refused by an [inference state budget](crate::resource::InferenceStateBudget) (US-027).
    #[error(
        "state budget rejected: prompt {prompt_tokens} + new {max_new_tokens} exceeds limit {limit_tokens}: {reason}"
    )]
    StateBudgetRejected {
        prompt_tokens: u32,
        max_new_tokens: u32,
        limit_tokens: u32,
        reason: String,
    },
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
