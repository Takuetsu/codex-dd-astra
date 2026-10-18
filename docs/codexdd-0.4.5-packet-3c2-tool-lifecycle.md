# CodexDD 0.4.5 — Packet 3C.2: cancel-safe dynamic tools and adaptive direct-tool routing

**Date:** 2026-10-18  
**Status:** INCOMPLETE / NOT A RELEASE CANDIDATE. Two three-way Rust source merges and official upstream cancellation tests checkpointed. **11 static tests pending Daniel-CL validation; native Rust build and runtime checks pending source completion.**  
**Integration branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Reviewed three-way identities

| File | Tracked official 0.162.0 | CodexDD 0.4.4 | Official 0.163.0 | Merged or retained Git blob |
| --- | --- | --- | --- | --- |
| `codex-rs/core/src/tools/handlers/dynamic.rs` | `f658d6742b158fbc5abbf6a8e1a78dba890466e1` | `4f039ad37bf9dbd49c585dd5a98e859777fbb1c6` | `9e78325058546a3924b12fa8c9de938d6421fbcf` | `05353ec9dc97df37db5c85ed1b232d7c9eb5e6d1` |
| `codex-rs/core/src/tools/mod.rs` | `41ed0189b8bfd80d938a16073b2af5c40b18d2f9` | `4ddea6c560bbc0ff39c47abe2b4535169a4813ba` | `501ce5fe4eb8395ae84f8b7a1b7f14b530a22e5e` | `2a05e28de35348b79e02d80e4bdd8c3381b88279` |
| `codex-rs/core/src/tools/handlers/dynamic_tests.rs` (new upstream source) | absent | absent | `2d926b321d0347ebc051c5604244e21dee390cb3` | **exact official blob, unchanged** |

## Semantics preserved

**Dynamic tool cancellation:** The merged handler begins from official 0.163.0. It retains `OrCancelExt`, per-invocation `CancellationToken` forwarding to `request_dynamic_tool`, an explicit `finishes_on_cancellation()` implementation, cancel-before-wait behavior and the original upstream race fixture `response_cancel_race_completes_once`. CodexDD's 0.4.4 fork adds evidence binding `FunctionToolOutput::from_content(...).with_evidence_id(call_id)`. The reconciled call clones `call_id` before forwarding it so the original ID is still available to attach truthful tool evidence. This preserves both the cancellation and evidence contract.

**Adaptive direct-tool floor:** Upstream 0.163.0 removed the old `tool_user_shell_type` helper in `mod.rs`. The merged source retains that deletion and adds only the fork's `effective_tool_mode` overrides for controller-authorized reconnaissance, implementation and mechanical validation. Those turns continue to use built-in Direct mode even when the requested model defaults to CodeModeOnly. The production source comments explicitly state that this **does not grant native LVO execution** or override no-source-edit policy. Normal/terminalization turns retain their requested model tool mode.

**Non-interference:** The larger and more sensitive `codex-rs/core/src/tools/registry.rs` remains the exact **unchanged 0.4.4 fork blob** `5b67eb29406d04b021b0fd9a5e259eccdd7da3b2`; source-level registry, policy approval and sandbox differences require a separate reconciliation. No plugin permissions, GitHub Actions permissions or publication routes are changed by 3C.2.

## Tests and unresolved dependencies

`.github/scripts/test_codexdd_045_3c_tool_lifecycle.py` contains **11 offline source checks** of exact Git blob identity, evidence binding, call-ID ownership, cancellation propagation, single-result race fixture, authorized direct-only tool floor, intentional helper deletion, unchanged registry and release markers. This is not a cancellation runtime test: that requires a compiled 0.163.0 core and full native Rust dependencies.

**Outstanding:** integrate upstream `ToolInvocation`/executor APIs and registry lifecycle, affected core protocol and model schemas, and all remaining 109 overlap paths and 15 original conflict records with reviewed v2 evidence. The old Cargo.lock and 0.4.4 product/upstream markers remain unchanged. No `cargo --locked`, full build, 21-stage Windows validation, publication, CI release acceptance, PR or merge is authorized.

**Next cheap checkpoint:** run 3B.1's 12 and 3C.2's 11 Python source tests together on an exact detached Daniel-CL integration worktree, then continue work on the large registry and related security review in smaller source packets.
