# CodexDD 0.4.5 — Packet 3C.9c: upstream-owned TUI source staging

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** 200 exact official rust-v0.163.0 TUI source/test files staged, with **44 unresolved fork-sensitive TUI files deliberately unchanged**. Twelve new **source-only** static checks authored but unrun, not **native Rust** compilation or Windows TUI acceptance.

## Staged vs. protected source ownership

Compared the full `codex-rs/tui` Rust source tree across official rust-v0.162.0 `c1382380de69521303b416720a52f42d51af6248`, production CodexDD 0.4.4 `b25550dcb2c6e84a84074b603086c9db6f32a63b`, pinned official rust-v0.163.0 `83086188ae0614bd882818ff1aa23f3194962012`, and the integration head before this packet.

Of 244 upstream-changed TUI Rust paths, **200** had identical baseline and production 0.4.4 source blobs or were newly introduced only in upstream, and had not yet been changed in the integration branch. Those 200 are staged byte-for-byte from pinned official 0.163.0. `.github/scripts/test_codexdd_045_3c9c_tui_upstream.py` pins each original (or absent) file and expected target blob.

The other **44 unresolved** paths contain CodexDD-specific TUI changes and are not directly copied. Their complete production SHA inventory is pinned by the same static test, which rejects any accidental overwrite during this packet. They include adaptive startup and terminalization UX, owned events, status routing, elevated Windows app-server behavior, and related tests. The detailed three-way reconciliation will be a separate series of reviewed TUI packets.

The pinned upstream-owned changes include app-server approval/request dispatch, agents overview navigation, transcript ownership and rendering, asynchronous questions and composer state, configuration pages, remote-control UI, user verification, and matching native TUI regression sources. This staging does not enable any UI command privilege or override the CodexDD complexity-floor/trusted signal policy.

## Validation and future integration

Earlier 3C.4–3C.6 source checks passed 106/106 on Daniel-CL. The 3C.7, 3C.8 and 3C.9 static checks remain unrun. The next packets reconcile each fork-sensitive TUI behavior against 0.163.0 with byte-exact review; no native Cargo compile is claimed until all those dependencies and generated schemas are aligned.

**Cargo.lock** remains the original 0.4.4 graph. Final native Rust security/runtime tests, Windows elevated-SSH TUI regressions, GitHub CI and the long 21-stage Windows release suite remain blocked. Production markers are 0.4.4/rust-v0.162.0 and automatic publishing stays hard-disabled. Do not merge, auto-open a release PR or publish artifacts on this checkpoint.
