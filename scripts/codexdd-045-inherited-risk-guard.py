#!/usr/bin/env python3
"""Fail-closed guard for exactly three owner-approved 0.4.5 INFO risk dispositions.

A PASS here is not a green security audit or release authorization:
cargo-deny must still run all-target with every other advisory enforced.
"""
from __future__ import annotations

import argparse
from collections import deque
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import sys
import tomllib
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]
CARGO = ROOT / "codex-rs"
CUTOFF = datetime(2026, 11, 20, 0, 0, tzinfo=timezone.utc)
TARGET_UPSTREAM = "0.163.0"
PINNED_I18N_REV = "2ce7b1dfe9a8cc224288bf50cbc2a67a6de863c4"
PINNED_I18N_URL = "https://github.com/kellpossible/cargo-i18n"
PINNED_RAGE_URL = "https://github.com/str4d/rage"
PINNED_RAGE_REV = "b5b68c4f4f7ea5c368bc060f6fe0237549c3a13f"
PINNED_CLATTER_URL = "https://github.com/jmlepisto/clatter"
PINNED_CLATTER_REV = "9a8d15c4f80d5911ca0403aa22e0de99ea59df08"
PINNED_AWS_VENDOR_TREE_SHA256 = "0bc974a6b452f6b68d3e1506ee096290f45e8bd129584f6288634cf58032c24a"
ACCEPTED = (
    ("RUSTSEC-2026-0371", "zbus_names", "3.0.0"),
    ("RUSTSEC-2026-0370", "zvariant", "4.2.0"),
    ("RUSTSEC-2026-0372", "zvariant_derive", "4.2.0"),
)
BASELINE = (
    "RUSTSEC-2024-0388",
    "RUSTSEC-2025-0057",
    "RUSTSEC-2024-0436",
    "RUSTSEC-2023-0089",
    "RUSTSEC-2024-0320",
    "RUSTSEC-2025-0141",
    "RUSTSEC-2026-0118",
    "RUSTSEC-2026-0119",
    "RUSTSEC-2026-0194",
    "RUSTSEC-2026-0195",
)
BASE_GIT_URLS = (
    "https://github.com/dzbarsky/rules_rust",
    "https://github.com/helix-editor/nucleo.git",
    "https://github.com/hyperium/h3",
    "https://github.com/juberti-oai/rust-sdks.git",
    "https://github.com/microsoft/mxc",
    "https://github.com/modelcontextprotocol/rust-sdk",
    "https://github.com/openai-oss-forks/crossterm",
    "https://github.com/openai-oss-forks/tokio-tungstenite",
    "https://github.com/openai-oss-forks/tungstenite-rs",
)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise RuntimeError("BLOCKED: " + message)


def read_toml(path: Path) -> dict:
    with path.open("rb") as fh:
        return tomllib.load(fh)


