//! Per-story acceptance tests live under `crates/uie-acceptance/tests/`.
//! See `docs/acceptance-tests.md`.

pub mod composition;
pub mod mode;
pub mod observability;
pub mod story_assert;

pub use mode::{acceptance_mode, is_ci_mode, is_live_mode, AcceptanceMode};
pub use observability::{assert_story_events, StoryEventLog};
pub use story_assert::{assert_infer_outcome_trace, assert_ipc_infer_trace, assert_trace_summary};
