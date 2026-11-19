---
name: uie-user-stories
description: Require every UIE feature to have a user story (US-0NN) and at least one acceptance test (Rust or Python). Use when adding crates, APIs, loaders, transport, catalog, or pipelines in the spiro/UIE repo.
---

# UIE user stories and acceptance tests

## Rule (mandatory)

**No new end-user-facing functionality** without:

1. A user story in `docs/stories/NNN-slug.md` (`US-0NN`) per [story conventions](../../docs/story-conventions.md).
2. At least one **acceptance test** registered in [acceptance index](../../docs/acceptance-tests.md):
   - **Rust:** `crates/uie-acceptance/tests/us_0NN_*.rs`, or an existing crate test **linked from the story** (must be named in the story file).
   - **Python:** `python/acceptance/test_us_0NN_*.py` when the story involves IPC, `uie-serve`, or external orchestration.

Planned work: story `Status: planned` is OK; acceptance test may use `#[ignore]` or `pytest.mark.skip` with reason until shipped.

## Workflow for agents

1. Read `docs/stories/README.md` for the next ID; copy `_template.md`.
2. Add glossary terms if needed.
3. Implement the feature.
4. Add or extend acceptance test(s); update story **Acceptance tests** table and **Acceptance notes**.
5. Run acceptance in the right **mode**:
   - **CI / stub (default):** `./scripts/acceptance-ci.sh` or `UIE_ACCEPTANCE_MODE=ci cargo test -p uie-acceptance`
   - **Live (real weights):** `./scripts/acceptance-live.sh` or `UIE_ACCEPTANCE_MODE=live` + see [acceptance-tests.md](../../docs/acceptance-tests.md)
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

## Acceptance test conventions

- Test function or method names prefix with story id: `us_020_hello_roundtrip`.
- Use repo-relative paths via `env!("CARGO_MANIFEST_DIR")` or `python/acceptance/repo_root.py`.
- IPC stories: prefer in-process Rust tests first; Python tests may spawn `uie-serve` when `UIE_ACCEPTANCE_SOCKET` is unset.
- Modality stories (US-010–US-019): **CI** = stub contract via `us_010_019_modalities.rs`; **live** = `UIE_LIVE_MODEL_PATH` or registry-discovered GGUF when loaders ship.
- Gate live tests with `support::skip_unless_live(...)`; they no-op in CI (still pass).

## Do not

- Ship crates without a story row in `docs/stories/README.md`.
- Add acceptance tests without updating the story file.
- Duplicate story IDs.