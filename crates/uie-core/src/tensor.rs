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

    pub fn from_i32_slice(shape: Shape, values: &[i32]) -> Result<Self> {
        if shape.num_elements() != values.len() {
            return Err(Error::InvalidShape(format!(
                "shape {:?} has {} elements, got {}",
                shape.dims(),
                shape.num_elements(),
                values.len()
            )));
        }
        let bytes: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
        Self::from_bytes(DataType::I32, shape, bytes)
    }

    pub fn as_i32_slice(&self) -> Result<&[i32]> {
        if self.dtype() != DataType::I32 {
            return Err(Error::DtypeMismatch {
                expected: DataType::I32,
                got: self.dtype(),
            });
        }
        let data = self.as_bytes();
        if !data.len().is_multiple_of(4) {
            return Err(Error::InvalidShape("i32 buffer length not aligned".into()));
        }
        Ok(bytemuck::cast_slice(data))
    }

    pub fn from_f32_slice(shape: Shape, values: &[f32]) -> Result<Self> {
        if shape.num_elements() != values.len() {
            return Err(Error::InvalidShape(format!(
                "shape {:?} has {} elements, got {}",
                shape.dims(),
                shape.num_elements(),
                values.len()
            )));
        }
        let bytes: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
        Self::from_bytes(DataType::F32, shape, bytes)
    }

    pub fn zeros(dtype: DataType, shape: Shape) -> Result<Self> {
        Self::empty(dtype, shape)
    }

    pub fn like_zeros(template: &Self) -> Result<Self> {
        Self::empty(template.dtype(), template.shape().clone())
    }

    /// UTF-8 payload in a 1-D `U8` tensor (may be empty).
    pub fn text_tensor(text: &str) -> Result<Self> {
        let bytes = text.as_bytes();
        Self::from_bytes(DataType::U8, Shape::new([bytes.len().max(1)]), {
            let mut out = bytes.to_vec();
            if out.is_empty() {
                out.push(0);
            }
            out
        })
    }

    pub fn as_text(&self) -> Result<String> {
        if self.dtype() != DataType::U8 {
            return Err(Error::DtypeMismatch {
                expected: DataType::U8,
                got: self.dtype(),
            });
        }
        let bytes = self.as_bytes();
        let trimmed = if bytes == [0] { &[][..] } else { bytes };
        String::from_utf8(trimmed.to_vec()).map_err(|e| Error::InvalidShape(e.to_string()))
    }

    pub fn as_f32_slice(&self) -> Result<&[f32]> {
        if self.dtype != DataType::F32 {
            return Err(Error::DtypeMismatch {
                expected: DataType::F32,
                got: self.dtype,
            });
        }
        let data = self.as_bytes();
        if !data.len().is_multiple_of(4) {
            return Err(Error::InvalidShape("f32 buffer length not aligned".into()));
        }
        Ok(bytemuck::cast_slice(data))
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
