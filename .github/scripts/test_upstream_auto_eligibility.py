"""Packet 2A.2: read-only, default-deny updater planner regression matrix."""

import hashlib
import unittest
from dataclasses import replace
from unittest.mock import patch

import upstream_auto_eligibility as planner
from upstream_auto_eligibility import (
    CanonicalSnapshot,
    assess_eligibility,
)
from upstream_sync import build_candidate_manifest, candidate_manifest_json


def entries(path: str = "docs/changelog/overview.md"):
    a, b, c = ("a" * 40, "b" * 40, "c" * 40)
    tracked = {path: f"100644 blob {a}"}
    production = {path: f"100644 blob {a}", "codexdd/local.txt": f"100644 blob {b}"}
    upstream = {path: f"100644 blob {c}"}
    return tracked, production, upstream


def baseline(
    path: str = "docs/changelog/overview.md", *,
    transplant: str = "clean",
    modify_snapshot: bool = False,
):
    tracked, production, upstream = entries(path)
    p = dict(
        discovery_timestamp="2026-10-17T20:00:00Z",
        production_branch="dd/astra-policy-v2",
        production_sha="1" * 40,
        production_tree="2" * 40,
        codexdd_product_version="0.4.4",
        tracked_upstream_tag="rust-v0.162.0",
        tracked_upstream_sha="3" * 40,
        tracked_upstream_tree="4" * 40,
        target_upstream_tag="rust-v0.163.0",
        target_upstream_sha="5" * 40,
        target_upstream_tree="6" * 40,
        ancestry_status="diverged",
        merge_base_sha="7" * 40,
        tracked_entries=tracked,
        production_entries=production,
        target_entries=upstream,
        transplant_result=transplant,
    )
    manifest = build_candidate_manifest(**p)
    packages = {"rustls": "0.23.46", "quinn-proto": "0.11.18", "noyalib-serde-yaml": "0.0.57"}
    snapshot = CanonicalSnapshot(
        upstream_repository="openai/codex",
        production_branch=p["production_branch"],
        production_sha=p["production_sha"],
        production_tree=p["production_tree"],
        tracked_tag=p["tracked_upstream_tag"],
        tracked_sha=p["tracked_upstream_sha"],
        tracked_tree=p["tracked_upstream_tree"],
        target_tag=p["target_upstream_tag"],
        target_sha=p["target_upstream_sha"],
        target_tree=p["target_upstream_tree"],
        tag_kind="annotated",
        pinned_tag_oid="8" * 40,
        observed_tag_oid="8" * 40,
        tracked_entries=tracked,
        production_entries=production,
        target_entries=upstream,
        transplant_result=transplant,
        production_packages=packages,
        candidate_packages=packages,
    )
    sha = hashlib.sha256(candidate_manifest_json(manifest).encode("utf-8")).hexdigest()
    return manifest, snapshot, sha


def check(manifest, snapshot, digest):
    return assess_eligibility(manifest, pinned_manifest_sha256=digest, snapshot=snapshot)


