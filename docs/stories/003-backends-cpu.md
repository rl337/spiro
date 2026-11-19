# US-003: Running compute on the CPU backend

**Status:** shipped (minimal operation set)  
**Crate / path:** `crates/uie-core/src/backend.rs`, `backend/cpu.rs`

## User story

**As a** runtime user  
**I want** a default backend that works everywhere  
**So that** I can run demos and tests without a GPU or extra SDKs.

## In plain terms

A [backend](../glossary.md#backend) answers two questions: *where* does work run, and *how* are kernels invoked?

Today:

1. **[CpuBackend](../glossary.md#cpubackend)** is the reference implementation on the host [device](../glossary.md#device).
2. It can **allocate** [tensors](./002-tensors.md) on CPU memory.
3. It can **execute** a few [operations](./006-operations.md)—notably filling a tensor with a constant. Most graph ops still need fuller wiring ([US-005](./005-execution-plans.md)).

The `Backend` trait is the extension point: future crates (CUDA, WASM, etc.) register the same interface so the [runtime](./004-runtime-sessions.md) does not change.

## Related stories

- [US-004 — Runtime](./004-runtime-sessions.md) holds the active backend
- [US-006 — Operations](./006-operations.md) backends implement
- [US-007 — Models](./007-models-loaders.md) may delegate to backends when loading

## Terms used

- [Backend](../glossary.md#backend)
- [CpuBackend](../glossary.md#cpubackend)
- [Device](../glossary.md#device)
- [Operation](../glossary.md#operation)

## Acceptance notes

- `CpuBackend::new()` / `default()`.
- `allocate` and `Fill` op execution on CPU.
- `DeviceKind::Accelerator` exists as a placeholder only.