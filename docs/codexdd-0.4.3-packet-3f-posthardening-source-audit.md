# CodexDD 0.4.3 — 3F post-hardening frozen-source object audit

**Status: GIT OBJECT ACCOUNTING RECONCILED; LOCAL CHEAP UNIT GATE PASSED; OWNER SEMANTIC ACCEPTANCE AND RELEASE VALIDATION STILL PENDING.**

## Identities

- Isolated integration branch source: [`343be968e5c10b5703f95517eb6ed4be955f4f91`](https://github.com/Takuetsu/codex-dd-astra/commit/343be968e5c10b5703f95517eb6ed4be955f4f91), tree `4f58649164ea5ffcfa782b8c9075d68eafe5db65`, with later documentation and CI-guard commits only at observation head `d537ebf6b1fd492c6ee65ccc28422df93d5ed586`.
- Previous fully [audited 136-path inventory](codexdd-0.4.3-packet-3f-postformat-136-blob-audit.json): `32ab09a141d626df5851965a557e0dce877ac2ec`, tree `a0bce7166cccb52e9b4fc8d658e0c08616752e62`. That immutable historical inventory is **not overwritten**.
- Production remains CodexDD 0.4.2 at `16fdc9826e800ebabab920eadd8e1d976d2f645f`; official `rust-v0.162.0` upstream remains pinned at `c1382380de69521303b416720a52f42d51af6248`.
- [Current 136-row Git-object inventory](docs/codexdd-0.4.3-packet-3f-posthardening-136-blob-audit.json) records inherited and changed blobs separately; **all owner semantic/native flags remain false**.

## Mechanically verified 136 overlap object delta

[GitHub complete compare from prior audited source to documentation head](https://github.com/Takuetsu/codex-dd-astra/compare/32ab09a141d626df5851965a557e0dce877ac2ec...d537ebf6b1fd492c6ee65ccc28422df93d5ed586) enumerated **13** changed files, including only two `codex-rs/` files; all other changes are CI workflow and audit documentation. Previous snapshot has a verified complete 136-path Git-object identity ledger.

| Property | Verified count |
| --- | ---: |
| Original fork/upstream overlap paths | 136 |
| Same Git blob as previously verified postformat source | 135 |
| Changed overlap blob | 1 |
| Existing formatter-only changes preserved from original historical ledger | 15 |
| Prior historical review blob unchanged from *before* formatting | 120 |
| Original textual conflict paths (still tracked) | 22 |
| Original classifier sensitive blind spots (still flagged) | 12 |
| Previously documented upstream-only exceptions | 3 |
| Current upstream-only exceptions | 4 |
| Owner-accepted semantic paths / native release paths | **0 / 0** |

The single changed overlap is [`codex-rs/external-agent-migration/src/sessions/append.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/343be968e5c10b5703f95517eb6ed4be955f4f91/codex-rs/external-agent-migration/src/sessions/append.rs#L216-L279), formerly `4efe3d065c832f11846207a7ec1976efcebc7458`, now **`47bc6fb417cfdc1e3415cf5f8f0aa1ea8b44be7b`**. `source_model_items` fails closed on source `WorkflowState`; native destination history is still ignored only for transcript-identity matching. No original textual conflict was reclassified as resolved or owner approved.

The newly changed **upstream-only** [`append_tests.rs`](https://github.com/Takuetsu/codex-dd-astra/blob/343be968e5c10b5703f95517eb6ed4be955f4f91/codex-rs/external-agent-migration/src/sessions/append_tests.rs#L104-L135) was byte-identical to official target `rust-v0.162.0` before the change (`81f3973f799c563715f088145123a1c4a1a5267d`). Its new blob is **`3c4d1f4a86582aab173eef6c3ff2e286953f28f0`**. The other three upstream-only and six fork-only exceptions remain as originally recorded.

This is transitive source-object verification from an immutable prior fully audited Git tree plus a complete 13-path GitHub compare and independent changed-blob fetches. It is not a new line-by-line source-semantic sign-off.

## Local Daniel-CL gate — actual operator evidence

The owner supplied a PowerShell screenshot on October 11 following the exact isolated checkout/format/unit-test command block; the visible output from `E:\codexdd-043-validation\codex-rs` shows:

- `cargo test --locked -p codex-external-agent-migration --lib append::tests -- --nocapture` finished with **3 passed, 0 failed, 0 ignored, 162 filtered out**. All three `sessions::append::tests` cases ran, including the newly added fail-closed regression.
- Compilation completed in **10m 39s**; each unit test subsequently completed successfully.
- The preceding `cargo fmt --all -- --check` is considered successful because the provided script throws and stops on a nonzero format exit before running tests. The screenshot does not separately print `FORMAT_EXIT=0`, and the test log does not display the Git HEAD.
- This is a credible **local cheap-check result**, *not* a cryptographically SHA-bound 21-stage candidate release receipt, final CI, or real elevated SSH F2/daemon observation.

See [implementation and authority-boundary packet](codexdd-0.4.3-packet-3f-import-workflow-authority.md) for exact source reasoning.

## Current release blockers

1. Finalize all **136 owner semantic dispositions**, **22 original conflict strategies** and **six security-domain** reviews; no document or automated test is a substitute for explicit source owner acceptance.
2. The original 628 Linux runtime, native hosted Windows focused daemon/F2 and 13-generator-drift CI runs are valid *older-tree* evidence, not new-source final CI. Following the local-first policy, use subsequent relevant CI only at the accepted source gate.
3. Complete the real elevated-SSH TUI/F2, non-elevated detached daemon and actual resume/fork checks on the Windows candidate; do not reuse the one-time 0.4.2 waiver.
4. Only with separate authorization construct the canonical v2 sidecar/manifest, publish a guarded candidate, run its 21-stage native release profile, and then proceed through final CI, PR review, owner merge/install decisions.

**No candidate branch, production merge, local installation, final CI or release receipt resulted from this read-only audit.**
