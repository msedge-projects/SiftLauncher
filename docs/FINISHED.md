# Definition of finished, and where the work stands

**Position at the end of the 420-round budget (2026-11-10, 888 commits, 269 tests passing).** This is a working launcher that opens
Modrinth-shaped pages, and it is **not** a 1:1 clone. Four of the objective's six clauses are measured and unmet; two are unreachable
on this backend for reasons of arithmetic. The rest of this file is the evidence, page by page, and the history of what changed it.

**What is met**: the tree builds and the suite passes (269 tests); five screen sizes are captured side by side and measured on the final build (**27 933 / 36 268 / 31 356 / 45 470 / 57 864** for 1280x720, 1366x768, 1440x900, 1600x900 and 1920x1080); 174 matrix
rows are DONE and 65 VERIFIED of 800 status-bearing rows (a further 38 OPEN-WITH-CAUSE, 16 NOT-IN-SOURCE, 19 RENDERER-RISK and 13 BLOCKED-SERVICE), each with a capture or a source line; every slice is committed with its measurement.

**What is not met, in one line each**: 99 per cent identity (75 per cent of what remains is glyph edges and photo resampling -- 24 816
pixels on Home against a 12 960 budget); idle RSS under 20 MB as the lab runs today (22 472 kB on this tree, and the same tree read
13 720 kB earlier, and **22 680 kB measured on this build** -- see the decomposition below); **475 of 800** matrix rows OPEN; and the sidebar's news list,
which needs a scroll container rather than a draw call.

**Two measurement bases, stated because they differ**: `compare.py` reads **46 204** for Settings across its four slabs, while a
whole-frame scan of the same pair splits **30 639 inside the dialog and 27 941 behind it**. The slab scan excludes rows the whole-frame one
counts, so a figure from one is not comparable with a figure from the other. Where a row below names one number, its method is in its own cell.

**What would change the answer**: a text renderer that rasterises glyph edges the way WebKit does, and an image resampler that matches the
reference's filtering. Both are backend work rather than page work, and both are outside what 420 rounds of page-by-page porting could reach.


The bar: every screen at **≤2 000 differing pixels** in a 50/50 blend against the reference at 1440x900, both
clients in the same state, with a capture behind every claim.

## Measured, with evidence

| screen | differing pixels | evidence | where the remainder is |
|---|---:|---|---|
| Home | **31 356** | `homeCh239-ref.png` vs `homeCh239-our.png` (2026-11-09, verified pair: both 1440x900, both on Home, breadcrumbs 1.8% apart) | type of the news copy (13 points, the measured optimum) and the card's own foot; the news sidebar wraps one word later per line than the reference |
| Discover | **79 146** | `discV-ref.png` vs `discV-our.png` (2026-11-09, captured by `pair.py`: both windows verified 1440x900, both on Discover, breadcrumbs 4.8% apart) | type, card artwork, and the emoji in API summaries that this font has no glyph for |
| Screenshots (populated) | **27 745** | `sc-away.png` vs `our-shots2.png` (2026-11-09, **one screenshot in both instances**) | 2 166 inside the card is the image resample; 3 688 elsewhere is the top chrome. Was 23 573 for two *empty* pages -- an empty page has nothing to misalign, and hid a 24-point double padding worth 11 338 |
| Instance page | **30 471** | `inst-ref.png` vs `inst-our.png` (2026-11-09, both on Content, empty) | type; the sidebar is 16 816 of it |
| Settings | **31 721 inside its own plate**, rows aligned | `sb-ref.png` vs `pad-our.png` (2026-11-09, both on the Appearance tab, verified same size). Raw page 46 437; of that **31 721 is inside the dialog's rect** and the rest is the dimmed page behind it. The figure is *higher* than the 27 256 measured before the rows were aligned, and that is the point: at a four-point offset the two renderings' edges partly cancel under the threshold. Aligned rows are the honest reading |
| Project -- Description | **71 398** | `projL-ref.png` vs `proj-gap4.png` | the body\'s type |
| Project -- Versions | **63 660** | `ver3-ref.png` vs `ver-chan3.png` | the filter row\'s widths and the type; ~400 px is the API\'s download count, fetched minutes apart |
| Project -- Gallery | **172 200** | `gal-ref.png` vs `viewer-escaped.png` | the grid\'s ratio and a 15-point card-top offset; both grids carry the same six cards |
| Files tab | **57 726** | `files-ref2.png` vs `files-our3.png` (2026-11-09, six folders in both) | **the mtimes**: same six folders, same order, same five columns; the dates differ because the folders were made today. Higher than the 49 056 measured when this build listed one row -- more rows, more to compare |
| Skin selector | **135 516** | `skin-ref.png` vs `skin-our.png` (2026-11-09, both in the demo-account state) | **the skin art**: the heading and "Saved skins" land within a point, and the structure is identical, but the reference draws nine real skin textures where this build draws flat stand-ins (144 609 px), plus its 3D model shading (13 891) |
| Servers | not comparable | `ref-servers.png` | the reference refuses its own page |
**The table above is the current lab; everything below it is history.** Later tables in this file record what each round measured at the
time, against the lab as it stood then, and several of them describe states that no longer exist -- an empty Screenshots page, an instance
with one folder, a reference with no screenshot. They are kept because they are the record of what was proved, and they are **not**
comparable with the table above or with each other.

