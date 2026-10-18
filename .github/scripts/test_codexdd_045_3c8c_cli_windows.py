"""CodexDD 0.4.5 3C.8c: CLI Windows startup and adaptive governance.

Source-only offline provenance and policy markers. No Rust compilation,
Windows elevation proof, runtime auth check, or release acceptance.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
EXACT = {
    "codex-rs/cli/src/cloud_config.rs": [
        "5e87d12b0d82aaa7ba0e946239938a1b98321f3e",
        "9dd74aae8d5b4b9613b263e05f7e3e18ec4ea0b6"
    ],
    "codex-rs/cli/src/debug_sandbox.rs": [
        "73dd99cfe85a1dea9433e67016e40c405d47eb82",
        "c0efcdb1f6e308a56ba8d5a2d734f449862f1380"
    ],
    "codex-rs/cli/src/doctor.rs": [
        "b9f26e9d8693870e15a5f4a1e3fdb19a0f885cff",
        "533cf0d4e8e028368aa453f17b3cc7895c8ae18e"
    ],
    "codex-rs/cli/src/doctor/sandbox.rs": [
        "10a7d351f49a245bc8e82f9eef8a926bd73653c3",
        "4e1d54bb192338caf159e53a58154e0aa9a76466"
    ],
    "codex-rs/cli/src/lib.rs": [
        "c008567b96a899a30baa2afc9a712e47dd477880",
        "f35544276be3bab516320bad75f28354828b683e"
    ],
    "codex-rs/cli/src/mcp_cmd.rs": [
        "ac4b0154b36ea7db008ce3e47de2d3ac06320b4e",
        "02eb45c3e43aba947be26e2a8181746b121868f9"
    ],
    "codex-rs/cli/src/mcp_cmd_tests.rs": [
        "98ebe6a7090bd1e4f84df1015ec8aa53f59d26ca",
        "c968695f3918b6c86573fa5b7aa6cafa41889aa2"
    ],
    "codex-rs/cli/src/plugin_cmd.rs": [
        "d2824526e73cf28c7e82d2632ef66d9e4561840c",
        "e32c1d8d1933a7415a5fb4e331859e5cbdae3bbd"
    ],
    "codex-rs/cli/src/plugin_cmd_tests.rs": [
        "",
        "db49273d84679a27cc5e1ac79464b20de26987eb"
    ],
    "codex-rs/cli/tests/daemon_telemetry.rs": [
        "5e265992f3e41de2b3a44c4a1db1e0102aaa0bcc",
        "1d34cefe14b92efcc6cdf01c1f94566b1498d785"
    ],
    "codex-rs/cli/tests/exec_server.rs": [
        "8f4f8b742e78bf03935020c4dacf8066c8a4a95f",
        "cc100b7f6c73aff373b15e35af7c2fe00803ee0d"
    ],
    "codex-rs/cli/tests/plugin_cli.rs": [
        "b1c316860ca1ca87b37c89d7b823b9c6fff42077",
        "ea80f55a849a6fb5283e511cddbdd518d25e526d"
    ],
    "codex-rs/cli/tests/queue.rs": [
        "b4f5f77a59a8965c013788c3f2555330c04e25cc",
        "a10615d86c01666bb285dfa0a4fc54438616fee4"
    ],
    "codex-rs/cli/tests/sandbox_network_proxy.rs": [
        "21747c3919524a38ca5a285a058892319b49fade",
        "a21b5b4ee8c36b83bb47d302bba744b5665083af"
    ],
    "codex-rs/cli/tests/worktree.rs": [
        "928a57e6495b68ab4dca15daf44d7322650503ea",
        "664b2e1d65d78866737d2741a43d03532365f831"
    ]
}
MERGED = {
    "codex-rs/cli/BUILD.bazel": [
        "5da93d14962eb0a19f5c14542e62d2d0d68f6ee0",
        "d9673f989f062b5ea2e3f4bba124295bf010f7da",
        "5dd2b8b9629275fb2c392c59134ea95a3ce317d5"
    ],
    "codex-rs/cli/src/main.rs": [
        "173d95554ec555bd4ea5e04408a95d350f209015",
        "9d540c3124ee692250d345719d122aa1884ba12f",
        "9379d52b41473f1a57cb49d07048af7c139ebcf9"
    ],
    "codex-rs/cli/tests/daemon_startup.rs": [
        "a530450d73cb88cea034b2792ae19cb83400d6f6",
        "f50758363578497af6e1139e8a1410858706e38c",
        "e1c6d330f602f3c4cebf9d7163ecdcd2adb9100f"
    ]
}


def blob(ref: str, path: str) -> str | None:
    process = subprocess.run(
        ["git", "rev-parse", "--verify", ref + ":" + path],
        cwd=ROOT, capture_output=True, text=True,
    )
    return process.stdout.strip() if process.returncode == 0 else None


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT, check=True, capture_output=True, text=True, encoding="utf-8",
    ).stdout


class CliSourceReconciliationTests(unittest.TestCase):
    def assertIn(self, item, values, msg=None):
        if isinstance(values, str) and len(values) > 2048:
            self.assertTrue(item in values, msg or f"missing source marker: {item!r}")
            return
        super().assertIn(item, values, msg)

    def test_fifteen_exact_pinned_official_cli_blobs(self):
        self.assertEqual(len(EXACT), 15)
        for path, (before, after) in EXACT.items():
            with self.subTest(path=path):
                self.assertEqual(blob("HEAD", path), after)

    def test_original_fork_unchanged_in_fifteen_upstream_paths(self):
        for path, (before, _) in EXACT.items():
            with self.subTest(path=path):
                self.assertEqual(blob(PRODUCTION, path), before or None)

    def test_three_cli_conflicts_have_pinned_reviewed_merge_blobs(self):
        self.assertEqual(len(MERGED), 3)
        for path, (original, _, reconciled) in MERGED.items():
            with self.subTest(path=path):
                self.assertEqual(blob(PRODUCTION, path), original)
                self.assertEqual(blob("HEAD", path), reconciled)

    def test_bazel_still_uses_codexdd_version_file(self):
        bazel = source("codex-rs/cli/BUILD.bazel")
        self.assertIn('build_script_data = ["//codex-rs:codexdd-version.txt"]', bazel)
        self.assertIn('"CODEXDD_VERSION_FILE": "$(execpath //codex-rs:codexdd-version.txt)"', bazel)

    def test_startup_cli_carries_explicit_adaptive_assignment(self):
        cli = source("codex-rs/cli/src/main.rs")
        self.assertIn("        adaptive,", cli)
        self.assertIn("interactive.adaptive = adaptive;", cli)

    def test_elevated_windows_embedded_mode_has_no_f2_warning_dependency(self):
        tests = source("codex-rs/cli/tests/daemon_startup.rs")
        self.assertIn('steps.push_back(("GPT-5.6-Terra", b"/status\\r"));', tests)
        self.assertIn('"Model:"', tests)
        self.assertIn('elevated_windows_implicit_startup_uses_embedded_without_f2_warning', tests)

    def test_elevated_fork_and_resume_never_create_shared_daemon(self):
        tests = source("codex-rs/cli/tests/daemon_startup.rs")
        self.assertIn('ensure!(!pid_file.exists(), "unexpected shared daemon PID");', tests)
        self.assertIn('home.path().join("app-server-daemon")', tests)

    def test_restrictive_job_keeps_separate_warning_path(self):
        tests = source("codex-rs/cli/tests/daemon_startup.rs")
        self.assertIn('command == "restrictive-job"', tests)
        self.assertIn('b"\\x14"', tests)

    def test_new_cli_plugin_regressions_staged(self):
        self.assertIn("fn ", source("codex-rs/cli/src/plugin_cmd_tests.rs"))
        self.assertIn("fn ", source("codex-rs/cli/tests/plugin_cli.rs"))

    def test_windows_and_exec_server_integration_fixtures_staged(self):
        self.assertIn("fn ", source("codex-rs/cli/tests/exec_server.rs"))
        self.assertIn("fn ", source("codex-rs/cli/tests/sandbox_network_proxy.rs"))

    def test_codexdd_security_dispatch_blob_preserved(self):
        self.assertEqual(blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
                         "fc75b3bbf91f1881bd32faaaca93356fbb598b5e")

    def test_version_and_lockfile_not_yet_bumped(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(blob("HEAD", "codex-rs/Cargo.lock"),
                         "0c913b46674d3593ee84f1fee5508d16e33efae5")

    def test_release_remains_blocked(self):
        text = (ROOT / "docs/codexdd-0.4.5-packet-3c8c-cli-windows.md").read_text(encoding="utf-8")
        for marker in ("INCOMPLETE / NOT A RELEASE CANDIDATE", "source-only",
                       "native Rust", "Cargo.lock"):
            self.assertIn(marker, text)


if __name__ == "__main__":
    unittest.main()
