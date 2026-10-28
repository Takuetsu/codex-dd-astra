"""Reconstruct the exact 0.163 auth matrix test change from pinned Git blobs, no release signoff."""

from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RECEIPT = ROOT / "docs/upstream-candidates/codexdd-045-3f25-dynamic-auth-test-edits.json"


def git(*args: str) -> bytes:
    return subprocess.check_output(["git", *args], cwd=ROOT)


def oid(raw: bytes) -> str:
    return subprocess.check_output(
        ["git", "hash-object", "--stdin"], input=raw, cwd=ROOT
    ).decode("ascii").strip()


class DynamicToolAuthMatrixRestoration(unittest.TestCase):
    def test_only_eight_intended_upstream_test_contract_edits(self):
        receipt = json.loads(RECEIPT.read_text(encoding="utf-8"))
        self.assertEqual(receipt["contract_version"], 1)
        self.assertEqual(receipt["source_scope"], "NONRELEASE_EXACT_OFFICIAL_163_TEST_RESTORATION")
        self.assertEqual(receipt["release_status"], "NOT_ACCEPTED")
        self.assertEqual(receipt["before_blob"], "994d54b631493b3d0784e836c544c57a275e9985")
        self.assertEqual(receipt["upstream_blob"], "062efa383da797e46f6aa56540cbf76a9c130353")
        self.assertEqual(len(receipt["edits"]), 8)
        source_path = ROOT / receipt["path"]
        prior = git("cat-file", "blob", receipt["before_blob"])
        self.assertEqual(oid(prior), receipt["before_blob"])
        for edit in receipt["edits"]:
            before, after = edit["before"].encode("utf-8"), edit["after"].encode("utf-8")
            self.assertEqual(prior.count(before), edit["count"])
            prior = prior.replace(before, after)
        # Windows checkout may normalize LF Git blobs to CRLF in the working tree.
        # Require no tracked source changes, then compare exact committed Git bytes.
        subprocess.run(
            ["git", "diff", "--quiet", "HEAD", "--", receipt["path"]],
            cwd=ROOT,
            check=True,
        )
        actual = git("cat-file", "blob", "HEAD:" + receipt["path"])
        self.assertEqual(prior, actual)
        self.assertEqual(oid(actual), receipt["after_blob"])
        self.assertEqual(
            git("rev-parse", "HEAD:" + receipt["path"]).decode("ascii").strip(),
            receipt["after_blob"],
        )

    def test_upstream_matrix_and_codexdd_adaptive_cases_are_both_present(self):
        receipt = json.loads(RECEIPT.read_text(encoding="utf-8"))
        source = (ROOT / receipt["path"]).read_text(encoding="utf-8")
        for fragment in (
            '(true, Chatgpt, false, Current, Some("daybreakBlue"))',
            '(false, Chatgpt, false, Current, None)',
            '(true, ApiKey, false, Current, None)',
            '(true, ApiKey, true, Current, Some("daybreakBlue"))',
            '(true, Chatgpt, false, ConfigReadUnsupported(-32601), None)',
            '(true, Chatgpt, false, ConfigReadUnsupported(-32600), None)',
            "if auth_mode == codex_app_server_protocol::AuthMode::ApiKey",
            "history_capabilities == HistoryCapabilities::Current",
            "serde_json::json!(expected_program)",
            "fresh_fork_inherits_parent_adaptive_state_when_server_state_is_empty",
            "new_session_worker_binding_resets_dirty_adaptive_state",
            "AdaptiveWorkerContext::default()",
        ):
            with self.subTest(fragment=fragment):
                self.assertIn(fragment, source)
        self.assertEqual(
            (ROOT / "codex-rs/codexdd-version.txt").read_text().strip(),
            "0.4.4",
        )
        self.assertEqual(
            (ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(),
            "rust-v0.162.0",
        )


if __name__ == "__main__":
    unittest.main()
