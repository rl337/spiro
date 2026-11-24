//! Universal Inference Engine (UIE) — core abstractions.
//!
//! This crate defines a small, backend-agnostic stack for loading models,
//! representing tensors, scheduling execution, and dispatching work to
//! pluggable compute backends.

pub mod backend;
pub mod capability;
pub mod error;
pub mod infer;
pub mod model;
pub mod ops;
pub mod resource;
pub mod runtime;
pub mod tensor;

pub mod telemetry {
    pub use uie_telemetry::*;
}

pub use backend::cpu::CpuBackend;
pub use backend::{Backend, BackendId, Device, DeviceKind};
pub use capability::{
    fill_f32_dispatch, fill_f32_scalar, require_feature, select_kernel, select_kernel_traced,
    CapabilityEvent, CapabilityProbe, CpuFeature, DispatchDecision, FakeCapabilityProbe,
    KernelPath,
};
pub use error::{Error, Result};
pub use infer::InferOutcome;
pub use model::{
    InferenceExecutor, InferenceModel, IoSpec, ModelLoader, ModelLoaderRegistry, ModelMetadata,
};
pub use ops::{Op, OpKind};
pub use resource::{
    resolve_context_n_ctx, AdmissionEvent, ContextRequest, EstimateConfidence,
    FakeInferenceStateBudget, FakeMemoryBudget, InferenceStateBudget, MemoryBudget, MemoryEstimate,
    Reservation, StateAdmission, StateEvent, UnlimitedInferenceStateBudget, UnlimitedMemoryBudget,
};
pub use runtime::{ExecutionPlan, InferenceRuntime, Session, SessionConfig};
pub use tensor::{DataType, Shape, Tensor, TensorView};
