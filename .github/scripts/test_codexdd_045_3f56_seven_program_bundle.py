"""3F.56 cheap fail-closed seven-program inputs: source/receipt tamper fixtures."""
from __future__ import annotations
import hashlib
import json
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "scripts"))
import codexdd_045_3f56_seven_program_assembler as assembler

TOOLING_SHA = "b" * 40
ACTION_SHA = "7e3457c3218c6933b8b30de65061ccf56c951619862e7d4f38ed9f2be086908e"


def create_component(root: Path, component: str, source_sha: str = assembler.BASELINE,
                     tooling_sha: str = TOOLING_SHA) -> Path:
    folder = root / component
    folder.mkdir()
    base = "codex-windows-sandbox-service" if component == "service" else "codex-responses-api-proxy"
    files = {}
    for suffix in (".exe", ".pdb"):
        name = base + suffix
        body = b"MZ" if suffix == ".exe" else assembler.SOURCE_MARKERS[1]
        payload = body + b"\x00" * 4500
        (folder / name).write_bytes(payload)
        files[name] = {
            "bytes": len(payload),
            "sha256": hashlib.sha256(payload).hexdigest(),
            "bazel_path": "bazel-out/fake/" + name,
        }
    receipt = {
        "contract_version": 1,
        "source_sha": source_sha,
        "workflow_sha": tooling_sha,
        "component": component,
        "result": "REAL_OPTIMIZED_MSVC_EXE_AND_PDB_VERIFIED_NONRELEASE",
        "target": {
            "service": "//codex-rs/windows-sandbox-service:release_files",
            "proxy": "//codex-rs/responses-api-proxy:release_files",
        }[component],
        "action_path_sha256": ACTION_SHA,
        "files": files,
        "release_approved": False,
    }
    (folder / (component + "-integrity.json")).write_text(json.dumps(receipt), encoding="utf-8")
    return folder


class SevenWindowsPrograms(unittest.TestCase):
    def test_four_real_sidecar_files_with_distinct_source_receipts(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            svc = create_component(root, "service")
            proxy = create_component(root, "proxy")
            files = assembler.verify_sidecar_provenance(svc, proxy, TOOLING_SHA)
            self.assertEqual(len(files), 4)
            self.assertEqual(set(files), {
                "codex-windows-sandbox-service.exe",
                "codex-windows-sandbox-service.pdb",
                "codex-responses-api-proxy.exe",
                "codex-responses-api-proxy.pdb",
            })
            self.assertTrue(all(x["source_sha"] == assembler.BASELINE for x in files.values()))

    def test_old_source_and_mismatched_tooling_fail_closed(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            svc = create_component(root, "service", source_sha="0" * 40)
            proxy = create_component(root, "proxy")
            with self.assertRaisesRegex(ValueError, "different source SHA"):
                assembler.verify_sidecar_provenance(svc, proxy, TOOLING_SHA)
            svc = root / "service"
            receipt = svc / "service-integrity.json"
            obj = json.loads(receipt.read_text())
            obj["source_sha"] = assembler.BASELINE
            receipt.write_text(json.dumps(obj))
            with self.assertRaisesRegex(ValueError, "tooling SHA mismatch"):
                assembler.verify_sidecar_provenance(svc, proxy, "1" * 40)

    def test_tampered_pdb_or_reused_folder_fails_closed(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            svc = create_component(root, "service")
            proxy = create_component(root, "proxy")
            (proxy / "codex-responses-api-proxy.pdb").write_bytes(b"corrupt" * 900)
            with self.assertRaisesRegex(ValueError, "Bad physical PE/PDB signature"):
                assembler.verify_sidecar_provenance(svc, proxy, TOOLING_SHA)
            with self.assertRaisesRegex(ValueError, "separate build receipts"):
                assembler.verify_sidecar_provenance(svc, svc, TOOLING_SHA)

    def test_extra_file_and_false_release_approval_fail(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            svc = create_component(root, "service")
            proxy = create_component(root, "proxy")
            (svc / "unexpected.txt").write_text("not allowed")
            with self.assertRaisesRegex(ValueError, "Unknown supplemental files"):
                assembler.verify_sidecar_provenance(svc, proxy, TOOLING_SHA)
            (svc / "unexpected.txt").unlink()
            receipt = svc / "service-integrity.json"
            obj = json.loads(receipt.read_text())
            obj["release_approved"] = True
            receipt.write_text(json.dumps(obj))
            with self.assertRaisesRegex(ValueError, "Premature sidecar release approval"):
                assembler.verify_sidecar_provenance(svc, proxy, TOOLING_SHA)

    def test_build_scripts_only_target_two_missing_components(self):
        script = (ROOT / ".github/scripts/codexdd-045-3f56-build-windows-sidecar.ps1").read_text()
        self.assertIn("'57152222a1ef523fe6e2f5079871672423c6cb56'", script)
        self.assertIn("//codex-rs/windows-sandbox-service:release_files", script)
        self.assertIn("//codex-rs/responses-api-proxy:release_files", script)
        self.assertIn("Non-deterministic hosted Ruby", script)
        self.assertIn("Protected source drift", script)
        self.assertIn("Windows compiler action PATH drift", script)
        self.assertIn("--config=release", script)
        self.assertIn("--disk_cache=D:/_bazel-disk", script)
        self.assertIn("NONRELEASE_SIDECAR_READY=", script)
        self.assertNotIn("//codex-rs:runtime_package_combined", script)
        self.assertNotIn("//codex-rs/cli:release_files", script)

if __name__ == "__main__":
    unittest.main()
