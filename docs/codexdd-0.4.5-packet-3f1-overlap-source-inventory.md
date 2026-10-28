# CodexDD 0.4.5 — Packet 3F.1: reproducible four-way source-overlap receipt

**Status: 109-PATH SOURCE EVIDENCE VERIFIED BY GITHUB STATIC CI; SEMANTIC ACCEPTANCE OPEN. NOT A RELEASE CANDIDATE.**

## Exact pinned inputs and scope

- Official prior baseline: `rust-v0.162.0`, `c1382380de69521303b416720a52f42d51af6248`.
- Official new target: `rust-v0.163.0`, `83086188ae0614bd882818ff1aa23f3194962012`.
- Fork production: CodexDD 0.4.4, `b25550dcb2c6e84a84074b603086c9db6f32a63b`.
- Reviewed integration snapshot: `f6fb89c098da28626c7c072a40d5e9b7306c8866` on branch `dd/codexdd-v0.4.5-upstream-0.163.0-integration`.
- Immutable source receipt: `docs/upstream-candidates/codexdd-upstream-rust-v0.163.0-overlap-source-receipt.json`. Regression: `.github/scripts/test_codexdd_045_3f1_overlap_snapshot.py`. No production source, Cargo manifest/lock, Bazel lockfile, Rust code, advisory policy or version markers changed by this packet.

Using complete, nontruncated Git tree listings for each of the four inputs, an overlap path is one where **official 0.162 blob differs from official 0.163** and **CodexDD 0.4.4 production blob differs from official 0.162**. Absent file versus present file is a difference. The official upstream update changed **1,314** paths; exactly **109** of those were fork-modified overlaps: **106 `codex-rs/`**, **1 `.github/`**, **2 `sdk/`**.

All 109 reviewed integration blobs now differ from the original CodexDD production blob. **Zero** overlap files are accidentally still byte-identical to the original fork. The receipt pins each path's four full Git SHA-1 blob IDs so a future source edit cannot silently reopen the frozen identity review.

## Direct-conflict ledger correction

`docs/upstream-candidates/original-blocked-conflicts.txt` lists **22 historic 0.4.3 / upstream-0.162 transplant conflict paths**. That list is _not_ the original 0.163 merge-conflict list. At the 0.163 target, **13** of those historic paths independently changed upstream and **9** did not. The receipt records both classes without deleting either set or inventing an acceptance for the separate **15 original 0.163 direct conflicts** described in the 0.4.5 planning documents. That latter 15-path roster/semantic review remains its own gate; do not infer acceptance from this 22-path historical roster.

## What continuous source tests actually verify

The Python unittest suite in `.github/scripts/test_codexdd_045_3f1_overlap_snapshot.py` is automatically included in the existing read-only `codexdd-045-source-guards.yml` workflow after this script is committed. It:

1. verifies exact refs/counts and ancestor relationship for snapshot-to-CI HEAD;
2. loads the actual production and CI-HEAD Git tree objects and rejects a missing/different blob for any of 109 overlaps;
3. rejects production identity reversion and changed frozen reviewed blobs until the source receipt is intentionally renewed;
4. checks the separate 22-path historic list and 13/9 classification; and
5. checks unchanged `0.4.4`/`rust-v0.162.0` release markers and Cargo/deny source hashes.

Upstream 0.162/0.163 blob IDs were captured directly from pinned official GitHub trees for this receipt; ordinary source-guard CI does **not** fetch official upstream live or re-perform a semantic merge. The tests prove source identity and provenance **only**. They do **not** prove each file preserves intended CodexDD behavior, that the original 0.163 15-conflict list is semantically closed, or that native Windows tests and all-target security audits pass.

## Remaining independent acceptance gates

- Review the 15 original 0.163 direct conflicts path by path with semantic checks and native security-relevant behavior receipts; attach to a separate v2 reconciliation artifact.
- Preserve Windows-specific CodexDD adaptive/Guardian/workers/daemon/F2 and resume behavior, successful focused Windows native tests and generated schema audits on a frozen exact commit.
- Remediate the four inherited RustSec INFO/unmaintained advisories or obtain a separately named, expiring owner decision. The all-target `cargo-deny` FAIL stays **RED** until one of those is accepted; no exception is created here.
- Full detached 21-stage Windows release suite, version promotion, production merge, packaging, installation and automatic upstream updates remain forbidden until separate acceptance.

**No Daniel-CL work requested for this source-only packet.**

## Source-fixture CI closeout — October 21, 2026

[CodexDD source-guard run #39626573120](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39626573120) **passed 439/439 static cases** at exact SHA `8e8ff2df11268326453a6fde3265aa9beea9849d`. This includes the pinned 109-overlap blob fixture, the 15 original conflict ledger guard, and seven additional reconciled-v2 identity cases; its separate 112 updater and 85 historical reconciliation tests were also green. Later branch changes through `7384da0b29e57c48e763b3794ec69ed67d41053f` only added documentation and the nonrelease Bazel analysis workflow; no reconciled Rust source or original 109 overlap blobs changed. This static receipt does not claim semantic acceptance of all overlaps or complete native release validation.
