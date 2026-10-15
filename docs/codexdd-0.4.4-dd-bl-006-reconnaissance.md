# CodexDD 0.4.4 — DD-BL-006 cross-repository LVO blocker

**Status:** ACTIVE — packets 1A–1D source changes checkpointed on isolated branches; Windows tests and integration proof PENDING. Not releasable, merged, or installed.

**Source of priority:** `codex-dd — Upcoming Version Backlog` (current post-0.4.3 queue dated 2026-10-11), DD-BL-006. The observed defect affected the Windows CodexDD 0.4.2 Implementation Worker in the TypeScript/Node BREAKWATER worktree `C:\bw55`. The 0.4.3 upstream refresh is now merged separately as PR #52.

## Baseline

- Production base: `dd/astra-policy-v2` @ `d8ad52cc223362a03f4dc5b40ec51695b3c48936` (merged 0.4.3).
- Active fix branch: `dd/codexdd-0.4.4-dd-bl-006`.
- Upstream remains `rust-v0.162.0`. The 0.4.4 product-version bump belongs to the eventual approved release candidate, not to reconnaissance.
- Platform: Windows; authoritative cheap local validation on Daniel-CL over SSH, then final GitHub CI after the local gate. No production installation/merge is implied.

## OBSERVED → REPRO → EXPECTED → OWNING PATH

1. **Observed:** protected Mechanical Validation demands `scripts/codexdd-test-workpacket.ps1` beneath a directory with `justfile` and `codex-rs/Cargo.toml`; a valid Node repository fails before native testing. Follow-on shell commands and retries cannot legitimately create a PASS receipt.
2. **Reproduction target:** the standalone `discover_repository_root` test on a mock Git worktree with `package.json`, and a nested Git repository beneath a parent with a different configured profile.
3. **Expected:** resolve the nearest active Git worktree and _only_ its explicit repo-owned contract-v1 profile; if missing, give a clear nonrepairable configuration blocker, no cross-repo execution, no PASS/repair evidence and no model-family escalation. Preserve worktree edits, valid identity-bound receipts, trusted terminals, and mechanical no-write guard.
4. **Owning paths:** `codex-rs/core/src/tools/handlers/codexdd_validation.rs` and tests, mechanical command policy in `codex-rs/core/src/tools/registry.rs`, and TUI admission/continuation paths including `codex-rs/tui/src/chatwidget/adaptive_admission.rs`.

## Packet 1A — implemented, not yet locally validated

- Removed Rust-only root markers from the runner's profile resolution.
- Anchored profile discovery to the nearest `.git` directory or worktree-marker file; never fall back to a different repository's script.
- Kept profile names, script invocation, runtime log handling, strict schema and native SHA/run/evidence handling unchanged.
- Require the selected `scripts/codexdd-test-*.ps1` to be explicit and canonically contained in the active Git repository. A missing profile fails with remediation text rather than being treated as a product/test failure.
- Added regression fixtures for Rust, explicitly configured Node/TypeScript, missing Node profile in a nested repository, no Git root, and a Unix symlink escape.
- GitHub source checkpoints: `fe2a4876239d57a79d4ba3e2aa42c185f367a486`, `3f07a4806e3156dd0ff3968db2e91806e0f1c2e5`. **These have NOT run on Daniel-CL.**

## Follow-up packets (required before claiming DD-BL-006 fixed)

- **1B — mechanical safe-command policy (IMPLEMENTED; UNVALIDATED):** diagnose and narrowly allow safe Node/TypeScript read-only inspection and approved project validation commands without permitting mutating arbitrary `npm run` scripts, bypassing the repo's heavy-validation semaphore, or allowing direct execution to impersonate a native LVO PASS.
- **1C — fail-closed workflow handoff (IMPLEMENTED; UNVALIDATED):** ensure absent/unregistered profile reports one actionable configuration blocker instead of repeated automatic continuation or capability pressure; ensure source edits and uncommitted work are preserved, no synthetic receipt, no premature `ready_for_validation`.
- **1D — explicit Node repository contract (IMPLEMENTED ON ISOLATED BREAKWATER TOOLING BRANCH; UNVALIDATED):** define or integrate a fixed, reviewed `work_packet` (plus `targeted` for repairs) contract-v1 script for BREAKWATER or a generic supported registration mechanism. BREAKWATER's existing `package.json` validation scripts do **not** by themselves produce LVO contract-v1 receipts.
- **1E — test and release gates:** inexpensive source-local checks on Daniel-CL first; then Windows behavioral proof including positive Rust, configured Node, unconfigured Node, safe git inspection, staged/uncommitted worktree preservation, deterministic refusal with no escalation, receipt evidence; final CI only after local pass. Owner-governed merge and install remain separate.

