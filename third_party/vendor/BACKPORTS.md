# CodexDD security backports

These source overlays retain the original upstream licenses and identify local revisions in the Cargo lockfile. The security, licensing, and source policies apply unchanged.

## Rama DNS and Hickory transport

- Base: [rama-dns 0.3.0-alpha.4](https://crates.io/crates/rama-dns/0.3.0-alpha.4), source revision `4733273a10a791762e2b7727850032b0c9b8536d`, crate SHA-256 `e340fef2799277e204260b17af01bc23604712092eacd6defe40167f304baed8`.
- Upstream adapter: [Rama 0.3.0 Hickory client](https://github.com/plabayo/rama/blob/1e094e455b9fa29d7ad4bc5d597139934cb73978/rama-dns/src/client/hickory.rs). The overlay adapts construction and A, AAAA, and TXT record access to the published Hickory API while retaining the existing Rama DNS trait and system resolver routing.
- Security fix: published [Hickory 0.26.2](https://github.com/hickory-dns/hickory-dns/releases/tag/v0.26.2) replaces both affected 0.25 crates. It addresses RUSTSEC-2026-0487, -0489, -0490, -0491, -0504, -0505, -0511, -0512, -0514, -0530, -0531, -0532, -0549, -0550, and -0551.
- Policy compatibility: [hickory-net 0.26.4](https://crates.io/crates/hickory-net/0.26.4), source revision `12fd043e6b3a7d56c5a53170ed0e7c6517cb96aa`, crate SHA-256 `ea9b43710d47761c90da994eeec7e1bee50095f2b4bcdfd42acd67ca5f814e47`. Explicit futures replace the banned async trait macro in the socket and timer traits. Request handling, certificate verification and DNSSEC code stay upstream.
- Removal: switch to a supported Rama release that works with the repository Rust and SQLite time constraints and resolves an advisory-clean, policy-compliant Hickory graph. Retest system DNS, proxy routing and TLS.
- Daniel-CL validation: 310 proxy tests passed on both the overlay and the exact PR #89 baseline. Cargo-deny 0.19.0 with all features removed the 15 advisories and reported zero ban, license and source errors. Bazel module refresh and the network-proxy dependency query passed.

## ZIP bzip2 backend selection

- Base: [zip 9.0.5](https://crates.io/crates/zip/9.0.5), source revision `0123ed144cfa7d672dae78b79a034d8900f9aaeb`, crate SHA-256 `6e04976da629d3b04204ba8105b396a193af8ad2583b2c79bfb3ab8af580a11e`. This retains the published fixes for RUSTSEC-2026-0367, -0368, -0369, -0429, -0545, and -0548.
- Upstream backend support: [zip2 PR 685](https://github.com/zip-rs/zip2/pull/685) uses the existing `bzip2-sys` backend with the `bzip2-rs` feature. The local manifest makes the dependency feature reference weak because the same list already enables the dependency explicitly. This prevents activating ZIP's unrelated default bzip2 feature and removes `libbz2-rs-sys` from the locked graph. No codec or archive handling source changed.
- Daniel-CL validation: 16 plugin startup tests passed, including a bzip2 archive extraction regression. The all-features Linux-target license scan found zero errors, and the Bazel plugin dependency query passed.
- Removal: switch to a patched release whose supported bzip2 feature resolves the approved backend alone. Recheck the full compression list, the Linux lock graph and the archive tests.

## Localization TOML parser

- Base: [i18n-config 0.4.8](https://crates.io/crates/i18n-config/0.4.8), source revision `ae88dc79fda061e1bbf96b8d8e8fe8d23f1e52e7`, crate SHA-256 `3e06b90c8a0d252e203c94344b21e35a30f3a3a85dc7db5af8f8df9f3e0c63ef`.
- [Upstream commit 2ce7b1d](https://github.com/kellpossible/cargo-i18n/commit/2ce7b1dfe9a8cc224288bf50cbc2a67a6de863c4) replaces both parser calls and their error type with `toml`. This overlay applies those three changes and uses the compatible `toml` already locked in the workspace. This removes the dependency affected by RUSTSEC-2026-0374, an unmaintained-library advisory.
- Daniel-CL validation: all nine secrets tests passed and the lockfile and all-feature audit confirm `basic-toml` is absent. The Bazel dependency query passed. The separately pinned Linux cargo-deny 0.19.0 license scan also passed after ZIP's backend adjustment.
- Removal: replace the overlay with the published upstream parser migration and rerun the secrets tests and full dependency policy.

## Linux credential store and D-Bus

- Base: [keyring 3.6.3](https://crates.io/crates/keyring/3.6.3), source revision `315cbdf6c6a9153d8c9f88b56568f29862d3e39d`, crate SHA-256 `eebcc3aff044e5944a8fbaf69eb277d11986064cba30c468730e8b9909fb551c`.
- The manifest overlay adopts published [secret-service 6.0.0](https://crates.io/crates/secret-service/6.0.0) and [zbus 6.5.0](https://crates.io/crates/zbus/6.5.0) for the existing asynchronous backend. The TUI and typed error checks also use the published versions. The lockfile no longer contains the crates affected by RUSTSEC-2026-0370, -0371, and -0372. Credential lookup, storage attributes and fallback behavior keep their original implementation.
- Validation: four Windows and six locked Linux credential-store tests passed. In a private Linux D-Bus session, keyring 3.6.3 wrote an isolated test credential, the overlay read and updated it, and the baseline read and deleted it. The full policy still reports only unrelated advisory errors.
- Removal: adopt a release that can use maintained D-Bus crates while retaining platform storage behavior. Repeat the old-to-new and new-to-old credential checks before dropping the overlay.
