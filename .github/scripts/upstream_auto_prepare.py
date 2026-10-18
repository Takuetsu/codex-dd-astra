"""CodexDD 0.4.5 packet 2B.1: guarded isolated candidate construction.

This is source-only groundwork, NOT an enabled automatic publisher.
AUTO_PUBLISH_ENABLED is intentionally false until a separate owner-reviewed
admission policy and privileged-workflow acceptance. No scheduled workflow
imports or invokes this file.

The candidate tree is built from Git objects in a temporary *index*. Neither
upstream source nor its hooks, build steps, filters, or scripts are executed.
"""

from __future__ import annotations

import hashlib
import os
import re
import subprocess
import tempfile
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Mapping

import upstream_auto_report as report
from upstream_sync import (
    build_delta_inventory,
    candidate_branch,
    candidate_manifest_json,
    git_tree_entries,
)

AUTO_PUBLISH_ENABLED = False
PRODUCTION_BRANCH = "dd/astra-policy-v2"
CANDIDATE_AREA = "docs/upstream-candidates/"
SAFE_GIT_PATH = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._/-]*$")
WINDOWS_DEVICE = re.compile(r"^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\..*)?$", re.I)
OID = re.compile(r"^[0-9a-f]{40}$")


class PreparationBlocked(RuntimeError):
    """Fail closed without claiming a candidate was published."""


@dataclass(frozen=True)
class PreparedCandidate:
    candidate_branch: str
    candidate_key: str
    production_sha: str
    target_sha: str
    prepared_commit_sha: str
    prepared_tree_sha: str
    manifest_path: str
    manifest_sha256: str


def _git(
    repo: Path,
    *args: str,
    index: Path | None = None,
    data: bytes | None = None,
    metadata: dict[str, str] | None = None,
    preserve_output: bool = False,
) -> bytes:
    env = dict(os.environ)
    env.update({
        "GIT_CONFIG_NOSYSTEM": "1",
        "GIT_CONFIG_GLOBAL": os.devnull,
        "GIT_NO_REPLACE_OBJECTS": "1",
        "GIT_TERMINAL_PROMPT": "0",
    })
    if index is not None:
        env["GIT_INDEX_FILE"] = str(index)
    if metadata:
        env.update(metadata)
    completed = subprocess.run(
        ["git", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=" + os.devnull,
         *args],
        cwd=repo, env=env, input=data, check=True, capture_output=True,
    )
    return completed.stdout if preserve_output else completed.stdout.strip()


def _checked_path(path: str) -> str:
    # Caller-supplied artifacts cannot supply Git paths. Even future reviewed
    # paths must be simple, canonical and non-escaping.
    if (
        not isinstance(path, str)
        or SAFE_GIT_PATH.fullmatch(path) is None
        or path.startswith("/")
        or "\\" in path
        or "//" in path
        or any(
            part in {"", ".", ".."}
            or part.lower() == ".git"
            or part.endswith(".")
            or WINDOWS_DEVICE.fullmatch(part) is not None
            for part in path.split("/")
        )
    ):
        raise PreparationBlocked("unsafe_git_path")
    return path


def _verified_blob_change(
    path: str, tracked: Mapping[str, str], target: Mapping[str, str]
) -> str:
    _checked_path(path)
    old = tracked.get(path)
    # Even a normal-looking target blob cannot make an executable/symlink/
    # submodule mode transition safe on the Windows release platform.
    if old is not None and old.split(" ")[:2] != ["100644", "blob"]:
        raise PreparationBlocked("upstream_mode_transition")
    new = target.get(path)
    if new is None:
        raise PreparationBlocked("upstream_deletion_not_reviewed")
    parts = new.split(" ")
    if len(parts) != 3 or parts[:2] != ["100644", "blob"]:
        raise PreparationBlocked("upstream_nonregular_change")
    if OID.fullmatch(parts[2]) is None:
        raise PreparationBlocked("upstream_blob_invalid")
    return parts[2]


