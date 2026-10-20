"""CodexDD 0.4.5 3C.8b: feature and plugin source ownership guardrails.

Offline static tests; do not interpret them as Rust runtime policy proof,
security validation, marketplace approval or release authorization.
"""
from __future__ import annotations

import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCTION = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
STAGED = {
    "codex-rs/features/src/feature_configs.rs": [
        "a67162ee9ba39bfa1565c0e134a0d29166c227c3",
        "e3ae224609dd5c4cad5d4cfbaeb609a63089cda5"
    ],
    "codex-rs/features/src/lib.rs": [
        "c68bc3bb748982281b62ce82b664c646e6bb4c7b",
        "13ff893c1c962ed049f80e6858a130e8bbfbc651"
    ],
    "codex-rs/features/src/tests.rs": [
        "5891dc0c9bcbe6244c9373fd06e4eb07c0362ea4",
        "a8a8fca058a776388db1860447a5bf520aae1f5b"
    ],
    "codex-rs/core-plugins/src/executor_hooks.rs": [
        "b848d97502de81c0c22e665ce187b3fa6ac621bf",
        "abc59ab80b088f33329ab857a7056b2c20a17c7a"
    ],
    "codex-rs/core-plugins/src/executor_hooks_tests.rs": [
        "b0e5f8c35a157ea9a7742ba710a419eef274cda0",
        "fab975d6551c53e26e6faaa193dc93d8c3caa94a"
    ],
    "codex-rs/core-plugins/src/loader.rs": [
        "d0a9e06622cd5440e0a0b01b1eb12c0c04a5d4f6",
        "f0f9a12a779208e683503354f60fffe13d0fa14e"
    ],
    "codex-rs/core-plugins/src/loader_tests.rs": [
        "e997825b0cd682e5a80c9a085ac113f9b8c6afdc",
        "605879bea86f0dbc84744959fffdb356bc7067ec"
    ],
    "codex-rs/core-plugins/src/manager.rs": [
        "07eb179f452cc575811d7fdab0608aebccc0122b",
        "d4411b50da9f459bd8ba267b9c785247c131312d"
    ],
    "codex-rs/core-plugins/src/manager_tests.rs": [
        "27ec9eb0ca77549fb6fe61fbb55d0f6867ef6ce2",
        "6ba94a673866f3aecfa45dd763d7fe7d5a8e9923"
    ],
    "codex-rs/core-plugins/src/marketplace_add/metadata.rs": [
        "07813ce4fe78de16ebfe349dc6a17c6328e03390",
        "9a00fa79f751ab3f4a377bb22308c952b6ed3442"
    ],
    "codex-rs/core-plugins/src/marketplace_policy.rs": [
        "d5907554fb0db454e81d6279927e8531fb552591",
        "46879d2a85f79884598d2a133d88e5c79167d862"
    ],
    "codex-rs/core-plugins/src/marketplace_policy/curated_skill_tests.rs": [
        "958ad664e18e3fded7b4dc7ffbbe53f1d0bc6b65",
        "780163ef27cce5a7c115376ca82d49c9bfd9a289"
    ],
    "codex-rs/core-plugins/src/marketplace_policy_tests.rs": [
        "a2bef0066396f842913b42e2cbb6d1ba73cba695",
        "f838b064fbba3f775fcfe565388a7df68967a222"
    ],
    "codex-rs/core-plugins/src/plugins_config_input_tests.rs": [
        "58c7afb6f44fa706f89dc46b56daefa163f7222c",
        "69cadf451071c03c69412d961fe9a07bb6c51183"
    ],
    "codex-rs/core-plugins/src/test_support.rs": [
        "6b06e32cc13b74cc3398ea209b86ea2c5bd15e75",
        "39e5ea0b4822abc035ae6e7124b7ee1cb666a8ab"
    ]
}


def blob(ref: str, path: str) -> str | None:
    result = subprocess.run(
        ["git", "rev-parse", "--verify", ref + ":" + path],
        cwd=ROOT, capture_output=True, text=True,
    )
    return result.stdout.strip() if result.returncode == 0 else None


