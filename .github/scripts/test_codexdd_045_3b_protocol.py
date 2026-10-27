"""CodexDD 0.4.5 / packet 3B.1: read-only upstream protocol provenance checks.

Static invariants only. Do NOT interpret green tests as Rust compilation,
serialization integration, regenerated schemas, or a native Windows release.
"""

from __future__ import annotations

import hashlib
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PROTOCOL = "codex-rs/protocol/src/protocol.rs"
INTEGRATED_PROTOCOL_BLOB = "9ff46a4e1331e15f35f95b8e17ba41dfc931a5cf"
OFFICIAL_PROTOCOL_BLOB = "146209407b78b71c461c854c5d9d1e4d7816b873"
UNMODIFIED_UPSTREAM_BLOBS = {
    "codex-rs/protocol/src/approvals.rs": "09b7115824932453ab6891dbba33d4173e68d684",
    "codex-rs/protocol/src/items.rs": "709890809d58bdd81339683e8334764135094d4e",
    "codex-rs/protocol/src/models.rs": "b2ce4a10a60553fa195d3eb92768c4759eae0636",
    "codex-rs/protocol/src/models/executed_tool_calls.rs": "7b2fe4c2d57d809abe6e794164e1f92914b7f36f",
    "codex-rs/protocol/src/models/executed_tool_calls_tests.rs": "7ce715997496aa04b748725690cf41f9de72f0ce",
    "codex-rs/protocol/src/models/item_metadata_tests.rs": "ad8d66d73eb12368160208c9c7de51508d1db756",
    "codex-rs/protocol/src/openai_models.rs": "45e3d5c0963048f197afcc4e5cdff3770df54a9b",
    "codex-rs/protocol/src/turn_input.rs": "7e9977959c37db4187317859f9742e7f8be49874",
    "codex-rs/protocol/src/conversation_start_params_tests.rs": "6ccb5e4f1e3c956e6439442aa313c7be16d92e80",
    "codex-rs/protocol/src/openai_models/incremental_tools.rs": "f051318136ae1a2f7da7bff02d8d0bc570ab5d05",
}
FORK_PERMISSION_BLOB = "140090f5ef1649ed9e4717643dc85773be93da5f"


def committed(path: str) -> bytes:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT,
        check=True,
        capture_output=True,
    ).stdout


def contents(path: str) -> str:
    return committed(path).decode("utf-8")


def blob_sha(data: bytes) -> str:
    return hashlib.sha1(
        b"blob " + str(len(data)).encode("ascii") + b"\0" + data
    ).hexdigest()


