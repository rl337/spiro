# Acceptance tests

Every [user story](./stories/README.md) (`US-0NN`) must have at least one automated acceptance test **with a telemetry or event-capture assertion** relevant to that story. Check-ins must cite `US-0NN` in commit messages when touching product paths (see [contributing.md](./contributing.md)). Agents follow [.cursor/skills/uie-user-stories/SKILL.md](../.cursor/skills/uie-user-stories/SKILL.md).

CI runs [`scripts/verify-story-governance.py`](../scripts/verify-story-governance.py) against [`config/story_governance.yaml`](../config/story_governance.yaml).

### Observability in acceptance tests

| Kind | When | Helpers |
|------|------|---------|
| Execution trace | `infer`, IPC `infer_result` | `uie_acceptance::assert_infer_outcome_trace`, `assert_ipc_infer_trace`, `uie_stub_models::telemetry_assert::assert_stub_executor_telemetry`, `uie_core::telemetry::assert_phases` |
| Story events | Layout, registry scan, tensors without infer | `StoryEventLog`, `assert_story_events` |
| Python IPC | US-020 | `assert_ipc_infer_has_trace` in `test_us_020_ipc.py` |

## Two modes

| Mode | Env | What runs |
|------|-----|-----------|
| **CI / stub** (default) | `UIE_ACCEPTANCE_MODE=ci` or unset | `us_0NN_*.rs` — stubs, fixtures, in-process IPC |
| **Live** | `UIE_ACCEPTANCE_MODE=live` | Above **plus** `live_models.rs` — scan LM Studio / Comfy roots, load `UIE_LIVE_MODEL_PATH`, optional real ONNX |

Scripts:

```bash
./scripts/acceptance-ci.sh          # CI default
./scripts/acceptance-live.sh        # laptop / nightly (--features live-onnx when ONNX builds)
```

Example env: [`config/acceptance.env.example`](../config/acceptance.env.example).

### CI (GitHub)

[`.github/workflows/uie-acceptance-ci.yml`](../.github/workflows/uie-acceptance-ci.yml) sets `UIE_ACCEPTANCE_MODE=ci` and runs `acceptance-ci.sh`.

### Live on your laptop

```bash
cp config/acceptance.env.example .env.acceptance  # edit paths
export UIE_ACCEPTANCE_MODE=live
export SPIRO_MODEL_ROOTS=$PWD/config/model_roots.yaml
export UIE_LIVE_MODEL_PATH=/path/to/model.gguf   # or .onnx when loader works
./scripts/acceptance-live.sh
```

Optional strict checks:

- `UIE_LIVE_REQUIRE_LMSTUDIO=1` — fail if `~/.lmstudio/models` has no `.gguf`
- `UIE_ACCEPTANCE_RUN_STUBS_IN_LIVE=1` — re-run stub `us_*` tests after live suite

### ONNX live infer

When `uie-loader-onnx` compiles:

```bash
UIE_ACCEPTANCE_MODE=live cargo test -p uie-acceptance --features live-onnx --test live_models live_us_009_mnist_onnx_infer
```

## Run everything

```bash
# CI
cargo test -p uie-acceptance
python3 -m unittest discover -s python/acceptance -v

# Live (Rust)
UIE_ACCEPTANCE_MODE=live cargo test -p uie-acceptance
```

## Index

| Story | CI (stub) | Live | Python |
|-------|-----------|------|--------|
| US-001–008 | `us_001` … `us_008` | — | — |
| US-009 | `us_009` fixture | `live_us_009_mnist_onnx_infer` (`live-onnx`) | — |
| US-015 | `us_015_modality_catalog` | — | — |
| US-010–019 | `us_010_019_modalities` | `UIE_LIVE_MODEL_PATH` via `live_us_007_*` | — |
| US-020 | `us_020_byte_channel` | `UIE_LIVE_MODEL_PATH` + `test_us_020_ipc_live` | stub + live IPC |
| US-021 | `us_021_model_registry` | `live_us_021_scan_*` | — |
| US-022 | `us_022_byte_pipeline` | `live_us_022_byte_pipeline_*` | — |

| Story | Rust acceptance | Python acceptance | Notes |
|-------|-----------------|-------------------|--------|
| US-001 | `us_001_why_uie.rs` | — | workspace smoke |
| US-002 | `us_002_tensors.rs` | — | |
| US-003 | `us_003_cpu_backend.rs` | — | |
| US-004 | `us_004_runtime_session.rs` | — | |
| US-005 | `us_005_execution_plan.rs` | — | |
| US-006 | `us_006_operations.rs` | — | |
| US-007 | `us_007_model_loader.rs` | — | stub loader |
| US-008 | `us_008_spiro.rs` | — | workload math |
| US-009 | `us_009_mnist.rs` | — | fixture; live ONNX in `live_models.rs` |
| US-015 | `us_015_modality_catalog.rs` | — | modality index |
| US-010–019 | `us_010_019_modalities.rs` | — | stub I/O + telemetry |
| US-020 | `us_020_byte_channel.rs` | `test_us_020_ipc.py`, `test_us_020_ipc_live.py` | |
| US-021 | `us_021_model_registry.rs` | — | tempdir CI; real roots live |
| US-022 | `us_022_byte_pipeline.rs` | — | multi-hop IPC |
| US-023 | `us_023_telemetry.rs` | — | TTFT / trace phases |
| US-024 | `us_024_dependency_injection.rs` | — | `uie-iv` CI/live injector |

Additional crate tests (also cited in stories):

- `crates/uie-stub-models/tests/deterministic_policy.rs` — policy overrides per modality
- `crates/uie-ipc/tests/roundtrip.rs` — US-020 in-process + Unix
- `crates/uie-catalog/tests/scan.rs` — US-021 layout scan
- `crates/uie-acceptance/tests/live_models.rs` — live-only paths

## Environment

| Variable | Purpose |
|----------|---------|
| `UIE_ACCEPTANCE_MODE` | `ci` (default) or `live` |
| `UIE_ACCEPTANCE_FEATURES` | e.g. `live-onnx` for `acceptance-live.sh` |
| `UIE_ACCEPTANCE_RUN_STUBS_IN_LIVE` | Re-run stub tests after live suite |
| `UIE_LIVE_MODEL_PATH` | Real weights to load in live tests (.gguf / .onnx / .stub) |
| `UIE_LIVE_REQUIRE_LMSTUDIO` | If `1`, require GGUF under `~/.lmstudio/models` |
| `UIE_ACCEPTANCE_SOCKET` | Python US-020 socket (skip spawn) |
| `UIE_SOCKET` | Server bind when spawning `uie-serve` |
| `SPIRO_MODEL_ROOTS` | `model_roots.yaml` for US-021 live scan |
| `UIE_MNIST_MODEL` / `UIE_MNIST_FIXTURE` | US-009 live ONNX paths |