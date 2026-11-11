# CodexDD 0.4.6 — Packet 3B.2o: typed native execution-policy refusal

**Branch:** `dd/codexdd-v0.4.6-agent-reliability`  
**State:** COMPLETE for scoped source/receipt classification and native Windows regression; broader 3B.2 and milestone acceptance remain OPEN (NOT release approval).  
**Parent checkpoint:** `0ff13778ee382a05f71d10411587b15512f26bce` (3B.2n evidence recorded).  
**Owning files:** Core LVO handler and focused Core test, plus opt-in Windows GitHub regression job. No permission defaults or production deployments change.

## Confirmed defect and desired disposition

At [Windows Actions #42954490135](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42954490135), the native delegated `exec_command` returned `Rejected: blocked by policy`. No PowerShell process created the required log. The handler failed closed, but reported `error_type=missing_native_log` rather than the known policy boundary. A model/operator could mistake that for a transient file/log error and keep retrying or treating it as a test failure.

The handler now classifies **only an error returned by the trusted delegated execution call** containing the explicit `blocked by policy` refusal as `execution_policy_blocked`. Without that error, an absent log remains `missing_native_log`. Both remain `status=error, exit_code=2, failed_stage=None`; neither can become test PASS or an authorized repair fingerprint. The complete underlying diagnostic stays in the existing out-of-source log. On confirmed policy refusal, the model-facing message is an actionable `SETUP_BLOCKED` requiring existing approval/sandbox authority, without echoing the raw command or changing permissions.

## Scoped proof

- Cheap exact-head source-contract preflight and repository Rustfmt.
- A single marked [`3b2o-policy-native`] push runs the existing isolated Windows Core job and **two exact selectors** using the same build: prior 3B.2n positive handler and the new 3B.2o policy-classification unit regression. Both must enumerate exactly one executable test and pass. The new test also establishes a generic absent-log error stays generic and policy denial has no repair fingerprint.
- A source-only PASS is not native Windows acceptance. A green synthetic policy-classifier regression is not proof of a normally restricted real agent's Windows execution.
- Retain all prior negative-role/source-binding gates and the separate independent Validator requirement. No unrestricted production execution, no BREAKWATER/CV modification, no frozen 0.4.5 work and no 0.4.6 release/install.

## Deferred acceptance

The private BREAKWATER Node profile needs its own owner-authorized isolated Windows checkout and proof; the restricted Windows sandbox needs explicit valid execution authority rather than a CI permission bypass. Independent live Validation/Repair, 3B.3 evidence provenance, 3B.4 progress/refusal limits, 3C–3F and final owner acceptance remain separate.

## Accepted native Windows proof — 2026-11-11

**PASS: focused 3B.2o acceptance, exact source `c9ebf3a79d6c300b578f3d46e7a198cf1bcad83a`.** [GitHub Actions #43037872070](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43037872070), native Windows job `130582092005`. The source-contract preflight and repository Rustfmt passed on the same SHA. The Windows job compiled `codex-core --lib`, discovered exactly one matching test per selector and executed both without suppressing the previous 3B.2n handler test:

- `codexdd_046_3b2t_real_core_lvo_handler_runs_trusted_pinned_fixture`: **1/1 PASS**, 4.10 seconds. This reruns the disposable synthetic positive handler and native receipt path; it is not a real project validation receipt.
- `codexdd_046_3b2o_policy_denial_is_typed_nonrepairable_setup_blocker`: **1/1 PASS**, 0.00 seconds. It checks that a trusted delegated `blocked by policy` failure yields the typed `execution_policy_blocked` setup blocker, retains no failed stage or repair fingerprint, and generic missing-log cases remain `missing_native_log`.

**Prior RED retained:** [#43035105776](https://github.com/Takuetsu/codex-dd-astra/actions/runs/43035105776) at `c66d567b` failed Rustfmt only (one multiline assertion); its native job did not execute. Exact formatting repair `c9ebf3a7` supersedes that formatting defect. The older policy-denial run [#42954490135](https://github.com/Takuetsu/codex-dd-astra/actions/runs/42954490135) remains correct evidence that ordinary restricted execution did not start a native profile; this packet improves failure classification, not authorization.

**Disposition:** 3B.2o **COMPLETE** for its actual source change and executable focused Windows test. No production sandbox or approval default changed, no live role escalated, no game repository was modified, and no release, merge, build distribution or installation occurred.

## Remaining 3B.2 gate and safe continuation

3B.2 **cannot yet close in full**. Prior 3B.2k and 3B.2l results cover the real Rust targeted profile and negative-role tests, while 3B.2n/o cover a synthetic Core handler and policy-denial classification. Still outstanding are an isolated authorized Windows run of the **actual private BREAKWATER Node** repository-owned profile, normally restricted execution/approval behavior for that profile, separate live independent Validator evidence and full real app-server/TUI cold-restart evidence. These are *not* inferred from the two synthetic green cases.

Read-only inspection confirms the private project's fixed Node targeted/work-packet profiles exist and require a true, project-specific source-matched Windows checkout. The private project's current execution/QA authority must be checked independently before initiating its test profile. Do not start its QA or alter an active milestone to generate a CodexDD receipt; do not transfer private code into public CodexDD Actions or create a fake Node PASS. The CodexDD evidence/progress work of 3B.3 and 3B.4 may be scoped separately without claiming the unfinished private profile.

**Next packet candidate:** 3B.2p — bounded private Node-profile acceptance/authority preflight, or 3B.3a — scoped gate-provenance source audit while private execution remains reserved for authorized acceptance. This document closes only 3B.2o.
