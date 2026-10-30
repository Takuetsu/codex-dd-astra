# CodexDD 0.4.5 — 3F.58: verified seven-program unsigned Windows build

**Verified build result:** GitHub Actions [#41073817590](https://github.com/Takuetsu/codex-dd-astra/actions/runs/41073817590), both jobs GREEN, all native Windows steps passed. The original five-program archive was downloaded and independently checked; two new genuine optimized Windows x64 release targets were linked and their EXE/PDB pairs uploaded immediately before attempting the next stage.

The final seven-program archive was independently reopened, exactly seven PE EXEs and seven MSVC PDBs were validated and SHA-256 checked, and the artifact was successfully uploaded.

**Archive SHA-256:** `dccb86a05caa7bf695f4ddd2b471025a50683853fd5dffacc35f0cb981d6ed77`

**Artifact ID:** `12990307741`, unsigned seven-program developer bundle, expires **November 6, 2026 15:42 UTC**.

**Two newly built pairs:**

| File | Bytes | SHA-256 |
|---|---:|---|
| `codex-windows-sandbox-service.exe` | 55,867,392 | `7738934e39ce451f15f342b4ce65710aaab192e5c29fca7707c05d20c523e2ac` |
| `codex-windows-sandbox-service.pdb` | 165,584,896 | `13f2abafd1a312bfb69e1310124796a99af7cff83850bba457dce8ba7e58fc6b` |
| `codex-responses-api-proxy.exe` | 5,852,672 | `690d0e855464135a197e2e6cca08d7da3f63c3a76aab405f1b934eb05866b6a6` |
| `codex-responses-api-proxy.pdb` | 21,147,648 | `e001827c2562b87241f70ad1b9722d1d829b5d8468a6fb15f0f63e54eecb8d6d` |

Complete file metadata and all 10 original member hashes are in `docs/upstream-candidates/codexdd-045-3f58-seven-program-windows-evidence.json`; `test_codexdd_045_3f58_seven_binary_evidence.py` guards the original baseline and new four hashes with exact source SHA/approval status. Evidence was read from the **actual GitHub job log and artifact metadata**; this receipt is not a claim of a second independent local download/rehash.

**Source-provenance boundary:** All seven physical EXEs and PDBs were compiled at old optimized baseline SHA `57152222a1ef523fe6e2f5079871672423c6cb56`, while review candidate SHA is `72cd09d3f52c076d445078758af20489ccdb4e74`. The later differences are test/source-review changes; **do not assume binary equivalence without explicit comparison.** Source-bound distribution approval still requires accepted final source.

**Completed:** seven-program optimized unsigned Windows x64 developer bundle with PDB integrity.

**NOT completed:** seven-path semantic signoff, remaining 65 owner source reviews, 109 formal owner acceptances, Microsoft code signing, voice DLL closure, timestamped installer, update/rollback, or final 21-stage Daniel-CL suite. The original 109-path canonical owner matrix remains `PENDING_OWNER_REVIEW / NOT_ACCEPTED`. The three Bazel owner entries have new *evidence*, not authorization. No 0.4.5 production install, signing, publishing or merge.
