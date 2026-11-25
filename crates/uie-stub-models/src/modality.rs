use std::path::Path;
use std::str::FromStr;

use uie_core::error::{Error, Result};

/// Modality covered by stub user stories (US-010 – US-019, US-032).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModalityKind {
    /// US-010: text → text (LLM completion).
    LlmTextCompletion,
    /// US-011: judge / system-one.
    Judge,
    /// US-012: text → image (diffusion).
    DiffusionTextToImage,
    /// US-013: audio → text (ASR).
    SpeechToText,
    /// US-014: text → audio (TTS).
    TextToSpeech,
    /// US-016: image → text.
    ImageToText,
    /// US-017: text + image → text.
    MultimodalTextImageToText,
    /// US-018: audio → audio.
    AudioToAudio,
    /// US-019: image → image.
    ImageToImage,
    /// US-032: image + structured JSON filters → image (img2img-shaped stub).
    ImageFilter,
}

impl ModalityKind {
    pub const ALL: [Self; 10] = [
        Self::LlmTextCompletion,
        Self::Judge,
        Self::DiffusionTextToImage,
        Self::SpeechToText,
        Self::TextToSpeech,
        Self::ImageToText,
        Self::MultimodalTextImageToText,
        Self::AudioToAudio,
        Self::ImageToImage,
        Self::ImageFilter,
    ];

    pub fn story_id(self) -> &'static str {
        match self {
            Self::LlmTextCompletion => "US-010",
            Self::Judge => "US-011",
            Self::DiffusionTextToImage => "US-012",
            Self::SpeechToText => "US-013",
            Self::TextToSpeech => "US-014",
            Self::ImageToText => "US-016",
            Self::MultimodalTextImageToText => "US-017",
            Self::AudioToAudio => "US-018",
            Self::ImageToImage => "US-019",
            Self::ImageFilter => "US-032",
        }
    }

    pub fn id_str(self) -> &'static str {
        match self {
            Self::LlmTextCompletion => "llm",
            Self::Judge => "judge",
            Self::DiffusionTextToImage => "diffusion_t2i",
            Self::SpeechToText => "stt",
            Self::TextToSpeech => "tts",
            Self::ImageToText => "image_to_text",
            Self::MultimodalTextImageToText => "vlm",
            Self::AudioToAudio => "audio_to_audio",
            Self::ImageToImage => "image_to_image",
            Self::ImageFilter => "image_filter",
        }
    }

    pub fn parse_file(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path).map_err(|e| Error::ModelLoad(e.to_string()))?;
        let line = raw
            .lines()
            .find(|l| {
                let t = l.trim();
                !t.is_empty() && !t.starts_with('#')
            })
            .ok_or_else(|| Error::ModelLoad("empty .stub manifest".into()))?
            .trim();
        Self::from_str(line)
    }
}

impl FromStr for ModalityKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "llm" | "llm_text_completion" | "text_completion" => Ok(Self::LlmTextCompletion),
            "judge" | "system_one" | "system-one" => Ok(Self::Judge),
            "diffusion" | "diffusion_t2i" | "text_to_image" | "t2i" => {
                Ok(Self::DiffusionTextToImage)
            }
            "stt" | "speech_to_text" | "asr" => Ok(Self::SpeechToText),
            "tts" | "text_to_speech" => Ok(Self::TextToSpeech),
            "image_to_text" | "caption" | "i2t" => Ok(Self::ImageToText),
            "vlm" | "multimodal" | "text_image_to_text" => Ok(Self::MultimodalTextImageToText),
            "audio_to_audio" | "a2a" => Ok(Self::AudioToAudio),
            "image_to_image" | "i2i" | "img2img" => Ok(Self::ImageToImage),
            "image_filter" | "filter" | "i2i_filter" => Ok(Self::ImageFilter),
            other => Err(Error::ModelLoad(format!("unknown stub modality `{other}`"))),
        }
    }
}
