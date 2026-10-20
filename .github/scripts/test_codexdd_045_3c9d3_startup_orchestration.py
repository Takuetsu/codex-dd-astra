"""CodexDD 0.4.5 3C.9d3: reviewed TUI startup three-way merge source guard.

Static git/source checks only. NOT native Rust compilation, elevation/runtime
verification, complete 0.163.0 TUI integration, or release acceptance.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
PATH = "codex-rs/tui/src/startup_orchestration.rs"
OLD_BLOB = "e03aa30b4723717cfc72935d118e074fbf4f2aae"
NEW_BLOB = "6b3bf4a9cf3d639107126b29992690b19bf4c932"


def git(*args: str) -> str:
    return subprocess.run(
        ["git", *args], cwd=ROOT, check=True,
        text=True, encoding="utf-8", capture_output=True,
    ).stdout.strip()


def source(path: str = PATH, ref: str = "HEAD") -> str:
    return subprocess.run(
        ["git", "show", f"{ref}:{path}"], cwd=ROOT, check=True,
        text=True, encoding="utf-8", capture_output=True,
    ).stdout


def replace_once(value: str, old: str, new: str) -> str:
    if value.count(old) != 1:
        raise AssertionError(f"non-unique 3C.9d3 merge anchor: {old!r}")
    return value.replace(old, new, 1)


class StartupOrchestrationMergeTests(unittest.TestCase):
    def test_exact_source_blobs(self):
        self.assertEqual(git("rev-parse", f"{PRODUCTION}:{PATH}"), OLD_BLOB)
        self.assertEqual(git("rev-parse", f"HEAD:{PATH}"), NEW_BLOB)

    def test_entire_fork_preserved_except_reviewed_upstream_edits_and_handoff(self):
        expected = source(ref=PRODUCTION)
        expected = replace_once(
            expected,
            "        main_execve_wrapper_exe: arg0_paths.main_execve_wrapper_exe.clone(),\n",
            "",
        )
        expected = replace_once(
            expected,
            "    let mut managed_daemon = false;\n",
            "    let mut managed_daemon = cli.managed_daemon;\n",
        )
        expected = replace_once(
            expected,
            "        daemon_recovery::check(\n"
            "            &mut startup_draft,\n"
            "            &app_server_target,\n"
            "            &config,\n"
            "            managed_daemon,\n"
            "        )",
            "        daemon_recovery::check(\n"
            "            &mut startup_draft,\n"
            "            &app_server_target,\n"
            "            &config,\n"
            "            &cli_kv_overrides,\n"
            "            managed_daemon,\n"
            "        )",
        )
        expected = replace_once(
            expected,
            "        managed_worktree.clone(),\n"
            "        daemon_startup_warning,\n"
            "        launch_telemetry,\n",
            "        managed_worktree.clone(),\n"
            "        daemon_startup_warning,\n"
            "        managed_daemon,\n"
            "        launch_telemetry,\n",
        )
        self.assertEqual(source(), expected)

    def test_elevated_windows_token_inspection_remains_fail_closed(self):
        src = source()
        self.assertIn("is_current_process_elevated().map_err(|err|", src)
        self.assertIn("Windows elevation probe failed; using embedded mode", src)
        self.assertIn("elevated_windows_implicit_exclusion(", src)
        self.assertIn("let reuse_implicit_local_daemon = daemon_exclusion.is_none();", src)

    def test_explicit_remote_and_agents_overview_retained(self):
        src = source()
        self.assertIn("&& !cli.agents_overview\n        && explicit_remote_endpoint.is_none()", src)
        self.assertIn("        cli.agents_overview,\n        explicit_remote_endpoint.is_some(),", src)

    def test_legacy_elevated_warning_and_no_daemon_mutation_not_reintroduced(self):
        src = source()
        self.assertNotIn("codex_app_server_daemon::is_elevated()", src)
        self.assertNotIn("cli.no_daemon = true;", src)
        self.assertNotIn("elevated_warning", src)

    def test_automatic_daemon_launch_fallback_preserves_reason(self):
        src = source()
        self.assertIn("windows_automatic_fallback_reason(&err)", src)
        self.assertIn(
            'daemon_exclusion = automatic_fallback_reason.or(Some("this Windows launcher"));',
            src,
        )
        self.assertIn("Ok((Some(output), None))", src)

    def test_f2_warning_policy_and_selection_reasons_retained(self):
        src = source()
        self.assertIn(
            "daemon_startup::automatic_exclusion_warning(daemon_exclusion, auto_start_daemon)",
            src,
        )
        self.assertIn('"elevated_windows_embedded"', src)
        self.assertIn('"windows_elevation_unverified_embedded"', src)

    def test_adaptive_startup_family_preserved(self):
        src = source()
        self.assertIn("crate::adaptive_policy::AdaptiveFamily::Luna", src)
        self.assertIn("            .or(model),", src)

    def test_managed_daemon_identity_captured_and_recovery_bound(self):
        src = source()
        self.assertIn("let mut managed_daemon = cli.managed_daemon;", src)
        self.assertIn("managed_daemon = output.backend.is_some();", src)
        self.assertIn(
            "            &config,\n            &cli_kv_overrides,\n            managed_daemon,",
            src,
        )
        self.assertIn("    cli_kv_overrides: &[(String, toml::Value)],\n    managed_daemon: bool,",
                      source("codex-rs/tui/src/daemon_recovery.rs"))

    def test_removed_override_field_not_reintroduced(self):
        self.assertNotIn("main_execve_wrapper_exe:", source())

    def test_managed_handoff_forwarded_with_protected_lib_merge(self):
        src = source()
        self.assertIn(
            "        daemon_startup_warning,\n        managed_daemon,\n        launch_telemetry,\n        startup_draft,",
            src,
        )
        lib = source("codex-rs/tui/src/lib.rs")
        self.assertIn(
            "    daemon_startup_warning: Option<String>,\n"
            "    managed_daemon: bool,\n"
            "    launch_telemetry: daemon_telemetry::Launch",
            lib,
        )

    def test_existing_tui_source_guard_pins_this_review(self):
        guard = source(".github/scripts/test_codexdd_045_3c9c_tui_upstream.py")
        self.assertIn(f'"{PATH}": "{NEW_BLOB}"', guard)

    def test_release_markers_and_lockfile_unchanged(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(
            source("codex-rs/upstream-codex-release.txt").strip(),
            "rust-v0.162.0",
        )
        self.assertEqual(
            git("rev-parse", "HEAD:codex-rs/Cargo.lock"),
            "a8eb8a076276cb3db5674b33c67d743b1c3d21fc",
        )

    def test_checkpoint_denies_runtime_and_release_authority(self):
        doc = (ROOT / "docs/codexdd-0.4.5-packet-3c9d3-startup-merge.md").read_text(
            encoding="utf-8"
        )
        for marker in (
            "INCOMPLETE / NOT A RELEASE CANDIDATE",
            "native Rust",
            "managed_daemon",
            "Cargo.lock",
            "39 remain",
        ):
            with self.subTest(marker=marker):
                self.assertIn(marker, doc)


if __name__ == "__main__":
    unittest.main()
