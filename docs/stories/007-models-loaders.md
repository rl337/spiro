# US-007: Models, loaders, and registries

**Status:** partial (traits shipped; ONNX loader integrating)  
**Crate / path:** `crates/uie-core/src/model.rs`

## User story

**As a** user with a model file on disk  
**I want** the runtime to pick the right loader and run the model  
**So that** I add new formats in separate crates without editing the core.

## In plain terms

UIE splits “models” into two styles:

| Style | Trait | Typical use |
|-------|--------|-------------|
| **Plan-based** | [Inference model](../glossary.md#inference-model) | Expose an [execution plan](./005-execution-plans.md); run via `run_model`. |
| **End-to-end** | [Inference executor](../glossary.md#inference-executor) | One `infer(inputs)` call; used by ONNX today. |

A [model loader](../glossary.md#model-loader):

1. Declares its `format_id` (for example `"onnx"`).
2. Answers `can_load(path)` (often by file extension).
3. Returns a boxed **inference executor**.

The [model loader registry](../glossary.md#model-loader-registry) on the [runtime](./004-runtime-sessions.md) tries each loader until one matches. [I/O specs](../glossary.md#io-spec) describe expected input/output tensors after load.

## Related stories

- [US-004 — Runtime](./004-runtime-sessions.md) (`load_model`, `infer`)
- [US-009 — ONNX MNIST](./009-onnx-mnist.md) (first real loader)
- [US-005 — Plans](./005-execution-plans.md) (plan-style models)

## Terms used

- [Model loader](../glossary.md#model-loader)
- [Model loader registry](../glossary.md#model-loader-registry)
- [Inference executor](../glossary.md#inference-executor)
- [Inference model](../glossary.md#inference-model)
- [I/O spec](../glossary.md#io-spec)

## Acceptance notes

- `ModelLoader`, `InferenceExecutor`, `IoSpec`, `ModelLoaderRegistry`, `ModelSource`.
- `InferenceRuntime::with_loader` + `load_model`.
- ONNX implementation tracked in [US-009](./009-onnx-mnist.md).