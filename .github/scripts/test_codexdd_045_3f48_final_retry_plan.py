"""3F.48: no replay of core, no post-compile loss of actual Windows CLI/PDB."""
from pathlib import Path
import unittest
R=Path(__file__).resolve().parents[2]
W=(R/".github/workflows/codexdd-045-3f48-optimized-cli-and-combined-retry.yml").read_text()
class FinalRetry(unittest.TestCase):
 def test_source_frozen_one_shot_and_bounded(self):
  self.assertRegex(W,r"github.event.before == '(?:NOT_ARMED_FOR_045_OPTIMIZED_RELEASE|[0-9a-f]{40})'")
  self.assertEqual(W.count("runs-on: windows-2022"),1)
  self.assertIn("timeout-minutes: 358",W)
  self.assertIn("codexdd045-3f47-40884725763-core-complete",W)
  self.assertIn("fail-on-cache-miss: true",W)
  self.assertNotIn("core_seed:",W)
  self.assertNotIn("core_finish:",W)
  self.assertNotIn("workflow_dispatch:",W)
  self.assertNotIn("gh release",W)
 def test_cli_verified_and_artifact_uploaded_before_combined(self):
  keys=("-Stage cli -SoftMinutes 195","name: Preserve physical CLI/PDB if native compilation succeeded","Upload ACTUAL CLI executable and PDB immediately",
        "-Stage combined -SoftMinutes 145")
  positions=[W.index(k) for k in keys]
  self.assertEqual(positions,sorted(positions))
  self.assertIn("if: always()",W)
  self.assertIn("steps.salvage.outputs.cli_artifact_ready == 'true'",W)
  self.assertIn("id: upload_cli",W)
  self.assertIn("if: always() && steps.salvage.outputs.cli_artifact_ready == 'true'",W)
  self.assertIn("if: always() && steps.salvage.outputs.cli_artifact_ready == 'true' && steps.upload_cli.outcome == 'success'",W)
  self.assertNotIn("steps.cli.outcome == 'success'",W)
  self.assertRegex(W,r"(?s)id: upload_cli.*?if-no-files-found: error.*?id: combined")
  self.assertIn("codex.exe",W)
  self.assertIn("codex.pdb",W)
  self.assertIn("cli-integrity.json",W)
  self.assertIn("combined-sha256-inventory.json",W)
  self.assertIn("codexdd_045_verify_windows_archive.py",W)
  self.assertIn("compression-level: 0",W)
 def test_final_combined_validation_has_exact_archive_and_no_install(self):
  self.assertIn("file_count -ne 10",W)
  self.assertIn("release_approved",W)
  self.assertIn("runtime_package_combined.tar.gz",W)
  self.assertIn("nonrelease-artifacts",W)
  self.assertNotIn("create-release",W)
  self.assertNotIn("download-artifact",W)
if __name__=="__main__":unittest.main()
