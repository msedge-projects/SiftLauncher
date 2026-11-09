#!/usr/bin/env python3
"""Measure two 1440x900 captures against each other and print the numbers a fix needs.

    compare.py REF OURS [--bands X0 X1] [--threshold N]

Printed: differing pixels overall and per slab, then -- with --bands -- one line per band of the given column
strip, so a row-by-row difference can be read without writing a new script each time.
"""
import sys
from PIL import Image, ImageChops

SLABS_FOR = None

_UNUSED = {
    # **Derived from the capture, not fixed at 1440x900.** The four slabs used to be literals for the one size, so a
    # sweep at 1280, 1366, 1600 or 1920 measured its "sidebar" inside the content column and its "content" inside the
    # rail -- visible as a sidebar term that read 16 816 at 1440 and 1 196 at 1920 for the same page. The proportions
    # are the 1440x900 ones: 48 up top, a 64-point rail, a 1140/1440 split, and the rest the sidebar.
}


def main() -> None:
    ref, ours = sys.argv[1], sys.argv[2]
    threshold = 40
    bands = None
    if "--threshold" in sys.argv:
        threshold = int(sys.argv[sys.argv.index("--threshold") + 1])
    if "--bands" in sys.argv:
        at = sys.argv.index("--bands")
        bands = (int(sys.argv[at + 1]), int(sys.argv[at + 2]), int(sys.argv[at + 3]), int(sys.argv[at + 4]))

    a = Image.open(ref).convert("RGB")
    b = Image.open(ours).convert("RGB")
    if a.size != b.size:
        print(f"size mismatch {a.size} vs {b.size}")
        return
    mask = ImageChops.difference(a, b).convert("L").point(lambda v: 255 if v > threshold else 0)
    total = sum(1 for p in mask.getdata() if p)
    # **The slabs come from the capture.** 48 points of title bar, a 64-point rail, and a sidebar that is a fixed
    # 300-point panel at the right of whatever width this is -- not a fraction of it. The literals that used to sit
    # here were 1440x900 numbers, so every other size measured its sidebar inside the content column.
    width, height = a.size
    slabs = {
        "titlebar": (0, 0, width, 48),
        "rail": (0, 48, 64, height),
        "content": (64, 48, max(64, width - 300), height),
        "sidebar": (max(64, width - 300), 48, width, height),
    }
    parts = " ".join(f"{name} {sum(1 for p in mask.crop(box).getdata() if p)}" for name, box in slabs.items())
    print(f"total {total} | {parts}")

    if bands:
        x0, x1, y0, y1 = bands
        for top in range(y0, y1, 20):
            box = mask.crop((x0, top, x1, min(top + 20, y1)))
            count = sum(1 for p in box.getdata() if p)
            if count:
                print(f"  y {top:>4} {count:>6}")


main()
