# CodexDD 0.4.6 — Packet 3B.2c: first-message Worker binding intake

**Source parent:** `c322a8b51648f61268a62db6d3844f3c8cc95277`, previous 3B.2b source-matched Windows CI green (#42268421089).  
**Status:** implementation checkpoint; no merge, install, release approval, or complete 3B.2 acceptance.

## Defect and repair
Source reconnaissance found `AdaptiveEffortState::observe_worker_assignment_text` was exercised by pure-state tests but **not called from live interactive submission**. Consequently the first user-assignment header was only a tested parser, not a dependable Worker binding in the real UI.

The interactive input submission now:
- Prevalidates the first unsent owner prompt's `[adaptive_worker]` header before model submission; a malformed header keeps its draft and cannot consume a model turn or lock the Worker.
- Records the trusted first-message decision only when submission succeeds. An absent header locks the thread unbound. Pinned launch-config authority still wins without parsing the prompt.
- Does not inspect a later prompt/queued answer/continuation as a new authority source; the first accepted message remains the only mutable slot.
- Persists the authoritative role, scope and optional read-only source candidate with the existing TUI workflow snapshot, without adding a new privilege route.

## Explicit remaining 3B.2 work and release holds
This patch **does not bridge a first-message source pin to Core**. Core's independent exact-SHA LVO handler currently recognizes startup-config pins; TUI first-message pins alone cannot establish native end-to-end candidate authority. Do not claim full LVO source provenance, release readiness, or permission to bypass this hold. 3B.2d must implement a secure first-turn Core authority binding and source SHA checkpoint transition (or a correctly typed `SETUP_BLOCKED` if the trusted Core bridge is unavailable), with matching executable tests.

GitHub branch CI has new `codexdd_046_3b2c_` TUI executable regressions and still requires Rustfmt, Core/TUI compilation, and prior source/role/profile tests. Frozen 0.4.5 remains untouched.
