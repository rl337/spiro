use std::collections::HashMap;

use tracing::{debug, instrument};

use crate::backend::{Backend, Device};
use crate::error::{Error, Result};
use crate::model::InferenceModel;
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
}

impl InferenceRuntime {
    pub fn new(backend: Box<dyn Backend>) -> Self {
        Self { backend }
    }

    pub fn backend(&self) -> &dyn Backend {
        self.backend.as_ref()
    }

    pub fn create_session(&self) -> Session<'_> {
        Session::new(self.backend.as_ref())
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
