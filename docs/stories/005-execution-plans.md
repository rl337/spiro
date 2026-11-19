# US-005: Execution plans from graphs

**Status:** shipped (scaffold; full op wiring incomplete)  
**Personas:** Frank  
**Crate / path:** `crates/uie-core/src/model.rs`, `runtime.rs`

## User story

**As** Frank, who is fine experimenting with half-built graph machinery  
**I want** an ordered list of operations the runtime can walk  
**So that** plan-based models run the same way as imported ONNX graphs will eventually decompose.

## In plain terms

An [execution plan](../glossary.md#execution-plan) is simply `Vec<Op>` in order. A [graph](../glossary.md#graph) adds names for inputs, outputs, and the op list; `Graph::to_plan()` drops the names and keeps the ordered steps.

[Plan-style models](../glossary.md#inference-model) expose `plan()` for this path. `InferenceRuntime::run_model` loops the plan and calls `run_op` for each entry.

**Limitation today:** most [op kinds](./006-operations.md) besides `Noop` and backend-side `Fill` still return “needs explicit I/O binding.” The plan runner exists so later work can connect tensor slots without redesigning the API.

## Related stories

- [US-006 — Operations](./006-operations.md)
- [US-004 — Runtime](./004-runtime-sessions.md)
- [US-007 — Models](./007-models-loaders.md) (plan vs executor split)

## Terms used

- [Execution plan](../glossary.md#execution-plan)
- [Graph](../glossary.md#graph)
- [Inference model](../glossary.md#inference-model)

## Acceptance notes

- `ExecutionPlan`, `Graph::to_plan`, `PlanModel`, `run_model` loop.
- Documented limitation: graph ops not fully executed on session bindings yet.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_005_execution_plan.rs` | `cargo test -p uie-acceptance --test us_005_execution_plan` |

## Observability

- **Events:** `execution_plan_emitted`
- **CI test:** `us_005_execution_plan.rs`
