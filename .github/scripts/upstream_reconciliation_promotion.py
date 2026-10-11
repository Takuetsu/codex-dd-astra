"""Read-only v2 promotion receipt + owner review gate.

The executable GitHub workflow must independently perform promotion-mode Git
and upstream-tag preflight before invoking this parser. Passing this module
never creates a PR, moves a ref, merges, or installs.
"""

from __future__ import annotations

import hashlib
from pathlib import Path

from upstream_reconciliation import (
    canonical_repo_path,
    parse_strict_json,
    require_sha,
)
from upstream_reconciliation_receipt import validate_v2_release_receipt
from upstream_reconciliation_v2 import _url
from upstream_sync import candidate_branch


def validate_v2_owner_review_and_receipt(
    *,
    manifest_bytes: bytes,
    sidecar_bytes: bytes,
    receipt_bytes: bytes,
    observed_candidate_branch: str,
    observed_candidate_sha: str,
    manifest_path: str,
    semantic_review_url: str,
    owner_source_acceptance: str,
) -> dict[str, object]:
    """Require explicit owner acceptance of pinned source and full native receipt.

    The UI dispatch must be performed by the repository owner and recorded
    in the workflow audit log. A self-asserted Daniel-CL receipt is not
    cryptographically authenticated to that machine.
    """
    require_sha(observed_candidate_sha, name="observed remote candidate SHA")
    manifest_path = canonical_repo_path(
        manifest_path, prefix="docs/upstream-candidates"
    )
    manifest = parse_strict_json(manifest_bytes, canonical=True)
    sidecar = parse_strict_json(sidecar_bytes, canonical=True)
    if manifest.get("contract_version") != 2 or manifest.get("state") != (
        "manual_conflict_reconciled_review_required"
    ):
        raise ValueError("owner review cannot promote v1 or an unresolved candidate")
    source_sha = require_sha(
        manifest.get("source_snapshot_commit_sha"), name="reviewed source SHA"
    )
    target = manifest.get("target_upstream_tag")
    target_sha = manifest.get("target_upstream_sha")
    if target != "rust-v0.162.0" or target_sha != (
        "c1382380de69521303b416720a52f42d51af6248"
    ):
        raise ValueError("owner review cannot silently change the pinned upstream target")
    expected_candidate_branch = candidate_branch(target, target_sha)
    if observed_candidate_branch != expected_candidate_branch or (
        manifest.get("candidate_branch") != expected_candidate_branch
    ):
        raise ValueError("observed candidate branch differs from pinned target")

    _url(semantic_review_url, "owner semantic review")
    if sidecar.get("semantic_review_url") != semantic_review_url:
        raise ValueError("owner semantic review link does not match attested sidecar")
    if not (
        semantic_review_url.startswith(
            f"https://github.com/Takuetsu/codex-dd-astra/commit/{source_sha}"
        )
        or semantic_review_url.startswith(
            f"https://github.com/Takuetsu/codex-dd-astra/blob/{source_sha}/"
        )
        or semantic_review_url.startswith(
            "https://github.com/Takuetsu/codex-dd-astra/pull/"
        )
    ):
        raise ValueError("owner review must identify the frozen source or accepted PR")

    expected_acceptance = "ACCEPT_CODEXDD_0.4.3_RECONCILED_" + source_sha
    if owner_source_acceptance != expected_acceptance:
        raise ValueError("missing exact owner acceptance of frozen source snapshot")

    receipt = validate_v2_release_receipt(
        receipt_bytes,
        manifest_bytes=manifest_bytes,
        sidecar_bytes=sidecar_bytes,
        expected_manifest_path=manifest_path,
        actual_candidate_sha=observed_candidate_sha,
    )
    if receipt["source_snapshot_commit_sha"] != source_sha:
        raise ValueError("native receipt is stale relative to owner-accepted source")
    if receipt["manifest_sha256"] != hashlib.sha256(manifest_bytes).hexdigest():
        raise ValueError("native receipt manifest hash does not match committed evidence")
    return receipt
