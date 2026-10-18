"""0.4.5 packet 3C.2: static provenance and policy for dynamic tool lifecycle.

Offline Git object checks only; not a native build or validation receipt.
"""

from __future__ import annotations

import hashlib
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DYNAMIC = "codex-rs/core/src/tools/handlers/dynamic.rs"
MOD = "codex-rs/core/src/tools/mod.rs"
RACE_TEST = "codex-rs/core/src/tools/handlers/dynamic_tests.rs"
EXPECTED = {
    DYNAMIC: "05353ec9dc97df37db5c85ed1b232d7c9eb5e6d1",
    MOD: "2a05e28de35348b79e02d80e4bdd8c3381b88279",
    RACE_TEST: "2d926b321d0347ebc051c5604244e21dee390cb3",
}


def git_bytes(path: str) -> bytes:
    return subprocess.run(
        ["git", "show", "HEAD:" + path], cwd=ROOT,
        check=True, capture_output=True,
    ).stdout


def source(path: str) -> str:
    return git_bytes(path).decode("utf-8")


def blob_oid(payload: bytes) -> str:
    return hashlib.sha1(
        b"blob " + str(len(payload)).encode("ascii") + b"\0" + payload
    ).hexdigest()


class CoreToolLifecycleIntegrationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.dynamic = source(DYNAMIC)
        cls.modes = source(MOD)
        cls.races = source(RACE_TEST)

    def test_dynamic_tool_source_blob_is_exact_reviewed_merge(self):
        self.assertEqual(blob_oid(git_bytes(DYNAMIC)), EXPECTED[DYNAMIC])

    def test_tool_mode_source_blob_is_exact_reviewed_merge(self):
        self.assertEqual(blob_oid(git_bytes(MOD)), EXPECTED[MOD])

    def test_upstream_cancellation_race_fixture_is_unmodified(self):
        self.assertEqual(blob_oid(git_bytes(RACE_TEST)), EXPECTED[RACE_TEST])

    def test_codexdd_dynamic_tool_evidence_remains_attached_to_call_id(self):
        self.assertIn("call_id.clone()", self.dynamic)
        self.assertIn(".with_evidence_id(call_id)", self.dynamic)
        self.assertIn(
            "FunctionToolOutput::from_content(body, Some(success))",
            self.dynamic,
        )

    def test_upstream_cancellation_is_forwarded_into_dynamic_tool_request(self):
        self.assertIn("cancellation_token,", self.dynamic)
        self.assertIn("use tokio_util::sync::CancellationToken;", self.dynamic)
        self.assertIn("use codex_async_utils::OrCancelExt;", self.dynamic)
        self.assertIn(".or_cancel(&cancellation_token)", self.dynamic)

    def test_upstream_dynamic_tool_finishes_on_cancellation(self):
        self.assertIn(
            "fn finishes_on_cancellation(&self) -> bool {", self.dynamic
        )
        self.assertIn("        true\n    }", self.dynamic)
        self.assertIn("if cancellation_token.is_cancelled()", self.dynamic)

    def test_upstream_cancellation_race_asserts_exactly_one_completion(self):
        for marker in (
            "async fn response_cancel_race_completes_once()",
            "DynamicToolCallStatus::Completed",
            "DynamicToolCallStatus::Failed",
            "receiver.try_recv()",
        ):
            self.assertIn(marker, self.races)

    def test_fork_direct_mode_restrictions_survive_1630_tool_update(self):
        for trigger in (
            "CODEXDD_ADAPTIVE_RECONNAISSANCE_TURN_TRIGGER",
            "CODEXDD_ADAPTIVE_IMPLEMENTATION_TURN_TRIGGER",
            "CODEXDD_ADAPTIVE_MECHANICAL_VALIDATION_TURN_TRIGGER",
        ):
            self.assertIn(trigger, self.modes)
        self.assertIn("return ToolMode::Direct;", self.modes)

    def test_removed_upstream_shell_type_helper_is_not_reintroduced(self):
        self.assertNotIn(
            "pub(crate) fn tool_user_shell_type(", self.modes
        )
        self.assertIn("pub(crate) fn effective_tool_mode(", self.modes)

    def test_large_fork_registry_is_untouched_until_later_semantic_review(self):
        self.assertEqual(
            blob_oid(git_bytes("codex-rs/core/src/tools/registry.rs")),
            "5b67eb29406d04b021b0fd9a5e259eccdd7da3b2",
        )

    def test_incomplete_integration_release_gates_remain_closed(self):
        self.assertEqual(git_bytes("codex-rs/codexdd-version.txt").strip(), b"0.4.4")
        self.assertEqual(
            git_bytes("codex-rs/upstream-codex-release.txt").strip(),
            b"rust-v0.162.0",
        )
        doc = (
            ROOT / "docs/codexdd-0.4.5-packet-3c2-tool-lifecycle.md"
        ).read_text(encoding="utf-8")
        self.assertIn("INCOMPLETE / NOT A RELEASE CANDIDATE", doc)
        self.assertIn("native Rust", doc)
        self.assertIn("registry.rs", doc)


if __name__ == "__main__":
    unittest.main()
