//! US-009 (CI stub): fixture shape only. Real ONNX infer → `live_models.rs` + `live-onnx` feature.
mod support;

use uie_acceptance::{assert_story_events, StoryEventLog};

#[test]
fn us_009_fixture_input_is_28x28_f32() {
    let mut log = StoryEventLog::new("US-009");
    let path = support::repo_root().join("models/fixtures/digit_one_f32.bin");
    assert!(path.is_file());
    log.record("mnist_fixture_present");
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(bytes.len(), 28 * 28 * 4);
    log.record("mnist_input_tensor_bytes");
    assert_story_events(&log, &["mnist_fixture_present", "mnist_input_tensor_bytes"]);
}