//! Chain multiple [**inference executors**](../uie-core) through the same [**byte channel**](../uie-ipc) framing.

mod byte_pipeline;
mod comfy_stack;

pub use byte_pipeline::{ByteChannelPipelineRunner, PipelineHop};
pub use comfy_stack::{build_comfy_byte_pipeline, ComfyPipelinePlan};