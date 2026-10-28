"""Fail-closed current-source 0.4.5 owner-class matrix, never release acceptance."""

from __future__ import annotations

import json
import re
import subprocess
import unittest
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MATRIX = ROOT / "docs/upstream-candidates/codexdd-045-109-semantic-owner-matrix.json"
OVERLAPS = ROOT / "docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-overlap-source-receipt.json"
CONFLICTS = ROOT / "docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-original-direct-conflicts.json"
FORMAT = ROOT / "docs/upstream-candidates/codexdd-045-format-followup.json"


def git(*args: str) -> str:
    return subprocess.check_output(
        ["git", *args], cwd=ROOT, text=True
    ).strip()


class ExactSemanticOwnerRoster(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.matrix = json.loads(MATRIX.read_text(encoding="utf-8"))
        cls.overlaps = json.loads(OVERLAPS.read_text(encoding="utf-8"))
        cls.conflicts = json.loads(CONFLICTS.read_text(encoding="utf-8"))
        cls.formatters = json.loads(FORMAT.read_text(encoding="utf-8"))

    def test_every_one_of_109_overlap_paths_has_explicit_owner(self):
        rows = self.matrix["paths"]
        original = self.overlaps["overlaps"]
        self.assertEqual(len(rows), 109)
        self.assertEqual(
            [r["path"] for r in rows], [r["path"] for r in original]
        )
        self.assertEqual(len({r["path"] for r in rows}), 109)
        self.assertEqual(
            dict(Counter(r["owner"] for r in rows)),
            self.matrix["owner_counts"],
        )
        self.assertEqual(
            self.matrix["owner_counts"],
            {
                "bazel_packaging": 3,
                "dependency": 2,
                "generated_protocol": 12,
                "app_server_protocol": 11,
                "cli_startup": 2,
                "config": 1,
                "core_authority_history": 21,
                "persistence_rollout_memory": 13,
                "tui_adaptive_daemon": 44,
            },
        )
        self.assertEqual(set(self.matrix["owner_counts"]), set(self.matrix["owner_contracts"]))
        for name, info in self.matrix["owner_contracts"].items():
            with self.subTest(owner=name):
                self.assertTrue(info["contract"])
                self.assertTrue(info["next_gate"])
                self.assertTrue(all(isinstance(n, int) and n > 0 for n in info["evidence"]))

    def test_historical_blob_review_is_immutable_and_current_bytes_are_pinned(self):
        reviewed = {r["path"]: r for r in self.overlaps["overlaps"]}
        formats = {r["path"]: r for r in self.formatters["files"]}
        security = self.matrix["actual_candidate_security_blobs"]
        for row in self.matrix["paths"]:
            with self.subTest(path=row["path"]):
                path = row["path"]
                old = reviewed[path]
                self.assertEqual(row["historical_reviewed_blob"], old["reviewed_045_blob"])
                self.assertEqual(row["production_044_blob"], old["production_044_blob"])
                current = (
                    security[path]
                    if path in ("codex-rs/Cargo.lock", "codex-rs/Cargo.toml")
                    else formats[path]["formatted_blob"]
                    if path in formats
                    else old["reviewed_045_blob"]
                )
                self.assertEqual(row["current_blob"], current)
                self.assertRegex(current, r"^[0-9a-f]{40}$")
                self.assertNotEqual(current, old["production_044_blob"])
                self.assertEqual(git("rev-parse", "HEAD:" + path), current)

    def test_all_15_original_conflicts_stay_pending(self):
        direct = {r["path"]: r for r in self.conflicts["conflicts"]}
        self.assertEqual(len(direct), 15)
        for row in self.matrix["paths"]:
            with self.subTest(path=row["path"]):
                self.assertEqual(row["original_direct_conflict"], row["path"] in direct)
                self.assertEqual(row["semantic_status"], "PENDING_OWNER_REVIEW")
                self.assertEqual(row["release_status"], "NOT_ACCEPTED")
                if row["path"] in direct:
                    self.assertEqual(
                        row["original_conflict_gate"],
                        direct[row["path"]]["remaining_native_gate"],
                    )
                else:
                    self.assertIsNone(row["original_conflict_gate"])
        self.assertEqual(
            self.matrix["total_original_direct_conflicts"], 15
        )

    def test_seven_formatter_and_two_secured_dependency_deltas_are_separate(self):
        count = Counter(r["disposition"] for r in self.matrix["paths"])
        self.assertEqual(
            dict(count),
            {
                "frozen_reviewed_identity": 100,
                "audited_secure_dependency_migration": 2,
                "verified_formatter_only": 7,
            },
        )
        self.assertEqual(count, self.matrix["source_dispositions"])

    def test_three_focused_review_receipts_are_scoped_and_source_bound(self):
        expected = {
            "codex-rs/core/src/tools/registry.rs",
            "codex-rs/core/src/context_manager/normalize.rs",
            "codex-rs/tui/src/chatwidget/interaction.rs",
        }
        selected = {
            row["path"]: row
            for row in self.matrix["paths"]
            if row.get("focused_review", {}).get("review_kind")
            == "focused_source_and_native_behavior_evidence_only"
        }
        self.assertEqual(set(selected), expected)
        self.assertEqual(
            len([r for r in self.matrix["paths"] if "focused_review" in r]), 5
        )
        native = (
            ROOT / ".github/workflows/codexdd-045-109-core-tui-native.yml"
        ).read_text(encoding="utf-8")
        for path, row in selected.items():
            with self.subTest(path=path):
                evidence = row["focused_review"]
                self.assertTrue(row["original_direct_conflict"])
                self.assertEqual(row["semantic_status"], "PENDING_OWNER_REVIEW")
                self.assertEqual(row["release_status"], "NOT_ACCEPTED")
                self.assertEqual(evidence["semantic_signoff"], "NOT_GRANTED")
                self.assertEqual(
                    evidence["review_kind"],
                    "focused_source_and_native_behavior_evidence_only",
                )
                self.assertEqual(evidence["native_evidence_run"], 40583499451)
                self.assertEqual(evidence["source_blob"], row["current_blob"])
                self.assertEqual(git("rev-parse", "HEAD:" + path), evidence["source_blob"])
                self.assertTrue(evidence["review_scope"])
                self.assertTrue(evidence["remaining_gate"])
                source = (ROOT / path).read_text(encoding="utf-8")
                for fragment in evidence["source_fragments"]:
                    self.assertIn(fragment, source)
                for selector in evidence["native_selectors"]:
                    self.assertIn("'" + selector + "'", native)

    def test_security_and_product_markers_are_not_promoted(self):
        self.assertEqual(self.matrix["contract_version"], 1)
        self.assertEqual(
            self.matrix["scope"], "NONRELEASE_SOURCE_OWNER_REVIEW_ONLY"
        )
        self.assertEqual(self.matrix["release_status"], "NOT_ACCEPTED")
        for path, pinned in self.matrix["actual_candidate_security_blobs"].items():
            with self.subTest(path=path):
                self.assertTrue(re.fullmatch(r"[0-9a-f]{40}", pinned))
                self.assertEqual(git("rev-parse", "HEAD:" + path), pinned)
        self.assertEqual(git("show", "HEAD:codex-rs/codexdd-version.txt"), "0.4.4")
        self.assertEqual(
            git("show", "HEAD:codex-rs/upstream-codex-release.txt"),
            "rust-v0.162.0",
        )


if __name__ == "__main__":
    unittest.main()
