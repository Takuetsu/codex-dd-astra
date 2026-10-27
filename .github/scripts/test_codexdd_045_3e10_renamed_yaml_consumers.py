"""Guard all declared serde_yaml rename consumers without reintroducing Bazel cycles."""

from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[2]

DIRECT_CONSUMERS = {
    "codex-rs/skills",
    "codex-rs/core-plugins",
    "codex-rs/ext/skills",
    "codex-rs/external-agent-migration",
}
BUILD_BLOBS = {
    "codex-rs/skills": "9f4e136ec76fc298e32ccb94fd0800b1f5656c76",
    "codex-rs/core-plugins": "16dc432cd2f9b6e16de5b4f367a64654a2e492e6",
    "codex-rs/ext/skills": "e013d31eb8837cc3a7296e216c941427cf1cb084",
    "codex-rs/external-agent-migration": "1ae14ac88a1a765328a26026e65a3bd0328e8f17",
}


class AllDeclaredYamlAliases(unittest.TestCase):
    def test_exact_consumer_roster_across_all_cargo_manifests(self):
        files = (ROOT / "codex-rs").rglob("Cargo.toml")
        matches = {
            p.parent.relative_to(ROOT).as_posix()
            for p in files
            if re.search(
                r"(?m)^serde_yaml\s*=\s*\{\s*workspace\s*=\s*true\s*\}",
                p.read_text(encoding="utf-8"),
            )
        }
        self.assertEqual(matches, DIRECT_CONSUMERS)
        self.assertEqual(len(matches), 4)

    def test_each_declared_consumer_has_its_own_filtered_alias_only(self):
        defs = (ROOT / "defs.bzl").read_text(encoding="utf-8")
        self.assertIn("lib_aliases = None", defs)
        self.assertIn("aliases = lib_aliases or {}", defs)
        self.assertNotIn('load("@crates//:defs.bzl", "aliases"', defs)
        for dirname in sorted(DIRECT_CONSUMERS):
            with self.subTest(crate=dirname):
                b = (ROOT / dirname / "BUILD.bazel").read_text(encoding="utf-8")
                self.assertIn('load("@crates//:defs.bzl", "aliases")', b)
                self.assertEqual(b.count('if local == "serde_yaml"'), 1)
                self.assertEqual(b.count("lib_aliases = _yaml_aliases"), 1)
                self.assertNotRegex(b, r"(?m)^if[ \t]+")
                self.assertNotIn("aliases = aliases(", b)
        workspace = (ROOT / "codex-rs/Cargo.toml").read_text(encoding="utf-8")
        self.assertIn(
            'serde_yaml = { package = "noyalib-serde-yaml", version = "=0.0.57" }',
            workspace,
        )

    def test_remaining_two_windows_smoke_is_read_only_frozen_and_exact(self):
        f = (
            ROOT / ".github/workflows/codexdd-045-windows-remaining-yaml-aliases.yml"
        ).read_text(encoding="utf-8")
        self.assertIn("permissions:\n  contents: read", f)
        self.assertIn("persist-credentials: false", f)
        self.assertIn("runs-on: windows-2022", f)
        self.assertIn("timeout-minutes: 60", f)
        self.assertIn("'--lockfile_mode=error'", f)
        self.assertIn("'--jobs=2'", f)
        self.assertIn("'//codex-rs/ext/skills:skills'", f)
        self.assertIn(
            "'//codex-rs/external-agent-migration:external-agent-migration'", f
        )
        self.assertIn(
            "Missing remaining YAML consumer native compile success receipt", f
        )
        for dirname, sha in BUILD_BLOBS.items():
            self.assertIn("'" + dirname + "/BUILD.bazel' = '" + sha + "'", f)
        self.assertNotIn("'//codex-rs/cli:release_files'", f)
        self.assertNotIn("contents: write", f)
        self.assertNotIn("git push", f)
        self.assertNotIn("BAZEL_MSVC_RUNTIME_VISUAL_STUDIO_EULA=1", f)


if __name__ == "__main__":
    unittest.main()
