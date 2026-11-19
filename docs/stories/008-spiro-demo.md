# US-008: Spirograph demo workload

**Status:** shipped  
**Personas:** Tracy, Jenna  
**Crate / path:** `crates/uie-spiro`, `spiro.py`

## User story

**As** Tracy, who needs something eye-catching—and **as** Jenna, who judges the final picture  
**I want** to run the classic Spirograph drawing through UIE  
**So that** we see tensors, runtime, and sessions doing something visual—not only ML jargon—and get a JPEG worth showing off.

## In plain terms

The Spirograph [workload](../glossary.md#workload) traces hypocycloid curves into a 2048×2048 RGB grid—the same math as `spiro.py`, ported to Rust:

1. Allocate a [tensor](./002-tensors.md) shaped `[2048, 2048, 3]` with type `U8`.
2. Create an [inference runtime](./004-runtime-sessions.md) with the [CPU backend](./003-backends-cpu.md).
3. Bind the tensor as `"canvas"` in a [session](../glossary.md#session).
4. Run the spiro kernels (Rust) that paint pixels until each layer’s path repeats.
5. Save `output.jpg`.

The demo does **not** yet express spiro steps as [graph operations](./006-operations.md); it calls Rust drawing code directly. That is intentional: it shows session + tensor flow first; graph ops can wrap the same kernels later.

## Related stories

- [US-001 — Why UIE](./001-why-uie.md)
- [US-002 — Tensors](./002-tensors.md)
- [US-004 — Runtime](./004-runtime-sessions.md)

## Terms used

- [Workload](../glossary.md#workload)
- [Tensor](../glossary.md#tensor)
- [Session](../glossary.md#session)
- [UIE](../glossary.md#uie)

## Acceptance notes

- `cargo run -p uie-spiro` → `output.jpg` (large image; may take time).
- Compare with `python spiro.py` for parity reference.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_008_spiro.rs` | `cargo test -p uie-acceptance --test us_008_spiro` |

## Observability

- **Events:** `spiro_demo_entrypoint`, `spiro_module_sources_present`
- **CI test:** `us_008_spiro.rs`
