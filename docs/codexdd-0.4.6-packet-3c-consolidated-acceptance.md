# CodexDD 0.4.6 — Phase 3C consolidated source/proof audit

**Opened:** 2026-11-13. **Status: WIP — Phase 3C NOT YET COMPLETE.**
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
| 3C.1b.2j3b2b2b **positive** App-owned actual two Core native Workers, fresh automatic Validator and one READY across real OS restart | `docs/codexdd-0.4.6-packet-3c1b2j3b2b2b-positive-app-core-restart.md`; test `codexdd_046_3c1b2j3b2b2b_real_app_owned_native_validator_after_restart` | **OPEN** until exact native Windows compile + real PASS |
| 3C.2 first-origin header, immutable scope/CWD and actionable diagnostics | 3B.2c, 3C.1b.2j2 and 3C.1b.2j3a accepted original-role/CWD controls; new tests `codexdd_046_3c2a_*` in `chatwidget/adaptive_effort.rs` | **OPEN** until 3/3 native Windows status tests; audit next-action priority |
| Current-head integration and regressions | One newest exact SHA, source-contract/Rustfmt, 3C.1 positive + 3C.2 diagnostics, preserve existing negative refusals | **OPEN** |

## Phase 3C exit contract

1. Preserve exact source root, Git SHA, task kind, first accepted user role/scope and protected approval/sandbox rules. No old timestamp, queued instruction, detached worktree or duplicated Core event can create fresh authority.
2. Real Core native work_packet PASS is source-only, not independent QA. Fresh separate Validation Worker must run a new Core independent_review native profile; no generic green exec, fixture, unsupported provider or old cached READY can satisfy the owner gate.
3. Normal TUI `App::handle_app_server_event` and thread buffer, `AppEvent::StartAutomaticValidation`, separate real app-server OS children, first Validation prompt, native dynamic-tool receipts/signals and `PersistWorkflowState(SetReadyForOwnerQa)` each participate in the **same** positive test, and READY is persisted only once.
4. Missing/wrong header, insufficient profile availability, stale source or paused/blocked state yields actionable `SETUP_BLOCKED`/operator next step. A status must never claim an unavailable Core tool is installed solely because old LVO ran, nor suggest source validation for an operations-only task.
5. Run the exact current-source native Windows tests and document immutable run URLs, SHA, verified counts, any red history and narrowly corrected root cause. No waived/synthetic/skipped test is accepted as PASS.

**Boundary:** Even a fully ACCEPTED Phase 3C still leaves 3B.4, conditional 3D, 3E, 3F and 4.X (full Windows real-agent/release gates, owner acceptance and installation) OPEN. Only after full 0.4.6 release and owner-approved installation may the previously authorized roughly seven-day **single BREAKWATER milestone soak** begin.

## Current-run hold

Positive App/Core and status diagnostics were prepared on `dd/codexdd-v0.4.6-agent-reliability` through source commit `7b1f2f274ab5fc77f534fb8cef4e6e5b20316750`. The early one-shot native opt-in [Actions #43507811429](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43507811429) is on older immutable source `36e32e1d71e1af5795f67967f216ee956a976ee3` and is **diagnostic**, not necessarily current-head acceptance. The latest corrected source must receive its own exact-source native PASS. Status/Phase 3C completion remains blocked until that is demonstrated.
