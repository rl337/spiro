//! Structured **execution traces** for Spiro inference evaluation (TTFT, throughput, device time).

pub mod assert;
pub mod ipc_assert;
pub mod story_events;
mod device;
mod metrics;
mod recorder;
mod sink;
mod trace;

pub use device::ComputeDevice;
pub use metrics::InferMetrics;
pub use recorder::ExecutionRecorder;
pub use sink::{global_sink, set_global_sink, JsonlSink, NoopSink, TelemetrySink};
pub use assert::assert_phases;
pub use ipc_assert::assert_ipc_infer_trace_summary;
pub use story_events::{assert_story_events, StoryEventLog};
pub use trace::{ExecutionTrace, PhaseSpan, TraceSummary};