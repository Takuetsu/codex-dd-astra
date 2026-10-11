"""Strict *read-only* v2 reconciled-candidate manifest and sidecar validation.

The v1 blocked discovery remains blocked.  These validations do not establish
official Git tag authenticity, inspect source Git objects, accept Windows
receipts, publish refs, or promote PRs.  Those are separate required gates.
"""

from __future__ import annotations

import hashlib
import re
from collections.abc import Mapping

from upstream_reconciliation import (
    canonical_repo_path,
    parse_strict_json,
    require_fields,
    require_sha,
    unique_repo_paths,
    verify_blocked_v1_discovery,
)
from upstream_sync import candidate_branch, candidate_key, stable_release_version

_MANIFEST = frozenset(
    {
        "contract_version",
        "state",
        "transplant_dry_run",
        "candidate_key",
        "candidate_branch",
        "production_branch",
        "production_sha",
        "production_tree",
        "codexdd_product_version",
        "tracked_upstream_tag",
        "tracked_upstream_tag_object_sha",
        "tracked_upstream_sha",
        "tracked_upstream_tree",
        "target_upstream_tag",
        "target_upstream_tag_object_sha",
        "target_upstream_sha",
        "target_upstream_tree",
        "discovery_run_id",
        "discovery_manifest_sha256",
        "discovery_conflicts_sha256",
        "overlap_paths",
        "sensitive_overlap_categories",
        "source_snapshot_commit_sha",
        "source_snapshot_tree_sha",
        "sidecar_path",
        "sidecar_sha256",
    }
)
_SIDECAR = frozenset(
    {
        "attestation_type",
        "contract_version",
        "candidate_key",
        "discovery",
        "production",
        "tracked_upstream",
        "target_upstream",
        "source_snapshot",
        "conflict_resolutions",
        "overlap_reviews",
        "generated_artifacts",
        "security_reviews",
        "semantic_review_url",
    }
)
_DISCOVERY = frozenset(
    {"run_id", "artifact_name", "manifest_sha256", "conflicts_sha256"}
)
_PRODUCTION = frozenset({"branch", "sha", "tree"})
_UPSTREAM = frozenset({"tag", "tag_object_sha", "sha", "tree"})
_SNAPSHOT = frozenset({"branch", "commit_sha", "tree_sha"})
_RESOLUTION = frozenset(
    {"path", "owner_packet", "strategy", "review_sha", "test_evidence_url", "semantic_evidence_url"}
)
_OVERLAP = frozenset(
    {
        "path",
        "owner_packet",
        "disposition",
        "sensitivity_categories",
        "test_evidence_url",
        "semantic_evidence_url",
    }
)
_GENERATED = frozenset(
    {"path", "generator_command", "generator_source_sha", "drift_evidence_url"}
)
_SECURITY = frozenset(
    {
        "windows_elevated_f2",
        "adaptive_model_routing",
        "worker_scope_and_lifecycle",
        "lvo_receipt",
        "updater_fail_closed",
        "resume_fork",
    }
)
_REVIEW = frozenset({"source_snapshot_commit_sha", "evidence_url"})
_GITHUB_EVIDENCE = re.compile(
    r"https://github[.]com/Takuetsu/codex-dd-astra/"
    r"(?:commit/[0-9a-f]{40}|pull/[1-9][0-9]*|actions/runs/[1-9][0-9]*"
    r"|blob/[0-9a-f]{40}/[A-Za-z0-9_./-]+)\Z"
)
_PACKET = re.compile(r"3[A-E](?:[.][0-9]+)*\Z")
_VERSION = re.compile(r"[0-9]+[.][0-9]+[.][0-9]+\Z")
_CATEGORY = re.compile(r"[a-z][a-z0-9_]*\Z")
_GENERATED_ROOT = (
    "codex-rs/app-server-protocol/schema/",
    "codex-rs/sdk/python/src/openai_codex/generated/",
    "sdk/python/src/openai_codex/generated/",
)


def _fields(value: object, names: frozenset[str], label: str) -> Mapping[str, object]:
    return require_fields(value, names, context=label)


def _same(actual: object, expected: object, label: str) -> None:
    if type(actual) is not type(expected) or actual != expected:
        raise ValueError(f"reconciliation binding mismatch: {label}")


def _positive(value: object, label: str) -> int:
    if type(value) is not int or value <= 0:
        raise ValueError(f"{label} must be a positive integer")
    return value


def _text(value: object, label: str) -> str:
    if not isinstance(value, str) or not value.strip() or value != value.strip():
        raise ValueError(f"{label} must be a nonblank canonical string")
    return value


