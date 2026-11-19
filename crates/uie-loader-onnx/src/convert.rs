use ndarray::ArrayD;
use tract_onnx::prelude::TValue;
use uie_core::error::{Error, Result};
use uie_core::tensor::{DataType, Shape, Tensor};

pub(crate) fn tensor_to_tvalue(tensor: &Tensor) -> Result<TValue> {
    Ok(tensor_to_array(tensor)?.into())
}

pub(crate) fn tensor_to_array(tensor: &Tensor) -> Result<ArrayD<f32>> {
    if tensor.dtype() != DataType::F32 {
        return Err(Error::DtypeMismatch {
            expected: DataType::F32,
            got: tensor.dtype(),
        });
    }
    let shape = tensor.shape().dims();
    let data = tensor.as_f32_slice()?;
    ArrayD::from_shape_vec(shape, data.to_vec()).map_err(|e| Error::InvalidShape(e.to_string()))
}

pub(crate) fn tvalue_to_tensor(value: &TValue, fallback_name: &str) -> Result<Tensor> {
    let _ = fallback_name;
    let view = value
        .to_array_view::<f32>()
        .map_err(|e| Error::Runtime(e.to_string()))?;
    let shape = Shape::new(view.shape().to_vec());
    Tensor::from_f32_slice(
        shape,
        view.as_slice()
            .ok_or_else(|| Error::InvalidShape("tract output not contiguous".into()))?,
    )
}
