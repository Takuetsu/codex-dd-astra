"""Exact-source 0.163 startup behavior restoration, preserving CodexDD adaptive startup."""

from __future__ import annotations
import json
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
P = "codex-rs/tui/src/app/startup.rs"
RECEIPT = ROOT / "docs/upstream-candidates/codexdd-045-3f32-startup-restoration-edits.json"
MATRIX = ROOT / "docs/upstream-candidates/codexdd-045-109-semantic-owner-matrix.json"
CORRECTIONS = ROOT / "docs/upstream-candidates/codexdd-045-reviewed-semantic-corrections.json"


def git_bytes(*args: str) -> bytes:
    return subprocess.check_output(["git", *args], cwd=ROOT)


def git_hash(data: bytes) -> str:
    return subprocess.check_output(
        ["git", "hash-object", "--stdin"], input=data, cwd=ROOT
    ).decode("ascii").strip()


class OriginalUpstreamStartupBehaviorRestored(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.receipt = json.loads(RECEIPT.read_text(encoding="utf-8"))
        cls.matrix = json.loads(MATRIX.read_text(encoding="utf-8"))
        cls.corrections = json.loads(CORRECTIONS.read_text(encoding="utf-8"))

    def test_exact_git_blob_reconstruction_and_three_bounded_upstream_edits(self):
        doc = self.receipt
        self.assertEqual(doc["path"], P)
        self.assertEqual(doc["scope"], "NONRELEASE_PINNED_OFFICIAL_163_STARTUP_RECOVERY")
        self.assertEqual(doc["release_status"], "NOT_ACCEPTED")
        self.assertEqual(doc["before_blob"], "f092c3cf69b8302609f975ee2931e8fd15aa6797")
        self.assertEqual(doc["after_blob"], "e51254ea2f16f2a356670413ae29c9de0d002e9d")
        self.assertEqual(doc["official_upstream_blob"], "4c329f378702414c3c17cc31802c778a8dbb2266")
        previous = git_bytes("cat-file", "blob", doc["before_blob"])
        self.assertEqual(git_hash(previous), doc["before_blob"])
        self.assertEqual(len(doc["edits"]), 3)
        for edit in doc["edits"]:
            start, after = edit["before"].encode(), edit["after"].encode()
            self.assertEqual(previous.count(start), edit["count"])
            previous = previous.replace(start, after)
        current = git_bytes("cat-file", "blob", "HEAD:" + P)
        self.assertEqual(previous, current)
        self.assertEqual(git_hash(current), doc["after_blob"])

    def test_upstream_startup_lifecycle_guards_and_adaptive_preservation(self):
        source = (ROOT / P).read_text(encoding="utf-8")
        for expected in (
            "app.right_click_paste_environment.wsl",
            "app.local_settings.tui.right_click_paste",
            "app.local_settings.transcript_mode",
            "tui.clipboard.warm_text_reader(tui.frame_requester())",
            "break Err(err);",
            "if let Some(task) = app.daemon_restart.take()",
            "failed to finish background server restart during exit",
            "app.finish_reconnect(tui, &mut app_server, &mut app_event_rx, connected, CODEX_CLI_VERSION).await",
            "adaptive_startup_preset: adaptive_family",
            "activate_adaptive_startup(family)",
            "Err(err) if err.kind() == std::io::ErrorKind::TimedOut",
        ):
            with self.subTest(expected=expected):
                self.assertIn(expected, source)
        self.assertNotIn(
            "app.finish_reconnect(tui, &mut app_server, &mut app_event_rx, connected, CODEX_CLI_VERSION).await?;",
            source,
        )
        self.assertEqual(source.count("if let Some(task) = app.daemon_restart.take()"), 1)
        self.assertEqual(source.count("tui.clipboard.warm_text_reader(tui.frame_requester())"), 1)

    def test_owner_matrix_history_and_nonrelease_markers(self):
        row = next(r for r in self.matrix["paths"] if r["path"] == P)
        recorded = next(r for r in self.corrections["paths"] if r["path"] == P)
        self.assertEqual(len(self.matrix["paths"]), 109)
        self.assertEqual(row["current_blob"], "e51254ea2f16f2a356670413ae29c9de0d002e9d")
        self.assertEqual(row["focused_review"]["source_blob"], "e51254ea2f16f2a356670413ae29c9de0d002e9d")
        self.assertEqual(row["focused_review"]["semantic_signoff"], "NOT_GRANTED")
        self.assertEqual(row["semantic_status"], "PENDING_OWNER_REVIEW")
        self.assertEqual(row["release_status"], "NOT_ACCEPTED")
        self.assertEqual(recorded["after_blob"], "e51254ea2f16f2a356670413ae29c9de0d002e9d")
        self.assertEqual(recorded["timeout_recovery_blob"], "f092c3cf69b8302609f975ee2931e8fd15aa6797")
        self.assertEqual(recorded["before_blob"], "3b111def463e2f52b4786b7a6db8c6e3aa7954f7")
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")
        self.assertEqual((ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(), "rust-v0.162.0")


if __name__ == "__main__":
    unittest.main()
