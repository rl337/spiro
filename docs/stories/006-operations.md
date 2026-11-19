# US-006: Operations in the graph

**Status:** shipped (scaffold)  
**Personas:** Toofer  
**Crate / path:** `crates/uie-core/src/ops.rs`

## User story

**As** Toofer, who cares that every step is labeled correctly  
**I want** each step labeled with a kind, name, and tensor shapes  
**So that** backends know what to implement and logs remain readable.

## In plain terms

An [operation](../glossary.md#operation) (`Op`) contains:

- **`OpKind`** — `Noop`, `Fill { value }`, `MapUnary { name }`, or `Custom { name }` (placeholders for richer kernels).
- A **name** for tracing.
- **Input/output shapes and dtypes** for validation (future).

The [CPU backend](./003-backends-cpu.md) implements `Noop` and `Fill` when invoked through `Backend::execute`. The [runtime](./004-runtime-sessions.md) plan runner calls `run_op`, which currently forwards to the same limited set until session I/O is wired.

Custom ops are how specialized workloads (or imported ONNX nodes) will register without changing core enums every time.

## Related stories

- [US-003 — Backends](./003-backends-cpu.md)
- [US-005 — Plans](./005-execution-plans.md)

## Terms used

- [Operation](../glossary.md#operation)
- [Backend](../glossary.md#backend)

## Acceptance notes

- `Op`, `OpKind`, `Op::noop` constructor.
- CPU `execute` handles `Noop` / `Fill`.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_006_operations.rs` | `cargo test -p uie-acceptance --test us_006_operations` |

## Observability

- **Events:** `op_kind_fill_constructed`
- **CI test:** `us_006_operations.rs`
