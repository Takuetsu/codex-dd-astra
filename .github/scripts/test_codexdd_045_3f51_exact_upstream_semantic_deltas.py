"""3F.51: reconstruct exact official upstream 0.163 source from frozen fork deltas.

The receipts are per-source-hunk evidence, not native runtime or owner signoff.
"""
from __future__ import annotations

import hashlib
import json
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
LEDGER = ROOT / "docs/upstream-candidates/codexdd-045-109-semantic-owner-matrix.json"
OVERLAPS = ROOT / "docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-overlap-source-receipt.json"
RECEIPTS = (
    ROOT / "docs/upstream-candidates/codexdd-045-3f51-appserver-upstream-delta.json",
    ROOT / "docs/upstream-candidates/codexdd-045-3f51-core-tui-upstream-delta.json",
)
EXPECTED = {
    "codex-rs/app-server-protocol/src/protocol/common.rs": ("6cb1a33ea48ed0de73f420ff6c179d9629fec45a", "b710f18f14e711a748318ee980bcdc4168a7d843"),
    "codex-rs/app-server-protocol/src/protocol/thread_history.rs": ("82c980c427501b594389fed439e105c2f696b9a5", "53817bb9e50e87ad8a85f39c6a27037bc87d5081"),
    "codex-rs/app-server-protocol/src/protocol/v2/thread.rs": ("4180b91565e1a47b38c4f40114716e55b2973d3f", "1746ceb7bcf4ed50ceca643170ffc65dada37f41"),
    "codex-rs/app-server-protocol/src/protocol/v2/turn.rs": ("04f38d2535ba46da2bc22358589aeb1bd2840499", "b09a191289349a68e6602a30ec9d887664dab73f"),
    "codex-rs/app-server/src/message_processor.rs": ("d9d1b8403c7bd9bb963f4d3a3b8d24ddf06a393a", "c9858cd9d513c69a917f6505410ee89b16bdd955"),
    "codex-rs/app-server/src/request_processors/turn_processor.rs": ("1186f0a181704d8ea45243a850f971f1057cb080", "1b9a253becc97a7a2c1b0de9e92def7c192691c2"),
    "codex-rs/core/src/context_manager/history_tests.rs": ("4f8415b4ce3f3994d4fe8eb87bcbd22e6588848d", "8f22910ed2086c07edbd41f84a1327705f4700f3"),
    "codex-rs/tui/src/app/tests/session_lifecycle_requests.rs": ("7d2be3352380d43df714399b441a95960b87a59f", "062efa383da797e46f6aa56540cbf76a9c130353"),
    "codex-rs/tui/src/app/tests/startup_defaults_tests.rs": ("e60808df0448682d47af1a14b5bd8d5c52206397", "86e0da6fd4e0550050cc1a704186a9a07038278b"),
    "codex-rs/tui/src/app_server_session/workspace_roots_tests.rs": ("d6447d22a0a1e943b1352b8c945cf7f6cf09734e", "525eed5ece8b9c0110b3b296b68fb6c727d6d096"),
}

def git_blob_sha(data: bytes) -> str:
    return hashlib.sha1(b"blob " + str(len(data)).encode("ascii") + b"\x00" + data).hexdigest()

def current_blob(path: str) -> bytes:
    return subprocess.check_output(["git", "show", "HEAD:" + path], cwd=ROOT)

