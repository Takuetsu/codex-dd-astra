# CodexDD 0.4.5 — Packet 3D.3: generated schema and SDK acceptance

**Status: exact artifact accepted; independent post-acceptance regeneration gate pending.**

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
