//! US-026: browser WASM packaging + DevicePolicy story events.
use std::path::PathBuf;

use uie_acceptance::{assert_story_events, StoryEventLog};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

#[test]
fn us_026_wasm_packaging_assets() {
    let mut log = StoryEventLog::new("US-026");
    let root = repo_root();

    assert!(
        root.join("crates/uie-wasm/Cargo.toml").is_file(),
        "missing uie-wasm crate"
    );
    log.record("uie_wasm_crate_present");

    assert!(
        root.join("web/mnist/index.html").is_file(),
        "missing web/mnist demo"
    );
    log.record("web_mnist_demo_present");

    assert!(
        root.join("web/mnist/mnist.spec.mjs").is_file(),
        "missing playwright spec"
    );
    log.record("playwright_spec_present");

    assert!(
        root.join("scripts/wasm-mnist-ci.sh").is_file(),
        "missing wasm-mnist-ci.sh"
    );
    log.record("wasm_ci_script_present");

    assert!(
        root.join(".github/workflows/uie-wasm-chromium.yml")
            .is_file(),
        "missing chromium workflow"
    );
    log.record("wasm_chromium_workflow_present");

    assert_story_events(
        &log,
        &[
            "uie_wasm_crate_present",
            "web_mnist_demo_present",
            "playwright_spec_present",
            "wasm_ci_script_present",
            "wasm_chromium_workflow_present",
        ],
    );
}
