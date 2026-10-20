"""CodexDD 0.4.5 packet 3C.4a: offline sandbox-source provenance checks.

This is a source-only checkpoint, not Rust compilation, a Windows security
test receipt, a release gate, or permission to publish.
"""

from __future__ import annotations

import hashlib
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"

# The fork did not modify these files relative to tracked official 0.162.0.
# Stage exact pinned official rust-v0.163.0 blobs before resolving callers.
STAGED = {
    "codex-rs/core/src/tools/orchestrator.rs": (
        "3d7e2b62c5227910980c56fc6be5f426d35e20c1",
        "e7f544458bafcb27fb9b86bd8b5828e95cbc6baf",
    ),
    "codex-rs/core/src/tools/sandboxing.rs": (
        "bf1fd2d79e6adae7bcb7cc1aa7cd36df1221bced",
        "d49273beb37fadf1dda778ef5ba8a047438dff69",
    ),
    "codex-rs/core/src/tools/sandboxing_tests.rs": (
        "95731415baf4906cf0a2834b991264df0cff9ba4",
        "a4839cfd33da520cb53b41107a038a9e60f7470c",
    ),
    "codex-rs/core/src/sandboxing/mod.rs": (
        "69f6ff90447ffd6389bc0d0972c3cf4d6f4bfcea",
        "e3ff8053b0f29a68e8bd4e3d10d5d6bdad230598",
    ),
    "codex-rs/protocol/src/sandbox.rs": (
        "b0304634ad8fb45be9b35b48591ef6c6a4e62099",
        "47fd51cbce1785a08cf199bf52effa93dd9315a3",
    ),
    "codex-rs/core/src/tools/runtimes/apply_patch.rs": (
        "45cc7e2e9b08e163f50054fd43051091f629eb44",
        "981958b17a18c06a88a5f618e2d5fc685d539188",
    ),
}


def committed(ref: str, path: str) -> bytes:
    return subprocess.run(
        ["git", "show", ref + ":" + path],
        cwd=ROOT, check=True, capture_output=True,
    ).stdout


def blob(raw: bytes) -> str:
    return hashlib.sha1(
        b"blob " + str(len(raw)).encode("ascii") + b"\x00" + raw
    ).hexdigest()


def src(path: str) -> str:
    return committed("HEAD", path).decode("utf-8")


class SandboxContractStagingTests(unittest.TestCase):
    def test_six_upstream_sources_are_exact_official_163_git_blobs(self):
        for path, (_, official_blob) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(blob(committed("HEAD", path)), official_blob)

    def test_fork_had_no_162_delta_on_staged_sources(self):
        for path, (production_blob, _) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(blob(committed(PRODUCTION, path)), production_blob)

    def test_restricted_escalation_still_requires_native_sandbox(self):
        protocol = src("codex-rs/protocol/src/sandbox.rs")
        sandbox = src("codex-rs/core/src/tools/sandboxing.rs")
        self.assertIn("pub enum SandboxOverride", protocol)
        self.assertIn("EscalatedSandboxWithRestrictions", protocol)
        self.assertIn("pub(crate) use codex_protocol::sandbox::SandboxOverride;", sandbox)
        self.assertIn("pub(crate) fn ensure_native_sandbox(", sandbox)
        self.assertIn("sandbox == SandboxType::None", sandbox)
        self.assertIn("requires an available filesystem sandbox", sandbox)

    def test_first_retry_and_network_approval_attempts_retain_override(self):
        orchestrator = src("codex-rs/core/src/tools/orchestrator.rs")
        self.assertIn("ensure_native_sandbox(sandbox_override, initial_sandbox)?;", orchestrator)
        self.assertEqual(orchestrator.count("SandboxAttempt {"), 3)
        self.assertGreaterEqual(orchestrator.count("sandbox_override,"), 3)
        self.assertIn("sandbox_override: attempt.sandbox_override", orchestrator)

    def test_sandbox_execution_preparation_is_async_and_carries_override(self):
        sandbox = src("codex-rs/core/src/tools/sandboxing.rs")
        core = src("codex-rs/core/src/sandboxing/mod.rs")
        self.assertIn("pub async fn env_for(", sandbox)
        self.assertIn("pub async fn env_for_exec_server(", sandbox)
        self.assertEqual(sandbox.count("request.sandbox_override = self.sandbox_override;"), 2)
        self.assertIn("sandbox_override: self.sandbox_override", sandbox)
        self.assertIn("pub(crate) async fn from_sandbox_exec_request(", core)
        self.assertIn("codex_exec_server::run_integrity_checks(&request).await;", core)

    def test_windows_denied_reads_regression_kept_async(self):
        tests = src("codex-rs/core/src/tools/sandboxing_tests.rs")
        self.assertIn("async fn windows_sandbox_env_preserves_denied_reads_or_rejects_unsupported_backend", tests)
        self.assertIn("ensure_native_sandbox(SandboxOverride::EscalatedSandboxWithRestrictions, SandboxType::None)", tests)
        self.assertIn("attempt.sandbox_override = SandboxOverride::EscalatedSandboxWithRestrictions", tests)

    def test_apply_patch_exec_server_retains_override(self):
        patch = src("codex-rs/core/src/tools/runtimes/apply_patch.rs")
        self.assertIn("sandbox_override: attempt.sandbox_override", patch)
        self.assertIn("fn file_system_sandbox_context_for_attempt(", patch)

    def test_release_and_lockfile_remain_unadvanced(self):
        self.assertEqual(committed("HEAD", "codex-rs/codexdd-version.txt").strip(), b"0.4.4")
        self.assertEqual(
            committed("HEAD", "codex-rs/upstream-codex-release.txt").strip(),
            b"rust-v0.162.0",
        )
        self.assertEqual(
            blob(committed("HEAD", "codex-rs/Cargo.lock")),
            "48ae4fb45a83de4d72dee6200b83a770dbf8f760",
        )

    def test_checkpoint_explicitly_requires_companion_reconciliation(self):
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c4-sandbox-contract.md").read_text(
            encoding="utf-8"
        )
        for marker in (
            "INCOMPLETE / NOT A RELEASE CANDIDATE",
            "unified_exec",
            "exec-server",
            "native Rust",
            "9/9",
        ):
            self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
