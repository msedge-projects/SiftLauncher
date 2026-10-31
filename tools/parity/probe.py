#!/usr/bin/env python3
"""Measure a window capture instead of looking at it.

The parity loop in this repo compares the reference client and this launcher by
photograph. A photograph is only evidence if it can be turned into numbers, so
everything here prints numbers: colour runs along a scanline, bounding boxes of
flat regions (buttons, plates, dividers), and the per-tile difference between two
captures of the same screen.

Nothing here guesses. A value it prints is the value of a pixel in the file it
was given; a rectangle it prints is a rectangle whose pixels all share a colour
and touch each other.

Subcommands
-----------
  info PATH                    size, and the N most common colours with coverage
  palette PATH [N]             the same, N defaulting to 12
  row PATH Y [X0 X1]           contiguous colour runs along row Y
  col PATH X [Y0 Y1]           contiguous colour runs along column X
  boxes PATH [--min-area N] [--colors HEX,HEX]
                               bounding boxes of flat regions, largest first
  aim PATH HEX [--tolerance N] white-box the centroid of the region nearest a colour
  compare A B [--tile N]       RMS difference overall and the worst tiles
  pair A B OUT                 side-by-side composite (A left, B right)
"""
from __future__ import annotations

import argparse
import sys
from collections import Counter

try:
    from PIL import Image, ImageChops, ImageDraw, ImageStat
except ImportError:  # pragma: no cover - the lab image installs Pillow
    sys.exit("Pillow is required: pip install pillow")


def load(path):
    return Image.open(path).convert("RGB")


def hx(c):
    return "#%02x%02x%02x" % tuple(c)


def cmd_info(args):
    im = load(args.path)
    print("path", args.path)
    print("size", im.size)
    total = im.size[0] * im.size[1]
    c = Counter(im.getdata())
    for col, n in c.most_common(args.n):
        print("colour %s %8d %6.2f%%" % (hx(col), n, 100.0 * n / total))


def runs(seq, x0, length):
    """Yield (start, end, colour) for each maximal run of one colour in seq."""
    out = []
    start = x0
    prev = seq[0]
    for i in range(1, length):
        if seq[i] != prev:
            out.append((start, x0 + i - 1, prev))
            start = x0 + i
            prev = seq[i]
    out.append((start, x0 + length - 1, prev))
    return out


def print_runs(label, rs, min_len=1):
    for s, e, col in rs:
        if e - s + 1 < min_len:
            continue
        print("%s %5d..%-5d len %4d %s" % (label, s, e, e - s + 1, hx(col)))


def cmd_row(args):
    im = load(args.path)
    w, h = im.size
    y = args.y
    x0 = args.x0 if args.x0 is not None else 0
    x1 = args.x1 if args.x1 is not None else w - 1
    px = [im.getpixel((x, y)) for x in range(x0, x1 + 1)]
    print("row y=%d x=%d..%d  %d runs" % (y, x0, x1, len(runs(px, x0, len(px)))))
    print_runs("run", runs(px, x0, len(px)), args.min_len)


def cmd_col(args):
    im = load(args.path)
    w, h = im.size
    x = args.x
    y0 = args.y0 if args.y0 is not None else 0
    y1 = args.y1 if args.y1 is not None else h - 1
    px = [im.getpixel((x, y)) for y in range(y0, y1 + 1)]
    print("col x=%d y=%d..%d  %d runs" % (x, y0, y1, len(runs(px, y0, len(px)))))
    print_runs("run", runs(px, y0, len(px)), args.min_len)


