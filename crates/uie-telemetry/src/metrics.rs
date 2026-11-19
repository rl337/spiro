use serde::{Deserialize, Serialize};

use crate::device::ComputeDevice;

/// Aggregate counters for one inference run (LLM-oriented; other modalities fill what applies).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InferMetrics {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    /// Time to first **generated** token (ms); `None` if no generation step.
    pub ttft_ms: Option<f64>,
    pub prefill_ms: f64,
    pub decode_ms: f64,
    pub total_ms: f64,
    /// Sum of phase times tagged [`ComputeDevice::Cpu`].
    pub cpu_time_ms: f64,
    /// Sum of phase times tagged [`ComputeDevice::Gpu`]; `None` if no GPU phases recorded.
    pub gpu_time_ms: Option<f64>,
    /// Completion tokens per second over decode window only.
    pub decode_tokens_per_sec: Option<f64>,
    /// End-to-end tokens per second (prompt + completion over `total_ms`).
    pub overall_tokens_per_sec: Option<f64>,
}

impl InferMetrics {
    pub fn recompute_rates(&mut self) {
        if self.decode_ms > 0.0 && self.completion_tokens > 0 {
            self.decode_tokens_per_sec =
                Some(self.completion_tokens as f64 / (self.decode_ms / 1000.0));
        }
        let total_tokens = self.prompt_tokens.saturating_add(self.completion_tokens);
        if self.total_ms > 0.0 && total_tokens > 0 {
            self.overall_tokens_per_sec = Some(total_tokens as f64 / (self.total_ms / 1000.0));
        }
    }

    pub fn add_phase_time(&mut self, device: ComputeDevice, duration_ms: f64) {
        match device {
            ComputeDevice::Cpu => self.cpu_time_ms += duration_ms,
            ComputeDevice::Gpu => {
                *self.gpu_time_ms.get_or_insert(0.0) += duration_ms;
            }
            ComputeDevice::Accelerator | ComputeDevice::Unknown => {}
        }
    }
}
