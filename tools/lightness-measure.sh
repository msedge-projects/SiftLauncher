#!/bin/bash
# Measure the three budgets a machine can answer for the egui build: startup to first window,
# idle RSS after five seconds, and idle CPU over a ten-second window.
#
# The binary is an argument because the answer differs by build: the debug binary is what every
# capture is taken from, and the release one is what ships. Measuring the wrong one silently is
# how a budget appears to pass while the shipped artifact was never checked.
set -u
export DISPLAY=:99
# The binary defaults to this checkout's own build, not a path from another machine. The
# previous default pointed at a checkout that no longer exists, and the script answered
# idle_rss_mb=0.0 for a file that was not there -- a measurement tool that reports zeroes is
# worse than no tool.
binary="${1:-$(cd "$(dirname "$0")/.." && pwd)/crates/sift-ui/target/release/SiftUi}"
if [ ! -x "$binary" ]; then
    echo "lightness-measure: no executable at $binary -- build it first (cargo build --release)" >&2
    exit 2
fi
name=$(basename "$binary")

pkill -x SiftUi 2>/dev/null
sleep 1

start=$(date +%s%N)
setsid "$binary" > /tmp/measure.log 2>&1 < /dev/null &
pid=$!

# Startup: poll for the window rather than assuming a sleep.
window=""
for _ in $(seq 1 300); do
  window=$(xdotool search --name "Sift Launcher" 2>/dev/null | head -1)
  if [ -n "$window" ]; then break; fi
  sleep 0.02
done
mapped=$(date +%s%N)
echo "binary=$name"
# A window that never appeared measures nothing: the previous version printed idle_rss_mb=0.0 and
# a startup time for a process it could not see, which reads as a pass. Refuse instead.
if [ "${window:-none}" = "none" ]; then
    echo "lightness-measure: no window appeared -- is DISPLAY set (the lab uses :99 inside the container)?" >&2
    exit 2
fi
echo "startup_ms=$(( (mapped - start) / 1000000 )) window=${window:-none}"

# Idle: five seconds to settle, then RSS and a CPU delta over ten seconds.
sleep 5
rss_kb=$(awk '/VmRSS/ {print $2}' /proc/$pid/status 2>/dev/null)
echo "idle_rss_mb=$(awk -v kb="${rss_kb:-0}" 'BEGIN {printf "%.1f", kb/1024}')"

cpu_start=$(awk '{print $14+$15}' /proc/$pid/stat 2>/dev/null)
sleep 10
cpu_end=$(awk '{print $14+$15}' /proc/$pid/stat 2>/dev/null)
ticks=$(( ${cpu_end:-0} - ${cpu_start:-0} ))
echo "idle_cpu_ticks_over_10s=$ticks  (0 means the window drew nothing while idle)"

stat -c 'binary_bytes=%s' "$binary"
