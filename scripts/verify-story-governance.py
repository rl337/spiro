#!/usr/bin/env python3
"""Verify US-0NN governance: stories ↔ acceptance tests ↔ observability assertions."""

from __future__ import annotations

import argparse
import fnmatch
import re
import subprocess
import sys
from pathlib import Path

try:
    import yaml
except ImportError:
    print("verify-story-governance: install PyYAML (pip install pyyaml)", file=sys.stderr)
    sys.exit(2)

REPO = Path(__file__).resolve().parent.parent
CONFIG = REPO / "config" / "story_governance.yaml"
STORY_ID = re.compile(r"US-\d{3}")


def load_config() -> dict:
    with CONFIG.open(encoding="utf-8") as f:
        return yaml.safe_load(f)


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def glob_match(path: str, pattern: str) -> bool:
    return fnmatch.fnmatch(path.replace("\\", "/"), pattern)


def story_doc_path(story_id: str) -> Path | None:
    n = story_id.split("-")[1]
    matches = list((REPO / "docs" / "stories").glob(f"{n}-*.md"))
    return matches[0] if matches else None


def verify_story_docs(cfg: dict) -> list[str]:
    errors: list[str] = []
    for story_id in (cfg.get("stories") or {}).keys():
        doc = story_doc_path(story_id)
        if not doc or not doc.is_file():
            errors.append(f"{story_id}: missing docs/stories file")
            continue
        body = read_text(doc)
        if "## Observability" not in body:
            errors.append(f"{story_id}: docs/stories missing ## Observability section")
    return errors


def verify_coverage(cfg: dict) -> list[str]:
    errors: list[str] = []
    markers = cfg.get("observability_markers") or []
    stories = cfg.get("stories") or {}

    checked: set[str] = set()
    for story_id, meta in stories.items():
        paths = (meta.get("acceptance") or []) + (meta.get("supplemental_acceptance") or [])
        for rel in paths:
            if rel in checked:
                continue
            checked.add(rel)
            path = REPO / rel
            if not path.is_file():
                errors.append(f"{story_id}: missing acceptance file {rel}")
                continue
            body = read_text(path)
            if not any(m in body for m in markers):
                errors.append(
                    f"{story_id}: {rel} has no telemetry/event assertion "
                    f"(need one of {markers})"
                )
    return errors


def git_changed_files(base: str) -> list[str]:
    try:
        out = subprocess.check_output(
            ["git", "diff", "--name-only", f"{base}...HEAD"],
            cwd=REPO,
            text=True,
        )
    except subprocess.CalledProcessError:
        out = subprocess.check_output(
            ["git", "diff", "--name-only", "HEAD"],
            cwd=REPO,
            text=True,
        )
    return [ln.strip() for ln in out.splitlines() if ln.strip()]


def commit_story_ids() -> set[str]:
    try:
        log = subprocess.check_output(
            ["git", "log", "--format=%s%n%b", "HEAD", "-n", "50"],
            cwd=REPO,
            text=True,
        )
    except subprocess.CalledProcessError:
        return set()
    return set(STORY_ID.findall(log))


def verify_checkins(cfg: dict, base: str) -> list[str]:
    errors: list[str] = []
    changed = git_changed_files(base)
    if not changed:
        return errors

    exempt = cfg.get("diff_exempt_globs") or []
    rules = cfg.get("diff_rules") or []
    stories = cfg.get("stories") or {}

    story_to_doc: dict[str, Path] = {}
    for doc in (REPO / "docs" / "stories").glob("*.md"):
        if doc.name.startswith("_"):
            continue
        m = re.match(r"(\d{3})-", doc.name)
        if m:
            story_to_doc[f"US-{m.group(1)}"] = doc

    cited_in_commits = commit_story_ids()
    touched_stories: set[str] = set()
    for rel in changed:
        for sid, doc in story_to_doc.items():
            if rel == f"docs/stories/{doc.name}":
                touched_stories.add(sid)

    required_stories: set[str] = set()
    for rel in changed:
        if any(glob_match(rel, pat) for pat in exempt):
            continue
        rel_posix = rel.replace("\\", "/")
        matched_rule = False
        for rule in rules:
            if glob_match(rel_posix, rule["glob"]):
                matched_rule = True
                required_stories.update(rule.get("stories") or [])
        if not matched_rule:
            continue

    if not required_stories:
        return errors

    justified = cited_in_commits | touched_stories
    # Any acceptance test file edited satisfies its own story ids.
    for rel in changed:
        for sid, meta in stories.items():
            for acc in meta.get("acceptance") or []:
                if rel == acc.replace("\\", "/"):
                    justified.add(sid)

    missing = sorted(required_stories - justified)
    if missing:
        errors.append(
            "Check-in must cite user story ID(s) in commit message (US-0NN) and/or "
            f"update docs/stories for the feature. Touched paths require one of "
            f"{sorted(required_stories)}; missing justification for: {missing}"
        )
    return errors


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--diff-base",
        default="origin/main",
        help="Git merge base for check-in story citation (default: origin/main)",
    )
    parser.add_argument(
        "--skip-diff",
        action="store_true",
        help="Only verify story ↔ acceptance ↔ observability coverage",
    )
    args = parser.parse_args()

    if not CONFIG.is_file():
        print(f"missing {CONFIG}", file=sys.stderr)
        return 2

    cfg = load_config()
    errors = verify_story_docs(cfg) + verify_coverage(cfg)
    if not args.skip_diff:
        errors.extend(verify_checkins(cfg, args.diff_base))

    if errors:
        for e in errors:
            print(f"governance: {e}", file=sys.stderr)
        return 1
    print("governance: OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())