# User stories index

Each story has an ID (`US-*`) so you can cite it in issues and PRs. Terms link to the [glossary](../glossary.md). Personas are [30 Rock first names](../personas.md).

Product mandate ([docs/mandate.md](../mandate.md)): good inference **at any scale**; when we must compromise, prefer **edge/embedded** performance.

## Foundation (shipped / partial)

| ID | Title | Personas | Status |
|----|--------|----------|--------|
| [US-001](./001-why-uie.md) | Why UIE exists | Liz, Jack | shipped |
| [US-002](./002-tensors.md) | Representing data as tensors | Kenneth | shipped |
| [US-003](./003-backends-cpu.md) | Running compute on the CPU backend | Pete | shipped (minimal ops) |
| [US-004](./004-runtime-sessions.md) | Runtime and named sessions | Liz | shipped |
| [US-005](./005-execution-plans.md) | Execution plans from graphs | Frank | shipped (scaffold) |
| [US-006](./006-operations.md) | Operations in the graph | Toofer | shipped (scaffold) |
| [US-007](./007-models-loaders.md) | Models, loaders, and registries | Jack, Liz | shipped |
| [US-008](./008-spiro-demo.md) | Spirograph demo workload | Tracy, Jenna | shipped |
| [US-009](./009-onnx-mnist.md) | ONNX MNIST classification | Liz, Kenneth | shipped |
| [US-010](./010-llm-text-completions.md) | LLM text completions & chatbot | Liz | shipped |

## Modalities (planned)

Start with [US-015](./015-modality-catalog.md) for the input → output map.

| ID | Title | Personas | Modality |
|----|--------|----------|----------|
| [US-011](./011-judge-system-one-models.md) | System-one / judge models | Jack, Toofer | Text → Text / labels |
| [US-012](./012-diffusion-text-to-image.md) | Diffusion text-to-image | Tracy, Jenna | Text → Image |
| [US-013](./013-speech-to-text.md) | Speech-to-text | Kenneth, Pete | Audio → Text |
| [US-014](./014-text-to-speech.md) | Text-to-speech | Jenna, Kenneth | Text → Audio |
| [US-015](./015-modality-catalog.md) | Modality catalog | Liz, Jack | (hub) |
| [US-016](./016-image-to-text.md) | Image-to-text | Frank | Image → Text |
| [US-017](./017-multimodal-text-image-to-text.md) | Multimodal chat | Liz | Text + Image → Text |
| [US-018](./018-audio-to-audio.md) | Audio-to-audio | Pete | Audio → Audio |
| [US-019](./019-image-to-image.md) | Image-to-image / inpainting | Tracy | Image → Image |

## Platform (shipped)

| ID | Title | Personas | Status |
|----|--------|----------|--------|
| [US-020](./020-inference-byte-channel.md) | Inference byte channel | Liz, Pete | shipped |
| [US-021](./021-model-registry-scan.md) | Model roots & registry scan | Jack, Liz | shipped |
| [US-022](./022-byte-channel-pipeline.md) | Multi-hop byte channel pipeline | Tracy, Pete | shipped |
| [US-023](./023-inference-telemetry.md) | Execution telemetry (TTFT, throughput) | Pete, Jack | shipped |
| [US-024](./024-dependency-injection.md) | Dependency injection (uie-iv / pyiv-style) | Jack, Pete | shipped |
| [US-026](./026-browser-wasm-inference.md) | Browser WASM inference (ONNX MNIST) | Pete, Liz | shipped |

## Edge resources & measurement (planned)

Derived from Trouble inspiration (`docs/inspiration/`); see the [implementation plan](../inspiration/implementation-plan-edge-2026-11-24.md).

| ID | Title | Personas | Status |
|----|--------|----------|--------|
| [US-025](./025-bounded-model-admission.md) | Bounded model admission & residency | Jack, Liz | partial |
| [US-027](./027-bounded-inference-state.md) | Bounded inference state | Liz, Pete | partial |
| [US-028](./028-constrained-benchmark-controls.md) | Constrained benchmarks & trustworthy comparisons | Pete, Jack | partial |
| [US-029](./029-capability-aware-kernels.md) | Capability-aware optimized execution | Pete, Toofer | planned |
| [US-030](./030-pi-gpu-backend.md) | Raspberry Pi GPU backend | Pete, Jack | planned (hardware-gated) |
| [US-031](./031-npu-backend.md) | NPU / on-device accelerator backend | Pete, Jack | planned (hardware-gated) |

## Acceptance tests

See [acceptance-tests.md](../acceptance-tests.md) and [contributing.md](../contributing.md). Every story must list at least one Rust or Python test in its **Acceptance tests** section, and that test must include telemetry or event-capture assertions (enforced by `scripts/verify-story-governance.py`).

## By crate

- **`uie-core`:** US-002 – US-007  
- **`uie-spiro`:** US-008  
- **`uie-loader-onnx`, `uie-mnist`, `models/`:** US-009  
- **Future loaders / demos:** US-010 – US-019
- **`uie-ipc`, `uie-serve`, `python/`:** US-020  
- **`uie-catalog`, `config/model_roots.example.yaml`:** US-021  
- **`uie-pipeline`:** US-022  
- **`uie-acceptance`, `python/acceptance/`:** all US-*  
- **Edge (planned):** US-025 admission, US-027 state, US-028 bench controls, US-029 kernels, US-030 Pi GPU, US-031 NPU — see [`docs/inspiration/implementation-plan-edge-2026-11-24.md`](../inspiration/implementation-plan-edge-2026-11-24.md)