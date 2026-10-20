# CodexDD 0.4.5 — Packet 3D.6: ZIP 9 security migration reconnaissance (nonrelease)

**Status: GREEN LOCKED ZIP 9 RESOLVER PREVIEW; focused Windows native plugin regressions pending. NO RELEASE OR PRODUCTION CHANGES.**

## Exact scope and evidence

- Source: `codex-rs/core-plugins/src/startup_sync.rs` on 0.4.5 integration SHA `25b6cd25945005d67700e6214e9bfa1d9150dd50` (ZIP handling at lines 1033–1129). The crate contains 93 Rust source/test files; an exact-tree source scan found production `zip` usage only in `startup_sync.rs` and fixture ZIP writing in `startup_sync_tests.rs`.
- A curated plugin sync attempts Git, then GitHub HTTP ZIP archive by exact remote SHA, then an export ZIP bootstrap only when no local snapshot exists. Both ZIP pathways call `extract_zipball_to_dir`.
- The extractor uses `ZipArchive::new`, `by_index`, `enclosed_name`, `is_dir`, `File::create`, `std::io::copy`, then `apply_zip_permissions`. It does **not** call `ZipArchive::extract` or `ZipStreamReader::extract`. `enclosed_name` guards archive path traversal. The archive is unpacked into a freshly staged directory before activation.
- **Separate raw-Unix-mode concern:** `apply_zip_permissions` passes the entry's complete `unix_mode()` to `Permissions::from_mode`, without masking SUID/SGID/sticky bits. The Windows implementation is a no-op. No exploit has been demonstrated; this behavior warrants a code fix and regression tests even when the dependency is updated.
- Current locked `zip 2.4.2` is the only `zip` package and its only direct resolved dependent is `codex-core-plugins`. The other nonrelease audit findings (`basic-toml`, `zbus_names`, `zvariant`, `zvariant_derive`, and yanked `spin`) are separately tracked in Packet 3D.5.

## Remediation decision and API reconnaissance

Three `zip` advisories (RUSTSEC-2026-0367/0368/0369) require `zip >=9.0.0` for an advisory-clean resolver. Target `zip 9.0.5` for a narrowly pinned Windows preview. Its upstream-declared MSRV is Rust 1.88; the CI toolchain is Rust 1.95.0. Avoid changing the existing GitHub archive structure, fallback order, or trust boundaries.

Expected production migration is limited to `startup_sync.rs`, its tests, workspace `Cargo.toml`, and the exact Cargo-generated lockfile:
1. Retain explicit entry-by-entry enclosed-path extraction. Avoid replacing it with bulk extraction semantics without a separate threat review.
2. Adjust `ZipFile`'s v9 generic type API by passing `Option<u32>` Unix metadata into the permissions helper; v9 `ZipFile::name()` returns a `ZipResult<Cow<str>>`, so use safe raw-name formatting only for error text.
3. Reject symlink entries before creating output files, and mask Unix file modes to ordinary `0o777` permission bits before applying them; do not preserve privileged bits from archives.
4. Add tests for normal ZIP bootstrap, symlink rejection and privileged-mode stripping (Unix-only assertion). Confirm Windows compatibility and do not weaken other source security invariants.

## Safe sequencing

1. Run the read-only workflow `.github/workflows/codexdd-045-zip9-resolver-preview.yml` on the reviewed integration branch. It checks pre-edit manifest/lock Git blobs (`8ae5758c9baa6091b8ab5dda4e5b848365c0009d` and `096939fcec32cb259bccc0cb36c5d4120701da57`), changes the manifest **only in the disposable CI checkout**, asks Cargo to resolve `zip 9.0.5`, and archives the exact produced lockfile/diff.
2. Inspect the actual resolver output for unrelated version shifts, Rust version compatibility and retained security pins (`rustls 0.23.46`, `quinn-proto 0.11.18`, `noyalib-serde-yaml 0.0.57`). Fail closed on drift or loss of those pins. Do not manually synthesize lockfile changes.
3. After preview acceptance, stage minimal source/API migration and the exact resolver artifact atomically on an isolated candidate branch. Run targeted Windows locked core-plugins startup-sync tests and an unmodified cargo-deny audit before considering integration.
4. Keep all remaining advisories and security migration conflicts visible. **A passing ZIP preview is not a successful security audit or release acceptance.** No release marker advancement, production merge, deployment, or 21-stage Windows validation is permitted at this checkpoint.

