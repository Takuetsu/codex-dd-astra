"""Regression check: Bazel --announce_rc omits direct --disk_cache CLI options.

3F.44 Windows seed reached real Rust actions then failed an invalid log-string
assertion. The explicit cache argument must be verified in the argument vector
and an actual nonempty on-disk cache measured even if the seed timed out.
"""
from __future__ import annotations

import re
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / ".github/scripts/codexdd-045-3f44-checkpointed-opt-cache.ps1"


class DiskCacheCheckpointGuard(unittest.TestCase):
    def test_direct_cli_arg_is_checked_after_config_expansion(self):
        source = SCRIPT.read_text(encoding="utf-8")
        args = re.search(r"\$args=@\((.*?)\n\)", source, flags=re.DOTALL)
        self.assertIsNotNone(args)
        argv = args.group(1)
        self.assertEqual(argv.count("'--disk_cache=D:/_bazel-disk'"), 1)
        self.assertLess(argv.index("'--config=ci-windows-msvc'"),
                        argv.index("'--disk_cache=D:/_bazel-disk'"))
        self.assertIn("$requiredCacheFlag='--disk_cache=D:/_bazel-disk'", source)
        self.assertIn("[Array]::IndexOf([string[]]$args,$requiredCacheFlag)", source)
        self.assertIn("$cacheIndex -le $ciConfigIndex", source)
        self.assertIn("VERIFIED_EXPLICIT_BAZEL_DISK_CACHE_ARGUMENT=", source)

    def test_announce_rc_is_not_required_to_echo_direct_cache_args(self):
        source = SCRIPT.read_text(encoding="utf-8")
        self.assertIn("foreach ($opt in @('--compilation_mode=opt','--@rules_rust//rust/settings:lto=thin'))", source)
        self.assertNotIn("foreach ($opt in @('--compilation_mode=opt','--@rules_rust//rust/settings:lto=thin','--disk_cache=D:/_bazel-disk'))", source)
        # Actual Windows rc fragment from 3F.44 captured the release profile
        # but did not echo the command-line disk-cache override.
        sample_log = (
            "INFO: Found applicable config definition build:release "
            "--compilation_mode=opt --@rules_rust//rust/settings:lto=thin\n"
            "[1,008 / 1,320] Compiling Rust proc-macro ... disk-cache ..."
        )
        self.assertIn("--compilation_mode=opt", sample_log)
        self.assertIn("--@rules_rust//rust/settings:lto=thin", sample_log)
        self.assertNotIn("--disk_cache=D:/_bazel-disk", sample_log)

    def test_checkpoint_bytes_report_before_any_log_based_failure(self):
        s = SCRIPT.read_text(encoding="utf-8")
        self.assertLess(s.index("CHECKPOINT_DISK_CACHE_INVENTORY="),
                        s.index("foreach ($opt in @('--compilation_mode=opt'"))
        self.assertIn("$files.Count -lt 2", s)
        self.assertIn("$bytes -gt $MaxCacheBytes", s)
        self.assertIn("checkpoint_ready=true", s)
        self.assertIn("if ($timedOut -and $Mode -ne 'seed')", s)
        self.assertIn("NONRELEASE_PARTIAL_ACTION_CHECKPOINT_READY_NOT_TARGET_PASS", s)
        self.assertIn("if ($Mode -eq 'restore' -and $summary -notmatch", s)

    def test_source_security_and_release_are_unchanged(self):
        for path, expected in {
            "codex-rs/config/Cargo.toml": "b4be94bb585c6757234299a78f8b42a02b3d2017",
            "codex-rs/Cargo.lock": "13bc15ff844e93dfaf7777c73bb7313b35bb3b81",
            "codex-rs/deny.toml": "f282c7c55d05201e308e60804f85754c8522d462",
        }.items():
            with self.subTest(path=path):
                self.assertEqual(
                    subprocess.check_output(["git", "rev-parse", "HEAD:" + path],
                                            cwd=ROOT, text=True).strip(),
                    expected,
                )
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")
        self.assertEqual((ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(), "rust-v0.162.0")


if __name__ == "__main__":
    unittest.main()
