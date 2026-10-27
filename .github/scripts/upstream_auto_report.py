"""Read-only, Git-backed 0.4.5 automatic-update eligibility reporting.

A report is evidence, never permission for candidate publishing or deployment.
No upstream scripts are executed, no remote writes or PR mutations occur.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import tomllib
from dataclasses import fields
from pathlib import Path

from upstream_auto_eligibility import CanonicalSnapshot, assess_eligibility
from upstream_sync import (
    CandidateManifest,
    candidate_branch,
    candidate_manifest_json,
    git_tree_entries,
    stable_release_version,
)

UPSTREAM_URL = "https://github.com/openai/codex.git"
ORIGIN_URLS = frozenset(
    {
        "https://github.com/Takuetsu/codex-dd-astra.git",
        "https://github.com/Takuetsu/codex-dd-astra",
    }
)
PRODUCTION_REF = "refs/heads/dd/astra-policy-v2"
OID = re.compile(r"^[0-9a-f]{40}$")
SAFE_REASON = re.compile(r"^[a-z_]+$")


class VerificationError(Exception):
    def __init__(self, state: str, reason: str):
        super().__init__(reason)
        self.state = state
        self.reason = reason


def _command(repo: Path, *args: str) -> str:
    return subprocess.run(
        list(args), cwd=repo, check=True, capture_output=True, text=True
    ).stdout.strip()


def _git(repo: Path, *args: str) -> str:
    return _command(repo, "git", *args)


def _oid(value: str) -> str:
    if not isinstance(value, str) or OID.fullmatch(value) is None:
        raise VerificationError("blocked_identity", "invalid_git_object_id")
    return value


def _remote_branch(repo: Path, ref: str) -> str | None:
    if _git(repo, "remote", "get-url", "origin") not in ORIGIN_URLS:
        raise VerificationError("blocked_identity", "untrusted_origin")
    result = _git(repo, "ls-remote", "--heads", "origin", ref)
    if not result:
        return None
    parts = result.splitlines()
    if len(parts) != 1:
        raise VerificationError("blocked_identity", "ambiguous_origin_ref")
    sha, actual_ref = parts[0].split("\t", 1)
    if actual_ref != ref:
        raise VerificationError("blocked_identity", "unexpected_origin_ref")
    return _oid(sha)


def _official_oid(repo: Path, tag: str) -> str:
    if stable_release_version(tag) is None:
        raise VerificationError("blocked_identity", "invalid_official_tag")
    ref = "refs/tags/" + tag
    result = _command(
        repo,
        "git",
        "ls-remote",
        "--exit-code",
        "--tags",
        "--refs",
        UPSTREAM_URL,
        ref,
    ).splitlines()
    if len(result) != 1:
        raise VerificationError("blocked_identity", "ambiguous_official_tag")
    sha, actual_ref = result[0].split("\t", 1)
    if actual_ref != ref:
        raise VerificationError("blocked_identity", "unexpected_official_tag")
    return _oid(sha)


def _pairs_no_duplicates(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise VerificationError("blocked_identity", "duplicate_json_key")
        result[key] = value
    return result


def _manifest(raw: bytes) -> CandidateManifest:
    try:
        obj = json.loads(raw.decode("utf-8"), object_pairs_hook=_pairs_no_duplicates)
    except (UnicodeDecodeError, json.JSONDecodeError) as exc:
        raise VerificationError("blocked_identity", "invalid_manifest_json") from exc
    if not isinstance(obj, dict) or set(obj) != {
        field.name for field in fields(CandidateManifest)
    }:
        raise VerificationError("blocked_identity", "invalid_manifest_fields")
    paths = obj["overlap_paths"]
    categories = obj["sensitive_overlap_categories"]
    if (
        type(obj["contract_version"]) is not int
        or not isinstance(paths, list)
        or any(not isinstance(path, str) for path in paths)
        or not isinstance(categories, dict)
        or any(
            not isinstance(cat, str)
            or not isinstance(items, list)
            or any(not isinstance(path, str) for path in items)
            for cat, items in categories.items()
        )
    ):
        raise VerificationError("blocked_identity", "invalid_manifest_types")
    obj["overlap_paths"] = tuple(paths)
    obj["sensitive_overlap_categories"] = {
        cat: tuple(items) for cat, items in categories.items()
    }
    result = CandidateManifest(**obj)
    if candidate_manifest_json(result).encode("utf-8") != raw:
        raise VerificationError("blocked_identity", "noncanonical_manifest")
    return result


def _tree(repo: Path, sha: str) -> str:
    return _oid(_git(repo, "rev-parse", "--verify", _oid(sha) + "^{tree}"))


def _production_lockfile(repo: Path, sha: str) -> bytes:
    # Preserve exact Git blob bytes: stripping text would change the SHA-256.
    return subprocess.run(
        ["git", "show", _oid(sha) + ":codex-rs/Cargo.lock"],
        cwd=repo,
        check=True,
        capture_output=True,
    ).stdout


def _packages(raw: bytes) -> dict[str, str]:
    data = tomllib.loads(raw.decode("utf-8"))
    entries = data.get("package")
    if not isinstance(entries, list):
        raise VerificationError("blocked_incomplete", "invalid_dependency_lockfile")
    versions: dict[str, set[str]] = {}
    for package in entries:
        if not isinstance(package, dict):
            raise VerificationError("blocked_incomplete", "invalid_dependency_lockfile")
        name, version = package.get("name"), package.get("version")
        if not isinstance(name, str) or not isinstance(version, str):
            raise VerificationError("blocked_incomplete", "invalid_dependency_lockfile")
        versions.setdefault(name, set()).add(version)
    result = {name: ",".join(sorted(vs)) for name, vs in versions.items()}
    # Capture all dependency/checksum/source drift, not just selected versions.
    result["$lockfile_sha256"] = hashlib.sha256(raw).hexdigest()
    return result


def _open_pr_exists(repo: Path, branch: str) -> bool:
    if not os.environ.get("GH_TOKEN"):
        raise VerificationError("blocked_incomplete", "pr_read_unavailable")
    results = json.loads(
        _command(
            repo,
            "gh",
            "pr",
            "list",
            "--repo",
            "Takuetsu/codex-dd-astra",
            "--state",
            "open",
            "--limit",
            "1000",
            "--json",
            "headRefName",
        )
    )
    if not isinstance(results, list) or any(
        not isinstance(row, dict) or not isinstance(row.get("headRefName"), str)
        for row in results
    ):
        raise VerificationError("blocked_incomplete", "invalid_pr_response")
    if len(results) >= 1000:
        raise VerificationError("blocked_incomplete", "pr_result_truncated")
    return any(row["headRefName"] == branch for row in results)


def _blocked(state: str, reason: str, digest: str) -> dict[str, object]:
    return dict(
        contract_version=1,
        report_only=True,
        may_publish=False,
        windows_validated=False,
        status=state,
        reasons=[reason],
        manifest_sha256=digest,
        candidate_key=None,
        candidate_branch=None,
        production_sha=None,
        target_upstream_sha=None,
        pinned_tag_oid=None,
        observed_tag_oid=None,
    )


def evaluate(
    repo: Path,
    raw: bytes,
    *,
    pinned_target_oid: str,
    pinned_tracked_oid: str,
    observed_transplant: str,
    conflicts: bytes,
    candidate_lockfile: bytes | None,
) -> dict[str, object]:
    digest = hashlib.sha256(raw).hexdigest()
    manifest = _manifest(raw)
    if manifest.production_branch != "dd/astra-policy-v2":
        raise VerificationError("blocked_identity", "unexpected_production_branch")
    for sha in (
        manifest.production_sha,
        manifest.production_tree,
        manifest.tracked_upstream_sha,
        manifest.tracked_upstream_tree,
        manifest.target_upstream_sha,
        manifest.target_upstream_tree,
    ):
        _oid(sha)
    if _git(repo, "rev-parse", "HEAD") != manifest.production_sha:
        raise VerificationError("blocked_stale_base", "checkout_head_moved")
    current_remote = _remote_branch(repo, PRODUCTION_REF)
    if current_remote != manifest.production_sha:
        raise VerificationError("blocked_stale_base", "production_ref_moved")
    if _tree(repo, current_remote) != manifest.production_tree:
        raise VerificationError("blocked_identity", "production_tree_changed")

    tracked_ref = "refs/tags/" + manifest.tracked_upstream_tag
    target_ref = "refs/tags/" + manifest.target_upstream_tag
    pinned_target_oid, pinned_tracked_oid = (
        _oid(pinned_target_oid),
        _oid(pinned_tracked_oid),
    )
    tracked_oid = _official_oid(repo, manifest.tracked_upstream_tag)
    observed_target_oid = _official_oid(repo, manifest.target_upstream_tag)
    if pinned_tracked_oid != tracked_oid or pinned_target_oid != observed_target_oid:
        raise VerificationError("blocked_identity", "official_tag_moved")
    if (
        _git(repo, "rev-parse", tracked_ref) != tracked_oid
        or _git(repo, "rev-parse", target_ref) != observed_target_oid
    ):
        raise VerificationError("blocked_identity", "local_tag_mismatch")
    tag_type = _git(repo, "cat-file", "-t", observed_target_oid)
    if tag_type not in {"tag", "commit"}:
        raise VerificationError("blocked_identity", "unsupported_tag_type")
    tracked_sha = _git(repo, "rev-parse", tracked_ref + "^{commit}")
    target_sha = _git(repo, "rev-parse", target_ref + "^{commit}")
    if (
        tracked_sha != manifest.tracked_upstream_sha
        or target_sha != manifest.target_upstream_sha
        or _tree(repo, tracked_sha) != manifest.tracked_upstream_tree
        or _tree(repo, target_sha) != manifest.target_upstream_tree
    ):
        raise VerificationError("blocked_identity", "peeled_commit_tree_mismatch")
    live_tracked = _git(
        repo, "show", current_remote + ":codex-rs/upstream-codex-release.txt"
    )
    live_product = _git(repo, "show", current_remote + ":codex-rs/codexdd-version.txt")
    if (
        live_tracked != manifest.tracked_upstream_tag
        or live_product != manifest.codexdd_product_version
    ):
        raise VerificationError("blocked_stale_base", "tracked_product_version_moved")
    if observed_transplant not in {"clean", "conflict"}:
        raise VerificationError("blocked_incomplete", "invalid_transplant_state")
    if observed_transplant != manifest.transplant_dry_run:
        raise VerificationError("blocked_incomplete", "transplant_state_mismatch")
    if observed_transplant == "clean" and conflicts.strip():
        raise VerificationError("blocked_incomplete", "conflicts_in_clean_transplant")

    branch = candidate_branch(
        manifest.target_upstream_tag, manifest.target_upstream_sha
    )
    snapshot = CanonicalSnapshot(
        upstream_repository="openai/codex",
        production_branch="dd/astra-policy-v2",
        production_sha=current_remote,
        production_tree=_tree(repo, current_remote),
        tracked_tag=live_tracked,
        tracked_sha=tracked_sha,
        tracked_tree=_tree(repo, tracked_sha),
        target_tag=manifest.target_upstream_tag,
        target_sha=target_sha,
        target_tree=_tree(repo, target_sha),
        tag_kind="annotated" if tag_type == "tag" else "lightweight",
        pinned_tag_oid=pinned_target_oid,
        observed_tag_oid=observed_target_oid,
        tracked_entries=git_tree_entries(repo, tracked_sha),
        production_entries=git_tree_entries(repo, current_remote),
        target_entries=git_tree_entries(repo, target_sha),
        transplant_result=observed_transplant,
        production_packages=_packages(_production_lockfile(repo, current_remote)),
        candidate_packages=_packages(candidate_lockfile)
        if candidate_lockfile
        else None,
        # A known transplant conflict is terminal for auto-preparation;
        # avoid letting unavailable PR APIs obscure that actionable result.
        candidate_ref_exists=(
            observed_transplant == "clean"
            and _remote_branch(repo, "refs/heads/" + branch) is not None
        ),
        active_candidate_pr_exists=(
            observed_transplant == "clean" and _open_pr_exists(repo, branch)
        ),
        # Cannot prove absence of races until a later, isolated write-time recheck.
        competing_preparation_exists=False,
    )
    decision = assess_eligibility(
        manifest, pinned_manifest_sha256=digest, snapshot=snapshot
    )
    return dict(
        contract_version=1,
        report_only=True,
        may_publish=False,
        windows_validated=False,
        status=decision.status,
        reasons=list(decision.reasons),
        manifest_sha256=digest,
        candidate_key=decision.candidate_key,
        candidate_branch=decision.candidate_branch,
        production_sha=current_remote,
        target_upstream_sha=target_sha,
        pinned_tag_oid=pinned_target_oid,
        observed_tag_oid=observed_target_oid,
    )


def summary_text(report: dict[str, object]) -> str:
    # Only invariant policy codes and prevalidated identity fields are emitted.
    reasons = report["reasons"]
    if any(not isinstance(r, str) or SAFE_REASON.fullmatch(r) is None for r in reasons):
        raise VerificationError("blocked_incomplete", "unsafe_report_reason")
    quoted = chr(96)
    reason_text = ", ".join(quoted + r + quoted for r in reasons) or "none"

    def code(value: object) -> str:
        return quoted + str(value or "unverified") + quoted

    return (
        "## CodexDD auto-update eligibility (read-only report)\n\n"
        + "- State: "
        + code(report["status"])
        + "\n"
        + "- Reasons: "
        + reason_text
        + "\n"
        + "- Candidate key: "
        + code(report["candidate_key"])
        + "\n"
        + "- Production SHA: "
        + code(report["production_sha"])
        + "\n"
        + "- Manifest SHA-256: "
        + code(report["manifest_sha256"])
        + "\n\n"
        + "No branch, PR, release, merge, version bump or deployment is authorized. "
        + "Windows validation and owner gates remain mandatory.\n"
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--conflicts", type=Path, required=True)
    parser.add_argument("--candidate-lockfile", type=Path, required=True)
    parser.add_argument("--pinned-target-oid", required=True)
    parser.add_argument("--pinned-tracked-oid", required=True)
    parser.add_argument("--observed-transplant", required=True)
    parser.add_argument("--report", type=Path, required=True)
    parser.add_argument("--summary", type=Path, required=True)
    args = parser.parse_args()
    try:
        raw = args.manifest.read_bytes()
    except OSError:
        raw = b""
    digest = hashlib.sha256(raw).hexdigest()
    try:
        report = evaluate(
            Path(".").resolve(),
            raw,
            pinned_target_oid=args.pinned_target_oid,
            pinned_tracked_oid=args.pinned_tracked_oid,
            observed_transplant=args.observed_transplant,
            conflicts=args.conflicts.read_bytes(),
            candidate_lockfile=(
                args.candidate_lockfile.read_bytes()
                if args.candidate_lockfile.is_file()
                else None
            ),
        )
    except VerificationError as error:
        report = _blocked(error.state, error.reason, digest)
    except (
        OSError,
        UnicodeError,
        ValueError,
        TypeError,
        KeyError,
        subprocess.SubprocessError,
    ):
        report = _blocked("blocked_incomplete", "git_evidence_unavailable", digest)
    args.report.write_text(
        json.dumps(report, sort_keys=True, separators=(",", ":")) + "\n",
        encoding="utf-8",
    )
    with args.summary.open("a", encoding="utf-8") as output:
        output.write(summary_text(report))
    print("read-only eligibility report: " + report["status"])
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
