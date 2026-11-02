# The goal this run is measured against

**A screen is done when a 50/50 blend against the reference at 1440x900 differs by 2 000 pixels or fewer**
(threshold 40 per channel, out of 1 296 000), **with both clients in the same state** -- same page, same data,
same window geometry -- and a capture behind every claim.

## Where each screen stands

| screen | differing pixels | of which |
|---|---:|---|
| Home, empty state | **49 444** | rail 267, title bar 3 344, content 13 587, sidebar 32 246 |
| Home, populated | 73 584 | content 35 371 (the reference was empty here -- not a fair comparison) |

The lesson from the first fair measurement: an A/B against a *different state* is worthless. The reference has no
instances in this lab, so its Home is the empty state, and comparing it with a seven-row library measured the
library, not the parity. `XDG_DATA_HOME=/tmp/emptydata` runs this build empty and makes the comparison honest.
