"""Exact formatter follow-up identities; historical review receipts stay frozen."""

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
RECEIPT = json.loads(
    (ROOT / "docs/upstream-candidates/codexdd-045-format-followup.json").read_text()
)
FORMATTED = {row["path"]: row for row in RECEIPT["files"]}
CORRECTIONS = {
    row["path"]: row
    for row in json.loads(
        (ROOT / "docs/upstream-candidates/codexdd-045-reviewed-semantic-corrections.json").read_text()
    )["paths"]
}


def current_blob(path, reviewed_blob):
    row = FORMATTED.get(path)
    if row is not None and reviewed_blob != row["before_blob"]:
        raise AssertionError(
            "Formatting receipt does not match historical review: " + path
        )
    formatted = row["formatted_blob"] if row else reviewed_blob
    correction = CORRECTIONS.get(path)
    if correction is not None:
        if formatted != correction["before_blob"]:
            raise AssertionError("Semantic correction source does not match formatter: " + path)
        return correction["after_blob"]
    return formatted
