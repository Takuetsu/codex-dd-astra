"""Safety checks for owner-dispatched v2 publication and PR-only promotion."""
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[2] / ".github" / "workflows"
PUBLISH = (ROOT / "upstream-reconcile-publish.yml").read_text()
PROMOTE = (ROOT / "upstream-reconcile-promote.yml").read_text()

class ManualWorkflowSafetyTests(unittest.TestCase):
    def test_both_require_owner_dispatch_only(self):
        for src in (PUBLISH, PROMOTE):
            trigger = src.split("\non:\n", 1)[1].split("\npermissions:\n", 1)[0]
            self.assertIn("  workflow_dispatch:", trigger)
            for event in ("push:", "pull_request:", "schedule:", "workflow_run:"):
                self.assertNotIn("\n  " + event, trigger)
            self.assertIn("if: github.actor == 'Takuetsu'", src)
            self.assertIn("cancel-in-progress: false", src)
            self.assertIn("ref: dd/astra-policy-v2", src)
            self.assertIn("c1382380de69521303b416720a52f42d51af6248", src)
            self.assertIn("upstream-codex-discovery", src)
            self.assertIn("gh run download", src)
            self.assertIn("sha256sum --check --status", src)
            self.assertIn("--mode promotion", src)
            self.assertIn('["discovery_run_id"]', src)
            self.assertNotIn("gh pr merge", src)
    def test_publish_is_single_absent_branch_only(self):
        self.assertIn("contents: write", PUBLISH)
        self.assertNotIn("pull-requests: write", PUBLISH)
        self.assertIn("git push --force-with-lease=", PUBLISH)
        self.assertIn("refs/heads/$" + "{CANDIDATE_BRANCH}:", PUBLISH)
        self.assertIn("git ls-remote --exit-code --heads origin", PUBLISH)
        self.assertIn("construct_local_v2_candidate", PUBLISH)
        self.assertNotIn("gh pr create", PUBLISH)
    def test_promotion_has_no_git_write_and_requires_release_receipt(self):
        self.assertIn("contents: read", PROMOTE)
        self.assertIn("pull-requests: write", PROMOTE)
        self.assertNotIn("git push", PROMOTE)
        self.assertIn("gh pr create", PROMOTE)
        self.assertIn("--state all", PROMOTE)
        self.assertIn("validate_v2_owner_review_and_receipt", PROMOTE)
        self.assertIn("normalize_console_or_file_receipt_json", PROMOTE)
        self.assertIn("OWNER_ACCEPTANCE", PROMOTE)
        self.assertNotIn("codexdd-test-release.ps1", PROMOTE)
    def test_v1_workflows_never_implicitly_invoke_v2(self):
        for filename in ("upstream-prepare.yml", "upstream-promote.yml"):
            text = (ROOT / filename).read_text()
            self.assertNotIn("upstream_reconciliation_stage", text)
            self.assertNotIn("upstream_reconciliation_promotion", text)

if __name__ == "__main__":
    unittest.main()
