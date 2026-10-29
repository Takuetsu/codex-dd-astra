"""Cheap source contract for real optimized Windows CLI/PDB combined archive."""
from pathlib import Path
import subprocess
import unittest
ROOT=Path(__file__).resolve().parents[2]
W=(ROOT/".github/workflows/codexdd-045-3f47-optimized-windows-cli-pdb-archive.yml").read_text()
C=(ROOT/".github/scripts/codexdd-045-3f44-checkpointed-opt-cache.ps1").read_text()
S=(ROOT/".github/scripts/codexdd-045-staged-optimized-windows.ps1").read_text()

class RealBuild(unittest.TestCase):
    def test_source_and_one_shot_trigger(self):
        self.assertRegex(W,r"github.event.before == '(?:NOT_ARMED_FOR_045_OPTIMIZED_RELEASE|[a-f0-9]{40})'")
        self.assertEqual(W.count("runs-on: windows-2022"),3)
        self.assertNotIn("workflow_dispatch:",W)
        self.assertNotIn("\n  schedule:",W)
        self.assertNotIn("create-release",W)
        self.assertEqual(W.count("disk-cache: false"),3)

    def test_real_cached_core_and_binary_outputs(self):
        self.assertIn("codexdd045-3f45b-40877812353-seed",W)
        self.assertEqual(W.count("codexdd045-3f47-"+"$"+"{{ github.run_id }}-core-seed"),2)
        self.assertEqual(W.count("codexdd045-3f47-"+"$"+"{{ github.run_id }}-core-complete"),2)
        self.assertEqual(W.count("fail-on-cache-miss: true"),3)
        self.assertIn("-Mode seed -Target '//codex-rs/core:core' -SoftMinutes 70",W)
        self.assertIn("-Mode restore -Target '//codex-rs/core:core' -SoftMinutes 155",W)
        self.assertIn("Target='//codex-rs/config:config'",C)
        self.assertIn("NONRELEASE_PARTIAL_ACTION_CHECKPOINT_READY_NOT_TARGET_PASS",C)
        self.assertIn("-Stage cli -SoftMinutes 165",W)
        self.assertIn("-Stage combined -SoftMinutes 90",W)
        self.assertIn("nonrelease-artifacts",S)
        self.assertIn("codex.exe",S)
        self.assertIn("codex.pdb",S)
        self.assertIn("runtime_package_combined.tar.gz",S)
        self.assertIn("codexdd_045_verify_windows_archive.py",S)
        self.assertIn("REAL_OPTIMIZED_CLI_PDB_COMBINED_ARCHIVE_VERIFIED=TRUE",W)
        self.assertIn("binaries-and-symbols",W)
        self.assertIn("compression-level: 0",W)
        self.assertIn("release_approved=$false",S)

    def test_secure_unpromoted_source(self):
        pins={
            "codex-rs/Cargo.lock":"13bc15ff844e93dfaf7777c73bb7313b35bb3b81",
            "codex-rs/deny.toml":"f282c7c55d05201e308e60804f85754c8522d462",
            "bazel/rules/runtime_package.bzl":"ba4cdad58f916f04602109c7b4d2512e83da5485",
        }
        for p,s in pins.items():
            self.assertEqual(subprocess.check_output(["git","rev-parse","HEAD:"+p],cwd=ROOT,text=True).strip(),s)
        self.assertEqual((ROOT/"codex-rs/codexdd-version.txt").read_text().strip(),"0.4.4")

if __name__=="__main__":
    unittest.main()