**Current outcome:** Source changes exist on two isolated feature/tooling branches. The live BREAKWATER worker remains on its original branch and therefore does not yet have the opt-in scripts; no unblock, source merge, receipt PASS or release decision is claimed.

## Source checkpoint — October 12, 2026 (validation not run)

CodexDD integration: `dd/codexdd-0.4.4-dd-bl-006` (product `0.4.4`, upstream retained at `rust-v0.162.0`). Changes:

- Runner now discovers the nearest active Git worktree (`.git` file or directory), rejects path escapes and missing profiles, and emits a structured **non-PASS** result for a missing/invalid repo-owned profile or missing execution log. It preserves native SHA/run-bound evidence, log capture and repair fingerprints only for real test failures.
- Mechanical command admission permits bounded, named BREAKWATER-style npm diagnostics (typecheck/lint/format checks/build/test/focused/maps) plus read-only Git inspection, with arbitrary scripts/flags/unguarded raw commands rejected. Shell diagnostics **do not** count as a native LVO PASS.
- The current-turn, built-in failed `run_codexdd_validation` event without a repairable stage fingerprint deterministically blocks the Implementation Worker's Mechanical Validation phase, suppressing unfinished-turn retries, capability pressure and false `ready_for_validation` terminalization. Actual failed test stages retain their bounded repair path.
- Regression tests added for Node/Rust resolution and repository isolation, safe/unsafe npm commands, deterministic BLOCKED handling, capability-pressure override, and same-turn repairable/other-turn isolation.
- Candidate product identity is `0.4.4`; no CI, binary or installation evidence yet.

BREAKWATER opt-in on **separate untouched-gameplay branch**:

- Repository: `Takuetsu/breakwater`
- Branch: `codex/tooling/codexdd-044-node-lvo-contract`
- Original owner-accepted evidence-tooling base: `fc20cb9c0576668b2853a09032b3d5086e5b423d`
- Native profile entrypoints: `scripts/codexdd-test-targeted.ps1`, `scripts/codexdd-test-workpacket.ps1`, `scripts/codexdd-test-release.ps1`.
- Contract-v1 event emitter: `scripts/codexdd-validation-common.ps1`. Fixed stage names/commands, no external model text; `git diff --check` and staged diff check remain read-only; guarded npm commands retain BREAKWATER's existing focused/heavy semaphore.
- `targeted`: diff/staged-diff/focused. `work_packet`: diff/staged-diff/changed-format/focused/lint/Vitest/build. `release` (explicit only): diff/staged-diff/official full-regression.
- This tooling-only branch is NOT merged into Z-A0.55E, canonical main or any active worker checkout; it does not change gameplay code or waive the independent milestone gate.

### Next required validation gate (do not dispatch broad GitHub CI first)

1. On Daniel-CL, fetch **only** the CodexDD feature SHA into a separate worktree, verify identity, run source formatting and narrow `codex-core` and `codex-tui` regression filters first. Use existing warmed Rust target if safe. No 21-stage release profile during ordinary packet validation.
2. On Windows, parse all four BREAKWATER PowerShell files with the built-in parser (syntax-only); check profile failure/contract-only cases without running heavy npm. Do not alter active Z-A0.55E lineage or its dirty worktree.
3. Only after narrow source checks pass, exercise the configured Node contract in a disposable BREAKWATER checkout; verify PASS, product FAIL, profile-error BLOCKED, no fake native PASS, no cross-repo fallback and preserved staged/uncommitted files.
4. Reinspect source and semantic diff, then run final targeted GitHub CI, owner-governed squash merge and separate install/soak gates. No owner's release decision has been inferred from branch creation.

## Local evidence checkpoint — October 12 (Daniel-CL)

