#!/usr/bin/env python3
"""3F.56: assemble exactly seven *unsigned* Windows programs and seven PDBs.

The five-program archive and BOTH sidecar builds must originate from the same
immutable 3F.48 source commit. Reject archive/signature/source drift. Never sign,
publish, or present this developer bundle as an approved release installer.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
import sys
import tarfile
from pathlib import Path, PurePosixPath

import codexdd_045_distribution_gate as gate

BASELINE = "57152222a1ef523fe6e2f5079871672423c6cb56"
EXPECTED = tuple(sorted({p for x in gate.COMBINED + gate.SIDECARS for p in (x, x[:-4] + ".pdb")}))
SOURCE_MARKERS = (b"MZ", b"Microsoft C/C++ MSF 7.00")


def require(cond: bool, message: str) -> None:
    if not cond:
        raise ValueError(message)


def file_hash(path: Path) -> str:
    return gate.sha_file(path)


def _read_receipt(directory: Path, component: str, tooling_sha: str) -> dict:
    path = directory / (component + "-integrity.json")
    require(path.is_file() and not path.is_symlink(), "Missing unlinked " + component + " source build receipt")
    obj = json.loads(path.read_text(encoding="utf-8-sig"))
    require(obj.get("contract_version") == 1, "Unknown sidecar receipt version")
    require(obj.get("source_sha") == BASELINE, "Sidecar built from a different source SHA")
    require(obj.get("workflow_sha") == tooling_sha, "Sidecar build tooling SHA mismatch")
    require(obj.get("component") == component, "Sidecar component swapped")
    require(obj.get("result") == "REAL_OPTIMIZED_MSVC_EXE_AND_PDB_VERIFIED_NONRELEASE", "Not a successful optimized sidecar")
    require(obj.get("release_approved") is False, "Premature sidecar release approval")
    require(obj.get("action_path_sha256") == "7e3457c3218c6933b8b30de65061ccf56c951619862e7d4f38ed9f2be086908e", "Unexpected compilation environment")
    correct = {
        "service": "//codex-rs/windows-sandbox-service:release_files",
        "proxy": "//codex-rs/responses-api-proxy:release_files",
    }
    require(obj.get("target") == correct[component], "Wrong Bazel target for " + component)
    exe = "codex-windows-sandbox-service.exe" if component == "service" else "codex-responses-api-proxy.exe"
    required = {exe, exe[:-4] + ".pdb"}
    require(set(obj.get("files", {})) == required, "Not exact sidecar EXE+PDB inventory")
    require({p.name for p in directory.iterdir()} == required | {path.name}, "Unknown supplemental files")
    out = {}
    for name in sorted(required):
        p = directory / name
        require(p.is_file() and not p.is_symlink() and p.stat().st_size > 4096, "Invalid physical sidecar: " + name)
        with p.open("rb") as f:
            signature = f.read(32)
        require(signature.startswith(SOURCE_MARKERS[1] if name.endswith(".pdb") else SOURCE_MARKERS[0]), "Bad physical PE/PDB signature: " + name)
        meta = obj["files"][name]
        digest = file_hash(p)
        require(meta.get("bytes") == p.stat().st_size and meta.get("sha256") == digest, "Sidecar SHA-256 or byte-size mismatch: " + name)
        require(re.fullmatch(r"[0-9a-f]{64}", digest) is not None, "Malformed digest")
        out[name] = {"bytes": meta["bytes"], "sha256": digest, "source_sha": BASELINE}
    return out


def verify_sidecar_provenance(service_dir: Path, proxy_dir: Path, tooling_sha: str) -> dict:
    require(re.fullmatch(r"[0-9a-f]{40}", tooling_sha) is not None, "Unpinned tooling SHA")
    require(service_dir.resolve() != proxy_dir.resolve(), "Service and proxy must come from separate build receipts")
    found = {**_read_receipt(service_dir, "service", tooling_sha),
             **_read_receipt(proxy_dir, "proxy", tooling_sha)}
    expected = {p for x in gate.SIDECARS for p in (x, x[:-4] + ".pdb")}
    require(set(found) == expected, "Missing either source-frozen pair")
    return found


def assemble(combined: Path, service_dir: Path, proxy_dir: Path, out_archive: Path,
             out_manifest: Path, tooling_sha: str) -> dict:
    contract = gate.load_manifest()
    gate.verify_manifest_contract(contract)
    require(contract["source_sha"] == BASELINE, "Five-program baseline source changed")
    sidecars = verify_sidecar_provenance(service_dir, proxy_dir, tooling_sha)
    prior = gate.verify_combined_archive(contract, combined)
    require(prior["member_count"] == 10, "The five-program archive was not verified")
    members = {name: dict(detail, source_sha=BASELINE)
               for name, detail in contract["build_evidence"]["combined_members"].items()}
    members.update(sidecars)
    require(tuple(sorted(members)) == EXPECTED and len(members) == 14, "Expected exactly seven EXE/PDB pairs")
    require(not out_archive.is_symlink() and not out_manifest.is_symlink(), "Output cannot be symlink")
    out_archive.parent.mkdir(parents=True, exist_ok=True)
    out_manifest.parent.mkdir(parents=True, exist_ok=True)
    physical = {
        "codex-windows-sandbox-service.exe": service_dir,
        "codex-windows-sandbox-service.pdb": service_dir,
        "codex-responses-api-proxy.exe": proxy_dir,
        "codex-responses-api-proxy.pdb": proxy_dir,
    }
    with tarfile.open(combined, "r:gz") as original, tarfile.open(
        out_archive, "w:gz", compresslevel=2
    ) as destination:
        old_items = original.getmembers()
        require(len(old_items) == 10 and all(item.isfile() for item in old_items), "Source archive includes unsafe/non-file entries")
        require({item.name for item in old_items} == set(contract["build_evidence"]["combined_members"]), "Five-file archive path drift")
        for item in sorted(old_items, key=lambda item: item.name):
            require(PurePosixPath(item.name).name == item.name, "Unexpected archive path traversal")
            info = tarfile.TarInfo(item.name)
            info.size = item.size
            info.mode = 0o644 if item.name.endswith(".pdb") else 0o755
            info.mtime = 0
            stream = original.extractfile(item)
            require(stream is not None and item.size == members[item.name]["bytes"], "Source item mismatch: " + item.name)
            with stream:
                destination.addfile(info, stream)
        for name in sorted(physical):
            path = physical[name] / name
            info = tarfile.TarInfo(name)
            info.size = path.stat().st_size
            info.mode = 0o644 if name.endswith(".pdb") else 0o755
            info.mtime = 0
            with path.open("rb") as f:
                destination.addfile(info, f)
    require(out_archive.is_file() and out_archive.stat().st_size > 4096, "No physical seven-program bundle")
    # Reopen the *finished* output and authenticate its real contents byte-for-byte.
    seen = {}
    with tarfile.open(out_archive, "r:gz") as result:
        for item in result:
            require(item.isfile() and item.name in members and
                    PurePosixPath(item.name).name == item.name and item.name not in seen,
                    "Unexpected, duplicate or dangerous archive entry")
            require(item.size == members[item.name]["bytes"], "Output member length changed: " + item.name)
            digest = hashlib.sha256()
            with result.extractfile(item) as f:
                first = f.read(32)
                require(first.startswith(SOURCE_MARKERS[1] if item.name.endswith(".pdb") else SOURCE_MARKERS[0]),
                        "Wrong PE/PDB bytes in assembled archive: " + item.name)
                digest.update(first)
                for block in iter(lambda: f.read(1024 * 1024), b""):
                    digest.update(block)
            require(digest.hexdigest() == members[item.name]["sha256"], "Output digest mismatch: " + item.name)
            seen[item.name] = True
    require(set(seen) == set(members), "Incomplete seven-program physical archive")
    report = {
        "contract_version": 1,
        "result": "SEVEN_REAL_OPTIMIZED_WINDOWS_PROGRAMS_AND_SEVEN_PDBS_VERIFIED_NONRELEASE",
        "source_sha": BASELINE,
        "workflow_tooling_sha": tooling_sha,
        "five_program_baseline_run": contract["build_evidence"]["run_id"],
        "original_five_archive_sha256": prior["archive_sha256"],
        "seven_bundle_sha256": file_hash(out_archive),
        "program_count": 7,
        "symbol_count": 7,
        "file_count": len(members),
        "files": dict(sorted(members.items())),
        "source_provenance_verified": True,
        "code_signed": False,
        "voice_runtime_included": False,
        "official_installer_built": False,
        "release_approved": False,
    }
    out_manifest.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    return report


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--combined", required=True, type=Path)
    ap.add_argument("--service-dir", required=True, type=Path)
    ap.add_argument("--proxy-dir", required=True, type=Path)
    ap.add_argument("--out-archive", required=True, type=Path)
    ap.add_argument("--out-manifest", required=True, type=Path)
    ap.add_argument("--tooling-sha", required=True)
    args = ap.parse_args(argv)
    try:
        report = assemble(args.combined, args.service_dir, args.proxy_dir,
                          args.out_archive, args.out_manifest, args.tooling_sha)
        print(json.dumps({k: report[k] for k in (
            "result", "source_sha", "seven_bundle_sha256", "program_count",
            "symbol_count", "code_signed", "release_approved"
        )}, sort_keys=True))
        return 0
    except (ValueError, KeyError, OSError, tarfile.TarError) as exc:
        print("SEVEN_PROGRAM_ASSEMBLY_BLOCKED: " + str(exc), file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
