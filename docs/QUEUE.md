# The work queue

Ordered, measured, and resumable. The heartbeat (`tools/parity/loop.sh`, running in the lab container) appends a
measurement to `docs/PARITY-LOG.md` every four minutes, so this list's effect is visible even between steps.

## The bar
`docs/GOAL.md`: a screen is done at **2 000 differing pixels or fewer** in a 50/50 blend against the reference at
1440x900, both clients in the same state.

## Now: Home, empty state -- 49 444 differing pixels
| slab | pixels | what it is | next measurement |
|---|---:|---|---|
| sidebar | 32 246 | banner art placement, banner text metrics, news card's rows | the banner art's own edges in both, then the heading's baseline |
| content | 13 587 | empty-state hero: logo, title, button, two captions | band scan of the hero's five rows in both |
| title bar | 3 344 | wordmark, breadcrumb, update pill, window controls | x-positions of each element in both |
| rail | 267 | done to half a per cent | -- |

## Then, in this order
1. **Discover -- 159 399 differing pixels as first measured**, and the anatomy, not the text, is what differs:
   each of the reference's cards carries a right-hand column with the Install button at its top, the downloads
   and likes under it, and the age under that, where this build puts the age first and the counts after it. The
   first card's green Install plate measures x 939..1098, y 194..259 in the reference against x 1042..1077,
   y 194..202 here -- a different shape, not a different colour.
2. Skins.
3. Screenshots.
4. Servers.
5. Settings, all thirteen tabs.
6. Instance page, all five tabs.
7. Project page.

## Rules that have cost the most time when broken
- Measure both clients **in the same state**, or the number is measuring the data.
- Print the widget's rect from the code; never read a coordinate off a capture.
- A pressed control that does not answer is a bug in the aim until the rect proves otherwise.
- Correct the test when the measurement says so; do not satisfy it with the old number.
