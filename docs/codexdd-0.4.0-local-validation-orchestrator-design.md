# CodexDD 0.4.0 — Local Validation Orchestrator design

## Packet 3A.1 — reconnaissance and design

**Status:** COMPLETE

**Release line:** CodexDD 0.4.0

**Feature branch:** `dd/codexdd-v0.4.0-local-validation-orchestrator`

**Frozen production baseline:** `06465383abb1acf7c74c0a84b00726853b74fdc4` (CodexDD 0.3.14)

**Target platform:** Windows

This packet is reconnaissance/design only. It intentionally makes no runtime/source behavior change and does not bump the product version.

## Scope

Define the first implementable architecture for the Local Validation Orchestrator without creating a parallel adaptive state machine or weakening existing safety/governance boundaries.

Owned by this packet:

- map the current Implementation -> mechanical-validation -> Implementation/terminal lifecycle;
- map the existing protected mechanical-validation command gate;
- map native command/tool evidence ingestion;
- choose the initial validation-profile contract;
- choose where deterministic validation execution should attach to the runtime;
- define the bounded repair policy;
- define the durable result/evidence contract;
- split implementation into bounded follow-on packets.

Deferred:

- runtime/source implementation;
- product version bump;
- upstream Codex refresh;
- Windows release packaging/tagged-release automation;
- CI promotion, merge, production install, and release publication.

## Reconnaissance findings

### Existing lifecycle is reusable

The current adaptive runtime already has the lifecycle transitions needed by the orchestrator:

- `mechanical_validation` transitions a bound Implementation Worker from source-changing Implementation into protected mechanical validation;
- `implementation_work` transitions back to source-changing Implementation when validation proves that a source/config/test change is required;
- `ready_for_validation` remains the hard Implementation/Repair handoff terminal;
- an independent Validation Worker remains a separate role/context and is not silently merged into the implementation Worker.

The Local Validation Orchestrator should extend this lifecycle rather than introduce a second state machine.

### Existing mechanical-validation gate is the correct safety boundary

Protected mechanical-validation turns already restrict tool dispatch.

The current gate:

- allows read-only reconnaissance commands;
- allows bounded validation commands such as Cargo test/check/build/clippy/metadata and `cargo fmt -- --check`;
- allows bounded `powershell` / `pwsh -File <script>.ps1` execution;
- rejects command chaining, redirection, dynamic shell expressions, source-edit-capable tools, and unsupported commands;
- requires a trusted `implementation_work` transition before source edits resume.

0.4.0 must preserve this boundary. The orchestrator must not become a generic command bypass.

### Native evidence already exists

Completed command/tool items already register thread-bound adaptive evidence with:

- evidence ID;
- thread ID;
- source turn ID;
- success/failure/cancelled/incomplete outcome;
- evidence kind.

The orchestrator should reuse native tool completion/evidence instead of treating model prose as proof.

### Current operator-mediated validation policy is intentionally still production truth

`docs/codexdd-major-phase-work-packet-standard.md` currently says pre-PR local validation is operator-mediated on Daniel-CL.

That remains correct for production 0.3.14.

Do **not** rewrite that policy as if automation already exists. Update the policy only after the 0.4.0 runtime is implemented and proven on Windows.

## Architecture decision

### 1. Attach the orchestrator to mechanical validation

The first implementation targets a bound **Implementation** Worker after it enters `MechanicalValidation`.

Do not initially broaden the feature into Validation-role terminalization or unrelated general shell automation.

Initial lifecycle:

1. Implementation completes source-changing work.
2. Worker reports trusted `mechanical_validation`.
3. CodexDD enters protected mechanical validation.
4. CodexDD selects and runs a deterministic repo-owned validation profile.
5. PASS produces a durable validation receipt and permits `ready_for_validation`.
6. FAIL produces a durable failure receipt.
7. If the failure is in scope and repair budget remains, the Worker reports `implementation_work`, repairs, then returns to mechanical validation.
8. If the failure persists beyond the bounded repair budget or is out of scope, CodexDD stops and reports a concise blocked result.

