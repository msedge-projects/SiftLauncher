#!/usr/bin/env python3
"""Capture a **verified** pair of screenshots.

Three rounds of this project were lost to a pair that was not a pair: a modal left open on one
side, a different page on each, and a reference window left at 1440x625 while this build was at
1440x900. Crops of mismatched windows look plausible, and compare.py never sees them because the
comparison is run by hand afterwards. This does the whole sequence and refuses to hand back a
number unless the two captures agree on size.

    python3 tools/parity/pair.py NAME [--size 1440x900] [--rail 65] [--settle 3]

NAME becomes .scratch/cap/NAME-ref.png and .scratch/cap/NAME-our.png.
--rail presses that y on both rails first, so both clients land on the same page.
"""
import subprocess
import sys
from pathlib import Path

from PIL import Image

ROOT = ".scratch/cap"
REF_DISPLAY, OUR_DISPLAY = ":101", ":100"
REF_NAME = "Modrinth App"
OUR_NAME = "Sift Launcher"


def run(cmd, timeout=300):
    return subprocess.run(
        ["docker", "exec", "sift-lab", "bash", "-lc", cmd],
        capture_output=True, text=True, timeout=timeout,
    ).stdout.strip()


def main():
    args = sys.argv[1:]
    name = args[0]
    size = "1440x900"
    rail = None
    settle = 3
    if "--size" in args:
        size = args[args.index("--size") + 1]
    if "--rail" in args:
        rail = int(args[args.index("--rail") + 1])
    if "--settle" in args:
        settle = int(args[args.index("--settle") + 1])
    width, height = (int(v) for v in size.split("x"))

    ref = "$(DISPLAY=%s xdotool search --name %r | tail -1)" % (REF_DISPLAY, REF_NAME)
    our = "$(DISPLAY=%s xdotool search --name %r | tail -1)" % (OUR_DISPLAY, OUR_NAME)
    steps = [
        "WR=" + ref + "; WO=" + our,
        "DISPLAY=%s xdotool windowsize $WR %d %d" % (REF_DISPLAY, width, height),
        "DISPLAY=%s xdotool windowsize $WO %d %d" % (OUR_DISPLAY, width, height),
        "DISPLAY=%s xdotool windowmove $WR 0 0" % REF_DISPLAY,
        "DISPLAY=%s xdotool windowmove $WO 0 0" % OUR_DISPLAY,
        "sleep %d" % settle,
        "DISPLAY=%s xdotool windowfocus $WR" % REF_DISPLAY,
        "DISPLAY=%s xdotool windowfocus $WO" % OUR_DISPLAY,
    ]
    if rail is not None:
        steps += [
            "DISPLAY=%s xdotool mousemove 28 %d" % (REF_DISPLAY, rail),
            "DISPLAY=%s xdotool mousemove 28 %d" % (OUR_DISPLAY, rail),
            "sleep 1",
            "DISPLAY=%s xdotool click 1" % REF_DISPLAY,
            "DISPLAY=%s xdotool click 1" % OUR_DISPLAY,
            "sleep %d" % settle,
        ]
    steps += [
        "DISPLAY=%s xdotool mousemove 700 %d" % (REF_DISPLAY, height - 30),
        "DISPLAY=%s xdotool mousemove 700 %d" % (OUR_DISPLAY, height - 30),
        "sleep 2",
        "DISPLAY=%s import -window $WR /scratch/%s/%s-ref.png" % (REF_DISPLAY, ROOT, name),
        "DISPLAY=%s import -window $WO /scratch/%s/%s-our.png" % (OUR_DISPLAY, ROOT, name),
    ]
    run("; ".join(steps))

    ref_path = Path(ROOT) / (name + "-ref.png")
    our_path = Path(ROOT) / (name + "-our.png")
    ref_im, our_im = Image.open(ref_path), Image.open(our_path)
    print("ref  %s %s" % (ref_im.size, ref_path))
    print("ours %s %s" % (our_im.size, our_path))
    if ref_im.size != our_im.size:
        print("REFUSED: different sizes, so any number would be meaningless")
        return 2
    # **The other half of the check: the same page.** The breadcrumb sits at x 120..640, y 4..44 on both.
    # A modal on one side, or a different destination, shows up here first and costs nothing to look at.
    from PIL import ImageChops
    ref_crumb = ref_im.convert("RGB").crop((110, 4, min(700, width), 44))
    our_crumb = our_im.convert("RGB").crop((110, 4, min(700, width), 44))
    ref_crumb.save(Path(ROOT) / (name + "-crumb-ref.png"))
    our_crumb.save(Path(ROOT) / (name + "-crumb-our.png"))
    diff = ImageChops.difference(ref_crumb, our_crumb).convert("L")
    lit = sum(1 for p in diff.getdata() if p > 40)
    print("breadcrumb differing pixels: %d of %d" % (lit, ref_crumb.size[0] * ref_crumb.size[1]))

    # **The state, not just the page.** A sweep two rounds ago pressed this build's "Light" theme card and the choice
    # persisted; the next capture read 829 926 against a 46 204 baseline and the page was unrecognisable -- the client was
    # in light mode and the reference in dark. Sizes and breadcrumbs both passed. So compare the two frames' own mean
    # brightness, which separates a light theme from a dark one by a wide margin and moves by a couple of points for
    # anything a real layout change does.
    def mean_luma(im):
        grey = im.convert("L").resize((64, 40))
        pixels = list(grey.getdata())
        return sum(pixels) / max(len(pixels), 1)

    luma_ref, luma_our = mean_luma(ref_im), mean_luma(our_im)
    print("mean brightness: ref %.1f, ours %.1f" % (luma_ref, luma_our))
    if abs(luma_ref - luma_our) > 25.0:
        print(
            "REFUSED: the two frames differ by %.1f in mean brightness -- one client is in a different theme, "
            "and every count from this pair would measure that instead of the page" % abs(luma_ref - luma_our)
        )
        return 3
    # Calibrated: the same page reads 419 of 23 600 (1.8%) from glyph rendering alone, so 15% is the floor at
    # which the words themselves must differ.
    if lit > 0.15 * ref_crumb.size[0] * ref_crumb.size[1]:
        print("WARNING: the breadcrumbs differ -- check %s-crumb-ref.png and %s-crumb-our.png before trusting a number" % (name, name))

    if ref_im.size != (width, height):
        print("WARNING: asked %dx%d, both came back %s -- window manager adjustment" % (width, height, ref_im.size))
    return 0


if __name__ == "__main__":
    sys.exit(main())