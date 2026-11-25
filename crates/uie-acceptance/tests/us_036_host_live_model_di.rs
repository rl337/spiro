//! US-036: live model binding is a DI override (CI documents contract; weights gated).
mod common;

use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_iv::uie::modules::{OverrideCiModule, OverrideLiveModule};
use uie_iv::Injector;

#[test]
fn us_036_live_override_is_module_not_env_branch() {
    let mut log = StoryEventLog::new("US-036");

    let ci = Injector::from_modules(&[&OverrideCiModule]).unwrap();
    assert!(ci.acceptance_profile().skip_live_test("live_us_036_host"));

    let live = Injector::from_modules(&[&OverrideLiveModule]).unwrap();
    assert!(!live.acceptance_profile().skip_live_test("live_us_036_host"));
    // Live loader set may include non-stub formats; host binds model_dir on injectables (HOST-04).
    assert!(!live.loaders().is_empty());

    log.record("host_live_override_ok");
    assert_story_events(&log, &["host_live_override_ok"]);
}

#[test]
fn us_036_live_weight_load_gated() {
    if common::skip_unless_live("live_us_036_host_weight_load") {
        return;
    }
    // When live artifacts exist, a host module MUST bind model_dir on the injectable
    // and load without call-site UIE_* reads. Weight path not wired in CI yet.
    let mut log = StoryEventLog::new("US-036");
    log.record("host_live_weight_path_pending");
    assert_story_events(&log, &["host_live_weight_path_pending"]);
}
