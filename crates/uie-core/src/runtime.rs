use std::collections::HashMap;
use std::path::Path;

use tracing::{debug, instrument};

use crate::backend::{Backend, Device};
use crate::error::{Error, Result};
use crate::model::{InferenceExecutor, InferenceModel, ModelLoader, ModelLoaderRegistry};
use crate::ops::{Op, OpKind};
use crate::tensor::{DataType, Shape, Tensor};

#[derive(Debug, Clone, Default)]
pub struct SessionConfig {
    pub device: Option<Device>,
}

#[derive(Debug, Clone, Default)]
pub struct ExecutionPlan {
    pub ops: Vec<Op>,
}

/// Owns backend reference and tensor arena for one inference session.
pub struct Session<'a> {
    backend: &'a dyn Backend,
    tensors: HashMap<String, Tensor>,
}

impl<'a> Session<'a> {
    pub fn new(backend: &'a dyn Backend) -> Self {
        Self {
            backend,
            tensors: HashMap::new(),
        }
    }

    pub fn bind(&mut self, name: impl Into<String>, tensor: Tensor) {
        self.tensors.insert(name.into(), tensor);
    }

    pub fn get(&self, name: &str) -> Option<&Tensor> {
        self.tensors.get(name)
    }

    pub fn take(&mut self, name: &str) -> Option<Tensor> {
        self.tensors.remove(name)
    }

    pub fn allocate(
        &mut self,
        name: impl Into<String>,
        dtype: DataType,
        shape: Shape,
    ) -> Result<()> {
        let t = self.backend.allocate(dtype, shape)?;
        self.tensors.insert(name.into(), t);
        Ok(())
    }
}

pub struct InferenceRuntime {
    backend: Box<dyn Backend>,
    loaders: ModelLoaderRegistry,
}

impl InferenceRuntime {
    pub fn new(backend: Box<dyn Backend>) -> Self {
        Self {
            backend,
            loaders: ModelLoaderRegistry::new(),
        }
    }

    pub fn with_loader(mut self, loader: Box<dyn ModelLoader>) -> Self {
        self.loaders.register(loader);
        self
    }

    pub fn register_loader(&mut self, loader: Box<dyn ModelLoader>) {
        self.loaders.register(loader);
    }

    pub fn backend(&self) -> &dyn Backend {
        self.backend.as_ref()
    }

    pub fn create_session(&self) -> Session<'_> {
        Session::new(self.backend.as_ref())
    }

    pub fn load_model(&self, path: &Path) -> Result<Box<dyn InferenceExecutor>> {
        self.loaders.load(path, self.backend.as_ref())
    }

    #[instrument(skip(self, model, inputs, session), fields(backend = %self.backend.id()))]
    pub fn infer(
        &self,
        model: &dyn InferenceExecutor,
        inputs: &[Tensor],
        session: &mut Session<'_>,
    ) -> Result<Vec<Tensor>> {
        if inputs.len() != model.inputs().len() {
            return Err(Error::Runtime(format!(
                "expected {} inputs, got {}",
                model.inputs().len(),
                inputs.len()
            )));
        }
        let outputs = model.infer(inputs)?;
        for (i, spec) in model.outputs().iter().enumerate() {
            if let Some(tensor) = outputs.get(i) {
                session.bind(spec.name.clone(), tensor.clone());
            }
        }
        Ok(outputs)
    }

    #[instrument(skip(self, model, session), fields(backend = %self.backend.id()))]
    pub fn run_model(&self, model: &dyn InferenceModel, session: &mut Session<'_>) -> Result<()> {
        let plan = model.plan();
        debug!(ops = plan.ops.len(), "executing plan");
        for op in &plan.ops {
            self.run_op(op, session)?;
        }
        Ok(())
    }

    pub fn run_op(&self, op: &Op, _session: &mut Session<'_>) -> Result<()> {
        match &op.kind {
            OpKind::Noop => Ok(()),
            OpKind::Fill { .. } | OpKind::MapUnary { .. } | OpKind::Custom { .. } => {
                Err(Error::UnsupportedOp(format!(
                    "op `{}` requires explicit I/O binding in this scaffold",
                    op.name
                )))
            }
        }
    }
}
