use thiserror::Error;

#[derive(Debug, Error)]
pub enum IvError {
    #[error("dependency not bound: {0}")]
    Unbound(&'static str),
    #[error("duplicate binding: {0}")]
    Duplicate(&'static str),
}