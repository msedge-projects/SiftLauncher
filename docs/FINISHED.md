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

### The two readings that were wrong, now measured correctly

Our settings rail's rows are **45 points apart**, measured from the capture's own ink bands (`m-appearance.png`, x
300..470): DISPLAY 220, Appearance 257, Features 302, Behavior 347, Language 392, ACCOUNT 438, Profile 477, Social
520, Privacy 565, INSTANCE DEFAULTS 612, **Java and memory 652**, **Window 694**, Launch hooks 733.

Clicking by those numbers:

| tab | differing pixels | against |
|---|---:|---|
| Java and memory | **29 544** | `ref101-java.png` |
| Window | **31 259** | `ref101-window.png` |

Both corroborate the workstream's own figures (30 055 and 31 771) to within a few hundred pixels, which is what two
independent drives of the same pair of states should do. The earlier 50 664 and 52 049 were Privacy compared against
the wrong reference, caused by assuming a 44-point pitch the dialog does not have.

### All thirteen Settings tabs, and the defect that hid four of them

Measured against the live reference captures (`ref101-<tab>.png`), this build, 1440x900:

| tab | differing pixels |
|---|---:|
| Java and memory | **29 544** |
| Window | **31 259** |
| Profile | **33 869** |
| Social | **35 034** |
| Appearance | **35 231** |
| Features | **35 971** |
| Behavior | **40 070** |
| Language | **55 593** |
| Privacy | **53 350** |
| Launch hooks, Syncing, Resources, Troubleshooting | **not trustworthily measured** |

**The defect behind the last row**: our settings rail holds fourteen rows that need ~630 points, and the window's plate
is 668 tall, so the SYSTEM group (Resources, Troubleshooting) falls below the visible list -- and **the rail does not
scroll**. Four wheel steps over it move nothing (measured: the ink bands at x 300..470 are identical before and
after). The reference's rail scrolls; this one draws the rows off the bottom and offers no way to reach them.

The four readings I took for those tabs (56 721, 46 278, 67 013, 56 104) are recorded as **unverified**, because the
clicks that produced them could only have landed on whichever rows were visible -- the Resources capture's body was
"App cache / App database backups / Debug info", which is not the Resources page.

### The settings rail, after an attempt to fix it (open)

Two measurements settled what the rail actually is, and one attempt is recorded as failed:

- The reference's rail rows are **44 to 46 points apart** (its own capture: 257, 301, 348, 392, 438, 476 for
  Appearance, Features, Behavior, Language, ACCOUNT, Profile). A "30-point pitch" reading I took earlier came from the
  wrong element; tightening our rows to it made the dialog measurably **worse** (Appearance 35 231 -> 40 992) and is
  reverted with that number in the source.
- Fourteen rows at 44 cannot fit the 668-point plate, so the reference's rail scrolls. **Ours still does not**: five
  wheel notches with the pointer over the rail leave its ink bands at 218/257/300/345/392, byte for byte where they
  were. The handler now reads `MouseWheel` events directly instead of `smooth_scroll_delta` (which is only filled when
  a scroll area consumes it), and that alone did not move it. The SYSTEM group -- Resources and Troubleshooting --
  therefore remains unreachable, and those two tabs remain unmeasured.

Next attempt is to give the rail a real `ScrollArea` over its clipped pane rather than hand-rolling the offset.

### The settings rail scrolls now, but a press after scrolling still lands on the wrong row (open)

What changed and what it proved:

- The rail's offset is drawn inside an **`egui::Area`** with a real `ScrollArea`. Before that, three wheel notches
  over the rail moved **neither the rail nor the body** -- the events were dropped, because a child ui of the dialog
  inherits its clip and registers no scroll. With the Area, five notches bring the SYSTEM group into view:
  `rail-area.png` shows Privacy, INSTANCE DEFAULTS, Java and memory, Window, Launch hooks, Syncing, SYSTEM, **Resources,
  Troubleshooting**.
- The offset is also kept in the page's state and fed back with `vertical_scroll_offset`, because left to the
  ScrollArea's own memory the rows moved for one frame and snapped back.

**Still wrong**: pressing the row that *reads* "Resources" after scrolling selects Appearance (measured twice; the pane
comes back "Color theme"). So the drawn rows and the rows the press is tested against are not using the same offset.
Until that is fixed, Resources and Troubleshooting remain **unmeasured**, and the honest count of verified Settings tabs
stays at nine.

### Fixed: the rail's offset had two owners, and the second one kept resetting it

The click-path block clamped `rail_scroll` against its own overflow figure, and that figure could be zero -- so every
frame it put the list back at the top *after* `draw_rail` had scrolled it. The rows were drawn scrolled and the press
was tested unscrolled: pressing the row that read "Resources" selected Appearance, twice. With that block removed and
`draw_rail` as the only owner, pressing Resources selects Resources.

**Resources and Troubleshooting are now measured** against the live reference:

| tab | differing pixels |
|---|---:|
| Resources | **73 597** |
| Troubleshooting | **56 104** |

Both replace earlier figures that came from presses landing elsewhere (93 835 and an unverified 56 104 -- the second
turns out to have been right, which is why it is recorded as corroborated rather than as a fresh number).

