"""CodexDD 0.4.5 3C.9d2: Windows elevated startup safety source regressions.

Offline static checks. Not native Rust, live Windows elevation behavior,
SSH safety proof, or release acceptance.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
REVIEWED = {
    "codex-rs/tui/src/daemon_startup.rs": [
        "2a4c1190086bee4b16f86252ab802bf06e4a0361",
        "9e2b577a840d56efee13856ad9f11fd83a936d0d"
    ],
    "codex-rs/tui/src/daemon_startup_tests.rs": [
        "f103b69a4629f9d019cd2a39c3823ae29dc646bf",
        "4340c3c42ee4dcd4f5c5e06b918648bee4da51a8"
    ]
}


def git_blob(ref: str, path: str) -> str:
    return subprocess.run(
        ["git", "rev-parse", "--verify", ref + ":" + path],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout.strip()


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT, check=True, capture_output=True, text=True, encoding="utf-8",
    ).stdout


class TuiElevatedWindowsFallbackTests(unittest.TestCase):
    def assertIn(self, marker, contents, msg=None):
        if isinstance(contents, str) and len(contents) > 2048:
            self.assertTrue(marker in contents, msg or f"missing source marker: {marker!r}")
            return
        super().assertIn(marker, contents, msg)

    def test_two_reviewed_daemon_source_blobs(self):
        self.assertEqual(len(REVIEWED), 2)
        for path, (original, merged) in REVIEWED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob(PRODUCTION, path), original)
                self.assertEqual(git_blob("HEAD", path), merged)

    def test_elevated_implicit_sessions_choose_embedded_fallback(self):
        handler = source("codex-rs/tui/src/daemon_startup.rs")
        self.assertIn("elevated_windows_implicit_exclusion", handler)
        self.assertIn("ELEVATED_WINDOWS_EMBEDDED_REASON", handler)

    def test_failed_windows_elevation_probe_remains_fail_closed(self):
        handler = source("codex-rs/tui/src/daemon_startup.rs")
        self.assertIn("WINDOWS_ELEVATION_UNVERIFIED_REASON", handler)
        self.assertIn("Err(()) => Some(WINDOWS_ELEVATION_UNVERIFIED_REASON)", handler)

    def test_intentional_elevated_embedded_mode_has_no_f2_warning(self):
        handler = source("codex-rs/tui/src/daemon_startup.rs")
        self.assertIn("pub(super) fn automatic_exclusion_warning(", handler)
        self.assertIn("reason != ELEVATED_WINDOWS_EMBEDDED_REASON", handler)

    def test_explicit_remote_daemon_transport_remains_distinct(self):
        handler = source("codex-rs/tui/src/daemon_startup.rs")
        self.assertIn("existing.is_some() || agents_overview || explicit_remote", handler)
        self.assertIn("windows_automatic_fallback_reason", handler)

    def test_windows_launch_error_classification_remains(self):
        handler = source("codex-rs/tui/src/daemon_startup.rs")
        self.assertIn("codex_app_server_daemon::ElevatedLaunchRestricted", handler)
        self.assertIn("codex_app_server_daemon::DetachedLaunchRestricted", handler)

    def test_native_elevated_windows_regression_fixtures_preserved(self):
        tests = source("codex-rs/tui/src/daemon_startup_tests.rs")
        for fixture in (
            "elevated_daemon_auto_start_falls_back_to_embedded_mode",
            "elevated_windows_implicit_startup_uses_embedded_without_f2_warning",
            "failed_windows_elevation_probe_uses_embedded_with_visible_f2_warning",
            "elevated_windows_implicit_policy_preserves_explicit_transport_and_real_warnings",
        ):
            self.assertIn(fixture, tests)

    def test_upstream_0_163_daemon_compatibility_requirements_remain(self):
        handler = source("codex-rs/tui/src/daemon_startup.rs")
        self.assertIn("ConfigRequirementsReadResponse", handler)
        self.assertIn("read_effective_config_if_supported", handler)

    def test_startup_orchestration_preserves_elevated_fallback_after_reviewed_merge(self):
        path = "codex-rs/tui/src/startup_orchestration.rs"
        self.assertEqual(git_blob(PRODUCTION, path), "e03aa30b4723717cfc72935d118e074fbf4f2aae")
        self.assertEqual(git_blob("HEAD", path), "6b3bf4a9cf3d639107126b29992690b19bf4c932")
        merged = source(path)
        self.assertIn("elevated_windows_implicit_exclusion(", merged)
        self.assertIn("windows_automatic_fallback_reason(&err)", merged)
        self.assertIn("automatic_exclusion_warning(daemon_exclusion, auto_start_daemon)", merged)

    def test_existing_cli_daemon_regression_is_untouched(self):
        self.assertEqual(
            git_blob("HEAD", "codex-rs/cli/tests/daemon_startup.rs"),
            "e1c6d330f602f3c4cebf9d7163ecdcd2adb9100f",
        )

    def test_dispatch_versions_lockfile_and_release_blocker(self):
        self.assertEqual(
            git_blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(git_blob("HEAD", "codex-rs/Cargo.lock"),
                         "0c913b46674d3593ee84f1fee5508d16e33efae5")
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c9d2-elevated-startup.md").read_text(
            encoding="utf-8"
        )
        for marker in ("INCOMPLETE / NOT A RELEASE CANDIDATE",
                       "source-only", "native Rust", "Cargo.lock", "40"):
            self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
