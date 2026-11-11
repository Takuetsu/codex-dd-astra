# CodexDD 0.4.6 — Packet 3B.3a: gate provenance source audit

**Date:** 2026-11-11  
**State:** SOURCE AUDIT CHECKPOINT COMPLETE; provenance enforcement and Windows native acceptance NOT YET IMPLEMENTED.  
**Development branch:** `dd/codexdd-v0.4.6-agent-reliability`  
**Inspected parent HEAD:** `d1cc0575472d61e9f9beb2c0d39b820c9b984876` (3B.2o accepted evidence).  
**Source family:** production 0.4.4 baseline `b25550dcb2c6e84a84074b603086c9db6f32a63b`; frozen 0.4.5 not imported.

## One-outcome packet card

- **Outcome:** Identify the exact source-level gap that allows incomplete/wrong-source success evidence to be mistaken for full `ready_for_owner_qa`, and pin the smallest safe ownership split before modifying the evidence/wire contract.
- **Authority:** 0.4.6 P0/A; 3A.3 fixture cases A01–A04 and A07; independent validator and native terminal remain mandatory. This packet authorizes only read-only source audit and a documentation checkpoint.
- **Acceptance:** Pin current ingester, registry, Core LVO source, TUI terminal and rollout persistence paths; specify required negative/positive regressions and how to preserve prior security controls.
- **Exclusions:** No source or protocol edits, CI, Daniel-CL commands, Node project execution, operations privilege, new model routing, frozen 0.4.5 work, production install/merge or release approval.
- **Stop/handoff:** Commit the audit alone, then start 3B.3b as a separate narrow implementation packet with exact source-matched cheap Windows validation before any wider CI.

## OBSERVED → EXPECTED → owning path

| Boundary | Source observation at inspected HEAD | Required 0.4.6 guard / owner |
| --- | --- | --- |
| Native validation emission | `codex-rs/core/src/tools/handlers/codexdd_validation.rs::emit_validation_result` sends a trusted completed `DynamicToolCall` with `profile`, `result`, `run_id`, `branch`, `head_sha`, `log_path` and optional failure fields. The handler separately validates Core Worker role, selected worktree and authorized SHA before the protected profile. The dynamic receipt does **not** itself carry all task-required gate IDs, target-root identity, worker role or live preview proof. | Preserve native receipt and Core pin checks; correlate emitted run/source facts to the pinned task and requirements, rather than treating any successful command as validation. 3B.3b. |
| TUI evidence extraction | `tui/src/adaptive_evidence.rs::record_from_item_completion` stores evidence ID, thread, turn, outcome, kind, tool, validation profile and failure fingerprint. No source SHA/root, run ID, assigned gate, effective role or completion timestamp is retained on `AdaptiveEvidenceRecord`. Generic command/MCP success is not tied to an official validation profile. | Add bounded typed provenance or a verified binding layer; ambiguous/unbound successes remain informational, not full-gate proof. 3B.3b. |
| Registry guard | `AdaptiveEvidenceRegistry::resolve_for_thread`, `resolve_for_turn` and `validate_outcome` correctly reject missing/conflicted, cross-thread, wrong-turn and wrong-outcome references. `conclusive_refs_for_thread` admits success by thread without source/gate semantics. | Keep these existing checks; add current candidate/root/worker/gate/profile matching without accepting stale or narrowed refs. 3B.3b. |
| Live status observation | `tui/src/chatwidget/adaptive_evidence.rs::validation_status_from_item_completion` extracts current profile, result, run ID and `head_sha` from the native LVO completion arguments, but stores this independently from generic evidence records. `register_adaptive_evidence` calls state persistence, not a prerequisite-receipt ledger. | Reuse genuine native source facts where proven, and fail closed on disagreement/absence. Never trust command output, model prose or manually supplied `head_sha` as native proof. 3B.3b–c. |
| Protected terminal admission | `tui/src/chatwidget/adaptive_trusted_signal.rs::valid_evidence_refs` checks only current thread and expected outcome across the supplied IDs. The `ReadyForOwnerQa` arm accepts a Validation Worker with any such successful ref; it does not verify mandatory full source/operations prerequisites or current source SHA. | Require all *explicitly assigned* gates to be satisfied by current admissible evidence before accepting the trusted READY signal. Missing/blocked gates must remain visible; no fake terminal. 3B.3c and 3C.1. |
| Implementation handoff | The existing `ReadyForValidation` arm requires current-turn native `work_packet` PASS, plus `targeted` after repair, and creates a fresh independent Validation Worker. This is already stronger than generic reviewer READY. | Preserve same-turn Implementation LVO control and independent Validator; a green work_packet is SOURCE_VALIDATED/ready_for_validation, not owner QA. 3B.3c; A07 safety control. |
| Durable replay | `tui/src/chatwidget/adaptive_effort.rs::durable_workflow_snapshot` persists assignment task kind, root/SHA, role/scope, last validation status and terminal through `app-server-protocol` → app-server `turn_processor` → rollout state. It does not persist the evidence registry or an explicit pending required-gate ledger. | Persist only compact scope/source-bound outstanding requirements and validated evidence/progress digest without raw prompts/secrets; on restart, never promote a previous terminal from an unverified ref. 3B.3d / 3B.4. |
| Owner QA latch | `latch_workflow_terminal` persists `SetReadyForOwnerQa` after the TUI accepts a native signal; that operation does not independently examine source/operations prerequisites. | Enforce prerequisites before latch; keep native-signal transport and final owner QA as distinct gates. 3B.3c, 3C.1. |

