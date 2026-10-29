"""Protect optimized CLI/PDB artifacts from any later CI telemetry failures."""
from pathlib import Path
import subprocess
import unittest
R=Path(__file__).resolve().parents[2]
S=(R/".github/scripts/codexdd-045-preserve-real-cli-pdb.ps1").read_text(encoding="utf-8")
ST=(R/".github/scripts/codexdd-045-staged-optimized-windows.ps1").read_text(encoding="utf-8")
class RealBinaryPreservation(unittest.TestCase):
 def test_authoritative_success_and_actual_payload(self):
  for s in ("Target //codex-rs/cli:release_files up-to-date:","INFO: Build completed successfully",
            "bazel-out","codex.exe","codex.pdb",
            "Microsoft C/C++ MSF 7.00","StartsWith('MZ')",
            "Get-FileHash","sha256","REAL_OPTIMIZED_CLI_AND_PDB_SALVAGED_AND_HASHED",
            "release_approved=$false","cli-integrity.json","$env:GITHUB_SHA"):
   with self.subTest(s=s):self.assertIn(s,S)
  self.assertIn("NONRELEASE",S)
  self.assertIn("cli_artifact_ready=true",S)
  self.assertIn("git status --porcelain --untracked-files=no",S)
  self.assertNotIn("Start-Process",S)
  self.assertNotIn("gh release",S)
 def test_no_cache_efficiency_false_rejection(self):
  self.assertIn("NO_DISK_CACHE_HITS_BUT_NATIVE_BINARY_BUILD_SUCCEEDED",ST)
  self.assertNotIn("if ($Stage -in @('core','cli')",ST)
  self.assertIn("[ValidateRange(2,240)][int]$SoftMinutes",ST)
 def test_protected_release_deps_unchanged(self):
  for p,s in {"codex-rs/Cargo.lock":"13bc15ff844e93dfaf7777c73bb7313b35bb3b81","codex-rs/deny.toml":"f282c7c55d05201e308e60804f85754c8522d462"}.items():
   self.assertEqual(subprocess.check_output(["git","rev-parse","HEAD:"+p],cwd=R,text=True).strip(),s)
  self.assertEqual((R/"codex-rs/codexdd-version.txt").read_text().strip(),"0.4.4")
if __name__=="__main__":unittest.main()
