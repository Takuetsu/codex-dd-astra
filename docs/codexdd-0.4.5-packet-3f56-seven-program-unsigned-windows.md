# CodexDD 0.4.5 — 3F.56: source-matched seven-program Windows developer distribution

**NONRELEASE. UNSIGNED. NOT A PUBLIC INSTALLER.** This packet attempts to complete the *developer* Windows x64 binary closure without wasting the previous successful 3h46m five-program build.

## Original verified baseline must remain untouched

- 3F.48 optimized source SHA: `57152222a1ef523fe6e2f5079871672423c6cb56`.
- [3F.48 green Windows build #40922368730](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40922368730), real five-program `runtime_package_combined.tar.gz`, ten EXE/PDB objects SHA-256-verified against `docs/upstream-candidates/codexdd-045-windows-distribution-contract.json`.
- Real combined archive artifact #12933707437 expires **2026-11-05 21:18 UTC**. Stage preflight first validates artifact availability via GitHub's actions API, and fails before rebuilding anything if unavailable.
- The 3F.54 source `72cd09d3...` has further **test-only changes**, but its binaries are not claimed byte-equal by assumption. To combine the previously verified five with two fresh builds, the workflow deliberately compiles the two sidecar programs from **exact 3F.48 SHA**, not from 3F.54 HEAD. Separate read-only packaging tools are sourced from the new 3F.56 workflow SHA, which must be recorded in the receipt.

## Two genuine optimized sidecar Bazel targets

The service and proxy release targets are already in the protected 0.163 source; do not edit Bazel rule or cargo graph:

`//codex-rs/windows-sandbox-service:release_files`

`//codex-rs/responses-api-proxy:release_files`

The one-shot workflow `.github/workflows/codexdd-045-3f56-seven-program-windows.yml` runs **only when its draft PR is first opened with base `dd/codexdd-v0.4.5-109-overlap-semantic-review`**. This is important: GitHub Actions allows PR runs to restore the base branch's cache but ordinary pushes to a sibling branch cannot access it. The workflow uses two separately checked-out trees (packaging tools and exact binary source), pinned Windows MSVC x64, secure Cargo/Bazel hashes, native opt thin-LTO settings and **the actual previous completed core action cache**. A cache miss blocks compilation. No five-program rebuild is allowed. Sandbox service and matching PDB are saved to a GitHub artifact **immediately after that build**, before proxy compilation; proxy and matching PDB are likewise saved before archive assembly.

Each sidecar must have a real Bazel `release_files` target receipt, exactly one physical executable and exactly one MSVC PDB, native PE/PDB headers, byte sizes, SHA-256 hashes, matching Bazel/source/tooling attestations and a copied-payload integrity receipt. Build failure prevents downstream labeling as complete.

## Seven-program archive assembly

The independent Python assembler `scripts/codexdd_045_3f56_seven_program_assembler.py`:

1. Rechecks **every byte and manifest hash** of the original ten files inside the historical five-program archive.
2. Rejects missing/extra/symlinked/bad-signature sidecars, source SHA divergence and any missing independent build receipt.
3. Creates an archive with **exactly seven PE EXEs and seven matching MSVC PDBs**, then opens the *finished* archive and SHA-256 verifies all fourteen files anew, including duplicate/path-traversal rejection.
4. Emits `seven-program-sha256-inventory.json` and `codexdd045-seven-programs-win-x64-unsigned.tar.gz` into a GitHub Actions artifact, with `source_provenance_verified=true`, `code_signed=false`, `voice_runtime_included=false`, `official_installer_built=false`, `release_approved=false`.

The successful nonrelease output would close the **seven real x64 programs and PDBs developer-bundle build** gate *for baseline source `5715...` only*. Production signing and official Windows installer still require code-signing credentials, signed/verified voice DLL closure, Python wheel, update and rollback checks, **all 109 formal owner approvals**, final 21-stage Daniel-CL validation on an accepted source and explicit release authorization.

No distribution completion is claimed until all four GitHub artifacts actually exist and the final 14-member receipt is verified.

## Validation policy

The preflight job is source-only and inexpensive; the pull-request opening event, not later PR updates, is the only trigger for this expensive operation. It checks the 3F.55 formal 109-review ledger, the established five-program Windows distribution contract, multiple synthetic sidecar tamper/unsigned-negative tests and PowerShell parseability. Native MSVC sidecar building is **gated on successful preflight and original archive availability**. Every step is one-shot; no manual Daniel-CL edits or CI repetition expected from the user.

The seven-program artifact is **developer-only** and will also expire unless moved into an approved longer-lived package after release authorization. **Do not install or publish 0.4.5 from this packet.**
