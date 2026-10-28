"""Prove two TUI test fixture files differ from official 0.163 by one CodexDD argument each."""

from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CONFLICTS = ROOT / "docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-original-direct-conflicts.json"
CASES = {
    "codex-rs/tui/src/app/tests/startup_defaults_tests.rs": {
        "current": "e60808df0448682d47af1a14b5bd8d5c52206397",
        "official": "86e0da6fd4e0550050cc1a704186a9a07038278b",
        "extra": b"        /*adaptive_family*/ None,\n",
    },
    "codex-rs/tui/src/app_server_session/workspace_roots_tests.rs": {
        "current": "d6447d22a0a1e943b1352b8c945cf7f6cf09734e",
        "official": "525eed5ece8b9c0110b3b296b68fb6c727d6d096",
        "extra": b"                        /*turn_trigger*/ None,\n",
    },
}


def git_bytes(*args: str, input_bytes: bytes | None = None) -> bytes:
    return subprocess.check_output(["git", *args], input=input_bytes, cwd=ROOT)


def git_text(*args: str) -> str:
    return git_bytes(*args).decode("ascii").strip()


class TwoExactAdditiveTuiTestFixtures(unittest.TestCase):
    def test_candidate_is_official_163_plus_only_one_codexdd_parameter(self):
        receipt = json.loads(CONFLICTS.read_text(encoding="utf-8"))
        reviewed = {r["path"]: r for r in receipt["conflicts"]}
        self.assertEqual(len(receipt["conflicts"]), 15)
        for path, expected in CASES.items():
            with self.subTest(path=path):
                original = reviewed[path]
                self.assertEqual(original["release_status"], "NOT_ACCEPTED")
                self.assertEqual(original["official_163_blob"], expected["official"])
                self.assertEqual(original["integration_reviewed_blob"], expected["current"])
                self.assertEqual(git_text("rev-parse", "HEAD:" + path), expected["current"])
                fork = git_bytes("cat-file", "blob", "HEAD:" + path)
                self.assertEqual(fork.count(expected["extra"]), 1)
                official = fork.replace(expected["extra"], b"", 1)
                official_sha = git_bytes("hash-object", "--stdin", input_bytes=official)
                self.assertEqual(official_sha.decode("ascii").strip(), expected["official"])

    def test_no_runtime_or_release_promotion(self):
        self.assertEqual(
            (ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4"
        )
        self.assertEqual(
            (ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(),
            "rust-v0.162.0",
        )


if __name__ == "__main__":
    unittest.main()
