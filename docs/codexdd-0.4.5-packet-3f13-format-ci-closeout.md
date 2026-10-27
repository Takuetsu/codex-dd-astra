# CodexDD 0.4.5 — Packet 3F.13: verified formatter-only CI reconciliation

**Status: EXACT 72-PATH FORMATTING ONLY PROVEN AND STAGED; RELEASE CI / HISTORICAL TEST REANCHOR STILL PENDING. NOT A RELEASE CANDIDATE.**

## Motivation and exact before/after source

The [0.4.5 blocking-ci run #40598236522](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40598236522) passed Windows PR-fast adaptive tests, codespell, original `cargo-deny`, and blob policy, but `just fmt-check` rejected **68 Python scripts and 4 Bazel BUILD files**. The Python backlog comprised old 0.163 reconciliation and security scripts; the Bazel issue was the location of exactly one `lib_aliases = _yaml_aliases` keyword in each of four crates.

The isolated, one-time [audited formatter run #40603996779](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40603996779) **PASSED** with the exact reviewed parent chain and produced commit [`e50aeb29`](https://github.com/Takuetsu/codex-dd-astra/commit/e50aeb29ffc66490668edbc40921eab1ec3c5966). All **72 files** are listed in `docs/upstream-candidates/codexdd-045-secure-format-only-receipt.json`.

### Fail-closed source acceptance

- Exactly **68 Python files** changed under the original CI roster; each complete Python `ast.dump(include_attributes=False)` matched before vs. after. No additional Python files or unrelated tracked files changed.
- Exactly **4 Bazel BUILD files** changed under the original CI roster. Full source-line multisets were identical, each file retained precisely one `lib_aliases = _yaml_aliases`, and normalized ASTs matched with **only keyword-order normalization of `codex_rust_crate`**. No dependency, alias value, code expression or Bazel target was added/removed.
- Ruff and Buildifier both passed their second, check-only idempotence run. `git diff --check`, the immutable AWS vendor/crypto guard, current dependency-staging check and AWS-vendor manifest unit tests all passed. The historical nine-file Rust formatter provenance tests passed with full Git history available.
- `codex-rs/Cargo.toml`, `Cargo.lock`, `deny.toml`, all 74 files of the reviewed AWS vendor, production 0.4.4/rust-v0.162.0 markers and the original 109 Rust/SDK/CI overlap identities remained untouched.
- The one-shot `contents: write` workflow was deleted by [`976ff547`](https://github.com/Takuetsu/codex-dd-astra/commit/976ff54799b4d7271d34b0274c731bcb7e46eae1) immediately after source staging. No unattended formatter writer remains active on the finished branch.

## Separate next gate: historical source-test reanchor

The old full `test_codexdd_045_*.py` suite is **not green** on this new secure Cargo graph: legacy tests still require obsolete earlier lock/source SHA identities, and prior shallow clones lacked historical Git objects. The formatter-only job deliberately did not rewrite, skip or falsely mark those original feature tests accepted. Renew their provenance pins against the **exact** reviewed `crypto-bigint 0.7.10` lock and target-specific sources, then run the full nonempty suite with full history. Security decisions must remain explicit, not universal exceptions.

A subsequent full `blocking-ci`, native optimized Windows Bazel packaging, complete 109-owner semantics, frozen-source 21-stage Daniel-CL validation and explicit version/production promotion remain open. No installer or shipping release is authorized by this formatting result.
