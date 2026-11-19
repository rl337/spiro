//! Example tests: extend stubs with [`RuleStubPolicy`] for input X → output Y.

use std::sync::Arc;

use uie_core::InferenceExecutor;
use uie_core::tensor::{Shape, Tensor};
use uie_telemetry::ExecutionRecorder;
use uie_stub_models::{
    infer_blank, BlankPolicy, ModalityKind, RuleStubPolicy, StubExecutor, StubMatcher, StubPolicy,
    MAX_ASR_TEXT_TOKENS, MAX_NEW_TOKENS,
};

fn token_ids(values: &[i32]) -> Tensor {
    Tensor::from_i32_slice(Shape::new([1, values.len()]), values).unwrap()
}

fn infer_stub(stub: &StubExecutor, inputs: &[Tensor]) -> Vec<Tensor> {
    let mut telem = ExecutionRecorder::new("deterministic-policy");
    stub.infer(inputs, &mut telem).unwrap()
}

#[test]
fn llm_stub_maps_prompt_tokens_to_fixed_completion() {
    let prompt = token_ids(&[100, 200, 42]);
    let expected = Tensor::from_i32_slice(Shape::new([1, MAX_NEW_TOKENS]), &[999_i32; MAX_NEW_TOKENS])
        .unwrap();

    let policy = RuleStubPolicy::new(ModalityKind::LlmTextCompletion).rule_tensors(
        StubMatcher::FirstInputTokenPrefix(vec![100, 200, 42]),
        vec![expected.clone()],
    );
    let stub = StubExecutor::with_policy(ModalityKind::LlmTextCompletion, Arc::new(policy));

    let out = infer_stub(&stub,&[prompt]);
    assert_eq!(out[0].as_i32_slice().unwrap()[0], 999);

    let other = token_ids(&[1, 2, 3]);
    let default = infer_stub(&stub,&[other]);
    assert_eq!(default[0].as_i32_slice().unwrap()[0], 0);
}

#[test]
fn judge_stub_fail_label_for_toxic_prefix() {
    let toxic = token_ids(&[666, 666]);
    let policy = RuleStubPolicy::new(ModalityKind::Judge).rule(
        StubMatcher::FirstInputTokenPrefix(vec![666, 666]),
        |_| {
            Ok(vec![
                Tensor::from_f32_slice(Shape::new([1, 2]), &[0.1, 0.9]).unwrap(),
                Tensor::from_i32_slice(Shape::new([1, 1]), &[1]).unwrap(),
            ])
        },
    );
    let stub = StubExecutor::with_policy(ModalityKind::Judge, Arc::new(policy));
    let out = infer_stub(&stub,&[toxic]);
    assert_eq!(out[1].as_i32_slice().unwrap()[0], 1);
}

#[test]
fn diffusion_stub_white_pixel_from_marker_embedding() {
    let embed = {
        let n = 77 * 768;
        let mut vals = vec![0.0f32; n];
        vals[0] = 0.42;
        Tensor::from_f32_slice(Shape::new([1, 77, 768]), &vals).unwrap()
    };
    let policy = RuleStubPolicy::new(ModalityKind::DiffusionTextToImage).rule(
        StubMatcher::FirstPixelF32(0.42),
        |inputs| {
            let mut img = infer_blank(ModalityKind::DiffusionTextToImage, inputs)?;
            let mut patched = img[0].as_f32_slice().unwrap().to_vec();
            patched[0] = 1.0;
            img[0] = Tensor::from_f32_slice(img[0].shape().clone(), &patched);
            Ok(img)
        },
    );
    let stub = StubExecutor::with_policy(ModalityKind::DiffusionTextToImage, Arc::new(policy));
    let out = infer_stub(&stub,&[embed]);
    assert_eq!(out[0].as_f32_slice().unwrap()[0], 1.0);
}

#[test]
fn stt_stub_transcript_for_whistle_prefix() {
    let audio = Tensor::from_f32_slice(Shape::new([1, 160]), &[0.88f32; 160]);
    let mut ids = vec![0_i32; MAX_ASR_TEXT_TOKENS];
    ids[..3].copy_from_slice(&[50, 51, 52]);
    let policy = RuleStubPolicy::new(ModalityKind::SpeechToText).rule_tensors(
        StubMatcher::FirstInputF32Prefix(vec![0.88]),
        vec![Tensor::from_i32_slice(Shape::new([1, MAX_ASR_TEXT_TOKENS]), &ids).unwrap()],
    );
    let stub = StubExecutor::with_policy(ModalityKind::SpeechToText, Arc::new(policy));
    let out = infer_stub(&stub,&[audio]);
    assert_eq!(&out[0].as_i32_slice().unwrap()[..3], &[50, 51, 52]);
}

