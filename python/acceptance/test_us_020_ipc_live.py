#!/usr/bin/env python3
"""US-020 live: requires UIE_ACCEPTANCE_MODE=live and optional UIE_LIVE_MODEL_PATH."""

from __future__ import annotations

import os
import unittest

from test_us_020_ipc import Us020IpcAcceptance


class Us020IpcLiveAcceptance(Us020IpcAcceptance):
    def setUp(self) -> None:
        if os.environ.get("UIE_ACCEPTANCE_MODE", "ci").lower() not in (
            "live",
            "real",
            "local",
        ):
            self.skipTest("set UIE_ACCEPTANCE_MODE=live for live IPC acceptance")
        super().setUp()


if __name__ == "__main__":
    unittest.main()
