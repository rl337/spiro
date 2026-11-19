//! US-022
mod support;

use uie_pipeline::{ByteChannelPipelineRunner, PipelineHop};
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

#[test]
fn us_022_two_hop_byte_channel_pipeline() {
    let stubs = support::stubs_dir();
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
    let mut runner =
        ByteChannelPipelineRunner::from_loaders(vec![Box::new(StubModelLoader::default())], hops);
    let inputs = sample_input(ModalityKind::LlmTextCompletion).unwrap();
    let out = runner.run_tensors("acc-022", &[inputs[0].clone()]).unwrap();
    assert!(!out.is_empty());
}