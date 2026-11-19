# US-010: Large language models and text completions

**Status:** planned  
**Personas:** Liz  
**Crate / path:** future `uie-loader-*` / `uie-llm` (not shipped)

## User story

**As** Liz, building a chatbot on top of the same engine that runs Spiro and MNIST  
**I want** to load a large language model and generate **text completions** from a **prompt**  
**So that** my bot replies in natural language through UIE—not a separate Python stack.

## In plain terms

Liz’s **concrete example:** a small chatbot CLI that:

1. Creates an [inference runtime](../glossary.md#inference-runtime) with a future LLM [model loader](./007-models-loaders.md).
2. Opens a [session](./004-runtime-sessions.md) and binds the user message as a text **prompt** [tensor](./002-tensors.md) (or a token sequence—see [token](../glossary.md#token)).
3. Calls `infer` on a **[large language model](../glossary.md#large-language-model)** to get the next **[completion](../glossary.md#completion)** (one turn of assistant text).
4. Appends the completion to chat history and repeats.

This is **text → text** [modality](./015-modality-catalog.md). It reuses the same path Jack wants for any [inference executor](../glossary.md#inference-executor); only the loader and I/O specs change.

## Related stories

- [US-001](./001-why-uie.md) — one engine for demos and models
- [US-004](./004-runtime-sessions.md) — session bindings for `prompt` / `completion`
- [US-007](./007-models-loaders.md) — pluggable loaders
- [US-015](./015-modality-catalog.md) — modality map
- [US-017](./017-multimodal-text-image-to-text.md) — chat with images later

## Terms used

- [Large language model](../glossary.md#large-language-model)
- [Completion](../glossary.md#completion)
- [Prompt](../glossary.md#prompt)
- [Chatbot](../glossary.md#chatbot)
- [Token](../glossary.md#token)
- [Modality](../glossary.md#modality)
- [Inference runtime](../glossary.md#inference-runtime)

## Acceptance notes

- **Today:** story only; no LLM loader in repo.
- **Future acceptance:** `cargo run -p uie-llm -- --prompt "Hello"` prints a completion; docs cite this story.
- **Dependencies:** tokenizer crate, KV-cache session extension ([US-004](./004-runtime-sessions.md)), likely GPU [backend](./003-backends-cpu.md) for real models.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_010_019_modalities.rs` | `cargo test -p uie-acceptance --test us_010_019_modalities us_010_llm_stub` |
