"""Local Git metadata-only candidate constructor tests; zero remote pushes."""

import hashlib
import json
import subprocess
import tempfile
import unittest
from pathlib import Path

from test_upstream_reconciliation_v2 import NEW_TAG, TARGET_SHA, fixtures
from upstream_reconciliation import canonical_json_bytes
from upstream_reconciliation_stage import construct_local_v2_candidate


def git(repo: Path, *args: str) -> str:
    return subprocess.run(
        ["git", "-C", str(repo), *args],
        check=True, capture_output=True, text=True,
    ).stdout.strip()


class MetadataConstructionTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.repo = Path(self.tmp.name)
        git(self.repo, "init", "-q")
        git(self.repo, "config", "user.name", "fixture")
        git(self.repo, "config", "user.email", "fixture@example.invalid")
        git(self.repo, "config", "gc.auto", "0")
        git(self.repo, "config", "maintenance.auto", "false")
        git(self.repo, "checkout", "-qb", "dd/astra-policy-v2")
        root = self.repo / "codex-rs"
        root.mkdir()
        (root / "codexdd-version.txt").write_text("0.4.2\n")
        (root / "upstream-codex-release.txt").write_text("rust-v0.159.2\n")
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "production")
        self.prod = git(self.repo, "rev-parse", "HEAD")
        self.prod_tree = git(self.repo, "rev-parse", "HEAD^{tree}")
        git(self.repo, "checkout", "-qb", "dd/codexdd-v0.4.3-upstream-0.162.0-integration")
        (root / "codexdd-version.txt").write_text("0.4.3\n")
        (root / "upstream-codex-release.txt").write_text("rust-v0.162.0\n")
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "merged source")
        self.source = git(self.repo, "rev-parse", "HEAD")
        self.source_tree = git(self.repo, "rev-parse", "HEAD^{tree}")

        old, conflicts, manifest, sidecar = fixtures()
        original = json.loads(old)
        original["production_sha"] = self.prod
        original["production_tree"] = self.prod_tree
        self.original_raw = canonical_json_bytes(original)
        self.conflicts_raw = conflicts
        self.old_digest = hashlib.sha256(self.original_raw).hexdigest()
        self.conflicts_digest = hashlib.sha256(conflicts).hexdigest()
        manifest["production_sha"] = self.prod
        manifest["production_tree"] = self.prod_tree
        manifest["source_snapshot_commit_sha"] = self.source
        manifest["source_snapshot_tree_sha"] = self.source_tree
        manifest["discovery_manifest_sha256"] = self.old_digest
        manifest["discovery_conflicts_sha256"] = self.conflicts_digest
        sidecar["production"]["sha"] = self.prod
        sidecar["production"]["tree"] = self.prod_tree
        sidecar["source_snapshot"]["commit_sha"] = self.source
        sidecar["source_snapshot"]["tree_sha"] = self.source_tree
        sidecar["discovery"]["manifest_sha256"] = self.old_digest
        sidecar["discovery"]["conflicts_sha256"] = self.conflicts_digest
        for entry in sidecar["generated_artifacts"]:
            entry["generator_source_sha"] = self.source
        for entry in sidecar["security_reviews"].values():
            entry["source_snapshot_commit_sha"] = self.source
        self.sidecar_bytes = canonical_json_bytes(sidecar)
        manifest["sidecar_sha256"] = hashlib.sha256(self.sidecar_bytes).hexdigest()
        self.manifest_bytes = canonical_json_bytes(manifest)
        self.manifest_path = "docs/upstream-candidates/codexdd-upstream-rust-v0.162.0.json"
        self.sidecar_path = manifest["sidecar_path"]
        git(self.repo, "checkout", "-qb", "dd/codexdd-v0.4.3-v2-attestation-staging", self.prod)
        docs = self.repo / "docs/upstream-candidates"
        docs.mkdir(parents=True)
        (self.repo / self.manifest_path).write_bytes(self.manifest_bytes)
        (self.repo / self.sidecar_path).write_bytes(self.sidecar_bytes)
        (docs / "original-blocked-v1-manifest.json").write_bytes(self.original_raw)
        (docs / "original-blocked-conflicts.txt").write_bytes(self.conflicts_raw)
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "staged attestation only")
        self.staging = git(self.repo, "rev-parse", "HEAD")

    def build(self, **changes):
        values = dict(
            production_sha=self.prod,
            source_sha=self.source,
            source_tree=self.source_tree,
            staging_sha=self.staging,
            manifest_path=self.manifest_path,
            approved_manifest_sha256=self.old_digest,
            approved_conflicts_sha256=self.conflicts_digest,
            target_tag=NEW_TAG,
            target_sha=TARGET_SHA,
        )
        values.update(changes)
        return construct_local_v2_candidate(self.repo, **values)

    def test_creates_candidate_git_object_with_exact_parent_and_no_ref_write(self):
        before_ref = git(self.repo, "rev-parse", "HEAD")
        sha, branch = self.build()
        self.assertEqual(git(self.repo, "rev-list", "--parents", "-n", "1", sha),
                         sha + " " + self.source)
        self.assertIn("automation/upstream-candidate-", branch)
        self.assertEqual(git(self.repo, "rev-parse", "HEAD"), before_ref)
        self.assertEqual(git(self.repo, "rev-parse", "dd/astra-policy-v2"), self.prod)

    def test_changed_approved_original_digest_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "digest"):
            self.build(approved_manifest_sha256="f" * 64)

    def test_staging_cannot_include_any_source_file(self):
        (self.repo / "codex-rs/codexdd-version.txt").write_text("0.4.5\n")
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "bad stage edit")
        with self.assertRaises(ValueError):
            self.build(staging_sha=git(self.repo, "rev-parse", "HEAD"))

    def test_wrong_approved_source_tree_fails(self):
        with self.assertRaisesRegex(ValueError, "source Git tree"):
            self.build(source_tree="f" * 40)

    def test_wrong_staging_parent_fails(self):
        with self.assertRaisesRegex(ValueError, "staging must be"):
            self.build(staging_sha=self.source)

    def test_wrong_target_identity_fails(self):
        with self.assertRaises(ValueError):
            self.build(target_sha="f" * 40)


if __name__ == "__main__":
    unittest.main()
