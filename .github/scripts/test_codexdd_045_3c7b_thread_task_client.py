"""CodexDD 0.4.5 3C.7b: thread/task/client source provenance (offline only)."""
import subprocess
import unittest
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
PROD = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
STAGED = {
    "codex-rs/core/src/thread_manager/shutdown.rs": [
        "11f89bf39a895596343f97a443f47aa1b24c7241",
        "cfa4af1ca62228b07171d92374ab5a8ba9c10bd8"
    ],
    "codex-rs/core/src/thread_spawn.rs": [
        None,
        "29dc6c44ab21547bb2c9713aaf10effa83eb014a"
    ],
    "codex-rs/core/src/thread_rollout_truncation_tests.rs": [
        "cf034dff7959bf5ae392fe61c5792673404c04da",
        "a6ee1d1296f15698243a54538308f4b43d7c0de5"
    ],
    "codex-rs/core/src/tasks/mod.rs": [
        "bfc9c13415be9fb6c25f19910bbdf4969e24483b",
        "5612045b974ba3354540da28139ab0cf909e329a"
    ],
    "codex-rs/core/src/tasks/mod_tests.rs": [
        "86a0dbf56153f54dc2c62efa6ba2250bf10f8f90",
        "beeda10229c0cfee3eac4e0a811b88d3a47cdfb3"
    ],
    "codex-rs/core/src/tasks/review.rs": [
        "243592221971fdd818bec096874213644199aa1b",
        "7d9bf760a8a65cd4e20a5cad147700638a382c69"
    ],
    "codex-rs/core/src/client_common.rs": [
        "7a870f415150220f672610c802eb8fdd127c8db3",
        "a19431c5de830ca7a06c79e0a7c40a1f9dd79561"
    ],
    "codex-rs/core/src/client_common_tests.rs": [
        "5af77bec75b38afc7c656089509d9888f933ab08",
        "e4f431ad45460c63e317c545e3203b5e5ef09bef"
    ]
}
def blob(ref,path):
    r=subprocess.run(["git","rev-parse","--verify",ref+":"+path],cwd=ROOT,
                     capture_output=True,text=True)
    return r.stdout.strip() if r.returncode==0 else None
def src(path):
    return subprocess.run(["git","show","HEAD:"+path],cwd=ROOT,check=True,
                          capture_output=True,text=True,encoding="utf-8").stdout
class ThreadTaskClientTests(unittest.TestCase):
    def test_all_eight_exact_upstream_blobs(self):
        self.assertEqual(len(STAGED),8)
        for path,(_,official) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(blob("HEAD",path),official)
    def test_no_fork_source_overwritten(self):
        for path,(old,_) in STAGED.items():
            with self.subTest(path=path):
                self.assertEqual(blob(PROD,path),old)
    def test_thread_spawn_new_module(self):
        path="codex-rs/core/src/thread_spawn.rs"
        self.assertIsNone(blob(PROD,path))
        self.assertIn("ThreadSpawnRequest",src(path))
        self.assertIn("SpawnConfigOptions",src(path))
    def test_shutdown_retains_agent_tree_cleanup(self):
        s=src("codex-rs/core/src/thread_manager/shutdown.rs")
        self.assertIn("request_agent_tree_shutdown",s)
        self.assertIn("try_evict_v2_thread",s)
    def test_task_abort_and_process_cleanup(self):
        s=src("codex-rs/core/src/tasks/mod.rs")
        self.assertIn("abort_turn_if_active",s)
        self.assertIn("close_unified_exec_processes",s)
    def test_review_task_and_rollback_test_sources_staged(self):
        self.assertIn("process_review_events",src("codex-rs/core/src/tasks/review.rs"))
        self.assertIn("truncates_rollout_after_terminal_canonical_turn_id",
                      src("codex-rs/core/src/thread_rollout_truncation_tests.rs"))
    def test_client_request_and_native_serialization_regression(self):
        self.assertIn("normalize_image_details",src("codex-rs/core/src/client_common.rs"))
        self.assertIn("serializes_flex_service_tier_when_set",src("codex-rs/core/src/client_common_tests.rs"))
    def test_existing_adaptive_workflow_retained(self):
        s=src("codex-rs/core/src/agent/control/spawn.rs")
        self.assertEqual(s.count("RolloutItem::WorkflowState(_)"),2)
        self.assertEqual(blob("HEAD","codex-rs/core/src/tools/registry.rs"),"fc75b3bbf91f1881bd32faaaca93356fbb598b5e")
    def test_release_versions_and_lockfile_unadvanced(self):
        self.assertEqual(src("codex-rs/codexdd-version.txt").strip(),"0.4.4")
        self.assertEqual(src("codex-rs/upstream-codex-release.txt").strip(),"rust-v0.162.0")
        self.assertEqual(blob("HEAD","codex-rs/Cargo.lock"),"a6cd27b0b70fde2f2be9c1777f3666ad3fa318bb")
    def test_packet_source_only(self):
        s=(ROOT/"docs/codexdd-0.4.5-packet-3c7b-thread-task-client.md").read_text(encoding="utf-8")
        for label in ("INCOMPLETE / NOT A RELEASE CANDIDATE","native Rust","Cargo.lock","source-only"):
            self.assertIn(label,s)
if __name__=="__main__":
    unittest.main()
