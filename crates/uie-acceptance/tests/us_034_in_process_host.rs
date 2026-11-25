//! US-034: in-process host–shaped injector (loaders + embedder, no byte channel).
use std::path::PathBuf;

use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_core::backend::cpu::CpuBackend;
use uie_core::InferenceRuntime;
use uie_embeddings::{assert_embed_telemetry, STUB_MODEL_ID};
use uie_iv::uie::modules::OverrideCiModule;
use uie_iv::Injector;
use uie_telemetry::ExecutionRecorder;

fn stubs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("models/stubs")
}

#[test]
fn us_034_host_injector_resolves_stub_stack() {
    let mut log = StoryEventLog::new("US-034");
    let inj = Injector::from_modules(&[&OverrideCiModule]).unwrap();

    assert!(inj.loaders().iter().any(|l| l.format_id() == "stub"));
    assert_eq!(inj.text_embedder().model_id(), STUB_MODEL_ID);
    log.record("host_injector_ok");

    let mut rt = InferenceRuntime::new(Box::new(CpuBackend::new()));
    for loader in inj.loaders() {
        rt.register_loader(loader);
    }
    let model = rt.load_model(&stubs_dir().join("llm.stub")).unwrap();
    let inputs = uie_stub_models::sample_input(uie_stub_models::ModalityKind::LlmTextCompletion)
        .unwrap();
    let mut session = rt.create_session();
    let outcome = rt
        .infer(
            model.as_ref(),
            &inputs,
            &mut session,
            ExecutionRecorder::new("us-034-host"),
        )
        .unwrap();
    assert!(!outcome.outputs.is_empty());
    log.record("host_stub_loader_ok");

    let mut embed_rec = ExecutionRecorder::new("us-034-embed");
    let vectors = inj
        .text_embedder()
        .embed(&["host chunk".into()], &mut embed_rec)
        .unwrap();
    assert_eq!(vectors.len(), 1);
    assert_embed_telemetry(&embed_rec.finish());

    assert_story_events(&log, &["host_injector_ok", "host_stub_loader_ok"]);
}
