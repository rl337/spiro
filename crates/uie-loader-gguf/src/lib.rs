//! Load LM Studio–style `.gguf` weights as [US-010](https://github.com) text→text executors.

mod complete;
mod loader;
mod model;

pub use complete::{complete_from_prompt, CompleteOptions, CompleteResult};
pub use loader::GgufModelLoader;
pub use model::{prompt_tensor, GgufLlmExecutor};
