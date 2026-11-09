#!/usr/bin/env python3
"""Compare ONE element between two captures, property by property.

The whole-frame threshold-40 count in `compare.py` cannot see an element whose fill is
close to the page tone, and it punishes enlarging such an element towards the reference's
own size (a 30-point fill step is under the threshold, so only the fringe of an
overlap counts). That left the filter row visibly wrong and measurably "best" in its wrong
form. This tool answers the question that metric cannot: *what colour, what ink, what
size* -- per element.

Usage:
    python3 tools/parity/element.py REF.png OURS.png X0,Y0,X1,Y1 [--name label]

It reports, for each side:
  fill      the most common colour inside the rectangle
  ink       the brightest pixel (the text/icon colour)
  ink rows  the rows carrying ink, and their height
  extent    the rows the fill covers (the element's own height)
and then a verdict per property: match, or by how much it is off.
"""
import sys
from collections import Counter

from PIL import Image


def describe(image, box):
    x0, y0, x1, y1 = box
    pixels = [
        image.getpixel((x, y))
        for y in range(y0, y1)
        for x in range(x0, x1)
    ]
    fill, _ = Counter(pixels).most_common(1)[0]
    brightest = max(pixels, key=lambda p: sum(p))
    # Ink rows: a row counts when it has at least three pixels brighter than the fill by 40.
    rows = []
    for y in range(y0, y1):
        lit = sum(
            1
            for x in range(x0, x1)
            if sum(image.getpixel((x, y))) - sum(fill) > 120
        )
        if lit >= 3:
            rows.append(y)
    # The element's own rows: sample a column just inside its left edge, away from any
    # centred text, and keep the rows that still carry the fill.
    probe_x = x0 + 4
    extent = [
        y
        for y in range(y0, y1)
        if max(abs(image.getpixel((probe_x, y))[i] - fill[i]) for i in range(3)) <= 6
    ]
    return {
        "fill": fill,
        "brightest": brightest,
        "ink_rows": (rows[0], rows[-1], len(rows)) if rows else None,
        "extent": (extent[0], extent[-1]) if extent else None,
    }


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    name = ""
    for i, a in enumerate(sys.argv):
        if a == "--name" and i + 1 < len(sys.argv):
            name = sys.argv[i + 1]
            args = [b for b in args if b != name]
    ref_path, ours_path, box_spec = args[:3]
    box = tuple(int(v) for v in box_spec.split(","))
    ref = describe(Image.open(ref_path).convert("RGB"), box)
    ours = describe(Image.open(ours_path).convert("RGB"), box)
    print(f"{name or 'element'} in {box}:")
    for key in ("fill", "brightest", "ink_rows", "extent"):
        print(f"  {key:<10} ref {ref[key]}\n  {'':<10} our {ours[key]}")
    for key in ("fill", "brightest"):
        delta = max(abs(ref[key][i] - ours[key][i]) for i in range(3))
        print(f"  {key:<10} off by {delta}")
    if ref["ink_rows"] and ours["ink_rows"]:
        print(
            "  ink rows   off by "
            f"{ours['ink_rows'][0] - ref['ink_rows'][0]:+d} top, "
            f"{ours['ink_rows'][1] - ref['ink_rows'][1]:+d} bottom, "
            f"{ours['ink_rows'][2] - ref['ink_rows'][2]:+d} tall"
        )
    if ref["extent"] and ours["extent"]:
        print(
            "  extent     off by "
            f"{ours['extent'][0] - ref['extent'][0]:+d} top, "
            f"{ours['extent'][1] - ref['extent'][1]:+d} bottom"
        )


if __name__ == "__main__":
    main()
