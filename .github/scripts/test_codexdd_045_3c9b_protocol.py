"""CodexDD 0.4.5 3C.9b: protocol permission and tool env source guards.

Offline static provenance only; no Rust runtime, permission proof,
generated schema acceptance, or authorization to publish.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
STAGED = {
    "codex-rs/protocol/src/error.rs": [
        "9e44430d6f911c284ee418b1c11ca0e0b3ad38c5",
        "8dfe556d94c8300dd10e6f93b10421eb1674567c"
    ],
    "codex-rs/protocol/src/permissions.rs": [
        "140090f5ef1649ed9e4717643dc85773be93da5f",
        "bd6757543a56063ecbced6dd49e52241dffb4af8"
    ],
    "codex-rs/protocol/src/shell_environment_tests.rs": [
        "c987f631db816339182d33d0e4083c773e325bce",
        "8167a2afa146f0c48ecfed711412039624ea491e"
    ]
}
MERGED_PROTOCOL = "9ff46a4e1331e15f35f95b8e17ba41dfc931a5cf"
TOOL_CALL_ENV = "0c6c515c416d3829122e8037a3b9c49c1a581608"


def git_blob(ref: str, path: str) -> str | None:
    result = subprocess.run(["git", "rev-parse", "--verify", ref + ":" + path],
                            cwd=ROOT, capture_output=True, text=True)
    return result.stdout.strip() if result.returncode == 0 else None


def source(path: str) -> str:
    return subprocess.run(["git", "show", "HEAD:" + path], cwd=ROOT,
                          check=True, capture_output=True, text=True,
                          encoding="utf-8").stdout


class ProtocolErrorPermissionsIntegrationTests(unittest.TestCase):
    def assertIn(self, value, whole, msg=None):
        if isinstance(whole, str) and len(whole) > 2048:
            self.assertTrue(value in whole, msg or f"missing source marker: {value!r}")
            return
        super().assertIn(value, whole, msg)

    def test_three_official_protocol_blobs_staged_exactly(self):
        self.assertEqual(len(STAGED), 3)
        for path, (_, official) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob("HEAD", path), official)

    def test_three_sources_unmodified_by_original_fork(self):
        for path, (original, _) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob(PRODUCTION, path), original)

    def test_codexdd_protocol_custom_merge_not_overwritten(self):
        self.assertEqual(git_blob("HEAD", "codex-rs/protocol/src/protocol.rs"), MERGED_PROTOCOL)

    def test_canonical_tool_call_environment_source_is_staged(self):
        self.assertEqual(git_blob("HEAD", "codex-rs/protocol/src/shell_environment.rs"), TOOL_CALL_ENV)

    def test_protocol_error_conversion_remains_defined(self):
        errors = source("codex-rs/protocol/src/error.rs")
        self.assertIn("CodexErr", errors)
        self.assertIn("ResponseIncompleteError", errors)
        self.assertIn("to_error_event", errors)

    def test_permission_source_retains_read_deny_and_protected_metadata(self):
        permissions = source("codex-rs/protocol/src/permissions.rs")
        for marker in ("ReadDenyMatcher", "FileSystemSandboxPolicy",
                       "forbidden_agent_metadata_write"):
            self.assertIn(marker, permissions)

    def test_windows_tool_call_id_case_fold_test_is_staged(self):
        tests = source("codex-rs/protocol/src/shell_environment_tests.rs")
        self.assertIn("tool_call_id_replaces_platform_equivalent_names", tests)
        self.assertIn("CODEX_TOOL_CALL_ID", tests)

    def test_adaptive_policy_is_unchanged(self):
        self.assertEqual(git_blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
                         "fc75b3bbf91f1881bd32faaaca93356fbb598b5e")

    def test_release_versions_and_original_lockfile_remain(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(git_blob("HEAD", "codex-rs/Cargo.lock"),
                         "096939fcec32cb259bccc0cb36c5d4120701da57")

    def test_protocol_packet_is_not_a_release_gate(self):
        doc = (ROOT / "docs/codexdd-0.4.5-packet-3c9b-protocol.md").read_text(encoding="utf-8")
        for marker in ("INCOMPLETE / NOT A RELEASE CANDIDATE",
                       "native Rust", "Cargo.lock", "source-only"):
            self.assertIn(marker, doc)


if __name__ == "__main__":
    unittest.main()
