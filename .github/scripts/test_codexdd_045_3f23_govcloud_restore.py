"""Validate only the seven exact upstream 0.163 GovCloud startup restorations."""

import json
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SOURCE_PATH = "codex-rs/tui/src/lib.rs"
OLD_BLOB = "7b0746fe66b55537d3c445bfac194cbdb005b2b2"
NEW_BLOB = "f45b01a668998d817abec91313f425f2fa6a9ad7"
UPSTREAM_BLOB = "a3fb5447b06d48d3a2924eb25dae4174243cb844"
LEDGER = ROOT / "docs/upstream-candidates/codexdd-045-reviewed-semantic-corrections.json"
EDITS = [
  [
    "use crate::legacy_core::config::bootstrap_auth_config;\n",
    "use crate::legacy_core::config::bootstrap_auth_config;\nuse crate::legacy_core::config::edit::ConfigEdit;\nuse crate::legacy_core::config::edit::ConfigEditsBuilder;\n"
  ],
  [
    "use crate::legacy_core::config::resolve_profile_v2_config_path;\n",
    "use crate::legacy_core::config::resolve_profile_v2_config_path;\nuse crate::onboarding::check_gov_cloud;\n"
  ],
  [
    "use crate::onboarding::onboarding_screen::OnboardingScreenArgs;\n",
    "use crate::onboarding::onboarding_screen::OnboardingScreenArgs;\nuse crate::onboarding::onboarding_screen::run_gov_cloud_guidance;\n"
  ],
  [
    "    let startup_prefetch_started_at = Instant::now();\n",
    "    // Connected servers resolve their own provider; the local config may differ.\n    let check_gov_cloud_guidance = config.notices.hide_gov_cloud_guidance != Some(true)\n        && (config.model_provider.is_amazon_bedrock()\n            || !matches!(app_server_target, AppServerTarget::Embedded));\n    let guidance_request_handle = app_server.request_handle();\n    let startup_prefetch_started_at = Instant::now();\n"
  ],
  [
    "                load_startup_hooks_review_entry(hooks_request_handle, hooks_cwd),\n",
    "                load_startup_hooks_review_entry(hooks_request_handle, hooks_cwd),\n                async {\n                    check_gov_cloud_guidance && check_gov_cloud(guidance_request_handle).await\n                },\n"
  ],
  [
    "    let (startup_bootstrap, startup_hooks_entry) = match startup_prefetch {\n",
    "    let (startup_bootstrap, startup_hooks_entry, show_gov_cloud_guidance) = match startup_prefetch {\n"
  ],
  [
    "    let startup_elapsed_before_app = startup_prefetch_started_at.elapsed();\n",
    "    let startup_elapsed_before_app = startup_prefetch_started_at.elapsed();\n    if show_gov_cloud_guidance {\n        if let Err(err) = run_gov_cloud_guidance(&mut tui).await {\n            shutdown_startup_session(Some(app_server), &mut terminal_restore_guard).await;\n            return Err(err);\n        }\n        if let Err(err) = ConfigEditsBuilder::for_config(&config)\n            .with_edits([ConfigEdit::SetPath {\n                segments: vec![\"notice\".into(), \"hide_gov_cloud_guidance\".into()],\n                value: toml_edit::value(true),\n            }])\n            .apply()\n            .await\n        {\n            tracing::warn!(%err, \"Failed to save GovCloud guidance acknowledgment\");\n        }\n    }\n"
  ]
]


def oid(data: bytes) -> str:
    return subprocess.check_output(
        ["git", "hash-object", "--stdin"], cwd=ROOT, input=data
    ).decode("ascii").strip()


class ExactUpstreamGovCloudRestoration(unittest.TestCase):
    def test_exact_seven_edits_only_and_provenance(self):
        records = json.loads(LEDGER.read_text(encoding="utf-8"))
        self.assertEqual(records["contract_version"], 1)
        self.assertEqual(len(records["paths"]), 3)
        receipt = next(x for x in records["paths"] if x["path"] == SOURCE_PATH)
        self.assertEqual(receipt["before_blob"], OLD_BLOB)
        self.assertEqual(receipt["after_blob"], NEW_BLOB)
        self.assertEqual(receipt["official_163_blob"], UPSTREAM_BLOB)
        self.assertEqual(receipt["release_status"], "NOT_ACCEPTED")
        source = (ROOT / SOURCE_PATH).read_bytes()
        self.assertEqual(oid(source), NEW_BLOB)
        head = subprocess.check_output(
            ["git", "rev-parse", "HEAD:" + SOURCE_PATH], cwd=ROOT
        ).decode("ascii").strip()
        self.assertEqual(head, NEW_BLOB)
        # Reconstruct the whole prior fork blob with exact inverse changes.
        # Any other code change, including adaptive model/daemon routing,
        # would break this Git-object identity proof.
        reconstructed = source
        self.assertEqual(len(EDITS), 7)
        for prior, official in reversed(EDITS):
            old = prior.encode("utf-8")
            new = official.encode("utf-8")
            self.assertEqual(reconstructed.count(new), 1)
            reconstructed = reconstructed.replace(new, old, 1)
        self.assertEqual(oid(reconstructed), OLD_BLOB)

    def test_protected_provider_detection_and_acknowledgment(self):
        source = (ROOT / SOURCE_PATH).read_text(encoding="utf-8")
        self.assertIn("config.notices.hide_gov_cloud_guidance != Some(true)", source)
        self.assertIn("config.model_provider.is_amazon_bedrock()", source)
        self.assertIn("!matches!(app_server_target, AppServerTarget::Embedded)", source)
        self.assertIn("check_gov_cloud(guidance_request_handle).await", source)
        self.assertIn("run_gov_cloud_guidance(&mut tui).await", source)
        self.assertIn('"hide_gov_cloud_guidance".into()', source)
        self.assertIn("and_then(crate::chatwidget::adaptive_effort::AdaptiveFamily::parse)", source)
        self.assertEqual(
            (ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4"
        )


if __name__ == "__main__":
    unittest.main()