- CodexDD feature SHA `e7d1271a637f2e7fa9528ff57b2a846030e1adac` passed the owner's **targeted Windows regression gate** (cargo fmt --check, core LVO tests, mechanical gate, TUI error/terminal/evidence filters). This does **not** constitute the 21-stage release suite or final Windows/Node end-to-end gate.
- The BREAKWATER Node contract smoke initially failed because its own test dereferenced `.status` on a `profile_begin` event in PowerShell StrictMode. The production contract was not implicated by this exception. Smoke code corrected at `3eedd91b111e4aa4049adce59764d4d94fee4463`.
- Daniel-CL then verified corrected exact BREAKWATER SHA `3eedd91b...`, which emitted **`CODEXDD NODE LVO CONTRACT SMOKE PASSED (syntax + fail-closed; npm not run)`**. It validated all registered scripts syntactically, negative profile ERROR=2, and no false PASS. This was a **mock missing-profile** test, not a real `npm` run.

## Pre-Node hardening — latest GitHub-only change (requires narrow Rust retest)

While reviewing the native runner after the successful Node smoke, a separate unsafe case was found: the LVO log parser accepted a lone `profile_end: pass` even if the declared validation stages were absent. This contradicted DD-BL-006's no-false-PASS requirement.

- The parser now requires exactly one profile_begin, sequenced and matching stage_begin/stage_end pairs, consistent stage ordinal and result, and for PASS exactly the declared number of successful stages, completed_stages, exit_code 0 and stage native exit code 0. Duplicate/malformed terminal events or unexecuted stages cause an ERROR, not PASS.
- Stage FAIL still carries its deterministic failed_stage/failure_fingerprint for bounded source repair. Runtime/contract ERROR remains nonrepairable and terminalizes BLOCKED.
- Additional regression fixtures include Rust + Node event streams, incomplete fake PASS, zero-stage fake PASS, failed stage followed by fake PASS, stage-name mismatch and duplicate terminal events.
- This parser change is **not yet locally validated**. Before broad CI, re-run the cheap `cargo fmt --all -- --check` and `cargo test --locked -p codex-core --lib codexdd_validation` on the latest exact CodexDD feature commit, then (when dependencies are available) run only BREAKWATER `targeted` to exercise the real Node/TypeScript contract. Avoid `work_packet`, `release` and 21-stage suites at this step.

## Real BREAKWATER targeted proof and independent exit guard — October 13, 2026

**Owner-observed Daniel-CL PASS:** On the isolated `D:\bw55\.worktrees\codexdd-044-node-contract` tooling worktree at `3eedd91b111e4aa4049adce59764d4d94fee4463`, the Windows `targeted` LVO entrypoint executed the actual BREAKWATER semaphore-backed `npm run validation:focused` flow. Git diff, staged diff and TypeScript typecheck all exited zero. Native contract-v1 events reported `stage_count=3`, sequential stage PASS and `profile_end status=pass, exit_code=0, completed_stages=3`. Wall clock in the profile: 7.19 seconds; TypeScript focused check: 6.27 seconds. **This is not proof that CodexDD's tool itself emitted a bound completion receipt**; the check called the repo-owned profile directly.

In the same owner command group, CodexDD Rust format and `codexdd_validation` unit-test filters completed without stopping. Those tests apply to earlier candidate `11f604bd7bf6abdad8c98650d534553f80856219`; additional SHA-bound changes after that require new CI.

**Independent native child-exit safeguard:** A subsequent source audit identified a second fake-PASS vector: the outer runner previously validated contract events without comparing the subprocess's true PowerShell exit code to the terminal declaration. The wrapper now records the child's exit code in a separate per-run temporary sidecar, outside source; any missing or mismatched exit code downgrades to a typed nonrepairable `error` and no PASS or repair fingerprint, even if all JSON events claim success. Malformed UTF-8 output is also parsed fail-closed rather than throwing away the native result. Dedicated unit tests added for match, mismatch and absent exit file. **This newest candidate has not yet passed its GitHub CI.**

**Independent BREAKWATER CI diagnostic:** existing push CI on the isolated Node tooling branch reached maps/typecheck/lint, then failed or was superseded during full Vitest. A completed prior run at `777e63d177137c0f570f293ff73d7d773d619a4f` logged 11 gameplay-test timeouts/failures among 1,210 tests; no gameplay source was changed on the tooling branch relative to `fc20cb9c0576668b2853a09032b3d5086e5b423d`. Do not treat unrelated historical gameplay-test flakes as this Node contract's PASS, change gameplay, or waive the actual independent milestone gate.

### Release boundary remains open

