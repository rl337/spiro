# Contributing to UIE (Spiro)

## Check-ins and user stories

Every change that touches product code must map to a **[user story](stories/README.md)** (`US-0NN`).

1. **Story first** — Add or update `docs/stories/NNN-slug.md` (see [story conventions](./story-conventions.md)).
2. **Acceptance test** — Each story needs at least one automated acceptance test ([acceptance-tests.md](./acceptance-tests.md)).
3. **Observability assertion** — Every acceptance test must assert **telemetry** (execution trace / phases) or **captured events** relevant to that story. Helpers live in `uie_acceptance::story_assert` and `uie_acceptance::observability`.
4. **Cite the story in commits** — Commit subjects or bodies must include the `US-0NN` id(s) your change implements (e.g. `US-023: require traces on stub infer`).

CI runs [`scripts/verify-story-governance.py`](../scripts/verify-story-governance.py) before acceptance tests:

- Every `US-*` in [`config/story_governance.yaml`](../config/story_governance.yaml) has on-disk acceptance sources.
- Those sources contain an observability marker (telemetry or event capture).
- On pull requests, changed paths under `crates/`, `python/`, `models/`, or `docs/stories/` must be justified by a cited `US-0NN` in recent commits and/or an edited story file.

Agents must follow [.cursor/skills/uie-user-stories/SKILL.md](../.cursor/skills/uie-user-stories/SKILL.md).

## Running checks locally

```bash
python3 scripts/verify-story-governance.py --skip-diff   # structure only
python3 scripts/verify-story-governance.py             # includes check-in citation vs main
./scripts/acceptance-ci.sh
```

## Pull requests

- Title or description should list `US-0NN` ids.
- Keep acceptance tables in story files in sync with new tests.