//! Blank [InferenceExecutor] implementations for US-010 – US-019 modalities.

mod blanks;
mod conventions;
mod executor;
mod infer_blank;
mod loader;
mod modality;
mod policy;
mod samples;
mod stub_infer_metrics;
mod stub_input_validate;
pub mod telemetry;
pub mod telemetry_assert;
mod validate;

pub use conventions::*;

pub use executor::StubExecutor;
pub use infer_blank::infer_blank;
pub use loader::StubModelLoader;
pub use modality::ModalityKind;
pub use policy::{
    infer_with_policy, BlankPolicy, RuleStubPolicy, StubMatcher, StubPolicy, StubRule,
};
pub use samples::sample_input;
