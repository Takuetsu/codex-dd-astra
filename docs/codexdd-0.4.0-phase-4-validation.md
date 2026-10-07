# CodexDD 0.4.0 Phase 4 pre-install validation

Phase 4 is the robust Windows pre-install proof for the CodexDD 0.4.0 Local Validation Orchestrator after completion of 3F.1.

Target branch: `dd/codexdd-v0.4.0-local-validation-orchestrator`

Production/base: `06465383abb1acf7c74c0a84b00726853b74fdc4` (CodexDD 0.3.14)

Target upstream: OpenAI Codex `rust-v0.159.2`

Platform authority: Daniel-CL / Windows.

No production installation occurs until every Phase-4 packet is green. GitHub PR/CI is a later promotion gate.

## Packet 4.1 — profile contract and release-profile viability

**Status: COMPLETE — DANIEL-CL GREEN.**

Purpose:

- prove the three fixed LVO profiles still satisfy the contract after all implementation repairs;
- ensure the `release` profile is a meaningful green/red signal on the frozen Windows upstream rather than a wrapper around known upstream-baseline failures;
- preserve the broad pre-install coverage learned during 0.3.14 Phase 4.

Checks:

- validation contract fixture PASS/fail/error semantics;
- fixed profile names and bounded runner timeouts;
- direct shell execution of the three protected profile scripts remains blocked during mechanical validation;
- `targeted` and `work_packet` remain unchanged in authority;
- `release` is broader than `work_packet` but excludes permanently noisy upstream-baseline-only full-workspace/full-TUI gates.

Release-profile policy for 0.4.0:

- strict format and core/TUI Clippy;
- full history, rollout, protocol, state, thread-store, and app-server library suites;
- core/app-server/TUI adaptive regressions;
- TUI persisted workflow-state + detached-fork regressions;
- TUI daemon-startup regressions including elevated automatic fallback and explicit failure classification;
- locked release CLI build.

The full canonical TUI library suite is not used as an LVO PASS/FAIL stage because exact upstream `rust-v0.159.2` is already known to carry a Windows baseline failure set. Phase 4 may use differential/manual evidence when broad upstream comparison is needed, but the product-owned `release` profile itself must be deterministic on a healthy candidate.

Exit:

- contract fixtures green on Daniel-CL;
- targeted profile green;
- corrected release profile committed.

## Packet 4.2 — native receipt and candidate-authority matrix

**Status: COMPLETE — DANIEL-CL GREEN.**

Purpose: prove that native LVO evidence, not prose or shell output, controls handoff.

Required evidence:

- native `work_packet` PASS receipt;
- receipt is same-thread/same-turn and carries native run/candidate branch+SHA operator identity;
- targeted-only, failed, missing, cross-thread, and stale-turn receipts cannot authorize Implementation handoff;
- direct shell/model prose cannot manufacture validation authority;
- returning to implementation after a failed validation creates a later validation turn, so an earlier PASS cannot authorize the repaired candidate;
- source-changing tools remain blocked during protected mechanical validation.

The same-turn protected-lifecycle rule is the source-change invalidation mechanism: source edits require lifecycle re-entry and therefore a later validation turn. External operator edits during an active protected turn are outside automated authority and require a fresh validation run before promotion.

Existing regression coverage mapped during the Phase-4 audit:

- native runner summary preserves run ID, branch, and HEAD identity;
- native dynamic-tool completion becomes typed LVO evidence;
- evidence resolution rejects missing, cross-thread, stale-turn, and wrong-outcome references;
- Implementation handoff rejects missing, targeted-only, stale, or failed work-packet receipts;
- direct execution of the fixed validation scripts is blocked during protected mechanical validation;
- source-edit-capable tools remain blocked during that lifecycle.

Exit: all receipt/admission regressions green on Daniel-CL.

## Packet 4.3 — bounded failure/repair/retest adversarial matrix

**Status: COMPLETE — DANIEL-CL GREEN.**

Purpose: prove the full 3C.2 loop as one release gate.

Required evidence:

- initial repair must originate from failed `work_packet`;
- initial failed `targeted` cannot bypass the work-packet gate;
- infrastructure/contract errors do not authorize edits;
- first and second admitted repair consume the same global budget;
- changing failed stage/fingerprint does not reset the budget;
- after repair, `targeted` PASS is mandatory before `work_packet` PASS;
- third repair request hard-blocks for owner review;
- successful handoff clears repair state;
- failure selection remains deterministic with `work_packet` preferred over `targeted`.

Phase-4 audit hardening added two dedicated regressions that were previously implicit in implementation behavior:

- a failed native LVO call without a repairable fingerprint (execution/contract/infrastructure class) cannot authorize `implementation_work` or consume repair budget;
- after cycle 1, a failure with a different stage-scoped fingerprint consumes global cycle 2 instead of resetting the repair budget.

Existing regressions already cover the initial work-packet-only gate, targeted+work-packet retest requirement, deterministic work-packet failure preference, third-repair hard block, and successful repair-state clearing.

