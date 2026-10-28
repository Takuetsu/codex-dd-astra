"""CodexDD 0.4.5 / 3C.6c: tool router/exposure and accepted-call provenance.

Offline source checks only. No Rust compilation, tool execution, approvals,
security proof, version bump, or release authorization.
"""

from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
UPSTREAM = "83086188ae0614bd882818ff1aa23f3194962012"
STAGED = {
    "codex-rs/core/src/tools/code_mode/execute_handler.rs": [
        "75733b9dfaf69906de891e724770f4d567e08bbf",
        "25661c6c359fc97a7c4400d0cb6ae379d32d4e0b",
    ],
    "codex-rs/core/src/tools/code_mode/tool_definitions.rs": [
        "84b262319a7730b5acaaf5b396bde2f34f91ad9b",
        "568f6dabd6b5f804d5ceeea5dab6e34f01b69c83",
    ],
    "codex-rs/core/src/tools/executed_tool_calls.rs": [
        "636d3cbd92ae170e5a439e294323e98214b78523",
        "853c88a9727ee7e0892b1f8f7276b47175a8a8f0",
    ],
    "codex-rs/core/src/tools/executed_tool_calls/mcp_attribution_tests.rs": [
        "8d90927ddc43a7e62d6a0e96e837c8d7d9e2b36c",
        "9085d18a43266a00886332f8d27ccddb062baa6e",
    ],
    "codex-rs/core/src/tools/executed_tool_calls/request_metadata.rs": [
        "1ae9eefb166eccf6046cc22117dbf0bad196a90f",
        "01d0056b596434c2aa33954b19b67e181a4822d4",
    ],
    "codex-rs/core/src/tools/executed_tool_calls/request_metadata_tests.rs": [
        "b4ffbc8276afedecbea3ece7e8e39cd5ee70bfd2",
        "1c504bf87253dccbf88a035e26fc6cc0af6541a7",
    ],
    "codex-rs/core/src/tools/executed_tool_calls_direct_tests.rs": [
        "23f2033d252d98011ba5444c8e81f30537b21948",
        "304375bbb84b75364adb1f662f5c88bde9f0657c",
    ],
    "codex-rs/core/src/tools/handlers/current_time.rs": [
        "527701b963e95831c7c9e06908418a54cacb4ab2",
        "c09a9deab3bbed37cea5230c499148b8bb28b4a5",
    ],
    "codex-rs/core/src/tools/handlers/extension_tools.rs": [
        "ea483528d0e5f2e21f674208ce187c53a16b1ca9",
        "d6eeb84616b4ce071b0ccaaf0994528f19bd0826",
    ],
    "codex-rs/core/src/tools/handlers/get_context_remaining.rs": [
        "3eecdc54f7e03186db3ddd559972088442fec4f1",
        "23c3a6086860eb4187afced9b8f84b602e9f856b",
    ],
    "codex-rs/core/src/tools/handlers/list_available_plugins_to_install.rs": [
        "1b001f7c45fcce1b07567d08d6bbeb8b845b49d5",
        "0860df56bf3456dd68f8cd3d2d59d90161bde8b3",
    ],
    "codex-rs/core/src/tools/handlers/mcp.rs": [
        "4cce55018f5a47a9402263fd4db2bce0dd67fb05",
        "98d61aff08443f7209f386e643734b5e27f01460",
    ],
    "codex-rs/core/src/tools/handlers/mcp_search_tests.rs": [
        "e52b8c361808a5ec81d6cd5e63cf310a6273d891",
        "50c11dfd5a63c8c49b436bc7c07f13bc1301bc62",
    ],
    "codex-rs/core/src/tools/handlers/multi_agents/send_input.rs": [
        "3d2942f1196d6840bfc16a6e4a3e519bee4c8c5a",
        "9998a17367e249254ed150deeeaaabe76131b969",
    ],
    "codex-rs/core/src/tools/handlers/multi_agents/spawn.rs": [
        "94079e0ebad490c4f4a56f4935a56639aab59cca",
        "10c793f5b9e8293c3a0a05a089372619f6689c5e",
    ],
    "codex-rs/core/src/tools/handlers/multi_agents_tests.rs": [
        "6991af1476c5ce847064ee9fbce8dcf1bae074b8",
        "f6ffb7a8bb92ae27cd2e235b3288741d06ac7774",
    ],
    "codex-rs/core/src/tools/handlers/multi_agents_v2/list_agents.rs": [
        "9e0eaa74c03e232cfa86660047797a199df0c6c8",
        "b340a1fe494f784cafd3978cf77b94ea38c832bf",
    ],
    "codex-rs/core/src/tools/handlers/multi_agents_v2/spawn.rs": [
        "da7325bccb7faf9c1b81c4701eab0911b84ac225",
        "c23caa014599fb37ab77738a82396a0cb81cb7c1",
    ],
    "codex-rs/core/src/tools/handlers/new_context_window.rs": [
        "45b1a191db05d519718dd4d9883f8df2be6ca4bd",
        "4b11300b541cc1d1c15e365fc14fc0ceab8714ee",
    ],
    "codex-rs/core/src/tools/handlers/tool_search.rs": [
        "570b6610a2d66c25862edd998f860b5336e27543",
        "220fb3c11ab6a347ef2416bbfb31213c5e843628",
    ],
    "codex-rs/core/src/tools/parallel.rs": [
        "3daefd3925542942e6fce0db126093e8a57de8a1",
        "e0b50c48dd7ed4558b4423d25bae6e62d99ccea5",
    ],
    "codex-rs/core/src/tools/router.rs": [
        "baf39f082af77e3fa2c8b451a87980b7081ace8b",
        "a7a192815b6a2f45e96b6903f133e5564306a419",
    ],
    "codex-rs/core/src/tools/router_tests.rs": [
        "05a1e2e65a3b18611b4accebb2787a4bb6f41d54",
        "aff6d416600a75ce43c72aa32c580db10d9d97de",
    ],
}
RETIRED_ZSH = {
    "codex-rs/core/src/tools/runtimes/zsh_fork.rs": "4daa0f18b0c76075027cd38b3ceb6727546edf7e",
    "codex-rs/core/src/tools/runtimes/zsh_fork/unix_escalation.rs": "f0aaf9d5f4e5e5c247ae1cfe9be706c4c9c81b91",
    "codex-rs/core/src/tools/runtimes/zsh_fork/unix_escalation_tests.rs": "ca282cd696f49aaded9457f6c7b77880192e2400",
}
UNMODIFIED_CUSTOM = {
    "codex-rs/core/src/tools/registry.rs": "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
    "codex-rs/core/src/tools/handlers/dynamic.rs": "05353ec9dc97df37db5c85ed1b232d7c9eb5e6d1",
    "codex-rs/core/src/tools/mod.rs": "2a05e28de35348b79e02d80e4bdd8c3381b88279",
}


