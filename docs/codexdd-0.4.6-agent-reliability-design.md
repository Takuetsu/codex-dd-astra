# CodexDD 0.4.6 — Agent Reliability & Completion
**Milestone handoff / owner-selected version: 0.4.6 (2026-11-05)**

**State: PLANNED / READY FOR A NEW-THREAD 3A.1 RECONNAISSANCE. NOT IMPLEMENTED, BUILT, MERGED, INSTALLED OR RELEASE-APPROVED.**

## 1. Purpose and authority

The owner selected **0.4.6** to restore dependable real-world CodexDD agent behavior instead of continuing the frozen upstream/daemon/TUI 0.4.5 migration. This is an explicit superseding milestone-selection decision relative to the older (October 19–21) backlog text that said the next release number was unassigned and 0.4.5 must finish first. It **does not** waive independent tests, security guarantees, GitHub promotion, owner acceptance, or freeze conditions.

Success means a correctly bound agent can **finish the complete authorized task** or produce one actionable, evidence-grounded BLOCKED result; it must not claim full-gate READY on partial/stale proof, validate a wrong source tree, repeatedly escalate on unchanged tool errors, or force the owner to reconstruct a lost handoff. Task efficiency means useful completed work per total time/tokens and operator interventions, not short individual turns.

**Do NOT integrate the frozen 0.4.5 candidate or simply bump its version.** Version 0.4.6 is an intentionally separate patch/minor-series successor based on 0.4.4. If 0.4.6 later ships first, 0.4.5 remains historically shelved, and its feature work must be forward-ported onto a newer approved version line; never later deploy an older 0.4.5 binary over a repaired 0.4.6 installation.

## 2. Immutable starting identities

