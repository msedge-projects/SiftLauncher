#!/usr/bin/env python3
"""Resize both clients, prove the *captures*, then hand back two paths.

Two lies this exists to stop, both observed here:
1. `xdotool getwindowgeometry` reports the size that was requested, while `import` can produce another size.
2. An earlier version printed "captured ... (1440x900)" for files it never wrote, because it parsed stdout and never
   looked at the exit code.

Every step now checks its own exit status, the probe must return exactly two integers or the pair is REFUSED, and
`--selftest` points the probe at a file that does not exist to prove the refusal fires.

    python3 tools/parity/drive.py --size 1440x900 --tag home
    python3 tools/parity/drive.py --selftest
"""
import subprocess
import sys
import time

TITLES = {"ref": "Modrinth App", "ours": "Sift Launcher"}
DISPLAYS = {"ref": ":101", "ours": ":100"}
CAPTURE_DIR = "/scratch/.scratch/cap"
PROBE = "import sys; from PIL import Image; im = Image.open(sys.argv[1]); print(im.size[0], im.size[1])"
PLATE_BOX = (1150, 300, 1430, 640)  # the news card inside the sidebar, at 1440x900
PLATE_SIGNATURES = {0: ".scratch/cap/plate0.png", 1: ".scratch/cap/plate1.png"}
PLATE_CHECK = ("import sys", "from PIL import Image",
                "a = Image.open(sys.argv[1]).convert('RGB').crop(" + str(PLATE_BOX) + ")",
                "b = Image.open(sys.argv[2]).convert('RGB')",
                "n = 0",
                "for y in range(b.size[1]):",
                "    for x in range(b.size[0]):",
                "        p, q = a.getpixel((x, y)), b.getpixel((x, y))",
                "        if max(abs(p[i] - q[i]) for i in range(3)) > 40:",
                "            n += 1",
                "print(n)")


def run(cmd):
    """Run one command inside the container, which is where X and xdotool live."""
    return subprocess.run(["docker", "exec", "sift-lab", "bash", "-lc", cmd], capture_output=True, text=True)


def window(display, title):
    found = run('DISPLAY=' + display + ' xdotool search --name "' + title + '" | tail -1')
    return found.stdout.strip() if found.returncode == 0 else ""


def png_size(path):
    """(w, h) of a PNG, or None. None covers every failure: missing, unreadable, unexpected output."""
    probe = subprocess.run(["python3", "-c", PROBE, path], capture_output=True, text=True)
    if probe.returncode != 0:
        return None
    parts = probe.stdout.split()
    if len(parts) != 2:
        return None
    try:
        return int(parts[0]), int(parts[1])
    except ValueError:
        return None


def host_path(container_path):
    """The same file as this script sees it: the container mounts the repo at /scratch."""
    if container_path.startswith("/scratch/"):
        return container_path[len("/scratch/"):]
    return container_path


def news_plate(ref_path):
    """Which of the two news plates the reference is showing, or None when it cannot be told.

    The carousel rotates every ten seconds, so a table's figures depend on a state that changes while it is
    being measured. Signatures are this build's own sidebar at each pinned plate; the closer one wins.
    """
    scores = {}
    for index, signature in PLATE_SIGNATURES.items():
        if not __import__("os").path.exists(signature):
            return None
        probe = subprocess.run(["python3", "-c", "\n".join(PLATE_CHECK), ref_path, signature],
                               capture_output=True, text=True)
        if probe.returncode != 0:
            return None
        try:
            scores[index] = int(probe.stdout.strip())
        except ValueError:
            return None
    best = min(scores, key=lambda key: scores[key])
    other = max(scores, key=lambda key: scores[key])
    if scores[best] * 2 > scores[other]:
        return None
    return best


def selftest():
    missing = host_path("/scratch/.scratch/cap/definitely-not-here-481.png")
    if png_size(missing) is not None:
        print("SELFTEST FAILED: the probe returned a size for a file that does not exist")
        return 1
    print("selftest ok: a missing file reads as None, so an unwritten capture cannot pass")
    # The plate check must tell the two plates apart, not merely answer something.
    for index in (0, 1):
        full = '.scratch/cap/pin' + str(index) + '.png'
        told = news_plate(full)
        if told != index:
            print("SELFTEST FAILED: a capture of plate " + str(index) + " was read as " + str(told))
            return 1
    print("selftest ok: each plate signature reads back as its own index")
    return 0


def main():
    args = sys.argv[1:]
    if "--selftest" in args:
        return selftest()
    size = args[args.index("--size") + 1] if "--size" in args else "1440x900"
    tag = args[args.index("--tag") + 1] if "--tag" in args else "drive"
    want = tuple(int(value) for value in size.split("x"))

    for role, display in DISPLAYS.items():
        wid = window(display, TITLES[role])
        if not wid:
            print(role + ": no window on " + display)
            return 2
        run("DISPLAY=" + display + " xdotool windowsize " + wid + " " + str(want[0]) + " " + str(want[1]))
    time.sleep(3.0)

    captured = {}
    for role, display in DISPLAYS.items():
        wid = window(display, TITLES[role])
        if not wid:
            print(role + ": window vanished")
            return 2
        path = CAPTURE_DIR + "/" + tag + "-" + role + ".png"
        shot = run("DISPLAY=" + display + " import -window " + wid + " " + path)
        if shot.returncode != 0:
            print(role + ": import failed: " + shot.stderr.strip()[:120])
            return 3
        got = png_size(host_path(path))
        if got != want:
            print(role + ": wrote " + path + " but it is " + str(got) + ", wanted " + str(want) + " -- REFUSED")
            return 3
        captured[role] = host_path(path)
        print(role + ": " + captured[role] + " (" + str(got[0]) + "x" + str(got[1]) + ")")
    if want == (1440, 900):
        plate = news_plate(captured["ref"])
        print("reference news plate: " + (str(plate) if plate is not None else "undetermined"))
    print("PAIR OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())