class ProtocolStagingChecks(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.protocol = contents(PROTOCOL)

    def test_all_ten_upstream_protocol_blobs_match_official_pinned_commit(self):
        for path, expected in UNMODIFIED_UPSTREAM_BLOBS.items():
            with self.subTest(path=path):
                self.assertEqual(blob_sha(committed(path)), expected)

    def test_protocol_entrypoint_reconciliation_is_exact_git_blob(self):
        actual = blob_sha(committed(PROTOCOL))
        self.assertEqual(actual, INTEGRATED_PROTOCOL_BLOB)
        self.assertNotEqual(actual, OFFICIAL_PROTOCOL_BLOB)

    def test_adaptive_runtime_signal_type_and_variant_survive(self):
        for term in (
            "pub enum AdaptiveRuntimeSignalKind {",
            "pub struct AdaptiveRuntimeSignalEvent {",
            "AdaptiveRuntimeSignal(AdaptiveRuntimeSignalEvent)",
            "Private, inert adaptive runtime transport data.",
        ):
            with self.subTest(term=term):
                self.assertIn(term, self.protocol)
        self.assertEqual(
            self.protocol.count("AdaptiveRuntimeSignal(AdaptiveRuntimeSignalEvent)"),
            1,
        )

    def test_adaptive_signal_source_identity_stays_session_owned(self):
        start = self.protocol.index("pub struct AdaptiveRuntimeSignalEvent {")
        end = self.protocol.index("\n}", start)
        payload = self.protocol[start:end]
        self.assertIn("pub evidence_refs: Vec<String>", payload)
        self.assertIn("pub signal_kind: AdaptiveRuntimeSignalKind", payload)
        self.assertNotIn("thread_id", payload)
        self.assertNotIn("turn_id", payload)

    def test_all_original_adaptive_signal_kinds_are_preserved(self):
        start = self.protocol.index("pub enum AdaptiveRuntimeSignalKind {")
        end = self.protocol.index("\n}", start)
        block = self.protocol[start:end]
        for kind in (
            "Complexity",
            "Capability",
            "MechanicalValidation",
            "ImplementationWork",
            "ReadyForValidation",
            "RepairRequired",
            "ReadyForOwnerQa",
        ):
            self.assertIn(kind + ",", block)

    def test_upstream_conversation_debug_redacts_sensitive_content(self):
        self.assertIn(
            "Text, instructions, and transport endpoints can contain credentials.",
            self.protocol,
        )
        self.assertIn('f.debug_struct("ConversationStartParams")', self.protocol)
        self.assertIn(".finish_non_exhaustive()", self.protocol)

    def test_official_elicitation_abandonment_event_and_definition_survive(self):
        self.assertIn(
            "ElicitationAbandoned(ElicitationAbandonedEvent)",
            self.protocol,
        )
        self.assertIn(
            "pub struct ElicitationAbandonedEvent {",
            contents("codex-rs/protocol/src/approvals.rs"),
        )

    def test_upstream_encrypted_compaction_budgeting_survives(self):
        self.assertIn(
            "Estimate the model-visible content length of an encrypted compaction.",
            self.protocol,
        )
        self.assertIn("replay caches", self.protocol)

    def test_upstream_encrypted_model_fields_are_staged(self):
        model = contents("codex-rs/protocol/src/models.rs")
        self.assertIn("pub enum ResponseItem {", model)
        self.assertIn("encrypted_content: Option<String>", model)
        self.assertIn("annotations: Option<Vec<serde_json::Value>>", model)
        self.assertIn("logprobs: Option<Vec<serde_json::Value>>", model)

    def test_incremental_tool_module_and_new_protocol_tests_are_present(self):
        self.assertIn(
            "mod incremental_tools;",
            contents("codex-rs/protocol/src/openai_models.rs"),
        )
        self.assertTrue(
            contents("codex-rs/protocol/src/openai_models/incremental_tools.rs").strip()
        )
        self.assertTrue(
            contents("codex-rs/protocol/src/conversation_start_params_tests.rs").strip()
        )
        self.assertIn(
            "conversation_start_params_tests;",
            self.protocol,
        )

    def test_reviewed_0163_permissions_keep_deny_and_protected_metadata_policy(self):
        # 3B.1 intentionally did not touch protected permissions; 3C.9b later
        # reviewed and staged the official 0.163 source. Assert its exact blob
        # and the security primitives instead of pinning an obsolete HEAD.
        self.assertEqual(
            blob_sha(committed("codex-rs/protocol/src/permissions.rs")),
            "bd6757543a56063ecbced6dd49e52241dffb4af8",
        )
        permissions = contents("codex-rs/protocol/src/permissions.rs")
        for marker in (
            "ReadDenyMatcher",
            "FileSystemSandboxPolicy",
            "forbidden_agent_metadata_write",
        ):
            self.assertIn(marker, permissions)
        production = subprocess.run(
            [
                "git",
                "rev-parse",
                "b25550dcb2c6e84a84074b603086c9db6f32a63b:codex-rs/protocol/src/permissions.rs",
            ],
            cwd=ROOT,
            check=True,
            capture_output=True,
            text=True,
        ).stdout.strip()
        self.assertEqual(production, FORK_PERMISSION_BLOB)

    def test_release_markers_and_pending_native_gate_are_unmodified(self):
        self.assertEqual(committed("codex-rs/codexdd-version.txt").strip(), b"0.4.4")
        self.assertEqual(
            committed("codex-rs/upstream-codex-release.txt").strip(),
            b"rust-v0.162.0",
        )
        notes = (ROOT / "docs/codexdd-0.4.5-packet-3b1-protocol-models.md").read_text(
            encoding="utf-8"
        )
        self.assertIn("INCOMPLETE / NOT A RELEASE CANDIDATE", notes)
        self.assertIn("schema regeneration", notes)
        self.assertIn("native Rust", notes)


if __name__ == "__main__":
    unittest.main()
