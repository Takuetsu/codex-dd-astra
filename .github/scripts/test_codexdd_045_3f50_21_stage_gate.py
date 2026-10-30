"""3F.50: final Daniel-CL 21-stage launcher must fail closed before expensive work."""
from __future__ import annotations

import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LAUNCHER = (ROOT / 'scripts/codexdd-045-run-final-21.ps1').read_text(encoding='utf-8')
RELEASE = (ROOT / 'scripts/codexdd-test-release.ps1').read_text(encoding='utf-8')


class NativeReleaseGate(unittest.TestCase):
    def test_existing_release_profile_still_has_exactly_21_unique_stages(self):
        names = re.findall(r'New-CodexDDValidationStage -Name "([^"]+)"', RELEASE)
        self.assertEqual(len(names), 21)
        self.assertEqual(len(set(names)), 21)
        self.assertIn('diff-check', names)
        self.assertIn('cli-release-build', names)
        self.assertIn('tui-daemon-startup-tests', names)
        self.assertIn('cli-daemon-startup-tests', names)
        self.assertIn('Invoke-CodexDDValidationProfile -Profile "release"', RELEASE)

    def test_no_release_profile_without_frozen_source_and_all_semantics(self):
        for token in (
            "[ValidateSet('Preflight', 'Final')]",
            "$Mode = 'Preflight'",
            "'Daniel-CL'",
            "-ExpectedCommit",
            "Candidate HEAD differs from explicit expected commit",
            "Tracked source dirty before validation",
            "owner-preflight.ps1",
            "codexdd_045_distribution_gate.py",
            "test_codexdd_045_3f50_scoped_owner_review.py",
            "test_codexdd_045_3f50_distribution_gate.py",
            "$rows.Count -ne 109",
            "$_.semantic_status -ne 'OWNER_ACCEPTED'",
            "Final profile BLOCKED:",
            "FULL_21_STAGE_RELEASE_PROFILE=NOT_EXECUTED",
        ):
            with self.subTest(token=token):
                self.assertIn(token, LAUNCHER)

    def test_terminal_receipt_is_not_approving_distribution(self):
        for item in (
            "codexdd-test-release.ps1",
            "'CODEXDD_VALIDATION_JSON '",
            "$begins[0].stage_count -ne 21",
            "$ends[0].completed_stages -ne 21",
            "$passed.Count -ne 21",
            "candidate_sha = $head",
            "release_approved = $false",
            "combined_distribution_verified = $false",
            "NATIVE_RELEASE_PROFILE_PASS_NOT_PRODUCT_APPROVAL",
            "TOTAL_TIME_WORKED_SECONDS:",
            "DONE_UTC:",
        ):
            self.assertIn(item, LAUNCHER)
        self.assertNotIn("gh release create", LAUNCHER)
        self.assertNotIn("git push", LAUNCHER)
        self.assertNotIn("Set-ExecutionPolicy Unrestricted", LAUNCHER)


if __name__ == '__main__':
    unittest.main()
