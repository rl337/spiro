#!/usr/bin/env python3
"""DI-first (pyiv / uie-iv) lint: env profile vars only in uie-iv (DI-VERIFY-01)."""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
CRATES = REPO / "crates"
IV_PREFIX = "crates/uie-iv/"

PATTERNS = [
    re.compile(r"""std::env::var(?:_os)?\s*\(\s*["']UIE_ACCEPTANCE_MODE"""),
    re.compile(r"""std::env::var(?:_os)?\s*\(\s*["']UIE_SERVE_PROFILE"""),
]


def scan_rust_file(rel: str, path: Path) -> list[str]:
    if rel.startswith(IV_PREFIX):
        return []
    text = path.read_text(encoding="utf-8")
    errors: list[str] = []
    for i, line in enumerate(text.splitlines(), start=1):
        for pat in PATTERNS:
            if pat.search(line):
                errors.append(f"{rel}:{i}: direct env read (use uie-iv modules / injector)")
    return errors


def main() -> int:
    if not CRATES.is_dir():
        print("verify-di-first: no crates/", file=sys.stderr)
        return 2

    errors: list[str] = []
    for path in CRATES.rglob("*.rs"):
        rel = path.relative_to(REPO).as_posix()
        errors.extend(scan_rust_file(rel, path))

    if errors:
        for e in errors:
            print(f"di-first: {e}", file=sys.stderr)
        print(
            "di-first: see docs/requirements/dependency-injection.md (DI-03, DI-VERIFY-01)",
            file=sys.stderr,
        )
        return 1
    print("di-first: OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())