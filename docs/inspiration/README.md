# Inspiration from Trouble

These notes collect ideas from the private [rl337/trouble](https://github.com/rl337/trouble) X-post ingest that are useful to **spiro / UIE**, without copying the ingest itself into this repo.

Because this repository is public, every entry uses only Trouble's **public** facet: paraphrased `summary_md`, optional `pull_quotes`, `referenced_urls`, author handle/name, and post time. Raw post text, threads, OCR, and media from Trouble's `private` facet are not copied here. The canonical public post page, when published, is `https://rl337.org/trouble/posts/<id>`.

## Layout

| Path | Purpose |
|------|---------|
| [inference-optimizations.md](./inference-optimizations.md) | Curated inference-optimization ideas, grouped by theme (18 kept items from 2026-11-24 scan) |
| [trouble-edge-inference-2026-11-24.md](./trouble-edge-inference-2026-11-24.md) | Edge / Pi-first candidate requirements (`EDGE-MEM-01` … `EDGE-ARM-04`) |
| [trouble-measurement-controls-2026-11-24.md](./trouble-measurement-controls-2026-11-24.md) | Measurement-validity amendment for `EDGE-BENCH-03` / US-023 |
| [`sources/`](./sources/) | Machine-readable ingest records (post ids scanned vs kept) |

Read order for promotion: **edge candidates** (what to build) → **measurement controls** (how to prove wins) → **inference-optimizations** (broader technique backlog).

This tree is **not** a user story. It does not live under `docs/stories/`, does not get a `US-0NN` id, and does not need an acceptance test. `scripts/verify-story-governance.py` only requires a story citation for changes under `crates/`, `python/`, `models/`, or `docs/stories/`.

## How to append a later ingest

1. Search Trouble read-only (`data/topics`, `data/tags`, `data/urls`, `data/featured`, and `data/x/posts/…/<id>.json`). Do not write to `rl337/trouble`.
2. Keep only posts that genuinely apply to UIE (loaders, graph executor, backends, KV/session, `uie-llm` / `uie-serve`).
3. Add or update theme markdown under `docs/inspiration/`. For each kept item include: `https://x.com/i/status/<id>`, author, date in America/Los_Angeles, a one-paragraph summary drawn from the public facet, referenced paper/repo URLs, and a concrete mapping to a crate or README next step.
4. Add a new JSON file `docs/inspiration/sources/YYYY-MM-DD-<theme>.json` listing query methods, counts, kept ids, and rejected-after-review ids so the next ingest can skip duplicates.
5. Open a PR against `main`. Do not merge it yourself.