def source(path: str) -> str:
    return subprocess.run(
        ["git", "show", "HEAD:" + path],
        cwd=ROOT, capture_output=True, text=True, check=True, encoding="utf-8",
    ).stdout


class FeaturePluginSourceIntegrationTests(unittest.TestCase):
    def assertIn(self, item, values, msg=None):
        if isinstance(values, str) and len(values) > 2048:
            self.assertTrue(item in values, msg or f"missing source marker: {item!r}")
            return
        super().assertIn(item, values, msg)

    def test_fifteen_exact_official_source_blobs(self):
        self.assertEqual(len(STAGED), 15)
        for path, (_, official) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(blob("HEAD", path), official)

    def test_fifteen_sources_have_no_original_fork_customization(self):
        for path, (original, _) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(blob(PRODUCTION, path), original or None)

    def test_guardian_v2_feature_flags_and_schemas_staged(self):
        features = source("codex-rs/features/src/feature_configs.rs")
        root = source("codex-rs/features/src/lib.rs")
        self.assertIn("deserialize_guardian_v2_feature", features)
        self.assertIn("plugin_recommendations_enabled", root)

    def test_feature_regressions_preserve_unknown_configuration_rejection(self):
        tests = source("codex-rs/features/src/tests.rs")
        self.assertIn("guardian_v2_feature_config_rejects_invalid_settings", tests)
        self.assertIn("tool_registry_config_is_not_a_feature_toggle", tests)

    def test_plugin_executor_hook_routing_retained(self):
        hooks = source("codex-rs/core-plugins/src/executor_hooks.rs")
        self.assertIn("executor_plugin_hook_sources", hooks)
        self.assertIn("resolve_mcp_routing", hooks)

    def test_plugin_loader_scoped_policy_and_refresh_preserved(self):
        loader = source("codex-rs/core-plugins/src/loader.rs")
        for marker in ("load_plugins_from_layer_stack_with_scope",
                       "refresh_curated_plugin_cache",
                       "plugin_is_eligible_for_target_marketplace"):
            self.assertIn(marker, loader)

    def test_plugin_manager_recommendation_cache_hooks_staged(self):
        manager = source("codex-rs/core-plugins/src/manager.rs")
        for marker in ("featured_plugin_ids_cache_key",
                       "recommended_plugins_cache_key"):
            self.assertIn(marker, manager)

    def test_managed_marketplace_policy_still_restricts_sources(self):
        policy = source("codex-rs/core-plugins/src/marketplace_policy.rs")
        self.assertIn("validate_marketplace_source_for_add", policy)
        self.assertIn("policy_filtered_plugin_config", policy)
        self.assertIn("is_restricted", policy)

    def test_plugin_native_regression_sources_staged(self):
        for path in (
            "codex-rs/core-plugins/src/manager_tests.rs",
            "codex-rs/core-plugins/src/loader_tests.rs",
            "codex-rs/core-plugins/src/marketplace_policy_tests.rs",
        ):
            self.assertIn("fn ", source(path))

    def test_codexdd_adaptive_dispatch_policy_unchanged(self):
        self.assertEqual(
            blob("HEAD", "codex-rs/core/src/tools/registry.rs"),
            "fc75b3bbf91f1881bd32faaaca93356fbb598b5e",
        )

    def test_versions_and_original_lockfile_remain_unadvanced(self):
        self.assertEqual(source("codex-rs/codexdd-version.txt").strip(), "0.4.4")
        self.assertEqual(source("codex-rs/upstream-codex-release.txt").strip(), "rust-v0.162.0")
        self.assertEqual(blob("HEAD", "codex-rs/Cargo.lock"),
                         "a8eb8a076276cb3db5674b33c67d743b1c3d21fc")

    def test_source_checkpoint_does_not_release(self):
        note = (ROOT / "docs/codexdd-0.4.5-packet-3c8b-features-plugins.md").read_text(encoding="utf-8")
        for marker in ("INCOMPLETE / NOT A RELEASE CANDIDATE",
                       "source-only", "native Rust", "Cargo.lock"):
            self.assertIn(marker, note)


if __name__ == "__main__":
    unittest.main()
