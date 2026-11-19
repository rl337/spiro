//! US-004
use uie_acceptance::{assert_infer_outcome_trace, assert_story_events, StoryEventLog};
use uie_core::backend::cpu::CpuBackend;
use uie_core::telemetry::ExecutionRecorder;
use uie_core::tensor::{DataType, Shape, Tensor};
use uie_core::InferenceRuntime;
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

#[test]
fn us_004_session_bind_and_take() {
    let mut log = StoryEventLog::new("US-004");
    let rt =
        InferenceRuntime::new(Box::new(CpuBackend::new())).with_loader(Box::new(StubModelLoader));
    let mut session = rt.create_session();
    let t = Tensor::zeros(DataType::F32, Shape::new([1])).unwrap();
    session.bind("input", t);
    assert!(session.get("input").is_some());
    log.record("session_bind");
    assert!(session.take("input").is_some());
    log.record("session_take");

    let path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../models/stubs/llm.stub");
    let model = rt.load_model(&path).unwrap();
    let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let outcome = rt
        .infer(
            model.as_ref(),
            &inputs,
            &mut session,
            ExecutionRecorder::new("us-004"),
        )
        .unwrap();
    assert_infer_outcome_trace(&outcome, "US-004");
    log.record("runtime_infer_trace");
    assert_story_events(&log, &["session_bind", "runtime_infer_trace"]);
}
