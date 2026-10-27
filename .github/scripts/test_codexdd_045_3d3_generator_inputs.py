"""Read-only 0.4.5 schema drift workflow must observe producer and resolver edits."""

from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github/workflows/codexdd-045-generated-schema-drift.yml"


class GeneratorInputTriggers(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.source = WORKFLOW.read_text(encoding="utf-8")
        header = cls.source.split("\npermissions:\n", 1)[0]
        cls.trigger_paths = set(re.findall(r"^      - '([^']+)'$", header, re.M))

    def test_input_paths_include_resolver_and_generators(self):
        required = {
            ".github/workflows/codexdd-045-generated-schema-drift.yml",
            "codex-rs/Cargo.toml",
            "codex-rs/Cargo.lock",
            "codex-rs/app-server-protocol/Cargo.toml",
            "codex-rs/app-server-protocol/src/**",
            "codex-rs/app-server-protocol/scripts/**",
            "codex-rs/core/src/config/**",
            "codex-rs/config-schema/**",
            "sdk/python/scripts/**",
            "sdk/python/pyproject.toml",
            "sdk/python/uv.lock",
            "codex-rs/app-server-protocol/schema/**",
            "codex-rs/core/config.schema.json",
            "sdk/python/src/openai_codex/generated/**",
        }
        self.assertFalse(required - self.trigger_paths, required - self.trigger_paths)

    def test_source_bound_nonrelease_permissions(self):
        s = self.source
        self.assertIn("permissions:\n  contents: read\n", s)
        self.assertIn("persist-credentials: false", s)
        self.assertIn("runs-on: ubuntu-24.04", s)
        self.assertIn(
            "github.ref == 'refs/heads/dd/codexdd-v0.4.5-upstream-0.163.0-integration'",
            s,
        )
        self.assertNotIn("contents: write", s)
        self.assertNotIn("git push", s)

    def test_locked_reproduction_and_no_drift_acceptance(self):
        s = self.source
        self.assertIn("cargo metadata --locked --format-version 1", s)
        self.assertIn("'a6cd27b0b70fde2f2be9c1777f3666ad3fa318bb'", s)
        self.assertIn("write_schema_fixtures.py --experimental", s)
        self.assertIn("codex-write-config-schema", s)
        self.assertIn("precomputed_exports_tests::", s)
        self.assertIn("test_contract_generation.py", s)
        self.assertIn("if changed or deleted:", s)
        self.assertIn("if forbidden:", s)
        self.assertIn(
            'raise SystemExit("BLOCKED: generated outputs differ from checkout;', s
        )
        self.assertIn("retention-days: 7", s)


if __name__ == "__main__":
    unittest.main()
