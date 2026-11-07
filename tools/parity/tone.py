#!/usr/bin/env python3
"""How far apart are two captures in *tone*, not in thresholded ink?

The page totals count pixels whose worst channel differs by more than 40. A fill that is
nine points light on every channel therefore counts as zero -- and reads as "at its floor"
in every report, while a side-by-side crop shows it plainly. This measures the drift the
threshold hides.

usage: tone.py REF OURS X0 X1 Y0 Y1 [X0 X1 Y0 Y1 ...]
prints, per region: mean/median per-channel delta, a histogram of the worst-channel delta,
and the commonest colour pair in the 5..39 band -- which is the drift worth fixing.
"""
import sys
from collections import Counter

from PIL import Image


def band(region):
    if region <= 4:
        return "0-4"
    if region <= 9:
        return "5-9"
    if region <= 19:
        return "10-19"
    if region <= 39:
        return "20-39"
    return "40+"


def main():
    if len(sys.argv) < 7 or (len(sys.argv) - 3) % 4 != 0:
        print(__doc__)
        return
    ref = Image.open(sys.argv[1]).convert("RGB")
    ours = Image.open(sys.argv[2]).convert("RGB")
    args = sys.argv[3:]
    for i in range(0, len(args), 4):
        x0, x1, y0, y1 = (int(v) for v in args[i : i + 4])
        hist = Counter()
        pairs = Counter()
        total = 0
        sums = [0, 0, 0]
        for y in range(y0, y1):
            for x in range(x0, x1):
                a = ref.getpixel((x, y))
                b = ours.getpixel((x, y))
                d = [b[k] - a[k] for k in range(3)]
                worst = max(abs(v) for v in d)
                hist[band(worst)] += 1
                for k in range(3):
                    sums[k] += abs(d[k])
                total += 1
                if 5 <= worst <= 39:
                    pairs[(a, b)] += 1
        mean = [round(s / max(total, 1), 2) for s in sums]
        print(f"region {x0},{y0}..{x1},{y1}  mean |d| rgb {mean}  pixels {total}")
        print("   worst-channel histogram: " + "  ".join(f"{k}:{v}" for k, v in sorted(hist.items())))
        for (a, b), n in pairs.most_common(4):
            print(f"   {n:6} px  ref {a} -> ours {b}")


if __name__ == "__main__":
    main()
