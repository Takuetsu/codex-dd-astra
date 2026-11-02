#!/usr/bin/env python3
"""CodexDD 0.4.5: exact-SHA, format-only repair in a separate Windows worktree.

Does not modify an existing checkout, security manifests, Rust sources, or release state.
Formats and verifies the source before any optional fast-forward push to draft PR #78.
"""

from __future__ import annotations

import argparse
import ast
import re
import shutil
import subprocess
import sys
from pathlib import Path

BRANCH = "dd/codexdd-v0.4.5-3f69-safe-ci-repair"
EXPECTED_HEAD = "aea11360a477a57159142f932f2ea01f0d075a83"
BAZEL_FILE = "bazel/rules/testing/runtime_archive_fixture/BUILD.bazel"


def git(root: Path, *args: str, capture: bool = False) -> str:
    process = subprocess.run(
        ["git", "-C", str(root), *args],
        check=True,
        text=True,
        capture_output=capture,
    )
    return process.stdout.strip() if capture else ""


def tool(root: Path, *args: str) -> None:
    subprocess.run(list(args), cwd=root, check=True)


def check_ast(root: Path, changed: list[str]) -> None:
    for path in changed:
        if not path.endswith(".py"):
            continue
        before = subprocess.check_output(["git", "-C", str(root), "show", f"HEAD:{path}"])
        after = (root / path).read_bytes()
        if ast.dump(ast.parse(before), include_attributes=False) != ast.dump(
            ast.parse(after), include_attributes=False
        ):
            raise RuntimeError(f"Refusing non-format Python AST change: {path}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--repo", required=True, type=Path, help="Existing Daniel-CL Git checkout")
    parser.add_argument("--publish", action="store_true", help="Commit and fast-forward the draft PR branch")
    args = parser.parse_args()
    root = args.repo.resolve()
    if not root.exists():
        raise RuntimeError(f"Repository path does not exist: {root}")
    origin = git(root, "remote", "get-url", "origin", capture=True)
    if not re.search(r"(?:github\.com[:/])Takuetsu/codex-dd-astra(?:\.git)?/?$", origin):
        raise RuntimeError(f"Unexpected origin; refusing remote operation: {origin}")
    if git(root, "status", "--porcelain", "--untracked-files=no", capture=True):
        raise RuntimeError("Existing checkout has tracked edits; refusing to proceed")
    for command in ("uv", "dotslash"):
        if shutil.which(command) is None:
            raise RuntimeError(f"Required tool missing on Daniel-CL: {command}")

    git(root, "fetch", "--no-tags", "origin", f"+refs/heads/{BRANCH}:refs/remotes/origin/{BRANCH}")
    current = git(root, "rev-parse", f"refs/remotes/origin/{BRANCH}", capture=True)
    if current != EXPECTED_HEAD:
        raise RuntimeError(f"Branch changed; no modifications attempted. Expected {EXPECTED_HEAD}, found {current}")

    worktree = root / "worktrees" / "codexdd-045-3f70-format-only"
    if worktree.exists():
        raise RuntimeError(f"Worktree already exists; refusing to overwrite: {worktree}")
    worktree.parent.mkdir(parents=True, exist_ok=True)
    git(root, "worktree", "add", "--detach", str(worktree), EXPECTED_HEAD)

    print(f"Created isolated worktree: {worktree}", flush=True)
    tool(worktree, "uv", "run", "--frozen", "--project", "scripts", "ruff", "format", ".")
    tool(worktree, "dotslash", "tools/buildifier", "-mode=fix", "-lint=off", BAZEL_FILE)
    tool(worktree, "uv", "run", "--frozen", "--project", "scripts", "ruff", "format", "--check", ".")
    tool(worktree, "dotslash", "tools/buildifier", "-mode=check", "-lint=off", BAZEL_FILE)
    git(worktree, "diff", "--check")

    names = git(worktree, "diff", "--name-only", "--diff-filter=AM", capture=True)
    changed = [line.strip() for line in names.splitlines() if line.strip()]
    if not changed:
        raise RuntimeError("Formatter made no changes; unexpected CI discrepancy")
    for path in changed:
        valid_python = path.endswith(".py") and path.startswith(("scripts/", ".github/scripts/"))
        if not (valid_python or path == BAZEL_FILE):
            raise RuntimeError(f"Refusing a file outside the CI formatting defect: {path}")
    check_ast(worktree, changed)
    if BAZEL_FILE not in changed:
        raise RuntimeError("Expected Bazel fixture formatting change missing")

    py_count = sum(p.endswith(".py") for p in changed)
    print(f"FORMAT_ONLY_PASS: {py_count} Python files, 1 Bazel file; Python AST unchanged")
    if not args.publish:
        print("Dry run: no commit/push. Re-run with --publish after review.")
        return 0

    remote = git(root, "ls-remote", "origin", f"refs/heads/{BRANCH}", capture=True)
    if not remote or remote.split()[0] != EXPECTED_HEAD:
        raise RuntimeError("Remote advanced; refusing to publish a stale candidate")
    git(worktree, "add", "--", *changed)
    git(worktree, "commit", "-m", "style(0.4.5): apply exact Ruff and Buildifier fixes for blocking CI")
    git(worktree, "push", "origin", f"HEAD:refs/heads/{BRANCH}")
    final = git(worktree, "rev-parse", "HEAD", capture=True)
    print(f"PUBLISHED_FORMAT_ONLY_COMMIT={final}")
    print("PR #78 will run regular blocking CI; no release or merge is authorized")
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, subprocess.CalledProcessError, RuntimeError, SyntaxError) as exc:
        print(f"FORMAT_REPAIR_BLOCKED: {exc}", file=sys.stderr)
        sys.exit(1)
