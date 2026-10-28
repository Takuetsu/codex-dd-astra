"""Verify formatter follow-ups against exact historical bytes and logged edits."""

import subprocess
import unittest

from codexdd_045_format_provenance import FORMATTED, RECEIPT, ROOT, current_blob


class FormatterFollowup(unittest.TestCase):
    def test_exact_nine_file_followup(self):
        self.assertEqual(len(FORMATTED), 9)
        self.assertEqual(
            RECEIPT["baseline"], "2cd0f77e9f4719739aa645832f7bf2fde93c6ac2"
        )
        self.assertEqual(RECEIPT["formatter_job"], 121909294810)
        for path, row in FORMATTED.items():
            with self.subTest(path=path):
                original = subprocess.check_output(
                    ["git", "show", RECEIPT["baseline"] + ":" + path], cwd=ROOT
                )
                historical_blob = subprocess.check_output(
                    ["git", "rev-parse", RECEIPT["baseline"] + ":" + path],
                    cwd=ROOT,
                    text=True,
                ).strip()
                self.assertEqual(historical_blob, row["before_blob"])
                expected = original.decode("utf-8")
                for edit in row["edits"]:
                    self.assertEqual(expected.count(edit["before"]), 1)
                    expected = expected.replace(edit["before"], edit["after"], 1)
                actual = subprocess.check_output(
                    ["git", "show", "HEAD:" + path], cwd=ROOT
                )
                formatter_sha = subprocess.check_output(
                    ["git", "hash-object", "--stdin"], cwd=ROOT,
                    input=expected.encode("utf-8"),
                ).decode("ascii").strip()
                self.assertEqual(formatter_sha, row["formatted_blob"])
                current = subprocess.check_output(
                    ["git", "rev-parse", "HEAD:" + path], cwd=ROOT, text=True
                ).strip()
                self.assertEqual(current, current_blob(path, row["before_blob"]))
                if current == row["formatted_blob"]:
                    self.assertEqual(actual.decode("utf-8"), expected)
