use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use tracing::debug;

use crate::trace::ExecutionTrace;

pub trait TelemetrySink: Send + Sync {
    fn emit(&self, trace: &ExecutionTrace);
}

pub struct NoopSink;

impl TelemetrySink for NoopSink {
    fn emit(&self, _trace: &ExecutionTrace) {}
}

pub struct JsonlSink {
    path: PathBuf,
}

impl JsonlSink {
    pub fn from_env() -> Option<Self> {
        std::env::var("UIE_TRACE_JSONL")
            .ok()
            .map(|p| Self { path: PathBuf::from(p) })
    }

    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl TelemetrySink for JsonlSink {
    fn emit(&self, trace: &ExecutionTrace) {
        let line = match serde_json::to_string(trace) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!(error = %e, "trace serialize failed");
                return;
            }
        };
        if let Ok(mut f) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
        {
            let _ = writeln!(f, "{line}");
            debug!(path = %self.path.display(), run_id = %trace.run_id, "trace appended");
        }
    }
}

fn default_sink() -> Box<dyn TelemetrySink> {
    JsonlSink::from_env()
        .map(|s| Box::new(s) as Box<dyn TelemetrySink>)
        .unwrap_or_else(|| Box::new(NoopSink))
}

static GLOBAL_SINK: OnceLock<Mutex<Box<dyn TelemetrySink>>> = OnceLock::new();

pub fn global_sink() -> SinkGuard<'static> {
    let lock = GLOBAL_SINK.get_or_init(|| Mutex::new(default_sink()));
    SinkGuard { lock }
}

pub struct SinkGuard<'a> {
    lock: &'a Mutex<Box<dyn TelemetrySink>>,
}

impl SinkGuard<'_> {
    pub fn emit(&self, trace: &ExecutionTrace) {
        if let Ok(guard) = self.lock.lock() {
            guard.emit(trace);
        }
    }
}

pub fn set_global_sink(sink: Box<dyn TelemetrySink>) {
    let lock = GLOBAL_SINK.get_or_init(|| Mutex::new(default_sink()));
    if let Ok(mut guard) = lock.lock() {
        *guard = sink;
    }
}