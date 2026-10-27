"""Exact formatter follow-up identities; historical review receipts stay frozen."""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RECEIPT = json.loads(
    (ROOT / "docs/upstream-candidates/codexdd-045-format-followup.json").read_text()
)
FORMATTED = {row["path"]: row for row in RECEIPT["files"]}


def current_blob(path, reviewed_blob):
    row = FORMATTED.get(path)
    if row is None:
        return reviewed_blob
    if reviewed_blob != row["before_blob"]:
        raise AssertionError(
            "Formatting receipt does not match historical review: " + path
        )
    return row["formatted_blob"]
