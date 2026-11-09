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
  ink       the brightest pixel in the box -- **the extreme, not the subject**
  ink rows  the rows carrying ink, and their height
  extent    the rows the fill covers (the element's own height)
and then a verdict per property: match, or by how much it is off.

**`brightest` answers for whichever pixel is lightest, so a box holding two elements answers for the
lighter one.** That has twice been read as the element itself: a magnifier glyph over a placeholder, and a
plate beside a chip. `fill share` is the guard -- if the most common colour is under half the box, the box
is not one element and the verdicts are about the wrong one.
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
    fill, fill_count = Counter(pixels).most_common(1)[0]
    fill_share = fill_count / len(pixels) if pixels else 0.0
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
        "fill_share": fill_share,
        "brightest": brightest,
        "ink_rows": (rows[0], rows[-1], len(rows)) if rows else None,
        "extent": (extent[0], extent[-1]) if extent else None,
    }


def selftest() -> int:
    """**The check for the mistake this tool invites.** A box with one element must report that element;
    a box straddling two must say so rather than answering for the brighter one.
    """
    from PIL import Image as _Image

    failures = 0
    one = _Image.new("RGB", (100, 40), (40, 42, 48))
    for y in range(8, 20):
        for x in range(10, 30):
            one.putpixel((x, y), (200, 200, 200))
    stats = describe(one, (0, 0, 100, 40))
    if stats["fill"] != (40, 42, 48) or stats["fill_share"] < 0.5:
        print(f"FAIL one element: fill {stats['fill']} share {stats['fill_share']:.2f}")
        failures += 1
    if stats["brightest"] != (200, 200, 200):
        print(f"FAIL one element: brightest {stats['brightest']}")
        failures += 1

    # Three bands of near-equal size: no single colour can dominate, which is the case the warning exists
    # for. A 50/50 tie sits exactly on the threshold and is not the test.
    split = _Image.new("RGB", (99, 40), (40, 42, 48))
    for y in range(40):
        for x in range(33, 66):
            split.putpixel((x, y), (30, 32, 36))
        for x in range(66, 99):
            split.putpixel((x, y), (50, 52, 58))
    stats = describe(split, (0, 0, 99, 40))
    if stats["fill_share"] >= 0.5:
        print(f"FAIL three-way box: share {stats['fill_share']:.2f} would not warn")
        failures += 1

    print("selftest: " + ("ok" if failures == 0 else f"{failures} failures"))
    return 1 if failures else 0


def main():
    if "--selftest" in sys.argv:
        sys.exit(selftest())

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
    for key in ("fill", "fill_share", "brightest", "ink_rows", "extent"):
        print(f"  {key:<10} ref {ref[key]}\n  {'':<10} our {ours[key]}")
    for side, stats in (("ref", ref), ("our", ours)):
        if stats["fill_share"] < 0.5:
            print(
                f"  WARNING    the {side} box is not one element: its most common colour is only "
                f"{stats['fill_share'] * 100:.0f}% of it, so `brightest` and `ink rows` answer for whatever else is in there"
            )
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
