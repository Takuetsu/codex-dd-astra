#!/usr/bin/env python3
"""CodexDD 0.4.5 dependency remediation staging checks; NOT a cargo-deny substitute."""

from __future__ import annotations

import argparse
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LOCK = ROOT / "codex-rs" / "Cargo.lock"
MANIFEST = ROOT / "codex-rs" / "Cargo.toml"

# These checks are additional to the unchanged all-target cargo-deny policy.
REQUIRED = {
    "zstd": "0.14.3",
    "zstd-safe": "8.2.0",
    "ctutils": "0.4.4",
    "rustls": "0.23.46",
    "quinn-proto": "0.11.18",
    "noyalib-serde-yaml": "0.0.57",
    "zip": "9.0.5",
    "rustls-webpki": "0.103.16",
    "webpki-roots": "1.0.10",
    "webpki-root-certs": "1.0.10",
}
INHERITED_MAINTENANCE = {
    "basic-toml": "RUSTSEC-2026-0374",
    "zbus_names": "RUSTSEC-2026-0371",
    "zvariant": "RUSTSEC-2026-0370",
    "zvariant_derive": "RUSTSEC-2026-0372",
}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--mode", required=True, choices=("staging", "release"),
        help="staging verifies only remediated packages; release fails closed on known open advisories",
    )
    opts = parser.parse_args()
    with LOCK.open("rb") as fh:
        locked = tomllib.load(fh)
    with MANIFEST.open("rb") as fh:
        manifest = tomllib.load(fh)

    packages = {}
    for package in locked["package"]:
        packages.setdefault(package["name"], set()).add(package["version"])

    problems = []
    for name, version in REQUIRED.items():
        versions = packages.get(name, set())
        if versions != {version}:
            problems.append(f"{name}: expected only {version}, found {sorted(versions)!r}")
        else:
            print(f"STAGED PATCH: {name} {version}")

    zstd_spec = manifest["workspace"]["dependencies"].get("zstd")
    if zstd_spec != "0.14":
        problems.append(f"workspace zstd spec expected 0.14, found {zstd_spec!r}")

    for filename, expected in (
        ("codexdd-version.txt", "0.4.4"),
        ("upstream-codex-release.txt", "rust-v0.162.0"),
    ):
        observed = (ROOT / "codex-rs" / filename).read_text(encoding="utf-8").strip()
        if observed != expected:
            problems.append(f"nonrelease marker {filename}: {observed!r} != {expected!r}")

    bigint = packages.get("crypto-bigint", set())
    if not bigint:
        print("CRYPTO-BIGINT: not present in this resolved lock")
    elif bigint == {"0.7.6"}:
        print("CRYPTO-BIGINT: patched 0.7.6, follow up with cargo-deny and native crypto tests")
    else:
        print(f"CRYPTO-BIGINT BLOCKED: {sorted(bigint)!r}; RUSTSEC-2026-0387 patched >=0.7.6")
        if opts.mode == "release":
            problems.append("RUSTSEC-2026-0387 crypto-bigint remains in the all-target graph")

    for name, advisory in INHERITED_MAINTENANCE.items():
        if name in packages:
            print(f"INHERITED MAINTENANCE OPEN: {advisory} {name} {sorted(packages[name])!r}")
            if opts.mode == "release":
                problems.append(f"{advisory} {name} is still present")

    if problems:
        for problem in problems:
            print("BLOCKED:", problem, file=sys.stderr)
        return 2
    if opts.mode == "staging":
        print("STAGING CHECK PASS ONLY: no cargo-deny acceptance, crypto-bigint closure or release approval")
    else:
        print("KNOWN-ISSUE PRECHECK PASS ONLY: unmodified cargo-deny and native release gates still required")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
