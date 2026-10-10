# CodexDD 0.4.3 — 3E.4a additive v2 recovery tooling prerequisites

**Status:** READ-ONLY PARSER AND IMMUTABLE DISCOVERY VERIFIER ONLY. NOT A V2 IMPLEMENTATION, PROMOTION PATH, OR RELEASE GATE.

Base: `dd/astra-policy-v2` at `4828e3b4232781963594cfaaebdc49c5531de8b1`. Tooling-only branch: `dd/codexdd-v0.4.3-v2-reconciliation-tooling`.

The upstream integration's original `rust-v0.162.0` discovery was blocked by 22 actual textual conflicts. Existing production v1 prepare/promote/receipt tooling must continue to reject this case and must not be re-labeled clean.

## Packet scope

- `.github/scripts/upstream_reconciliation.py`: **read-only** strict UTF-8 JSON parser, canonical artifact serialization, duplicate-key/nonfinite/unknown-field validation primitives, lowercase SHA-1/SHA-256 checks, canonical repository-relative path guards, exact ordered conflict inventory verification, and original v1 blocked-manifest/digest/target/prod identity checks.
- `.github/scripts/test_upstream_reconciliation.py`: positive and adversarial contract tests. These are generic regression fixtures; they do **not** claim to verify the original 22-path artifact until the exact evidence bytes have been bound by the future v2 sidecar builder.
- A read-only CI workflow runs these tests and re-runs existing v1 upstream-sync tests, asserting no production-version/provenance or previous updater write-path modification.

## Explicit non-authority

This initial packet does **not** define or accept a complete v2 manifest, conflict sidecar, Windows validation receipt, candidate snapshot, or GitHub PR. It does not verify official tag refs, final source commit/tree, native Windows release PASS, semantic review links, or an allowed metadata-only candidate delta. Lexical paths are not filesystem/symlink or Git object validation.

The required follow-up packets are: complete strict v2 manifest/sidecar/receipt schemas with cross-identity checks and exhaustive negative tests; isolated source snapshot and generated-file provenance verifier; Windows native receipt support; guarded publish-once canonical candidate from current production ancestry; and v2-aware PR-only promoter with owner semantic approval. Preserve normal v1 behavior and keep all new write paths opt-in. Publish the bridge via a **tooling-only PR**, and do not merge before explicit owner acceptance. After any tooling merge, production identity changes and a fresh pinned discovery is mandatory.
