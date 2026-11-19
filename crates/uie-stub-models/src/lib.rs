//! Blank [InferenceExecutor] implementations for US-010 – US-019 modalities.

mod blanks;
mod conventions;
mod executor;
mod loader;
mod modality;
mod samples;
mod validate;

pub use conventions::*;

pub use executor::StubExecutor;
pub use loader::StubModelLoader;
pub use modality::ModalityKind;
pub use samples::sample_input;
