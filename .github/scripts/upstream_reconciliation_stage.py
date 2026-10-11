"""Construct *local only* reconciled-candidate Git commit from approved metadata.

Requires a one-parent, metadata-only staging commit based on current production.
The only output is a local Git commit object. No refs are created or pushed.
The future publish workflow must independently verify refs, official tags,
semantic acceptance and duplicate-branch absence before its single guarded push.
"""

from __future__ import annotations

import os
import subprocess
import tempfile
from pathlib import Path

from upstream_reconciliation import canonical_repo_path, parse_strict_json, require_sha
from upstream_reconciliation_git import verify_metadata_only_candidate_git
from upstream_reconciliation_v2 import validate_reconciled_v2_documents
from upstream_sync import candidate_branch

_ORIGINAL_MANIFEST = "docs/upstream-candidates/original-blocked-v1-manifest.json"
_ORIGINAL_CONFLICTS = "docs/upstream-candidates/original-blocked-conflicts.txt"


def _git(repo: Path, *args: str, extra_env: dict[str, str] | None = None) -> bytes:
    env = dict(os.environ)
    if extra_env:
        env.update(extra_env)
    process = subprocess.run(
        ["git", "-C", str(repo), *args],
        env=env,
        capture_output=True,
        timeout=90,
        check=False,
    )
    if process.returncode:
        raise ValueError(
            f"local-only candidate construction failed at {args[0]}: "
            + process.stderr.decode("utf-8", errors="replace")[:600]
        )
    return process.stdout


def _commit(repo: Path, sha: object, *, expected_parent: str | None = None) -> None:
    require_sha(sha, name="staging/source commit")
    if _git(repo, "cat-file", "-t", sha).strip() != b"commit":
        raise ValueError("candidate input is not a Git commit object")
    if expected_parent:
        ancestors = _git(repo, "rev-list", "--parents", "-n", "1", sha).decode().split()
        if ancestors != [sha, expected_parent]:
            raise ValueError("staging must be exactly one metadata-only commit on production")


def _metadata_delta(
    repo: Path, *, production_sha: str, staging_sha: str, allowed: tuple[str, ...]
) -> None:
    raw = _git(repo, "diff", "--name-status", "--no-renames", "-z", production_sha, staging_sha)
    chunks = [x.decode("utf-8") for x in raw.split(b"\0") if x]
    if len(chunks) % 2:
        raise ValueError("malformed staging Git metadata delta")
    edits = dict(zip(chunks[::2], chunks[1::2]))
    if set(edits) != {"A"} or len(chunks) // 2 != len(allowed) or set(chunks[1::2]) != set(allowed):
        raise ValueError("staging contains unauthorized source edits or metadata paths")
    for path in allowed:
        leaf = _git(repo, "ls-tree", "--full-tree", staging_sha, "--", path).decode().strip()
        if not leaf.startswith("100644 blob ") or not leaf.endswith("\t" + path):
            raise ValueError("staging metadata must be regular Git blobs")


def construct_local_v2_candidate(
    repo: Path,
    *,
    production_sha: str,
    source_sha: str,
    source_tree: str,
    staging_sha: str,
    manifest_path: str,
    approved_manifest_sha256: str,
    approved_conflicts_sha256: str,
    target_tag: str,
    target_sha: str,
) -> tuple[str, str]:
    """Create only a local commit object, with no remote or ref side effects."""
    for name, value in (
        ("production", production_sha),
        ("source", source_sha),
        ("source tree", source_tree),
        ("staging", staging_sha),
        ("target", target_sha),
    ):
        require_sha(value, name=name)
    _commit(repo, production_sha)
    _commit(repo, source_sha)
    _git(repo, "merge-base", "--is-ancestor", production_sha, source_sha)
    _commit(repo, staging_sha, expected_parent=production_sha)
    actual_tree = _git(repo, "rev-parse", f"{source_sha}^{{tree}}").decode().strip()
    if actual_tree != source_tree:
        raise ValueError("source Git tree does not match approved snapshot")
    main = canonical_repo_path(manifest_path, prefix="docs/upstream-candidates")
    manifest_bytes = _git(repo, "show", f"{staging_sha}:{main}")
    staged_manifest = parse_strict_json(manifest_bytes, canonical=True)
    sidecar = canonical_repo_path(
        staged_manifest["sidecar_path"], prefix="docs/upstream-candidates"
    )
    paths = (main, sidecar, _ORIGINAL_MANIFEST, _ORIGINAL_CONFLICTS)
    if len(set(paths)) != 4:
        raise ValueError("staging evidence paths must be distinct")
    _metadata_delta(
        repo, production_sha=production_sha, staging_sha=staging_sha, allowed=paths
    )
    sidecar_bytes = _git(repo, "show", f"{staging_sha}:{sidecar}")
    original_bytes = _git(repo, "show", f"{staging_sha}:{_ORIGINAL_MANIFEST}")
    conflicts_bytes = _git(repo, "show", f"{staging_sha}:{_ORIGINAL_CONFLICTS}")
    manifest, _ = validate_reconciled_v2_documents(
        original_manifest_bytes=original_bytes,
        original_conflicts_bytes=conflicts_bytes,
        approved_manifest_sha256=approved_manifest_sha256,
        approved_conflicts_sha256=approved_conflicts_sha256,
        reconciled_manifest_bytes=manifest_bytes,
        reconciliation_sidecar_bytes=sidecar_bytes,
        expected_production_sha=production_sha,
        expected_target_tag=target_tag,
        expected_target_sha=target_sha,
        expected_source_snapshot_sha=source_sha,
        expected_source_snapshot_tree=source_tree,
    )
    expected_branch = candidate_branch(target_tag, target_sha)
    if manifest["candidate_branch"] != expected_branch:
        raise ValueError("staging manifest does not name the canonical candidate branch")

    with tempfile.TemporaryDirectory(prefix="codexdd-v2-index-") as root:
        index = str(Path(root) / "index")
        context = {"GIT_INDEX_FILE": index}
        _git(repo, "read-tree", source_sha, extra_env=context)
        for path in paths:
            blob = _git(repo, "rev-parse", f"{staging_sha}:{path}").decode().strip()
            _git(
                repo, "update-index", "--add", "--cacheinfo", f"100644,{blob},{path}",
                extra_env=context,
            )
        tree = _git(repo, "write-tree", extra_env=context).decode().strip()
    author = {
        "GIT_AUTHOR_NAME": "codexdd-v2-candidate-preparer",
        "GIT_AUTHOR_EMAIL": "codexdd-v2-preparer@example.invalid",
        "GIT_COMMITTER_NAME": "codexdd-v2-candidate-preparer",
        "GIT_COMMITTER_EMAIL": "codexdd-v2-preparer@example.invalid",
    }
    candidate_sha = _git(
        repo, "commit-tree", tree, "-p", source_sha,
        "-m", "codexdd: exact reconciled v2 candidate metadata",
        extra_env=author,
    ).decode().strip()
    verify_metadata_only_candidate_git(
        repo,
        manifest,
        candidate_sha=candidate_sha,
        manifest_path=main,
        authorized_evidence_paths=(_ORIGINAL_MANIFEST, _ORIGINAL_CONFLICTS),
    )
    return candidate_sha, expected_branch
