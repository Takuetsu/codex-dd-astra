# CodexDD 0.4.5 — Packet 3C.6a: child-agent workflow-state ownership

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Eleven pinned official upstream 0.163.0 agent sources and one three-way child-spawn merge staged. Eleven static Python tests **11/11 PASSED** on Daniel-CL at the 3C.6a checkpoint (operator screenshot, 2026-10-18). This is a **source-only** checkpoint, not native Rust validation or a release gate.  
**Branch:** `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.

## Exact provenance

Official 0.162.0 baseline `c1382380de69521303b416720a52f42d51af6248`; production CodexDD 0.4.4 `b25550dcb2c6e84a84074b603086c9db6f32a63b`; official 0.163.0 target `83086188ae0614bd882818ff1aa23f3194962012`. The eleven source/test paths under `codex-rs/core/src/agent/` other than `control/spawn.rs` were byte-identical in the official 0.162.0 baseline and production 0.4.4, so each is staged as its pinned official 0.163.0 blob. All identities are recorded in `.github/scripts/test_codexdd_045_3c6a_agent_workflow.py`.

The production `agent/control/spawn.rs` added exactly two small independent `RolloutItem::WorkflowState(_)` match cases relative to official 0.162.0: retain the workflow item when preserving the child's inherited context baseline, and keep it in the child-history branch that also retains inter-agent communication metadata. Reapplying only those original two changes reconstructs the entire 0.4.4 fork version byte-for-byte before applying them to official 0.163.0. This packet preserves official 0.163.0 spawning and residency changes while retaining the fork's two workflow-state cases. The merged blob is recorded in Git; native behavior is untested.

## Security and remaining gates

The existing session replay `WorkflowState` match arms, CodexDD trusted adaptive signals, and 3C.3 pre/post-hook authorization constraints remain unchanged. Inheriting a workflow event does not grant child execution permission or allow editing before complexity-floor reconnaissance acceptance.

The 3C.4 checkpoint passed 33/33 static tests; 3C.5 passed 39/39 static tests on Daniel-CL, per operator screenshots 2026-10-18. 3C.6a **11/11 static tests passed**; 3C.6b **11/11 passed in 0.964s** and 3C.6c **12/12 passed in 1.763s** on detached commit `0a545888884c1c7c6d27aa882494d30fd6ff9032`. Total 3C.6 static checkpoint **34/34 PASSED**. Rust compilation, native child-agent tests, permissions, generated schema validation, reconciled **Cargo.lock**, 21-stage Windows release suite, and GitHub CI acceptance remain pending; the upstream automation publisher is hard-disabled.

**Next:** remaining core tool- and agent-consumer API changes and fork overlaps, then one cheap grouped static checkpoint. Do not publish, merge, bump versions or distribute artifacts at this stage.
