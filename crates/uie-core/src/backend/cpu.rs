use crate::backend::{Backend, BackendId, Device};
use crate::error::{Error, Result};
use crate::model::Graph;
use crate::ops::{Op, OpKind};
use crate::runtime::ExecutionPlan;
use crate::tensor::{DataType, Shape, Tensor};

/// Reference CPU backend — always available, implements basic tensor alloc and ops.
#[derive(Debug)]
pub struct CpuBackend {
    id: BackendId,
}

impl CpuBackend {
    pub fn new() -> Self {
        Self {
            id: BackendId::new("cpu"),
        }
    }
}

impl Default for CpuBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for CpuBackend {
    fn id(&self) -> BackendId {
        self.id.clone()
    }

    fn device(&self) -> Device {
        Device::cpu()
    }

    fn allocate(&self, dtype: DataType, shape: Shape) -> Result<Tensor> {
        Tensor::empty(dtype, shape)
    }

    fn execute(&self, op: &Op, inputs: &[Tensor], outputs: &mut [Tensor]) -> Result<()> {
        match &op.kind {
            OpKind::Noop => Ok(()),
            OpKind::Fill { value } => {
                if outputs.len() != 1 {
                    return Err(Error::InvalidShape("fill expects one output".into()));
                }
                fill_tensor(&mut outputs[0], *value)?;
                Ok(())
            }
            OpKind::MapUnary { name } => {
                let _ = (name, inputs);
                Err(Error::UnsupportedOp(
                    "MapUnary not implemented on CPU yet".into(),
                ))
            }
            OpKind::Custom { name } => Err(Error::UnsupportedOp(format!(
                "custom op `{name}` not registered on CPU"
            ))),
        }
    }
}

fn fill_tensor(tensor: &mut Tensor, value: f64) -> Result<()> {
    match tensor.dtype() {
        DataType::U8 => {
            let v = value.clamp(0.0, 255.0) as u8;
            let data = tensor.as_bytes();
            let mut owned = data.to_vec();
            owned.fill(v);
            *tensor = Tensor::from_bytes(DataType::U8, tensor.shape().clone(), owned)?;
            Ok(())
        }
        DataType::F32 => {
            let v = value as f32;
            let n = tensor.shape().num_elements();
            let bytes: Vec<u8> = (0..n).flat_map(|_| v.to_le_bytes()).collect();
            *tensor = Tensor::from_bytes(DataType::F32, tensor.shape().clone(), bytes)?;
            Ok(())
        }
        other => Err(Error::UnsupportedOp(format!("fill for {:?}", other))),
    }
}

pub fn demo_plan_from_graph(graph: &Graph) -> ExecutionPlan {
    graph.to_plan()
}
