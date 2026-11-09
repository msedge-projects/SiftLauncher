#!/usr/bin/env python3
"""Is this page pair aligned? Reports the shift that best matches, and what it would save.

A pixel count at a fixed threshold is not monotone in fidelity: at a small offset two renderings' edges partly
cancel, so a misaligned page can score *better* than an aligned one (measured on Settings: dy +4 scored 27 256
inside the modal where dy 0 scored 31 721). So a count means nothing until the alignment is known, and this is the
measurement that says so.

    python3 tools/parity/align.py REF OURS [--rect L,T,R,B]
"""
import sys

from PIL import Image


def count(a, b, x0, x1, y0, y1, dx, dy):
    n = 0
    for y in range(y0, y1, 2):
        for x in range(x0, x1, 2):
            pa = a[x, y]
            pb = b[x + dx, y + dy]
            if max(abs(pa[i] - pb[i]) for i in range(3)) > 40:
                n += 1
    return n


def main():
    ref = Image.open(sys.argv[1]).convert("RGB")
    our = Image.open(sys.argv[2]).convert("RGB")
    w, h = ref.size
    x0, y0, x1, y1 = 80, 140, w - 320, h - 80
    if "--rect" in sys.argv:
        values = [int(v) for v in sys.argv[sys.argv.index("--rect") + 1].split(",")]
        x0, y0, x1, y1 = values
    best = None
    at_zero = None
    for dy in range(-6, 7):
        for dx in (-2, -1, 0, 1, 2):
            n = count(ref.load(), our.load(), x0, x1, y0, y1, dx, dy)
            if dx == 0 and dy == 0:
                at_zero = n
            if best is None or n < best[0]:
                best = (n, dx, dy)
    print(f"in place {at_zero}  best {best[0]} at dx {best[1]} dy {best[2]}")
    if (best[1], best[2]) != (0, 0):
        print(f"  MISALIGNED: a shift of ({best[1]},{best[2]}) matches better, {100 * (at_zero - best[0]) / max(at_zero, 1):.0f}% fewer")
    else:
        print("  aligned: no shift beats in-place")
    return 0


if __name__ == "__main__":
    sys.exit(main())