# CodexDD 0.4.6 — Packet 3B.2o: typed native execution-policy refusal

**Branch:** `dd/codexdd-v0.4.6-agent-reliability`  
**State:** scoped source fix and executable regression staged for exact-source native Windows acceptance (NOT release approval).  
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
