# CodexDD operating instructions (Windows-only fork)

For CodexDD work, follow the [Major-Phase + Numbered Work-Packet Standard](docs/codexdd-major-phase-work-packet-standard.md), including its **October 11, 2026 Daniel-CL-first testing directive**.

- The assistant makes all GitHub-side source/design/documentation changes and commits. The operator only supplies limited Windows validation through SSH to Daniel-CL; do not ask the operator to edit source or use RDP.
- After each source work packet, perform the **smallest cheap relevant local test on Daniel-CL first**, before broad GitHub CI. Use the native Local Validation Orchestrator's `targeted` or `work_packet` receipt where supported. When validating an uninstalled integration candidate, supply short, non-destructive PowerShell commands for the operator to execute over SSH.
- If local tests fail, repair GitHub-side code, then repeat only affected cheap Daniel-CL checks. **Do not use GitHub Actions as a routine remote debugging loop or create throwaway CI workflows for feasible local diagnostics.**
- Use GitHub Actions for the **final relevant CI gate after local validation passes**. A GitHub-only exception needs a documented reason local testing is unavailable and explicit owner approval. A failed final CI requires log triage, narrow repair, and cheap local retest before intentionally running CI again.
- The release candidate's **native Windows release profile and live F2/daemon/resume behavior** are separate owner-gated checks. CI success is not a substitute. Source semantic acceptance, candidate publication, production merge, install and release remain separately authorized.
- Use read-only GitHub reconnaissance freely when needed, avoid unnecessary Rust tests for docs-only changes, preserve exact source SHA evidence, and do not modify production as a side effect of unapproved integration.

This file is a durable project-specific instruction for future CodexDD agents. It does not itself grant release or production permissions.
