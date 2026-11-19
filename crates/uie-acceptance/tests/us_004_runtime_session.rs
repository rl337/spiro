//! US-004
use uie_core::backend::cpu::CpuBackend;
use uie_core::tensor::{DataType, Shape, Tensor};
use uie_core::InferenceRuntime;

#[test]
fn us_004_session_bind_and_take() {
    let rt = InferenceRuntime::new(Box::new(CpuBackend::new()));
    let mut session = rt.create_session();
    let t = Tensor::zeros(DataType::F32, Shape::new([1])).unwrap();
    session.bind("input", t);
    assert!(session.get("input").is_some());
    assert!(session.take("input").is_some());
}