- Latest code must pass Windows CI with the new exit-status unit coverage, plus actual native CodexDD `run_codexdd_validation` receipt identity checks.
- `targeted` Node contract PASS is verified; the standalone `work_packet` profile and native end-to-end handoff have not yet passed on an exact immutable candidate.
- Keep BREAKWATER's scripts on the isolated tooling branch until project-level authorization to integrate. Do not merge into active 0.55E gameplay lineage or main without explicit gate approval.
- No release/installer signoff or 21/21 native Windows full profile is asserted.

## October 14, 2026 — protected native tool-mode isolation

**State: narrow source candidate staged, no live native `work_packet` receipt.** PR #53 remains draft at `a79378618728a5ad63fbf7b204ae9a0ea1fecb29` with its prior blocking CI green. The later bound BREAKWATER Worker did enter trusted Mechanical Validation, but reported a pre-execution tool rejection, then stopped BLOCKED after three no-native-progress turns. The screenshot did not include the original tool invocation/error, so the attribution to the Implementation read-only gate is not independently verified. There were **zero** completed native profile stages, no run ID, no log receipt and no gameplay edit. Do not count the attempt as an LVO FAIL/PASS.

On the isolated CodexDD branch `dd/codexdd-0.4.4-native-tool-direct-guidance`:

- The Mechanical Validation continuation explicitly requires the direct, top-level built-in `run_codexdd_validation`, never a Code Mode nested call or ad-hoc shell substitute. An earlier TUI assertion was corrected to match the new wording.
- `effective_tool_mode` now selects **Direct** only when the per-turn, trusted `CODEXDD_ADAPTIVE_MECHANICAL_VALIDATION_TURN_TRIGGER` is present. That prevents Code Mode from concealing/rerouting the native validation control tool in precisely the protected turn. Ordinary, reconnaissance and ValidationTerminalization turns retain their configured Code Mode settings; `registry.rs` retains its strict per-turn no-source-edit dispatch allowlist.
- `spec_plan_tests.rs` now checks direct Mechanical Validation tool visibility and absence of Code Mode exec, together with isolation and nonexposure on other turns.
- The existing release profile stage `cli-release-build` now builds **both** `codex-cli` and `codex-code-mode-host` using locked Rust release artifacts, without adding or dropping any of the 21 release stages. Its isolated PowerShell static gate passed.
- Owner's Daniel-CL targeted TUI tests on earlier guidance commit `96758db02a411f435ad26bf8c0e3df3676305a92`: **2 passed, 0 failed** (`mechanical_validation_prompt` normal and repaired paths). The earlier filtered core CodeModeOnly exposure test did not stop the owner's scripted sequence, but the updated Direct-mode test is a new revision and **has not yet been executed on Daniel-CL**.
- [Isolated Windows source, parser, formatting and boundary run 38650799192](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38650799192): **PASS**. Source code was clean and the release profile remained exactly 21 stages. This is not a compiled unit test or a native Worker profile. Its temporary workflow has been removed from the isolated branch.

**Next acceptance (smallest first):** On Daniel-CL, refresh the isolated candidate at its exact current SHA and reuse the warmed Rust target; run **only** `cargo test --locked -p codex-core --lib codexdd_mechanical_turn_forces_direct_tools_without_widening_other_turns` first. If green, build only `codex-cli` as a debug test executable, verify its `0.4.4` commit identity, then conduct one fresh, header-bound, zero-edit native Implementation Worker session on the isolated BREAKWATER Node tooling SHA. Require a real `run_codexdd_validation(profile=work_packet)` event, repository SHA/run ID, all seven stage events, independent child exit, persisted log and clean worktree. Capture the **raw** rejection/tool identity if any failure recurs. Only after that receipt is verified should the source be promoted into PR #53, exact-head CI re-run, detached 21-stage Windows release gate executed and owner approval for merge/install considered. No active BREAKWATER gameplay worktree or production installation should be changed.


## October 15, 2026 — first-reconnaissance no-signal deadlock

**New owner-observed local result:** A fresh `0.4.4+g7e5cf6eefe97` CLI started in the exact isolated BREAKWATER Node tooling worktree with the mandatory `[adaptive_worker]` Implementation header. The first Luna Low assistant turn claimed the complexity reconnaissance was reported, then ended. `/status` immediately afterward showed **Worker binding: Bound**, **Complexity: None**, **Implementation phase: Reconnaissance**, **Attempt: 1**, **Unfinished pressure: 0/2**, **Workflow terminal: None**, **LVO last: not run**, **LVO run/log: None**. This proves no accepted trusted complexity report and no automatic handoff. No Node validation or gameplay edit was performed.

