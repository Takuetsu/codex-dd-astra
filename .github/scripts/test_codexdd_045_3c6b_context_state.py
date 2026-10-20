"""CodexDD 0.4.5 3C.6b: context history, world state and session state provenance.

Read-only static source guards. They do not prove native Rust execution,
user authorization semantics, context restoration, or release acceptance.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
UPSTREAM = "83086188ae0614bd882818ff1aa23f3194962012"
STAGED = {
    "codex-rs/core/src/context/guardian_sender_messages.rs": [
        "b1bc4ece091f313ae1ad374233f68d07dee38ef3",
        "ad472d5fb78beb9f84328fa84fc90b7e0027e8f3"
    ],
    "codex-rs/core/src/context/world_state/environment.rs": [
        "c40ee11c15d8b17972305b813d5ec9e693448dd4",
        "516b634cd4b1e0b475778241072b0a2eddd7d1a3"
    ],
    "codex-rs/core/src/context/world_state/mod.rs": [
        "6a5d22db80eb289e9c7d124a476ee3fde1869692",
        "857649f66285b750f8b11476a4c1b67537dfe606"
    ],
    "codex-rs/core/src/context/world_state/top_level_tools.rs": [
        "5fc9b5b6f31f59102d50930d6061c611b02657f9",
        "e046f193e9239b89c070fa027c1cc3ffce14b536"
    ],
    "codex-rs/core/src/context/world_state/top_level_tools_tests.rs": [
        "dd15408619fc1cb950e863b751a4670d51279314",
        "d205e37a1fc32968f682251d9eea80757e93d878"
    ],
    "codex-rs/core/src/context/world_state/world_state_tests.rs": [
        "a5ceddc775bfe59be841f6e66d49a00f99a7d59a",
        "8e9769394b070e54cf32a851e24bf53655d52699"
    ],
    "codex-rs/core/src/context_manager/history.rs": [
        "38ef4a59c012ae5ac3bc40330ea81b7314458c04",
        "db1c0e4096a8c39eec4958e0a25829ba0d3257fa"
    ],
    "codex-rs/core/src/context_manager/history_user_authorization.rs": [
        "1a2fafcd93939c9464e6958dce22e96afd6c3d21",
        "3d0f72b1f840e395c2db253273e76c098451f285"
    ],
    "codex-rs/core/src/context_manager/updates.rs": [
        "576d546849eeaeb83eb30f536c6373b391d94cae",
        "1052d92d9afd49804c44f64ee6fef12a100b0312"
    ],
    "codex-rs/core/src/context_manager/updates_tests.rs": [
        "026058bbdfdf234dd1bb5d0a182ad14c5c7324ec",
        "50472118bcfb2c89b32e938119062711c3cdf0c8"
    ],
    "codex-rs/core/src/state/service.rs": [
        "bd723c453f1a466c39dcdba229a777d8c1c846a9",
        "3671386bd4bf4e57a94f4ce18e782a9401fd511c"
    ],
    "codex-rs/core/src/state/session.rs": [
        "53857e1cc2c719ef7a5616471e180247d00e88b8",
        "fff1bf667f6b21c15cb0dbce686adeaf03a12677"
    ],
    "codex-rs/core/src/state/turn.rs": [
        "292e0a21fba545c489d9bad585c6fe5e43febe67",
        "aa43776d867a28dde1692ee79bff965b6652deca"
    ]
}
EXISTING_CUSTOM = {
    "codex-rs/core/src/context_manager/normalize.rs": "4ca88c6a668e9896df06ca81940a0e41e357898d",
    "codex-rs/core/src/context_manager/history_tests.rs": "4f8415b4ce3f3994d4fe8eb87bcbd22e6588848d",
}


def git_blob(ref: str, path: str) -> str:
    return subprocess.run(
        ["git", "rev-parse", ref + ":" + path],
        cwd=ROOT, check=True, capture_output=True, text=True,
    ).stdout.strip()


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT, check=True, capture_output=True, text=True, encoding="utf-8",
    ).stdout


class HistoryAndStateReconciliationTests(unittest.TestCase):
    def assertIn(self, member, container, msg=None):
        if isinstance(container, str) and len(container) > 2048:
            self.assertTrue(member in container, msg or f"missing source marker: {member!r}")
            return
        super().assertIn(member, container, msg)

    def test_thirteen_pinned_upstream_blobs(self):
        self.assertEqual(len(STAGED), 13)
        for path, (original, official) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob("HEAD", path), official)
                # The target blob is pinned from official 0.163.0 during GitHub staging.
                # Do not dereference UPSTREAM locally: it may not be fetched here.

    def test_thirteen_files_have_no_0_4_4_fork_changes(self):
        for path, (original, _) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob(PRODUCTION, path), original)

    def test_existing_interrupted_history_merge_is_unchanged(self):
        for path, expected in EXISTING_CUSTOM.items():
            with self.subTest(path=path):
                self.assertEqual(git_blob("HEAD", path), expected)

    def test_upstream_history_retains_guardian_review_revision_api(self):
        history = source("codex-rs/core/src/context_manager/history.rs")
        self.assertIn("guardian_review_context_revision", history)
        self.assertIn("record_retained_context", history)
        self.assertIn("guardian_history_checkpoint", history)

    def test_user_history_authorization_source_staged(self):
        authorized = source("codex-rs/core/src/context_manager/history_user_authorization.rs")
        self.assertIn("restore_retained_context", authorized)
        self.assertIn("record_retained_message", authorized)

    def test_world_state_rendering_has_current_legacy_matchers(self):
        world = source("codex-rs/core/src/context/world_state/mod.rs")
        self.assertIn("matches_retained_fragment", world)
        self.assertIn("render_diff", world)

    def test_core_state_approval_waiters_are_preserved(self):
        state = source("codex-rs/core/src/state/turn.rs")
        self.assertIn("insert_pending_approval", state)
        self.assertIn("remove_pending_approval", state)
        self.assertIn("remove_pending_request_permissions", state)

    def test_context_and_world_state_native_test_sources_staged(self):
        self.assertIn("fn ", source("codex-rs/core/src/context/world_state/world_state_tests.rs"))
        self.assertIn("fn ", source("codex-rs/core/src/context/world_state/top_level_tools_tests.rs"))
        self.assertIn("fn ", source("codex-rs/core/src/context_manager/updates_tests.rs"))

    def test_adaptive_policy_still_restricts_dispatch(self):
        self.assertEqual(git_blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
                         "fc75b3bbf91f1881bd32faaaca93356fbb598b5e")

    def test_release_markers_and_lockfile_unadvanced(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(git_blob("HEAD", "codex-rs/Cargo.lock"),
                         "a8eb8a076276cb3db5674b33c67d743b1c3d21fc")

    def test_checkpoint_awaits_native_rust_validation(self):
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c6b-context-state.md").read_text(encoding="utf-8")
        for marker in ("INCOMPLETE / NOT A RELEASE CANDIDATE", "native Rust",
                       "Cargo.lock", "source-only"):
            self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
