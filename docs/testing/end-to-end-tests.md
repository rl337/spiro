# End-to-end tests

End-to-end (E2E) tests are a **class of test** distinct from stub acceptance (`cargo test -p uie-acceptance` in CI).

The **`uie-e2e` binary is both an application and a test harness**: it links the same libraries we ship, drives real operator paths (registry, GGUF load, completion), emits instrumentation (phases/events on stderr), and applies assertions via `E2eContext`.

Each scenario is a **command object** ([command pattern](https://refactoring.guru/design-patterns/command)): a type that implements `E2eTest` with:

| Surface | Role |
|---------|------|
| `name()` | Stable identifier for `list` and selection |
| `description()` | Human-readable summary (printed when the test runs) |
| `run(ctx, args)` | Execute the scenario; **args** are test-specific CLI flags (after `--`) |

The harness selects tests when you run it:

- **Full name** — e.g. `run qwen3_2b_chat_completion`
- **Glob pattern** — `*` and `?` match test names (e.g. `run 'qwen*'`)

Acceptance tests prove story contracts in CI with stubs and fast gates. E2E tests prove **operator paths** on a laptop (GGUF load, registry scan, completion latency) without bloating default CI.

Browser WASM MNIST (US-026) is a separate Chromium gate: `./scripts/wasm-mnist-ci.sh` / `.github/workflows/uie-wasm-chromium.yml`.

## Harness: `uie-e2e`

```bash
# Linux: set CXX/CC before first GGUF build (see docs/runbooks/qwen3-2b-e2e.md)
export SPIRO_MODEL_ROOTS=$PWD/config/model_roots.yaml

cargo run -p uie-e2e --features gguf -- list
cargo run -p uie-e2e --features gguf -- run qwen3_2b_chat_completion
cargo run -p uie-e2e --features gguf -- run 'qwen*' -- --prompt "Hello" --max-new 32

**Cat + VL smoke** (proves T2I tensor → PNG → Qwen VL yes/no on a known cat fixture):

```bash
./scripts/download-qwen3-vl-0.6b-fixture.sh   # once, for VL weights
export SPIRO_MODEL_ROOTS=$PWD/config/model_roots.fixture-vl.yaml
cargo run -p uie-e2e --features spiro-chain -- run cat_t2i_vl_yes
cargo run -p uie-e2e --features spiro-chain -- run vl_cat_chat
```

| Subcommand | Purpose |
|------------|---------|
| `list` | Print registered E2E scenarios (`name` + `description`) |
| `list-models` | Print **discovered models** from roots (+ mock overlay by default) |
| `run <pattern>` | Run every scenario whose **name** matches `pattern` (exact or glob `*`, `?`) |

Mock stubs under `models/mock/` are merged by [`TestModelDetector`](../../crates/uie-catalog/src/detector.rs) (LM Studio + Comfy layouts). Disable with `UIE_E2E_MOCK_MODELS=0` or `list-models --no-mock`.

```bash
cargo run -p uie-e2e -- list-models
cargo run -p uie-e2e -- run mock_registry_discovery
cargo run -p uie-e2e -- run mock_registry_discovery -- --load-stub
cargo run -p uie-e2e -- run modality_live_smoke   # all 9 stub modalities, live CPU infer (US-010–019)
```

### Live modality tiers (cloud vs laptop)

| Tier | What runs | Where |
|------|-----------|--------|
| **A — Stub live** | `modality_live_smoke`: load `.stub` + `sample_input` → real `InferenceRuntime::infer` | Cloud agent (no GPU) |
| **B — Registry + ONNX** | `list-models`, MNIST `cargo run -p uie-mnist` | Cloud (tiny ONNX in repo) |
| **C — GGUF LLM** | `qwen3_2b_chat_completion` | Laptop + weights + native build |
| **D — Comfy weights** | `comfy-shared:` entries from `~/ComfyUI-Shared/models` | Laptop; optional `./scripts/bootstrap-cloud-live.sh` in cloud for scan-only |

`./scripts/bootstrap-cloud-live.sh` creates a Comfy-shaped tree and optionally downloads a ~176 KB HF `safetensors` for catalog smoke (no ComfyUI server).

Arguments after `--` are passed to each matched test’s `run` (test-specific CLI flags).

Environment and registry conventions match [acceptance live mode](../acceptance-tests.md) (`SPIRO_MODEL_ROOTS`, `UIE_GGUF_MODEL`, etc.).

## Anatomy of an `E2eTest`

```rust
pub trait E2eTest {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn run(&self, ctx: &mut E2eContext, args: &[String]) -> Result<(), E2eError>;
}
```

- **Getters** `name` / `description` identify the command for `list` and documentation.
- **`run`** executes the scenario: call `uie-loader-gguf`, `uie-catalog`, IPC, etc.; record events on `E2eContext`; assert with `ctx.assert(...)` / structured errors.
- **Instrumentation**: `E2eContext::phase`, `record`, stderr progress; optional hooks for trace JSON later.

New scenarios add a type under `crates/uie-e2e/src/scenarios/`, implement `E2eTest`, and register in `registry.rs`.

## Stories

E2E scenarios cite user stories in their module docs and commit messages (e.g. **US-010** for Qwen3 chat completion). They are not part of default `acceptance-ci.sh`.