def cmd_boxes(args):
    """Bounding boxes of runs that are wide enough, merged down columns.

    A flat region of one colour -- a button, a card, a rail -- produces a run on
    every row it occupies. Grouping identical runs across adjacent rows gives the
    region's rectangle without any thresholding guess.
    """
    im = load(args.path)
    w, h = im.size
    want = None
    if args.colors:
        want = {tuple(int(c[i:i + 2], 16) for i in (1, 3, 5)) for c in args.colors.split(",")}
    open_regions = []  # [x0, x1, y0, y1, colour]
    done = []
    prev = []
    for y in range(h):
        row = [im.getpixel((x, y)) for x in range(w)]
        cur = [r for r in runs(row, 0, w) if (want is None or r[2] in want)]
        used = set()
        nxt = []
        for s, e, col in cur:
            if e - s + 1 < args.min_width:
                continue
            match = None
            for idx, reg in enumerate(open_regions):
                if idx in used:
                    continue
                if reg[4] == col and abs(reg[0] - s) <= args.slack and abs(reg[1] - e) <= args.slack:
                    match = idx
                    break
            if match is None:
                nxt.append([s, e, y, y, col])
            else:
                reg = open_regions[match]
                reg[3] = y
                used.add(match)
                nxt.append(reg)
        for idx, reg in enumerate(open_regions):
            if idx not in used:
                done.append(reg)
        open_regions = nxt
        prev = cur
    done.extend(open_regions)
    done = [r for r in done if (r[1] - r[0] + 1) * (r[3] - r[2] + 1) >= args.min_area]
    done.sort(key=lambda r: -((r[1] - r[0] + 1) * (r[3] - r[2] + 1)))
    for x0, x1, y0, y1, col in done[: args.limit]:
        bw, bh = x1 - x0 + 1, y1 - y0 + 1
        print("box x %4d..%-4d y %4d..%-4d  %4dx%-4d  area %7d  %s  centre (%d,%d)"
              % (x0, x1, y0, y1, bw, bh, bw * bh, hx(col), (x0 + x1) // 2, (y0 + y1) // 2))


def cmd_aim(args):
    im = load(args.path)
    w, h = im.size
    want = tuple(int(args.hex[i:i + 2], 16) for i in (1, 3, 5))
    sx = sy = n = 0
    for y in range(h):
        for x in range(w):
            p = im.getpixel((x, y))
            if all(abs(p[i] - want[i]) <= args.tolerance for i in range(3)):
                sx += x
                sy += y
                n += 1
    if not n:
        print("no pixel within %d of %s" % (args.tolerance, args.hex))
        return
    print("aim %s: %d px, centroid (%d,%d)" % (args.hex, n, sx // n, sy // n))


def cmd_compare(args):
    a, b = load(args.path_a), load(args.path_b)
    if a.size != b.size:
        print("SIZE MISMATCH ref %s ours %s -- resize before comparing" % (a.size, b.size))
        return
    diff = ImageChops.difference(a, b)
    stat = ImageStat.Stat(diff)
    overall = sum(stat.rms) / 3.0
    print("rms overall %.3f  (mean %s)" % (overall, "/".join("%.1f" % m for m in stat.mean)))
    t = args.tile
    tiles = []
    for y in range(0, a.size[1], t):
        for x in range(0, a.size[0], t):
            box = (x, y, min(x + t, a.size[0]), min(y + t, a.size[1]))
            st = ImageStat.Stat(diff.crop(box))
            tiles.append((sum(st.rms) / 3.0, box))
    tiles.sort(key=lambda p: -p[0])
    print("worst %d tiles of %dpx:" % (args.top, t))
    for score, box in tiles[: args.top]:
        print("  rms %7.2f  x %4d..%-4d y %4d..%-4d" % (score, box[0], box[2], box[1], box[3]))
    same = sum(1 for s, _ in tiles if s < 1.0)
    print("tiles matching within rms<1: %d/%d (%.1f%%)" % (same, len(tiles), 100.0 * same / len(tiles)))


def cmd_pair(args):
    a, b = load(args.path_a), load(args.path_b)
    gap = 8
    out = Image.new("RGB", (a.size[0] + b.size[0] + gap, max(a.size[1], b.size[1])), (255, 0, 255))
    out.paste(a, (0, 0))
    out.paste(b, (a.size[0] + gap, 0))
    d = ImageDraw.Draw(out)
    d.text((4, 4), "ref", fill=(255, 0, 255))
    d.text((a.size[0] + gap + 4, 4), "ours", fill=(255, 0, 255))
    out.save(args.out)
    print("wrote", args.out, out.size)


RAMP = " .:-=+*#%@"


def cmd_map(args):
    """A text picture of the window: one character per block of pixels.

    There is no eyeball in the loop here, so this is the closest thing to one: a
    luminance ramp sampled at block centres, plus an optional colour-class map
    that prints the palette index of each block instead. A layout question ("is
    the rail 60 or 64 wide") is answered by the row subcommand, not by this; this
    answers "what is where".
    """
    im = load(args.path)
    w, h = im.size
    cols = args.cols
    bw = max(1, w // cols)
    bh = max(1, int(bw * args.aspect))
    rows = h // bh
    pal = Counter(im.getdata()).most_common(args.colours)
    classes = [c for c, _ in pal]
    print("block %dx%d -> %d cols x %d rows" % (bw, bh, cols, rows))
    print("classes: " + " ".join("%d=%s" % (i, hx(c)) for i, c in enumerate(classes)))
    for ry in range(rows):
        line = []
        for rx in range(cols):
            x = min(w - 1, rx * bw + bw // 2)
            y = min(h - 1, ry * bh + bh // 2)
            p = im.getpixel((x, y))
            if args.classes:
                best, dist = 0, 1 << 30
                for i, c in enumerate(classes):
                    d = sum((p[k] - c[k]) ** 2 for k in range(3))
                    if d < dist:
                        best, dist = i, d
                line.append("%X" % best if best < 16 else "+")
            else:
                lum = (0.2126 * p[0] + 0.7152 * p[1] + 0.0722 * p[2]) / 255.0
                line.append(RAMP[min(len(RAMP) - 1, int(lum * (len(RAMP) - 1) + 0.5))])
        print("%4d|%s" % (ry * bh, "".join(line)))


def cmd_text(args):
    """OCR the capture and print every word with the box it occupies.

    A photograph can be read as well as measured: tesseract returns each word and
    its bounding box, which is the closest a capture gets to a DOM inspector. Text
    that is present but in the wrong place shows up here as the same word at two
    different y values, and copy that differs shows up as two different words.

    Light-on-dark is inverted before recognition because tesseract expects dark
    glyphs on light paper.
    """
    import subprocess
    import tempfile
    im = load(args.path)
    if args.crop:
        x0, y0, x1, y1 = (int(v) for v in args.crop.split(","))
        im = im.crop((x0, y0, x1, y1))
        ox, oy = x0, y0
    else:
        ox, oy = 0, 0
    if args.scale != 1.0:
        im = im.resize((int(im.size[0] * args.scale), int(im.size[1] * args.scale)))
    grey = im.convert("L").point(lambda v: 255 - v)
    with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as fh:
        grey.save(fh.name)
        out = subprocess.run(
            ["tesseract", fh.name, "-", "--psm", str(args.psm), "tsv"],
            capture_output=True,
            text=True,
        ).stdout
    rows = []
    for line in out.splitlines()[1:]:
        parts = line.split("\t")
        if len(parts) < 12:
            continue
        word = parts[11].strip()
        if not word:
            continue
        try:
            conf = float(parts[10])
        except ValueError:
            conf = -1.0
        if conf < args.min_conf:
            continue
        left, top, w, h = (int(parts[6]), int(parts[7]), int(parts[8]), int(parts[9]))
        x = ox + int(left / args.scale)
        y = oy + int(top / args.scale)
        ww = int(w / args.scale)
        hh = int(h / args.scale)
        rows.append((y, x, word, ww, hh, conf))
    rows.sort()
    for y, x, word, ww, hh, conf in rows:
        print("x %4d y %4d w %3d h %2d conf %5.1f  %s" % (x, y, ww, hh, conf, word))
    print("%d words" % len(rows))


def cmd_bbox(args):
    """The bounding box of every pixel within a tolerance of one colour.

    `boxes` finds flat regions row by row and is good for plates; this answers the
    blunter question a layout comparison asks -- where is the green thing, exactly --
    and it is the one to cite when a number in `geometry.rs` or a page has to be
    checked against the reference.
    """
    im = load(args.path)
    if args.crop:
        x0, y0, x1, y1 = (int(v) for v in args.crop.split(","))
        im = im.crop((x0, y0, x1, y1))
    else:
        x0 = y0 = 0
        x1, y1 = im.size
    want = tuple(int(args.hex[i:i + 2], 16) for i in (1, 3, 5))
    minx, miny, maxx, maxy, n = 1 << 30, 1 << 30, -1, -1, 0
    for y in range(im.size[1]):
        for x in range(im.size[0]):
            p = im.getpixel((x, y))
            if all(abs(p[i] - want[i]) <= args.tolerance for i in range(3)):
                n += 1
                minx = min(minx, x); maxx = max(maxx, x)
                miny = min(miny, y); maxy = max(maxy, y)
    if not n:
        print("no pixel within %d of %s" % (args.tolerance, args.hex))
        return
    print("bbox %s: x %d..%d (%d) y %d..%d (%d) px %d"
          % (args.hex, x0 + minx, x0 + maxx, maxx - minx + 1, y0 + miny, y0 + maxy,
             maxy - miny + 1, n))


def main():
    p = argparse.ArgumentParser()
    sub = p.add_subparsers(dest="cmd", required=True)

    s = sub.add_parser("info")
    s.add_argument("path")
    s.add_argument("--n", type=int, default=10)
    s.set_defaults(func=cmd_info)

    s = sub.add_parser("palette")
    s.add_argument("path")
    s.add_argument("n", nargs="?", type=int, default=12)
    s.set_defaults(func=lambda a: cmd_info(argparse.Namespace(path=a.path, n=a.n)))

    for name, fn in (("row", cmd_row), ("col", cmd_col)):
        s = sub.add_parser(name)
        s.add_argument("path")
        s.add_argument("y" if name == "row" else "x", type=int)
        s.add_argument("x0" if name == "row" else "y0", nargs="?", type=int)
        s.add_argument("x1" if name == "row" else "y1", nargs="?", type=int)
        s.add_argument("--min-len", type=int, default=2)
        s.set_defaults(func=fn)

    s = sub.add_parser("boxes")
    s.add_argument("path")
    s.add_argument("--min-area", type=int, default=400)
    s.add_argument("--min-width", type=int, default=8)
    s.add_argument("--slack", type=int, default=1)
    s.add_argument("--colors")
    s.add_argument("--limit", type=int, default=40)
    s.set_defaults(func=cmd_boxes)

    s = sub.add_parser("aim")
    s.add_argument("path")
    s.add_argument("hex")
    s.add_argument("--tolerance", type=int, default=6)
    s.set_defaults(func=cmd_aim)

    s = sub.add_parser("compare")
    s.add_argument("path_a")
    s.add_argument("path_b")
    s.add_argument("--tile", type=int, default=32)
    s.add_argument("--top", type=int, default=10)
    s.set_defaults(func=cmd_compare)

    s = sub.add_parser("pair")
    s.add_argument("path_a")
    s.add_argument("path_b")
    s.add_argument("out")
    s.set_defaults(func=cmd_pair)

    s = sub.add_parser("map")
    s.add_argument("path")
    s.add_argument("--cols", type=int, default=96)
    s.add_argument("--aspect", type=float, default=2.1)
    s.add_argument("--colours", type=int, default=8)
    s.add_argument("--classes", action="store_true")
    s.set_defaults(func=cmd_map)

    s = sub.add_parser("text")
    s.add_argument("path")
    s.add_argument("--psm", type=int, default=11)
    s.add_argument("--scale", type=float, default=1.0)
    s.add_argument("--crop")
    s.add_argument("--min-conf", type=float, default=40.0)
    s.set_defaults(func=cmd_text)

    s = sub.add_parser("bbox")
    s.add_argument("path")
    s.add_argument("hex")
    s.add_argument("--tolerance", type=int, default=3)
    s.add_argument("--crop")
    s.set_defaults(func=cmd_bbox)

    args = p.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