def _check_windows_collisions(paths: set[str]) -> None:
    # An ordinary trusted source tree contains .github/.gitattributes and may
    # have legitimate spaces; do NOT apply the much narrower NEW-path allowlist
    # grammar to unchanged files. Check actual Windows hazards instead.
    seen: dict[str, str] = {}
    for path in paths:
        if not isinstance(path, str) or path.startswith("/") or "\\" in path:
            raise PreparationBlocked("windows_path_unsafe")
        for part in path.split("/"):
            if (
                not part or part in {".", ".."}
                or part.lower() == ".git"
                or part.endswith((".", " "))
                or WINDOWS_DEVICE.fullmatch(part) is not None
            ):
                raise PreparationBlocked("windows_path_unsafe")
        folded = path.casefold()
        previous = seen.get(folded)
        if previous is not None and previous != path:
            raise PreparationBlocked("windows_path_case_collision")
        seen[folded] = path


def _manifest_path(tag: str, sha: str) -> str:
    if OID.fullmatch(sha) is None or report.stable_release_version(tag) is None:
        raise PreparationBlocked("invalid_candidate_identity")
    return CANDIDATE_AREA + "codexdd-upstream-" + tag + "-" + sha[:12] + ".json"


def _index_blob(repo: Path, index: Path, path: str, blob_sha: str) -> None:
    _checked_path(path)
    if OID.fullmatch(blob_sha) is None:
        raise PreparationBlocked("invalid_blob_sha")
    _git(repo, "update-index", "--add", "--cacheinfo", "100644", blob_sha, path,
         index=index)


def _add_bytes(repo: Path, index: Path, path: str, contents: bytes) -> str:
    sha = _git(repo, "hash-object", "-w", "--stdin", data=contents).decode("ascii")
    if OID.fullmatch(sha) is None:
        raise PreparationBlocked("invalid_hashed_blob")
    _index_blob(repo, index, path, sha)
    return sha


def _git_blob(repo: Path, treeish: str, path: str) -> bytes:
    _checked_path(path)
    if OID.fullmatch(treeish) is None:
        raise PreparationBlocked("invalid_treeish")
    return _git(repo, "show", treeish + ":" + path, preserve_output=True)


def _readiness(
    repo: Path,
    manifest_bytes: bytes,
    *,
    pinned_target_oid: str,
    pinned_tracked_oid: str,
    candidate_lock: bytes,
) -> dict[str, object]:
    # Never accept an artifact's own readiness claim as authority. The same
    # canonical tag, production, tree and candidate/PR checks are repeated
    # against live Git state, including after the local tree has been built.
    decision = report.evaluate(
        repo, manifest_bytes,
        pinned_target_oid=pinned_target_oid,
        pinned_tracked_oid=pinned_tracked_oid,
        observed_transplant="clean",
        conflicts=b"",
        candidate_lockfile=candidate_lock,
    )
    if (
        decision["status"] != "auto_preparation_ready"
        or decision["may_publish"] is not False
        or decision["report_only"] is not True
    ):
        raise PreparationBlocked(
            "eligibility_" + str(decision["status"])
        )
    return decision


def _commit_metadata(timestamp: str) -> dict[str, str]:
    try:
        instant = datetime.fromisoformat(timestamp.replace("Z", "+00:00"))
    except ValueError as exc:
        raise PreparationBlocked("invalid_discovery_timestamp") from exc
    if instant.tzinfo is None:
        raise PreparationBlocked("missing_discovery_timezone")
    seconds = int(instant.astimezone(timezone.utc).timestamp())
    if seconds <= 0:
        raise PreparationBlocked("invalid_discovery_timestamp")
    git_date = "@" + str(seconds) + " +0000"
    return {
        "GIT_AUTHOR_NAME": "codexdd-upstream-preparer",
        "GIT_AUTHOR_EMAIL": "codexdd-upstream-preparer@example.invalid",
        "GIT_COMMITTER_NAME": "codexdd-upstream-preparer",
        "GIT_COMMITTER_EMAIL": "codexdd-upstream-preparer@example.invalid",
        "GIT_AUTHOR_DATE": git_date,
        "GIT_COMMITTER_DATE": git_date,
    }


