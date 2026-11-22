use std::path::PathBuf;

use uie_core::telemetry::ExecutionRecorder;
use uie_core::CpuBackend;
use uie_core::InferenceRuntime;
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

#[test]
fn diffusion_stub_infer_direct() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../models/stubs/diffusion_t2i.stub");
    let rt =
        InferenceRuntime::new(Box::new(CpuBackend::new())).with_loader(Box::new(StubModelLoader));
    let model = rt.load_model(&path).expect("load");
    let inputs = sample_input(ModalityKind::DiffusionTextToImage).unwrap();
    let mut session = rt.create_session();
    let out = rt
        .infer(
            model.as_ref(),
            &inputs,
            &mut session,
            ExecutionRecorder::new("test"),
        )
        .expect("infer");
    assert!(!out.outputs.is_empty());
}