### 2. Use repo-owned validation profiles

Target repositories opt into deterministic validation through conventional Windows scripts:

```text
scripts/codexdd-test-targeted.ps1
scripts/codexdd-test-workpacket.ps1
scripts/codexdd-test-release.ps1
```

The scripts are owned by the repository being validated, not generated ad hoc by the model during a validation turn.

Profile semantics:

- **targeted** — smallest deterministic checks for a focused component/failure;
- **work-packet** — normal post-implementation gate; targeted checks plus the repository's required format/lint/build/regression checks for a completed work packet;
- **release** — broad pre-CI/pre-install local regression gate.

Initial selection rule:

- ordinary Implementation mechanical validation defaults to **work-packet**;
- **targeted** is used for focused retest/diagnosis after a failed work-packet stage;
- **release** is never silently selected by an ordinary Implementation Worker and remains an explicit release/Phase-4 action.

This keeps profile selection deterministic and prevents normal work packets from unexpectedly launching release-scale validation.

### 3. Add a bounded validation runner tool, not hidden direct process spawning

The preferred runtime surface is a CodexDD built-in control tool with a narrow schema, conceptually:

```text
run_codexdd_validation(profile = targeted | work_packet | release)
```

The tool should:

- resolve the selected conventional script from the active workspace/repository root;
- fail closed when the required profile script is missing or ambiguous;
- execute through the existing Codex execution/sandbox/approval infrastructure;
- preserve normal cancellation, interruption, environment, and tool lifecycle behavior;
- produce a native completed tool/command item so the existing evidence registry remains authoritative;
- never accept arbitrary shell text from the model.

Do not implement this by spawning an unrestricted process directly from TUI code. Reuse the existing execution/orchestration layer so permission and sandbox semantics remain intact.

### 4. Mechanical-validation tool gate remains authoritative

During an orchestrated mechanical-validation turn:

- `run_codexdd_validation` is allowed;
- `report_adaptive_signal` remains allowed for lifecycle/terminal signaling;
- required read-only inspection/status helpers remain allowed;
- arbitrary source editing remains blocked;
- raw `exec_command` remains constrained by the existing mechanical-validation allowlist.

The new tool must not widen the raw shell allowlist beyond what is necessary for the profile contract.

### 5. Validation receipts are runtime state, not prose

A successful profile run should create a structured receipt associated with the current Worker/thread and candidate state.

Minimum receipt fields:

- profile;
- repository/workspace identity;
- branch;
- HEAD SHA;
- run ID;
- start/end time;
- overall PASS/FAIL;
- stage summaries;
- native evidence ID(s);
- log location;
- failed stage when applicable;
- repair-attempt count/failure fingerprint when applicable.

A PASS receipt should be current for the candidate HEAD. A source-changing repair invalidates the prior PASS receipt.

Implementation `ready_for_validation` should only be accepted once the current candidate has a successful required validation receipt. This is stronger than relying on a final answer that merely says tests passed.

### 6. Compact result; full logs out of model context

Routine validation output returned to the model should be structured and compact.

PASS example:

```text
CodexDD validation: PASS
profile: work-packet
branch: <branch>
sha: <sha>
run: <run-id>

targeted:   PASS
format:     PASS
lint:       PASS
build:      PASS
regression: PASS
git-tree:   EXPECTED/CLEAN

log: <runtime-log-path>
READY_FOR_VALIDATION
```

FAIL example should include only:

- profile;
- run ID;
- failed stage;
- command/script stage name;
- concise error excerpt;
- native evidence ID;
- log path;
- repair attempts used/remaining;
- in-scope vs blocked classification.

Do not inject full compiler/test logs into model context by default.

### 7. Keep logs outside the source worktree

Validation logs must not dirty the repository being validated.

Use CodexDD runtime storage under the user's Codex home, conceptually:

```text
<CODEX_HOME>/codexdd/validation/<repo-id>/<run-id>/
```

The implementation packet must define bounded retention/cleanup so repeated local validation does not create unbounded disk growth.

