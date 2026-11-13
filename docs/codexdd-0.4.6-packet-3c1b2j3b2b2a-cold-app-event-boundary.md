# CodexDD 0.4.6 — 3C.1b.2j3b2b2a: real cold App event-buffer and stale automatic handoff refusal

**Opened:** 2026-11-13. **Parent:** 3C.1b.2j3b2b1 exact Windows run [#43475890208](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43475890208), native Core receipts 1/1 PASS on `3772a8e9fed0ff1ee71614fad883bd8ebe74b29f`. **Status:** WIP, test and gated native job submitted; not yet accepted. 3C.1b.2j3b2b2, 3C.1 and 0.4.6 remain OPEN.

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
