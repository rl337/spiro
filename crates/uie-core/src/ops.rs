use crate::tensor::{DataType, Shape};

/// Serializable operation kinds for graph execution and future model import.
#[derive(Debug, Clone, PartialEq)]
pub enum OpKind {
    Noop,
    Fill {
        value: f64,
    },
    /// Element-wise map: out[i] = f32(in[i]) for demo workloads.
    MapUnary {
        name: String,
    },
    Custom {
        name: String,
    },
}

#[derive(Debug, Clone)]
pub struct Op {
    pub kind: OpKind,
    pub name: String,
    pub input_shapes: Vec<Shape>,
    pub output_shapes: Vec<Shape>,
    pub output_dtypes: Vec<DataType>,
}

impl Op {
    pub fn noop(name: impl Into<String>) -> Self {
        Self {
            kind: OpKind::Noop,
            name: name.into(),
            input_shapes: vec![],
            output_shapes: vec![],
            output_dtypes: vec![],
        }
    }
}
