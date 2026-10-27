"""Single-leaf Windows Bazel compiler job must retain read-only, exact-source limits."""

import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


class ZstdMsvcCompileSmoke(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.workflow = (
            ROOT / ".github/workflows/codexdd-045-windows-bazel-zstd-smoke.yml"
        ).read_text(encoding="utf-8")

    def test_target_is_only_zstd_syss_native_dependency(self):
        s = self.workflow
        self.assertIn("'@@rules_rs++crate+crates//:zstd-sys-2.1.2+zstd.1.5.7'", s)
        self.assertIn("866a458a2df4dd0d565f8066deadf73e2fdf8176", s)
        self.assertIn("'--lockfile_mode=error'", s)
        self.assertIn("'--jobs=2'", s)
        self.assertNotIn("--local_ram_resources", s)
        self.assertNotIn("cquery", s)
        self.assertNotIn("runtime_package_combined", s)
        self.assertNotIn("cli:release_files", s)

    def test_read_only_and_real_build_receipt(self):
        s = self.workflow
        self.assertIn("permissions:\n  contents: read", s)
        self.assertIn("persist-credentials: false", s)
        self.assertIn("runs-on: windows-2022", s)
        self.assertIn("timeout-minutes: 40", s)
        self.assertIn("BLOCKED: zstd-sys native compile", s)
        self.assertIn("Missing Bazel successful build receipt", s)
        self.assertNotIn("contents: write", s)
        self.assertNotIn("BAZEL_MSVC_RUNTIME_VISUAL_STUDIO_EULA=1", s)
        self.assertNotIn("git push", s)


if __name__ == "__main__":
    unittest.main()
