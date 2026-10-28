"""Guard intentional CodexDD non-panic history recovery against accidental upstream replacement."""

from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
NORMALIZE = "codex-rs/core/src/context_manager/normalize.rs"
HISTORY = "codex-rs/core/src/context_manager/history_tests.rs"
CONFLICTS = ROOT / "docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-original-direct-conflicts.json"
SOURCE_PINS = {
    NORMALIZE: "4ca88c6a668e9896df06ca81940a0e41e357898d",
    HISTORY: "4f8415b4ce3f3994d4fe8eb87bcbd22e6588848d",
}


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


class HistoryRecoveryForkSemantics(unittest.TestCase):
    def test_exact_conflict_source_pins_and_no_release_promotion(self):
        conflicts = json.loads(CONFLICTS.read_text(encoding="utf-8"))
        rows = {r["path"]: r for r in conflicts["conflicts"]}
        self.assertEqual(len(rows), 15)
        for path, sha in SOURCE_PINS.items():
            with self.subTest(path=path):
                self.assertEqual(git("rev-parse", "HEAD:" + path), sha)
                self.assertEqual(rows[path]["release_status"], "NOT_ACCEPTED")
        self.assertEqual(
            (ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4"
        )
        self.assertEqual(
            (ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(),
            "rust-v0.162.0",
        )

    def test_both_local_and_inherited_cases_are_exercised_under_debug(self):
        tests = (ROOT / HISTORY).read_text(encoding="utf-8")
        for name in (
            "normalize_adds_missing_output_for_custom_tool_call",
            "normalize_adds_missing_output_for_local_shell_call_with_id",
        ):
            with self.subTest(case=name):
                marker = (
                    '#[test_case::test_case(false; "local")]\n'
                    '#[test_case::test_case(true; "inherited")]\n'
                    "fn " + name + "(inherited: bool)"
                )
                self.assertIn(marker, tests)
                body = tests.split("fn " + name + "(inherited: bool)", 1)[1][:3600]
                self.assertIn("h.normalize_history(&default_input_modalities());", body)
                self.assertIn('from_text("aborted".to_string())', body)
        for missing_upstream_debug_panic in (
            "normalize_adds_missing_output_for_custom_tool_call_panics_in_debug",
            "normalize_adds_missing_output_for_local_shell_call_with_id_panics_in_debug",
        ):
            self.assertNotIn("fn " + missing_upstream_debug_panic + "(", tests)

    def test_normalizer_preserves_local_nonpanic_and_unrelated_orphan_guards(self):
        source = (ROOT / NORMALIZE).read_text(encoding="utf-8")
        custom = source.split("ResponseItem::CustomToolCall { id, call_id, .. }", 1)[1].split(
            "// LocalShellCall is represented", 1
        )[0]
        shell = source.split("// LocalShellCall is represented", 1)[1].split(
            "            _ => {}", 1
        )[0]
        for case, fragment in (("custom", custom), ("shell", shell)):
            with self.subTest(case=case):
                self.assertIn("metadata.inherited_user_message", fragment)
                self.assertIn("synthesizing aborted output", fragment)
                self.assertIn('from_text("aborted".to_string())', fragment)
                self.assertNotIn("error_or_panic(", fragment)
        self.assertIn("pub(crate) fn remove_orphan_outputs(", source)
        self.assertIn('error_or_panic(format!("Orphan tool search output', source)


if __name__ == "__main__":
    unittest.main()
