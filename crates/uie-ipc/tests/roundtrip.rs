use std::path::PathBuf;
use uie_ipc::{
    InferRequest, InProcessChannel, InferenceEngineHost, IpcMessage, PROTOCOL_VERSION, TensorPayload,
};
use uie_ipc::ByteChannel;
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

fn workspace_stub(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../models/stubs")
        .join(name)
}

#[test]
fn in_process_stub_llm_roundtrip() {
    let host = InferenceEngineHost::new(vec![Box::new(StubModelLoader::default())]);
    let mut ch = InProcessChannel::new(host);

    let ack = ch
        .roundtrip(&IpcMessage::Hello {
            protocol_version: PROTOCOL_VERSION,
        })
        .unwrap();
    assert!(matches!(ack, IpcMessage::HelloAck { .. }));

    let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let payloads: Vec<TensorPayload> = inputs
        .iter()
        .zip(["input_ids"])
        .map(|(t, n)| TensorPayload::from_tensor(n, t).unwrap())
        .collect();

    let resp = ch
        .roundtrip(&IpcMessage::Infer {
            request_id: "t1".into(),
            model_path: workspace_stub("llm.stub").to_string_lossy().into(),
            inputs: payloads,
        })
        .unwrap();

    match resp {
        IpcMessage::InferResult { outputs, .. } => assert_eq!(outputs.len(), 1),
        other => panic!("unexpected {other:?}"),
    }
}

#[cfg(unix)]
#[test]
fn unix_socket_stub_roundtrip() {
    use std::thread;

    let dir = tempfile::tempdir().unwrap();
    let sock = dir.path().join("uie-test.sock");

    let host = InferenceEngineHost::new(vec![Box::new(StubModelLoader::default())]);
    let listener = uie_ipc::UnixListenerChannel::bind(&sock, host).unwrap();

    let sock_c = sock.clone();
    let server = thread::spawn(move || listener.serve_one().unwrap());

    let mut client = uie_ipc::UnixSocketChannel::connect(&sock_c).unwrap();
    client
        .roundtrip(&IpcMessage::Hello {
            protocol_version: PROTOCOL_VERSION,
        })
        .unwrap();

    let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let payloads = vec![TensorPayload::from_tensor("input_ids", &inputs[0]).unwrap()];
    let resp = client
        .roundtrip(&IpcMessage::Infer {
            request_id: "u1".into(),
            model_path: workspace_stub("llm.stub").to_string_lossy().into(),
            inputs: payloads,
        })
        .unwrap();
    assert!(matches!(resp, IpcMessage::InferResult { .. }));
    drop(client);
    server.join().unwrap();
}