"""Exact-lock CI trigger census after the 0.4.5 TLS security patch.

Historical one-shot probes remain on their ORIGINAL lock and must not silently
be repurposed for the patched candidate. Active integration gates must verify
the new exact lock rather than disable safety checks.
"""
from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[2]
WORKFLOWS = ROOT / ".github" / "workflows"
LEGACY_LOCK = "a8eb8a076276cb3db5674b33c67d743b1c3d21fc"
PATCHED_LOCK = "a6cd27b0b70fde2f2be9c1777f3666ad3fa318bb"
INTEGRATION_BRANCH = "dd/codexdd-v0.4.5-upstream-0.163.0-integration"

ACTIVE = (
    "codexdd-045-generated-schema-drift.yml",
    "codexdd-045-windows-appserver-safety.yml",
    "codexdd-045-windows-bazel-core-plugins-alias.yml",
    "codexdd-045-windows-bazel-release-analysis.yml",
    "codexdd-045-windows-bazel-skills-alias.yml",
    "codexdd-045-windows-bazel-zstd-smoke.yml",
    "codexdd-045-windows-cli-focused.yml",
    "codexdd-045-windows-core-authority.yml",
    "codexdd-045-windows-core-scenarios-compile.yml",
    "codexdd-045-windows-focused-native.yml",
    "codexdd-045-windows-guardian-focused.yml",
    "codexdd-045-windows-history-compat.yml",
    "codexdd-045-windows-registry-focused.yml",
    "codexdd-045-windows-remaining-yaml-aliases.yml",
    "codexdd-045-windows-rollout-repair.yml",
    "codexdd-045-windows-zip9-focused.yml",
)
HISTORICAL = (
    "codexdd-045-advisory-decision-preview.yml",
    "codexdd-045-dependency-role-recon.yml",
    "codexdd-045-inherited-advisory-recon.yml",
    "codexdd-045-tls-root-patch-preview.yml",
    "codexdd-045-windows-cli-optimized-preflight.yml",
    "codexdd-045-windows-cli-release-smoke.yml",
    "codexdd-045-windows-secrets-focused.yml",
    "codexdd-045-zip9-feature-lock-blob-import.yml",
)


class TlsLockWorkflowCensus(unittest.TestCase):
    def test_active_workflows_keep_exact_patched_hash_and_safety_floor(self):
        self.assertEqual(len(ACTIVE), 16)
        for name in ACTIVE:
            with self.subTest(name=name):
                source = (WORKFLOWS / name).read_text(encoding="utf-8")
                self.assertIn(PATCHED_LOCK, source)
                self.assertNotIn(LEGACY_LOCK, source)
                self.assertIn(INTEGRATION_BRANCH, source)
                self.assertIn("permissions:\n  contents: read\n", source)
                self.assertNotIn("contents: write", source)
                self.assertNotIn("git push", source)

    def test_historical_workflows_keep_original_byte_identity(self):
        self.assertEqual(len(HISTORICAL), 8)
        for name in HISTORICAL:
            with self.subTest(name=name):
                source = (WORKFLOWS / name).read_text(encoding="utf-8")
                self.assertIn(LEGACY_LOCK, source)
                self.assertIn(INTEGRATION_BRANCH, source)
                self.assertNotIn("contents: write", source)
                header = source.split("\npermissions:\n", 1)[0]
                paths = re.findall(r"^      - '([^']+)'$", header, re.M)
                self.assertNotIn("codex-rs/Cargo.lock", paths)
                self.assertNotIn("codex-rs/**", paths)
                self.assertTrue(paths)
                # Changing the verified lock alone must not restart legacy probes.
                self.assertFalse(any(
                    p == "codex-rs/Cargo.lock" or p.startswith("codex-rs/**")
                    for p in paths
                ))

    def test_stale_hash_cannot_hide_in_another_045_workflow(self):
        with_legacy = {
            p.name for p in WORKFLOWS.glob("codexdd-045-*.yml")
            if LEGACY_LOCK in p.read_text(encoding="utf-8")
        }
        self.assertEqual(with_legacy, set(HISTORICAL))

    def test_release_markers_remain_unpromoted(self):
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")
        self.assertEqual((ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(), "rust-v0.162.0")
        self.assertFalse((ROOT / "codex-rs/deny-preview.toml").exists())


if __name__ == "__main__":
    unittest.main()
