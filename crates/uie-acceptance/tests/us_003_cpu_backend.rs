//! US-003
use uie_core::backend::cpu::CpuBackend;
use uie_core::backend::Backend;
use uie_core::tensor::{DataType, Shape};

#[test]
fn us_003_cpu_backend_allocates() {
    let backend = CpuBackend::new();
    let t = backend
        .allocate(DataType::F32, Shape::new([4]))
        .expect("allocate");
    assert_eq!(t.shape().dims(), &[4]);
}