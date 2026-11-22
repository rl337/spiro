#!/usr/bin/env python3
"""Queue a ComfyUI API workflow (Qwen-Image or any exported graph) and save the first output PNG."""

from __future__ import annotations

import argparse
import json
import os
import sys
from pathlib import Path

_REPO = Path(__file__).resolve().parent.parent
if str(_REPO) not in sys.path:
    sys.path.insert(0, str(_REPO))

from python.comfy_client import (  # noqa: E402
    ComfyConfig,
    apply_patches,
    download_image,
    first_output_image,
    health_check,
    load_workflow,
    queue_prompt,
    wait_outputs,
)

_REPLACE_SENTINEL = "REPLACE_VIA_SHELL_PROMPT"


def substitute_prompt_text(patches: dict, prompt_text: str) -> dict:
    """Deep-copy patches and fill REPLACE_VIA_SHELL_PROMPT inputs."""
    out = json.loads(json.dumps(patches))
    for body in out.values():
        if not isinstance(body, dict):
            continue
        inputs = body.get("inputs")
        if not isinstance(inputs, dict):
            continue
        for key, val in list(inputs.items()):
            if val == _REPLACE_SENTINEL:
                inputs[key] = prompt_text
    return out


def load_patches(args: argparse.Namespace) -> dict:
    if args.patches:
        raw = json.loads(args.patches.read_text(encoding="utf-8"))
        return substitute_prompt_text(raw, args.prompt_text)
    if os.environ.get("COMFY_PATCHES_JSON"):
        raw = json.loads(os.environ["COMFY_PATCHES_JSON"])
        return substitute_prompt_text(raw, args.prompt_text)
    node = os.environ.get("COMFY_PROMPT_NODE")
    if node:
        field = os.environ.get("COMFY_PROMPT_FIELD", "text")
        return {node: {"inputs": {field: args.prompt_text}}}
    return {}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument(
        "--workflow",
        type=Path,
        required=True,
        help='Comfy Export (API) JSON (bare graph or {"prompt": graph})',
    )
    ap.add_argument(
        "--patches",
        type=Path,
        help="JSON: {node_id: {inputs: {...}}} — applied before queue",
    )
    ap.add_argument("--prompt-text", default="a red circle on white background, simple icon")
    ap.add_argument(
        "--out",
        type=Path,
        default=Path(os.environ.get("UIE_ARTIFACT_DIR", "/tmp/uie-artifacts"))
        / "comfy_qwen_image.png",
    )
    ap.add_argument(
        "--comfy-url",
        default=os.environ.get("COMFYUI_URL", "http://127.0.0.1:8188"),
    )
    args = ap.parse_args()

    cfg = ComfyConfig(base_url=args.comfy_url)
    if not health_check(cfg):
        print(
            f"ComfyUI not reachable at {args.comfy_url} — start Comfy (Qwen-Image workflow).",
            file=sys.stderr,
        )
        return 2

    wf = load_workflow(args.workflow)
    patches = load_patches(args)
    if patches:
        apply_patches(wf, patches)
    else:
        print(
            "No patches — workflow submitted as exported (set --patches or COMFY_PROMPT_NODE).",
            file=sys.stderr,
        )

    prompt_id = queue_prompt(cfg, wf)
    print(f"queued prompt_id={prompt_id}")
    outputs = wait_outputs(cfg, prompt_id)
    fn, sub, typ = first_output_image(outputs)
    data = download_image(cfg, fn, sub, typ)
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_bytes(data)
    print(f"saved {args.out} ({len(data)} bytes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
