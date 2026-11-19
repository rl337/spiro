# US-002: Representing data as tensors

**Status:** shipped  
**Personas:** Kenneth  
**Crate / path:** `crates/uie-core/src/tensor.rs`

## User story

**As** Kenneth, learning how the engine stores data  
**I want** one way to describe multi-dimensional data and its element type  
**So that** the Spirograph canvas, MNIST digits, and future model weights all use the same container I can memorize once.

## In plain terms

A [tensor](../glossary.md#tensor) is three things together:

1. A **[shape](../glossary.md#shape)** — for example `28 × 28` or `2048 × 2048 × 3` for RGB.
2. A **[data type](../glossary.md#datatype)** — such as `U8` bytes for images or `F32` floats for neural nets.
3. A byte buffer sized to `shape × element size`.

The Spiro demo uses a `U8` tensor shaped like an image. The MNIST story uses `F32` with shape `[1, 1, 28, 28]`. Helpers can pack or read `f32` slices when loaders need floats.

If the buffer length does not match the shape, UIE rejects the tensor rather than silently corrupting memory.

## Related stories

- [US-003 — Backends](./003-backends-cpu.md) allocate empty tensors
- [US-004 — Sessions](./004-runtime-sessions.md) bind tensors by name
- [US-008 — Spirograph](./008-spiro-demo.md) writes a canvas tensor
- [US-009 — MNIST](./009-onnx-mnist.md) feeds a float tensor into ONNX

## Terms used

- [Tensor](../glossary.md#tensor)
- [Shape](../glossary.md#shape)
- [DataType](../glossary.md#datatype)

## Acceptance notes

- `Tensor::empty`, `from_bytes`, `from_f32_slice`, `as_f32_slice` in `uie-core`.
- Unit tests in `tensor.rs` for shape math and `f32` round-trip.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_002_tensors.rs` | `cargo test -p uie-acceptance --test us_002_tensors` |
