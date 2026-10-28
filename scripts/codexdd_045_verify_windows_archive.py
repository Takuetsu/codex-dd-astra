#!/usr/bin/env python3
"""Inspect a real optimized Windows MSVC combined Bazel archive, fail closed.

Source/checkpoint audits are not release approval. This verifier is deliberately
independent of Bazel: it validates what the archive *actually contains*.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sys
import tarfile
from pathlib import Path, PurePosixPath

EXECUTABLES = (
    "codex.exe",
    "codex-app-server.exe",
    "codex-code-mode-host.exe",
    "codex-command-runner.exe",
    "codex-windows-sandbox-setup.exe",
)
DEBUG_SYMBOLS = tuple(name[:-4] + ".pdb" for name in EXECUTABLES)
REQUIRED_FILES = frozenset(EXECUTABLES + DEBUG_SYMBOLS)
MIN_EXE_BYTES = 4096
MIN_PDB_BYTES = 1024
MAX_MEMBER_BYTES = 2 * 1024 * 1024 * 1024
PDB_SIGNATURE = b"Microsoft C/C++ MSF 7.00"


class ArchiveValidationError(ValueError):
    pass


def _hash_member(stream, size: int, name: str) -> tuple[str, bytes]:
    digest = hashlib.sha256()
    prefix = bytearray()
    remaining = size
    while remaining:
        block = stream.read(min(1024 * 1024, remaining))
        if not block:
            raise ArchiveValidationError(f"Truncated archive member: {name}")
        if len(prefix) < 64:
            prefix.extend(block[:64 - len(prefix)])
        digest.update(block)
        remaining -= len(block)
    return digest.hexdigest(), bytes(prefix)


def inspect_archive(path: Path) -> dict:
    if not path.is_file() or path.stat().st_size < 1024:
        raise ArchiveValidationError("Missing or empty combined release archive")
    if path.suffixes[-2:] != [".tar", ".gz"]:
        raise ArchiveValidationError("Expected a .tar.gz combined archive")
    observed = {}
    with tarfile.open(path, mode="r:gz") as archive:
        for item in archive:
            name = item.name
            if "\\" in name or name.startswith("/") or ".." in PurePosixPath(name).parts:
                raise ArchiveValidationError(f"Unsafe tar member name: {name!r}")
            if item.isdir():
                # Bazel pkg_tar may include a root directory marker only.
                if name not in (".", "./", ""):
                    raise ArchiveValidationError(f"Unexpected directory: {name!r}")
                continue
            if not item.isfile():
                raise ArchiveValidationError(f"Unexpected special tar member: {name!r}")
            canonical = str(PurePosixPath(name))
            if len(PurePosixPath(canonical).parts) != 1:
                raise ArchiveValidationError(f"Unexpected nested archive file: {name!r}")
            if canonical not in REQUIRED_FILES:
                raise ArchiveValidationError(f"Unexpected combined archive file: {name!r}")
            if canonical in observed:
                raise ArchiveValidationError(f"Duplicate archive filename: {canonical}")
            if item.size < (MIN_PDB_BYTES if canonical.endswith(".pdb") else MIN_EXE_BYTES):
                raise ArchiveValidationError(f"Truncated or placeholder artifact: {canonical}")
            if item.size > MAX_MEMBER_BYTES:
                raise ArchiveValidationError(f"Oversized artifact: {canonical}")
            reader = archive.extractfile(item)
            if reader is None:
                raise ArchiveValidationError(f"Unreadable archive member: {canonical}")
            with reader:
                digest, prefix = _hash_member(reader, item.size, canonical)
            if canonical.endswith(".exe") and not prefix.startswith(b"MZ"):
                raise ArchiveValidationError(f"Not a Windows PE executable: {canonical}")
            if canonical.endswith(".pdb") and not prefix.startswith(PDB_SIGNATURE):
                raise ArchiveValidationError(f"Not an MSVC PDB: {canonical}")
            observed[canonical] = {"sha256": digest, "bytes": item.size}
    missing = sorted(REQUIRED_FILES.difference(observed))
    if missing:
        raise ArchiveValidationError("Missing required archive file(s): " + ", ".join(missing))
    archive_digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            archive_digest.update(block)
    return {
        "result": "NONRELEASE_OPTIMIZED_WINDOWS_ARCHIVE_CONTENT_VERIFIED",
        "archive": path.name,
        "archive_sha256": archive_digest.hexdigest(),
        "archive_bytes": path.stat().st_size,
        "files": dict(sorted(observed.items())),
        "file_count": len(observed),
        "separately_packaged_not_expected": (
            "codex-windows-sandbox-service.exe",
            "codex-responses-api-proxy.exe",
        ),
        "release_approved": False,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("archive", type=Path)
    parser.add_argument("--json-output", type=Path, default=None)
    opts = parser.parse_args(argv)
    try:
        result = inspect_archive(opts.archive)
    except (ArchiveValidationError, OSError, tarfile.TarError) as exc:
        print(f"BLOCKED: {exc}", file=sys.stderr)
        return 1
    report = json.dumps(result, indent=2, sort_keys=True)
    if opts.json_output is not None:
        opts.json_output.parent.mkdir(parents=True, exist_ok=True)
        opts.json_output.write_text(report + "\n", encoding="utf-8")
    print(report)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