**Classification:** Product contract gap at source level (P0/A), **not** verified attribution of a past installed binary failure. The existing receiver/registry tests correctly establish narrower thread/turn/receipt protections, but not source-matched whole-task readiness. Existing green 3B.2o native checks are synthetic Core handler/policy tests, not private Node or live preview acceptance.

## Smallest executable follow-on packets

### 3B.3b — provenance identity and pure matching tests

1. Introduce a bounded, typed gate-evidence match contract for authorized repository/target worktree, exact candidate SHA, Worker thread/role, task scope, gate/profile/case, run ID, UTC completion time and native result. A field that cannot be trusted from the native completion is **unknown**, never copied from model prose to manufacture proof.
2. Connect actual LVO `head_sha`/profile/run ID to the Core-authorized source pin; distinguish a same-SHA `targeted` proof from required `work_packet` or owner-required full regression. A normal successful shell/MCP call is not automatically an official gate receipt or listener health proof.
3. Add deterministic A03 (prior SHA), A04 (wrong Worker or narrower scope), conflicting-ref, legacy-unbound and positive matching-record unit cases to `tui/src/adaptive_evidence_tests.rs`. Preserve existing thread/turn/repair-fingerprint protections.
4. Keep changes limited to evidence provenance and tests. No TUI terminal relaxation, new task kind, generated protocol changes, or policy grant in this subpacket.

### 3B.3c — explicit prerequisite ledger and terminal guard

1. Represent required SOURCE_VALIDATED, RUNTIME_PREPARED and independent VALIDATION prerequisites scoped to the owner-authorized task; pending, blocked and satisfied are distinct. OWNER_QA_PASS remains an owner-owned action **after** ready_for_owner_qa.
2. Evaluate all requirements before `ReadyForOwnerQa`; refuse full READY if a required official full regression never ran (A01), owned/healthy preview is absent (A02), or evidence is stale/wrong-scope (A03/A04). Preserve focused PASS and identify the exact missing gate instead of rerunning completed work.
3. Keep `ReadyForValidation` separate from independent Validator QA (A07). Do not reinterpret a missing signal as native success; trusted-terminal recovery belongs to 3C.1 (A05/A06).
4. Review the existing `green_reviewer_terminal_overrides_armed_failure_pressure_and_admits_no_successor` test: its bare successful command currently admits full READY. Replace the outdated oracle only alongside a valid typed-gate positive test, not as test-only masking of a real product gap.

### 3B.3d — durable, safe resume and narrow native acceptance

1. Persist compact pending required gates and source-bound digest through TUI snapshot, app-server protocol, app-server conversion and rollout state, with schema/backward-compatibility tests; a restored old status without matching proof remains pending.
2. Test fork/resume and fresh independent Worker identity without serializing raw command text, secrets or unsupported cross-thread authority. Do not leak stale `exec-*` refs across a new source SHA.
3. After source changes, use the cheapest applicable **Daniel-CL** exact-head targeted checks first; GitHub-only oracle needs an explicitly documented local blocker and owner approval under the packet standard. Prior GitHub green receipts are not a fresh native run for this code.

## Fixed proof boundary and negative controls

- Required gate list must be explicit and owner/task-authorized. Do not infer that every task requires a preview, and do not infer that a preview is healthy from source tests. A missing gate definition or trusted evidence for a requested full READY is `GATE_PENDING`/typed setup handoff, not PASS.
- Identity is repository + canonical authorized root + immutable SHA + worker role/thread + task/scope + exact required profile/case + native run/time/outcome; do not allow one matching field to compensate for missing others.
- A genuine source-profile PASS cannot establish listener ownership/port/health, and a valid local listener cannot establish full source regression or owner acceptance.
- Keep 3B.1–3B.2 safe source/worktree admission, 3B.2o typed policy refusal, Implementation-only LVO, repair budget, same-turn mechanical receipts, fresh independent validation and no privilege escalation unchanged.
- Do not create a synthetic `PASS`, bypass sandbox policy, move the owner's milestone-root launch CWD, edit active BREAKWATER/CV repositories or import frozen 0.4.5 code.

## Validation and disposition

**Performed:** Live GitHub development-branch HEAD and named source inspected; source behavior traced from Core native completion to TUI terminal and persisted rollout, and mapped against 3A.3 A01–A04/A07 fixtures. This is a **nonexecuting read-only audit**, not unit test, Windows compile, acceptance receipt or release approval. Per the packet standard, documentation-only scope does not warrant Cargo or new CI.

**Still open:** 3B.2 private isolated BREAKWATER Node-profile/approval and independent live Validation acceptance; 3B.3b–d implementation; 3B.4 progress controls; 3C–3F and Phase 4. The current 0.4.6 candidate remains nonproduction.

**Next packet:** 3B.3b — add source/Worker/gate provenance matching with focused negative tests, without altering terminal policy until 3B.3c.
