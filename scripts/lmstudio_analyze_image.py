#!/usr/bin/env python3
"""LM Studio OpenAI-compatible chat with an image (vision GGUF required)."""

from __future__ import annotations

import argparse
import os
import sys
from pathlib import Path

_REPO = Path(__file__).resolve().parent.parent
if str(_REPO) not in sys.path:
    sys.path.insert(0, str(_REPO))

from python.lmstudio_client import LmStudioConfig, chat, health_check  # noqa: E402


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--image", type=Path, required=True)
    ap.add_argument(
        "--question",
        default="Describe this image in 2–3 sentences. What colors and shapes do you see?",
    )
    ap.add_argument(
        "--base-url",
        default=os.environ.get("LMSTUDIO_BASE_URL", "http://127.0.0.1:1234/v1"),
    )
    ap.add_argument(
        "--model",
        default=os.environ.get("LMSTUDIO_VISION_MODEL"),
        help="Vision model in LM Studio (e.g. Qwen2.5-VL). Text-only Qwen3-2B cannot read images.",
    )
    ap.add_argument("--max-tokens", type=int, default=256)
    args = ap.parse_args()

    if not args.image.is_file():
        print(f"missing image: {args.image}", file=sys.stderr)
        return 2

    cfg = LmStudioConfig(base_url=args.base_url, model=args.model)
    if not health_check(cfg):
        print(
            f"LM Studio API not reachable at {args.base_url} — enable the local server.",
            file=sys.stderr,
        )
        return 2

    if cfg.model and "vl" not in cfg.model.lower() and "vision" not in cfg.model.lower():
        print(
            f"warning: {cfg.model!r} may be text-only; use Qwen2.5-VL or Qwen3-VL for images.",
            file=sys.stderr,
        )

    answer = chat(cfg, args.question, image_path=args.image, max_tokens=args.max_tokens)
    print(answer)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
