# CodexDD 0.4.5 — Packet 3E.10: exhaustive renamed serde_yaml consumer inventory

**Status: FOUR DIRECT CARGO CONSUMERS REVIEWED; TWO MORE NARROW BUILD FIXES STAGED. NONRELEASE.**

## Why this is necessary before another long CLI build

The same `serde_yaml` / pinned `noyalib-serde-yaml =0.0.57` Cargo rename already caused two separate 25-30 minute CLI debug build failures: [skills at #39636959563](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39636959563) and [core-plugins at #39663703563](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39663703563). The [skills leaf #39660443013](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39660443013) passed. The [core-plugins leaf #39669345757](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39669345757) was still running at this checkpoint; do not declare it green without the actual receipt.

A complete GitHub-tree inventory of **161 `codex-rs/**/Cargo.toml` manifests**, source snapshot `8ee64f46d8f9ef95100006e26e5039d29e06e6ff`, found precisely **four** packages declaring `serde_yaml = { workspace = true }`: `codex-rs/skills`, `codex-rs/core-plugins`, `codex-rs/ext/skills` and `codex-rs/external-agent-migration`. There were no failed manifest reads during the audit. The last two are not hypothetical test-only consumers: `codex-rs/ext/skills/src/loader/environment.rs` and `loader/metadata.rs` call `serde_yaml::from_str`, and `codex-rs/external-agent-migration/src/subagents.rs` imports `serde_yaml::Value` and uses `serde_yaml::from_str`. Both current Bazel BUILD files were missing alias wiring.

## Strictly limited remediation

Preserve the accepted `codex_rust_crate(lib_aliases = None)` opt-in API and the exact `noyalib-serde-yaml` security pin. Add the same **local filtered** `_yaml_aliases` rule to **only** `codex-rs/ext/skills/BUILD.bazel` and `codex-rs/external-agent-migration/BUILD.bazel`; this completes the verified four-consumer set while avoiding workspace-wide alias transition cycles. No Rust application code, Cargo files, Bazel macro, module/resolver lock, security audit, upstream marker or product marker changes.

Add source-guard tests that exhaustively rescan all Cargo manifests for renamed dependencies and reject any new, unreviewed direct consumer lacking an explicit alias. Separately, on GitHub-hosted Windows-2022/MSVC with Bazel 9.2.0, compile **only** `//codex-rs/ext/skills:skills` and `//codex-rs/external-agent-migration:external-agent-migration` with strict module locking, `--jobs=2`, exact Git-blob pins and read-only permissions. Upload diagnostics only. Stop on first compilation failure; successful target analysis alone is not a release receipt.

**Exit criteria:** Exact-source nonempty source guards + green native core-plugins leaf + green two-target remaining-consumer leaf, before considering a new full CLI debug smoke. No optimized release build or detached Daniel-CL 21-stage suite, and no action on unresolved four inherited RustSec INFO/unmaintained all-target findings.
