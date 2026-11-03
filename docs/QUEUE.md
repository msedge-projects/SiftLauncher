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

## The library page, driven in the reference (2026-11-04)

I drove Modrinth App 0.22.0 on display :107 with a fresh profile, created an instance through its own dialog and
looked at what its Home page actually is. **It is not a list of rows**, which is what this build draws. Captures:
`ref107-library.png`, `ref107-grid.png`, `ref107-step2.png`, `ref107-after-create.png`.

Top to bottom:

1. **"Jump in"** -- a heading with a collapse chevron, then a **dashed-border strip** holding the last-played
   instance: its 48-point icon, the name with a green **"New instance"** badge, a subtitle ("Fabric 26.3 •
   Never played"), a brand **Play** button with its glyph, and a kebab.
2. **"Library"** -- a 26-point heading.
3. A toolbar: a **Search** field with the magnifier at its left and the placeholder "Search", then **"New group"**
   (with a plus-in-square glyph) and **"+ New instance"** (brand, with a plus).
4. A filter row: **"Last played ▾"**, **"Custom group ▾"**, a vertical divider, then **"+ Add filter"**.
5. A **grid of cards**, not rows: each card is a raised plate at radius 12 with a 128-point icon at its top, the
   name under it and the loader+version under that. The first card's plate starts at x 88.

Our Home draws a search box and a list of full-width rows with a Play pill and a kebab per row. Every one of the five
parts above is missing or a different object.

### What this changes
The library section of `pages/home.rs` is a rebuild, not a nudge: the jump-in strip, the toolbar with its three
controls, the filter row, and the card grid. The instance row's internals (icon, name, loader+version) carry over into
the card.

The instance page has a matching lesson from the same session: its `Installing…` state puts a **speed readout with a
pause button** in the header *and* mirrors it in the title bar, and its strip is five tabs by default
(`layout.vue:530`). The tabs are fixed; the install progress is not built.
