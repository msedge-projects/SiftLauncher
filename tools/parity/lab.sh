#!/usr/bin/env bash
# One command that puts both clients on one screen and photographs them.
#
# The reference (Modrinth App 0.22.0) and this launcher are different programs
# with different toolkits, so "identical scale" has to mean one X server, one
# screen size, one fontconfig -- otherwise a 4px difference could be the desktop
# rather than the drawing. Everything therefore runs inside the Arch image in
# tools/parity/Dockerfile.modrinth: Xvfb :99 at 1440x900, the AppImage, and
# SiftUi built there too.
#
# Usage:
#   tools/parity/lab.sh up                 build/start the container, start Xvfb
#   tools/parity/lab.sh ref                (re)launch the reference at 1440x900
#   tools/parity/lab.sh ours               build SiftUi, then (re)launch it
#   tools/parity/lab.sh shoot NAME         capture the screen to .scratch/cap/NAME.png
#   tools/parity/lab.sh both NAME          ref on screen, shoot, ours on screen, shoot
#   tools/parity/lab.sh click X Y          click at a screen coordinate
#   tools/parity/lab.sh move X Y           move the pointer
#   tools/parity/lab.sh key KEYS...        xdotool key (e.g. Return, Escape, ctrl+c)
#   tools/parity/lab.sh type TEXT...       type a literal string
#   tools/parity/lab.sh windows            list windows with ids and geometry
#   tools/parity/lab.sh sh 'CMD'           run a command in the lab container
#
# Every subcommand is idempotent: 'up' twice is a no-op, 'ref' twice restarts the
# client, 'ours' twice rebuilds nothing when the binary is current.
set -euo pipefail
cd "$(dirname "$0")/../.."
ROOT=$(pwd)
NAME=sift-lab
IMAGE=sift-lab:1
SCREEN=1440x900x24

dc() { docker "$@"; }
inside() { docker exec "$NAME" bash -lc "$1"; }
ensure_up() {
  if ! docker ps --format '{{.Names}}' | grep -qx "$NAME"; then
    if ! docker image inspect "$IMAGE" >/dev/null 2>&1; then
      echo "building $IMAGE (first run; several minutes)" >&2
      docker build -f tools/parity/Dockerfile.modrinth -t "$IMAGE" tools/parity >&2
    fi
    docker run -d --name "$NAME" --shm-size=1g -v "$ROOT":/scratch "$IMAGE" >/dev/null
  fi
  if ! inside 'pgrep -x Xvfb >/dev/null'; then
    inside "Xvfb :99 -screen 0 $SCREEN -ac -nolisten tcp > /scratch/.scratch/logs/xvfb.log 2>&1 &
            sleep 1"
  fi
}

case "${1:-}" in
  up) ensure_up; inside 'DISPLAY=:99 xdpyinfo | head -3' ;;
  ref)
    ensure_up
    inside 'pkill -x SiftUi || true; pkill -x ModrinthApp || true; sleep 0.5'
    inside 'cd /scratch/.scratch/dl/squashfs-root && ./AppRun > /scratch/.scratch/logs/ref-app.log 2>&1 &
            sleep 8'
    inside 'for w in $(DISPLAY=:99 xdotool search --name "Modrinth App"); do
              DISPLAY=:99 xdotool windowsize $w 1440 900; DISPLAY=:99 xdotool windowmove $w 0 0; done'
    inside 'sleep 1; DISPLAY=:99 xdotool search --name "Modrinth App" | head -1'
    ;;
  ours)
    ensure_up
    inside 'cd /scratch/crates/sift-ui && cargo build --release 2>&1 | tail -3'
    inside 'pkill -x ModrinthApp || true; pkill -x SiftUi || true; sleep 0.5'
    inside 'cd /scratch/crates/sift-ui && ./target/release/SiftUi > /scratch/.scratch/logs/siftui.log 2>&1 &
            sleep 3'
    inside 'for w in $(DISPLAY=:99 xdotool search --class SiftUi); do
              DISPLAY=:99 xdotool windowsize $w 1440 900; DISPLAY=:99 xdotool windowmove $w 0 0; done
            for w in $(DISPLAY=:99 xdotool search --name "Sift"); do
              DISPLAY=:99 xdotool windowsize $w 1440 900; DISPLAY=:99 xdotool windowmove $w 0 0; done'
    inside 'DISPLAY=:99 xdotool search --name "." | while read w; do echo "$w $(DISPLAY=:99 xdotool getwindowname $w)"; done'
    ;;
  shoot)
    ensure_up
    mkdir -p .scratch/cap
    inside "DISPLAY=:99 import -window root /scratch/.scratch/cap/$2.png"
    python3 tools/parity/probe.py info ".scratch/cap/$2.png" --n 3 | head -4
    ;;
  both)
    "$0" ref; "$0" shoot "${2}-ref"
    "$0" ours; "$0" shoot "${2}-ours"
    python3 tools/parity/probe.py pair ".scratch/cap/${2}-ref.png" \
      ".scratch/cap/${2}-ours.png" ".scratch/cap/${2}-pair.png"
    ;;
  click) ensure_up; inside "DISPLAY=:99 xdotool mousemove $2 $3 click 1" ;;
  move)  ensure_up; inside "DISPLAY=:99 xdotool mousemove $2 $3" ;;
  key)   ensure_up; shift; inside "DISPLAY=:99 xdotool key $*" ;;
  type)  ensure_up; shift; inside "DISPLAY=:99 xdotool type --delay 30 $(printf '%q' "$*")" ;;
  windows)
    ensure_up
    inside 'DISPLAY=:99 xdotool search --name "." | while read w; do
              echo "$w $(DISPLAY=:99 xdotool getwindowname $w) $(DISPLAY=:99 xdotool getwindowgeometry $w | tr "\n" " ")"; done'
    ;;
  sh) inside "$2" ;;
  *) sed -n '1,30p' "$0"; exit 2 ;;
esac