def validate_offline() -> None:
    now = datetime.now(timezone.utc)
    require(now < CUTOFF, "owner acceptance expired at 2026-11-20T00:00:00Z")
    manifest = read_toml(CARGO / "Cargo.toml")
    require(
        manifest["workspace"]["package"]["version"] == TARGET_UPSTREAM,
        "next upstream integration requires new owner security review",
    )
    product = (CARGO / "codexdd-version.txt").read_text(encoding="utf-8").strip()
    upstream = (CARGO / "upstream-codex-release.txt").read_text(encoding="utf-8").strip()
    require(
        (product, upstream) in (("0.4.4", "rust-v0.162.0"), ("0.4.5", "rust-v0.163.0")),
        "new product/upstream marker requires a new acceptance record",
    )
    patch = manifest.get("patch", {}).get("crates-io", {}).get("i18n-config")
    require(
        patch == {"git": PINNED_I18N_URL, "rev": PINNED_I18N_REV},
        "unreviewed i18n-config source or changed immutable Git revision",
    )
    patches = manifest.get("patch", {}).get("crates-io", {})
    for crate, url, rev in (
        ("age", PINNED_RAGE_URL, PINNED_RAGE_REV),
        ("age-core", PINNED_RAGE_URL, PINNED_RAGE_REV),
        ("clatter", PINNED_CLATTER_URL, PINNED_CLATTER_REV),
    ):
        require(patches.get(crate) == {"git": url, "rev": rev},
                "unreviewed cryptographic source: " + crate)
    require(patches.get("aws-config") == {"path": "vendor/aws-config-1.13.0"},
            "AWS DPoP patch path differs from reviewed vendor source")
    require(manifest["workspace"]["dependencies"]["clatter"]["version"] == "3.0.0",
            "stable clatter 3.0.0 KEM owner missing")
    vendor = CARGO / "vendor" / "aws-config-1.13.0"
    require(vendor.is_dir() and all(not p.is_symlink() for p in vendor.rglob("*")),
            "missing AWS vendor or unexpected symlink")
    aws = read_toml(vendor / "Cargo.toml")
    require(aws["package"]["name"] == "aws-config" and aws["package"]["version"] == "1.13.0",
            "unreviewed AWS source")
    require(aws["dependencies"]["p256"]["version"] == "0.14.0"
            and aws["dependencies"]["rand"]["version"] == "0.10.1"
            and aws["dependencies"]["rand"]["features"] == ["std", "std_rng", "sys_rng"],
            "secure DPoP signer RNG or P256 API modified")
    files = sorted(p for p in vendor.rglob("*") if p.is_file())
    require(len(files) == 74, "reviewed AWS vendor file inventory changed")
    digest = hashlib.sha256()
    for p in files:
        digest.update(p.relative_to(vendor).as_posix().encode("utf-8"))
        digest.update(bytes([0]))
        digest.update(hashlib.sha256(p.read_bytes()).digest())
    require(digest.hexdigest() == PINNED_AWS_VENDOR_TREE_SHA256,
            "AWS vendor byte fingerprint does not match tested published crate patch")
    require(
        manifest["workspace"]["dependencies"].get("zstd") == "0.14",
        "Zstd security fix missing or changed",
    )
    policy = read_toml(CARGO / "deny.toml")
    audit = read_toml(CARGO / ".cargo" / "audit.toml")
    wanted = list(BASELINE) + [row[0] for row in ACCEPTED]
    entries = policy["advisories"]["ignore"]
    ids = [entry["id"] for entry in entries]
    require(ids == wanted and len(set(ids)) == len(ids), "cargo-deny exception list changed")
    require(audit["advisories"]["ignore"] == wanted, "cargo-audit and cargo-deny exception scope differ")
    for entry, (advisory_id, crate, _) in zip(entries[len(BASELINE):], ACCEPTED):
        reason = entry["reason"]
        require(advisory_id == entry["id"], "incorrect advisory accepted")
        require(
            all(item in reason for item in ("INFO/unmaintained", "Linux-only", "2026-11-20", crate)),
            "missing explicit severity, target, crate or expiry in " + advisory_id,
        )
    require(policy["graph"]["targets"] == [], "all-target audit was narrowed")
    require(policy["sources"]["unknown-git"] == "deny", "Git source enforcement weakened")
    require(policy["sources"]["unknown-registry"] == "deny", "registry enforcement weakened")
    require(policy["sources"]["required-git-spec"] == "rev", "immutable Git requirement weakened")
    allowed_sources = policy["sources"]["allow-git"]
    require(
        allowed_sources == list(BASE_GIT_URLS) + [PINNED_I18N_URL, PINNED_RAGE_URL, PINNED_CLATTER_URL],
        "Git source allowlist differs from reviewed exact entries",
    )
    lock = read_toml(CARGO / "Cargo.lock")
    pkgs: dict[str, set[str]] = {}
    expected_git = {
        "i18n-config": (PINNED_I18N_URL, PINNED_I18N_REV),
        "age": (PINNED_RAGE_URL, PINNED_RAGE_REV),
        "age-core": (PINNED_RAGE_URL, PINNED_RAGE_REV),
        "clatter": (PINNED_CLATTER_URL, PINNED_CLATTER_REV),
    }
    source_counts = {name: 0 for name in expected_git}
    for item in lock["package"]:
        name = item["name"]
        source = item.get("source", "")
        pkgs.setdefault(name, set()).add(item["version"])
        if name in expected_git:
            url, revision = expected_git[name]
            require(source == f"git+{url}?rev={revision}#{revision}",
                    "unpinned or unexpected Git source for " + name)
            source_counts[name] += 1
        elif any(source.startswith("git+" + url) for url in
                 (PINNED_I18N_URL, PINNED_RAGE_URL, PINNED_CLATTER_URL)):
            require(False, "extra, unreviewed crate from pinned Git source: " + name)
    require(source_counts == {name: 1 for name in expected_git},
            "reviewed pinned Git crate omitted or duplicated")
    require("basic-toml" not in pkgs, "basic-toml has reentered the candidate lock")
    require(pkgs.get("i18n-config") == {"0.4.8"}, "unexpected i18n-config version")
    for crate, version in {
        "crypto-bigint": "0.7.10",
        "p256": "0.14.0",
        "p384": "0.14.0",
        "hpke": "0.14.1",
        "age": "0.12.1",
        "age-core": "0.12.0",
        "aws-config": "1.13.0",
        "clatter": "3.0.0",
        "rtc": "0.21.1",
        "webrtc": "0.21.1",
    }.items():
        require(pkgs.get(crate) == {version}, "secure crypto owner changed: " + crate)
    print("PASS: exact modern crypto-bigint, P256/P384, age, HPKE, AWS and RTC sources")
    for _, crate, version in ACCEPTED:
        require(pkgs.get(crate) == {version}, f"{crate} dependency versions changed")
    for crate, version in {
        "zbus": "4.4.0",
        "zstd": "0.14.3",
        "zstd-safe": "8.2.0",
        "ctutils": "0.4.6",
        "rustls": "0.23.46",
        "quinn-proto": "0.11.18",
        "noyalib-serde-yaml": "0.0.57",
        "zip": "9.0.5",
    }.items():
        require(pkgs.get(crate) == {version}, f"protected {crate} version changed")
    print("PASS: exactly three approved INFO exceptions, in both cargo-deny and cargo-audit")
    print("PASS: all-target policy, Git source pin and dependency versions unchanged")
    print("PASS: 2026-11-20 expiry and upstream-integration review gate still valid")
    if pkgs.get("crypto-bigint") == {"0.5.5"}:
        print("RELEASE BLOCKED: crypto-bigint 0.5.5 RUSTSEC-2026-0387 remains unexcepted")


