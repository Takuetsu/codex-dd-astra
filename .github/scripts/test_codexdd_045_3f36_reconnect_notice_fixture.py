"""Exact source-only Windows reconnect snapshot notice fixture regression, nonrelease."""

from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SRC = "codex-rs/tui/src/app/tests/active_reconnect_tests.rs"
SNAP = "codex-rs/tui/src/app/tests/snapshots/codex_tui__app__tests__active_reconnect__unavailable_conversation.snap"
OLD = "18fbde768d8e88e954c5c266c56cd67fd21d3dec"
NEW = "3faafe11a166fca1354dfc35ff797384c68838cf"
LATEST = "fc7a9e1871d511c1453f259b08c903bf21e66b08"
INSERT = "        // The snapshot intentionally covers an unavailable thread with a server-version notice.\n        // Do not rely on the user's/default show_server_version_notice setting.\n        app.local_settings.tui.show_server_version_notice = true;\n"


def git(*args: str, input_bytes: bytes | None = None) -> bytes:
    return subprocess.check_output(["git", *args], cwd=ROOT, input=input_bytes)


def oid(b: bytes) -> str:
    return git("hash-object", "--stdin", input_bytes=b).decode("ascii").strip()


class ReconnectSnapshotVersionNoticeFixture(unittest.TestCase):
    def test_single_fixture_edit_and_exact_snapshot_preservation(self):
        current = git("cat-file", "blob", "HEAD:" + SRC)
        self.assertEqual(oid(current), LATEST)
        latest = json.loads(
            (ROOT / "docs/upstream-candidates/codexdd-045-3f38-native-two-failure-test-edits.json").read_text(encoding="utf-8")
        )["edits"][0]
        before, after = latest["before"].encode(), latest["after"].encode()
        self.assertEqual(current.count(after), 1)
        current = current.replace(after, before, 1)
        self.assertEqual(oid(current), NEW)
        extra = INSERT.encode("utf-8")
        self.assertEqual(current.count(extra), 1)
        old = current.replace(extra, b"", 1)
        self.assertEqual(oid(old), OLD)
        self.assertEqual(
            git("rev-parse", "HEAD:" + SNAP).decode("ascii").strip(),
            "335071a07742e41b50ce0eb62805f740d019b586",
        )
        snap = (ROOT / SNAP).read_text(encoding="utf-8")
        self.assertIn(
            "A background Codex service is running v2.0.0, different from your Codex CLI",
            snap,
        )
        self.assertIn("This conversation is unavailable", snap)

    def test_fixture_is_opt_in_only_for_failed_reconnect_snapshot(self):
        s = (ROOT / SRC).read_text(encoding="utf-8")
        target = s.split(
            "async fn reconnect_allows_slow_hydration_but_bounds_a_stalled_server()", 1
        )[1]
        self.assertIn(
            "app.local_settings.tui.show_server_version_notice = true;", target[:900]
        )
        self.assertEqual(s.count("app.local_settings.tui.show_server_version_notice = true;"), 1)
        self.assertIn('assert_snapshot!(\n                "unavailable_conversation",', target)
        reconnect = (ROOT / "codex-rs/tui/src/app/reconnect.rs").read_text(encoding="utf-8")
        self.assertIn("if !self.local_settings.tui.show_server_version_notice", reconnect)
        self.assertIn("self.chat_widget.add_server_version_warning(notice);", reconnect)

    def test_security_and_production_are_not_promoted(self):
        for path, expected in {
            "codex-rs/Cargo.lock": "13bc15ff844e93dfaf7777c73bb7313b35bb3b81",
            "codex-rs/Cargo.toml": "cfebd9b784ef9ff027b274aa9f7101c9c2496613",
            "codex-rs/deny.toml": "f282c7c55d05201e308e60804f85754c8522d462",
            "codex-rs/tui/src/app/startup.rs": "e51254ea2f16f2a356670413ae29c9de0d002e9d",
            "codex-rs/tui/src/app/tests/session_lifecycle_requests.rs": "2460017301fa1c57480e1092e7ce69413aa7867e",
        }.items():
            with self.subTest(path=path):
                self.assertEqual(git("rev-parse", "HEAD:" + path).decode().strip(), expected)
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")
        self.assertEqual((ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(), "rust-v0.162.0")


if __name__ == "__main__":
    unittest.main()
