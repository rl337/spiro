//! US-007
mod support;

use uie_acceptance::{assert_infer_outcome_trace, assert_story_events, StoryEventLog};
use uie_core::backend::cpu::CpuBackend;
use uie_core::telemetry::ExecutionRecorder;
use uie_core::InferenceRuntime;
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};
use uie_stub_models::telemetry_assert::assert_stub_executor_telemetry;

#[test]
fn us_007_stub_loader_loads_executor() {
    let mut log = StoryEventLog::new("US-007");
    let rt = InferenceRuntime::new(Box::new(CpuBackend::new()))
        .with_loader(Box::new(StubModelLoader::default()));
    let path = support::stubs_dir().join("llm.stub");
    let model = rt.load_model(&path).expect("load stub");
    assert!(!model.inputs().is_empty());
    log.record("stub_executor_loaded");

    let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let mut session = rt.create_session();
    let outcome = rt
        .infer(
            model.as_ref(),
            &inputs,
            &mut session,
            ExecutionRecorder::new("us-007"),
        )
        .unwrap();
    assert_infer_outcome_trace(&outcome, "US-007");
    assert_stub_executor_telemetry(&outcome.trace);
    log.record("loader_infer_trace");
    assert_story_events(&log, &["stub_executor_loaded", "loader_infer_trace"]);
}