## Reviewed ZIP 9 dependency acceptance (October 20, 2026)

- [Green Windows resolver preview #39459826776](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39459826776) produced the sealed artifact `12281749078`, SHA-256 `cf7e9883d128756a1081f39e5c78f47c682e709f3e7282b14a371b4129157d95`.
- The generated lockfile has Git blob `48ae4fb45a83de4d72dee6200b83a770dbf8f760`; its Windows CRLF-encoded manifest is blob `e401338b7e91071eececa93635d05412f0184f43`, or canonical Linux/Git LF blob `43775319466241f34565fa2611172f2239b5bbdf`. The ZIP version is the only manifest edit.
- [Green read-only blob import #39462421319](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39462421319) checked the artifact archive and generated blob and wrote only an unreferenced Git object. Earlier import attempt #39462155864 failed closed on CRLF/LF byte comparison; no branch/source change was made by either job.
- Cargo `zip 9.0.5` resolves without changing the fork's required `rustls 0.23.46`, `quinn-proto 0.11.18` or `noyalib-serde-yaml 0.0.57` pins. Supporting compression and crypto dependencies changed as recorded in the exact artifact lock diff, including `zstd-sys`.
- The source migration keeps manual enclosed-name extraction, rejects symbolic-link ZIP entries before disk writes, masks Unix permissions to `0o777`, and adapts the ZIP 9 API. Dedicated native Windows filters cover both security cases and the HTTP/export fallback lifecycle. A separate Unix-only test asserts privileged bits are stripped.
- Both the integration source edit and byte-identical generated `Cargo.lock` are reviewed together with hash-pinned source guards. This does not waive the remaining `basic-toml`, `zbus*`, `zvariant*` or yanked `spin` findings and does not authorize release promotion or the long 21-stage suite.

## ZIP9 no-BZIP2 license closure (October 20, 2026)

- [Preview #39464423065](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39464423065) proved that ZIP `9.0.5` resolves with `default-features = false` and feature allowlist `aes-crypto, deflate64, deflate, lzma, ppmd, time, zstd, xz`; it removes `bzip2` and `libbz2-rs-sys` without dropping the reviewed rustls/quinn-proto/YAML security pins.
- The exact preview Cargo.lock, Git blob `a8eb8a076276cb3db5674b33c67d743b1c3d21fc`, was imported as an **unreferenced Git object only** by the one-shot [run #39467022057](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39467022057). Commit [`c6c65a7`](https://github.com/Takuetsu/codex-dd-astra/commit/c6c65a7f603d651b35288310a2a220e30d79370f) accepted the exact blob together with matching manifest `c78e1e92e3cfd21a4d16ceea2d55b43d1976e8e4`, and reanchored the SHA-bound native and static guards in a single source commit.
- Native source provenance tests passed on `c6c65a7` ([run #39467522147](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39467522147)); Windows TUI library compilation passed on the same source ([run #39467522119](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39467522119)). Exact-source ZIP fallback runtime and other focused tests remain independently gated; they must pass before release.
- The new archive feature selection intentionally does not decode BZIP2-compressed ZIP entries; curated GitHub/export fallbacks need to remain fail-closed on unsupported input. ZIP extraction path confinement, symlink rejection and Unix permission masking are preserved. The independent cargo-deny [run #39467522626](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39467522626) now reports **licenses ok**, with only the separately tracked inherited informational advisories still failing.

No silent `deny.toml` changes, product/version-marker advancement, full-release 21-stage run, merge, installer, or deployment is authorized by this ZIP acceptance.
