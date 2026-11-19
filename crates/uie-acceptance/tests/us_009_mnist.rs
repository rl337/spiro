//! US-009
mod support;

#[test]
#[ignore = "uie-loader-onnx may not compile until tract imports are fixed"]
fn us_009_fixture_input_is_28x28_f32() {
    let path = support::repo_root().join("models/fixtures/digit_one_f32.bin");
    assert!(path.is_file());
    let bytes = std::fs::read(path).unwrap();
    assert_eq!(bytes.len(), 28 * 28 * 4);
}