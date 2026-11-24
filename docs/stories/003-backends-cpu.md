# US-003: Running compute on the CPU backend

**Status:** shipped (minimal operation set)  
**Personas:** Pete  
**Crate / path:** `crates/uie-core/src/backend.rs`, `backend/cpu.rs`

## User story

**As** Pete, trying to ship something on a normal laptop  
**I want** a default backend that works everywhere  
**So that** I can run demos and tests without a GPU or extra SDKs on anyone’s machine.

## In plain terms

A [backend](../glossary.md#backend) answers two questions: *where* does work run, and *how* are kernels invoked?

Today:

1. **[CpuBackend](../glossary.md#cpubackend)** is the reference implementation on the host [device](../glossary.md#device).
2. It can **allocate** [tensors](./002-tensors.md) on CPU memory.
3. It can **execute** a few [operations](./006-operations.md)—notably filling a tensor with a constant. Most graph ops still need fuller wiring ([US-005](./005-execution-plans.md)).

The `Backend` trait is the extension point: future crates (CUDA, WASM, **Pi GPU**, **NPU**, etc.) register the same interface so the [runtime](./004-runtime-sessions.md) does not change. Edge accelerators are roadmap items gated on hardware access ([US-030](./030-pi-gpu-backend.md), [US-031](./031-npu-backend.md)).

## Related stories

- [US-004 — Runtime](./004-runtime-sessions.md) holds the active backend
- [US-006 — Operations](./006-operations.md) backends implement
- [US-007 — Models](./007-models-loaders.md) may delegate to backends when loading
- [US-030 — Pi GPU](./030-pi-gpu-backend.md) / [US-031 — NPU](./031-npu-backend.md) (planned, hardware-gated)

## Terms used

- [Backend](../glossary.md#backend)
- [CpuBackend](../glossary.md#cpubackend)
- [Device](../glossary.md#device)
- [Operation](../glossary.md#operation)

## Acceptance notes

- `CpuBackend::new()` / `default()`.
- `allocate` and `Fill` op execution on CPU.
- `DeviceKind::Accelerator` exists as a placeholder only.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_003_cpu_backend.rs` | `cargo test -p uie-acceptance --test us_003_cpu_backend` |

## Observability

- **Events:** `cpu_backend_constructed`, `tensor_allocated_on_cpu`
- **CI test:** `us_003_cpu_backend.rs`
