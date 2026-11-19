//! US-002
use uie_core::tensor::{DataType, Shape, Tensor};

#[test]
fn us_002_tensor_roundtrip_bytes() {
    let data = vec![1.0_f32, 2.0, 3.0, 4.0];
    let t = Tensor::from_f32_slice(Shape::new([2, 2]), &data).unwrap();
    let back = t.as_f32_slice().unwrap();
    assert_eq!(back, data.as_slice());
    assert_eq!(t.dtype(), DataType::F32);
}