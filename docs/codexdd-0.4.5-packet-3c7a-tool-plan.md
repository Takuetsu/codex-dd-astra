# CodexDD 0.4.5 — 3C.7a protected tool-plan merge

**Date:** 2026-10-18  
**Status:** **INCOMPLETE / NOT A RELEASE CANDIDATE.** Two reviewed 0.163.0 three-way source/test merges, ten static tests authored but unrun. **source-only**, not **native Rust** validation.

## Provenance and preservation

The 0.4.4 fork `core/src/tools/spec_plan.rs` changed official 0.162.0 source by exactly four additions (three imports and a scoped registration block). Those additions reconstruct the production source byte-for-byte, and only those additions were transplanted into pinned official 0.163.0, yielding Git blob `422870dba8b5a3b36994a598380c93f58760ad1b`. The native tests added exactly three functions to the official 0.162.0 test file; removal reconstructs baseline byte-for-byte. Those functions are preserved in official 0.163.0 tests, producing blob `379eb8d44dfa672b70ed3b7d5408e882d8e803e3`.

The tool plan still makes `report_adaptive_signal` directly available and exposes `run_codexdd_validation` only during the trusted mechanical validation turn. Original tests for ordinary, reconnaissance, implementation and terminalization turns remain present. Actual runtime authorization and hook behavior are untested.

## Validation and release gates

3C.4, 3C.5 and 3C.6 completed **106/106 static checks** on Daniel-CL. 3C.7a has 10 source-only checks **not yet run**. **Cargo.lock**, native Rust compilation/security validation, schema generation, long Windows suite and GitHub CI are still pending. Publisher remains hard-disabled; production version markers remain 0.4.4 and upstream 0.162.0.
