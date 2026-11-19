//! Shared helpers for integration tests (each test binary may not use every item).
#![allow(dead_code)]

use std::path::PathBuf;

mod composition {
    pub use uie_acceptance::composition::injector;
}

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

pub fn stubs_dir() -> PathBuf {
    repo_root().join("models/stubs")
}

/// Live-only tests: delegated to injected [`AcceptanceProfile`], not raw env checks.
pub fn skip_unless_live(test_name: &str) -> bool {
    composition::injector()
        .acceptance_profile()
        .skip_live_test(test_name)
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

use uie_acceptance::is_ci_mode;