Exit: all repair-loop regressions green on Daniel-CL.

## Packet 4.4 — persistence, interruption, resume, and fork matrix

**Status: COMPLETE — DANIEL-CL GREEN.**

Purpose: prove durable LVO state survives lifecycle boundaries without restoring ephemeral authority.

Required evidence:

- validation repair fingerprint/cycle count/targeted requirement persist;
- compact validation operator status, including native failure fingerprint, persists;
- over-budget or internally inconsistent persisted state fails closed;
- repair state cannot attach to an invalid worker/lifecycle;
- resume restores mechanical-validation state correctly;
- detached fork inherits durable LVO state;
- evidence registry, pending signals, pending attempts, and successor permits do not become durable authority;
- interrupted-turn handling remains correct.

Use the established Windows test stack where needed:

`RUST_MIN_STACK=16777216`

Phase-4 audit hardening added:

- an explicit regression that partially populated repair state (for example cycle count without a fingerprint) fails closed;
- explicit resume assertions that pending attempt, pending signal, successor admission, evidence registry, last outcome, and last failure kind are not restored as authority.

The detached-fork regression already proves durable repair/operator status survives while pending attempts/signals/successor permits are absent in the fork. Existing interrupted-turn regressions remain part of the Daniel-CL matrix.

Exit: persistence/interruption matrix green on Daniel-CL.

## Daniel-CL Phase 4.1–4.4 closure evidence

The consolidated Windows gate completed successfully on Daniel-CL:

- validation contract fixture tests passed;
- release profile parsed successfully;
- targeted profile passed with the full adaptive regression slice;
- native validation-runner regressions passed;
- TUI session-state persistence regressions passed;
- detached-fork durable-state regression passed with the enlarged Windows test stack;
- app-server adaptive runtime-signal identity binding passed;
- app-server interrupted-turn behavior passed.

The Phase-4 hardening regressions for infrastructure-failure repair rejection, global cycle-2 consumption across changed fingerprints, and incomplete persisted repair state also passed the temporary Windows oracles before those workflow files were removed.

## Packet 4.5 — broad release profile and candidate executable dogfood

**Status: ACTIVE — RELEASE PROFILE REPAIR PENDING RERUN.**

First Daniel-CL release-profile attempt:

- stages 1 through 19 passed;
- stage 20, `cli-daemon-startup-tests`, ran 4 Windows tests: 3 passed and 1 failed;
- the failing test was `restrictive_launcher_uses_embedded_if_daemon_cannot_start`;
- the actual candidate rendered the intended Administrator-PowerShell fallback warning: `Running without the shared background server: an elevated Windows terminal requires embedded mode.`;
- the stale test still waited only for the non-elevated restrictive-job wording, `this Windows launcher requires embedded mode`, and timed out after 45 seconds;
- runtime behavior was therefore correct, but the release gate correctly exposed an environment-sensitive test expectation;
- the test now accepts exactly the two supported automatic fallback causes (elevated terminal or restrictive Windows launcher) and still rejects any unrelated fallback reason.

The repaired daemon-startup smoke and exact rustfmt both passed on the temporary Windows oracle. The oracle workflow was removed immediately afterward. Daniel-CL Administrator PowerShell remains the authoritative environment for the narrow rerun and final release-profile pass.

Purpose: exercise the complete repository-owned `release` profile and actual release-shaped candidate without installing it.

Required evidence:

- `scripts/codexdd-test-release.ps1` returns `status=pass`, `exit_code=0`;
- release profile records all configured stages;
- locked release build succeeds;
- built executable reports CodexDD `0.4.0` identity and current Git identity;
- upstream provenance remains `rust-v0.159.2`;
- basic non-installing CLI help/version smoke succeeds;
- source worktree remains clean.

Exit: broad local release gate green.

## Packet 4.6 — operator-context startup / daemon security smoke

Purpose: cover the exact 0.3.14 soak gap before installation.

From the release-shaped candidate, prove in the same Administrator PowerShell used for normal CodexDD operation:

- normal interactive startup succeeds;
- automatic shared-daemon creation denied because of elevation safely falls back to embedded mode;
- normal startup does not require `--no-daemon`;
- explicit elevated shared-daemon creation remains rejected;
- unrelated daemon startup failures do not get misclassified as elevation fallback;
- candidate exits cleanly and does not replace the installed production binary.

Exit: actual operator-context startup behavior is green before install.

## Phase 4 closure

Phase 4 closes only when packets 4.1 through 4.6 are green on Daniel-CL and any discovered defects have had their affected packet rerun.

After Phase 4:

1. normal PR / CI promotion gate;
2. merge only after CI is green;
3. build/install CodexDD 0.4.0 on Daniel-CL;
4. post-install smoke and soak;
5. then begin GPT-6.1 Sol workflow integration;
6. automatic upstream-update enhancement remains after GPT-6.1 Sol.
