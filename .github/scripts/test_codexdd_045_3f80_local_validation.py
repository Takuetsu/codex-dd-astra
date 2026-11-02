"""Cheap structural regressions for the 3F.80 local Windows validator.

Does not execute PowerShell or Rust; checks that an accidental truncation or
duplicated script cannot masquerade as a valid validation entrypoint.
"""
from __future__ import annotations

import re
import unittest
from pathlib import Path

SCRIPT = Path(__file__).with_name("codexdd-045-3f80-local-validation.ps1")


def assert_balanced_powershell_delimiters(text: str) -> None:
    """Check plain delimiters while ignoring comments and quoted literals."""
    pairs = {")": "(", "]": "[", "}": "{"}
    stack: list[tuple[str, int]] = []
    quote: str | None = None
    comment = False
    line = 1
    i = 0
    while i < len(text):
        c = text[i]
        following = text[i + 1] if i + 1 < len(text) else ""
        if c == "\n":
            line += 1
            comment = False
        if comment:
            i += 1
            continue
        if quote is not None:
            if quote == "'" and c == "'" and following == "'":
                i += 2
                continue
            if quote == '"' and c == "\x60":
                i += 2
                continue
            if c == quote:
                quote = None
            i += 1
            continue
        if c == "#":
            comment = True
        elif c in ("'", '"'):
            quote = c
        elif c in "([{":
            stack.append((c, line))
        elif c in ")]}":
            if not stack or stack[-1][0] != pairs[c]:
                raise AssertionError(
                    f"unexpected closing delimiter {c!r} at line {line}"
                )
            stack.pop()
        i += 1
    if quote is not None:
        raise AssertionError(f"unterminated {quote!r} quoted string")
    if stack:
        raise AssertionError(f"unclosed delimiter(s): {stack[-3:]}")


class ThreeF80ValidatorSourceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.source = SCRIPT.read_text(encoding="utf-8")

    def test_no_duplicate_or_truncated_script(self) -> None:
        s = self.source
        self.assertEqual(s.count("function Invoke-CargoStage {"), 1)
        self.assertEqual(s.count("exit $exitCode"), 1)
        self.assertTrue(s.rstrip().endswith("exit $exitCode"))
        self.assertEqual(s.count("CODEXDD_045_3F80_NATIVE=PASS_5_OF_5_NONRELEASE"), 1)
        self.assertEqual(s.count("FULL_STAGE_LOG:"), 1)
        assert_balanced_powershell_delimiters(s)

    def test_fails_closed_before_native_tests(self) -> None:
        s = self.source
        self.assertIn("13", s)
        for name in (
            "13bc15ff844e93dfaf7777c73bb7313b35bb3b81",
            "cfebd9b784ef9ff027b274aa9f7101c9c2496613",
            "f282c7c55d05201e308e60804f85754c8522d462",
            "workflow_mutation_matches_known_turn(",
            "subscribed_connection_ids(thread_id)",
            "git diff --check",
        ):
            self.assertIn(name, s)
        self.assertLess(s.index("source-rustfmt"), s.index("integration-compile"))
        self.assertLess(s.index("integration-compile"), s.index("foreach ($test in $selectors)"))
        self.assertIn("'--test', 'all', '--no-run'", s)
        self.assertEqual(s.count("@{ selector = '"), 5)

    def test_logs_all_compiler_diagnostics_and_never_publishes(self) -> None:
        s = self.source
        self.assertIn("COMPILER_ERROR_DIGEST:", s)
        self.assertIn("FULL_STAGE_LOG:", s)
        self.assertIn("exit_code = $code", s)
        self.assertIn("exactly_one_test_passed = $onePassed", s)
        self.assertIn("formal_owner_approvals = ", s)
        self.assertIn("release_approved = $false", s)
        self.assertNotRegex(s, re.compile(r"\bgit\s+(?:push|commit|checkout|merge|reset)\b"))
        self.assertNotIn("workflow_dispatch", s)


if __name__ == "__main__":
    unittest.main()
