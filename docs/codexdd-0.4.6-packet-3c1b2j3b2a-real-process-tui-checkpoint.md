# CodexDD 0.4.6 — 3C.1b.2j3b2a: real OS process TUI checkpoint

**Opened:** 2026-11-13. **Parent:** 3C.1b.2j3b1 accepted at `4d6dc84ecd33b8cb8528b2acea8d7b0c01254674`, [GitHub Actions #43457407670](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43457407670) (Windows 1/1 PASS). **State:** ACCEPTED focused Windows native process/restart checkpoint at `4bbda6aab623c4fbf29196af19010e300684b222` ([Actions #43468082893](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43468082893), SUCCESS, 2026-11-13). The full 3C.1b.2j3, 3C.1 and 0.4.6 release remain OPEN. The full 3C.1b.2j3, 3C.1 and 0.4.6 release remain OPEN.

## Distinct process acceptance boundary

Previous j3b1 used separate **embedded instances within the same OS process**. This packet builds the real, source-matched `codex-app-server.exe` and launches it using a bounded Windows child-process harness. The TUI connects through its actual WebSocket remote client and `AppServerSession` mapper, not direct `ChatWidget` injection or the embedded client. The child process is killed/reaped and restarted for each stage; the test fails if the executable environment variable is absent. It is an ignored-by-default TUI library test, activated only in the dedicated Windows job.

The fixture is a disposable Git checkout, pinned SHA, same-host temporary CODEX_HOME and saved Validator rollout. A first app-server process writes a typed previously terminalized Validator checkpoint to `thread/workflowState/update`. A **second operating-system process** resumes that same thread: historical READY/PASS are not fresh authority; old run evidence and queued successor must be absent, and original source/gate prerequisites remain pending. The process is replaced a third time after an unrelated Git commit and again after deleting the worktree; neither may silently confer validation authority.

This test intentionally verifies TUI's **remote-transport same-host** session mapping, not the all-native first-user/direct Worker assignment, automatic Validator creation, or a genuine new Core validation receipt in the reconnected App event loop. The latter are still required as j3b2b. Remote Core first-user pins must not bypass the existing embedded-only reconciliation rule.

## Isolation, stop/go and documentation

Owner test: `codex-rs/tui/src/app_server_session/real_process_restart_tests.rs`. The isolated `[3c1b2j3b2a-native]` workflow builds `codex-app-server.exe` at the pinned development SHA and executes exactly one ignored-by-default `codex-tui --lib` test on Windows. Explicit startup, connect and process-reaping bounds; source preflight/Rustfmt gates first. All test files and process instances remain disposable and must not change live installations. Any source, OS process, owner-identity or READY assertion failure is blocking until classified. Never relax native-proof requirements to make an assertion green.

**Following acceptance:** 3C.1b.2j3b2b must couple actual Core native source/Validator receipts and App user-origin/Validator lifecycle across the process restart, including duplicate READY and stale/replayed receipt refusals. Release and BREAKWATER soak remain blocked.

## Accepted Windows proof and strict boundary

At exact checkpoint `4bbda6aab623c4fbf29196af19010e300684b222`, GitHub Actions [#43468082893](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43468082893) completed **SUCCESS**: source-contract preflight PASS, Rustfmt PASS and dedicated native Windows OS-process regression **1 passed, 0 failed; 5916 filtered out, 3.18s test runtime**. The test launched a real `codex-app-server.exe` child, persisted a checkpoint, then resumed it across fresh OS processes, source HEAD drift and removed-root refusal. Historical READY, old receipts, unbound successors and unauthorized repinning remained disallowed.

The immediately preceding red [#43461857927](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43461857927) compiled but the child exited with code 2 before its WebSocket could connect. The discriminating source inspection showed the fixture used `ws://127.0.0.1:PORT/` even though the production `--listen` parser accepts only `ws://IP:PORT` without the trailing slash. The correction changed the **test URL only**, not the parser, policy, transport or READY logic.

**Do not extrapolate this green result:** the original saved state is a typed test fixture, not real Core-produced native Validator evidence; there is no full first-user source -> automatically created Validation Worker -> fresh native receipt -> App-owned READY across restart. That specific coupled acceptance belongs to 3C.1b.2j3b2b, and release/installation/soak remain prohibited until later gates.
