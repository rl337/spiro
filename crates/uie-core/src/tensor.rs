use std::sync::Arc;

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    F32,
    F64,
    I32,
    U8,
}

impl DataType {
    pub fn size_bytes(self) -> usize {
        match self {
            DataType::F32 | DataType::I32 => 4,
            DataType::F64 => 8,
            DataType::U8 => 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    dims: Vec<usize>,
}

impl Shape {
    pub fn new(dims: impl Into<Vec<usize>>) -> Self {
        Self { dims: dims.into() }
    }

    pub fn dims(&self) -> &[usize] {
        &self.dims
    }

    pub fn rank(&self) -> usize {
        self.dims.len()
    }

    pub fn num_elements(&self) -> usize {
        self.dims.iter().product()
    }

    pub fn is_contiguous_valid(&self) -> bool {
        !self.dims.is_empty() && self.dims.iter().all(|&d| d > 0)
    }
}

/// Owns or shares byte storage for a typed, shaped buffer.
#[derive(Debug, Clone)]
pub struct Tensor {
    dtype: DataType,
    shape: Shape,
    data: Arc<Vec<u8>>,
}

impl Tensor {
    pub fn empty(dtype: DataType, shape: Shape) -> Result<Self> {
        if !shape.is_contiguous_valid() {
            return Err(Error::InvalidShape(format!("{:?}", shape.dims())));
        }
        let nbytes = shape.num_elements().saturating_mul(dtype.size_bytes());
        Ok(Self {
            dtype,
            shape,
            data: Arc::new(vec![0u8; nbytes]),
        })
    }

    pub fn from_bytes(dtype: DataType, shape: Shape, data: Vec<u8>) -> Result<Self> {
        let expected = shape.num_elements().saturating_mul(dtype.size_bytes());
        if data.len() != expected {
            return Err(Error::InvalidShape(format!(
                "data len {} != expected {} for shape {:?}",
                data.len(),
                expected,
                shape.dims()
            )));
        }
        Ok(Self {
            dtype,
            shape,
            data: Arc::new(data),
        })
    }

    pub fn dtype(&self) -> DataType {
        self.dtype
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn nbytes(&self) -> usize {
        self.data.len()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn view(&self) -> TensorView<'_> {
        TensorView {
            dtype: self.dtype,
            shape: self.shape.clone(),
            data: &self.data,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TensorView<'a> {
    pub dtype: DataType,
    pub shape: Shape,
    pub data: &'a [u8],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shape_num_elements() {
        let s = Shape::new([2, 3, 4]);
        assert_eq!(s.num_elements(), 24);
    }

    #[test]
    fn u8_tensor_round_trip() {
        let shape = Shape::new([4]);
        let t = Tensor::from_bytes(DataType::U8, shape, vec![1, 2, 3, 4]).unwrap();
        assert_eq!(t.as_bytes(), &[1, 2, 3, 4]);
    }
}
