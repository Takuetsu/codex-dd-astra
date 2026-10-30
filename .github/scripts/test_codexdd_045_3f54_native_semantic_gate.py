"""3F.54: native test-only fixes remain stack safe and source-bound (nonrelease)."""
from __future__ import annotations
import json
import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
R = ROOT / "codex-rs"
MATRIX = ROOT / "docs/upstream-candidates/codexdd-045-109-semantic-owner-matrix.json"
RUNNER = ROOT / "scripts/codexdd-045-3f54-targeted-semantic.ps1"
TUI = R / "tui/src/app/tests/session_lifecycle_requests.rs"
APP_SERVER = R / "app-server/src/bespoke_event_handling.rs"

class NativeSemanticGate(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.matrix = json.loads(MATRIX.read_text(encoding="utf-8"))
        cls.rows = {r["path"]: r for r in cls.matrix["paths"]}
        cls.tui = TUI.read_text(encoding="utf-8")
        cls.app_server = APP_SERVER.read_text(encoding="utf-8")
        cls.runner = RUNNER.read_text(encoding="utf-8")

    def test_only_source_fingerprint_changes_and_release_gate_remains_closed(self):
        self.assertEqual(len(self.rows), 109)
        self.assertEqual(self.matrix["release_status"], "NOT_ACCEPTED")
        self.assertTrue(all(r["semantic_status"] == "PENDING_OWNER_REVIEW" and r["release_status"] == "NOT_ACCEPTED" for r in self.rows.values()))
        self.assertEqual(self.rows["codex-rs/tui/src/app/tests/session_lifecycle_requests.rs"]["current_blob"], "7d2be3352380d43df714399b441a95960b87a59f")
        self.assertEqual(self.rows["codex-rs/app-server/src/bespoke_event_handling.rs"]["current_blob"], "693eac395673e062d27bbf2cf7b467e9f6af9a5a")
        self.assertEqual((R / "codexdd-version.txt").read_text().strip(), "0.4.4")
        self.assertEqual((R / "upstream-codex-release.txt").read_text().strip(), "rust-v0.162.0")

    def test_mcp_daemon_restores_upstream_boxed_wrapper(self):
        fn = "local_daemon_registers_approval_gated_mcp_tools_for_both_start_paths"
        self.assertEqual(len(re.findall(r"async fn " + fn + r"\(", self.tui)), 1)
        body = self.tui.split("async fn " + fn + "()", 1)[1]
        self.assertIn("boxed_future!(" + fn + "_scenario())", body)
        self.assertIn("async fn " + fn + "_scenario()", body)
        self.assertIn("app_server.start_dynamic_tool_mcp(", body)
        self.assertIn("!tools[\"create_thread\"]", body)
        self.assertIn("tool_server.suspend();", body)
        self.assertIn("tool_server.reconnect(", body)

    def test_failed_and_interrupted_turn_native_regression_never_replays(self):
        marker = "async fn adaptive_runtime_signal_does_not_replay_on_failed_or_interrupted_turn()"
        self.assertIn(marker, self.app_server)
        case = self.app_server.split(marker, 1)[1].split("\n    #[tokio::test]", 1)[0]
        self.assertIn("for interrupted in [false, true]", case)
        self.assertIn("ServerNotification::AdaptiveRuntimeSignal(_)", case)
        self.assertIn("handle_turn_interrupted(", case)
        self.assertIn("handle_turn_complete(", case)
        self.assertIn("TurnStatus::Interrupted", case)
        self.assertIn("TurnStatus::Failed", case)
        self.assertIn("rx.try_recv().is_err()", case)
        self.assertIn("adaptive_runtime_signal_binds_event_turn_and_conversation_identity", self.app_server)

    def test_runner_is_daniel_cl_only_exact_source_and_nonrelease(self):
        for token in (
            "[ValidateSet('Preflight', 'Native')]",
            "$Mode = 'Preflight'",
            "$env:COMPUTERNAME -ine 'Daniel-CL'",
            "Native mode requires -ExpectedCommit",
            "Tracked source",
            "codexdd-045-owner-preflight.ps1",
            "test_codexdd_045_3f51_exact_upstream_semantic_deltas.py",
            "test_codexdd_045_3f52_appserver_owner_source.py",
            "cargo-format",
            "'--no-fail-fast'",
            "'--lib'",
            "RUST_MIN_STACK",
            "CARGO_INCREMENTAL",
            "release_approved = $false",
            "CODEXDD_045_3F54_RECEIPT_JSON ",
            "TOTAL_TIME_WORKED_SECONDS:",
        ):
            with self.subTest(token=token):
                if token == "Tracked source":
                    self.assertIn("Source worktree contains tracked modifications", self.runner)
                else:
                    self.assertIn(token, self.runner)
        self.assertEqual(self.runner.count("@('core-"), 3)
        self.assertEqual(self.runner.count("@('tui-"), 4)
        self.assertEqual(self.runner.count("@('app-server-"), 2)
        for selector in (
            "normalize_adds_missing_output_for_custom_tool_call",
            "normalize_adds_missing_output_for_local_shell_call_with_id",
            "normalize_removes_orphan_",
            "adaptive_runtime_signal",
            "local_daemon_registers_approval_gated_mcp_tools_for_both_start_paths",
            "older_external_server_starts_without_unsupported_dynamic_tools_or_history",
        ):
            self.assertIn(selector, self.runner)
        self.assertNotIn("git push", self.runner)
        self.assertNotIn("gh release", self.runner)
        self.assertNotIn("cargo build --release", self.runner)
        self.assertNotIn("codexdd-test-release.ps1", self.runner)

if __name__ == "__main__":
    unittest.main()
