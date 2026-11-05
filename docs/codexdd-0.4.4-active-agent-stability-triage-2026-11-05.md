# CodexDD 0.4.4 active-agent stability triage — 2026-11-05

**Status: source-only reconnaissance; NO runtime changes, release approval, merge, CI or install.**
**Authority:** owner's explicit 2026-11-05 request to freeze 0.4.5 and prioritize unusable CodexDD agent workflows. Not authorization to weaken validation/security/role/independence gates or alter active game worktrees.

## Baseline and release boundary

- Production source branch: `dd/astra-policy-v2` @ `b25550dcb2c6e84a84074b603086c9db6f32a63b`, `codex-rs/codexdd-version.txt = 0.4.4`. Exact installed binary SHAs and active agent version /status must still be verified before attributing live incidents to this source.
- This triage is intentionally branched from production **0.4.4**, not from 0.4.5. Do not cherry-pick unresolved 0.4.5 TUI/daemon work into a stability hotfix.
- 0.4.5 freeze evidence: [freeze-only commit a69ce2e](https://github.com/Takuetsu/codex-dd-astra/commit/a69ce2ee19a50b834bc116246f94940bef8b8806). PR #83 = 3F.93 3/3 nonrelease pass; PR #84 = 3F.94 native supervisor 0/2 blocked; PR #85 = 3F.95 TUI test compile red. No automatic expensive 3F.95 workflow. **13 high-risk holds and 0/109 formal owner signoffs unchanged.**
- Canonical source: [codex-dd — Upcoming Version Backlog](https://docs.google.com/document/d/1vJ617lLax8GFUVWjJaNE2veCfSyjD5OOydBvywWA8iA/edit), current 2026-11-04 entries. Supporting [BREAKWATER — CodexDD Operational Guide](https://docs.google.com/document/d/1g9n1a09TmftzxuAYY_C8IQ239My4CGm-SOFg8XSOjgo/edit) and [0.4.4 DD-BL-006 source reconnaissance](../docs/codexdd-0.4.4-dd-bl-006-reconnaissance.md).
- These are **incident classes / regression candidates**; absence of current 0.4.4 reproduction does NOT authorize calling them proven current-code defects. Do not use old historical DD-BL statuses without newer evidence.

## P0-A — False-ready and premature ValidationTerminalization (DD-BL-007 / DD-BL-005)

**Observed:** 2026-11-04 Z-A0.58A STONKS diagnostic session had no listeners on the required host/frontend ports 4331/4332 and health checks refused connections. Its adaptive terminalization nevertheless cited older successful exec refs and declared green owner handoff without a live preview. In Z-A0.56E1 a fresh independent validator ended with an unproven/missing trusted `ready_for_owner_qa` or `repair_required` signal after real scoped proof and extra continuation turns; one documented attempt correctly ended BLOCKED rather than falsely green.

**Unknown:** exact installed version and trusted terminal event state were not independently proved in all reports. In some examples partial `source validated` outcome was truthful, but a claim of completed *full owner-authorized runtime gate* was not.

**Reproduce on 0.4.4:** fresh nonauthor Validation Worker, pinned SHA, valid 55/55 focused proof, explicit pending/full gate or required runtime+LAN preview absent, older successful exec refs; inspect `/status`, typed signal ledger, source SHA and real service state. Also run a genuinely complete case.

**Expected:** full-gate READY only when every **required owner-authorized prerequisite** has current evidence; partial source pass + missing operations/service readiness must remain incomplete. No stale or unrelated exec-ref promotion. If signal transport fails, preserve proof and allow one bounded safe recovery or truthful typed block; never silently fabricate a trusted terminal. Keep owner-QA separate from source verification and independent review.

**Likely owning source to inspect:** `codex-rs/tui/src/chatwidget/adaptive_admission.rs`, `codex-rs/tui/src/chatwidget/adaptive_effort.rs`, `codex-rs/core/src/tools/handlers/adaptive_signal.rs`, persisted WorkflowState and role-bound receipt handling.

## P0-B — Wrong checkout/profile and unavailable native LVO tool (DD-BL-006)

**Observed:** BREAKWATER Z-A0.57E benchmark Worker started from D:\\bw57 while requested source lived in a different clean 4035e248 worktree; it entered MechanicalValidation without running benchmarks and `run_codexdd_validation(work_packet)` reported `profile_unavailable` against an old root. The script existed at the correct authorized SHA. Z-A0.58A reported absent D:\\bw58\\scripts\\codexdd-test-workpacket.ps1. The 2026-11-04 Z-A0.56E1 Repair Worker had no native runner; **missing tool in Repair role is NOT automatically a bug**, because normal LVO `work_packet` is reserved for Implementation. Distinguish role/permission, checkout mismatch, actual profile availability and provider capabilities.

**Reproduce on 0.4.4:** two same-project Git worktrees (active parent and target nested), only target has declared contract-v1 `work_packet`; verify the tool's selected worktree ID, binding, permission/role, CWD and exact source SHA *before* protected phase admission. Include missing script, wrong launch root, unbound first assignment, dirty/staged checkpoint and supported Node/Rust cases.

**Expected:** never validate an unrelated worktree or treat a missing script as source-test FAIL/model incapability. Preflight target identity and profile/tool availability before Mechanical Validation. If absent, single actionable `SETUP_BLOCKED` with observed target/candidate/required registration, preserving staged/uncommitted files. Do not fabricate native PASS, bypass no-write gates or rerun a heavy benchmark as LVO.

**Likely owning source:** `codex-rs/core/src/tools/handlers/codexdd_validation.rs`, `codex-rs/core/src/tools/registry.rs`, TUI role/binding and MechanicalValidation admission logic.

## P0-C — Repeated no-progress adaptive continuation (DD-BL-007)

**Observed:** Z-A0.56C PROVIDER_REJECTED forensic task reached BLOCKED on unchanged evidence across additional ~16s, ~7s and ~5s continuations; no new proof, diagnosis, authorized tool action or concrete recovery. Other recent workers used multiple short classification/preflight turns, repeated unavailable-tool attempts and model escalations despite infrastructure/config blocks.

**Reproduce:** start one bounded task with missing native profile, unsupported tool or unknown provider subtype; assert no new evidence/tool capability after first blocked turn. Include a source-changing task where real progress does occur as a positive control.

**Expected:** do not spend further model turns or increase capability/failure pressure for identical unchanged infrastructure/permission blocks. Exactly one actionable next step or a typed BLOCKED with sufficient path/role/source detail; preserve all previous evidence, pending work and user edits. Do not suppress legitimate multi-turn productive work.

**Likely owning source:** adaptive continuation scheduling and pressure classification, WorkflowState persistence, tool refusal categories, evidence-delta comparison.

## P1-D — Worker spawn failures under concurrency

**Project screenshot evidence:** BREAKWATER-DOUG requested 8 workers but reported `SPAWNED=0, FAILED=8` and `collab spawn failed: agent thread limit reached`. A separate eight-worker SHA-256 probe likewise reported zero successes. The screenshots do not give the exact installed CodexDD commit, configured worker capacity, whether stale threads occupy seats, or whether 8 simultaneous workers is intentionally unsupported.

**Reproduce:** capture `codexdd --version`, `/status`, max-agents config and actual live/closed thread count; 1 worker, configured allowed parallelism, and one above-limit case on Daniel-CL or Doug's disposable test profile. Distinguish capacity exhaustion, leaked/stale seats, and a zero-capacity configuration. Expected in-capacity workers spawn; over-limit gives actionable, bounded refusal without false worker success or parallel overload. Do NOT raise global limits blindly.

## P1-E — Provider API compatibility / first-turn role binding

**Observed:** BREAKWATER Z-A0.56C rustponsesapi returned HTTP 400 `unsupported_parameter: tool_choice is not supported` in a terminalization attempt; the provider version/request shape is not yet proven. Another Designer assignment began with a Markdown heading instead of the required first-nonblank `[adaptive_worker]`, so the role binding parser correctly returned none; the exact incident's bound role is unverified.

**Expected:** capability-aware provider request construction/fail-closed diagnostic without degrading security, and clear first-assignment role-binding preflight without quietly trusting late header edits. Preserve original model routing/sandbox. No source bug claim until request+runtime config is captured.

## P1-F — Operations/benchmark task lane (product gap, not a minimal hotfix)

Source 0.4.4 supports Implementation/Validation/Repair role values, not first-class Operations/Research/Benchmark. Z-A0.57E needed a safe 2-vs-4-thread benchmark; forcing Mechanical Validation was incorrect for the requested task. Separate this **larger feature design** from the emergency behavioral P0 hotfix. Short-term protection should be accurate early task-kind/setup blockage, not uncontrolled operations permissions, new service privileges or an invented native LVO PASS.

## Current status / next validation plan

1. **Do not modify active BREAKWATER/CV/Doug project checkouts.** Confirm installed binary identity (not only source branch) when a short live smoke is authorized; use project records and nonsecret artifacts.
2. Start with **P0-A and P0-B**, since false owner readiness and wrong-source validation create trust/provenance risk, then **P0-C** no-progress admission. Instrument read-only diagnostic tests and add narrow, failing 0.4.4 regression tests prior to any repair.
3. Independently determine whether P1-D worker seat exhaustion is the current day-to-day hard blocker; if reproduced within configured limits, it moves ahead of lower-impact P0 work.
4. Implement only minimal 0.4.4-safe corrections, with exact source/checkpoint and no feature/upstream dependency merges. Reuse safe existing 0.4.4 controls; preserve all security and permission fail-closes.
5. **Cheap tests FIRST on Daniel-CL**, then source-appropriate targeted Windows behavior including a real nonauthor agent smoke. Only after local green run narrowly-scoped GitHub CI. Maintain rollback backup and don't deploy until live proof and owner release authorization.
6. When the active agents actually work, forward-port only verified hotfixes into shelved 0.4.5 with original full release holds intact.

**No code fix, compile, local installation, ownership signoff, or production hotfix approval has occurred from this triage document.**
