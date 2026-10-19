# CodexDD 0.4.5 — 3C.9d4 managed-daemon TUI handoff

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Reviewed source-only implementation and regression guards; native Rust compilation and runtime checks not yet performed.

## Intent and bounded changes

Official `rust-v0.163.0` conveys local managed-daemon status through the TUI call chain so the user interface can distinguish a Codex-managed local daemon from a shared unmanaged service. CodexDD 0.4.4 protects adaptive startup and Windows elevated-session fallback across the same call sites. This packet ports only that provenance chain without replacing protected source wholesale.

Reviewed changes:

1. `codex-rs/tui/src/startup_orchestration.rs` forwards the `managed_daemon` bool into `run_ratatui_app` immediately after `daemon_startup_warning`. The bool was obtained from `cli.managed_daemon` and can be updated from verified daemon-start backend ownership. The fail-closed Windows elevated-token and daemon-launch fallback are unchanged.
2. `codex-rs/tui/src/lib.rs` accepts the bool and passes it to `App::run` immediately after `daemon_startup_warning`, **before** the protected CodexDD adaptive-family argument. The original adaptive module wiring and session startup policy remain untouched.
3. `codex-rs/tui/src/app/startup.rs` accepts the bool and sets `remote_connection.is_managed_daemon = connection.is_local_daemon && managed_daemon` only for a locally classified daemon. Embedded connections have no remote-connection status; explicit remote connections cannot be promoted to managed local daemon status by this provenance bit.

## Exact provenance

| Source | Original CodexDD fork blob | Reviewed integration blob |
|---|---|---|
| `tui/src/startup_orchestration.rs` | `e03aa30b4723717cfc72935d118e074fbf4f2aae` | `6b3bf4a9cf3d639107126b29992690b19bf4c932` |
| `tui/src/lib.rs` | `83ad7124e531be410477bd097cde8950bca40ed2` | `a40147cca1333114d82e0335521279254f5c1f21` |
| `tui/src/app/startup.rs` | `86f1afec7ce715799c9fa62859e53f312918d067` | `3dd4fd65185f407e01f41df7544a97f57f9048fa` |

The production reference for the original CodexDD forks is `b25550dcb2c6e84a84074b603086c9db6f32a63b`. The 3C.9d3 source-only baseline for startup-orchestration itself is `06ea648374cbdcc3ae7276ba146f4dbe45da3118` (prior to adding its managed-daemon argument). The source-provenance guard `.github/scripts/test_codexdd_045_3c9c_tui_upstream.py` now records the seventh reviewed exception among the original 44 protected TUI paths: **37 remain unresolved**.

## Validation work and remaining risk

- `.github/scripts/test_codexdd_045_3c9d4_managed_daemon.py` validates all three exact source transformations, app call-chain alignment, preservation of adaptive/Worker startup and Windows fallback, strict local-daemon UI classification, and unchanged release controls.
- The 3C.9d2 guard was reconciled to assert elevated fallback preservation in the now-reviewed orchestration rather than requiring that file to be byte-identical to production.
- The 3C.9d3 guard was carried forward to preserve its complete provenance reconstruction across the new 3C.9d4 handoff.
- The 3C.9c 44-path source inventory now recognizes the three new reviewed blob pins; historical checkpoint documents remain dated snapshots, not release acceptance.

**Source guard execution still pending**. The screenshots from Daniel-CL established 3C.7a 10/10 static PASS and eight visible 3C.8–3C.9d2 packets 91/91 static PASS *before* this integration. They do not validate 3C.9d3/3C.9d4 code, native Rust compilation or live Windows elevated-startup behavior.

Broader 0.163.0 TUI differences (GovCloud guidance, modal-input quarantine, reconnect, daemon restart), **Cargo.lock**, schema generation, native app/TUI tests, the 21-stage Windows release suite and GitHub CI remain outstanding. Automatic publishing remains disabled; product markers stay CodexDD `0.4.4` / upstream `rust-v0.162.0` until formal release acceptance.
