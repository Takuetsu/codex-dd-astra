# CodexDD 0.4.6 — 3B.2h durable Core-to-native-turn binding

**Parent:** `b3b801165afd1010f7d07abe620ee50be93bdd54` (3B.2g exact-head Windows CI #42318301892 green).
**Branch:** `dd/codexdd-v0.4.6-agent-reliability`.
**Status:** nonproduction source checkpoint; requires new exact-head CI before validation.

## Behavior
- The accepted first direct user-origin Worker assignment still latches in Core and enters a typed native thread-settings snapshot. Now the checkpoint method distinguishes an actual successful thread-store append + flush from no live recorder: only a confirmed flush marks the authority eligible for a future native turn.
- Future **full** Core TurnContexts receive the immutable first-message Worker binding when it is checkpointed (or restored from an exact-owner native snapshot) and no launcher-config Worker authority is present. Other source modes and pre-accept/draft/current-first-turn contexts do not gain that authority. A cloned turn Config is never used to persist or override the original launcher config.
- Native LVO still independently enforces Implementation-only role, canonical selected root, clean Git worktree, and exact authorized SHA. Unpinned source changes, absent profiles, mismatched SHAs and unrelated worktrees remain `SETUP_BLOCKED`, not repair authorization.
- Three focused executable Core regressions cover unflushed header rejection, plain/launcher authority precedence, and the real next-turn config receiving an explicitly checkpointed first-message pin.

## Safety limits
- First user turn contexts are constructed **before** their first-message binding is committed. This packet deliberately does not retroactively authorize that context. Additional end-to-end proof is needed for direct native invocations during that turn.
- A source edit that changes HEAD cannot run protected validation until a separate authorized candidate-SHA handoff is implemented (next 3B.2 packet). Never silently substitute HEAD for the initial SHA.
- Snapshot provenance relies on Core's same-thread native rollout record and ID checks. It is not a cryptographically authenticated proof of a filesystem under hostile local modification.
- Real Windows Node/Rust profile execution and cold-resume acceptance remain pending. No merge, release, install or 0.4.5 branch change.
