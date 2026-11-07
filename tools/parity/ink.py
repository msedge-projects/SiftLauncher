#!/usr/bin/env python3
"""Where does each block's ink start, in two captures?

Blind comparisons of whole pages say "text differs" and stop there. The question that
finds real layout bugs is narrower: for this block, at this x range, on which row does
the first ink appear -- and is that row the same in both pictures?

Measured this way, the "Jump in" heading was 20 points high in one build while the
Library heading below it matched exactly, which no whole-page diff could have said.

usage: ink.py REF OURS LABEL X0 X1 Y0 Y1 [LABEL X0 X1 Y0 Y1 ...]
prints one line per label: "label ref ours delta"
"""
import sys
from PIL import Image


def first_ink(image, x0, x1, y0, y1, threshold=260):
    for y in range(y0, y1):
        hits = 0
        for x in range(x0, x1):
            r, g, b = image.getpixel((x, y))[:3]
            if r + g + b > threshold:
                hits += 1
        if hits > 3:
            return y
    return None


def main():
    if len(sys.argv) < 6 or (len(sys.argv) - 3) % 5 != 0:
        print(__doc__)
        return
    ref = Image.open(sys.argv[1]).convert("RGB")
    ours = Image.open(sys.argv[2]).convert("RGB")
    args = sys.argv[3:]
    for i in range(0, len(args), 5):
        label = args[i]
        x0, x1, y0, y1 = (int(v) for v in args[i + 1 : i + 5])
        a = first_ink(ref, x0, x1, y0, y1)
        b = first_ink(ours, x0, x1, y0, y1)
        delta = "" if a is None or b is None else b - a
        print(f"{label:<22} ref {a}   ours {b}   delta {delta}")


if __name__ == "__main__":
    main()
