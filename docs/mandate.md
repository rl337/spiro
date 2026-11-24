# Project mandate

**Spiro / UIE** builds a **good inference engine at any scale**.

That trajectory does **not** change when we talk about edge or embedded. Edge is
where we **break ties**: if a design choice must compromise one way or another,
we compromise in favor of **edge and embedded performance** (tight RAM/CPU,
WASM, Raspberry Pi–class ARM), not the other way around.

## Trajectory (unchanged)

- One engine story from CI stubs → live models → larger desktop/server backends.
- Shared loaders, telemetry, and DI wiring — not a cloud fork and an edge fork.
- Server GPU, Metal/CUDA, and bigger models remain in scope when they ride the same core.

## When we must compromise

Prefer the option that keeps **edge/embedded** viable and measurable:

| Prefer | Over |
|--------|------|
| Explicit budgets, admission, and safe fallbacks | Assuming “enough RAM/GPU” |
| Paths that work on **WASM** and **Pi-class** hosts | Designs that only pay off in the datacenter |
| Measurable cost ([US-023](./stories/023-inference-telemetry.md), [US-028](./stories/028-constrained-benchmark-controls.md)) | Unverified vendor tok/s folklore |
| Claiming techniques with a user story | Importing every GPU serving idea by default |

Edge resource stories ([US-025](./stories/025-bounded-model-admission.md)–[US-029](./stories/029-capability-aware-kernels.md)) and embedded WASM ([US-026](./stories/026-browser-wasm-inference.md)) exist to make that compromise concrete — not to redefine the product as “Pi-only.”

## What this is not

- Not “abandon any-scale; only ship for the Pi.”
- Not “cloud-only with edge as an afterthought.”
- Not a mandate to reimplement every GPU serving stack on embedded hardware.
- Not a promise that every item in [inspiration](./inspiration/README.md) ships — claim them with a user story first.

## How we keep score

- User stories and acceptance tests ([contributing](./contributing.md)).
- Structured telemetry and trustworthy comparisons.
- DI-owned config ([US-024](./stories/024-dependency-injection.md)) so scale/profile knobs stay injectable.

## See also

- [US-001 — Why UIE exists](./stories/001-why-uie.md)
- [Implementation plan (edge stories)](./inspiration/implementation-plan-edge-2026-11-24.md)
- Root [README](../README.md)
