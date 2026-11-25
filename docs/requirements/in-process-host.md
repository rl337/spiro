# In-process hosts (subtree / path consumers)

Normative contract so sibling products ([weirdness](https://github.com/rl337/weirdness) and later hubs) can depend on Spiro UIE crates **inside their own process**—via git subtree, path dependency, or published crates—and run models without standing up `uie-serve` or the [byte channel](../stories/020-inference-byte-channel.md) for the hot path.

Product mandate: [docs/mandate.md](../mandate.md). Wiring: [dependency-injection.md](./dependency-injection.md). Embeddings specialty: [text-embeddings.md](./text-embeddings.md) / [US-033](../stories/033-text-embeddings.md).

Shipping stories: [US-034](../stories/034-in-process-host.md), [US-035](../stories/035-host-stub-bindings.md), [US-036](../stories/036-host-live-model-di.md).

## Motivation

Hosts like weirdness are Rust binaries with their own DB, HTTP, and MCP surface. They need:

1. **Library call sites** — `TextEmbedder`, `InferenceRuntime` / loaders — not Unix sockets for every embed or infer.
2. **Deterministic CI** — stub modalities and stub embedders with fixed ids, no weight downloads.
3. **Configurable live weights** — model directories, thread counts, and expected dimensions owned by injectables and bound in `uie-iv` modules, not `std::env::var` at call sites ([DI-03](./dependency-injection.md)).

Byte channel and `uie-serve` remain for multi-hop and out-of-process demos; they are **not** the primary host integration for an in-process consumer.

## Requirements

### HOST-01: In-process dependency

A host MUST be able to depend on UIE crates (`uie-core`, `uie-stub-models`, `uie-embeddings`, `uie-iv`, loaders as needed) and call them in-process.

The hot path MUST NOT require a Unix socket, HTTP inference server, or `InProcessChannel` framing. A host MAY still use the byte channel for optional side paths.

### HOST-02: Composition root in the host

The host binary (or its test harness) MUST build an [`Injector`](../../crates/uie-iv/src/injector.rs) from modules. Host-specific knobs (data dir, embedding model path, API keys for non-UIE backends) stay on **host** injectables; UIE knobs stay on UIE injectables ([DI-01](./dependency-injection.md), [DI-05](./dependency-injection.md)).

Hosts MUST NOT read `UIE_*` outside `uie-iv` modules. Prefer host modules that call `OverrideCiModule` / `OverrideLiveModule` or bind a dedicated loader set / embedder.

### HOST-03: Stub bindings for testability

CI and unit tests MUST be able to force:

- stub-only [`ModelLoader`](../stories/007-models-loaders.md) chain (`.stub` modalities), and
- [`StubTextEmbedder`](../stories/033-text-embeddings.md) (or equivalent stub executor)

via module override—without mutating process environment mid-test ([DI-04](./dependency-injection.md)).

Stub `model_id` / modality ids MUST be stable and MUST NOT collide with live reference model ids.

### HOST-04: Live model via DI configuration

Loading real weights MUST be a **binding change**: a module installs a live loader set and/or a live embedder whose config owns `model_dir`, `threads`, `expected_dimensions`, and related paths.

Library code MUST NOT construct a live embedder or loader as a silent fallback when the binding is missing ([DI-02](./dependency-injection.md)). Failed load fails the injector build or the first explicit load—not with zeros or a hash embedder disguised as the live model.

Admission of live weight resident size SHOULD go through the existing budget path ([US-025](../stories/025-bounded-model-admission.md)).

### HOST-05: Telemetry visible to the host

Infer and embed paths used by hosts MUST record phases on [`ExecutionRecorder`](../stories/023-inference-telemetry.md). Host acceptance (in Spiro or the host repo) MUST assert telemetry or story events the same way other US-* tests do.

### HOST-06: Vendoring without forking the contract

When a host vendors Spiro (e.g. `vendor/spiro` subtree), it MUST treat `docs/requirements/*.md` and the cited user stories as the API contract. Local patches under `vendor/` that change HOST-* / EMB-* / DI-* behavior MUST be upstreamed or documented as intentional forks.

## Out of scope

- Replacing weirdness’s OpenAI-compatible HTTP embedder until the host chooses to bind UIE live/stub.
- Publishing crates to crates.io (path/subtree is enough for now).
- Making every modality a high-level string API (tensor/`InferenceExecutor` remains the general surface; embeddings are the first string-facing specialty).
