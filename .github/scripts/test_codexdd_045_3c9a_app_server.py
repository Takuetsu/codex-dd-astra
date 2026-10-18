"""CodexDD 0.4.5 3C.9a: app-server runtime source graph integration.

Offline static source and Git-blob assertions only. Not a native Rust,
Windows sandbox, approval, client transport, or release validation.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
UPSTREAM_FILES = {
    "codex-rs/app-server/src/config_manager.rs": [
        "21d1780910e9aabb2fd7a59a683cd363c103064a",
        "d5f2c2b89a6d64d9331b0c9c045a65cd5b5e527c"
    ],
    "codex-rs/app-server/src/config_manager_service_tests.rs": [
        "af60acb652992da6370c7dbe8b39d2e3a61e8472",
        "d390205cf57d3194d3dfb15385406ddfb8ce7dbf"
    ],
    "codex-rs/app-server/src/error_code.rs": [
        "b5b44c027430b0c8983938f0b226ba17fc54cd73",
        "ce5e1947d2fdce7ddbe0acba5820071ed6049a99"
    ],
    "codex-rs/app-server/src/external_agent_migration/session_importer.rs": [
        "33f28ed8787ac7c0c3a9b89722b0d862bbac4722",
        "13fbb936274f09d43d935c8afd8e1072e70cc33e"
    ],
    "codex-rs/app-server/src/in_process_bootstrap.rs": [
        "a21c3b0b96d02be44296e5575af73ebe867fb139",
        "17b37b9d6dc7d9a8803db001f7e38d902d3e04b7"
    ],
    "codex-rs/app-server/src/lib.rs": [
        "896aa6623b690bff7789a830699ae498f1a3b6dc",
        "70e8519ab84031347d837877582f57202102fdf3"
    ],
    "codex-rs/app-server/src/message_processor_thread_lifecycle_tests.rs": [
        "930a4d7a32c9a84b0ff69ea322f192cfc45da72f",
        "752452815d1ac74056764d369f25218e418f1c02"
    ],
    "codex-rs/app-server/src/outgoing_message.rs": [
        "961eccc2aad51cb17b3e9206ed95c34dab686c2e",
        "9ed210ebd3472daf7c98fa64d4ac59d45c4c0442"
    ],
    "codex-rs/app-server/src/plugin_config_reload.rs": [
        "13842d1ba0b67963726c14ffb0ee88e6e3fb6326",
        "72b301cf60884fc60a2696f1659b8216a63b4c4b"
    ],
    "codex-rs/app-server/src/request_processors/account_processor.rs": [
        "eeaba7882ed446a2efe4c34183d82b14a8de2f20",
        "b8d56053b11e6d4983a39c841bfd3fbbe1f830a5"
    ],
    "codex-rs/app-server/src/request_processors/account_processor/workspace_routing.rs": [
        "3eedf082f5c9a9a750cf882dbed4aac7c173eff8",
        "90c8781b5ba6d908ba6e67889ba23e41208feaf4"
    ],
    "codex-rs/app-server/src/request_processors/account_processor/workspace_routing_tests.rs": [
        "a7c1b8c7315635dfcf932e090cb8a76f86e82783",
        "cc06d5086e7fc03cac3886b127f7bbeb24c9a244"
    ],
    "codex-rs/app-server/src/request_processors/command_exec_processor.rs": [
        "0115a13bd6135ef5d308208663336a9c5ba59292",
        "1d5c3c7056057512591ff64d952947495176fe75"
    ],
    "codex-rs/app-server/src/request_processors/config_processor.rs": [
        "914abf4fd8b8e50d55b2049b761ab98c82fb899b",
        "1343329ae1df0f3992e0db111e4664a5a1d71048"
    ],
    "codex-rs/app-server/src/request_processors/daemon_snapshot.rs": [
        "038373f3a1081181530a6218df30c33dd00b8ac0",
        "81462202d16d6a3fb47e06e09842c5c5c0df7033"
    ],
    "codex-rs/app-server/src/request_processors/environment_processor.rs": [
        "991de9b1c27792dcf82385fd9be0aac6a0e3d10f",
        "4bd80d12b473fbb8df8227284f04b012c0f623f5"
    ],
    "codex-rs/app-server/src/request_processors/feedback_processor.rs": [
        "2558c004cbc50e2293ef4c40e857f5711a57b453",
        "f3f207197387c5fea1cd64ddd0a86ce53e008645"
    ],
    "codex-rs/app-server/src/request_processors/feedback_thread_index.rs": [
        "624477af70e2e286c3af3b464bcb6eba23a235a5",
        "cb38f4f4562efe5e446c27d311ad9f900fc51199"
    ],
    "codex-rs/app-server/src/request_processors/marketplace_processor.rs": [
        "31ff3c8229b3237024caa677226a99ba88c26349",
        "5d961528cffde86001771676c72df1cb76364fc4"
    ],
    "codex-rs/app-server/src/request_processors/mcp_processor.rs": [
        "16adf8eb21330a56907932502dd2706e591f4729",
        "e65e6d744e3d7bf015b33e5c8d12724aacacb495"
    ],
    "codex-rs/app-server/src/request_processors/plugins.rs": [
        "5d6e8fbaf422d966cf2610463a538d504dc80bb7",
        "b5adda30bb4e4ad0f60849f5607e03a2dabeba94"
    ],
    "codex-rs/app-server/src/request_processors/plugins/search.rs": [
        "6cc25e4b8bf1fdec5d58f9cd4c8c9e9e5c64bb85",
        "b3d3de93c7b23315839b51e036276bc1c389bfe1"
    ],
    "codex-rs/app-server/src/request_processors/remote_control_processor.rs": [
        "f92f74288c738d6be88dcefde79f3a8a8792f766",
        "7456b9a1d02c1d41999c8c4aefa1ef55e6dc8e56"
    ],
    "codex-rs/app-server/src/request_processors/thread_lifecycle.rs": [
        "34fff277d45ebdf61fa11014e6d9b5a30a52e653",
        "a7754975b5a47ff2b46b818145a5e66d6eccf528"
    ],
    "codex-rs/app-server/src/request_processors/thread_processor.rs": [
        "8bdf6bbed3ee667905a9b1428fb4b0ffcac08f02",
        "9563a250cc43bf5691537ed009199112265a5548"
    ],
    "codex-rs/app-server/src/request_processors/windows_sandbox_processor.rs": [
        "50b6351b244417ca3278fa81c9b2b00dda103a75",
        "d22779fa7042f8c54f6bc56a1a11481929aa340b"
    ],
    "codex-rs/app-server/src/turn_admission_tests.rs": [
        "b5cf7bf41f87252a967d4fec6efbeb5e0b28d88e",
        "472954a9467489fd4cc6df634ef48b3baa064235"
    ],
    "codex-rs/app-server/src/user_verification_response.rs": [
        "6d57afcd81cdcd3a4f922c945691e00be97f80c1",
        "0395d142a54a3e32c092f5caaee307c52adaebf1"
    ],
    "codex-rs/app-server/src/user_verification_response_tests.rs": [
        "57ee37626b1dc204f7b6e4d88f072abef5afef9e",
        "4be8c9922275dc6f2e8a88b8702cd4a09e56b9d6"
    ]
}
CUSTOM_MERGES = {
    "codex-rs/app-server/src/bespoke_event_handling.rs": [
        "65824b0da966df860bcc901d681017545de0227e",
        "1c4a913d2373a614a2dfe73c10e6f65c64a727a1",
        "5e5430826edf33b22c5cdbdec3f5e5a6078f20aa"
    ],
    "codex-rs/app-server/src/message_processor.rs": [
        "c4011790aa019b25978cc86fad98d59c4206d88c",
        "d9d1b8403c7bd9bb963f4d3a3b8d24ddf06a393a",
        "c9858cd9d513c69a917f6505410ee89b16bdd955"
    ],
    "codex-rs/app-server/src/notification_media.rs": [
        "05256bae425f635997229a3b29b26c5f1e8eee19",
        "c93967717664145a72390384749747212fd5c63a",
        "a1572cbaf7b9026f789c96ce9d027e0a7cc4b1be"
    ],
    "codex-rs/app-server/src/request_processors.rs": [
        "da10d7c0781e81034b872292a280cf31b5a5b9c8",
        "5c4633ab48ea77692e28193ea6e128ec58067a37",
        "a40486e0d512af66aeef1c1307696b2740053500"
    ],
    "codex-rs/app-server/src/request_processors/turn_processor.rs": [
        "8b0825017d7a05793184b98134474a0f03c7c5de",
        "1186f0a181704d8ea45243a850f971f1057cb080",
        "1b9a253becc97a7a2c1b0de9e92def7c192691c2"
    ],
    "codex-rs/app-server/src/thread_state.rs": [
        "79bb2c10a6994dfdfcd96d7f458fdc15c5f60ba7",
        "42b434e017c527a8785fbaedf43292a83e1af09d",
        "6dfe969ab3adfc5d703184ee2ee61a1b6335f282"
    ]
}


def blob(ref: str, path: str) -> str | None:
    result = subprocess.run(
        ["git", "rev-parse", "--verify", ref + ":" + path],
        cwd=ROOT, capture_output=True, text=True,
    )
    return result.stdout.strip() if result.returncode == 0 else None


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT, capture_output=True, check=True, text=True, encoding="utf-8",
    ).stdout


class AppServerSourceIntegrationTests(unittest.TestCase):
    def assertIn(self, value, text, msg=None):
        if isinstance(text, str) and len(text) > 2048:
            self.assertTrue(value in text, msg or f"missing source marker: {value!r}")
            return
        super().assertIn(value, text, msg)

    def test_twenty_nine_exact_official_app_server_blobs(self):
        self.assertEqual(len(UPSTREAM_FILES), 29)
        for path, (_, official) in UPSTREAM_FILES.items():
            with self.subTest(path=path):
                self.assertEqual(blob("HEAD", path), official)

    def test_original_fork_code_preserved_in_staged_upstream_paths(self):
        for path, (production, _) in UPSTREAM_FILES.items():
            with self.subTest(path=path):
                self.assertEqual(blob(PRODUCTION, path), production or None)

    def test_six_reviewed_adaptive_app_server_merges_untouched(self):
        self.assertEqual(len(CUSTOM_MERGES), 6)
        for path, (production, expected, _) in CUSTOM_MERGES.items():
            with self.subTest(path=path):
                self.assertEqual(blob(PRODUCTION, path), production)
                self.assertEqual(blob("HEAD", path), expected)

    def test_app_server_module_registration_matches_new_service(self):
        lib = source("codex-rs/app-server/src/lib.rs")
        self.assertIn("mod plugin_config_reload", lib)
        self.assertIn("remote_control", lib)

    def test_config_manager_retains_retained_session_loader(self):
        config = source("codex-rs/app-server/src/config_manager.rs")
        self.assertIn("load_retained_session_config", config)
        self.assertIn("load_latest_config_with_session_layers", config)

    def test_remote_control_requests_cover_pairing_and_client_revocation(self):
        control = source("codex-rs/app-server/src/request_processors/remote_control_processor.rs")
        for name in ("pairing_start", "pairing_status", "clients_revoke"):
            self.assertIn(name, control)

    def test_thread_lifecycle_resolves_pending_server_requests(self):
        lifecycle = source("codex-rs/app-server/src/request_processors/thread_lifecycle.rs")
        self.assertIn("resolve_pending_server_request", lifecycle)
        self.assertIn("populate_thread_turns_from_history", lifecycle)

    def test_windows_sandbox_service_still_checks_allowed_mode(self):
        sandbox = source("codex-rs/app-server/src/request_processors/windows_sandbox_processor.rs")
        self.assertIn("resolve_allowed_windows_sandbox_setup_mode", sandbox)
        self.assertIn("windows_sandbox_setup_start", sandbox)

    def test_approval_and_thread_admission_regressions_staged(self):
        tests = source("codex-rs/app-server/src/turn_admission_tests.rs")
        self.assertIn("drain_rejects_new_work_without_waiting_for_admitted_work", tests)
        workspace = source("codex-rs/app-server/src/request_processors/account_processor/workspace_routing_tests.rs")
        self.assertIn("fn ", workspace)

    def test_codexdd_adaptive_tool_guards_not_overwritten(self):
        self.assertEqual(
            blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )

    def test_version_markers_and_lockfile_stay_unadvanced(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(blob("HEAD", "codex-rs/Cargo.lock"),
                         "0c913b46674d3593ee84f1fee5508d16e33efae5")

    def test_source_staging_does_not_make_a_release_candidate(self):
        doc = (ROOT / "docs/codexdd-0.4.5-packet-3c9a-app-server.md").read_text(encoding="utf-8")
        for label in ("INCOMPLETE / NOT A RELEASE CANDIDATE",
                      "source-only", "native Rust", "Cargo.lock"):
            self.assertIn(label, doc)


if __name__ == "__main__":
    unittest.main()
