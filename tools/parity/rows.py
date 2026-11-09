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


def identifiers(text):
    """Component and icon names as the source writes them, in our snake_case: `StopCircle` -> `stop_circle`.

    The literal pass drained because the strings left are the ones our code mentions only to say it lacks them. A
    *name* is a different signal: `EmptyState`, `ScrollablePanel`, `StopCircle` are what the implementation would be
    called, and a name in our source is a stronger lead than a sentence in a comment.
    """
    found = re.findall(r"\b([A-Z][a-z]+(?:[A-Z][a-z]+)+)\b", text)
    out = []
    for name in found:
        snake = re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()
        out.append(snake)
    return out


# **Which file a row is about.** A name like `empty_state` exists once per page, so a hit anywhere is not evidence
# for a row about one page. The element cell leads with its area, and the area names the file.
AREAS = {
    "sidebar": "panel.rs",
    "screenshots": "screenshots.rs",
    "skins": "skins.rs",
    "settings": "settings.rs",
    "rail": "shell.rs",
    "header": "shell.rs",
    "titlebar": "shell.rs",
    "toggle": "panel.rs",
    "hero": "home.rs",
    "home": "home.rs",
    "card": "home.rs",
    "content": "instance.rs",
    "worlds": "instance.rs",
    "files": "instance.rs",
    "logs": "instance.rs",
    "versions": "instance.rs",
    "instance": "instance.rs",
    "about": "about.rs",
    "servers": "servers.rs",
    "discover": "shell.rs",
    "search": "shell.rs",
}


def scope(element):
    area = element.split("—")[0].strip().lower()
    for key, name in AREAS.items():
        if key in area:
            return name
    return None


def find(needle, only=None):
    try:
        result = subprocess.run(
            ["grep", "-rn", "--fixed-strings", "--include=*.rs", needle, str(SOURCE / only) if only else str(SOURCE)],
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
        only = scope(element)
        if status not in ("OPEN", "OPEN-WITH-CAUSE"):
            continue
        if "absent" not in reference.lower() and "none" not in reference.lower():
            pass
        checked += 1
        found_any = False
        for ident in identifiers(reference):
            hits_lines = find(ident, only)
            real = [h for h in hits_lines if "///" not in h and "//!" not in h]
            if real:
                hits += 1
                print(f"row {number}  {element[:60]}")
                print(f"  identifier: {ident}")
                print(f"  found:      {real[0][:140]}")
                found_any = True
                break
        if found_any:
            if hits >= limit:
                break
            continue
        for literal in literals(reference):
            found = find(literal, only)
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