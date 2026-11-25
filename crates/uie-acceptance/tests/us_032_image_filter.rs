//! US-032: image + JSON filters → filtered image (img2img-shaped stub).
mod common;

use uie_core::backend::cpu::CpuBackend;
use uie_core::telemetry::ExecutionRecorder;
use uie_core::InferenceRuntime;
use uie_stub_models::telemetry_assert::assert_stub_modality_telemetry;
use uie_stub_models::{
    apply_filters_nchw, parse_filter_json, sample_input, FilterOp, FilterRequest, ModalityKind,
    StubModelLoader,
};

#[test]
fn us_032_image_filter_stub_applies_invert() {
    let rt =
        InferenceRuntime::new(Box::new(CpuBackend::new())).with_loader(Box::new(StubModelLoader));
    let path = common::stubs_dir().join("image_filter.stub");
    let model = rt.load_model(&path).unwrap();
    let inputs = sample_input(ModalityKind::ImageFilter).unwrap();
    let mut session = rt.create_session();
    let outcome = rt
        .infer(
            model.as_ref(),
            &inputs,
            &mut session,
            ExecutionRecorder::new("us-032"),
        )
        .unwrap();

    assert_eq!(outcome.outputs.len(), 1);
    assert_stub_modality_telemetry(&outcome.trace, ModalityKind::ImageFilter);

    let req = FilterRequest {
        filters: vec![FilterOp::Invert],
    };
    let expected = apply_filters_nchw(&inputs[0], &req).unwrap();
    let got = outcome.outputs[0].as_f32_slice().unwrap();
    let want = expected.as_f32_slice().unwrap();
    assert_eq!(got.len(), want.len());
    for (a, b) in got.iter().zip(want.iter()) {
        assert!((a - b).abs() < 1e-5);
    }
}

#[test]
fn us_032_filter_json_roundtrip() {
    let raw = br#"{"filters":[{"op":"grayscale"},{"op":"contrast","amount":1.5}]}"#;
    let req = parse_filter_json(raw).unwrap();
    assert_eq!(req.filters.len(), 2);
}
