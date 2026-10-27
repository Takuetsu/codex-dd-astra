"""CodexDD 0.4.5: locked Windows 0.163 resolver acceptance and security floors.

Read-only Git provenance checks. Source and dependency acceptance do NOT
authorize release publication or substitute for running the Windows tests.
"""

from __future__ import annotations

import hashlib
import subprocess
import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
BASELINE_LOCK = "0c913b46674d3593ee84f1fee5508d16e33efae5"
WINDOWS_RESOLVED_LOCK = "a6cd27b0b70fde2f2be9c1777f3666ad3fa318bb"
RMCP_REV = "4048ac40e958fb1a7d614617a2ae8d26666ee2fa"


def git_file(ref: str, path: str) -> bytes:
    return subprocess.run(
        ["git", "show", ref + ":" + path],
        cwd=ROOT,
        capture_output=True,
        check=True,
    ).stdout


def oid(payload: bytes) -> str:
    header = b"blob " + str(len(payload)).encode() + b"\x00"
    return hashlib.sha1(header + payload).hexdigest()


class NativeWindowsLockAcceptance(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.lock_bytes = git_file("HEAD", "codex-rs/Cargo.lock")
        cls.lock = tomllib.loads(cls.lock_bytes.decode("utf-8"))
        cls.packages = {
            (item["name"], item["version"]): item for item in cls.lock["package"]
        }

    def test_exact_resolver_produced_lock_and_historical_production(self):
        self.assertEqual(oid(self.lock_bytes), WINDOWS_RESOLVED_LOCK)
        self.assertEqual(
            oid(git_file(PRODUCTION, "codex-rs/Cargo.lock")), BASELINE_LOCK
        )
        self.assertEqual(self.lock["version"], 4)

    def test_upstream_0163_workspace_versions_and_rmcp_revision(self):
        for crate in ("codex-core", "codex-tui", "codex-app-server", "codex-feedback"):
            with self.subTest(crate=crate):
                self.assertIn((crate, "0.163.0"), self.packages)
        for crate in ("rmcp", "rmcp-macros"):
            package = self.packages[(crate, "3.5.1")]
            self.assertIn("?rev=" + RMCP_REV + "#" + RMCP_REV, package["source"])
        self.assertIn(("linkify", "0.11.0"), self.packages)
        self.assertFalse(
            any(name == "codex-shell-escalation" for name, _ in self.packages)
        )

    def test_reviewed_security_versions_and_yaml_alias_persist(self):
        for name, version in (
            ("rustls", "0.23.46"),
            ("quinn-proto", "0.11.18"),
            ("noyalib", "0.0.57"),
            ("noyalib-serde-yaml", "0.0.57"),
        ):
            with self.subTest(package=name):
                self.assertIn((name, version), self.packages)
        self.assertFalse(
            any(name in ("serde_yaml", "unsafe-libyaml") for name, _ in self.packages)
        )
        manifest = tomllib.loads(git_file("HEAD", "codex-rs/Cargo.toml").decode())
        self.assertEqual(
            manifest["workspace"]["dependencies"]["serde_yaml"],
            {"package": "noyalib-serde-yaml", "version": "=0.0.57"},
        )
        self.assertEqual(
            manifest["workspace"]["dependencies"]["rustls"]["version"],
            "0.23.46",
        )

    def test_zip9_remediation_is_locked_without_replacing_other_security_floors(self):
        self.assertIn(("zip", "9.0.5"), self.packages)
        self.assertFalse(
            any(name == "zip" and version != "9.0.5" for name, version in self.packages)
        )
        manifest = tomllib.loads(git_file("HEAD", "codex-rs/Cargo.toml").decode())
        self.assertEqual(
            manifest["workspace"]["dependencies"]["zip"],
            {
                "version": "9.0.5",
                "default-features": False,
                "features": [
                    "aes-crypto",
                    "deflate64",
                    "deflate",
                    "lzma",
                    "ppmd",
                    "time",
                    "zstd",
                    "xz",
                ],
            },
        )
        self.assertFalse(
            any(name in ("bzip2", "libbz2-rs-sys") for name, _version in self.packages),
            "Do not import unapproved BZIP2 license dependencies",
        )
        self.assertNotIn("bzip2", self.packages[("zip", "9.0.5")]["dependencies"])

    def test_release_markers_not_promoted_by_lock_acceptance(self):
        self.assertEqual(
            git_file("HEAD", "codex-rs/codexdd-version.txt").strip(), b"0.4.4"
        )
        self.assertEqual(
            git_file("HEAD", "codex-rs/upstream-codex-release.txt").strip(),
            b"rust-v0.162.0",
        )


if __name__ == "__main__":
    unittest.main()
