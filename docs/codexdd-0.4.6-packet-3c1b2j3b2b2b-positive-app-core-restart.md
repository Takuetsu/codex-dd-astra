# CodexDD 0.4.6 — 3C.1b.2j3b2b2b: positive App-owned two-Worker native restart acceptance

**Opened:** 2026-11-13. **ACCEPTED:** 2026-11-14 via [GitHub Actions #43603946333](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43603946333), immutable tested source `c1e5be2dce8f35e234f5f91442b22239f59cffa2` — native Windows real-process positive lifecycle **1/1 passed, 0 failed, 0 ignored**. Parent: 3C.1b.2j3b2b2a accepted via exact source SHA `59cecfa0ef8d772fb5858f330bd00d13003c5c07` ([Actions #43501797235](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43501797235), both native tests green).

## Required positive integration

The test `codexdd_046_3c1b2j3b2b2b_real_app_owned_native_validator_after_restart` is Windows-only and ignored outside its dedicated real-app-server CI. It creates two genuine disposable source-pinned native PowerShell profiles and commits them to one Git SHA. The locally started deterministic mock Responses backend returns only scripted tool selections, **not tool success**, terminal authority or evidence. Every native `run_codexdd_validation(work_packet)` and `run_codexdd_independent_validation(independent_review)` result must be emitted by the actual Core process and contain completed PASS, correct source SHA, distinct nonempty run IDs, matching profile and worker thread.

1. Build an independent `codex-app-server.exe`, launch a real WebSocket child and start the source Worker from the actual first user-origin `[adaptive_worker]` assignment.
2. Deliver app-server native user/message, item/completed, and adaptive terminal events through `App::handle_app_server_event`, the App foreground buffer, and `App::handle_thread_event_now` (never direct ChatWidget event injection).
3. At an already-authorized MechanicalValidation floor, run the real source work*packet and require source gate PASS while independent gate remains pending. Require the \_App* to queue exactly one `StartAutomaticValidation` for that origin thread/SHA.
4. Tear down the source OS child and launch a distinct real app-server OS process. Deliver the queued authorized automatic handoff through `App::handle_event`, which must create a distinct Core Validation thread and generate exactly one first-user independent assignment with the original Git root.
5. Drive the generated initial Validator assignment through the normal typed `AppServerSession::turn_start`, then the genuine independent_review built-in native tool and adaptive signal. App-owned READY requires original source and independent receipts on distinct Core threads and only one `PersistWorkflowState(SetReadyForOwnerQa)`.
6. Execute that App-owned READY persistence with the real app-server and shut down the child. No fabricated source PASS, assigned Validator ID, owner gate or READY proof is allowed anywhere in the fixture.

The fixture explicitly enters the previously accepted `MechanicalValidation` phase after a real owner-origin first turn; that setup does not replace preceding implementation/complexity-policy tests. A green result proves only this bounded positive seam; it is not broad real-agent/project usability or release acceptance.

## Exact-source acceptance criteria

Source-contract PASS, Rustfmt PASS and a dedicated isolated native Windows job compiling the real app-server, enumerating **exactly one** named ignored test, executing it without skip, and observing **1 passed, 0 failed**. Failure to compile, tool absence, protocol rejection, wrong tool/profile/SHA, missing/duplicate READY, process leak or wrong worker thread is a real failure to classify. Never weaken source pin, role/approval policy or native receipts to turn CI green. The active BREAKWATER/CV/0.4.5/installed CodexDD programs remain untouched.

After this acceptance, 3C.1 subpacket audit remains required and separate 3C.2 role/status tests must pass before claiming Phase 3C complete. 3D/3E/3F/4, owner release authorization and the roughly one-week BREAKWATER soak are later distinct gates.

## Acceptance record — 2026-11-14

[GitHub Actions #43603946333](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43603946333), immutable tested source `c1e5be2dce8f35e234f5f91442b22239f59cffa2` succeeded after the exact source-contract preflight and Rustfmt checks. The dedicated Windows job built the real `codex-app-server.exe`, enumerated the single ignored-positive test, ran it with `--ignored`, and reported **1 passed / 0 failed / 0 ignored**; no enumeration-only or prior-source proof was counted. The test asserts actual first-user source authority, distinct native Core source and Validator receipts, matching Git source root and SHA, a separate OS child after restart, an App-owned fresh independent Validator, and a single READY persistence operation. The deterministic mock selects tools only; it cannot mint native PASS or READY.

The earlier Windows attempt [#43600000652](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43600000652) failed at the newly created Validator Git root, not due to successful end-to-end behavior. Its same-host WebSocket test transport used remote-workspace semantics, which intentionally omit client local CWD/pinned Core override. Test-only helper `AppServerSession::with_same_host_daemon_params_for_test` now models a real same-machine **local daemon** without altering production remote-workspace authority isolation. The accepted build includes the exact Rustfmt correction to this fixture.

The same source also passed five 3C.2 status tests, two cold controls, two stale/duplicate replay refusals and one selected-thread native routing refusal: eleven named native tests total. This **closes the positive 3C.1b.2j3b2b2b packet**, not product release, broad real-agent validation, or later owner QA.