The Resources pane also differs in *state*, not shape: the reference's store was empty ("0 bytes stored", a grey bar)
while this build's holds 2.5 MB ("Cache 2.5 MB", a green bar). That difference is honest and stays.

### The rail's rows are now the rows it draws (fixed for good), and all thirteen tabs are measured

The mismatch was structural, not arithmetic: `draw_rail` computed its rows from the offset **after** the scroll area ran,
while the press was tested against a **second** call to `rail_rows` earlier in the frame. So the row that read "Launch
hooks" selected Troubleshooting, and the one that read "Resources" selected Appearance. `draw_rail` now hands back the
list it drew, the page keeps it in `state.drawn_rail`, and the press is tested against exactly that list.

All thirteen tabs, this build, against the live reference captures:

| tab | differing pixels |
|---|---:|
| Java and memory | **29 544** |
| Window | **31 259** |
| Profile | **33 869** |
| Social | **35 034** |
| Appearance | **35 231** |
| Features | **35 971** |
| Behavior | **40 070** |
| Syncing | **58 876** |
| Privacy | **53 350** |
| Language | **55 593** |
| Troubleshooting | **56 104** |
| Launch hooks | **56 721** |
| Resources | **73 597** |

Each capture was checked for *identity* as well as counted: the Launch hooks pane reads "Pre-launch hook / Enter
pre-launch command... / Wrapper hook", and the Resources pane reads "Content storage". Three earlier readings were
replaced because their panes were the wrong page; they are noted above rather than deleted.

### Resources: the path field belongs under its heading (fixed, 73 597 -> 71 582)

Driving the reference's Resources page shows the order plainly: "App directory", its sentence, **the path field with
the folder button**, and only then "Browse for an app directory". This build had `Block::PathText` before the heading,
which put the field above the words that name it. Moving it took the page from **73 597 to 71 582**.

What still differs there, and stays: the reference's bar is keyed **Unique / Shared / Orphaned** because it splits a
content cache this build does not have; ours is keyed by folder and reads "Cache 2.5 MB" with a green bar, because that
is what is actually on disk. The shape of the block matches; the state behind it does not.

### The settings probe prints the rects a press is read against, and it settles the rail

`SIFT_SETTINGS_PROBE=1` makes the page print every tab row's rectangle on any frame with a press. A clean run under
Xvfb gives:

```
Appearance      272,246  218x40      Profile         272,464  218x40
Features        272,290  218x40      Social          272,508  218x40
Behavior        272,334  218x40      Privacy         272,552  218x40
Language        272,378  218x40      JavaAndMemory   272,638  218x40
                                    Window          272,682  218x40
                                    LaunchHooks     272,726  218x40
                                    Syncing         272,770  218x40
                                    Resources       272,856  218x40
                                    Troubleshooting 272,900  218x40
```

Three facts fall out of it:

1. **The pitch is 44**, exactly as the reference's captured rail shows (257, 301, 348, 392...). The "30" reading is
   dead for good.
2. **`Syncing`'s centre is 790, and the rail's visible list ends at about 783** -- so the press test (which skips a row
   whose centre is outside the rail) can never accept it. It is not a drawing bug: the row is drawn and correctly
   positioned, and its centre is a few points below the scissor line.
3. **Resources and Troubleshooting sit at 856 and 900**, 73 and 117 points below that line.

So the rail needs either a taller pane or a scroll that actually holds -- the wheel moves it in some runs and not in
others, and every tab change resets the offset. Recorded as the next thing to fix, with the numbers above as the target:
after a scroll of ~120, Syncing's centre lands at 670, inside the pane.

This is also the answer to the whole class of mislabelled captures this record keeps finding: **print the rect, click the
rect**. A probe line is worth a dozen guessed coordinates.

### The rail: presses and the wheel both work, and the probe is how it was settled

The `Area` that owned the wheel **swallowed every press** on the rows -- the probe reported `tab=Appearance` after
clicking rects the same probe had printed one line earlier, in both layer orders. Removing it and reading the wheel from
the frame's own `MouseWheel` events fixes both at once:

- pressing Features reports **`tab=Features`**;
- five notches move **Syncing from 770 to 550**, so every row is inside the pane.

Measured at probed rects, with the tab confirmed in the same run:

| tab | differing pixels |
|---|---:|
| Syncing | **46 278** |
| Troubleshooting | **56 104** |
| Resources | see below |

Syncing's and Troubleshooting's earlier figures were right; they are now *verified* rather than lucky.

### The library card: same shape, different data (2026-11-05)

Cropping the first card from the reference's own library and from ours showed a green rounded tile with a picture on
the reference's side and a large grey cube on ours. That is **not** a shape difference: both cards draw a 152-point tile
inset 8 points inside a 168x208 plate, and the picture is the instance's own `icon.png`. The reference's instance is a
modpack that ships one; of this lab's eight instances only `Instance 3` has an `icon.png` on disk:

```
instances/Instance 3/icon.png     <- the only one
```

So the cube is the reference's own default avatar doing its job, and the difference is data. Recorded rather than
"fixed" by inventing icons.

**The library's layout is now measured and aligned**: heading 157, toolbar 193, filter row 239, first card row 287,
each taken from `docs/captures/ref-lib-real.png`, and the three filter chips carry the reference's glyphs.
