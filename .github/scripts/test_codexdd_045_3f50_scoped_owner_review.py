"""0.4.5: exactly five scoped Bazel/dependency source reviews, no semantic signoff."""
from __future__ import annotations

import json
import subprocess
import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
EVIDENCE = ROOT / 'docs/upstream-candidates/codexdd-045-3f50-scoped-owner-review.json'
MATRIX = ROOT / 'docs/upstream-candidates/codexdd-045-109-semantic-owner-matrix.json'
DISTRIBUTION = ROOT / 'docs/upstream-candidates/codexdd-045-windows-distribution-contract.json'


class ScopedOwnerReview(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.review = json.loads(EVIDENCE.read_text(encoding='utf-8'))
        cls.matrix = json.loads(MATRIX.read_text(encoding='utf-8'))
        cls.dist = json.loads(DISTRIBUTION.read_text(encoding='utf-8'))

    def test_only_five_explicit_scoped_reviews_and_no_signoff(self):
        result = self.review
        self.assertEqual(result['contract_version'], 1)
        self.assertEqual(result['status'], 'NONRELEASE_NOT_ACCEPTED')
        self.assertEqual(result['scope'], 'SCOPED_SOURCE_REVIEW_NOT_FULL_OWNER_APPROVAL')
        self.assertEqual(result['source_reviewed_count'], 5)
        self.assertEqual(result['remaining_owner_signoffs'], 109)
        self.assertEqual(result['total_paths'], 109)
        self.assertEqual(result['total_original_conflicts'], 15)
        names = {row['path'] for row in result['reviewed']}
        self.assertEqual(names, {
            '.github/workflows/bazel.yml', 'codex-rs/BUILD.bazel',
            'codex-rs/cli/BUILD.bazel', 'codex-rs/Cargo.toml',
            'codex-rs/Cargo.lock',
        })
        self.assertEqual(len(result['reviewed']), len(names))

    def test_source_semantics_are_sha_bound_and_no_release_status_changed(self):
        matrix = {row['path']: row for row in self.matrix['paths']}
        self.assertEqual(len(matrix), 109)
        for row in self.review['reviewed']:
            with self.subTest(row=row['path']):
                path = row['path']
                self.assertIn(path, matrix)
                self.assertEqual(row['owner'], matrix[path]['owner'])
                self.assertEqual(row['current_blob'], matrix[path]['current_blob'])
                self.assertEqual(row['original_direct_conflict'], matrix[path]['original_direct_conflict'])
                self.assertEqual(row['semantic_signoff'], 'NOT_GRANTED')
                self.assertTrue(row['remaining_semantic_gate'])
                self.assertTrue(row['source_reviewed'])
                self.assertTrue(all(isinstance(r, int) and r > 0 for r in row['native_evidence_runs']))
                self.assertEqual(matrix[path]['semantic_status'], 'PENDING_OWNER_REVIEW')
                self.assertEqual(matrix[path]['release_status'], 'NOT_ACCEPTED')
                actual = subprocess.check_output(
                    ['git', 'rev-parse', 'HEAD:' + path], cwd=ROOT, text=True
                ).strip()
                self.assertEqual(actual, row['current_blob'])
                source = (ROOT / path).read_text(encoding='utf-8')
                for fragment in row['source_fragments']:
                    self.assertIn(fragment, source)

    def test_secured_cargo_lock_has_only_fixed_crypto_bigint(self):
        lock = tomllib.loads((ROOT / 'codex-rs/Cargo.lock').read_text(encoding='utf-8'))
        versions = sorted(p['version'] for p in lock['package'] if p['name'] == 'crypto-bigint')
        self.assertEqual(versions, ['0.7.10'])
        self.assertIn('13bc15ff844e93dfaf7777c73bb7313b35bb3b81',
                      [r['current_blob'] for r in self.review['reviewed']])
        deny = (ROOT / 'codex-rs/deny.toml').read_text(encoding='utf-8')
        self.assertNotIn('RUSTSEC-2026-0387', deny)
        self.assertIn('unknown-git = "deny"', deny)

    def test_combined_build_has_full_manifest_but_not_full_installer(self):
        self.assertEqual(self.dist['build_evidence']['run_id'], 40922368730)
        self.assertEqual(len(self.dist['build_evidence']['combined_members']), 10)
        self.assertEqual(len(self.dist['combined_optimized_exes']), 5)
        self.assertEqual(len(self.dist['full_installer_binaries']), 7)
        self.assertEqual(len(self.dist['separately_required_windows_exes']), 2)
        self.assertFalse(self.dist['build_evidence']['release_approved'])


if __name__ == '__main__':
    unittest.main()
