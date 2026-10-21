"""CodexDD 0.4.5 packet 2B.2: disabled, review-only draft-PR adapter.

The source gate is deliberately OFF. No Actions job imports or executes this
adapter. A future owner-approved route may propose an explicitly UNVALIDATED
draft PR; GitHub Actions may forbid PR creation despite a write permission.
Any denial produces a safe handoff, never a permission bypass or auto-merge.
"""

from __future__ import annotations

import hashlib
import json
import os
import re
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path

import upstream_auto_eligibility as policy
import upstream_auto_prepare as preparer
import upstream_auto_report as report
from upstream_sync import (
    build_delta_inventory,
    candidate_branch,
    candidate_key,
    git_tree_entries,
)

DRAFT_PR_WRITES_ENABLED = False
REPOSITORY = "Takuetsu/codex-dd-astra"
BASE_BRANCH = "dd/astra-policy-v2"
GITHUB_ACTIONS_FORBIDDEN = "github actions is not permitted to create or approve pull requests"
PR_URL = re.compile(r"^https://github\.com/Takuetsu/codex-dd-astra/pull/[1-9][0-9]*$")
SAFE_OID = re.compile(r"^[0-9a-f]{40}$")
SAFE_DIGEST = re.compile(r"^[0-9a-f]{64}$")


class DraftPRBlocked(RuntimeError):
    """No PR was authorized or its exact identity could not be proved."""


@dataclass(frozen=True)
class DraftPRPlan:
    candidate_branch: str
    candidate_sha: str
    production_sha: str
    target_sha: str
    target_tag: str
    manifest_path: str
    manifest_sha256: str
    title: str
    body: str


@dataclass(frozen=True)
class DraftPRResult:
    status: str
    reason: str
    candidate_branch: str
    candidate_sha: str
    pr_url: str | None = None

    def canonical_json(self) -> str:
        from dataclasses import asdict

        return json.dumps(asdict(self), sort_keys=True, separators=(",", ":")) + "\n"


def _result(plan: DraftPRPlan, status: str, reason: str, url: str | None = None) -> DraftPRResult:
    return DraftPRResult(
        status=status, reason=reason,
        candidate_branch=plan.candidate_branch,
        candidate_sha=plan.candidate_sha, pr_url=url,
    )


def draft_plan(
    prepared: preparer.PreparedCandidate,
    manifest_bytes: bytes,
) -> DraftPRPlan:
    """Format a proposed PR from canonical immutable source evidence only.

    This function does not claim a published branch, accepted Windows receipt,
    semantic owner acceptance or permission to open a PR.
    """
    manifest = report._manifest(manifest_bytes)
    tag = manifest.target_upstream_tag
    target = manifest.target_upstream_sha
    expected_path = preparer._manifest_path(tag, target)
    identity = (
        manifest.contract_version == 1
        and manifest.transplant_dry_run == "clean"
        and manifest.state == "preparation_ready"
        and manifest.overlap_path_count == 0
        and manifest.sensitive_overlap_count == 0
        and manifest.overlap_paths == ()
        and manifest.sensitive_overlap_categories == {}
        and prepared.candidate_branch == candidate_branch(tag, target)
        and prepared.candidate_key == candidate_key(tag, target)
        and prepared.target_sha == target
        and prepared.production_sha == manifest.production_sha
        and prepared.manifest_path == expected_path
        and prepared.manifest_sha256 == hashlib.sha256(manifest_bytes).hexdigest()
        and SAFE_OID.fullmatch(prepared.prepared_commit_sha) is not None
        and SAFE_OID.fullmatch(prepared.prepared_tree_sha) is not None
        and SAFE_DIGEST.fullmatch(prepared.manifest_sha256) is not None
    )
    if not identity:
        raise DraftPRBlocked("inconsistent_candidate_provenance")

    title = "[UNVALIDATED] CodexDD upstream " + tag + " candidate"
    body = (
        "## UNVALIDATED / NEEDS_WINDOWS_AND_OWNER_REVIEW\n\n"
        "An isolated candidate was prepared from official upstream Git objects. "
        "This draft is **not** a release, source approval, native validation "
        "receipt or authorization to merge.\n\n"
        "### Exact candidate identity\n\n"
        + "- Candidate branch: \x60" + prepared.candidate_branch + "\x60\n"
        + "- Candidate HEAD: \x60" + prepared.prepared_commit_sha + "\x60\n"
        + "- Candidate tree: \x60" + prepared.prepared_tree_sha + "\x60\n"
        + "- Production base: \x60" + prepared.production_sha + "\x60\n"
        + "- Official upstream target: \x60" + tag + "@" + target + "\x60\n"
        + "- Immutable v1 manifest: \x60" + prepared.manifest_path + "\x60\n"
        + "- Manifest SHA-256: \x60" + prepared.manifest_sha256 + "\x60\n\n"
        "### Outstanding mandatory gates\n\n"
        "- [ ] Native Daniel-CL Windows tests and exact candidate release receipt\n"
        "- [ ] Reviewed source/semantic/security/permission impact\n"
        "- [ ] Independent GitHub required CI at exact tested HEAD\n"
        "- [ ] Explicit owner PR acceptance and merge decision\n"
        "- [ ] Separate release, installer and deployment approvals\n\n"
        "**DO NOT MERGE, TAG, INSTALL, DISTRIBUTE OR ENABLE AUTO-MERGE.** "
        "Draft creation never triggers the existing receipt-bound "
        "upstream promotion workflows.\n"
    )
    return DraftPRPlan(
        candidate_branch=prepared.candidate_branch,
        candidate_sha=prepared.prepared_commit_sha,
        production_sha=prepared.production_sha,
        target_sha=target, target_tag=tag,
        manifest_path=prepared.manifest_path,
        manifest_sha256=prepared.manifest_sha256,
        title=title, body=body,
    )


