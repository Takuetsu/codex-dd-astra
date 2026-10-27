"""CodexDD 0.4.5 / 3B.2: offline app-server protocol source checks.

Check exact committed Git provenance and fork-vs-upstream message contracts.
No network, runtime codegen, native Rust compile or publication occurs.
"""

from __future__ import annotations

import hashlib
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FORK = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
BASE = "codex-rs/app-server-protocol/src/"
PATHS = {
    "protocol/common.rs": "6cb1a33ea48ed0de73f420ff6c179d9629fec45a",
    "protocol/thread_history.rs": "82c980c427501b594389fed439e105c2f696b9a5",
    "protocol/v2/thread.rs": "4180b91565e1a47b38c4f40114716e55b2973d3f",
    "protocol/v2/turn.rs": "04f38d2535ba46da2bc22358589aeb1bd2840499",
    "export.rs": "0f90faf920cfcf6565c55548fc4064ba7dcccd96",
    "protocol/v2/environment.rs": "3794780c18b67113374dcf0c9148fdb6e903b6f1",
}


def git_bytes(ref: str, path: str) -> bytes:
    return subprocess.run(
        ["git", "show", ref + ":" + path],
        cwd=ROOT,
        check=True,
        capture_output=True,
    ).stdout


def source(short: str, ref: str = "HEAD") -> str:
    return git_bytes(ref, BASE + short).decode("utf-8")


def oid(raw: bytes) -> str:
    return hashlib.sha1(
        b"blob " + str(len(raw)).encode("ascii") + b"\0" + raw
    ).hexdigest()


class AppServerProtocolIntegrationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.common = source("protocol/common.rs")
        cls.history = source("protocol/thread_history.rs")
        cls.thread = source("protocol/v2/thread.rs")
        cls.turn = source("protocol/v2/turn.rs")

    def test_six_exact_reviewed_git_blob_objects(self):
        for short, expected in PATHS.items():
            with self.subTest(short=short):
                self.assertEqual(oid(git_bytes("HEAD", BASE + short)), expected)

    def test_fork_workflow_mutation_rpc_keeps_thread_scoped_serialization(self):
        needle = 'ThreadWorkflowStateUpdate => "thread/workflowState/update"'
        self.assertEqual(self.common.count(needle), 1)
        beginning = self.common.index(needle)
        body = self.common[beginning : self.common.index("\n    },", beginning)]
        self.assertIn("v2::ThreadWorkflowStateUpdateParams", body)
        self.assertIn("serialization: thread_id(params.thread_id)", body)
        self.assertIn("v2::ThreadWorkflowStateUpdateResponse", body)

    def test_fork_adaptive_signal_notification_retained_once(self):
        marker = 'AdaptiveRuntimeSignal => "turn/adaptiveRuntimeSignal"'
        self.assertEqual(self.common.count(marker), 1)
        self.assertIn('experimental("turn/adaptiveRuntimeSignal")', self.common)
        self.assertIn("v2::AdaptiveRuntimeSignalNotification", self.common)

    def test_new_upstream_thread_read_state_and_item_read_rpcs_preserved(self):
        for value in (
            'ThreadReadStateUpdate => "thread/readState/update"',
            'ThreadItemsRead => "thread/items/read"',
            'ThreadReadStateChanged => "thread/readState/changed"',
        ):
            with self.subTest(api=value):
                self.assertIn(value, self.common)

    def test_saved_workflow_rollout_is_skipped_for_model_visible_history(self):
        self.assertEqual(self.history.count("| RolloutItem::WorkflowState(_)"), 1)
        self.assertIn("| RolloutItem::SessionMeta(_) => {}", self.history)

    def test_upstream_encrypted_and_status_history_model_fields_retained(self):
        self.assertIn("status: None,", self.history)
        self.assertIn("encrypted_content: None,", self.history)

    def test_fork_adaptive_signal_payload_remains_session_bound(self):
        self.assertIn("pub struct AdaptiveRuntimeSignalEnvelope", self.turn)
        self.assertIn("pub struct AdaptiveRuntimeSignalNotification", self.turn)
        self.assertIn("pub source_turn_id: String", self.turn)
        self.assertIn(
            "pub signal_kind: codex_protocol::protocol::AdaptiveRuntimeSignalKind",
            self.turn,
        )
        self.assertIn("pub thread_id: String", self.turn)

    def test_upstream_turn_output_retention_field_survives(self):
        self.assertIn("pub retain: bool,", self.turn)
        self.assertIn('skip_serializing_if = "std::ops::Not::not"', self.turn)

    def test_workflow_state_structs_and_terminal_operations_preserved(self):
        for marker in (
            "pub enum ThreadWorkflowStateOperation",
            "SetReadyForOwnerQa,",
            "SetAdaptiveState,",
            "pub struct ThreadAdaptiveWorkflowState",
            "pub struct ThreadAdaptiveValidationStatus",
            "pub struct ThreadWorkflowStateUpdateParams",
            "pub struct ThreadWorkflowStateUpdateResponse",
            "pub worker_assignment_locked: bool",
        ):
            self.assertIn(marker, self.thread)

    def test_workflow_state_fields_use_serde_camel_case(self):
        section_start = self.thread.index("pub enum ThreadWorkflowStateOperation")
        self.assertIn(
            '#[serde(rename_all = "camelCase")]',
            self.thread[section_start - 150 : section_start],
        )
        self.assertIn('#[ts(export_to = "v2/")]', self.thread)

    def test_upstream_prediction_read_state_and_item_reads_retained(self):
        for name in (
            "pub experimental_prediction_mode: bool",
            "pub excluded_thread_ids: Option<Vec<String>>",
            "pub struct ThreadReadState",
            "pub struct ThreadItemsReadParams",
            "pub struct ThreadItemsReadResponse",
            "pub enum ThreadUnreadPosition",
        ):
            with self.subTest(name=name):
                self.assertIn(name, self.thread)

    def test_workflow_definitions_equal_original_fork_type_block(self):
        after = "pub struct ThreadInjectItemsResponse {}\n\n"
        before = "pub struct ThreadTurnsListParams"
        old = source("protocol/v2/thread.rs", FORK)
        old_start = old.index(after) + len(after)
        old_end = old.rfind("#[derive(", 0, old.index(before))
        retained = old[old_start:old_end]
        self.assertIn(retained, self.thread)

    def test_fork_adaptive_notification_definitions_match_original(self):
        old = source("protocol/v2/turn.rs", FORK)
        begin = old.rfind(
            "#[derive(", 0, old.index("pub struct AdaptiveRuntimeSignalEnvelope")
        )
        end = old.rfind(
            "#[derive(", 0, old.index("pub struct TurnDiffUpdatedNotification")
        )
        self.assertIn(old[begin:end], self.turn)

    def test_release_markers_and_native_validation_still_pending(self):
        self.assertEqual(
            git_bytes("HEAD", "codex-rs/codexdd-version.txt").strip(), b"0.4.4"
        )
        self.assertEqual(
            git_bytes("HEAD", "codex-rs/upstream-codex-release.txt").strip(),
            b"rust-v0.162.0",
        )
        doc = (ROOT / "docs/codexdd-0.4.5-packet-3b2-appserver-protocol.md").read_text(
            encoding="utf-8"
        )
        self.assertIn("INCOMPLETE / NOT A RELEASE CANDIDATE", doc)
        self.assertIn("native Rust", doc)
        self.assertIn("schema regeneration", doc)


if __name__ == "__main__":
    unittest.main()