**Read the modal rows as modals.** Settings and the project\'s dialog states are plates over a *blurred* page, so their figure is the plate plus
whatever shows through the blur -- two different things. Measured on Settings (`sb-ref.png` vs `pad-our.png`): **31 721 inside the plate and 27 941 outside it** -- the page behind the scrim, which this build draws differently for reasons that have nothing to do with Settings. An earlier pair split 27 256 / 10 189 because its reference was on a different tab. The
backdrop\'s count is far below Home\'s own 32 781 because the blur suppresses glyph detail, so a modal row understates its dialog and cannot be
compared with a full-page row. Rows below that are modals: Settings, and the project page\'s dialog states.

**Home, by threshold**: 31 049 at 40, and the gradient below it is in `GATES.md`. The 3.6% global type scale was measured as a
wash and reverted; what moved the number since was **geometry and standalone type**, not a knob:

| change | Home |
|---|---:|
| round 180 baseline | 36 196 |
| the jump-in strip's icon, then its 8-point offset | 36 196 -> 32 287 |
| the toolbar chips' contents (not their boxes) | 32 287 -> 32 158 |
| the strip's name, then its badge and meta | 32 158 -> 31 949 |
| both page headings, each measured against its own ink | 31 949 -> 31 215 |
| the sidebar's "News" heading | 31 215 -> 31 049 |
## What is plainly not finished, with the reason and the starting point (2026-11-10)

Every item here was found by measurement or by reading 0.22.0, and every one is open. They are listed together because the clause table
above counts them and because a reader deserves the list rather than a sum.

| what | why it is not done | where a future attempt starts |
|---|---|---|
| the sidebar shows **one news card** where the reference shows up to four and a "View all news" button | the plan allocates one card's space and pins the hosting banner to the panel's foot; the reference fits four because its sidebar **scrolls** | `panel.rs` — `plan.banner == WINDOW.1 - BANNER_H`, the card ends 26 points above it, and the fix is a scroll container, not a card |
| the **selection bar** on Screenshots carries Clear and Delete but not New group or Export ZIP, and is a centred pill rather than the reference's wide bar | its width comes from `selection_bar_rect` and its rows from the drawing code below it: the two must agree, so a row cannot be added alone | `screenshots.rs:1380` (`bar_rect`) and the `clear`/`delete` rects below it — make the width derive from the same list the drawing iterates |
| the card's **copy button** cannot be verified | `arboard` does not return on this display (Xvfb, no clipboard manager); the write runs on a worker so the window no longer freezes | a desktop session with a clipboard, or a different clipboard backend |
| the **Edit** and **Go to instance** rows, and the **Export ZIP** action, are absent | each needs machinery this build does not have: an image editor, a route out of a page whose `draw` returns a `bool`, and an archive writer | three separate pieces of work, none of them drawing |
| the **Appearance pane drifts five points** below the theme cards | measured by cross-correlation (`Sync with system` prefers dy -5 while the cards prefer 0); the term is in the theme block's own height and was not located in three greps | the Appearance tab's block list — find the height that follows the cards and subtract five |
| **475 of 800 matrix rows are OPEN** and 174 are DONE (65 more VERIFIED) | 23 rounds remain; the rows that could be closed cheaply were (11 re-verified by reading our source, 7 by driving both clients) and the rest need building | `GAP-MATRIX.md`, sorted by the areas with the most rows |
| the **20 MB idle-RSS gate** is not met as the lab now runs | the figure is environment-dependent: the same tree read 13 720 kB earlier and 22 472 kB when rebuilt, with the whole binary resident rather than half | re-measure on a host with the page cache the first measurement had; the code's own contribution is ~2.5 MB of heap |

