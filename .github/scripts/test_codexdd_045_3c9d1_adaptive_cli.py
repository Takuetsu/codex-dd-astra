"""CodexDD 0.4.5 3C.9d1: bounded TUI adaptive CLI and slash routing.

Offline static provenance only. These additions must not enable untrusted
adaptive signals, grant permissions, or authorize release.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
RECONCILED = {
    "codex-rs/tui/src/cli.rs": [
        "841e9e282941bc856c8c7acf62f094c03c274b58",
        "da5c0e4f1ba02b7ff68811323945b0fd4d26f697"
    ],
    "codex-rs/tui/src/slash_command.rs": [
        "edd98b26ffdfba98d095ca6631830f2571928fe7",
        "36e5def7042e71baeeb87945de4fa0b029cd177a"
    ]
}


def blob(ref: str, path: str) -> str:
    return subprocess.run(
        ["git", "rev-parse", "--verify", ref + ":" + path],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout.strip()


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT, check=True, capture_output=True, text=True, encoding="utf-8",
    ).stdout


class TuiAdaptiveUiMergeTests(unittest.TestCase):
    def assertIn(self, marker, container, msg=None):
        if isinstance(container, str) and len(container) > 2048:
            self.assertTrue(marker in container, msg or f"missing marker: {marker!r}")
            return
        super().assertIn(marker, container, msg)

    def test_two_reviewed_fork_blobs_pinned(self):
        self.assertEqual(len(RECONCILED), 2)
        for path, (production, merged) in RECONCILED.items():
            with self.subTest(path=path):
                self.assertEqual(blob(PRODUCTION, path), production)
                self.assertEqual(blob("HEAD", path), merged)

    def test_tui_cli_preserves_external_adaptive_option(self):
        cli = source("codex-rs/tui/src/cli.rs")
        self.assertIn('long = "adaptive"', cli)
        self.assertIn("pub adaptive: Option<String>,", cli)

    def test_slash_command_has_adaptive_variant(self):
        cmd = source("codex-rs/tui/src/slash_command.rs")
        self.assertIn("    Adaptive,", cmd)
        self.assertIn("SlashCommand::Adaptive", cmd)

    def test_slash_command_description_remains_bounded(self):
        cmd = source("codex-rs/tui/src/slash_command.rs")
        self.assertIn("control thread-local adaptive effort", cmd)

    def test_slash_command_match_arms_remain_exhaustive_for_adaptive(self):
        cmd = source("codex-rs/tui/src/slash_command.rs")
        self.assertGreaterEqual(cmd.count("SlashCommand::Adaptive"), 3)

    def test_cli_launcher_routes_flag_to_tui_config(self):
        self.assertEqual(
            blob("HEAD", "codex-rs/cli/src/main.rs"),
            "9379d52b41473f1a57cb49d07048af7c139ebcf9",
        )

    def test_chatwidget_dispatch_still_requires_three_way_review(self):
        path = "codex-rs/tui/src/chatwidget/slash_dispatch.rs"
        self.assertEqual(blob("HEAD", path), blob(PRODUCTION, path))

    def test_adaptive_registry_policy_unchanged(self):
        self.assertEqual(
            blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )

    def test_product_versions_and_cargo_lock_not_advanced(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(blob("HEAD", "codex-rs/Cargo.lock"),
                         "0c913b46674d3593ee84f1fee5508d16e33efae5")

    def test_packet_still_incomplete(self):
        doc = (ROOT / "docs/codexdd-0.4.5-packet-3c9d1-adaptive-cli.md").read_text(encoding="utf-8")
        for marker in ("INCOMPLETE / NOT A RELEASE CANDIDATE",
                       "source-only", "native Rust", "Cargo.lock", "42"):
            self.assertIn(marker, doc)


if __name__ == "__main__":
    unittest.main()
