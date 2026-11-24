# Contributing to UIE (Spiro)

**Mandate:** [docs/mandate.md](./mandate.md) — good inference at any scale, with
emphasis on edge and embedded (WASM, Raspberry Pi–class ARM). Prefer designs that
fit tight resource budgets; do not treat server GPU as the only design center.

## Check-ins and user stories

Every change that touches product code must map to a **[user story](stories/README.md)** (`US-0NN`).

1. **Story first** — Add or update `docs/stories/NNN-slug.md` (see [story conventions](./story-conventions.md)).
2. **Acceptance test** — Each story needs at least one automated acceptance test ([acceptance-tests.md](./acceptance-tests.md)).
3. **Observability assertion** — Every acceptance test must assert **telemetry** (execution trace / phases) or **captured events** relevant to that story. Helpers live in `uie_acceptance::story_assert` and `uie_acceptance::observability`.
4. **Cite the story in commits** — Commit subjects or bodies must include the `US-0NN` id(s) your change implements (e.g. `US-023: require traces on stub infer`).

CI runs [`scripts/verify-story-governance.py`](../scripts/verify-story-governance.py) before acceptance tests:

- Every `US-*` in [`config/story_governance.yaml`](../config/story_governance.yaml) has on-disk acceptance sources.
- Those sources contain an observability marker (telemetry or event capture).
- Optional **`supplemental_acceptance`** paths in `story_governance.yaml` (e.g. `live_models.rs`, `deterministic_policy.rs`) are checked the same way.
- On pull requests, changed paths under `crates/`, `python/`, `models/`, or `docs/stories/` must be justified by a cited `US-0NN` in recent commits and/or an edited story file.

Agents must follow [.cursor/skills/uie-user-stories/SKILL.md](../.cursor/skills/uie-user-stories/SKILL.md).

## Dependency injection (pyiv / uie-iv first)

Wiring CI vs live, loader chains, and host entrypoints is **DI-first**: bind in
[`uie-iv`](../crates/uie-iv/README.md) modules and consume via
[`composition::injector`](../crates/uie-acceptance/src/composition.rs). Normative rules:
[`docs/requirements/dependency-injection.md`](requirements/dependency-injection.md),
[`.cursor/rules/di-first.mdc`](../.cursor/rules/di-first.mdc).

## Running checks locally

```bash
./scripts/rust-static-check.sh          # rustfmt + clippy (-D warnings)
./scripts/python-static-check.sh        # ruff lint + format
python3 scripts/verify-story-governance.py --skip-diff   # structure only
python3 scripts/verify-story-governance.py             # includes check-in citation vs main
python3 scripts/verify-di-first.py                      # DI-VERIFY-01 env-branch lint
./scripts/acceptance-ci.sh
```

### CI on pull requests

| Workflow | What |
|----------|------|
| [static-checks.yml](../.github/workflows/static-checks.yml) | Rust `fmt`/`clippy`, Python `ruff` |
| [uie-acceptance-ci.yml](../.github/workflows/uie-acceptance-ci.yml) | Story governance + acceptance tests |
| [bugbot-gate.yml](../.github/workflows/bugbot-gate.yml) | Waits for **Cursor Bugbot** when the App posts a check |

Enable Bugbot on the repo in [Cursor Agents → Automations → Bugbot](https://cursor.com/docs/bugbot). Repo config: [`.cursor/config/bugbot.yaml`](../.cursor/config/bugbot.yaml). If the gate times out, comment `bugbot run` on the PR; set repo variable `BUGBOT_GATE_STRICT=true` only if you want timeouts to fail CI.

## Pull requests

- Title or description should list `US-0NN` ids.
- Keep acceptance tables in story files in sync with new tests.