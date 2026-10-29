"""3F.46: audit actual 3F.45B native Windows cache evidence without Rust compilation.

The source CI run is RED because its last verification expression read the
compile step's output before that step completed. Actual Bazel optimized Rust
compilation, distinct Windows runner cache transport, and cache hits succeeded.
"""
from __future__ import annotations
import json
import os
import re
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DATA = Path(os.environ.get("CODEXDD_3F46_EVIDENCE", "/__missing__"))
RUN = 40877812353
SOURCE = "f7c5836615ac3a347af20c8068048adc28a5c82e"
SEED_ID = 123461228848
RESTORE_ID = 123466929289
PATH_SHA = "7e3457c3218c6933b8b30de65061ccf56c951619862e7d4f38ed9f2be086908e"
CACHE_KEY = "codexdd045-3f45b-40877812353-seed"
BLAZEL_TARGET = "//codex-rs/config:config"


def record(name: str) -> dict:
    return json.loads((DATA / (name + ".json")).read_text(encoding="utf-8"))


def log(name: str) -> str:
    value = (DATA / (name + ".log")).read_text(encoding="utf-8")
    assert 5_000 < len(value) < 8_000_000, (name, len(value))
    return value


class MediumRustCacheEvidence(unittest.TestCase):
    def test_historical_failure_is_only_after_successful_native_build(self):
        run, seed, restore = record("run"), record("seed"), record("restore")
        self.assertEqual((run["id"], run["head_sha"], run["conclusion"]),
                         (RUN, SOURCE, "failure"))
        self.assertEqual((seed["id"], seed["run_id"], seed["conclusion"]),
                         (SEED_ID, RUN, "success"))
        self.assertEqual((restore["id"], restore["run_id"], restore["conclusion"]),
                         (RESTORE_ID, RUN, "failure"))
        self.assertEqual(
            [s["name"] for s in restore["steps"] if s.get("conclusion") == "failure"],
            ["Finish the real optimized Rust target with independently restored cache"],
        )
        text = log("restore")
        self.assertIn("Restored runner did not emit a valid cache receipt", text)
        self.assertIn("NONRELEASE_REAL_OPTIMIZED_RUST_RESTORE_TARGET_PASS", text)
        self.assertLess(text.index("NONRELEASE_REAL_OPTIMIZED_RUST_RESTORE_TARGET_PASS"),
                        text.rindex("Exception:"))

    def test_seed_checkpoint_is_real_partial_action_cache_not_build_success(self):
        text = log("seed")
        self.assertEqual(re.findall(r"BAZEL_ACTION_PATH_SHA256=([a-f0-9]{64})", text),
                         [PATH_SHA])
        self.assertIn("CHECKPOINT_DISK_CACHE_INVENTORY=BYTES:771982018; FILES:1970", text)
        self.assertIn("3F44_SEED_BUILD_COMPLETE=False", text)
        self.assertIn("NONRELEASE_PARTIAL_ACTION_CHECKPOINT_READY_NOT_TARGET_PASS", text)
        self.assertIn("Cache saved with key: " + CACHE_KEY, text)
        self.assertNotIn("NONRELEASE_REAL_OPTIMIZED_RUST_SEED_TARGET_PASS", text)

    def test_independent_runner_exact_cache_restore_and_real_1320_action_build(self):
        text = log("restore")
        self.assertEqual(re.findall(r"BAZEL_ACTION_PATH_SHA256=([a-f0-9]{64})", text),
                         [PATH_SHA])
        for phrase in (
            "Cache hit for: " + CACHE_KEY,
            "Cache restored successfully",
            "Cache restored from key: " + CACHE_KEY,
            "Target " + BLAZEL_TARGET + " up-to-date:",
            "INFO: 1320 processes: 432 disk cache hit, 530 internal, 358 local.",
            "INFO: Build completed successfully, 1320 total actions",
            "CHECKPOINT_DISK_CACHE_INVENTORY=BYTES:1897073424; FILES:3417",
            "3F44_RESTORE_BUILD_COMPLETE=True",
            "NONRELEASE_REAL_OPTIMIZED_RUST_RESTORE_TARGET_PASS",
        ):
            with self.subTest(phrase=phrase):
                self.assertIn(phrase, text)
        self.assertLess(text.index("Build completed successfully, 1320 total actions"),
                        text.rindex("Exception:"))
        self.assertLess(1_897_073_424, 3_221_225_472)

    def test_same_step_output_bug_repaired_without_new_native_run(self):
        wf = (ROOT / ".github/workflows/codexdd-045-3f45b-optimized-cache-restore-proof.yml").read_text(encoding="utf-8")
        one, two = wf.split(
            "      - name: Require finished real Rust target, not just a partial cache", 1
        )
        compile_section = one.rsplit(
            "      - name: Finish the real optimized Rust target with independently restored cache", 1
        )[-1]
        self.assertIn("id: compile", compile_section)
        self.assertNotIn("steps.compile.outputs.checkpoint_ready", compile_section)
        self.assertIn("steps.compile.outputs.checkpoint_ready", two)
        self.assertIn("steps.compile.outputs.build_complete", two)
        self.assertIn("fail-on-cache-miss: true", wf)
        full = (ROOT / ".github/workflows/codexdd-045-3f43-manual-optimized-windows.yml").read_text(encoding="utf-8")
        self.assertIn("NOT_ARMED_FOR_045_OPTIMIZED_RELEASE", full)
        self.assertIn("-Stage warm -SoftMinutes 32 -MaxCacheBytes 3221225472", full)
        self.assertIn("-Stage core -SoftMinutes 96 -MaxCacheBytes 5368709120", full)

    def test_security_and_version_markers_remain_unpromoted(self):
        pins = {
            "codex-rs/config/Cargo.toml": "b4be94bb585c6757234299a78f8b42a02b3d2017",
            "codex-rs/Cargo.lock": "13bc15ff844e93dfaf7777c73bb7313b35bb3b81",
            "codex-rs/deny.toml": "f282c7c55d05201e308e60804f85754c8522d462",
        }
        for path, expected in pins.items():
            self.assertEqual(
                subprocess.check_output(["git", "rev-parse", "HEAD:" + path],
                                        cwd=ROOT, text=True).strip(), expected
            )
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")
        self.assertEqual((ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(), "rust-v0.162.0")


if __name__ == "__main__":
    unittest.main()
