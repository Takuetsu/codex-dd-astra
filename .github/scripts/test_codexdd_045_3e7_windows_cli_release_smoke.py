"""Exact-source 0.4.5 Windows CLI debug release-files smoke contract (nonrelease)."""
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

class CliReleaseSmokeContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.flow = (ROOT / ".github/workflows/codexdd-045-windows-cli-release-smoke.yml").read_text(encoding="utf-8")

    def test_only_native_cli_release_files_under_debug_smoke_profile(self):
        s = self.flow
        self.assertIn("'//codex-rs/cli:release_files'", s)
        self.assertEqual(s.count("'//codex-rs/cli:release_files'"), 1)
        self.assertIn("'--//codex-rs:runtime_package_build_mode=debug'", s)
        self.assertIn("'--host_platform=//:local_windows_msvc'", s)
        self.assertIn("'--jobs=2'", s)
        self.assertIn("'--lockfile_mode=error'", s)
        self.assertNotIn("local_ram_resources", s)
        self.assertNotIn("runtime_package_combined", s)
        self.assertNotIn("'--config=release'", s)

    def test_exact_source_security_and_read_only_artifacts(self):
        s = self.flow
        self.assertIn("permissions:\n  contents: read", s)
        self.assertIn("persist-credentials: false", s)
        self.assertIn("runs-on: windows-2022", s)
        self.assertIn("timeout-minutes: 45", s)
        self.assertIn("866a458a2df4dd0d565f8066deadf73e2fdf8176", s)
        self.assertIn("a8eb8a076276cb3db5674b33c67d743b1c3d21fc", s)
        self.assertIn("codexdd045-cli-release-smoke/*.log", s)
        self.assertIn("if: always()", s)
        self.assertNotIn("contents: write", s)
        self.assertNotIn("git push", s)
        self.assertNotIn("BAZEL_MSVC_RUNTIME_VISUAL_STUDIO_EULA=1", s)

    def test_compiler_fail_fast_and_authentic_success_receipt(self):
        s = self.flow
        self.assertIn("BLOCKED: CLI release-files debug smoke", s)
        self.assertIn("Missing native CLI release-files build receipt", s)
        self.assertIn("Missing expected CLI release-files output", s)
        self.assertIn("Source or lock changed during build", s)

if __name__ == "__main__":
    unittest.main()