def _url(value: object, label: str) -> str:
    if not isinstance(value, str) or _GITHUB_EVIDENCE.fullmatch(value) is None:
        raise ValueError(f"{label} must be an exact same-repository GitHub evidence URL")
    return value


def _packet(value: object) -> str:
    if not isinstance(value, str) or _PACKET.fullmatch(value) is None:
        raise ValueError("missing or invalid reconciliation packet owner")
    return value


def _categories(value: object) -> tuple[str, ...]:
    if type(value) is not list or not value:
        raise ValueError("each overlap requires explicit nonempty sensitivity review")
    categories: list[str] = []
    for category in value:
        if not isinstance(category, str) or _CATEGORY.fullmatch(category) is None:
            raise ValueError("invalid overlap sensitivity category")
        categories.append(category)
    if sorted(set(categories)) != categories:
        raise ValueError("overlap sensitivity categories must be sorted and unique")
    return tuple(categories)


def _list(value: object, label: str) -> list[object]:
    if type(value) is not list:
        raise ValueError(f"{label} must be an array")
    return value


def _require_provenance(
    obj: Mapping[str, object], *, tag: str, sha: str, tree: str, label: str
) -> None:
    if stable_release_version(tag) is None:
        raise ValueError(f"{label} official stable tag is invalid")
    _same(obj["tag"], tag, f"{label} tag")
    _same(obj["sha"], sha, f"{label} peeled SHA")
    _same(obj["tree"], tree, f"{label} tree")
    require_sha(obj["tag_object_sha"], name=f"{label} annotated tag object")


