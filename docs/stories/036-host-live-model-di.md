# US-036: Host live models via DI configuration

**Status:** planned (CI documents binding contract; live gated)  
**Personas:** Pete, Liz  
**Crate / path:** `uie-iv` modules + host-owned config on embedder/loader injectables

## User story

**As a** Pete (operator / weirdness deploy)  
**I want** live model directories and thread counts owned by injectables and bound in modules  
**So that** switching from stubs to real weights is a composition change—not scattered env checks in hub code.

## In plain terms

1. Host (or UIE) module binds a live loader set and/or live `TextEmbedder` whose config owns `model_dir`, `threads`, `expected_dimensions`.
2. Call sites only see `injector().text_embedder()` / `injector().loaders()`—no `UIE_*` reads ([DI-03](../requirements/dependency-injection.md)).
3. Missing live binding fails loudly; no silent hash/zeros fallback ([HOST-04](../requirements/in-process-host.md)).
4. Live admission respects [US-025](./025-bounded-model-admission.md) budgets when weights are resident.
5. CI still uses stubs ([US-035](./035-host-stub-bindings.md)); live tests use `OverrideLiveModule` / host live module and `skip_unless_live`.

## Related stories

- [US-034](./034-in-process-host.md) — host umbrella
- [US-035](./035-host-stub-bindings.md) — stub counterpart
- [US-024](./024-dependency-injection.md) — DI platform
- [US-033](./033-text-embeddings.md) — live sentence-transformer still open
- [US-021](./021-model-registry-scan.md) — model roots / bundles
- [US-025](./025-bounded-model-admission.md) — memory budget

## Terms used

- [In-process host](../glossary.md#in-process-host)
- [Composition root](../glossary.md#composition-root)
- [Model registry](../glossary.md#model-registry)
- [Memory budget](../glossary.md#memory-budget)

## Acceptance notes

- **CI:** assert live override flips profile / loader policy without env at call site; document that live weight load is hardware/artifact gated.
- **Live:** when weights are present, bind via module and embed/infer once with telemetry (future; `skip_unless_live` today).

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_036_host_live_model_di.rs` | `cargo test -p uie-acceptance --test us_036_host_live_model_di` |

## Observability

- **Events:** `host_live_override_ok` via `StoryEventLog` (CI)
- **Live:** embed/infer telemetry when live path is enabled
- **CI test:** `us_036_host_live_model_di.rs`
