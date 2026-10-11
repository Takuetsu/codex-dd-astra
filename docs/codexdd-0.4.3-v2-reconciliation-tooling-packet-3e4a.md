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

## Packet 3E.4e - native Windows PowerShell receipt writer

- Added scripts/codexdd-v2-receipt.ps1 as a standalone, nonpromoting canonical receipt writer. It accepts only a v2 reconciled manifest, exact candidate identity, Daniel-CL host claim, and successful native release-profile result with at least 21 stages.
- Added scripts/codexdd-v2-receipt-tests.ps1 and a Windows PowerShell 5.1 GitHub Actions job. The tests compare emitted JSON bytes against Python canonical UTF-8 serialization and reject work-packet or unrelated-host evidence.
- The existing production v1 candidate-validation script is not yet changed, and this writer cannot trigger a release. The next packet must bind the v2 writer to a complete verified candidate and approved native profile result before enabling the branch.

## Packet 3E.4f - integrated read-only v2 candidate preflight

Added a standalone read-only CLI which combines strict blocked v1 artifact digests, typed v2 manifest/sidecar, source snapshot Git commit/tree and live production ancestry, canonical candidate branch, and exact four-file metadata-only candidate diff. It rejects dirty local worktrees, escaping/symlinked paths, changed evidence blobs, and changed source/provenance. Promotion-mode preflight additionally checks official annotated upstream tags through the separate Git verifier.

The CLI requires independent accepted original discovery digests and the exact production, source, and upstream target inputs. Native mode is a *preflight only*, not promotion authority. The canonical candidate does not yet exist; the separate tooling-only branch is not a release or a candidate.

## Packet 3E.4g - v2 native release entrypoint

Existing candidate-validator v1 behavior remains the default and unchanged for v1 manifest contracts. V2 is dispatched only on an explicit release profile and with independently accepted original discovery digests. A new native-only PowerShell script checks Daniel-CL Windows, the clean canonical branch, pinned original blocked discovery and Git source/candidate metadata through the Python read-only preflight before executing the existing repository release profile.

On a complete release PASS it rechecks source/head/provenance and emits a candidate-bound canonical v2 receipt. Windows CI parses the entrypoints and exercises receipt byte serialization, but no release profile has run. Publication/promotion are still prohibited until separate checked workflows and owner gates are complete.

## Packet 3E.4h - local-only canonical candidate constructor

Added upstream_reconciliation_stage.py and isolated Git fixtures. The constructor accepts an accepted source snapshot and a one-parent metadata-only staging commit from the current production base. It verifies exactly four Git-blob evidence files and the complete immutable discovery/v2 sidecar, then creates a candidate commit object on the accepted source snapshot using a temporary alternate Git index. Its output is an unreferenced local commit SHA and deterministic branch name; it never writes a Git ref or pushes.

Only a separately accepted manual GitHub workflow may subsequently verify current production, official upstream tags, absent candidate branch, and perform a single absence-guarded push. No staging branch or canonical candidate exists in the actual repository yet.

## Packet 3E.4i - manually guarded publish-once workflow source

The new upstream-reconcile-publish workflow is owner-dispatched only, with write permission limited to candidate publication. It pins 0.162.0, requires current production and frozen source/staging SHAs, downloads the original blocked discovery artifact from the accepted GitHub run and compares the staging bytes exactly, then constructs a local-only candidate commit and verifies all typed/Git/official-tag contracts before checking candidate ref absence.

The only remote write is a force-with-lease requiring the candidate ref to be absent. It does not promote a PR, merge, install, alter production, or launch automatically. It remains inert on the unmerged tooling branch and must not be dispatched without explicit approval and a new current-production discovery after the tooling merge.