**Confirmed source deadlock:** `chatwidget/protocol.rs::hold_completed_turn_for_adaptive_signal` previously set `AdaptivePendingSignal::Awaiting` on *any* completed first Implementation reconnaissance turn with `complexity_class=None`, even without a successful native `report_adaptive_signal` tool-output receipt. With no native tool event to transport, it could wait forever, suppressing both the terminal unfinished-turn reducer and automatic successor admission. **Final-answer prose is never a trusted lifecycle signal.**

**Isolated product correction:** Remove the unconditional `awaiting_initial_complexity` exception: a completed turn is held only if its own function output proves `report_adaptive_signal` actually emitted a trusted request. Otherwise it enters the existing bounded unfinished-turn reducer and must never linger at Attempt 1 with zero pressure. On the controller-authorized initial `CODEXDD_ADAPTIVE_RECONNAISSANCE_TURN_TRIGGER` only, select direct tool mode so the trusted `report_adaptive_signal` is not competing with Luna's indirect Code Mode executor. The prior protected Mechanical Validation direct-tool selection remains intact. Both phase-specific registry read-only/no-source-edit allowlists remain unchanged. Ordinary, Implementation and terminalization turns retain normal model-selected modes.

**New narrow regressions:** core `codexdd_protected_turns_force_direct_tools_without_widening_other_turns` now verifies direct top-level `report_adaptive_signal` exposure during reconnaissance without granting `run_codexdd_validation`, and unchanged Mechanical Validation direct mode / other-turn isolation. TUI `first_reconnaissance_without_trusted_tool_receipt_does_not_wait_forever` simulates a completed bound first turn with no trusted tool output and requires automatic unfinished pressure/Attempt 2 instead of `Awaiting`. The broader native `work_packet` gate and detached 21-stage Windows release suite remain explicitly held until the new candidate is proven locally.

**Next gate:** narrow GitHub source/format check, then **only** the two new exact-SHA filtered Rust tests on Daniel-CL using the warmed Cargo target. If green, rebuild only the debug `codex` binary and run one new isolated Worker acceptance test (not a manual continuation of the old Awaiting session). Capture real native run ID, profile output/child exit/log, seven stage receipts, repository SHA, source integrity; never invent a PASS from model prose.

## October 15, 2026 — Validation Worker terminal is not native LVO acceptance

**Owner evidence:** The latest isolated BREAKWATER Node acceptance screenshot shows a controller-generated `ValidationTerminalization` continuation with successful `exec-...` evidence references and a subsequent `READY_FOR_OWNER_QA` for an independent **Validation** review on clean SHA `691fdfecaff8501ec8edc69617774843ae48a3bd`. It does **not** display any `run_codexdd_validation` call, native LVO run ID, stored LVO profile log, or typed native profile receipt. Earlier `/status` from the independent Validation review explicitly reported `LVO last: not run`, `LVO run: None`, and `LVO log: None`; do not transfer that historical status to a different new session without checking.

**Code-level explanation, not a new runner defect:** `chatwidget/adaptive_trusted_signal.rs` intentionally accepts `ready_for_owner_qa` for a Worker **bound as Validation** when its provided evidence references resolve to successful generic turn evidence; `adaptive_evidence.rs` includes successful command-execution `exec-...` records in that category. It is a legitimate independent-review terminal, **not a native Mechanical Validation or Implementation acceptance event**. Separately, the Implementation-only `ready_for_validation` path enforces `implementation_phase=MechanicalValidation` and requires a typed `DynamicToolCall` record for **`run_codexdd_validation` profile `work-packet`** in the same authorized turn; generic shell `exec-` evidence cannot satisfy it. This source boundary is already present in PR #53 and remains intact.

**Operational preflight for the remaining gate:** A future acceptance run must use the isolated compiled `0.4.4+gaf631af03ebe` binary, not the PATH-installed `codexdd 0.4.3`, and must begin a **fresh** session with a literal first-message `[adaptive_worker]` header with `role = "implementation"` and an authorized scope. Before accepting any expensive WorkPacket results, verify its in-process `/status` shows **Worker role: Implementation**, **Worker binding: Bound**, and the expected `0.4.4` candidate. If role is Validation, Unspecified or Locked unbound, STOP and do not run repository scripts. The prior Validation Worker terminal does not transition or rebind into Implementation; no manual `Continue` can change that.

