use std::sync::Arc;

use uie_telemetry::{assert_story_events, StoryEventLog};
use uie_iv::uie::loaders::{LiveLoaderSet, ModelLoaderSet, StubOnlyLoaderSet};
use uie_iv::uie::profile::{AcceptanceProfile, CiAcceptanceProfile, LiveAcceptanceProfile};
use uie_iv::uie::modules::{OverrideCiModule, OverrideLiveModule};
use uie_iv::{Binder, Injector, Module};

#[test]
fn ci_profile_skips_live_tests() {
    let mut log = StoryEventLog::new("US-024");
    let p = CiAcceptanceProfile;
    assert!(p.skip_live_test("live_us_021"));
    assert!(!p.run_live_integration());
    log.record("ci_profile_skips_live");
    assert_story_events(&log, &["ci_profile_skips_live"]);
}

#[test]
fn live_profile_runs_live_tests() {
    let p = LiveAcceptanceProfile;
    assert!(!p.skip_live_test("live_us_021"));
}

#[test]
fn override_modules_build_injector() {
    let inj = Injector::from_modules(&[&OverrideCiModule]).unwrap();
    assert_eq!(inj.acceptance_profile().id(), "ci");
    assert_eq!(inj.loaders().len(), 1);

    let inj = Injector::from_modules(&[&OverrideLiveModule]).unwrap();
    assert_eq!(inj.acceptance_profile().id(), "live");
}

#[test]
fn loader_sets_stub_vs_live_chain() {
    let stub = StubOnlyLoaderSet;
    assert_eq!(stub.loaders().len(), 1);
    let live = LiveLoaderSet;
    assert!(live.loaders().len() >= 1);
}

#[test]
fn binder_rejects_duplicate_profile() {
    let mut b = Binder::default();
    b.bind_acceptance_profile(Arc::new(CiAcceptanceProfile)).unwrap();
    assert!(b.bind_acceptance_profile(Arc::new(LiveAcceptanceProfile)).is_err());
}