def validate_reconciled_v2_documents(
    *,
    original_manifest_bytes: bytes,
    original_conflicts_bytes: bytes,
    approved_manifest_sha256: str,
    approved_conflicts_sha256: str,
    reconciled_manifest_bytes: bytes,
    reconciliation_sidecar_bytes: bytes,
    expected_production_sha: str,
    expected_target_tag: str,
    expected_target_sha: str,
    expected_source_snapshot_sha: str,
    expected_source_snapshot_tree: str,
) -> tuple[dict[str, object], dict[str, object]]:
    """Validate every typed field against a *hash-pinned blocked* v1 discovery.

    Does not accept a release candidate; the caller must independently verify
    official Git tag refs, live production ancestry and source/tree identities.
    """
    for label, value in (
        ("production commit", expected_production_sha),
        ("target commit", expected_target_sha),
        ("source snapshot commit", expected_source_snapshot_sha),
        ("source snapshot tree", expected_source_snapshot_tree),
    ):
        require_sha(value, name=label)
    if stable_release_version(expected_target_tag) is None:
        raise ValueError("expected target must be an official stable version")

    original = parse_strict_json(original_manifest_bytes)
    overlap_paths = unique_repo_paths(original.get("overlap_paths"), name="original overlaps")
    conflict_paths = unique_repo_paths(
        original_conflicts_bytes.decode("utf-8", errors="strict").splitlines(),
        name="original conflicts",
    )
    if set(conflict_paths) - set(overlap_paths):
        raise ValueError("original conflicts must be contained in original overlaps")
    verify_blocked_v1_discovery(
        manifest_bytes=original_manifest_bytes,
        conflict_report_bytes=original_conflicts_bytes,
        manifest_sha256=approved_manifest_sha256,
        conflicts_sha256=approved_conflicts_sha256,
        expected_production_sha=expected_production_sha,
        expected_target_tag=expected_target_tag,
        expected_target_sha=expected_target_sha,
        expected_conflict_paths=conflict_paths,
    )
    if type(original.get("overlap_path_count")) is not int or (
        original["overlap_path_count"] != len(overlap_paths)
    ):
        raise ValueError("original overlap count does not match original paths")
    if overlap_paths != tuple(sorted(overlap_paths)):
        raise ValueError("original overlap list must be sorted")
    for field in (
        "production_tree",
        "tracked_upstream_tag",
        "tracked_upstream_sha",
        "tracked_upstream_tree",
        "target_upstream_tree",
    ):
        if field not in original:
            raise ValueError(f"missing source-of-truth v1 field: {field}")

    manifest = parse_strict_json(reconciled_manifest_bytes, canonical=True)
    sidecar = parse_strict_json(reconciliation_sidecar_bytes, canonical=True)
    _fields(manifest, _MANIFEST, "v2 candidate manifest")
    _fields(sidecar, _SIDECAR, "v1 reconciliation sidecar")
    _same(manifest["contract_version"], 2, "v2 contract version")
    _same(manifest["state"], "manual_conflict_reconciled_review_required", "candidate state")
    _same(manifest["transplant_dry_run"], "conflict", "immutable conflict outcome")
    _same(sidecar["contract_version"], 1, "sidecar contract version")
    _same(
        sidecar["attestation_type"],
        "codexdd_upstream_conflict_reconciliation",
        "sidecar attestation type",
    )
    _same(manifest["production_branch"], "dd/astra-policy-v2", "production branch")
    _same(manifest["production_sha"], expected_production_sha, "current production SHA")
    _same(manifest["source_snapshot_commit_sha"], expected_source_snapshot_sha, "snapshot SHA")
    _same(manifest["source_snapshot_tree_sha"], expected_source_snapshot_tree, "snapshot tree")

    for field in ("production_sha", "production_tree", "tracked_upstream_tag", "tracked_upstream_sha",
                  "tracked_upstream_tree", "target_upstream_tag", "target_upstream_sha",
                  "target_upstream_tree"):
        _same(manifest[field], original[field], f"v1 discovery {field}")
    for field in (
        "production_tree", "tracked_upstream_sha", "tracked_upstream_tree",
        "target_upstream_sha", "target_upstream_tree",
        "tracked_upstream_tag_object_sha", "target_upstream_tag_object_sha",
    ):
        require_sha(manifest[field], name=field)
    _same(manifest["target_upstream_tag"], expected_target_tag, "selected official tag")
    _same(manifest["target_upstream_sha"], expected_target_sha, "selected peeled commit")
    if stable_release_version(manifest["tracked_upstream_tag"]) is None:
        raise ValueError("tracked upstream tag is not stable")
    if not (
        stable_release_version(manifest["tracked_upstream_tag"])
        < stable_release_version(manifest["target_upstream_tag"])
    ):
        raise ValueError("target upstream must be newer than tracked upstream")
    if not isinstance(manifest["codexdd_product_version"], str) or (
        _VERSION.fullmatch(manifest["codexdd_product_version"]) is None
    ):
        raise ValueError("invalid CodexDD product version")
    if manifest["codexdd_product_version"] == original.get("codexdd_product_version"):
        raise ValueError("reconciled product version must be explicitly advanced")

    key = candidate_key(expected_target_tag, expected_target_sha)
    _same(manifest["candidate_key"], key, "candidate key")
    _same(manifest["candidate_branch"], candidate_branch(expected_target_tag, expected_target_sha),
          "canonical candidate branch")
    _same(sidecar["candidate_key"], key, "sidecar candidate key")
    _same(manifest["discovery_manifest_sha256"], approved_manifest_sha256,
          "original manifest digest")
    _same(manifest["discovery_conflicts_sha256"], approved_conflicts_sha256,
          "original conflicts digest")
    require_sha(manifest["sidecar_sha256"], sha256=True, name="reconciliation sidecar digest")
    if hashlib.sha256(reconciliation_sidecar_bytes).hexdigest() != manifest["sidecar_sha256"]:
        raise ValueError("v2 reconciliation sidecar digest mismatch")
    sidecar_path = canonical_repo_path(
        manifest["sidecar_path"], prefix="docs/upstream-candidates"
    )
    if not sidecar_path.endswith(".json"):
        raise ValueError("reconciliation sidecar must be JSON")
    _same(manifest["overlap_paths"], list(overlap_paths), "full original overlap inventory")

    discovery = _fields(sidecar["discovery"], _DISCOVERY, "discovery binding")
    _positive(discovery["run_id"], "discovery run ID")
    _same(manifest["discovery_run_id"], discovery["run_id"], "discovery run ID")
    _same(discovery["manifest_sha256"], approved_manifest_sha256, "discovery manifest digest")
    _same(discovery["conflicts_sha256"], approved_conflicts_sha256, "discovery conflicts digest")
    _text(discovery["artifact_name"], "original discovery artifact name")

    production = _fields(sidecar["production"], _PRODUCTION, "production binding")
    for field, value in (
        ("branch", manifest["production_branch"]),
        ("sha", manifest["production_sha"]),
        ("tree", manifest["production_tree"]),
    ):
        _same(production[field], value, f"sidecar production {field}")
    tracked = _fields(sidecar["tracked_upstream"], _UPSTREAM, "tracked upstream")
    target = _fields(sidecar["target_upstream"], _UPSTREAM, "target upstream")
    for which, obj in (("tracked", tracked), ("target", target)):
        _require_provenance(
            obj,
            tag=manifest[f"{which}_upstream_tag"],
            sha=manifest[f"{which}_upstream_sha"],
            tree=manifest[f"{which}_upstream_tree"],
            label=which,
        )
        _same(
            obj["tag_object_sha"], manifest[f"{which}_upstream_tag_object_sha"],
            f"{which} tag object",
        )
    snapshot = _fields(sidecar["source_snapshot"], _SNAPSHOT, "source snapshot")
    if snapshot["branch"] != "dd/codexdd-v0.4.3-upstream-0.162.0-integration":
        raise ValueError("unexpected source-development branch")
    _same(snapshot["commit_sha"], expected_source_snapshot_sha, "source commit")
    _same(snapshot["tree_sha"], expected_source_snapshot_tree, "source tree")

    categories = _fields(
        manifest["sensitive_overlap_categories"],
        frozenset(original.get("sensitive_overlap_categories", {})),
        "sensitive overlap categories",
    )
    originals = original.get("sensitive_overlap_categories")
    if type(originals) is not dict or not originals:
        raise ValueError("original sensitive overlap inventory missing")
    for category, paths in originals.items():
        if _CATEGORY.fullmatch(category) is None:
            raise ValueError("invalid original sensitivity category")
        if type(paths) is not list or len(set(paths)) != len(paths):
            raise ValueError("malformed original sensitivity inventory")
        _same(categories[category], paths, f"original sensitive category {category}")
        if set(paths) - set(overlap_paths):
            raise ValueError("original sensitivity contains nonoverlap path")

    conflicts = _list(sidecar["conflict_resolutions"], "conflict resolutions")
    if len(conflicts) != len(conflict_paths):
        raise ValueError("not all original textual conflicts have an owner and resolution")
    for index, record in enumerate(conflicts):
        entry = _fields(record, _RESOLUTION, "per-conflict resolution")
        _same(entry["path"], conflict_paths[index], "ordered original conflict path")
        _packet(entry["owner_packet"])
        if entry["strategy"] not in {"manual_merge", "generator_rebuilt", "upstream_supersedes"}:
            raise ValueError("invalid conflict resolution strategy")
        require_sha(entry["review_sha"], name="conflict source review commit")
        _url(entry["test_evidence_url"], "conflict test")
        _url(entry["semantic_evidence_url"], "conflict semantic review")

    overlaps = _list(sidecar["overlap_reviews"], "overlap reviews")
    if len(overlaps) != len(overlap_paths):
        raise ValueError("incomplete semantic overlap review")
    for index, record in enumerate(overlaps):
        entry = _fields(record, _OVERLAP, "per-overlap review")
        _same(entry["path"], overlap_paths[index], "ordered overlap path")
        _packet(entry["owner_packet"])
        if entry["disposition"] not in {"merged", "upstream_supersedes", "fork_preserved",
                                        "generator_rebuilt"}:
            raise ValueError("unreviewed overlap disposition")
        _url(entry["test_evidence_url"], "overlap tests")
        _url(entry["semantic_evidence_url"], "overlap semantic evidence")
        actual_categories = _categories(entry["sensitivity_categories"])
        for category, paths in originals.items():
            if entry["path"] in paths and category not in actual_categories:
                raise ValueError("overlap drops an original sensitive category")

    generated = _list(sidecar["generated_artifacts"], "generated artifact evidence")
    expected_generated = {
        path for path in overlap_paths if path.startswith(_GENERATED_ROOT)
        or path.endswith(".json.zst")
    }
    if len(generated) != len(expected_generated):
        raise ValueError("generated artifact provenance must cover every generated overlap")
    seen: set[str] = set()
    for record in generated:
        entry = _fields(record, _GENERATED, "generated artifact")
        path = canonical_repo_path(entry["path"])
        if path in seen or path not in expected_generated:
            raise ValueError("duplicate or unrecognized generated-artifact evidence")
        seen.add(path)
        if not _text(entry["generator_command"], "generator command").startswith(
            ("cargo ", "python", "pnpm ", "just ")
        ):
            raise ValueError("generator command is not an accepted tool family")
        require_sha(entry["generator_source_sha"], name="generated source commit")
        _same(entry["generator_source_sha"], expected_source_snapshot_sha,
              "generated artifact snapshot")
        _url(entry["drift_evidence_url"], "generator drift test")

    reviews = _fields(sidecar["security_reviews"], _SECURITY, "security review matrix")
    for name, review in reviews.items():
        item = _fields(review, _REVIEW, f"security review {name}")
        _same(item["source_snapshot_commit_sha"], expected_source_snapshot_sha,
              f"security review {name} source commit")
        _url(item["evidence_url"], f"security review {name}")
    _url(sidecar["semantic_review_url"], "owner source semantic review")

    return manifest, sidecar
