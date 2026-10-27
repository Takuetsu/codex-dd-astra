#!/usr/bin/env python3
"""CodexDD 0.4.5 dependency remediation staging checks; NOT a cargo-deny substitute."""

from __future__ import annotations

import argparse
import subprocess
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
    "ctutils": "0.4.6",
    "rustls": "0.23.46",
    "quinn-proto": "0.11.18",
    "noyalib-serde-yaml": "0.0.57",
    "zip": "9.0.5",
    "rustls-webpki": "0.103.16",
    "webpki-roots": "1.0.10",
    "webpki-root-certs": "1.0.10",
    # Every cryptographic owner in the accepted 0.4.5 migration is pinned.
    "crypto-bigint": "0.7.10",
    "p256": "0.14.0",
    "p384": "0.14.0",
    "hpke": "0.14.1",
    "rtc": "0.21.1",
    "webrtc": "0.21.1",
    "clatter": "3.0.0",
    "aws-config": "1.13.0",
}
ACCEPTED_LINUX_INFO = {"zbus_names", "zvariant", "zvariant_derive"}
INHERITED_MAINTENANCE = {
    "basic-toml": "RUSTSEC-2026-0374",
    "zbus_names": "RUSTSEC-2026-0371",
    "zvariant": "RUSTSEC-2026-0370",
    "zvariant_derive": "RUSTSEC-2026-0372",
}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--mode",
        required=True,
        choices=("staging", "release"),
        help="staging verifies only remediated packages; release fails closed on known open advisories",
    )
    parser.add_argument("--windows-metadata", type=Path)
    parser.add_argument("--linux-metadata", type=Path)
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
            problems.append(
                f"{name}: expected only {version}, found {sorted(versions)!r}"
            )
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
            problems.append(
                f"nonrelease marker {filename}: {observed!r} != {expected!r}"
            )

    bigint = packages.get("crypto-bigint", set())
    if not bigint:
        print("CRYPTO-BIGINT: not present in this resolved lock")
    elif bigint == {"0.7.10"}:
        print(
            "CRYPTO-BIGINT: secure 0.7.10, still subject to exact all-target cargo-deny and native behavior tests"
        )
    else:
        print(
            f"CRYPTO-BIGINT BLOCKED: {sorted(bigint)!r}; RUSTSEC-2026-0387 patched >=0.7.6"
        )
        if opts.mode == "release":
            problems.append(
                "RUSTSEC-2026-0387 crypto-bigint remains in the all-target graph"
            )

    # Release precheck may use only these three accepted INFO exceptions,
    # and only after live RustSec + dual-target reachability checks pass.
    guard_passed = False
    if opts.mode == "release":
        if not opts.windows_metadata or not opts.linux_metadata:
            problems.append(
                "release precheck needs both Windows and Linux locked Cargo metadata files"
            )
        else:
            cmd = [
                sys.executable,
                str(ROOT / "scripts" / "codexdd-045-inherited-risk-guard.py"),
                "--windows-metadata",
                str(opts.windows_metadata),
                "--linux-metadata",
                str(opts.linux_metadata),
                "--check-db",
            ]
            guard_passed = subprocess.run(cmd, check=False).returncode == 0
            if not guard_passed:
                problems.append("owner-approved INFO risk guard is expired or failed")

    for name, advisory in INHERITED_MAINTENANCE.items():
        if name in packages:
            if name in ACCEPTED_LINUX_INFO:
                print(
                    f"ACCEPTED INFO ONLY (NOT ADVISORY-FREE): {advisory} {name} {sorted(packages[name])!r}"
                )
                # The release-mode guard failure is recorded above.
            else:
                print(
                    f"INHERITED MAINTENANCE OPEN: {advisory} {name} {sorted(packages[name])!r}"
                )
                if opts.mode == "release":
                    problems.append(f"{advisory} {name} is still present")

    if problems:
        for problem in problems:
            print("BLOCKED:", problem, file=sys.stderr)
        return 2
    if opts.mode == "staging":
        print(
            "STAGING CHECK PASS ONLY: accepted INFO scope does not clear cargo-deny or crypto-bigint"
        )
    else:
        print(
            "KNOWN-ISSUE PRECHECK PASS ONLY: unmodified cargo-deny and native release gates still required"
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
