"""Avoid long optimized release builds failing on a Bazel announce_rc non-echoed CLI flag."""
from __future__ import annotations

import re
import subprocess
import unittest
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
S=(ROOT/".github/scripts/codexdd-045-staged-optimized-windows.ps1").read_text(encoding="utf-8")
W=(ROOT/".github/workflows/codexdd-045-3f43-manual-optimized-windows.yml").read_text(encoding="utf-8")

class SourceBoundFullReleaseArgumentVerification(unittest.TestCase):
    def test_direct_cache_arg_is_verified_in_staged_build_argv(self):
        m=re.search(r"\$buildArgs=@\(.*?\)\+\$opts\+@\(.*?\)\+\$targets\[\$Stage\]",S)
        self.assertIsNotNone(m)
        self.assertIn("'--disk_cache=D:/_bazel-disk'",S)
        self.assertIn("$requiredCacheArg='--disk_cache=D:/_bazel-disk'",S)
        self.assertIn("@($buildArgs | Where-Object { $_ -ceq $requiredCacheArg }).Count -ne 1",S)
        self.assertIn("[Array]::IndexOf([string[]]$buildArgs,$requiredCacheArg)",S)
        self.assertIn("$cacheOptionIndex -le $ciOptionIndex",S)
        self.assertIn("VERIFIED_EXPLICIT_OPT_RELEASE_BAZEL_DISK_CACHE=",S)
        self.assertLess(S.index("'--config=ci-windows-msvc'"),S.index("'--disk_cache=D:/_bazel-disk'"))

    def test_logs_are_used_only_for_available_release_flag_receipts(self):
        self.assertIn("foreach ($required in @('Build completed successfully','--compilation_mode=opt','--@rules_rust//rust/settings:lto=thin'))",S)
        self.assertNotIn("foreach ($required in @('Build completed successfully','--compilation_mode=opt','--@rules_rust//rust/settings:lto=thin','--disk_cache=D:/_bazel-disk'))",S)
        self.assertIn("if ($exitCode -ne 0)",S)
        self.assertIn("Get-ChildItem $cache -Recurse -File | Measure-Object Length -Sum",S)

    def test_no_premature_full_release_trigger_or_weakening(self):
        self.assertIn("NOT_ARMED_FOR_045_OPTIMIZED_RELEASE",W)
        self.assertIn("inputs.approve_expensive",W)
        self.assertIn("fail-on-cache-miss: true",W)
        self.assertIn("runtime_package_combined",S)
        self.assertIn("codexdd_045_verify_windows_archive.py",S)
        self.assertIn("release_approved=$false",S)
        self.assertIn("0.4.4",S)
        self.assertIn("rust-v0.162.0",S)
        for p,expected in {
          "codex-rs/Cargo.lock":"13bc15ff844e93dfaf7777c73bb7313b35bb3b81",
          "codex-rs/deny.toml":"f282c7c55d05201e308e60804f85754c8522d462",
        }.items():
          self.assertEqual(subprocess.check_output(["git","rev-parse","HEAD:"+p],
                             cwd=ROOT,text=True).strip(),expected)

if __name__=="__main__":
    unittest.main()
