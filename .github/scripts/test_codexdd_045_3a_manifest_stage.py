"""0.4.5 packet 3A.3 — offline checks of an INCOMPLETE upstream manifest stage.

This suite validates Git blob provenance and dependency guardrails only.
It does not run Cargo, fetch upstream, attest Windows release success, or
authorize candidate publication. Run on a clean, exact detached Git checkout.
"""

from __future__ import annotations

import hashlib
import subprocess
import tomllib
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]

OFFICIAL_CARGO_BLOBS = {
    "codex-rs/arg0/Cargo.toml": "fad5169ffa9e150e70fbe725b853f68594ee38bc",
    "codex-rs/code-mode/Cargo.toml": "daa2de54718268e12418ca981af1481841312867",
    "codex-rs/codex-mcp/Cargo.toml": "f54c6f1cbdf112af2f93524508bcce9935700740",
    "codex-rs/core/Cargo.toml": "39c860803af2517d97344d491cd9b995e2e89288",
    "codex-rs/exec-server/Cargo.toml": "0d1f1f5209ca66c6efdd9f6bb54058e89bc7a542",
    "codex-rs/exec/Cargo.toml": "d5190c2caa2d37217a0335e4c9dd84f796486458",
    "codex-rs/external-agent-migration/Cargo.toml": "3f49a1ae00cc48720b766d9e76bc050e954cf0fa",
    "codex-rs/tools/Cargo.toml": "fc75ad22ce146ee5dd852ef2cad03b4bbcdffffb",
    "codex-rs/tui/Cargo.toml": "ea0894b3e1d3777f4c598727b03199c1f69cbd05",
}
WORKSPACE_BLOB = "cfebd9b784ef9ff027b274aa9f7101c9c2496613"
RESOLVED_LOCK_BLOB = "13bc15ff844e93dfaf7777c73bb7313b35bb3b81"
BASELINE_SHELL_ESCALATION_BLOB = "92ae8b033f3c38edb1609e6347084be6dd9fdc47"


def git_bytes(path: str) -> bytes:
    result = subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT,
        capture_output=True,
        check=True,
    )
    return result.stdout


def git_blob_sha(payload: bytes) -> str:
    header = b"blob " + str(len(payload)).encode("ascii") + b"\x00"
    return hashlib.sha1(header + payload).hexdigest()


def cargo(path: str):
    return tomllib.loads(git_bytes(path).decode("utf-8"))


