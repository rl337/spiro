//! US-024: pyiv-style DI for CI vs live (no env conditionals at call sites).
use uie_acceptance::composition::{injector, loader_format_ids};
use uie_iv::uie::modules::{OverrideCiModule, OverrideLiveModule};
use uie_iv::Injector;
use uie_acceptance::{assert_story_events, StoryEventLog};

#[test]
fn us_024_ci_injector_skips_live_without_env_branch() {
    let mut log = StoryEventLog::new("US-024");
    let inj = Injector::from_modules(&[&OverrideCiModule]).unwrap();
    assert!(inj.acceptance_profile().skip_live_test("live_us_007"));
    log.record("ci_profile_injected");
    assert!(
        inj.loaders()
            .iter()
            .all(|l| l.format_id() == "stub")
    );
    assert_story_events(&log, &["ci_profile_injected"]);
}

#[test]
fn us_024_live_override_enables_integration() {
    let inj = Injector::from_modules(&[&OverrideLiveModule]).unwrap();
    assert!(!inj.acceptance_profile().skip_live_test("live_us_007"));
}

#[test]
fn us_024_acceptance_composition_root_resolves() {
    let mut log = StoryEventLog::new("US-024");
    let _ = injector().acceptance_profile().id();
    log.record("composition_root_ok");
    assert!(!loader_format_ids().is_empty());
    assert_story_events(&log, &["composition_root_ok"]);
}