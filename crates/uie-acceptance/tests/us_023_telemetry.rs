//! US-023: execution traces and TTFT-style metrics on infer.
use uie_core::backend::cpu::CpuBackend;
use uie_core::telemetry::{ComputeDevice, ExecutionRecorder};
use uie_core::InferenceRuntime;
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

mod support;

#[test]
fn us_023_stub_infer_emits_trace_with_phases() {
    let rt = InferenceRuntime::new(Box::new(CpuBackend::new()))
        .with_loader(Box::new(StubModelLoader::default()));
    let path = support::stubs_dir().join("llm.stub");
    let model = rt.load_model(&path).unwrap();
    let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let mut session = rt.create_session();
    let rec = ExecutionRecorder::new("acc-023").with_transport("in_process");
    let outcome = rt
        .infer_with_trace(model.as_ref(), &inputs, &mut session, rec)
        .unwrap();

    assert!(outcome.trace.metrics.total_ms >= 0.0);
    assert!(!outcome.trace.phases.is_empty());
    assert_eq!(outcome.trace.phases[0].device, ComputeDevice::Cpu);
}

#[test]
fn us_023_trace_summary_serializes_for_ipc() {
    use uie_core::telemetry::TraceSummary;
    let rt = InferenceRuntime::new(Box::new(CpuBackend::new()))
        .with_loader(Box::new(StubModelLoader::default()));
    let path = support::stubs_dir().join("llm.stub");
    let model = rt.load_model(&path).unwrap();
    let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let mut session = rt.create_session();
    let outcome = rt
        .infer_with_trace(
            model.as_ref(),
            &inputs,
            &mut session,
            ExecutionRecorder::new("acc-023-json"),
        )
        .unwrap();
    let summary = TraceSummary::from(&outcome.trace);
    let json = serde_json::to_string(&summary).unwrap();
    assert!(json.contains("total_ms"));
}