"""Guard the two corrected native TUI tests without altering protected policy."""

from __future__ import annotations
import json
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
R = ROOT / "docs/upstream-candidates/codexdd-045-3f38-native-two-failure-test-edits.json"


def git(*args: str, input_bytes: bytes | None = None) -> bytes:
    return subprocess.check_output(["git", *args], cwd=ROOT, input=input_bytes)


def oid(raw: bytes) -> str:
    return git("hash-object", "--stdin", input_bytes=raw).decode().strip()


class NativePolicyReconciliation(unittest.TestCase):
    def test_exact_git_source_inversion_and_release_status(self):
        receipt = json.loads(R.read_text(encoding="utf-8"))
        self.assertEqual(receipt["scope"], "NONRELEASE_TWO_NATIVE_TEST_EXPECTATIONS_RECONCILED_WITH_EXISTING_POLICY")
        self.assertEqual(receipt["original_results"], {"passed": 6, "failed": 2})
        self.assertEqual(receipt["release_status"], "NOT_ACCEPTED")
        self.assertEqual(len(receipt["edits"]), 2)
        for entry in receipt["edits"]:
            with self.subTest(path=entry["path"]):
                current = git("cat-file", "blob", "HEAD:" + entry["path"])
                self.assertEqual(oid(current), entry["after_blob"])
                before, after = entry["before"].encode(), entry["after"].encode()
                self.assertEqual(current.count(after), 1)
                restored = current.replace(after, before, 1)
                self.assertEqual(oid(restored), entry["before_blob"])
                self.assertEqual(restored.replace(before, after, 1), current)

    def test_reconnect_version_snapshot_source_build_intent(self):
        src = (ROOT / "codex-rs/tui/src/app/tests/active_reconnect_tests.rs").read_text(encoding="utf-8")
        target = src.split("async fn reconnect_allows_slow_hydration_but_bounds_a_stalled_server()", 1)[1]
        self.assertIn('/*client_version*/ "0.0.0"', target)
        self.assertIn("app.local_settings.tui.show_server_version_notice = true;", target)
        snap_path = "codex-rs/tui/src/app/tests/snapshots/codex_tui__app__tests__active_reconnect__unavailable_conversation.snap"
        self.assertEqual(git("rev-parse", "HEAD:" + snap_path).decode().strip(), "335071a07742e41b50ce0eb62805f740d019b586")
        self.assertIn("v0.0.0.", (ROOT / snap_path).read_text(encoding="utf-8"))
        version_policy = (ROOT / "codex-rs/tui/src/update_versions.rs").read_text(encoding="utf-8")
        self.assertIn("client > server", version_policy)

    def test_api_key_daybreak_off_remains_unset_by_policy(self):
        src = (ROOT / "codex-rs/tui/src/app/tests/session_lifecycle_requests.rs").read_text(encoding="utf-8")
        self.assertIn('vec![None, None, None, Some("daybreakBlue"), None]', src)
        self.assertIn("(true, true, false)", src)
        policy = (ROOT / "codex-rs/tui/src/chatwidget/settings.rs").read_text(encoding="utf-8")
        self.assertIn("/// API-key turns need no explicit program while Daybreak is off.", policy)
        self.assertIn("self.daybreak_account_eligible() && (self.has_chatgpt_account || enabled)", policy)
        route = (ROOT / "codex-rs/tui/src/app/thread_routing.rs").read_text(encoding="utf-8")
        self.assertIn("self.chat_widget.daybreak_turn_eligible(enabled)", route)
        self.assertIn("crate::daybreak::program_for_turn(", route)

    def test_runtime_and_secure_sources_unchanged(self):
        for path, sha in {
            "codex-rs/Cargo.toml": "cfebd9b784ef9ff027b274aa9f7101c9c2496613",
            "codex-rs/Cargo.lock": "13bc15ff844e93dfaf7777c73bb7313b35bb3b81",
            "codex-rs/deny.toml": "f282c7c55d05201e308e60804f85754c8522d462",
            "codex-rs/tui/src/app/startup.rs": "e51254ea2f16f2a356670413ae29c9de0d002e9d",
            "codex-rs/tui/src/chatwidget/settings.rs": "bb747507416e7712fe4ff5832446783c0127994e",
            "codex-rs/tui/src/update_versions.rs": "b0aa3deb4766830f29bd9166ae0dd4ba2d697425",
        }.items():
            with self.subTest(path=path):
                self.assertEqual(git("rev-parse", "HEAD:" + path).decode().strip(), sha)
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")
        self.assertEqual((ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(), "rust-v0.162.0")


if __name__ == "__main__":
    unittest.main()
