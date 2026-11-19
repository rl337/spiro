# User stories index

Each story has an ID (`US-*`) so you can cite it in issues and PRs. Terms link to the [glossary](../glossary.md). Personas are [30 Rock first names](../personas.md).

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

## By crate

- **`uie-core`:** US-002 – US-007  
- **`uie-spiro`:** US-008  
- **`uie-loader-onnx`, `uie-mnist`, `models/`:** US-009  