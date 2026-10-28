"""Audit actual two-runner 3F.28 Windows logs, without recompiling Rust.

This gate audits source-bound historical job receipts and explicitly preserves
their original CI conclusions. A verifier false-negative is not a new green
Windows build; native execution and source provenance are distinct.
"""
from __future__ import annotations

import json
import os
import re
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DATA = Path(os.environ.get("CODEXDD_3F31_EVIDENCE", "/__missing__"))
RUN_ID = 40739170655
SOURCE_SHA = "49bc278dfe31408f05c7bde949b6857f196d0984"
SHA_PATH = "7e3457c3218c6933b8b30de65061ccf56c951619862e7d4f38ed9f2be086908e"
JOBS = {"seed": 122995762452, "restore": 122998612039}
TARGET = "//codex-rs/utils/elapsed:elapsed"
CACHE_BYTES = 8_492_218


def git(*args: str) -> str:
    return subprocess.check_output(["git", *args], cwd=ROOT, text=True).strip()


def log(name: str) -> str:
    data = (DATA / f"{name}.log").read_text(encoding="utf-8")
    if len(data) < 2000 or len(data) > 3_000_000:
        raise AssertionError(f"Invalid {name} diagnostic log length: {len(data)}")
    return data


def metadata(name: str) -> dict:
    return json.loads((DATA / f"{name}.json").read_text(encoding="utf-8"))


def bazel_summary(contents: str) -> str:
    matches = re.findall(r"INFO: 130 processes:\s*([^\r\n]+)", contents)
    if len(matches) != 1:
        raise AssertionError(f"Expected exactly one Bazel process summary, got {matches}")
    return matches[0].strip()


class RealRustCacheEvidence(unittest.TestCase):
    def test_historical_job_identity_and_only_verifier_failure(self):
        run = metadata("run")
        self.assertEqual(run["id"], RUN_ID)
        self.assertEqual(run["head_sha"], SOURCE_SHA)
        self.assertEqual(run["status"], "completed")
        self.assertEqual(run["conclusion"], "failure")
        seed = metadata("seed")
        restore = metadata("restore")
        self.assertEqual(seed["id"], JOBS["seed"])
        self.assertEqual(seed["run_id"], RUN_ID)
        self.assertEqual(seed["conclusion"], "success")
        self.assertEqual(restore["id"], JOBS["restore"])
        self.assertEqual(restore["run_id"], RUN_ID)
        self.assertEqual(restore["conclusion"], "failure")
        failed = [step["name"] for step in restore["steps"] if step.get("conclusion") == "failure"]
        self.assertEqual(
            failed,
            ["Rebuild same optimized Rust library from independent runner and prove cache reuse"],
        )
        self.assertIn(
            "No explicit optimized Rust library disk cache-hit evidence",
            log("restore"),
        )
        self.assertNotIn("BOUNDED_OPT_RUST_BUILD_FAILED", log("restore").split("BAZEL_ACTION_PATH_SHA256=")[-1].split("INFO: 130 processes:")[1])

    def test_same_source_path_and_real_release_profile_flags(self):
        for name in JOBS:
            with self.subTest(job=name):
                contents = log(name)
                path_hashes = re.findall(r"BAZEL_ACTION_PATH_SHA256=([a-f0-9]{64})", contents)
                self.assertEqual(path_hashes, [SHA_PATH])
                self.assertIn(f"Target {TARGET} up-to-date:", contents)
                self.assertIn("Build completed successfully, 130 total actions", contents)
                self.assertIn("SUBCOMMAND: # rust_library rule target " + TARGET, contents)
                self.assertIn("--crate-name=codex_utils_elapsed", contents)
                self.assertIn("--codegen=opt-level=3", contents)
                self.assertIn("-Clto=thin", contents)
                self.assertIn("-Ccodegen-units=4", contents)
                self.assertIn(f"RUST_OPT_CACHE_SIZE_BYTES={CACHE_BYTES}", contents)
                self.assertLess(CACHE_BYTES, 512 * 1024 * 1024)

    def test_all_five_native_compiler_actions_reused(self):
        self.assertEqual(bazel_summary(log("seed")), "125 internal, 5 local.")
        self.assertEqual(bazel_summary(log("restore")), "5 disk cache hit, 125 internal.")
        self.assertIn("Successfully saved cache", log("seed"))
        self.assertIn("Cache restored successfully", log("restore"))
        self.assertNotIn("local", bazel_summary(log("restore")))

    def test_corrected_verifier_reads_combined_log_without_recompilation(self):
        path = ".github/workflows/codexdd-045-3f28-optimized-rust-cache.yml"
        content = (ROOT / path).read_text(encoding="utf-8")
        self.assertIn(
            "$hits = @(Select-String -LiteralPath $combined -Pattern", content
        )
        self.assertNotIn(
            "$hits = @(Select-String -LiteralPath $log -Pattern", content
        )
        self.assertIn("Unexpected local action rebuild on consumer", content)
        self.assertIn("BOUNDED_OPT_RUST_BUILD_TIMEOUT", content)
        self.assertIn("536870912", content)
        for path, pinned in (
            ("codex-rs/utils/elapsed/BUILD.bazel", "dcc7b206d205e376306ad0d8291cf0c98ae3633b"),
            ("codex-rs/utils/elapsed/Cargo.toml", "7ba5e6ae6614fddadaa1ba1e1f8299ffc251d78f"),
            ("codex-rs/utils/elapsed/src/lib.rs", "209657e8c416a814a87d77886aeb592eda734959"),
            ("codex-rs/Cargo.lock", "13bc15ff844e93dfaf7777c73bb7313b35bb3b81"),
            ("codex-rs/deny.toml", "f282c7c55d05201e308e60804f85754c8522d462"),
        ):
            self.assertEqual(git("rev-parse", "HEAD:" + path), pinned)
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")
        self.assertEqual((ROOT / "codex-rs/upstream-codex-release.txt").read_text().strip(), "rust-v0.162.0")


if __name__ == "__main__":
    unittest.main()
