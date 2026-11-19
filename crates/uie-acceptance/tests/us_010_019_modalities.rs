//! US-010 through US-019: stub tensor contracts per modality.
mod support;

use uie_core::backend::cpu::CpuBackend;
use uie_core::InferenceRuntime;
use uie_stub_models::{sample_input, ModalityKind, StubModelLoader};

fn infer_stub(kind: ModalityKind, stub_file: &str) {
    let rt = InferenceRuntime::new(Box::new(CpuBackend::new()))
        .with_loader(Box::new(StubModelLoader::default()));
    let path = support::stubs_dir().join(stub_file);
    let model = rt.load_model(&path).unwrap();
    let inputs = sample_input(kind).unwrap();
    let mut session = rt.create_session();
    let outputs = rt.infer(model.as_ref(), &inputs, &mut session).unwrap();
    assert!(!outputs.is_empty());
}

#[test]
fn us_010_llm_stub() {
    infer_stub(ModalityKind::LlmTextCompletion, "llm.stub");
}

#[test]
fn us_011_judge_stub() {
    infer_stub(ModalityKind::Judge, "judge.stub");
}

#[test]
fn us_012_diffusion_stub() {
    infer_stub(ModalityKind::DiffusionTextToImage, "diffusion_t2i.stub");
}

#[test]
fn us_013_stt_stub() {
    infer_stub(ModalityKind::SpeechToText, "stt.stub");
}

#[test]
fn us_014_tts_stub() {
    infer_stub(ModalityKind::TextToSpeech, "tts.stub");
}

#[test]
fn us_016_image_to_text_stub() {
    infer_stub(ModalityKind::ImageToText, "image_to_text.stub");
}

#[test]
fn us_017_vlm_stub() {
    infer_stub(ModalityKind::MultimodalTextImageToText, "vlm.stub");
}

#[test]
fn us_018_audio_to_audio_stub() {
    infer_stub(ModalityKind::AudioToAudio, "audio_to_audio.stub");
}

#[test]
fn us_019_image_to_image_stub() {
    infer_stub(ModalityKind::ImageToImage, "image_to_image.stub");
}