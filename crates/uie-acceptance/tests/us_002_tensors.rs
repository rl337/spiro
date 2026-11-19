//! US-002
use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_core::tensor::{DataType, Shape, Tensor};

#[test]
fn us_002_tensor_roundtrip_bytes() {
    let mut log = StoryEventLog::new("US-002");
    let data = vec![1.0_f32, 2.0, 3.0, 4.0];
    let t = Tensor::from_f32_slice(Shape::new([2, 2]), &data).unwrap();
    let back = t.as_f32_slice().unwrap();
    assert_eq!(back, data.as_slice());
    log.record("tensor_f32_roundtrip");
    assert_eq!(t.dtype(), DataType::F32);
    log.record("dtype_preserved");
    assert_story_events(&log, &["tensor_f32_roundtrip", "dtype_preserved"]);
}
