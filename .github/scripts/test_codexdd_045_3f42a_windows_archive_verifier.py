"""Source-only unit coverage of fail-closed Windows Bazel release tar validation."""
from __future__ import annotations

import importlib.util
import io
import json
import subprocess
import tarfile
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TARGET = ROOT / "scripts/codexdd_045_verify_windows_archive.py"
spec = importlib.util.spec_from_file_location("windows_archive_verifier", TARGET)
assert spec and spec.loader
verify = importlib.util.module_from_spec(spec)
spec.loader.exec_module(verify)


def fake_payload(name: str) -> bytes:
    if name.endswith(".exe"):
        return b"MZ" + b"\0" * 5000
    if name.endswith(".pdb"):
        return verify.PDB_SIGNATURE + b"\0" * 2048
    return b"X" * 2000


def build_tar(path: Path, names: list[str], overrides: dict | None = None) -> None:
    with tarfile.open(path, mode="w:gz") as tf:
        for name in names:
            payload = (overrides or {}).get(name, fake_payload(name))
            info = tarfile.TarInfo(name)
            info.size = len(payload)
            tf.addfile(info, io.BytesIO(payload))


class WindowsArchiveGuard(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.path = Path(self.temp.name) / "runtime_package_combined.tar.gz"

    def test_exact_five_executables_and_five_pdbs(self):
        self.assertEqual(len(verify.REQUIRED_FILES), 10)
        build_tar(self.path, sorted(verify.REQUIRED_FILES))
        result = verify.inspect_archive(self.path)
        self.assertEqual(result["file_count"], 10)
        self.assertFalse(result["release_approved"])
        self.assertEqual(set(result["files"]), verify.REQUIRED_FILES)
        self.assertEqual(len(result["archive_sha256"]), 64)

    def test_missing_pdb_is_a_hard_failure(self):
        build_tar(self.path, sorted(verify.REQUIRED_FILES - {"codex.pdb"}))
        with self.assertRaisesRegex(verify.ArchiveValidationError, "Missing required"):
            verify.inspect_archive(self.path)

    def test_duplicate_member_is_a_hard_failure(self):
        names = sorted(verify.REQUIRED_FILES) + ["codex.exe"]
        build_tar(self.path, names)
        with self.assertRaisesRegex(verify.ArchiveValidationError, "Duplicate archive"):
            verify.inspect_archive(self.path)

    def test_unknown_executable_is_a_hard_failure(self):
        build_tar(self.path, sorted(verify.REQUIRED_FILES) + ["unexpected.exe"])
        with self.assertRaisesRegex(verify.ArchiveValidationError, "Unexpected combined archive file"):
            verify.inspect_archive(self.path)

    def test_non_pe_or_non_pdb_content_is_rejected(self):
        for name in ("codex.exe", "codex.pdb"):
            with self.subTest(name=name):
                build_tar(self.path, sorted(verify.REQUIRED_FILES), {name: b"WRONG" * 2000})
                with self.assertRaisesRegex(verify.ArchiveValidationError, "Not a Windows PE|Not an MSVC PDB"):
                    verify.inspect_archive(self.path)

    def test_traversal_is_rejected_before_extraction(self):
        build_tar(self.path, sorted(verify.REQUIRED_FILES) + ["../codex.exe"])
        with self.assertRaisesRegex(verify.ArchiveValidationError, "Unsafe tar"):
            verify.inspect_archive(self.path)

    def test_symlink_is_never_followed(self):
        with tarfile.open(self.path, mode="w:gz") as tf:
            m = tarfile.TarInfo("codex.exe")
            m.type = tarfile.SYMTYPE
            m.linkname = "/c/secret"
            tf.addfile(m)
        with self.assertRaisesRegex(verify.ArchiveValidationError, "Unexpected special"):
            verify.inspect_archive(self.path)

    def test_release_source_manifest_and_cache_contract_stay_pinned(self):
        manifest = (ROOT / "codex-rs/BUILD.bazel").read_text(encoding="utf-8")
        combined = manifest.split('name = "runtime_package_combined"', 1)[1].split(
            'name = "runtime_package_primary"', 1
        )[0]
        required = (
            "//codex-rs/cli:release_files",
            "//codex-rs/app-server:release_files",
            "//codex-rs/code-mode-host:release_files",
            "//codex-rs/windows-sandbox-rs:command-runner-release_files",
            "//codex-rs/windows-sandbox-rs:sandbox-setup-release_files",
        )
        for item in required:
            self.assertIn(item, combined)
        self.assertNotIn("windows-sandbox-service:release_files", combined)
        self.assertNotIn("responses-api-proxy:release_files", combined)
        for path, expected in {
            "codex-rs/BUILD.bazel": "843e9fe095e0982df2043bb17c7da8e6e1c95025",
            "bazel/rules/runtime_package.bzl": "ba4cdad58f916f04602109c7b4d2512e83da5485",
            "codex-rs/cli/BUILD.bazel": "5dd2b8b9629275fb2c392c59134ea95a3ce317d5",
            "codex-rs/Cargo.lock": "13bc15ff844e93dfaf7777c73bb7313b35bb3b81",
            "codex-rs/deny.toml": "f282c7c55d05201e308e60804f85754c8522d462",
        }.items():
            with self.subTest(path=path):
                self.assertEqual(subprocess.check_output(
                    ["git", "rev-parse", "HEAD:" + path], cwd=ROOT, text=True
                ).strip(), expected)
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")


if __name__ == "__main__":
    unittest.main()
