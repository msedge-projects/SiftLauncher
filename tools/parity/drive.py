#!/usr/bin/env python3
"""Resize both clients to one size, prove it, then capture -- the missing precondition.

Four rounds of this project were lost to a reference window left at 1248x262 while the code assumed 1440x900. The capture
looked valid, the coordinates fell outside the frame, and the failure surfaced as an exception rather than a size mismatch.
This is the guard: set the size, read it back from xdotool, and refuse rather than capture if either window disagrees.

    python3 tools/parity/drive.py [--size 1440x900] [--tag NAME]
"""
import subprocess
import sys

DISPLAYS = {"ref": ":101", "ours": ":100"}
TITLES = {"ref": "Modrinth App", "ours": "Sift Launcher"}
CAPTURES = {"/scratch/.scratch/cap"}


def run(cmd):
    # **Inside the container**: xdotool and the X displays live in `sift-lab`, not on the host this script runs from.
    return subprocess.run(
        ["docker", "exec", "sift-lab", "bash", "-lc", cmd],
        capture_output=True,
        text=True,
    )


def window(display, title):
    out = run(f'DISPLAY={display} xdotool search --name "{title}" | tail -1').stdout.strip()
    return out


def geometry(display, wid):
    out = run(f"DISPLAY={display} xdotool getwindowgeometry --shell {wid}").stdout
    values = dict(
        line.split("=", 1) for line in out.splitlines() if "=" in line
    )
    return int(values.get("WIDTH", 0)), int(values.get("HEIGHT", 0))


def main():
    args = sys.argv[1:]
    size = "1440x900"
    if "--size" in args:
        size = args[args.index("--size") + 1]
    tag = "drive"
    if "--tag" in args:
        tag = args[args.index("--tag") + 1]
    want_w, want_h = (int(v) for v in size.split("x"))
    fails = 0
    for role, display in DISPLAYS.items():
        wid = window(display, TITLES[role])
        if not wid:
            print(f"{role}: no window on {display}")
            return 2
        run(f"DISPLAY={display} xdotool windowsize {wid} {want_w} {want_h}")
    import time
    time.sleep(3.0)
    for role, display in DISPLAYS.items():
        wid = window(display, TITLES[role])
        got_w, got_h = geometry(display, wid)
        ok = (got_w, got_h) == (want_w, want_h)
        print(f"{role}: wanted {want_w}x{want_h}, got {got_w}x{got_h} -- {'ok' if ok else 'MISMATCH'}")
        if not ok:
            fails += 1
        else:
            path = f"/scratch/.scratch/cap/{tag}-{role}.png"
            run(f"DISPLAY={display} import -window {wid} {path}")
            print(f"  captured {path}")
    if fails:
        print("REFUSED: a window did not take the size, so no pair from here is comparable")
        return 3
    return 0


if __name__ == "__main__":
    sys.exit(main())