"""Minimal ComfyUI HTTP client (queue workflow, poll, download output images)."""

from __future__ import annotations

import json
import time
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class ComfyConfig:
    base_url: str = "http://127.0.0.1:8188"

    def api(self, path: str) -> str:
        return f"{self.base_url.rstrip('/')}{path}"


def load_workflow(path: Path) -> dict[str, Any]:
    with path.open(encoding="utf-8") as f:
        data = json.load(f)
    if "prompt" in data and isinstance(data["prompt"], dict):
        return data["prompt"]
    return data


def apply_patches(workflow: dict[str, Any], patches: dict[str, Any]) -> None:
    """patches: { \"node_id\": { \"inputs\": { \"text\": \"...\" } } }"""
    for node_id, body in patches.items():
        if node_id not in workflow:
            raise KeyError(f"workflow has no node {node_id!r}")
        node = workflow[node_id]
        for key, val in body.get("inputs", {}).items():
            node.setdefault("inputs", {})[key] = val


def queue_prompt(cfg: ComfyConfig, workflow: dict[str, Any]) -> str:
    payload = json.dumps({"prompt": workflow}).encode("utf-8")
    req = urllib.request.Request(
        cfg.api("/prompt"),
        data=payload,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=120) as resp:
        body = json.loads(resp.read().decode())
    return str(body["prompt_id"])


def wait_outputs(
    cfg: ComfyConfig, prompt_id: str, poll_s: float = 1.0, timeout_s: float = 3600.0
) -> dict[str, Any]:
    deadline = time.monotonic() + timeout_s
    while time.monotonic() < deadline:
        req = urllib.request.Request(cfg.api(f"/history/{prompt_id}"))
        try:
            with urllib.request.urlopen(req, timeout=60) as resp:
                history = json.loads(resp.read().decode())
        except urllib.error.HTTPError as e:
            if e.code == 404:
                time.sleep(poll_s)
                continue
            raise
        entry = history.get(prompt_id)
        if entry and entry.get("outputs"):
            return entry["outputs"]
        time.sleep(poll_s)
    raise TimeoutError(f"ComfyUI prompt {prompt_id} did not finish within {timeout_s}s")


def download_image(
    cfg: ComfyConfig,
    filename: str,
    subfolder: str = "",
    folder_type: str = "output",
) -> bytes:
    params = urllib.parse.urlencode(
        {"filename": filename, "subfolder": subfolder, "type": folder_type}
    )
    req = urllib.request.Request(cfg.api(f"/view?{params}"))
    with urllib.request.urlopen(req, timeout=120) as resp:
        return resp.read()


def first_output_image(outputs: dict[str, Any]) -> tuple[str, str, str]:
    for _nid, out in outputs.items():
        for img in out.get("images") or []:
            return (
                str(img["filename"]),
                str(img.get("subfolder") or ""),
                str(img.get("type") or "output"),
            )
    raise RuntimeError("no images in Comfy outputs")


def health_check(cfg: ComfyConfig) -> bool:
    try:
        urllib.request.urlopen(cfg.api("/system_stats"), timeout=5)
        return True
    except OSError:
        return False
