"""Helpers for guarded OpenAI Codex stable-release synchronization."""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Iterable, Mapping


RELEASE_TAG = re.compile(
    r"^rust-v(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)"
    r"(?:-(?P<kind>alpha|beta|rc)(?:\.(?P<parts>\d+(?:\.\d+)*))?)?$"
)
PRODUCT_VERSION = re.compile(r"^(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)$")
GIT_OBJECT_ID = re.compile(r"^[0-9a-f]{40}$")
KIND_ORDER = {"alpha": 0, "beta": 1, "rc": 2}

CANDIDATE_MANIFEST_CONTRACT_VERSION = 1
CANDIDATE_STATES = frozenset(
    {
        "up_to_date",
        "discovered",
        "blocked_identity",
        "blocked_stale_base",
        "blocked_duplicate",
        "blocked_transplant_conflict",
        "preparation_ready",
        "manual_semantic_review_required",
    }
)
TRANSPLANT_RESULTS = frozenset({"not_run", "clean", "conflict"})


@dataclass(frozen=True)
class DeltaInventory:
    customization_paths: tuple[str, ...]
    upstream_change_paths: tuple[str, ...]
    overlap_paths: tuple[str, ...]
    sensitive_overlap_categories: dict[str, tuple[str, ...]]

    @property
    def sensitive_overlap_paths(self) -> tuple[str, ...]:
        return tuple(
            sorted(
                {
                    path
                    for paths in self.sensitive_overlap_categories.values()
                    for path in paths
                }
            )
        )


@dataclass(frozen=True)
class CandidateManifest:
    contract_version: int
    candidate_key: str
    discovery_timestamp: str
    state: str
    production_branch: str
    production_sha: str
    production_tree: str
    codexdd_product_version: str
    tracked_upstream_tag: str
    tracked_upstream_sha: str
    tracked_upstream_tree: str
    target_upstream_tag: str
    target_upstream_sha: str
    target_upstream_tree: str
    ancestry_status: str
    merge_base_sha: str | None
    customization_path_count: int
    upstream_change_path_count: int
    overlap_path_count: int
    overlap_paths: tuple[str, ...]
    sensitive_overlap_count: int
    sensitive_overlap_categories: dict[str, tuple[str, ...]]
    transplant_dry_run: str

    def to_dict(self) -> dict[str, object]:
        return asdict(self)


def release_version(tag: str) -> tuple[int, ...] | None:
    match = RELEASE_TAG.fullmatch(tag)
    if match is None:
        return None
    base = tuple(int(match.group(name)) for name in ("major", "minor", "patch"))
    kind = match.group("kind")
    if kind is None:
        return (*base, 3)
    parts = tuple(int(part) for part in (match.group("parts") or "0").split("."))
    return (*base, KIND_ORDER[kind], *parts)


def stable_release_version(tag: str) -> tuple[int, int, int] | None:
    match = RELEASE_TAG.fullmatch(tag)
    if match is None or match.group("kind") is not None:
        return None
    return tuple(int(match.group(name)) for name in ("major", "minor", "patch"))


def product_version(version: str) -> tuple[int, int, int] | None:
    match = PRODUCT_VERSION.fullmatch(version)
    if match is None:
        return None
    return tuple(int(match.group(name)) for name in ("major", "minor", "patch"))


def next_patch_version(version: str) -> str:
    """Legacy preparation helper retained until the 2A.2 workflow migration."""

    parsed = product_version(version)
    if parsed is None:
        raise ValueError(f"invalid codexdd product version: {version}")
    major, minor, patch = parsed
    return f"{major}.{minor}.{patch + 1}"


def rewrite_version_test(source: str, current: str, next_version: str) -> str:
    """Legacy preparation helper retained until the 2A.2 workflow migration."""

    if product_version(current) is None:
        raise ValueError(f"invalid current codexdd product version: {current}")
    if product_version(next_version) is None:
        raise ValueError(f"invalid next codexdd product version: {next_version}")
    current_prefix = f"codexdd {current}+g"
    next_prefix = f"codexdd {next_version}+g"
    if source.count(current_prefix) != 1:
        raise ValueError(
            "version-reporting test must contain exactly one current codexdd version prefix"
        )
    return source.replace(current_prefix, next_prefix, 1)


