# CodexDD 0.4.5 — Zstd, ctutils and crypto-bigint security closure

**2026-10-27 — NONRELEASE / PARTIAL REMEDIATION. This is NOT an advisory-free candidate.**

Branch: `dd/codexdd-v0.4.5-dependency-security-remediation`, based on draft PR #54 head `da14012c5c11d449e4a049dc016a0bb804bc1cf2`. Production remains CodexDD 0.4.4 / rust-v0.162.0. No release marker, `deny.toml`, source behavior, Bazel packaging or production branch was changed.

## Remediation staged

| Security notice                                         | Old graph       | Staged graph    | Exact evidence                                                                                                                                                                               |
| ------------------------------------------------------- | --------------- | --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| RUSTSEC-2026-0391, Zstd abandoned-stream reuse          | zstd 0.13.3     | zstd 0.14.3     | Green disposable Cargo resolver run [40524515029](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40524515029); exact generated lock blob `7af0b0f1d1e6f750d3bab7a81f21a05bf084de3d` |
| RUSTSEC-2026-0398, -0403 and related zstd-safe findings | zstd-safe 7.2.4 | zstd-safe 8.2.0 | Same exact Cargo-generated resolver output; no other crate version changed                                                                                                                   |
| RUSTSEC-2026-0385, ctutils timing leaks                 | ctutils 0.4.2   | ctutils 0.4.4   | Exact checksum from earlier green disposable ctutils resolver blob `72a72ebcf5f69d47a6bf195f43c93405290db7cd`                                                                                |

Combined staged Cargo.lock Git blob: `8be43898f94d59f403f8e03d9d4b5361e492f086`; manifest Git blob: `0eb50f54cfbc192e149bd90960878c27bf211a42`. Current native Windows compile/regressions and the combined all-target cargo-deny verdict have **not** been executed on this combined graph. Prior green feasibility proves compatible Cargo resolution, not native correctness. Keep the integration PR draft.

## RUSTSEC-2026-0387 — crypto-bigint is still a release BLOCKER

The accepted prior lock includes `crypto-bigint 0.5.5`, reached directly from `elliptic-curve 0.13.8`. RustSec's fixed floor is **0.7.6**; there is no compatible 0.5.x patch accepted in the audited dependency graph. `elliptic-curve 0.13` requires the earlier major/minor crypto-bigint API, so `cargo update -p crypto-bigint --precise 0.7.6` is not a valid isolated fix.

Concrete reverse edges in the 0.4.5 full lock:

- `elliptic-curve 0.13.8` ← `p256 0.13.2`, `p384 0.13.1`, `ecdsa 0.16.9`, `primeorder 0.13.6`.
- `p256 0.13.2` ← `age 0.12.1` (Windows `codex-secrets`), `aws-config 1.8.12` (AWS credential login through `codex-aws-auth`), first-party `codex-user-verification`, `hpke 0.12.0` (via age/age-core), `rtc-dtls 0.20.3` and `rtc-shared 0.20.3` (via WebRTC).
- `p384 0.13.1` ← `rtc-dtls 0.20.3`.

Latest published `age 0.12.1` still requires `p256 ^0.13` and `hpke ^0.12` directly ([age metadata](https://docs.rs/crate/age/latest)). The updated AWS `aws-config 1.13.0` still lists optional `p256 ^0.13.2` ([AWS metadata](https://docs.rs/crate/aws-config/latest)); `codex-aws-auth` currently explicitly enables its `credentials-login` feature. Direct user verification also pins p256 0.13. These are **multiple independent owners**, not a simple Cargo.lock patch.

The newer `p256 0.14.0`, `elliptic-curve 0.14.1` and `crypto-bigint 0.7.6` are available, and WebRTC/RTC 0.21.x uses a provider-neutral crypto architecture; however, moving these sources requires explicit API, data-format, credentials and protocol compatibility review. Upgrading only the first-party p256 edge **does not remove** the vulnerable transitive 0.5.5 from age and AWS. Replacing encrypted secrets or signing semantics to silence a security scan is **not approved**.

**Next bounded work packet:** migrate supported owners independently (first-party user-verification; WebRTC 0.21 with protocol regression tests) and resolve the remaining `age`/AWS credential-login dependencies with maintained upstream releases or separately reviewed secure-provider implementations. Verify 0.5.x is absent from both Windows-resolved and all-target Cargo graphs, then run key signing/verification, credential, encrypted-store and RTC/DTLS compatibility regressions, exact locked native tests, unchanged cargo-deny and 21-stage release profile. No fake versioning, `[patch]` masquerade, broad deny ignores, target pruning or advisory waiver.

The four inherited unmaintained advisories RUSTSEC-2026-0374, -0371, -0370 and -0372 also remain open independently, as documented in Packet 3D.7.

## Fail-closed checks

`python3 scripts/codexdd-045-dependency-security-check.py --mode staging` is only a pinned Zstd/ctutils staging consistency check; a green result explicitly **does not** clear crypto-bigint. `--mode release` fails on crypto-bigint 0.5.5 and the known inherited maintenance packages, and is not a substitute for full cargo-deny or Windows validation. The narrow GitHub workflow on this isolated branch runs the staging checker and locked Windows Cargo metadata without publishing or promoting.
