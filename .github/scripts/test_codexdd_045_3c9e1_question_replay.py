"""CodexDD 0.4.5 3C.9e1 protected replay and resolved-question input guard.

Static source-provenance checks only, not native Rust, Windows runtime,
interactive keyboard-chord safety acceptance or release authorization.
"""

from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
PATH = "codex-rs/tui/src/chatwidget/replay.rs"
OLD_BLOB = "8b079cdb3863f0d613ec1f9eb702c142c6445133"
NEW_BLOB = "d17ff1c77f63c8c57e0a847c58377999fda78480"
OLD = "                    self.bottom_pane.question_editor().resolve_answers(&ids);"
NEW = (
    "                    let displayed_answer_changed =\n"
    "                        self.bottom_pane.question_editor().resolve_answers(&ids);\n"
    "                    if displayed_answer_changed && !self.bottom_pane.has_active_view() {\n"
    "                        self.bottom_pane.key_chord_reset_requested = true;\n"
    "                    }"
)


def git(*args: str) -> str:
    return subprocess.run(
        ["git", *args],
        cwd=ROOT,
        text=True,
        encoding="utf-8",
        capture_output=True,
        check=True,
    ).stdout.strip()


def source(path: str, ref: str = "HEAD") -> str:
    return subprocess.run(
        ["git", "show", ref + ":" + path],
        cwd=ROOT,
        text=True,
        encoding="utf-8",
        capture_output=True,
        check=True,
    ).stdout


class QuestionReplayMergeTests(unittest.TestCase):
    def test_source_blob_pins_original_and_reviewed(self):
        self.assertEqual(git("rev-parse", PRODUCTION + ":" + PATH), OLD_BLOB)
        self.assertEqual(git("rev-parse", "HEAD:" + PATH), NEW_BLOB)

    def test_exact_single_upstream_input_guard_transplant(self):
        original = source(PATH, PRODUCTION)
        self.assertEqual(original.count(OLD), 1)
        self.assertNotIn("key_chord_reset_requested", original)
        self.assertEqual(source(PATH), original.replace(OLD, NEW, 1))

    def test_question_resolution_controls_key_chord_reset(self):
        merged = source(PATH)
        self.assertEqual(merged.count("let displayed_answer_changed ="), 1)
        self.assertIn(
            "if displayed_answer_changed && !self.bottom_pane.has_active_view() {",
            merged,
        )
        self.assertIn("self.bottom_pane.key_chord_reset_requested = true;", merged)

    def test_native_question_state_returns_transition_bool(self):
        state = source("codex-rs/tui/src/bottom_pane/async_questions/state.rs")
        self.assertIn(
            "pub(crate) fn resolve_answers(&mut self, question_ids: &[String]) -> bool {",
            state,
        )

    def test_bottom_pane_exposes_only_staged_input_guard(self):
        pane = source("codex-rs/tui/src/bottom_pane/mod.rs")
        self.assertIn("pub(crate) key_chord_reset_requested: bool,", pane)
        self.assertIn("pub(crate) fn has_active_view(&self) -> bool {", pane)

    def test_worker_replay_duration_accounting_preserved(self):
        merged = source(PATH)
        old = source(PATH, PRODUCTION)
        for token in (
            "self.adaptive_effort.worker_context.role",
            "crate::adaptive_worker::AdaptiveWorkerRole::Unspecified",
            "record_worker_duration(&turn.id, duration_ms);",
        ):
            with self.subTest(token=token):
                self.assertIn(token, merged)
                self.assertIn(token, old)

    def test_after_reply_history_presentation_remains_in_order(self):
        merged = source(PATH)
        ordered = [
            "let displayed_answer_changed =",
            "key_chord_reset_requested = true;",
            "self.refresh_pending_input_preview();",
            "self.request_redraw();",
            "self.on_committed_user_message(",
        ]
        positions = [merged.index(item) for item in ordered]
        self.assertEqual(positions, sorted(positions))

    def test_result_does_not_claim_untrusted_turn_authority(self):
        merged = source(PATH)
        self.assertNotIn("run_codexdd_validation", merged)
        self.assertNotIn("report_adaptive_signal", merged)
        self.assertNotIn("unsafe {", NEW)

    def test_protected_fork_inventory_pins_reviewed_replay(self):
        guard = source(".github/scripts/test_codexdd_045_3c9c_tui_upstream.py")
        self.assertIn('"' + PATH + '": "' + NEW_BLOB + '"', guard)
        self.assertIn("self.assertEqual(len(PRESERVED_FORK), 44)", guard)

    def test_version_lockfile_and_tool_policy_unadvanced(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(
            source("codex-rs/upstream-codex-release.txt").strip(),
            "rust-v0.162.0",
        )
        self.assertEqual(
            git("rev-parse", "HEAD:codex-rs/Cargo.lock"),
            "13bc15ff844e93dfaf7777c73bb7313b35bb3b81",
        )
        self.assertEqual(
            git("rev-parse", "HEAD:codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )

    def test_checkpoint_documents_partial_status_and_config_panel_blocker(self):
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c9e1-question-replay.md").read_text(
            encoding="utf-8"
        )
        for marker in (
            "INCOMPLETE / NOT A RELEASE CANDIDATE",
            "source-only",
            "native Rust",
            "36 unresolved",
            "config_panel_tui",
            "Cargo.lock",
        ):
            with self.subTest(marker=marker):
                self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
