use std::path::PathBuf;

/// Operator knobs owned by the embedder injectable (EMB-08 / DI-01).
#[derive(Debug, Clone)]
pub struct TextEmbedderConfig {
    /// Bundle directory (weights + tokenizer). Unused by the CI stub.
    pub model_dir: Option<PathBuf>,
    /// Expected output dimension; stub must match, live load must match.
    pub expected_dimensions: usize,
    /// Intra-op thread hint for CPU backends (1–4 recommended on Pi).
    pub threads: usize,
    /// Soft character budget before stub/live truncation (hub `CHUNK_MAX_CHARS`).
    pub max_chars: usize,
}

impl Default for TextEmbedderConfig {
    fn default() -> Self {
        Self {
            model_dir: None,
            expected_dimensions: crate::stub::STUB_DIMENSIONS,
            threads: 2,
            max_chars: 1200,
        }
    }
}

impl TextEmbedderConfig {
    /// Stub-only config (no weight directory).
    pub fn stub() -> Self {
        Self::default()
    }
}
