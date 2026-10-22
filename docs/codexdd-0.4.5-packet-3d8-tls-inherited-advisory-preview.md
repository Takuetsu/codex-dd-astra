# CodexDD 0.4.5 — Packet 3D.8: disposable TLS remediation and inherited advisory preview

**Status: NONRELEASE SECURITY PROBE REQUESTED; exact result pending. No advisory acceptance or lock promotion.**

At nonrelease integration source \`cf6be5add91e943389c9a32ab41509b8f0dae751\`, fresh cargo-deny reported *three actual TLS trust/certificate vulnerabilities* (RUSTSEC-2026-0381, -0382, -0383) in addition to the previously documented four inherited INFO/unmaintained advisories. The latter four must not be used as a blanket excuse to overlook the new TLS vulnerabilities.

The [read-only Cargo TLS patch preview #39770359113](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39770359113) passed. It upgraded only \`rustls-webpki 0.103.15 → 0.103.16\`, \`webpki-roots 1.0.5 → 1.0.10\` and \`webpki-root-certs 1.0.5 → 1.0.10\`, with no Cargo dependency-edge, manifest, policy or other package edits. Cargo's generated candidate \`Cargo.lock\` is exact Git blob \`a6cd27b0b70fde2f2be9c1777f3666ad3fa318bb\`; retained accepted branch lock is \`a8eb8a076276cb3db5674b33c67d743b1c3d21fc\`. Verified raw [artifact #12422077318](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39770359113/artifacts/12422077318) records exact patch.

The initial scratch inherited-advisory [run #39769752179](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39769752179) failed its preview step because \`--config\` was incorrectly passed as a *global* cargo-deny CLI argument rather than an argument to its \`check\` subcommand. This is a workflow command-layout error, not a native application test. Even after correcting the argument placement, the unchanged branch Cargo.lock would still contain three real TLS vulnerabilities.

This revised scratch probe:
1. checks unchanged source/deny markers and runs the authentic *unmodified* all-target audit, expected RED;
2. updates **only three TLS crates** in a disposable runner, verifies the exact CI-produced candidate Git blob and \`cargo metadata --locked\`;
3. constructs a scratch-only \`deny-preview.toml\` that ignores **exactly** four inherited INFO maintenance IDs, never the three TLS security vulnerability IDs, then runs the **all-target** audit against that scratch file with correct check-level flags;
4. always removes the scratch policy and restores the branch lock, rejecting any source or policy mutation.

Even a green scratch result establishes feasibility only. The normal \`codex-rs/deny.toml\` stays unchanged; the active original all-target cargo-deny gate stays RED. The candidate TLS lock must still be integrated via exact-byte controlled source provenance, revalidated with focused Windows trust-network behavior, and pass remaining release/security checks. The four inherited maintenance notices require remediation or a separate named, expiring release-risk acceptance before 0.4.5 release. No version marker promotion, merge, automatic publishing, Daniel-CL acceptance, or installation is authorized.
