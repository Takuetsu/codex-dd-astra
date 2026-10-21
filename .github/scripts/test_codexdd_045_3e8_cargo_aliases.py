"""Static guards for the Cargo rename -> Bazel dependency alias contract."""
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2]


class BazelCargoAliasContract(unittest.TestCase):
    def test_generated_alias_mapping_applied_to_library_binaries_and_tests(self):
        s = (ROOT / "defs.bzl").read_text(encoding="utf-8")
        self.assertIn('load("@crates//:defs.bzl", "aliases", "all_crate_deps")', s)
        self.assertEqual(s.count("aliases = aliases("), 7)
        self.assertIn("lib_rule(\n            name = name,\n            crate_name = crate_name,\n            crate_features = crate_features,", s)
        self.assertIn("aliases = aliases(),\n            deps = all_crate_deps()", s)
        # rules_rs 0.0.96 generates aliases(package_name=None): no kind filters.
        self.assertEqual(s.count("aliases = aliases()"), 7)
        self.assertNotIn("aliases(normal =", s)
        self.assertNotIn("aliases(normal_dev =", s)

    def test_existing_pinned_yaml_replacement_and_parser_remain_unchanged(self):
        manifest = (ROOT / "codex-rs/skills/Cargo.toml").read_text(encoding="utf-8")
        workspace = (ROOT / "codex-rs/Cargo.toml").read_text(encoding="utf-8")
        parser = (ROOT / "codex-rs/skills/src/parser.rs").read_text(encoding="utf-8")
        self.assertIn("serde_yaml = { workspace = true }", manifest)
        self.assertIn('serde_yaml = { package = "noyalib-serde-yaml", version = "=0.0.57" }', workspace)
        self.assertIn("serde_yaml::from_str", parser)

    def test_read_only_skills_smoke_isolated_from_cli_release_build(self):
        flow = (ROOT / ".github/workflows/codexdd-045-windows-bazel-skills-alias.yml").read_text(encoding="utf-8")
        self.assertIn("permissions:\n  contents: read", flow)
        self.assertIn("persist-credentials: false", flow)
        self.assertIn("runs-on: windows-2022", flow)
        self.assertIn("'//codex-rs/skills:skills'", flow)
        self.assertIn("'--lockfile_mode=error'", flow)
        self.assertIn("Missing skills native compile success receipt", flow)
        self.assertNotIn("contents: write", flow)
        self.assertNotIn("git push", flow)
        self.assertNotIn("BAZEL_MSVC_RUNTIME_VISUAL_STUDIO_EULA=1", flow)
        self.assertNotIn("runtime_package_combined", flow)
        self.assertNotIn("'//codex-rs/cli:release_files'", flow)


if __name__ == "__main__":
    unittest.main()
