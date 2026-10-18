"""CodexDD 0.4.5 Packet 3C.4b: offline Git source and sandbox contract tripwires.

These are static checks only. They do not compile Rust, prove security
behavior, renew Cargo.lock, or permit publication.
"""
from __future__ import annotations

import re
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
STAGED = {
    "codex-rs/core/src/tools/handlers/unified_exec.rs": [
        "763fa155766dc67536ab89214861705d90dc027f",
        "6eeaf861c12548086d780407c2d2e4ff00512bea"
    ],
    "codex-rs/core/src/tools/handlers/unified_exec/exec_command.rs": [
        "faf8b90848eee08a570d4e2dd8a0009083730f8c",
        "502848230d0e89bb051ca28b354fa1243a7cae0e"
    ],
    "codex-rs/core/src/tools/handlers/unified_exec_tests.rs": [
        "d834c249825cd532f7e29ba7d814a2e69f2b181a",
        "ecff8f503902403c7866d70b4f9cb9c7f724a96a"
    ],
    "codex-rs/core/src/tools/runtimes/unified_exec.rs": [
        "2642a9c1c00b7b6ec217b79dd066d917e6c231d8",
        "61d66c6927bf18ec852de797a151a1235882101e"
    ],
    "codex-rs/core/src/tools/runtimes/unified_exec/snapshot_metrics.rs": [
        "2f5ff3ecd19904aafdbee110b5c9ea2f7054f162",
        "e5b8c8f60a0ca4d552318ac7a9559d52f202a36f"
    ],
    "codex-rs/core/src/tools/runtimes/mod.rs": [
        "7145665cc5765cbf1e6fdfa031a6a1f6f706c398",
        "359bc770f2b48fb8ce032cdca41c861e624acb2e"
    ],
    "codex-rs/core/src/tools/runtimes/mod_tests.rs": [
        "e6f63ff8ca33f10ab46a905e8ce2863db993ef64",
        "d5933281d818fced11215d7e92e4104ac509a24d"
    ],
    "codex-rs/core/src/tools/runtimes/apply_patch_tests.rs": [
        "fc1bbe56306da3f0dac1a33b20c277930b8ee515",
        "8dd60a07c8b8a1758568912b55b82597a1a02e28"
    ],
    "codex-rs/core/src/unified_exec/mod.rs": [
        "28af9b02290f9ac4b9dc9efa2eb3476df45cf556",
        "c4ea0991abf73157f469fd9ca34a172300471a95"
    ],
    "codex-rs/core/src/unified_exec/mod_tests.rs": [
        "1b8af5bc0d6f47598b615901e80155712948b556",
        "20f2236668e40aab7a93d5a8b0ff4a990fe885ab"
    ],
    "codex-rs/core/src/unified_exec/oneshot.rs": [
        "460aaeb462391824a608befbe3757ca7cb0068c2",
        "5136c9139e12d67229ad6e8ed003325622e8afd6"
    ],
    "codex-rs/core/src/unified_exec/process.rs": [
        "8305c02894737e70708a19b9dace6fee8d425ae5",
        "b3ddb029521bfe67e57e08114f60056f45b22a79"
    ],
    "codex-rs/core/src/unified_exec/process_manager.rs": [
        "2940235c6a3b254a867fbac57d7b4accd15c2053",
        "cda8c59dd87a4b3959985272d9b63a77c4fe5fb4"
    ],
    "codex-rs/core/src/unified_exec/process_manager_tests.rs": [
        "ce56681e68b43214f3baebcf144234a394ef44e7",
        "cc560d3cba42236bc123d62adbc4506dd2d97fdf"
    ],
    "codex-rs/core/src/unified_exec/process_tests.rs": [
        "ed59dfd4865406ae472bbae1fd95c2abe20d21a3",
        "29dabeee2db9848f835a04fdcab87690f24c2222"
    ],
    "codex-rs/core/src/unified_exec/async_watcher_tests.rs": [
        "844399455168f00ad8b705a965036e4c8e14ed75",
        "5e9973c75e80cec190ba92cf15cb4d02f2d03178"
    ],
    "codex-rs/core/src/unified_exec/shell_snapshot.rs": [
        "dd0e489c577a86341d2e56a15662a157aa7f2557",
        "6606f6786df89f78ffae5f9e0bcecf31d040f25d"
    ],
    "codex-rs/core/src/exec_env.rs": [
        "ca1dcb6a935ad9034f62e4d4a3096780dbf1e737",
        "a119fa33a22fc3432a569256d7ade1c5bfc61d93"
    ],
    "codex-rs/protocol/src/shell_environment.rs": [
        "427fe4969f967f5d7fa97a9cba7934a69fb94fd5",
        "0c6c515c416d3829122e8037a3b9c49c1a581608"
    ],
    "codex-rs/file-system/src/lib.rs": [
        "6edd302cf167744eb19ec716b251e4c5c8ef45ef",
        "55ecf43f8dea479a15dc815f20ad820204ae2f38"
    ]
}


def committed(ref: str, path: str) -> str:
    return subprocess.run(
        ["git", "rev-parse", ref + ":" + path],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout.strip()


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT, check=True, capture_output=True, text=True,
        encoding="utf-8",
    ).stdout


