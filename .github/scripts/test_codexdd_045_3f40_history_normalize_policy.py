"""Nonrelease source-bound audit for core history recovery versus debug orphan policy."""

from __future__ import annotations

import json
import re
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
NORMALIZE = "codex-rs/core/src/context_manager/normalize.rs"
HISTORY = "codex-rs/core/src/context_manager/history_tests.rs"
MATRIX = ROOT / "docs/upstream-candidates/codexdd-045-109-semantic-owner-matrix.json"
TUI_WORKFLOW = ROOT / ".github/workflows/codexdd-045-3f39-startup-session-windows-native.yml"


def git_sha(path: str) -> str:
    return subprocess.check_output(
        ["git", "rev-parse", "HEAD:" + path], cwd=ROOT, text=True
    ).strip()


class HistoryNormalizationForkPolicy(unittest.TestCase):
    def test_immutable_fork_behavior_for_interrupted_and_inherited_calls(self):
        self.assertEqual(git_sha(NORMALIZE), "4ca88c6a668e9896df06ca81940a0e41e357898d")
        self.assertEqual(git_sha(HISTORY), "4f8415b4ce3f3994d4fe8eb87bcbd22e6588848d")
        code = (ROOT / NORMALIZE).read_text(encoding="utf-8")
        self.assertIn("metadata.inherited_user_message", code)
        for expression in (
            "Custom tool call output is missing for call id:",
            "Local shell call output is missing for call id:",
            "synthesizing aborted output",
            'synthetic_output_id("ctco", id.as_deref())',
            'synthetic_output_id("fco", id.as_deref())',
            'FunctionCallOutputPayload::from_text("aborted".to_string())',
        ):
            with self.subTest(expression=expression):
                self.assertIn(expression, code)
        self.assertIn("error_or_panic(format!(\n                    \"Orphan custom tool call output", code)
        self.assertIn("error_or_panic(format!(\n                    \"Orphan function call output", code)

    def test_debug_and_replay_regressions_both_remain_registered(self):
        text = (ROOT / HISTORY).read_text(encoding="utf-8")
        for selector in (
            "normalize_adds_missing_output_for_custom_tool_call",
            "normalize_adds_missing_output_for_local_shell_call_with_id",
        ):
            with self.subTest(selector=selector):
                self.assertIn('#[test_case::test_case(false; "local")]', text)
                self.assertIn('#[test_case::test_case(true; "inherited")]', text)
                self.assertRegex(text, r"fn " + selector + r"\(inherited: bool\)")
        for selector in (
            "normalize_removes_orphan_function_call_output_panics_in_debug",
            "normalize_removes_orphan_custom_tool_call_output_panics_in_debug",
            "normalize_removes_orphan_client_tool_search_output_panics_in_debug",
        ):
            with self.subTest(selector=selector):
                self.assertRegex(
                    text,
                    r"(?s)#\[cfg\(debug_assertions\)\]\s*#\[test\]\s*#\[should_panic\]\s*fn "
                    + selector
                    + r"\(",
                )
        self.assertIn("normalize_keeps_server_tool_search_output_without_matching_call", text)
        self.assertIn("normalize_adds_missing_output_for_tool_search_call", text)

    def test_eight_native_tui_results_recorded_but_not_owner_signoff(self):
        m = json.loads(MATRIX.read_text(encoding="utf-8"))
        self.assertEqual(len(m["paths"]), 109)
        startup = next(r for r in m["paths"] if r["path"] == "codex-rs/tui/src/app/startup.rs")
        session = next(r for r in m["paths"] if r["path"] == "codex-rs/tui/src/app/tests/session_lifecycle_requests.rs")
        core = next(r for r in m["paths"] if r["path"] == NORMALIZE)
        self.assertEqual(startup["focused_review"]["focused_native_run_3f39"], 40760413516)
        self.assertEqual(session["session_test_reconciliation"]["native_run_3f39"], 40760413516)
        self.assertEqual(startup["focused_review"]["semantic_signoff"], "NOT_GRANTED")
        self.assertEqual(session["session_test_reconciliation"]["semantic_signoff"], "NOT_GRANTED")
        self.assertEqual(core["focused_review"]["semantic_signoff"], "NOT_GRANTED")
        self.assertEqual(startup["current_blob"], git_sha(startup["path"]))
        self.assertEqual(session["current_blob"], git_sha(session["path"]))
        for row in m["paths"]:
            self.assertEqual(row["semantic_status"], "PENDING_OWNER_REVIEW")
            self.assertEqual(row["release_status"], "NOT_ACCEPTED")
        workflow = TUI_WORKFLOW.read_text(encoding="utf-8")
        self.assertIn("Run all eight startup, reconnect, daemon, Daybreak", workflow)
        self.assertIn("if ($index -ne 8)", workflow)

    def test_locked_sources_and_product_are_not_promoted(self):
        pins = {
            "codex-rs/Cargo.lock": "13bc15ff844e93dfaf7777c73bb7313b35bb3b81",
            "codex-rs/Cargo.toml": "cfebd9b784ef9ff027b274aa9f7101c9c2496613",
            "codex-rs/deny.toml": "f282c7c55d05201e308e60804f85754c8522d462",
        }
        for path, expected in pins.items():
            with self.subTest(path=path):
                self.assertEqual(git_sha(path), expected)
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")
        self.assertEqual((ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(), "rust-v0.162.0")


if __name__ == "__main__":
    unittest.main()
