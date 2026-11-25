# US-034: In-process host consumers

**Status:** partial (CI host-shaped graph; live weight path under [US-036](./036-host-live-model-di.md))  
**Personas:** Liz, Jack  
**Crate / path:** `uie-iv` + host-facing crates (`uie-embeddings`, `uie-stub-models`, loaders); consumers e.g. [weirdness](https://github.com/rl337/weirdness) via subtree/path

## User story

**As a** Liz (weirdness / sibling hub)  
**I want** to depend on Spiro UIE crates inside my own process and resolve models through a composition root  
**So that** my product can run inference and embeddings without a second process or byte-channel hop on the hot path.

## In plain terms

1. Host adds UIE as a path or git-subtree dependency (`vendor/spiro` or workspace path).
2. Host builds an [`Injector`](../../crates/uie-iv) from modules (host modules + UIE CI/live overrides).
3. Call sites use injected loaders / `TextEmbedder` / runtime—not `uie-serve` sockets.
4. Normative rules: [in-process-host.md](../requirements/in-process-host.md) (HOST-01, HOST-02, HOST-05, HOST-06).

## Related stories

- [US-024](./024-dependency-injection.md) — composition root
- [US-033](./033-text-embeddings.md) — first string-facing in-process API
- [US-035](./035-host-stub-bindings.md) — stub-only CI bindings
- [US-036](./036-host-live-model-di.md) — live weights via DI config
- [US-007](./007-models-loaders.md) — loader chain
- [US-020](./020-inference-byte-channel.md) — out-of-process alternative (not the host hot path)

## Terms used

- [In-process host](../glossary.md#in-process-host)
- [Composition root](../glossary.md#composition-root)
- [Model loader](../glossary.md#model-loader)
- [Text embedding](../glossary.md#text-embedding)

## Acceptance notes

- **Today (CI):** acceptance builds a host-shaped injector, resolves stub loaders + stub embedder, loads a `.stub` model, records story events + embed telemetry.
- **Live:** covered by [US-036](./036-host-live-model-di.md); not required for CI green.

## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_034_in_process_host.rs` | `cargo test -p uie-acceptance --test us_034_in_process_host` |

## Observability

- **Events:** `host_injector_ok`, `host_stub_loader_ok` via `StoryEventLog`
- **Trace:** embed path uses `assert_embed_telemetry` when exercising `text_embedder`
- **CI test:** `us_034_in_process_host.rs`
