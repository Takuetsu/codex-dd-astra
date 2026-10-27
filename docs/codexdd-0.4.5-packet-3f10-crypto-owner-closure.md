# CodexDD 0.4.5 — Packet 3F.10: coordinated crypto-bigint security closure

**Status: #1 CRYPTO-BIGINT FOCUSED SECURITY REMEDIATION COMPLETE. ALL ACCEPTANCE CHECKS GREEN. NOT A 0.4.5 RELEASE CANDIDATE.**

## Accepted source and security boundaries

- Candidate base: `dd/codexdd-v0.4.5-linux-info-risk-accepted`; isolated implementation: `dd/codexdd-v0.4.5-crypto-integrated-staging`.
- Pinned Cargo manifest Git blob: `cfebd9b784ef9ff027b274aa9f7101c9c2496613`.
- Cargo-generated lock Git blob: `13bc15ff844e93dfaf7777c73bb7313b35bb3b81`.
- AWS source originates from published `aws-config 1.13.0`, immutable crate SHA256 `939e256cfe0ed68e96d32f194339d88210d5c10685cff0787400158f852c6819`; the reviewed DPoP P-256/strong-RNG changes are vendored as **74 tracked files** with exact immutable Git subtree `5b1db93de9e14ab5c060794d23a567d907b66f21`. The source guard checks complete Git tree identity, exact tracked file set, and clean worktree, not a platform-dependent CRLF-normalized byte digest.
- The `age` and `age-core` patches are from reviewed exact upstream `str4d/rage` commit `b5b68c4f4f7ea5c368bc060f6fe0237549c3a13f`. `clatter` has reviewed exact source `jmlepisto/clatter` commit `9a8d15c4f80d5911ca0403aa22e0de99ea59df08`. No unbounded Git source trust.
- Complete Cargo graph on both Windows MSVC and Linux GNU has `crypto-bigint 0.7.10`, `p256 0.14.0`, `p384 0.14.0`, `hpke 0.14.1`, `rtc/webrtc 0.21.1`, `clatter 3.0.0`, `aws-config 1.13.0`, `zstd 0.14.3` and `ctutils 0.4.6`; **vulnerable crypto-bigint 0.5.5 absent**. The original three time-limited Linux-only INFO exceptions remain exact; **RUSTSEC-2026-0387 is never ignored**.
- GitHub exact locked source staging [#40576631676](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40576631676) passed and committed all source. The write-capable workflow was subsequently removed. Original production 0.4.4 and release markers remain unchanged.

## Focused regression evidence

| Evidence | Result | Boundary |
| --- | --- | --- |
| [Combined Windows/Linux crypto graph #40573091568](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40573091568) | PASS | Read-only scratch graph, first-party secrets library, P-256 library and AWS owner compile |
| [Legacy age cross-version encryption/decryption #40564129112](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40564129112) | PASS | Published age ↔ reviewed pinned age, both directions, synthetic scrypt fixture only |
| [AWS DPoP crypto-owner compile and tests #40565258093](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40565258093) | PASS | Actual CodexDD AWS auth code compiles; DPoP tests, not a live login |
| [Original all-target cargo-deny #40577886527](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40577886527) | PASS | Real bans, licenses, advisories and Git source controls with existing three Linux INFO owner-accepted exceptions |
| [Exact combined voice WebRTC 0.21.1 #40578510589](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40578510589) | PASS | Native-free Linux musl voice host: **14 passed, zero failed, one intentionally ignored subprocess fixture**; UDP/TCP, ICE limits, network loss, suppression, privacy and teardown |
| [Exact combined Windows native crypto #40579717552](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40579717552) | **PASS** | Windows 2022: **9/9 secrets** library tests, **13/13 ECDSA/user-verification** tests, native `codex-aws-auth --lib` compilation; no live AWS server login |

| [Final exact-source all-target cargo-deny #40581389379](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40581389379) | **PASS** | Latest cross-platform canonical Git vendor tree guard, precise INFO expiry checks and unchanged actual all-target security audit |

## Safety and status

**Packet #1 crypto-bigint focused remediation is COMPLETE**: all four owner migrations staged in an exact checked-in lock; both targets eliminate vulnerable crypto-bigint 0.5.x, native Windows secrets/signatures and AWS owner compile pass, Linux WebRTC real negotiation/privacy/outage regressions pass, and authentic all-target security policy is green. **This is deliberately not a complete 0.4.5 release**. Do not claim real AWS cloud login, customer ciphertext recovery, final Bazel packaging, version promotion, production installation or the full detached 21-stage Windows release suite have passed. The remaining broad 0.4.5 release work (109 overlapping upstream/fork files, native Bazel release package, final fixed-source suite, installation) remains separate.

**#2 109-overlap semantic review has started** on isolated branch `dd/codexdd-v0.4.5-109-overlap-semantic-review`: source inventory [#40580764913](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40580764913) passed (109/109 present, 100 same frozen identities and 9 subsequent changes); see `docs/codexdd-0.4.5-packet-3f11-current-109-semantic-review.md` on that branch. The 15 direct conflicts and full 109 semantic owners still need review. Historical protected core 15/15 and TUI 25/25 are bounded earlier evidence, not final frozen-source release acceptance.
