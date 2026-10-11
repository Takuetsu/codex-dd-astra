"""Fail-closed, read-only tests for the CodexDD v2 tooling prerequisites."""

import hashlib
import math
import unittest

from upstream_reconciliation import (
    canonical_json_bytes,
    canonical_repo_path,
    parse_strict_json,
    require_fields,
    require_sha,
    unique_repo_paths,
    verify_blocked_v1_discovery,
)


class StrictReconciliationJsonTests(unittest.TestCase):
    def test_canonical_roundtrip(self):
        expected = {"a": [1, 2], "b": "é"}
        payload = canonical_json_bytes({"b": "é", "a": [1, 2]})
        self.assertEqual(payload, b'{"a":[1,2],"b":"\\xc3\\xa9"}\n'.replace(b"\\xc3\\xa9", "é".encode()))
        self.assertEqual(parse_strict_json(payload, canonical=True), expected)

    def test_original_v1_json_may_be_pretty_but_new_artifacts_must_be_canonical(self):
        pretty = b'{ "state" : "blocked_transplant_conflict" }\n'
        self.assertEqual(parse_strict_json(pretty)["state"], "blocked_transplant_conflict")
        with self.assertRaisesRegex(ValueError, "not canonical"):
            parse_strict_json(pretty, canonical=True)

    def test_rejects_duplicate_keys_at_any_depth(self):
        for source in (b'{"a":1,"a":2}', b'{"a":{"x":1,"x":2}}'):
            with self.subTest(source=source), self.assertRaisesRegex(ValueError, "duplicate"):
                parse_strict_json(source)

    def test_rejects_nonfinite_numbers(self):
        for source in (
            b'{"x":NaN}', b'{"x":Infinity}', b'{"x":-Infinity}',
            b'{"x":1e9999}', b'{"x":-1e9999}',
        ):
            with self.subTest(source=source), self.assertRaisesRegex(ValueError, "non-finite"):
                parse_strict_json(source)
        with self.assertRaises(ValueError):
            canonical_json_bytes({"x": math.nan})

    def test_rejects_invalid_utf8_bom_and_root_type(self):
        for source in (b"\xff", b'\xef\xbb\xbf{}', b'[]', b'{"x":1'):
            with self.subTest(source=source), self.assertRaises(ValueError):
                parse_strict_json(source)

    def test_requires_complete_exact_field_set(self):
        self.assertEqual(require_fields({"a": 1}, frozenset({"a"}), context="receipt")["a"], 1)
        for item in ({"a": 1, "surprise": 2}, {}, ["a"]):
            with self.subTest(item=item), self.assertRaises(ValueError):
                require_fields(item, frozenset({"a"}), context="receipt")


class IdentityAndPathTests(unittest.TestCase):
    def test_lowercase_shas_only(self):
        self.assertEqual(require_sha("a" * 40), "a" * 40)
        self.assertEqual(require_sha("b" * 64, sha256=True), "b" * 64)
        for value in ("A" * 40, "0" * 64, "123", True, None, 4):
            with self.subTest(value=value), self.assertRaises(ValueError):
                require_sha(value)

    def test_git_path_confinement(self):
        self.assertEqual(
            canonical_repo_path("docs/upstream-candidates/attestation.json", prefix="docs/upstream-candidates"),
            "docs/upstream-candidates/attestation.json",
        )
        for value in (
            "/abs", "../outside", "docs/../outside", r"docs\outside",
            "docs//file", "docs/./file", "C:/Windows", "docs/%2e%2e/file",
            "docs/file with space", "docs/\0bad", "", None,
        ):
            with self.subTest(value=value), self.assertRaises(ValueError):
                canonical_repo_path(value)
        with self.assertRaisesRegex(ValueError, "confined"):
            canonical_repo_path("scripts/executable.py", prefix="docs/upstream-candidates")

    def test_requires_ordered_unique_nonempty_array(self):
        paths = ["codex-rs/core/src/lib.rs", "codex-rs/tui/src/lib.rs"]
        self.assertEqual(unique_repo_paths(paths), tuple(paths))
        for bad in ([], ["a", "a"], ["a", "../b"], "a", None):
            with self.subTest(bad=bad), self.assertRaises(ValueError):
                unique_repo_paths(bad)


class ImmutableBlockedDiscoveryTests(unittest.TestCase):
    def setUp(self):
        self.paths = [
            "codex-rs/app-server-daemon/src/backend/windows.rs",
            "codex-rs/tui/src/app_server_session.rs",
        ]
        self.doc = {
            "contract_version": 1,
            "state": "blocked_transplant_conflict",
            "transplant_dry_run": "conflict",
            "production_sha": "1" * 40,
            "target_upstream_tag": "rust-v0.162.0",
            "target_upstream_sha": "2" * 40,
        }
        self.report = ("\n".join(self.paths) + "\n").encode()

    def verify(self, *, doc=None, report=None, **overrides):
        raw = canonical_json_bytes(self.doc if doc is None else doc)
        report = self.report if report is None else report
        args = {
            "manifest_bytes": raw,
            "conflict_report_bytes": report,
            "manifest_sha256": hashlib.sha256(raw).hexdigest(),
            "conflicts_sha256": hashlib.sha256(report).hexdigest(),
            "expected_production_sha": "1" * 40,
            "expected_target_tag": "rust-v0.162.0",
            "expected_target_sha": "2" * 40,
            "expected_conflict_paths": self.paths,
        }
        args.update(overrides)
        verify_blocked_v1_discovery(**args)

    def test_preserves_actual_conflict_failure_identity(self):
        self.verify()

    def test_rejects_original_artifact_hash_change(self):
        for field in ("manifest_sha256", "conflicts_sha256"):
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, "digest mismatch"):
                self.verify(**{field: "a" * 64})

    def test_rejects_relabeling_conflict_as_clean_or_ready(self):
        for field, value in (
            ("state", "preparation_ready"),
            ("transplant_dry_run", "clean"),
            ("contract_version", 2),
            ("contract_version", True),
        ):
            altered = dict(self.doc)
            altered[field] = value
            with self.subTest(field=field, value=value), self.assertRaisesRegex(ValueError, "immutably blocked"):
                self.verify(doc=altered)

    def test_rejects_stale_production_or_changed_target(self):
        for kwargs in (
            {"expected_production_sha": "9" * 40},
            {"expected_target_sha": "9" * 40},
            {"expected_target_tag": "rust-v0.163.0"},
        ):
            with self.subTest(kwargs=kwargs), self.assertRaisesRegex(ValueError, "identity mismatch"):
                self.verify(**kwargs)

    def test_rejects_missing_extra_reordered_or_duplicate_conflicts(self):
        reports = (
            self.paths[:1],
            list(reversed(self.paths)),
            [*self.paths, "codex-rs/tui/src/lib.rs"],
            [*self.paths, self.paths[0]],
        )
        for lines in reports:
            with self.subTest(lines=lines), self.assertRaises(ValueError):
                self.verify(report=("\n".join(lines) + "\n").encode())

    def test_rejects_malformed_duplicate_key_v1(self):
        original = canonical_json_bytes(self.doc)
        malformed = original.replace(b'"contract_version":1', b'"contract_version":1,"contract_version":1')
        with self.assertRaisesRegex(ValueError, "duplicate"):
            self.verify(
                manifest_bytes=malformed,
                manifest_sha256=hashlib.sha256(malformed).hexdigest(),
            )


if __name__ == "__main__":
    unittest.main()
