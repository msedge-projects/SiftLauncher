# Definition of finished, and the list of what is left

The bar: every screen below at **2 000 differing pixels or fewer** in a 50/50 blend against the reference at
1440x900, both clients in the same state, with a capture behind every claim — and every control driven at least
once, or recorded as untestable with its cause.

## Measured now
| screen | differing pixels | where it stands |
|---|---:|---|
| Home, empty | 31 171 | rail done (267); title bar 3 091; content ~11 000 (hero text + dotted field); sidebar ~17 000 (banner text) |
| Home, populated | not comparable | the reference's library is empty in this lab |
| Discover | 134 450 | header and card frames aligned; cards' internals and the icons' art are the remainder |
| Skins | not measured | 3D preview exists without overlay layers; cards 3D-drawn |
| Screenshots | not measured | groups store exists, dialog reverted (unexplained presses) |
| Servers | not comparable | the reference gates its own page behind "update required" |
| Settings (13 tabs) | not measured | compared once by eye; needs the same A/B |
| Instance page (5 tabs) | not measured | |
| Project page | not measured | |

## The list, in order
1. **Home to 2 000**: title bar 3 091 -> measure each element; content hero; sidebar banner text.
2. **Discover to 2 000**: card internals against the reference's own card, then the project icons.
3. **Skins**: A/B against a fresh reference capture; overlay layers on the 3D model.
4. **Screenshots**: A/B; then the groups dialog, whose presses could not be made to register.
5. **Servers**: A/B is impossible while the reference refuses its own page; record it as such.
6. **Settings**: thirteen tabs, one A/B each.
7. **Instance page**: five tabs, one A/B each.
8. **Project page**: one A/B, plus the versions pager.
9. **Every control driven once**: the queue's "proved by driving" table is the record.
10. **Docs**: GAP-MATRIX's ~450 open rows closed or given a cause.

## Structural gaps that are known and not yet closed
- The reference's **font rasterisation** differs from egui's: its glyphs are consistently 1-2 points taller at the
  same nominal size, which is where most of the residual comes from on every screen. Measuring a string's advance
  width in both and scaling our sizes to match is the next experiment.
- **Screenshot groups**, **ZIP export**, **drag-and-drop reordering**.
- **Follow / Save / Report / sign-in**: no client for those services; recorded as BLOCKED-SERVICE.
- **accesskit**: the app exposes no accessibility tree, so every drive is by coordinates. Enabling it and naming
  the painted hit regions would make driving deterministic (see agent-desktop note).
