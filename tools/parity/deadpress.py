#!/usr/bin/env python3
"""Dead-press candidates: functions that paint heavily and register no press.

The class this finds was named at round 436, after four rounds on one instance of it: a drawing block added to a long
function that collects its `Action` at the end, with no `ui.interact` of its own. The row is drawn and completely dead,
and no pixel count can see it. This is the cheap static half -- a function that mentions `Action::` but never calls
`interact` is a candidate for a press it draws but does not read.
"""
import re
import sys
from pathlib import Path

root = Path(sys.argv[1] if len(sys.argv) > 1 else "crates/sift-ui/src")
fn_start = re.compile(r"^\s*(?:pub )?fn ([a-z_0-9]+)")

findings = []
for path in sorted(root.rglob("*.rs")):
    lines = path.read_text().splitlines()
    current = None
    start = 0
    body = []
    for i, line in enumerate(lines, 1):
        m = fn_start.match(line)
        if m:
            if current and len(body) > 40:
                text = "\n".join(body)
                if "Action::" in text and "interact(" not in text and "clicked()" not in text and "painter." in text:
                    findings.append((len(body), str(path), current, start))
            current, start, body = m.group(1), i, []
        body.append(line)
    if current and len(body) > 40:
        text = "\n".join(body)
        if "Action::" in text and "interact(" not in text and "clicked()" not in text and "painter." in text:
            findings.append((len(body), str(path), current, start))

findings.sort(reverse=True)
print(f"{len(findings)} candidates: functions that name an Action, paint, and register no press")
for size, path, name, line in findings[:14]:
    print(f"  {size:5d} lines  {path}:{line}  fn {name}")