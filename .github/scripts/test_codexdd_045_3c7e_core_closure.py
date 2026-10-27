"""CodexDD 0.4.5 3C.7e active core source inventory; static only."""

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
STAGED = {
    "codex-rs/core/src/exec_policy/executable_identity.rs": (
        "47363da0e0426533de6888fdc2df69c8a013190e",
        "c85fa184bb1234566e486ba45cd65604956b9914",
    ),
    "codex-rs/core/src/exec_policy/executable_identity_tests.rs": (
        "9f90cd629ba0d333cb98817efc6ef763b4751900",
        "57be81541621a356d943bdac09b699da4836d23a",
    ),
    "codex-rs/core/src/mcp_tool_call/conversation_history.rs": (
        "a3aa36ee66861e00d3b460d9e9925f61d0718221",
        "7a68a95698e6202126b129ca895c508d35588bc5",
    ),
}


def blob(ref, path):
    return subprocess.run(
        ["git", "rev-parse", ref + ":" + path],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()


def source(path):
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    ).stdout


class ActiveCoreSourceTests(unittest.TestCase):
    def test_three_exact_upstream_blobs(self):
        self.assertEqual(len(STAGED), 3)
        for path, (_, upstream) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(blob("HEAD", path), upstream)

    def test_production_sources_not_customized(self):
        for path, (old, _) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(blob(PRODUCTION, path), old)

    def test_policy_identity_tests_present(self):
        self.assertIn(
            "#[test]",
            source("codex-rs/core/src/exec_policy/executable_identity_tests.rs"),
        )

    def test_mcp_history_source_present(self):
        self.assertIn(
            "history",
            source("codex-rs/core/src/mcp_tool_call/conversation_history.rs").lower(),
        )

    def test_old_unix_zsh_fork_not_registered(self):
        self.assertNotIn(
            "mod zsh_fork;", source("codex-rs/core/src/tools/runtimes/mod.rs")
        )

    def test_registry_and_release_still_unadvanced(self):
        self.assertEqual(
            blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(
            source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0"
        )

    def test_native_gate_outstanding(self):
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c7e-core-closure.md").read_text(
            encoding="utf-8"
        )
        for marker in (
            "INCOMPLETE / NOT A RELEASE CANDIDATE",
            "native Rust",
            "Cargo.lock",
        ):
            self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
