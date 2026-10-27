"""CodexDD 0.4.5 packet 3C.7c: core root source provenance.
Source-only, not native Rust or runtime-security validation.
"""

from __future__ import annotations
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
STAGED = {
    "codex-rs/core/src/agent_message_board.rs": [
        "62e60c92b632b64d34ce9f16902eac5c55295694",
        "8af388dce5de94fd32143163f2016d52c4a05529",
    ],
    "codex-rs/core/src/agents_md_manager.rs": [
        "4aaa1d0c3c382aab877e4d885d5a9a5d5cda1595",
        "ee80cebce50a98248f4503c3861797e86c54d882",
    ],
    "codex-rs/core/src/code_mode_host.rs": [
        None,
        "de328875cbf47ff92593e9f669678e2731afb6ac",
    ],
    "codex-rs/core/src/code_mode_host_tests.rs": [
        None,
        "b850c3c94755d55102bc20b0b035459e72ceae53",
    ],
    "codex-rs/core/src/codex_delegate.rs": [
        "db6db608962e40b7073cfdc43f9f580a0831a0e3",
        "e2c30ad988ca193a8bc9f1fb0661e20f70b2ef8a",
    ],
    "codex-rs/core/src/codex_delegate_tests.rs": [
        "d4e347c2529be5172c7ce4879423cc451f8b6ae1",
        "9288d30001625e4bc16c6bbbdf16c7e9f02b6838",
    ],
    "codex-rs/core/src/compact.rs": [
        "f4db71b6882fdece78909298fbefa4e3b92f3392",
        "71761349e499b109b46baa3ca3e662d5ed68cda2",
    ],
    "codex-rs/core/src/compact_model_fallback.rs": [
        "9e9a6213d54f6bfb3231e4728d85b910f78d95b1",
        "7d4c0e25cbe06795a4e1e3c0d651cc53c5ce7913",
    ],
    "codex-rs/core/src/compact_remote_history.rs": [
        "aa7cd7607eeebef75a295a4409aa8f25dc2b2d6d",
        "82fc5e3f0e2dabf03b30d56a0453702e4257402a",
    ],
    "codex-rs/core/src/compact_remote_v2.rs": [
        "6a061086b1d325a58280e2dd13274fbb904f3c11",
        "bacf8c3a5c5453051a20c57df1c3275c57ac27c3",
    ],
    "codex-rs/core/src/compact_remote_v2_attempt.rs": [
        "4df467edf4c396f01a38cabdd59262f0db432c61",
        "3f14ac5be7da8ed6c72271e8ce287ff672feffc6",
    ],
    "codex-rs/core/src/compact_remote_v2_image_budget_tests.rs": [
        "ea37694606b2c07d3ebbdee70da7c3467fc0b529",
        "a4db94d5e12296d4d4cf8347e042a60311eeeccd",
    ],
    "codex-rs/core/src/compact_remote_v2_images.rs": [
        "e1ae5be0fa8633481dd98224971af9f31059685c",
        "cd8697813a9d359a0f8c6ff6a00effdf26a64d56",
    ],
    "codex-rs/core/src/compact_remote_v2_installation.rs": [
        None,
        "a13b18ba448680976c0ce047aeb296e5c2c9d1ee",
    ],
    "codex-rs/core/src/compact_remote_v2_installation_tests.rs": [
        None,
        "5f9e290bd993a2e8ae7252750d0f185492f4ec9e",
    ],
    "codex-rs/core/src/compact_retained_tool_outputs.rs": [
        None,
        "0ea3b68f10102a60e59e0e51bf7380d99dedec7e",
    ],
    "codex-rs/core/src/compact_tests.rs": [
        "28051a77c36c3c1dd644ca61c3d90f81972de62d",
        "2b5f93c69ed70cb49eb99b2fcd4833db4ad4046c",
    ],
    "codex-rs/core/src/cyber_access_program.rs": [
        "0b7803b7b1fc0f714b5cba7e94ba392ae6b8e618",
        "bbc50b90c533146b3895e70269c600342390a552",
    ],
    "codex-rs/core/src/environment_selection.rs": [
        "29005bc6bfd600b13516144fafe2c0ae8271549a",
        "f85efc1e7988babc121cc8f2a01f6db942efc5f6",
    ],
    "codex-rs/core/src/event_mapping.rs": [
        "176e33da175d0edb8f74bf84b312b4ea5674f5a2",
        "c0fe8308fae8ae949942fd5c83b040af78f12173",
    ],
    "codex-rs/core/src/event_mapping_tests.rs": [
        "d6d9f59f2cc4a82dc51251e9c22f47e1fdd918d6",
        "853f652691e91bc4829a8ca13faae05a5ec95e17",
    ],
    "codex-rs/core/src/exec.rs": [
        "fa458ed1b71b74805a096df0862e4e3412be21fc",
        "f72396e38320e838e21b6a9ceaaa433ada62f0c0",
    ],
    "codex-rs/core/src/exec_policy.rs": [
        "21e414ed01b2040830790e8b486d17edd83309dd",
        "79d564e338a99a753729a2df4ddaaf1e0a4df77b",
    ],
    "codex-rs/core/src/exec_tests.rs": [
        "db07135e3b9cf84e606c91d84ab1eeda1bd98cc5",
        "f86df094b80fe364d1c8b137481ecca4b9ebd76c",
    ],
    "codex-rs/core/src/function_tool.rs": [
        "86863648554b0076b8da7c58c4029f7130078cf9",
        "23cf6c1d16754827c10853fde15ba9983445ef23",
    ],
    "codex-rs/core/src/hook_runtime.rs": [
        "30c4162ffc06307dc0a73189cb4b7944b6a949c2",
        "1eb54fa25b18e62d31bc27154adc3ae6a6f71f5c",
    ],
    "codex-rs/core/src/image_preparation_tests.rs": [
        "3bee21f1522e0aa54c8bc4d10ba60683e4fa5dae",
        "f890b48e20c09aed37857d39946502e2ce70853f",
    ],
    "codex-rs/core/src/lib.rs": [
        "a2b44e3f1fb6b8c3ede999ec306f5592b9a5a0b9",
        "08b52fc2e10782fb444a2ff8b6132f2121d362f5",
    ],
    "codex-rs/core/src/mcp.rs": [
        "55c36f77b0c85044264ed412ba0670be1786bc38",
        "0eb7bdc3ad72fa0527351d9def964825f0fecd03",
    ],
    "codex-rs/core/src/mcp_skill_dependencies.rs": [
        "5931da9100e86cae45c5527c876265fbbb258563",
        "5f4af05235c413ce278bc825eb5a32a700ca6e4b",
    ],
    "codex-rs/core/src/mcp_tool_call.rs": [
        "d6053a0f5d5b062d5cc404476064ce8664442153",
        "991ad9083c73d899ebbf413c293dca9604c328d0",
    ],
    "codex-rs/core/src/mcp_tool_call_tests.rs": [
        "ddd2377edec0e75247a98f1166d10eba8de04ff5",
        "1dc06e3d0044572bf18f57e3123ea6176fad3218",
    ],
    "codex-rs/core/src/mcp_tool_exposure.rs": [
        "fa9b6eb61b51321c3fe97c45e50de27d57e4e1cf",
        "e7aa97993a8a6b153b2e5d3425a16ed7b3cf8e4b",
    ],
    "codex-rs/core/src/otel_init.rs": [
        "bd9df090ff431c253cbed84dd6fc36cd01c3480b",
        "a6b6a70ec064823fd6c8488164ad578e0542f995",
    ],
    "codex-rs/core/src/realtime_context_tests.rs": [
        "b1519e6c1e0a664d0f15622f79c2d539cd2e6201",
        "fb34e8ecf8d402d2dbc3eccf663ef8e58c73b5e4",
    ],
    "codex-rs/core/src/realtime_conversation.rs": [
        "007898db4b1b9b84a96ff49a070376d292d92e7f",
        "dab0d42f6ac4e47d73e32dde39b9bc1193cf8c20",
    ],
    "codex-rs/core/src/realtime_conversation_tests.rs": [
        "16af93eb53b0803b8643f727092beef11d9eafad",
        "98e1e61a18082347b580370e7c6f80e51ee8628f",
    ],
    "codex-rs/core/src/responses_metadata.rs": [
        "bfdeb3b40f524401e1a57879ab2335a45130f588",
        "bcc2f9e945370cd9659e7162154831c10b038aec",
    ],
    "codex-rs/core/src/shell_snapshot.rs": [
        "994506d24c6a2ca345d81380556ac34c2d3fda09",
        "01bd13688a75f8cfb9b8cf8e316f6e4a407235bf",
    ],
    "codex-rs/core/src/shell_snapshot_sandbox.rs": [
        "1da6854995147d6ef526175aaf6625f8715dcf58",
        "0735470154b031b4c3ef2a07993ca7592d8d0bab",
    ],
    "codex-rs/core/src/shell_snapshot_sandbox_tests.rs": [
        "cea68a9a0f15e7652a8471319cb0111222aadbdc",
        "312b802b95ead65a52bfbe0255e4724d63f2cfbd",
    ],
    "codex-rs/core/src/shell_snapshot_tests.rs": [
        "8c816500d7c2ca3d14312dde83e9d5ebaa0259de",
        "fc191bf590cfc9d5fce2b7a674b407b8b58067ae",
    ],
    "codex-rs/core/src/spawn.rs": [
        "79526b8bfde277083b774d6438909efb1691f3cb",
        "3ac3493f9b9b6348ee87dd2de5c9acc267a472b9",
    ],
    "codex-rs/core/src/stream_events_utils.rs": [
        "0264a09389b55cc3b38b520083395217e9fb217c",
        "2081c42cd2c828864d1672c0603eacf54a71320d",
    ],
    "codex-rs/core/src/stream_events_utils_tests.rs": [
        "53b20da43672b199d0a6577c2e84445790ed2d69",
        "daad318bb28deae0be55c7e7663b59b644c16ca1",
    ],
    "codex-rs/core/src/turn_metadata_tests.rs": [
        "5ad52a1dda4509acd0d9d3ddbe11a6801bb148d3",
        "f0f69e641ed1ab82c7b0266cbe4107b75c4bb793",
    ],
    "codex-rs/core/src/turn_timing_tests.rs": [
        "65185ecea2e99397aae39a0f11081815afa61c1a",
        "7aa7d3d177a9b2eeab4c2f1aef0aa6e0b43b6e39",
    ],
    "codex-rs/core/src/windows_sandbox.rs": [
        "316e9ff7fc86dce74425796f76dd81f24f87b0df",
        "2c2d1fdb366c9c587752762376427baaa6522339",
    ],
}
MERGED_FORK = {
    "codex-rs/core/src/client.rs": [
        "a08e2386475b0c4e87d0d79aaccc27efeaff641d",
        "583447f35eaba564771771813bd4d897cb6b73de",
    ],
    "codex-rs/core/src/client_tests.rs": [
        "ebfeb73694a5d22b4a030ca7d89e2d370511924e",
        "2e5525612379a29af24116dd738a6612dc4420e6",
    ],
    "codex-rs/core/src/codex_thread.rs": [
        "24fc487a32cd930f39a3c8137c40790d37e60713",
        "84b32544000fde7d16a4b4cd4f4b27f26d7ceaf9",
    ],
    "codex-rs/core/src/thread_manager.rs": [
        "ff796b8c877f3e671252d6db885ac968b08e5893",
        "88fa659a3c45839d9a3dce6b6a39e6e47795f103",
    ],
    "codex-rs/core/src/thread_manager_tests.rs": [
        "fe36ae0ae22feca5118d52d0dbee762b06b00c6d",
        "d7b27ac523a2f6c3d2e8a1beabc9189b17622e25",
    ],
}


