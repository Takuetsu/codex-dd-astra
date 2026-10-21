# CodexDD 0.4.5 — Packet 3E.9: core-plugins Cargo rename scoped alias

**Status: EXACT-SOURCE FOCUSED CORE-PLUGINS WINDOWS MSVC BUILD PASSED. NONRELEASE.**

## Exact first failure

The corrected full CLI debug smoke at [run #39663703563](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39663703563), integration SHA `cc89a756b6516ba1cf8f1a6bd53fd1900066bc8f`, failed after approximately 28 minutes **first** in `//codex-rs/core-plugins:core-plugins`: one `serde_yaml` unresolved-import E0432 and two missing-module E0433 errors. The earlier `codex-skills` failure did not recur; that library independently passed [#39660443013](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39660443013), completing 1,281 native Bazel build actions. Source guards on the CLI-failing exact SHA passed **648/648** in [#39663703585](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39663703585).

## Smallest reviewed correction

`codex-rs/core-plugins/Cargo.toml` already declares `serde_yaml = { workspace = true }`. The workspace manifest aliases pinned `noyalib-serde-yaml =0.0.57` under Rust name `serde_yaml`; no dependency change is required or authorized. The generated `@crates//:defs.bzl` knows the alias, but the fork's `codex_rust_crate` macro requires explicit `lib_aliases` at the target.

- Change **only** `codex-rs/core-plugins/BUILD.bazel` to load `aliases()`, filter to local `serde_yaml` and pass that mapping via `lib_aliases = _yaml_aliases`. No top-level Starlark `if`, workspace-wide generated alias map, or `defs.bzl` change; prior blanket aliases caused an analysis transition cycle.
- Source regression assertions cover both direct consumers, the existing pinned Cargo alias, absence of invalid BUILD conditions and the source-pinned native workflow. No `MODULE.bazel`, `MODULE.bazel.lock`, `Cargo.lock`, `deny.toml`, Rust application code, product marker or upstream marker changes.
- Run **only** `//codex-rs/core-plugins:core-plugins` under Windows-2022 / hosted MSVC / Bazel 9.2.0 / strict module lock / maximum 2 jobs. Read-only token; no EULA override, release artifact or PR. The focused job must report a genuine native build success and upload only diagnostics.

**Next after green:** checkpoint the exact build receipt, update the full CLI debug-smoke pinned `core-plugins/BUILD.bazel` SHA and retry that build separately. Do not run the expensive full CLI test while the core-plugins leaf remains red. Four inherited INFO/unmaintained all-target advisories remain unaccepted and blocking.

## Proactive next-consumer audit — October 21, 2026

The complete 161-manifest tree scan discovered two more legitimate runtime consumers with the same pinned dependency rename: `codex-rs/ext/skills` and `codex-rs/external-agent-migration`. They are covered separately by [3E.10](codexdd-0.4.5-packet-3e10-renamed-yaml-consumer-inventory.md); do not infer they compiled from the separate pending core-plugins CI run. No global aliases, release authorization, security exception, or Cargo mutation has been introduced.

## Focused native MSVC closeout — October 21, 2026

[GitHub Windows core-plugins build #39669345757](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39669345757) **PASSED** on SHA `8ee64f46d8f9ef95100006e26e5039d29e06e6ff` with 2,479 Bazel actions and an actual compiled Rust library. The three earlier `serde_yaml` diagnostics were not reproduced. All remaining direct YAML consumers are separately green under [3E.10](codexdd-0.4.5-packet-3e10-renamed-yaml-consumer-inventory.md); this is not a successful CLI release receipt.
