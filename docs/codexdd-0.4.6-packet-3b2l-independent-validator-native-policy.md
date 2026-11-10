# CodexDD 0.4.6 — Packet 3B.2l: native negative-policy proof for independent Validator

**Branch:** `dd/codexdd-v0.4.6-agent-reliability`.
**Starting evidence checkpoint:** `6aed56034d76beb46ab65087212fa874ddb2afc5`.
**State:** source audit complete; new exact-source Windows focused policy proof **PENDING**.
**Scope:** Nonrelease, read-only test of existing Core negative-policy controls. No Worker permission change, product source edit, installation, production merge or live game-worktree access.

## Basis

- Packet 3B.2j exact-source native Windows [#42890342163](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42890342163) passed 2/2 source-binding and LVO Git deadline tests at `80a20985`.
- Packet 3B.2k native Windows [#42900729499](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42900729499) passed actual unmodified repository-owned Rust `targeted` profile at `14380e844f8ff31b00e937ca8457f20a27bb4c76`: `diff-check`, 21/21 Core adaptive tests and 219/219 TUI adaptive tests all PASS; prior detached-fork stack overflow passed on a 16 MiB test stack. Three TUI tests were reported `leaky` by nextest; monitor during later broad acceptance.
- The native Rust profile is **not** a live independent Validator handoff or direct Core tool invocation. Earlier Core tests exist for denied Validation/Repair roles, pinned versus unpinned source policy, no-Git-root `SETUP_BLOCKED`, nonrepairable error receipt and terminalization-only trusted reporting; execution on this exact current candidate is still unproven.
- BREAKWATER's private Node repo-owned profiles are available but **not native accepted**. Never copy private project source into public CodexDD CI, and do not modify its active game branch solely to manufacture a green Node receipt.

## Exact native Windows negative-gate validation

Add a separate commit-message-opt-in Windows job to the existing 0.4.6 development-only workflow. Gate it behind same-source contract+Rustfmt. With repository-pinned Rust 1.95 and verified Windows V8, run the following **existing** `codex-core --lib` test selectors individually and require exactly one listed + one native executed PASS per selector:

1. `codexdd_046_3b2_native_role_gate_rejects_independent_workers` — Validation and Repair cannot enter Implementation-only LVO; unspecified/partial pins block.
2. `codexdd_046_3b2d_protected_mechanical_requires_core_bound_source` — first protected call without trusted Core source cannot run the profile.
3. `codexdd_046_3b2d_unbound_adaptive_receipt_cannot_authorize_repair` — typed setup error has no repair fingerprint/failed-stage pressure.
4. `codexdd_046_3b2_native_preflight_without_git_returns_nonrepairable_setup_blocker` — missing Git identity/profile produces a non-PASS, nonrepairable receipt.
5. `adaptive_validation_terminalization_allows_only_trusted_signal_tool` — independently gated terminalization denies execution tools.
6. `codexdd_046_3b2_mechanical_node_rust_commands_remain_bounded` — read-only/build checks allowed, unsafe mutation and profile impersonation denied.

**STOP rule:** If any selector is absent, compiled only, or fails, keep packet open; report exact failed test and don't declare a successful native handoff. Do not re-run broader 21-stage Windows CI to fix a narrow negative-policy test.

## What remains after this scoped gate

Even if all six negative tests pass, a true independent Validator **running a separate live session**, receiving owner-controlled source-specific native evidence and reporting terminalization, remains **not proven**. Still outstanding: actual native Node profile in its isolated authorized repo; direct Core LVO tool-call source-bound happy-path; separate TUI/app-server cold restart; 3B.3 durable gate provenance; 3B.4 progress/refusal taxonomy; 3C/3E/4 comprehensive and owner-controlled acceptance. Do not release/install or claim 0.4.6 usable from unit-policy PASS.
