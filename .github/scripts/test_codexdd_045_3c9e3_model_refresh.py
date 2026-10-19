"""CodexDD 0.4.5 3C.9e3 source-only API-key session model refresh checks.

Provenance and structural checks only, not native Rust compilation,
runtime model-fetch acceptance, or release authorization.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
PATH = "codex-rs/tui/src/chatwidget/session_flow.rs"
OLD_BLOB = "73d7f75b26e5de276be3357a5fda836c24b4090b"
NEW_BLOB = "17500409f70dfda7fa1c489258fa19ab01bed747"
OLD = (
    "        if previous_thread_id != self.thread_id {\n"
    "            self.backend_banner_notice_model = None;"
)
NEW = (
    "        if previous_thread_id != self.thread_id {\n"
    "            if (previous_thread_id.is_some()\n"
    "                || (self.remote_connection.is_some() && self.config.cwd != session.cwd))\n"
    "                && self.status_account_display == Some(StatusAccountDisplay::ApiKey)\n"
    "            {\n"
    "                self.refresh_account_scoped_models();\n"
    "            }\n"
    "            self.backend_banner_notice_model = None;"
)


def git(*args: str) -> str:
    return subprocess.run(
        ["git", *args], cwd=ROOT, text=True, encoding="utf-8",
        check=True, capture_output=True,
    ).stdout.strip()


def source(path: str, ref: str = "HEAD") -> str:
    return subprocess.run(
        ["git", "show", ref + ":" + path], cwd=ROOT,
        text=True, encoding="utf-8", check=True, capture_output=True,
    ).stdout


class SessionModelRefreshSourceTests(unittest.TestCase):
    def test_pins_fork_and_reviewed_file(self):
        self.assertEqual(git("rev-parse", PRODUCTION + ":" + PATH), OLD_BLOB)
        self.assertEqual(git("rev-parse", "HEAD:" + PATH), NEW_BLOB)

    def test_only_reviewed_six_line_upstream_insert(self):
        original = source(PATH, PRODUCTION)
        self.assertEqual(original.count(OLD), 1)
        self.assertEqual(source(PATH), original.replace(OLD, NEW, 1))

    def test_only_existing_thread_change_may_refresh(self):
        merged = source(PATH)
        self.assertIn(
            "if previous_thread_id != self.thread_id {\n"
            "            if (previous_thread_id.is_some()",
            merged,
        )

    def test_remote_cwd_trigger_cannot_bypass_account_gate(self):
        merged = source(PATH)
        self.assertIn(
            "(self.remote_connection.is_some() && self.config.cwd != session.cwd)",
            merged,
        )
        self.assertIn(
            "&& self.status_account_display == Some(StatusAccountDisplay::ApiKey)",
            merged,
        )

    def test_refresh_invocation_once_and_before_banner_reset(self):
        merged = source(PATH)
        self.assertEqual(merged.count("self.refresh_account_scoped_models();"), 1)
        self.assertLess(
            merged.index("self.refresh_account_scoped_models();"),
            merged.index("self.backend_banner_notice_model = None;"),
        )

    def test_staged_model_helper_retains_access_program_scrubbing(self):
        helper = source("codex-rs/tui/src/chatwidget/model_popup_state.rs")
        self.assertIn("fn refresh_account_scoped_models(&mut self) {", helper)
        self.assertIn("Some(StatusAccountDisplay::ApiKey)", helper)
        self.assertIn("model.available_access_programs = None;", helper)
        self.assertIn("self.app_event_tx.send(AppEvent::FetchModels { request_id });", helper)

    def test_protected_worker_adaptive_session_restore_retained(self):
        merged = source(PATH)
        self.assertIn("self.adaptive_effort = session.adaptive_effort.clone();", merged)
        self.assertIn("self.adaptive_effort.current_family", merged)
        self.assertIn("self.adaptive_effort.current_effort", merged)
        self.assertIn("AppEvent::UpdateModel(model.to_string())", merged)
        self.assertIn("AppEvent::UpdateReasoningEffort(Some(effort))", merged)

    def test_connector_scope_and_permission_handoff_untouched(self):
        merged = source(PATH)
        self.assertIn("self.invalidate_connector_scope();", merged)
        self.assertIn("set_permission_profile_from_session_snapshot(", merged)
        self.assertIn("self.turn_lifecycle.reset_thread();", merged)

    def test_tui_fork_guard_pins_reviewed_session_source(self):
        guard = source(".github/scripts/test_codexdd_045_3c9c_tui_upstream.py")
        self.assertIn('"' + PATH + '": "' + NEW_BLOB + '"', guard)
        self.assertIn("self.assertEqual(len(PRESERVED_FORK), 44)", guard)

    def test_release_markers_and_lockfile_not_advanced(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(
            git("rev-parse", "HEAD:codex-rs/Cargo.lock"),
            "0c913b46674d3593ee84f1fee5508d16e33efae5",
        )

    def test_checkpoint_explicitly_does_not_claim_runtime_acceptance(self):
        note = (
            ROOT / "docs/codexdd-0.4.5-packet-3c9e3-model-refresh.md"
        ).read_text(encoding="utf-8")
        for marker in (
            "INCOMPLETE / NOT A RELEASE CANDIDATE",
            "source-only",
            "34 unresolved",
            "native Rust",
            "Cargo.lock",
        ):
            with self.subTest(marker=marker):
                self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