### 8. Bounded repair policy

Initial automatic repair budget: **2 source-changing repair cycles per failure fingerprint/work packet**.

A repair cycle is consumed when:

1. an orchestrated validation run fails;
2. the failure is classified as in-scope;
3. the Worker transitions back through trusted `implementation_work`;
4. source-changing repair occurs;
5. the Worker re-enters mechanical validation.

Rules:

- after a repair, rerun the smallest affected targeted profile/stage first;
- after targeted PASS, rerun the required work-packet profile;
- a successful work-packet run clears the active failure fingerprint;
- the same persistent failure after the repair budget is exhausted must stop rather than loop indefinitely;
- out-of-scope, permission, environment, missing-profile, or infrastructure failures do not authorize opportunistic source edits.

The repair counter/fingerprint must survive the same resume/fork lifecycle boundaries as other adaptive state that controls execution.

### 9. Preserve consequential human gates

0.4.0 automation must not silently perform:

- GitHub CI promotion where the project uses an owner gate;
- squash/merge;
- production installation;
- release publication;
- destructive Git operations;
- deployment or changes to Doug's machine or other external machines;
- other production-impacting actions.

The feature removes Daniel as the routine shell-command typist. It does not remove Daniel as the owner of consequential release/workflow decisions.

## Initial source ownership map

Expected implementation surfaces, subject to packet-level reconfirmation:

- `codex-rs/core/src/tools/handlers/`
  - add the bounded validation runner handler/spec/tests;
- `codex-rs/core/src/tools/registry.rs`
  - mechanical-validation tool-gate integration;
- `codex-rs/tui/src/chatwidget/adaptive_effort.rs`
  - persisted orchestrator/repair/receipt state;
- `codex-rs/tui/src/chatwidget/adaptive_admission.rs`
  - mechanical-validation continuation contract;
- `codex-rs/tui/src/chatwidget/adaptive_trusted_signal.rs`
  - receipt-aware lifecycle/terminal acceptance;
- `codex-rs/tui/src/chatwidget/adaptive_evidence.rs` and/or `codex-rs/tui/src/adaptive_evidence.rs`
  - receipt/evidence binding where needed;
- status/persistence/replay files only where required by the durable state contract;
- repo-owned PowerShell profile scripts for CodexDD self-hosting/dogfood validation;
- documentation/tests for the profile contract.

This is an ownership hypothesis, not authorization to edit all listed files in one packet.

## Work-packet plan

### 3A.1 — Reconnaissance/design

**Status:** COMPLETE with this checkpoint.

Deliverables:

- current lifecycle/tool/evidence map;
- profile contract;
- runner architecture;
- repair policy;
- result/logging contract;
- bounded implementation packet split.

### 3B.1 — Validation profile contract + self-hosting scripts

**Status:** COMPLETE and Windows-validated on Daniel-CL.

Validation evidence:

- contract outcome-class smoke passed with exit code 0;
- targeted profile passed with exit code 0;
- Codex TUI adaptive slice reported 174/174 tests passed;
- native stderr is correctly treated as output evidence rather than an infrastructure exception.

Scope:

- define exact script invocation/exit-code/output contract;
- add CodexDD's own targeted/work-packet profile scripts for dogfooding;
- add script-level tests/fixtures where practical;
- no adaptive lifecycle integration yet.

Exit:

- conventional scripts are deterministic, bounded, and usable manually on Windows;
- scripts do not dirty the source worktree except for expected build/test artifacts;
- checkpoint commit.

### 3B.2 — Bounded validation runner tool

**Status:** COMPLETE and targeted-Rust-validated on Daniel-CL.

Validation evidence:

- the two new runner Rust files passed targeted rustfmt after correction;
- `cargo nextest run --no-fail-fast -p codex-core --lib codexdd_validation` passed 7/7 tests with exit code 0.

Current implementation decisions:

