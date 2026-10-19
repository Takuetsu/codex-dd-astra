"""CodexDD 0.4.5 3C.7a: protected tool-plan source guard (not native Rust)."""
import subprocess
import unittest
from pathlib import Path
ROOT = Path(__file__).resolve().parents[2]
PROD = "b25550dcb2c6e84a84074b603086c9db6f32a63b"
SPEC = "codex-rs/core/src/tools/spec_plan.rs"
TESTS = "codex-rs/core/src/tools/spec_plan_tests.rs"
PINS = {SPEC: ("a7e9353da4e397bc49df84521793cc06ae3341de","422870dba8b5a3b36994a598380c93f58760ad1b"),
        TESTS: ("d9abdf0072136154432d2fbd080ddf3b2f1672be","379eb8d44dfa672b70ed3b7d5408e882d8e803e3")}
FORK_TESTS = ("codexdd_validation_runner_is_visible_only_during_mechanical_validation",
              "codexdd_protected_turns_force_direct_tools_without_widening_other_turns",
              "adaptive_signal_tool_is_registered_with_runtime_bound_authority")
def blob(ref,path):
    return subprocess.run(["git","rev-parse",ref+":"+path],cwd=ROOT,
                          check=True,capture_output=True,text=True).stdout.strip()
def src(path):
    return subprocess.run(["git","show","HEAD:"+path],cwd=ROOT,check=True,
                          capture_output=True,text=True,encoding="utf-8").stdout
class ToolPlanTests(unittest.TestCase):
    def test_both_merges_pinned_and_original_fork_intact(self):
        for path,(old,new) in PINS.items():
            with self.subTest(path=path):
                self.assertEqual(blob(PROD,path),old)
                self.assertEqual(blob("HEAD",path),new)
    def test_adaptive_signal_exposed_direct(self):
        self.assertTrue("registry.add_with_exposure(AdaptiveSignalHandler, ToolExposure::DirectModelOnly);" in src(SPEC))
    def test_mechanical_validation_gate_not_unconditional(self):
        s=src(SPEC); i=s.index("registry.add_with_exposure(CodexDDValidationHandler")
        self.assertTrue(0<=i-s.rfind("if turn_trigger.as_deref() ==",0,i)<300)
    def test_registration_is_unique(self):
        s=src(SPEC)
        for handler in ("AdaptiveSignalHandler","CodexDDValidationHandler"):
            self.assertEqual(s.count("registry.add_with_exposure("+handler),1)
    def test_three_native_fork_regressions_preserved_once(self):
        s=src(TESTS)
        for test in FORK_TESTS:
            with self.subTest(test=test):
                self.assertEqual(s.count("async fn "+test+"()"),1)
    def test_protected_turn_triggers_preserved(self):
        s=src(TESTS)
        for trigger in ("RECONNAISSANCE","IMPLEMENTATION","MECHANICAL_VALIDATION","VALIDATION_TERMINALIZATION"):
            self.assertTrue("CODEXDD_ADAPTIVE_"+trigger+"_TURN_TRIGGER" in s,trigger)
    def test_native_gate_checks_ordinary_and_mechanical(self):
        s=src(TESTS)
        self.assertTrue('ordinary.assert_visible_lacks(&["run_codexdd_validation"])' in s)
        self.assertTrue('mechanical.assert_visible_contains(&["run_codexdd_validation"])' in s)
    def test_upstream_reviewer_regression_still_exists(self):
        self.assertTrue("async fn reviewer_tool_policy_exclude_optional_core_tools()" in src(TESTS))
    def test_registry_policy_release_and_lockfile_not_advanced(self):
        self.assertEqual(blob("HEAD","codex-rs/core/src/tools/registry.rs"),"fc75b3bbf91f1881bd32faaaca93356fbb598b5e")
        self.assertEqual(src("codex-rs/codexdd-version.txt").strip(),"0.4.4")
        self.assertEqual(src("codex-rs/upstream-codex-release.txt").strip(),"rust-v0.162.0")
        self.assertEqual(blob("HEAD","codex-rs/Cargo.lock"),"0c913b46674d3593ee84f1fee5508d16e33efae5")
    def test_packet_is_not_release_authority(self):
        s=(ROOT/"docs/codexdd-0.4.5-packet-3c7a-tool-plan.md").read_text(encoding="utf-8")
        for label in ("INCOMPLETE / NOT A RELEASE CANDIDATE","native Rust","Cargo.lock","source-only"):
            self.assertTrue(label in s,label)
if __name__=="__main__":
    unittest.main()
