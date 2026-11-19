# US-018: Audio-to-audio (enhancement and conversion)

**Status:** planned  
**Personas:** Pete  
**Crate / path:** future `uie-audio-fx` (not shipped)

## User story

**As** Pete, cleaning up a bad take before air  
**I want** a model that maps one audio clip to another  
**So that** UIE supports **audio-to-audio** without bouncing through text.

## In plain terms

**Audio → audio** examples:

- Denoise / enhance podcast audio
- Voice conversion (keep words, change timbre)
- Simple style transfer on short clips

Flow mirrors [US-013](./013-speech-to-text.md) input binding, but output is waveform [tensor](./002-tensors.md) like [US-014](./014-text-to-speech.md). Can chain **STT → LLM → TTS** instead when text editing is enough ([US-015](./015-modality-catalog.md)).

## Related stories

- [US-013](./013-speech-to-text.md)
- [US-014](./014-text-to-speech.md)
- [US-015](./015-modality-catalog.md)

## Terms used

- [Modality](../glossary.md#modality)
- [Tensor](../glossary.md#tensor)

## Acceptance notes

- **Today:** story only.
- **Future acceptance:** SNR or duration preserved on fixture clip in integration test.