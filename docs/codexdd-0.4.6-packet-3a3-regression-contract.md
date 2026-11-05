# CodexDD 0.4.6 — Packet 3A.3 regression fixture and acceptance contract

**Date:** 2026-11-05  
**State:** REGRESSION CONTRACT CHECKPOINT; NOT EXECUTED / NOT AN IMPLEMENTATION PASS.  
**Parent:** `3d21a5614fb2142a2371884ef3c7eef1550d785e` (3A.2), production baseline `b25550dcb2c6e84a84074b603086c9db6f32a63b` (0.4.4).  
**Source fixture:** [machine-readable 25-case matrix](regression-fixtures/codexdd-046-agent-reliability-cases.json).

## 1. Scope, semantics and non-claims

This packet establishes deterministic, versioned **input/expected-outcome fixtures**, a test ownership contract, and numeric provisional A/B acceptance thresholds **before implementation**. The JSON is a specification consumed by upcoming focused unit/integration tests; there is **no fixture harness and no Rust test executed yet**. Do not confuse a machine-readable expected outcome with a native PASS, live installed source verification, or an authorized operating action.

Each case records `id`, P0/P1 classification, owning implementation packet, initial `given` state, `stimulus`, expected disposition/terminal and negative controls, and test owner. Proposed disposition labels are contract vocabulary, not claims that matching enum variants already exist. An implementation may map them to stable typed reason codes while retaining these semantics. Preserve the `source_sha`, worker identity, gate scope, profile and evidence provenance across that mapping.

There are **25 cases: 21 P0 and four conditional P1**, reviewed against [3A.1 evidence](codexdd-0.4.6-packet-3a1-evidence.md), [3A.2 source map](codexdd-0.4.6-packet-3a2-source-authority-map.md), the [0.4.6 design](codexdd-0.4.6-agent-reliability-design.md) and the November 4 owner backlog. All cases remain `contract_only` until executable assertions run on source-matched Windows.

## 2. Case ownership

| Group | IDs | Required behavior | Next executable owner |
| --- | --- | --- | --- |
| P0 A — gate evidence and terminal | A01–A07 (7) | Prevent partial/stale/wrong-worker READY; complete independent validator may emit a trusted READY; preserve bounded terminal recovery and independent QA | 3B.3 and 3C.1; `adaptive_evidence_tests.rs`, `adaptive_trusted_signal.rs` |
| P0 B — target/role/profile admission | B01–B06 (6) | Milestone-root launch vs nested authorized worktree; wrong repo; missing profile; Repair missing Implementation-only LVO; staged changes; valid Node/Rust preflight | 3B.1–3B.2; `codexdd_validation_tests.rs`, role/registry tests |
| P0 C — progress/refusal | C01–C05 (5) | One useful recovery or typed block for unchanged infrastructure/setup/policy refusal; continue when new admissible evidence exists; restore digest on resume | 3B.4; `adaptive_controller_tests.rs`, runtime bridge tests |
| P0 D — bounded non-source tasks | D01–D03 (3) | Actual safe 2-vs-4 benchmark, owned/healthy QA-preview, reject foreign stale PID; never invent source LVO proof | 3D.1; guarded operation and task-kind tests |
| P1 E/F/G conditional | E01–E02, F01, G01 (4) | In-capacity allocation, over-capacity refusal, supported-backend tool_choice diagnosis and first-assignment guidance | 3D.2, 3D.3, 3C.2 after exact source-matched repro |

**P1 exit rule:** E/F/G remain diagnostic/test-contract cases until exact installed/configured support and in-capacity or supported-provider defect are demonstrated. An eight-worker request exceeds the reviewed defaults (V1 six subagent slots; V2 four total threads, including parent). Repair Worker not seeing Implementation-only LVO and malformed header refusal are expected safety boundaries. No automatic cap increase, provider replacement, role relaxation, or blanket fallback.

## 3. Exact evidence requirements (future implementation)

An admissible source/operations/QA record must be bound to the current authorized **repository + target root + source SHA + worker identity/role + task/gate scope + required command/profile + run ID + timestamp + actual native outcome**. Include observed service ownership/port/health response for a required runtime preview; evidence from an older successful `exec-*`, another worker, or a narrower focused gate cannot satisfy a missing official or live gate.