def build_isolated_candidate(
    repo: Path,
    manifest_bytes: bytes,
    *,
    pinned_target_oid: str,
    pinned_tracked_oid: str,
) -> PreparedCandidate:
    """Create an unreferenced, deterministic local commit; NEVER push or PR.

    Requires the trusted working checkout to be the exact production SHA,
    all complete immutable discovery evidence, and a future owner-reviewed
    safe allowlist. In the shipped 2B.1 policy the allowlist remains EMPTY.
    """
    repo = Path(repo).resolve()
    manifest = report._manifest(manifest_bytes)
    expected_digest = hashlib.sha256(manifest_bytes).hexdigest()
    manifest_file = _manifest_path(
        manifest.target_upstream_tag, manifest.target_upstream_sha
    )
    production_lock = _git_blob(
        repo, manifest.production_sha, "codex-rs/Cargo.lock"
    )
    first = _readiness(
        repo, manifest_bytes,
        pinned_target_oid=pinned_target_oid,
        pinned_tracked_oid=pinned_tracked_oid,
        candidate_lock=production_lock,
    )
    production = git_tree_entries(repo, manifest.production_sha)
    target = git_tree_entries(repo, manifest.target_upstream_sha)
    tracked = git_tree_entries(repo, manifest.tracked_upstream_sha)
    delta = build_delta_inventory(tracked, production, target)
    if delta.overlap_paths:
        raise PreparationBlocked("overlap_requires_semantic_review")
    if manifest_file in production or manifest_file in target:
        raise PreparationBlocked("duplicate_candidate_metadata")
    if first["candidate_branch"] != candidate_branch(
        manifest.target_upstream_tag, manifest.target_upstream_sha
    ):
        raise PreparationBlocked("candidate_identity_mismatch")
    marker = "codex-rs/upstream-codex-release.txt"
    if marker in delta.upstream_change_paths:
        raise PreparationBlocked("protected_release_marker_changed")
    _check_windows_collisions(
        set(production) | set(target) | {marker, manifest_file}
    )

    # Only Git object/index plumbing; no checkouts, external merge drivers,
    # cherry-picks, shell interpolation, filters, hook execution or builds.
    with tempfile.TemporaryDirectory(prefix="codexdd-safe-index-") as scratch:
        index = Path(scratch) / "index"
        _git(repo, "read-tree", manifest.production_tree, index=index)
        expected = dict(production)
        for path in delta.upstream_change_paths:
            blob = _verified_blob_change(path, tracked, target)
            _index_blob(repo, index, path, blob)
            expected[path] = target[path]

        marker_sha = _add_bytes(
            repo, index, marker,
            (manifest.target_upstream_tag + "\n").encode("ascii")
        )
        manifest_sha = _add_bytes(repo, index, manifest_file, manifest_bytes)
        expected[marker] = "100644 blob " + marker_sha
        expected[manifest_file] = "100644 blob " + manifest_sha
        staged_tree = _git(repo, "write-tree", index=index).decode("ascii")
        if OID.fullmatch(staged_tree) is None:
            raise PreparationBlocked("invalid_staged_tree")
        if git_tree_entries(repo, staged_tree) != expected:
            raise PreparationBlocked("staged_tree_content_mismatch")

    # This is an exact comparison of the synthesized tree's lockfile bytes,
    # not an assertion based on the earlier scratch/dry-run artifact.
    staged_lock = _git_blob(repo, staged_tree, "codex-rs/Cargo.lock")
    second = _readiness(
        repo, manifest_bytes,
        pinned_target_oid=pinned_target_oid,
        pinned_tracked_oid=pinned_tracked_oid,
        candidate_lock=staged_lock,
    )
    if second != first:
        raise PreparationBlocked("recheck_identity_changed")
    message = (
        "CodexDD: prepare " + manifest.target_upstream_tag + " isolated candidate\n\n"
        + "Original-Manifest-SHA256: " + expected_digest + "\n"
        + "Target-Commit: " + manifest.target_upstream_sha + "\n"
        + "Production-Commit: " + manifest.production_sha + "\n"
        + "Requires-Windows-And-Owner-Review: true\n"
    )
    commit = _git(
        repo, "commit-tree", staged_tree, "-p", manifest.production_sha,
        data=message.encode("utf-8"),
        metadata=_commit_metadata(manifest.discovery_timestamp),
    ).decode("ascii")
    if OID.fullmatch(commit) is None:
        raise PreparationBlocked("invalid_candidate_commit")
    if _git(repo, "rev-parse", commit + "^{tree}").decode() != staged_tree:
        raise PreparationBlocked("commit_tree_mismatch")
    parents = _git(repo, "rev-list", "--parents", "-n", "1", commit).decode().split()
    if parents != [commit, manifest.production_sha]:
        raise PreparationBlocked("candidate_parent_mismatch")
    if _git_blob(repo, staged_tree, manifest_file) != manifest_bytes:
        raise PreparationBlocked("manifest_bytes_changed")
    return PreparedCandidate(
        candidate_branch=str(first["candidate_branch"]),
        candidate_key=str(first["candidate_key"]),
        production_sha=manifest.production_sha,
        target_sha=manifest.target_upstream_sha,
        prepared_commit_sha=commit,
        prepared_tree_sha=staged_tree,
        manifest_path=manifest_file,
        manifest_sha256=expected_digest,
    )


