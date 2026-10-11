"""Standalone read-only v2 candidate preflight (never prepares or promotes).

Expected production/source IDs and original discovery SHA-256 hashes MUST come
from a separate accepted discovery/owner gate, not untrusted candidate files.
The promotion mode additionally checks the official upstream annotated tags.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

from upstream_reconciliation import canonical_repo_path
from upstream_reconciliation_git import (
    verify_metadata_only_candidate_git,
    verify_official_annotated_tags_git,
    verify_source_snapshot_git,
)
from upstream_reconciliation_v2 import validate_reconciled_v2_documents

_ORIGINAL_MANIFEST = "docs/upstream-candidates/original-blocked-v1-manifest.json"
_ORIGINAL_CONFLICTS = "docs/upstream-candidates/original-blocked-conflicts.txt"


def _git(repo: Path, *args: str) -> bytes:
    command = subprocess.run(
        ["git", "-C", str(repo), *args],
        check=False,
        capture_output=True,
        timeout=60,
    )
    if command.returncode != 0:
        raise ValueError(f"candidate preflight Git lookup failed: {args[0]}")
    return command.stdout


def _candidate_file(repo: Path, candidate_sha: str, path: str) -> bytes:
    safe = canonical_repo_path(path, prefix="docs/upstream-candidates")
    root = repo.resolve(strict=True)
    target = root.joinpath(*safe.split("/"))
    for ancestor in (target, *target.parents):
        if ancestor == root:
            break
        if ancestor.is_symlink():
            raise ValueError("candidate evidence path contains a symlink")
    if not target.is_file() or not target.resolve().is_relative_to(root):
        raise ValueError(f"missing or escaping candidate evidence file: {safe}")
    local = target.read_bytes()
    committed = _git(repo, "show", f"{candidate_sha}:{safe}")
    if local != committed:
        raise ValueError(f"candidate evidence has uncommitted byte drift: {safe}")
    return local


def verify_native_or_promotion_preflight(
    *,
    repo: Path,
    manifest_path: str,
    expected_production_sha: str,
    expected_source_sha: str,
    expected_source_tree: str,
    expected_target_tag: str,
    expected_target_sha: str,
    approved_manifest_sha256: str,
    approved_conflicts_sha256: str,
    promotion: bool,
    production_ref: str = "refs/remotes/origin/dd/astra-policy-v2",
    source_ref: str = "refs/remotes/origin/dd/codexdd-v0.4.3-upstream-0.162.0-integration",
) -> str:
    """Return the observed candidate SHA only after all read-only guards pass."""
    current_sha = _git(repo, "rev-parse", "HEAD").decode("ascii").strip()
    current_branch = _git(repo, "branch", "--show-current").decode("utf-8").strip()
    status = _git(repo, "status", "--porcelain")
    if status:
        raise ValueError("candidate worktree must be clean before preflight")

    manifest_raw = _candidate_file(repo, current_sha, manifest_path)
    from upstream_reconciliation import parse_strict_json

    v2_manifest = parse_strict_json(manifest_raw, canonical=True)
    if current_branch != v2_manifest.get("candidate_branch"):
        raise ValueError("not on canonical local candidate branch")
    sidecar_raw = _candidate_file(repo, current_sha, v2_manifest["sidecar_path"])
    original_raw = _candidate_file(repo, current_sha, _ORIGINAL_MANIFEST)
    conflicts_raw = _candidate_file(repo, current_sha, _ORIGINAL_CONFLICTS)
    manifest, _sidecar = validate_reconciled_v2_documents(
        original_manifest_bytes=original_raw,
        original_conflicts_bytes=conflicts_raw,
        approved_manifest_sha256=approved_manifest_sha256,
        approved_conflicts_sha256=approved_conflicts_sha256,
        reconciled_manifest_bytes=manifest_raw,
        reconciliation_sidecar_bytes=sidecar_raw,
        expected_production_sha=expected_production_sha,
        expected_target_tag=expected_target_tag,
        expected_target_sha=expected_target_sha,
        expected_source_snapshot_sha=expected_source_sha,
        expected_source_snapshot_tree=expected_source_tree,
    )
    verify_source_snapshot_git(repo, manifest, production_ref=production_ref, source_ref=source_ref)
    verify_metadata_only_candidate_git(
        repo,
        manifest,
        candidate_sha=current_sha,
        manifest_path=manifest_path,
        authorized_evidence_paths=(_ORIGINAL_MANIFEST, _ORIGINAL_CONFLICTS),
    )
    if promotion:
        verify_official_annotated_tags_git(repo, manifest)
    return current_sha


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, default=Path("."))
    parser.add_argument("--manifest", required=True)
    parser.add_argument("--production-sha", required=True)
    parser.add_argument("--source-sha", required=True)
    parser.add_argument("--source-tree", required=True)
    parser.add_argument("--target-tag", required=True)
    parser.add_argument("--target-sha", required=True)
    parser.add_argument("--approved-manifest-sha256", required=True)
    parser.add_argument("--approved-conflicts-sha256", required=True)
    parser.add_argument("--mode", required=True, choices=("native", "promotion"))
    args = parser.parse_args()

    try:
        actual = verify_native_or_promotion_preflight(
            repo=args.repo,
            manifest_path=args.manifest,
            expected_production_sha=args.production_sha,
            expected_source_sha=args.source_sha,
            expected_source_tree=args.source_tree,
            expected_target_tag=args.target_tag,
            expected_target_sha=args.target_sha,
            approved_manifest_sha256=args.approved_manifest_sha256,
            approved_conflicts_sha256=args.approved_conflicts_sha256,
            promotion=args.mode == "promotion",
        )
    except (ValueError, OSError, UnicodeError, KeyError, TypeError) as error:
        print(f"CODEXDD_RECONCILIATION_V2_PREFLIGHT_FAIL {error}", file=sys.stderr)
        return 2
    print(f"CODEXDD_RECONCILIATION_V2_PREFLIGHT_PASS {args.mode} {actual}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
