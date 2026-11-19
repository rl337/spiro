# User stories index

Each story has an ID (`US-*`) so you can cite it in issues and PRs. Terms link to the [glossary](../glossary.md). Personas are [30 Rock first names](../personas.md).

## Foundation (shipped / partial)

| ID | Title | Personas | Status |
|----|--------|----------|--------|
| [US-001](./001-why-uie.md) | Why UIE exists | Liz, Jack | shipped |
| [US-002](./002-tensors.md) | Representing data as tensors | Kenneth | shipped |
| [US-003](./003-backends-cpu.md) | Running compute on the CPU backend | Pete | shipped (minimal ops) |
| [US-004](./004-runtime-sessions.md) | Runtime and named sessions | Liz | shipped |
| [US-005](./005-execution-plans.md) | Execution plans from graphs | Frank | shipped (scaffold) |
| [US-006](./006-operations.md) | Operations in the graph | Toofer | shipped (scaffold) |
| [US-007](./007-models-loaders.md) | Models, loaders, and registries | Jack, Liz | partial |
| [US-008](./008-spiro-demo.md) | Spirograph demo workload | Tracy, Jenna | shipped |
| [US-009](./009-onnx-mnist.md) | ONNX MNIST classification | Liz, Kenneth | partial |

## Modalities (planned)

Start with [US-015](./015-modality-catalog.md) for the input → output map.

| ID | Title | Personas | Modality |
|----|--------|----------|----------|
| [US-010](./010-llm-text-completions.md) | LLM text completions & chatbot | Liz | Text → Text |
| [US-011](./011-judge-system-one-models.md) | System-one / judge models | Jack, Toofer | Text → Text / labels |
| [US-012](./012-diffusion-text-to-image.md) | Diffusion text-to-image | Tracy, Jenna | Text → Image |
| [US-013](./013-speech-to-text.md) | Speech-to-text | Kenneth, Pete | Audio → Text |
| [US-014](./014-text-to-speech.md) | Text-to-speech | Jenna, Kenneth | Text → Audio |
| [US-015](./015-modality-catalog.md) | Modality catalog | Liz, Jack | (hub) |
| [US-016](./016-image-to-text.md) | Image-to-text | Frank | Image → Text |
| [US-017](./017-multimodal-text-image-to-text.md) | Multimodal chat | Liz | Text + Image → Text |
| [US-018](./018-audio-to-audio.md) | Audio-to-audio | Pete | Audio → Audio |
| [US-019](./019-image-to-image.md) | Image-to-image / inpainting | Tracy | Image → Image |
| [US-020](./020-inference-byte-channel.md) | Inference byte channel | Liz, Pete | (transport) |

## By crate

- **`uie-core`:** US-002 – US-007  
- **`uie-spiro`:** US-008  
- **`uie-loader-onnx`, `uie-mnist`, `models/`:** US-009  
- **Future loaders / demos:** US-010 – US-019
- **`uie-ipc`, `uie-serve`, `python/`:** US-020  