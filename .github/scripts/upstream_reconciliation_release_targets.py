"""Pinned release identities for the read-only v2 reconciliation validators.

This data alone cannot authorize source promotion, native validation or merge.
"""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class ReviewedReconciliationTarget:
    target_sha: str
    product_version: str
    source_branch: str


_REVIEWED = {
    "rust-v0.162.0": ReviewedReconciliationTarget(
        "c1382380de69521303b416720a52f42d51af6248",
        "0.4.3",
        "dd/codexdd-v0.4.3-upstream-0.162.0-integration",
    ),
    "rust-v0.163.0": ReviewedReconciliationTarget(
        "83086188ae0614bd882818ff1aa23f3194962012",
        "0.4.5",
        "dd/codexdd-v0.4.5-upstream-0.163.0-integration",
    ),
}


def reviewed_reconciliation_target(tag: object) -> ReviewedReconciliationTarget:
    if type(tag) is not str or tag not in _REVIEWED:
        raise ValueError("unreviewed manually reconciled upstream target")
    return _REVIEWED[tag]
