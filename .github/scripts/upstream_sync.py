"""Helpers for guarded OpenAI Codex stable-release synchronization."""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from pathlib import Path
from typing import Iterable


RELEASE_TAG = re.compile(
    r"^rust-v(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)"
    r"(?:-(?P<kind>alpha|beta|rc)(?:\.(?P<parts>\d+(?:\.\d+)*))?)?$"
)
PRODUCT_VERSION = re.compile(r"^(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)$")
KIND_ORDER = {"alpha": 0, "beta": 1, "rc": 2}


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
    parsed = product_version(version)
    if parsed is None:
        raise ValueError(f"invalid codexdd product version: {version}")
    major, minor, patch = parsed
    return f"{major}.{minor}.{patch + 1}"


def rewrite_version_test(source: str, current: str, next_version: str) -> str:
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
