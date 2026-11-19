use std::path::PathBuf;

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
    let mut runner =
        ByteChannelPipelineRunner::from_loaders(vec![Box::new(StubModelLoader::default())], hops);
    let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let out = runner.run_tensors("pipe", &[inputs[0].clone()]).unwrap();
    assert!(!out.is_empty());
}