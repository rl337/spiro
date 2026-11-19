//! US-020
mod support;

use uie_acceptance::assert_ipc_infer_trace;
use uie_ipc::{ByteChannel, IpcMessage, PROTOCOL_VERSION, TensorPayload};
use uie_ipc::{InferenceEngineHost, InProcessChannel};
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

#[test]
fn us_020_in_process_infer_roundtrip() {
    let host = InferenceEngineHost::new(vec![Box::new(StubModelLoader::default())]);
    let mut ch = InProcessChannel::new(host);
    ch.roundtrip(&IpcMessage::Hello {
        protocol_version: PROTOCOL_VERSION,
    })
    .unwrap();

    let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let payloads = vec![TensorPayload::from_tensor("input_ids", &inputs[0]).unwrap()];
    let path = support::stubs_dir().join("llm.stub");
    let resp = ch
        .roundtrip(&IpcMessage::Infer {
            request_id: "acc-020".into(),
            model_path: path.to_string_lossy().into(),
            inputs: payloads,
        })
        .unwrap();
    assert_ipc_infer_trace(&resp, "US-020");
}