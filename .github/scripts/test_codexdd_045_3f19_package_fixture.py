"""Inspect a synthetic Windows runtime archive; never treats it as an installable binary."""

import sys
import tarfile
from pathlib import Path


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit("usage: package-fixture-check.py PATH_TO_ARCHIVE")
    archive = Path(sys.argv[1])
    if not archive.is_file() or archive.stat().st_size < 80:
        raise SystemExit("Missing or empty fixture tar.gz")
    expected = {
        "fixture_codex.exe": (b"CodexDD synthetic nonrelease executable fixture\n", 0o755),
        "fixture_codex.pdb": (b"CodexDD synthetic nonrelease symbol fixture\n", 0o644),
    }
    with tarfile.open(archive, "r:gz") as tar:
        members = tar.getmembers()
        files = {}
        for member in members:
            name = member.name.removeprefix("./")
            if not name and member.isdir():
                continue
            if name.startswith("/") or ".." in name.split("/"):
                raise AssertionError("Unsafe archive member: " + name)
            if not member.isfile() or name in files or "/" in name:
                raise AssertionError("Unexpected member or duplicate: " + name)
            files[name] = member
        if set(files) != set(expected):
            raise AssertionError(f"Wrong fixture archive files: {sorted(files)}")
        for name, (data, mode) in expected.items():
            member = files[name]
            reader = tar.extractfile(member)
            if reader is None or reader.read() != data:
                raise AssertionError("Wrong unmodified fixture bytes: " + name)
            if member.mode & 0o777 != mode:
                raise AssertionError(
                    f"Wrong {name} permissions: {member.mode & 0o777:04o}"
                )
    print("PASS: flat, unstripped, unsigned synthetic Windows EXE+PDB tar.gz")
    print("PASS: exact two-file whitelist; no duplicate/path traversal/extra files")
    print("WARNING: this is NOT a native compiled CodexDD binary or release package")


if __name__ == "__main__":
    main()
