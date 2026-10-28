# CodexDD 0.4.5 — 3C.9d3 startup orchestration source merge

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Reviewed source merge, not native Rust compilation or release acceptance. Static validation is not yet executed for 3C.9d3.

## Reviewed three-way provenance

- Production CodexDD 0.4.4: `b25550dcb2c6e84a84074b603086c9db6f32a63b`.
- Production fork source blob: `e03aa30b4723717cfc72935d118e074fbf4f2aae`.
- Official upstream `rust-v0.162.0` source blob: `87c1c42f54d99c2844a9f5939659206948edc7ce`.
- Official upstream `rust-v0.163.0` source blob: `c29d464adcb2f52a7a6dc38329e19319217cc3ad`.
- Reviewed integration source blob: `06ea648374cbdcc3ae7276ba146f4dbe45da3118`.
- Path: `codex-rs/tui/src/startup_orchestration.rs`.

The official 0.162.0 -> 0.163.0 comparison contains four bounded changes; the production fork added eleven nonoverlapping protected hunks. The staged merge preserves the production fork verbatim except for three 0.163.0 adaptations:

1. Remove the deleted `ConfigOverrides.main_execve_wrapper_exe` initializer.
2. Initialize `managed_daemon` from the top-level `cli.managed_daemon` provenance instead of `false`.
3. Pass `&cli_kv_overrides` to the new `daemon_recovery::check` signature.

These changes preserve fail-closed Windows elevation probing, deliberate embedded fallback, absence of the spurious elevated-session F2 warning, explicit remote/agents exceptions, adaptive startup model selection, fallback diagnostics and daemon telemetry reason categories. The helper implementation and associated native tests remain those already reviewed in packet 3C.9d2.

## Deferred adjacent upstream change — explicit boundary

Official 0.163.0 also forwards `managed_daemon` to `run_ratatui_app` and thence to TUI application startup. The integration branch currently retains the production protected `tui/src/lib.rs` run-function signature, which does **not** accept the new parameter. Adding only the call-site argument here would break compilation. Therefore the 3C.9d3 startup call site intentionally keeps its existing argument list; the provenance is applied immediately to `daemon_recovery::check` where the 0.163.0 signature is already staged.

**Next companion packet:** review the protected `tui/src/lib.rs`, TUI app startup call sites, and `managed_daemon` lifecycle together before adding the deferred argument. Do not claim the entire TUI 0.163.0 integration complete.

## Source guards and remaining gates

- `.github/scripts/test_codexdd_045_3c9c_tui_upstream.py` now pins this reviewed fifth protected TUI merge. Of the original 44 protected fork TUI paths, **39 remain without reviewed merges** at this checkpoint.
- `.github/scripts/test_codexdd_045_3c9d3_startup_orchestration.py` checks the exact production-to-integrated transformation, elevated-session fallback, adaptive selection, new signature compatibility, deferred call-site boundary and unchanged release controls.
- Prior Daniel-CL static results: 3C.7a 10/10 PASS; eight visible 3C.8–3C.9d2 packets 91/91 PASS. These were _source-only_ checks, not native Rust compilation.
- 3C.9d3 static suite, native Rust compile/tests, Cargo.lock reconciliation, schema generation, full 21-stage Windows suite and GitHub CI remain outstanding.
- Publisher stays disabled. Product version markers remain CodexDD 0.4.4 and upstream `rust-v0.162.0`. No release or deployment is authorized.
