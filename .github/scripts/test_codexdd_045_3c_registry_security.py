"""CodexDD 0.4.5 3C.3: offline Git registry provenance and policy tripwires.

Static only: never a Rust build, Windows release receipt or permission to publish.
"""

from __future__ import annotations

import hashlib
import re
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
FILE = "codex-rs/core/src/tools/registry.rs"
RUST_TESTS = "codex-rs/core/src/tools/registry_tests.rs"
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
MERGED_SHA = "fc75b3bbf91f1881bd32faaaca93356fbb598b5e"
FORK_TEST_SHA = "b9af52dbb0b771e0b9119d5166b6a047c21bd24c"
START_HELPERS = "const ADAPTIVE_RECONNAISSANCE_REJECTION:"
END_HELPERS = "pub use codex_tools::ToolExecutor;"
REJECT = "if let Some(message) = adaptive_reconnaissance_rejection(&invocation)"
HOOK = "if let Some(pre_tool_use_payload) = tool.pre_tool_use_payload(&invocation) {"
START_TOOL = "if tool.mcp_server_name().is_none() {"


def committed(ref: str, path: str) -> bytes:
    return subprocess.run(
        ["git", "show", ref + ":" + path],
        cwd=ROOT, check=True, capture_output=True,
    ).stdout


def git_blob(raw: bytes) -> str:
    return hashlib.sha1(
        b"blob " + str(len(raw)).encode("ascii") + b"\0" + raw
    ).hexdigest()


def helper_block(src: str) -> str:
    i = src.index(START_HELPERS)
    return src[i:src.index(END_HELPERS, i)]


class RegistrySecurityIntegrationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.source = committed("HEAD", FILE).decode("utf-8")
        cls.fork = committed(PRODUCTION, FILE).decode("utf-8")
        cls.tests = committed("HEAD", RUST_TESTS).decode("utf-8")

    def test_exact_merged_registry_git_blob(self):
        self.assertEqual(git_blob(committed("HEAD", FILE)), MERGED_SHA)

    def test_original_codexdd_rust_policy_tests_unchanged(self):
        original = committed(PRODUCTION, RUST_TESTS)
        current = committed("HEAD", RUST_TESTS)
        self.assertEqual(git_blob(original), FORK_TEST_SHA)
        self.assertTrue(current.startswith(original))
        self.assertIn(b"adaptive_reconnaissance_rejects_hook_rewritten_write_command", current)

    def test_seventeen_codexdd_policy_helpers_preserved_byte_for_byte(self):
        merged = helper_block(self.source)
        self.assertEqual(merged, helper_block(self.fork))
        self.assertEqual(len(re.findall(r"(?m)^fn adaptive_", merged)), 17)

    def test_controller_phase_triggers_and_denials_remain(self):
        for marker in (
            "CODEXDD_ADAPTIVE_RECONNAISSANCE_TURN_TRIGGER",
            "CODEXDD_ADAPTIVE_MECHANICAL_VALIDATION_TURN_TRIGGER",
            "CODEXDD_ADAPTIVE_VALIDATION_TERMINALIZATION_TURN_TRIGGER",
            "ADAPTIVE_RECONNAISSANCE_REJECTION",
            "ADAPTIVE_MECHANICAL_VALIDATION_REJECTION",
            "ADAPTIVE_VALIDATION_TERMINALIZATION_REJECTION",
        ):
            self.assertIn(marker, self.source)

    def test_recon_and_mechanical_validation_have_distinct_whitelists(self):
        self.assertIn('"report_adaptive_signal" | "update_plan"', self.source)
        self.assertIn('| "run_codexdd_validation"', self.source)
        self.assertIn("fn adaptive_reconnaissance_shell_command_allowed", self.source)
        self.assertIn("fn adaptive_mechanical_validation_shell_command_allowed", self.source)

    def test_terminalization_authorizes_only_trusted_signal_tool(self):
        first = self.source.index("fn adaptive_validation_terminalization_rejection(")
        last = self.source.index("\nfn adaptive_reconnaissance_shell_command_allowed(", first)
        body = self.source[first:last]
        self.assertIn('invocation.tool_name.name == "report_adaptive_signal"', body)
        self.assertNotIn("exec_command", body)
        self.assertNotIn("run_codexdd_validation", body)

    def test_command_redirect_and_dynamic_expansion_fail_closed(self):
        helpers = helper_block(self.source)
        for token in (
            "command.contains('>')", "command.contains('<')",
            "command.contains('&')", 'command.contains("$(")',
            "command.contains('" + chr(96) + "')",
        ):
            self.assertIn(token, helpers)

    def test_powershell_batch_parser_and_git_safety_survive(self):
        helpers = helper_block(self.source)
        for marker in (
            "parse_powershell_script_into_plain_commands",
            "if command.contains(';')",
            "adaptive_reconnaissance_has_unsafe_pipeline_stage",
            "parse_shell_script(command)",
            '"--output"', '"--output="', '"--ext-diff"', '"--textconv"',
        ):
            self.assertIn(marker, helpers)

    def test_upstream_registered_specs_and_tool_search_survive(self):
        for marker in (
            "FunctionsNamespaceFunctionPrefixes",
            "ToolSearchInfo",
            "pub(crate) model_spec: Option<Arc<ToolSpec>>",
            "pub(crate) fn search_info(",
            "pub(crate) fn registered_tool(",
            "fn finishes_on_cancellation(&self) -> bool",
        ):
            self.assertIn(marker, self.source)

    def test_upstream_cancellation_continues_through_dispatch(self):
        for marker in (
            "use crate::function_tool::OrCancelToolExt;",
            ".or_cancel_tool(&cancellation_token)",
            ".or_cancel_tool(&invocation.cancellation_token)",
            "ToolCallOutcome::Aborted",
        ):
            self.assertIn(marker, self.source)

    def test_initial_denial_occurs_before_pre_tool_use_hook(self):
        first = self.source.index(REJECT)
        pre = self.source.index(HOOK)
        self.assertLess(first, pre)
        self.assertIn("dispatch_trace.record_failed(&err)", self.source[first:pre])

    def test_hook_rewritten_input_rechecked_before_handler_execution(self):
        first = self.source.index(REJECT)
        pre = self.source.index(HOOK)
        second = self.source.index(REJECT, first + 1)
        start = self.source.index(START_TOOL, second)
        self.assertLess(first, pre)
        self.assertLess(pre, second)
        self.assertLess(second, start)
        self.assertIn("with_updated_hook_input", self.source[pre:second])
        self.assertEqual(self.source.count(REJECT), 2)

    def test_both_rejections_mark_terminal_blocked_lifecycle(self):
        self.assertEqual(self.source.count(REJECT), 2)
        for part in self.source.split(REJECT)[1:]:
            denial = part.split("\n        }\n\n", 1)[0]
            self.assertIn("notify_tool_finish_if_unclaimed(", denial)
            self.assertIn("ToolCallOutcome::Blocked", denial)
            self.assertIn("dispatch_trace.record_failed(&err)", denial)

    def test_codexdd_rust_tests_still_cover_allowed_and_denied_calls(self):
        for name in (
            "adaptive_reconnaissance_shell_gate_allows_inspection_commands",
            "adaptive_reconnaissance_shell_gate_rejects_write_capable_commands",
            "adaptive_mechanical_validation_shell_gate_allows_bounded_validation_commands",
            "adaptive_mechanical_validation_shell_gate_blocks_source_edit_capable_commands",
            "adaptive_reconnaissance_dispatch_blocks_non_read_tool",
            "adaptive_validation_terminalization_allows_only_trusted_signal_tool",
        ):
            self.assertIn("fn " + name + "(", self.tests)

    def test_production_markers_still_unchanged(self):
        self.assertEqual(committed("HEAD", "codex-rs/codexdd-version.txt").strip(), b"0.4.4")
        self.assertEqual(
            committed("HEAD", "codex-rs/upstream-codex-release.txt").strip(), b"rust-v0.162.0"
        )

    def test_packet_remains_documented_as_incomplete(self):
        notes = (
            ROOT / "docs/codexdd-0.4.5-packet-3c3-registry-security.md"
        ).read_text(encoding="utf-8")
        self.assertIn("INCOMPLETE / NOT A RELEASE CANDIDATE", notes)
        self.assertIn("native Rust", notes)
        self.assertIn("109", notes)


if __name__ == "__main__":
    unittest.main()
