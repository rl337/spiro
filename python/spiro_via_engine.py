#!/usr/bin/env python3
"""
Drive the Rust Spiro-style tensor path through uie-serve (stub or future workload).

Today this demonstrates the byte channel: Python sends an infer request; the engine
returns tensors. To paint output.jpg via Rust, use `cargo run -p uie-spiro` or load
a dedicated spiro workload manifest once exposed over IPC.

  Terminal 1: cargo run -p uie-serve
  Terminal 2: python3 python/spiro_via_engine.py
"""
from pathlib import Path

from uie_client import roundtrip, UIE_PROTOCOL_VERSION

REPO = Path(__file__).resolve().parent.parent
STUB = REPO / "models/stubs/llm.stub"


def main() -> None:
    sock = "/tmp/uie.sock"
    roundtrip(sock, {"type": "hello", "protocol_version": UIE_PROTOCOL_VERSION})
    # Example: same IPC path a future `spiro.stub` workload would use for canvas tensors.
    print("Connected to UIE engine on", sock)
    print("Stub path for experiments:", STUB)
    print("For full Spiro JPEG output, run: cargo run -p uie-spiro")


if __name__ == "__main__":
    main()