# CodexDD 0.4.5 — 3F.19: cheap synthetic Windows archive rule acceptance

The secured Windows CLI debug-profile compilation [#40708502368](https://github.com/Takuetsu/codex-dd-astra/actions/runs/40708502368) passed all 2,858 build actions in 38m41s, but the true optimized thin-LTO build timed out at 90 minutes. To inspect the packaging rule without another cold optimized build, this packet adds a non-executable synthetic EXE/PDB pair and tests the **unchanged** `bazel/rules/runtime_package.bzl` on Windows/MSVC:

- Build only a tiny `fixture_combined.tar.gz`; verify exactly two flat files, original unstripped bytes, EXE mode 0755 and PDB mode 0644.
- Negatively test a fake executable with no PDB and require the native `Missing pdb_file` error.
- Pin secure Cargo/Bazel/deny/vendor and packaging-rule Git objects, with unchanged production 0.4.4 markers and no EULA acceptance.
- Never build or publish a real CodexDD binary from this fixture. Original 109 semantic and optimized/combined runtime acceptance remain pending.

This is **package-rule behavior evidence**, not shipping archive validation. It keeps CI cheap while the reproducible optimized release strategy is worked out.
