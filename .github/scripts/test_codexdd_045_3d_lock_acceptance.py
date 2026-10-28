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
# Current 0.4.5 nonrelease graph, verified by native Windows owner tests
# and original unchanged all-target cargo-deny. Not a shipping acceptance.
WINDOWS_RESOLVED_LOCK = "13bc15ff844e93dfaf7777c73bb7313b35bb3b81"
SECURE_CARGO_TOML = "cfebd9b784ef9ff027b274aa9f7101c9c2496613"
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
        self.assertEqual(
            oid(git_file("HEAD", "codex-rs/Cargo.toml")), SECURE_CARGO_TOML
        )

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

    def test_crypto_bigint_owners_and_tls_root_fixes_remain_locked(self):
        # All dependency owners must remain on the reviewed secure graph;
        # no vulnerable 0.5.x crypto-bigint is permitted in any target.
        self.assertEqual(
            sorted(version for name, version in self.packages if name == "crypto-bigint"),
            ["0.7.10"],
        )
        for name, version in (
            ("ctutils", "0.4.6"),
            ("p256", "0.14.0"),
            ("p384", "0.14.0"),
            ("hpke", "0.14.1"),
            ("age", "0.12.1"),
            ("age-core", "0.12.0"),
            ("clatter", "3.0.0"),
            ("aws-config", "1.13.0"),
            ("rtc", "0.21.1"),
            ("rustls-webpki", "0.103.16"),
            ("webpki-roots", "1.0.10"),
            ("webpki-root-certs", "1.0.10"),
        ):
            with self.subTest(name=name):
                self.assertIn((name, version), self.packages)
                self.assertFalse(
                    any(key == name and ver != version for key, ver in self.packages)
                )
        # Exact Git revisions rather than floating Git refs or registry-only
        # substitutions are required for the reviewed encryption and KEM owners.
        for name, url, rev, version in (
            ("age", "https://github.com/str4d/rage", "b5b68c4f4f7ea5c368bc060f6fe0237549c3a13f", "0.12.1"),
            ("age-core", "https://github.com/str4d/rage", "b5b68c4f4f7ea5c368bc060f6fe0237549c3a13f", "0.12.0"),
            ("clatter", "https://github.com/jmlepisto/clatter", "9a8d15c4f80d5911ca0403aa22e0de99ea59df08", "3.0.0"),
        ):
            with self.subTest(name=name):
                self.assertEqual(
                    self.packages[(name, version)]["source"],
                    f"git+{url}?rev={rev}#{rev}",
                )
        self.assertNotIn("source", self.packages[("aws-config", "1.13.0")])

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
