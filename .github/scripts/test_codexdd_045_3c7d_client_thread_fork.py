"""CodexDD 0.4.5 3C.7d: client/thread fork-source preservation.
Offline source-only checks, not native Rust, runtime policy, or release proof.
"""

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PROD = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
MERGED = {
    "codex-rs/core/src/client.rs": [
        "a08e2386475b0c4e87d0d79aaccc27efeaff641d",
        "583447f35eaba564771771813bd4d897cb6b73de",
    ],
    "codex-rs/core/src/client_tests.rs": [
        "ebfeb73694a5d22b4a030ca7d89e2d370511924e",
        "2e5525612379a29af24116dd738a6612dc4420e6",
    ],
    "codex-rs/core/src/codex_thread.rs": [
        "24fc487a32cd930f39a3c8137c40790d37e60713",
        "84b32544000fde7d16a4b4cd4f4b27f26d7ceaf9",
    ],
    "codex-rs/core/src/thread_manager.rs": [
        "ff796b8c877f3e671252d6db885ac968b08e5893",
        "88fa659a3c45839d9a3dce6b6a39e6e47795f103",
    ],
    "codex-rs/core/src/thread_manager_tests.rs": [
        "fe36ae0ae22feca5118d52d0dbee762b06b00c6d",
        "d7b27ac523a2f6c3d2e8a1beabc9189b17622e25",
    ],
}


def blob(ref, path):
    return subprocess.run(
        ["git", "rev-parse", ref + ":" + path],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()


def src(path):
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
        encoding="utf-8",
    ).stdout


class ClientThreadMergeTests(unittest.TestCase):
    def test_five_original_and_reviewed_merged_git_blobs(self):
        self.assertEqual(len(MERGED), 5)
        for path, (original, merged) in MERGED.items():
            with self.subTest(path=path):
                self.assertEqual(blob(PROD, path), original)
                self.assertEqual(blob("HEAD", path), merged)

    def test_terminalization_requires_tool_until_signal_called(self):
        client = src("codex-rs/core/src/client.rs")
        for x in (
            "CODEXDD_ADAPTIVE_VALIDATION_TERMINALIZATION_TURN_TRIGGER",
            "terminalization_signal_already_called",
            '"required"',
            "tool_choice: tool_choice.to_string()",
        ):
            self.assertTrue(x in client, x)

    def test_terminalization_is_bound_to_trusted_tool_name(self):
        client = src("codex-rs/core/src/client.rs")
        self.assertTrue('name == "report_adaptive_signal"' in client)

    def test_terminalization_native_regression_preserved(self):
        tests = src("codex-rs/core/src/client_tests.rs")
        self.assertEqual(
            tests.count(
                "fn validation_terminalization_requests_required_tool_choice()"
            ),
            1,
        )

    def test_thread_persists_and_flushes_workflow_items(self):
        thread = src("codex-rs/core/src/codex_thread.rs")
        self.assertTrue("pub async fn persist_workflow_state(" in thread)
        self.assertTrue("self.flush_rollout().await" in thread)

    def test_fork_inherits_workflow_state_snapshot(self):
        manager = src("codex-rs/core/src/thread_manager.rs")
        for x in (
            "workflow_state_from_history",
            "append_child_workflow_state_snapshot",
            "RestoredWorkflowState::Adaptive",
        ):
            self.assertTrue(x in manager, x)

    def test_native_child_snapshot_regression_present(self):
        tests = src("codex-rs/core/src/thread_manager_tests.rs")
        self.assertEqual(
            tests.count("fn child_workflow_snapshot_preserves_adaptive_state()"), 1
        )

    def test_session_replay_and_adaptive_dispatch_untouched(self):
        self.assertEqual(
            src("codex-rs/core/src/session/rollout_reconstruction.rs").count(
                "RolloutItem::WorkflowState(_)"
            ),
            4,
        )
        self.assertEqual(
            blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )

    def test_product_version_upstream_marker_and_lockfile_unadvanced(self):
        self.assertEqual(src("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(
            src("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0"
        )
        self.assertEqual(
            blob("HEAD", "codex-rs/Cargo.lock"),
            "a6cd27b0b70fde2f2be9c1777f3666ad3fa318bb",
        )

    def test_packet_not_release_candidate(self):
        s = (ROOT / "docs/codexdd-0.4.5-packet-3c7d-client-thread-fork.md").read_text(
            encoding="utf-8"
        )
        for x in (
            "INCOMPLETE / NOT A RELEASE CANDIDATE",
            "source-only",
            "native Rust",
            "Cargo.lock",
        ):
            self.assertTrue(x in s, x)


if __name__ == "__main__":
    unittest.main()
