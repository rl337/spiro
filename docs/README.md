# UIE documentation

The **Universal Inference Engine (UIE)** is this repo’s Rust engine for running models and demo workloads. These docs explain it in plain language.

## Start here

| Doc | Purpose |
|-----|---------|
| [Glossary](./glossary.md) | Short definitions for every bold term used in stories |
| [Personas](./personas.md) | 30 Rock first names used in stories (Liz, Jack, …) |
| [Story conventions](./story-conventions.md) | Story IDs, linking rules, how to add a new story |
| [Acceptance tests](./acceptance-tests.md) | One automated test (Rust or Python) per `US-0NN` |
| [User stories index](./stories/README.md) | All scenarios, in learning order |

## Learning path

1. [US-001 — Why UIE exists](./stories/001-why-uie.md)
2. [US-002 — Tensors](./stories/002-tensors.md)
3. [US-003 — Backends and CPU](./stories/003-backends-cpu.md)
4. [US-004 — Runtime and sessions](./stories/004-runtime-sessions.md)
5. [US-005 — Execution plans](./stories/005-execution-plans.md)
6. [US-006 — Operations](./stories/006-operations.md)
7. [US-007 — Models and loaders](./stories/007-models-loaders.md)
8. [US-008 — Spirograph demo](./stories/008-spiro-demo.md)
9. [US-009 — ONNX MNIST (in progress)](./stories/009-onnx-mnist.md)

### Modalities (planned)

Use [US-015 — Modality catalog](./stories/015-modality-catalog.md) as the hub, then US-010 – US-019 for LLM chatbots, judges, diffusion, speech, and multimodal I/O.

## Code map

| Crate | Stories |
|-------|---------|
| `crates/uie-core` | US-002 – US-007 |
| `crates/uie-spiro` | US-008 |
| `crates/uie-loader-onnx`, `crates/uie-mnist` | US-009 |
| `spiro.py` | US-008 (original reference) |
| (planned) | US-010 – US-019 modality stories |