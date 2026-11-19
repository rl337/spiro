//! Per-story acceptance tests live under `crates/uie-acceptance/tests/`.
//! See `docs/acceptance-tests.md`.

pub mod mode;

pub use mode::{acceptance_mode, is_ci_mode, is_live_mode, AcceptanceMode};