class ExactUpstreamSemanticDelta(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.receipts = [json.loads(path.read_text(encoding="utf-8")) for path in RECEIPTS]
        cls.rows = [row for doc in cls.receipts for row in doc["paths"]]
        cls.matrix = json.loads(LEDGER.read_text(encoding="utf-8"))
        cls.overlaps = {x["path"]: x for x in json.loads(OVERLAPS.read_text(encoding="utf-8"))["overlaps"]}

    def test_exact_ten_audited_paths_and_no_approval(self):
        self.assertEqual([doc["reviewed_count"] for doc in self.receipts], [6, 4])
        self.assertEqual(len(self.rows), 10)
        self.assertEqual({r["path"] for r in self.rows}, set(EXPECTED))
        self.assertEqual(len({r["path"] for r in self.rows}), 10)
        for doc in self.receipts:
            self.assertEqual(doc["contract_version"], 1)
            self.assertEqual(doc["scope"], "NONRELEASE_EXACT_OFFICIAL_UPSTREAM_0163_RECONSTRUCTION")
            self.assertEqual(doc["official_ref"], "openai/codex@rust-v0.163.0")
            self.assertIs(doc["release_approved"], False)
        self.assertEqual(len(self.matrix["paths"]), 109)
        self.assertEqual(self.matrix["release_status"], "NOT_ACCEPTED")
        self.assertTrue(all(x["semantic_status"] == "PENDING_OWNER_REVIEW" and x["release_status"] == "NOT_ACCEPTED" for x in self.matrix["paths"]))

    def test_fork_restoration_byte_equals_exact_official_upstream_git_blob(self):
        owners = {r["path"]: r for r in self.matrix["paths"]}
        for row in self.rows:
            with self.subTest(path=row["path"]):
                path = row["path"]
                candidate, upstream = EXPECTED[path]
                self.assertEqual((row["candidate_blob"], row["upstream_blob"]), (candidate, upstream))
                self.assertEqual(row["review_kind"], "REVERSIBLE_SOURCE_DELTA_ONLY")
                self.assertEqual(row["semantic_signoff"], "NOT_GRANTED")
                self.assertEqual(owners[path]["current_blob"], candidate)
                self.assertEqual(owners[path]["semantic_status"], "PENDING_OWNER_REVIEW")
                if path in self.overlaps:
                    self.assertEqual(self.overlaps[path]["official_163_blob"], upstream)
                source = current_blob(path)
                self.assertEqual(git_blob_sha(source), candidate)
                text = source.decode("utf-8").split("\n")
                last_start = len(text) + 1
                for hunk in reversed(row["hunks"]):
                    start = hunk["candidate_start_line"] - 1
                    self.assertLess(start, last_start)
                    self.assertGreaterEqual(start, 0)
                    replacement = hunk["candidate_lines"]
                    self.assertEqual(text[start:start + len(replacement)], replacement)
                    text[start:start + len(replacement)] = hunk["upstream_lines"]
                    last_start = start
                self.assertEqual(git_blob_sha("\n".join(text).encode("utf-8")), upstream)

    def test_appserver_rpc_is_thread_serialized_and_model_history_excludes_private_state(self):
        common = (ROOT / "codex-rs/app-server-protocol/src/protocol/common.rs").read_text(encoding="utf-8")
        thread_history = (ROOT / "codex-rs/app-server-protocol/src/protocol/thread_history.rs").read_text(encoding="utf-8")
        request = (ROOT / "codex-rs/app-server/src/message_processor.rs").read_text(encoding="utf-8")
        processor = (ROOT / "codex-rs/app-server/src/request_processors/turn_processor.rs").read_text(encoding="utf-8")
        self.assertIn('ThreadWorkflowStateUpdate => "thread/workflowState/update"', common)
        self.assertIn('serialization: thread_id(params.thread_id)', common)
        self.assertIn('| RolloutItem::WorkflowState(_)', thread_history)
        self.assertIn('ClientRequest::ThreadWorkflowStateUpdate { params, .. }', request)
        self.assertIn('.thread_workflow_state_update(params)', request)
        self.assertIn('codex_rollout::WorkflowStateItem::clear()', processor)
        self.assertIn('codex_rollout::WorkflowStateItem::set_adaptive_state(', processor)
        self.assertIn('.persist_workflow_state(item)', processor)

    def test_core_history_conflict_is_explicit_and_does_not_silently_drop_orphan_guards(self):
        source = (ROOT / "codex-rs/core/src/context_manager/history_tests.rs").read_text(encoding="utf-8")
        self.assertEqual(source.count('#[test_case::test_case(false; "local")]'), 2)
        self.assertNotIn("fn normalize_adds_missing_output_for_custom_tool_call_panics_in_debug", source)
        self.assertNotIn("fn normalize_adds_missing_output_for_local_shell_call_with_id_panics_in_debug", source)
        for name in (
            "normalize_removes_orphan_function_call_output_panics_in_debug",
            "normalize_removes_orphan_custom_tool_call_output_panics_in_debug",
            "normalize_removes_orphan_client_tool_search_output_panics_in_debug",
        ):
            self.assertIn("fn " + name, source)
        normalize = (ROOT / "codex-rs/core/src/context_manager/normalize.rs").read_text(encoding="utf-8")
        self.assertIn("synthesizing aborted output", normalize)
        self.assertIn('synthetic_output_id("ctco"', normalize)

    def test_tui_conflicts_preserve_upstream_tests_and_track_stack_boxing_difference(self):
        startup = (ROOT / "codex-rs/tui/src/app/tests/startup_defaults_tests.rs").read_text(encoding="utf-8")
        workspace = (ROOT / "codex-rs/tui/src/app_server_session/workspace_roots_tests.rs").read_text(encoding="utf-8")
        lifecycle = (ROOT / "codex-rs/tui/src/app/tests/session_lifecycle_requests.rs").read_text(encoding="utf-8")
        self.assertIn("/*adaptive_family*/ None,", startup)
        self.assertIn("fresh_startup_uses_server_defaults_with_explicit_and_managed_precedence", startup)
        self.assertIn("/*turn_trigger*/ None,", workspace)
        self.assertIn("remote_workspace_roots_survive_start_turn_resume_and_fork", workspace)
        self.assertIn("local_daemon_registers_approval_gated_mcp_tools_for_both_start_paths", lifecycle)
        self.assertIn("boxed_future!(local_daemon_registers_approval_gated_mcp_tools_for_both_start_paths_scenario())", lifecycle)
        self.assertIn("dynamic_tool_requests_ignore_other_namespaces_and_dispatch_tui_namespace", lifecycle)
        self.assertIn("older_external_server_starts_without_unsupported_dynamic_tools_or_history", lifecycle)
        self.assertIn('vec![None, None, None, Some("daybreakBlue"), None]', lifecycle)

if __name__ == "__main__":
    unittest.main()
