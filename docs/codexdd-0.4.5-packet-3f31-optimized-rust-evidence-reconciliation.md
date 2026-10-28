# CodexDD 0.4.5 — Packet 3F.31: reconcile optimized Rust cache-run verifier false negative

**Nonrelease evidence reconciliation, NOT a new Windows build.** Historical [3F.28 GitHub Actions #40739170655](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40739170655) completed with an overall **red FAILURE**. This label must be retained in historical CI records: the `restore-cache` job's *verification step* failed even though the actual native optimized Rust compilation was successful. The initial `seed-cache` job completed **SUCCESS**.

## Native evidence from the original run

- Exact source commit `49bc278dfe31408f05c7bde949b6857f196d0984`, target `//codex-rs/utils/elapsed:elapsed`, Rust MSVC `--codegen=opt-level=3`, `-Clto=thin`, `-Ccodegen-units=4` and no debug-profile substitution.
- Seed Windows job **122995762452**: `130 processes: 125 internal, 5 local`, successful target build, saved disk/repository caches, cache bytes `8492218` (~8.5 MB).
- Independent restore job **122998612039**: identical normalized Windows action PATH SHA256 `7e3457c3218c6933b8b30de65061ccf56c951619862e7d4f38ed9f2be086908e`, `130 processes: 5 disk cache hit, 125 internal`, no local actions, successful target build and same cache size.
- Failure was a narrow **verifier error**: its `$hits` assertion searched the stdout-only `$log`; Bazel's process summary was in the already-created `$combined` stdout/stderr log. The source verifier has been corrected to read `$combined`, and its local-action regex hardened.
- The historical GitHub workflow stays red; it is *not* relabeled green. Packet 3F.31's new cheap Ubuntu CI reads GitHub's original immutable run/job metadata and job logs with `actions:read`, validates source hashes, native release compiler flags, 5/5 Bazel action-cache reuse and the exact lone failure classification, **without starting Windows or recompiling any Rust**.

**Acceptance if 3F.31 receipt verification passes:** `SMALL_OPTIMIZED_RUST_ACTION_CACHE_REUSE_VERIFIED`. This is stronger evidence than 3F.27's synthetic fixture but is strictly limited to a 130-action single Rust library. It does not establish disk/cache capacity for all 2,858 optimized CodexDD CLI actions, thin-LTO *binary link*, any EXE/PDB/combined archive, full 109-owner semantic signoff, final 21-stage Daniel-CL suite, installer, merge, or release readiness. Production remains at 0.4.4. Do not launch another cold 90-minute build on this evidence alone.
