# CodexDD 0.4.3 — Gate 3 owner source freeze for controlled candidate preparation

**Decision recorded:** OWNER APPROVED SOURCE FREEZE AND CONTROLLED CANDIDATE PREPARATION. **This is not a release, candidate publication, native Windows PASS, final CI success, production merge, or installation authorization.**

## Exact decision and authority boundary

On October 11, 2026, after the 3F.5 exact upstream/fork source comparison and the 136-path overlap audit had been presented, the owner responded **"yes"** to the precise question:

> "Do you approve freezing the current 0.4.3 source for controlled candidate preparation, with Windows release testing, final CI, publication, merge, and installation remaining separate approval gates?"

This GitHub document is an accurate assistant-authored record of that **conversation-level decision**. It is **not** a cryptographically signed, independently authenticated owner attestation, nor does the owner claim to have personally audited each individual source line. The acceptance is limited to freezing the integrated source and proceeding with **controlled metadata/candidate preparation** on the specifically reviewed release scope, subject to all remaining gates.

## Frozen source identities and accepted review scope

- Repository: `Takuetsu/codex-dd-astra`, Windows-only CodexDD.
- Official upstream target remains **`rust-v0.162.0`**, peeled `c1382380de69521303b416720a52f42d51af6248`; no unreviewed newer upstream substitution is permitted.
- Pre-acceptance integration branch checkpoint: [`35b84a420ebc695b190cf21d510d3ec9efd22924`](https://github.com/Takuetsu/codex-dd-astra/commit/35b84a420ebc695b190cf21d510d3ec9efd22924), Git tree `456311031e1d35b419751fcc37450b710f3623eb`.
- Last product-source-changing commit: [`343be968e5c10b5703f95517eb6ed4be955f4f91`](https://github.com/Takuetsu/codex-dd-astra/commit/343be968e5c10b5703f95517eb6ed4be955f4f91), tree `4f58649164ea5ffcfa782b8c9075d68eafe5db65`. Every subsequent commit through this freeze record is restricted to review documents and read-only CI evidence guards.
- The **frozen source snapshot for v2 staging is the actual Git commit and tree that contain this approval record**, verified against the live integration branch after the document commit; use that actual SHA, never assume the older product-source-changing SHA is the branch HEAD. The source branch must not move again once metadata is tied to this frozen snapshot; an update invalidates staging.
- Current untouched production base: `dd/astra-policy-v2` at `16fdc9826e800ebabab920eadd8e1d976d2f645f`, tree `76ce3087f8808dc57ed3db809ea0b919076e5f65`, product `0.4.2` and tracked upstream `rust-v0.159.2`.
- Reviewed scope: the original **136** overlap paths, **22** textual-conflict strategies (**16 manual_merge**, **5 generator_rebuilt**, **1 upstream_supersedes**), the **12** previously unclassified security-sensitive overlaps, source-level findings for the six mandatory security domains, and the one fail-closed imported-`WorkflowState` hardening patch.
- Technical references: [current 136-path source inventory](codexdd-0.4.3-packet-3f-posthardening-136-blob-audit.json), [post-hardening identity review](codexdd-0.4.3-packet-3f-posthardening-source-audit.md), [22 proposed source conflict resolutions and six-domain matrix](codexdd-0.4.3-packet-3f-technical-semantic-evidence.md), [12 path-by-path official upstream deltas](codexdd-0.4.3-packet-3f-sensitive-upstream-delta.md), and [owner-facing decision packet](codexdd-0.4.3-packet-3f-semantic-owner-review.md). Prior historical JSON fields marking semantic acceptance false record their *earlier audit state*; do not silently rewrite those immutable records.

## Evidence actually completed

- Daniel-CL isolated checkout `E:\codexdd-043-validation`: `cargo fmt --all -- --check` cleared the fail-fast gate, and the focused external migration `append::tests` ran **3 passed / 0 failed** including imported-workflow-authority rejection. The screenshot does not independently print a cryptographically attested HEAD.
- Earlier frozen-source Linux targeted regression CI (628 executed), hosted Windows daemon/TUI compilation and focused unit tests, and generated 13-artifact drift had passed on prior source checkpoints. These runs are *bounded historical evidence*, **not** final CI or native release qualification of the frozen HEAD.

## Permitted next actions (no further owner approval for read-only preparation)

1. Recheck current production, frozen source Git commit/tree, official annotated upstream tags, archived **blocked v1 discovery** (22 conflicts and 136 overlaps), and exact artifact SHA-256 digests.
2. Prepare source-SHA-bound **v2 reconciliation manifest and 136-path/22-conflict sidecar** with genuine, specific existing code review/test evidence. Keep original blocked-discovery artifact bytes unchanged; do not re-label it clean, invent evidence, or silently mark native tests as passing.
3. Produce a **one-parent metadata-only staging commit rooted in the unchanged production base** with **exactly four** allowlisted files (original manifest, original conflict list, reconciled v2 manifest, v1 sidecar). That staging Git object/branch is preparation evidence only, not the canonical candidate or release. Do not modify the source snapshot or product runtime to stage the metadata.
4. Perform read-only preflight against the approved source Git objects and sidecar and report precise remaining gates.

## Explicit exclusions — separate owner gates

- **No canonical candidate publication** to `automation/upstream-candidate-rust-v0.162.0-c1382380de69` and no dispatch of `upstream-reconcile-publish.yml` under this approval.
- No PR promotion, production merge, release tag, Windows installation, Doug deployment, automatic updater action or destructive Git operation.
- No fabricated `release` profile receipt or source-version-bound full-test claim. **Real Daniel-CL native 21-stage release PASS**, elevated SSH TUI/F2, explicit elevated daemon denial, non-elevated detached daemon lifecycle and real resume/fork remain test gates; the one-time 0.4.2 waiver does not carry.
- No final GitHub CI until the applicable cheap local tests and owner-gated candidate acceptance have been met. No unapproved source changes after freeze; any required repair reopens source review and invalidates staging.

**Decision:** Scoped source-freeze approval received. Candidate preparation may proceed; every consequential downstream gate remains separately authorized.
