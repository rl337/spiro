//! US-035: host CI forces stub loaders + stub embedder via DI (no env at call site).
use std::path::PathBuf;

use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_core::backend::cpu::CpuBackend;
use uie_core::InferenceRuntime;
use uie_embeddings::{STUB_DIMENSIONS, STUB_MODEL_ID};
use uie_iv::uie::modules::OverrideCiModule;
use uie_iv::Injector;
use uie_stub_models::telemetry_assert::assert_stub_executor_telemetry;
use uie_telemetry::ExecutionRecorder;

fn stubs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("models/stubs")
}

#[test]
fn us_035_override_ci_binds_stubs_without_env() {
    let mut log = StoryEventLog::new("US-035");
    let inj = Injector::from_modules(&[&OverrideCiModule]).unwrap();

    assert!(inj.acceptance_profile().skip_live_test("live_us_007"));
    assert!(inj.loaders().iter().all(|l| l.format_id() == "stub"));
    log.record("host_ci_stub_bound");

    let embedder = inj.text_embedder();
    assert_eq!(embedder.model_id(), STUB_MODEL_ID);
    assert_eq!(embedder.dimensions(), STUB_DIMENSIONS);
    log.record("host_stub_embedder_id");

    let mut rt = InferenceRuntime::new(Box::new(CpuBackend::new()));
    for loader in inj.loaders() {
        rt.register_loader(loader);
    }
    let model = rt
        .load_model(&stubs_dir().join("image_filter.stub"))
        .unwrap();
    let inputs =
        uie_stub_models::sample_input(uie_stub_models::ModalityKind::ImageFilter).unwrap();
    let mut session = rt.create_session();
    let outcome = rt
        .infer(
            model.as_ref(),
            &inputs,
            &mut session,
            ExecutionRecorder::new("us-035"),
        )
        .unwrap();
    assert_stub_executor_telemetry(&outcome.trace);

    assert_story_events(&log, &["host_ci_stub_bound", "host_stub_embedder_id"]);
}
