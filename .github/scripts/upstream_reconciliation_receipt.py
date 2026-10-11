"""Strict receipt-v2 schema for manually reconciled *Windows release* candidates.

This read-only parser is not a candidate publisher, native host attestation,
or PR promotion authorization.  The publisher/promoter must first check the
blocked discovery, v2 sidecar and Git source/candidate proof independently.
"""

from __future__ import annotations

import hashlib
import re
from datetime import datetime, timezone
from pathlib import Path
from typing import Mapping

from upstream_reconciliation import (
    canonical_repo_path,
    parse_strict_json,
    require_fields,
    require_sha,
)
from upstream_sync import candidate_branch, candidate_key

_RECEIPT_FIELDS = frozenset(
    {
        "contract_version",
        "receipt_type",
        "candidate_key",
        "candidate_branch",
        "candidate_head_sha",
        "production_sha",
        "target_upstream_tag",
        "target_upstream_sha",
        "manifest_path",
        "manifest_sha256",
        "reconciliation_sidecar_path",
        "reconciliation_sidecar_sha256",
        "source_snapshot_commit_sha",
        "source_snapshot_tree_sha",
        "profile",
        "validation_contract_version",
        "validation_status",
        "completed_stages",
        "completed_at",
        "native_platform",
        "native_validation_host",
    }
)
_TIME = re.compile(
    r"\d{4}-\d\d-\d\dT\d\d:\d\d:\d\d(?:[.]\d{1,7})?(?:Z|[+]00:00)\Z"
)


def validate_v2_release_receipt(
    receipt_json: bytes,
    *,
    manifest_bytes: bytes,
    sidecar_bytes: bytes,
    expected_manifest_path: str,
    actual_candidate_sha: str,
) -> dict[str, object]:
    """Verify strict receipt fields and full digest/identity binding.

    This validates *claims* about native validation, not the origin or
    authenticity of the Daniel-CL machine. That requires operator acceptance.
    """
    candidate_sha = require_sha(actual_candidate_sha, name="observed candidate HEAD")
    path = canonical_repo_path(
        expected_manifest_path, prefix="docs/upstream-candidates"
    )
    manifest = parse_strict_json(manifest_bytes, canonical=True)
    if manifest.get("contract_version") != 2 or (
        manifest.get("state") != "manual_conflict_reconciled_review_required"
        or manifest.get("transplant_dry_run") != "conflict"
    ):
        raise ValueError("v2 receipt cannot promote a v1, clean or blocked candidate")
    sidecar = parse_strict_json(sidecar_bytes, canonical=True)
    if (
        sidecar.get("attestation_type") != "codexdd_upstream_conflict_reconciliation"
        or type(sidecar.get("contract_version")) is not int
        or sidecar["contract_version"] != 1
    ):
        raise ValueError("v2 receipt requires a typed conflict reconciliation sidecar")
    digest = require_sha(
        manifest.get("sidecar_sha256"), sha256=True, name="manifest sidecar digest"
    )
    if hashlib.sha256(sidecar_bytes).hexdigest() != digest:
        raise ValueError("manifest/sidecar checksum mismatch")

    receipt = parse_strict_json(receipt_json, canonical=True)
    require_fields(receipt, _RECEIPT_FIELDS, context="v2 Windows release receipt")
    for key, expected in (
        ("contract_version", 2),
        ("receipt_type", "codexdd_upstream_local_validation"),
        ("candidate_key", candidate_key(manifest["target_upstream_tag"],
                                        manifest["target_upstream_sha"])),
        ("candidate_branch", candidate_branch(manifest["target_upstream_tag"],
                                              manifest["target_upstream_sha"])),
        ("candidate_head_sha", candidate_sha),
        ("production_sha", manifest["production_sha"]),
        ("target_upstream_tag", manifest["target_upstream_tag"]),
        ("target_upstream_sha", manifest["target_upstream_sha"]),
        ("manifest_path", path),
        ("manifest_sha256", hashlib.sha256(manifest_bytes).hexdigest()),
        ("reconciliation_sidecar_path", manifest["sidecar_path"]),
        ("reconciliation_sidecar_sha256", digest),
        ("source_snapshot_commit_sha", manifest["source_snapshot_commit_sha"]),
        ("source_snapshot_tree_sha", manifest["source_snapshot_tree_sha"]),
        ("profile", "release"),
        ("validation_contract_version", 1),
        ("validation_status", "pass"),
        ("native_platform", "windows"),
        ("native_validation_host", "Daniel-CL"),
    ):
        if type(receipt[key]) is not type(expected) or receipt[key] != expected:
            raise ValueError(f"v2 native receipt identity/validation mismatch: {key}")
    for field in (
        "candidate_head_sha",
        "production_sha",
        "target_upstream_sha",
        "source_snapshot_commit_sha",
        "source_snapshot_tree_sha",
    ):
        require_sha(receipt[field], name=f"receipt {field}")
    for field in ("manifest_sha256", "reconciliation_sidecar_sha256"):
        require_sha(receipt[field], sha256=True, name=f"receipt {field}")
    if type(receipt["completed_stages"]) is not int or receipt["completed_stages"] < 21:
        raise ValueError("v2 release receipt missing complete native release stages")
    completed_at = receipt["completed_at"]
    if not isinstance(completed_at, str) or _TIME.fullmatch(completed_at) is None:
        raise ValueError("v2 native validation timestamp must be UTC ISO-8601")
    try:
        date = datetime.fromisoformat(completed_at.replace("Z", "+00:00"))
    except ValueError as error:
        raise ValueError("invalid v2 release completion timestamp") from error
    if date.utcoffset() != timezone.utc.utcoffset(None):
        raise ValueError("v2 native validation completion must be UTC")

    return receipt