def publish_isolated_candidate(
    repo: Path,
    prepared: PreparedCandidate,
    manifest_bytes: bytes,
    *,
    pinned_target_oid: str,
    pinned_tracked_oid: str,
) -> str:
    """Inert until a separately approved code change opens this gate.

    After authorization this still requires a complete final Git recheck,
    an exact prepared commit/tree/manifest and an absent-ref remote lease.
    It can NEVER merge, open a PR, install or modify the production ref.
    """
    if not AUTO_PUBLISH_ENABLED:
        raise PreparationBlocked("owner_publication_gate_closed")
    manifest = report._manifest(manifest_bytes)
    if (
        prepared.manifest_sha256 != hashlib.sha256(manifest_bytes).hexdigest()
        or prepared.production_sha != manifest.production_sha
        or prepared.target_sha != manifest.target_upstream_sha
        or prepared.candidate_branch != candidate_branch(
            manifest.target_upstream_tag, manifest.target_upstream_sha
        )
        or prepared.manifest_path != _manifest_path(
            manifest.target_upstream_tag, manifest.target_upstream_sha
        )
    ):
        raise PreparationBlocked("prepared_identity_mismatch")
    repo = Path(repo).resolve()
    if _git(repo, "rev-parse", prepared.prepared_commit_sha + "^{tree}").decode() != prepared.prepared_tree_sha:
        raise PreparationBlocked("prepared_commit_tree_mismatch")
    if _git_blob(repo, prepared.prepared_tree_sha, prepared.manifest_path) != manifest_bytes:
        raise PreparationBlocked("prepared_manifest_mismatch")
    parents = _git(repo, "rev-list", "--parents", "-n", "1",
                   prepared.prepared_commit_sha).decode().split()
    if parents != [prepared.prepared_commit_sha, prepared.production_sha]:
        raise PreparationBlocked("prepared_commit_parent_mismatch")
    actual_lock = _git_blob(repo, prepared.prepared_tree_sha, "codex-rs/Cargo.lock")
    _readiness(
        repo, manifest_bytes, pinned_target_oid=pinned_target_oid,
        pinned_tracked_oid=pinned_tracked_oid, candidate_lock=actual_lock
    )
    ref = "refs/heads/" + prepared.candidate_branch
    # An absent-ref lease is mandatory: no candidate rewrite under any race.
    _git(
        repo, "push", "--force-with-lease=" + ref + ":", "origin",
        prepared.prepared_commit_sha + ":" + ref,
    )
    return prepared.prepared_commit_sha
