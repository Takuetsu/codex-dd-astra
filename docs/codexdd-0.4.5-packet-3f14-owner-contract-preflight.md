# CodexDD 0.4.5 — Packet 3F.14: explicit 109-owner ledger and cheap local preflight

**NONRELEASE. No production promotion, publication, installation, or native release acceptance.**

The new machine-checkable ledger is `docs/upstream-candidates/codexdd-045-109-semantic-owner-matrix.json`. It maps all 109 official 0.163.0 overlaps to one of nine owners; every path remains `PENDING_OWNER_REVIEW` and `NOT_ACCEPTED`. The original 15 direct conflicts remain independently tracked. The ledger records source identity and bounded test evidence **without** using a green owner-level run as proof that every path is accepted.

| Owner | Files | Original direct conflicts |
| --- | ---: | ---: |
| TUI adaptive/daemon | 44 | 6 |
| Core authority/history | 21 | 3 |
| Persistence/rollout/memory | 13 | 0 |
| Generated protocol and SDK | 12 | 4 |
| App-server and protocol | 11 | 0 |
| Bazel packaging | 3 | 1 |
| Dependency lock and manifest | 2 | 1 |
| CLI startup | 2 | 0 |
| Config | 1 | 0 |
| **Total** | **109** | **15** |

The 109 source dispositions remain **100 historical reviewed blobs, seven exact documented formatter changes and two intentionally secured Cargo manifest/lock changes**. The standalone `.github/scripts/test_codexdd_045_3f14_owner_matrix.py` test fails closed on missing paths, changed Git objects, changed owner counts, unexpected conflict status or weakened protected Cargo/deny/vendor identities.

## Local Daniel-CL preflight before release CI

Run from an isolated exact-candidate worktree beneath `E:\\codexdd`; do not run in the installed 0.4.4 production checkout. The script `scripts/codexdd-045-owner-preflight.ps1` is read-only and supports:

- `-Mode Source` (default): verifies locked source/version markers, only six focused Python source/ownership tests, secure dependency staging and inherited INFO guard; avoids Rust compilation, Bazel, and broad GitHub CI.
- `-Mode Bazel`: performs those Source checks first, then strictly read-only Bazel 9.2.0 module lock analysis and only the CLI and combined-archive MSVC `cquery` labels. It does **not** build, package, install, modify Cargo policy or accept a Microsoft runtime EULA.

Both modes print `GREEN` or `RED`, the tested commit, elapsed time and finish timestamp; the caller can redirect SSH output to a file on Stonks. A passing source or cquery check **does not** replace the necessary optimized Windows native Bazel build, 21-stage Daniel-CL final release receipt or final release CI.

## Remaining #2 work

1. Execute and fix only failing Source selectors on Daniel-CL. Historical frozen receipts must not be rewritten.
2. Run focused current secure-graph MSVC Bazel module/CLI/archive analysis. The October 21 upstream-0.163 run [#39630905182](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39630905182) was green on an older Cargo graph and is not secure-graph packaging approval.
3. If the native Bazel analysis is green, perform a bounded optimized `release_files` compile, validate archive contents and Windows launch behavior, then complete owner semantic signoff before the expensive final 21-stage suite.
4. Preserve the unchanged all-target cargo-deny security policy; do not clear source-only or semantic `NOT_ACCEPTED` release gates prematurely.

The existing secure-graph focused native owner evidence remains bounded at [40/40 Windows core/TUI #40583499451](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40583499451), [14/14 app-server/history/rollout #40598678934](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40598678934) and [schema/SDK generation #40598386099](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40598386099). These were not a 21-stage final release validation.
