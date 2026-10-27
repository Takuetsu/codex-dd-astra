#!/usr/bin/env python3
"""Pin the CodexDD 0.4.5 AWS login exception to reviewed source only."""

from __future__ import annotations

import importlib.util
from pathlib import Path
from unittest import TestCase
from unittest.mock import patch
import subprocess

ROOT = Path(__file__).resolve().parents[2]
VERIFIER = ROOT / ".github/scripts/verify_cargo_workspace_manifests.py"
spec = importlib.util.spec_from_file_location("codexdd_manifest_verifier", VERIFIER)
assert spec is not None and spec.loader is not None
verifier = importlib.util.module_from_spec(spec)
spec.loader.exec_module(verifier)

KEY = "codex-rs/vendor/aws-config-1.13.0/Cargo.toml"


class ReviewedAwsManifestContractTests(TestCase):
    def test_exact_aws_vendor_git_tree_is_accepted(self):
        self.assertEqual(verifier.immutable_vendor_errors(KEY), [])

    def test_no_unpinned_external_manifest_skip(self):
        self.assertEqual(set(verifier.PINNED_EXTERNAL_VENDOR), {KEY})
        self.assertEqual(verifier.OPTIONAL_DEPENDENCY_EXCEPTIONS, set())

    def test_only_required_aws_login_feature_is_allowed(self):
        self.assertEqual(
            verifier.INTERNAL_DEPENDENCY_FEATURE_EXCEPTIONS,
            {
                ("codex-rs/aws-auth/Cargo.toml", "dependencies", "aws-config"): (
                    "credentials-login",
                ),
            },
        )
        manifest = verifier.load_manifest(ROOT / "codex-rs/aws-auth/Cargo.toml")
        self.assertEqual(
            manifest["dependencies"]["aws-config"]["features"],
            ["credentials-login"],
        )

    def test_reviewed_vendor_fingerprint_mismatch_fails_closed(self):
        result = subprocess.CompletedProcess([], 0, stdout="wrong-tree\n")
        clean = subprocess.CompletedProcess([], 0, stdout="")
        with patch.object(verifier.subprocess, "run", side_effect=[result, clean]):
            self.assertTrue(verifier.immutable_vendor_errors(KEY))

    def test_vendor_worktree_mutation_fails_closed(self):
        expected = verifier.PINNED_EXTERNAL_VENDOR[KEY][1]
        result = subprocess.CompletedProcess([], 0, stdout=expected + "\n")
        modified = subprocess.CompletedProcess([], 0, stdout=" M Cargo.toml\n")
        with patch.object(verifier.subprocess, "run", side_effect=[result, modified]):
            self.assertTrue(verifier.immutable_vendor_errors(KEY))
