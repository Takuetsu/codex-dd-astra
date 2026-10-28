"""CodexDD 0.4.5 3F.2: freeze 15 authentic direct conflicts, no release acceptance."""

from __future__ import annotations

import json
import re
import subprocess
import unittest
from pathlib import Path
from codexdd_045_format_provenance import current_blob

ROOT = Path(__file__).resolve().parents[2]
RECEIPT = (
    ROOT
    / "docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-original-direct-conflicts.json"
)
OVERLAPS = (
    ROOT
    / "docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-overlap-source-receipt.json"
)
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
REPLAY = "77bd55e2360a3c35f94ee1608c8b7eed49247365"
PRE_TLS_INTEGRATION = "e35cf007be31d2fd5f6aa98c1360b297e5b9325a"
# Original 15-conflict ledger remains frozen at the pre-TLS snapshot.
# The crypto-owner migration replaced only the current lock identity.
SECURE_CARGO_TOML = "cfebd9b784ef9ff027b274aa9f7101c9c2496613"
SECURE_CARGO_LOCK = "13bc15ff844e93dfaf7777c73bb7313b35bb3b81"
SECURE_DENY = "f282c7c55d05201e308e60804f85754c8522d462"


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


class OriginalDirectConflictReceipt(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.data = json.loads(RECEIPT.read_text(encoding="utf-8"))
        cls.overlaps = {
            r["path"]: r
            for r in json.loads(OVERLAPS.read_text(encoding="utf-8"))["overlaps"]
        }

    def test_replay_roster_has_exact_fifteen_sorted_paths(self):
        d = self.data
        self.assertEqual(d["contract_version"], 1)
        self.assertEqual(
            d["receipt_kind"],
            "codexdd_045_official_163_original_direct_conflict_ledger",
        )
        self.assertEqual(d["evidence_run"], 39613477624)
        self.assertEqual(d["source_checkpoint"], REPLAY)
        self.assertEqual(d["total_direct_conflicts"], 15)
        self.assertEqual(
            d["conflicts_by_group"],
            {"build": 1, "dependency": 1, "generated_protocol": 4, "core": 3, "tui": 6},
        )
        names = [row["path"] for row in d["conflicts"]]
        self.assertEqual(len(names), 15)
        self.assertEqual(names, sorted(set(names)))
        self.assertIn(".github/workflows/bazel.yml", names)
        self.assertIn("codex-rs/Cargo.lock", names)
        self.assertIn("codex-rs/core/src/tools/registry.rs", names)
        self.assertIn("codex-rs/tui/src/chatwidget/interaction.rs", names)
        subprocess.run(
            ["git", "merge-base", "--is-ancestor", REPLAY, "HEAD"], cwd=ROOT, check=True
        )

    def test_conflicts_are_reviewed_four_way_source_blobs(self):
        self.assertTrue(self.data["all_identity_moved_off_production"])
        for r in self.data["conflicts"]:
            with self.subTest(path=r["path"]):
                o = self.overlaps[r["path"]]
                self.assertEqual(r["official_162_blob"], o["official_162_blob"])
                self.assertEqual(r["official_163_blob"], o["official_163_blob"])
                self.assertEqual(r["production_044_blob"], o["production_044_blob"])
                self.assertEqual(r["integration_reviewed_blob"], o["reviewed_045_blob"])
                self.assertNotEqual(
                    r["integration_reviewed_blob"], r["production_044_blob"]
                )
                if r["path"] == "codex-rs/Cargo.lock":
                    # Historical direct-conflict receipts are immutable evidence.
                    self.assertEqual(
                        git("rev-parse", PRE_TLS_INTEGRATION + ":" + r["path"]),
                        r["integration_reviewed_blob"],
                    )
                    self.assertEqual(
                        git("rev-parse", "HEAD:" + r["path"]), SECURE_CARGO_LOCK
                    )
                else:
                    self.assertEqual(
                        git("rev-parse", "HEAD:" + r["path"]),
                        current_blob(r["path"], r["integration_reviewed_blob"]),
                    )
                self.assertEqual(
                    git("rev-parse", PRODUCTION + ":" + r["path"]),
                    r["production_044_blob"],
                )

    def test_no_false_semantic_or_security_acceptance(self):
        self.assertIn("NOT_ACCEPTED", self.data["acceptance_scope"])
        for r in self.data["conflicts"]:
            with self.subTest(path=r["path"]):
                self.assertEqual(
                    r["source_status"], "identity_reconciled_semantics_pending"
                )
                self.assertEqual(r["release_status"], "NOT_ACCEPTED")
                self.assertIn(
                    r["source_strategy"],
                    (
                        "reviewed_fork_ci_merge",
                        "cargo_resolver_rebuilt",
                        "generator_rebuilt",
                        "manual_security_merge",
                        "manual_three_way_merge",
                    ),
                )
                self.assertTrue(r["remaining_native_gate"])
                self.assertTrue(r["prior_nonrelease_ci_runs"])
                self.assertTrue(
                    all(
                        isinstance(n, int) and n > 0
                        for n in r["prior_nonrelease_ci_runs"]
                    )
                )
        registry = next(
            r
            for r in self.data["conflicts"]
            if r["path"] == "codex-rs/core/src/tools/registry.rs"
        )
        self.assertEqual(registry["source_strategy"], "manual_security_merge")
        self.assertEqual(registry["release_status"], "NOT_ACCEPTED")
        self.assertEqual(
            (ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4"
        )
        self.assertEqual(
            (ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(),
            "rust-v0.162.0",
        )
        self.assertEqual(
            git("hash-object", "codex-rs/deny.toml"),
            SECURE_DENY,
        )

    def test_current_secure_manifest_is_still_pinned(self):
        self.assertEqual(
            git("rev-parse", "HEAD:codex-rs/Cargo.toml"), SECURE_CARGO_TOML
        )
        self.assertEqual(
            git("rev-parse", "HEAD:codex-rs/Cargo.lock"), SECURE_CARGO_LOCK
        )

    def test_retired_original_conflict_diagnostic(self):
        self.assertFalse(
            (
                ROOT / ".github/workflows/codexdd-045-original-conflict-recon.yml"
            ).exists()
        )


if __name__ == "__main__":
    unittest.main()
