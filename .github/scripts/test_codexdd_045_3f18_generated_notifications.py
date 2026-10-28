"""Fail-closed source proof: generated TS notifications add one method to upstream 0.163."""

from __future__ import annotations

import json
import re
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OVERLAPS = ROOT / "docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-overlap-source-receipt.json"
MATRIX = ROOT / "docs/upstream-candidates/codexdd-045-109-semantic-owner-matrix.json"
PREFIX = "codex-rs/app-server-protocol/schema/typescript/"
FILES = (PREFIX + "ServerNotification.ts", PREFIX + "ServerNotificationEnvelope.ts")
IMPORT = b'import type { AdaptiveRuntimeSignalNotification } from "./v2/AdaptiveRuntimeSignalNotification";\n'
UNION = b' | { "method": "turn/adaptiveRuntimeSignal", "params": AdaptiveRuntimeSignalNotification }'
OFFICIAL = {
    FILES[0]: "4af7a4307ada139592b325f0aa1da2301deed861",
    FILES[1]: "a5b07d304d2095bb69864400a1c83f255ed04f83",
}


def oid(blob: bytes) -> str:
    return subprocess.check_output(
        ["git", "hash-object", "--stdin"], cwd=ROOT, input=blob
    ).decode("ascii").strip()


def at_head(path: str) -> str:
    return subprocess.check_output(
        ["git", "rev-parse", "HEAD:" + path], cwd=ROOT
    ).decode("ascii").strip()


class GeneratedNotificationAdditiveCompatibility(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.matrix = json.loads(MATRIX.read_text(encoding="utf-8"))
        cls.overlaps = {
            x["path"]: x
            for x in json.loads(OVERLAPS.read_text(encoding="utf-8"))["overlaps"]
        }

    def test_exact_one_method_delta_and_no_dropped_upstream_variants(self):
        all_methods = []
        for path in FILES:
            with self.subTest(path=path):
                row = next(r for r in self.matrix["paths"] if r["path"] == path)
                detail = row["focused_review"]
                self.assertTrue(row["original_direct_conflict"])
                self.assertEqual(row["semantic_status"], "PENDING_OWNER_REVIEW")
                self.assertEqual(row["release_status"], "NOT_ACCEPTED")
                self.assertEqual(detail["semantic_signoff"], "NOT_GRANTED")
                self.assertEqual(
                    detail["review_kind"], "generated_notification_additive_source_provenance"
                )
                self.assertEqual(detail["generator_native_evidence_run"], 40598386099)
                self.assertEqual(detail["baseline_upstream_163_blob"], OFFICIAL[path])
                self.assertEqual(self.overlaps[path]["official_163_blob"], OFFICIAL[path])
                self.assertEqual(detail["current_blob"], row["current_blob"])
                data = (ROOT / path).read_bytes()
                self.assertEqual(oid(data), row["current_blob"])
                self.assertEqual(at_head(path), row["current_blob"])
                self.assertEqual(data.count(IMPORT), 1)
                self.assertEqual(data.count(UNION), 1)
                # Exactly remove the one CodexDD import and event variant. Git's
                # hash-object must equal the original official 0.163 source blob.
                # No historical tree checkout, fetching or manual schema edits.
                recovered = data.replace(IMPORT, b"").replace(UNION, b"")
                self.assertEqual(oid(recovered), OFFICIAL[path])
                methods = re.findall(rb'"method":\s*"([^"]+)"', data)
                self.assertEqual(len(methods), 88)
                self.assertEqual(len(set(methods)), 88)
                self.assertEqual(methods.count(b"turn/adaptiveRuntimeSignal"), 1)
                self.assertEqual(detail["upstream_method_count"], 87)
                self.assertEqual(detail["candidate_method_count"], 88)
                self.assertEqual(detail["added_method"], "turn/adaptiveRuntimeSignal")
                all_methods.append(methods)
        self.assertEqual(all_methods[0], all_methods[1])

    def test_no_product_promotion_or_false_semantic_approval(self):
        self.assertEqual(self.matrix["release_status"], "NOT_ACCEPTED")
        self.assertEqual(
            (ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4"
        )
        self.assertTrue(all(r["release_status"] == "NOT_ACCEPTED" for r in self.matrix["paths"]))


if __name__ == "__main__":
    unittest.main()
