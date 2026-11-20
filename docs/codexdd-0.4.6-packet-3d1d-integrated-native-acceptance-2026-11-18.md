# CodexDD 0.4.6 — Packet 3D.1d source-matched native acceptance

**Reviewed:** 2026-11-18  
**Branch:** `dd/codexdd-v0.4.6-agent-reliability`  
**Exact tested SHA:** `87e89e416611553c22807c9df6a4b25c7d0589ed`  
**Immutable evidence:** [GitHub Actions #44403595032](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44403595032), job `135153602996` (`3D.1d Windows same-SHA D01-D03 plus 3B.4 and retained 3C safety baseline`).  
**Disposition:** **3D.1d DISPOSABLE WINDOWS NATIVE ACCEPTANCE GREEN; REAL INSTALLED CANDIDATE / OWNER QA / RELEASE REMAIN OPEN.** This is the exact-source regression audit, not 0.4.6 release authorization.

## Packet card

- **Outcome:** Verify D01 approved 2-vs-4 benchmarking, D02 live Core-owned HTTP/independent Validator handoff to one-use QA READY, and D03 foreign-process/scope refusals on one identical source SHA, retaining 3B.4 progress and 3C cold-resume gates.
- **Authority:** Preapproved 0.4.6 P0 D01-D03 in a disposable GitHub Windows 2022 environment. No BREAKWATER/CV host or installed CodexDD is a target.
- **Owning tests:** `codex-rs/tui/src/adaptive_benchmark.rs`, `codex-rs/tui/src/chatwidget/adaptive_live_app_server_tests.rs`, `codex-rs/app-server/tests/suite/v2/codexdd_owned_runtime_native.rs`, `codex-rs/core/src/unified_exec/owned_runtime_health.rs`, and bounded TUI/PTY/Core source and replay checks.
- **Stop boundary:** A green CI job does not replace owner-reviewed release/version allocation, full Windows release profile, independent security review or the real-agent Daniel-CL smoke.

## Actual source-matched results

The GitHub run is **completed/success**. Source/contract preflight and Rustfmt are **success**. The Windows integrated job is **success** at the same `87e89e41` SHA; its test inventory and execution explicitly require zero failed/ignored tests. Other independently opt-in jobs show **skipped**, which are not counted as passing tests.

| Contract                               | Native executed evidence at the same SHA                                                                                                                                                                                                                                                                            | Result                                               |
| -------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| **D01 benchmark**                      | `codexdd_046_3d1c_`: 3/3, including real separate 2- and 4-thread OS process measurements under an approved resource lock; four `3d1c2b_` one-test guards for real Core lifecycle, immutable original owner, local ephemeral launcher and no inherited fork authority                                               | **7/7 green** for these explicitly enumerated groups |
| **D02 Core/HTTP safety**               | PTY OS PID/remote-driver refusal 2/2; Core bounded TCP/HTTP 3/3; original owner scope and command authority 2/2; Core review protocol and model refusal 1/1 each; unique process ownership 2/2; TUI one-use admission 4/4; source-bound independent preview and automatic Validator tests and TUI evidence refusals | **All executed focused groups green**                |
| **D02 real native health/cold resume** | `codexdd_046_3d1h_real_core_owned_http_review_and_cold_resume`: 1/1 in a disposable real Windows app-server process with fresh Core-held health observation and restart refusal                                                                                                                                     | **PASS**                                             |
| **D02 positive handoff**               | `codexdd_046_3d1i_real_ops_preview_independent_validator_live_qa_ready`: 1/1; the log confirms original owned preview launch, Core source receipt, distinct independent Validator receipt, fresh host QA and one-use READY, with exact source/turn binding                                                          | **PASS**                                             |
| **D03 boundary**                       | `codexdd_046_3d1b1_owned_host_pid_gate_and_source_preflight_refuse_foreign_process` 1/1; plus unique original Core process, wrong PID/port/source, non-200/wildcard/ambiguity and stale ticket/cold-resume refusal tests                                                                                            | **All enumerated negatives green**                   |
| **Retained 3B.4 and 3C**               | `codexdd_046_3b4_` 8/8; source-pinned READY cold-resume refusal 1/1; pending operations-health/benchmark guard 1/1; blocked/status prioritization 1/1                                                                                                                                                               | **11/11** for these explicitly enumerated controls   |

The job ends by confirming a clean runner checkout and no change to installed CodexDD. The positive fixture runs are genuine process-based _test execution_ and cannot be represented as an actual user machine, production service or completed owner acceptance.

## Earlier red runs and resolution

- [#44393949053](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44393949053) reached Core prerequisites but hit TUI Rust `E0382` (moved `native.turn.id`). Separate source-format failure [#44395340580](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44395340580) required an exact Rustfmt change.
- [Commit `be2d1509`](https://github.com/Takuetsu/codex-dd-astra/commit/be2d1509cc035049eb005264ca24e8ff40f1fa1d) repaired those two test-source defects.
- [#44402347339](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44402347339) was green **preflight/Rustfmt only**; Windows native was opt-in and skipped.
- [Commit `87e89e41`](https://github.com/Takuetsu/codex-dd-astra/commit/87e89e416611553c22807c9df6a4b25c7d0589ed) activated the `[3d1i-native]` same-source integration gate. The resulting [#44403595032](https://github.com/Takuetsu/codex-dd-astra/actions/runs/44403595032) passed.

This history must not be simplified to "all 0.4.6 jobs green" or a complete production pass.

## Residual release and operational holds

1. **Installed host health/QA not proven.** The positive D02 test uses disposable PowerShell/Windows app-server fixtures, not a real owner-installed candidate. The log's end-of-job disclaimer explicitly says `No installed CodexDD, production, release or trusted D02 receipt`: treat this as **no _production_ receipt**, not as a reversal of the native fixture's positive result. Keep the installed/runtime gate PENDING until independently revalidated on the final exact candidate.
2. **Version and provenance not allocated.** `codex-rs/codexdd-version.txt` still reads `0.4.4`; the 0.4.6 development branch is **not** a 0.4.6 packaged binary or approved promotion. Frozen 0.4.5 remains excluded.
3. **P1 conditional E/F/G.** Do not implement speculative worker-seat or provider work absent verified in-capacity/supported-backend reproduction. Preserve documented fail-closed diagnostics.
4. **Release packets remain:** 3E.1 candidate/version/provenance planning and allocation; 3E.2 exact-source Windows integration; 3F.1 independent changed-source/security audit; 3F.2 candidate artifact/provenance audit; 4.1 Daniel-CL real-agent smoke, 4.2 comparable scenarios (10 pairs if feasible), and 4.3 full 21-stage native release profile/owner acceptance.
5. No production merge, installation, tag, release publication, 0.4.5 unfreeze, BREAKWATER/CV write, remote process action, or owner QA PASS occurred.

## Next packet

**3E.1 — source/candidate identity and release-version preflight.** Read the exact current branch HEAD, keep the 0.4.4 marker unchanged until a separate isolated candidate allocation is justified, inspect CLI version assertions and native release profile prerequisites, and record exact source/provenance plus the remaining acceptance matrix. Documentation-only gate recording needs no new Cargo/CI. No release/signoff authority is implied.
