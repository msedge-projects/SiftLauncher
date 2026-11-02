#!/usr/bin/env bash
# The unattended heartbeat, run INSIDE the lab container (where it survives between agent steps).
#
# One job on a timer: capture the running launcher, compare it with the reference's own capture using
# ImageMagick (the container has no PIL), and append the number to docs/PARITY-LOG.md. It never presses
# anything and never rebuilds -- it reads the screen and writes a line -- so it cannot disturb a drive.
set -u
CAP=/scratch/.scratch/cap
LOG=/scratch/docs/PARITY-LOG.md
REF=$CAP/ab-ref-home.png
INTERVAL=${INTERVAL:-300}

while true; do
  stamp=$(date '+%Y-%m-%d %H:%M:%S')
  shot=$CAP/loop-$(date '+%H%M%S').png
  if DISPLAY=:99 import -window root "$shot" 2>/dev/null; then
    if [ -f "$REF" ]; then
      metric=$(compare -metric AE -fuzz 15% "$REF" "$shot" null: 2>&1 | head -1 | tr -d "\n")
      printf -- "- %s heartbeat: %s pixels differ (ImageMagick AE, 15%% fuzz)\n" "$stamp" "${metric:-?}" >> "$LOG"
    fi
    ls -1t "$CAP"/loop-*.png 2>/dev/null | tail -n +21 | xargs -r rm -f
  else
    printf -- "- %s heartbeat: no window to capture\n" "$stamp" >> "$LOG"
  fi
  sleep "$INTERVAL"
done
