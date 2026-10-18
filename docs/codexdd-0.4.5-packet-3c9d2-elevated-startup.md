# CodexDD 0.4.5 — Packet 3C.9d2: elevated-Windows daemon fallback preservation

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Two reviewed 0.163.0 TUI source merges, eleven **source-only** static tests authored but unrun. Not **native Rust** compilation, a Windows runtime receipt, or release approval.

## Exact fork preservation

The tracked official rust-v0.162.0 and production CodexDD 0.4.4 sources differ by exactly one self-contained 49-line insertion in `tui/src/daemon_startup.rs` and one 175-line native regression block in `tui/src/daemon_startup_tests.rs`. Both original fork sources were reconstructed byte-for-byte before inserting precisely those blocks at unique anchors in pinned official rust-v0.163.0.

The helper retains `elevated_windows_implicit_exclusion`, distinct classifications for elevated and unverified Windows token state, intentionally warning-free embedded fallback for an approved automatic elevated startup, and continued warning for other excluded launchers. The native regression block tests explicit transport, elevation-probe failure, and F2-warning behavior. The complete upstream 0.163.0 daemon capability/managed-configuration handshake remains unchanged.

The two original and reviewed Git blobs are pinned by `.github/scripts/test_codexdd_045_3c9d2_elevated_startup.py`. The original 44 fork TUI paths now have four reviewed merge exceptions (3C.9d1 and d2), leaving **40 unresolved**. The significant `tui/src/startup_orchestration.rs` and its call sites are deliberately unchanged and require separate three-way analysis; these static source checks do not prove runtime elevation behavior.

## Validation and release gates

3C.4–3C.6 cheap static checks passed 106/106 on Daniel-CL. New 3C.7–3C.9 static checks have not yet run. The connected native Windows F2 regression, shell/daemon transport, user approval policy, TUI state and transient history require integrated compilation and focused runtime tests before acceptance.

**Cargo.lock**, generated schemas, native Rust tests, the 21-stage Windows release suite, GitHub CI and final release provenance remain outstanding. Automatic upstream publishing remains disabled. Product markers remain 0.4.4 and rust-v0.162.0.
