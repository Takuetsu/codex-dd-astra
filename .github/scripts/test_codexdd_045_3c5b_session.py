"""CodexDD 0.4.5 packet 3C.5b: session/replay source provenance tripwires.

Static, offline only. No native Rust compile, event-stream replay receipt or
authorization to publish.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
STAGED = {
    "codex-rs/core/src/session/async_compaction.rs": [
        None,
        "bb80f857fd2f72eaac83f3a4ae91d4acd95ebf2f"
    ],
    "codex-rs/core/src/session/async_compaction_tests.rs": [
        None,
        "e4a048124b83f15ab2c5d23c33e39cfff556d70e"
    ],
    "codex-rs/core/src/session/config_refresh.rs": [
        "0fe59df6c67009236f59fc90bcdf12dfa25aa877",
        "166c1f604593bd83874be313559fea901daa268c"
    ],
    "codex-rs/core/src/session/config_refresh_tests.rs": [
        "72663109c3dbde13c7cbbb01488427c6929cc497",
        "927a2863095f77ead0b822b0bdfb16459a384410"
    ],
    "codex-rs/core/src/session/handlers.rs": [
        "c569b04ec95770768385fcafcaeee5f1aa2fc26e",
        "91bf8de9bf91ffd98895e976c9dc1b0e2d238968"
    ],
    "codex-rs/core/src/session/inject.rs": [
        "ea999374a39c488a7a619962e39d1702f00cb094",
        "a7ddaf75d3113f2fc12956cb48f18ae04ac4c713"
    ],
    "codex-rs/core/src/session/mcp.rs": [
        "b53b8445c03d2ee4d0e2d2324717785e718a2135",
        "d2599a4d5c00c5d240f25af1417c47cf710a6334"
    ],
    "codex-rs/core/src/session/mcp_tests.rs": [
        "28c099f20527a299e453894fb630d835a6a8d9e9",
        "47791f423347572eff29e911be328115a0edb080"
    ],
    "codex-rs/core/src/session/mod.rs": [
        "bc086f8661f32ca94cdac8fc161c8ff86f5aaf38",
        "a541c5529fea30aa0f7b9f32df049f22228d1d15"
    ],
    "codex-rs/core/src/session/notification_tests.rs": [
        "456970a7bb4feeeb8d6994b703265901d4e95ea4",
        "eb8dfed84b4c2dfce0a74a104dcafe5433a4ab4a"
    ],
    "codex-rs/core/src/session/review.rs": [
        "44ec95ff60bbb73e82ce62905721f8b1d6d94438",
        "e93643f1d598e577edb5550e9c17a51b5abc3de7"
    ],
    "codex-rs/core/src/session/tests/guardian_tests.rs": [
        "349376c5876240b2c6d483dfebe93344ebf5eb05",
        "ab9a2be02ec59790b9b06b9cfa9f623f95e9c276"
    ],
    "codex-rs/core/src/session/turn_context.rs": [
        "f74f5558ca87a10b9503822f208cfd43082f75c0",
        "40cf7657a6dbe96d2e5dc3bacbe3a0761b1e161e"
    ],
    "codex-rs/core/src/session/turn_input.rs": [
        "be2d43d6206a149d876cc913b5a193348ef338ad",
        "74a7592de8eaeabacad73bcf15cd404beff8ba1f"
    ],
    "codex-rs/core/src/session/turn_input_tests.rs": [
        "7fb4c7564cf970d981de9ffea8728c5db2a4a510",
        "ee5c64648391a670cfe2f52437e793126109dbb3"
    ],
    "codex-rs/core/src/session/turn_tests.rs": [
        "85d8fac5113807092d59395847155d9c9ca330c8",
        "c1554d79c0241e43f59cb02e9ca62f8811f66ddb"
    ],
    "codex-rs/core/src/session/world_state.rs": [
        "a050fccf1318e3917976772186310ae5cf54cd19",
        "37dda837fce0fb6889310105809d86bdfd759f3e"
    ]
}
MERGED = {
    "codex-rs/core/src/session/rollout_reconstruction.rs": "118907bac6f1f084c96b81e78c2d1b7fc923583d",
    "codex-rs/core/src/session/rollout_reconstruction_tests.rs": "106f07a29d8cde54784c516898472518736b9c37",
    "codex-rs/core/src/session/session.rs": "68fb4db5fe8f11638a22c3b249d9f1bc7668cadd",
    "codex-rs/core/src/session/tests.rs": "dc7d8179b298113fedcbcb5d5dc0573737a5c2e1",
    "codex-rs/core/src/session/turn.rs": "50f524a055a92f0d19be5e6bc2665fe18718e35b"
}


def git_blob(ref: str, path: str) -> str | None:
    result = subprocess.run(
        ["git", "rev-parse", ref + ":" + path],
        cwd=ROOT, capture_output=True, text=True,
    )
    return result.stdout.strip() if result.returncode == 0 else None


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT, check=True, capture_output=True, text=True, encoding="utf-8",
    ).stdout


class SessionReplayReconciliationTests(unittest.TestCase):
    def assertIn(self, needle, haystack, msg=None):
        if isinstance(haystack, str) and len(haystack) > 2048:
            self.assertTrue(needle in haystack, msg or f"missing source marker {needle!r}")
            return
        super().assertIn(needle, haystack, msg)

    def test_seventeen_exact_upstream_sources(self):
        self.assertEqual(len(STAGED), 17)
        for path, (_, upstream) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob("HEAD", path), upstream)

    def test_original_fork_not_overwritten_by_upstream_stage(self):
        for path, (original, _) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob(PRODUCTION, path), original)

    def test_two_new_session_modules_absent_from_original_fork(self):
        for path in (
            "codex-rs/core/src/session/async_compaction.rs",
            "codex-rs/core/src/session/async_compaction_tests.rs",
        ):
            self.assertIsNone(git_blob(PRODUCTION, path))
            self.assertIsNotNone(git_blob("HEAD", path))

    def test_five_three_way_merged_source_blobs_pinned(self):
        self.assertEqual(len(MERGED), 5)
        for path, expected in MERGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob("HEAD", path), expected)

    def test_four_workflow_state_reconstruction_arms_preserved(self):
        reconstruction = source("codex-rs/core/src/session/rollout_reconstruction.rs")
        self.assertEqual(reconstruction.count("RolloutItem::WorkflowState(_)"), 4)

    def test_session_history_accepts_workflow_state(self):
        session = source("codex-rs/core/src/session/session.rs")
        self.assertEqual(session.count("RolloutItem::WorkflowState(_)"), 1)

    def test_session_history_regressions_ignore_workflow_state(self):
        tests = source("codex-rs/core/src/session/tests.rs")
        self.assertEqual(tests.count("RolloutItem::WorkflowState(_)"), 2)

    def test_adaptive_runtime_signal_is_handled_by_turn_stream(self):
        turn = source("codex-rs/core/src/session/turn.rs")
        self.assertIn("EventMsg::AdaptiveRuntimeSignal(_)", turn)

    def test_interrupted_custom_tool_fixture_preserved(self):
        tests = source("codex-rs/core/src/session/rollout_reconstruction_tests.rs")
        self.assertIn("fn resumed_history_recovers_orphan_custom_tool_call_without_panicking()", tests)
        self.assertIn('"interrupted-custom-tool"', tests)
        self.assertIn('"aborted"', tests)

    def test_upstream_async_compaction_and_session_config_refresh_preserved(self):
        compaction = source("codex-rs/core/src/session/async_compaction.rs")
        refresh = source("codex-rs/core/src/session/config_refresh.rs")
        self.assertIn("async", compaction)
        self.assertIn("config", refresh)

    def test_trusted_adaptive_handler_untouched(self):
        handler = "codex-rs/core/src/tools/handlers/adaptive_signal.rs"
        self.assertEqual(git_blob("HEAD", handler), git_blob(PRODUCTION, handler))
        self.assertEqual(
            git_blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )

    def test_product_markers_and_lockfile_unadvanced(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(git_blob("HEAD", "codex-rs/Cargo.lock"),
                         "a6cd27b0b70fde2f2be9c1777f3666ad3fa318bb")

    def test_packet_not_a_release_candidate(self):
        doc = (ROOT / "docs/codexdd-0.4.5-packet-3c5b-session-replay.md").read_text(
            encoding="utf-8"
        )
        for marker in ("INCOMPLETE / NOT A RELEASE CANDIDATE", "Cargo.lock", "native Rust", "3C.5"):
            self.assertIn(marker, doc)


if __name__ == "__main__":
    unittest.main()
