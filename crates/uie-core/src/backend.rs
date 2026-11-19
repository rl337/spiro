pub mod cpu;

use std::fmt;
use std::sync::Arc;

use crate::error::Result;
use crate::model::InferenceModel;
use crate::ops::Op;
use crate::tensor::{DataType, Shape, Tensor};

/// Stable identifier for a registered backend implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BackendId(pub Arc<str>);

impl BackendId {
    pub fn new(name: impl Into<Arc<str>>) -> Self {
        Self(name.into())
    }
}

impl fmt::Display for BackendId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceKind {
    Cpu,
    /// Placeholder for CUDA/ROCm/Metal/etc.
    Accelerator,
}

#[derive(Debug, Clone)]
pub struct Device {
    pub kind: DeviceKind,
    pub index: u32,
    pub label: Option<String>,
}

impl Device {
    pub fn cpu() -> Self {
        Self {
            kind: DeviceKind::Cpu,
            index: 0,
            label: Some("host".into()),
        }
    }
}

/// Pluggable execution backend: memory, kernels, and model hydration.
pub trait Backend: Send + Sync {
    fn id(&self) -> BackendId;

    fn device(&self) -> Device;

    fn allocate(&self, dtype: DataType, shape: Shape) -> Result<Tensor>;

    /// Execute a single op, reading/writing tensors by id in `context`.
    fn execute(&self, op: &Op, inputs: &[Tensor], outputs: &mut [Tensor]) -> Result<()>;

    /// Optional fast path: load a serialized model into an in-memory graph.
    fn load_model(
        &self,
        loader: &dyn crate::model::ModelLoader,
    ) -> Result<Box<dyn InferenceModel>> {
        let _ = loader;
        Err(crate::error::Error::UnsupportedOp(
            "load_model not implemented for this backend".into(),
        ))
    }
}