def reconcile_workspace_lockfile(
    source: str, workspace_version: str
) -> tuple[str, int]:
    """Legacy preparation helper retained until the 2A.2 workflow migration."""

    if product_version(workspace_version) is None:
        raise ValueError(f"invalid workspace version: {workspace_version}")

    blocks = source.split("\n[[package]]\n")
    updated = [blocks[0]]
    changed = 0
    for block in blocks[1:]:
        if 'source = "' not in block:
            block, count = re.subn(
                r'(?m)^version = "[^"]+"$',
                f'version = "{workspace_version}"',
                block,
                count=1,
            )
            changed += count
        updated.append(block)

    if changed == 0:
        raise ValueError("no local workspace packages found in Cargo.lock")
    return "\n[[package]]\n".join(updated), changed


def select_latest_release(tags: Iterable[str]) -> str:
    candidates = [(stable_release_version(tag), tag) for tag in tags]
    valid = [(version, tag) for version, tag in candidates if version is not None]
    if not valid:
        raise ValueError("no stable official rust-vX.Y.Z release tags found")
    return max(valid)[1]


def integration_branch(tag: str) -> str:
    if stable_release_version(tag) is None:
        raise ValueError(f"invalid stable official release tag: {tag}")
    return f"automation/upstream-sync-{tag}"


def candidate_branch(tag: str, commit_sha: str) -> str:
    if stable_release_version(tag) is None:
        raise ValueError(f"invalid stable official release tag: {tag}")
    _validate_object_id(commit_sha, "target upstream commit")
    return f"automation/upstream-candidate-{tag}-{commit_sha[:12]}"


def pr_marker(tag: str) -> str:
    if stable_release_version(tag) is None:
        raise ValueError(f"invalid stable official release tag: {tag}")
    return f"codexdd-upstream-sync: {tag}"


def conflict_issue_title(tag: str) -> str:
    if stable_release_version(tag) is None:
        raise ValueError(f"invalid stable official release tag: {tag}")
    return f"codexdd: resolve upstream {tag} conflicts"


def conflict_marker(tag: str) -> str:
    if stable_release_version(tag) is None:
        raise ValueError(f"invalid stable official release tag: {tag}")
    return f"codexdd-upstream-conflict: {tag}"


def candidate_key(tag: str, commit_sha: str) -> str:
    if stable_release_version(tag) is None:
        raise ValueError(f"invalid stable official release tag: {tag}")
    _validate_object_id(commit_sha, "target upstream commit")
    return f"{tag}@{commit_sha}"


def tree_delta_paths(
    base: Mapping[str, str], other: Mapping[str, str]
) -> tuple[str, ...]:
    """Return exact sorted paths whose tree entry identity changed."""

    paths = set(base) | set(other)
    return tuple(sorted(path for path in paths if base.get(path) != other.get(path)))


def git_tree_entries(repo: Path, treeish: str) -> dict[str, str]:
    """Return exact recursive Git leaf entries keyed by repository path.

    The value includes mode, object type, and object id so mode-only changes are
    preserved in the delta inventory. NUL-delimited output avoids quoting/path
    ambiguity.
    """

    completed = subprocess.run(
        ["git", "ls-tree", "-rz", "--full-tree", treeish],
        cwd=repo,
        check=True,
        capture_output=True,
    )
    entries: dict[str, str] = {}
    for record in completed.stdout.split(b"\0"):
        if not record:
            continue
        metadata, separator, raw_path = record.partition(b"\t")
        if not separator:
            raise ValueError("malformed git ls-tree record")
        parts = metadata.decode("ascii").split()
        if len(parts) != 3:
            raise ValueError("malformed git ls-tree metadata")
        mode, object_type, object_id = parts
        path = raw_path.decode("utf-8", "surrogateescape")
        entries[path] = f"{mode} {object_type} {object_id}"
    return entries


