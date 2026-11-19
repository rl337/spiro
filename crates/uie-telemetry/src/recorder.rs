use std::time::Instant;

use crate::device::ComputeDevice;
use crate::metrics::InferMetrics;
use crate::sink::{global_sink, TelemetrySink};
use crate::trace::{ExecutionTrace, PhaseSpan};

#[derive(Debug, Clone)]
pub struct ExecutionRecorder {
    run_id: String,
    model_name: Option<String>,
    model_format: Option<String>,
    backend_id: Option<String>,
    transport: Option<String>,
    started: Instant,
    phases: Vec<PhaseSpan>,
    metrics: InferMetrics,
}

impl ExecutionRecorder {
    pub fn new(run_id: impl Into<String>) -> Self {
        Self {
            run_id: run_id.into(),
            model_name: None,
            model_format: None,
            backend_id: None,
            transport: None,
            started: Instant::now(),
            phases: Vec::new(),
            metrics: InferMetrics::default(),
        }
    }

    pub fn with_model(mut self, name: impl Into<String>, format: impl Into<String>) -> Self {
        self.model_name = Some(name.into());
        self.model_format = Some(format.into());
        self
    }

    pub fn with_backend(mut self, id: impl Into<String>) -> Self {
        self.backend_id = Some(id.into());
        self
    }

    pub fn with_transport(mut self, t: impl Into<String>) -> Self {
        self.transport = Some(t.into());
        self
    }

    pub fn metrics_mut(&mut self) -> &mut InferMetrics {
        &mut self.metrics
    }

    pub fn record_phase_ms(&mut self, name: impl Into<String>, device: ComputeDevice, duration_ms: f64) {
        self.metrics.add_phase_time(device, duration_ms);
        self.phases.push(PhaseSpan {
            name: name.into(),
            device,
            duration_ms,
        });
    }

    /// Time a closure and record a phase span.
    pub fn phase<R>(
        &mut self,
        name: impl Into<String>,
        device: ComputeDevice,
        f: impl FnOnce() -> R,
    ) -> R {
        let t0 = Instant::now();
        let out = f();
        let ms = t0.elapsed().as_secs_f64() * 1000.0;
        self.record_phase_ms(name, device, ms);
        out
    }

    pub fn finish(mut self) -> ExecutionTrace {
        self.metrics.total_ms = self.started.elapsed().as_secs_f64() * 1000.0;
        self.metrics.recompute_rates();
        let trace = ExecutionTrace {
            run_id: self.run_id,
            model_name: self.model_name,
            model_format: self.model_format,
            backend_id: self.backend_id,
            transport: self.transport,
            phases: self.phases,
            metrics: self.metrics,
        };
        global_sink().emit(&trace);
        trace
    }
}