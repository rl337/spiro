use std::path::PathBuf;

use uie_acceptance::{is_ci_mode, is_live_mode};

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

pub fn stubs_dir() -> PathBuf {
    repo_root().join("models/stubs")
}

/// Live-only tests return early in CI/stub mode (pass without running).
pub fn skip_unless_live(test_name: &str) -> bool {
    if is_live_mode() {
        return false;
    }
    eprintln!("skip {test_name}: UIE_ACCEPTANCE_MODE is ci (stub); set live to run real models");
    true
}

/// Stub-only tests return early in live mode when a dedicated live test exists (optional gate).
pub fn skip_stub_in_live(test_name: &str) -> bool {
    if is_ci_mode() {
        return false;
    }
    if std::env::var("UIE_ACCEPTANCE_RUN_STUBS_IN_LIVE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        return false;
    }
    eprintln!(
        "skip {test_name}: live mode (set UIE_ACCEPTANCE_RUN_STUBS_IN_LIVE=1 to re-run stub tests)"
    );
    true
}