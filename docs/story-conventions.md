# Story conventions

Every feature in UIE should have a **user story**: something a person can read without opening Rust files.

## Story file layout

Stories live under [`docs/stories/`](./stories/). Each file is named `NNN-short-slug.md` and starts with a stable ID:

```markdown
# US-0NN: Human title

**Status:** shipped | partial | planned
**Personas:** Liz | Jack | … (see [personas](./personas.md))
**Crate / path:** where the code lives
```

Use [30 Rock personas](./personas.md) in the *As a …* line (first name only, e.g. **Liz**, **Kenneth**). List every persona in the **Personas** metadata line.

## Required sections

1. **User story** — *As a … I want … so that …* (persona name in *As a*)
2. **In plain terms** — what happens step by step
3. **Related stories** — links to other `US-*` files
4. **Terms used** — bullet list linking into the [glossary](./glossary.md)
5. **Acceptance notes** — how we know it works today (command, test, or limitation)
6. **Acceptance tests** — table linking to Rust (`crates/uie-acceptance/tests/us_0NN_*.rs`) and/or Python (`python/acceptance/test_us_0NN_*.py`); see [acceptance-tests.md](./acceptance-tests.md)
7. **Observability** — each linked acceptance test must assert execution **telemetry** or captured **story events** (see [contributing.md](./contributing.md))

## Cross-references

- **Stories → stories:** use relative links, e.g. `[US-002](./002-tensors.md)`.
- **Stories → terms:** link to glossary anchors, e.g. `[Tensor](../glossary.md#tensor)`.
- **Glossary → stories:** each term lists “See also” story IDs.

## Adding a story

1. Pick the next `US-0NN` in [`stories/README.md`](./stories/README.md).
2. Copy [`stories/_template.md`](./stories/_template.md).
3. Add glossary entries for any new terms (one definition per anchor).
4. Link the story from the README index and from related stories.
5. Register acceptance paths in [`config/story_governance.yaml`](../config/story_governance.yaml).