class UpstreamAutoEligibilityTests(unittest.TestCase):
    def test_default_safe_class_is_empty_and_report_only(self):
        m, s, h = baseline()
        self.assertFalse(planner.REVIEWED_SAFE_UPSTREAM_PATHS)
        decision = check(m, s, h)
        self.assertEqual(decision.status, "manual_semantic_review_required")
        self.assertIn("upstream_path_not_allowlisted", decision.reasons)
        self.assertEqual(
            decision.candidate_branch,
            "automation/upstream-candidate-rust-v0.163.0-" + "5" * 12,
        )
        self.assertEqual(decision.canonical_json(), decision.canonical_json())

    def test_hypothetical_owner_reviewed_exact_path_can_pass_pure_planner(self):
        # This is a test-only policy injection; the shipped allowlist is EMPTY.
        path = "docs/changelog/overview.md"
        m, s, h = baseline(path)
        with patch.object(planner, "REVIEWED_SAFE_UPSTREAM_PATHS", frozenset({path})):
            decision = check(m, s, h)
        self.assertEqual(decision.status, "auto_preparation_ready")
        self.assertEqual(decision.reasons, ("read_only_policy_checks_passed",))

    def test_opaque_overlap_blocks_even_with_clean_transplant(self):
        path = "unreviewed/change.dat"
        m, s, h = baseline(path)
        old = dict(s.tracked_entries)
        prod = dict(s.production_entries)
        prod[path] = "100644 blob " + "d" * 40
        m = build_candidate_manifest(
            discovery_timestamp=m.discovery_timestamp,
            production_branch=m.production_branch,
            production_sha=m.production_sha,
            production_tree=m.production_tree,
            codexdd_product_version=m.codexdd_product_version,
            tracked_upstream_tag=m.tracked_upstream_tag,
            tracked_upstream_sha=m.tracked_upstream_sha,
            tracked_upstream_tree=m.tracked_upstream_tree,
            target_upstream_tag=m.target_upstream_tag,
            target_upstream_sha=m.target_upstream_sha,
            target_upstream_tree=m.target_upstream_tree,
            ancestry_status=m.ancestry_status,
            merge_base_sha=m.merge_base_sha,
            tracked_entries=old,
            production_entries=prod,
            target_entries=s.target_entries,
            transplant_result="clean",
        )
        s = replace(s, production_entries=prod)
        h = hashlib.sha256(candidate_manifest_json(m).encode()).hexdigest()
        with patch.object(planner, "REVIEWED_SAFE_UPSTREAM_PATHS", frozenset({path})):
            decision = check(m, s, h)
        self.assertEqual(decision.status, "manual_semantic_review_required")
        self.assertIn("overlap_requires_semantic_review", decision.reasons)
        self.assertIn("unclassified_overlap", decision.reasons)

    def test_real_conflict_not_relabelled_clean(self):
        m, s, h = baseline(transplant="conflict")
        decision = check(m, s, h)
        self.assertEqual(decision.status, "blocked_transplant_conflict")
        m2, s2, h2 = baseline()
        self.assertEqual(
            check(m2, replace(s2, transplant_result="conflict"), h2).status,
            "blocked_transplant_conflict",
        )
        self.assertEqual(
            check(m, replace(s, transplant_result="clean"), h).status,
            "blocked_transplant_conflict",
        )

    def test_not_run_and_invalid_transplant_never_pass(self):
        m, s, h = baseline(transplant="not_run")
        self.assertEqual(check(m, s, h).status, "blocked_incomplete")
        m, s, h = baseline()
        self.assertEqual(
            check(m, replace(s, transplant_result="ambiguous"), h).status,
            "blocked_incomplete",
        )

    def test_stale_production_head_tree_or_tracked_base_block(self):
        m, s, h = baseline()
        for field, replacement in (
            ("production_sha", "9" * 40),
            ("production_tree", "9" * 40),
            ("tracked_tag", "rust-v0.161.0"),
            ("tracked_sha", "9" * 40),
            ("tracked_tree", "9" * 40),
            ("production_branch", "not-production"),
        ):
            with self.subTest(field=field):
                self.assertEqual(
                    check(m, replace(s, **{field: replacement}), h).status,
                    "blocked_stale_base",
                )

    def test_upstream_tag_replacement_and_repo_spoof_block(self):
        m, s, h = baseline()
        for changed in (
            dict(pinned_tag_oid="9" * 40),
            dict(observed_tag_oid="9" * 40),
            dict(target_sha="9" * 40),
            dict(target_tree="9" * 40),
            dict(target_tag="rust-v0.164.0"),
            dict(upstream_repository="attacker/codex"),
            dict(tag_kind="unknown"),
            dict(tag_kind="lightweight"),
        ):
            with self.subTest(changed=changed):
                self.assertEqual(
                    check(m, replace(s, **changed), h).status, "blocked_identity"
                )

    def test_valid_lightweight_tag_pins_peeled_commit(self):
        m, s, h = baseline()
        lightweight = replace(s, tag_kind="lightweight", pinned_tag_oid="5" * 40, observed_tag_oid="5" * 40)
        with patch.object(
            planner, "REVIEWED_SAFE_UPSTREAM_PATHS", frozenset({"docs/changelog/overview.md"})
        ):
            self.assertEqual(check(m, lightweight, h).status, "auto_preparation_ready")
        self.assertEqual(
            check(m, replace(lightweight, observed_tag_oid="8" * 40), h).status,
            "blocked_identity",
        )

    def test_invalid_sha_and_manifest_checksum_are_rejected(self):
        m, s, h = baseline()
        self.assertEqual(check(m, s, "f" * 64).status, "blocked_identity")
        self.assertEqual(check(m, s, "bad-hash").status, "blocked_identity")
        self.assertEqual(
            check(m, replace(s, production_sha="garbage"), h).status,
            "blocked_identity",
        )

    def test_manifest_mutation_does_not_inherit_old_receipt(self):
        m, s, h = baseline()
        modified = replace(m, discovery_timestamp="mutated timestamp")
        self.assertEqual(check(modified, s, h).status, "blocked_identity")
        rehashed = hashlib.sha256(candidate_manifest_json(modified).encode()).hexdigest()
        self.assertEqual(check(replace(modified, overlap_path_count=2), s, rehashed).status, "blocked_identity")
        rehashed = hashlib.sha256(candidate_manifest_json(replace(m, candidate_key="wrong")).encode()).hexdigest()
        self.assertEqual(check(replace(m, candidate_key="wrong"), s, rehashed).status, "blocked_identity")

    def test_candidate_branch_pr_and_concurrent_publisher_are_duplicate(self):
        m, s, h = baseline()
        for flag in ("candidate_ref_exists", "active_candidate_pr_exists", "competing_preparation_exists"):
            with self.subTest(flag=flag):
                self.assertEqual(
                    check(m, replace(s, **{flag: True}), h).status,
                    "blocked_duplicate",
                )

    def test_protected_upstream_path_cannot_be_overridden_by_allowlist(self):
        for path in (
            ".github/workflows/ci.yml",
            "codex-rs/core/src/tools/registry.rs",
            "codex-rs/Cargo.lock",
            "sdk/python/src/openai_codex/generated/v2_all.py",
            "docs/AGENTS.md",
        ):
            m, s, h = baseline(path)
            with self.subTest(path=path), patch.object(
                planner, "REVIEWED_SAFE_UPSTREAM_PATHS", frozenset({path})
            ):
                decision = check(m, s, h)
                self.assertEqual(decision.status, "manual_semantic_review_required")
                self.assertIn("protected_upstream_delta", decision.reasons)

    def test_symlink_or_executable_mode_denied_even_if_path_allowlisted(self):
        path = "docs/changelog/overview.md"
        m, s, h = baseline(path)
        for mode in ("100755 blob", "120000 blob", "160000 commit"):
            changed = dict(s.target_entries)
            changed[path] = mode + " " + "c" * 40
            # New immutable discovery must agree with exact Git modes.
            updated = build_candidate_manifest(
                discovery_timestamp=m.discovery_timestamp,
                production_branch=m.production_branch,
                production_sha=m.production_sha,
                production_tree=m.production_tree,
                codexdd_product_version=m.codexdd_product_version,
                tracked_upstream_tag=m.tracked_upstream_tag,
                tracked_upstream_sha=m.tracked_upstream_sha,
                tracked_upstream_tree=m.tracked_upstream_tree,
                target_upstream_tag=m.target_upstream_tag,
                target_upstream_sha=m.target_upstream_sha,
                target_upstream_tree=m.target_upstream_tree,
                ancestry_status=m.ancestry_status,
                merge_base_sha=m.merge_base_sha,
                tracked_entries=s.tracked_entries,
                production_entries=s.production_entries,
                target_entries=changed,
                transplant_result="clean",
            )
            digest = hashlib.sha256(candidate_manifest_json(updated).encode()).hexdigest()
            with self.subTest(mode=mode), patch.object(
                planner, "REVIEWED_SAFE_UPSTREAM_PATHS", frozenset({path})
            ):
                decision = check(updated, replace(s, target_entries=changed), digest)
                self.assertEqual(decision.status, "manual_semantic_review_required")
                self.assertIn("upstream_nonregular_or_mode_delta", decision.reasons)

    def test_security_floor_no_yaml_replacement_and_no_dependency_drift(self):
        m, s, h = baseline()
        mutations = (
            ({"rustls": "0.23.45"}, "security_dependency_downgrade"),
            ({"quinn-proto": "0.11.15"}, "security_dependency_downgrade"),
            ({"noyalib-serde-yaml": "0.0.58"}, "security_dependency_replacement_changed"),
            ({"serde_yaml": "0.9.34"}, "legacy_yaml_dependency_restored"),
            ({"hyper": "1.11.1"}, "dependency_graph_changed"),
        )
        with patch.object(
            planner, "REVIEWED_SAFE_UPSTREAM_PATHS", frozenset({"docs/changelog/overview.md"})
        ):
            for changed, expected in mutations:
                with self.subTest(changed=changed):
                    candidate = dict(s.candidate_packages)
                    candidate.update(changed)
                    decision = check(m, replace(s, candidate_packages=candidate), h)
                    self.assertEqual(decision.status, "manual_semantic_review_required")
                    self.assertIn(expected, decision.reasons)

    def test_missing_dependency_snapshots_or_security_pin_block(self):
        m, s, h = baseline()
        self.assertIn(
            "missing_dependency_snapshot",
            check(m, replace(s, candidate_packages=None), h).reasons,
        )
        packages = dict(s.candidate_packages)
        del packages["rustls"]
        self.assertIn(
            "missing_security_dependency",
            check(m, replace(s, candidate_packages=packages), h).reasons,
        )

    def test_inconsistent_classifier_or_tree_inventories_fail_closed(self):
        m, s, h = baseline()
        modified = dict(s.target_entries)
        modified["other.txt"] = "100644 blob " + "e" * 40
        self.assertEqual(
            check(m, replace(s, target_entries=modified), h).status,
            "blocked_identity",
        )
        changed_manifest = replace(m, sensitive_overlap_count=2)
        digest = hashlib.sha256(candidate_manifest_json(changed_manifest).encode()).hexdigest()
        self.assertEqual(check(changed_manifest, s, digest).status, "blocked_identity")

    def test_no_changed_upstream_paths_cannot_fake_candidate(self):
        m, s, h = baseline()
        upstream = dict(s.tracked_entries)
        updated = build_candidate_manifest(
            discovery_timestamp=m.discovery_timestamp,
            production_branch=m.production_branch,
            production_sha=m.production_sha,
            production_tree=m.production_tree,
            codexdd_product_version=m.codexdd_product_version,
            tracked_upstream_tag=m.tracked_upstream_tag,
            tracked_upstream_sha=m.tracked_upstream_sha,
            tracked_upstream_tree=m.tracked_upstream_tree,
            target_upstream_tag=m.target_upstream_tag,
            target_upstream_sha=m.target_upstream_sha,
            target_upstream_tree=m.target_upstream_tree,
            ancestry_status=m.ancestry_status,
            merge_base_sha=m.merge_base_sha,
            tracked_entries=s.tracked_entries,
            production_entries=s.production_entries,
            target_entries=upstream,
            transplant_result="clean",
        )
        digest = hashlib.sha256(candidate_manifest_json(updated).encode()).hexdigest()
        result = check(updated, replace(s, target_entries=upstream), digest)
        self.assertEqual(result.status, "manual_semantic_review_required")
        self.assertIn("empty_upstream_delta", result.reasons)


if __name__ == "__main__":
    unittest.main()