def validate_reachability(windows: Path, linux: Path) -> None:
    def roots_and_reachable(path: Path) -> dict[str, set[str]]:
        data = json.loads(path.read_text(encoding="utf-8"))
        by_id = {item["id"]: item for item in data["packages"]}
        nodes = {item["id"]: item for item in data["resolve"]["nodes"]}
        roots = [item for item in data["workspace_members"] if item in nodes]
        require(bool(roots), "metadata has no workspace roots: " + str(path))
        seen = set(roots)
        pending = deque(roots)
        while pending:
            node = nodes[pending.popleft()]
            for edge in node.get("deps", []):
                target = edge["pkg"]
                if target in nodes and target not in seen:
                    seen.add(target)
                    pending.append(target)
        result: dict[str, set[str]] = {}
        for item in seen:
            pkg = by_id[item]
            result.setdefault(pkg["name"], set()).add(pkg["version"])
        return result

    win = roots_and_reachable(windows)
    lin = roots_and_reachable(linux)
    for _, crate, version in ACCEPTED:
        require(crate not in win, f"{crate} has a new Windows dependency path")
        require(lin.get(crate) == {version}, f"{crate} Linux reachability/version changed")
    require(lin.get("zbus") == {"4.4.0"}, "Linux zbus dependency chain changed")
    print("PASS: all three risk-accepted packages Linux-reachable, absent on Windows")


def validate_rustsec_database() -> None:
    for advisory_id, crate, _ in ACCEPTED:
        url = (
            "https://raw.githubusercontent.com/RustSec/advisory-db/main/"
            f"crates/{crate}/{advisory_id}.md"
        )
        request = Request(url, headers={"User-Agent": "codexdd-045-accepted-risk-guard"})
        with urlopen(request, timeout=20) as response:
            body = response.read(80_000).decode("utf-8")
        ticks = chr(96) * 3
        match = re.match(r"^" + re.escape(ticks) + r"toml\r?\n(.*?)\r?\n" + re.escape(ticks), body, re.DOTALL)
        require(match is not None, "cannot verify RustSec metadata: " + advisory_id)
        info = tomllib.loads(match.group(1))
        advisory = info["advisory"]
        require(
            advisory.get("id") == advisory_id
            and advisory.get("package") == crate
            and advisory.get("informational") == "unmaintained",
            advisory_id + " reclassified/edited; renewed owner review required",
        )
        require(
            info.get("versions", {}).get("patched") == [],
            advisory_id + " now has a maintenance fix; renewed review required",
        )
        print("PASS: latest RustSec record remains INFO/unmaintained: " + advisory_id)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--windows-metadata", type=Path)
    parser.add_argument("--linux-metadata", type=Path)
    parser.add_argument("--check-db", action="store_true")
    args = parser.parse_args()
    try:
        require(
            (args.windows_metadata is None) == (args.linux_metadata is None),
            "must supply both Windows and Linux Cargo metadata files",
        )
        validate_offline()
        if args.windows_metadata is not None:
            validate_reachability(args.windows_metadata, args.linux_metadata)
        if args.check_db:
            validate_rustsec_database()
        print("ACCEPTED-RISK GUARD PASS ONLY: crypto-bigint, cargo-deny and release checks remain mandatory")
        return 0
    except (KeyError, ValueError, OSError, RuntimeError) as error:
        print("BLOCKED:", error, file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
