//! Structured **execution traces** for Spiro inference evaluation (TTFT, throughput, device time).

pub mod assert;
mod device;
pub mod ipc_assert;
mod metrics;
mod recorder;
mod sink;
pub mod story_events;
mod trace;

pub use assert::assert_phases;
pub use device::ComputeDevice;
pub use ipc_assert::assert_ipc_infer_trace_summary;
pub use metrics::InferMetrics;
pub use recorder::ExecutionRecorder;
pub use sink::{global_sink, set_global_sink, JsonlSink, NoopSink, TelemetrySink};
pub use story_events::{assert_story_events, StoryEventLog};
pub use trace::{ExecutionTrace, PhaseSpan, TraceSummary};
