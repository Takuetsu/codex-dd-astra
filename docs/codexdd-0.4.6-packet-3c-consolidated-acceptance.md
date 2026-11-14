# CodexDD 0.4.6 — Phase 3C consolidated source/proof audit

**Opened:** 2026-11-13. **Accepted:** 2026-11-14. **Status: PHASE 3C ACCEPTED — focused source-matched native Windows contract complete; not a release or installation.**
**Branch:** `dd/codexdd-v0.4.6-agent-reliability`. Production remains `0.4.4`, `0.4.5` remains frozen. No installation, milestone coding, owner QA, or release implied by focused CI.

## Evidence matrix (do not count a skipped or earlier-source test)

| Phase 3C capability | Evidence and immutable checkpoint | Disposition |
| --- | --- | --- |
| 3C.1a rejection of false READY / foreign green exec | [Actions #43117038186](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43117038186), `24b596110cba9c3d3b849d155413fbca0b06f1d0` | ACCEPTED focused negative |
| 3C.1b first-user bound source/independent native role | 3C.1b.2f–2i accepted packet records and Core/TUI native Windows proofs; see their own exact-SHA docs | ACCEPTED focused seams, not complete App lifecycle |
| 3C.1b.2j1 real App event routing and foreign-READY refusal | [Actions #43347103729](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43347103729), `58dd0a3e06f4fbc619f7fc5d85209c36298f2af2` | ACCEPTED focused App event boundary |
| 3C.1b.2j2 one App-started fresh Validator, pinned root and owner gates | [Actions #43435363662](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43435363662), `628d75c606ba499818344f561178fa214f72db76` | ACCEPTED focused App/recording-server |
| 3C.1b.2j3a stale/foreign automatic handoff and duplicate refusal | [Actions #43440575225](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43440575225), `25d0ebc5db9af7fc2af07d968a633033317b5d12` | ACCEPTED 2/2 |
| 3C.1b.2j3b1 embedded app-server cold Validator gate checkpoint | [Actions #43457407670](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43457407670), `4d6dc84ecd33b8cb8528b2acea8d7b0c01254674` | ACCEPTED 1/1 |
| 3C.1b.2j3b2a real OS-process Validator TUI checkpoint and source/root drift | [Actions #43468082893](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43468082893), `4bbda6aab623c4fbf29196af19010e300684b222` | ACCEPTED 1/1 |
| 3C.1b.2j3b2b1 native Core source + Validator receipts across OS restart | [Actions #43475890208](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43475890208), `3772a8e9fed0ff1ee71614fad883bd8ebe74b29f` | ACCEPTED 1/1 |
| 3C.1b.2j3b2b2a cold App event routing and stale pinned READY quarantine | [Actions #43501797235](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43501797235), `59cecfa0ef8d772fb5858f330bd00d13003c5c07` | ACCEPTED 2/2 |
| 3C.1b.2j3b2b2b **positive** App-owned actual two Core native Workers, fresh automatic Validator and one READY across real OS restart | [GitHub Actions #43603946333](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43603946333), immutable tested source `c1e5be2dce8f35e234f5f91442b22239f59cffa2`; `codexdd_046_3c1b2j3b2b2b_real_app_owned_native_validator_after_restart` | **ACCEPTED 1/1** real Windows process fixture with native Core receipts |
| 3C.2 first-origin header, immutable scope/CWD and actionable diagnostics | [GitHub Actions #43603946333](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43603946333), immutable tested source `c1e5be2dce8f35e234f5f91442b22239f59cffa2`; `codexdd_046_3c2` filter covers 3C.2a/b/c, including locked invalid first-header and blocked/operations-specific next actions | **ACCEPTED 5/5**, zero ignored |
| Final source-matched 3C integration and regressions | [GitHub Actions #43603946333](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43603946333), immutable tested source `c1e5be2dce8f35e234f5f91442b22239f59cffa2`; source preflight + Rustfmt passed; 5 status, 1 positive restart, 2 cold controls, 2 replay refusals and 1 native routing guard | **ACCEPTED 11/11 native**; 0 failed, 0 ignored. Source SHA pinned to tested commit; later documentation-only closeout is not a new binary source validation |

## Phase 3C exit contract

1. Preserve exact source root, Git SHA, task kind, first accepted user role/scope and protected approval/sandbox rules. No old timestamp, queued instruction, detached worktree or duplicated Core event can create fresh authority.
2. Real Core native work_packet PASS is source-only, not independent QA. Fresh separate Validation Worker must run a new Core independent_review native profile; no generic green exec, fixture, unsupported provider or old cached READY can satisfy the owner gate.
3. Normal TUI `App::handle_app_server_event` and thread buffer, `AppEvent::StartAutomaticValidation`, separate real app-server OS children, first Validation prompt, native dynamic-tool receipts/signals and `PersistWorkflowState(SetReadyForOwnerQa)` each participate in the **same** positive test, and READY is persisted only once.
4. Missing/wrong header, insufficient profile availability, stale source or paused/blocked state yields actionable `SETUP_BLOCKED`/operator next step. A status must never claim an unavailable Core tool is installed solely because old LVO ran, nor suggest source validation for an operations-only task.
5. Run the exact current-source native Windows tests and document immutable run URLs, SHA, verified counts, any red history and narrowly corrected root cause. No waived/synthetic/skipped test is accepted as PASS.

**Boundary:** Even a fully ACCEPTED Phase 3C still leaves 3B.4, conditional 3D, 3E, 3F and 4.X (full Windows real-agent/release gates, owner acceptance and installation) OPEN. Only after full 0.4.6 release and owner-approved installation may the previously authorized roughly seven-day **single BREAKWATER milestone soak** begin.

## Final source-matched acceptance — 2026-11-14

**Source of executable proof:** [GitHub Actions #43603946333](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43603946333), immutable tested source `c1e5be2dce8f35e234f5f91442b22239f59cffa2`. GitHub confirms preflight PASS, Rustfmt PASS, and native Windows PASS. The full Windows job log explicitly enumerates every required test and reports these **six native test invocations / 11 total passed**:

| Exact-source native group | Required result | Verified result |
| --- | --- | --- |
| 3C.2a/b/c status diagnostics, `codexdd_046_3c2` | Five role, source, gate, blocked/operations and malformed first-message diagnostics | 5 passed / 0 failed / 0 ignored |
| 3C.1 positive App-owned Core two-Worker restart | Real WebSocket process restart, native implementation `work_packet`, distinct native independent `independent_review`, single owner-READY persistence | 1 passed / 0 failed / 0 ignored |
| 3C.1 cold App replay refusal | Old automatic Validator handoff after real app-server restart cannot create authority | 1 passed / 0 failed / 0 ignored |
| 3C.1 historical source-pinned READY quarantine | Cold resume cannot reuse stale source-pinned READY | 1 passed / 0 failed / 0 ignored |
| 3C.1 duplicate/stale origin handoff refusals | New Validator duplicate and old origin-SHA/reload cannot start another authorized Validator | 2 passed / 0 failed / 0 ignored |
| 3C.1 selected-thread/native evidence routing guard | Only selected-thread evidence can contribute; foreign READY is refused | 1 passed / 0 failed / 0 ignored |

**Final count: 11 passed, 0 failed, 0 ignored.** All six invocations were enumerated and executed on `c1e5be2dce8f35e234f5f91442b22239f59cffa2`, not inferred from historical jobs or an overall green checkmark. All earlier historical evidence matrix entries remain narrow and independently traceable.

**Failure history and resolution:** older [Actions #43530400184](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43530400184) exposed invalid first-user Worker-header delimiter placement. A correction in `9e01f122` repaired the owner's first-message grammar; [Actions #43600000652](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43600000652) then exposed a genuine test-harness mismatch: the same-host real WebSocket app-server was modeled as a remote workspace, which intentionally suppresses local Core CWD/source binding. Source `56b98323` added a `#[cfg(test)]` same-host daemon transport-mode helper so the independent real app-server OS children exercise **local-daemon** parameters, while production remote-workspace security remains unchanged. The exact Rustfmt correction `c1e5be2d` yielded the accepted CI run.

**Audit boundary:** Phase 3C only is ACCEPTED; this proves the bounded native source/Validator lifecycle and diagnostics contract. No broad agent soak, release, merge, installation, owner acceptance, or production version promotion is implied. **Remaining:** Phase 3B.4, conditional 3D, 3E, 3F and 4.X still require their own independent entry/exit gates. A documentation-only acceptance commit after the tested SHA records evidence but is not substituted for a fresh native source PASS.


## 3C combined acceptance tightening — exact current-source regression gate

The scoped positive Windows job is extended to reuse its already compiled native `codex-tui --lib` test binary to re-execute the real OS cold-App automatic-handoff rejection and source-pinned historical READY quarantine guard **on the same final SHA** as the five 3C.2 diagnostics and the positive App-owned two-Worker sequence. Each is individually enumerated and required to report 1 passed / 0 failed; neither skipped, foreign-source nor historical results count. Source/Rustfmt prerequisites still precede the Windows job. This exact-source plan passed in [GitHub Actions #43603946333](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43603946333), immutable tested source `c1e5be2dce8f35e234f5f91442b22239f59cffa2`; each native group was individually listed and executed with no ignored results.