- model-visible input is only the fixed enum `targeted | work_packet | release`;
- no arbitrary shell text is accepted;
- the selected conventional PowerShell profile is resolved from the nearest valid repository root;
- execution delegates through the existing unified exec command/sandbox path with a profile-specific hard timeout;
- the validation process writes its complete output to a temporary UTF-8 log outside the source worktree;
- core persists that log under `<CODEX_HOME>/codexdd/validation/<repo-id>/<run-id>/validation.log`;
- retention is bounded to the 20 newest runs per repository;
- only parsed contract events are returned as a compact structured tool result;
- malformed/missing terminal contract output fails closed;
- source/tool registration into the adaptive mechanical-validation phase remains deferred to 3C.1.

Scope:

- implement the narrow profile enum/tool;
- resolve profile scripts from the active repo/workspace;
- run through existing execution/sandbox/approval plumbing;
- capture compact structured output and full external logs;
- fail closed for missing/ambiguous/invalid profiles.

Exit:

- runner cannot accept arbitrary shell;
- native success/failure evidence is produced;
- targeted unit tests green;
- checkpoint commit.

### 3C.1 — Mechanical-validation lifecycle integration

**Status:** COMPLETE and Windows-validated on Daniel-CL.

Validation evidence:

- targeted core validation-runner tests passed;
- mechanical-validation core regression slice passed;
- full Codex TUI adaptive regression slice passed 178/178 with exit code 0;
- receipt-gate fallout in legacy tests was corrected to model the new required work-packet receipt rather than weakening the gate.

Current implementation:

- `run_codexdd_validation` is registered only when the trusted turn trigger is mechanical validation;
- the protected mechanical-validation dispatch gate explicitly allows the bounded runner while retaining the existing source-edit restrictions;
- the mechanical-validation continuation instructs the Worker to run the repo-owned `work_packet` profile instead of manually recreating validation commands;
- the bounded runner emits its own native default-namespace dynamic-tool receipt using the original tool-call ID plus the fixed profile argument; the internal exec uses a separate call ID, so arbitrary shell output cannot masquerade as the receipt;
- Implementation `ready_for_validation` now requires a successful `work_packet` receipt from the same mechanical-validation turn;
- targeted-only, failed, cross-thread, or stale prior-turn receipts cannot satisfy the handoff gate;
- Repair-worker handoff behavior is intentionally unchanged in this packet.

Scope:

- expose/authorize the runner only in the intended adaptive phase;
- default ordinary Implementation mechanical validation to work-packet;
- create/invalidate current validation receipts;
- require a current PASS receipt before Implementation `ready_for_validation`.

Exit:

- no source-edit bypass;
- stale PASS cannot authorize a changed HEAD;
- normal successful implementation reaches `READY_FOR_VALIDATION` without owner-entered shell commands;
- checkpoint commit.

### 3C.2 — Bounded diagnose/repair/retest loop

**Status:** COMPLETE and Windows-validated on Daniel-CL.

Validation evidence:

- CodexDD validation-runner slice passed with exit code 0;
- adaptive workflow history/persistence slice passed with exit code 0;
- persisted validation-repair-state TUI slice passed with exit code 0;
- full Codex TUI adaptive regression slice passed 185/185 with exit code 0;
- pre-handoff compile oracles also proved codex-core, codex-history, and codex-tui library-test compilation on the Windows target.

Current implementation:

- repairable validation failures carry a stage-scoped fingerprint in the native LVO receipt;
- `implementation_work` now requires a same-turn failed `targeted` or `work_packet` LVO receipt rather than model prose;
- at most two source-changing repair cycles are admitted for the active work packet;
- a third repair request with valid failure evidence deterministically latches `Blocked`;
- after any admitted repair, the next mechanical-validation turn must pass `targeted` before `work_packet`;
- `ready_for_validation` after repair requires both same-turn successful targeted and work-packet receipts;
- a successful handoff clears the repair fingerprint, cycle count, and targeted-retest requirement;
- failure fingerprint, repair-cycle count, and targeted-retest requirement are durable workflow state and survive resume/fork;
- persisted repair state is validated fail-closed and cannot restore with an over-budget or internally inconsistent combination;
- infrastructure/contract failures do not produce a repairable fingerprint and therefore cannot authorize `implementation_work`.

