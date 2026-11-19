# US-004: Runtime and named sessions

**Status:** shipped  
**Personas:** Liz  
**Crate / path:** `crates/uie-core/src/runtime.rs`

## User story

**As** Liz, orchestrating a run in the repo she maintains  
**I want** a runtime that owns the backend and a session that holds named tensors  
**So that** I can load a model, bind `"input"`, and fetch `"output"` without manual pointer juggling.

## In plain terms

The [inference runtime](../glossary.md#inference-runtime) is the object you construct with a [backend](./003-backends-cpu.md):

1. Optionally **register** [model loaders](./007-models-loaders.md).
2. **Create a session**—a map from string names to [tensors](./002-tensors.md).
3. **Bind** tensors (`session.bind("canvas", tensor)`).
4. Either run an [execution plan](./005-execution-plans.md) step by step or call **direct inference** on an [inference executor](../glossary.md#inference-executor) (ONNX path).

Sessions survive across steps in one logical “run.” Outputs from `infer` can be stored back into the session under each [I/O spec](../glossary.md#io-spec) name.

## Related stories

- [US-003 — Backends](./003-backends-cpu.md)
- [US-005 — Execution plans](./005-execution-plans.md)
- [US-007 — Loaders](./007-models-loaders.md)
- [US-008 — Spiro](./008-spiro-demo.md) binds `canvas`
- [US-009 — MNIST](./009-onnx-mnist.md) binds `input`

## Terms used

- [Inference runtime](../glossary.md#inference-runtime)
- [Session](../glossary.md#session)
- [Model loader registry](../glossary.md#model-loader-registry)

## Acceptance notes

- `InferenceRuntime::new`, `with_loader`, `create_session`, `load_model`, `infer`, `run_model`.
- `Session::bind`, `get`, `take`, `allocate`.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_004_runtime_session.rs` | `cargo test -p uie-acceptance --test us_004_runtime_session` |
