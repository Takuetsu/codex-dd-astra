"""Nonrelease source-level Windows release packaging inventory, not binary proof."""

from __future__ import annotations

import json
import re
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MATRIX = ROOT / "docs/upstream-candidates/codexdd-045-109-semantic-owner-matrix.json"
PACKAGE = ROOT / "codex-rs/BUILD.bazel"
RULE = ROOT / "bazel/rules/runtime_package.bzl"
CLI = ROOT / "codex-rs/cli/BUILD.bazel"
BAZELRC = ROOT / ".bazelrc"


def git_sha(path: str) -> str:
    return subprocess.check_output(
        ["git", "rev-parse", "HEAD:" + path], cwd=ROOT, text=True
    ).strip()


class WindowsReleaseInventory(unittest.TestCase):
    def test_exact_protected_packaging_source(self):
        pins = {
          "codex-rs/BUILD.bazel": "843e9fe095e0982df2043bb17c7da8e6e1c95025",
          "bazel/rules/runtime_package.bzl": "ba4cdad58f916f04602109c7b4d2512e83da5485",
          "codex-rs/cli/BUILD.bazel": "5dd2b8b9629275fb2c392c59134ea95a3ce317d5",
          ".bazelrc": "b7698b896e8d0830b60477999d330b115cf5ee8e"
}
        pins.update({
            "codex-rs/Cargo.lock": "13bc15ff844e93dfaf7777c73bb7313b35bb3b81",
            "codex-rs/Cargo.toml": "cfebd9b784ef9ff027b274aa9f7101c9c2496613",
            "codex-rs/deny.toml": "f282c7c55d05201e308e60804f85754c8522d462",
        })
        for path, expected in pins.items():
            with self.subTest(path=path):
                self.assertEqual(git_sha(path), expected)

    def test_combined_archive_and_separate_optional_components_are_distinct(self):
        text = PACKAGE.read_text(encoding="utf-8")
        self.assertIn('actual = ":runtime_package_combined"', text)
        combined = text.split('name = "runtime_package_combined"', 1)[1].split('name = "runtime_package_primary"', 1)[0]
        primary = text.split('name = "runtime_package_primary"', 1)[1].split('name = "runtime_package_app-server"', 1)[0]
        helpers = text.split('name = "runtime_package_helpers"', 1)[1]
        for artifact in (
            "//codex-rs/cli:release_files",
            "//codex-rs/app-server:release_files",
            "//codex-rs/code-mode-host:release_files",
            "//codex-rs/windows-sandbox-rs:command-runner-release_files",
            "//codex-rs/windows-sandbox-rs:sandbox-setup-release_files",
        ):
            with self.subTest(required_combined=artifact):
                self.assertIn(artifact, combined)
        self.assertNotIn("//codex-rs/windows-sandbox-service:release_files", combined)
        self.assertNotIn("//codex-rs/responses-api-proxy:release_files", combined)
        self.assertIn("//codex-rs/windows-sandbox-service:release_files", helpers)
        self.assertIn("//codex-rs/responses-api-proxy:release_files", primary)
        self.assertIn("allow_duplicates_with_different_content", RULE.read_text(encoding="utf-8"))

    def test_release_transition_and_windows_pdb_are_mandatory(self):
        rule = RULE.read_text(encoding="utf-8")
        rc = BAZELRC.read_text(encoding="utf-8")
        cli = CLI.read_text(encoding="utf-8")
        for item in (
            'symbol_group = select({',
            '"@platforms//os:windows": "pdb_file"',
            'fail("Missing %s for %s"',
            'fail("Expected one %s for %s"',
            'cfg = _release_profile',
            'allow_duplicates_with_different_content = False',
            'mode = "0755"',
        ):
            with self.subTest(item=item):
                self.assertIn(item, rule)
        for setting in (
            "build:release --compilation_mode=opt",
            "build:release --@rules_rust//rust/settings:lto=thin",
            "build:release --@rules_rust//rust/settings:codegen_units=4",
        ):
            self.assertIn(setting, rc)
        self.assertIn('release_binary(\n    binary = ":codex",', cli)
        self.assertIn('build_script_data = ["//codex-rs:codexdd-version.txt"]', cli)

    def test_greens_do_not_promote_109_owners_or_product(self):
        m = json.loads(MATRIX.read_text(encoding="utf-8"))
        self.assertEqual(len(m["paths"]), 109)
        for row in m["paths"]:
            self.assertEqual(row["semantic_status"], "PENDING_OWNER_REVIEW")
            self.assertEqual(row["release_status"], "NOT_ACCEPTED")
        core = next(row for row in m["paths"] if row["path"] == "codex-rs/core/src/context_manager/normalize.rs")
        self.assertEqual(core["focused_review"]["focused_native_run_3f40b"], 40765870072)
        self.assertEqual(core["focused_review"]["focused_native_result"], "EIGHT_CORE_SELECTORS_PASS_TEN_ACTUAL_TEST_CASES")
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")
        self.assertEqual((ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(), "rust-v0.162.0")


if __name__ == "__main__":
    unittest.main()
