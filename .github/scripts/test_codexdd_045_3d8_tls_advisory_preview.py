"""Verify 0.4.5 nonrelease TLS+advisory preview cannot waive vulnerabilities."""
import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = ROOT / ".github/workflows/codexdd-045-advisory-decision-preview.yml"


class AdvisoryTrialSafety(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.source = WORKFLOW.read_text(encoding="utf-8")

    def test_exact_tls_patch_and_locked_preview_provenance(self):
        s = self.source
        for pkg, current, patched in (
            ("rustls-webpki", "0.103.15", "0.103.16"),
            ("webpki-roots", "1.0.5", "1.0.10"),
            ("webpki-root-certs", "1.0.5", "1.0.10"),
        ):
            self.assertIn(f"-p {pkg}@{current} --precise {patched}", s)
        self.assertIn("cargo metadata --manifest-path codex-rs/Cargo.toml --locked", s)
        self.assertIn("'a6cd27b0b70fde2f2be9c1777f3666ad3fa318bb'", s)
        self.assertIn("test \"$(git diff --name-only)\" = 'codex-rs/Cargo.lock'", s)
        self.assertIn("'a8eb8a076276cb3db5674b33c67d743b1c3d21fc'", s)

    def test_exactly_four_inherited_maintenance_preview_ids(self):
        s = self.source
        ids = re.findall(r'^\s+"(RUSTSEC-2026-\d+)",\s*$', s, re.M)
        self.assertEqual(ids, [
            "RUSTSEC-2026-0374", "RUSTSEC-2026-0371",
            "RUSTSEC-2026-0370", "RUSTSEC-2026-0372",
        ])
        self.assertIn("if actual != existing + list(ids):", s)
        self.assertIn("if parsed.get(section) != baseline.get(section):", s)
        self.assertNotIn("RUSTSEC-2026-0381\",", s)
        self.assertNotIn("RUSTSEC-2026-0382\",", s)
        self.assertNotIn("RUSTSEC-2026-0383\",", s)

    def test_correct_cargo_deny_argument_boundary_and_restore(self):
        s = self.source
        self.assertIn("arguments: --all-features\n          command-arguments: --config ./codex-rs/deny-preview.toml", s)
        self.assertNotIn("arguments: --all-features --config", s)
        self.assertIn("continue-on-error: true", s)
        self.assertIn("git restore --source=HEAD -- codex-rs/Cargo.lock", s)
        self.assertIn("rm -f codex-rs/deny-preview.toml", s)
        self.assertIn("if: always()", s)
        self.assertIn("persist-credentials: false", s)
        self.assertIn("permissions:\n  contents: read\n", s)
        self.assertNotIn("contents: write", s)
        self.assertNotIn("git push", s)

if __name__ == "__main__":
    unittest.main()
