"""3F.54: exact fifth-set source proof for all original core/TUI direct conflicts."""
from __future__ import annotations
import hashlib
import json
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / "docs/upstream-candidates"
LATEST = BASE / "codexdd-045-3f54-core-tui-remaining-conflict-deltas.json"
EARLIER = BASE / "codexdd-045-3f51-core-tui-upstream-delta.json"
MATRIX = BASE / "codexdd-045-109-semantic-owner-matrix.json"
OVERLAPS = BASE / "codexdd-upstream-rust-v0.163.0-overlap-source-receipt.json"
EXPECTED = {
    "codex-rs/core/src/context_manager/normalize.rs": ("4ca88c6a668e9896df06ca81940a0e41e357898d", "532cfdfb34a74f774c41a9a1914347a2492997de"),
    "codex-rs/core/src/tools/registry.rs": ("fc75b3bbf91f1881bd32faaaca93356fbb598b5e", "ebde5a7bdc449a11dc9c1fc1d8475c8e4432010f"),
    "codex-rs/tui/src/app/startup.rs": ("e51254ea2f16f2a356670413ae29c9de0d002e9d", "4c329f378702414c3c17cc31802c778a8dbb2266"),
    "codex-rs/tui/src/chatwidget/interaction.rs": ("e83bc3d1860a8f2f04a14bc136c4b6468785eb67", "8657de2e8373e4a07fd4e97eb7ca5b1ca6048a54"),
    "codex-rs/tui/src/lib.rs": ("f45b01a668998d817abec91313f425f2fa6a9ad7", "a3fb5447b06d48d3a2924eb25dae4174243cb844"),
}

def oid(content: bytes) -> str:
    return hashlib.sha1(b"blob " + str(len(content)).encode("ascii") + b"\x00" + content).hexdigest()

class CoreTuiAllDirectConflicts(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.latest = json.loads(LATEST.read_text(encoding="utf-8"))
        cls.earlier = json.loads(EARLIER.read_text(encoding="utf-8"))
        cls.matrix = json.loads(MATRIX.read_text(encoding="utf-8"))
        cls.overlaps = {r["path"]: r for r in json.loads(OVERLAPS.read_text(encoding="utf-8"))["overlaps"]}
        cls.rows = {r["path"]: r for r in cls.matrix["paths"]}

    def test_exact_nine_original_core_tui_conflicts_have_frozen_source_proof(self):
        self.assertEqual(self.latest["contract_version"], 1)
        self.assertEqual(self.latest["reviewed_count"], 5)
        self.assertIs(self.latest["release_approved"], False)
        self.assertEqual(self.latest["scope"], "NONRELEASE_EXACT_FIVE_REMAINING_CORE_TUI_ORIGINAL_DIRECT_CONFLICTS")
        latest = {r["path"] for r in self.latest["paths"]}
        earlier = {r["path"] for r in self.earlier["paths"]}
        original = {r["path"] for r in self.matrix["paths"]
                    if r["original_direct_conflict"] and r["owner"] in ("core_authority_history", "tui_adaptive_daemon")}
        self.assertEqual(len(original), 9)
        self.assertEqual(len(earlier), 4)
        self.assertEqual(latest, set(EXPECTED))
        self.assertTrue(latest.isdisjoint(earlier))
        self.assertEqual(latest | earlier, original)
        self.assertTrue(all(self.rows[path]["semantic_status"] == "PENDING_OWNER_REVIEW" for path in original))

    def test_exact_current_fork_reverse_hunks_match_official_upstream_0163(self):
        for r in self.latest["paths"]:
            with self.subTest(path=r["path"]):
                path = r["path"]
                self.assertEqual((r["candidate_blob"], r["upstream_blob"]), EXPECTED[path])
                self.assertEqual(r["candidate_blob"], self.rows[path]["current_blob"])
                self.assertEqual(r["upstream_blob"], self.overlaps[path]["official_163_blob"])
                self.assertTrue(r["original_direct_conflict"])
                self.assertEqual(r["semantic_signoff"], "NOT_GRANTED")
                data = subprocess.check_output(["git", "show", "HEAD:" + path], cwd=ROOT)
                self.assertEqual(oid(data), r["candidate_blob"])
                lines = data.decode("utf-8").split("\n")
                prior = len(lines) + 1
                for h in reversed(r["hunks"]):
                    at = h["candidate_start_line"] - 1
                    self.assertGreaterEqual(at, 0)
                    self.assertLess(at, prior)
                    self.assertEqual(lines[at:at + len(h["candidate_lines"])], h["candidate_lines"])
                    lines[at:at + len(h["candidate_lines"])] = h["upstream_lines"]
                    prior = at
                self.assertEqual(oid("\n".join(lines).encode("utf-8")), r["upstream_blob"])

    def test_focused_semantic_review_contracts_remain_not_accepted(self):
        for r in self.latest["paths"]:
            path = r["path"]
            m = self.rows[path]
            with self.subTest(path=path):
                detail = m["focused_review"]
                self.assertEqual(detail["source_blob"], r["candidate_blob"])
                self.assertEqual(detail["semantic_signoff"], "NOT_GRANTED")
                self.assertTrue(detail["remaining_gate"])
                self.assertTrue(detail["native_selectors"])
                source = (ROOT / path).read_text(encoding="utf-8")
                for fragment in detail["source_fragments"]:
                    self.assertIn(fragment, source)

    def test_source_review_only_not_release_or_full_native_claim(self):
        self.assertEqual(len(self.matrix["paths"]), 109)
        self.assertTrue(all(x["release_status"] == "NOT_ACCEPTED" for x in self.matrix["paths"]))
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")
        self.assertNotIn("OWNER_ACCEPTED", {x["semantic_status"] for x in self.matrix["paths"]})

if __name__ == "__main__":
    unittest.main()
