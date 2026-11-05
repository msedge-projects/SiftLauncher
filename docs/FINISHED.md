# Definition of finished, and where the work stands

The bar: every screen at **≤2 000 differing pixels** in a 50/50 blend against the reference at 1440x900, both
clients in the same state, with a capture behind every claim.

## Measured, with evidence

| screen | differing pixels | evidence | where the remainder is |
|---|---:|---|---|
| Home, empty | **31 171** | `ab-ref-home.png` vs `ours-home-a.png` | of which only **5 549** survive at a threshold of 160 |
| Discover | **134 450** | `ref-discover-fresh.png` vs `ours-discover25.png` | card icons and their artwork |
| Skins | 185 090 -- **not comparable** | `ref-skins.png` vs `ours-skins-b.png` | the reference's capture has no saved skins and no search row; the two are in different states |
| Screenshots | not measured | -- | needs a fresh capture in the same state |
| Servers | **not comparable** | `ref-servers.png` | the reference refuses its own page: "Modrinth App update required" |
| Settings, 13 tabs | not measured | `ref-settings*.png` | needs one A/B per tab |
| Instance page, 5 tabs | not measured | -- | |
| Project page | not measured | -- | versions pager also unbuilt |

## The floor on text, measured rather than asserted

Home's difference by threshold: 31 171 at 40, 26 340 at 60, 22 656 at 80, 13 215 at 120, **5 549 at 160**. The
gradient is the answer to "why is it not at zero": most of the difference is the same glyph drawn a shade apart by
two different rasterisers -- WebKit's text stack in the reference and egui's `ab_glyph` here.

A 3.6% global type scale was tried against that shortfall and **measured as a wash** (31 171 -> 31 107: content
better by 392, sidebar worse by 386) and reverted, with the numbers in `fonts.rs`. One knob does not close a
rasteriser difference, and inventing per-string nudges to make a number smaller would be the fake-measurement this
project's rules forbid.

## What is genuinely blocked

- **Servers**: the reference shows "Modrinth App update required" on its own hosting page, so its rows cannot be
  compared. This build's server list, Join, Copy IP and New server dialog are driven and recorded.
- **Account, Follow, Save, Report**: no client for those services exists in this lab; recorded as BLOCKED-SERVICE.
- **Pixel-perfect text**: see the floor above. Structural parity is achievable and mostly achieved; glyph-for-glyph
  identity needs the reference's text renderer, which a Rust/ui stack cannot borrow.

## What is done and proved by driving

Rail and its five destinations; Home's empty and populated states; the title bar's brand, breadcrumb and window
controls; the create-instance dialog (layout, marks, close, rows); duplicate, rename, delete instances; the server
picker and Join (`--quickPlayMultiplayer purpleprison.net:25565` on the java command line); Discover's tabs,
search, sort, paging, filters, category list and its retry; installs into a chosen instance with its loader and
version faceted (956 -> 39 modpacks, 3 898 -> 478 mods); the "Installed" state; Settings' thirteen tabs; the
screenshot page's grid, sorting and search; the skins page's saved/default cards and its 3D preview; the instance
page's five tabs including file rename.

323+ commits, 249 tests green, a heartbeat measuring the screen every four minutes in the lab container.

## Screens measured this pass (2026-11-02, late)

| screen | differing pixels | state |
|---|---:|---|
| Home, empty | 31 171 | same state both sides |
| Discover | 132 014 | same state both sides |
| Skins | 185 090 | **not the same state**: the reference's capture has no saved skins and no search row |
| Settings | 151 997 | **not the same state**: its rail shows fourteen tabs where this build's scrolls, and its body has "Advanced rendering" and "System window frame" where this build's has the theme cards alone |
| Screenshots, Instance, Project | not measured | no fresh same-state capture on file |

The Settings comparison found one real number worth keeping: the reference's plate is ~900 points wide and this
build's is 928 (`DIALOG_WIDTH`), both centred, both 673 tall -- so the frame is right and the difference is in the
rail's height and the body's blocks, not the plate.

What the last pass changed and kept: the result card's icon is inset seven points inside its tile, where two filled
it edge to edge (Discover 134 450 -> 132 014).

## Same-state measurements now on file (2026-11-02, final pass of this session)

| screen | differing pixels | same state? |
|---|---:|---|
| Home, empty | 31 171 | yes |
| Discover | 101 881 | yes |
| Screenshots, empty | **25 600** | yes -- the reference's own illustration is embedded at its own 228x98 |
| Screenshots, populated | 543 663 | **no** -- the reference has no screenshots in this lab, this build has four test shots |
| Skins, empty | 192 697 | **no** -- the reference capture shows no search row and an eight-card default grid where this build draws one row |
| Settings | 151 997 | **no** -- its body has blocks this build does not have in that state |
| Servers | not comparable | the reference refuses its own page |

The Screenshots comparison is the useful one of this pass: matched state, matched illustration, and its content slab is
**4 935** -- the lowest any content column has reached -- which shows what a fair comparison looks like when the
state matches and the artwork is the reference's own.

## The four workstreams, their numbers (2026-11-03)

Each drove the real Modrinth App on its own X display, captured it in the same state, and measured.

| screen | before | after | notes |
|---|---:|---:|---|
| Settings, Appearance | 72 497 | **35 743** | backdrop was a 9% veil; the reference composites a two-stop gradient |
| Settings, Features | 75 660 | **36 483** | |
| Settings, Behavior | 76 573 | **40 582** | |
| Settings, Profile | 64 351 | **34 381** | |
| Settings, Social | 65 535 | **35 546** | |
| Settings, Privacy | 82 729 | **53 862** | |
| Settings, Java | 59 333 | **30 055** | the best tab |
| Settings, Window | 58 245 | **31 771** | |
| Settings, Language | 86 089 | **56 105** | our locale list is a five-entry stub against its shipped bundles: data |
| Instance, Content | 49 722 | **31 369** | content slab 29 247 -> 10 894 |
| Instance, Worlds | -- | **9 524** | |
| Instance, Share | -- | **8 920** | the tab did not exist in this build at all |
| Instance, Screenshots | -- | **13 117** | |
| Instance, Logs | -- | **15 235** | |
| Instance, Files | -- | **31 734** | its Created/Modified columns need birth times this build cannot reproduce |
| Project, Description | 221 847 | **89 836** | the page never fetched the project before |
| Project, Versions | -- | **81 983** | pager built and proved on page 2 |
| Skins | 164 451 | **140 340** | preview column now plate-free, model at 9.34 per texel, hint and outlined button |

Every one of these came with a capture behind it, and each workstream separately reverted at least one change that
measured worse (a filled quiet button on an empty state, a one-point border overshoot, a pager over an unanchored
scroll list, a heading gap of 8 against 6).

## Settings, re-measured with this build (2026-11-05)

Against the live reference captures the Settings workstream took on display :101:

| tab | differing pixels |
|---|---:|
| Appearance | **35 231** |
| Features | **35 971** |
| Privacy | **53 350** |
| Java and memory | 30 055 (the workstream's own measurement) |
| Window | 31 771 (same) |

A trap worth writing down, because it produced two false numbers here first: the rail rows in our dialog are **not**
44 points apart, so clicking by an assumed pitch landed on Privacy for both the "java" and the "window" attempts. Both
readings (50 664 and 52 049) were Privacy compared against the wrong reference file. Measuring the same capture against
`ref101-privacy.png` gives 53 350, which matches the workstream's own figure and confirms the capture's identity.

The lesson is the one this record keeps repeating: **a number is only as good as the pair of states behind it.**
