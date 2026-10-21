"""Read-only, default-deny upstream auto-preparation admission assessment.

Packet 2A.2: This module never fetches refs, launches commands, creates
branches, writes files, or opens pull requests. The later trusted recheck
adapter must independently derive every CanonicalSnapshot field from Git
before any result may be used by an isolated preparer. A planner result is
not an authorization to publish or an attestation of Windows validation.
"""

from __future__ import annotations

import hashlib
import json
import re
from dataclasses import asdict, dataclass
from typing import Mapping

from upstream_sync import (
    CandidateManifest,
    auto_preparation_overlap_reasons,
    build_delta_inventory,
    candidate_branch,
    candidate_key,
    candidate_manifest_json,
    candidate_state,
    stable_release_version,
)

OID = re.compile(r"^[0-9a-f]{40}$")
DIGEST = re.compile(r"^[0-9a-f]{64}$")
VERSION = re.compile(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$")

# No auto-publication path is approved in 2A.2. A subsequent owner-reviewed
# source change may add an EXACT path to this set after proving its safety.
# Never populate this policy from an artifact, workflow input, or PR content.
REVIEWED_SAFE_UPSTREAM_PATHS: frozenset[str] = frozenset()

PROTECTED_PREFIXES = (
    ".github/",
    ".cargo/",
    ".codex/",
    "codex-rs/",
    "sdk/",
    "scripts/",
    "packages/",
    "bazel/",
    "npm/",
)
PROTECTED_BASENAMES = frozenset(
    {
        "agents.md",
        "cargo.toml",
        "cargo.lock",
        "package.json",
        "package-lock.json",
        "pnpm-lock.yaml",
        "yarn.lock",
        "makefile",
        "justfile",
        "codexdd-version.txt",
        "upstream-codex-release.txt",
    }
)
SECURITY_FLOORS = {
    "rustls": (0, 23, 46),
    "quinn-proto": (0, 11, 18),
}
SECURITY_EXACT = {"noyalib-serde-yaml": (0, 0, 57)}


@dataclass(frozen=True)
class CanonicalSnapshot:
    """Independently rederived facts; NOT user-supplied manifest assertions.

    Tag object OIDs must be read from the official upstream remote both at
    the original discovery and this evaluation. The later recheck adapter
    must also verify that each tree-entry mapping matches its Git tree OID.
    """

    upstream_repository: str
    production_branch: str
    production_sha: str
    production_tree: str
    tracked_tag: str
    tracked_sha: str
    tracked_tree: str
    target_tag: str
    target_sha: str
    target_tree: str
    tag_kind: str
    pinned_tag_oid: str
    observed_tag_oid: str
    tracked_entries: Mapping[str, str]
    production_entries: Mapping[str, str]
    target_entries: Mapping[str, str]
    transplant_result: str
    production_packages: Mapping[str, str] | None = None
    candidate_packages: Mapping[str, str] | None = None
    candidate_ref_exists: bool = False
    active_candidate_pr_exists: bool = False
    competing_preparation_exists: bool = False


@dataclass(frozen=True)
class EligibilityDecision:
    status: str
    reasons: tuple[str, ...]
    candidate_key: str | None = None
    candidate_branch: str | None = None

    def canonical_json(self) -> str:
        return json.dumps(asdict(self), sort_keys=True, separators=(",", ":")) + "\n"


def _decision(status: str, *reasons: str, manifest: CandidateManifest | None = None) -> EligibilityDecision:
    if manifest is None:
        return EligibilityDecision(status, tuple(sorted(set(reasons))))
    # Identifiers are derived only from already-validated stable tag and OID.
    return EligibilityDecision(
        status,
        tuple(sorted(set(reasons))),
        candidate_key(manifest.target_upstream_tag, manifest.target_upstream_sha),
        candidate_branch(manifest.target_upstream_tag, manifest.target_upstream_sha),
    )


def _is_protected(path: str) -> bool:
    lower = path.replace("\\", "/").lower()
    basename = lower.rsplit("/", 1)[-1]
    return (
        lower.startswith(PROTECTED_PREFIXES)
        or basename in PROTECTED_BASENAMES
        or lower.endswith((".bzl", "/build.bazel", ".json.zst", ".py", ".rs", ".ts", ".yml", ".yaml"))
    )


def _version(raw: str | None) -> tuple[int, int, int] | None:
    if not isinstance(raw, str):
        return None
    match = VERSION.fullmatch(raw)
    if match is None:
        return None
    return tuple(int(part) for part in match.groups())


def _security_reasons(
    production: Mapping[str, str] | None, candidate: Mapping[str, str] | None
) -> tuple[str, ...]:
    if production is None or candidate is None:
        return ("missing_dependency_snapshot",)
    reasons: set[str] = set()
    for name, minimum in SECURITY_FLOORS.items():
        old = _version(production.get(name))
        new = _version(candidate.get(name))
        if old is None or new is None:
            reasons.add("missing_security_dependency")
        elif old < minimum or new < minimum or new < old:
            reasons.add("security_dependency_downgrade")
    for name, exact in SECURITY_EXACT.items():
        old = _version(production.get(name))
        new = _version(candidate.get(name))
        if old != exact or new != exact:
            reasons.add("security_dependency_replacement_changed")
    if "serde_yaml" in candidate:
        reasons.add("legacy_yaml_dependency_restored")
    # Until dependency graphs have an independently reviewed safe class,
    # every change of package/version (including upgrades) requires review.
    if dict(production) != dict(candidate):
        reasons.add("dependency_graph_changed")
    return tuple(sorted(reasons))


def _manifest_inventory_reasons(
    manifest: CandidateManifest, snapshot: CanonicalSnapshot
) -> tuple[str, ...]:
    inventory = build_delta_inventory(
        snapshot.tracked_entries,
        snapshot.production_entries,
        snapshot.target_entries,
    )
    mismatched = (
        manifest.customization_path_count != len(inventory.customization_paths)
        or manifest.upstream_change_path_count != len(inventory.upstream_change_paths)
        or manifest.overlap_path_count != len(inventory.overlap_paths)
        or manifest.overlap_paths != inventory.overlap_paths
        or manifest.sensitive_overlap_count != len(inventory.sensitive_overlap_paths)
        or manifest.sensitive_overlap_categories != inventory.sensitive_overlap_categories
    )
    return ("manifest_inventory_mismatch",) if mismatched else ()


def assess_eligibility(
    manifest: CandidateManifest,
    *,
    pinned_manifest_sha256: str,
    snapshot: CanonicalSnapshot,
) -> EligibilityDecision:
    """Evaluate independently checked evidence, without any side effects.

    This is a decision-support primitive. Its future caller MUST independently
    re-read official tag refs, peeled commit, real Git trees, production HEAD,
    candidate refs/PRs and dependencies, and MUST recheck them at write time.
    """

    expected_digest = hashlib.sha256(
        candidate_manifest_json(manifest).encode("utf-8")
    ).hexdigest()
    identity_errors: set[str] = set()
    if manifest.contract_version != 1:
        identity_errors.add("unsupported_manifest_contract")
    if DIGEST.fullmatch(pinned_manifest_sha256) is None or pinned_manifest_sha256 != expected_digest:
        identity_errors.add("manifest_digest_mismatch")

    pinned_tag = stable_release_version(manifest.target_upstream_tag)
    tracked_tag = stable_release_version(manifest.tracked_upstream_tag)
    if pinned_tag is None or tracked_tag is None:
        identity_errors.add("unstable_or_invalid_tag")
    elif pinned_tag <= tracked_tag:
        return _decision("up_to_date", "target_not_newer_than_tracked")
    if snapshot.upstream_repository != "openai/codex":
        identity_errors.add("untrusted_upstream_repository")
    if snapshot.tag_kind not in {"annotated", "lightweight"}:
        identity_errors.add("unknown_tag_object_kind")
    for oid in (
        manifest.production_sha,
        manifest.production_tree,
        manifest.tracked_upstream_sha,
        manifest.tracked_upstream_tree,
        manifest.target_upstream_sha,
        manifest.target_upstream_tree,
        snapshot.pinned_tag_oid,
        snapshot.observed_tag_oid,
        snapshot.production_sha,
        snapshot.production_tree,
        snapshot.tracked_sha,
        snapshot.tracked_tree,
        snapshot.target_sha,
        snapshot.target_tree,
    ):
        if not isinstance(oid, str) or OID.fullmatch(oid) is None:
            identity_errors.add("invalid_git_object_id")
    if snapshot.tag_kind == "lightweight" and snapshot.observed_tag_oid != snapshot.target_sha:
        identity_errors.add("invalid_lightweight_tag_identity")
    if snapshot.tag_kind == "annotated" and snapshot.observed_tag_oid == snapshot.target_sha:
        identity_errors.add("invalid_annotated_tag_identity")
    if snapshot.pinned_tag_oid != snapshot.observed_tag_oid:
        identity_errors.add("upstream_tag_object_moved")
    if snapshot.target_tag != manifest.target_upstream_tag:
        identity_errors.add("upstream_tag_name_changed")
    if (
        snapshot.target_sha != manifest.target_upstream_sha
        or snapshot.target_tree != manifest.target_upstream_tree
    ):
        identity_errors.add("upstream_peeled_target_changed")
    if manifest.candidate_key != candidate_key(manifest.target_upstream_tag, manifest.target_upstream_sha) if pinned_tag is not None and OID.fullmatch(manifest.target_upstream_sha) else False:
        identity_errors.add("candidate_key_mismatch")
    if identity_errors:
        return _decision("blocked_identity", *identity_errors)

    if (
        snapshot.production_branch != manifest.production_branch
        or snapshot.production_sha != manifest.production_sha
        or snapshot.production_tree != manifest.production_tree
        or snapshot.tracked_tag != manifest.tracked_upstream_tag
        or snapshot.tracked_sha != manifest.tracked_upstream_sha
        or snapshot.tracked_tree != manifest.tracked_upstream_tree
    ):
        return _decision("blocked_stale_base", "production_or_tracked_upstream_moved", manifest=manifest)

    # Validate every path set against the real trees, never trusting a
    # historical sensitive_overlap_count == 0 claim for auto-admission.
    if _manifest_inventory_reasons(manifest, snapshot):
        return _decision("blocked_identity", "manifest_inventory_mismatch", manifest=manifest)

    inventory = build_delta_inventory(
        snapshot.tracked_entries, snapshot.production_entries, snapshot.target_entries
    )
    if snapshot.transplant_result not in {"not_run", "clean", "conflict"}:
        return _decision("blocked_incomplete", "invalid_transplant_evidence", manifest=manifest)
    if manifest.transplant_dry_run == "conflict" or snapshot.transplant_result == "conflict":
        return _decision("blocked_transplant_conflict", "synthetic_transplant_conflicted", manifest=manifest)
    if manifest.transplant_dry_run != "clean" or snapshot.transplant_result != "clean":
        return _decision("blocked_incomplete", "clean_transplant_not_proven", manifest=manifest)
    expected_state = candidate_state(manifest.transplant_dry_run, manifest.sensitive_overlap_count)
    if manifest.state != expected_state:
        return _decision("blocked_identity", "manifest_state_inconsistent", manifest=manifest)

    if (
        snapshot.candidate_ref_exists
        or snapshot.active_candidate_pr_exists
        or snapshot.competing_preparation_exists
    ):
        return _decision("blocked_duplicate", "candidate_or_preparation_already_exists", manifest=manifest)

    reasons: set[str] = set(auto_preparation_overlap_reasons(inventory))
    changed = inventory.upstream_change_paths
    if not changed:
        reasons.add("empty_upstream_delta")
    if any(_is_protected(path) for path in changed):
        reasons.add("protected_upstream_delta")
    if any(path not in REVIEWED_SAFE_UPSTREAM_PATHS for path in changed):
        reasons.add("upstream_path_not_allowlisted")
    if any(
        snapshot.target_entries.get(path, "").split(" ", 2)[:2] != ["100644", "blob"]
        for path in changed
    ):
        reasons.add("upstream_nonregular_or_mode_delta")
    reasons.update(_security_reasons(snapshot.production_packages, snapshot.candidate_packages))
    if reasons:
        return _decision("manual_semantic_review_required", *reasons, manifest=manifest)
    return _decision("auto_preparation_ready", "read_only_policy_checks_passed", manifest=manifest)
