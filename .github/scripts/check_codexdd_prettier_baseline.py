#!/usr/bin/env python3
"""Guard the exact frozen CodexDD 0.4.3 Prettier baseline.

Legacy audit evidence is not silently autoformatted as part of an unrelated
0.4.4 runtime hotfix. Every excluded blob remains byte-for-byte checked.
"""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
MANIFEST = ROOT / ".github/codexdd-prettier-frozen-baseline.json"
IGNORE = ROOT / ".prettierignore"
START = "# BEGIN CODEXDD-043-FROZEN-PRETTIER-BASELINE"
END = "# END CODEXDD-043-FROZEN-PRETTIER-BASELINE"


def main() -> None:
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    if manifest.get("schema") != 1:
        raise SystemExit("Unsupported frozen formatting baseline schema")

    entries = manifest["files"]
    ignored = IGNORE.read_text(encoding="utf-8").splitlines()
    if ignored.count(START) != 1 or ignored.count(END) != 1:
        raise SystemExit("Frozen baseline ignore markers must occur once")
    start = ignored.index(START)
    end = ignored.index(END)
    if end <= start or set(ignored[start + 1:end]) != set(entries):
        raise SystemExit("Frozen Prettier exclusions do not match audited manifest")

    for relpath, expected in sorted(entries.items()):
        path = ROOT / relpath
        if not path.is_file():
            raise SystemExit(f"Frozen source missing: {relpath}")
        content = path.read_bytes()
        digest = hashlib.sha1(
            f"blob {len(content)}".encode("ascii") + bytes([0]) + content
        ).hexdigest()
        if digest != expected:
            raise SystemExit(
                f"Frozen baseline changed: {relpath}; "
                "format and rebaseline in a separate reviewed change"
            )
    print(f"Verified {len(entries)} frozen CodexDD 0.4.3 format-baseline blobs")


if __name__ == "__main__":
    main()
