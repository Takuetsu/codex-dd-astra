"""Read-only Windows optimized CLI build boundaries for CodexDD 0.4.5."""

import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


class OptimizedCliSafety(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.source = (
            ROOT / ".github/workflows/codexdd-045-windows-cli-optimized-preflight.yml"
        ).read_text(encoding="utf-8")

    def test_read_only_source_pinned_platform(self):
        s = self.source
        self.assertIn("permissions:\n  contents: read\n", s)
        self.assertIn("persist-credentials: false", s)
        self.assertIn("runs-on: windows-2022", s)
        self.assertIn("timeout-minutes: 90", s)
        self.assertNotIn("git push", s)
        self.assertNotIn("contents: write", s)
        self.assertNotIn("BAZEL_MSVC_RUNTIME_VISUAL_STUDIO_EULA=1", s)

    def test_optimized_cli_without_runtime_archive(self):
        s = self.source
        self.assertIn("'--config=release'", s)
        self.assertIn("'--//codex-rs:runtime_package_build_mode=release'", s)
        self.assertIn("'--lockfile_mode=error'", s)
        self.assertIn("'--jobs=2'", s)
        self.assertIn("'//codex-rs/cli:release_files'", s)
        self.assertIn("'cquery'", s)
        self.assertIn("BLOCKED: optimized CLI Bazel cquery", s)
        self.assertNotIn("runtime_package_combined", s)
        self.assertNotIn("'--//codex-rs:runtime_package_build_mode=debug'", s)
        self.assertNotIn("local_ram_resources", s)

    def test_native_binary_receipt_and_dependency_identity(self):
        s = self.source
        self.assertIn("Optimized CLI EXE or PDB missing", s)
        self.assertIn("Missing optimized CLI build success receipt", s)
        self.assertIn(
            "'codex-rs/Cargo.lock' = 'a8eb8a076276cb3db5674b33c67d743b1c3d21fc'", s
        )
        self.assertIn(
            "'codex-rs/deny.toml' = 'ea7fdc149fc880c6b36d75c29851641034289a4e'", s
        )
        for p in (
            "codex-rs/skills",
            "codex-rs/core-plugins",
            "codex-rs/ext/skills",
            "codex-rs/external-agent-migration",
        ):
            with self.subTest(p=p):
                self.assertIn(f"'{p}/BUILD.bazel' =", s)
                self.assertIn(f"'{p}/Cargo.toml' =", s)
        self.assertIn("Source or lock changed during build", s)
