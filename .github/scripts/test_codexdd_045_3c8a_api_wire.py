"""CodexDD 0.4.5 3C.8a: pinned API, request and executor wire consumers.

Offline provenance/source guards; no network, Rust compilation, runtime
authorization proof, lockfile update, or release permission.
"""

from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
STAGED = {
    "codex-rs/app-server-client/src/lib.rs": [
        "df5dc3e99f75eb00c2f103257f86c1029cf15615",
        "75544173d1cac8ef44a84aade927c8e57056b89b",
    ],
    "codex-rs/app-server-client/src/remote.rs": [
        "6b5571bf03a2733d00a6e7181c87bfd274ba084e",
        "c5d9b0321f0574bce819c561fe32faf87f79c6ce",
    ],
    "codex-rs/app-server-client/src/server_request.rs": [
        "",
        "464b26b7f2aa52c2f337812a6316d8bb78bfffa9",
    ],
    "codex-rs/app-server-client/src/server_request_tests.rs": [
        "",
        "24e9dd1e48b8794a07eb83c5a660dcf80b8a28bb",
    ],
    "codex-rs/app-server-client/src/tests/reply_tests.rs": [
        "",
        "0fe5a0f7ac0a52d8faa50761edc37719e6adfde5",
    ],
    "codex-rs/app-server-test-client/src/lib.rs": [
        "4d3c6efa87d5646bfde5981bcc68358afac5f95e",
        "26aeed921fe965cad19aaba4a4ebe26491535fdd",
    ],
    "codex-rs/codex-api/src/api_bridge.rs": [
        "abf0bf0d7041d3ce2c5b3e4a5f8f5535e064e31b",
        "98d8d17ef621924c23e1f1cd9ecc2b20976424d1",
    ],
    "codex-rs/codex-api/src/common.rs": [
        "42c9fecd1a3065cf1729ce7b53f017fe1dbac53c",
        "1abc89c1b73f6d68619662745a7fda0d8192b227",
    ],
    "codex-rs/codex-api/src/endpoint/responses_websocket.rs": [
        "4e041ab0a57aaa002eb7da334d4880a5d1814292",
        "77ed1ea60dba3e8a79ee08472c31bbe263d61ccb",
    ],
    "codex-rs/codex-api/src/endpoint/search.rs": [
        "2de48fe7dbd99372bab1362ca707df85bead55b6",
        "502e264b4bedea1f13985330a23fd81f19863b07",
    ],
    "codex-rs/codex-api/src/error.rs": [
        "2834137129e73709ed4349bd2bcd2b3ec97d4e93",
        "6d98cd4cb22d1864cfd0c87509c0f6dd67212307",
    ],
    "codex-rs/codex-api/src/lib.rs": [
        "b7511d9e50f4b4eb02a32908be1ec3c88bf48df0",
        "3afbcc6ef1f1110e426e46c049d9569c25209bc8",
    ],
    "codex-rs/codex-api/src/sse/responses.rs": [
        "78885f43de273a876cee54c59dafbd404d1b6c31",
        "34d55118c8a9e4846ea19325c9a796fc37968efc",
    ],
    "codex-rs/codex-api/tests/clients.rs": [
        "b17da453440ad49cb8e495ea72f21081b99a4ea2",
        "a7400d306e1177eddc78037b3a5e919a02fe41c2",
    ],
    "codex-rs/exec-server-protocol/src/protocol.rs": [
        "62d901e0f41209424523e817166441ce4265f5f9",
        "26614350c343003cf9da3b5d2429ae807a382255",
    ],
}


def blob(ref: str, path: str) -> str | None:
    result = subprocess.run(
        ["git", "rev-parse", "--verify", ref + ":" + path],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip() if result.returncode == 0 else None


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT,
        capture_output=True,
        text=True,
        check=True,
        encoding="utf-8",
    ).stdout


class ApiWireConsumerSourceTests(unittest.TestCase):
    def assertIn(self, part, whole, msg=None):
        if isinstance(whole, str) and len(whole) > 2048:
            self.assertTrue(part in whole, msg or f"missing source marker: {part!r}")
            return
        super().assertIn(part, whole, msg)

    def test_fifteen_pinned_official_source_blobs(self):
        self.assertEqual(len(STAGED), 15)
        for path, (_, target) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(blob("HEAD", path), target)

    def test_no_original_fork_customization_was_overwritten(self):
        for path, (production, _) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(blob(PRODUCTION, path), production or None)

    def test_three_new_client_request_modules_absent_from_production(self):
        for path in (
            "codex-rs/app-server-client/src/server_request.rs",
            "codex-rs/app-server-client/src/server_request_tests.rs",
            "codex-rs/app-server-client/src/tests/reply_tests.rs",
        ):
            with self.subTest(path=path):
                self.assertIsNone(blob(PRODUCTION, path))
                self.assertIsNotNone(blob("HEAD", path))

    def test_app_server_reply_handle_and_client_requests(self):
        request = source("codex-rs/app-server-client/src/server_request.rs")
        client = source("codex-rs/app-server-client/src/lib.rs")
        self.assertIn("pub struct AppServerReplyHandle", request)
        self.assertIn("pub fn server_request_reply_handle", request)
        self.assertIn("resolve_server_request", client)
        self.assertIn("reject_server_request", client)

    def test_remote_client_resolves_and_rejects_server_requests(self):
        remote = source("codex-rs/app-server-client/src/remote.rs")
        self.assertIn("resolve_server_request", remote)
        self.assertIn("reject_server_request", remote)

    def test_codex_api_search_and_websocket_contract_staged(self):
        search = source("codex-rs/codex-api/src/endpoint/search.rs")
        ws = source("codex-rs/codex-api/src/endpoint/responses_websocket.rs")
        self.assertIn("pub struct SearchClient", search)
        self.assertIn("pub async fn search", search)
        self.assertIn("probe_handshake", ws)
        self.assertIn("merge_request_headers", ws)

    def test_executor_wire_transport_retains_sandbox_shapes(self):
        wire = source("codex-rs/exec-server-protocol/src/protocol.rs")
        for symbol in (
            "pub struct ExecParams",
            "pub struct ShellSnapshotRequest",
            "sandbox_contexts",
        ):
            self.assertIn(symbol, wire)

    def test_app_server_test_client_can_exercise_responses(self):
        tests = source("codex-rs/app-server-test-client/src/lib.rs")
        self.assertIn("send_message_v2_endpoint", tests)
        self.assertIn("thread_resume_follow", tests)

    def test_codexdd_registry_authorization_source_unchanged(self):
        self.assertEqual(
            blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )

    def test_product_and_upstream_markers_not_advanced(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(
            source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0"
        )
        self.assertEqual(
            blob("HEAD", "codex-rs/Cargo.lock"),
            "13bc15ff844e93dfaf7777c73bb7313b35bb3b81",
        )

    def test_release_remains_unapproved(self):
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c8a-api-wire.md").read_text(
            encoding="utf-8"
        )
        for marker in (
            "INCOMPLETE / NOT A RELEASE CANDIDATE",
            "Cargo.lock",
            "native Rust",
            "source-only",
        ):
            self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
