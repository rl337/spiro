# US-001: Why UIE exists

**Status:** shipped  
**Personas:** Liz, Jack  
**Crate / path:** workspace root, `README.md`

## User story

**As** Liz, who inherited a Spirograph art script—and **as** Jack, who wants one platform for demos and models  
**I want** a single Rust engine that can run both toy workloads and real models  
**So that** Liz is not maintaining parallel scripts and Jack sees one scalable story for the team.

## In plain terms

This repo began as `spiro.py`, which fills a big pixel grid and saves a JPEG. UIE keeps that idea—**compute writes into a buffer**—and generalizes it into a small engine:

1. Data lives in [tensors](./002-tensors.md).
2. A [backend](../glossary.md#backend) runs the math.
3. A [runtime](./004-runtime-sessions.md) ties sessions, models, and plans together.
4. Demos like the [Spirograph](./008-spiro-demo.md) prove the plumbing; [model loaders](./007-models-loaders.md) extend it to ONNX and beyond.

You do not need to know Rust internals to follow the stories—start with [US-002](./002-tensors.md) if you care how data is stored.

## Related stories

- [US-002 — Tensors](./002-tensors.md) (foundation)
- [US-008 — Spirograph demo](./008-spiro-demo.md) (original motivation)

## Terms used

- [UIE](../glossary.md#uie)
- [Tensor](../glossary.md#tensor)
- [Workload](../glossary.md#workload)

## Acceptance notes

- Workspace `Cargo.toml` lists `uie-core` and demo crates.
- Root `README.md` describes the migration from Python Spirograph to UIE.