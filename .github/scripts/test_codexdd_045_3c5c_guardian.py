"""CodexDD 0.4.5 3C.5c: upstream Guardian and approvals source checks.

Only Git source/text checks; no native approvals, Rust build or release receipt.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
STAGED = {
    "codex-rs/core/src/guardian/approval_request.rs": [
        "ee6fc74330e668e57e170c609683ee4c070f889b",
        "63893f6a30241d10e304c3feae58d7e97c5da4b2"
    ],
    "codex-rs/core/src/guardian/coverage.rs": [
        "f505bcb198ae54a4a40cf73463cbb27661664e7b",
        "d0b2d124349f27c0a5b8ec7cc38464e580be662f"
    ],
    "codex-rs/core/src/guardian/decision.rs": [
        "62999c2f59e474fd6301656ebb2915ffb523386e",
        "2f7dc7c7b0a43beb109785310d5056534c836409"
    ],
    "codex-rs/core/src/guardian/feedback.rs": [
        "efff7dc8401758b54ac904c2b01bace1bb21cb5e",
        "4bf98acdb72b4a990e09df403e46d318c845bdb8"
    ],
    "codex-rs/core/src/guardian/input_budget.rs": [
        "529c0d65a5c2412273b335a29c21bcc485fe593d",
        "9c5fbfdb977c6e47bb28203998fa7f2d14aba9aa"
    ],
    "codex-rs/core/src/guardian/input_budget_tests.rs": [
        "3f0b66f0c4fd3caa0900a45088fea5651dbc5d54",
        "d623337dc9269b6aed6976dd2b83793b95b092cd"
    ],
    "codex-rs/core/src/guardian/mod.rs": [
        "c1cdb2ffb3ac1bf124f97b68d2049c38100e075d",
        "a0f4dedddfa1aad1769035d5518ec0a322c97635"
    ],
    "codex-rs/core/src/guardian/prompt.rs": [
        "fd432462d8001c4357647f99cf84bf7ee9b5cc5f",
        "212a66cb219c6f6010010f6172febc96a6caa4f5"
    ],
    "codex-rs/core/src/guardian/review.rs": [
        "a3e28cea27ab404992bbf00af703ff8abf744b46",
        "355360d1609172b6dce8e0894a94ce059c152074"
    ],
    "codex-rs/core/src/guardian/review_request.rs": [
        "2f4783355e499fc698b22ee747ec45a13549ba5d",
        "798c8806e1bcd76a37e1eb62835bb0dd56be4606"
    ],
    "codex-rs/core/src/guardian/review_session.rs": [
        "f19e9b1401166ca428b1d56fc271581f4873daa4",
        "78ce4b6a5b8bf5f4b7fa05797d609fbfbe55cf08"
    ],
    "codex-rs/core/src/guardian/review_session_setup.rs": [
        "50f5f7299e09af53c79fa43d9481e235cb844e6b",
        "ce3fd4bd137bab2918750f032c735f5c2c79d6a5"
    ],
    "codex-rs/core/src/guardian/review_session_tests.rs": [
        "0533730b2b2d7b6ff0aa0aa3fa4908ed28c4e962",
        "1af6d709182e4d1ef5ff8d8188534af1f01dc77b"
    ],
    "codex-rs/core/src/guardian/runtime.rs": [
        "9fe65392ffe5e49a36c121bfec9ccfdff27a0ec9",
        "9cb154233eae9e16c5b4226ebb94df6a0a63575c"
    ],
    "codex-rs/core/src/guardian/test_host.rs": [
        "655f4e150a7cb98ea85a09e84d8add28ec436c73",
        "3f98917afbb5e51a6672f864dc29aa75e2a9cf66"
    ],
    "codex-rs/core/src/guardian/tests.rs": [
        "bacd664825d5a2daf7b12ae6098f91fc765e4308",
        "f1bb71163eac8ab875a904f54b769a72486a3c2d"
    ],
    "codex-rs/core/src/tools/approvals.rs": [
        "e892a6e805b9a4a4c15689729b3c14d62109b151",
        "16197b56d33965567eab13ababe4761673fbc1bf"
    ]
}


def git_blob(ref: str, path: str) -> str:
    return subprocess.run(
        ["git", "rev-parse", ref + ":" + path],
        cwd=ROOT, capture_output=True, check=True, text=True,
    ).stdout.strip()


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT, capture_output=True, check=True, text=True, encoding="utf-8",
    ).stdout


class GuardianApprovalReconciliationTests(unittest.TestCase):
    def assertIn(self, member, container, msg=None):
        if isinstance(container, str) and len(container) > 2048:
            self.assertTrue(member in container, msg or f"missing source marker {member!r}")
            return
        super().assertIn(member, container, msg)

    def assertNotIn(self, member, container, msg=None):
        if isinstance(container, str) and len(container) > 2048:
            self.assertFalse(member in container, msg or f"unexpected source marker {member!r}")
            return
        super().assertNotIn(member, container, msg)

    def test_seventeen_exact_official_guardian_and_approval_blobs(self):
        self.assertEqual(len(STAGED), 17)
        for path, (_, expected) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob("HEAD", path), expected)

    def test_seventeen_original_fork_blobs_were_unmodified(self):
        for path, (original, _) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob(PRODUCTION, path), original)

    def test_guardian_approval_request_types_remain(self):
        src = source("codex-rs/core/src/guardian/approval_request.rs")
        self.assertIn("GuardianApprovalRequest", src)
        self.assertIn("approval_id", src)

    def test_approval_router_retains_apply_patch_and_review_decisions(self):
        approvals = source("codex-rs/core/src/tools/approvals.rs")
        self.assertIn("ApprovalAction::ApplyPatch", approvals)
        self.assertIn("ReviewDecision", approvals)
        self.assertIn("GuardianApprovalRequest", approvals)

    def test_removed_execve_action_not_reintroduced(self):
        approvals = source("codex-rs/core/src/tools/approvals.rs")
        self.assertNotIn("Self::Execve", approvals)
        self.assertNotIn("ApprovalAction::Execve", approvals)

    def test_guardian_review_session_and_regressions_staged(self):
        review = source("codex-rs/core/src/guardian/review_session.rs")
        tests = source("codex-rs/core/src/guardian/review_session_tests.rs")
        self.assertIn("GuardianApprovalRequest", review)
        self.assertIn("async fn ", tests)

    def test_guardian_input_budget_and_tests_staged(self):
        budget = source("codex-rs/core/src/guardian/input_budget.rs")
        tests = source("codex-rs/core/src/guardian/input_budget_tests.rs")
        self.assertIn("fn ", budget)
        self.assertIn("fn ", tests)

    def test_original_guardian_permissions_unchanged(self):
        path = "codex-rs/core/src/guardian/permissions.rs"
        expected = "3580d73eb94de4a13a4244a97ae4faf40ecec30e"
        self.assertEqual(git_blob("HEAD", path), expected)
        self.assertEqual(git_blob(PRODUCTION, path), expected)

    def test_codexdd_adaptive_policy_remains_fixed(self):
        self.assertEqual(
            git_blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )

    def test_product_markers_and_cargo_lock_unchanged(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(
            git_blob("HEAD", "codex-rs/Cargo.lock"),
            "48ae4fb45a83de4d72dee6200b83a770dbf8f760",
        )

    def test_packet_remains_explicitly_incomplete(self):
        doc = (ROOT / "docs/codexdd-0.4.5-packet-3c5c-guardian-approvals.md").read_text(
            encoding="utf-8"
        )
        for marker in ("INCOMPLETE / NOT A RELEASE CANDIDATE", "native Rust", "Cargo.lock", "3C.5"):
            self.assertIn(marker, doc)


if __name__ == "__main__":
    unittest.main()
