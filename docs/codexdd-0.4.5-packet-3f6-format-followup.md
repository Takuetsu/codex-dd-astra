# CodexDD 0.4.5 — Packet 3F.6: formatter follow-up

Status: source repair and offline provenance checks complete; fresh Rust formatter
execution and native Windows acceptance remain pending. Development only.

Baseline: `2cd0f77e9f4719739aa645832f7bf2fde93c6ac2` on
`dd/codexdd-v0.4.5-patched-lock-integration` (draft PR #54).

## Finding and repair

The [Windows PR job 121909294810](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40419261316/job/121909294810)
failed at `cargo fmt` before compilation or adaptive regressions executed.
Its output contains 16 formatter hunks across nine files in core registry tests,
plugin ZIP tests, rollout search and TUI source/tests. Apply exactly those hunks:
line wrapping, one redundant blank line, the formatter's match-arm block and
generic-argument trailing comma. No intended behavior or dependency change.

The exact before/after Git blobs and all 16 replacements are recorded in
`docs/upstream-candidates/codexdd-045-format-followup.json`. The new 3F.6 check
reconstructs every formatted file from the frozen baseline and verifies exact
current bytes and blob identities. This protects the full files, including their
strings and comments; it does not use a broad whitespace-stripping exemption.
The historical four-way and direct-conflict receipts remain byte-for-byte intact.
Their checks now distinguish the frozen reviewed source from its explicit
formatter follow-up. The current TUI inventory tracks the new hashes.

## Validation and limits

- The full `test_codexdd_045_*.py` suite passed locally: **470 tests**, including
  exact nine-file reconstruction and the original overlap/provenance checks.
- `git diff --check` passed for the repair and provenance changes.
- No Rust toolchain is installed in this execution environment. Matching the
  recorded formatter output is verified; a fresh `cargo fmt --all -- --check`
  has **not** been executed here. Do not label the overall PR CI green.
- This is a mechanical formatting packet; no operator development diagnostics,
  broad GitHub build or Daniel-CL validation was requested. Checkpoint commits
  use `[skip ci]` to avoid launching another build during this narrow repair.

## Separate open work

The ctutils feasibility [run 40419255444](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40419255444)
passed at the baseline. It proved only a disposable `ctutils 0.4.2 -> 0.4.4`
resolution with unchanged dependency edges, producing candidate lock Git blob
`72a72ebcf5f69d47a6bf195f43c93405290db7cd`; that candidate was restored rather
than committed. Its log explicitly retains crypto-bigint and zstd vulnerabilities
and four inherited maintenance advisories as unresolved. The active TLS-patched
lock is still `a6cd27b0b70fde2f2be9c1777f3666ad3fa318bb`. No advisory waiver.

Next packet: reconcile the newly reported dependency vulnerabilities and their
minimal compatible remedies, then coordinate exact-lock gates before any further
dependency promotion. A fresh formatter gate, exact-source native checks,
semantic review, Bazel packaging and final detached 21-stage Daniel-CL release
validation remain outstanding. This packet does not authorize integration merge,
production promotion, installation or release. Shipped markers remain 0.4.4 and
rust-v0.162.0.