#[test]
fn tts_stub_square_wave_for_magic_tokens() {
    let tokens = token_ids(&[7, 7, 7]);
    let policy = RuleStubPolicy::new(ModalityKind::TextToSpeech).rule(
        StubMatcher::FirstInputTokenPrefix(vec![7, 7, 7]),
        |_| {
            let n = uie_stub_models::TTS_OUTPUT_SAMPLES;
            let wave: Vec<f32> = (0..n).map(|i| if i % 2 == 0 { 0.5 } else { -0.5 }).collect();
            Ok(vec![Tensor::from_f32_slice(Shape::new([1, n]), &wave).unwrap()])
        },
    );
    let stub = StubExecutor::with_policy(ModalityKind::TextToSpeech, Arc::new(policy));
    let out = infer_stub(&stub,&[tokens]);
    assert_eq!(out[0].as_f32_slice().unwrap()[0], 0.5);
}

#[test]
fn image_to_text_stub_caption_for_bright_pixel() {
    let mut px = vec![0.0f32; 3 * 224 * 224];
    px[0] = 0.99;
    let image = Tensor::from_f32_slice(Shape::new([1, 3, 224, 224]), &px);
    let policy = RuleStubPolicy::new(ModalityKind::ImageToText).rule_tensors(
        StubMatcher::FirstPixelF32(0.99),
        vec![token_ids(&[42, 43])],
    );
    let stub = StubExecutor::with_policy(ModalityKind::ImageToText, Arc::new(policy));
    let out = infer_stub(&stub,&[image]);
    assert_eq!(out[0].as_i32_slice().unwrap()[0], 42);
}

#[test]
fn vlm_stub_answer_when_prompt_prefix_matches() {
    let prompt = token_ids(&[1000, 1001]);
    let mut px = vec![0.0f32; 3 * 336 * 336];
    let image = Tensor::from_f32_slice(Shape::new([1, 3, 336, 336]), &px);
    let policy = RuleStubPolicy::new(ModalityKind::MultimodalTextImageToText).rule_tensors(
        StubMatcher::FirstInputTokenPrefix(vec![1000, 1001]),
        vec![token_ids(&[88, 89])],
    );
    let stub =
        StubExecutor::with_policy(ModalityKind::MultimodalTextImageToText, Arc::new(policy));
    let out = infer_stub(&stub,&[prompt, image]);
    assert_eq!(out[0].as_i32_slice().unwrap()[0], 88);
}

#[test]
fn audio_to_audio_stub_copy_input_with_gain() {
    let audio = Tensor::from_f32_slice(Shape::new([1, 4]), &[0.1, 0.2, 0.3, 0.4]);
    let policy = RuleStubPolicy::new(ModalityKind::AudioToAudio).rule(
        StubMatcher::Always,
        |inputs| {
            let slice = inputs[0].as_f32_slice();
            let boosted: Vec<f32> = slice.iter().map(|v| v * 2.0).collect();
            Ok(vec![Tensor::from_f32_slice(inputs[0].shape().clone(), &boosted).unwrap()])
        },
    );
    let stub = StubExecutor::with_policy(ModalityKind::AudioToAudio, Arc::new(policy));
    let out = infer_stub(&stub,&[audio]);
    assert_eq!(out[0].as_f32_slice().unwrap(), &[0.2, 0.4, 0.6, 0.8]);
}

#[test]
fn image_to_image_stub_invert_first_channel() {
    let mut px = vec![0.25f32; 3 * 512 * 512];
    let image = Tensor::from_f32_slice(Shape::new([1, 3, 512, 512]), &px);
    let policy = RuleStubPolicy::new(ModalityKind::ImageToImage).rule(
        StubMatcher::FirstPixelF32(0.25),
        |inputs| {
            let mut out = infer_blank(ModalityKind::ImageToImage, inputs)?;
            let mut inv = out[0].as_f32_slice().unwrap().to_vec();
            inv[0] = 1.0 - inv[0];
            out[0] = Tensor::from_f32_slice(out[0].shape().clone(), &inv);
            Ok(out)
        },
    );
    let stub = StubExecutor::with_policy(ModalityKind::ImageToImage, Arc::new(policy));
    let out = infer_stub(&stub,&[image]);
    assert_eq!(out[0].as_f32_slice().unwrap()[0], 1.0);
}

#[test]
fn blank_policy_matches_infer_blank() {
    let kind = ModalityKind::LlmTextCompletion;
    let prompt = token_ids(&[3, 4, 5]);
    let a = infer_blank(kind, &[prompt.clone()]);
    let b = BlankPolicy.infer(kind, &[prompt]);
    assert_eq!(a[0].as_i32_slice().unwrap(), b[0].as_i32_slice().unwrap());
}