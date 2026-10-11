"""Read-only Git/object/metadata verification for conflicted upstream candidates.

Callers MUST first validate the typed v2 manifest/sidecar and MUST separately
require native Windows release-profile PASS and explicit owner authorization.
No function in this module pushes refs, changes production or opens PRs.
"""

from __future__ import annotations

import hashlib
import subprocess
from pathlib import Path

from upstream_reconciliation import (
    canonical_json_bytes,
    canonical_repo_path,
    parse_strict_json,
    require_sha,
)
from upstream_sync import stable_release_version

_PRODUCTION_REF = frozenset(
    {"refs/remotes/origin/dd/astra-policy-v2", "refs/heads/dd/astra-policy-v2"}
)
_SOURCE_REF = frozenset(
    {
        "refs/remotes/origin/dd/codexdd-v0.4.3-upstream-0.162.0-integration",
        "refs/heads/dd/codexdd-v0.4.3-upstream-0.162.0-integration",
    }
)
_OFFICIAL_URLS = frozenset(
    {"https://github.com/openai/codex.git", "https://github.com/openai/codex"}
)


def _git(repo: Path, *args: str) -> bytes:
    try:
        result = subprocess.run(
            ["git", "-C", str(repo), *args],
            capture_output=True,
            timeout=60,
            check=False,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        raise ValueError(
            f"Git identity verification infrastructure error: {args[0]}"
        ) from error
    if result.returncode:
        raise ValueError(
            f"Git identity verification failed ({args[0]}, code {result.returncode}): "
            + result.stderr.decode("utf-8", errors="replace")[:600]
        )
    return result.stdout


def _git_text(repo: Path, *args: str) -> str:
    return _git(repo, *args).decode("utf-8", errors="strict").strip()


def _commit_and_tree(repo: Path, sha: object, tree: object, label: str) -> None:
    require_sha(sha, name=f"{label} commit")
    require_sha(tree, name=f"{label} tree")
    if _git_text(repo, "cat-file", "-t", sha) != "commit":
        raise ValueError(f"{label} must name a commit object")
    if _git_text(repo, "rev-parse", f"{sha}^{{tree}}") != tree:
        raise ValueError(f"{label} Git object/tree SHA mismatch")


def _is_ancestor(repo: Path, parent: str, child: str) -> bool:
    r = subprocess.run(
        ["git", "-C", str(repo), "merge-base", "--is-ancestor", parent, child],
        capture_output=True,
        timeout=60,
        check=False,
    )
    if r.returncode == 0:
        return True
    if r.returncode == 1:
        return False
    raise ValueError("Git ancestry verification failed")


def verify_source_snapshot_git(
    repo: Path,
    manifest: dict[str, object],
    *,
    production_ref: str = "refs/remotes/origin/dd/astra-policy-v2",
    source_ref: str = "refs/remotes/origin/dd/codexdd-v0.4.3-upstream-0.162.0-integration",
) -> None:
    """Check live production, exact source Git objects and release markers.

    The production/source refs must both be fetched, not synthesized from
    self-declared values in the manifest. A moving ref fails closed.
    """
    if production_ref not in _PRODUCTION_REF or source_ref not in _SOURCE_REF:
        raise ValueError("untrusted release verification Git ref")
    production = manifest["production_sha"]
    snapshot = manifest["source_snapshot_commit_sha"]
    _commit_and_tree(repo, production, manifest["production_tree"], "production")
    _commit_and_tree(
        repo, snapshot, manifest["source_snapshot_tree_sha"], "source snapshot"
    )
    if _git_text(repo, "rev-parse", "--verify", production_ref) != production:
        raise ValueError("live production ref changed after discovery")
    if _git_text(repo, "rev-parse", "--verify", source_ref) != snapshot:
        raise ValueError("source branch moved after snapshot attestation")
    if not _is_ancestor(repo, production, snapshot):
        raise ValueError("source snapshot is not descended from live production")
    # No candidate can claim a target release while the source still reports
    # the old fork version or old upstream provenance.
    for path, expected in (
        ("codex-rs/codexdd-version.txt", manifest["codexdd_product_version"]),
        ("codex-rs/upstream-codex-release.txt", manifest["target_upstream_tag"]),
    ):
        actual = _git(repo, "show", f"{snapshot}:{path}").decode("utf-8").strip()
        if actual != expected:
            raise ValueError(f"source snapshot version/provenance mismatch at {path}")


def verify_official_annotated_tags_git(
    repo: Path,
    manifest: dict[str, object],
    *,
    expected_remote_url: str = "https://github.com/openai/codex.git",
) -> None:
    """Verify *official remote refs*, annotated tags, peeled commits and trees.

    Publishing workflow MUST use the default official remote, not override it.
    The expected_remote_url injection exists for isolated fixture tests only.
    """
    upstream_url = _git_text(repo, "remote", "get-url", "upstream")
    if expected_remote_url in _OFFICIAL_URLS:
        if upstream_url not in _OFFICIAL_URLS:
            raise ValueError("upstream Git remote is not official openai/codex")
    elif upstream_url != expected_remote_url:
        raise ValueError("upstream Git remote does not match fixture")
    for prefix in ("tracked", "target"):
        tag = manifest[f"{prefix}_upstream_tag"]
        if not isinstance(tag, str) or stable_release_version(tag) is None:
            raise ValueError(f"{prefix} tag is not stable")
        expected_tag_obj = require_sha(
            manifest[f"{prefix}_upstream_tag_object_sha"],
            name=f"{prefix} official annotated tag",
        )
        expected_commit = require_sha(
            manifest[f"{prefix}_upstream_sha"], name=f"{prefix} peeled commit"
        )
        expected_tree = require_sha(
            manifest[f"{prefix}_upstream_tree"], name=f"{prefix} peeled tree"
        )
        ref = f"refs/tags/{tag}"
        remote = _git_text(repo, "ls-remote", "--tags", "--refs", "upstream", ref)
        if remote != f"{expected_tag_obj}\t{ref}":
            raise ValueError(f"official upstream {prefix} annotated tag has moved")
        if _git_text(repo, "rev-parse", "--verify", ref) != expected_tag_obj:
            raise ValueError(f"local {prefix} tag differs from official remote")
        if _git_text(repo, "cat-file", "-t", expected_tag_obj) != "tag":
            raise ValueError(f"{prefix} tag object is not an annotated tag")
        if _git_text(repo, "rev-parse", f"{ref}^{{commit}}") != expected_commit:
            raise ValueError(f"{prefix} peeled commit differs from official target")
        if _git_text(repo, "rev-parse", f"{ref}^{{tree}}") != expected_tree:
            raise ValueError(f"{prefix} peeled tree differs from official target")


def verify_metadata_only_candidate_git(
    repo: Path,
    manifest: dict[str, object],
    *,
    candidate_sha: str,
    manifest_path: str,
    authorized_evidence_paths: tuple[str, ...] = (),
) -> None:
    """Verify a one-parent, metadata-*only* candidate over the source snapshot.

    The expected candidate SHA must be obtained from the actual candidate ref
    by the calling workflow. It is not sufficient to trust a receipt's SHA.
    """
    require_sha(candidate_sha, name="candidate commit")
    source_sha = require_sha(
        manifest["source_snapshot_commit_sha"], name="source snapshot commit"
    )
    if candidate_sha == source_sha:
        raise ValueError("candidate requires an explicit metadata-only commit")
    if _git_text(repo, "cat-file", "-t", candidate_sha) != "commit":
        raise ValueError("candidate must be a commit object")
    parents = _git_text(repo, "rev-list", "--parents", "-n", "1", candidate_sha).split()
    if parents != [candidate_sha, source_sha]:
        raise ValueError(
            "candidate must be an immediate single-parent source metadata commit"
        )
    main_path = canonical_repo_path(manifest_path, prefix="docs/upstream-candidates")
    sidecar_path = canonical_repo_path(
        manifest["sidecar_path"], prefix="docs/upstream-candidates"
    )
    if not main_path.endswith(".json") or not sidecar_path.endswith(".json"):
        raise ValueError("manifest and sidecar must be JSON files")
    if main_path == sidecar_path:
        raise ValueError("manifest cannot overwrite reconciliation sidecar")
    allowed = (main_path, sidecar_path) + tuple(authorized_evidence_paths)
    if len(allowed) != len(set(allowed)):
        raise ValueError("duplicate metadata-only candidate path")
    for path in allowed:
        canonical_repo_path(path, prefix="docs/upstream-candidates")

    raw = _git(
        repo, "diff", "--name-status", "--no-renames", "-z", source_sha, candidate_sha
    )
    changes = [
        chunk.decode("utf-8", errors="strict") for chunk in raw.split(b"\0") if chunk
    ]
    if len(changes) % 2 != 0:
        raise ValueError("malformed Git candidate metadata delta")
    found: dict[str, str] = {}
    for i in range(0, len(changes), 2):
        status, path = changes[i : i + 2]
        if path in found:
            raise ValueError("duplicate candidate metadata path")
        found[path] = status
    if set(found) != set(allowed) or any(value != "A" for value in found.values()):
        raise ValueError(
            "candidate contains source edits, removals, or unauthorized metadata"
        )
    for path in allowed:
        leaf = _git_text(repo, "ls-tree", "--full-tree", candidate_sha, "--", path)
        if not leaf.startswith("100644 blob ") or not leaf.endswith("\t" + path):
            raise ValueError("candidate metadata must be regular tracked Git blobs")

    data = _git(repo, "show", f"{candidate_sha}:{main_path}")
    if data != canonical_json_bytes(manifest):
        raise ValueError("committed candidate manifest bytes differ from attestation")
    parsed = parse_strict_json(data, canonical=True)
    if parsed != manifest:
        raise ValueError("candidate manifest was not canonical")
    sidecar = _git(repo, "show", f"{candidate_sha}:{sidecar_path}")
    parse_strict_json(sidecar, canonical=True)
    expected_digest = require_sha(
        manifest["sidecar_sha256"], sha256=True, name="sidecar digest"
    )
    if hashlib.sha256(sidecar).hexdigest() != expected_digest:
        raise ValueError("committed sidecar SHA-256 mismatch")
