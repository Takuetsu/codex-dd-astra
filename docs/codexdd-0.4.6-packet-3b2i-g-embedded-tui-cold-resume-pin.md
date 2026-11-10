# CodexDD 0.4.6 — Packet 3B.2i-g: embedded TUI source pin after Core cold resume

**Parent:** `d23e0ffc542f6ce4c6cef9ea16919e9225e033d4` on `dd/codexdd-v0.4.6-agent-reliability`.
**Type:** Source-only development packet awaiting focused formatting and Windows native acceptance. **Not a release, production installation, 0.4.5 thaw, or completed 3B.2 gate.**

## Baseline evidence and one outcome

- Focused real app-server process + bounded Git child tests passed at exact commit `7583e0012f16c6f06d2633908f3af7a594a71da5` in GitHub Actions [#42847772628](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42847772628). Two tests executed, 1/1 pass each. It proved Core can persist an owner-authorized candidate, notify clients, cold-restart a new app-server process, and accept a second approved SHA.
- Diagnostic warnings were removed and source formatting verified on `d23e0ffc542f6ce4c6cef9ea16919e9225e033d4` in [#42857074974](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42857074974).
- **Missing behavior:** when the TUI process also shuts down, the next `thread/resume` response previously did not carry Core's authoritative post-checkpoint Worker SHA. Separate persisted TUI workflow state could replay an older source pin with no corresponding new live notification.

**Outcome:** a resumed *same-thread embedded, non-forked* TUI may reconcile its already locked source pin to the exact durable Core Worker source SHA, but only for a clean verified descendant checkout with unchanged role, scope, task kind and source root. Unknown or incompatible state remains fail-closed rather than silently assigning authority.

## Implementation boundaries

1. Add optional `codexddFirstWorker` to `ThreadResumeResponse` with serde backward compatibility; no new client command or authorization argument. Both warm/listener and cold/recreated native app-server resume paths project `CodexThread::restorable_thread_settings().codexdd_first_worker`.
2. When mapping an embedded unforked resume, the TUI restores its own persisted workflow state first and invokes its existing `reconcile_core_candidate_preflight` with the Core resume snapshot. Never create an initial binding when no local Worker assignment exists. Do not reconcile remote clients, forks, wrong-scope, dirty, changed-root, invalid-SHA, or non-descendant candidates. Failed checks leave the old local pin unchanged.
3. The TUI preflight runs on a blocking thread with a 20-second **async** safety timeout, so startup does not indefinitely await a stalled Git command. This outer timeout alone does not terminate any underlying Git child; that existing TUI preflight subprocess limitation is recorded separately for later hardening. It does not elevate a failed check to acceptance.
4. Extend existing real-process app-server test `codexdd_046_3b2n_*` to assert that an actual *new-process* resume response contains the persisted, exact successor SHA, original role and scope before attempting the next candidate. Add `codexdd_046_3b2q_*` isolated TUI mapping regression with remote/fork and wrong-scope/dirty denial. Verify old app-server JSON without the new optional field deserializes unchanged.
5. The pre-existing opt-in `[3b2-focused-native]` GitHub workflow now executes the new narrow TUI regression in addition to bounded Git and process-restart tests. The broad Windows build remains `workflow_dispatch` only.

## Validation plan and exclusions

First run only source-contract and Rustfmt push-time gates, fix any formatting-only differences without triggering the heavy runner. After a green source checkpoint, deliberately trigger the existing focused Windows test with the explicit opt-in commit marker. Its test-specific green result is the requirement for this packet; skipped and unrun tests are not counted.

A green result will **not** prove a complete live TUI + app-server subprocess end-to-end after both programs restart; follow with an isolated native TUI integration smoke and independent Validator, wrong-thread replay/drift checks, approved Windows Node/Rust mechanical-validation profiles, and final daemon/F2/distribution/21-stage gates at owner-controlled release acceptance.

No live Breakwater/Cycle Vengeance worktree mutation; no new model privileges; no automatic promotion or Git HEAD inference; frozen 0.4.5 and installed 0.4.4 remain unchanged.
