#!/usr/bin/env python3
"""Find the brand-green controls in a capture, and print where to press.

Blind coordinates have cost this project more rounds than any bug: a click that lands
four points off a button looks exactly like a dead control. This finds the green fill
instead, so a press is aimed at something the capture proves is there.

usage: at.py CAPTURE [--min-area N]
prints one line per green region: "cx cy w h area"
"""
import sys
from PIL import Image

BRAND = (0x1b, 0xd9, 0x6b)


def is_brand(px):
    r, g, b = px[:3]
    return g > 140 and g > r + 45 and g > b + 45


def regions(path, min_area=400):
    im = Image.open(path).convert("RGB")
    w, h = im.size
    seen = [[False] * w for _ in range(h)]
    out = []
    for y in range(0, h, 2):
        for x in range(0, w, 2):
            if seen[y][x] or not is_brand(im.getpixel((x, y))):
                continue
            stack = [(x, y)]
            seen[y][x] = True
            xs, ys = [], []
            while stack:
                cx, cy = stack.pop()
                xs.append(cx)
                ys.append(cy)
                for dx, dy in ((2, 0), (-2, 0), (0, 2), (0, -2)):
                    nx, ny = cx + dx, cy + dy
                    if 0 <= nx < w and 0 <= ny < h and not seen[ny][nx]:
                        seen[ny][nx] = True
                        if is_brand(im.getpixel((nx, ny))):
                            stack.append((nx, ny))
            if len(xs) * 4 >= min_area:
                out.append((sum(xs) // len(xs), sum(ys) // len(ys),
                            max(xs) - min(xs), max(ys) - min(ys), len(xs) * 4))
    out.sort(key=lambda r: (r[1], r[0]))
    return out


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return
    path = sys.argv[1]
    min_area = 400
    if "--min-area" in sys.argv:
        min_area = int(sys.argv[sys.argv.index("--min-area") + 1])
    for cx, cy, w, h, area in regions(path, min_area):
        print(f"{cx} {cy} {w} {h} {area}")


if __name__ == "__main__":
    main()
