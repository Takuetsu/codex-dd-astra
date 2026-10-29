"""3F.48: a completed optimized CLI/PDB cannot be invalidated by cache telemetry."""
from __future__ import annotations
import subprocess
import unittest
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
STAGE=(ROOT/".github/scripts/codexdd-045-staged-optimized-windows.ps1").read_text()
RULE=(ROOT/"bazel/rules/runtime_package.bzl").read_text()

class SuccessfulCLIIsNotACacheFailure(unittest.TestCase):
    def test_real_historic_2858_action_zero_hit_cli_is_accepted(self):
        self.assertIn("if ($Stage -in @('cli','combined')",STAGE)
        self.assertIn("NO_DISK_CACHE_HITS_BUT_NATIVE_BINARY_BUILD_SUCCEEDED",STAGE)
        self.assertNotIn("if ($Stage -in @('core','cli')",STAGE)
        self.assertIn("if ($Stage -eq 'core'",STAGE)
        self.assertIn("Build completed successfully",STAGE)
        self.assertIn("codex.exe",STAGE)
        self.assertIn("codex.pdb",STAGE)
        self.assertIn("nonrelease-artifacts",STAGE)
        self.assertIn("codexdd_045_verify_windows_archive.py",STAGE)
        self.assertIn("[ValidateRange(2,240)][int]$SoftMinutes",STAGE)
    def test_different_release_transition_explains_possible_key_miss(self):
        self.assertIn('settings[key] + ["cargo-release-profile"]',RULE)
        self.assertIn('cfg = _release_profile',RULE)
        self.assertIn('"-Ccodegen-units=16"',RULE)
        self.assertIn('"@rules_rust//rust/settings:lto"',RULE)
        self.assertIn('symbol_group = select({',RULE)
    def test_security_and_staging_unpromoted(self):
        self.assertEqual((ROOT/"codex-rs/codexdd-version.txt").read_text().strip(),"0.4.4")
        self.assertEqual((ROOT/"codex-rs/upstream-codex-release.txt").read_text().strip(),"rust-v0.162.0")
        for path,sha in {"codex-rs/Cargo.lock":"13bc15ff844e93dfaf7777c73bb7313b35bb3b81","codex-rs/deny.toml":"f282c7c55d05201e308e60804f85754c8522d462","bazel/rules/runtime_package.bzl":"ba4cdad58f916f04602109c7b4d2512e83da5485"}.items():
            self.assertEqual(subprocess.check_output(["git","rev-parse","HEAD:"+path],cwd=ROOT,text=True).strip(),sha)

if __name__=="__main__":
    unittest.main()
