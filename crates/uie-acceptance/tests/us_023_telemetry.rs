//! US-023: execution traces and TTFT-style metrics on infer.
use uie_core::backend::cpu::CpuBackend;
use uie_core::telemetry::{assert_phases, ExecutionRecorder};
use uie_core::InferenceRuntime;
use uie_stub_models::telemetry::{phase_modality, PHASE_POLICY_INFER, PHASE_VALIDATE_INPUTS};
use uie_stub_models::telemetry_assert::assert_stub_executor_telemetry;
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

mod common;

#[test]
fn us_023_stub_infer_emits_required_phases() {
    let rt =
        InferenceRuntime::new(Box::new(CpuBackend::new())).with_loader(Box::new(StubModelLoader));
    let path = common::stubs_dir().join("llm.stub");
    let model = rt.load_model(&path).unwrap();
    let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let mut session = rt.create_session();
    let outcome = rt
        .infer(
            model.as_ref(),
            &inputs,
            &mut session,
            ExecutionRecorder::new("acc-023").with_transport("in_process"),
        )
        .unwrap();

    assert_stub_executor_telemetry(&outcome.trace);
    assert_phases(
        &outcome.trace,
        &[
            PHASE_VALIDATE_INPUTS,
            PHASE_POLICY_INFER,
            "runtime_session_bind",
        ],
    );
    assert!(outcome.trace.metrics.total_ms >= 0.0);
    assert_eq!(
        outcome.trace.user_story.as_deref(),
        Some(ModalityKind::LlmTextCompletion.story_id())
    );
    assert!(outcome.trace.metrics.prompt_tokens > 0);
    assert!(outcome.trace.metrics.completion_tokens > 0);
    assert!(outcome.trace.metrics.ttft_ms.is_some());
    assert!(outcome
        .trace
        .phases
        .iter()
        .any(|p| p.name == phase_modality(ModalityKind::LlmTextCompletion)));
}

#[test]
fn us_023_trace_summary_serializes_for_ipc() {
    use uie_core::telemetry::TraceSummary;
    let rt =
        InferenceRuntime::new(Box::new(CpuBackend::new())).with_loader(Box::new(StubModelLoader));
    let path = common::stubs_dir().join("llm.stub");
    let model = rt.load_model(&path).unwrap();
    let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let mut session = rt.create_session();
    let outcome = rt
        .infer(
            model.as_ref(),
            &inputs,
            &mut session,
            ExecutionRecorder::new("acc-023-json"),
        )
        .unwrap();
    let summary = TraceSummary::from(&outcome.trace);
    let json = serde_json::to_string(&summary).unwrap();
    assert!(json.contains("total_ms"));
    assert_stub_executor_telemetry(&outcome.trace);
}
