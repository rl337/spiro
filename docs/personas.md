# Personas (30 Rock)

User stories use first names from **30 Rock** as stand-ins for who cares about each feature. Names are flavor for readability—not endorsements of character behavior.

| Persona | Typical need in UIE stories |
|---------|------------------------------|
| **Liz** | Maintainer turning a quirky script into a real engine; wires runtime and loaders. |
| **Jack** | Wants one scalable platform and clean handoffs from “demo” to “model.” |
| **Kenneth** | Eager to learn; needs simple, dependable building blocks (tensors, tiny models). |
| **Pete** | Producer mindset—things must run on ordinary hardware, on schedule. |
| **Frank** | Comfortable with weird graphs and half-finished compilers. |
| **Toofer** | Cares that every operation has a precise name and shape. |
| **Tracy** | Wants a flashy demo that proves the engine “does something.” |
| **Jenna** | Cares about the visible output (images, scores) more than internals. |

## Story → persona map

| Story | Personas | Perspective |
|-------|----------|-------------|
| [US-001](./stories/001-why-uie.md) | Liz, Jack | Why the repo exists and who wants a unified engine. |
| [US-002](./stories/002-tensors.md) | Kenneth | Learning how data is stored. |
| [US-003](./stories/003-backends-cpu.md) | Pete | Default CPU path for everyone’s machine. |
| [US-004](./stories/004-runtime-sessions.md) | Liz | Orchestrating a run with named tensors. |
| [US-005](./stories/005-execution-plans.md) | Frank | Ordered op lists / future graph compiler. |
| [US-006](./stories/006-operations.md) | Toofer | Op kinds, labels, and validation detail. |
| [US-007](./stories/007-models-loaders.md) | Jack, Liz | Loading files from disk; integrating formats. |
| [US-008](./stories/008-spiro-demo.md) | Tracy, Jenna | Visual Spirograph demo and JPEG output. |
| [US-009](./stories/009-onnx-mnist.md) | Liz, Kenneth | Small MNIST model to learn real inference. |

When adding a story, pick one or more personas from the table (or justify a new name in this file).