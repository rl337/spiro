# US-013: Speech-to-text (audio-to-text)

**Status:** planned  
**Personas:** Kenneth, Pete  
**Crate / path:** future `uie-asr` (not shipped)

## User story

**As** Kenneth, transcribing a voice memo—and **as** Pete, who needs it on a laptop without cloud APIs  
**I want** an audio model that converts recorded speech into text  
**So that** UIE handles **speech-to-text** with the same runtime as everything else.

## In plain terms

**Audio → text** ([modality](./015-modality-catalog.md)):

1. Load WAV/PCM into a [tensor](./002-tensors.md) (for example `F32` waveform or log-mel features).
2. Run an ASR [inference executor](../glossary.md#inference-executor) (Whisper-style ONNX, etc.).
3. Bind output text or token IDs in a [session](./004-runtime-sessions.md).
4. Optionally chain to [US-010](./010-llm-text-completions.md) for summarization.

Pete cares about batch length limits and CPU fallback; Kenneth starts with a 10-second clip and a tiny model.

## Related stories

- [US-002](./002-tensors.md)
- [US-007](./007-models-loaders.md)
- [US-014](./014-text-to-speech.md) — opposite direction
- [US-015](./015-modality-catalog.md)

## Terms used

- [Speech-to-text](../glossary.md#speech-to-text)
- [Modality](../glossary.md#modality)
- [Tensor](../glossary.md#tensor)

## Acceptance notes

- **Today:** story only.
- **Future acceptance:** test fixture WAV → expected transcript string within edit distance.
- New [I/O spec](../glossary.md#io-spec) for sample rate and channel count required.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_010_019_modalities.rs` | `cargo test -p uie-acceptance --test us_010_019_modalities us_013_stt_stub` |
