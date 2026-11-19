//! US-022
mod common;

use uie_acceptance::assert_ipc_infer_trace;
use uie_ipc::{ByteChannel, InProcessChannel, InferenceEngineHost, IpcMessage, TensorPayload};
use uie_pipeline::{ByteChannelPipelineRunner, PipelineHop};
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

#[test]
fn us_022_two_hop_byte_channel_pipeline() {
    let stubs = common::stubs_dir();
    let hops = vec![
        PipelineHop {
            label: "llm_a".into(),
            model_path: stubs.join("llm.stub"),
        },
        PipelineHop {
            label: "llm_b".into(),
            model_path: stubs.join("llm.stub"),
        },
    ];
    let mut runner = ByteChannelPipelineRunner::from_loaders(vec![Box::new(StubModelLoader)], hops);
    let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let out = runner.run_tensors("acc-022", &[inputs[0].clone()]).unwrap();
    assert!(!out.is_empty());

    // Each hop must return infer telemetry on the wire (US-023 / US-020).
    let host = InferenceEngineHost::new(vec![Box::new(StubModelLoader)]);
    let mut ch = InProcessChannel::new(host);
    let payloads = vec![TensorPayload::from_tensor("input_ids", &inputs[0]).unwrap()];
    for (idx, hop_path) in [stubs.join("llm.stub"), stubs.join("llm.stub")]
        .into_iter()
        .enumerate()
    {
        let resp = ch
            .roundtrip(&IpcMessage::Infer {
                request_id: format!("acc-022-hop-{idx}"),
                model_path: hop_path.to_string_lossy().into(),
                inputs: payloads.clone(),
            })
            .unwrap();
        assert_ipc_infer_trace(&resp, "US-022");
    }
}