**And the one that cannot be fixed by any of the above**: 99 per cent identity is unreachable on this backend, because 75 per cent of the
remaining difference sits on glyph edges and photograph resampling -- 24 816 pixels on Home alone against a 12 960-pixel budget. That is
arithmetic, not effort, and it is stated in the clause table.
## Where this stands against the objective, clause by clause (2026-11-09)

The objective names six things. Four are measurable, and each is measured; two are not met, and the reasons are measurements
rather than estimates.

| clause | status | the evidence |
|---|---|---|
| **99% visual identity (<=1% of pixels at threshold 40)** | **NOT MET, and unreachable on this backend** | measured 2026-11-09 by classifying differing pixels by the reference's local contrast: Home **33 004 differ -- 24 816 textured (75%)**; Screenshots **39 822 -- 25 340 textured (64%)**. At 1440x900 the 1% budget is **12 960 pixels**, so the textured share alone is 1.9x the whole budget: glyph edge coverage (`ab_glyph` against WebKit, 13 215 pixels at threshold 120) and photograph resampling (19 424 on the news artwork at 1920). The remainder was first quoted as a 0.6% reachable target -- **that figure is withdrawn**: the classifier counts smooth artwork interiors as flat, so it cannot separate layout from scale. The artwork's own scale is a separate, measurable mismatch |
| verified at five standard sizes | **MET** | re-run on the current build (2026-11-09), every capture size-checked and the reference crops checked to be on Home: 1280x720 **27 933**, 1366x768 **36 268**, 1440x900 **31 356**, 1600x900 **45 470**, 1920x1080 **57 864** -- 2.42 to 3.46 per cent of each frame. The per-slab split is valid at every size now that `compare.py` derives its slabs, but the sidebar term is fixed at 300 points and does not scale, so the split still reads best at 1440 |
| a GAP-MATRIX row per element with a capture behind every done | **PARTIAL** | recounted 2026-11-09 against the file itself: **800 status-bearing rows -- 174 DONE, 65 VERIFIED, 475 OPEN, 38 OPEN-WITH-CAUSE**, plus 16 not-in-source, 19 renderer-risk and 13 blocked-service. Eleven rows were re-verified this stretch by reading our source, and seven were closed by driving both clients (`GAP-MATRIX.md`, rows 822-828) |
| idle RSS under 20 MB on the software backend | **environment-dependent: MET on the lab as first measured, NOT MET as it now runs** | 2026-11-09 measurements, all at 1440x900 with nothing opened: **13 720 kB** on the tree of round 348 (`RssAnon` 2 464, `RssFile` 6 240); **22 504 kB** on the current tree (`RssAnon` 5 608, `RssFile` ~16 900); and **22 472 kB** on a rebuild of the *older* tree in the same session -- so the growth is the environment, not the code. `RssFile` rising from 6 240 to ~11 800 against an 11 MB binary means the whole executable is now resident where half once was. The viewer adds 14 MB and never releases it, which is a real defect and is not environmental |
| tree building, tests green | **MET** | `cargo build --release` clean, **269 tests** pass |
| every slice committed and measured | **MET** | **892 commits** at the last count; each round's measurement is an entry in `GATES.md`, and the figures in this table say the date they were taken |