def _git(repo: Path, *args: str) -> str:
    return preparer._git(repo, *args).decode("utf-8")


def verify_published_source(
    repo: Path,
    plan: DraftPRPlan,
    prepared: preparer.PreparedCandidate,
    manifest_bytes: bytes,
    *,
    pinned_target_oid: str,
    pinned_tracked_oid: str,
) -> None:
    """Read-only official tag/ref/tree, dependency and publication verification."""
    if plan != draft_plan(prepared, manifest_bytes):
        raise DraftPRBlocked("draft_plan_tampered")
    manifest = report._manifest(manifest_bytes)
    repo = Path(repo).resolve()
    if _git(repo, "rev-parse", "HEAD") != prepared.production_sha:
        raise DraftPRBlocked("local_production_checkout_moved")
    production = report._remote_branch(repo, "refs/heads/" + BASE_BRANCH)
    if production != prepared.production_sha:
        raise DraftPRBlocked("production_ref_moved")
    candidate = report._remote_branch(repo, "refs/heads/" + plan.candidate_branch)
    if candidate is None:
        raise DraftPRBlocked("candidate_not_published")
    if candidate != plan.candidate_sha:
        raise DraftPRBlocked("candidate_ref_moved")

    manifest_tag = manifest.tracked_upstream_tag
    for tag, pinned, sha, tree in (
        (manifest_tag, pinned_tracked_oid, manifest.tracked_upstream_sha, manifest.tracked_upstream_tree),
        (plan.target_tag, pinned_target_oid, plan.target_sha, manifest.target_upstream_tree),
    ):
        if SAFE_OID.fullmatch(pinned) is None:
            raise DraftPRBlocked("invalid_pinned_official_tag")
        if report._official_oid(repo, tag) != pinned:
            raise DraftPRBlocked("official_tag_ref_moved")
        local_ref = "refs/tags/" + tag
        if _git(repo, "rev-parse", local_ref) != pinned:
            raise DraftPRBlocked("local_tag_ref_moved")
        if _git(repo, "rev-parse", local_ref + "^{commit}") != sha:
            raise DraftPRBlocked("peeled_upstream_commit_moved")
        if _git(repo, "rev-parse", sha + "^{tree}") != tree:
            raise DraftPRBlocked("peeled_upstream_tree_moved")
    if _git(repo, "rev-parse", production + "^{tree}") != manifest.production_tree:
        raise DraftPRBlocked("production_tree_moved")
    if _git(repo, "rev-parse", plan.candidate_sha + "^{tree}") != prepared.prepared_tree_sha:
        raise DraftPRBlocked("candidate_tree_moved")
    parents = _git(repo, "rev-list", "--parents", "-n", "1", plan.candidate_sha).split()
    if parents != [plan.candidate_sha, production]:
        raise DraftPRBlocked("candidate_parent_mismatch")

    if preparer._git_blob(repo, prepared.prepared_tree_sha, plan.manifest_path) != manifest_bytes:
        raise DraftPRBlocked("manifest_blob_mismatch")
    marker = "codex-rs/upstream-codex-release.txt"
    if preparer._git_blob(repo, prepared.prepared_tree_sha, marker) != (plan.target_tag + "\n").encode("ascii"):
        raise DraftPRBlocked("candidate_upstream_marker_mismatch")
    product = "codex-rs/codexdd-version.txt"
    if preparer._git_blob(repo, production, product) != preparer._git_blob(
        repo, prepared.prepared_tree_sha, product
    ):
        raise DraftPRBlocked("unexpected_product_version_change")
    lockfile = "codex-rs/Cargo.lock"
    if preparer._git_blob(repo, production, lockfile) != preparer._git_blob(
        repo, prepared.prepared_tree_sha, lockfile
    ):
        raise DraftPRBlocked("candidate_dependency_graph_changed")

    tracked_entries = git_tree_entries(repo, manifest.tracked_upstream_sha)
    production_entries = git_tree_entries(repo, production)
    target_entries = git_tree_entries(repo, plan.target_sha)
    inventory = build_delta_inventory(tracked_entries, production_entries, target_entries)
    if (
        inventory.overlap_paths
        or manifest.overlap_path_count != len(inventory.overlap_paths)
        or manifest.upstream_change_path_count != len(inventory.upstream_change_paths)
        or manifest.customization_path_count != len(inventory.customization_paths)
        or any(
            path not in policy.REVIEWED_SAFE_UPSTREAM_PATHS or policy._is_protected(path)
            for path in inventory.upstream_change_paths
        )
    ):
        raise DraftPRBlocked("upstream_delta_not_reviewed")
    if not inventory.upstream_change_paths:
        raise DraftPRBlocked("empty_upstream_delta")
    expected = dict(production_entries)
    for path in inventory.upstream_change_paths:
        _blob = preparer._verified_blob_change(path, tracked_entries, target_entries)
        expected[path] = target_entries[path]
    actual = git_tree_entries(repo, prepared.prepared_tree_sha)
    for path in (marker, plan.manifest_path):
        expected.pop(path, None)
        actual.pop(path, None)
    if expected != actual:
        raise DraftPRBlocked("candidate_tree_delta_mismatch")


