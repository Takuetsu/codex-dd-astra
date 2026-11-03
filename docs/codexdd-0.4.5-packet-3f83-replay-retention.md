# CodexDD 0.4.5 — 3F.83 replay-retention pressure (NONRELEASE)

Base: 3F.82 owner review checkpoint at `38800f17744cc600d31b308e17a22ff2462ed013`.
Candidate branch: `dd/codexdd-v0.4.5-3f83-replay-byte-pressure`.

## Proposed source behavior

- Limit serialized retained **replay event payload** to 8 MiB **per thread** in addition to the existing 32,768-entry maximum, 256 KiB agent delta maximum, and separate bounded live channel / 2 MiB per-event ingestion limit. This is not a total-process bound for all threads, session turns or restored voice state.
- Preserve unresolved exec/file approvals, permissions prompts, request-user-input and MCP elicitation requests during count, delta and byte-pressure eviction. Evict older noninteractive entries first, retaining the relative order of survivors.
- Before recording a new pending interactive request, simulate the possible safe evictions. If all remaining entries are protected and the request cannot fit, return an explicit error to app-server ingestion **before** changing pending approval state. Never silently mark an approval resolved just to meet a memory budget.
- Recount serialized retained bytes after session refresh strips already-replayed text. Keep realtime/raw private audio exclusions unchanged; do not log content.

## Added native regression candidates

- Aggregate 8 MiB replay pressure with a pending approval and repeated ~1 MiB agent messages.
- Multiple ~1 MiB pending approvals fail closed rather than silently dropping authorization; releasing one approval permits a new request.
- Refresh/rebase recalculates retained bytes after preserving a live question.
- Existing count/256 KiB delta eviction tests updated to require the still-pending approval to survive.

## Validation and release gates

**Source candidate only:** no native Windows compile/test result is claimed for 3F.83. The existing 3F.81 five authorization tests and earlier 36 TUI selectors were green on earlier source; those receipts do not validate this new replay change. Cheap source/Rustfmt/test compile on Daniel-CL must be green before any expensive CI. Then run focused new replay selectors and full owner-review negatives.

13 high-risk Core/TUI holds remain OPEN (4 Core + 9 TUI), 0/109 formal owner signoffs. No merge into production, signing, distribution, 21-stage release approval or installation. Production remains 0.4.4.