Scope:

- persist failure fingerprint and repair budget;
- authorize at most two automatic source-changing repair cycles for the same failure class;
- targeted retest before work-packet revalidation;
- deterministic stop when budget is exhausted.

Exit:

- persistent failure cannot run away;
- resume/fork does not reset the repair budget incorrectly;
- checkpoint commit.

### 3D.1 — Status, summaries, and operator evidence

**Status:** IN PROGRESS.

Implementation direction:

- extend the existing `/status` Adaptive Effort block; do not add a separate status command;
- record the latest native LVO profile/result/run/branch/HEAD/log/failed-stage metadata from the validation-runner receipt;
- persist that compact operator status across resume/fork without persisting the ephemeral evidence registry;
- show repair cycles used/remaining and the deterministic next validation step;
- enrich the compact validation-runner result with run/branch/HEAD identity so PASS/FAIL handoff evidence is useful without full logs.

Scope:

- surface current validation profile/run/result/repair budget in existing `/status` surfaces where useful;
- produce compact PASS/FAIL handoff evidence;
- avoid creating a separate status command.

Exit:

- operator can identify what ran, against which SHA, and what remains;
- checkpoint commit.

### 3E.1 — Version/provenance/docs policy transition

Scope:

- bump product to 0.4.0 at the correct release stage;
- update current Worker/Designer guidance;
- update the major-phase local-validation policy from operator-mediated to orchestrator-owned only after runtime proof exists;
- keep consequential human gates explicit.

Exit:

- installed/source identity and docs agree;
- checkpoint commit.

### 3F.1 — Full implementation audit

Scope:

- audit all CodexDD-owned changed paths;
- verify no hidden shell bypass, stale receipt path, repair-budget reset, or release-gate regression;
- verify upstream refresh was not silently mixed into the feature.

Exit:

- implementation ownership complete;
- ready for Phase 4.

### Phase 4 — Robust Windows pre-install validation

Use numbered Phase-4 packets.

At minimum prove on Daniel-CL:

- targeted runner behavior;
- normal work-packet autonomous validation;
- PASS receipt bound to branch/SHA;
- source change invalidates stale PASS;
- failed validation -> bounded repair -> targeted retest -> work-packet retest;
- persistent failure stops after the repair budget;
- missing/malformed profile fails closed;
- interruption/resume/fork preserves state correctly;
- full adaptive/TUI regressions;
- release profile/dogfood;
- release-shaped Windows build/startup smoke in the actual operator context;
- explicit elevated shared-daemon creation remains blocked while automatic elevated startup safely falls back as established in 0.3.14.

Only after local Phase 4 is green should the normal PR/CI gate begin.

## Post-LVO enhancement priority

After the Local Validation Orchestrator is complete and proven, the next CodexDD enhancement is **GPT-6.1 Sol workflow integration**.

Required order:

1. complete and prove the Local Validation Orchestrator;
2. integrate GPT-6.1 Sol into the adaptive model/effort workflow;
3. only then proceed to the automatic upstream-update enhancement.

The GPT-6.1 Sol packet must begin with its own bounded reconnaissance/design pass covering model catalog identity, adaptive routing ladders, complexity/failure/quality/budget routes, status surfaces, persistence/resume behavior, and regression coverage. Do not mix that model-workflow integration into the active LVO implementation packets.

## Upstream boundary

Do not combine a Codex upstream refresh with packets 3B.1-3F.1.

If 0.4.0 adopts a newer stable upstream, create a separate numbered work packet with its own reconnaissance, compatibility review, local validation, CI, and soak evidence.

## 3A.1 exit state

- 0.4.0 feature branch exists from the exact frozen production baseline.
- No runtime/source behavior has changed.
- The initial Local Validation Orchestrator architecture is defined.
- Implementation can begin with **3B.1 — Validation profile contract + self-hosting scripts**.
