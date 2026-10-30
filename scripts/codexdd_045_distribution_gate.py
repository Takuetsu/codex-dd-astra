#!/usr/bin/env python3
"""CodexDD 0.4.5: verify source-bound Windows staging requirements, never publish."""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MANIFEST = ROOT / 'docs/upstream-candidates/codexdd-045-windows-distribution-contract.json'
COMBINED = (
    'codex.exe', 'codex-app-server.exe', 'codex-code-mode-host.exe',
    'codex-command-runner.exe', 'codex-windows-sandbox-setup.exe',
)
SIDECARS = ('codex-windows-sandbox-service.exe', 'codex-responses-api-proxy.exe')
SIG_PDB = b'Microsoft C/C++ MSF 7.00'


class DistributionError(ValueError):
    pass


def require(condition: bool, explanation: str) -> None:
    if not condition:
        raise DistributionError(explanation)


def sha_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open('rb') as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b''):
            digest.update(chunk)
    return digest.hexdigest()


def source_sha(path: str, root: Path = ROOT) -> str:
    return subprocess.check_output(['git', 'rev-parse', 'HEAD:' + path], cwd=root, text=True).strip()


def load_manifest(path: Path = MANIFEST) -> dict:
    return json.loads(path.read_text(encoding='utf-8'))


def verify_manifest_contract(contract: dict) -> None:
    require(contract['contract_version'] == 1, 'Unrecognized manifest contract version')
    require(contract['status'] == 'NONRELEASE_REQUIREMENTS_FIXED_VALIDATION_INCOMPLETE', 'Premature distribution approval')
    require(contract['build_evidence']['release_approved'] is False, 'Nonrelease build cannot become approved')
    require(contract['source_sha'] == '57152222a1ef523fe6e2f5079871672423c6cb56', 'Unknown baseline release build')
    require(contract['architecture'] == 'x86_64-pc-windows-msvc', 'Wrong target architecture')
    require(tuple(contract['combined_optimized_exes']) == COMBINED, 'Combined five-program inventory changed')
    require(tuple(contract['separately_required_windows_exes']) == SIDECARS, 'Two separate Windows executables missing')
    require(
        set(contract['separately_required_development_symbols']) == {name[:-4] + '.pdb' for name in SIDECARS},
        'Two separate PDBs missing from developer distribution requirements',
    )
    require(
        set(contract['full_installer_binaries']) == set(COMBINED + SIDECARS)
        and len(contract['full_installer_binaries']) == 7,
        'Official seven-program Windows installer roster drifted',
    )
    members = contract['build_evidence']['combined_members']
    expected = {name for exe in COMBINED for name in (exe, exe[:-4] + '.pdb')}
    require(set(members) == expected, 'Optimized archive ten-member manifest changed')
    require(contract['build_evidence']['run_id'] == 40922368730, 'Missing exact green artifact run')
    for name, detail in members.items():
        require(isinstance(detail['bytes'], int) and detail['bytes'] > 4096, 'Invalid member size: ' + name)
        require(bool(re.fullmatch(r'[0-9a-f]{64}', detail['sha256'])), 'Invalid member digest: ' + name)
    require(
        bool(re.fullmatch(r'[0-9a-f]{64}', contract['build_evidence']['combined_archive_sha256'])),
        'Invalid combined archive SHA-256',
    )
    for key, value in contract['public_release_requirements'].items():
        require(value not in ('PASS', 'ACCEPTED', 'APPROVED', True), 'False release clearance: ' + key)


