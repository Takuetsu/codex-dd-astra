"""CodexDD 0.4.5 packet 3C.4c: sandbox-integrity source/provenance tripwires.

Read-only and offline. Not Rust compilation, native sandbox security validation,
CI acceptance, approval to grant privileges, or release authorization.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
STAGED = {
    "codex-rs/sandboxing/src/bwrap.rs": [
        "ebf7f8d60e02e2fd2e53e86ee8cf33a4a4cf5c2b",
        "876323ce074f1134a6057f92374ba49b48f5a193"
    ],
    "codex-rs/sandboxing/src/lib.rs": [
        "e65ead4d5e397cb8b6e14511f7454339ab2a974a",
        "5db90bba1ed508ca489297feebea342dc92254a0"
    ],
    "codex-rs/sandboxing/src/manager.rs": [
        "540754116ce9d512928e633e3ccf5179b59a4aca",
        "f6c281a061d0af01ddf0ee9d56646ea5c0943ad9"
    ],
    "codex-rs/sandboxing/src/seatbelt.rs": [
        "665f24f86d7f73145efcc7923ac12c93ca4be654",
        "0f8701a167117562d41fc372f63f4daee630cf73"
    ],
    "codex-rs/sandboxing/src/seatbelt_tests.rs": [
        "88ae9b97c3f067a2d09f2fbe15b935998563544d",
        "9f8083877182f4330b8c4c59fde0d9135f595985"
    ],
    "codex-rs/sandboxing/src/sandbox_integrity/contents.rs": [
        "",
        "219c71e81513150a049a23e2b793002c62a86dd9"
    ],
    "codex-rs/sandboxing/src/sandbox_integrity/contents_tests.rs": [
        "",
        "cd1e2df0f1a11972b926d5f678e9859f26ed747b"
    ],
    "codex-rs/sandboxing/src/sandbox_integrity/finding.rs": [
        "",
        "ea2945b900b46e498b22facf30e17013f3bc3ef1"
    ],
    "codex-rs/sandboxing/src/sandbox_integrity/mod.rs": [
        "",
        "063465855ae1c2f85e2e93c862f44455dada0c73"
    ],
    "codex-rs/sandboxing/src/sandbox_integrity/policy.rs": [
        "",
        "ebf9c9adc1afe07131072067eec9062bfbe014f9"
    ],
    "codex-rs/sandboxing/src/seatbelt_network_tests.rs": [
        "",
        "6c005cc4ecd37d325b396b7830c8fbd79bed4569"
    ],
    "codex-rs/exec-server/src/client.rs": [
        "0369bb7185d4a68d5ad77c47f06c2524c249aa35",
        "2ce504dabbbde62794bb36b3ea0e5d13075ff1ac"
    ],
    "codex-rs/exec-server/src/client/accepted.rs": [
        "ae6d5f84e7b2b305f0144830059dcfb3a00100e5",
        "61f12219fe01189ff4f1bd1a55411aa3a76fa47b"
    ],
    "codex-rs/exec-server/src/client/connection_failure.rs": [
        "5265d610c28bcb72090ddc12a48fbc8a74d62c4d",
        "3dd54f7925f2a907ca8dd128a8470256f7867422"
    ],
    "codex-rs/exec-server/src/client_error.rs": [
        "e77d56a0f76a6d3c7c881001ff04bb2d8d9b132a",
        "97487c21b03c360db68546964c7d3377fd4c01a3"
    ],
    "codex-rs/exec-server/src/client_refresh.rs": [
        "3579cb4f72f3abfa69256bb476178a7bcc098740",
        "43cae9282040133e650a7cf20eef8dc72fab5876"
    ],
    "codex-rs/exec-server/src/client_transport.rs": [
        "beb47a1c37662a2b6cc7a52080b98c49f8d42aa9",
        "85a56ca2cc714bd7be2feeb200300ad950095b77"
    ],
    "codex-rs/exec-server/src/client_transport_tests.rs": [
        "fe1966f8fc1e9709ce1d5a64cbac52c16921e1c3",
        "8858aed896d6eaa97bb87f4c16e47737b19f43d6"
    ],
    "codex-rs/exec-server/src/discoverV2/capability_discoveries.rs": [
        "bda6f20e67c221bf4077a3c88fb62f44b3bedcf9",
        "b681a417c0260118f66730da07e7beaba5fe942e"
    ],
    "codex-rs/exec-server/src/discoverV2/capability_manager.rs": [
        "2f3e37de5781400074e203ded7ee34471455ea69",
        "037a1c6afb80c4383fbd5342a06be1b7e76ccdab"
    ],
    "codex-rs/exec-server/src/discoverV2/capability_manager/capability_watchers.rs": [
        "be92ca91a7818da4ef601a736fafcdecc4b864b2",
        "11af0eb074d66b7e3ce5c31333f8891bf03130cf"
    ],
    "codex-rs/exec-server/src/discoverV2/mod.rs": [
        "faab1775e67a26df6e76ecb510b4b822069fcdec",
        "cfbe4c812dc6a7aa29b5c07351712bb0e62cb5da"
    ],
    "codex-rs/exec-server/src/discoverV2/tests/capability_discoveries_tests.rs": [
        "23f2a267ae8b429e05c4e71c9a83891d8bb84732",
        "dd144081ec7fb4137ce5b109fb8e6e1d2f957be5"
    ],
    "codex-rs/exec-server/src/discoverV2/tests/capability_manager_tests.rs": [
        "ca300006e1f95776cf6d9527f6bad8b3979f05f6",
        "996492181e84a3071f7d561b7cef31de4920d4c3"
    ],
    "codex-rs/exec-server/src/discoverV2/tests/capability_watchers_tests.rs": [
        "4f3484bf6abdafdf78ebc309b518c04b7c1e701c",
        "65ccf360bea6f55506c1a4a333c44c50052d631b"
    ],
    "codex-rs/exec-server/src/environment.rs": [
        "baa0f22049fb4f6e98c4150e1334d7ddb024c59c",
        "8c3d89b88dabd3822e2cc6b21e0b9bcad4696e81"
    ],
    "codex-rs/exec-server/src/fs_sandbox.rs": [
        "89058f3ff8d79b1eacf4eef4500ff4f95f5d8f4c",
        "f3e39c4e9425b04fb54d995ed3c91966442f2a2e"
    ],
    "codex-rs/exec-server/src/fs_sandbox_windows_tests.rs": [
        "099d5946234943a6e7a00d28f64fac7ea75223d9",
        "d21e72cf328ad5d58a02d00f1b85f3d73062a5d7"
    ],
    "codex-rs/exec-server/src/lib.rs": [
        "1a7c5c2aa62406c63cf358237c58f3ceb2296023",
        "adca2b271c5ab930096fbf4697755301ae5c8d6a"
    ],
    "codex-rs/exec-server/src/local_process.rs": [
        "f384d84d88787fd936ed2d052fe52884fb927dd0",
        "271f8aaaf02434dfa20bbe3573282c19f3a01ef6"
    ],
    "codex-rs/exec-server/src/process_sandbox.rs": [
        "8c81ce5ec56b59b49a14216eefd2e2a5c7b23fd6",
        "cb7f061961d528a3bf508fb3743b431f146727fc"
    ],
    "codex-rs/exec-server/src/sandboxed_file_system.rs": [
        "374058f0d0a1bb0af861ce82520cbbf6a43b0bb8",
        "f592edfbf374159b1e0f42b1e5fe8305c45d1e78"
    ],
    "codex-rs/exec-server/src/server/handler.rs": [
        "d4ecb1b07ff17c4db47bdb22a3a012c4fbabfa26",
        "e7f41edcba062431e7f172c172359b534e15ae3e"
    ],
    "codex-rs/exec-server/src/server/handler/tests.rs": [
        "00de894176995d5243c95ebb4cd49824548d8826",
        "bf42de8febb5b5577d5b96cad098012f199506aa"
    ],
    "codex-rs/exec-server/src/server/process_otel_tests.rs": [
        "93638142d87a27c63e693eb9f0cfd94de147dd60",
        "e7142c3dcb1badf27ef545d2989e2b848c25dc14"
    ],
    "codex-rs/exec-server/src/server/processor.rs": [
        "81cf9c54bfe03e4591d6790aa64a913d88634e56",
        "0db3bc38a39cc39df317b39ab5cbe1523d18467d"
    ],
    "codex-rs/exec-server/src/server/registry.rs": [
        "609a3a748e0761cc6a3154fd0d196a248e2f2dea",
        "842af18bee9f342c10133734db2d7b38c97c8f04"
    ],
    "codex-rs/exec-server/src/server/request_dispatcher_tests.rs": [
        "5da5ed0d2f58e011852814f1d6cd6d5f853bc689",
        "680a4bea22c7a76df8dafbffcce3a641f73b9ed6"
    ],
    "codex-rs/exec-server/src/server/transport.rs": [
        "94b48774d828b041fca068322973e24714574e71",
        "4bb1d1f1209de73a2a6deba69b6cc6c47edd37a0"
    ],
    "codex-rs/exec-server/src/shell_snapshot.rs": [
        "6619bbb733f60a76d9ac012359184aba11b41d08",
        "dfb02ff92a54e08e4d3112afe272c616a87a6c21"
    ],
    "codex-rs/exec-server/src/shell_snapshot_tests.rs": [
        "adc0ceb141ef5e1b2d5aa4b660403e2db01a3b30",
        "82a34ff73937270b0e42126fe6bcd729dac55837"
    ],
    "codex-rs/exec-server/src/client/tests/config_timeout_tests.rs": [
        "",
        "2c894403dac096590e52dc9dfe4796fb768b43b7"
    ],
    "codex-rs/exec-server/src/environment/discovery_recovery.rs": [
        "",
        "a4f23a004580e74f5a1fbfb6c9eaf9009cf5ab53"
    ],
    "codex-rs/exec-server/src/environment/discovery_v2.rs": [
        "",
        "ec927bae8c381dc6aeb504e38321f0b64747dfcb"
    ],
    "codex-rs/exec-server/src/environment/discovery_v2_tests.rs": [
        "",
        "db280f09d9fb59974c5336a3d2eb7b01cee76b8f"
    ],
    "codex-rs/exec-server/src/sandbox_integrity/AGENTS.md": [
        "",
        "adf9af66e2722618586ef77b828465be0f5b8918"
    ],
    "codex-rs/exec-server/src/sandbox_integrity/backend_tests.rs": [
        "",
        "998d0507790d086efbe91dab2bfe4575eea307c8"
    ],
    "codex-rs/exec-server/src/sandbox_integrity/bubblewrap.rs": [
        "",
        "337a79120e32cf91cd4e16a9659442d8acf01d37"
    ],
    "codex-rs/exec-server/src/sandbox_integrity/dependencies.rs": [
        "",
        "c07ab51f52fe9c591776b0f7c39b8de9ed5e7c19"
    ],
    "codex-rs/exec-server/src/sandbox_integrity/mod.rs": [
        "",
        "e355d096033f0f1d75bba05bad714a779b89dd16"
    ],
    "codex-rs/exec-server/src/sandbox_integrity/mxc.rs": [
        "",
        "8d4c7eefa19ff0a3b468cb6286df78270e1e4851"
    ],
    "codex-rs/exec-server/src/sandbox_integrity/runner.rs": [
        "",
        "76d566c429c990defe35f06b2af382cf2c8624ca"
    ],
    "codex-rs/exec-server/src/sandbox_integrity/runner_tests.rs": [
        "",
        "299f2a6983eb6b9fd02d5a99c8161f91c15d6ec9"
    ],
    "codex-rs/exec-server/src/sandbox_integrity/seatbelt.rs": [
        "",
        "ab1e4be93ac76a15f5bea45d7d57b463428dda87"
    ],
    "codex-rs/exec-server/src/sandbox_integrity/telemetry.rs": [
        "",
        "0c65de555fbfcb90682b7b2b51af7df4b5cc2c2c"
    ]
}


def git_blob(ref: str, path: str) -> str | None:
    result = subprocess.run(
        ["git", "rev-parse", ref + ":" + path],
        cwd=ROOT, capture_output=True, text=True,
    )
    return result.stdout.strip() if result.returncode == 0 else None


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT, capture_output=True, check=True, text=True,
        encoding="utf-8",
    ).stdout


class SandboxIntegritySourceTests(unittest.TestCase):

    # Keep failing static marker diagnostics bounded: never dump a 70KB Rust file.
    def assertIn(self, member, container, msg=None):
        if isinstance(container, str) and len(container) > 2048:
            self.assertTrue(member in container, msg or f"missing source marker: {member!r}")
            return
        super().assertIn(member, container, msg)

    def assertNotIn(self, member, container, msg=None):
        if isinstance(container, str) and len(container) > 2048:
            self.assertFalse(member in container, msg or f"unexpected source marker: {member!r}")
            return
        super().assertNotIn(member, container, msg)

    def test_fifty_five_exact_pinned_upstream_source_blobs(self):
        self.assertEqual(len(STAGED), 55)
        for path, (_, official_blob) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob("HEAD", path), official_blob)

    def test_no_original_codexdd_fork_source_was_replaced(self):
        for path, (original_blob, _) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob(PRODUCTION, path), original_blob or None)

    def test_exec_server_exports_integrity_checks_to_core(self):
        api = source("codex-rs/exec-server/src/lib.rs")
        module = source("codex-rs/exec-server/src/sandbox_integrity/mod.rs")
        self.assertIn("mod sandbox_integrity;", api)
        self.assertIn("pub use sandbox_integrity::run_integrity_checks;", api)
        self.assertIn("pub use runner::run_integrity_checks;", module)

    def test_integrity_checker_is_telemetry_only_and_uses_bounded_timeout(self):
        runner = source("codex-rs/exec-server/src/sandbox_integrity/runner.rs")
        self.assertIn("pub async fn run_integrity_checks(request: &SandboxExecRequest)", runner)
        self.assertIn("const CHECK_TIMEOUT: Duration = Duration::from_secs(/*secs*/ 5);", runner)
        self.assertIn("tokio::time::timeout(CHECK_TIMEOUT, work)", runner)
        self.assertIn("tokio::task::spawn_blocking", runner)
        self.assertIn("telemetry.record_check(", runner)

    def test_approved_escalation_is_skipped_before_preparation(self):
        runner = source("codex-rs/exec-server/src/sandbox_integrity/runner.rs")
        first = runner.index("request.sandbox_override == SandboxOverride::EscalatedSandboxWithRestrictions")
        prepare = runner.index("let policy = request.permission_profile.file_system_sandbox_policy();")
        self.assertLess(first, prepare)
        self.assertIn("return;", runner[first:prepare])

    def test_windows_backend_uses_mxc_without_enforcement_mutations(self):
        mxc = source("codex-rs/exec-server/src/sandbox_integrity/mxc.rs")
        self.assertIn("SandboxType::WindowsMxc", mxc)
        self.assertIn('pub(super) const NAME: &str = "windows_mxc";', mxc)
        self.assertIn("fn critical_dependencies(", mxc)
        self.assertIn("fn prepare_policy(", mxc)

    def test_sandboxing_crate_exports_checker_and_finding(self):
        lib = source("codex-rs/sandboxing/src/lib.rs")
        module = source("codex-rs/sandboxing/src/sandbox_integrity/mod.rs")
        self.assertIn("FileContentsChecker", lib)
        self.assertIn("IntegrityFinding", lib)
        self.assertIn("FileContentsChecker", module)
        self.assertIn("IntegrityFinding", module)

    def test_integrity_metrics_use_bounded_tags_not_paths(self):
        metrics = source("codex-rs/exec-server/src/sandbox_integrity/telemetry.rs")
        for marker in (
            '"codex.sandbox_integrity.duration_ms"',
            '"codex.sandbox_integrity.preparation_ms"',
            '"codex.sandbox_integrity.check"',
            '("error_kind", error_kind)',
        ):
            self.assertIn(marker, metrics)
        self.assertNotIn("error.to_string()", metrics)

    def test_executor_can_observe_integrity_from_sandbox_paths(self):
        fs = source("codex-rs/exec-server/src/fs_sandbox.rs")
        self.assertIn("run_integrity_checks", fs)
        self.assertIn("sandbox_override", fs)

    def test_native_regression_fixtures_are_present_but_unrun(self):
        self.assertIn("fn ", source("codex-rs/exec-server/src/sandbox_integrity/backend_tests.rs"))
        self.assertIn("fn ", source("codex-rs/exec-server/src/sandbox_integrity/runner_tests.rs"))
        self.assertIn("fn ", source("codex-rs/sandboxing/src/sandbox_integrity/contents_tests.rs"))

    def test_codexdd_0_4_4_version_and_policy_remain_unchanged(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(
            git_blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )
        self.assertEqual(git_blob("HEAD", "codex-rs/exec-server/Cargo.toml"),
                         "0d1f1f5209ca66c6efdd9f6bb54058e89bc7a542")

    def test_packet_is_not_a_release_candidate(self):
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c4c-exec-server-sandbox-integrity.md").read_text(
            encoding="utf-8"
        )
        for marker in ("INCOMPLETE / NOT A RELEASE CANDIDATE", "Cargo.lock", "native Rust", "telemetry-only"):
            self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
