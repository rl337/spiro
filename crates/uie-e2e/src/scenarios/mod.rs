pub mod mock_registry;
pub mod modality_live_smoke;

#[cfg(feature = "gguf")]
pub mod qwen3_2b;

#[cfg(feature = "gguf")]
pub mod cat_t2i_vl_yes;
#[cfg(feature = "gguf")]
pub mod vl_cat_chat;

#[cfg(feature = "safetensors")]
pub mod spiro_native_chain;