def git_blob(ref: str, path: str) -> str:
    return subprocess.run(
        ["git", "rev-parse", ref + ":" + path],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    ).stdout.strip()


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
        encoding="utf-8",
    ).stdout


class ToolCallRoutingIntegrationTests(unittest.TestCase):
    def assertIn(self, member, container, msg=None):
        if isinstance(container, str) and len(container) > 2048:
            self.assertTrue(
                member in container, msg or f"missing source marker: {member!r}"
            )
            return
        super().assertIn(member, container, msg)

    def test_twenty_three_upstream_tool_source_blobs(self):
        self.assertEqual(len(STAGED), 23)
        for path, (old, new) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob("HEAD", path), new)
                # The target blob is pinned from official 0.163.0 during GitHub staging.
                # Do not dereference UPSTREAM locally: it may not be fetched here.

    def test_no_codexdd_custom_tool_source_was_overwritten(self):
        for path, (old, _) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob(PRODUCTION, path), old)

    def test_tool_registry_and_dynamic_control_merge_unchanged(self):
        for path, expected in UNMODIFIED_CUSTOM.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob("HEAD", path), expected)

    def test_router_uses_current_registered_dispatch_contract(self):
        router = source("codex-rs/core/src/tools/router.rs")
        self.assertIn("dispatch_tool_call_with_state", router)
        self.assertIn("registered_tool", router)
        self.assertIn("model_visible_specs", router)

    def test_accepted_tool_call_history_preserves_mcp_attribution(self):
        calls = source("codex-rs/core/src/tools/executed_tool_calls.rs")
        self.assertIn("record_mcp_source", calls)
        self.assertIn("mcp_attribution_snapshot", calls)
        self.assertIn("record_accepted_result", calls)

    def test_extension_hook_inputs_remain_editable_but_policy_checked(self):
        mcp = source("codex-rs/core/src/tools/handlers/mcp.rs")
        self.assertIn("pre_tool_use_payload", mcp)
        self.assertIn("with_updated_hook_input", mcp)
        self.assertIn("post_tool_use_payload", mcp)
        self.assertIn("ToolRegistry", source("codex-rs/core/src/tools/registry.rs"))

    def test_code_mode_and_search_have_current_handlers(self):
        code = source("codex-rs/core/src/tools/code_mode/execute_handler.rs")
        search = source("codex-rs/core/src/tools/handlers/tool_search.rs")
        self.assertIn("handle_call", code)
        self.assertIn("search_entries", search)

    def test_multi_agent_tool_calls_staged(self):
        spawn = source("codex-rs/core/src/tools/handlers/multi_agents/spawn.rs")
        send = source("codex-rs/core/src/tools/handlers/multi_agents/send_input.rs")
        self.assertIn("handle_spawn_agent", spawn)
        self.assertIn("fn ", send)

    def test_new_native_tool_regression_sources_staged(self):
        for path in (
            "codex-rs/core/src/tools/executed_tool_calls_direct_tests.rs",
            "codex-rs/core/src/tools/handlers/mcp_search_tests.rs",
            "codex-rs/core/src/tools/router_tests.rs",
        ):
            self.assertIn("fn ", source(path))

    def test_orphaned_unix_zsh_sources_left_untouched_and_unregistered(self):
        runtime = source("codex-rs/core/src/tools/runtimes/mod.rs")
        self.assertNotIn("mod zsh_fork;", runtime)
        self.assertEqual(len(RETIRED_ZSH), 3)
        for path, old in RETIRED_ZSH.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob(PRODUCTION, path), old)
                self.assertEqual(git_blob("HEAD", path), old)

    def test_version_markers_and_cargo_lock_remain_unadvanced(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(
            source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0"
        )
        self.assertEqual(
            git_blob("HEAD", "codex-rs/Cargo.lock"),
            "13bc15ff844e93dfaf7777c73bb7313b35bb3b81",
        )

    def test_release_remains_explicitly_blocked(self):
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c6c-tool-routing.md").read_text(
            encoding="utf-8"
        )
        for marker in (
            "INCOMPLETE / NOT A RELEASE CANDIDATE",
            "native Rust",
            "Cargo.lock",
            "source-only",
        ):
            self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
