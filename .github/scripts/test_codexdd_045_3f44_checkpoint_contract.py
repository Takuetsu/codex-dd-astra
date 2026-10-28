"""Static safety contracts for checkpointing Rust Bazel action outputs on timeout."""
from pathlib import Path
import subprocess
import unittest

ROOT=Path(__file__).resolve().parents[2]
S=(ROOT/".github/scripts/codexdd-045-3f44-checkpointed-opt-cache.ps1").read_text(encoding="utf-8")
W=(ROOT/".github/workflows/codexdd-045-3f44-salvage-optimized-cache.yml").read_text(encoding="utf-8")

class BoundedCheckpoint(unittest.TestCase):
    def test_only_medium_real_rust_target_and_two_windows_runners(self):
        self.assertEqual(W.count("runs-on: windows-2022"),2)
        self.assertIn("-SoftMinutes 10",W)
        self.assertIn("-SoftMinutes 22",W)
        self.assertIn("//codex-rs/config:config",S)
        self.assertNotIn("//codex-rs/cli:release_files",W)
        self.assertNotIn("//codex-rs:runtime_package_combined",W)
        self.assertNotIn("workflow_dispatch:",W)
        self.assertIn("release_approved=$false",S)
        self.assertIn("rust-v0.162.0",S)

    def test_incomplete_seed_can_save_cache_but_not_report_compilation_success(self):
        for fragment in (
            "checkpoint_ready=true","build_complete=","if ($timedOut) {",
            "taskkill.exe /PID $proc.Id /T /F",
            "NONRELEASE_PARTIAL_ACTION_CHECKPOINT_READY_NOT_TARGET_PASS",
            "if ($timedOut -and $Mode -ne 'seed')",
            "BAZEL_ACTION_PATH_SHA256=","MSVC action environment drift",
            "--disk_cache=D:/_bazel-disk","--@rules_rust//rust/settings:lto=thin",
            "Compiling Rust","Build completed successfully","disk cache hits?",
        ):
            with self.subTest(fragment=fragment): self.assertIn(fragment,S)
        self.assertIn("if: success() && steps.compile.outputs.checkpoint_ready == 'true'",W)
        self.assertEqual(W.count("codexdd045-3f44-"+"$"+"{{ github.run_id }}-seed"),2)
        self.assertIn("fail-on-cache-miss: true",W)
        self.assertIn("steps.cache.outputs.cache-hit",W)
        self.assertIn("steps.compile.outputs.build_complete",W)
        self.assertIn("steps.compile.outputs.checkpoint_ready",W)

    def test_strict_security_and_storage_caps(self):
        for word in ("MODULE.bazel.lock","codex-rs/Cargo.lock","codex-rs/deny.toml",
                     "codex-rs/config/Cargo.toml","git rev-parse HEAD^",
                     "1610612736","3221225472","Ruby"):
            with self.subTest(word=word): self.assertIn(word,W+S)
        self.assertEqual(W.count("disk-cache: false"),2)
        self.assertEqual(W.count("cache-save: false"),2)
        self.assertEqual(W.count("uses: actions/cache/save@0400d5f644dc74513175e3cd8d07132dd4860809"),1)
        self.assertEqual(W.count("uses: actions/cache/restore@0400d5f644dc74513175e3cd8d07132dd4860809"),1)
        self.assertEqual(subprocess.check_output(["git","rev-parse","HEAD:codex-rs/config/Cargo.toml"],cwd=ROOT,text=True).strip(),"b4be94bb585c6757234299a78f8b42a02b3d2017")

if __name__=="__main__": unittest.main()