Admission order: **role/scope → task kind → explicitly authorized target SHA/root → profile/tool availability → safe execution → new admissible evidence → remaining prerequisites → trusted native terminal**. Missing profile, policy restriction, provider rejection, capacity shortage and setup problems are typed blocks, not model capability signals, source FAIL, synthetic native validation, or permission to mutate active BREAKWATER/CV worktrees.

On missing native terminal after real proof: preserve exact evidence; allow no more than **one** bounded authorized recovery before typed block. Productive multi-turn work is not subject to an arbitrary global turn limit. A changed digest must mean *new applicable evidence or an authorized phase advance*, not merely a successful command or new prose.

## 4. Provisional numeric acceptance, defined before benchmarking

| Dimension | Required criterion |
| --- | --- |
| Deterministic P0 tests | **21/21 applicable P0 fixtures green** on the changed source after corresponding owner packets; all existing safety regressions retained |
| Safety-critical failures | **0** false full READY, wrong-root validation, foreign-process lifecycle action, manufactured native PASS, or unauthorized role grant in fixture and acceptance testing |
| Evidence-exhausted loops | At most **one** additional safe recovery after unchanged proof, then exact typed block; **0** unwarranted model-family escalations |
| P1 conditional | Four diagnostic cases recorded; an unconfirmed backend/capacity hypothesis does **not** become a speculative release feature |
| Paired end-to-end scenarios | At least **10 comparable paired runs**, same task/authority/profile/resources against ordinary Codex where feasible; use isolated disposable targets |
| Completion | Target at least **9/10 complete, correctly evidenced CodexDD runs**, and no more than **one fewer completion** than equal-access control per ten paired runs |
| Cumulative token use | Median paired per-task total tokens **≤1.15×** equal-access control; report all runs, not just successful turns |
| Wall-clock | Median paired task wall-clock **≤1.20×** equal-access control; include validation and interventions |
| Operator interventions | Total manual interventions **≤ control** for comparable tasks; separately count setup blockers and explicit owner gates |
| Data quality | Record source SHA, scope, worker role, native run evidence, actual final disposition, time, tokens, retries, escalations and interventions; no raw secrets/prompts |

These are **provisional no-material-regression targets**, not demonstrated measurements or a waiver. Ten pairs are a small sample, so report individual cases, denominators, failures and uncertainty rather than claiming a statistically proven efficiency gain. If ordinary Codex lacks equal authorized tool access, mark that pair incomparable; do not force a misleading comparison. If a performance criterion is infeasible or the task mix changes, document it and obtain owner review **before** changing the acceptance contract; never waive the zero-false-ready/security rules.

Suggested ten-case scenario mix: four independent source+full-gate cases (including one pending official gate), two nested-root/profile/role cases, two explicitly owned QA-preview/operations cases, and two safe non-source benchmark or unchanged-error recovery cases. Never touch live BREAKWATER/CV branches, assume a running host, or perform privileged service actions without explicit authority. Conditional P1 probes run separately when supported.

## 5. Validation and next packet

**3A.3 cheap validation:** the JSON is syntactically parseable, case IDs unique, all 25 cases have `given`/`stimulus`/`expected`/test owner, exactly 21 P0 plus four conditional P1, and numeric acceptance fields are present. Only source identity and text/fixture shape are checked here; **no Cargo, Windows native receipt or GitHub CI is warranted for this docs/fixture-spec checkpoint**.

**Next: 3B.1 — implementation of fail-closed authorized target/worktree, role and task-kind preflight.** First refresh the branch SHA, inspect the exact 0.4.4-based Rust owners, add focused executable tests from B01/B02/B05 (and B04 safety control as applicable), and checkpoint a narrow source patch. After that commit, use cheapest matching validation on **Daniel-CL** in `E:\codexdd`; no GitHub CI before local green. Owner's `D:\bw58` CodexDD launch convention stays unchanged. No 0.4.5 thaw, production merge, installation, release/version bump or new privilege is authorized.
