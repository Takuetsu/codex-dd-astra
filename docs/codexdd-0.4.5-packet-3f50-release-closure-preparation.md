# CodexDD 0.4.5 — Packet 3F.50: Windows distribution requirements and native final gate

**Development-only. Not a release, installation approval, all-owner semantic acceptance, or full Windows validation.** Source baseline: `57152222a1ef523fe6e2f5079871672423c6cb56`. Production stays CodexDD 0.4.4 / upstream rust-v0.162.0.

## Real optimized Windows evidence (now complete)

[3F.48 run #40922368730](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40922368730) completed successfully in approximately 3h46m. CLI/PDB were verified and uploaded *before* the combined archive compilation. The combined target compiled successfully; its independent second verifier confirmed exactly **five Windows PE executables + five MSVC PDB files** and SHA-256 inventory. Binary bytes and actual SHA-256 digests of all ten members are frozen in `docs/upstream-candidates/codexdd-045-windows-distribution-contract.json`.

Combined `runtime_package_combined.tar.gz` SHA-256:
`ce6bd0115fc0f46c1d861517221b2f898e763bfa7e539bb672a26bdb2eeaeb8e`.

Artifacts: [CLI/PDB #12930092426](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40922368730/artifacts/12930092426), [combined #12933707437](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40922368730/artifacts/12933707437), [diagnostics #12933467682](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40922368730/artifacts/12933467682). **These original artifacts have seven-day retention.** Do not rebuild the 2,858-action CLI merely to regenerate metadata.

## Windows distribution decision: preserve official upstream seven-program closure

The optimized five-program combined archive is a **developer/test artifact**, not a complete standalone installer. It contains the CLI, app-server, code-mode host, command runner and sandbox setup executables, each with its matching PDB. Current Bazel packaging intentionally places the **Windows sandbox service** in `runtime_package_helpers` and the **responses API proxy** in `runtime_package_primary`.

The official `.github/workflows/rust-release-windows.yml` explicitly stages and signs **seven** x64 Windows binaries, including those two; it additionally builds and checks a signed voice runtime, official archive outputs, Python wheel, and timestamped installer. Therefore:

1. **Keep the current combined Bazel target unchanged** (five EXEs plus five PDBs). Renaming the combined archive to a full installer or silently injecting optional components would break source/upstream packaging semantics.
2. **Require two separately built source-matched supplemental executables** (`codex-windows-sandbox-service.exe`, `codex-responses-api-proxy.exe`) and matching PDBs for the internal optimized developer distribution. A future production installer must include/sign all seven programs using the official release manifest and must meet voice, installer and trust checks.
3. For complete developer staging, use `python -B scripts/codexdd_045_distribution_gate.py --combined-archive <actual-combined.tar.gz> --supplemental-dir <four-file-folder>`. It verifies every one of the ten original combined bytes against run #40922368730; the sidecars must contain **exactly two physical EXEs and two PDBs** with signatures and SHA-256 receipts. The sidecar mode explicitly **does not** attest sidecar source provenance or approve a public release; that still needs an exact source-bound build receipt.
4. `python -B scripts/codexdd_045_distribution_gate.py --source-only` performs only fast source-contract checks. It rejects mismatch with the official seven-binary Windows signer and refuses premature `release_approved` status.

This is a finalized **distribution requirement**, not completed distribution validation. **The two extra optimized binaries and PDBs, signed voice closure, public installer, and signing are still pending.**

## Five additional scoped owner reviews

`docs/upstream-candidates/codexdd-045-3f50-scoped-owner-review.json` adds source-identity-bound, testable scope notes for these five of the 109 upstream overlaps:

- `.github/workflows/bazel.yml`: source Windows shards, native job and 90-minute time bounds. A successful 3F.48 run **does not** prove the full sharded Bazel CI workflow.
- `codex-rs/BUILD.bazel`: official five-program combined target and distinct service/proxy targets, now paired with genuine optimized 3F.48 artifacts.
- `codex-rs/cli/BUILD.bazel`: release-profile CLI target with CodexDD version input and genuine optimized CLI/PDB.
- `codex-rs/Cargo.toml`: secured dependency owner manifests and retained age/AWS/RTC features; focused native owner receipts `40581389379`, `40579717552`, `40578510589`.
- `codex-rs/Cargo.lock`: secured Cargo-generated graph and independently audited `crypto-bigint 0.7.10` only, no vulnerable 0.5.5.

These are **five scoped source reviews, not five final semantic approvals**. The original 109-row machine ledger deliberately remains **109 PENDING_OWNER_REVIEW / NOT_ACCEPTED**, including all 15 direct conflicts. Remaining acceptance requires per-path upstream intent and fork behavior review, final source-frozen native tests and release owner approval; no historical green should be silently transferred to a new source SHA.

## Daniel-CL release validation prepared, not executed

`scripts/codexdd-045-run-final-21.ps1` runs on **Daniel-CL only** and defaults to `-Mode Preflight`. It checks clean pinned candidate source, existing owner/security preflight, seven-program distribution-source policy, and the focused review tests **without compiling Rust or running the 21-stage suite**. It produces an unambiguous nonrelease result.

`-Mode Final -ExpectedCommit <full-40-character-SHA> -LogPath <path-outside-worktree>` will invoke the **existing unchanged 21-stage native release profile** only once all 109 paths have independently reached `OWNER_ACCEPTED`, on the exact frozen candidate SHA. It fails closed on dirty/source drift, any stage failure, missing/ambiguous machine-readable terminal event, or fewer than 21 unique passed stages. The log and receipt stay outside the Git worktree and explicitly state `release_approved=false` and `combined_distribution_verified=false`. For Stonks-first operation, SSH stdout can be piped to a local log so the user never needs to copy Daniel-CL logs manually.

Do **not** invoke final mode yet: all 109 owner semantic states are still pending. Native preflight on Daniel-CL and 21-stage acceptance are **not green** from this packet. The physical source-bound release distribution also needs the two sidecar binaries, voice runtime, code signatures and explicit owner authorization before installation.

## Validation and next work

Cheap standalone Python syntax and synthetic distribution negative cases were exercised locally without the repository checkout (five synthetic cases PASS); the full exact-source tests and Windows PowerShell parser still require the isolated repository/Windows host. No full Rust build or broad GitHub CI was intentionally launched for this documentation/tooling change.

**Next work:** run the cheap Daniel-CL preflight on the candidate, review the remaining 104 paths (including remaining original direct conflicts and generated/app-server contracts), build/check the two missing Windows binaries in a focused source-bound stage, close all 109 owner semantic gates, then invoke the 21-stage Daniel-CL final validation and release CI. Keep 0.4.5 draft/nonrelease until approval.
