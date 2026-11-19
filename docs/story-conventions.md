# Story conventions

Every feature in UIE should have a **user story**: something a person can read without opening Rust files.

## Story file layout

Stories live under [`docs/stories/`](./stories/). Each file is named `NNN-short-slug.md` and starts with a stable ID:

```markdown
# US-0NN: Human title

**Status:** shipped | partial | planned
**Crate / path:** where the code lives
```

## Required sections

1. **User story** — *As a … I want … so that …*
2. **In plain terms** — what happens step by step
3. **Related stories** — links to other `US-*` files
4. **Terms used** — bullet list linking into the [glossary](./glossary.md)
5. **Acceptance notes** — how we know it works today (command, test, or limitation)

## Cross-references

- **Stories → stories:** use relative links, e.g. `[US-002](./002-tensors.md)`.
- **Stories → terms:** link to glossary anchors, e.g. `[Tensor](../glossary.md#tensor)`.
- **Glossary → stories:** each term lists “See also” story IDs.

## Adding a story

1. Pick the next `US-0NN` in [`stories/README.md`](./stories/README.md).
2. Copy [`stories/_template.md`](./stories/_template.md).
3. Add glossary entries for any new terms (one definition per anchor).
4. Link the story from the README index and from related stories.