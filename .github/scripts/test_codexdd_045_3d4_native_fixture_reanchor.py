"""Nonrelease 0.4.5 3D.4: exact 0.163 native-test fixture provenance.

The original 3C.5b/3C.7c/3C.7d reviewed source blobs are immutable historical
checkpoints. These checks independently prove that the only subsequent changes
to the two fork-preserved test files add required optional 0.163 fields.
They do not waive source guard failures or authorize release promotion.
"""

from __future__ import annotations

import hashlib
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

LINEAGE = (
    (
        "codex-rs/core/src/client_tests.rs",
        "927ef6e51c248d7979e391272b188982c8a7d4b9",
        "2e5525612379a29af24116dd738a6612dc4420e6",
        b'        input: vec![ResponseItem::FunctionCall {\n            id: None,\n            name: "report_adaptive_signal".to_string(),',
        b'        input: vec![ResponseItem::FunctionCall {\n            encrypted_content: None,\n            status: None,\n            id: None,\n            name: "report_adaptive_signal".to_string(),',
    ),
    (
        "codex-rs/core/src/session/rollout_reconstruction_tests.rs",
        "6286fb33b5d936da253abe9efdc090e494de0ac4",
        "106f07a29d8cde54784c516898472518736b9c37",
        b"    let call = ResponseItem::CustomToolCall {\n        id: None,\n        status: None,\n        call_id:",
        b"    let call = ResponseItem::CustomToolCall {\n        encrypted_content: None,\n        id: None,\n        status: None,\n        call_id:",
    ),
)


def git_bytes(*args: str) -> bytes:
    return subprocess.run(
        ["git", *args], cwd=ROOT, check=True, capture_output=True
    ).stdout


def oid(payload: bytes) -> str:
    return hashlib.sha1(
        b"blob " + str(len(payload)).encode("ascii") + b"\x00" + payload
    ).hexdigest()


class ReviewedNativeFixtureReanchor(unittest.TestCase):
    def test_two_exact_rewrites_preserve_every_other_byte(self):
        for path, historical, accepted, before, after in LINEAGE:
            with self.subTest(path=path):
                original = git_bytes("cat-file", "blob", historical)
                self.assertEqual(oid(original), historical)
                self.assertEqual(original.count(before), 1)
                self.assertNotIn(after, original)
                expected = original.replace(before, after, 1)
                self.assertEqual(oid(expected), accepted)
                current = git_bytes("show", "HEAD:" + path)
                self.assertEqual(current, expected)
                self.assertEqual(oid(current), accepted)

    def test_release_markers_still_frozen(self):
        self.assertEqual(
            git_bytes("show", "HEAD:codex-rs/codexdd-version.txt").strip(), b"0.4.4"
        )
        self.assertEqual(
            git_bytes("show", "HEAD:codex-rs/upstream-codex-release.txt").strip(),
            b"rust-v0.162.0",
        )
