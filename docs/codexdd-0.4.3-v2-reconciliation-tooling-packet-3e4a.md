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

## Verified parser preflight CI — 2026-10-10

- [GitHub Actions #38089565371](https://github.com/Takuetsu/codex-dd-astra/actions/runs/38089565371) completed **success**, pinned to source `cf17f0203bcb9d5be1d24027f05cec39480b9446`.
- New read-only conflict-evidence/strict-JSON primitives: **15 tests passed**, zero failed.
- Existing v1 upstream-sync and receipt tests re-executed unchanged: **23 tests passed**, zero failed.
- Job's isolation assertions checked unchanged production version (`0.4.2`), unchanged tracked upstream (`rust-v0.159.2`), exact production ancestry and no drift in production v1 prepare/promote/discovery files or candidate receipt script.
- This parser does not yet accept any v2 receipt, write a candidate ref, or create a release PR. Missing complete sidecar/manifest/receipt schemas, native receipt adaptation, official-ref/current-base Git verification, source snapshot metadata-only proof, publish-once guard, and separate operator approval remain hard blockers. **Do not merge this incomplete tooling branch.**

## Packet 3E.4b — typed v2 manifest and reconciliation sidecar (2026-10-11)

- Added \`.github/scripts/upstream_reconciliation_v2.py\`, a **read-only** parser for exact typed v2 candidate manifests and v1 reconciliation sidecars, using the existing strict JSON/digest primitives. It rejects unknown fields, wrong contract states, clean-relabeled blocked evidence, truncated or reordered conflict/overlap inventories, absent per-path owner and test/semantic evidence, missing generator provenance, and mismatched production/upstream/source identity.
- Sidecar and manifest cross-bind the immutable discovery by SHA-256, candidate key, official target/peeled commit identity, full overlap inventory, typed security-review matrix, snapshot commit/tree, and a digest of the canonical sidecar bytes. The original blocked v1 artifact stays blocked, not promoted or modified.
- Added \`.github/scripts/test_upstream_reconciliation_v2.py\` positive and negative fixtures; extended read-only parser CI and preserved existing v1 tests unchanged.
- This schema validator is **not a candidate publisher**. Git tree verification, official annotated upstream-tag authenticity, Windows receipt v2 parsing, publish-once branch guards and PR-only promotion remain separately required. A parse PASS cannot make a v2 candidate releasable.

## Packet 3E.4c — source Git objects and metadata-only boundary

- Added read-only Git verifier \`.github/scripts/upstream_reconciliation_git.py\` and isolated actual-Git-repo regression fixtures. The verifier checks the source snapshot's exact Git commit/tree, live production and source refs, production ancestry and source-baked product/upstream markers. It rejects a moved production ref, stale source snapshot, forged tree, wrong version/provenance or injected ref.
- A future canonical candidate must be an **immediate single-parent metadata-only commit** on the pinned source snapshot. Git diff must contain **only added, canonical, allowlisted \`docs/upstream-candidates/\` paths**, and manifest/sidecar must be exact canonical blob bytes with digest binding. Any source edit, deleted file, unexpected path, second parent or post-attestation mutation fails closed.
- Official upstream tag verifier checks both remote annotated ref object IDs, object type \`tag\`, peeled commit and peeled tree against pinned manifest. The publishing workflow must fetch only official \`openai/codex\` tags and use the default official remote URL. No network call runs in the parser CI fixtures and no branch publication is implemented by this packet.
- Tests do **not** authorize v2 promotion, validate native Windows release evidence or bypass the existing clean-only v1 workflow. The isolated bridge remains incomplete.

## Packet 3E.4d — strict native release receipt v2 (parser only)

- Added \`.github/scripts/upstream_reconciliation_receipt.py\` and positive/adversarial receipt tests. The v2 receipt is canonical JSON and binds the exact candidate HEAD, production/base SHA, upstream target tag+SHA, full v2 manifest and sidecar SHA-256, source snapshot commit/tree and canonical deterministic candidate branch.
- Only an explicitly recorded Windows \`release\` profile PASS with at least the current 21 release stages, UTC completion timestamp and \`Daniel-CL\` host claim passes schema checking. The host string is an operator-provided assertion, **not** independent proof of where the process ran; native operator approval remains required. Work-packet, targeted, partial, failed and CI-only evidence cannot satisfy this receipt schema.
- This parser is **not yet wired to the PowerShell candidate receipt generator or upstream promotion workflow**. Existing v1 candidates remain on the original acceptance path; existing v1 conflict rejection is untouched. Next packets must add Windows-native v2 receipt generation, exact Git metadata candidate checks at promotion, and publish-once/PR-only GitHub workflows before this lane can be used.