def classify_sensitive_overlaps(
    paths: Iterable[str],
) -> dict[str, tuple[str, ...]]:
    """Classify overlap paths that require semantic CodexDD review."""

    categories: dict[str, set[str]] = {
        "adaptive_routing": set(),
        "worker_lifecycle": set(),
        "persistence_resume_fork": set(),
        "local_validation_orchestrator": set(),
        "status_rendering": set(),
        "generated_protocol": set(),
        "version_provenance": set(),
        "build_release_ci": set(),
    }

    for path in paths:
        normalized = path.replace("\\", "/")
        lower = normalized.lower()

        if (
            "/adaptive_" in lower
            or "adaptive_signal" in lower
            or "model_catalog" in lower
            or "model/catalog" in lower
        ):
            categories["adaptive_routing"].add(normalized)

        if (
            "agent/control" in lower
            or "adaptive_worker" in lower
            or "session_lifecycle" in lower
            or "thread_routing" in lower
            or "chatwidget/session_flow" in lower
            or "startup_orchestration" in lower
            or "worker" in lower
        ):
            categories["worker_lifecycle"].add(normalized)

        if (
            "/history/" in lower
            or "/rollout/" in lower
            or "/thread-store/" in lower
            or "workflow_state" in lower
            or "workflowstate" in lower
            or "/session/" in lower
            or "resume" in lower
            or "fork" in lower
        ):
            categories["persistence_resume_fork"].add(normalized)

        if (
            "run_codexdd_validation" in lower
            or "local_validation" in lower
            or "validation_profile" in lower
            or "codexdd-test-" in lower
            or "adaptive_validation" in lower
        ):
            categories["local_validation_orchestrator"].add(normalized)

        if (
            "/tui/src/status/" in lower
            or "status_and_layout" in lower
            or "/status/snapshots/" in lower
        ):
            categories["status_rendering"].add(normalized)

        if (
            "/app-server-protocol/schema/" in lower
            or "/sdk/python/src/openai_codex/generated/" in lower
            or "generated" in lower
            and ("protocol" in lower or "schema" in lower)
        ):
            categories["generated_protocol"].add(normalized)

        if (
            "codexdd-version.txt" in lower
            or "upstream-codex-release.txt" in lower
            or "version_reporting" in lower
            or "provenance" in lower
        ):
            categories["version_provenance"].add(normalized)

        if (
            lower.startswith(".github/")
            or lower.endswith("/cargo.lock")
            or lower.endswith("/cargo.toml")
            or lower == "defs.bzl"
            or lower.startswith("bazel/")
            or lower.startswith("module.bazel")
        ):
            categories["build_release_ci"].add(normalized)

    return {
        category: tuple(sorted(category_paths))
        for category, category_paths in categories.items()
        if category_paths
    }


def build_delta_inventory(
    tracked_upstream: Mapping[str, str],
    production: Mapping[str, str],
    target_upstream: Mapping[str, str],
) -> DeltaInventory:
    customization_paths = tree_delta_paths(tracked_upstream, production)
    upstream_change_paths = tree_delta_paths(tracked_upstream, target_upstream)
    upstream_change_set = set(upstream_change_paths)
    overlap_paths = tuple(
        path for path in customization_paths if path in upstream_change_set
    )
    return DeltaInventory(
        customization_paths=customization_paths,
        upstream_change_paths=upstream_change_paths,
        overlap_paths=overlap_paths,
        sensitive_overlap_categories=classify_sensitive_overlaps(overlap_paths),
    )


def candidate_state(transplant_result: str, sensitive_overlap_count: int) -> str:
    if transplant_result not in TRANSPLANT_RESULTS:
        raise ValueError(f"invalid transplant result: {transplant_result}")
    if sensitive_overlap_count < 0:
        raise ValueError("sensitive overlap count must be non-negative")
    if transplant_result == "not_run":
        return "discovered"
    if transplant_result == "conflict":
        return "blocked_transplant_conflict"
    if sensitive_overlap_count:
        return "manual_semantic_review_required"
    return "preparation_ready"


