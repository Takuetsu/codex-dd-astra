"""CodexDD 0.4.5 3C.9e2: source-only read-state event routing guard.

This is not native Rust validation, a live app-server test, Windows release
acceptance, or authorization for upstream auto-publishing.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
PATH = "codex-rs/tui/src/app/app_server_event_targets.rs"
OLD_BLOB = "521afcee6d57e0adfdb6ffdf89aad77f6bc7c11e"
NEW_BLOB = "16d6b02da4469a7f1ef2d82916a48d04719124d5"
OLD = (
    "        | ServerNotification::AccountUpdated(_)\n"
    "        | ServerNotification::GatewayOAuthChanged(_)"
)
NEW = (
    "        | ServerNotification::AccountUpdated(_)\n"
    "        | ServerNotification::ThreadReadStateChanged(_)\n"
    "        | ServerNotification::GatewayOAuthChanged(_)"
)


def git(*args: str) -> str:
    return subprocess.run(
        ["git", *args], cwd=ROOT, check=True,
        text=True, encoding="utf-8", capture_output=True,
    ).stdout.strip()


def source(path: str, ref: str = "HEAD") -> str:
    return subprocess.run(
        ["git", "show", ref + ":" + path], cwd=ROOT, check=True,
        text=True, encoding="utf-8", capture_output=True,
    ).stdout


class ThreadReadStateRoutingTests(unittest.TestCase):
    def test_reviewed_source_provenance(self):
        self.assertEqual(git("rev-parse", PRODUCTION + ":" + PATH), OLD_BLOB)
        self.assertEqual(git("rev-parse", "HEAD:" + PATH), NEW_BLOB)

    def test_one_upstream_match_arm_is_only_change(self):
        original = source(PATH, PRODUCTION)
        self.assertEqual(original.count(OLD), 1)
        self.assertEqual(source(PATH), original.replace(OLD, NEW, 1))

    def test_one_read_state_arm_and_no_accidental_duplicate(self):
        self.assertEqual(source(PATH).count(
            "ServerNotification::ThreadReadStateChanged(_)"
        ), 1)

    def test_read_state_is_broadcast_global_not_retargeted_to_thread(self):
        value = source(PATH)
        fragment = value.split("ServerNotification::ProjectChanged(_)", 1)[1]
        fragment = fragment.split("=> None,", 1)[0]
        self.assertIn("ServerNotification::ThreadReadStateChanged(_)", fragment)
        self.assertIn("ServerNotification::AccountUpdated(_)", fragment)
        self.assertIn("ServerNotification::GatewayOAuthChanged(_)", fragment)
        self.assertIn("None => ServerNotificationThreadTarget::Global,", value)

    def test_trusted_adaptive_runtime_signal_is_still_thread_scoped(self):
        value = source(PATH)
        self.assertIn("ServerNotification::AdaptiveRuntimeSignal(notification)", value)
        self.assertIn("Some(notification.thread_id.as_str())", value)
        self.assertIn("ServerNotificationThreadTarget::Thread(thread_id)", value)

    def test_existing_thread_id_and_invalid_id_handling_preserved(self):
        value = source(PATH)
        self.assertIn("ServerNotificationThreadTarget::InvalidThreadId(", value)
        self.assertIn("ServerNotification::Warning(notification) => notification.thread_id.as_deref()", value)

    def test_protocol_has_the_new_notification_variant(self):
        proto = source("codex-rs/app-server-protocol/src/protocol/common.rs")
        self.assertIn(
            'ThreadReadStateChanged => "thread/readState/changed"',
            proto,
        )

    def test_protocol_payload_retains_thread_id_and_read_state(self):
        thread = source("codex-rs/app-server-protocol/src/protocol/v2/thread.rs")
        self.assertIn("pub struct ThreadReadStateChangedNotification {", thread)
        self.assertIn("pub thread_id: String,", thread)
        self.assertIn("pub read_state: ThreadReadState,", thread)

    def test_inventory_pins_new_reviewed_blob(self):
        guard = source(".github/scripts/test_codexdd_045_3c9c_tui_upstream.py")
        self.assertIn('"' + PATH + '": "' + NEW_BLOB + '"', guard)
        self.assertIn("self.assertEqual(len(PRESERVED_FORK), 44)", guard)

    def test_release_and_lockfile_controls_unadvanced(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(
            source("codex-rs/upstream-codex-release.txt").strip(),
            "rust-v0.162.0",
        )
        self.assertEqual(
            git("rev-parse", "HEAD:codex-rs/Cargo.lock"),
            "48ae4fb45a83de4d72dee6200b83a770dbf8f760",
        )

    def test_checkpoint_denies_native_or_release_authority(self):
        note = (
            ROOT / "docs/codexdd-0.4.5-packet-3c9e2-read-state-routing.md"
        ).read_text(encoding="utf-8")
        for marker in (
            "INCOMPLETE / NOT A RELEASE CANDIDATE",
            "source-only",
            "35 unresolved",
            "Native Rust",
            "Cargo.lock",
        ):
            with self.subTest(marker=marker):
                self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
