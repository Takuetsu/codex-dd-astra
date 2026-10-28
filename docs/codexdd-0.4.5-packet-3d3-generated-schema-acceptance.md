# CodexDD 0.4.5 — Packet 3D.3: generated schema and SDK acceptance

**Status: exact generated artifacts accepted; two independent byte-for-byte regeneration runs passed on prior commits. Recheck under current accepted security lock graph is pending.**

The read-only generator [GitHub run 39356360311](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39356360311) ran successfully through:

- pinned Cargo metadata and stable/experimental app-server generator,
- core configuration schema generation,
- native schema fixture/precomputed export unit tests,
- Python SDK contract-generation tests.

Its final comparison deliberately failed because 31 **generated** outputs differed from the pre-0.163 committed files. It detected no out-of-scope changes and no Cargo.lock change. It archived the exact 31 generated files in artifact `12235229160` (SHA-256 `e67bc0a1f3ebde2018b9feaec5b1460de3adf5bb217aaae60626f496546098bd`) and a manifest naming source commit `f3a5fbc3b5de5886d8d8f7f76fc7a90a61c31ec4`.

A restricted, one-shot [GitHub acceptance run 39425635641](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39425635641) verified the archive checksum, exact manifest checksum and 31 expected paths, unchanged 0.163 lockfile, and unchanged original generated baseline, then committed **exactly** those outputs in `d4d3c51ca84073092c9b457919a697a20d6b70b3`. The commit contains no handwritten runtime code, lockfile, product version markers, installer or release artifact. This includes two compressed binary `.json.zst` bundles that must never be hand-edited.

The acceptance workflow is narrowly triggered by its own YAML file and is guarded to its original parent SHA. It is not a general automatic publication mechanism. Ordinary source/compile and generated audit workflows continue to have read-only contents permissions. The generated drift and source guard workflows now trigger on schema output changes as well as their own files.

## Required post-acceptance gates

- Native locked regeneration run against the **new exact commit** must finish with zero drift and all schema/SDK unit tests green, without accepting any new changes.
- Source provenance checks must be re-run and updated only for genuine staged 0.163 contract changes; do not suppress stale-blob failures.
- Continue focused native Windows runtime approvals/adaptive Worker/Guardian/daemon tests, protected source overlap audit, and only then the final SHA-bound 21-stage Windows release suite.
- Production version and released upstream remain `0.4.4` / `rust-v0.162.0`; upstream `rust-v0.163.0` is still an unreleased integration.

No local Daniel-CL validation is necessary for the generator's read-only drift check.

## Independent generator checks and trigger repair — October 21, 2026

- The independent post-acceptance [GitHub generator run #39426026717](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39426026717) **passed** on integration SHA \`c9913f575e8ed95450f92956ea67bbfcd5cd427c\`; the later [read-only run #39463513284](https://github.com/Takuetsu/codex-dd-astra/actions/runs/39463513284) **also passed** at \`8ebadcc04cc5386aec48067ad7e90f85861d8efb\`. Both reported successful stable/experimental schema and Python SDK generation, native protocol/export tests, Python contract tests, and the final **zero-drift / no-out-of-scope-writes** comparison. The original 31 generated outputs have not been silently replaced.
- The subsequent ZIP9/BZIP2 security resolver review changed \`codex-rs/Cargo.toml\` and \`codex-rs/Cargo.lock\` to the accepted lock blob \`a8eb8a076276cb3db5674b33c67d743b1c3d21fc\`. The prior green runs **do not** establish the generator result against this newer exact dependency graph or a later frozen candidate.
- The generator audit previously reacted to _generated outputs_ and its own YAML, but not changes to Cargo resolver inputs or generator source. This packet adds tightly scoped source- and dependency-input path triggers (protocol sources/writer, config schema and config sources, Python SDK generator and lock inputs), so the current reviewed graph receives a read-only CI rerun when this workflow changes. A new Python source guard asserts these triggers and the existing strict nonrelease, no-drift checks.
- Any failure is **RED** and requires isolated resolution; no automatically accepted generator output, lock change, release gate override, artifact publication, or Daniel-CL action is authorized. Even a green run on this integration branch is not a substitute for frozen-source 21-stage native release validation or the separate inherited RustSec decision.
