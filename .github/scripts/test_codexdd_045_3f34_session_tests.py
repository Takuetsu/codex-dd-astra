"""Prove bounded official 0.163 TUI auth/Daybreak and WS mock regressions (nonrelease)."""

from __future__ import annotations
import json
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RECEIPT = ROOT / "docs/upstream-candidates/codexdd-045-3f34-session-tests-source-edits.json"
P = "codex-rs/tui/src/app/tests/session_lifecycle_requests.rs"
CORRECTIONS = ROOT / "docs/upstream-candidates/codexdd-045-reviewed-semantic-corrections.json"
MATRIX = ROOT / "docs/upstream-candidates/codexdd-045-109-semantic-owner-matrix.json"


def git(*args: str) -> bytes:
    return subprocess.check_output(["git", *args], cwd=ROOT)


def oid(data: bytes) -> str:
    return subprocess.check_output(
        ["git", "hash-object", "--stdin"], input=data, cwd=ROOT
    ).decode("ascii").strip()


class SessionTestOnlyRestoration(unittest.TestCase):
    def test_seven_exact_edits_from_parent_source(self):
        r = json.loads(RECEIPT.read_text(encoding="utf-8"))
        self.assertEqual(r["source_scope"], "NONRELEASE_EXACT_OFFICIAL_163_SESSION_TEST_RESTORATION")
        self.assertEqual(r["path"], P)
        self.assertEqual(r["before_blob"], "c10799047d696edcafd24d994c84f1fd87fd6a26")
        self.assertEqual(r["after_blob"], "1a99fcac6b070fff957248a5e13ec37910a0e588")
        self.assertEqual(r["release_status"], "NOT_ACCEPTED")
        self.assertEqual(len(r["edits"]), 7)
        current = git("cat-file", "blob", "HEAD:" + P)
        self.assertEqual(oid(current), r["after_blob"])
        historical = current
        # Reverse the current committed source to its original 3F.25 test blob.
        for edit in reversed(r["edits"]):
            with self.subTest(inverse=edit["kind"]):
                before, after = edit["before"].encode(), edit["after"].encode()
                self.assertEqual(edit["count"], 1)
                self.assertEqual(historical.count(after), 1)
                historical = historical.replace(after, before, 1)
        self.assertEqual(oid(historical), r["before_blob"])
        replayed = historical
        for edit in r["edits"]:
            before, after = edit["before"].encode(), edit["after"].encode()
            self.assertEqual(replayed.count(before), 1)
            replayed = replayed.replace(before, after, 1)
        self.assertEqual(replayed, current)
        subprocess.run(["git", "diff", "--quiet", "HEAD", "--", P], cwd=ROOT, check=True)

    def test_five_daybreak_cases_and_all_disconnect_paths(self):
        s = (ROOT / P).read_text(encoding="utf-8")
        for item in (
            "(false, false, true)",
            "(false, true, true)",
            "(true, false, true)",
            "(true, true, true)",
            "(true, true, false)",
            'vec![None, None, None, Some("daybreakBlue"), Some("standard")]',
            'expect("account model refresh")',
            "app.chat_widget.on_models_loaded(request_id, Ok(models))",
            "Feature::ApiKeyCyberAccessPrograms, /*enabled*/ false,",
            "fn is_recording_client_disconnect(error: &WebSocketError) -> bool",
            "ProtocolError::ResetWithoutClosingHandshake",
            "std::io::ErrorKind::ConnectionAborted",
            "result.as_ref().is_err_and(is_recording_client_disconnect)",
        ):
            with self.subTest(fragment=item):
                self.assertIn(item, s)
        self.assertEqual(s.count("is_recording_client_disconnect"), 5)
        self.assertIn("fresh_fork_inherits_parent_adaptive_state_when_server_state_is_empty", s)
        self.assertIn("new_session_worker_binding_resets_dirty_adaptive_state", s)
        self.assertIn("dynamic_tool_requests_ignore_other_namespaces_and_dispatch_tui_namespace", s)

    def test_owner_ledger_and_nonrelease_markers(self):
        m = json.loads(MATRIX.read_text(encoding="utf-8"))
        c = json.loads(CORRECTIONS.read_text(encoding="utf-8"))
        row = next(x for x in m["paths"] if x["path"] == P)
        r = next(x for x in c["paths"] if x["path"] == P)
        self.assertEqual(len(m["paths"]), 109)
        self.assertEqual(row["current_blob"], "1a99fcac6b070fff957248a5e13ec37910a0e588")
        self.assertEqual(row["semantic_status"], "PENDING_OWNER_REVIEW")
        self.assertEqual(row["release_status"], "NOT_ACCEPTED")
        self.assertEqual(row["session_test_reconciliation"]["semantic_signoff"], "NOT_GRANTED")
        self.assertEqual(r["auth_matrix_restoration_blob"], "c10799047d696edcafd24d994c84f1fd87fd6a26")
        self.assertEqual(r["after_blob"], "1a99fcac6b070fff957248a5e13ec37910a0e588")
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")
        self.assertEqual((ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(), "rust-v0.162.0")


if __name__ == "__main__":
    unittest.main()