def build_candidate_manifest(
    *,
    discovery_timestamp: str,
    production_branch: str,
    production_sha: str,
    production_tree: str,
    codexdd_product_version: str,
    tracked_upstream_tag: str,
    tracked_upstream_sha: str,
    tracked_upstream_tree: str,
    target_upstream_tag: str,
    target_upstream_sha: str,
    target_upstream_tree: str,
    ancestry_status: str,
    merge_base_sha: str | None,
    tracked_entries: Mapping[str, str],
    production_entries: Mapping[str, str],
    target_entries: Mapping[str, str],
    transplant_result: str = "not_run",
) -> CandidateManifest:
    """Build an immutable, deterministic upstream candidate manifest."""

    if not discovery_timestamp.strip():
        raise ValueError("discovery timestamp is required")
    if not production_branch.strip():
        raise ValueError("production branch is required")
    if product_version(codexdd_product_version) is None:
        raise ValueError(f"invalid codexdd product version: {codexdd_product_version}")

    tracked_version = stable_release_version(tracked_upstream_tag)
    target_version = stable_release_version(target_upstream_tag)
    if tracked_version is None:
        raise ValueError(
            f"invalid tracked stable upstream release: {tracked_upstream_tag}"
        )
    if target_version is None:
        raise ValueError(
            f"invalid target stable upstream release: {target_upstream_tag}"
        )
    if target_version <= tracked_version:
        raise ValueError(
            "target upstream release must be strictly newer than tracked upstream"
        )

    for value, label in (
        (production_sha, "production commit"),
        (production_tree, "production tree"),
        (tracked_upstream_sha, "tracked upstream commit"),
        (tracked_upstream_tree, "tracked upstream tree"),
        (target_upstream_sha, "target upstream commit"),
        (target_upstream_tree, "target upstream tree"),
    ):
        _validate_object_id(value, label)
    if merge_base_sha is not None:
        _validate_object_id(merge_base_sha, "merge base commit")
    if not ancestry_status.strip():
        raise ValueError("ancestry status is required")

    inventory = build_delta_inventory(
        tracked_entries, production_entries, target_entries
    )
    sensitive_paths = inventory.sensitive_overlap_paths
    state = candidate_state(transplant_result, len(sensitive_paths))

    return CandidateManifest(
        contract_version=CANDIDATE_MANIFEST_CONTRACT_VERSION,
        candidate_key=candidate_key(target_upstream_tag, target_upstream_sha),
        discovery_timestamp=discovery_timestamp,
        state=state,
        production_branch=production_branch,
        production_sha=production_sha,
        production_tree=production_tree,
        codexdd_product_version=codexdd_product_version,
        tracked_upstream_tag=tracked_upstream_tag,
        tracked_upstream_sha=tracked_upstream_sha,
        tracked_upstream_tree=tracked_upstream_tree,
        target_upstream_tag=target_upstream_tag,
        target_upstream_sha=target_upstream_sha,
        target_upstream_tree=target_upstream_tree,
        ancestry_status=ancestry_status,
        merge_base_sha=merge_base_sha,
        customization_path_count=len(inventory.customization_paths),
        upstream_change_path_count=len(inventory.upstream_change_paths),
        overlap_path_count=len(inventory.overlap_paths),
        overlap_paths=inventory.overlap_paths,
        sensitive_overlap_count=len(sensitive_paths),
        sensitive_overlap_categories=inventory.sensitive_overlap_categories,
        transplant_dry_run=transplant_result,
    )


def candidate_manifest_json(manifest: CandidateManifest) -> str:
    """Serialize a manifest deterministically for durable evidence."""

    return json.dumps(
        manifest.to_dict(),
        sort_keys=True,
        separators=(",", ":"),
    ) + "\n"


