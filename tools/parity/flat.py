#!/usr/bin/env python3
"""Where the FIXABLE differences are: flat mismatches, clustered.

About three quarters of this project's remaining difference sits on glyph edges and photographs, which the rasteriser and
the sampler own. The rest is flat -- fills, rules, insets, colours -- and that is the part a round can close. This counts
only the flat pixels, into 32-point cells, and prints the heaviest cells so the next fix has an address.

    python3 tools/parity/flat.py REF OURS
"""
import sys
from collections import Counter

from PIL import Image

CELL = 32


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    ref = Image.open(args[0]).convert("RGB")
    our = Image.open(args[1]).convert("RGB")
    # `--rect L,T,R,B` restricts the scan: a modal over a page inherits the page\'s difference, so half a modal page\'s
    # figure can be about the thing behind it (measured on Settings: 27 941 of 55 197).
    box = None
    for i, a in enumerate(sys.argv):
        if a == "--rect" and i + 2 < len(sys.argv):
            values = [int(v) for v in sys.argv[i + 1].split(",")]
            box = (values[0], values[1], values[2], values[3])
    left, top = (box[0], box[1]) if box else (64, 48)
    right, bottom = (box[2], box[3]) if box else (ref.size[0], ref.size[1])
    if ref.size != our.size:
        print("sizes differ:", ref.size, our.size)
        return 2
    w, h = ref.size
    rp, op = ref.load(), our.load()
    cells = Counter()
    total = flat = 0
    for y in range(top, min(bottom, h - 1)):
        for x in range(left, min(right, w - 1)):
            a = rp[x, y]
            b = op[x, y]
            if max(abs(a[i] - b[i]) for i in range(3)) <= 40:
                continue
            total += 1
            vals = [sum(rp[x + dx, y + dy]) // 3 for dy in (-1, 0, 1) for dx in (-1, 0, 1)]
            if max(vals) - min(vals) > 12:
                continue
            flat += 1
            cells[(x // CELL * CELL, y // CELL * CELL)] += 1
    print(f"differing {total}, flat {flat} ({100 * flat / max(total, 1):.0f}%)")
    for (cx, cy), n in cells.most_common(12):
        print(f"  cell x {cx:4d} y {cy:4d}  {n:4d} flat pixels")
    return 0


if __name__ == "__main__":
    sys.exit(main())