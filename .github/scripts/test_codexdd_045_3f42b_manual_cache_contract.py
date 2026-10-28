"""Cheap static guard for GitHub Actions exact-key staged Bazel action cache."""
from __future__ import annotations
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github/workflows/codexdd-045-3f42b-medium-optimized-cache-probe.yml"
TARGET = "//codex-rs/config:config"


class CacheProbeContract(unittest.TestCase):
    def test_two_independent_jobs_bounded_and_real_rust_target(self):
        s = WORKFLOW.read_text(encoding="utf-8")
        self.assertIn("seed:\n", s)
        self.assertIn("restore:\n", s)
        self.assertIn("needs: seed", s)
        self.assertEqual(s.count("runs-on: windows-2022"), 2)
        self.assertEqual(s.count("timeout-minutes: 29"), 2)
        self.assertEqual(s.count("WaitForExit(960000)"), 2)
        self.assertEqual(s.count("'" + TARGET + "'"), 2)
        self.assertNotIn("//codex-rs/cli:release_files", s)
        self.assertNotIn("//codex-rs:runtime_package_combined", s)

    def test_exact_action_cache_transport_and_stable_environment(self):
        s = WORKFLOW.read_text(encoding="utf-8")
        for clause in (
            "uses: actions/cache/save@0400d5f644dc74513175e3cd8d07132dd4860809",
            "uses: actions/cache/restore@0400d5f644dc74513175e3cd8d07132dd4860809",
            "fail-on-cache-miss: true",
            "cache-hit",
            "github.run_id",
            "--disk_cache=D:/_bazel-disk",
            "1073741824",
            "BAZEL_ACTION_PATH_SHA256=",
            "Ruby",
            "Expected exactly one unrelated hosted Ruby PATH",
            "EXPECTED_PATH_SHA",
            "No independent Rust action cache reuse",
            "Restore runner rebuilt local compiler actions",
        ):
            with self.subTest(clause=clause):
                self.assertIn(clause, s)
        self.assertEqual(s.count("disk-cache: false"), 2)
        self.assertEqual(s.count("cache-save: false"), 2)
        self.assertEqual(s.count("'--config=release'"), 2)
        self.assertEqual(s.count("'--config=ci-windows-msvc'"), 2)

    def test_locked_source_and_no_stage_auto_release(self):
        s = WORKFLOW.read_text(encoding="utf-8")
        self.assertIn("github.event.before ==", s)
        self.assertIn("codex-rs/config/Cargo.toml", s)
        self.assertIn("codex-rs/deny.toml", s)
        self.assertIn("rust-v0.162.0", s)
        self.assertIn("0.4.4", s)
        self.assertNotIn("workflow_dispatch:", s)
        self.assertNotIn("push: [main]", s)
        self.assertNotIn("actions/upload-release-asset", s)
        self.assertEqual(subprocess.check_output([
            "git", "rev-parse", "HEAD:codex-rs/config/Cargo.toml"
        ], cwd=ROOT, text=True).strip(), "b4be94bb585c6757234299a78f8b42a02b3d2017")


if __name__ == "__main__":
    unittest.main()
