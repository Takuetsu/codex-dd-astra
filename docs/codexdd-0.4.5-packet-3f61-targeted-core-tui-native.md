# CodexDD 0.4.5 — 3F.61: focused native Windows acceptance for the 56 previously queued Core/TUI owner paths

**NONRELEASE; NO OWNER SIGNOFF AUTOMATION.** Source-only 3F.60 [GREEN #41086629411](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41086629411) proved 18 Core and 38 TUI exact Git-blob upstream source reconstructions. The formal source-review ledger now has 109/109 source comparisons and **0/109 OWNER_ACCEPTED**.

The next stage is actual native Windows regression testing, **not another 3h+ optimized release build**.

## Gated test plan

The one-shot `codexdd-045-3f61-core-tui-native.yml` performs a cheap Ubuntu source check first:

- Run the full 56-path Git-blob source proof and the 109-owner nonrelease ledger check.
- Verify all **14 Core + 18 TUI** exact native selector names have unique Rust function declarations in `core/src` and `tui/src`; no missing/zero-case/ignored-selector false greens.
- Parse the native PowerShell script without executing Cargo.

Only on that green preflight does a **Windows-2022 MSVC runner** use pinned Rust 1.95.0, DotSlash and repo Rusty V8 to compile the selected library tests at the secured unchanged Cargo.lock and Rust source. It runs a full `cargo fmt --all -- --check` and locked Cargo metadata before the first native test.

Native focus: terminal tool choice, source-turn policy, post-hook write rejections, child/fork workflow-state identity, interrupted custom/local-shell recovery, approval authority, early startup preset binding, disconnected view/agent navigation, stale Worker input, local daemon MCP authorization, Windows elevated fallback, explicit remote endpoint, server-owned defaults, remote workspace roots, manual Escape and buffered approval handling.

The runner uses **raw redirected Cargo output, not merged PowerShell SSH error streams**; every individual exact native selector must report actual nonzero passed tests. Any failure or zero-case selection is RED and yields a saved log and source-pinned machine-readable receipt. Production 0.4.4, protected source SHA and the release markers are not modified.

## Explicit non-waived native/semantic gaps

The 32 native cases are a **representative focused gate**, not approval for all 109 paths. The 3F.60 review identified 13 critical owner holds, including absent upstream startup-modal input quarantine, removed foreground background-resolution regression, unbounded TUI event buffer semantics, missing API-key Daybreak and Windows paste shortcut negative tests, lost terminal-program-status test module, and changed realtime voice/daemon validation. **These still require restoration, equivalent demonstrable behavior or additional test evidence.**

Do not infer acceptance from a green source/test run. No merge, final 21-stage suite, production installer, code signing or public release is authorized.