def git_blob(ref, p):
    result = subprocess.run(
        ["git", "rev-parse", "--verify", ref + ":" + p],
        cwd=ROOT,
        capture_output=True,
        text=True,
    )
    return result.stdout.strip() if result.returncode == 0 else None


def source(p):
    return subprocess.run(
        ["git", "show", "HEAD:" + p],
        cwd=ROOT,
        capture_output=True,
        check=True,
        text=True,
        encoding="utf-8",
    ).stdout


class CoreRootSourceTests(unittest.TestCase):
    def test_exact_forty_eight_upstream_blobs(self):
        self.assertEqual(len(STAGED), 48)
        for p, (_, upstream) in STAGED.items():
            with self.subTest(path=p):
                self.assertEqual(git_blob("HEAD", p), upstream)

    def test_original_fork_source_unmodified_for_staged_files(self):
        for p, (original, _) in STAGED.items():
            with self.subTest(path=p):
                self.assertEqual(git_blob(PRODUCTION, p), original)

    def test_upstream_new_files_absent_in_production(self):
        for name in (
            "code_mode_host.rs",
            "code_mode_host_tests.rs",
            "compact_retained_tool_outputs.rs",
            "compact_remote_v2_installation.rs",
            "compact_remote_v2_installation_tests.rs",
        ):
            path = "codex-rs/core/src/" + name
            self.assertIsNone(git_blob(PRODUCTION, path))
            self.assertIsNotNone(git_blob("HEAD", path))

    def test_lib_wires_current_code_mode_and_retention(self):
        code = source("codex-rs/core/src/lib.rs")
        self.assertTrue("code_mode_host" in code)
        self.assertTrue("compact_retained_tool_outputs" in code)

    def test_code_mode_host_and_retained_output_helper_present(self):
        self.assertTrue(
            "create_session" in source("codex-rs/core/src/code_mode_host.rs")
        )
        self.assertTrue(
            "is_retained_tool_output"
            in source("codex-rs/core/src/compact_retained_tool_outputs.rs")
        )

    def test_hook_lifecycle_paths_preserved(self):
        code = source("codex-rs/core/src/hook_runtime.rs")
        for marker in (
            "run_pre_tool_use_hooks",
            "run_post_tool_use_hooks",
            "run_permission_request_hooks",
        ):
            self.assertTrue(marker in code, marker)

    def test_windows_sandbox_and_execution_policy_present(self):
        self.assertTrue(
            "prepare_elevated_sandbox" in source("codex-rs/core/src/windows_sandbox.rs")
        )
        self.assertTrue(
            "prompt_is_rejected_by_policy" in source("codex-rs/core/src/exec_policy.rs")
        )

    def test_native_mcp_and_event_regressions_staged(self):
        self.assertTrue(
            "approval_required_when_read_only_false_and_destructive"
            in source("codex-rs/core/src/mcp_tool_call_tests.rs")
        )
        self.assertTrue("fn " in source("codex-rs/core/src/event_mapping_tests.rs"))

    def test_five_fork_sources_now_have_pinned_reviewed_merges(self):
        self.assertEqual(len(MERGED_FORK), 5)
        for p, (old, merged) in MERGED_FORK.items():
            with self.subTest(path=p):
                self.assertEqual(git_blob(PRODUCTION, p), old)
                self.assertEqual(git_blob("HEAD", p), merged)

    def test_registry_and_release_markers_unadvanced(self):
        self.assertEqual(
            git_blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(
            source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0"
        )
        self.assertEqual(
            git_blob("HEAD", "codex-rs/Cargo.lock"),
            "a6cd27b0b70fde2f2be9c1777f3666ad3fa318bb",
        )

    def test_packet_is_not_release_candidate(self):
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c7c-core-root.md").read_text(
            encoding="utf-8"
        )
        for marker in (
            "INCOMPLETE / NOT A RELEASE CANDIDATE",
            "source-only",
            "native Rust",
            "Cargo.lock",
        ):
            self.assertTrue(marker in note, marker)


if __name__ == "__main__":
    unittest.main()
