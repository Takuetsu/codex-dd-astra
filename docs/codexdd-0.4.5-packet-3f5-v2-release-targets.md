# CodexDD 0.4.5 — Packet 3F.5: v2 reconciled release-target compatibility

**Status: EXACT-SOURCE GITHUB NONRELEASE CHECKS GREEN; NOT A RELEASE CANDIDATE.**

## Scope

Historical manually-reconciled v2 promotion and Git source checks recognize only upstream 0.162.0 / CodexDD 0.4.3. The selected 0.163.0 upgrade has 15 authentic direct conflicts and cannot be passed through the v1 clean-transplant lane. This packet extends **only** the pure read-only v2 helper modules to recognize an explicit release-specific source/target pair, leaving the old release contract intact.

The closed map binds old 0.4.3 / rust-v0.162.0 / c1382380de69521303b416720a52f42d51af6248 and new 0.4.5 / rust-v0.163.0 / 83086188ae0614bd882818ff1aa23f3194962012 to their respective source branches. A sidecar source branch or Git source ref from a different release is rejected even if the individual ref is otherwise allowlisted. The existing CLI still defaults to its old 0.4.3 ref; an owner-reviewed new workflow will need to pass --source-ref explicitly. Promotion checks the exact pinned target SHA, product version, correct 0.4.5 owner token bound to frozen source SHA, and all unchanged strict candidate/sidecar/native receipt claims.

Seven new synthetic positive/fail-closed 0.4.5 tests exercise identity binding, bad source refs, wrong release branch, old acceptance token, incorrect product/target identity and fewer than 21 native stages. The original historical v2 test suites must run alongside source guards and updater regressions. CI permissions remain contents: read. These tests cannot authenticate Daniel-CL and cannot replace any native release receipt.

## Hard boundaries

No version-marker advance, no automatic release or PR writer, no change to the two existing historical 0.4.3 owner-dispatch publication/promotion workflows, no original discovery byte changes, no Rust/Cargo/Bazel policy waiver or runtime source edits. Release packaging, the four inherited INFO/unmaintained advisories, candidate evidence and manual publication, the complete detached 21-stage Windows release run, CI review and production installation remain independent gates.

## GitHub contract closeout (October 21, 2026)

[Read-only source-guard run #39626573120](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39626573120) **PASSED** at integration commit `8e8ff2df11268326453a6fde3265aa9beea9849d`. Raw job logs confirm three separately checked totals: **439/439** source/provenance checks (including all seven new synthetic v2 target tests); **112/112** guarded updater and historical discovery tests; and **85/85** historical manually reconciled v2 suite tests. **636/636** combined. No gate was waived or test skipped by empty filters.

Acceptance is limited to helper source compatibility and synthetic fixture behavior; the new release-specific publication/promotion workflows, authentic 0.163 discovery evidence and 109 overlap semantic review, all-target RustSec remediation/decision, native Bazel release package, candidate source/receipt freeze and 21-stage Daniel-CL native release gate are not complete. The historically owner-dispatched 0.4.3 workflows remain intact. No product markers or production state changed.