**Why the first clause cannot be closed by working harder.** The measurement is monotone in text density, and every page is text:
Home's difference is 13 215 at threshold 120 and 5 549 at 160, which is the signature of one glyph drawn a shade apart by two rasterisers
rather than of misplaced boxes. Two families of fix were tried and both reverted on measurement: a global type scale (a wash) and a glyph
coverage factor (+1 312, worse). The remaining 2.5% is the same drawing in a different engine.

**Why the third clause cannot either.** At one row per round -- the rate this session has managed on rows that need a drive -- the
remaining rows need more rounds than this project has left.

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

**579 commits, 263 tests green** (2026-11-08). A heartbeat measures the screen every four minutes in the lab container.

**The tables from here down are dated logs from 2026-11-02 and are kept for history. The current measurements are the table at
the top of this file.**

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

### The empty content state, compared properly (2026-11-05)

With the windows on equal terms (1440x900 both) and an **empty** instance on each side -- the reference's one instance
has no mods, and this lab's `Instance 3` has none either -- the instance page reads:

| slab | pixels |
|---|---:|
| total | **51 526** |
| sidebar | 32 005 |
| content | 15 490 |
| titlebar | 2 929 |
| rail | 1 102 |

The empty state itself matches closely: the box art, "No content installed", "Browse or upload projects to get started",
and the "Upload files" / "Browse content" pair. Measured off the two captures, the art's ink is **81 wide and 59 tall on
both sides**; it sat 33 points too low here and has been moved up, leaving a **9-point** residual that is recorded rather
than silently rounded off.

Two glyphs still differ by eye and are recorded as open: the reference's "Upload files" carries a box-with-arrow mark
where this build draws a plain upload arrow, and "Browse content" carries a circular mark where this build draws a
target. Both are icon-name questions, not layout.

**And one false alarm, written down because it nearly became a bad "fix"**: our Content tab listed 26 jars that I took for
internals (`.fabric/processedMods`, `local/crash_assistant`). Checking the folder showed **49 real mods** in
`minecraft/mods` -- the list was right and the state was simply different from the reference's empty instance.

# Measured state of the clone, consolidated (2026-11-05)

Every number below was taken with **both windows at 1440x900**, with the state described beside it. Where a page could
not be measured honestly, that is said instead of a figure.

## Pages

| page | state | differing pixels | note |
|---|---|---:|---|
| Home (empty library) | same | **24 444** | sidebar now at its rasterisation floor |
| Library | one instance each side | **89 051** | instance name and art differ (data) |
| Discover | live catalogue | ~79 800 | text and project artwork |
| Project (Description) | same project | **92 195** | gallery image drawn on both |
| Instance, Content tab | both empty | **51 516** | art 81x59 ink on both |
| Instance, Files tab | both at root | 68 687 | reference has a `config` row, this lab has none |
| Instance, Worlds tab | both empty | matches | art, copy and buttons identical |
| Instance, Logs tab | both idle | matches | frame, chips and Live Log select rebuilt |
| Instance, Share tab | both empty | **49 545** | art and copy identical |
| Skins | no saved skins | **155 619** | structure identical; skin artwork differs |
| Screenshots | both empty | **22 351** | 14 213 of it is sidebar text rasterisation |
| Settings, 13 tabs | same tab | 29 544 .. 73 597 | best Java, worst Resources |

## What is at its floor

Text rasterisation. The evidence is a threshold sweep on the sidebar: 16 324 at 40, 11 137 at 80, 8 173 at 120,
**5 235 at 160**. Layout differences do not behave that way -- a shifted block stays flat until the threshold is enormous.

## What is blocked, and on what

- **Servers** -- the reference gates its own page behind an account this environment cannot create.
- **Account, Follow, Save, Report** -- the same reason: they need Modrinth's own service.
- **Two CI checks** -- no remote is configured for this tree; the build and the tests run locally and are green.

## What is open

- The project sidebar paints its checklist from a compact copy until `panel::draw` is switched to
  `getting_started_card` -- now a straight swap, with the 16-point question answered (58, not 34).
- Resource packs, Data packs and Shader packs tabs list files but have not been compared tab-by-tab.
- Drag-and-drop of screenshots between groups, and the ZIP export, are unbuilt.