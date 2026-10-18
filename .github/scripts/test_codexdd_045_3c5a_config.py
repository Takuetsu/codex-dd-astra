"""CodexDD 0.4.5 3C.5a configuration source and adaptive worker tripwires.

Static/offline only. No native Rust proof, lockfile renewal, or release authority.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
STAGED = {
    "codex-rs/core/src/config/config_loader_tests.rs": [
        "39df96b292327bd7e12993bd11edabbd8e7e2a58",
        "5e0c5b44351745dc4ebf152ff7ffc9d39e2bc935"
    ],
    "codex-rs/core/src/config/config_tests.rs": [
        "806bd8082ff81b2484e8e2c840407b20bf5eda50",
        "aaf2165a6b35895e3aba1a933d080809c8317bbf"
    ],
    "codex-rs/core/src/config/edit.rs": [
        "9996db9abd5ed8bf67fdbc1f85dcb338e4401e15",
        "5318db9f903eedefb17748934d4b852516e17e70"
    ],
    "codex-rs/core/src/config/edit_tests.rs": [
        "09c5eb0211e3efee954e3f69b66b73eb1a242424",
        "25fa3b1b04845b789f7efe6bff4d70517dfdfc69"
    ],
    "codex-rs/core/src/config/managed_features.rs": [
        "80c951c488567a85ca67e792d6678c3b2ca4ae0a",
        "a6519b2a442e00e3de0cfa10529d0c50ed9d213d"
    ],
    "codex-rs/core/src/config/network_config.rs": [
        "6de02ee17f0ddb1fa9e32173aef414204acbbc16",
        "505d896abec7ebf9d6cf5e9a3665ff29081aa065"
    ],
    "codex-rs/core/src/config/network_proxy_spec.rs": [
        "53a61ddd100fc05b7106f4e43d08daef121ca163",
        "ace90ef9c204f38c392f206db09a933a9c77cb3c"
    ],
    "codex-rs/core/src/config/permission_profile_catalog.rs": [
        "224465ce7da3a641e91a34623efd0f66d7887456",
        "1ebad6ab9bcd0742bb0eeed504598e7d92f06c8a"
    ],
    "codex-rs/core/src/config/permissions.rs": [
        "9cef5b075adfc6f62618e16b82deeef2b73fd4a5",
        "bd6ddb1f091c330761251e0265ffd323abc471bc"
    ],
    "codex-rs/core/src/config/permissions_tests.rs": [
        "6b385ff25ba09f1cf5beb93890450bc98ea01847",
        "b3c5d9bcb743ec91f5128e4f2c218e69db1d4a9c"
    ],
    "codex-rs/core/src/config/runtime_refresh.rs": [
        "995117dedfb730c98d8d78e81a885cbe7e2fc465",
        "05b017376a38049d904976c7772ad9e713a84203"
    ],
    "codex-rs/core/src/config/runtime_refresh_tests.rs": [
        "c52c89348cec12d4d888f71b0bd151e8021a2ef1",
        "698d7948674f163864d6fbe57b478b39134b8950"
    ],
    "codex-rs/core/src/config/windows_sandbox_config.rs": [
        "dbe88ba9f166a7b93f39947a04e18888ae16f7ed",
        "0b681f1d8ad654e7e8b1db1a50bef8dc938b4fd8"
    ],
    "codex-rs/config/src/config_requirements.rs": [
        "250b3b3c151f55e669a20940b1b0d3be112363be",
        "464c55e94a9c00f0eecb41e91820b612304800b1"
    ],
    "codex-rs/config/src/loader/local.rs": [
        "c235166cc2d1d629233f9942f23929c3886fc28c",
        "f45c2047466dd83a6a9afb9f2ba379232005a789"
    ],
    "codex-rs/config/src/loader/mod.rs": [
        "e524076d7b8dd0b8bff03c0082265fd4d8ab92b7",
        "46dbb8b055768542c4d0360059823411cdc66023"
    ],
    "codex-rs/config/src/loader/tests.rs": [
        "e1ae693f30340744eecc53d076c27b4c0d9ffbc8",
        "e38074b81298e5efb5c1349e6f0b67caaa32b252"
    ],
    "codex-rs/config/src/profile_toml.rs": [
        "501647034227ef3f2fef571b8553b0de9a1f6d4d",
        "5beb838d5456653bc49f079c9b73ed4b81c637f0"
    ],
    "codex-rs/config/src/schema.rs": [
        "fd144796ee2dcb8d4b91143d7f96083ade4e35f4",
        "770e73ee5cca4b51b73109a36045edc1bea65a61"
    ],
    "codex-rs/config/src/tui_keymap.rs": [
        "0caf64d5ea591137dd9e7e27abaeeef99d9a606f",
        "47360c88c46992609309366cecae3a7ccc4e86d5"
    ],
    "codex-rs/config/src/types.rs": [
        "fab6fb2e95e9a97b1d363bc8114508f43a3abb51",
        "2dbdfe2cc82f6e44c686801eb2bf1b56690c334a"
    ],
    "codex-rs/config/src/types_tests.rs": [
        "5a9b3d81d9161e0f8ed4928e96ac8b751eb5e1a2",
        "5b5303b6d94828cf6312f7237e4efa9e31a2e081"
    ]
}
RECONCILED_CORE_BLOB = "17680eeef69718965df66cc12466985c84bdcd63"
RECONCILED_TOML_BLOB = "a07f08c8831dd011d4950a52088c7233722620d8"


def committed_blob(ref: str, path: str) -> str:
    return subprocess.run(
        ["git", "rev-parse", ref + ":" + path],
        cwd=ROOT, capture_output=True, text=True, check=True,
    ).stdout.strip()


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT, capture_output=True, text=True, check=True, encoding="utf-8",
    ).stdout


class ConfigWorkerReconciliationTests(unittest.TestCase):
    def assertIn(self, member, container, msg=None):
        if isinstance(container, str) and len(container) > 2048:
            self.assertTrue(member in container, msg or f"missing marker: {member!r}")
            return
        super().assertIn(member, container, msg)

    def test_twenty_two_exact_upstream_config_blobs(self):
        self.assertEqual(len(STAGED), 22)
        for path, (_, expected) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(committed_blob("HEAD", path), expected)

    def test_original_0_4_4_fork_unchanged_in_staged_files(self):
        for path, (expected, _) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(committed_blob(PRODUCTION, path), expected)

    def test_both_merged_files_pin_reviewed_blobs(self):
        self.assertEqual(
            committed_blob("HEAD", "codex-rs/core/src/config/mod.rs"),
            RECONCILED_CORE_BLOB,
        )
        self.assertEqual(
            committed_blob("HEAD", "codex-rs/config/src/config_toml.rs"),
            RECONCILED_TOML_BLOB,
        )

    def test_external_role_is_strict_enum_not_privilege_toggle(self):
        toml = source("codex-rs/config/src/config_toml.rs")
        self.assertIn('pub enum AdaptiveWorkerRoleToml {', toml)
        for role in ("Unspecified,", "Implementation,", "Validation,", "Repair,"):
            self.assertIn(role, toml)
        self.assertIn('#[serde(rename_all = "snake_case")]', toml)
        self.assertIn("pub struct AdaptiveWorkerConfigToml {", toml)

    def test_external_worker_scope_is_optional_and_schema_strict(self):
        toml = source("codex-rs/config/src/config_toml.rs")
        self.assertIn("pub authorized_scope: Option<String>,", toml)
        self.assertIn("#[schemars(deny_unknown_fields)]\npub struct AdaptiveWorkerConfigToml {", toml)
        self.assertIn("pub adaptive_worker: Option<AdaptiveWorkerConfigToml>,", toml)

    def test_core_config_retains_external_authority(self):
        core = source("codex-rs/core/src/config/mod.rs")
        self.assertIn("use codex_config::config_toml::AdaptiveWorkerConfigToml;", core)
        self.assertIn("pub adaptive_worker: Option<AdaptiveWorkerConfigToml>,", core)
        self.assertIn("adaptive_worker: cfg.adaptive_worker,", core)

    def test_worker_config_deserialization_and_unknown_role_regressions_survive(self):
        toml = source("codex-rs/config/src/config_toml.rs")
        self.assertIn("fn adaptive_worker_and_daybreak_settings_deserialize_together()", toml)
        self.assertIn("fn adaptive_worker_rejects_unrecognized_role()", toml)
        self.assertIn('role = "administrator"', toml)

    def test_upstream_daybreak_and_service_tier_retained(self):
        core = source("codex-rs/core/src/config/mod.rs")
        toml = source("codex-rs/config/src/config_toml.rs")
        self.assertIn("daybreak_enabled: cfg.daybreak.unwrap_or(false),", core)
        self.assertIn("pub daybreak: Option<bool>,", toml)
        self.assertIn("pub service_tier: Option<String>,", core)

    def test_upstream_app_security_config_remains(self):
        core = source("codex-rs/core/src/config/mod.rs")
        for marker in ("application_network_policy", "application_auth_route_config",
                       "ConfigRequirements", "FileSystemSandboxPolicy"):
            self.assertIn(marker, core)

    def test_reconciled_core_has_exactly_one_worker_assignment(self):
        core = source("codex-rs/core/src/config/mod.rs")
        self.assertEqual(core.count("adaptive_worker: cfg.adaptive_worker,"), 1)
        self.assertEqual(core.count("pub adaptive_worker: Option<AdaptiveWorkerConfigToml>,"), 1)

    def test_reconciled_toml_has_exactly_one_worker_field(self):
        toml = source("codex-rs/config/src/config_toml.rs")
        self.assertEqual(toml.count("pub adaptive_worker: Option<AdaptiveWorkerConfigToml>,"), 1)
        self.assertEqual(toml.count("pub enum AdaptiveWorkerRoleToml {"), 1)

    def test_runtime_refresh_and_windows_config_sources_staged(self):
        runtime = source("codex-rs/core/src/config/runtime_refresh.rs")
        windows = source("codex-rs/core/src/config/windows_sandbox_config.rs")
        self.assertIn("RuntimeConfigRefresh", runtime)
        self.assertIn("Windows", windows)

    def test_original_adaptive_registry_policy_still_fixed(self):
        self.assertEqual(
            committed_blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )

    def test_release_markers_and_lockfile_untouched(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(
            committed_blob("HEAD", "codex-rs/Cargo.lock"),
            "0c913b46674d3593ee84f1fee5508d16e33efae5",
        )

    def test_packet_documented_incomplete(self):
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c5a-configuration.md").read_text(encoding="utf-8")
        for marker in ("INCOMPLETE / NOT A RELEASE CANDIDATE",
                       "native Rust", "Cargo.lock", "3C.4"):
            self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
