# CodexDD 0.4.5 — Packet 3F.5: v2 reconciled release-target compatibility

**Status: SOURCE HELPER CHECKPOINT; GITHUB CI PENDING. NOT A RELEASE CANDIDATE.**

## Scope

Historical manually-reconciled v2 promotion and Git source checks recognize only upstream 0.162.0 / CodexDD 0.4.3. The selected 0.163.0 upgrade has 15 authentic direct conflicts and cannot be passed through the v1 clean-transplant lane. This packet extends **only** the pure read-only v2 helper modules to recognize an explicit release-specific source/target pair, leaving the old release contract intact.

The closed map binds old 0.4.3 / rust-v0.162.0 / c1382380de69521303b416720a52f42d51af6248 and new 0.4.5 / rust-v0.163.0 / 83086188ae0614bd882818ff1aa23f3194962012 to their respective source branches. A sidecar source branch or Git source ref from a different release is rejected even if the individual ref is otherwise allowlisted. The existing CLI still defaults to its old 0.4.3 ref; an owner-reviewed new workflow will need to pass --source-ref explicitly. Promotion checks the exact pinned target SHA, product version, correct 0.4.5 owner token bound to frozen source SHA, and all unchanged strict candidate/sidecar/native receipt claims.

Seven new synthetic positive/fail-closed 0.4.5 tests exercise identity binding, bad source refs, wrong release branch, old acceptance token, incorrect product/target identity and fewer than 21 native stages. The original historical v2 test suites must run alongside source guards and updater regressions. CI permissions remain contents: read. These tests cannot authenticate Daniel-CL and cannot replace any native release receipt.

## Hard boundaries

No version-marker advance, no automatic release or PR writer, no change to the two existing historical 0.4.3 owner-dispatch publication/promotion workflows, no original discovery byte changes, no Rust/Cargo/Bazel policy waiver or runtime source edits. Release packaging, the four inherited INFO/unmaintained advisories, candidate evidence and manual publication, the complete detached 21-stage Windows release run, CI review and production installation remain independent gates.
