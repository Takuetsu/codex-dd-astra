"""3F.51: cross-language generated protocol parity (source only, no regeneration).

An existing Python SDK intentionally excludes two upstream internal raw-response methods
that remain in the TypeScript union. No generated output is rewritten by this test.
"""
from __future__ import annotations

import json
import re
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SCHEMA = ROOT / "codex-rs/app-server-protocol/schema"
JSON_DIR = SCHEMA / "json"
TS_DIR = SCHEMA / "typescript"
SDK = ROOT / "sdk/python/src/openai_codex/generated"
INTERNAL = {"rawResponseItem/completed", "rawResponse/completed"}

def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))

def method_variants(schema: dict) -> dict[str, str]:
    pairs = []
    for variant in schema["oneOf"]:
        method = variant["properties"]["method"]["enum"]
        assert len(method) == 1, "Method JSON schema must use one exact enum value"
        pairs.append((method[0], variant["properties"]["params"]["$ref"]))
    assert len(pairs) == len(dict(pairs)), "Duplicate wire methods"
    return dict(pairs)

def ts_methods(path: Path) -> list[str]:
    return re.findall(r'"method":\s*"([^"]+)"', path.read_text(encoding="utf-8"))

def python_registry_methods() -> dict[str, str]:
    source = (SDK / "notification_registry.py").read_text(encoding="utf-8")
    # Only the source-level NOTIFICATION_MODELS dictionary; exclude imports.
    match = re.search(r"NOTIFICATION_MODELS:\s*dict\[.*?\]\s*=\s*\{(.*?)^\}", source, re.M | re.S)
    assert match is not None, "Missing Python SDK notification registry"
    values = re.findall(r'^\s+"([^"]+)": (\w+),\s*$', match.group(1), re.M)
    assert len(values) == len(dict(values)), "Duplicate Python notification method"
    return dict(values)


