"""Static safeguards for the disabled manual staged Windows optimized release build."""
from pathlib import Path
import unittest

ROOT=Path(__file__).resolve().parents[2]
W=ROOT/".github/workflows/codexdd-045-3f43-manual-optimized-windows.yml"
S=ROOT/".github/scripts/codexdd-045-staged-optimized-windows.ps1"

class WindowsStagingSafety(unittest.TestCase):
    def test_manual_opt_in_and_no_release(self):
        w=W.read_text(encoding="utf-8")
        self.assertIn("workflow_dispatch:",w)
        self.assertNotIn("\n  push:",w)
        self.assertNotIn("\n  schedule:",w)
        self.assertIn("AUTHORIZE_NONRELEASE_045_EXPENSIVE_WINDOWS_STAGING",w)
        self.assertIn("inputs.approve_expensive ==",w)
        self.assertNotIn("actions/create-release",w)
        self.assertNotIn("upload-release-asset",w)

    def test_unique_immutable_cache_chain_and_bounded_stages(self):
        w=W.read_text(encoding="utf-8")
        self.assertEqual(w.count("runs-on: windows-2022"),3)
        self.assertEqual(w.count("disk-cache: false"),3)
        self.assertEqual(w.count("cache-save: false"),3)
        self.assertEqual(w.count("actions/cache/save@0400d5f644dc74513175e3cd8d07132dd4860809"),2)
        self.assertEqual(w.count("actions/cache/restore@0400d5f644dc74513175e3cd8d07132dd4860809"),2)
        self.assertEqual(w.count("fail-on-cache-miss: true"),2)
        self.assertIn("github.run_id",w)
        self.assertIn("needs: warm",w)
        self.assertIn("needs: [warm, core]",w)
        self.assertIn("-Stage warm -SoftMinutes 32",w)
        self.assertIn("-Stage core -SoftMinutes 96",w)
        self.assertIn("-Stage cli -SoftMinutes 165",w)
        self.assertIn("-Stage combined -SoftMinutes 90",w)
        self.assertIn("timeout-minutes: 285",w)

    def test_real_targets_not_synthetic_debug_and_binary_outputs_checked(self):
        s=S.read_text(encoding="utf-8")
        for target in (
            "//codex-rs/config:config", "//codex-rs/protocol:protocol",
            "//codex-rs/core:core", "//codex-rs/cli:release_files",
            "//codex-rs:runtime_package_combined",
        ):
            with self.subTest(target=target):
                self.assertIn(target,s)
        for requirement in (
            "--disk_cache=D:/_bazel-disk", "--config=release",
            "--@rules_rust//rust/settings:lto=thin",
            "BAZEL_ACTION_PATH_SHA256", "ExpectedPathSha",
            "codex.exe","codex.pdb","runtime_package_combined.tar.gz",
            "codexdd_045_verify_windows_archive.py",
            "release_approved=$false",
        ):
            with self.subTest(fragment=requirement):
                self.assertIn(requirement,s)
        self.assertIn("output=files",s)
        self.assertIn("taskkill.exe",s)
        self.assertIn("No real compiler action cache reuse",s)

    def test_protected_dependency_graph_and_owner_matrix(self):
        w=W.read_text(encoding="utf-8")
        for item in (
            "codex-rs/Cargo.toml","codex-rs/Cargo.lock",
            "codex-rs/deny.toml","MODULE.bazel.lock",
            "codex-rs/cli/BUILD.bazel","bazel/rules/runtime_package.bzl",
            "test_codexdd_045_3f14_owner_matrix.py","0.4.4",
            "rust-v0.162.0",
        ):
            self.assertIn(item,w)
        self.assertIn("only SHA-256 receipts",w)

if __name__=="__main__":
    unittest.main()
