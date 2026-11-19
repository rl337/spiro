#!/usr/bin/env python3
"""
Minimal UIE client over a Unix domain socket (same framing as uie-serve).

Requires the engine daemon:
  cargo run -p uie-serve

Example:
  python3 python/uie_client.py --stub llm
"""

from __future__ import annotations

import argparse
import json
import socket
import struct
import sys
from pathlib import Path

UIE_PROTOCOL_VERSION = 1


def encode_frame(msg: dict) -> bytes:
    body = json.dumps(msg, separators=(",", ":")).encode("utf-8")
    return struct.pack("<I", len(body)) + body


def read_frame(fp) -> bytes:
    raw = fp.read(4)
    if len(raw) < 4:
        raise RuntimeError("connection closed")
    (n,) = struct.unpack("<I", raw)
    body = fp.read(n)
    if len(body) != n:
        raise RuntimeError("truncated frame")
    return body


def roundtrip(sock_path: str, msg: dict) -> dict:
    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as s:
        s.connect(sock_path)
        s.sendall(encode_frame(msg))
        body = read_frame(s.makefile("rb"))
        return json.loads(body)


def main() -> int:  # noqa: D103
    parser = argparse.ArgumentParser(description="UIE Unix socket inference client")
    parser.add_argument(
        "--socket",
        default="/tmp/uie.sock",
        help="Unix socket path (UIE_SOCKET on server)",
    )
    parser.add_argument(
        "--stub",
        default="llm",
        help="Stub name under models/stubs/<name>.stub",
    )
    parser.add_argument(
        "--repo",
        type=Path,
        default=Path(__file__).resolve().parent.parent,
        help="Repository root",
    )
    args = parser.parse_args()

    stub_path = (args.repo / "models/stubs" / f"{args.stub}.stub").resolve()
    if not stub_path.exists():
        print(f"stub not found: {stub_path}", file=sys.stderr)
        return 1

    hello = roundtrip(
        args.socket,
        {"type": "hello", "protocol_version": UIE_PROTOCOL_VERSION},
    )
    print("hello:", hello)

    # Placeholder infer: server validates tensor contracts; empty token row is rejected.
    # Use uie-stubs / Rust sample_input to build real payloads, or extend this script.
    infer = {
        "type": "infer",
        "request_id": "py-1",
        "model_path": str(stub_path),
        "inputs": [
            {
                "name": "input_ids",
                "dtype": "i32",
                "shape": [1, 4],
                "data": list(struct.pack("<4i", 100, 200, 42, 3)),
            }
        ],
    }
    result = roundtrip(args.socket, infer)
    print("result:", json.dumps(result, indent=2)[:500])
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
