# Acceptance tests

Every [user story](./stories/README.md) (`US-0NN`) must have at least one automated acceptance test. Agents follow [.cursor/skills/uie-user-stories/SKILL.md](../.cursor/skills/uie-user-stories/SKILL.md).

## Run everything

```bash
cargo test -p uie-acceptance
python3 -m unittest discover -s python/acceptance -v
```

Quick IPC sanity (Rust only):

```bash
cargo test -p uie-ipc
```

## Index

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
| US-009 | `us_009_mnist.rs` | — | `#[ignore]` if ONNX build broken |
| US-010–019 | `us_010_019_modalities.rs` | — | stub I/O contracts |
| US-020 | `us_020_byte_channel.rs` (in `uie-ipc` too) | `test_us_020_ipc.py` | socket optional |
| US-021 | `us_021_model_registry.rs` | — | directory scan |
| US-022 | `us_022_byte_pipeline.rs` | — | multi-hop IPC |

Additional crate tests (also cited in stories):

- `crates/uie-stub-models/tests/deterministic_policy.rs` — policy overrides per modality
- `crates/uie-ipc/tests/roundtrip.rs` — US-020 in-process + Unix
- `crates/uie-catalog/tests/scan.rs` — US-021 layout scan

## Environment

| Variable | Purpose |
|----------|---------|
| `UIE_ACCEPTANCE_SOCKET` | If set, Python US-020 uses this socket instead of spawning `uie-serve` |
| `UIE_SOCKET` | Server bind path when spawning `uie-serve` in tests |
| `SPIRO_MODEL_ROOTS` | Path to `model_roots.yaml` for US-021 (optional) |