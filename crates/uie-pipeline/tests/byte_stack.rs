use std::path::PathBuf;

use uie_ipc::{ByteChannel, InProcessChannel, InferenceEngineHost, IpcMessage, TensorPayload};
use uie_pipeline::{ByteChannelPipelineRunner, PipelineHop};
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

fn stub_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../models/stubs")
}

#[test]
fn us_022_byte_channel_two_stub_hops() {
    let stubs = stub_dir();
    let hops = vec![
        PipelineHop {
            label: "hop0".into(),
            model_path: stubs.join("llm.stub"),
        },
        PipelineHop {
            label: "hop1".into(),
            model_path: stubs.join("llm.stub"),
        },
    ];
    let mut runner = ByteChannelPipelineRunner::from_loaders(vec![Box::new(StubModelLoader)], hops);
    let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let out = runner.run_tensors("pipe", &[inputs[0].clone()]).unwrap();
    assert!(!out.is_empty());

    let host = InferenceEngineHost::new(vec![Box::new(StubModelLoader)]);
    let mut ch = InProcessChannel::new(host);
    let payloads = vec![TensorPayload::from_tensor("input_ids", &inputs[0]).unwrap()];
    let resp = ch
        .roundtrip(&IpcMessage::Infer {
            request_id: "us-022-pipeline-crate".into(),
            model_path: stubs.join("llm.stub").to_string_lossy().into(),
            inputs: payloads,
        })
        .unwrap();
    if let IpcMessage::InferResult { trace, .. } = &resp {
        uie_telemetry::assert_ipc_infer_trace_summary(
            trace.as_ref().expect("US-022 hop trace"),
            "US-022",
        );
    } else {
        panic!("expected InferResult, got {resp:?}");
    }
}
