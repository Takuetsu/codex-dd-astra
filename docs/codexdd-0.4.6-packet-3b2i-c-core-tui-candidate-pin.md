# CodexDD 0.4.6 — Packet 3B.2i-c embedded Core-to-TUI candidate pin reconciliation

**Parent:** `e336cbdf9ce0f1a9e8e48071df68a1c56a3bfe9c`, exact-head GitHub Actions #42647591313, all source/Rustfmt/Windows Core and TUI checks green.  
**Branch:** `dd/codexdd-v0.4.6-agent-reliability`.  
**Status:** nonproduction source checkpoint, pending exact-head formatting, compile, app-server translation and focused TUI regressions.

## Closed gap under test
The previously implemented Core candidate reauthorization durably appends and flushes the new exact source SHA before promoting the in-memory first-user Worker authority. TUI still held the original first-assignment SHA, so mechanical preflight could block even after a legitimate owner-authorized new commit. This packet sends a Core-owned `ThreadSettingsApplied` event **only after the successful native flush and in-memory candidate promotion**; this raw event is an acknowledgement, not a second approval or substitute for the persisted native record.

The local app-server projects the same-thread Core `codexdd_first_worker` snapshot into an optional `ThreadSettingsUpdatedNotification` field. A source-only transition causes an update even when ordinary model/settings fields have not changed. Unrelated or wrong-thread snapshots cannot be projected as current authority. This is a narrow additive optional protocol field; no new public command, privilege, model authorization, sandbox bypass, or automatic Git HEAD adoption.

The interactive TUI ignores candidate synchronization for remote transports, externally routed operations, nonmatching thread IDs, unbound/unlocked Workers and changed role, scope, task kind or root. A candidate is accepted only after a read-only Git preflight confirms that the *current native runtime root* is exactly the approved clean worktree at the Core-specified HEAD and the previously pinned SHA is an ancestor of it. A rejected or stale notification leaves the TUI binding untouched and cannot produce a native PASS. The independently guarded Core LVO must still verify selected target, role, clean checkout and exact authorized SHA.

## Targeted checks
- Existing Core 3B.2i four-case candidate authorization tests and three 3B.2j turn-intake tests remain.
- Two new `codexdd_046_3b2k_*` TUI tests cover positive matching local descendant with only the SHA changed, and rejection of wrong scope, role, root, invalid SHA, unbound Worker and missing checkout.
- The scoped GitHub workflow now compiles `codex-app-server --lib --tests` in addition to Core/TUI and runs the new TUI tests.
- Exact local Windows Node/Rust profile and actual daemon cold-resume acceptance are **still pending**. Unit fixture correctness is not proof of a real app-server notification round-trip. The TUI's owner milestone-root launch convention remains unchanged; a mismatched root must block, not silently switch worktrees.

## Invariants and next packet
- No source promotion from prose, model-generated tool arguments, future Git HEAD or remote messages.
- Core first assignment still requires a durable native checkpoint; all later candidate updates require a separate direct-owner idle-turn request and verified descendant commit.
- During interrupted acknowledgement, Core's durable same-thread snapshot controls cold resume; TUI may stay conservatively blocked pending explicit synchronization, not manufacture READY.
- Next: exact-head compile/rustfmt and targeted regression repair, then a true embedded app-server notification/Windows native profile and same-thread cold resume integration smoke. Phase 3B.3 scoped gate evidence and terminal prerequisites follow only after 3B.2 closure.

No frozen 0.4.5 branch activity, 0.4.4 production changes, merge, release, version bump, installation or live BREAKWATER/CV changes.
