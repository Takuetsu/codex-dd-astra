"""CodexDD 0.4.5 packet 3B.4 — offline RPC and workflow-persistence checks.

Static provenance only: never a Rust execution, native release receipt,
end-to-end app-server test, or authorization to publish/update production.
"""

from __future__ import annotations

import hashlib
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
APP = "codex-rs/app-server/src/"
FILES = {
    "message_processor.rs": "d9d1b8403c7bd9bb963f4d3a3b8d24ddf06a393a",
    "request_processors.rs": "5c4633ab48ea77692e28193ea6e128ec58067a37",
    "request_processors/turn_processor.rs": "1186f0a181704d8ea45243a850f971f1057cb080",
    "request_processors/thread_read_state.rs": "a64c31b55e7075cf13d8ac8e29e11f4c188096f3",
    "message_processor_workspace_routing_tests.rs": "db478a99602807fd436b121d4c6c46610e365832",
}


def git_bytes(path: str) -> bytes:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT,
        check=True,
        capture_output=True,
    ).stdout


def oid(data: bytes) -> str:
    return hashlib.sha1(
        b"blob " + str(len(data)).encode("ascii") + b"\0" + data
    ).hexdigest()


def text(path: str) -> str:
    return git_bytes(APP + path).decode("utf-8")


class AppServerRpcRoutingIntegrationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.processor = text("message_processor.rs")
        cls.request = text("request_processors.rs")
        cls.turn = text("request_processors/turn_processor.rs")
        cls.readstate = text("request_processors/thread_read_state.rs")

    def test_all_five_sources_equal_exact_reviewed_git_blobs(self):
        for path, sha in FILES.items():
            with self.subTest(path=path):
                self.assertEqual(oid(git_bytes(APP + path)), sha)

    def test_workflow_state_rpc_routes_to_persistence_handler_once(self):
        self.assertEqual(
            self.processor.count(
                "ClientRequest::ThreadWorkflowStateUpdate { params, .. }"
            ),
            1,
        )
        self.assertIn(".thread_workflow_state_update(params)", self.processor)

    def test_workflow_rpc_remains_separate_from_user_turn_steering(self):
        workflow = self.processor.index(
            "ClientRequest::ThreadWorkflowStateUpdate { params, .. }"
        )
        steering = self.processor.index("ClientRequest::TurnSteer { params, .. }")
        self.assertLess(workflow, steering)

    def test_upstream_read_state_route_retained(self):
        self.assertIn(
            "ClientRequest::ThreadReadStateUpdate { params, .. }", self.processor
        )
        self.assertIn(".thread_read_state_update(params)", self.processor)

    def test_upstream_thread_items_read_route_retained(self):
        self.assertIn("ClientRequest::ThreadItemsRead { params, .. }", self.processor)
        self.assertIn(".thread_items_read(params)", self.processor)

    def test_official_unload_map_and_workspace_test_module_survive(self):
        self.assertIn("HashMap::new()", self.processor)
        self.assertIn("message_processor_workspace_routing_tests.rs", self.processor)

    def test_protocol_workflow_types_are_imported_once_each(self):
        for name in (
            "ThreadWorkflowStateOperation",
            "ThreadWorkflowStateUpdateParams",
            "ThreadWorkflowStateUpdateResponse",
        ):
            self.assertEqual(
                self.request.count("use codex_app_server_protocol::" + name + ";"), 1
            )

    def test_upstream_read_state_module_and_thread_items_import_retained(self):
        self.assertIn("mod thread_read_state;", self.request)
        self.assertIn(
            "use codex_app_server_protocol::ThreadItemsReadParams;", self.request
        )
        self.assertIn("use crate::thread_state::PendingThreadUnloads;", self.request)

    def test_workflow_state_persists_through_loaded_thread_not_model_items(self):
        i = self.turn.index("pub(crate) async fn thread_workflow_state_update(")
        j = self.turn.index("pub(crate) async fn thread_settings_update(", i)
        body = self.turn[i:j]
        self.assertIn("self.load_thread(&params.thread_id).await?", body)
        self.assertIn(".persist_workflow_state(item)", body)
        self.assertNotIn("thread_inject_items(", body)
        self.assertNotIn("TurnInput::UserInput", body)

    def test_workflow_terminal_ready_and_clear_operations_preserved(self):
        self.assertIn("ThreadWorkflowStateOperation::SetReadyForOwnerQa", self.turn)
        self.assertIn("ThreadWorkflowStateOperation::Clear", self.turn)
        self.assertIn(
            "codex_rollout::WorkflowStateItem::set_ready_for_owner_qa",
            self.turn,
        )

    def test_missing_adaptive_state_fails_explicitly(self):
        self.assertIn("setAdaptiveState requires adaptiveState", self.turn)
        self.assertIn("return Err(invalid_request(", self.turn)

    def test_adaptive_validation_receipt_and_owner_scope_mapping_retained(self):
        for field in (
            "validation_repair_cycles_used",
            "validation_targeted_retest_required",
            "failure_fingerprint",
            "worker_assignment_locked",
            "authorized_scope",
            "workflow_terminal",
        ):
            with self.subTest(field=field):
                self.assertIn(field, self.turn)

    def test_upstream_retained_output_maps_to_annotated_response_items(self):
        for marker in (
            "if tool_output.retain {",
            "TurnInput::AnnotatedResponseItem(",
            "ResponseItemAnnotations { retain: true }",
            "TurnInput::ResponseItem(item)",
        ):
            self.assertIn(marker, self.turn)

    def test_new_official_read_state_handler_is_not_rewritten(self):
        self.assertIn(
            "pub(crate) async fn thread_read_state_update(",
            self.readstate,
        )
        self.assertIn("ReadStateOperation", self.readstate)
        self.assertIn("ThreadReadStateChangedNotification", self.readstate)

    def test_upstream_workspace_routing_tests_are_present(self):
        rust = text("message_processor_workspace_routing_tests.rs")
        self.assertIn("use super::*;", rust)
        self.assertIn("TEST_CONNECTION_ID", rust)
        self.assertIn("fn ", rust)

    def test_product_markers_and_native_test_gate_unchanged(self):
        self.assertEqual(git_bytes("codex-rs/codexdd-version.txt").strip(), b"0.4.4")
        self.assertEqual(
            git_bytes("codex-rs/upstream-codex-release.txt").strip(),
            b"rust-v0.162.0",
        )
        doc = (ROOT / "docs/codexdd-0.4.5-packet-3b4-rpc-routing.md").read_text(
            encoding="utf-8"
        )
        self.assertIn("INCOMPLETE / NOT A RELEASE CANDIDATE", doc)
        self.assertIn("native Rust", doc)
        self.assertIn("109", doc)


if __name__ == "__main__":
    unittest.main()
