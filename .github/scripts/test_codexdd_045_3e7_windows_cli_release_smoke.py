"""Exact-source 0.4.5 Windows CLI debug release-files smoke contract (nonrelease)."""

import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


class CliReleaseSmokeContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.flow = (
            ROOT / ".github/workflows/codexdd-045-windows-cli-release-smoke.yml"
        ).read_text(encoding="utf-8")

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

    def test_resume_uses_the_exact_green_skills_alias_source(self):
        s = self.flow
        self.assertIn("'defs.bzl' = 'f49bfc9be69d03d60a02e8aec6974a6fae9e200c'", s)
        self.assertIn(
            "'codex-rs/skills/BUILD.bazel' = '9f4e136ec76fc298e32ccb94fd0800b1f5656c76'",
            s,
        )
        self.assertNotIn("'defs.bzl' = 'b2f384d0687cbb81d0c2e8a257cc3d0d6aa469a4'", s)
        self.assertIn("      - 'defs.bzl'", s)
        self.assertIn("      - 'codex-rs/skills/BUILD.bazel'", s)
        self.assertIn(
            "git diff --exit-code -- MODULE.bazel MODULE.bazel.lock defs.bzl patches/zstd-sys_windows_msvc_include_dirs.patch codex-rs/skills/BUILD.bazel",
            s,
        )
        self.assertNotIn("contents: write", s)
        self.assertNotIn("git push", s)

    def test_all_four_reviewed_yaml_consumers_are_pinned(self):
        s = self.flow
        expected = {
            "codex-rs/skills": (
                "9f4e136ec76fc298e32ccb94fd0800b1f5656c76",
                "7342975aaea0e3ecc61da41f145674cd154a12c9",
            ),
            "codex-rs/core-plugins": (
                "16dc432cd2f9b6e16de5b4f367a64654a2e492e6",
                "668da25d6c4db37e737b3401e78333290bf1f2f8",
            ),
            "codex-rs/ext/skills": (
                "e013d31eb8837cc3a7296e216c941427cf1cb084",
                "80bcfba8cb37de4c62d0f80d3c54a8290fdb084d",
            ),
            "codex-rs/external-agent-migration": (
                "1ae14ac88a1a765328a26026e65a3bd0328e8f17",
                "3f49a1ae00cc48720b766d9e76bc050e954cf0fa",
            ),
        }
        self.assertEqual(len(expected), 4)
        for dirname, (build_blob, cargo_blob) in expected.items():
            with self.subTest(package=dirname):
                self.assertIn(f"'{dirname}/BUILD.bazel' = '{build_blob}'", s)
                self.assertIn(f"'{dirname}/Cargo.toml' = '{cargo_blob}'", s)
                self.assertIn(f"      - '{dirname}/BUILD.bazel'", s)
                self.assertIn(f"      - '{dirname}/Cargo.toml'", s)
                self.assertIn(f"{dirname}/BUILD.bazel {dirname}/Cargo.toml", s)
        self.assertEqual(s.count("'//codex-rs/cli:release_files'"), 1)
        self.assertNotIn("runtime_package_combined", s)
        self.assertNotIn("'--config=release'", s)
        self.assertNotIn("contents: write", s)

    def test_compiler_fail_fast_and_authentic_success_receipt(self):
        s = self.flow
        self.assertIn("BLOCKED: CLI release-files debug smoke", s)
        self.assertIn("Missing native CLI release-files build receipt", s)
        self.assertIn("Missing expected CLI release-files output", s)
        self.assertIn("Source or lock changed during build", s)


if __name__ == "__main__":
    unittest.main()