**Acceptance evidence to persist:** one direct top-level native `run_codexdd_validation(profile=work_packet)` call; typed completed control-tool receipt with matching repository SHA, profile, run ID, PASS, full seven stage results and real child exit 0; persisted out-of-tree log; clean source; subsequent accepted `ready_for_validation` transition. Do not substitute agent prose, GitHub CI, model-written scripts, or `exec-` evidence refs for this receipt. The already-green 1,210 Node tests and independent owner-QA review should not be rerun simply to manufacture native acceptance. Full 21-stage Windows release, promotion to PR #53, merge and install remain held.

## October 15, 2026 — bound Implementation mechanical handoff remains inaccessible

**New objective acceptance evidence (owner screenshots at 12:39 CDT):** This was a correctly bound **Implementation** Worker under the tested isolated `0.4.4+gaf631af03ebe` CLI, with an accepted `Complexity: Standard` and the required BREAKWATER tooling checkout. After the model tried to report `mechanical_validation` on several automatically authorized turns, the `run_codexdd_validation` tool never appeared. Controller STOP was `Workflow terminal: BLOCKED`, `Attempt: 4`, **`Implementation phase: None`**, and `LVO last: not run; LVO run/log: None`. The model's text that it "reported" `mechanical_validation` is not a trusted native receipt. No Node profile ran in this attempt and nothing in these screenshots establishes a source/test failure in BREAKWATER.

**Source gap confirmed on prior candidate:** `report_adaptive_signal` is `ToolExposure::DirectModelOnly`, but `core/src/tools/mod.rs::effective_tool_mode` previously forced Direct only for the trusted **Reconnaissance** and **Mechanical Validation** turn triggers. The intervening complexity-accepted **Implementation** successor could remain Luna CodeModeOnly, where the direct-only tool is absent from the model catalog. That makes the required `kind=mechanical_validation` handoff inaccessible even while the Worker is bound, and the three-no-native-progress watchdog eventually blocks. Independently, the status's `Complexity=Standard / implementation_phase=None` is inconsistent with normal `apply_adaptive_complexity_floor` (which sets phase Implementation). The exact timing of this state loss is not proven from the screenshot; a narrow recovery path must still require a new trusted tool signal and cannot silently grant Mechanical Validation authority.

**Staged isolated correction, not yet promoted to draft PR #53:**
- A new private protocol turn marker `CODEXDD_ADAPTIVE_IMPLEMENTATION_TURN_TRIGGER` is derived by the TUI only for an enabled, nonterminal, nonpaused, **bound Implementation Worker** with nonempty scope and already-accepted complexity, in Implementation or missing-phase recovery state. Ordinary/Validation/Repair Worker turns cannot acquire it.
- Core uses that trusted marker to select **Direct** rather than CodeModeOnly, exposing the direct-only `report_adaptive_signal` for the legitimate Implementation handoff. The native `run_codexdd_validation` handler remains **unregistered and unexposed** on this marker; it still requires the separate trusted Mechanical Validation trigger.
- `consume_adaptive_signal_at_terminal` can recover a `None` Implementation phase **only** after an explicit, accepted native `kind=mechanical_validation` signal from the bound role with nonempty scope and previously accepted complexity. Missing complexity, wrong Worker role, absent scope and generic model prose never authorize this transition. Mechanical Validation's existing no-edit gate is unchanged.
- New focused tests assert worker/scope/phase gating, distinct TUI per-turn trigger priorities, CodeModeOnly model direct-tool exposure only where authorized, early validator absence, accepted missing-phase recovery and fail-closed variants.

**Required checkpoints:** finish cheap Windows source/format gate, run ONLY the new filtered Rust tests (core tool plan, TUI routing/state/trusted signal) on Daniel-CL's warmed Cargo target, and only after green rebuild the isolated CLI. ONE fresh bound Implementation Worker may then attempt the native `work_packet` profile, requiring typed seven-stage PASS and run ID/exit/log/clean source. No repeated ad-hoc Node profile, full 21-stage Windows release test, PR merge, or installation while native receipt absent.
