"""LM Studio OpenAI-compatible chat (text + optional vision image)."""

from __future__ import annotations

import base64
import json
import mimetypes
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Any


@dataclass(frozen=True)
class LmStudioConfig:
    base_url: str = "http://127.0.0.1:1234/v1"
    model: str | None = None

    def chat_url(self) -> str:
        return f"{self.base_url.rstrip('/')}/chat/completions"


def _image_data_url(path: Path) -> str:
    mime, _ = mimetypes.guess_type(path.name)
    mime = mime or "image/png"
    raw = path.read_bytes()
    b64 = base64.standard_b64encode(raw).decode("ascii")
    return f"data:{mime};base64,{b64}"


def chat(
    cfg: LmStudioConfig,
    user_text: str,
    image_path: Path | None = None,
    max_tokens: int = 512,
) -> str:
    content: list[dict[str, Any]]
    if image_path is not None:
        content = [
            {"type": "text", "text": user_text},
            {
                "type": "image_url",
                "image_url": {"url": _image_data_url(image_path)},
            },
        ]
    else:
        content = user_text

    body: dict[str, Any] = {
        "messages": [{"role": "user", "content": content}],
        "max_tokens": max_tokens,
        "temperature": 0.2,
    }
    if cfg.model:
        body["model"] = cfg.model

    req = urllib.request.Request(
        cfg.chat_url(),
        data=json.dumps(body).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with urllib.request.urlopen(req, timeout=600) as resp:
        data = json.loads(resp.read().decode())
    return str(data["choices"][0]["message"]["content"])


def health_check(cfg: LmStudioConfig) -> bool:
    try:
        req = urllib.request.Request(cfg.base_url.rstrip("/") + "/models")
        with urllib.request.urlopen(req, timeout=5) as resp:
            resp.read()
        return True
    except OSError:
        return False
