"""0.4.5 3C.1: cheap, read-only source tripwires for upstream history replay.

This is NOT a Rust compile, cross-file API proof, Windows release PASS, or
approval to publish. The rest of the upstream source is not integrated yet.
"""

from __future__ import annotations

import hashlib
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
NORMALIZE = "codex-rs/core/src/context_manager/normalize.rs"
TESTS = "codex-rs/core/src/context_manager/history_tests.rs"
NORMALIZE_SHA = "4ca88c6a668e9896df06ca81940a0e41e357898d"
HISTORY_TEST_SHA = "4f8415b4ce3f3994d4fe8eb87bcbd22e6588848d"


def committed(path: str) -> bytes:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT,
        check=True,
        capture_output=True,
    ).stdout


def text(path: str) -> str:
    return committed(path).decode("utf-8")


def blob_sha(raw: bytes) -> str:
    return hashlib.sha1(
        b"blob " + str(len(raw)).encode("ascii") + b"\0" + raw
    ).hexdigest()


class CoreHistoryIntegrationChecks(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.normalization = text(NORMALIZE)
        cls.tests = text(TESTS)

    def test_normalization_merge_is_exact_source_checkpoint(self):
        self.assertEqual(blob_sha(committed(NORMALIZE)), NORMALIZE_SHA)

    def test_history_test_rebase_is_exact_source_checkpoint(self):
        self.assertEqual(blob_sha(committed(TESTS)), HISTORY_TEST_SHA)

    def test_fork_recovery_preserves_missing_custom_and_shell_outputs(self):
        for marker in (
            'synthetic_output_id("ctco", id.as_deref())',
            'synthetic_output_id("fco", id.as_deref())',
            'FunctionCallOutputPayload::from_text("aborted".to_string())',
            "ResponseItem::CustomToolCallOutput {",
            "ResponseItem::FunctionCallOutput {",
        ):
            with self.subTest(marker=marker):
                self.assertIn(marker, self.normalization)
        self.assertEqual(self.normalization.count("synthesizing aborted output"), 2)

    def test_upstream_inherited_metadata_guard_retained_for_both_tool_families(self):
        guard = ".is_some_and(|metadata| metadata.inherited_user_message)"
        self.assertEqual(self.normalization.count(guard), 2)
        self.assertIn(
            "Preserved-prefix forks mark copied calls as inherited", self.normalization
        )

    def test_interrupted_local_call_never_errors_or_panics_in_debug(self):
        for family in (
            "Custom tool call output is missing",
            "Local shell call output is missing",
        ):
            with self.subTest(family=family):
                self.assertIn(
                    family + " for call id: {call_id}; synthesizing aborted output",
                    self.normalization,
                )
                self.assertNotIn(
                    'error_or_panic(format!(\n                        "' + family,
                    self.normalization,
                )

    def test_upstream_testcases_execute_both_local_and_inherited_for_debug(self):
        self.assertEqual(self.tests.count('#[test_case::test_case(false; "local")]'), 2)
        self.assertEqual(
            self.tests.count('#[test_case::test_case(true; "inherited")]'), 2
        )
        self.assertEqual(self.tests.count("inherited_user_message = inherited;"), 2)
        self.assertNotIn("cfg_attr(not(debug_assertions), test_case::", self.tests)

    def test_only_obsolete_missing_output_debug_panics_are_removed(self):
        for name in (
            "normalize_adds_missing_output_for_custom_tool_call_panics_in_debug",
            "normalize_adds_missing_output_for_local_shell_call_with_id_panics_in_debug",
        ):
            self.assertNotIn(name, self.tests)
        for unaffected in (
            "normalize_removes_orphan_function_call_output_panics_in_debug",
            "normalize_removes_orphan_custom_tool_call_output_panics_in_debug",
        ):
            self.assertIn(unaffected, self.tests)

    def test_upstream_encrypted_compaction_replay_estimate_test_survives(self):
        self.assertIn(
            "fn encrypted_compactions_use_replay_cache_estimate()",
            self.tests,
        )

    def test_snapshot_intent_preserves_existing_product_release_markers(self):
        self.assertEqual(committed("codex-rs/codexdd-version.txt").strip(), b"0.4.4")
        self.assertEqual(
            committed("codex-rs/upstream-codex-release.txt").strip(),
            b"rust-v0.162.0",
        )

    def test_manual_cross_file_validation_remains_open(self):
        notes = ROOT / "docs/codexdd-0.4.5-packet-3c1-history-reconciliation.md"
        content = notes.read_text(encoding="utf-8")
        self.assertIn("INCOMPLETE / NOT A RELEASE CANDIDATE", content)
        self.assertIn("109", content)
        self.assertIn("15", content)
        self.assertIn("native Rust", content)


if __name__ == "__main__":
    unittest.main()
