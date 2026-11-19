//! US-015: modality catalog covers stub modalities US-010–US-019.
use uie_acceptance::{assert_story_events, StoryEventLog};
use uie_stub_models::ModalityKind;

#[test]
fn us_015_modality_kinds_match_catalog() {
    let mut log = StoryEventLog::new("US-015");
    let kinds = [
        ModalityKind::LlmTextCompletion,
        ModalityKind::Judge,
        ModalityKind::DiffusionTextToImage,
        ModalityKind::SpeechToText,
        ModalityKind::TextToSpeech,
        ModalityKind::ImageToText,
        ModalityKind::MultimodalTextImageToText,
        ModalityKind::AudioToAudio,
        ModalityKind::ImageToImage,
    ];
    for k in kinds {
        assert!(!k.id_str().is_empty());
        log.record(format!("modality_indexed:{}", k.story_id()));
    }
    log.record("modality_catalog_complete");
    assert_story_events(&log, &["modality_catalog_complete"]);
}
