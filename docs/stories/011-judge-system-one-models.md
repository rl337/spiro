# US-011: System-one and judge models

**Status:** planned  
**Personas:** Jack, Toofer  
**Crate / path:** future `uie-judge` (not shipped)

## User story

**As** Jack, who needs quality gates—and **as** Toofer, who wants rubrics written down  
**I want** a **system-one** or **judge** model that scores or classifies another model’s output  
**So that** UIE can run “fast worker + careful judge” flows without ad-hoc scripts.

## In plain terms

A **judge** (or system-one) model is usually **text → text** or **text → labels**: it reads a candidate answer and returns pass/fail, a score, or JSON rubric fields.

Typical flow:

1. [US-010](./010-llm-text-completions.md) produces a draft **completion**.
2. Liz binds that text into a judge **prompt** in the same [session](./004-runtime-sessions.md).
3. A smaller, deterministic **judge model** runs via `infer` (often ONNX or a tiny LLM).
4. Jack stores the verdict for logging or retries.

**System-one** here means the slower, deliberate pass—the name contrasts with a fast “system-zero” generator (see [judge model](../glossary.md#judge-model) in the glossary).

## Related stories

- [US-010](./010-llm-text-completions.md) — generator side of the loop
- [US-007](./007-models-loaders.md) — second loader for judge weights
- [US-009](./009-onnx-mnist.md) — pattern for small classification-style judges
- [US-015](./015-modality-catalog.md)

## Terms used

- [Judge model](../glossary.md#judge-model)
- [System-one model](../glossary.md#system-one-model)
- [Completion](../glossary.md#completion)
- [Prompt](../glossary.md#prompt)

## Acceptance notes

- **Today:** story only.
- **Future acceptance:** integration test: fixed prompt → judge returns expected label tensor / string.
- Judges may share [US-009](./009-onnx-mnist.md) loader path when exported as ONNX.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_010_019_modalities.rs` | `cargo test -p uie-acceptance --test us_010_019_modalities us_011_judge_stub` |
