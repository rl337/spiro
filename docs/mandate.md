# Project mandate

**Spiro / UIE** builds a **good inference engine at any scale**, with a deliberate
**emphasis on edge and embedded compute**.

## What we optimize for

| Priority | Meaning |
|----------|---------|
| **Correct & portable** | Same engine story from laptop CI stubs to live models — not a fork per device class. |
| **Edge-first defaults** | Designs assume tight RAM/CPU, explicit admission, measurable cost, and safe fallbacks ([US-025](./stories/025-bounded-model-admission.md)–[US-029](./stories/029-capability-aware-kernels.md)). |
| **Embedded WASM** | In-browser / `wasm32` CPU paths are first-class (e.g. [US-026](./stories/026-browser-wasm-inference.md)); WebGPU stays a reserved, non-silent path. |
| **Small boards** | Targets such as **Raspberry Pi 4/5** (ARM64 containers) guide live acceptance and resource budgets — datacenter GPU tricks are optional, not the default design center. |
| **Any scale still welcome** | Server GPU, desktop Metal/CUDA, and larger models remain in scope when they share the same loaders, telemetry, and DI wiring. |

## What this is not

- Not “cloud-only inference with an edge afterthought.”
- Not a mandate to reimplement every GPU serving stack on the Pi.
- Not a promise that every optimization from the [inspiration](./inspiration/README.md) backlog ships — claim them with a user story first.

## How we keep score

- User stories and acceptance tests ([contributing](./contributing.md)).
- Structured telemetry ([US-023](./stories/023-inference-telemetry.md)) and trustworthy comparisons ([US-028](./stories/028-constrained-benchmark-controls.md)).
- DI-owned config ([US-024](./stories/024-dependency-injection.md)) so edge vs live knobs stay injectable, not scattered env checks.

## See also

- [US-001 — Why UIE exists](./stories/001-why-uie.md)
- [Implementation plan (edge stories)](./inspiration/implementation-plan-edge-2026-11-24.md)
- Root [README](../README.md)
