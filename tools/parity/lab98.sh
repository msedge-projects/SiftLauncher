#!/usr/bin/env bash
# A private copy of tools/parity/lab.sh's client actions on display :98, so this session's
# window does not sit on the display other sessions photograph.
set -euo pipefail
NAME=sift-lab
D=:98
inside() { docker exec "$NAME" bash -lc "$1"; }
ensure() {
  if ! inside "pgrep -f 'Xvfb $D' >/dev/null"; then
    inside "Xvfb $D -screen 0 1440x900x24 -ac -nolisten tcp > /scratch/.scratch/logs/xvfb98.log 2>&1 & sleep 1"
  fi
}
case "${1:-}" in
  up) ensure; inside "DISPLAY=$D xdpyinfo | grep dimensions" ;;
  ref)
    ensure
    inside "pkill -f 'squashfs-root/AppRun' || true; sleep 0.5"
    inside "cd /scratch/.scratch/dl/squashfs-root && (DISPLAY=$D ./AppRun > /scratch/.scratch/logs/ref98.log 2>&1 &); sleep 9"
    inside "for w in \$(DISPLAY=$D xdotool search --class modrinth); do DISPLAY=$D xdotool windowsize \$w 1440 900; DISPLAY=$D xdotool windowmove \$w 0 0; done"
    inside "DISPLAY=$D xdotool search --name '.' | while read w; do echo \"\$w|\$(DISPLAY=$D xdotool getwindowname \$w)|\$(DISPLAY=$D xdotool getwindowgeometry \$w | tr '\\n' ' ')\"; done"
    ;;
  ours)
    ensure
    inside "cd /scratch/crates/sift-ui && CARGO_TARGET_DIR=/scratch/.scratch/target-settings cargo build --release 2>&1 | tail -3"
    inside "pkill -f 'target-settings/release/SiftUi' || true; sleep 0.5"
    inside "cd /scratch/crates/sift-ui && (DISPLAY=$D \${SIFT_ENV:-} ./../.scratch/target-settings/release/SiftUi > /scratch/.scratch/logs/siftui98.log 2>&1 &); sleep 4"
    inside "for w in \$(DISPLAY=$D xdotool search --class SiftUi); do DISPLAY=$D xdotool windowsize \$w 1440 900; DISPLAY=$D xdotool windowmove \$w 0 0; done"
    inside "DISPLAY=$D xdotool search --name '.' | while read w; do echo \"\$w|\$(DISPLAY=$D xdotool getwindowname \$w)\"; done"
    ;;
  shoot)
    ensure; mkdir -p .scratch/cap
    inside "DISPLAY=$D import -window root /scratch/.scratch/cap/$2.png"
    python3 tools/parity/probe.py info ".scratch/cap/$2.png" --n 3 | head -4
    ;;
  click) ensure; inside "DISPLAY=$D xdotool mousemove $2 $3 click 1" ;;
  move) ensure; inside "DISPLAY=$D xdotool mousemove $2 $3" ;;
  key) ensure; shift; inside "DISPLAY=$D xdotool key $*" ;;
  type) ensure; shift; inside "DISPLAY=$D xdotool type --delay 30 \$(printf '%q' \"$*\")" ;;
  wheel) ensure; inside "DISPLAY=$D xdotool mousemove $2 $3 click --repeat $4 --delay 90 5" ;;
  sh) inside "$2" ;;
esac
