"""Nonrelease 3F.50 seven-binary distribution: source/payload negative checks."""
from __future__ import annotations

import hashlib
import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'scripts'))
import codexdd_045_distribution_gate as gate


class WindowsDistributionContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.manifest = gate.load_manifest()

    def test_exact_committed_source_and_nonrelease_contract(self):
        source = gate.verify_source_contract(self.manifest)
        self.assertEqual(source['optimized_combined_exes'], 5)
        self.assertEqual(source['full_windows_exes_required'], 7)
        self.assertFalse(source['release_approved'])
        self.assertGreaterEqual(source['owner_paths_pending'], 0)

    def test_five_plus_two_roster_and_provenance_are_strict(self):
        gate.verify_manifest_contract(self.manifest)
        self.assertEqual(set(self.manifest['full_installer_binaries']), set(gate.COMBINED + gate.SIDECARS))
        self.assertEqual(len(self.manifest['build_evidence']['combined_members']), 10)
        self.assertEqual(self.manifest['build_evidence']['run_id'], 40922368730)
        self.assertFalse(self.manifest['build_evidence']['release_approved'])

    def test_missing_supplemental_exes_or_pdbs_fail_closed(self):
        with tempfile.TemporaryDirectory() as temp:
            with self.assertRaisesRegex(gate.DistributionError, 'exact standalone'):
                gate.verify_sidecars(Path(temp))

    def test_synthetic_header_hashes_then_corruption_or_extra_file_fail(self):
        with tempfile.TemporaryDirectory() as temp:
            folder = Path(temp)
            names = {name for exe in gate.SIDECARS for name in (exe, exe[:-4] + '.pdb')}
            for name in names:
                start = gate.SIG_PDB if name.endswith('.pdb') else b'MZ'
                (folder / name).write_bytes(start + b'\x00' * 8192)
            report = gate.verify_sidecars(folder)
            self.assertEqual(set(report['files']), names)
            self.assertEqual(report['result'], 'FOUR_SUPPLEMENTAL_WINDOWS_BYTES_VERIFIED_NONRELEASE')
            self.assertFalse(report['source_provenance_approved'])
            (folder / 'codex-responses-api-proxy.pdb').write_bytes(b'bad' * 3000)
            with self.assertRaisesRegex(gate.DistributionError, 'Not an MSVC'):
                gate.verify_sidecars(folder)
            (folder / 'codex-responses-api-proxy.pdb').write_bytes(gate.SIG_PDB + b'\x00' * 8192)
            (folder / 'unapproved.exe').write_bytes(b'MZ' + b'\x00' * 8192)
            with self.assertRaisesRegex(gate.DistributionError, 'exact standalone'):
                gate.verify_sidecars(folder)

    def test_swapped_archive_sha_cannot_be_relabelled_green(self):
        manifest = self.manifest
        forged = {
            'archive_sha256': 'f' * 64,
            'files': manifest['build_evidence']['combined_members'],
        }
        with mock.patch.dict(sys.modules, {'codexdd_045_verify_windows_archive': mock.Mock(inspect_archive=lambda p: forged)}):
            with self.assertRaisesRegex(gate.DistributionError, 'digest differs'):
                gate.verify_combined_archive(manifest, Path('fake.tar.gz'))

    def test_unknown_release_gate_rejected(self):
        changed = json.loads(json.dumps(self.manifest))
        changed['public_release_requirements']['release_authorization'] = 'APPROVED'
        with self.assertRaisesRegex(gate.DistributionError, 'False release clearance'):
            gate.verify_manifest_contract(changed)


if __name__ == '__main__':
    unittest.main()
