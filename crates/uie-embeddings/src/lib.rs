//! In-process **text → embedding** library for host binaries (US-033).
//!
//! Contract: [`docs/requirements/text-embeddings.md`](../../docs/requirements/text-embeddings.md).
//! CI ships [`StubTextEmbedder`]; live sentence-transformer weights are a later enablement.

mod assert;
mod config;
mod stub;
mod telemetry;
mod traits;

pub use assert::assert_embed_telemetry;
pub use config::TextEmbedderConfig;
pub use stub::{StubTextEmbedder, STUB_DIMENSIONS, STUB_MODEL_ID};
pub use telemetry::{PHASE_EMBED_ENCODE, PHASE_EMBED_VALIDATE};
pub use traits::TextEmbedder;
