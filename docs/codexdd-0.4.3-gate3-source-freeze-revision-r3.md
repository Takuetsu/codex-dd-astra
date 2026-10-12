# CodexDD 0.4.3 — Gate 3 native daemon-startup test repair (candidate revision r3)

**Status: CLI INTEGRATION 5/5 PASS ON DANIEL-CL; FULL RELEASE VALIDATION NOT YET PASSED.**

## Release provenance and previous attempts

- Production/base identity remains `16fdc9826e800ebabab920eadd8e1d976d2f645f`, its tree `76ce3087f8808dc57ed3db809ea0b919076e5f65`, and the approved official target remains Codex `rust-v0.162.0` peeled `c1382380de69521303b416720a52f42d51af6248`.
- Revision r1, candidate `c092b451c189bbda03bdae292889fccbd1ab07ed`, failed `rust-format-check`; its canonical metadata hashes were valid after Windows LF normalization. Its failure is preserved in `archive/codexdd-0.4.3-candidate-r1-failed-format`.
- Revision r2, candidate `677dd64923afd4a2236075b9da92e5f6ff500485`, passed its native reconciliation v2 preflight, but its first 21-stage attempt failed `app-server-tests` at stage 15 (376/377 passed). Building cross-package `test_stdio_server.exe` and running the single formerly failing MCP test on Daniel-CL produced 1/1 PASS; this supports a missing test helper, not a waived assertion. A later detached Windows Scheduled Task executed 19 stages successfully, then failed stage 20 `cli-daemon-startup-tests` (1/5 passed). There is **no successful 21-stage native receipt** for r2.
- A focused 5-case `codex-cli --test daemon_startup` run against r2 in an elevated Daniel-CL SSH PowerShell session reproduced the same 1/5, with four 45-second `deadline has elapsed` failures. This excludes a Scheduled Task S4U context as the sole cause.
- An isolated test adjustment `ff4ff9f5da92929d82709aa35bf240d071fe0dd5` checked the `OpenAI Codex` header instead of the model footer; on Daniel-CL it **still failed 4/5**. Its manually printed PASS label after a thrown failure is invalid evidence.
- The targeted correction `8fd46fbd52fd6bb98b17682bf80ceeea3504a175`, tree `d12ba376a3bb7944258f23c5a574d1e24c955b79`, modifies **only** `codex-rs/cli/tests/daemon_startup.rs`. Its source diff since the earlier frozen 0.4.3 source `3008cbfdd330e8907cd9512c872d23c99c5732a0` is that test file alone. It does not modify Rust runtime logic or relax release profile selection.

## Technical finding and test intent

The TUI deliberately suppresses its F2 shared-background-server exclusion warning in elevated Windows sessions, as asserted by `elevated_windows_implicit_startup_uses_embedded_without_f2_warning` in `codex-rs/tui/src/daemon_startup_tests.rs`. The Windows CLI integration test wrongly expected the obsolete warning text and timed out despite rendering a working terminal. The targeted correction checks readiness by the already-displayed `GPT-5.6-Terra` model footer, sends `/status`, and expects the `Model:` row for elevated fresh/resume/fork/restrictive-launch tests. It additionally checks absence of the shared-daemon PID and state in embedded picker mode. Non-elevated `--strict-config` and restrictive-job warning assertions, original timeouts, and the existing status checks remain.

**Real local validation:** Daniel-CL checkout `E:\codexdd-043-cli-readiness-test` at `8fd46fbd52fd6bb98b17682bf80ceeea3504a175` executed `cargo fmt --all -- --check` successfully, then `cargo nextest run --locked --no-fail-fast -p codex-cli --test daemon_startup --no-capture` using the shared candidate Cargo cache and elevated PowerShell. Output: `Summary [18.961s] 5 tests run: 5 passed, 0 skipped`; each of the four previously failing cases PASS, the fifth Bedrock test PASS, exit code 0. This is a **focused, exact-source** validation and does not substitute for all 21 release stages.

## Freeze scope and next steps

The revised integration source snapshot extends `8fd46fbd52fd6bb98b17682bf80ceeea3504a175` with this review document and five CI source-guard revisions. The source-guard comparisons are pinned to the just-tested Rust source revision `8fd46fbd52fd6bb98b17682bf80ceeea3504a175`; no product-source diff is permissible after it. The final snapshot **commit/tree** must be captured from the resulting integration HEAD and bound into both native v2 candidate manifest and full reconciliation sidecar.

- Preserve all original v1 blocked discovery bytes, SHA-256 checksums, official upstream Git identity, 136 overlap paths, 22 resolved textual conflicts, security reviews, and 12 generated-artifact evidence entries.
- In the sidecar, re-anchor the CLI test overlap's semantic review to this exact review document at the final integration commit, and update all source snapshot references and generated/security source identity locks to that commit. Do not misrepresent prior native-test evidence URLs as new CI runs.
- Stage exactly the four original v2 attestation file paths on an isolated metadata-only staging branch based on the unchanged production commit. Only then construct a one-parent revised candidate commit from the new integration HEAD plus precisely those four metadata files.
- Preserve failed r2 candidate SHA under a separate archive ref **before** exact-expected-SHA update of the canonical candidate ref. Never silently overwrite or re-use r2 release receipts.
- A fresh native v2 preflight and complete **21-stage native release PASS with candidate/source/production-bound receipt** remain mandatory; final GitHub CI/PR review and production merge/installation follow separately. No production merge while these gates are pending.
- The Windows Task Scheduler release runner and its ConPTY compatibility preflight are maintained separately under draft PR #51. Scheduled-task launch acknowledgement and historical 19-stage completion are **not** release PASS.

This record documents existing standing owner permission to carry out the narrow GitHub-side remediation; it does **not** claim a separately signed semantic approval or a release receipt.
