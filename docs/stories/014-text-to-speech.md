# US-014: Text-to-speech (text-to-audio)

**Status:** planned  
**Personas:** Jenna, Kenneth  
**Crate / path:** future `uie-tts` (not shipped)

## User story

**As** Jenna, who wants the chatbot to speak—and **as** Kenneth, testing on headphones  
**I want** a model that turns written text into playable audio  
**So that** UIE covers **text-to-speech** for accessibility and demos.

## In plain terms

**Text → audio**:

1. Bind user or [LLM](../glossary.md#large-language-model) **completion](./010-llm-text-completions.md) text as input.
2. TTS model `infer` returns waveform or mel [tensor](./002-tensors.md).
3. Write WAV/MP3 like Spiro writes JPEG ([US-008](./008-spiro-demo.md)).

Pairs with [US-013](./013-speech-to-text.md) for round-trip tests and with Liz’s chatbot for spoken replies.

## Related stories

- [US-010](./010-llm-text-completions.md)
- [US-013](./013-speech-to-text.md)
- [US-015](./015-modality-catalog.md)
- [US-004](./004-runtime-sessions.md)

## Terms used

- [Text-to-speech](../glossary.md#text-to-speech)
- [Completion](../glossary.md#completion)
- [Modality](../glossary.md#modality)

## Acceptance notes

- **Today:** story only.
- **Future acceptance:** CLI writes `out.wav` under one second for a short phrase on CPU (tiny model).

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_010_019_modalities.rs` | `cargo test -p uie-acceptance --test us_010_019_modalities us_014_tts_stub` |
