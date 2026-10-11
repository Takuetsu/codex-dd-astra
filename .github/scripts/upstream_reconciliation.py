"""Read-only, fail-closed primitives for the future CodexDD v2 conflict bridge.

This module grants NO candidate publication, receipt acceptance, or PR promotion.
A subsequent, separately tested packet must implement complete v2 attestation
schemas and verify Git source trees, official tags, and current production state.
"""

from __future__ import annotations

import hashlib
import json
import math
import re
from collections.abc import Mapping, Sequence

from upstream_sync import candidate_branch, candidate_key, stable_release_version

_SHA1 = re.compile(r"[0-9a-f]{40}\Z")
_SHA256 = re.compile(r"[0-9a-f]{64}\Z")
_COMPONENT = re.compile(r"[A-Za-z0-9_.-]+\Z")


def _reject_duplicate_keys(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError(f"duplicate JSON field: {key}")
        result[key] = value
    return result


def _reject_nonfinite_number(value: str) -> None:
    raise ValueError(f"non-finite JSON number is forbidden: {value}")

def _parse_finite_json_float(value: str) -> float:
    parsed = float(value)
    if not math.isfinite(parsed):
        raise ValueError(f"non-finite JSON exponent is forbidden: {value}")
    return parsed


def canonical_json_bytes(value: object) -> bytes:
    """One stable UTF-8 representation, including a single LF at EOF."""
    try:
        return (
            json.dumps(
                value,
                sort_keys=True,
                separators=(",", ":"),
                ensure_ascii=False,
                allow_nan=False,
            )
            + "\n"
        ).encode("utf-8")
    except (TypeError, ValueError) as error:
        raise ValueError("cannot encode canonical reconciliation JSON") from error


def parse_strict_json(source: bytes, *, canonical: bool = False) -> dict[str, object]:
    if not isinstance(source, bytes):
        raise ValueError("reconciliation JSON must be supplied as bytes")
    try:
        decoded = source.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        raise ValueError("reconciliation JSON must be valid UTF-8") from error
    if decoded.startswith("\ufeff"):
        raise ValueError("reconciliation JSON must not contain a BOM")
    try:
        parsed = json.loads(
            decoded,
            object_pairs_hook=_reject_duplicate_keys,
            parse_constant=_reject_nonfinite_number,
            parse_float=_parse_finite_json_float,
        )
    except (json.JSONDecodeError, UnicodeDecodeError) as error:
        raise ValueError("invalid reconciliation JSON") from error
    if type(parsed) is not dict:
        raise ValueError("reconciliation JSON root must be an object")
    if canonical and canonical_json_bytes(parsed) != source:
        raise ValueError("reconciliation JSON is not canonical UTF-8/LF")
    return parsed


def require_fields(
    obj: object, expected: frozenset[str], *, context: str
) -> Mapping[str, object]:
    if type(obj) is not dict:
        raise ValueError(f"{context} must be an object")
    actual = set(obj)
    if actual != expected:
        raise ValueError(
            f"{context} has missing/unknown fields: "
            f"missing={sorted(expected - actual)}, unknown={sorted(actual - expected)}"
        )
    return obj


def require_sha(value: object, *, sha256: bool = False, name: str = "sha") -> str:
    regex = _SHA256 if sha256 else _SHA1
    if not isinstance(value, str) or regex.fullmatch(value) is None:
        raise ValueError(f"invalid lowercase {name}")
    return value


def canonical_repo_path(path: object, *, prefix: str | None = None) -> str:
    """Lexical path confinement; future publisher must also resolve Git objects."""
    if not isinstance(path, str) or not path or path.startswith("/"):
        raise ValueError("invalid relative Git path")
    if "\\" in path or ":" in path or "\0" in path or "%" in path:
        raise ValueError("unsafe Git path syntax")
    if any(
        not segment or segment in {".", ".."} or _COMPONENT.fullmatch(segment) is None
        for segment in path.split("/")
    ):
        raise ValueError("noncanonical Git path")
    if prefix is not None and not path.startswith(prefix.rstrip("/") + "/"):
        raise ValueError(f"Git path must be confined under {prefix}/")
    return path


def unique_repo_paths(paths: object, *, name: str = "paths") -> tuple[str, ...]:
    if type(paths) is not list or not paths:
        raise ValueError(f"{name} must be a nonempty array")
    checked = tuple(canonical_repo_path(path) for path in paths)
    if len(set(checked)) != len(checked):
        raise ValueError(f"{name} contains duplicate Git paths")
    return checked


def verify_blocked_v1_discovery(
    *,
    manifest_bytes: bytes,
    conflict_report_bytes: bytes,
    manifest_sha256: str,
    conflicts_sha256: str,
    expected_production_sha: str,
    expected_target_tag: str,
    expected_target_sha: str,
    expected_conflict_paths: Sequence[str],
) -> None:
    """Bind original immutable *blocked* discovery to exact conflict ordering.

    A successful result does not authorize a v2 sidecar, candidate, or receipt.
    """
    require_sha(manifest_sha256, sha256=True, name="original manifest digest")
    require_sha(conflicts_sha256, sha256=True, name="original conflict digest")
    require_sha(expected_production_sha, name="expected production SHA")
    require_sha(expected_target_sha, name="expected target SHA")
    if stable_release_version(expected_target_tag) is None:
        raise ValueError("expected target is not a stable upstream release")
    candidate_key(expected_target_tag, expected_target_sha)
    candidate_branch(expected_target_tag, expected_target_sha)
    if hashlib.sha256(manifest_bytes).hexdigest() != manifest_sha256:
        raise ValueError("original discovery manifest digest mismatch")
    if hashlib.sha256(conflict_report_bytes).hexdigest() != conflicts_sha256:
        raise ValueError("original conflict report digest mismatch")

    manifest = parse_strict_json(manifest_bytes)
    if (
        type(manifest.get("contract_version")) is not int
        or manifest["contract_version"] != 1
        or manifest.get("state") != "blocked_transplant_conflict"
        or manifest.get("transplant_dry_run") != "conflict"
    ):
        raise ValueError("original v1 discovery is not immutably blocked")
    expected_identity = {
        "production_sha": expected_production_sha,
        "target_upstream_tag": expected_target_tag,
        "target_upstream_sha": expected_target_sha,
    }
    for key, value in expected_identity.items():
        if manifest.get(key) != value:
            raise ValueError(f"original discovery identity mismatch: {key}")

    try:
        report = conflict_report_bytes.decode("utf-8", errors="strict")
    except UnicodeDecodeError as error:
        raise ValueError("conflict report is not UTF-8") from error
    if report.startswith("\ufeff"):
        raise ValueError("conflict report must not contain BOM")
    original_paths = unique_repo_paths(report.splitlines(), name="original conflicts")
    expected_paths = unique_repo_paths(
        list(expected_conflict_paths), name="expected conflicts"
    )
    if original_paths != expected_paths:
        raise ValueError("missing, extra or reordered original textual conflicts")
