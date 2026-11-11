#!/bin/bash
# **Test first, then commit.** Four times this stretch a red suite was pushed and a follow-up commit was needed;
# each time the test was right. This script refuses to commit unless the suite passes, so the habit is structural
# rather than remembered.
set -u
msg="${1:?usage: commit.sh <message> <path>...}"
shift
if [ "$#" -eq 0 ]; then echo "commit.sh: no paths given"; exit 2; fi
out=$(docker exec sift-lab bash -lc "cd /scratch/crates/sift-ui && cargo test --release 2>&1 | tail -3")
echo "$out"
if ! echo "$out" | grep -q "test result: ok"; then
  echo "commit.sh: REFUSED -- the suite is not green; nothing committed"
  exit 1
fi
git add "$@" || exit 3
git -c user.email=parity@sift.local -c user.name="Sift parity" commit -m "$msg" | tail -1