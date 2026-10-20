# CodexDD 0.4.5 — 3D.1 Windows-resolved dependency lock acceptance

**Status: DEVELOPMENT LOCKFILE PIN; NATIVE LOCKED BUILD, RUNTIME/SECURITY TESTS, AND RELEASE ACCEPTANCE PENDING.**

## Provenance

The nonrelease native Windows library compile passed [GitHub Actions run #39346683480](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39346683480) on commit `e2383021ba6ef50624af77ec9d7b6f48d931df69`. Its published build artifact `codexdd045-windows-check-39346683480-1` (artifact ID `12231199895`) contains the exact Cargo-generated resolved `codex-rs/Cargo.lock` for this integration. The artifact also includes the Cargo log and resolved lock diff.

- Previous checked-in `Cargo.lock` Git blob: `0c913b46674d3593ee84f1fee5508d16e33efae5`.
- Actual Windows-resolved Cargo.lock Git blob: `096939fcec32cb259bccc0cb36c5d4120701da57`.
- The new blob was reconstructed solely from the checked-in old lock and the machine-generated changes; the reconstructed Git blob SHA matches the artifact's content *exactly*. It is not a copied official upstream lock.
- All 160 local workspace version entries advance from 0.162.0 to 0.163.0. The graph adopts official `rmcp` and `rmcp-macros` version 3.5.1 with pinned rev `4048ac40e958fb1a7d614617a2ae8d26666ee2fa`, adds `linkify 0.11.0`, and removes the outdated `codex-shell-escalation` package from the resolved graph. The source crate may remain until a separate removal audit.

## Fork security constraints

The Windows resolver retained `rustls 0.23.46`, `quinn-proto 0.11.18`, and the fork's replacement `noyalib-serde-yaml 0.0.57` (along with `noyalib 0.0.57`). Do not substitute the official upstream lock wholesale: it uses 0.0.0 local crate versions, a lower rustls version, and the unapproved serde_yaml chain.

The static test `.github/scripts/test_codexdd_045_3d_lock_acceptance.py` asserts the exact resolved Git blob, original production lock, key dependency pins, removal of the vulnerable YAML chain, and unchanged release markers. Earlier packet guards were reanchored to the exact reviewed resolver blob without dropping their fork-security and source-provenance invariants. Their earlier source-only checkpoints are historical, not proof that the new lock has passed `cargo check --locked`.

## Acceptance gates

The branch's focused Windows Cargo workflows now use `cargo metadata --locked --format-version 1` followed by native `cargo check --locked --keep-going -p codex-tui --lib` and `--tests`, respectively. These run in isolated read-only nonrelease GitHub runners, and the lockfile hash is no longer permitted to drift silently. Static packet source checks run independently.

Still required: complete native test compilation, targeted Windows adaptive Worker and Guardian approval regressions, a security advisory audit, regenerated-schema consistency, protected source inventory/merge review, the detached SHA-bound 21-stage Windows release suite, release CI evidence and explicit release decision. This checkpoint does **not** authorize a merge, updater deployment, packaging, publication or version-marker advancement.

Product remains CodexDD `0.4.4`; official shipped upstream marker remains `rust-v0.162.0` until release promotion. No Daniel-CL action is needed for these GitHub compiler checkpoints.