class UnifiedExecSandboxSourceTests(unittest.TestCase):
    def test_twenty_sources_exact_official_upstream_git_blobs(self):
        self.assertEqual(len(STAGED), 20)
        for path, (_, official_sha) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(committed("HEAD", path), official_sha)

    def test_original_fork_has_no_edits_to_these_twenty_sources(self):
        for path, (original_sha, _) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(committed(PRODUCTION, path), original_sha)

    def test_exec_handler_no_longer_requires_retired_shell_mode(self):
        handler = source("codex-rs/core/src/tools/handlers/unified_exec.rs")
        exec_handler = source("codex-rs/core/src/tools/handlers/unified_exec/exec_command.rs")
        self.assertIn("fn get_command(", handler)
        self.assertNotIn("UnifiedExecShellMode", handler)
        self.assertNotIn("shell_mode_for_environment", exec_handler)

    def test_zsh_fork_no_longer_registered_in_upstream_runtime(self):
        runtimes = source("codex-rs/core/src/tools/runtimes/mod.rs")
        runtime = source("codex-rs/core/src/tools/runtimes/unified_exec.rs")
        self.assertIn("pub(crate) mod unified_exec;", runtimes)
        self.assertNotIn("pub(crate) mod zsh_fork;", runtimes)
        self.assertNotIn("ZshFork", runtime)

    def test_runtime_and_snapshot_signatures_updated_together(self):
        runtime = source("codex-rs/core/src/tools/runtimes/unified_exec.rs")
        metrics = source("codex-rs/core/src/tools/runtimes/unified_exec/snapshot_metrics.rs")
        self.assertIn("pub fn new(manager: &'a UnifiedExecProcessManager) -> Self", runtime)
        self.assertIn("SnapshotMetrics::start(req, &ctx.step_context.turn.config)", runtime)
        self.assertIn("fn start(req: &UnifiedExecRequest, config: &Config)", metrics)

    def test_exec_sandbox_preparation_is_awaited(self):
        manager = source("codex-rs/core/src/unified_exec/process_manager.rs")
        self.assertIn("attempt.env_for_exec_server(command, options).await", manager)
        self.assertRegex(manager, r"\.env_for\(command, options, network, environment_id\)\s*\.await")
        self.assertNotIn("spawn_lifecycle", manager)

    def test_remote_process_output_drains_before_collection(self):
        process = source("codex-rs/core/src/unified_exec/process.rs")
        manager = source("codex-rs/core/src/unified_exec/process_manager.rs")
        for marker in ("REMOTE_OUTPUT_DRAIN_GRACE", "OutputDrainPolicy::WaitForOutputClosure", "collection_finished"):
            self.assertIn(marker, process)
        for marker in ("OutputRetention::UntilCollected", "OutputDrainPolicy::WaitForOutputClosure"):
            self.assertIn(marker, manager)

    def test_tool_call_environment_binding_is_sanitized(self):
        protocol = source("codex-rs/protocol/src/shell_environment.rs")
        exec_env = source("codex-rs/core/src/exec_env.rs")
        manager = source("codex-rs/core/src/unified_exec/process_manager.rs")
        self.assertIn("pub fn set_tool_call_id_env_var(", protocol)
        self.assertIn("eq_ignore_ascii_case(CODEX_TOOL_CALL_ID_ENV_VAR)", protocol)
        self.assertIn("set_tool_call_id_env_var;", exec_env)
        self.assertIn("set_tool_call_id_env_var(&mut env, Some(&context.call_id))", manager)

    def test_wire_sandbox_override_field_is_preserved(self):
        file_system = source("codex-rs/file-system/src/lib.rs")
        tests = source("codex-rs/core/src/tools/runtimes/apply_patch_tests.rs")
        self.assertIn("pub sandbox_override: SandboxOverride", file_system)
        self.assertIn('skip_serializing_if = "SandboxOverride::is_no_override"', file_system)
        self.assertIn("sandbox_override: SandboxOverride::NoOverride", tests)

    def test_native_regression_sources_are_staged_without_execution(self):
        manager_tests = source("codex-rs/core/src/unified_exec/process_manager_tests.rs")
        proc_tests = source("codex-rs/core/src/unified_exec/process_tests.rs")
        apply_tests = source("codex-rs/core/src/tools/runtimes/apply_patch_tests.rs")
        self.assertIn("fn output_collection_stays_bounded_across_repeated_drains(", manager_tests)
        self.assertIn("fn remote_process_preserves_executor_sandbox_type(", proc_tests)
        self.assertIn("fn file_system_sandbox_context_respects_sandbox_request(", apply_tests)

    def test_adaptive_policy_sources_and_release_versions_unmodified(self):
        for path in (
            "codex-rs/core/src/tools/handlers/adaptive_signal.rs",
            "codex-rs/core/src/tools/handlers/codexdd_validation.rs",
        ):
            self.assertEqual(committed("HEAD", path), committed(PRODUCTION, path))
        self.assertEqual(
            committed("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )
        self.assertEqual(
            source("codex-rs/codexdd-version.txt").strip(), "0.4.4"
        )
        self.assertEqual(
            source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0"
        )

    def test_release_checkpoint_stays_incomplete(self):
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c4b-unified-exec-reconciliation.md").read_text(
            encoding="utf-8"
        )
        for marker in ("INCOMPLETE / NOT A RELEASE CANDIDATE", "Cargo.lock", "exec-server", "native Rust"):
            self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