def _gh(repo: Path, *args: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        ["gh", *args], cwd=repo, text=True, capture_output=True,
        check=False, timeout=30,
    )


def _any_existing_pr(repo: Path, branch: str) -> bool:
    """All PR states count as duplicates. An incomplete query is NOT 'none'."""
    result = _gh(
        repo, "pr", "list", "--repo", REPOSITORY, "--head", branch,
        "--state", "all", "--limit", "1000",
        "--json", "number,headRefName",
    )
    if result.returncode != 0:
        raise DraftPRBlocked("pr_query_unavailable")
    try:
        rows = json.loads(result.stdout)
    except json.JSONDecodeError as exc:
        raise DraftPRBlocked("invalid_pr_query_response") from exc
    if not isinstance(rows, list) or len(rows) >= 1000:
        raise DraftPRBlocked("pr_query_incomplete")
    for row in rows:
        if (
            not isinstance(row, dict)
            or type(row.get("number")) is not int
            or not isinstance(row.get("headRefName"), str)
        ):
            raise DraftPRBlocked("invalid_pr_query_response")
        if row["headRefName"] != branch:
            raise DraftPRBlocked("unexpected_pr_head")
        return True
    return False


def _preflight(
    repo: Path,
    plan: DraftPRPlan,
    prepared: preparer.PreparedCandidate,
    manifest_bytes: bytes,
    *,
    pinned_target_oid: str,
    pinned_tracked_oid: str,
) -> None:
    verify_published_source(
        repo, plan, prepared, manifest_bytes,
        pinned_target_oid=pinned_target_oid,
        pinned_tracked_oid=pinned_tracked_oid,
    )
    if _any_existing_pr(repo, plan.candidate_branch):
        raise DraftPRBlocked("candidate_pr_already_exists")


