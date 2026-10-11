"""End-to-end synthetic Git candidate preflight with immutable blocked evidence."""

import copy
import hashlib
import json
import subprocess
import tempfile
import unittest
from pathlib import Path

from test_upstream_reconciliation_v2 import (
    CONFLICTS, NEW_TAG, OLD_TAG, OVERLAPS, SOURCE_SHA, TARGET_SHA, fixtures,
)
from upstream_reconciliation import canonical_json_bytes
from upstream_reconciliation_cli import (
    _ORIGINAL_CONFLICTS,
    _ORIGINAL_MANIFEST,
    verify_native_or_promotion_preflight,
)


def git(repo: Path, *args: str) -> str:
    return subprocess.run(
        ["git", "-C", str(repo), *args], capture_output=True, text=True, check=True
    ).stdout.strip()


class CommandPreflightTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.repo = Path(self.temp.name)
        git(self.repo, "init", "-q")
        git(self.repo, "config", "user.name", "CodexDD CLI fixture")
        git(self.repo, "config", "user.email", "fixture@example.invalid")
        git(self.repo, "checkout", "-qb", "dd/astra-policy-v2")
        root = self.repo / "codex-rs"
        root.mkdir()
        (root / "codexdd-version.txt").write_text("0.4.2\n")
        (root / "upstream-codex-release.txt").write_text("rust-v0.159.2\n")
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "production")
        self.production = git(self.repo, "rev-parse", "HEAD")
        self.production_tree = git(self.repo, "rev-parse", "HEAD^{tree}")
        git(self.repo, "checkout", "-qb", "dd/codexdd-v0.4.3-upstream-0.162.0-integration")
        (root / "codexdd-version.txt").write_text("0.4.3\n")
        (root / "upstream-codex-release.txt").write_text("rust-v0.162.0\n")
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "integrated")
        self.source_sha = git(self.repo, "rev-parse", "HEAD")
        self.source_tree = git(self.repo, "rev-parse", "HEAD^{tree}")
        original_data, conflicts_data, manifest, sidecar = fixtures()
        original = json.loads(original_data)
        original["production_sha"] = self.production
        original["production_tree"] = self.production_tree
        self.original_raw = canonical_json_bytes(original)
        self.conflicts_raw = conflicts_data
        self.approved_manifest_sha = hashlib.sha256(self.original_raw).hexdigest()
        self.approved_conflicts_sha = hashlib.sha256(conflicts_data).hexdigest()
        manifest["production_sha"] = self.production
        manifest["production_tree"] = self.production_tree
        manifest["source_snapshot_commit_sha"] = self.source_sha
        manifest["source_snapshot_tree_sha"] = self.source_tree
        manifest["discovery_manifest_sha256"] = self.approved_manifest_sha
        manifest["discovery_conflicts_sha256"] = self.approved_conflicts_sha
        sidecar["production"]["sha"] = self.production
        sidecar["production"]["tree"] = self.production_tree
        sidecar["source_snapshot"]["commit_sha"] = self.source_sha
        sidecar["source_snapshot"]["tree_sha"] = self.source_tree
        sidecar["discovery"]["manifest_sha256"] = self.approved_manifest_sha
        sidecar["discovery"]["conflicts_sha256"] = self.approved_conflicts_sha
        for record in sidecar["generated_artifacts"]:
            record["generator_source_sha"] = self.source_sha
        for review in sidecar["security_reviews"].values():
            review["source_snapshot_commit_sha"] = self.source_sha
        self.sidecar_raw = canonical_json_bytes(sidecar)
        manifest["sidecar_sha256"] = hashlib.sha256(self.sidecar_raw).hexdigest()
        self.manifest_raw = canonical_json_bytes(manifest)
        self.manifest_path = "docs/upstream-candidates/codexdd-upstream.json"
        git(self.repo, "checkout", "-qb", manifest["candidate_branch"])
        docs = self.repo / "docs/upstream-candidates"
        docs.mkdir(parents=True)
        (self.repo / self.manifest_path).write_bytes(self.manifest_raw)
        (self.repo / manifest["sidecar_path"]).write_bytes(self.sidecar_raw)
        (self.repo / _ORIGINAL_MANIFEST).write_bytes(self.original_raw)
        (self.repo / _ORIGINAL_CONFLICTS).write_bytes(conflicts_data)
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "verified metadata only")
        self.candidate = git(self.repo, "rev-parse", "HEAD")

    def verify(self, **kwargs):
        args = {
            "repo": self.repo,
            "manifest_path": self.manifest_path,
            "expected_production_sha": self.production,
            "expected_source_sha": self.source_sha,
            "expected_source_tree": self.source_tree,
            "expected_target_tag": NEW_TAG,
            "expected_target_sha": TARGET_SHA,
            "approved_manifest_sha256": self.approved_manifest_sha,
            "approved_conflicts_sha256": self.approved_conflicts_sha,
            "promotion": False,
        }
        args.update(kwargs)
        return verify_native_or_promotion_preflight(**args)

    def test_positive_candidate_is_read_only_verified(self):
        self.assertEqual(self.verify(), self.candidate)

    def test_manifest_evidence_digest_not_self_authorizing(self):
        with self.assertRaisesRegex(ValueError, "digest mismatch"):
            self.verify(approved_manifest_sha256="f" * 64)

    def test_tampered_worktree_bytes_fail_closed(self):
        path = self.repo / self.manifest_path
        path.write_bytes(path.read_bytes() + b" ")
        with self.assertRaisesRegex(ValueError, "worktree must be clean"):
            self.verify()

    def test_extra_candidate_source_commit_rejected(self):
        (self.repo / "codex-rs/codexdd-version.txt").write_text("0.4.4\n")
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "unexpected post-receipt source")
        with self.assertRaisesRegex(ValueError, "immediate single-parent"):
            self.verify()

    def test_untrusted_manifest_path_rejected(self):
        with self.assertRaises(ValueError):
            self.verify(manifest_path="../secret.json")

    def test_production_ref_move_rejected(self):
        git(self.repo, "checkout", "dd/astra-policy-v2")
        (self.repo / "codex-rs/codexdd-version.txt").write_text("0.4.5\n")
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "production changed")
        git(self.repo, "checkout", self.candidate)
        with self.assertRaises(ValueError):
            self.verify()

    def test_wrong_target_and_snapshot_rejected(self):
        with self.assertRaises(ValueError):
            self.verify(expected_target_sha="f" * 40)
        with self.assertRaises(ValueError):
            self.verify(expected_source_tree="f" * 40)


if __name__ == "__main__":
    unittest.main()
