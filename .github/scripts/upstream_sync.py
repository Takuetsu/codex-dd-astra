"""Helpers for selecting official OpenAI Codex release tags for sync automation."""

from __future__ import annotations

import argparse
import re
import sys
from typing import Iterable


RELEASE_TAG = re.compile(
    r"^rust-v(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)"
    r"(?:-(?P<kind>alpha|beta|rc)(?:\.(?P<parts>\d+(?:\.\d+)*))?)?$"
)
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


def select_latest_release(tags: Iterable[str]) -> str:
    candidates = [(release_version(tag), tag) for tag in tags]
    valid = [(version, tag) for version, tag in candidates if version is not None]
    if not valid:
        raise ValueError("no official rust-vX.Y.Z release tags found")
    return max(valid)[1]


def integration_branch(tag: str) -> str:
    if release_version(tag) is None:
        raise ValueError(f"invalid official release tag: {tag}")
    return f"automation/upstream-sync-{tag}"


def pr_marker(tag: str) -> str:
    if release_version(tag) is None:
        raise ValueError(f"invalid official release tag: {tag}")
    return f"codexdd-upstream-sync: {tag}"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--latest-from-stdin", action="store_true")
    args = parser.parse_args()
    if not args.latest_from_stdin:
        parser.error("--latest-from-stdin is required")
    try:
        print(select_latest_release(line.strip() for line in sys.stdin if line.strip()))
    except ValueError as error:
        print(error, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