def attempt_draft_pr(
    repo: Path,
    prepared: preparer.PreparedCandidate,
    manifest_bytes: bytes,
    *,
    pinned_target_oid: str,
    pinned_tracked_oid: str,
) -> DraftPRResult:
    """No-op by default. Later owner-enabled invocation is draft-only.

    GitHub Actions may refuse PR creation. This function does not retry with
    alternate credentials, modify repository settings or bypass the denial.
    """
    plan = draft_plan(prepared, manifest_bytes)
    if not DRAFT_PR_WRITES_ENABLED:
        return _result(plan, "owner_action_required", "automatic_draft_pr_gate_closed")
    if not os.environ.get("GH_TOKEN"):
        return _result(plan, "blocked_incomplete", "no_authorized_gh_token")
    try:
        _preflight(
            repo, plan, prepared, manifest_bytes,
            pinned_target_oid=pinned_target_oid, pinned_tracked_oid=pinned_tracked_oid,
        )
        # The second recheck reduces race window. GitHub has no CAS create-PR
        # operation; no duplicate/denied attempt may be silently retried.
        _preflight(
            repo, plan, prepared, manifest_bytes,
            pinned_target_oid=pinned_target_oid, pinned_tracked_oid=pinned_tracked_oid,
        )
    except DraftPRBlocked as exc:
        return _result(plan, "blocked_duplicate" if str(exc) == "candidate_pr_already_exists"
                       else "blocked_incomplete", str(exc))
    except (OSError, subprocess.SubprocessError, report.VerificationError,
            preparer.PreparationBlocked, ValueError):
        return _result(plan, "blocked_incomplete", "exact_identity_unavailable")

    with tempfile.TemporaryDirectory(prefix="codexdd-unvalidated-pr-") as directory:
        body_file = Path(directory) / "draft.md"
        body_file.write_text(plan.body, encoding="utf-8")
        try:
            created = _gh(
                repo, "pr", "create", "--draft", "--repo", REPOSITORY,
                "--base", BASE_BRANCH, "--head", plan.candidate_branch,
                "--title", plan.title, "--body-file", str(body_file),
            )
        except (OSError, subprocess.SubprocessError):
            return _result(plan, "blocked_incomplete", "pr_create_unavailable")

    if created.returncode != 0:
        denied = GITHUB_ACTIONS_FORBIDDEN in (created.stderr or "").lower()
        if denied:
            return _result(plan, "owner_action_required", "github_actions_pr_creation_forbidden")
        return _result(plan, "blocked_incomplete", "pr_create_failed_no_retry")
    url = created.stdout.strip()
    if PR_URL.fullmatch(url) is None:
        return _result(plan, "blocked_incomplete", "created_pr_url_not_verified")
    try:
        checked = _gh(
            repo, "pr", "view", url, "--repo", REPOSITORY,
            "--json", "url,isDraft,headRefName,headRefOid,baseRefName",
        )
        info = json.loads(checked.stdout) if checked.returncode == 0 else None
        valid = isinstance(info, dict) and (
            info.get("url") == url
            and info.get("isDraft") is True
            and info.get("headRefName") == plan.candidate_branch
            and info.get("headRefOid") == plan.candidate_sha
            and info.get("baseRefName") == BASE_BRANCH
        )
    except (OSError, subprocess.SubprocessError, json.JSONDecodeError):
        valid = False
    if not valid:
        return _result(plan, "blocked_incomplete", "created_pr_identity_unverified")
    return _result(plan, "draft_created_unvalidated", "needs_windows_and_owner_review", url)


def owner_handoff(plan: DraftPRPlan, result: DraftPRResult) -> str:
    """Text-only fallback; caller retains exact evidence, not a forged PR."""
    if result.candidate_sha != plan.candidate_sha or result.candidate_branch != plan.candidate_branch:
        raise DraftPRBlocked("handoff_candidate_mismatch")
    for value in (result.status, result.reason):
        if not isinstance(value, str) or re.fullmatch(r"[a-z_]+", value) is None:
            raise DraftPRBlocked("unsafe_handoff_status")
    if result.pr_url is not None and PR_URL.fullmatch(result.pr_url) is None:
        raise DraftPRBlocked("unsafe_handoff_pr_url")
    return (
        "UNVALIDATED / NEEDS_WINDOWS_AND_OWNER_REVIEW\n"
        + "Status: " + result.status + "\n"
        + "Reason: " + result.reason + "\n"
        + "Candidate branch: " + plan.candidate_branch + "\n"
        + "Expected candidate SHA: " + plan.candidate_sha + "\n"
        + "Manifest SHA-256: " + plan.manifest_sha256 + "\n"
        + "Draft PR title: " + plan.title + "\n"
        + "Owner may use a separately approved GitHub credential/route after "
        "independent source and branch review. Never alter Actions settings "
        "or claim the candidate is Windows validated.\n"
    )
