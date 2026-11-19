//! Tensor layouts aligned with common production models (drop-in for real loaders later).

/// GPT-2–style BPE vocabulary (OpenAI reference size).
pub const VOCAB_SIZE: usize = 50_257;
pub const PAD_TOKEN_ID: i32 = 0;
pub const EOS_TOKEN_ID: i32 = 2;

/// LLM / judge context (tokens).
pub const MAX_CONTEXT_TOKENS: usize = 2048;
pub const MAX_NEW_TOKENS: usize = 128;
pub const MAX_PROMPT_TOKENS: usize = 77;

/// ASR / STT (Whisper-style 16 kHz mono PCM).
pub const SAMPLE_RATE_ASR: usize = 16_000;
pub const MAX_AUDIO_SECONDS_ASR: usize = 30;
pub const MAX_ASR_SAMPLES: usize = SAMPLE_RATE_ASR * MAX_AUDIO_SECONDS_ASR;
pub const MAX_ASR_TEXT_TOKENS: usize = 256;

/// TTS output (22.05 kHz is common for speech codecs).
pub const SAMPLE_RATE_TTS: usize = 22_050;
pub const TTS_MAX_DURATION_SEC: usize = 5;
pub const TTS_OUTPUT_SAMPLES: usize = SAMPLE_RATE_TTS * TTS_MAX_DURATION_SEC;

/// CLIP text encoder (Stable Diffusion 1.x).
pub const CLIP_SEQ_LEN: usize = 77;
pub const CLIP_HIDDEN_DIM: usize = 768;

/// Generated RGB in VAE decode space (NCHW, normalized floats).
pub const DIFFUSION_HEIGHT: usize = 512;
pub const DIFFUSION_WIDTH: usize = 512;

/// Vision encoders (CLIP ViT / LLaVA-ish).
pub const CLIP_IMAGE_SIZE: usize = 224;
pub const VLM_IMAGE_SIZE: usize = 336;

/// Binary judge head (pass/fail logits).
pub const JUDGE_NUM_LABELS: usize = 2;
