# CodexDD 0.4.5 — 3C.9d3 startup orchestration reconnaissance

**Date:** 2026-10-18  
**Status:** RECONNAISSANCE ONLY — source merge not accepted; NOT A RELEASE CANDIDATE.

## Exact three-way inputs

- Production CodexDD 0.4.4 commit: `b25550dcb2c6e84a84074b603086c9db6f32a63b`
- Fork `codex-rs/tui/src/startup_orchestration.rs` blob: `e03aa30b4723717cfc72935d118e074fbf4f2aae`.
- Official `rust-v0.162.0` file blob: `87c1c42f54d99c2844a9f5939659206948edc7ce`.
- Official `rust-v0.163.0` file blob: `c29d464adcb2f52a7a6dc38329e19319217cc3ad`.
- Integration branch file remains byte-identical to production: **not yet upstream-reconciled**.

## Identified protected fork behaviors

1. Elevated or unverifiable Windows elevation token must select safe embedded mode without connecting to a shared daemon. For approved implicit elevated fallback the spurious F2 warning is suppressed; other failure reasons remain observable.
2. Adaptive CLI startup forces the initial model through the adaptive-family selection only when explicitly requested; ordinary startup model selection remains unaffected.
3. Automatic daemon-launch rejection uses `windows_automatic_fallback_reason`, retaining a meaningful reason and protective fallback rather than silently dropping any startup error.
4. The daemon warning and diagnostic reason classifications must remain consistent with the 3C.9d2 daemon-startup helper and its native tests.

## 0.163.0 changes requiring explicit reconciliation

- Upstream retains an earlier elevated-warning/`cli.no_daemon` pathway, which must **not** replace the protected elevation-probe fallback unreviewed.
- Upstream adds `cli.managed_daemon` provenance to startup state; the fork initializes `managed_daemon` to `false`. Determine the correct source-of-truth and downstream launch/telemetry semantics before selecting either value.
- Upstream changes one daemon-launch call argument set (includes `&cli_kv_overrides`), and adds `managed_daemon` to subsequent startup handoff. Preserve new signature and test behavior instead of blindly replaying fork code.
- Upstream removes the fork's `main_execve_wrapper_exe` initialization in the corresponding runtime-options structure; verify the struct/callsite signatures before merging.
- Review all changes to warnings, fallback paths, and command-line model initialization as interacting changes rather than isolated textual edits.

## Acceptance conditions for implementation packet

- Produce a reviewed, bounded three-way merge with explicit treatment of every fork-only hunk and each relevant 0.163.0 signature change.
- Keep ordinary startup, elevated token failure, manual no-daemon, managed daemon, explicit remote, and automatic fallback independent and fail-closed.
- Add/adjust targeted regression coverage; do not treat Python static source checks as native Rust execution.
- Reconcile any companion startup callsites and TUI test fixtures before claiming compile readiness.
- Do not advance `Cargo.lock`, release markers, publisher automation, generated schema or long Windows suite in this reconnaissance packet.
- Next step: implementation review of 3C.9d3 and only then focused, cheap validations.

## Validation evidence

Daniel-CL reported 3C.7a 10/10 static PASS; 3C.8a–3C.9d2 visible packets 91/91 static PASS. These do not establish native compile or runtime safety.
