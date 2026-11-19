//! Structured **execution traces** for Spiro inference evaluation (TTFT, throughput, device time).

mod device;
mod metrics;
mod recorder;
mod sink;
mod trace;

pub use device::ComputeDevice;
pub use metrics::InferMetrics;
pub use recorder::ExecutionRecorder;
pub use sink::{global_sink, set_global_sink, JsonlSink, NoopSink, TelemetrySink};
pub use trace::{ExecutionTrace, PhaseSpan, TraceSummary};