def verify_source_contract(contract: dict, root: Path = ROOT) -> dict:
    verify_manifest_contract(contract)
    for path, sha in {**contract['protected_bazel_blobs'], **contract['protected_security_blobs']}.items():
        require(source_sha(path, root) == sha, 'Protected source drift: ' + path)
    require((root / 'codex-rs/codexdd-version.txt').read_text().strip() == '0.4.4', 'Premature CodexDD promotion')
    require((root / 'codex-rs/upstream-codex-release.txt').read_text().strip() == 'rust-v0.162.0', 'Premature upstream marker promotion')
    package = (root / 'codex-rs/BUILD.bazel').read_text(encoding='utf-8')
    require('actual = ":runtime_package_combined"' in package, 'Default archive alias changed')
    require('name = "runtime_package_combined"' in package, 'Combined target missing')
    combined = package.split('name = "runtime_package_combined"', 1)[1].split('name = "runtime_package_primary"', 1)[0]
    for target in (
        '//codex-rs/cli:release_files', '//codex-rs/app-server:release_files',
        '//codex-rs/code-mode-host:release_files',
        '//codex-rs/windows-sandbox-rs:command-runner-release_files',
        '//codex-rs/windows-sandbox-rs:sandbox-setup-release_files',
    ):
        require(target in combined, 'Missing required combined Bazel target: ' + target)
    for target in ('//codex-rs/windows-sandbox-service:release_files', '//codex-rs/responses-api-proxy:release_files'):
        require(target not in combined, 'Separate component accidentally folded into combined archive: ' + target)
    require('//codex-rs/windows-sandbox-service:release_files' in package.split('name = "runtime_package_helpers"', 1)[1], 'Service helper target missing')
    require('//codex-rs/responses-api-proxy:release_files' in package.split('name = "runtime_package_primary"', 1)[1], 'Responses proxy target missing')
    workflow = (root / '.github/workflows/rust-release-windows.yml').read_text(encoding='utf-8')
    match = re.search(r'^\s*WINDOWS_BINARIES:\s*"([^"]+)"\s*$', workflow, re.M)
    require(match is not None, 'Official Windows release program list missing')
    require({name + '.exe' for name in match.group(1).split()} == set(COMBINED + SIDECARS), 'Official Windows signing list drifted')
    for necessary in ('Sign Windows release files with Azure Trusted Signing', 'Download matching native voice build', 'Build Codex package archives', 'Verify packaged Windows voice closure', 'Verify and stage signed PowerShell installer'):
        require(necessary in workflow, 'Required official Windows release step missing: ' + necessary)
    matrix = json.loads((root / 'docs/upstream-candidates/codexdd-045-109-semantic-owner-matrix.json').read_text(encoding='utf-8'))
    require(len(matrix['paths']) == 109 and all(p['semantic_status'] in ('PENDING_OWNER_REVIEW', 'OWNER_ACCEPTED') and p['release_status'] == 'NOT_ACCEPTED' for p in matrix['paths']), 'Invalid semantic owner state or premature product release')
    pending = sum(p['semantic_status'] != 'OWNER_ACCEPTED' for p in matrix['paths'])
    return {'result': 'SOURCE_DISTRIBUTION_REQUIREMENTS_VERIFIED_NONRELEASE', 'source_sha': contract['source_sha'], 'optimized_combined_exes': 5, 'full_windows_exes_required': 7, 'owner_paths_pending': pending, 'release_approved': False}


def verify_combined_archive(contract: dict, path: Path) -> dict:
    from codexdd_045_verify_windows_archive import inspect_archive
    verify_manifest_contract(contract)
    result = inspect_archive(path)
    require(result['archive_sha256'] == contract['build_evidence']['combined_archive_sha256'], 'Combined archive digest differs from the green 3F.48 run')
    require(result['files'] == contract['build_evidence']['combined_members'], 'Combined archive member hashes or lengths differ')
    return {'result': 'SOURCE_FROZEN_COMBINED_ARCHIVE_BYTES_VERIFIED_NONRELEASE', 'archive_sha256': result['archive_sha256'], 'member_count': len(result['files']), 'release_approved': False}


def verify_sidecars(folder: Path) -> dict:
    require(folder.is_dir() and not folder.is_symlink(), 'Separate component folder must exist and not be a symlink')
    required = {name for exe in SIDECARS for name in (exe, exe[:-4] + '.pdb')}
    actual = {item.name for item in folder.iterdir()}
    require(actual == required, 'Expected exact standalone service and proxy EXE/PDB files')
    out = {}
    for name in sorted(required):
        file = folder / name
        require(file.is_file() and not file.is_symlink() and file.stat().st_size > 4096, 'Missing, linked, or too-small supplemental file: ' + name)
        with file.open('rb') as source:
            prefix = source.read(32)
        require(prefix.startswith(SIG_PDB if name.endswith('.pdb') else b'MZ'), 'Not an MSVC executable/symbol file: ' + name)
        out[name] = {'sha256': sha_file(file), 'bytes': file.stat().st_size}
    return {'result': 'FOUR_SUPPLEMENTAL_WINDOWS_BYTES_VERIFIED_NONRELEASE', 'files': out, 'release_approved': False, 'source_provenance_approved': False}


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-only', action='store_true')
    parser.add_argument('--combined-archive', type=Path)
    parser.add_argument('--supplemental-dir', type=Path)
    args = parser.parse_args(argv)
    if args.supplemental_dir and not args.combined_archive:
        parser.error('--supplemental-dir requires --combined-archive')
    if not args.source_only and not args.combined_archive:
        parser.error('Pass --source-only and/or --combined-archive')
    try:
        contract = load_manifest()
        report = {'source_check': verify_source_contract(contract)}
        if args.combined_archive:
            report['combined_check'] = verify_combined_archive(contract, args.combined_archive)
        if args.supplemental_dir:
            report['supplemental_check'] = verify_sidecars(args.supplemental_dir)
        report['release_approved'] = False
        report['all_public_release_requirements_met'] = False
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0
    except (DistributionError, KeyError, ValueError, OSError, subprocess.CalledProcessError) as exc:
        print('BLOCKED: ' + str(exc), file=sys.stderr)
        return 1


if __name__ == '__main__':
    raise SystemExit(main())
