use uie_core::error::Result;
use uie_telemetry::ExecutionRecorder;

/// Host-facing embedder: UTF-8 strings in, L2-normalized equal-length vectors out.
pub trait TextEmbedder: Send + Sync {
    fn model_id(&self) -> &str;
    fn dimensions(&self) -> usize;

    /// Embed an ordered batch. Empty input → empty output (no network / no encode phase work).
    /// On any failure, no vectors are returned (EMB-05).
    fn embed(
        &self,
        texts: &[String],
        telemetry: &mut ExecutionRecorder,
    ) -> Result<Vec<Vec<f32>>>;
}
