"""CodexDD 0.4.5 source-only four-way overlap receipt regression.

Official 0.162 and 0.163 SHA columns were obtained from pinned GitHub trees.
CI verifies historical production and current integration Git object identities;
it does not re-fetch external upstream trees or prove semantic correctness.
"""
from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / "docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-overlap-source-receipt.json"
LEGACY = ROOT / "docs/upstream-candidates/original-blocked-conflicts.txt"
PINNED_BASE = "c1382380de69521303b416720a52f42d51af6248"
PINNED_UPSTREAM = "83086188ae0614bd882818ff1aa23f3194962012"
PINNED_PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
PINNED_SNAPSHOT = "f6fb89c098da28626c7c072a40d5e9b7306c8866"


def git(*args: str) -> bytes:
    return subprocess.check_output(["git", *args], cwd=ROOT)


def objects_at(ref: str) -> dict[str, str]:
    """List blob IDs by Git pathname without reading possibly compressed files."""
    entries = {}
    for record in git("ls-tree", "-r", "-z", "--full-tree", ref).split(b"\x00"):
        if not record:
            continue
        header, filename = record.split(b"\t", 1)
        _mode, kind, sha = header.decode("ascii").split(" ", 2)
        if kind == "blob":
            entries[filename.decode("utf-8")] = sha
    return entries


class FourWayOverlapReceipt(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.receipt = json.loads(MANIFEST.read_text(encoding="utf-8"))
        cls.production = objects_at(PINNED_PRODUCTION)
        cls.integration = objects_at("HEAD")

    def test_pinned_refs_and_no_release_claim(self):
        record = self.receipt
        self.assertEqual(record["contract_version"], 1)
        self.assertEqual(record["receipt_kind"], "codexdd_045_source_provenance_only")
        self.assertIn("no 0.163 semantic", record["acceptance"])
        self.assertEqual(record["refs"], {
            "baseline_official_162": PINNED_BASE,
            "target_official_163": PINNED_UPSTREAM,
            "production_codexdd_044": PINNED_PRODUCTION,
            "integration_snapshot": PINNED_SNAPSHOT,
        })
        subprocess.run(
            ["git", "merge-base", "--is-ancestor", PINNED_SNAPSHOT, "HEAD"],
            cwd=ROOT, check=True,
        )
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")
        self.assertEqual((ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(), "rust-v0.162.0")

    def test_exact_109_source_overlap_accounting(self):
        r = self.receipt
        self.assertEqual(r["upstream_changed_paths"], 1314)
        self.assertEqual(r["upstream_fork_overlap_paths"], 109)
        self.assertEqual(r["stale_production_overlap_paths"], 0)
        self.assertEqual(r["overlap_group_counts"], {".github": 1, "codex-rs": 106, "sdk": 2})
        rows = r["overlaps"]
        self.assertEqual(len(rows), 109)
        names = [p["path"] for p in rows]
        self.assertEqual(names, sorted(set(names)))
        self.assertEqual(sum(n.startswith("codex-rs/") for n in names), 106)
        self.assertEqual(sum(n.startswith(".github/") for n in names), 1)
        self.assertEqual(sum(n.startswith("sdk/") for n in names), 2)

    def test_every_overlap_remains_reviewed_nonproduction_blob(self):
        for row in self.receipt["overlaps"]:
            with self.subTest(path=row["path"]):
                path = row["path"]
                base = row["official_162_blob"]
                upstream = row["official_163_blob"]
                production = row["production_044_blob"]
                reviewed = row["reviewed_045_blob"]
                for sha in (base, upstream, production, reviewed):
                    self.assertRegex(sha, r"^[0-9a-f]{40}$")
                self.assertNotEqual(base, upstream)
                self.assertNotEqual(base, production)
                self.assertNotEqual(production, reviewed)
                self.assertEqual(self.production.get(path), production)
                self.assertEqual(self.integration.get(path), reviewed)

    def test_legacy_043_conflicts_are_not_mislabeled_163_conflicts(self):
        r = self.receipt
        paths = LEGACY.read_text(encoding="utf-8").strip().splitlines()
        rows = r["legacy_043_conflicts"]
        self.assertEqual(len(paths), 22)
        self.assertEqual([row["path"] for row in rows], paths)
        self.assertEqual(r["legacy_043_blocked_paths"], 22)
        self.assertEqual(r["legacy_paths_changed_in_upstream_163"], 13)
        self.assertEqual(r["legacy_paths_unchanged_in_upstream_163"], 9)
        self.assertEqual(sum(row["upstream_changed"] for row in rows), 13)
        for row in rows:
            with self.subTest(path=row["path"]):
                self.assertEqual(row["upstream_changed"], row["official_162_blob"] != row["official_163_blob"])
                self.assertEqual(row["production_changed"], row["production_044_blob"] != row["reviewed_045_blob"])
                self.assertEqual(self.production.get(row["path"]), row["production_044_blob"])
                self.assertEqual(self.integration.get(row["path"]), row["reviewed_045_blob"])

    def test_dependency_policy_not_implicitly_promoted(self):
        self.assertEqual(git("hash-object", "codex-rs/Cargo.toml").decode().strip(), "c78e1e92e3cfd21a4d16ceea2d55b43d1976e8e4")
        self.assertEqual(git("hash-object", "codex-rs/Cargo.lock").decode().strip(), "a6cd27b0b70fde2f2be9c1777f3666ad3fa318bb")
        self.assertEqual(git("hash-object", "codex-rs/deny.toml").decode().strip(), "ea7fdc149fc880c6b36d75c29851641034289a4e")


if __name__ == "__main__":
    unittest.main()
