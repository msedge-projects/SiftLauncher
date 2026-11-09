#!/usr/bin/env python3
"""Find OPEN matrix rows whose reference text is already present in our source.

The matrix has hundreds of OPEN rows and a history of some of them being complete and merely unreached. Reading each
row by hand costs twenty minutes, so this looks for the one signal that is hard to fake: a **quoted literal** in the
row's reference cells. Reference copy is verbatim from 0.22.0 -- "Drag and drop to add instances.", "No matching
screenshots" -- so if the same string is in crates/sift-ui/src, the element that shows it almost certainly exists.

    python3 tools/parity/rows.py [--limit 40]

It prints candidates for a human read and changes nothing. A hit is a lead, not a verdict.
"""
import re
import subprocess
import sys
from pathlib import Path

MATRIX = Path("docs/GAP-MATRIX.md")
SOURCE = Path("crates/sift-ui/src")


def rows():
    for line in MATRIX.read_text(encoding="utf-8").splitlines():
        if not re.match(r"^\| \d+ \|", line):
            continue
        cells = [cell.strip() for cell in line.strip().strip("|").split("|")]
        if len(cells) < 9:
            continue
        yield cells[0], cells[1], cells[3], cells[7]


def literals(text):
    """Quoted strings worth searching for: sentence-like and long enough not to be a token name."""
    found = re.findall(r"\"([^\"]{10,90})\"", text)
    found += re.findall(r"“([^”]{10,90})”", text)
    out = []
    for value in found:
        value = value.strip()
        if " " not in value:
            continue
        if value.lower().startswith(("--", "color-", "bg-", "text-")):
            continue
        out.append(value)
    return out


def find(needle):
    try:
        result = subprocess.run(
            ["grep", "-rn", "--fixed-strings", "--include=*.rs", needle, str(SOURCE)],
            capture_output=True, text=True, timeout=60,
        )
    except Exception:
        return []
    return [line for line in result.stdout.splitlines() if line.strip()][:2]


def main():
    limit = 40
    args = sys.argv[1:]
    if "--limit" in args:
        limit = int(args[args.index("--limit") + 1])
    hits = 0
    checked = 0
    for number, element, reference, status in rows():
        if status not in ("OPEN", "OPEN-WITH-CAUSE"):
            continue
        if "absent" not in reference.lower() and "none" not in reference.lower():
            pass
        checked += 1
        for literal in literals(reference):
            found = find(literal)
            if found:
                hits += 1
                print(f"row {number}  {element[:60]}")
                print(f"  literal: {literal!r}")
                print(f"  found:   {found[0][:150]}")
                break
        if hits >= limit:
            break
    print(f"\n{hits} candidates from {checked} OPEN rows")


if __name__ == "__main__":
    main()