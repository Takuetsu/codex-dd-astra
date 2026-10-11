"""Local temporary-Git-repository adversarial tests for v2 source/candidate integrity."""

import copy
import hashlib
import json
import subprocess
import tempfile
import unittest
from pathlib import Path

from upstream_reconciliation import canonical_json_bytes
from upstream_reconciliation_git import (
    verify_metadata_only_candidate_git,
    verify_source_snapshot_git,
)


def git(root: Path, *args: str) -> str:
    return subprocess.run(
        ["git", "-C", str(root), *args],
        check=True,
        text=True,
        capture_output=True,
    ).stdout.strip()


class CandidateGitBoundaryTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.repo = Path(self.temp.name)
        git(self.repo, "init", "-q")
        git(self.repo, "config", "user.name", "CodexDD verification fixture")
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
        git(
            self.repo,
            "checkout",
            "-qb",
            "dd/codexdd-v0.4.3-upstream-0.162.0-integration",
        )
        (root / "codexdd-version.txt").write_text("0.4.3\n")
        (root / "upstream-codex-release.txt").write_text("rust-v0.162.0\n")
        (root / "fixture.rs").write_text("fn test() {}\n")
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "reconciled source")
        self.snapshot = git(self.repo, "rev-parse", "HEAD")
        self.snapshot_tree = git(self.repo, "rev-parse", "HEAD^{tree}")
        self.manifest_path = "docs/upstream-candidates/codexdd-upstream.json"
        self.sidecar_path = "docs/upstream-candidates/codexdd-reconciliation.json"
        self.sidecar = {"contract_version": 1, "attestation_type": "fixture"}
        self.manifest = {
            "production_sha": self.production,
            "production_tree": self.production_tree,
            "codexdd_product_version": "0.4.3",
            "target_upstream_tag": "rust-v0.162.0",
            "source_snapshot_commit_sha": self.snapshot,
            "source_snapshot_tree_sha": self.snapshot_tree,
            "sidecar_path": self.sidecar_path,
            "sidecar_sha256": hashlib.sha256(
                canonical_json_bytes(self.sidecar)
            ).hexdigest(),
        }
        self.make_candidate()

    def make_candidate(self):
        git(self.repo, "checkout", self.snapshot)
        base = self.repo / "docs/upstream-candidates"
        base.mkdir(parents=True, exist_ok=True)
        (self.repo / self.manifest_path).write_bytes(
            canonical_json_bytes(self.manifest)
        )
        (self.repo / self.sidecar_path).write_bytes(canonical_json_bytes(self.sidecar))
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "metadata only")
        self.candidate = git(self.repo, "rev-parse", "HEAD")

    def verify_source(self, **kwargs):
        return verify_source_snapshot_git(
            self.repo,
            self.manifest,
            production_ref="refs/heads/dd/astra-policy-v2",
            source_ref="refs/heads/dd/codexdd-v0.4.3-upstream-0.162.0-integration",
            **kwargs,
        )

    def verify_candidate(self, **kwargs):
        parameters = {
            "candidate_sha": self.candidate,
            "manifest_path": self.manifest_path,
        }
        parameters.update(kwargs)
        return verify_metadata_only_candidate_git(
            self.repo, self.manifest, **parameters
        )

    def test_source_snapshot_and_metadata_only_candidate_pass(self):
        self.verify_source()
        self.verify_candidate()

    def test_stale_production_ref_is_rejected(self):
        old = self.production
        git(self.repo, "checkout", "dd/astra-policy-v2")
        (self.repo / "codex-rs/fixture.rs").write_text("changed\n")
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "production moved")
        with self.assertRaisesRegex(ValueError, "production ref changed"):
            self.verify_source()
        self.assertNotEqual(git(self.repo, "rev-parse", "HEAD"), old)

    def test_changed_source_branch_after_snapshot_is_rejected(self):
        git(self.repo, "checkout", "dd/codexdd-v0.4.3-upstream-0.162.0-integration")
        (self.repo / "codex-rs/fixture.rs").write_text("new source\n")
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "source moved")
        with self.assertRaisesRegex(ValueError, "source branch moved"):
            self.verify_source()

    def test_forged_snapshot_tree_is_rejected(self):
        self.manifest["source_snapshot_tree_sha"] = "f" * 40
        with self.assertRaisesRegex(ValueError, "tree SHA mismatch"):
            self.verify_source()

    def test_version_provenance_is_checked_at_source_commit(self):
        self.manifest["codexdd_product_version"] = "0.4.4"
        with self.assertRaisesRegex(ValueError, "version/provenance mismatch"):
            self.verify_source()

    def test_untrusted_ref_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "untrusted.*Git ref"):
            verify_source_snapshot_git(
                self.repo,
                self.manifest,
                production_ref="refs/heads/attacker",
                source_ref="refs/heads/dd/codexdd-v0.4.3-upstream-0.162.0-integration",
            )

    def test_extra_source_edit_in_candidate_metadata_is_rejected(self):
        git(self.repo, "checkout", self.snapshot)
        (self.repo / self.manifest_path).parent.mkdir(parents=True, exist_ok=True)
        (self.repo / self.manifest_path).write_bytes(
            canonical_json_bytes(self.manifest)
        )
        (self.repo / self.sidecar_path).write_bytes(canonical_json_bytes(self.sidecar))
        (self.repo / "codex-rs/fixture.rs").write_text("malicious edit\n")
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "candidate also changed product source")
        self.candidate = git(self.repo, "rev-parse", "HEAD")
        with self.assertRaisesRegex(ValueError, "source edits"):
            self.verify_candidate()

    def test_extra_metadata_file_is_rejected(self):
        git(self.repo, "checkout", self.snapshot)
        base = self.repo / "docs/upstream-candidates"
        base.mkdir(parents=True, exist_ok=True)
        (self.repo / self.manifest_path).write_bytes(
            canonical_json_bytes(self.manifest)
        )
        (self.repo / self.sidecar_path).write_bytes(canonical_json_bytes(self.sidecar))
        (base / "unexpected.json").write_text("{}\n")
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "extra side effects")
        self.candidate = git(self.repo, "rev-parse", "HEAD")
        with self.assertRaisesRegex(ValueError, "unauthorized metadata"):
            self.verify_candidate()

    def test_candidate_must_have_source_snapshot_as_direct_parent(self):
        git(self.repo, "checkout", self.candidate)
        (self.repo / "other.txt").write_text("later\n")
        git(self.repo, "add", ".")
        git(self.repo, "commit", "-qm", "post-attestation edit")
        self.candidate = git(self.repo, "rev-parse", "HEAD")
        with self.assertRaisesRegex(ValueError, "immediate single-parent"):
            self.verify_candidate()

    def test_committed_manifest_bytes_are_bound(self):
        self.manifest["codexdd_product_version"] = "0.4.4"
        with self.assertRaisesRegex(ValueError, "manifest bytes"):
            self.verify_candidate()

    def test_sidecar_digest_binding_rejects_tampering(self):
        self.manifest["sidecar_sha256"] = "f" * 64
        with self.assertRaisesRegex(ValueError, "manifest bytes"):
            self.verify_candidate()

    def test_candidate_ref_and_manifest_paths_are_not_interchangeable(self):
        with self.assertRaisesRegex(ValueError, "manifest cannot overwrite"):
            self.verify_candidate(manifest_path=self.sidecar_path)


if __name__ == "__main__":
    unittest.main()
