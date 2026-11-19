# CodexDD 0.4.6 — 3B.2e shared first-message Worker grammar

**Branch:** dd/codexdd-v0.4.6-agent-reliability
**Prior green:** 3B.2d, commit 0a6ab790, GitHub Actions run #42293333399
**Status:** nonproduction source checkpoint awaiting GitHub validation; not a finished Core authority bridge.

## Defect addressed
The TUI and Core need to interpret user-supplied Worker headers identically. Previously the TUI owned a private header parser and Core only recognized startup-configured source pins. Copying the parser into Core independently could create contradictory interpretations.

## Implementation
- Added the pure parser codex_config::config_toml::parse_adaptive_worker_first_message. It requires a first-line [adaptive_worker] TOML table, blank separator, nonempty body, recognized role, nonempty scope, supported task_kind and exact root/SHA pairing; it rejects unknown fields and validates bracketed-paste framing.
- Replaced TUI private decoding with the same shared parser, keeping the existing immutable TUI context/preflight flow. Existing first-submission, pasted-header, malformed-header and late-rebind regressions still exercise the TUI implementation.
- Added two executable Core tests, including one that verifies parsing a valid user message does not itself grant native validation access. The existing 3B.2d fail-closed Core LVO guard remains authoritative.
- Updated exact-source GitHub regression workflow to compile Core/TUI and run the new Core filter.

## Remaining before 3B.2 completes
1. Atomically bind the accepted first user-origin header to Core thread settings before protected LVO invocation; ensure later user/model messages cannot change authority, and persist/resume the binding without trusting replayed prose.
2. Establish an explicit, auditable post-edit candidate SHA transition. The originally authorized starting SHA must not silently become any later HEAD.
3. Validate repo-owned Node/Rust PowerShell profiles on a real Windows worktree with independent Validator acceptance. Keep setup blockers out of test-failure repair.
4. Do not merge, release, install or change frozen 0.4.5 based on this checkpoint.

The parser alone is never an authorization to run tools or bypass setup blockers.
