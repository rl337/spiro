# US-035: Host stub bindings for testability

**Status:** partial  
**Personas:** Jack, Liz  
**Crate / path:** `uie-iv` (`OverrideCiModule`), `uie-stub-models`, `uie-embeddings` (`StubTextEmbedder`)

## User story

**As a** Jack (host CI / weirdness tests)  
**I want** to force stub loaders and stub embedders through DI overrides  
**So that** host tests stay deterministic without downloading weights or flipping `UIE_*` mid-test.

## In plain terms

1. Host test graph installs `OverrideCiModule` (or a host module that binds the same stub set).
2. `injector().loaders()` is stub-only; `.stub` modalities load and infer with stub telemetry.
3. `injector().text_embedder()` is `StubTextEmbedder` with stable `uie-stub-text-embed-v1` id.
4. No process env mutation between assertions ([DI-04](../requirements/dependency-injection.md)).
5. Contract: [in-process-host.md](../requirements/in-process-host.md) HOST-03.

## Related stories

- [US-034](./034-in-process-host.md) — host umbrella
- [US-024](./024-dependency-injection.md) — override modules
- [US-010](./010-llm-text-completions.md)–[US-019](./019-image-to-image.md), [US-032](./032-image-filter-stub.md) — stub modalities
- [US-033](./033-text-embeddings.md) — stub embedder
- [US-036](./036-host-live-model-di.md) — live counterpart

## Terms used

- [In-process host](../glossary.md#in-process-host)
- [Stub model](../glossary.md#stub-model)
- [Composition root](../glossary.md#composition-root)

## Acceptance notes

- CI proves OverrideCi → stub loaders + stub embedder identity without env reads at the test call site.
- Stub model ids must not be labeled as live reference models.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_035_host_stub_bindings.rs` | `cargo test -p uie-acceptance --test us_035_host_stub_bindings` |

## Observability

- **Events:** `host_ci_stub_bound`, `host_stub_embedder_id` via `StoryEventLog`
- **Trace:** stub modality infer uses `assert_stub_executor_telemetry` (or modality helper)
- **CI test:** `us_035_host_stub_bindings.rs`
