# US-015: Modality catalog (input → output map)

**Status:** planned (documentation shipped; implementations vary)  
**Personas:** Liz, Jack  
**Crate / path:** `docs/` (organizing story); implementations per linked US-*

## User story

**As** Liz, maintaining the engine—and **as** Jack, planning which loaders to fund  
**I want** one catalog of **modalities** and how UIE stories map to them  
**So that** any input/output pair has a named user story instead of vague “multimodal” talk.

## In plain terms

A **modality** is the kind of data crossing the boundary: **text**, **image**, or **audio** (extend later for video). UIE expresses every crossing as [tensors](./002-tensors.md) plus an [inference executor](../glossary.md#inference-executor).

| Input → Output | Story | Personas | Status |
|----------------|-------|----------|--------|
| Text → Text | [US-010](./010-llm-text-completions.md) | Liz | planned |
| Text → Text (score) | [US-011](./011-judge-system-one-models.md) | Jack, Toofer | planned |
| Text → Image | [US-012](./012-diffusion-text-to-image.md) | Tracy, Jenna | planned |
| Audio → Text | [US-013](./013-speech-to-text.md) | Kenneth, Pete | planned |
| Text → Audio | [US-014](./014-text-to-speech.md) | Jenna, Kenneth | planned |
| Image → Text | [US-016](./016-image-to-text.md) | Frank | planned |
| Text + Image → Text | [US-017](./017-multimodal-text-image-to-text.md) | Liz | planned |
| Image → Image | [US-019](./019-image-to-image.md) | Tracy | planned |
| Audio → Audio | [US-018](./018-audio-to-audio.md) | Pete | planned |
| Image → Label (classify) | [US-009](./009-onnx-mnist.md) | Liz, Kenneth | partial |

**Concrete chatbot thread (Liz):** [US-010](./010-llm-text-completions.md) on the Spiro/UIE [runtime](./004-runtime-sessions.md); optional [US-011](./011-judge-system-one-models.md) for safety; optional [US-014](./014-text-to-speech.md) for voice.

## Related stories

- [US-001](./001-why-uie.md) through [US-009](./009-onnx-mnist.md) — foundation
- All US-010 – US-019 — modality implementations

## Terms used

- [Modality](../glossary.md#modality)
- [Tensor](../glossary.md#tensor)
- [UIE](../glossary.md#uie)

## Acceptance notes

- This file stays the index of modality stories; update the table when adding US-020+.
- Shipped acceptance for the catalog itself: links resolve in `docs/stories/`.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_015_modality_catalog.rs` | `cargo test -p uie-acceptance --test us_015_modality_catalog` |
| Rust | `crates/uie-acceptance/tests/us_010_019_modalities.rs` | modality stub contracts US-010–019 |
