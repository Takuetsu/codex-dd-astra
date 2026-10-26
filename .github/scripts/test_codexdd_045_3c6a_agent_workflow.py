"""CodexDD 0.4.5 / 3C.6a: exact-source and child-spawn workflow preservation.

Read-only static source tests only; no Rust compilation or release acceptance.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
ORIGINAL_FORK = "cd6bf20ea4259d72926835b08dc502fbdf289ff7"
OFFICIAL_UPSTREAM = "70cf8b7ef07e6ffaab9cfa32435bf6d7408137e3"
STAGED = {
    "codex-rs/core/src/agent/api.rs": [
        "1a498228b613ef8370f83f22778987fe19e75b9f",
        "622c45134301c64024afde9603241212730a86cd"
    ],
    "codex-rs/core/src/agent/child_config.rs": [
        "713c4418a912f98571ff95e8aec84df81524167e",
        "3d56d7e646e275b6d66b0501d5f7021e1d300bdf"
    ],
    "codex-rs/core/src/agent/control.rs": [
        "a8d57e19ce1e8a4aab762ab85f00fe7198cde0ea",
        "895cd26802d76ba34c3ece33a55592d6befa0b3b"
    ],
    "codex-rs/core/src/agent/control/api.rs": [
        "ef0a6994867fb212128e45493167fa4363cbfd2f",
        "4087cde92b5f399ab30bb82c9fe5973a58d6b97b"
    ],
    "codex-rs/core/src/agent/control/residency.rs": [
        "dd53eb6f2c5df381e6503ff3f67df6f4623464ae",
        "4755c221f8beb568fbf0eaf96c0410aa55badcf9"
    ],
    "codex-rs/core/src/agent/control/residency_tests.rs": [
        "5b524c8278091ebcbfd5ea25a01b99e8f82254d3",
        "5bf450e456ba2a21b6ae117c052f6d5978a4fadb"
    ],
    "codex-rs/core/src/agent/control/sender_context.rs": [
        "303840f78a2bfc19a8bf4d04d3a6e47e35f9fd2a",
        "184a5add1420541f9d40a1881ac80ab489e270ec"
    ],
    "codex-rs/core/src/agent/control/spawn_guard.rs": [
        "d8befedd40d89c9ff728d9f057a99982672dfd27",
        "c9095893b175f8d1b9ce3fa6cc4f12e7165e2703"
    ],
    "codex-rs/core/src/agent/control_tests.rs": [
        "801d4d9f4b7ca4fd7a4823c8bc0c41504f693f81",
        "6d97eceb2ed4a68f349562e7a3d8cfc5ef53cbe4"
    ],
    "codex-rs/core/src/agent/role_tests.rs": [
        "bcf8892aa633b6c26bf00a584ebc0d4e175549e2",
        "b480f585809a3a83fce5e07074d60c896d96aa7d"
    ],
    "codex-rs/core/src/agent/types.rs": [
        "852a51671364aa349c3bfb29dfdce906e5fc7441",
        "6cbe9acb252c6f68857f3e4b3704e57619a2c6b4"
    ]
}


def git_blob(ref: str, path: str) -> str:
    return subprocess.run(
        ["git", "rev-parse", ref + ":" + path],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout.strip()


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT, check=True, capture_output=True, text=True, encoding="utf-8",
    ).stdout


class AgentChildSpawnIntegrationTests(unittest.TestCase):
    def assertIn(self, member, container, msg=None):
        if isinstance(container, str) and len(container) > 2048:
            self.assertTrue(member in container, msg or f"missing source marker: {member!r}")
            return
        super().assertIn(member, container, msg)

    def test_eleven_upstream_owned_agent_blobs_are_pinned(self):
        self.assertEqual(len(STAGED), 11)
        for path, (production, target) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob(PRODUCTION, path), production)
                self.assertEqual(git_blob("HEAD", path), target)

    def test_production_agent_spawn_has_custom_workflow_cases(self):
        path = "codex-rs/core/src/agent/control/spawn.rs"
        self.assertEqual(git_blob(PRODUCTION, path), ORIGINAL_FORK)
        self.assertNotEqual(git_blob("HEAD", path), OFFICIAL_UPSTREAM)

    def test_forked_context_keeps_workflow_state_in_child_history(self):
        spawn = source("codex-rs/core/src/agent/control/spawn.rs")
        self.assertIn("RolloutItem::Compacted(_)\n        | RolloutItem::EventMsg(_)\n        | RolloutItem::SessionMeta(_)\n        | RolloutItem::WorkflowState(_) => true,", spawn)

    def test_forked_message_history_keeps_workflow_state(self):
        spawn = source("codex-rs/core/src/agent/control/spawn.rs")
        self.assertIn("| RolloutItem::InterAgentCommunicationMetadata { .. }\n                | RolloutItem::WorkflowState(_) => true,", spawn)

    def test_workflow_state_policy_appears_twice_not_more(self):
        spawn = source("codex-rs/core/src/agent/control/spawn.rs")
        self.assertEqual(spawn.count("RolloutItem::WorkflowState(_)"), 2)

    def test_child_spawn_config_and_role_fixtures_staged(self):
        child = source("codex-rs/core/src/agent/child_config.rs")
        roles = source("codex-rs/core/src/agent/role_tests.rs")
        self.assertIn("fn ", child)
        self.assertIn("fn ", roles)

    def test_agent_residency_tests_staged(self):
        residency = source("codex-rs/core/src/agent/control/residency_tests.rs")
        self.assertIn("fn ", residency)

    def test_workflow_state_reconstruction_stays_reconciled(self):
        reconstructed = source("codex-rs/core/src/session/rollout_reconstruction.rs")
        self.assertEqual(reconstructed.count("RolloutItem::WorkflowState(_)"), 4)

    def test_adaptive_reconnaissance_policy_remains_fixed(self):
        self.assertEqual(
            git_blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )

    def test_product_version_and_lockfile_unadvanced(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(git_blob("HEAD", "codex-rs/Cargo.lock"), "a6cd27b0b70fde2f2be9c1777f3666ad3fa318bb")

    def test_release_remains_blocked(self):
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c6a-agent-workflow.md").read_text(encoding="utf-8")
        for marker in ("INCOMPLETE / NOT A RELEASE CANDIDATE", "native Rust", "Cargo.lock", "source-only"):
            self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
