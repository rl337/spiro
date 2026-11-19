---
name: uie-user-stories
description: Require every UIE feature to have a user story (US-0NN) and at least one acceptance test (Rust or Python). Use when adding crates, APIs, loaders, transport, catalog, or pipelines in the spiro/UIE repo.
---

# UIE user stories and acceptance tests

## Rule (mandatory)

**No check-in** without a mapped **[user story](docs/stories/README.md)** (`US-0NN`) per [contributing.md](../../docs/contributing.md) and [`config/story_governance.yaml`](../../config/story_governance.yaml).

**No new end-user-facing functionality** without:

1. A user story in `docs/stories/NNN-slug.md` (`US-0NN`) per [story conventions](../../docs/story-conventions.md).
2. At least one **acceptance test** registered in [acceptance index](../../docs/acceptance-tests.md) and in `config/story_governance.yaml`:
   - **Rust:** `crates/uie-acceptance/tests/us_0NN_*.rs`, or an existing crate test **linked from the story** (must be named in the story file).
   - **Python:** `python/acceptance/test_us_0NN_*.py` when the story involves IPC, `uie-serve`, or external orchestration.
3. **Telemetry or event capture** in that acceptance test, relevant to the story:
   - **Infer paths:** `assert_infer_outcome_trace`, `assert_stub_executor_telemetry`, `assert_phases`, `assert_ipc_infer_trace`.
   - **Other paths:** `StoryEventLog` + `assert_story_events` from `uie_acceptance::observability`.

4. **Commit message cites `US-0NN`** for any change under `crates/`, `python/`, `models/`, or `docs/stories/`.

Planned work: story `Status: planned` is OK; acceptance test may use `#[ignore]` or `pytest.mark.skip` with reason until shipped.

## Workflow for agents

1. Read `docs/stories/README.md` for the next ID; copy `_template.md`.
2. Add glossary terms if needed.
3. Implement the feature.
4. Add or extend acceptance test(s) with observability assertions; update story **Acceptance tests** table, `docs/acceptance-tests.md`, and `config/story_governance.yaml`.
5. Run governance + acceptance:
   - `python3 scripts/verify-story-governance.py --skip-diff`
   - `./scripts/acceptance-ci.sh` or `UIE_ACCEPTANCE_MODE=ci cargo test -p uie-acceptance`
6. Stub coverage → `us_0NN_*.rs`. Real load/execute → `live_models.rs` (and optional `live-onnx` feature).

## Story file: required "Acceptance tests" section

After **Acceptance notes**, include:

```markdown
## Acceptance tests

| Runner | Path | Command |
|--------|------|---------|
| Rust | `crates/uie-acceptance/tests/us_0NN_foo.rs` | `cargo test -p uie-acceptance --test us_0NN_foo` |
| Python | `python/acceptance/test_us_0NN_foo.py` | `python3 -m unittest python.acceptance.test_us_0NN_foo` |
```

Note the **Observability** column expectation in [acceptance-tests.md](../../docs/acceptance-tests.md).

## Acceptance test conventions

- Test function or method names prefix with story id: `us_020_hello_roundtrip`.
- Use repo-relative paths via `env!("CARGO_MANIFEST_DIR")` or `python/acceptance/repo_root.py`.
- IPC stories: assert `InferResult.trace` via `assert_ipc_infer_trace` (Rust) or `assert_ipc_infer_has_trace` (Python).
- Modality stories (US-010–US-019): **CI** = stub contract + stub telemetry via `us_010_019_modalities.rs`; **live** = `UIE_LIVE_MODEL_PATH` or registry-discovered GGUF when loaders ship.
- Gate live tests with `support::skip_unless_live(...)`; they no-op in CI (still pass).

## Do not

- Ship crates without a story row in `docs/stories/README.md` and `config/story_governance.yaml`.
- Add acceptance tests without observability assertions (CI enforces markers).
- Duplicate story IDs.