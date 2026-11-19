//! US-009 (CI stub): fixture shape only. Real ONNX infer → `live_models.rs` + `live-onnx` feature.
mod support;

#[test]
fn us_009_fixture_input_is_28x28_f32() {
    let path = support::repo_root().join("models/fixtures/digit_one_f32.bin");
    assert!(path.is_file());
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(bytes.len(), 28 * 28 * 4);
}