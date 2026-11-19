//! US-003
use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_core::backend::cpu::CpuBackend;
use uie_core::backend::Backend;
use uie_core::tensor::{DataType, Shape};

#[test]
fn us_003_cpu_backend_allocates() {
    let mut log = StoryEventLog::new("US-003");
    let backend = CpuBackend::new();
    log.record("cpu_backend_constructed");
    let t = backend
        .allocate(DataType::F32, Shape::new([4]))
        .expect("allocate");
    assert_eq!(t.shape().dims(), &[4]);
    log.record("tensor_allocated_on_cpu");
    assert_story_events(&log, &["cpu_backend_constructed", "tensor_allocated_on_cpu"]);
}