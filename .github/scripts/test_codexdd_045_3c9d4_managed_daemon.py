"""CodexDD 0.4.5 3C.9d4 managed-daemon TUI handoff static source checks.

Exact provenance and source invariants only; NOT a native Rust compile,
Windows elevation/runtime proof, broader TUI upstream acceptance or a release gate.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
AFTER_3C9D3 = "99c40e0650e811487eb4bc2cd22c21eecc3b225c"
AFTER_3C9D4 = "1cf92eabd6bc9e7217c70a24c15182a58b5f9d28"
START = "codex-rs/tui/src/startup_orchestration.rs"
LIB = "codex-rs/tui/src/lib.rs"
APP = "codex-rs/tui/src/app/startup.rs"
ORIGINAL = {
    START: "e03aa30b4723717cfc72935d118e074fbf4f2aae",
    LIB: "83ad7124e531be410477bd097cde8950bca40ed2",
    APP: "86f1afec7ce715799c9fa62859e53f312918d067",
}
REVIEWED = {
    START: "6b3bf4a9cf3d639107126b29992690b19bf4c932",
    LIB: "a40147cca1333114d82e0335521279254f5c1f21",
    APP: "3dd4fd65185f407e01f41df7544a97f57f9048fa",
}
START_3C9D3_BLOB = "06ea648374cbdcc3ae7276ba146f4dbe45da3118"


def git(*args: str) -> str:
    return subprocess.run(
        ["git", *args], cwd=ROOT, check=True, capture_output=True,
        text=True, encoding="utf-8",
    ).stdout.strip()


def source(path: str, ref: str = "HEAD") -> str:
    return subprocess.run(
        ["git", "show", ref + ":" + path], cwd=ROOT, check=True,
        capture_output=True, text=True, encoding="utf-8",
    ).stdout


def replace_once(text: str, before: str, after: str) -> str:
    if text.count(before) != 1:
        raise AssertionError("non-unique or missing merge anchor: " + repr(before))
    return text.replace(before, after, 1)


class ManagedDaemonHandoffSourceTests(unittest.TestCase):
    def test_three_original_and_reviewed_blob_pins(self):
        for path, old in ORIGINAL.items():
            with self.subTest(path=path):
                self.assertEqual(git("rev-parse", PRODUCTION + ":" + path), old)
                self.assertEqual(git("rev-parse", AFTER_3C9D4 + ":" + path), REVIEWED[path])

    def test_3c9d3_source_baseline_is_pinned(self):
        self.assertEqual(git("rev-parse", AFTER_3C9D3 + ":" + START), START_3C9D3_BLOB)

    def test_startup_orchestration_only_adds_handoff_to_3c9d3_source(self):
        baseline = source(START, AFTER_3C9D3)
        expected = replace_once(
            baseline,
            "        managed_worktree.clone(),\n"
            "        daemon_startup_warning,\n"
            "        launch_telemetry,\n"
            "        startup_draft,",
            "        managed_worktree.clone(),\n"
            "        daemon_startup_warning,\n"
            "        managed_daemon,\n"
            "        launch_telemetry,\n"
            "        startup_draft,",
        )
        self.assertEqual(source(START), expected)

    def test_protected_tui_lib_only_adds_managed_handoff(self):
        baseline = source(LIB, PRODUCTION)
        expected = replace_once(
            baseline,
            "    managed_worktree: Option<ManagedTuiWorktree>,\n"
            "    daemon_startup_warning: Option<String>,\n"
            "    launch_telemetry: daemon_telemetry::Launch",
            "    managed_worktree: Option<ManagedTuiWorktree>,\n"
            "    daemon_startup_warning: Option<String>,\n"
            "    managed_daemon: bool,\n"
            "    launch_telemetry: daemon_telemetry::Launch",
        )
        expected = replace_once(
            expected,
            "        startup_hooks_browser,\n"
            "        daemon_startup_warning,\n"
            "        adaptive\n"
            "            .as_deref()",
            "        startup_hooks_browser,\n"
            "        daemon_startup_warning,\n"
            "        managed_daemon,\n"
            "        adaptive\n"
            "            .as_deref()",
        )
        self.assertEqual(source(LIB, AFTER_3C9D4), expected)

    def test_protected_app_startup_only_adds_managed_handoff_and_status(self):
        baseline = source(APP, PRODUCTION)
        expected = replace_once(
            baseline,
            "        startup_hooks_browser: Option<HooksListEntry>,\n"
            "        daemon_startup_warning: Option<String>,\n"
            "        adaptive_family: Option<crate::chatwidget::adaptive_effort::AdaptiveFamily>,",
            "        startup_hooks_browser: Option<HooksListEntry>,\n"
            "        daemon_startup_warning: Option<String>,\n"
            "        managed_daemon: bool,\n"
            "        adaptive_family: Option<crate::chatwidget::adaptive_effort::AdaptiveFamily>,",
        )
        expected = replace_once(
            expected,
            "        let remote_connection = crate::status::remote_connection::remote_connection_status_value(\n"
            "            &app_server_target,\n"
            "            app_server.server_version(),\n"
            "        );\n"
            "        let initial_server_version_notice =",
            "        let mut remote_connection = crate::status::remote_connection::remote_connection_status_value(\n"
            "            &app_server_target,\n"
            "            app_server.server_version(),\n"
            "        );\n"
            "        if let Some(connection) = &mut remote_connection {\n"
            "            connection.is_managed_daemon = connection.is_local_daemon && managed_daemon;\n"
            "        }\n"
            "        let initial_server_version_notice =",
        )
        self.assertEqual(source(APP, AFTER_3C9D4), expected)

    def test_tui_call_chain_type_and_argument_alignment(self):
        startup = source(START)
        lib = source(LIB)
        app = source(APP)
        self.assertIn(
            "        daemon_startup_warning,\n"
            "        managed_daemon,\n"
            "        launch_telemetry,",
            startup,
        )
        self.assertIn(
            "    daemon_startup_warning: Option<String>,\n"
            "    managed_daemon: bool,\n"
            "    launch_telemetry:",
            lib,
        )
        self.assertIn(
            "        daemon_startup_warning,\n"
            "        managed_daemon,\n"
            "        adaptive\n",
            lib,
        )
        self.assertIn(
            "        daemon_startup_warning: Option<String>,\n"
            "        managed_daemon: bool,\n"
            "        adaptive_family:",
            app,
        )

    def test_only_verified_local_daemon_can_receive_managed_status(self):
        app = source(APP)
        self.assertEqual(
            app.count(
                "connection.is_managed_daemon = connection.is_local_daemon && managed_daemon;"
            ),
            1,
        )
        self.assertIn("if let Some(connection) = &mut remote_connection {", app)
        remote_status = source("codex-rs/tui/src/status/remote_connection.rs")
        self.assertIn("is_managed_daemon: false,", remote_status)
        self.assertIn("AppServerTarget::Embedded => return None,", remote_status)
        self.assertIn(
            "is_local_daemon: matches!(app_server_target, AppServerTarget::LocalDaemon { .. }),",
            remote_status,
        )

    def test_provenance_starts_only_with_cli_or_verified_daemon_backend(self):
        startup = source(START)
        cli = source("codex-rs/tui/src/cli.rs")
        self.assertIn("let mut managed_daemon = cli.managed_daemon;", startup)
        self.assertIn("managed_daemon = output.backend.is_some();", startup)
        self.assertIn("pub managed_daemon: bool,", cli)
        self.assertIn("Internal: the launching CLI verified", cli)

    def test_f2_elevated_fallback_stays_fail_closed(self):
        startup = source(START)
        daemon = source("codex-rs/tui/src/daemon_startup.rs")
        self.assertIn("elevated_windows_implicit_exclusion(", startup)
        self.assertIn("windows_automatic_fallback_reason(&err)", startup)
        self.assertIn(
            "daemon_startup::automatic_exclusion_warning(daemon_exclusion, auto_start_daemon)",
            startup,
        )
        self.assertIn("reason != ELEVATED_WINDOWS_EMBEDDED_REASON", daemon)
        self.assertNotIn("cli.no_daemon = true;", startup)

    def test_adaptive_resume_worker_startup_not_overwritten(self):
        self.assertIn("crate::adaptive_policy::AdaptiveFamily::Luna", source(START))
        lib = source(LIB)
        self.assertIn("mod adaptive_controller;", lib)
        self.assertIn(
            ".and_then(crate::chatwidget::adaptive_effort::AdaptiveFamily::parse)",
            lib,
        )
        app = source(APP)
        self.assertIn("adaptive_startup_preset: adaptive_family,", app)
        self.assertIn("activate_adaptive_startup(family)", app)
        self.assertIn("worker_binding", app)

    def test_preserved_fork_inventory_pins_seven_reviewed_sources(self):
        guard = source(".github/scripts/test_codexdd_045_3c9c_tui_upstream.py")
        for path in (START, LIB, APP):
            self.assertIn('"' + path + '": "' + git("rev-parse", "HEAD:" + path) + '"', guard)
        self.assertIn("self.assertEqual(len(PRESERVED_FORK), 44)", guard)

    def test_release_controls_unadvanced(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(
            source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0"
        )
        self.assertEqual(
            git("rev-parse", "HEAD:codex-rs/Cargo.lock"),
            "0c913b46674d3593ee84f1fee5508d16e33efae5",
        )

    def test_checkpoint_does_not_claim_native_or_release_acceptance(self):
        path = ROOT / "docs/codexdd-0.4.5-packet-3c9d4-managed-daemon-handoff.md"
        note = path.read_text(encoding="utf-8")
        for marker in (
            "INCOMPLETE / NOT A RELEASE CANDIDATE",
            "source-only",
            "native Rust",
            "37 remain unresolved",
            "Cargo.lock",
        ):
            with self.subTest(marker=marker):
                self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
