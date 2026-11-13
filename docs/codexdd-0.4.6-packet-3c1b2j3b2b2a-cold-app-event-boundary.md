# CodexDD 0.4.6 — 3C.1b.2j3b2b2a: real cold App event-buffer and stale automatic handoff refusal

**Opened:** 2026-11-13. **Parent:** 3C.1b.2j3b2b1 exact Windows run [#43475890208](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43475890208), native Core receipts 1/1 PASS on `3772a8e9fed0ff1ee71614fad883bd8ebe74b29f`. **Status:** ACCEPTED (Windows native source-matched gate #43501797235 on 59cecfa0ef8d772fb5858f330bd00d13003c5c07). 3C.1b.2j3b2b2, 3C.1 and 0.4.6 remain OPEN.

## Why another targeted control is required

3C.1b.2j3b2b1 used *real Core-native source/independent receipts and app-server process replacement*, but delivered the typed stream directly into the production ChatWidget reducer, not through the normal `App::handle_app_server_event` foreground channel. 3C.1b.2j3b2a used real WebSocket TUI remapping and durable state restoration, but its previous READY state came from a saved fixture and did not traverse an App-owned automatically created Worker. Existing 3C.1b.2j2/j3a App automatic handoff tests used recording/embedded servers and did not span real OS processes.

This bounded subpacket supplies the **negative cold-App event routing seam** without pretending it is the complete coupled acceptance.

## Focused source and controls

- Test: `codexdd_046_3c1b2j3b2b2a_cold_real_process_app_rejects_old_auto_handoff` in `codex-rs/tui/src/app/tests/adaptive_cold_process_event_routing.rs`; Windows/ignored-by-default only.
- Create a disposable Git SHA and local CODEX_HOME. Persist an *explicitly fixture-derived historical Implementation* `ReadyForValidation` state with both original owner-required gates to the first genuine app-server OS process through `thread/workflowState/update`. Shutdown/reap that child and resume the same saved rollout using a second real `codex-app-server.exe` process with the normal TUI WebSocket `AppServerSession` mapper.
- Require cold remapping to retain original source root, SHA, role and pending requirements, but clear historical terminal/success evidence and pause pending authorization.
- Attach that live remote session to the normal TUI `App` selected-thread channel. Feed a *deliberately stale* old adaptive runtime signal via `App::handle_app_server_event`, inspect its own foreground buffer, and consume with `App::handle_thread_event_now`. No native PASS or terminal may materialize.
- Present a queued `StartAutomaticValidation` for original thread/SHA to the real App dispatcher. It must refuse while there is no valid active source READY, leaving the same source selected and no owner READY persistence. Advance source HEAD and repeat to verify the stale identity remains untrusted.
- The test reuses exactly the previously accepted b2a real OS child startup/reaping helper, with no changes to runtime product policy or source permissions. Source remains disposable; no installation/game changes.

## Native acceptance and explicit hold

Dedicated Windows job `[3c1b2j3b2b2a-native]` requires exact branch/source baseline, source-contract preflight, Rustfmt and exactly one isolated `codex-tui --lib` ignored real-process test. A skipped/failed native job cannot count as success.

**Not yet proven together:** genuine current Core-native Implementation-to-fresh-Validation receipts flowing through the live `App` multiplexer and its auto-start event **in the same process-restart sequence**. That final positive coupled run is separately required as 3C.1b.2j3b2b2b. The 0.4.6 release and owner-approved one-milestone BREAKWATER soak remain blocked until all subsequent phases and full Windows/security/release gates are green.

## First Windows attempt and focused correction — 2026-11-13

[GitHub Actions #43483188152](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43483188152), SHA `b8a15c7f0bf013b4c024c67bc0a438025df1bac5`: source-contract preflight **PASS**, Rustfmt **PASS**, dedicated Windows compile **FAIL** before native regression execution. The seven errors were Rust `E0659` on `assert_eq!` at lines 151, 152, 161, 178, 179, 252, and 287 in `adaptive_cold_process_event_routing.rs`. The root cause was ambiguity between a macro imported by `use super::*` and the standard prelude's macro; this is **test-code compilation**, not an observed restart/READY product defect.

The one-file correction at `d54341c8bc4c014942949692cf7b329183e77538` changes exactly those seven invocations to `std::assert_eq!`; test data, process boundaries, origin gate policy, role/source checks, and READY assertion conditions remain intact. Exact-source opt-in native rerun dispatched by `e8fc281b97f84479bd4d9521d8ef24c24d17f155`: [GitHub Actions #43495201379](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43495201379). **Pending native acceptance:** do not call this subpacket green until the new Windows job genuinely compiles and executes its one ignored-by-default regression.

## Second Windows attempt: valid compiler, invalid fixture and newly exposed product gap

[GitHub Actions #43495201379](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43495201379), exact SHA `e8fc281b97f84479bd4d9521d8ef24c24d17f155`, source preflight PASS, Rustfmt PASS, native TUI compilation PASS, exact one test enumerated, actual Windows test **0 passed / 1 failed** after 1.64s. Reported: `persisted enabled adaptive state is missing its active route or attempt`. The isolated saved fixture had `enabled=true` but no `current_family`, `current_effort` or `attempt_number`. Correction at `7ee8219e915684d60b42ba982995d9324af46638`: restore valid A/Sol/high/attempt 1, accepted Standard complexity and MechanicalValidation lifecycle in the saved Implementation fixture. The existing strict persisted-state validation must remain unchanged.

Source review then confirmed a more significant **product bug** independently of that fixture error: `session_state::apply_persisted_adaptive_workflow_state_for_thread` quarantined historical `ReadyForOwnerQa`, but not source-pinned `ReadyForValidation`. A stored original Implementation READY could therefore be replayed after restart without its original ephemeral native source PASS despite the ledger being restored as pending only. Source fix at `1edab32d56f5e2291bdea1d6ca9ed8222c21c863` quarantines `ReadyForValidation` **only for pinned source-change authority**, preserving legacy unpinned Repair-terminal compatibility and all original gate requirements. A new real typed-history regression was committed at `cd293dff875a119bfeb5f4053fb5b0d775e4b6d6` to verify role, phase, route, source SHA, cleared terminal/turn/evidence, paused state, and both original gates pending. The dedicated native Windows job now enumerates the exact cold-App test and independently runs this precise unit test. No authorization, proof requirement, or source check was relaxed.

**Acceptance hold:** the combined new code remains unaccepted until source/Rustfmt and both focused native Windows tests complete successfully on one exact commit. The positive real automatic App/Validator lifecycle across restart remains `3C.1b.2j3b2b2b`.

## Latest exact-source Windows regression — acceptance pending

The scoped source-contract preflight was successful on `e416e88331c6035b9a08a3d4706d8d77bc57f584`. Its Rustfmt review reported only three layout differences; exactly those were corrected in `9363e42a21ee50f9e95643fa3f19f1eb33871d6b` and `fb24aec2a137bb2b11a99fbadcc3c973a1e03d10`. The production source restoration condition and native Windows App test assertions were not relaxed.

Dedicated native gate [GitHub Actions #43501797235](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43501797235) targets exact SHA `59cecfa0ef8d772fb5858f330bd00d13003c5c07`. **ACCEPTED 2026-11-13** — source-contract PASS, Rustfmt PASS, native real-process App routing **1 passed, 0 failed (1.85s)**, separate source-pinned READY restore guard **1 passed, 0 failed (0.00s)**, zero skipped inside the native job. The job must separately enumerate and execute both (1) `codexdd_046_3c1b2j3b2b2a_cold_real_process_app_rejects_old_auto_handoff` (ignored-by-default, genuine Windows OS process restart plus old event negative test) and (2) `codexdd_046_3c1b2j3b2b2a_source_pinned_ready_cannot_reauthorize_after_cold_resume` (nonignored, exact persisted restoration guard). Source-contract and Rustfmt jobs remain hard prerequisites. Any failed/missing/skipped stage leaves b2b2a OPEN.

Still required after this packet: positive, App-owned automatic two-Worker native receipt sequence across the same OS-process restart, b2b2b. No release or install.

## Acceptance record

GitHub Actions [#43501797235](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43501797235) completed **SUCCESS** at immutable source SHA `59cecfa0ef8d772fb5858f330bd00d13003c5c07`. The source-contract and Rustfmt jobs were each green. The dedicated Windows job enumerated and actually ran exactly one ignored real OS app-server cold-App dispatcher test (1 passed / 0 failed, 1.85 seconds) plus exactly one separately filtered source-pinned READY quarantine guard (1 passed / 0 failed, 0.00 seconds). Prior red attempts remain classified, not promoted. No native READY was fabricated, no permission changed, no active project touched.

**Scope limit:** This is negative control only. 3C.1b.2j3b2b2b must still prove the positive App-owned real two-Worker lifecycle and evidence on one source-matched process-restart sequence. 3C.2 role/status diagnosis also remains mandatory before declaring Phase 3C complete. 0.4.6 release and install remain blocked.
