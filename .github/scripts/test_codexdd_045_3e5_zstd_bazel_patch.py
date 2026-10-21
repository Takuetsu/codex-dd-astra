"""Source-only guards for the pinned zstd-sys MSVC patch on release analysis.

The resolved zstd-sys 2.1.2+zstd.1.5.7 already gates symbol visibility using
hide_symbols() and target_is_msvc(). This patch adds include paths only.
Native Bazel repository patch application must be validated independently.
"""
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]


class ZstdMsvcPatchGuard(unittest.TestCase):
    def test_exact_two_hunk_patch_only_adds_host_independent_include_paths(self):
        patch = (ROOT / "patches/zstd-sys_windows_msvc_include_dirs.patch").read_text(encoding="utf-8")
        self.assertEqual(patch.count("@@ -"), 2)
        self.assertIn('let target = env::var("TARGET").unwrap_or_default();', patch)
        self.assertIn('if target.contains("msvc") {', patch)
        self.assertEqual(patch.count('if target.contains("msvc")'), 1)
        for directory in (
            "zstd/lib/compress",
            "zstd/lib/decompress",
            "zstd/lib/dictBuilder",
            "zstd/lib/legacy",
            "zstd/contrib/seekable_format",
        ):
            self.assertIn('config.include("' + directory + '");', patch)
        self.assertNotIn("-fvisibility=hidden", patch)
        self.assertNotIn("hide_symbols", patch)
        self.assertNotIn("target_is_msvc", patch)
        self.assertNotIn("fn target_is_msvc", patch)

    def test_bazel_module_retains_single_scoped_annotation(self):
        module = (ROOT / "MODULE.bazel").read_text(encoding="utf-8")
        self.assertIn('crate = "zstd-sys"', module)
        self.assertEqual(module.count('"//patches:zstd-sys_windows_msvc_include_dirs.patch"'), 1)
        self.assertIn('gen_build_script = "on"', module)
        self.assertIn('patch_args = ["-p1"]', module)
        exported = (ROOT / "patches/BUILD.bazel").read_text(encoding="utf-8")
        self.assertIn('"zstd-sys_windows_msvc_include_dirs.patch"', exported)

    def test_strict_read_only_windows_analysis_tracks_patch(self):
        workflow = (ROOT / ".github/workflows/codexdd-045-windows-bazel-release-analysis.yml").read_text(encoding="utf-8")
        self.assertIn("permissions:\n  contents: read\n", workflow)
        self.assertIn("      - 'patches/zstd-sys_windows_msvc_include_dirs.patch'", workflow)
        self.assertIn("'patches/zstd-sys_windows_msvc_include_dirs.patch' = '", workflow)
        self.assertNotIn("contents: write", workflow)
        self.assertNotIn("BAZEL_MSVC_RUNTIME_VISUAL_STUDIO_EULA=1", workflow)
        self.assertNotIn("git push", workflow)
        self.assertNotIn("bazel build", workflow)


if __name__ == "__main__":
    unittest.main()