class GeneratedSchemaTransportParity(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.notifications = read_json(JSON_DIR / "ServerNotification.json")
        cls.requests = read_json(JSON_DIR / "ClientRequest.json")
        cls.v2 = read_json(JSON_DIR / "codex_app_server_protocol.v2.schemas.json")
        cls.full = read_json(JSON_DIR / "codex_app_server_protocol.schemas.json")
        cls.py = (SDK / "v2_all.py").read_text(encoding="utf-8")

    def test_notifications_json_python_typescript_are_exact_except_two_internal_upstream_methods(self):
        m = method_variants(self.notifications)
        ts = ts_methods(TS_DIR / "ServerNotification.ts")
        envelope = ts_methods(TS_DIR / "ServerNotificationEnvelope.ts")
        py = python_registry_methods()
        self.assertEqual(ts, envelope)
        self.assertEqual(len(ts), 88)
        self.assertEqual(len(set(ts)), 88)
        self.assertEqual(len(m), 86)
        self.assertEqual(len(py), 86)
        self.assertEqual(set(m), set(py))
        self.assertEqual(set(ts) - set(m), INTERNAL)
        self.assertEqual(set(m) - set(ts), set())
        common = (ROOT / "codex-rs/app-server-protocol/src/protocol/common.rs").read_text(encoding="utf-8")
        for name in INTERNAL:
            self.assertIn('"' + name + '"', common)
        self.assertIn("This event is internal-only.", common)
        self.assertEqual(m["turn/adaptiveRuntimeSignal"], "#/definitions/AdaptiveRuntimeSignalNotification")
        self.assertEqual(py["turn/adaptiveRuntimeSignal"], "AdaptiveRuntimeSignalNotification")

    def test_adaptive_signal_json_wire_shape_crosses_rust_and_python(self):
        required = ("evidenceRefs", "signalKind", "sourceTurnId")
        signal = self.notifications["definitions"]["AdaptiveRuntimeSignalEnvelope"]
        self.assertEqual(set(signal["required"]), set(required))
        self.assertEqual(set(signal["properties"]), set(required) | {"diagnosticNote"})
        self.assertEqual(signal["properties"]["evidenceRefs"]["items"]["type"], "string")
        self.assertEqual(set(self.notifications["definitions"]["AdaptiveRuntimeSignalNotification"]["required"]), {"threadId", "signal"})
        self.assertEqual(self.v2["definitions"]["AdaptiveRuntimeSignalEnvelope"]["properties"], signal["properties"])
        kinds = {"complexity", "capability", "mechanical_validation", "implementation_work", "ready_for_validation", "repair_required", "ready_for_owner_qa"}
        self.assertEqual(set(self.notifications["definitions"]["AdaptiveRuntimeSignalKind"]["enum"]), kinds)
        self.assertEqual(set(self.v2["definitions"]["AdaptiveRuntimeSignalKind"]["enum"]), kinds)
        rust = (ROOT / "codex-rs/app-server-protocol/src/protocol/v2/turn.rs").read_text(encoding="utf-8")
        for token in ("pub struct AdaptiveRuntimeSignalEnvelope", "pub source_turn_id: String", "pub signal_kind:", "pub evidence_refs: Vec<String>", "pub struct AdaptiveRuntimeSignalNotification"):
            self.assertIn(token, rust)
        for token in ("class AdaptiveRuntimeSignalEnvelope(BaseModel):", "class AdaptiveRuntimeSignalNotification(BaseModel):", 'Field(alias="sourceTurnId")', 'Field(alias="signalKind")', 'Field(alias="evidenceRefs")'):
            self.assertIn(token, self.py)
        tsv2 = (TS_DIR / "v2/AdaptiveRuntimeSignalEnvelope.ts").read_text(encoding="utf-8")
        for token in ("sourceTurnId: string", "signalKind: AdaptiveRuntimeSignalKind", "evidenceRefs: Array<string>", "diagnosticNote: string | null"):
            self.assertIn(token, tsv2)

    def test_workflow_state_request_matches_json_ts_python_and_thread_owned_dispatch(self):
        methods = method_variants(self.requests)
        name = "thread/workflowState/update"
        self.assertEqual(methods[name], "#/definitions/ThreadWorkflowStateUpdateParams")
        request = self.requests["definitions"]["ThreadWorkflowStateUpdateParams"]
        self.assertEqual(set(request["required"]), {"operation", "threadId"})
        self.assertEqual(set(request["properties"]), {"operation", "threadId", "sourceTurnId", "adaptiveState"})
        self.assertEqual(set(self.v2["definitions"]["ThreadWorkflowStateUpdateParams"]["required"]), {"operation", "threadId"})
        operations = {"setReadyForOwnerQa", "setAdaptiveState", "clear"}
        self.assertEqual(set(self.requests["definitions"]["ThreadWorkflowStateOperation"]["enum"]), operations)
        self.assertEqual(set(self.v2["definitions"]["ThreadWorkflowStateOperation"]["enum"]), operations)
        typed = (TS_DIR / "v2/ThreadWorkflowStateUpdateParams.ts").read_text(encoding="utf-8")
        self.assertIn("threadId: string", typed)
        self.assertIn("sourceTurnId?: string | null", typed)
        self.assertIn("adaptiveState?: ThreadAdaptiveWorkflowState | null", typed)
        client = (TS_DIR / "ClientRequest.ts").read_text(encoding="utf-8")
        self.assertIn('"' + name + '"', client)
        self.assertIn("class ThreadWorkflowStateUpdateParams(BaseModel):", self.py)
        self.assertIn('Literal["thread/workflowState/update"]', self.py)
        common = (ROOT / "codex-rs/app-server-protocol/src/protocol/common.rs").read_text(encoding="utf-8")
        self.assertEqual(common.count('ThreadWorkflowStateUpdate => "' + name + '"'), 1)
        self.assertIn("serialization: thread_id(params.thread_id)", common)
        dispatcher = (ROOT / "codex-rs/app-server/src/message_processor.rs").read_text(encoding="utf-8")
        self.assertIn("ClientRequest::ThreadWorkflowStateUpdate { params, .. }", dispatcher)
        self.assertIn(".thread_workflow_state_update(params)", dispatcher)
        processor = (ROOT / "codex-rs/app-server/src/request_processors/turn_processor.rs").read_text(encoding="utf-8")
        self.assertIn("setAdaptiveState requires adaptiveState", processor)
        self.assertIn(".persist_workflow_state(item)", processor)

    def test_generated_precomputed_schema_source_identity_and_safe_binary_size(self):
        matrix = read_json(ROOT / "docs/upstream-candidates/codexdd-045-109-semantic-owner-matrix.json")
        rows = {p["path"]: p for p in matrix["paths"]}
        for name in ("app-server-exports-stable.json.zst", "app-server-exports-experimental.json.zst"):
            path = "codex-rs/app-server-protocol/schema/precomputed/" + name
            with self.subTest(path=path):
                r = rows[path]
                self.assertEqual(r["semantic_status"], "PENDING_OWNER_REVIEW")
                self.assertEqual(r["release_status"], "NOT_ACCEPTED")
                oid = subprocess.check_output(["git", "rev-parse", "HEAD:" + path], cwd=ROOT, text=True).strip()
                size = int(subprocess.check_output(["git", "cat-file", "-s", oid], cwd=ROOT, text=True).strip())
                self.assertEqual(oid, r["current_blob"])
                self.assertGreater(size, 1024)
        self.assertEqual(len(matrix["paths"]), 109)
        self.assertEqual((ROOT / "codex-rs/codexdd-version.txt").read_text().strip(), "0.4.4")


if __name__ == "__main__":
    unittest.main()