def stale_identity_fields(
    manifest: CandidateManifest,
    *,
    production_sha: str,
    tracked_upstream_tag: str,
    tracked_upstream_sha: str,
    target_upstream_tag: str,
    target_upstream_sha: str,
) -> tuple[str, ...]:
    """Return manifest identity fields that no longer match current state."""

    current = {
        "production_sha": production_sha,
        "tracked_upstream_tag": tracked_upstream_tag,
        "tracked_upstream_sha": tracked_upstream_sha,
        "target_upstream_tag": target_upstream_tag,
        "target_upstream_sha": target_upstream_sha,
    }
    stale = [
        field
        for field, value in current.items()
        if getattr(manifest, field) != value
    ]
    return tuple(sorted(stale))


def candidate_state_after_identity_check(
    manifest: CandidateManifest,
    *,
    production_sha: str,
    tracked_upstream_tag: str,
    tracked_upstream_sha: str,
    target_upstream_tag: str,
    target_upstream_sha: str,
) -> tuple[str, tuple[str, ...]]:
    stale = stale_identity_fields(
        manifest,
        production_sha=production_sha,
        tracked_upstream_tag=tracked_upstream_tag,
        tracked_upstream_sha=tracked_upstream_sha,
        target_upstream_tag=target_upstream_tag,
        target_upstream_sha=target_upstream_sha,
    )
    if stale:
        return "blocked_stale_base", stale
    return manifest.state, ()


def report_conflict_issue(
    body_path: Path, summary_path: Path, repository: str, title: str
) -> bool:
    """Publish primary conflict details before best-effort GitHub Issue reporting."""

    with summary_path.open("a", encoding="utf-8") as summary:
        summary.write(body_path.read_text(encoding="utf-8"))
        summary.write("\n")

    try:
        listed = subprocess.run(
            [
                "gh",
                "issue",
                "list",
                "--repo",
                repository,
                "--state",
                "open",
                "--limit",
                "100",
                "--json",
                "number,title",
            ],
            check=True,
            capture_output=True,
            text=True,
        )
        issues = json.loads(listed.stdout)
        existing = next(
            (item["number"] for item in issues if item["title"] == title), None
        )
        if existing is None:
            command = [
                "gh",
                "issue",
                "create",
                "--repo",
                repository,
                "--title",
                title,
                "--body-file",
                str(body_path),
            ]
        else:
            command = [
                "gh",
                "issue",
                "edit",
                str(existing),
                "--repo",
                repository,
                "--body-file",
                str(body_path),
            ]
        subprocess.run(command, check=True, capture_output=True, text=True)
    except (
        OSError,
        subprocess.CalledProcessError,
        ValueError,
        KeyError,
        TypeError,
    ) as error:
        detail = getattr(error, "stderr", None) or str(error)
        secondary = (
            f"Secondary diagnostic: GitHub Issue reporting failed: {detail.strip()}"
        )
        print(secondary, file=sys.stderr)
        with summary_path.open("a", encoding="utf-8") as summary:
            summary.write(f"\n{secondary}\n")
        return False
    return True


def _validate_object_id(value: str, label: str) -> None:
    if GIT_OBJECT_ID.fullmatch(value) is None:
        raise ValueError(f"invalid {label} SHA: {value}")


def main() -> int:
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--latest-from-stdin", action="store_true")
    group.add_argument("--next-patch")
    group.add_argument("--report-conflict", type=Path)
    args = parser.parse_args()
    try:
        if args.latest_from_stdin:
            print(
                select_latest_release(
                    line.strip() for line in sys.stdin if line.strip()
                )
            )
        elif args.report_conflict is not None:
            return (
                0
                if report_conflict_issue(
                    args.report_conflict,
                    Path(os.environ["GITHUB_STEP_SUMMARY"]),
                    os.environ["GITHUB_REPOSITORY"],
                    os.environ["CONFLICT_TITLE"],
                )
                else 1
            )
        else:
            print(next_patch_version(args.next_patch))
    except ValueError as error:
        print(error, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
