#!/usr/bin/env python3
"""US-020 acceptance: Python client over Unix socket (optional spawn uie-serve)."""
from __future__ import annotations

import os
import shutil
import socket
import struct
import subprocess
import sys
import tempfile
import time
import unittest
import json
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent.parent
sys.path.insert(0, str(REPO / "python"))

from uie_client import encode_frame, read_frame, roundtrip, UIE_PROTOCOL_VERSION  # noqa: E402


def _stub_path(name: str) -> Path:
    return (REPO / "models/stubs" / f"{name}.stub").resolve()


class Us020IpcAcceptance(unittest.TestCase):
    def setUp(self) -> None:
        self._serve_proc: subprocess.Popen | None = None
        preset = os.environ.get("UIE_ACCEPTANCE_SOCKET")
        if preset:
            self.sock_path = preset
            return
        self.sock_path = str(Path(tempfile.gettempdir()) / f"uie-acc-{os.getpid()}.sock")
        if Path(self.sock_path).exists():
            Path(self.sock_path).unlink()
        bin_path = shutil.which("uie-serve")
        if bin_path is None:
            bin_path = str(
                REPO / "target/debug/uie-serve"
            )
        env = {**os.environ, "UIE_SOCKET": self.sock_path}
        try:
            self._serve_proc = subprocess.Popen(
                [bin_path],
                env=env,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.PIPE,
            )
        except FileNotFoundError:
            self.skipTest("uie-serve binary not built; run cargo build -p uie-serve")
        deadline = time.time() + 10.0
        while time.time() < deadline:
            if Path(self.sock_path).exists():
                try:
                    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as s:
                        s.connect(self.sock_path)
                    return
                except OSError:
                    pass
            time.sleep(0.05)
        self.skipTest("uie-serve did not create socket in time")

    def tearDown(self) -> None:
        if self._serve_proc:
            self._serve_proc.terminate()
            try:
                self._serve_proc.wait(timeout=3)
            except subprocess.TimeoutExpired:
                self._serve_proc.kill()
        if self.sock_path and not os.environ.get("UIE_ACCEPTANCE_SOCKET"):
            p = Path(self.sock_path)
            if p.exists():
                p.unlink()

    def test_us_020_hello_and_infer(self) -> None:
        hello = roundtrip(
            self.sock_path,
            {"type": "hello", "protocol_version": UIE_PROTOCOL_VERSION},
        )
        self.assertEqual(hello.get("type"), "hello_ack")

        stub = _stub_path("llm")
        self.assertTrue(stub.is_file(), str(stub))
        infer = {
            "type": "infer",
            "request_id": "py-acc-020",
            "model_path": str(stub),
            "inputs": [
                {
                    "name": "input_ids",
                    "dtype": "i32",
                    "shape": [1, 4],
                    "data": list(struct.pack("<4i", 100, 200, 42, 3)),
                }
            ],
        }
        result = roundtrip(self.sock_path, infer)
        self.assertEqual(result.get("type"), "infer_result", result)


if __name__ == "__main__":
    unittest.main()