class CargoManifestStagingTests(unittest.TestCase):
    def test_eight_official_manifests_and_reviewed_fork_tools_manifest(self):
        # Packet 3A.3 initially staged all nine official manifests. The later
        # compile reconciliation restored two fork-owned tool dependencies;
        # require that exact reviewed change rather than a stale official blob.
        self.assertEqual(len(OFFICIAL_CARGO_BLOBS), 9)
        for path, expected in OFFICIAL_CARGO_BLOBS.items():
            with self.subTest(path=path):
                actual = git_blob_sha(git_bytes(path))
                if path == "codex-rs/tools/Cargo.toml":
                    self.assertEqual(actual, "f47815537ffdd381161130ac0c26bc8d1fd30868")
                    tools = cargo(path)["dependencies"]
                    self.assertEqual(
                        tools["codex-utils-absolute-path"], {"workspace": True}
                    )
                    self.assertEqual(tools["tracing"], {"workspace": True})
                else:
                    self.assertEqual(actual, expected)

    def test_all_staged_manifests_parse_as_toml(self):
        for path in ("codex-rs/Cargo.toml", *OFFICIAL_CARGO_BLOBS):
            with self.subTest(path=path):
                self.assertIsInstance(cargo(path), dict)

    def test_reconciled_workspace_manifest_remains_exact_reviewed_blob(self):
        self.assertEqual(git_blob_sha(git_bytes("codex-rs/Cargo.toml")), WORKSPACE_BLOB)

    def test_security_yaml_alias_is_pinned_to_fork_replacement(self):
        dependencies = cargo("codex-rs/Cargo.toml")["workspace"]["dependencies"]
        self.assertEqual(
            dependencies["serde_yaml"],
            {"package": "noyalib-serde-yaml", "version": "=0.0.57"},
        )
        self.assertNotIn(
            'serde_yaml = "0.9"', git_bytes("codex-rs/Cargo.toml").decode()
        )

    def test_rustls_minimum_and_rmcp_revisions_are_preserved(self):
        dependencies = cargo("codex-rs/Cargo.toml")["workspace"]["dependencies"]
        self.assertEqual(dependencies["rustls"]["version"], "0.23.46")
        self.assertFalse(dependencies["rustls"]["default-features"])
        rmcp = dependencies["rmcp"]
        self.assertEqual(rmcp["version"], "=3.5.1")
        self.assertEqual(rmcp["rev"], "4048ac40e958fb1a7d614617a2ae8d26666ee2fa")

    def test_added_upstream_workspace_dependencies_are_retained(self):
        deps = cargo("codex-rs/Cargo.toml")["workspace"]["dependencies"]
        self.assertEqual(
            deps["zip"],
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
        self.assertEqual(deps["linkify"], "0.11")
        self.assertEqual(deps["hyper-util"]["version"], "0.1.20")
        self.assertEqual(deps["hyper-util"]["features"], ["tokio"])
        self.assertNotIn("codex-shell-escalation", deps)
        self.assertNotIn("socket2", deps)

    def test_core_and_tui_adopt_required_upstream_dependencies(self):
        core = cargo("codex-rs/core/Cargo.toml")["dependencies"]
        tui = cargo("codex-rs/tui/Cargo.toml")["dev-dependencies"]
        for name in ("hyper-util", "tonic"):
            self.assertEqual(core[name], {"workspace": True})
        self.assertEqual(core["tower"]["version"], "0.5.3")
        self.assertEqual(tui["opentelemetry_sdk"], {"workspace": True})

    def test_other_upstream_crate_manifests_track_removals_and_additions(self):
        self.assertNotIn(
            "codex-shell-escalation", cargo("codex-rs/arg0/Cargo.toml")["dependencies"]
        )
        self.assertEqual(
            cargo("codex-rs/external-agent-migration/Cargo.toml")["dependencies"][
                "linkify"
            ],
            {"workspace": True},
        )
        self.assertIn("rand", cargo("codex-rs/code-mode/Cargo.toml")["dependencies"])
        self.assertIn("insta", cargo("codex-rs/exec/Cargo.toml")["dev-dependencies"])
        self.assertIn(
            "codex-utils-process",
            cargo("codex-rs/codex-mcp/Cargo.toml")["dependencies"],
        )

    def test_resolver_produced_lockfile_is_pinned_for_0163_integration(self):
        self.assertEqual(
            git_blob_sha(git_bytes("codex-rs/Cargo.lock")), RESOLVED_LOCK_BLOB
        )
        lock = tomllib.loads(git_bytes("codex-rs/Cargo.lock").decode("utf-8"))
        self.assertEqual(lock["version"], 4)

    def test_historical_shell_escalation_crate_is_not_silently_deleted_yet(self):
        # Target removes this crate from the workspace, but removing its
        # complete source tree is a separate source-integration decision.
        self.assertEqual(
            git_blob_sha(git_bytes("codex-rs/shell-escalation/Cargo.toml")),
            BASELINE_SHELL_ESCALATION_BLOB,
        )

    def test_product_and_tracked_upstream_release_markers_not_advanced(self):
        self.assertEqual(git_bytes("codex-rs/codexdd-version.txt").strip(), b"0.4.4")
        self.assertEqual(
            git_bytes("codex-rs/upstream-codex-release.txt").strip(),
            b"rust-v0.162.0",
        )

    def test_phase3_integration_remains_explicitly_incomplete(self):
        notes = ROOT / "docs/codexdd-0.4.5-packet-3a3-crate-manifests.md"
        text = notes.read_text(encoding="utf-8")
        self.assertIn("INCOMPLETE / NOT A RELEASE CANDIDATE", text)
        self.assertIn("Cargo.lock", text)
        self.assertIn("109 overlapping", text)


if __name__ == "__main__":
    unittest.main()
