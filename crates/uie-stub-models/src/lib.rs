//! Blank / deterministic [InferenceExecutor] stubs for UIE modalities (US-010–019, US-032).

mod blanks;
mod conventions;
mod executor;
mod filter;
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
pub use filter::{
    apply_filters_nchw, expect_filter_json, parse_filter_json, sample_filter_json, FilterOp,
    FilterRequest,
};
pub use infer_blank::infer_blank;
pub use loader::StubModelLoader;
pub use modality::ModalityKind;
pub use policy::{
    infer_with_policy, BlankPolicy, RuleStubPolicy, StubMatcher, StubPolicy, StubRule,
};
pub use samples::sample_input;
