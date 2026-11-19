use std::path::{Path, PathBuf};

use crate::backend::Backend;
use crate::error::{Error, Result};
use crate::ops::Op;
use crate::runtime::ExecutionPlan;
use crate::tensor::{DataType, Shape, Tensor};

#[derive(Debug, Clone, Default)]
pub struct ModelMetadata {
    pub name: String,
    pub format: String,
    pub ops: usize,
}

#[derive(Debug, Clone)]
pub struct IoSpec {
    pub name: String,
    pub shape: Shape,
    pub dtype: DataType,
}

/// End-to-end inference (ONNX, GGUF, etc.).
pub trait InferenceExecutor: Send + Sync {
    fn metadata(&self) -> &ModelMetadata;

    fn inputs(&self) -> &[IoSpec];

    fn outputs(&self) -> &[IoSpec];

    fn infer(&self, inputs: &[Tensor]) -> Result<Vec<Tensor>>;
}

/// Graph / op-plan representation (scaffold path).
pub trait InferenceModel: Send + Sync {
    fn metadata(&self) -> &ModelMetadata;

    fn plan(&self) -> &ExecutionPlan;
}

/// Pluggable model loading (ONNX, GGUF, custom JSON, etc.).
pub trait ModelLoader: Send + Sync {
    fn format_id(&self) -> &str;

    fn can_load(&self, path: &Path) -> bool;

    fn load(&self, path: &Path, backend: &dyn Backend) -> Result<Box<dyn InferenceExecutor>>;
}

/// In-memory model built from an execution plan (scaffold default).
pub struct PlanModel {
    metadata: ModelMetadata,
    plan: ExecutionPlan,
}

impl PlanModel {
    pub fn new(metadata: ModelMetadata, plan: ExecutionPlan) -> Self {
        Self { metadata, plan }
    }
}

impl InferenceModel for PlanModel {
    fn metadata(&self) -> &ModelMetadata {
        &self.metadata
    }

    fn plan(&self) -> &ExecutionPlan {
        &self.plan
    }
}

/// Descriptor for a model on disk (used by loaders and CLI).
#[derive(Debug, Clone)]
pub struct ModelSource {
    pub path: PathBuf,
    pub format_hint: Option<String>,
}

impl ModelSource {
    pub fn from_path(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            format_hint: None,
        }
    }
}

/// Ordered list of ops with named tensor slots.
#[derive(Debug, Clone, Default)]
pub struct Graph {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub ops: Vec<Op>,
}

impl Graph {
    pub fn to_plan(&self) -> ExecutionPlan {
        ExecutionPlan {
            ops: self.ops.clone(),
        }
    }
}

/// Loader registry: pick the first loader that recognizes the path.
#[derive(Default)]
pub struct ModelLoaderRegistry {
    loaders: Vec<Box<dyn ModelLoader>>,
}

impl ModelLoaderRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, loader: Box<dyn ModelLoader>) {
        self.loaders.push(loader);
    }

    pub fn load(&self, path: &Path, backend: &dyn Backend) -> Result<Box<dyn InferenceExecutor>> {
        for loader in &self.loaders {
            if loader.can_load(path) {
                return loader.load(path, backend);
            }
        }
        Err(Error::ModelLoad(format!(
            "no loader registered for {}",
            path.display()
        )))
    }
}