- Repository: `Takuetsu/codex-dd-astra`, **Windows-focused**.
- Canonical production source branch: `dd/astra-policy-v2` at reconnaissance `b25550dcb2c6e84a84074b603086c9db6f32a63b`, product version file `codex-rs/codexdd-version.txt = 0.4.4`.
- Reviewed triage-only checkpoint: `dd/codexdd-044-active-agent-stability-triage-20261105` at `8cee2c8a7d6ef7ebfaa65569dcd86b3ad263de41`.
- **0.4.6 planning/implementation branch:** `dd/codexdd-v0.4.6-agent-reliability`, initially created from the triage-only checkpoint. The new thread **must re-read its live HEAD**; never assume the pre-handoff SHA remains current after planning commits.
- `0.4.4` is the expected deployed *product family*; actual `codexdd --version`, active binary hash, `/status`, worker role, run origin and app-server transport on Daniel-CL, Doug and Stonks are **not yet verified for the incidents**. Do not conflate GitHub HEAD with the installed runtime.
- Frozen 0.4.5 checkpoint: `a69ce2ee19a50b834bc116246f94940bef8b8806`, [freeze decision](https://github.com/Takuetsu/codex-dd-astra/blob/a69ce2ee19a50b834bc116246f94940bef8b8806/docs/upstream-candidates/codexdd-045-frozen-2026-11-05.md). PRs #83–85 remain draft. 3F.93 3/3 nonrelease real-process tests green; 3F.94 native Windows supervisor 0/2 blocked by job object; 3F.95 TUI test failed to compile, not executed; no 0.4.5 release authority, 13 high-risk holds and 0/109 signoffs. No retry/CI/unfreeze of these workstreams.
- Canonical, freshest owner incident evidence: [codex-dd — Upcoming Version Backlog](https://docs.google.com/document/d/1vJ617lLax8GFUVWjJaNE2veCfSyjD5OOydBvywWA8iA/edit) (2026-11-04 entries) and [BREAKWATER — CodexDD Operational Guide](https://docs.google.com/document/d/1g9n1a09TmftzxuAYY_C8IQ239My4CGm-SOFg8XSOjgo/edit). Repository source triage: [docs/codexdd-0.4.4-active-agent-stability-triage-2026-11-05.md](codexdd-0.4.4-active-agent-stability-triage-2026-11-05.md). Prior 0.4.4 LVO source evidence: [DD-BL-006 reconnaissance](codexdd-0.4.4-dd-bl-006-reconnaissance.md).
- Obey [major-phase/numbered-work-packet standard](codexdd-major-phase-work-packet-standard.md), including durable Git checkpoint per packet, small focused validation, and operator-owned promotion.

## 3. Scope — required, prioritized, bounded

### P0 / Must ship (all four)

**A — Correct end-to-end completion and evidence-bound terminals (DD-BL-005 / DD-BL-007).**
- A typed prerequisite/gate-state contract distinguishes SOURCE_VALIDATED, RUNTIME_PREPARED, READY_FOR_OWNER_QA, and OWNER_QA_PASS, plus explicit pending/blocked requirements.
- Bind evidence to **current authorized source SHA, worker identity/role, required command/case/profile, timestamp and task/gate scope**. Older successful `exec-*` or narrower focused test results cannot satisfy a missing full regression or live runtime/preview requirement.
- An actual requested operational prerequisite (e.g. owned running host, current listener, successful health request on approved ports, valid safe world) is not satisfied by green source-only test output.
- Suppress false terminal admission; successfully finished **scope** can be accurately reported without claiming completed **overall gate**. Do not manufacture `ready_for_owner_qa` or misuse `repair_required` when the blocker is setup/permission.
- If a trusted terminal signal/tool is unavailable or rejected, preserve source/evidence, allow at most one bounded authorized recovery/handoff or yield a typed, actionable blocked result. Do not rerun completed tests merely to obtain a different terminal.

**B — Source/worktree, role and validation tool/profile preflight (DD-BL-006 plus DD-BL-002 regression).**
- Respect the **owner's established Windows launch convention**: launch CodexDD at milestone root (e.g. `D:\bw58`); authoritative target worktree may be nested elsewhere. A child shell's `Set-Location` must not be confused with native CodexDD runtime's process CWD. Resolve explicit owner-authorized target, nearest actual Git root, selected branch/commit, clean/dirty state, approved native profile and tool availability **before** protected MechanicalValidation.
- Fail closed on absent/mismatched repository/profile, wrong source SHA, incorrect role, or incomplete native runner. Return an actionable `SETUP_BLOCKED` with the expected path and the observed path. **No cross-repository fallback, fake PASS, status coercion, disabled safety policy, or pressure-driven model escalation.**
- Preserve 0.4.4's protected Implementation-only `run_codexdd_validation` contract; a Repair or independent Validation worker lacking this Implementation-only runner is not automatically a defect. Existing authorized repo-owned official validation commands remain usable when appropriate to the role.
- Safe read-only Git status/diff and approved bounded Node/Rust validation must remain available without making source-changing commands available in MechanicalValidation. A staged/uncommitted source checkpoint must never be silently destroyed.

**C — Progress-aware continuation and correct error classification (DD-BL-007).**
- Persist a small durable task progress/evidence digest with pending phases and exact source binding. Determine whether each proposed successor has a concrete authorized step or new admissible evidence.
- No repeated 5–30s no-op turns, stale-evidence restatement, or escalation for missing profile, policy refusal, capacity, unknown provider subtype or setup failure. Exactly one useful safe action or one typed actionable block when evidence is unchanged.
- Protect legitimate productive implementation/validation multi-turn continuation and independent Worker handoffs; do not impose an arbitrary total turn/time limit that truncates real work.

**D — Bounded task-appropriate operations/benchmark routing (DD-BL-007 product gap).**
- Add only the smallest explicitly authorized, narrowly scoped non-source-changing task route needed for active BREAKWATER operations/QA-preview or safe benchmark tasks. Do not force an Implementation Worker's `mechanical_validation`/LVO `work_packet` when no source was changed and the task calls for service readiness or 2-vs-4-thread benchmark evidence.
- First verify actual task authority, correct target SHA/worktree, process ownership, allowed approved commands and resource locks. With valid authority, execute existing repo-owned safe operations/benchmark tools through the normal guarded execution path; if implementation of a fully authorized lane would require new privileged managed-service architecture, **return an accurate setup/ownership handoff instead**. No accidental lifecycle operations on another project or stale PID; no elevating shared daemons.
- This is limited **task classification, safe execution and outcome evidence**, NOT a broad new operations framework or automatic uncontrolled process manager.

### P1 / Include if targeted real repro confirms impact; otherwise document and defer

**E — Worker spawning / seat exhaustion.** BREAKWATER-DOUG screenshots show eight requested workers, zero successes, `collab spawn failed: agent thread limit reached`. First verify installed version, `/status`, configured maximum, true live/closed thread count, leaks vs intentional limit, and test 1/in-capacity/over-capacity. Fix bounded seat accounting/release only if reproduced **inside supported capacity**. Never blindly increase default thread limits or circumvent resource governance.

**F — Provider compatibility and first-assignment role binding.** Z-A0.56C `rustponsesapi` reported HTTP 400 unsupported `tool_choice` in a terminalization attempt; exact request/backend version unverified. Test proper capability negotiation and safe typed error only for supported configured backends. A missing first-nonblank `[adaptive_worker]` header is an assignment/preflight problem; show actionable TUI guidance before expensive work without silently binding a later message as privileged authority.

**G — Minimal status/progress telemetry.** For proof and operator usability, expose compact reason codes, pending prerequisites, selected source SHA, tool/profile readiness, productive turn delta, and cumulative time/tokens/escalations without raw prompts/secrets. No wholesale UI redesign or speculative budget algorithm rewrite.

**P1 implementation exit rule:** Any P1 candidate not reproducible on a built source-matched Windows 0.4.4/0.4.6 test profile becomes a documented regression/diagnostic case, not an open-ended requirement to rewrite unrelated subsystems. P0 correctness is never deferred silently.

## 4. Explicit exclusions

No 0.4.5 upstream-sync, dependency/security migration, 3F.94 job-object/daemon-updater supervisor, unfinished 3F.95 real-process TUI test, CLI distribution overhaul, model-family catalogue refresh, full arbitrary operations manager, auto-install, policy weakening, broader agent-concurrency-limit increases, gameplay or BREAKWATER/CV source edits. Preserve existing GPT-6 / GPT-6.1 Sol / Luna / Astra routing and explicit budget/complexity guards except a demonstrated bug fix. Preserve privacy, local provider boundaries, all role/permission/sandbox/elevation controls, native receipts, stop rules and fresh independent validators.

## 5. Ordered numbered work packets

Use each numbered packet as the smallest checkpointed execution unit. **Start at 3A.1**, not the entire release in one run. For documentation-only packets, no Cargo/CI is warranted.

- **3A.1 — Read-only source and runtime evidence matrix.** Verify current production + 0.4.6 branch SHAs, actual installed-version evidence (request a short owner diagnostic only if indispensable), Google backlog incidents (observed vs hypotheses), existing 0.4.4 tests and owning paths, and freeze boundary. Produce `docs/codexdd-0.4.6-packet-3a1-evidence.md`. Do not modify production source or ask for a full build.
- **3A.2 — Source/authority map.** Trace exact workflow terminal/evidence stores, LVO/profile root discovery, phase decisions, remote/provider tool exposure, worktree selection, Windows agent capacity, and non-source task routing. Identify already-fixed behaviors and missing native coverage; avoid duplicate repairs.
- **3A.3 — Regression contract and feature limits.** Add small deterministic reproduction fixtures for P0/A-D. Mark P1/E-G inclusion by verified evidence and cap their implementation. Define numerical A/B performance acceptance thresholds **before** benchmarking, with owner review when policy scope changes.
- **3B.1 — Worktree/role/task-kind preflight.** Read-only target/authority verification before admission, especially milestone root vs nested worktree and immutable candidate SHA.
- **3B.2 — LVO tool/profile availability and typed policy blockers.** Native profile checks and safe Node/Rust mechanical command handling; no fake PASS, no new repair authorization from setup failures.
- **3B.3 — Durable gate prerequisites and evidence provenance.** Scoped source/operations/QA readiness, current evidence match, preservation and resume/worker handoff.
- **3B.4 — Progress digest and refusal taxonomy.** No-evidence successor cap, infrastructure/tool/provider/model distinctions, no spurious escalation.
- **3C.1 — TUI terminal admission and recovery.** Honor remaining prerequisites, trusted signals, valid positive terminal, missing-signal/blocker behavior and no stale-exec reuse.
- **3C.2 — TUI role-binding/status diagnostics.** First-message validation, actual tool availability and selected source, precise actionable next step, no destructive reset.
- **3D.1 — Bounded approved operations/research/benchmark routing.** One safe host-preview/recovery scenario and one 2-vs-4-thread benchmark scenario; obey roles and process ownership, actual proof of service response, no new privileged daemon service.
- **3D.2 — Worker-spawn concurrency diagnostics and targeted repair (conditional).** Fix only evidence-proven in-capacity defects, preserve limits and fresh-worker isolation.
- **3D.3 — Provider `tool_choice` compatibility and evidence-safe fallback (conditional).** Fix only confirmed request/adapter behavior; no model or authentication bypass.
- **3E.1 — Build/provenance, release-version selection.** Only after implementation tests, prepare explicit `0.4.6` candidate identity/locking; don't bump the production product version during planning.
- **3E.2 — Focused Windows integration proof.** On exact HEAD, verify no false ready, no wrong worktree/profile, no no-op escalation, live path handoff and actual agent usability, without inventing a native receipt.
- **3F.1 — Source/security/semantic audit** across all changed files and unchanged 0.4.5 holds; independent review, no accidental feature import.
- **3F.2 — Candidate artifact, CI and provenance audit** after local cheap checks and owner-approved promotion.
- **Phase 4.1 — Daniel-CL Windows targeted real-agent smoke** against disposable profiles; check elevated SSH embedded fallback, `/status` and F2, LVO Node/Rust, Worker/Validation/Repair paths.
- **Phase 4.2 — End-to-end project-like acceptance and efficiency comparison** on representative safe bounded BREAKWATER/CV/CodexDD scenarios (>=10 paired samples against equal-access ordinary Codex where feasible). Record completion, false-readiness count, safe-blocker accuracy, operator interventions, cumulative tokens/time, retries, and wall-clock; no fabricated claims of savings.
- **Phase 4.3 — Full release native gate** (repo-owned exact candidate 21-stage Windows profile as applicable), security/daemon boundaries, artifact PDB/provenance as required for 0.4.6, final CI only after local green, independent signoff, explicit owner acceptance, backup/install/rollback, staged Daniel-CL -> Doug -> Stonks smoke. Manual release/install authorization is separate.

At every packet: scope -> source/provenance read -> narrow change -> cheapest meaningful validation -> immutable Git commit -> status/results/next packet. If later detailed reconnaissance shows a bounded packet legitimately needs to split, add numbered subpackets rather than pushing a huge single phase.

## 6. Test matrix and nonnegotiable safety invariants

| Repro | Expected invariant |
| --- | --- |
| Focused 55/55 or 151/151 tests green, required official full gate unrun | No full READY; preserve evidence and pending official command |
| Source green but required approved preview port not listening | No full READY; setup/operations pending and actual health proof required |
| Previously valid exec refs from another worker/source SHA | No current source/ops terminal without fresh identity-bound proof |
| Terminal signal/tool unavailable after real evidence | One bounded useful recovery/typed BLOCKED; never falsify native terminal |
| Milestone root process CWD but authorized Git worktree nested at different SHA | Preflight binds correct explicit target or typed setup blocker; no fallback to old root |
| `work_packet` missing / Repair role intentionally lacks LVO | No fabricated product FAIL/PASS; correct role and profile-specific diagnosis |
| Git staged/uncommitted candidate in mechanical phase | Preserve edits and correct allowed read-only/validation tools; no unsafe mutation |
| Infra/tool/provider refusal with zero new evidence | No no-op adaptive loop and no capability-family escalation |
| Safe benchmark requested with no source edit | Actual approved bounded measurement or correct pending-operations explanation; no bogus mechanical source validation |
| Eight workers requested with unclear supported capacity | Inspect configured limit; one/in-limit succeeds, over-limit typed refusal; no blind raising |
| Unsupported `tool_choice` for alternate provider | No repeated invalid request; support declared capability or fail closed with actionable diagnosis |
| Properly bound authorized Worker completes all required source+operations gates | Trusted terminal, fresh independent validation preserved and owner QA still separately pending |

For failures, persist source SHA, executable identity, worker role, stage, exact needed evidence/permissions and deterministic classification. Avoid raw secrets; logs remain under safe temp/evidence paths, not active game worktrees. Do not use prior external CI as a substitute for a current exact-head local native gate.

## 7. Execution, ownership and release rules

- Assistant performs **GitHub-side** source edits, docs, worktree review and bounded packet commits. Operator does not manually edit Rust or juggle arbitrary CI failures.
- Canonical cheap local validation host is **Daniel-CL** via SSH. Keep worktrees under `E:\codexdd`. Do not run heavy builds on Stonks or Doug just because they are accessible.
- **Cheap local focused checks first**, after a committed source packet. Fix failures on GitHub, re-run the minimal local test. GitHub CI only after local passes. Docs-only checkpoint = source/diff/identity check, not a Rust build. Native non-elevated managed-daemon breakaway unavailable in restricted hosted runners is **not** to be waived, emulated or bypassed; no dependency on the frozen 3F.94 release scope.
- Never touch active BREAKWATER, CV, or Doug live branches/gameplay code to repair CodexDD. Use isolated disposable validation profiles/worktrees, read-only project reference context, and preserve uncommitted/staged work.
- Explicit owner approval for CI promotion when policy requires it, merging, installing or replacing production executables, real-machine releases, test waivers, new elevated/privileged operations, and scope changes. No 0.4.6 binary exists yet.
- A held/reproduced environment/setup condition does not authorize broad speculative feature work. Treat older version bug labels as hypotheses until the installed/runtime source proves recurrence.
- Do not restart 0.4.5 CI, move its branch refs, promote PR #83–#85 or reinterpret its 13 review holds / 0-of-109 owner signoffs.

## 8. New-thread startup contract

In a new ChatGPT conversation in the CodexDD project, paste the separate handoff in `docs/codexdd-0.4.6-new-thread-handoff.md`. It directs the assistant to start **3A.1 immediately**, re-fetch GitHub HEAD, read this design + triage + canonical Drive backlog and produce a durable read-only evidence checkpoint. Do not ask the owner to repeat previously settled decisions, send unnecessary PowerShell, open expensive CI, merge, build or install at activation.

**Immediate next packet: 3A.1; current milestone status: SETUP COMPLETE / NO IMPLEMENTATION OR RUNTIME PROOF.**

## 9. Owner release and BREAKWATER soak decision — 2026-11-12

**Owner direction (supersedes any proposed early BREAKWATER coding pilot):** Finish the full 0.4.6 development milestone and its required implementation, source/security review, focused and full native Windows acceptance, real-agent integration, release artifact/provenance checks, and explicit owner-controlled installation/rollback readiness **before** returning CodexDD to BREAKWATER coding. No interim BREAKWATER coding pilot is authorized solely because individual 3C or GitHub test packets pass.

**After 0.4.6 is finished and authorized for installation**, run an approximately **one-week soak test on one deliberately selected BREAKWATER milestone** before considering broader coding use. Treat the soak as an observed, limited production-readiness trial, not permission to ignore failed gates or waive rollback. Select the specific milestone and scope at that time; preserve the prior known-good executable and task worktree. Log actual agent completion, correct independent Validator/READY behavior, false completion signals, source/role mismatches, stalled/no-progress loops, restarts/resume, operator intervention, failures and regressions. Repair verified issues and repeat the appropriate validations before declaring general BREAKWATER use safe.

**Release sequencing:** Complete the remaining 3B/3C/3D/3E/3F/4 development and acceptance packets as applicable → obtain owner approval for installation/rollback → one-milestone roughly seven-day soak → review soak evidence and obtain owner approval before expanding to regular BREAKWATER milestone coding. This decision does not authorize an early install, modify active BREAKWATER/CV files, unfreeze 0.4.5 or override any security/source identity prerequisites.
