
## CORRECTION, 2026-11-06: idle RSS is ~110 MiB, not 48.4 MB

The figure recorded in GATES.md (48.4 MB idle) **could not be reproduced** and is superseded. Measured on the running
build, 1440x900, Xvfb + Mesa llvmpipe, after 15 s idle with a page drawn (reproduce with the commands below):

| measure | value |
|---|---:|
| **VmRSS** | **114 436 kB = 111.8 MiB** |
| VmHWM (peak) | 114 436 kB |
| PSS | 106 712 kB = 104.2 MiB |
| private (clean + dirty) | 101 008 kB = 98.6 MiB |
| RssAnon (the app's own heap + stacks) | 38 568 kB = 37.7 MiB |
| RssFile (binaries, fonts, libs) | 70 692 kB = 69.0 MiB |
| RssShmem (X server image) | 5 176 kB = 5.1 MiB |
| threads | 15 |

Steady state was 112 780 kB after a minute of idling, so this is not growth -- it is the footprint from the first frame.

### Where it goes

| mapping | RSS |
|---|---:|
| `/usr/lib/libLLVM.so.23.1` | 43 380 kB (42.4 MiB) -- llvmpipe's shader compiler |
| `[heap]` | 26 144 kB (25.5 MiB) |
| `libgallium-26.2.6` | 10 828 kB (10.6 MiB) |
| the `SiftUi` binary itself | 9 604 kB (9.4 MiB) |
| anonymous (fonts, textures, stacks) | 7 488 kB |
| SysV shared memory (X image) | 5 176 kB |

### What this means for the gate

- The bound is **<=20 MB idle RSS**. This build uses **111.8 MiB: 5.6x over**. Even the anonymous set alone
  (37.7 MiB) is 1.9x over, so the app's own allocations fail the bound before graphics are counted.
- The graphics stack costs ~58 MiB of it, and **42.4 MiB of that is LLVM**, which llvmpipe loads to JIT shaders. Any
  OpenGL or Vulkan UI on this machine pays it -- egui/glow, egui/wgpu, GPUI, or a browser.
- For scale: the **reference itself** (`ModrinthApp` 126 648 kB + `WebKitWebProcess` 128 692 kB) uses **~249 MiB**.
  The 20 MB bound would reject the application this project is cloning, in this environment, by a factor of twelve.

So the honest reading of the task's own rule -- "a software rasteriser ships iff it holds <=20 MB idle RSS and
<=16 ms p95" -- is that **the software rasteriser does not ship** on this machine: the gate is not a target to tune
towards, it is a verdict, and the frame budget passes while the memory budget cannot be met by any GL/Vulkan backend
available here. Recording it as a verdict rather than as a number to chip at.

Commands, so the figure can be checked rather than trusted:

```
P=$(ps -eo pid,comm | awk '$2=="SiftUi" {print $1}' | head -1)
grep -E "VmRSS|RssAnon|RssFile|RssShmem" /proc/$P/status
grep -E "^Pss|^Private_" /proc/$P/smaps_rollup
awk '/^[0-9a-f]+-/{name=$6} /^Rss:/{rss[name]+=$2} END{for (n in rss) print rss[n], n}' /proc/$P/smaps | sort -rn | head
```
## The CPU-rasteriser floor, measured (2026-11-06)

The question was whether replacing OpenGL-on-llvmpipe with CPU rasterisation can reach the <=20 MB gate. To answer it with
a number rather than an estimate, `tools/mem-spike` was built: **winit + softbuffer + tiny-skia**, one window, one filled
rectangle, no application logic at all. Its whole purpose is to be the smallest honest example of the architecture.

At 1440x900 on the same Xvfb display as the launcher:

| measure | value |
|---|---:|
| **VmRSS** | **17 064 kB = 16.7 MiB** |
| RssAnon (the tiny-skia pixmap + state) | 6 396 kB |
| RssFile (binary + libs) | 5 604 kB |
| RssShmem (the X shared buffer) | 5 064 kB |
| threads | 1 |
| binary on disk | 2.2 MB |

Against the launcher's current **111.3 MiB** through OpenGL/llvmpipe, that is the whole 53 MiB of LLVM and gallium, plus
the window-proportional buffers, gone.

### What it says about 10 MB

It says **no, not at this window size, and not for this architecture as written**:

- the X shared buffer (5 064 kB) and the tiny-skia pixmap (5 200 kB) are both full-screen, so **~10 MiB is spent on two
  copies of the same picture before any code runs**;
- the binary and libraries keep 5 604 kB resident;
- the remaining 6 396 kB is state.

And it fingers the next lever: **rasterise straight into the presentable buffer** instead of into a pixmap that is then
copied into one, and the second 5 MiB disappears. A real application then adds fonts, textures and its own state on top,
which is why the honest expectation for the launcher on a CPU rasteriser is **20-30 MiB, not 10** -- but that is a
measured floor, not an estimate: 16.7 MiB for the smallest possible app, at this resolution, in this environment.

### So where does 10 MB come from?

A smaller program. The iced-era launcher was an early slice -- a few pages, no image decoders, no network stack, no
screenshot cache -- and possibly a smaller window. 10 MB was true of *that* binary and is not a bound any version of this
product can hold at 1440x900, whatever renders it.
## The software backend: 113.3 MiB -> 25.8 MiB (2026-11-06)

Built, and measured on the same Xvfb display as everything else:

| build | idle RSS |
|---|---:|
| OpenGL on llvmpipe (before) | **111.3-113.3 MiB** |
| software backend, first working version | 27.4 MiB |
| software backend, textures shared not copied | 25.9 MiB |
| plus eframe/glow built out of the default binary | **25.8 MiB** |

It draws the whole launcher: rail, title bar, Jump in strip, library cards, news carousel with the reference's own
artwork, hosting banner. The capture is `.scratch/cap/soft-home.png`.

### What it is

`crates/sift-ui/src/soft/` -- a winit window and softbuffer surface, with egui's tessellated meshes rasterised by this
crate: barycentric fill, premultiplied source-over, nearest texture sampling. No shaders, no GL, no pixmap-then-copy:
it writes **into the presentable buffer**, which is the 4.94 MiB the spike showed a pixmap path would waste.

The backend is now the **default**; `eframe`/`glow` are behind an optional `gl` feature because the software path never
calls them. 255 tests pass.

### Where the remaining 25.8 MiB is, by mapping

| mapping | RSS |
|---|---:|
| the SiftUi binary's own text | 9 360 kB |
| the X shared buffer (`/dev/shm/softbuffer-...`) | 5 064 kB |
| heap | 3 564 kB |
| libc, libcrypto, libX11, libm, anonymous | ~5 400 kB |

**So the 20 MB gate is not reachable at 1440x900 by any version of this program**: the window buffer (5.1) plus system
libraries (5.4) are 10.5 MiB before this launcher's own code runs, which would leave under 9.5 MiB for a binary that
currently needs 9.4 MiB of text *and* its heap. Under 20 MB needs a smaller window or a much smaller program -- not a
better renderer, which is what the last three rounds bought.

The levers left, in order of expected value: `opt-level = "z"` for the binary (at some cost to frame time), keeping the
TLS stack out of memory until the first fetch, and trimming the heap. Recorded as the next step rather than a promise.
## Round 68: two more levers pulled, and the floor named (2026-11-07)

**opt-level = "z"** (was 3, chosen when the hot path was the GL driver): binary 14.2 MB -> 12.5 MB on disk, idle RSS
25.8 -> **25.4 MiB**. 450 kB for the whole optimisation level, so the text is mostly load-bearing code rather than
padding.

**The category-tags fetch is now gated to Discover.** It sat in `content_column`, which is drawn on *every* page, so
each launch fetched tag categories from the network before the reader had gone anywhere. It now asks only where the list
is shown. RSS 25.4 -> **25.2 MiB**, and no page except Discover touches the network at all -- worth having on its own
terms, even though it did not buy the megabytes hoped for.

**Why it did not:** `libcrypto.so.3` (1 052 kB resident) is a *link-time* dependency of the TLS stack, mapped at exec,
not lazily on first request. Deferring the fetch cannot remove it; only dropping OpenSSL could, and `rustls` would trade
it for more text in our own binary.

### Where the 25.2 MiB is

| item | RSS |
|---|---:|
| our binary's text pages | ~8 600 kB |
| the X shared buffer at 1440x900 | 5 064 kB |
| heap (font atlas 2 254 kB of textures, state, stacks) | 6 304 kB |
| libc, libcrypto, libX11, libm, anonymous | ~5 400 kB |

**The gate is 20 MB. The fixed costs are 10.5 MiB** -- the window buffer and system libraries -- so our code and heap
must fit in 10 MiB where they now need 14.9. Every remaining lever is about one megabyte: dropping `image`'s jpeg
decoder (~0.4), fewer Inter weights in the atlas (~0.5), heap trims (~0.5). That is ~1.4 in total, which lands at
**~23.8 MiB** -- short of the gate.

The only structure that reaches 20 is moving the network stack and the image decoders **out of the UI process** into a
helper the UI talks to over a pipe: that removes `reqwest`/OpenSSL/`image` and their buffers from the resident set, about
3 MiB, and would land ~20.5-21 MiB before the small trims and ~20 with them. Recorded as the next architectural step,
with the arithmetic above rather than an assurance.
### Subset fonts: 854 kB of glyphs nothing draws (2026-11-07)

The five Inter weights were shipped whole -- 1 532 512 bytes of TTF in `.rodata` -- for an interface whose own strings
contain **15 non-ASCII characters**. User data (instance names, project titles, descriptions) needs more than those, so
the subset keeps a generous superset: ASCII, Latin-1, Latin Extended-A, Greek, Cyrillic, general punctuation, currency
signs, arrows, geometric shapes and dingbats. Subset with `fonttools` (installed from the distro, since this image has
no pip and no apt):

| | bytes |
|---|---:|
| five full weights | 1 532 512 |
| five subset weights | **677 888** |

**Verified not to change a pixel.** The pre-subset and post-subset captures of Home differ by **0 pixels**, worst channel
delta 0 -- the removed glyphs are ones nothing in the interface draws.

Measured: binary 12.55 MB -> **11.69 MB**, idle RSS 25.2 -> **24.5 MiB**, PSS 20.2 MiB.

Running total for the memory gate this session: **113.3 -> 24.5 MiB**, a 4.6x reduction, with the UI rendering
identically throughout. The full weights are kept in `.scratch/fonts-full/` so the change is reversible.
## The framework floor: what the gate would cost even with an empty launcher (2026-11-07)

`tools/mem-floor` is egui + winit + softbuffer with **no launcher code**: it installs the same five Inter weights, lays
out six lines of text in them, and measures. At 1440x900:

| measure | value |
|---|---:|
| VmRSS | **11 680 kB = 11.4 MiB** |
| RssAnon | 2 640 kB |
| RssFile | 9 040 kB |
| PSS | 8 909 kB |

It never presents a frame, so it carries no X buffer. Adding the buffer the real window needs at 1440x900 (5 064 kB)
gives the true floor for *any* launcher built this way:

```
framework + window floor   11.4 + 4.9 = 16.7 MiB
the launcher itself                       7.8 MiB
----------------------------------------------
measured total                           24.5 MiB
```

### What that means for a 20 MB gate

The gate leaves **3.3 MiB for everything this launcher is** -- eleven pages, the instance store, the artwork, the icon
cache, the font atlas, the shell state. It has 7.8. So the gate is not a tuning target for this stack: it asks the whole
application to fit in less than half the space its own framework leaves.

**This also falsifies the plan recorded last round.** Moving the network stack and image decoders to a helper process was
estimated to reach ~20 MiB by removing ~3 MiB. Against a measured floor of 16.7 that leaves 20.0 only if the helper also
removes *every other* megabyte the launcher owns -- which no helper does. The estimate was arithmetic on the wrong base,
and the measurement corrects it.

The remaining honest options are: a lighter framework than egui (which forfeits the parity work this objective is built
on), or a gate expressed for this stack -- PSS is already **20.2 MiB**, and RSS at the reference's own 1280x800 default
would be ~23.4. Recorded for a decision rather than quietly redefined.
### egui's bundled fonts: 1.41 MB of faces this interface never asks for (2026-11-07)

`egui`'s `default_fonts` feature ships four faces -- Ubuntu-Light (361 676 B), NotoEmoji (418 804), an emoji icon face
(324 132) and Hack (309 408), **1 414 020 bytes** -- and `.monospace` appears nowhere in this interface: every string is
Inter, and the icons are drawn as vector glyphs rather than typed as characters.

The feature is off. `fonts::install` now builds both generic families from our own subset weights (monospace included, so
a widget asking for it gets Inter rather than a missing family), and nothing else changed.

| | before | after |
|---|---:|---:|
| binary | 11.69 MB | **10.28 MB** |
| idle RSS | 24.5 MiB | **23.8 MiB** |
| PSS | 20.2 MiB | **18.4 MiB** |

**Verified by capture**: the before-and-after renders of Home differ by **0 pixels**, worst channel delta 0. The fonts
were not merely rarely used; nothing on that page drew a glyph from them.

Session total: **113.3 -> 23.8 MiB** idle RSS. PSS -- the proportion actually attributable to this process -- is now
**18.4 MiB**, under the gate.
### Art plates upload on first use (2026-11-07)

`serverart::install` uploaded seven illustrations at startup, including the Screenshots empty state and the instance
page's "No content installed" box -- plates a reader may never see, each a decoded RGBA image held for the session.
Only the title bar's mark is unconditional now; the rest upload the first time their page draws them.

| | before | after |
|---|---:|---:|
| textures held at idle | 2 254 kB (10) | **1 964 kB (6)** |
| RssAnon | 6 220 kB | **5 928 kB** |
| idle RSS | 23.8 MiB | **23.6 MiB** |
| PSS | 18.4 MiB | **18.2 MiB** |

Home still draws the news card and the hosting banner, so those two plates are still resident there -- which is why the
saving is 290 kB rather than the 660 kB the six deferred plates add up to.
### The software renderer measured against the OpenGL one, and against the reference (2026-11-07)

Now that the software backend is the default, the parity numbers had to be re-taken through it. Home, with the carousel
pinned to the reference's article (art diff **0**, so the comparison is honest):

| path | total | titlebar | rail | content | sidebar |
|---|---:|---:|---:|---:|---:|
| OpenGL/glow (before) | 71 349 | 2 229 | 1 029 | 53 788 | 14 303 |
| software rasteriser | **73 660** | 2 227 | 1 034 | 55 763 | 14 636 |

So the new path is about **2 300 pixels further from the reference** on this page. The title bar is identical (2 227
against 2 229), which locates the difference in glyph and card edges rather than in geometry or layout.

**Bilinear texture sampling was tried as the suspected cause and did not help**: 73 660 against 73 662 -- two pixels --
for four texel reads and a mix per pixel in the inner loop. It was reverted, with the measurement kept in a comment
above the sampler so the experiment is not repeated.

What remains is that this rasteriser blends in premultiplied sRGB space without the gamma correction a GPU path applies,
so glyph edges carry slightly different values. Recorded as a known, quantified 3% difference on this page rather than a
mystery -- and as the next thing to look at if a page's number seems inexplicably high.
### The "Jump in" section was 20 points too high, and that was 1 435 pixels (2026-11-07)

Chasing the software renderer's extra 2 300 pixels turned up something else, and something worse: a layout error that
predates the renderer change entirely. Comparing the row where each block's ink starts:

| heading | reference | this build (both backends) |
|---|---:|---:|
| Jump in | 79 | **59** |
| Library | 211 | 211 |

The Library heading matches, so the page is not offset -- the section above it was 20 points high inside a space of the
same total height, which made the gap after it 20 points too wide. The heading, the dashed strip and the collapse
chevron all move down by that measured 20.

After the change both headings read **79** and **211**, and Home on the software backend measures:

| | before | after |
|---|---:|---:|
| total | 73 660 | **72 225** |
| content | 55 763 | **54 326** |
| titlebar | 2 227 | 2 229 |
| rail | 1 034 | 1 034 |
| sidebar | 14 636 | 14 636 |

The OpenGL-era figure for this page (71 349) was taken before the instance was renamed and is not comparable; 72 225 is
the honest number for the shipped path with the layout corrected.

**The lesson is the method, again**: the heat map said "text", the correlation said "maybe 7 points", and only measuring
where each heading's ink begins named the defect and its size.
### Ink measurement as a tool, and two more row fixes (2026-11-07)

The method that found the 20-point Jump in error is now `tools/parity/ink.py`: it reports, for any block and x range,
the row where its ink starts in both captures and the delta. Landmarks on Home after the two fixes:

| landmark | reference | this build | delta |
|---|---:|---:|---:|
| Jump in | 79 | 79 | 0 |
| Library | 211 | 211 | 0 |
| search placeholder | 258 | 260 | **+2** |
| filter row label | 303 | 300 | **-3** |
| card art | 367 | 345 | **-22** |
| sidebar title | 84 | 84 | 0 |
| sidebar rows | 138 | 138 | 0 |

The search field's top border was at 241 against the reference's 248 and the filter row at 291 against 294; their
constants moved by the measured amounts (+7 and +3). Home measures **72 225 -> 70 527**, so those two rows were worth
**1 698 pixels** between them -- far more than the text antialiasing that started this investigation.

**Two residuals are left open and named rather than tuned by eye:** the search placeholder sits 2 points low and the
filter label 3 points high after their rows moved (sub-glyph rounding, both under a pixel of ink), and the card's art
starts 22 points higher here than in the reference -- the reference insets its image about 38 points inside the card
where this build insets about 10. That last one is structural and wants the card's art rect measured against the
reference's card, not another constant nudge.
### The "card art -22" was my measurement, not the card (2026-11-07)

The ink tool said the card's art started 22 points higher here than in the reference, and it was right about the *ink*
and wrong about the meaning. At 1:1 the two cards are:

- **reference**: a green rounded plate inset from the card, with the pack's campfire image *centred inside it* -- so its
  first non-background pixel is the fire, 22 points below the plate's own top edge;
- **this build**: the same plate, holding a flat green icon -- so its first ink is the plate's top edge itself.

The plate tops are 341 and 345: **four points apart**, and the card's structure matches. The 22 was the difference between
a picture with a subject and a picture without one.

Recorded because the tool now carries a trap: `ink.py` measures the first pixel above a brightness threshold, which is
meaningful for text and rules and misleading for anything that contains an image. **Measure edges, not ink, for plates**
-- and when a delta appears only on an element that holds artwork, suspect the artwork first.

The real remaining differences on this card are the icon itself (data -- this lab's instance has a flat green icon where
the reference's pack has its own art) and a few points between the title and subtitle baselines.
### Screenshots, re-measured on the shipped backend (2026-11-07)

Both apps on Screenshots with an empty library, the ink tool run over the empty state:

| landmark | reference | this build | delta |
|---|---:|---:|---:|
| "No screenshots yet" | 554 | 551 | **-3** |
| subtitle | 588 | 587 | **-1** |
| illustration | aligned | aligned | -- |

So the 20-point class of error that Home had does not repeat here: this page is within three points everywhere, and the
difference between the two heading and subtitle deltas (-3 and -1) means the block's internal gap is two points wide
rather than the whole block being misplaced.

The page measures **24 667** on the software backend (content 6 113, sidebar 14 636, titlebar 2 865, rail 1 053). The
band breakdown puts that content difference almost entirely in the two text rows -- 2 015 pixels between y 580 and 600
alone -- which is glyph rasterisation between this rasteriser and the browser, the floor this record has measured before.

The 3-point heading offset is left as it is rather than nudged: three points is inside the +/-2 target's rounding
neighbourhood and moving it would trade a real 3 pixels for a guessed one.
### The title bar carried two brands at once, and the breadcrumb never knew about instances (2026-11-07)

Putting the two title bars side by side at 1:1 found both in one look:

1. **The wordmark was drawn twice.** The plate carries the reference's mark and wordmark together, and the painted
   fallback -- a mark rectangle plus "Sift Launcher" set in Inter -- was still being drawn underneath it, so the strip read
   as a bitmap of "modrinth" with our own name smeared across it. The fallback now runs **only** when the plate fails to
   upload. No measurement had caught this: both elements were "present", and only their overlap was wrong. A capture did.
   Title bar: **2 229 -> 1 526**, Home **70 527 -> 69 824**.

2. **An open instance replaces the rail name.** The breadcrumb handled the project case and printed `rail.title()`
   otherwise, so the instance page announced "Home". It now reads the instance: **"Fabric 26.3"**, matching the reference.
   Instance page title bar: **2 929 -> 2 281**.

Two differences in the same strip are left open and named:

- the reference puts **the instance's own icon** in the breadcrumb where this build still draws the rail's glyph, which
  needs the icon cache threaded into `title_bar`;
- the reference has a **download-arrow button** beside the Update pill that this build does not draw at all.

The instance page measured 52 157 in this capture, but its sidebar read 32 423 against 14 636 on the run before: the news
carousel had rotated between the two apps, which is the confound `SIFT_NEWS_SLIDE` exists to remove and which this
measurement forgot to use.
### The breadcrumb carries the instance, at the reference's own x (2026-11-07)

Two changes in the title bar's route badge, both measured rather than eyeballed:

1. **The instance's icon replaces the rail's glyph** when a page is open. The icon cache was already on the shell for the
   library cards, so it is the same lookup in a 16-point rect; the rail glyph remains the fallback for a page with no icon.
2. **The badge sits at x 256, not 262.** Edge scans across the strip: the reference's icon starts at 256, its name at 278;
   this build had 262 and 283. One constant moves both, because the name is drawn five points right of the badge.

After the change both icon edges sit at **256** and both names start at **278**.

The instance page measures **34 421** (titlebar 2 332, content 16 346, sidebar 14 636) with both apps freshly driven and
the carousel pinned -- the earlier 52 157 in this session was a phase mismatch and is superseded.

What still differs in this strip is now only:

- the icon's **content** (this lab's instance has a flat green icon where the reference's pack has its campfire) -- data;
- the **download-arrow button** beside the Update pill, which this build does not draw at all.
### The round download button, drawn at last (2026-11-07)

The action bar had two of its three controls. The missing one is a 34-point ringed circle with a download arrow, which
the reference puts immediately left of the Update pill. Measured off its strip: the ring runs **x 957..990** and
**y 8..42**, the arrow sits at x 972..976, and the Update pill begins at 999 -- nine points to its right, as the pill
constant already said.

Drawn with a `SURFACE_5` ring and a `SECONDARY` arrow. **No action is bound to it**: what the reference's button does on
a press is not something this session could observe, and a control that guesses its action is worse than one that waits
for evidence. Recorded as an affordance with an unknown handler rather than a finished control.

Instance page: **34 421 -> 34 380**, title bar **2 332 -> 2 291**.

One difference is left visible in the same crop and named: this build's "Update" label sets a little heavier than the
reference's, which reads as a 500-weight text where ours may be 600. Worth a look at the weight constant rather than a
nudge of the size.
### The Update label was semibold where the reference sets it medium (2026-11-07)

Counted, rather than eyeballed: green pixels inside the Update pill, excluding its ring.

| | interior ink |
|---|---:|
| reference | 207 |
| this build, `semibold` (600) | 242 |
| this build, `label` (500) | **221** |

Seventeen per cent more ink for the same words at the same size is a weight step, not a size one, and the fix is one
font id. Title bar **2 291 -> 2 277**, instance page **34 380 -> 34 366**.

The remaining 4% (221 against 207) is **not** chased further: this rasteriser draws glyph edges with different coverage
from a browser, which inflates any ink count slightly, so the difference between 500 and 400 cannot be told apart from
the renderer's own bias with this measurement. Recorded rather than guessed -- the way to settle it would be to count
the same pixels in a page the *reference* renders at a known weight and compare, which is more work than 14 pixels
warrant today.
### A colour the page totals cannot see, and an experiment that changed nothing (2026-11-07)

The instance page's sidebar was 14 636 differing pixels and had always been explained as glyph rasterisation. Every
landmark in it measures **delta 0** (Getting started 84, the three rows 138/145/190, the news art -1), so the layout is
right -- but a side-by-side crop showed the checklist rows sitting lighter than the reference's.

| | row fill |
|---|---|
| reference | (50, 59, 57) = **#323B39** |
| this build | (59, 68, 66) -- the card gradient tinted 8% white |

Nine points on every channel: **under the 40-point threshold the page totals use**, so no number ever flagged it, and the
card's own gradient is within a point or two of the reference's. The rows now paint #323B39 directly and both sides
sample identically.

That correction *raised* the page total by 149 pixels (34 366 -> 34 515), because the previously-matching ring was
derived from the fill, and a fill nine points darker moves every ring pixel with it. The honest resolution: the ring is
now derived from the corrected fill and reads (66,75,73) against the reference's blended (60,67,67) -- close, and the
149 is the metric's threshold shape moving with a colour that is now actually right.

Setting the ring to an explicit measured value was tried **three times with three different colours and changed not one
pixel**, which is how this build learned that the ring it samples without hover is not drawn by the code that was edited.
That constant was reverted rather than left in as decoration, and the question -- where that ring is painted -- is open.
## Tone, not threshold: `tools/parity/tone.py`, and a regression it caught in my own work (2026-11-07)

The page totals count pixels whose worst channel differs by more than 40, so a fill that is nine points light on every
channel counts as **zero**. That is how the checklist rows sat wrong for dozens of rounds under a report that said the
sidebar was "at its rasterisation floor".

`tools/parity/tone.py` measures the drift instead: mean per-channel delta, a histogram of worst-channel deltas, and the
commonest colour pairs in the 5..39 band. Run over the sidebar it immediately produced:

```
8 566 px  ref (58, 67, 65) -> ours (50, 59, 57)
```

which is the *inverse* of the "fix" made in the previous round -- and it was my mistake. I had sampled one pixel at a
row's lower edge, read (50,59,57), and repainted every row to it, making all of them eight too dark. **The code's own
comment, four lines away, already said `#4b5253` around `#3a4341`.** The metric had risen 149 when I made that change and
I explained the rise away instead of investigating it; the tone tool is what settled it.

Chasing the second drift then found the real structure: the reference has **two** row fills.

| row | fill | label |
|---|---|---|
| open | `#3A4341` (58,67,65) | theme base |
| **completed** | `#323B39` (50,59,57) | **(109,119,124)**, dimmed, not merely struck |

This build had one fill for both and painted the completed label in the theme's secondary, (150,162,176) -- bright enough
that the row read as active with a line through it. Both values are now measured constants.

**Result, with both instruments agreeing for the first time this session:** instance page **34 519 -> 34 173**, sidebar
14 789 -> 14 443, mean |d| in the sidebar 6.9 -> **5.97**, and pixels within 4 of the reference 67 374 -> **74 456** of
87 600.

The lesson is worth more than the 346 pixels: **a threshold hides a colour.** Any "at its floor" claim in this record
about a region that was never run through `tone.py` is a claim about layout only.
## The idle window was eating a third of a core (2026-11-07)

Asked to check whether the work was maxing out the machine, the answer was yes: load average **9.04 on six CPUs**, and
one of the processes was this launcher at **95.5% of a core while idle**. Three separate causes, found and fixed in order:

**1. A redraw loop.** `about_to_wait` requested a redraw *and* drew a frame itself, while `RedrawRequested` set the dirty
flag -- so every frame produced the event that asked for the next one. Draw now happens only in the redraw handler:
95% -> **30%**.

**2. Nothing bounded the frame rate.** An animating page scheduled the next frame for "now", which against a software
rasteriser means as fast as the  machine allows. There is now a 16 ms floor (60 fps, the most a reader can see).

**3. Each frame cost ~130 ms.** Tracing showed only 1.6 frames a second, so the CPU was not frequency but *cost*: every
pixel of a 1440x900 panel went through twelve interpolated channels, a bounds-checked texture fetch and a blend to arrive
at the colour it started with.

- A **constant-UV triangle samples once** -- solid fills and their antialiasing fringes point every vertex at the same
  texel, and those triangles cover most of the window.
- A **flat, fully opaque triangle writes a constant**: no interpolation, no texture, no blend.

**Result: idle CPU 30% -> 9% of a core, with 0 pixels changed** -- verified by diffing the frame before and after the fast
path across all 1 296 000 pixels of the window, and Home reading the identical 69 534 against the reference.

**An optimisation that was reverted, and why.** Replacing the per-pixel barycentric weights with incremental steps had a
sign error in the edge gradients; the window rendered as green fragments on black, and the CPU reading of "2%" that came
with it was the broken renderer drawing almost nothing. That is worth stating plainly: **a performance number measured on
a broken render is not a performance number.** The incremental version was reverted through `git checkout` and only the
sampling half survived, which is why the honest figure is 9% and not 2%.

**Two things on this box that are not this launcher:** the reference app's WebKit renderer is at **240% CPU** on its own,
and repeated launches have left **298 zombie `SiftUi` processes** (their parent shells exit and PID 1 does not reap them).
Neither consumes CPU, but both explain a load average that stays high while this build idles at 9%.
## Frame time on the software backend: 91 ms, then 38 ms, against a 16 ms gate (2026-11-07)

The memory gate got the attention; the frame gate had never been measured on the path that actually ships. The soft
backend did not write a frame log at all -- `frame_log` was wired into the eframe app only -- so it could not be held to a
p95 it never recorded. It is wired now, and the first honest reading was worse than the OpenGL path by six times:

| | p95 | mean |
|---|---:|---:|
| OpenGL/glow (recorded) | 13.0-14.9 ms | -- |
| software, one thread | **91.46 ms** | 64.67 ms |
| software, parallel bands | **37.70 ms** | 26.13 ms |

So the renderer swap **traded one gate for the other**: memory improved fourfold (113.3 -> 23.6 MiB) and frame time
worsened sixfold (14.9 -> 91.5 ms). That trade had been invisible because the gate had no instrument on this path.

**The parallel pass** splits the buffer into horizontal strips, one per core, and each thread paints every primitive
clipped to its own strip -- safe because a triangle only ever writes rows it covers. Six cores gave 2.4x rather than the
6x the arithmetic suggests, because each thread still walks the whole triangle list and most triangles are small.

**Verified pixel-identical**: 0 pixels changed across all 1 296 000 of the window, and Home reads the same 69 534.

### Where the two gates now stand

| gate | bound | measured | verdict |
|---|---:|---:|---|
| idle RSS | <= 20 MB | **23.6 MiB** | **FAIL** (PSS 18.2 MiB passes) |
| frame p95 | <= 16 ms | **37.7 ms** | **FAIL** |
| idle CPU | not a gate, but a fan | 9% of a core | -- |

Both gates now have honest instruments and both are short. The remaining frame work is span-filling opaque triangles
rather than testing coverage per pixel, which is where the 37 ms sits; the remaining memory work is the framework floor
already recorded at 16.7 MiB for egui plus a window.
### The span-fill optimisation was rejected for 486 pixels (2026-11-07)

Filling each row of an opaque flat triangle as one slice, instead of testing coverage pixel by pixel, is the obvious next
step and measured well: **p95 37.70 -> 22.35 ms**. It was reverted, twice, and the numbers are the reason.

| attempt | p95 | pixels that differ from the correct render |
|---|---:|---:|
| span fill, slope signs flipped | 22.35 ms | **262 908** |
| span fill, signs fixed | 31.20 ms | 518 |
| span fill, ends re-tested exactly | 26.40 ms | **486** |
| parallel only (kept) | 37.70 ms | **0** |

The first version was a sign error -- `w0`'s derivative is `(b.y - c.y)/area`, not the reverse -- and it filled mirrored
spans. The corrected version left 518 pixels disagreeing at triangle boundaries; re-testing each span's two end pixels with
the exact per-pixel condition brought it to 486 and no further, which means the disagreement is not only at the ends.

**486 pixels is 0.04% of the window, and 11 milliseconds is 30% of the frame. It was still the wrong trade**: the
objective counts differing pixels, and a renderer that is fast but paints the wrong picture is the failure mode this
record has been refusing all session. `git checkout` restored the parallel version, verified at 0 differing pixels.

### A caveat this round earned

The same parallel build measured **37.70 ms** in one run and **23.79 ms** in another, twenty minutes apart. The difference
is the box, not the code: the reference app's WebKit renderer sits at 240% CPU throughout, and frame timings taken under
that load are not comparable run to run. Any future frame number should be taken with the reference stopped, and this
record should be read as saying the software path is in the **20-38 ms** band rather than at a point inside it.
### A stale heartbeat loop was still running (2026-11-07)

`tools/parity/loop.sh` -- a 240-second heartbeat that compares two captures and appends a line to `docs/PARITY-LOG.md`
-- had been running since long before this session and was still going: **1 680 uncommitted heartbeat lines**, the newest
written at 09:34 today, reporting differences against captures that no longer describe the build.

It is stopped, and its uncommitted output is reverted rather than committed: those numbers measure yesterday's capture
against a build that has since changed its renderer, and putting them in the record would be worse than leaving the gap.
The loop itself is left in the tree for whoever wants it, with the note that a heartbeat whose captures are not refreshed
reports drift that is not there -- and, at 240 seconds, quietly kept a Python and an ImageMagick busy on a box already
loaded by the reference's WebKit renderer.
### Frame time with the box quiet: 24-27 ms, so the gate fails honestly (2026-11-07)

Every frame measurement this session has been taken with the reference app's WebKit renderer at 240% CPU in the same
container, which is why the same build read 37.70 ms once and 23.79 ms twenty minutes later. With the reference paused:

| run | mean | p95 | max |
|---|---:|---:|---:|
| idle-ish frames | 15.26 ms | **24.40 ms** | 29.79 ms |
| during pointer sweeps | 20.75 ms | **26.76 ms** | 29.81 ms |

So the software path sits at **roughly 25 ms p95** against a **16 ms** gate: a factor of 1.6 short, not the factor of 6
the first unmeasured reading suggested. Idle CPU with the box quiet is 11% of a core.

**What has been tried and what is left.** The parallel row bands took 91 -> 38 ms; the span-fill took it to 22-26 ms but
painted 486 wrong pixels and was rejected. What remains is not an algorithmic trick but arithmetic width: the per-pixel
coverage test is the cost, it cannot be replaced by a coarser span without changing pixels, and the honest next step is
SIMD over the inner loop or a different rasteriser -- both bigger than a slice, and neither claimed here.
### How a clean frame number is taken here (2026-11-07)

The reference app is a WebKit browser and its renderer sits at **240% CPU** on its own, which made every frame measurement
this session a measurement of contention as much as of code -- the same build reading 37.70 ms in one run and 23.79 ms in
another, twenty minutes apart.

The fix is to **suspend** it rather than close it: `kill -STOP <pid>`, take the measurement, `kill -CONT <pid>`. Its window,
its page and its state all survive -- verified by the window still answering to `xdotool getwindowpid` (PID 332733, state
`T`) and drawing again after `-CONT`. Load falls from 9 to **2.16** while it is suspended.

That is how the 24-27 ms figure was obtained, and how any future one should be: **a frame time measured beside a spinning
browser is not a frame time.**
### Settings, measured on the shipped backend (2026-11-07)

Both apps on Settings, the modal open, the carousel pinned. The page reads **34 893** (content 31 205, title bar 1 839,
rail 353, sidebar 1 496 -- the modal hides most of the shell, which is why every band is small).

Tone mean |d| over the content column is **5.58 / 6.16 / 5.94**, and the sampled frame matches exactly:

| point | reference | this build |
|---|---|---|
| backdrop, left | (20, 30, 31) | (20, 30, 31) |
| backdrop, top | (24, 34, 35) | (25, 35, 35) |
| modal plate | (39, 41, 46) | (39, 41, 46) |
| modal body | (39, 41, 46) | (39, 41, 46) |
| nav plate | (21, 30, 32) | (22, 31, 32) |

So the drift is in **components**, not the frame. `tone.py` names them by weight:

| pixels | reference | this build | suspected element |
|---:|---|---|---|
| 7 074 | (35, 95, 64) | (36, 85, 61) | the selected nav tab's green, ten points darker here |
| 6 773 | (39, 41, 46) | (45, 47, 50) | a surface six lighter than the plate it sits on |
| 6 747 | (16, 16, 19) | (25, 25, 25) | a dark card, nine lighter |
| 1 635 | (39, 41, 46) | (66, 68, 74) | a surface that should be the plate at all |

That is the work list for the next visit to this page, written with pixel counts so it can be checked off rather than
re-derived.

**One difference is deliberate and stays**: the reference's About footer reads "Modrinth App 0.22.0" where this build
reads "Sift Launcher 0.1.62". Everything else on the page is cloned; the product name is not, because this is not
Modrinth -- and pretending otherwise would put a false claim in the product itself, not just in a capture.
### The settings rail's selected row was brand at 25%, not 30% (2026-11-07)

First item off the Settings work list, and the arithmetic was exact rather than approximate: the reference's selected
row is `color-mix(in srgb, var(--color-brand) 30%, transparent)` over the modal plate, which is
`0.3*(27,217,107) + 0.7*(39,41,46) = (35,94,64)` -- and its capture reads (35,95,64). This build held **25%**, painting
(36,85,61): ten points dark in green across **7 074 pixels** of the rail.

The constant is now `from_rgba_premultiplied(0x08, 0x41, 0x20, 0x4d)`, and both sides sample (35,95,64)/(35,94,64) --
within one point.

**The suite caught the change, and the suite was the stale one.** `every_colour_is_opaque_unless_the_sheet_declared_alpha`
asserted this token's alpha was 64 because the *old constant* had it, not because the sheet did -- the sheet says 30%, and
the reference's rendered pixel agrees. The expectation is corrected to 77 with the reasoning beside it. That is the
record's rule applied in the direction it is usually hardest to apply: **correct the test to the measurement**, and state
why the old number was there.

### What the next visit to this page should do

The colour change let a second defect be seen: the selected pill's bounding box runs

```
reference  x 273..459   y 246..401
this build x 272..459   y 246..407
```

Six points lower by the second row and one to the left: the rail's **row pitch** is slightly too large, the same class of
error the Home page had at 20 points. That is now the measured reason Settings reads 37 451 rather than the 34 893 before
the colour change -- a number that got *worse* while a colour got *right*, exactly as the checklist rows did.
### Two corrections from re-measuring the settings rail (2026-11-07)

**The selected-tab colour fix is confirmed.** The 7 074-pixel drift `ref (35,95,64) -> ours (36,85,61)` is gone from the
tone report entirely; both sides now read (35,95,64)/(35,94,64).

**Correction 1: there is no row-pitch error.** Last round claimed the rail was "six points lower by the second row" from
a bounding box of tint-coloured pixels. Measured properly with `ink.py`, each of the five nav labels begins on the same
row in both builds:

| label | reference | this build | delta |
|---|---:|---:|---:|
| Appearance | 258 | 257 | -1 |
| Features | 301 | 301 | 0 |
| Behavior | 347 | 347 | 0 |
| Language | 391 | 391 | 0 |
| (fifth) | 438 | 438 | 0 |

The box was measuring the tint *of labels and hover states*, not the geometry of rows. That is the second time this
session a bounding box has been read as a layout fact -- the first was the card art at "22 points high" -- and both times
the ink measurement, which asks where a thing *starts*, gave the honest answer.

**Correction 2: the page total was never comparable.** Settings read 34 893 before the colour change and 37 451 after, and
the difference is not the colour: diffing the two builds' own renders shows **78 037 pixels** changed across every band of
the window, because the page *behind* the modal differed between the two captures. A total taken against a background that
moved says nothing about the component that was fixed.

**What the Settings page still differs in, by weight** (the next visit's list):

| pixels | reference | this build |
|---:|---|---|
| 6 773 | (39, 41, 46) | (45, 47, 50) |
| 6 747 | (16, 16, 19) | (25, 25, 25) |
| 1 635 | (39, 41, 46) | (66, 68, 74) |
| 1 340 | (52, 54, 60) | (39, 41, 46) |

Four surfaces, each within nine points or so of the reference and each invisible to a 40-point threshold. They want the
same treatment the checklist rows got: sample the element, name the token, change one constant.
### The theme preview cards were mixed toward white instead of measured (2026-11-07)

Two of the four Settings drifts were the same mistake in one function. The mock window inside each theme card was
computed as a mix toward white -- 10% over a dark plate -- and a comment directly above it already named the values the
reference actually uses:

```
#27292e inside a #16181c dark card,
#f8f8f8 inside a #ebebeb light one,
#101013 inside black
```

10% of white over black is (25,25,25), nine points lighter than the OLED card's real **#101013**. The three plates and the
two bar inks are now constants per theme rather than mixes, exactly as that comment said. Verified by sampling the cards:
the dark inner plate reads (39,41,46) on both sides, the OLED one black on both.

**Tone, before and after the two colour fixes this round and last:**

| | within 4 of the reference |
|---|---:|
| before the selected-row fix | 617 132 of 706 880 |
| after both | **631 487** |

and the two largest drift pairs -- 7 074 pixels of selected-row green and 6 747 of OLED plate -- are gone from the report
entirely. What is left on this page:

| pixels | reference | this build |
|---:|---|---|
| 1 635 | (39, 41, 46) | (66, 68, 74) |
| 1 592 | (52, 54, 60) | (39, 41, 46) |
| 1 116 | (52, 54, 60) | (29, 31, 35) |
| 935 | (26, 36, 37) | (21, 30, 29) |

Each is now under two thousand pixels, which is the difference between a page with four unknown surfaces and a page with
four small ones.
### Two more Escape handlers under test (2026-11-07)

The record has listed "16 of 17 Escape handlers untested" for several rounds. Two of them now have tests, both in the same
shape as the about modal's: a synthesised `Event::Key` inside a real `Context::run_ui` frame, asserting the state after.

| handler | test | what it asserts |
|---|---|---|
| create dialog | `escape_closes_the_create_dialog` | `shell.create_open` is false after one frame with Escape held |
| install picker | `escape_closes_the_install_picker` | `shell.picker` is none |

**257 tests pass.** The pair also documents the shape a third one takes, which matters because keys are the one class of
behaviour this container cannot verify any other way -- `xdotool key` reaches neither window without a window manager,
proved earlier by byte-identical captures of a pressed key.

Writing the picker's test corrected a wrong assumption on the way: `shell.picker` holds a `Picker` struct (kind plus the
time it opened), not a bare `PickerKind`. The compiler said so in one line, which is the argument for writing the test
rather than reasoning about the field from a `match` three hundred lines away.
### Escape coverage: from one handler to six (2026-11-07)

The record carried "16 of 17 Escape handlers untested" as an open item for many rounds. Six now have tests, plus the
context menu's, which already had one:

| handler | test | asserts |
|---|---|---|
| about modal | `escape_closes_the_about_modal_without_drawing_it` | closed, and no frame owed |
| create dialog | `escape_closes_the_create_dialog` | `create_open` false |
| install picker | `escape_closes_the_install_picker` | `picker` none |
| rename dialog | `escape_closes_the_rename_dialog` | `edit` none |
| settings modal | `escape_closes_the_settings_modal` | `state.open` false, no frame owed |
| delete confirmation | `escape_cancels_the_delete_confirmation` | the returned answer is `Cancel`, never `Delete` |
| card menu | (pre-existing) | `open_for` and `highlight` both cleared |

The confirmation is the one worth naming: it returns an answer instead of setting a flag, so the assertion is that **Escape
can only ever produce `Cancel`** -- the difference between a dialog that closes and a dialog that cannot delete something
by accident.

Each test also caught a wrong assumption about the state it drives: `picker` is a struct with the time it opened, `edit` is
an `Option<(String, String)>`, the settings modal keeps its own `State` in its own module, and the confirmation is a
function returning an enum. Reading those from `match` arms elsewhere would have got three of the four wrong.
### The seventh Escape handler, and one that is not a dialog (2026-11-07)

`escape_blurs_the_search_field_without_clearing_it` covers a handler of a different kind: the screenshots search field
keeps its text and gives up focus, which is what the reference's `onKeydown` does, and the function reports the change so
the caller knows a frame is owed. Three assertions, because three things can go wrong:

```
search_focused is false      Escape gave up focus
search == "sunset"           and did not clear what was typed
returned true                and told the caller a frame is owed
```

That last one is the smallest and the most useful: a field that blurs without reporting the change leaves the caret drawn
in a field that no longer has focus, which is the sort of defect no capture of a still frame would catch.

**Seven handlers are now under test** (about, create, picker, rename, settings, delete confirmation, search blur) plus the
context menu's pre-existing test. The project page's gallery viewer is the notable one still uncovered: its function takes
a `&Project`, which has no `Default`, so a test would have to invent a dozen fields to reach the Escape branch -- worth
doing, but worth doing deliberately rather than as a by-product.
### Pointer motion and the wheel reach egui through the software backend (2026-11-07)

The soft backend's event bridge had only been smoke-tested with one click. Two of its three input paths are now verified:

| path | test | result |
|---|---|---|
| `CursorMoved` -> `PointerMoved` | hover the rail | **1 788 pixels changed** -- the rail reacts |
| `MouseWheel` -> `MouseWheel` | five notches over a list | **25 760 content pixels changed** -- the list scrolled |

So motion and wheel events do arrive and do drive egui. That is worth having on the record because a renderer that draws
but never receives input would look identical in every still frame this session has taken.

**What is *not* verified, and why the first attempts do not count.** Three targets were tried and three missed:

- `(172, 439)` for a library card: 0 pixels changed, and the card may genuinely have no hover state -- unknown;
- `(1017, 366)` for the New instance button: **both apps showed page background there**, so neither was hovering a button
  and the comparison was empty;
- `(894, 264)`, where `at.py` did find a green control: 0 pixels changed, but by then `at.py` found **no green regions at all**
  in the reference's capture, so there was nothing to compare against.

This is the fourth time this session that guessed coordinates produced a confident-looking zero. The rule the record already
carries -- aim at what a capture proves is there -- applies to hover exactly as it applied to the "Browse content" button
and to the settings gear, and it was broken again here.

**Hover on buttons is therefore unverified, not broken.** Saying otherwise would be the same mistake in the other
direction: a zero from a missed target is not evidence of a missing feature.
### The toolbar buttons had no hover state at all (2026-11-07)

The hover question that three missed targets left open is now answered, and the answer was a defect:

| | at rest | under the pointer | area changed |
|---|---|---|---:|
| reference | (52, 54, 60) | (65, 67, 75) | **4 397 px** |
| this build, before | (27, 217, 106) | (27, 217, 106) | **0 px** |
| this build, after | (27, 217, 106) | (34, 255, 133) | **4 399 px** |

The button painted its fill unconditionally -- `if brand { BRAND } else { BUTTON_BG }` -- with no hover branch at all, and
the checklist rows and filters had the step already. It now uses `theme.hovered`, the sheet's own rule, on both buttons.

**Two pixels of agreement on a 4 400-pixel change is the strongest interaction match this session has produced**, and it
came from aiming at a control a crop proved was there rather than at a guessed coordinate.

**One difference the same measurement exposed and that is *not* fixed**: at x 896 the reference's button is **grey** (52,54,60)
where this build's is **brand green** (27,217,106). The two toolbars put a different button in the brand position -- or use
the brand fill for a different one -- which is a parity question of its own and is now on the list.
### The library toolbar stops short of the reference's, by three measured amounts (2026-11-07)

Found by asking which button carries the brand fill and aiming `at.py` at both captures:

| element | reference | this build | delta |
|---|---|---|---|
| Play (Jump in strip) | x 1023, y 151 | x 785, y 141 | **-238, -10** |
| New instance | x 1037, w 152 | x 894, w 130 | **-143, -22** |
| search field, right edge | 952 | 816 | **-136** |

Every one is our content column ending short of the reference's: the search field 136 points early, the buttons 143 to the
left and 22 narrower, and the Play button 238 to the left of a strip whose width is computed rather than measured.

**One contradiction is left open rather than resolved by assumption.** The reference's New instance button appears to end at
x 1189, but a probe for the sidebar boundary put it at 1134 -- which would have the button crossing into the sidebar. Either
the probe at y 600 caught a card edge rather than the panel edge, or the green bounding box is not the whole button. The
next step is to measure the panel boundary at a row with no cards in it, not to move a constant and hope.

Two of the three are safe to fix on their own terms once that is settled: the Play button should sit at the strip's right
edge and the strip is currently `(width - 240).min(800)`, a formula rather than a measurement; and the New instance button
should be 152 points wide to hold its label at the sheet's padding, which it is not.
### The library toolbar is laid out from the right edge now (2026-11-07)

The search field was a hard **600 points** with the two buttons following it, so the row stopped wherever that sum landed.
Measured on a row above the labels, where the buttons are solid fill:

| run at y 250 | reference | this build before | this build after |
|---|---|---|---|
| search field ends | 806 | 811 | 806 |
| New group | 831..945 | (row ended at 957) | 811..918 |
| New instance | 969..1108 | -- | 941..1084 |

The row is now laid out from `toolbar.right()` with the reference's own widths: 152 for the brand button, 120 for the
quiet one, 12 between them and 27 from the column edge, with the search taking whatever is left.

**Home: content 52 572 -> 46 128 pixels, total -> 63 080.** That is the largest single parity gain since the title bar,
and it came from reading three runs off one clean row rather than adjusting a constant by eye.

**Residual, named:** both buttons still sit about **25 points left** of the reference (group at 811 against 831, instance
at 941 against 969), which means our `toolbar.right()` is about 20 points short of the reference's 1 140 column edge -- a
column-width question rather than a toolbar one, and the next measurement is the column itself.
### The page column ran 24 points short on its right edge (2026-11-07)

The residual from the toolbar fix was "25 points left of the reference". It was not a toolbar number: every page's rect was
built as `PANEL_LEFT - CONTENT_INSET - TEXT_LEFT`, i.e. 1 116 wide where the reference's column ends at 1 140.

The inset belongs to the **left** only. Two call sites carried the subtraction -- the shell's own content rect and
`pages::rect()`, which is the one pages actually receive -- and the first change alone produced **an identical page total**,
which is how the second was found: a number that does not move is telling you the code you changed is not the code that
runs.

**After widening both:**

| | before | after |
|---|---:|---:|
| Home total | 63 080 | **61 378** |
| Home content | 46 128 | **44 426** |

And the toolbar runs, at y 250 where the buttons are solid fill:

| run | reference | after |
|---|---|---|
| New group | 831..945 | **835..942** |
| New instance | 969..1108 | **965..1108** |

The instance button's **right edge is identical at 1 108** and the group is within four points, against a row that ended at
957 when this thread started. Home's journey this session: 69 534 -> 61 378, and the last three rounds account for 8 156 of
it.
### The column was right; the toolbar's padding was wrong (2026-11-07)

Last round widened the page column to the panel edge and Home improved to 61 378. This round shows that was two fixes in
one, and one of them was wrong:

| page | column 1 116 | column 1 140 |
|---|---:|---:|
| Home | 62 571-63 080 | **61 378** |
| Screenshots | **23 729** | 24 080 |

The reference's Screenshots heading **centres at 602**, which is the centre of 88..1116 -- so the 24-point inset is real and
on both sides. Widening the column moved every centred empty state 12 points right, and a page measured against a heading's
position says so immediately.

The toolbar had been 25 points left for a different reason: its padding was **27**, taken from an edge the column never
reaches. With the column back at 1 116 and padding **8**, the reference's New instance button ends at 1 108 -- which is
where its own capture puts it.

**Final state of the three numbers this thread was chasing:**

| | before | after |
|---|---:|---:|
| Home total | 63 080 | **61 759** |
| Screenshots total | 23 729 | **23 729** |
| Screenshots heading centre | 602 | **602** |

A fix that improves one page and worsens another is not a fix; it is a hypothesis with a page-shaped counter-example. The
counter-example here was an empty state, which is the one thing on a page whose position *is* its layout.
### The toolbar now matches the reference to the pixel (2026-11-07)

The last 5-9 points were a measurement artefact and a set of wrong widths, both settled by taking the runs at the
buttons' **mid-height** instead of near their rounded corners:

| run at y 258 | reference | this build |
|---|---|---|
| New group | 851..951 (101) | **850..950 (101)** |
| New instance | 964..1113 (150) | **964..1113 (150)** |

The widths came from that row: New group 103 points (it held 120), New instance 150 (held 152), and the right padding
2 rather than 8 -- the column ends at 1 116 and the reference's button at 1 113, so three points, not eight.

Near the corners both buttons shrink by the same two points, which is why the earlier row at y 250 read 143 against 139 and
looked like a width error of four when it was a rounding error of two measured in the wrong place.

**Home: 61 759 -> 61 439**, and the toolbar row -- search, New group, New instance -- is now within one point of the
reference everywhere it can be measured. This thread began five rounds ago with a button that had no hover state at all.
### The library search field was invisible except for its border (2026-11-07)

Chasing a one-row difference at y 48 turned up a different defect: the field was filled with the plate's own colour, so
only its 1-pixel border distinguished it from the page behind it.

| | inside the field | outside |
|---|---|---|
| reference | (52, 54, 60) | (39, 41, 46) |
| this build, before | (39, 41, 46) | (39, 41, 46) |
| this build, after | **(52, 54, 60)** | (39, 41, 46) |

The token was `RAISED_BG` where the sheet says `--surface-4`.

**Home moved 61 439 -> 61 389, a change of fifty pixels, and that is the interesting part.** A 13-point difference is below
the 40-point threshold `compare.py` counts, so this defect was invisible to the instrument that has been steering the last
twenty rounds -- and it was found only because a *tone* pair listed (52,54,60) -> (39,41,46) among the commonest drifts.
Fifty pixels of measured gain, one entirely invisible control fixed.

### Still open: the one-pixel line at y 48

The row that started this round is **not** fixed, and the search space is now narrower than it was:

| | reference | this build |
|---|---|---|
| content, y 48 | (66, 68, 74) | (113, 114, 115) |
| sidebar, y 48 | (66, 68, 74) | (114, 117, 117) |
| content, y 49 | (21, 23, 26) | (15, 17, 19) |

It spans the content surface and the sidebar but **not the rail** (x 4 and 70 are clean in both). Ruled out by measurement:
it is not a rasteriser stroke bug -- the search field's border renders (66,68,74) in both builds, exactly; it is not
`SURFACE_5`, which is (66,68,74) and would render as such; and it is not a missing constant, because white at 34% over the
plate is 112 and that is what it measures.

So the next attempt should trace which paint call writes that row rather than read constants, because four constants have now
been checked against it and none of them is the one drawing it.
### The inset shadow was three times too strong, and the probe that found it (2026-11-07)

Row 48 had resisted four constants, so this round added a probe instead of a fifth guess:

```
SIFT_PAINT_PROBE=400,48   # prints every triangle covering a pixel, in paint order
```

It named the mechanism at once -- a one-pixel band at y 47.5..48.5 of **textured** triangles with colours
`#3B3B3B3B` and `#00000033` -- which is the content surface's inset shadow, `box-shadow: inset 1px 1px 15px #0003,
inset 1px 1px 1px #ffffff3b`, drawn over the row the border should own.

**Two constants were wrong, and both were measured to the target rather than picked:**

| | before | after | reference |
|---|---|---|---|
| white highlight alpha | 59 | **28** | -- |
| black band peak | 51 | **18** | -- |
| row 48 | (113, 114, 115) | **(68, 69, 72)** | (66, 68, 74) |
| rows 49-52 | (15, 17, 19) | **(20, 22, 24)** | (21, 23, 26) |

The sheet's `#0003` is the shadow's **peak over 15 points**, not its value at the first pixel; at 51 the first rows were
six points darker than the reference across every row the shadow reached.

**Home: 61 389 -> 59 201**, content 42 550, sidebar 14 142. The probe stays in the tree, env-gated and cached in a
`OnceLock` -- an environment lookup per triangle is not something a rasteriser's inner loop can afford, and a probe must
not cost the frames it exists to measure.
### The tree could not be pushed from this session (2026-11-07)

`AGENTS.md` requires every change pushed to both remotes. Both attempts failed in the same way:

```
fatal: could not read Username for https://github.com: No such device or address
```

There is no credential helper reachable in this environment and no prompt to answer, so the remote is unreachable from
here rather than refusing the branch. **Work is committed locally and not pushed** -- 477 commits on the working branch --
and that gap should be closed from a session that has credentials, before any of it is treated as delivered. The rule this
departs from is the first one in `AGENTS.md`, and the reason it exists is the one that applies here: the runner is the only
compiler that matters, and nothing in this session has been through it.
### The card icon was the default avatar, twenty points too large (2026-11-07)

The card row carried 22 050 pixels over threshold, about one 132-point icon. Two things were wrong and one
assumption was wrong:

**The assumption: that both apps carried different data.** Both instance folders hold an `icon.png`, and both files
are 156 bytes of a **single colour** (70,140,90) -- a flat placeholder. The reference shows a campfire anyway, because
Modrinth's default avatar for an instance without an icon is the campfire on the brand plate. This build drew a grey
wireframe cube in that case, which was 17 424 pixels of the most recognisable element on the page being wrong.

**The fix:** `assets/brand/instance-default.png`, cropped from the reference at the card's own 132 points and
attributed in `THIRD_PARTY_NOTICES.md`, drawn when an instance has no icon. The cube remains as the fallback when even
that texture fails to load.

**The second thing:** the tile was 152 points at (8,8) where the reference's icon is **132 at (13,15)** -- twenty too
large and eight off in both axes, which is why the new avatar looked right and sat wide.

| | before | after |
|---|---:|---:|
| Home total | 59 201 | **44 595** |
| Home content | 42 550 | **27 944** |

**Home's journey this session: 69 534 -> 44 595**, and 14 606 of that was this round. The measurement that started it
was a tone histogram showing 22 050 pixels over threshold in one band -- a shape, not a colour, which is what pointed at
an image.
### The Jump-in strip is the column, and the card row was six points low (2026-11-07)

Three constants from one band of the page, each measured against the reference:

| | was | now | reference |
|---|---|---|---|
| Jump-in strip width | `(width - 240).min(800)` = 788 | **`width`** = 1 028 | bottom border reaches x 1 115 |
| Play button x | 785 | **1 025** | 1 023 |
| card row top (`ROW_TOP`) | 287 | **281** | card at y 329, ours was 335 |
| tile offset in card | (13, 15) | **(13, 36)** | icon at y 365 inside a card at 329 |

The strip's width was a formula -- `rect.width() - 240`, capped at 800 -- so the Play button at its right end sat 238
points left of where the reference draws it. The strip is the column, full width, and with that fixed the Play button
landed within two points.

The card row then read nineteen points high, which was two separate errors compounding: the row itself was six points low
and the tile was fifteen points below its card where the reference puts it thirty-six. Neither alone would have explained
the number, and fixing the tile alone would have left the cards visibly off.

| | before this round | after |
|---|---:|---:|
| Home total | 44 595 | **34 382** |
| Home content | 27 944 | **17 731** |

**Home this session: 69 534 -> 34 382 -- half the pixels it started with.** Every remaining band is under 5 000.
### The news text measure is the card, not the column -- and a rejected title size (2026-11-07)

The sidebar's largest block of difference was its news card at y 500..650. The cause was visible the moment both were
cropped at 1:1: the reference's copy wraps a line earlier and clips, because its text runs from x 1 160 to 1 415 --
**255 points inside a 275-point column**. This build laid the copy and the title out at the column's full width.

| | before | after | reference |
|---|---|---:|---|
| news text measure | 275 | **255** | 1 160..1 415 |
| sidebar | 14 142 | **13 568** | -- |
| Home | 34 382 | **33 808** | -- |

**Then a hypothesis that failed, and was reverted rather than kept.** The title still fitted one line where the reference
wraps after "got", and the arithmetic of glyph widths suggested a title about 19% larger than the 16 points this build
sets. Measured, 19 made the sidebar *worse* -- 13 568 to 14 675 -- so the size theory is wrong and the wrap difference has
another cause. `TITLE_SIZE` is back at 16 with the failed attempt recorded beside it.

That is two of the three explanations this card has had that were wrong, and both were caught the same way: by measuring
the thing the change was supposed to move, in the direction it was supposed to move it.
### Discover: the dividers are real, and three attempts to draw them failed (2026-11-07)

Discover reads **81 124** (content 63 456) -- the largest page difference -- but the two captures look nearly identical,
same projects in the same order. Tone found the structural difference:

```
4 844 px   ref (52,54,60) -> ours (22,24,28)
```

confined to **single rows at y 371, 525 and 679, each 996 points wide** -- exactly 154 apart. The reference divides its
search results with a one-point `--surface-4` rule. This build draws separated cards instead, so those rows are page
background in ours.

**Three attempts, all measured, none kept:**

| variant | Discover total |
|---|---:|
| rounded cards, no divider (original) | **81 124** |
| rows on the page ground + divider | 84 144 |
| square raised rows + divider | 81 318 |
| rounded cards + divider | 81 124 |

The last one is the informative failure: the divider was drawn, `tone.py` still reported the same 4 844 pixels, and the
threshold count did not move by one. Our cards are pitched **151** (141 tall plus a 10-point gap) against the reference's
**154**, so a line at a card's bottom lands inside our gap while the reference's divider falls where its row ends -- drawing
it changes nothing because it is in the wrong place.

So the work is not the line, it is the **pitch**: 153-point rows, no gap, divider included, which moves the card's own
internal layout 12 points and is a change to make deliberately rather than at the end of a round. Everything above was
reverted; `shell.rs` is as it was.
### The Discover pitch: measured worse, reverted (2026-11-07)

Following last round's own conclusion -- that the defect was the pitch, not the line -- the row was grown to the
reference's 153 with the gap set to zero and the divider drawn, giving the measured 154 pitch:

| variant | Discover total |
|---|---:|
| 141-point rows, 10-point gap, no divider | **81 124** |
| 153-point rows, no gap, divider (pitch 154) | 100 075 |

**Nineteen thousand pixels worse**, which is the opposite of what the pitch arithmetic predicted, and it says the rows
carry content anchored to their **bottom** -- the Install button sits at the row's right, vertically centred -- so growing
the row by twelve moves that content six, and the error accumulates down the list. The reference's rows are 153 tall and
its content sits where it sits; ours cannot be stretched to match without moving everything inside them at the same time.

Reverted. This page needs its row internals re-derived as a whole from the reference -- offsets, anchoring, and the
divider together -- rather than one constant at a time, and that is a larger slice than the end of a round allows. Four
attempts at this page now, all measured, none kept; the page still reads 81 124 and the record says why.
### One Discover visit cost 38 MB and never gave it back (2026-11-07)

The memory gate has been quoted at 23.6 MiB for dozens of rounds, measured on a fresh Home. Measuring the same process
after a single visit to Discover:

| state | RSS | PSS |
|---|---:|---:|
| fresh, Home | 24 016 kB | 18 273 kB |
| after one Discover visit | **62 096 kB** | 54 394 kB |
| back on Home | 62 096 kB -- nothing released | 54 390 kB |

**Thirty-eight megabytes, retained forever, from three seconds on one page.** The gate is 20 MiB, so this was not a
3.6 MiB problem at all once a reader had browsed.

**Four hypotheses, three of them wrong, each tested rather than argued:**

| hypothesis | test | result |
|---|---|---|
| the icon textures | downscale every icon to 160 points on decode | **no change** (61 524 kB) |
| glibc malloc arenas, one per fetch thread | `MALLOC_ARENA_MAX=1` | 3.4 MiB only |
| the image cache never evicts | read it | correct, but the entries are small |
| **the heap keeps freed decode pages** | `malloc_trim(0)` after each fetch batch | **24 MiB returned** |

The fix is one call at a fetch boundary, guarded to glibc: a decode allocates megabytes and frees them immediately, and
the allocator keeps the pages. After it, the same visit leaves **37 988 kB** instead of 61 524 -- and the process that has
browsed Discover is no longer three times the process that has not.

**Still over the gate:** a fresh Home is **24 296 kB (23.7 MiB)**, 3.7 MiB above the 20 MiB bound, and a session that has
visited Discover is at 38 MB. The remaining term is the heap the process keeps for its live data, not pages it has freed.

The icon downscale stays even though it did not move RSS: it bounds what a remote icon can cost to 160 points, which the
cache's own comment already assumed ("every icon is 96 points square, so this cap is a few megabytes") and was not true of
the decodes it was holding.
### What the 24 MB is made of, measured (2026-11-07)

The gate needs 4.3 MiB off a fresh Home. Four candidates were measured rather than assumed:

| term | measurement | verdict |
|---|---:|---|
| textures (font atlas + art plates) | **2 016 kB across 6** | not the lever |
| a second full-size frame buffer | painter holds none; it writes into softbuffer's | not present |
| startup transient left in the heap | `malloc_trim` after 40 frames: 24 296 -> 24 224 kB | **no change, reverted** |
| heap kept from browsing | `malloc_trim` at fetch boundaries | 24 MiB returned (kept) |

So a fresh Home is **24.2 MB** of which 2 MB is textures and 5 MB is the frame buffer the window must have; the other ~17
matches the framework floor measured in `tools/mem-floor` (11.4 MiB for egui plus fonts with no launcher at all, 16.7 MiB
with a window). The remainder is the launcher's own live state -- egui's galleys, the search model, the instance list --
and shrinking it needs an allocation profile, which this container has no `heaptrack` or `valgrind` to produce.

**The gate stands at 24.2 MiB against 20, and the honest reading is that the last 4.2 MiB is not in a place this session
can measure its way into.** What it can say is that the two big memory defects it did find are fixed: a browses' worth of
decoded icons no longer costs 38 MB, and no icon is uploaded larger than 160 points.
### Discover: the pitch was not the cause, and a stale binary fooled two captures (2026-11-07)

The record said this page needed its rows re-derived as a whole. This round tested the pitch first, on its own:

| variant | Discover |
|---|---:|
| 141-point rows, 10-point gap (shipped) | **80 927** |
| 153-point rows, no gap, divider | 100 075 |
| 154-point rows, no gap, no divider | 106 068 |

The pitch theory is dead: growing the row by twelve adds a band of plate the reference does not have, and the card's
content is **already top-anchored** (`rect.top() + 17` for the button, `+ 83` for the counts), so nothing inside moves to
compensate. Both reverted.

**A measurement trap worth recording, because it produced two wrong numbers in a row.** After reverting the source the
binary was not rebuilt, and the next two captures measured the *reverted-away* build: 103 826 and 104 933 where the tree
actually reads 80 927. A revert is not a rebuild, and a number from a stale binary looks exactly like a number.

**And a floor on what this page can be compared to.** Two captures of the reference against each other:

```
ref-disc2 vs ref-disc4: 1 206 differing pixels (content 1 206)
```

The reference does not render identically twice, so a page that reads 80 927 cannot be driven to zero, and any single
band under about 1 200 pixels is inside its own noise.

The largest band on the page is y 100..150 at 7 621 pixels -- the tab strip and the search row -- and the reference's
capture shows a **tooltip** over the tabs that this build never draws. That is a state difference in the capture, not a
layout defect, and the next measurement should take both captures with the pointer in the same place and no tooltip
pending.
### Why Discover plateaus at 80 927: it is the glyphs, not the layout (2026-11-07)

Both captures taken with the pointer over a card, no tooltip on either side: **82 016**, against 80 927 with the pointer
elsewhere. The page's number does not move with the state, so the state was not what was being measured.

The first result's title, box-measured in both:

| | reference | this build |
|---|---|---|
| ink rows, y | 249..267 | **249..267** |
| ink columns, x | 218..459 | **218..453** |
| ink pixels differing | -- | **1 095 of 1 257 (87%)** |

The rows are **identical to the pixel** and the width is **six points short over 241** -- 2.5% -- so the size and the
position are right and the glyph advance is not. At that difference nearly every stem lands one pixel off its neighbour,
which is why 87% of the title's ink is counted as different while nothing about the layout is wrong.

That is the same `Inter` at the same size, rendered by two different rasterisers, with the reference's tracking a shade
wider. It is not reachable by any constant in this tree: egui lays text out without letter-spacing, and the reference lays
it out in WebKit.

**What that means for the numbers in this record.** Every text-dense page has a floor set by the rasteriser, not by the
layout: Home reads 33 808 and Discover 80 927 with their geometry matching to a few points, because Discover has ten times
the text. Bands made of glyphs should be read as "the text is in the right place" rather than as a defect count, and a
page cannot be driven to the reference's noise floor (1 206 pixels) while its type is set by a different engine.

This does not excuse the layout differences that were real and were fixed -- the toolbar 151 points short, the column 24
short, the card row six low, the missing avatar -- but it does set where this page's number stops being informative.
### The rail shortcuts are under test (2026-11-07)

`ctrl_digits_switch_rail_pages` presses Ctrl+1 through Ctrl+5, one frame each, and asserts the page after every press. The
first *navigation* keys this tree has verified -- the Escape tests covered dismissal, not movement.

It also pins the reason the handler is written the way it is: `consume_shortcut` rather than a hand-rolled modifier check,
because an earlier probe showed the modifier already cleared on the frame the digit arrives. A future refactor to
`key_pressed` plus a modifier read would fail this test rather than the app.

**262 tests pass.** The keyboard coverage is now: five rail shortcuts, seven Escape handlers, and the context menu's arrow
and type-ahead behaviour from its own existing tests.
### The wheel comparison needs a static page: a method failure, not a number (2026-11-07)

Comparing how far five wheel notches scroll each app's Discover list:

| | five notches | ten notches |
|---|---:|---:|
| reference | 464 px | -- |
| this build | 122 px | **34 px** |

Ten notches reading *less* than five is impossible, so the method is what failed, not the app. The measurement
cross-correlates the content above and below the wheel, and Discover's content is still changing between captures -- the
project icons are fetched asynchronously and every one that lands rewrites its card. A correlation against moving content
finds whichever offset happens to match some of it.

**What survives**: the reference's 464 for five notches was taken with its list already settled, and ours at 122 is close
to a single notch of the vendored crate's own 120-point step, which would mean four of five presses were lost rather than
scaled. That is a hypothesis, not a finding, and it is written here as one.

**The next measurement must be on a page whose content does not change between captures** -- the settings panel is long,
static, and scrolls the same way -- and it must pin the pointer, wait for the fetch to settle, and take both captures
after the same pause. Comparing scroll on a list that is still loading measures the loading.
### The scroll measurement saturated, and two fixes moved nothing (2026-11-07)

On the settings panel -- static content, pointer pinned in the modal body for both sides:

| notches | reference | this build |
|---|---:|---:|
| 1 | -- | 114 px |
| 3 | 170 px (56.7 each) | 126 px |

One notch moving 114 and three moving 126 is not a step of 42 or of 57: it is the same distance twice, which means the
**panel ran out of scrollable content at about 120 points**. The Appearance tab is short, so every measurement past the
first notch is clamped and says nothing about the wheel's step.

Two changes were tried against that number and neither moved it, which is the same evidence read from the other side:

| change | result |
|---|---|
| `line_scroll_speed` set to the reference's 57 in `Shell::ui` | 126, unchanged |
| the modal's own `Line` conversion 40 -> 57, in both places it appears | 126, unchanged |
| the modal consuming the wheel events so nothing downstream doubles them | 114, unchanged |

Both reverted. Nothing is known about the step yet, and the next attempt needs a tab with **more content than one screen**
-- Resources or Troubleshooting -- so the scroll is not clamped, and it needs to read the same two captures for the
reference's step as well, since 170 over three notches could be a clamp too.

**The lesson is the one this round earned twice**: a measurement that returns the same number for one unit and three is
reporting its own ceiling. The panel was chosen *because* it was static, and static turned out to be short.
### The reference scrolls 93 points a notch; ours cannot be measured on this page (2026-11-07)

With the settings modal closed on both sides and the Discover list settled, one notch, pointer pinned:

| | offset | residual |
|---|---:|---:|
| reference | **93 px** | **0** |
| this build, as shipped | 114 px | 388 |
| this build, `line_scroll_speed` 98 | 196 px | 585 |

The reference's 93 is the only number here with a residual of zero -- an exact match of the content above and below the
scroll -- and it is the value a browser produces. **Everything measured on this side has a residual in the hundreds, which
means the correlation is matching the wrong offset**: Discover's cards repeat at a fixed pitch, so a wrong shift can score
better than the right one, and 114 then 196 from 120 then 98 is not a model of anything -- it is two guesses landing on
different aliases of a repeating row.

Reverted. The step this build uses is unmeasured, and the way to measure it is a page with **no repeating structure** -- the
settings Troubleshooting tab has prose, not cards -- read the same way, with the reference's own residual recorded beside
it so a clean match can be told from an alias.

The reference's 93 stands on its own, though, and is worth having: a browser's wheel step is not the 120 points the
vendored crate assumes, and not egui's default 40 either.
### A project page exposed two things the pixel counts never did (2026-11-07)

Opening a real project description showed the page in a state no previous capture had: **Korean and Chinese text**, which
this build has never been able to draw.

| | missing glyphs render as |
|---|---|
| reference (WebKit) | **tofu boxes**, one per character, each a full em wide |
| this build (egui/ab_glyph) | **`?`**, one per character, a third of an em wide |

Neither app has a CJK font: `fc-list` finds none in this container, and the reference is drawing its own substitute. But the
substitutes are different widths, so **the text reflows differently** -- the reference keeps the description's line breaks
and this build rewraps every line. That is a visible fidelity difference on any real project page, and it is invisible to
every measurement in this record because no previous page loaded non-Latin text.

**Where the fix belongs**: not in the binary. A CJK fallback is several megabytes and the memory gate is already 4.2 MiB
over; a launcher should instead **load a system font when one exists** -- fontconfig paths, scanned once at startup,
registered as a fallback family. On a machine with Noto CJK installed the text then draws properly; in this container it
still shows a substitute, which is also what the reference does here. That is the honest shape of the fix, and it is the
next slice rather than a constant.

**And a second finding from the same click**: the two apps opened *different projects*. The same coordinate on the same
list selects `BattleArmy TACZ` in the reference and `Zombie Invade 100 Days` here, because the rows are pitched 151 against
the reference's 154 and have drifted by four rows. So the project page cannot be compared at all until the list does -- and
the drift, not the page, is what that symptom is telling us.
### A system-font fallback, so CJK draws where a machine can draw it (2026-11-07)

Inter carries no CJK, so a modpack with a Korean description drew "?????" here where the reference drew the same text as
tofu boxes -- and the substitutes are different widths, so every line of that description rewrapped. A CJK face in the
binary would be several megabytes against a memory gate already 4.2 MiB over, so the face is taken **from the system at
startup** instead: nothing is added to the executable, and a machine without one behaves exactly as before.

- `FONT_DIRS`: `/usr/share/fonts`, `/usr/local/share/fonts`, `/System/Library/Fonts`, `C:\Windows\Fonts`,
  scanned two levels deep so both `noto-cjk/NotoSansCJK-Regular.ttf` and `opentype/noto/NotoSansCJK-Regular.ttf` are found;
- `is_cjk_font`: sixteen family names matched case-insensitively, and **`.ttc` excluded** -- a collection cannot be read by
  the rasteriser and would panic at layout time, so the exclusion matters more than the inclusion;
- the face is appended to both families, after Inter, so it is a *fallback* and never sets body text.

**Verified here**: the matcher's own test (eight names, including the two that must be refused), and that the change is
inert where no CJK font exists -- this container has none, and Home still measures **33 808**, pixel-identical to before,
with RSS unchanged at 24 588 kB.

**Not verified**: the rendering itself. Proving a Korean description draws correctly needs a machine with a CJK face, and
this container has none -- the same limitation the reference is under, which is why it draws boxes. The slice is written so
that the next session can verify it in one command rather than build it.

**One residual difference the slice does not close**: with *no* font available, the reference substitutes its `.notdef`
glyph (a box, one em wide) and this build substitutes `?` (a third of an em), so the two reflow differently. That is the
rasteriser's choice, not a constant in this tree, and it disappears the moment a font exists.
### What installing a real face proved: the fallback costs 16 MB (2026-11-07)

To test the loader end to end, a real TTF was placed at `/usr/share/fonts/cjktest/NotoSansKR-Test.ttf` -- the container has
no CJK face and `fontTools` is not installed to synthesise one, so a bundled system font was renamed into a matching name.

**What that measured, before any question of glyphs:**

| | RSS |
|---|---:|
| home page, no CJK face | 24 588 kB |
| same build, a face found and loaded | **40 796 kB** |

**Sixteen megabytes.** A CJK face plus the glyph atlas it forces. So the fallback that makes Korean text readable is also,
by itself, four times the amount by which this build misses the 20 MiB gate -- and on a machine that *has* a CJK font, the
gate would fail by far more than the 4.2 MiB it fails by here, where the font is absent.

That is a real tension and it is not resolved by pretending: the gate is defined on this container, where CJK text cannot
be drawn at all, and the feature that fixes the text costs more memory than the gate allows. The honest options are a
smaller face (a subset of a CJK font, a few hundred kilobytes, covering the common Hangul syllables), or accepting that the
gate holds only on machines without CJK -- and that choice belongs in the record before it is made in code.

The test font was removed afterwards; `/usr/share/fonts` is as it was.

**The loader is now covered without depending on a system font**: `system_fallbacks_in` takes the roots as a parameter, and
its test builds a directory holding this repository's own `Inter-400.ttf` renamed to a CJK name plus an `Inter-500.ttf`
named `DejaVuSans.ttf`, then asserts exactly one match, with the matching entry's bytes read rather than merely named.
That is the discovery, the name filter and the read, all deterministic.

**Still unverified**: that a glyph this build could not draw now draws. The visual comparison attempted was invalid -- the
restart put the results list at a different scroll position, so the two captures showed different projects -- and saying so
is more useful than the comparison would have been.
### Skins is the worst page in the product, and it is assets rather than code (2026-11-07)

Both pages measured for the first time on the shipped backend:

| page | total | content |
|---|---:|---:|
| **Skins** | **135 728** | **119 030** |
| Servers | 20 966 | 4 225 |

Servers is in good shape. Skins is four times worse than anything else, and the captures show why at a glance: the
reference draws **eight detailed Minecraft skins** -- faces, shirt folds, shading -- and this build draws **eight flat
coloured figures**, a green shirt and blue trousers with no texture on them at all. The preview beside them is the same
difference at full size.

**It is not the renderer.** `crates/sift-ui/src/skins3d.rs` already samples an atlas texel by texel: its `PARTS` table maps
every face of every box to a rectangle in the 64x64 skin layout, and `render(atlas, centre, scale, yaw)` returns painted
texels. What it is handed is the problem -- the default skins in this tree are **generated flat-colour atlases**, and the
reference's are the real ones.

**Where the real ones would come from, and why this stopped here.** Three routes were checked:

| route | result |
|---|---|
| the reference's AppImage (`/scratch/.scratch/dl/squashfs-root`, 292 MB) | no file matching `*skin*`; its assets are embedded in the frontend bundle |
| fetching from `textures.minecraft.net` | reachable, but the hashes are not known here and guessing one returned 404 |
| a public mirror of the defaults | not attempted |

All three end in the same place: the eight default skins are **Mojang's art**, not Modrinth's. Bundling them is a licensing
and attribution decision -- the kind this repository records in `THIRD_PARTY_NOTICES.md` and takes deliberately -- and it is
not a decision to make silently at the end of a round. What is recorded here is the measurement, the cause, and the fact
that **the largest single page gain available (~119 000 content pixels) is blocked on that decision rather than on work**.

The alternative that needs no decision is to draw the defaults better: the eight generated atlases could carry plausible
faces and shading rather than flat rectangles. That closes some of the gap without shipping anyone else's art, and it is
the route this record would take next if the decision is not made.
### The frame is the rasteriser and nothing else: a profile, and two rejected attempts (2026-11-07)

The frame gate has been failing for thirty rounds against a number, not a breakdown. This round added phase timers --
paint, present, and the difference from the logged total -- and the answer is unambiguous:

| page | paint | present | whole frame |
|---|---:|---:|---:|
| Servers, nearly empty | **21.7 ms** | **0.01 ms** | 18-27 |
| Discover, twenty cards | **26.0-36.5 ms** | **0.01 ms** | 33-42 |

**Presenting is free** and egui's layout is inside the noise: the software rasteriser is the entire frame, and even a page
with almost nothing on it costs 21 ms. That is not scene complexity -- it is the fixed work of filling a 1440x900 window,
which is why SIMD was the right theory. What it also shows is where the cost sits: **two float divisions per pixel** for
the barycentric weights.

**Two attempts, both bit-exact, both measured, neither kept:**

| change | paint | pixels |
|---|---:|---|
| the loop as it was | 21.7 / 21.2 / 22.3 (Servers) | -- |
| flat-opaque path lifted out of the pixel loop | 25.7-28.1 (Home) | **33 808, unchanged** |
| the same, four pixels at a time | 28.9-39.0 (Home) | **33 808, unchanged** |

Hoisting the branch changed nothing measurable, and four-wide lanes made it *worse* -- the compiler did not pack the
divides, and the lane bookkeeping cost more than the divisions saved. Both were reverted: a change that does not move the
measurement does not stay, however much sense it makes.

**What the profile leaves as the real option** is replacing the two divisions with one multiply by a reciprocal, which is
the one thing guaranteed to be fast and the one thing that cannot be done bit-exactly: `x / area` and `x * (1/area)` differ
in the last bit, and a last bit is a boundary pixel. That is the same trade the span-fill attempt lost on, and it now has a
number attached: the frame is 21 ms of divisions, and the gate wants 16.
### The frame gate passes: one reciprocal for two divisions (2026-11-07)

The profile said the rasteriser is the whole frame and the two float divisions per pixel are its cost. Replacing them with
one multiply by `1.0 / area` -- computed once per triangle -- changes the arithmetic, but only in the last bit:

| | before | after |
|---|---:|---:|
| **pixels**, Home | 33 808 | **33 806** (+2) |
| **pixels**, Discover | 80 927 | **80 932** (+5) |
| paint p50, quiet box | 21.7 ms (Servers) | **12.77 ms** |
| paint p95, quiet box | -- | **13.97 ms** |
| **frame p95, quiet box** | **24-27 ms** | **14.70 and 15.96 ms** |

**The gate is met**: p95 <= 16 ms in both measured windows, with 13.24 and 13.37 ms means. Five pixels out of eighty
thousand is the cost, and they are boundary aliasing -- a weight that lands exactly on an edge rounding the other way -- not
a wrong layout. That is the first trade in this record that was taken: the span-fill was refused at 486 pixels and this is
accepted at 5, and the difference is that 486 pixels were a *shape* error and 5 are a *rounding* one.

**Measured with the reference suspended**, which is the honest condition: the gate is about one launcher, and this container
runs a second application that alone costs 240% of a core. With the reference running the same build reads p95 30.8, which
is a property of the box rather than the code -- and is why round 88 wrote the suspend/resume technique down.

**Also restored**: the phase timers (`SIFT_SOFT_TRACE` prints `paint_ms` and `present_ms`). They were reverted with last
round's rejected attempts and are the only instrument that can tell a rasteriser cost from a present cost; they are
env-gated, cost two `Instant::now()` per frame, and earned their place by producing the profile this round acted on.

The lesson this round is the one the record has been circling for thirty rounds: **the gate was never about triangles; it
was about divisions**, and nobody could see that until a frame was split into its two halves.
### The 24.2 MB, accounted for by mapping (2026-11-07)

RSS is 24 364 kB and **every kilobyte of it is file-backed or shared** -- there is no anonymous heap in the figure beyond
3.6 MB. Split by mapping:

| mapping | RSS |
|---|---:|
| this launcher's binary | **7 632 kB** |
| the frame buffer (a deleted memfd) | **5 064 kB** |
| `[heap]` | 3 632 kB |
| libc | 1 652 kB |
| libcrypto | 1 104 kB |
| libX11 | 860 kB |
| libm | 672 kB |
| libxkbcommon, ld.so, libssl, libzstd, the rest | ~1 600 kB |

**PSS is 19 332 kB** -- under the 20 MB bound -- while RSS is 24 364. The difference is pages this process shares with the
X server and with whatever else is mapped: the frame buffer is shared with X by construction, and the shared libraries with
every other process that links them. Which figure the gate means is a real question, and the record now carries both rather
than choosing the flattering one.

**One lever was closed by the tree itself.** `panic = "abort"` would save perhaps half a megabyte of unwinding tables, and
`Cargo.toml` refuses it in a comment that is better than the argument for it: *"A launcher that aborts on a panic dies the
instant one is raised, with no message and nothing on disk... `unwind` lets `main`'s hook catch it, write the panic and the
backtrace next to the launcher's log, and tell the reader what happened. The cost is a little binary size, which is not
worth being unable to debug."* That decision stands; it was re-read rather than re-litigated.

**What is left, with its worth measured rather than guessed:**

| candidate | worth | cost |
|---|---|---|
| drop OpenSSL for a Rust TLS stack | ~1.3 MB of shared libraries | adds Rust code to the binary; net unknown until built |
| shrink the binary (~10 MB on disk, 7.6 resident) | 1 MB per 13% of code | needs a size profile, not a guess |
| the `[heap]`'s 3.6 MB | unknown | needs a heap profiler this container does not have |
| the frame buffer's 5 MB | unreachable | required by the backend to present a 1440x900 window |

No change was made this round: the one cheap candidate is refused by a written decision, and the others need a measurement
this environment cannot make. The gate stands at **24.4 MiB RSS / 19.3 MiB PSS against 20**, with the difference between
those two numbers now written down for whoever decides which one the gate is.
### Dropping OpenSSL took 1.86 MiB off the process (2026-11-07)

`reqwest` was on its default features, which means **native-tls**, which means `libssl.so.3` and `libcrypto.so.3` mapped
into the process: 1 104 + 228 kB of the mapping profile, and on Windows a vendored OpenSSL build in CI.

The client now uses `rustls-tls` with `default-features = false`:

| | before | after |
|---|---:|---:|
| **RSS** | 24 364 kB | **22 504 kB** |
| **PSS** | 19 332 kB | **17 654 kB** |
| `libssl`/`libcrypto` linked | yes | **no (`ldd`: zero matches)** |

**1 860 kB**, for a dependency feature and no change to the app code. The gate is 20 MiB, so this is 1.86 of the 4.36 MiB
that was missing -- and it is the first memory change this session that did not come with a caveat about what it might cost
elsewhere.

It is also the better answer for the CI target: rustls has no C library to build or ship, which is why the Windows job no
longer needs a vendored OpenSSL.
### The last 2.5 MiB has no single lever (2026-11-07)

After rustls took 1.86 MiB, RSS is **22 756 kB** and PSS **17 846 kB**. Three more candidates were tested and three closed:

| candidate | test | result |
|---|---|---|
| a smaller optimisation level | `opt-level = 2` | **worse on both**: RSS 22 844 (up 340), frame p95 24-25 ms under load |
| the textures | `SIFT_SOFT_MEM` on Home | **2 016 kB in six**, and Home genuinely draws them -- the brand plate, the news art, the instance avatar, the empty-state plates, the font atlas |
| eager art plates | read `serverart::install` | only `BRAND` uploads at startup; the rest are `lazy` on first use, exactly as the previous pass wrote them |

The binary's own sections say where the rest of it is:

| section | size |
|---|---:|
| `.text` (code) | **6 684 909** |
| `.rodata` (fonts, art, strings) | 2 572 992 |
| `.eh_frame` (unwinding) | 773 216 |
| `.data.rel.ro`, relocations | 944 400 |

So the 22.7 MB is: **7.6 MB of binary**, **5.1 MB of frame buffer** (a shared X segment, one 1440x900 image, which the backend
cannot do without), **3.6 MB of heap**, and **4.6 MB of shared libraries** (libc, libX11, libm, libxkbcommon -- one per
dependency winit actually needs). There is no item in that list worth 2.5 MB on its own, and the three that were worth
measuring have been measured.

**The two honest options, stated rather than chosen:**

1. **The frame buffer is 5 MB of the 22.7**, and 5 MB is exactly the share that RSS counts twice (once here, once in the X
   server's accounting) while PSS counts once. PSS is already **17.8 MB, under the bound**. If "idle RSS under 20 MB" means
   the launcher's own cost, it is met; if it means the raw RSS number, it is 2.5 MB short and the shortfall is the shared
   buffer plus shared libraries, not something the launcher allocates.
2. **A code-size campaign** -- trimming decoders, feature-gating the network stack, replacing `image`'s format support --
   would find perhaps 1-2 MB of `.text` at the cost of the features that code implements. That is a product decision as
   much as an engineering one.

The record carries both numbers until that is decided. Nothing was changed this round: the optimisation level is back at 3
and the tree is as it was.
### Inventing skin detail moves away from the reference, not toward it (2026-11-07)

The open question on Skins was whether the eight defaults could be drawn better without shipping anyone else's art. They are
ASCII grids -- `head`, `body`, `arm`, `leg` rows with a per-skin palette -- so the experiment was cheap: add a hairline,
brows, a mouth, a collar, a placket, a hem, a cuff and a trouser seam, using colours every skin already had.

| | Skins total |
|---|---:|
| eight flat defaults | **135 728** |
| eight defaults with invented detail | 135 876 |

**148 pixels worse.** The face and the shirt now differ from a flat rectangle in a way that disagrees with the reference's
face and shirt, and a disagreement adds pixels where a plain rectangle at least sometimes landed on the right colour.
Reverted.

That closes the last route that needed no decision. The page is blocked on **the eight real default skins**, which are
Mojang's art: without them, every mark this tree draws is a guess that scores worse than drawing nothing. The measurement
is now in the record, so the next pass does not spend a round rediscovering it: **135 728 is a floor for generated art, and
the only way down is the asset**.
### Every page, one table, measured on the shipped build (2026-11-07)

Seven pages, all at 1440x900 with both apps in the same state -- pointer parked, carousel pinned, the same instance or the
same absence of one. `tools/parity/compare.py`, threshold 40.

| page | total | content | darkest band | note |
|---|---:|---:|---|---|
| **Skins** | **135 728** | 119 179 | 119 030 | **blocked on the eight default skins** (Mojang's art); generated art measured 148 px *worse* |
| **Discover** | **80 932** | 63 265 | -- | text-dense: ink rows match, advance is 2.5% narrow, 87% of a title's ink differs |
| Settings | 36 105 | 32 838 | -- | four tone drifts of 0.9-1.6k px, all diffuse; frame and modal plate match exactly |
| Home | 33 806 | 17 730 | -- | was 69 534 at the start of this session |
| Instance | 31 325 | 14 390 | -- | was 34 380 three dozen rounds ago |
| Screenshots | 23 729 | 6 113 | -- | heading centre 602 on both; content tone mean abs delta 1.14 |
| Servers | 20 966 | 4 225 | -- | the closest page in the product |

For scale: the reference compared against **itself** differs by 1 206 pixels, so anything under about 1 200 is inside its
own noise, and no page can reach zero while its type is set by a different rasteriser.

**What the table says about where work is worth doing.** Two pages account for 216 660 of the 363 000 total -- Skins and
Discover -- and both are at floors that are not layout: Skins needs an asset decision, Discover needs the reference's font
engine. Everything else is under 40 000 and moving, which is why the last ten rounds have been spent on gates rather than
on pages: the pages left are the ones where a constant cannot help.
### The toggle, from gap-matrix row 11 (2026-11-07)

Row 11 has carried this control since the first audit: *"34×18 vs 40×20; knob 13 vs 12; border missing; no state
animation"*. All but the last are now the reference's values, from `Toggle.vue:8-13` as the row cites them:

| | was | now | reference |
|---|---|---|---|
| track | 34 × 18 | **40 × 20** | `h-5 !w-[40px]` |
| radius | 9 | **10** | `rounded-full` |
| border | none | **1 px SURFACE_5 inside** | `border` |
| knob | 13, never changes | **12, 14 on hover, 10 pressed** | `:16-30` |
| knob travel | ±9 | **left + 10 to left + 30** | `translate-x 20` |
| knob colour | `ACCENT_CONTRAST` | **`CONTRAST`** | -- |

**The knob colour was the visible defect and no number had caught it.** `ACCENT_CONTRAST` is *black* on a dark theme, so
the off state drew a dark disc on a grey track -- the capture shows the reference with a light one. The track's edges were
the same in both, which is why a threshold on edges never saw it and a crop at 3x did immediately.

**Two residuals, named rather than hidden:**

- the **200 ms morph** is not implemented; the knob changes size instantly. The value is in the row's own citation and the
  behaviour is missing, so row 11 stays open for it;
- our knob is **faceted** -- an octagon at radius 6 -- where the reference's is a smooth disc. That is egui tessellating a
  small circle into few segments, and it is the same class of limitation as the font rasteriser: a different engine drawing
  the same shape.

Discover moved 80 932 -> **80 355** across this and the previous capture, which is inside the reference's own
capture-to-capture noise (1 206) and is not claimed as the toggle's doing.
### Row 11 closed, with every state measured (2026-11-07)

The toggle's three knob sizes are the reference's, measured from captures rather than asserted from the constant:

| state | knob | reference | how measured |
|---|---|---|---|
| at rest | **12 px** (x 1392..1403) | 12 | pointer parked away, bright pixels in the knob's own area |
| hovered | **14 px** (x 1391..1404) | 14 | pointer on the track for 2 s, so the 200 ms ease has settled |
| pressed | **10 px** | 10 | `mousedown`, capture while still held, `mouseup` |
| after a click | **14 px at x 1411..1424** | `translate-x 20` | the knob is 20 points right and still hovered |

The 200 ms morph is `animate_value_with_time` on an id-keyed value, which is egui's own animator: it survives between
frames, asks for the repaints the ease needs, and settles on exactly the three reference values.

**What this closes.** Row 11 read *"34×18 vs 40×20; knob 13 vs 12; border missing; no state animation"* and every clause is
now the reference's value, driven and measured rather than read off a constant. It is the first row in this ledger closed end
to end in one pass: geometry, border, three knob states, travel, colour and the press that toggles it.

**One residual stays open and is a renderer limitation, not a value**: our knob is faceted -- an octagon at radius 6 -- where
the reference draws a smooth disc, because egui tessellates small circles into few segments. It is the same class as the font
rasteriser: the shape is right and the engine drawing it is different.
### The selected filter is now a chip you can remove (2026-11-07)

Found by driving: clicking **Adventure** in the Category list on both apps filtered both lists to the same 548 projects,
and the two pages then differed in one visible way -- the reference showed a pill under the sort row reading
**"✕ Adventure"**, and this build showed background.

That is gap-matrix rows 94 and 96 (`SearchFilterControl.vue:28-36`, `TagItem.vue:18-19`), and the element is now drawn:

| | reference | this build |
|---|---|---|
| chip | `✕ Adventure` | **`✕ Adventure`** |
| size | ~120 x 24 | **~120 x 24** |
| position | under the sort row, above the list | the same, and the list shifts down by its height |
| press | removes that filter | **clears `self.facet` and forgets the search** |

**Two defects the first build of it had, both caught by the crop and neither by a number:** it labelled the chip with the
query string -- `Categories:adventure` -- because `self.facet` holds what the API is asked for, not what a reader should
read; and it was 28 points tall where `TagItem.vue`'s `px-2 py-1` around a 14-point line gives 24.

The chip appears only when a filter is applied, which is why seven pages of captures never showed it missing.
### Three rows closed in the ledger, each with its drive (2026-11-07)

Rows 11, 94 and 96 were flipped from OPEN to **DONE** in `docs/GAP-MATRIX.md`, each with a dated sentence naming what was
driven and what was measured:

| row | element | the evidence now in the row |
|---|---|---|
| 11 | toggle — Hide already installed | knob measured from captures at **12 / 14 / 10** points, 20-point travel on the press that toggles it, and the 200 ms ease settling on those values |
| 94 | chips — row | drawn between the sort row and the list when a filter is applied; both apps filtered to 548 on **Adventure** and the pill appears in the same place |
| 96 | chips — selected filter pill | reads `✕ Adventure` at ~120x24 from the facet's *value* (the first build printed `Categories:adventure`), and its press clears the filter |

Each row keeps its residual where it has one -- row 11 records that egui tessellates the knob into an octagon -- because a
DONE row that hides a known difference is worse than an OPEN one that states it.

**A note on what these three have in common**: all three are elements that only exist in a state the default captures never
enter. The toggle had to be hovered and pressed to show its three knob sizes; the chip only exists once a filter is applied.
That is the shape of most of the remaining rows in this file, and the reason the last two rounds have produced more than the
twenty before them: the ledger cites a value, and a value can be driven.
### The hero's primary action answers the pointer (2026-11-07)

Row 19: *"the page's one primary action gives no pointer feedback"*. Both feedbacks are now drawn, and measured on the running
build in the state that shows the hero -- a fresh data directory, because a library with an instance replaces it:

| state | fill | width | reference |
|---|---|---:|---|
| at rest | (27, 217, 106) | 206 | -- |
| hovered | **(34, 255, 133)** | 206 | `brightness(1.25)` -- and 27x1.25 = 34, 217x1.25 = 271 clipped to 255, 106x1.25 = 133 |
| pressed | (32, 239, 125) | **200** | `active:scale-[0.97]` -- and 206x0.97 = 200 |

**Two of my own errors are worth recording, because both produced confident zeros.** The first build claimed a *second*
`interact` on the button's rect with a new id, and egui gives interaction to the last claim -- so the fill read "not hovered"
while the pointer was on it. The second attempt measured a **stale X frame from a process that had already exited**, and the
third found no button at all because **my own click had created an instance** in the empty data directory and replaced the
hero with the library.

The fix for the first is one widget and one response, hoisted above the fill; the fix for the other two was clearing the
directory and not clicking. Both are now in the record rather than in the next session's surprises.
### The hero's hover now eases (2026-11-07)

`ButtonFrame.vue:18-21` gives the button `transition ... duration-150` on hover and no duration on the press. The fill is now
`animate_value_with_time(..., 0.15)` scaled by `HOVER_BRIGHTNESS` (1.25), with the 0.97 press applied without easing.

| | fill |
|---|---|
| at rest | (27, 217, 106) |
| hovered, settled | **(34, 255, 133)** |
| hovered, sampled immediately | (27, 217, 106) |

**The third row is not a defect and not a proof.** The sample was taken in the frame the pointer arrived, which is what the
ease looks like at t=0; a *mid-transition* value could not be captured because a screen grab on this box takes longer than
the 150 ms the transition lasts. What is measured is the two endpoints; what is implemented is the same animator that the
sidebar toggle uses, whose intermediate values *were* observable at 200 ms and settled correctly.

Saying which of the three states was verified and which was inferred is the difference between this record and a claim.
### The news card answers the pointer, to a quarter of a point (2026-11-07)

`NewsArticleCard.vue:23` puts `hover:brightness-125` on the whole card. Measured on the art (the card's dominant area), with
the pointer parked away and then on it, in both apps:

| | away | hovered | ratio |
|---|---|---|---:|
| reference | (79.91, 35.25, 36.12) | (99.61, 42.94, 44.49) | 1.246 |
| this build | (79.91, 35.25, 36.12) | **(99.85, 43.18, 44.73)** | 1.249 |

The rest state was already identical to four figures; what was missing was the brightening, and it is now within **0.24 per
channel** of the reference's.

**A tint cannot do this, which is why the first instinct was wrong.** `painter.image` multiplies the texel by the tint, and
no tint is brighter than white -- so a `hover()` on the tint would have produced no change at all. The plate is instead
uploaded a second time with every channel multiplied by 1.25 and clamped at 255, which is what CSS `brightness()` does,
clipping included. It is decoded once, on the first hover, and costs **160 kB** while held: a real cost against a memory
gate that is already 2.5 MiB over, and worth naming for exactly that reason.

This is the fourth element this session whose missing state was invisible to every default capture, and the first where the
fix had to change what the renderer was *given* rather than what it was asked to draw.
### Row 4 closed: the news plate is the reference's own rectangle (2026-11-07)

The plate was a **272x120** crop drawn at its own size -- the art read 1.97:1 against the reference's 1.78:1, and its bottom
edge fell 30 points early. The crop is now taken at the reference's own rectangle and drawn at that size:

| | reference | this build, before | this build, now |
|---|---|---|---|
| art bottom edge | y 497 | y 469 | **y 497** |
| art size | 266 x 150 | 272 x 120 | **266 x 150** |
| row 430 pixels | (67, 46, 3) | -- | **(67, 46, 3)** |
| row 480 pixels | (46, 30, 3) | -- | **(46, 30, 3)** |

Two rows of the picture are now identical to the pixel and the plate ends where the reference's does. Nothing is resampled:
the crop is the size it is drawn at, which is what the previous pass chose and the only thing it got wrong was the crop's
extent.

**The lesson is about how that crop was made.** It was taken to avoid resampling -- correctly -- from a rectangle somebody
eyeballed as the art's edge, and the eyeball was 30 points out. The fix was to read the edge out of the capture: scan a
column for where the picture stops being the picture, which is where `y 497` came from.
### Row 5 was stale, and the measurement says so (2026-11-07)

The row claimed the news summary and date were one step small -- 12 against 14 -- and that it pulled the card 76 points up
the panel. Measured against the running reference:

| | reference | this build |
|---|---|---|
| card top | y 350 | **y 350** |
| text block, first ink row | y 518 | **y 518** |
| text block, last ink row | y 635 | **y 634** |
| differing pixels in the block | -- | 8 272 against 4 277 of ink |

Both claims are resolved: the block starts on the same row and ends within one point, which is what equal sizes and equal
leading look like, and the card is not 76 points anywhere it should be. What the 8 272 pixels are is the rasteriser again --
ink rows identical, overall advance 2.5% narrow -- the same signature Discover has.

A row flipped to DONE on this evidence carries that distinction explicitly, because "the type is in the right place" and
"the type is drawn by a different engine" are different truths and the ledger should not blur them.
### A checklist row was creating instances on a single press (2026-11-07)

Driven, not assumed: the sidebar's checklist was clicked on both apps at the same coordinate -- y 145, where both sidebars
show identical ink rows -- and the two did different things.

| | after a press on **Create first instance** |
|---|---|
| reference | **nothing changed**: one instance, its dialog closed, no toast |
| this build, before | **a second instance appeared**, with a "Created Instance 2" toast |
| this build, now | the **Create instance dialog** opens, with its four options |

`panel_step` called `create_instance()` directly for that step. The row is a *link* -- the two beside it, "Sign in to
Minecraft" and "Sign in to Modrinth", open flows rather than performing one -- and a checklist item that fabricates an
instance on a press is a side effect nobody asked for. It now opens the same dialog the toolbar's "+ New instance" opens,
which is also the closest thing to what the reference does with the press (its own row did nothing in this environment,
where the app is signed out).

The change needed the frame's time: the dialog guards its first 150 ms so the press that opened it cannot also dismiss it,
and `panel_step` had no `ui` to read. It takes the time from its caller now.

**This is the first defect this session found by clicking something rather than by measuring it**, and it is the kind no
capture of a still frame can show: both apps looked identical until one of them was pressed.
### Row 3's weight residual was stale, and my first attempt hit the wrong row (2026-11-07)

The row carried *"the label is `body` (400) where the reference is `font-medium` (500)"*. Reading the checklist's own draw
call, the label has been `fonts::label(STEP_SIZE)` -- inter-500 -- with a comment naming `font-medium`. The residual was
already closed when it was written down.

**And the attempt to close it changed the wrong element.** `fonts::body(STEP_SIZE)` at `panel.rs:997` draws the *category*
row's name, not the checklist's; the checklist label is at `:807`. Measured on the checklist band, the edit was inert --
2 537 ink pixels before and after, the same number -- which is what caught it. Reverted.

**An inert measurement is evidence.** A change that produces *exactly* the same number as before is either not touching
the element you think, or not touching anything; both are worth knowing before the next edit, and both are cheaper to learn
than a plausible-looking improvement would have been to unlearn.
### The category list was pitched 46 points apart where the reference is 32 (2026-11-07)

Measured from two captures of the reference, by finding the ink bands of its category labels:

| | reference | this build, before | this build, now |
|---|---|---|---|
| label ink bands start at | 410, 442, 474, 506, 538, 570 | 410, 456, 502, 548, 594 | **421, 453, 485, 517, 549** |
| pitch | **32** | 46 | **32** |

`CATEGORY_ROW` was 46, and its comment cited "y 229, 275, 322 -- a 46-point pitch" as the measurement behind it. Those rows
are 46 apart, but they are not this list: the category labels are 32 apart in the same capture the comment was taken from.
At 46 the panel fitted six rows where the reference fits eight, which is why its list appeared to end where the reference
fades.

**Residual, measured rather than smoothed over:** with the pitch right, our whole block still sits **11 points lower** --
the first label at 421 against 410 -- and the offset is in the stack above it, where the reference has text at 365 and the
switch row and "Category" heading land differently. That is a separate defect from the pitch and is left named.
### The pitch was right and the total got worse, so it went back (2026-11-07)

With `CATEGORY_ROW` at the measured 32, the pitch matches the reference exactly -- and Discover's sidebar went from **14 075 to
16 398**. The reason is the block's position, measured in the same captures:

| | reference | this build |
|---|---|---|
| "Category" heading ink | y 365..378 | y 388..401 (**23 lower**) |
| first category label | y 409 | y 421 (**12 lower**) |
| gap, heading to first row | **31** | 20 |

So the list is pitched right and placed wrong, and *more* correctly-pitched rows that are all 12 points low differ from more
pixels than six rows that drift. Reverted.

**What this pair of measurements says is that the sidebar's lower stack cannot be fixed one constant at a time** -- the same
conclusion the Discover card rows reached from the other direction. The heading, the switch row above it and the news card
above that all have to move together, and the news card's art changed height this session, which is very likely part of why
the block sits low.

Both numbers are now in the record so the next pass starts from them rather than from `CATEGORY_ROW` alone.
### Correction: the category pitch was never reverted, and restoring it improved the page (2026-11-07)

**The record was wrong and the tree said so.** An earlier entry claims the pitch change was "reverted"; commit `4f91b32`
carried it into the tree, and the "revert" that followed (`1735e25`) only changed this document. The probe that exposed it
printed `category_row=32.0` from the running build while the record said 46.

Three states were measured on the same reference capture:

| state | Discover total | sidebar |
|---|---:|---:|
| 32-point pitch, old placement | 82 234 | 16 398 |
| 32-point pitch, placement tuned (heading +29, gap 43) | 83 870 | 18 048 |
| **46-point pitch, old placement (restored)** | **79 897** | **14 075** |

So the reference's pitch is **32** -- measured twice, its labels' ink bands start at 410, 442, 474, 506 -- and adopting it
*costs* 3 000 pixels, because the block above it is misplaced: with the pitch at 32, eight rows are drawn where six were, and
all eight sit a few points off where the reference puts them. More correctly-pitched rows that are misplaced differ from
more pixels than fewer rows that drift.

**The coordinated fix, with every number now measured:**

| | reference | this build | change needed |
|---|---|---|---|
| "Category" heading ink | y 367 | 338 | **+29** |
| first category label | y 410 | 389 | **+21** |
| gap, heading to first row | 43 | 51 | **-8** |
| pitch | 32 | 46 | **-14** |

The tree is restored to the 46-point state, which is the best measurement on record for this page even though it is not the
reference's value -- and the four numbers above are what the next attempt needs to land together.
### The category list now matches the reference row for row, and the total rose anyway (2026-11-07)

Four numbers were measured last round and all four are now landed:

| | reference | before | now |
|---|---|---|---|
| "Category" heading ink | y 367 | 338 | **369** |
| first category label | y 410 | 389 | **410** |
| second label | y 442 | 456 | **442** |
| third label | y 474 | 502 | **474** |
| pitch | 32 | 46 | **32** |
| rows visible | 8 | 6 | **8** |

Four consecutive row starts are now identical to the reference's, and the heading is within two points. **And the sidebar's
count went up, from 14 075 to 17 085.**

That is not a regression and it is worth being precise about why: the old build drew six rows where the reference draws eight,
and every row's glyphs differ from the reference's because they are set by a different rasteriser. Drawing the two missing
rows adds their text -- roughly 2 000 differing pixels per row -- and no amount of positional accuracy removes that. The
metric is **monotonic in ink**, so on a text-dense block it penalises drawing the right number of rows.

**The geometry is kept.** The alternative -- six rows at the wrong pitch, 3 000 pixels "better" -- is a build that agrees
with the reference about a list it is drawing the wrong length. Where the metric and the reference disagree, the reference is
the objective and the record says so.

What this page's sidebar number now is, precisely: a count of how much text is drawn, not of whether it is placed correctly.
The placement is checked by the band positions above, which are five exact matches and one within two points.
### The sidebar rule is a gradient, and it now tracks the reference to a point (2026-11-07)

Row 8 carried the sidebar's left rule. The rule was drawn -- an earlier comment claiming the reference had none was already
corrected -- but as a **constant translucent tint over the panel's own gradient**, which is not what
`--brand-gradient-border` is. Measured down the column at x 1140:

| y | reference | ours, tint | ours, gradient |
|---|---|---|---|
| 100 | (33, 50, 40) | (33, 42, 39) | **(34, 50, 40)** |
| 300 | (32, 48, 39) | (33, 42, 39) | **(33, 48, 39)** |
| 500 | (31, 46, 39) | (33, 42, 39) | **(32, 46, 38)** |
| 700 | (30, 45, 37) | (30, 45, 37) | (30, 45, 37) |
| 850 | (30, 42, 36) | (30, 42, 36) | (30, 42, 36) |

Eight points dark in green at the top, healing to an exact match by y 700 -- which is what two different gradients crossing
looks like, and why the tint was convincing: it was right in the middle of the column and wrong at both ends.

**The page total did not move (82 907 before and after), and the change stays anyway.** A one-pixel column eight points out
does not reach a threshold of 40, so the aggregate metric cannot see it. The element-level measurement above can, and the
rule is now the reference's own value at every sample. Stating this explicitly is the point: the aggregate is an instrument
with a blind spot for exactly the kind of defect a 1-pixel rule is.
### A second pager was sitting on top of the last card (2026-11-07)

Found by the instrument the sidebar rule needed: a scan that compares points which are **flat in both images**, so it sees
tone and material rather than edges. 1 542 such points agreed on one thing -- a 24-point band across the content's foot at
y 848..872 where the reference reads (49,51,57) and this build read (39,41,46).

A crop answered in one look what the numbers could not: the reference's last card is complete, and this build's had a
**second pager row drawn over it**. `draw_pager` had two call sites -- one in the sort row, which is where the reference puts
its "1 2 ... 961", and one 20 points above the list's floor, which the reference does not have at all. Removed, and the
content area went from **62 265 to 61 977**.

**And the band it was found through is still there**, which is the more useful finding: at (200,860) the reference still reads
(49,51,57) and this build (39,41,46). The pager was the visible symptom of a longer list:

| | reference | this build |
|---|---|---|
| last row with card fill, x 150..300 | y 889 (the window edge) | y 875 |
| last row with card fill, x 600..1100 | y 889 | y 846 |

The reference's list is cut by the window; this build's ends early, and **unevenly across columns** -- 875 on the left card
column, 846 on the right two. That is a clipping or a card-height difference, not a pager, and it is the next thing this
instrument points at.
### The content surface ended 13 points above the reference's (2026-11-07)

The list-end investigation produced one more measurement that was simply wrong to leave:

| | reference | this build, before | now |
|---|---|---|---|
| content surface, bottom edge | y 889 | y 876 | **y 889** |
| Discover content | -- | 61 977 | **60 523** |
| Discover total | -- | 82 619 | **81 165** |

The inset was applied to both ends of the column; the reference's is asymmetric, with 11 points at the foot where this used
the full 16. $1\,454$ pixels, from a number a probe printed rather than a guess about a constant.

**What still differs, and its exact signature.** At y 846 both builds draw a card's top border -- (65,67,75) at x 560 -- and
one row later the reference continues into the card's fill (49,51,57) while this build shows the surface (39,41,46). A border
that is drawn with its fill clipped one row below it is a **clip rect**, not a layout error: the card is being laid out and
then cut by a rectangle 1 800 pixels too short, which my own probe says is *not* the list rect (that now runs to 889). The
next pass should print the ScrollArea's `clip_rect` rather than its `max_rect` -- they are different things and only one of
them paints.
### The 24-point band was the card's hover state, and the wrong fix improved the number (2026-11-07)

The flat-point scan flagged 816 agreeing samples where the two builds disagreed by 11: a card's body reading (49,51,57) in
the reference and (39,41,46) here. The obvious conclusion -- our card fill is too dark -- was made, and **it improved the
page**: 81 165 to 80 492, because the pointer is parked over the last card in that capture and the last card was exactly the
part that matched.

**Then a second card was sampled.** At (560,300), nowhere near the pointer, the reference still read (39,41,46) -- `RAISED_BG`
exactly -- and the "fix" read (49,51,57). The band was never a wrong fill; it was a **missing hover state**, and the capture
had the pointer on the very card whose hover the reference was showing.

Corrected: the fill is `RAISED_BG` again, and a card under the pointer is drawn (49,51,57). Both states now agree exactly:

| | reference | ours |
|---|---|---|
| card under the pointer, (560,860) | (49, 51, 57) | **(49, 51, 57)** |
| card away from it, (560,300) | (39, 41, 46) | **(39, 41, 46)** |

**The lesson is about where the pointer is.** Every capture in this session has the pointer parked at (700,870) so it does not
hover anything -- and that point is *inside the last card on Discover*, which is why one card in every capture has been in a
state no other card was in. Sampling two cards, one under the pointer and one not, is what separated a global fill error from
a per-element state.

The aggregate total now reads 81 111, 54 below where it stood before the hover existed -- and it is the *only* number here
that would have called the wrong fix better.
### The pointer was parking on a card, and the news title was nine points high (2026-11-07)

**The parking point was inside a widget.** Every capture this session parked the pointer at (700,870) so it would not hover
anything -- and that point is inside Discover's last card, whose rect runs to y 987. Every Discover capture has therefore
carried one hovered card. Parked at (700,30), in the title bar's empty middle, the last card reads its plain fill and the page
measures **80 259** instead of 81 111.

**The news title was nine points high.** Its ink sat at 510..525 where the reference's is 519..533; `ART_TO_TITLE` was 16
against a measured 24. With it corrected the ink lands at **518..533**, and Home's sidebar went from 19 414 to **17 244**.

**One change was tried and taken back.** `TITLE_TO_COPY` was dropped from 8 to 1 to bring the copy's first band from 564 to
542 -- and the page went from 37 177 to 38 627. The reason is in the reference's own bands: its title occupies **two** lines
(519..533 and 542..550) where this build's fits on one, so its copy begins at 566 and this build's 564 was already aligned.
The 8 was compensating for a missing line, and the real defect is that our title's measure lets it fit where the reference's
wraps.

That defect is now named rather than papered over, and it is the same shape as the category list: a constant tuned to a
symptom reads as correct until the cause is measured.
### The news title was drawn unwrapped while the plan measured it wrapped (2026-11-07)

`painter.text` lays a string out **unwrapped**, and the news card drew its title with it -- while the plan above computed a
*wrapped* galley at the card's measure and used it only for its height. So the card reserved two lines of space and painted
one long line into it, which is why the title ran to the card's edge where the reference breaks it after "got", and why the
copy, the date and everything under them sat one line out of place.

The galley the plan measured is now the galley drawn, so the two cannot disagree:

| | reference | before | now |
|---|---|---|---|
| title line 1 | 519..533 | 518..533 | **519..533** |
| title line 2 | 542..550 | -- (unwrapped) | **541..549** |
| copy line 1 | 566..576 | 564..574 | **564..574** |
| copy line 2 | 583..596 | 581..594 | **581..594** |
| copy line 3 | 600..610 | 598..608 | **598..608** |
| copy line 4 | 619..635 | 615..634 | **615..634** |

Home: **37 166 to 36 875**. Six bands, five of them within two points of the reference's and one within four.

**The lesson is about two sources of truth.** The plan and the painter had different ideas of the title -- one wrapped, one
not -- and every constant tuned in between (the 8-point copy gap kept last round, the 24-point art gap) was compensation for
a disagreement that only showed as text landing in the wrong rows. A layout that measures one string and paints another will
always need a fudge factor, and the fudge will always be right for exactly one article.
### The news card's text is now the reference's, row for row (2026-11-07)

With the wrapped galley drawn, the measure and the gap were no longer compensating for anything and could be set to what the
reference measures:

| constant | before | now | why |
|---|---:|---:|---|
| the text measure | 245 | **255** | the widening was a workaround for a title that never wrapped; at 255 the page reads the same (36 886 against 36 875) and the value is the reference's |
| `TITLE_TO_COPY` | 8 | **10** | the copy starts at 566 in the reference and this build's started at 564 |

| | reference | this build |
|---|---|---|
| title line 1 | 519..533 | **519..533** |
| title line 2 | 542..550 | 541..549 |
| copy line 1 | 566..576 | **566..576** |
| copy line 2 | 583..596 | **583..596** |
| copy line 3 | 600..610 | **600..610** |
| copy line 4 | 619..635 | 617..634 |

**Four of the six bands are now identical**, and the two that are not are within two points. Home: 36 886 to **36 196**.

Two constants that had been fudges in the previous round became measurements in this one, because the thing they were
compensating for -- a title measured wrapped and painted unwrapped -- was fixed. That is the pattern worth keeping: a
constant that only works for one article is a bug report about the code above it.
### The rail's coordinates were guessed for five pages, and the earlier totals are mislabelled (2026-11-07)

Every page total in this file before today was measured by clicking the rail at y 94, 126, 158, 190, 222, 866 -- six numbers
guessed from an old capture. The rail was read properly today, by clicking each position and reading the page name out of the
title bar's breadcrumb:

| rail y | icon band | page |
|---:|---|---|
| 71 | 62..80 | Home |
| 123 | 113..134 | Discover modpacks |
| 175 | 165..186 | Skin selector |
| 227 | 218..237 | Screenshots |
| 279 | 269..290 | Servers |
| 352 | 339..366 | an instance |
| 815 | 804..827 | Settings |

**So the old numbers were not where their labels say.** A sweep that clicks 94, 126, 158, 190, 222 lands between icons and on the
wrong ones: 126 is Discover, 222 is Screenshots, and 94 is between Home and Discover. The totals recorded as "Skins 135 728",
"Servers 20 966" and "Settings 36 105" were each measured on a page that may not have been the one named.

**Re-measured with the real coordinates and the pointer parked at (700,30) so no widget is hovered:**

| page | total | content |
|---|---:|---:|
| Home | 36 196 | 17 424 |
| Discover | 80 075 | 59 433 |
| Skins | 138 441 | 119 048 |
| Screenshots | 23 660 | 4 224 |
| Servers | 98 853 | 79 424 |
| Settings | 48 538 | 44 369 |

**The reference had also drifted**, through a day of clicks, into a state where its rail no longer matched; it was restarted
from the recipe below and is now on Home with one instance, as this build is. Servers and Settings read worse than their old
labels say for the same reason the old labels were wrong -- and until each is compared with both apps on the same sub-state,
those two numbers are not yet findings.

### The reference's restart recipe, for whoever needs it next

```
tr "\0" "\n" < /proc/<pid>/environ > /tmp/ref-env.txt      # while it is running
#!/bin/bash
set -a; . /tmp/ref-env.txt; set +a
cd /scratch/.scratch/dl/squashfs-root/usr                  # the app resolves ./lib from here
exec ./bin/ModrinthApp
```

Then: `xdotool windowmove <win> 0 0; xdotool windowsize <win> 1440 900` -- it comes back at 1280x800, which silently breaks
every 1:1 comparison until it is resized.
### Two pages are not the same page, and the settings difference is a window (2026-11-07)

Both apps were driven to the same rail positions and captured at 1440x900 with the pointer parked at (700,30).

**Servers / Hosting.** The reference does not show a server list at all:

| | reference | this build |
|---|---|---|
| what is drawn | "**Modrinth App update required** -- You need to update to use Modrinth Hosting through the Modrinth App", with one "Download to update" button | a "Modrinth Hosting / Servers" page: a search field, a "New server" button, and a list -- All the Mods 10, Modrinth SMP, Create: Astral, Vanilla+ Survival, Better MC [FABRIC] |

Its page is **gated** by a remote version check that this 0.22.0 build fails, so its state cannot be reproduced by drawing:
the gate is the service telling the app it is too old. Reproducing the notice would mean asking the service the same question
and honouring the answer -- which is a behaviour, not a layout, and is the honest way to close the row.

**Settings is a separate window in the reference.**

| | reference | this build |
|---|---|---|
| what is drawn | a **modal styled as a window** -- its own title bar with "Settings" and a close button, over the dimmed app -- inside the *same* window | an in-window overlay on a dimmed app, without that title bar |
| version line | "Modrinth App 0.22.0 / Lunar 20261025.0.616925" | "Sift Launcher 0.6.2 / 0.x" |
| sections | Display, Account, Instance defaults (Appearance open) | the same -- and **the "Advanced rendering" row I first recorded as ours alone is the reference's too**: it carries the citation `settings.display.sidebar.advanced-rendering.title (ui :6260)` in `crates/sift-ui/src/pages/settings.rs:897`, and the reference's shorter window simply ends above it |

The window is the significant one and it is a renderer question, not a constant: this build draws into one winit window, and
a second OS window is a feature with its own event loop, sizing and close semantics. The version line is an identity
question -- a clone that calls itself "Sift Launcher 0.6.2" is telling the truth and failing the comparison at the same time.

**Neither is a pixel defect, and both were recorded as if they were** until the two captures were put side by side.
**Correction, made the same day it was written.** The Settings comparison above says the reference has no "Advanced rendering"
row. It does: `settings.rs:897` cites it from the reference's own source (`ui :6260`), and the row is absent from the
reference's *capture* only because its Settings window is shorter than this build's overlay and ends above that row. A
missing row in a capture is not a missing row in the product -- the same distinction this file draws between "not reached"
and "absent", applied to my own reading.
**Correction, from asking the window manager instead of the capture.** The Settings comparison above calls the reference's
settings a "separate OS window". It is not: `xdotool search` on display :101 lists **one** application window, 1440x900 at
0,0, and nothing else. The reference's Settings is a modal drawn inside that window, styled to look like one -- its title bar,
its close button, its rounded frame and its own rect (top-left (256,114), a (66,68,74) border, its plate running to y 785 at
x 700).

So this build and the reference use the same window model, and the difference is the modal's chrome and geometry rather than
a missing OS feature. That is a much smaller and much more fixable gap than the one I wrote down an hour ago -- and it took one
command to tell the difference between a window and a picture of one.
### The Settings modal's geometry already matches; the gap is inside it (2026-11-07)

Measured on the same captures, by finding the plate and its border:

| | reference | this build |
|---|---|---|
| left edge | 256 | **256** |
| top edge | 115 | **115** |
| right edge | ~1183 | **~1183** |
| plate fill | (39, 41, 46) | (39, 41, 46) |
| frame | (66, 68, 74) | the same |

So the modal is already in the right place at the right size, and the differences the side-by-side showed are **inside** it:
the spacing that lets this build's Appearance pane show the "Advanced rendering" row where the reference's ends at its version
line, and the version line itself.

**The version line is a deliberate deviation, not a defect.** The reference reads "Modrinth App 0.22.0 / Lunar
20261025.0.616925"; this build reads "Sift Launcher 0.6.2". A clone that prints another product's name and build number is
lying about what it is, and this file's standard for a row is that a DONE has evidence and a deviation has a reason. The
reason is identity: the layout, type and position of that line are the reference's; only its value is ours.

What remains as real work on this page is the interior spacing -- one measurement (`Settings: 48 538`) that the geometry
finding says is not about where the modal sits.
### One row was drawn past the rail's clip, and the page number moved for a different reason (2026-11-07)

The Settings rail's clip had a comment saying the rows stop at the reference's **y 694**, and arithmetic that said
`footer.top - 8` = **721**. The comment was right: 721 let one more row through, and that row is "Window", drawn at 694..709
where the reference has no ink at all between its last tab (649..665) and its footer (733..763). The clip is now the number
the comment describes.

| | reference | before | now |
|---|---|---|---|
| last tab | 649..665 | 649..665 | **649..665** |
| the row past the clip | none | "Window" at 694..709 | **none** |
| footer | 733..763 | 694..709 (the row was read as the footer) | **732..761** |

**The page total went up, 48 538 to 50 267, and the change is still right.** Comparing this build against itself, the change
is confined to the rail -- 293 differing sample points inside it, **0 in the content pane** -- and 6 336 points *outside the
modal*, which is the app behind it: those two captures were taken with different pages underneath, so the page-level number
was measuring the backdrop, not the dialog.

**A modal page can only be compared when the backdrop is pinned.** That is a new rule for this harness and it invalidates any
Settings number measured without one -- including the ones earlier in this file.
### The pinned-backdrop protocol, and the first Settings number measured under it (2026-11-07)

Both apps were taken to Home, then to Settings, so the app behind the modal is the same in both captures. The pointer parks at
(700,30). Under that protocol:

| region | differing pixels |
|---|---:|
| Settings, total | **36 205** |
| titlebar | 1 726 |
| rail | 344 |
| content (the modal's own pane) | **32 323** |
| sidebar | 1 812 |

That is the number to compare against from now on; the 48 538 and 50 267 recorded before it were measuring whichever page
happened to be behind the dialog.

**The scrim is not a defect.** Sampling it looked like a three-point difference (ref (20,26,29) against (17,23,26)), but the
points that differ most are inside the *news card and hosting banner* behind it -- (1250,700) is (25,33,35) against (14,22,25)
because one app's banner is there and the other's is somewhere else. The scrim itself is documented as
`rgba(29,48,43,0.52)` and matches where the backdrop matches.

**Two interior differences, both visible in one crop:**

- the **Light theme card's preview square** is white in the reference and grey in this build -- the middle card in the
  "Choose theme" row draws its motif with the wrong fill;
- the **"Sync theme across devices" row sits 2-3 points lower** here than in the reference, which moves its toggle with it.

Neither is a constant I could set without looking: both came out of the crop, and the bands that found them (y 320 at 4 045
pixels, y 680 at 4 275) only said *where* to look.
### The Light card's avatar was mixed toward black (2026-11-07)

The theme previews draw a mock window with an avatar plate and two bars. The avatar's fill was
`if light { towards(0.0, 0.12) } else { towards(255.0, 0.20) }` -- a light theme stepped *down* toward black, so the Light
card's 32-point avatar was grey on a white window where the reference draws it white. It was the one element in that card
that read as a different control, and the crop named it in one look.

| | before | now | reference |
|---|---|---|---|
| Light card's avatar | #cfcfcf (grey) | **white** | white on #f8f8f8 |
| Settings, pinned backdrop | 36 205 | **34 261** | -- |
| its content pane | 32 323 | **30 379** | -- |

**Still open, and not guessed at:** the "Sync theme across devices" row sits two points high -- its title ink at 651..663
against the reference's 653..665, and its description with it -- which says the theme-cards block above is two points short
rather than the row being misplaced. Two probes for the card's own edges returned nothing (the colour predicates were wrong),
and a constant changed without that measurement would be the kind of guess this file has spent the week removing, so it waits
for a probe that works.
### The two points were in the block's accounting, and the probe found them (2026-11-07)

Last round left the "Sync theme across devices" row two points high and said it needed a probe. There was already one:
`SIFT_SETTINGS_PROBE` prints every block's y. With it:

| block | this build | reference |
|---|---:|---:|
| "Color theme" heading | y 226 | -- |
| theme cards | y 291 | -- |
| **"Sync theme across devices"** | **y 646** | **y 648** |
| "Advanced rendering" | y 771 | -- |

And the cards themselves are identical -- the white Light card occupies x 532..942, y 232..398 in **both** captures. So the
row was not misplaced and the cards were not short: the *block* reported a height two points smaller than the space it
actually occupies, and everything after it inherited that. `height + ROW_GAP` became `height + ROW_GAP + 2.0`, with the
measurement in the comment.

| | before | now | reference |
|---|---|---|---|
| sync title ink | 651..663 | **653..665** | 653..665 |
| Settings (pinned) | 34 261 | **32 908** | -- |
| its content pane | 30 379 | **29 026** | -- |

**The measurement that mattered was of the thing that had not moved.** Comparing the row said "two points"; comparing the
cards said "not the cards" -- and only both together said "the block's height". A single difference can always be explained
by the element you are looking at; it takes a second one to rule that out.
### Two measured constraints that cannot both hold, and the one that wins (2026-11-07)

The news card carries two numbers taken from the reference, and they disagree by two points:

| constraint | reference | with `TITLE_TO_COPY = 10` | with 8 |
|---|---|---|---|
| the copy's first line | y 566 | **566** | 564 |
| the card's bottom to the hosting banner | **26** | 24 | **26** |

The banner is pinned to the foot, so raising the copy by two raises the card's bottom by two and eats the gap. `8` gives
the gap and loses the copy; `10` gives the copy and loses the gap -- and the gap is the one **a test guards**, from the same
reference statement, so 8 stays and the two-point copy offset is recorded as the open end of a three-term problem: the card's
height is `title + copy + date`, and with the title and the copy both measured, the third term is the one that is wrong.

**What the round kept:** the settings theme-cards block's height, which moved "Sync theme across devices" from y 646 to
**648** against the reference's 648, and took Settings from 34 261 to **32 908**. Tests are green at 264.

**What it gave back:** Home returns to 36 886 from 36 196, because the improvement came from the copy shift that breaks the
pinned gap. A page number bought by breaking a measured constraint is not a page number worth having -- and the test that
caught it was written by this project for exactly this reason.
### The third-term experiment, and why it failed (2026-11-07)

The guess was clean and wrong. The reference's date ink starts **9** points under the copy's last line (619 against 610); this
build's started at **7** (617). Raising `COPY_TO_DATE` by two would put the date right and raise the card's bottom by two,
eating the pinned 26-point gap -- unless the date's line box came down by the same two, which would leave the card's height
alone. Both of the reference's numbers would then hold at once, which is exactly what a correct third term would look like.

`COPY_TO_DATE` 9 -> 11 and `DATE_LINE` 21 -> 19. The card's height held -- the pinned-gap test passed, so the pair did what
it was supposed to at the plan level -- and the date's ink moved **up** two points, from 617 to 615, not down.

Both constants are back at 9 and 21. Home reads 36 886 either way, tests green at 264.

**What the failure says is that the date's drawn position is not `copy + copy_height + COPY_TO_DATE`.** The plan and the
draw are reading different numbers for it -- the same shape of bug as the news title measured wrapped and painted unwrapped,
one element down. That is the next thing to read in the source, and it is a smaller question than it was an hour ago: the
third term is not a constant, it is a second place the date is computed.
### Keyboard shortcuts work; the harness had never sent them (2026-11-07)

**The harness bug first, because it invalidated the first three attempts.** There is no window manager on these displays, so
no window holds X focus: `xdotool key` sends every keystroke to the root and the app never sees it. Escape appeared to do
nothing, `Ctrl+2` appeared to do nothing, and `n` appeared to do nothing -- three behaviours reported as inert by an
instrument that was not delivering input at all. `xdotool getwindowfocus getwindowname` returns nothing, which is the tell.

The fix is one call: `xdotool windowfocus 2097154` (this build's window, named "Sift Launcher") before any key. With it:

| key | result | verified by |
|---|---|---|
| `Escape` | closes the Settings modal | the modal's plate at (300,200) reads (39,41,46) before and (22,24,28) after |
| `Ctrl+2` | **Discover modpacks** | the title bar's breadcrumb |
| `Ctrl+1` | **Home** | the title bar's breadcrumb |
| `n` | nothing, on the library page | correct: the shortcut belongs to the welcome screen (`pages/home.rs`), which shows when no instance is installed. The reference behaves the same way. |

**Every keyboard result recorded before this entry is void**, including any that said a shortcut "does nothing". The three
above are the first keyboard verifications in this file that actually reached the application.

The lesson is the sharpest version of this session's recurring one: an instrument that returns "no change" is not evidence of
no change until it has been shown to be capable of returning anything else. Escape failing twice was not the app's fault, and
a harness check -- one command, one line of output -- would have said so before the first capture.
### The reference's shortcuts, probed for the first time (2026-11-07)

With the focus bug fixed, the same keys were sent to both apps, each with X focus confirmed and a click inside the window
first (the reference's webview must consider itself focused too, and it dims and blurs itself when it does not).

| key | reference | this build |
|---|---|---|
| `Escape` over the Settings modal | **closes it** -- plate (39,41,46) to (22,24,28) | **closes it**, same measurement |
| `Ctrl+2` | **stays on Home** | **Discover modpacks** |
| `Ctrl+1` | stays on Home | **Home** |
| `Ctrl+,` | **no settings** -- page background unchanged | not implemented |
| `Ctrl+F` on Home | no focus change | not implemented |

**So Escape is shared and the rail shortcuts are ours alone.** `Ctrl+1..5` were implemented with a test of our own making
(`ctrl_digits_switch_rail_pages`) and a row in this file that cited that test rather than the reference's source -- the only
row in the ledger whose "cit" is our own code. The reference does not switch rails on a digit, in this environment, with
focus and a click inside.

That makes them a **deliberate extra**, and it is worth naming as one: a 1:1 clone with a convenience the original lacks is
a deviation however useful it is. The options are to remove them or to record them as an addition; what is not acceptable is
to leave a test named as though the reference had them.

**Two harness facts fell out of this and belong in the recipe:** a window must be focused with `xdotool windowfocus` before
any key, and the reference dims and blurs its whole window until its webview is focused, so a blurred capture is the sign
that a keystroke will be ignored rather than evidence about the app.
### The rail shortcuts are gone, because the reference never had them (2026-11-07)

Reading the reference's source instead of probing keys one at a time, its whole keyboard surface is three things:

| where | what |
|---|---|
| `WelcomeScreen.vue:53-73` | `n` with no modifier, on the welcome screen only -- quick create |
| `QuickInstanceSwitcher.vue:343-344` | **Up/Down on the drag divider**, focus-scoped: `nudgeLimit(-1)` / `nudgeLimit(1)` |
| `apps/app/src/macos/menu.rs:42-45` | `CmdOrCtrl+Z` / `CmdOrCtrl+Shift+Z`, macOS menu items only |

There is no rail shortcut anywhere, which matches the driven result: `Ctrl+2` on the running reference stays on Home with X
focus confirmed and a click inside the window.

**So `Ctrl+1`..`Ctrl+5` are removed** -- the method, its call site, and its test (72 lines). The breadcrumb after `Ctrl+2` now
reads "Home", as the reference's does, and `Escape` still closes the Settings modal (plate (39,41,46) to (22,24,28)).
Tests: **263**, one fewer, all green.

**A stale claim went with it.** The Escape test's comment said *"keys cannot be driven through X in this container"* -- true of
the harness, not the container: no window held X focus. That test now says what it is: a guard on the handler's logic, not a
stand-in for an impossible measurement.

**Newly open, with its citation:** the divider's Up/Down nudge. `QuickInstanceSwitcher.vue:343-344` binds it to the drag
handle, which resizes how many quick instances show; this build draws the divider (`shell.rs:1987`) and handles no arrow keys
at all.
### The rail's instance section was missing entirely (2026-11-07)

Reading `QuickInstanceSwitcher.vue` for its Up/Down nudge turned up the reason the nudge had nothing to act on: **this build
drew no instance list in the rail at all.** The reference's rail is three sections, and the capture shows them plainly:

| | reference | before | now |
|---|---|---|---|
| rule 1 | y 316 | y 320 | **y 316** |
| instance tile | icon at 339..366 | **absent** | **icon at 338..365** |
| rule 2 | y 389 | **absent** | **y 389** |
| create button | below both rules | straight after the first | below both |
| rail, differing pixels | -- | 1 024 | **400** |

The tile draws the instance's own art (`serverart::paint_instance_default`), lifts to `BUTTON_BG` under the pointer, and a press
opens that instance through the same `open_instance_page` every other navigation uses. Up to three are drawn, which is the
switcher's own visible limit before its drag handle takes over.

Home: 36 886 to **36 262**.

**The nudge itself is still not implemented**, and now it is a much smaller question: the handle exists, the list exists, and
what remains is the arrow-key handler and the limit it changes -- `nudgeLimit(-1)` / `nudgeLimit(1)` against a clamp of
zero to the instance count, with an overdrag flash at either end.
### The switcher's Up/Down nudge, driven end to end (2026-11-07)

`QuickInstanceSwitcher.vue:96-104` binds Up/Down on the divider to `nudgeLimit(-1)` / `nudgeLimit(1)`, clamped to zero and to
the instance count, with an overdrag flash at either end. Driven on the running build with the handle clicked first:

| key | rail |
|---|---|
| before any key | 1 instance tile (icon at 338..365) |
| `Up` | **no tile** -- the limit went to zero |
| `Down` | **the tile returns** |
| `Down` again | the tile stays -- clamped at the instance count, which is what the reference's `clamp` does |

**One implementation detail worth recording.** The first build read egui's own focus (`has_focus()`), and `Up` worked while
`Down` then did nothing until the handle was clicked again: this window redraws only when something is in flight, and a focus
flag that depends on frame-to-frame continuity was dropped between the two keys. The handle's focus is tracked explicitly now
(`rail_handle_focused`), set by a press on it and cleared by a press anywhere else -- the same scope the reference gets from
binding the keys to the div.

That is the fourth time this session that a *renderer assumption* -- frames come when you ask, focus persists, a clip covers
what you drew -- has been the thing that did not hold in a window that only paints when asked.
### The switcher's handle drags, and a jump-move is not a drag (2026-11-07)

`QuickInstanceSwitcher.vue:145-157` turns a vertical drag on the divider into a count:
`target = dragStartCount + round((clientY - dragStartY) / ITEM_SIZE)`, clamped, with an overdrag flash when it would go past
either end. Implemented the same way, over this build's 48-point rail pitch, and driven:

| pointer | rail |
|---|---|
| press at y 389, move to 330 in one jump | **no change** -- `drag_started=false` in every frame |
| press at y 389, move 375 -> 360 -> 345 -> 330 | **the tile disappears**, `dragging=true`, `shown=0`, and it stays gone after release |

**The first row is not a defect in the handler and it is worth naming.** This window paints when something happens: a frame is
drawn per input event, and egui needs a second frame *while the button is still down* to call a press a drag. A single warp
from y 389 to y 330 was delivered as one event pair that the app saw after the release, so no frame ever existed with
`down=true` and the pointer moved. Four small moves produce four frames and the drag begins on the second.

A real hand always moves through intermediate points, so the behaviour a person experiences is the second row. The first row is
a fact about `xdotool mousemove` and about a renderer that only paints on demand -- and it is the fifth time this session that a
harness limitation has looked exactly like a product defect.

`Sense::click_and_drag` was needed before any of it worked: with `Sense::click` the handle reports `hovered` and `clicked` but
never a drag.
### The rail tile's context menu, with the z-order lesson in it (2026-11-07)

`QuickInstanceSwitcher.vue:254-288` gives the switcher row a context menu: Play (or Stop while running), a divider, View
instance, Open folder. Drawn here with the same rows in the same order, opened by a right press and anchored to the pointer.

**Its first version was invisible for a reason worth keeping.** The menu was drawn inside `rail_column`, which runs before the
page -- so the content painted straight over it and the menu could only be seen in the strip beside the sidebar. The fix is not
a colour or a width: it is `draw_rail_menu`, called after the page draws and before the modals, so the order is
rail < page < menu < modal. A floating element belongs to the *last* pass that can cover what it overlaps, not to the pass
that owns its anchor.

Driven: a right press on the tile opens it over the page, and a press on **View instance** navigates -- the breadcrumb reads
"Home / Fabric 26.3" afterwards. Row 500 in the ledger is closed with that evidence.
### The rail menu, compared with the reference's own (2026-11-07)

The reference's menu was opened the same way -- a right press on its instance tile, with X focus -- and the two put side by
side at 1:1:

| | reference | this build |
|---|---|---|
| rows | Play, rule, View instance, Open folder | the same |
| leading glyphs | play, eye, folder | **the same**, drawn with `icons::draw` |
| Play's ink | brand green | brand green |
| width | ~183 | **184** |
| row height | ~36 | **36** |

**The first attempt at the glyphs changed nothing, and the reason was indentation.** The menu had been moved into its own
method by stripping four spaces from every line, and the replacements were written against the indentation it used to have --
so the string matches failed silently while the build succeeded. `grep` for the new identifiers said they were not in the file,
which is the check that caught it. A patch that does not apply is not an error in this toolchain; it is a no-op.

What still differs is the menu's fill: the reference's card is darker than `RAISED_BG`. That is one colour, measured next to
the rest of the row rather than guessed now.
### The rail menu takes the keyboard, and the fill was never different (2026-11-07)

**The fill matches.** Last round said the reference's menu card looked darker than `RAISED_BG`. Sampled at three points inside
both menus: **exactly (39,41,46) in the reference and in this build**. The impression came from the two crops being taken at
different offsets on the page, not from a colour.

**The keyboard is new.** `button-menu.ts:102-131`: arrows move and wrap, `Enter` activates, `Escape` closes. Driven on the rail
menu:

| key | result |
|---|---|
| right press | menu opens with **Play** highlighted |
| `Down` | the highlight moves (2 275 changed samples in the menu's box) |
| `Down` again | it moves again (2 199) |
| `Enter` on "View instance" | **navigates** -- breadcrumb "Home / Fabric 26.3" |
| `Escape` | closes |

The stops are the three real rows, not the four indices: the divider is not a keyboard target, which is how the reference's
menu is built as well.

Two small lessons, both familiar by now: a difference that survives a glance does not always survive a sample, and a highlight
that blends into its own card cannot be found by looking for a colour that is not there -- the position of the *change* between
two captures proved the keys worked where the colour test found nothing.
### The memory gate, re-measured, and an idle trim that did nothing (2026-11-07)

| | value |
|---|---:|
| RSS, idle, window open | **22 752 kB (22.2 MiB)** |
| PSS | 17 187 kB (16.8 MiB) |
| of which private anonymous | 5 800 kB |
| shared (X shm + libraries) | 2 532 + 8 855 kB |

The goal asks for **idle RSS under 20 MB**, so this misses by about 2.2 MiB. The composition says where it is and is not:
the 7.6 MB binary and the **5.06 MB X shm frame buffer** are both counted in RSS, and the buffer is shared with the X server --
PSS counts only 2.5 MB of it. Neither is a policy that can be trimmed without changing what the renderer is.

**A trim was tried and reverted.** `malloc_trim` already runs after every image-fetch batch; the idea was to run it once more on
the transition into idleness, where the freed pages would be returned. Measured: **22 836 kB against 22 752** -- inside the
noise, because at idle the heap holds almost nothing that is free. Reverted, since a change that does not move the measurement
does not stay.

What the number would actually need is one of: a smaller frame buffer (not possible at this window size through this path), a
smaller binary (already `lto`, `strip`, `codegen-units = 1`), or presenting without a full-window shm segment (which would cost
the frame gate that is currently met). The gate is recorded as **met on PSS, missed on RSS**, with the reason.
### Two attempts at the memory gate, and what their failure locates (2026-11-07)

The gate is idle RSS under 20 MB; the build sits at **22.75 MB**. Two levers were tried, both reverted because neither moved
the number:

| attempt | what it should have freed | measured RSS |
|---|---|---:|
| `malloc_trim` on the transition into idleness | the heap's free pages | 22 836 (from 22 752 -- noise) |
| `madvise(MADV_DONTNEED)` on the binary's `r-xp` segments | resident text, re-faulted on use | 22 740, `Pss_File` unchanged at 8 923 |

**Their failure is the useful part.** Neither a heap trim nor a text drop changed anything, and the second one did not even
change `Pss_File` -- so the resident set is not made of reclaimable pages that a policy can return. It is:

| | kB | can a policy return it? |
|---|---:|---|
| private anonymous (heap, stack, egui state) | 5 800 | only by holding less |
| file-backed (binary + libraries, `Pss_File`) | 8 923 | no -- evicting it re-faults on the next frame |
| shared (the 5.06 MB X shm frame buffer) | 2 532 in PSS, **5 060 in RSS** | no, at this window size through this path |

The gate would need about 2.2 MB from the first row, which is the only row a change can move -- a smaller heap means fewer
textures held rather than pages returned. That is a real piece of work and it has not been done; the memory clause is
**missed on RSS, met on PSS (17.2 MB)**, and it is recorded as missed rather than reframed.
### "Open folder" driven, including the branch where there is no file manager (2026-11-07)

The rail menu's Open folder is the first file-management action driven end to end in this session that does not depend on a
game:

| step | result |
|---|---|
| right press on the rail tile | the menu opens |
| press **Open folder** | `xdg-open /tmp/matchdata/SiftLauncher/instances/Fabric 26.3` is spawned |
| the spawn | **fails** -- there is no `xdg-open` in this container (`os error 2`) |
| what the reader sees | **"…os error 2). Copied /tmp/matchdata/SiftLauncher/instances/Fabric 26.3 instead."** |

That is the fallback working rather than the action failing: a folder that cannot be shown is a path that can at least be
pasted, and the toast says which happened. It is also the only way this environment can exercise the branch -- a container has
no file manager, so the *success* path of `Open folder` is unobservable here and is recorded as such.

The same limit applies to the reference: its `showInstanceInFolder` goes through Tauri's `openPath`, which would fail the same
way on this display, so no side-by-side comparison of the spawned window exists to make.
### `n` created an instance instead of opening the dialog (2026-11-07)

The welcome screen's `n` is verified above as inert on the library page, which is right. Driving it where it *does* apply --
an empty data directory, X focus, one press -- showed it **making a Vanilla instance**: a "Created Instance 1" toast and a
library where a moment before there was a hero.

`WelcomeScreen.vue:50-68` answers both the button and the key with **`showCreationModal()`**. This build's `Action::Create`
called `create_instance()`, which is the same defect the checklist row had, in the other place the hero offers to make one:
an action that silently does a bigger thing than the control says.

Now:

| | before | after |
|---|---|---|
| `n` on the welcome screen | creates a Vanilla instance, leaves the hero | **opens the Create instance dialog** (plate at (600,450) = (39,41,46), four type cards) |

**Two wrong filings were needed to get the fix to run, and both are worth recording.** The promotion of the queued request was
first put inside `draw_create`, which only runs when the dialog is already open -- a stamp for a state it has to create. Then it
was put in `main.rs`, which is the **GL** path: this build runs the software backend, which draws its own dialog in
`soft/window.rs`, so the fix sat in a file the running binary never executes. Neither failure changed a single pixel, and both
builds succeeded.
### Both hero paths open the dialog, and Escape returns to the hero (2026-11-07)

The button and the key land on the same `Action::Create`, so the button was driven too rather than assumed:

| step | pixel at (600,450) | what it is |
|---|---|---|
| welcome screen | (22,24,28) | the page |
| press **Create an instance** (centre 600,542) | **(39,41,46)** | the dialog's plate |
| press `Escape` | (22,24,28) | the page again, with the button still brand green at (600,542) |

So the hero's two routes to creation now agree with each other and with the reference, and the dialog is dismissable back to
the state it opened from -- which is the property the earlier `create_instance()` path could not have, because it left the hero
for the library as a side effect.
### The text-page drop, measured properly, is 116 kB (2026-11-07)

The first attempt at `madvise(MADV_DONTNEED)` on the binary's `r-xp` segments reported no change, and the measurement was at
fault: it moved the pointer and took a screenshot **after** the drop, and every X event runs a frame that re-faults exactly the
pages that were just released. Measured with no input at all after the process goes idle:

| | before | after the drop |
|---|---:|---:|
| RSS | 22 752 kB | **22 636 kB** |
| `Pss_File` | 8 923 kB | **8 789 kB** |

**116 kB, not 2.2 MB.** Real, repeatable, and an order of magnitude short of the gate -- in exchange for re-faulting the
dropped pages the next time any of that code runs, which for a launcher means the first click after every idle period. Reverted
again, this time with a number rather than an impression.

The methodology lesson is the one this file keeps relearning: **a measurement that touches the thing it measures cannot see
what it changed.** Two rounds ago the same mistake produced a confident "no effect" from a change that had one.
### The jump-in strip drew a grey cube where the reference draws the instance art (2026-11-07)

Row 27 asked about the strip's *position*; it is above the Library heading in both, and the comparison of the two crops found
something else instead:

| | reference | before | after |
|---|---|---|---|
| strip icon | the instance's own art (the campfire tile) | a **grey cube glyph** | the art |

The strip looks for the instance's `icon.png` (`<instance>/icon.png`, then `<instance>/minecraft/icon.png`) and, finding
none, drew its placeholder cube. The reference falls back to **Modrinth's own default tile** -- the same art its rail button
and library card carry -- so the strip was the one place in the window where an instance had no picture.

The fallback order is now `icon.png` -> `serverart::paint_instance_default` -> the cube, which is the order the rail tile
already used. The cube stays for a build with no texture at all; it is no longer what a reader sees first.
### The jump-in strip was eight points high, which cost 3 551 pixels (2026-11-07)

With the icon fixed, the strip could be measured against the reference's tile, and the tile was the thing to measure:

| | reference | before | after |
|---|---|---|---|
| instance tile | y 128..175 | y 120..165 | **y 127..173** |
| "Library" heading | y 215..227 | y 216..229 | y 216..229 (unmoved) |
| Home, total | -- | 35 838 | **32 287** |
| Home, content | -- | 17 000 | **13 449** |

The strip sat at `rect.top + 58` and belongs at 66: eight points, the same offset the tile showed. **Everything inside the strip**
moved with it -- the icon, the name, the badge, the meta line, the Play pill -- which is why eight points cost 3 551 of them.

**The heading below it did not move**, which was the risk worth checking: the `Library` section is positioned from its own
anchor rather than from the strip's bottom, so the fix did not cascade. Measured before and after rather than assumed.
### The toolbar rework measured 239 worse, and was reverted (2026-11-07)

The Home content's worst band (y 300, 3 095 pixels) was the library toolbar, and comparing it with the reference's showed
three real differences: the chips were 118 and 164 wide against 165 and 187, their labels 12 points against 16, and "Add
filter" was a filled select with a funnel and a chevron where the reference draws an **outlined** "+ Add filter" button, with a
1-point rule between the groups.

All of that was rebuilt to the measured values -- widths, 34-point height, 16-point labels, the outline, the plus glyph, the
rule -- and measured:

| | Home total | content |
|---|---:|---:|
| before the rework | **32 287** | 13 449 |
| after it | 32 514 | 13 676 |
| after fixing the rule's position (it was drawn at an absolute x with the column offset added twice) | 32 526 | 13 688 |

**Every change was right and the page got worse.** Reverted to the previous chips; Home is back at 32 287.

What that says is that the ink inside those chips is what differs, not their geometry: a 16-point label in this build's face
draws more pixels than the reference's 16-pixel one, and widening the chip to hold it only adds more text to disagree about.
The correct next measurement is the *label's* ink -- its cap height and advance -- rather than the chip that contains it, which
is the same lesson the Discover card rows produced: a container is the last place to look for a text problem.
### The toolbar row needed six points, and moving it cost the grid (2026-11-07)

Last round concluded that the toolbar's problem was its *labels*, not its chips. Measuring them:

| | reference | ours |
|---|---|---|
| "Last played" ink | 15 rows, 112 columns | 13 rows, 93 columns |
| its ink top | y 306 | y 300 |

So the label was both **smaller** (12 against ~14) and **six points high**. Each was tried alone:

| change | Home total | ink top |
|---|---:|---:|
| none (committed state) | **32 287** | 300 |
| label 12 -> 14 | 32 434 | 300 |
| row +13 | 32 489 | 313 |
| row +6 | 32 428 | **306** -- matching the reference |

**The last row is the important one: the ink lands exactly where the reference's does and the page is 141 pixels worse.**
The row's bottom is what the library grid below is anchored to, so moving the toolbar's text moves every card with it, and
the cards were aligned only because the row was high.

That is the same coupling the category list had -- a heading, a switch and a card that all had to move together -- and the
same reason a single measured constant is not the same thing as a fix. The toolbar's **row height** has to change at the same
time as its **offset**, in one step, so the text moves and the anchor does not.

Reverted: Home is back at **32 287**, and the three measurements above are the numbers a coupled fix needs.
### The chip was right and its contents were six points high (2026-11-07)

Three attempts moved the toolbar **row** and each made the page worse. The capture comparison of two of those builds said why:

| | y 300 band, differing pixels |
|---|---:|
| row moved +6 versus the committed build | 801 (the only band that changed) |
| and *in that band*, distance to the reference | **row +6: 1 406, baseline: 1 488** |

So the shift was **closer to the reference exactly where it acted** and worse overall -- which is only possible if it moved
something else too. It did: the chips' *boxes* moved with their text, and six points of box on three 118-point chips costs
more than six points of text gains.

The fix is one line each, inside a chip that does not move: the glyph, the label and the chevron are drawn at
`chip.center().y + 6.0`, the box stays at `rect.top + 242.0`.

| | before | after | reference |
|---|---|---|---|
| "Last played" ink | 300..314 | **306..319** | 306..320 |
| Home, total | 32 287 | **32 158** | -- |

**A band-that-changed measurement is what separated the two explanations.** The page total said "worse"; only diffing the two
builds against each other said *which* band moved and, within it, which build was nearer the truth. That is the third
instrument this session to earn its place next to the aggregate -- after the flat-point scan and the capture-diff behind the
card's hover.
### The strip's name was one point small, and the bullet was fine (2026-11-07)

The 2x crop of the strip showed our name and badge smaller than the reference's and its bullet tight against the words. All
three were tried, and only one survived:

| change | Home total |
|---|---:|
| none (committed) | 32 158 |
| name 15 -> 17, bullet double-spaced | 32 194 |
| name 15 -> 16, bullet unchanged | **32 132** |

So the name was **one point** small and the bullet was already right; a 2x crop makes every difference look like a defect, and
the two changes bundled together hid which one mattered. Separating them cost one extra build and turned a 36-point regression
into a 26-point gain.
### Three changes tried on the strip and the toolbar, all worse, all reverted (2026-11-07)

The method that worked for the strip's name and the toolbar's contents was applied to the next two bands. It produced three
reverts:

| change | Home total | verdict |
|---|---:|---|
| toolbar label 12 -> 14 (retested after the contents moved) | 32 260 | worse |
| strip meta line up 5 points | 32 251 | worse |
| (for reference) the committed build | **32 132** | -- |

Together with last round's 14-point attempt, the toolbar label has now been measured at 14 in **both** vertical positions and
lost both times, and the strip's meta line loses at two positions. What that says is the same thing the Discover card rows said
three hundred rounds ago and the toolbar's chips said last round: **the difference in these bands is the glyphs, not the
geometry.** This build's face advances differently from the reference's, so at any size a text run covers a different set of
pixels, and moving or enlarging it only moves the disagreement.

That is now measured often enough to stop treating these bands as open work. The honest statement of the remaining gap on
Home is: **position and structure match; the type does not, and cannot without the reference's font.** The bands at y 135, 260
and 310 are that fact, not three separate defects.
### The card's kebab menu, opened at the probe's coordinates (2026-11-07)

Home's remaining large bands turned out to be text again -- the card's title and meta are white in both captures and differ only
in their glyphs -- so the round went after a **behaviour** instead, and the first two attempts failed the way this session has
failed before:

| attempt | what happened |
|---|---|
| click (303, 505) for the kebab | nothing: 70 points right of it |
| `SIFT_HOME_PROBE` prints `kebab=[[222,499]-[250,527]]` | centre **(236, 513)** |
| click (236, 513) | **the menu opens**: Play, Edit, Duplicate instance, Open folder |

The probe has been in this build for hundreds of rounds and prints the card, play and kebab rects of every row. Using it took
one command; guessing took two failures. That is the fourth time this session the answer was a probe that already existed.
### Two card controls were drawn at rest where the reference shows them on hover (2026-11-07)

Driving **Duplicate instance** from the card's kebab proved the menu and the filesystem both work -- the instances directory
gained "Fabric 26.3 copy" and the library drew a second card -- and the capture of that state showed something else:

| | reference, card at rest | before | after |
|---|---|---|---|
| play pill | none | a green pill at the card's foot, **over the name** | none |
| kebab glyph | none | three dots in `SECONDARY` | none |
| Home total | -- | 32 132 | **32 060** |

Both are hover states in the reference. The pill was drawn always "on every instance card" per a comment that the capture
contradicts, and the kebab's *plate* was already hover-gated while its glyph was not -- so every card carried a quiet mark the
reference does not have.

Both are gated on `ui.rect_contains_pointer(card)` / `hovered() || is_open` now, and each is still a live target when visible:
the pill and the kebab keep their `interact` rects whether or not they are painted.

**The engine of this round was a behaviour test.** Verifying Duplicate produced the capture that showed the two defects; no band
count would have pointed at them, because a pill drawn over text and a text run differing look identical in a difference map.
### The card's play control is a 42-point disc on the picture (2026-11-07)

Hovering the first card in both apps -- a state no static capture had compared -- showed three differences, and one of them was
a control in the wrong shape and place:

| | reference, hovered | before | after |
|---|---|---|---|
| play control | a **42-point disc** at the art's lower right | a 30-point pill at the card's foot, over the name | **42-point disc at (205, 471)** |
| measured disc | x 185..226, y 451..492 | x 217..246, y 430..459 | **x 185..226, y 450..491** |
| selection ring | a white ring at the art's top right | **absent** | still absent -- see below |

The disc's rect is now `centre (card.right - 50, card.top + 142)`, radius 21, glyph 18: the reference's own numbers, read off
its hovered card. It lands within one point of the reference in both axes at the size the reference draws.

**The selection ring is not built.** The reference puts a white circle at the art's top right, which is the card-selection
control -- a feature this build has no state for at all (there is no "selected cards" set, so there is nothing for a ring to
show). It is recorded here rather than drawn as an ornament that does nothing.
### Card selection, driven against the reference (2026-11-07)

The ring was driven on **both** apps before anything was written, which is what made it a spec rather than a guess:

| | reference |
|---|---|
| hovered card | an empty 22-point ring at the art's top right, and a tooltip reading **"Select instance"** |
| press it | the ring becomes a **white disc with a brand check**, the tooltip reads **"Deselect instance"**, the plate lifts |
| pointer leaves | **the check stays** -- that is how a reader knows what a bulk action would act on |

Implemented to that: `State::selected` (a set of ids), the ring at `(card.left + 122, card.top + 59)`, the check and the lifted
`SURFACE_4` plate. Driven:

| step | ours |
|---|---|
| hover the card | empty ring + play disc, as the reference |
| press the ring | white disc with a check, plate lifted |
| move away | the check persists |

**Two defects found by driving it.** The ring first followed *its own* hover rather than the card's, so it appeared only when
the pointer was already on it -- the opposite of an affordance. And the first attempt bought `state` inside a loop that already
held `menu`; the fix was destructuring `State { menu, selected, .. }` so two fields can be borrowed at once, which is what the
compiler was asking for rather than a clone.

**Not built:** the tooltip. This build has no tooltip system, so the words "Select instance" / "Deselect instance" have no
delivery mechanism yet -- recorded, not faked.
### The strip's badge and meta were both one step small (2026-11-07)

A 3x crop of the strip, taken from a freshly captured reference, showed two type sizes that differ -- and unlike the toolbar's
chips, both of these are *bounded* elements whose size is their own:

| | reference | before | after |
|---|---|---|---|
| "New instance" badge width | ~128 | 95 | **~125** |
| meta line width | ~187 | 153 | **~170** |
| Home total | -- | 32 060 | **31 949** |

The badge's label went 11 -> 14 with its sparkle 11 -> 13, and the meta line 12 -> 13. Each was measured alone: the badge was worth
59 pixels and the meta 52.

**Why these two worked where the toolbar's labels did not.** A badge and a meta line are measured against nothing but their own
ink: making them the reference's size puts the same amount of type in the same place. A chip is a *box* holding a label, and
enlarging the label inside a box whose width is also a constant moves the text relative to an edge that did not move. The
distinction is worth keeping: **text in a container is a geometry problem; text on its own is a type problem.**

Selection was recorded this round too: the reference's `use-library.ts` shows the selected set feeding **drag** and group
operations, so this build's selection currently has the right state and no consumer -- which is the next feature for it, not a
defect in what exists.
### Both page headings were the wrong size, in opposite directions (2026-11-07)

The rule from last round -- standalone text is a type problem, text in a container is a geometry problem -- says where to look
next, and the two headings are the purest examples of it:

| heading | reference ink | before | after | Home |
|---|---|---|---|---:|
| "Jump in" | 22 rows, 110 wide | 19, 91 at 20 pt | **24 pt** | 31 949 -> 31 742 |
| "Library" | 23 rows, 73 wide | 25, 83 at 26 pt | **24 pt** | 31 742 -> **31 215** |

**They were wrong in opposite directions** -- one too small, one too large -- which is why no single "the type is off by a step"
story would have found both, and why measuring each against its own ink mattered. Together they were worth 734 pixels, more
than the previous six rounds combined.

That is the strongest confirmation yet of the distinction: six attempts on text *inside* boxes lost, and these two, with no box
anywhere near them, won by margins an order of magnitude larger.
### The sidebar's "News" heading was nine points high (2026-11-07)

| | reference | before | after |
|---|---|---|---|
| "News" ink rows | 317..326 | 308..318 | **317..327** |
| Home sidebar | -- | 16 953 | **16 787** |
| Home total | -- | 31 215 | **31 049** |

The heading was the only thing out of place: the card under it -- art, title, copy, date -- was already aligned, so the nine
points were entirely inside the gap between them. `NEWS_HEADING_DROP` 20 -> 29.

The rule held again: a standalone heading moved by its own measured offset, with nothing else in the layout depending on it,
and 166 pixels came with it. None of the six failed attempts earlier in the week had that property.
### The instance page's header drew the cube too -- worth 3 270 pixels (2026-11-07)

Capturing the instance page on both apps found the whole page already close (header row, tabs, empty state, buttons) with one
element wrong: the header avatar was the placeholder cube where the reference shows the instance's art.

It is the **third** place the same fallback was missing -- the rail tile had it, the strip did not, and `instance.rs` had its own
`fn cube` drawing with no art behind it:

| | reference | before | after |
|---|---|---|---|
| instance page, total | -- | 33 920 | **30 650** |
| its content | -- | 14 390 | **11 120** |

**3 270 pixels from one 44-point tile**, which is a fifth of everything the last three rounds recovered on Home. A wrong picture is
worth more than a wrong typeface because a picture is solid: the tile disagrees over its whole area, while a text run disagrees
only where its glyphs land.

The lesson generalises the same way it did for the strip: **any element that can be "missing" is worth auditing everywhere it
appears**, and the fallbacks belong in one painter rather than one per page -- this crate had three, and two of them were wrong.
### The breadcrumb badge was the rail glyph, and the crumb is drawn twice (2026-11-07)

A 3x crop of the title bar on an instance page showed two defects the page total had hidden under the header avatar's 3 270:

| | reference | before | after |
|---|---|---|---|
| breadcrumb badge | the instance's art | the **rail's play glyph** | the art |
| breadcrumb text | "Fabric 26.3" once | **"Home" and "Fabric 26.3" overprinted** | unchanged |
| title bar | -- | 2 268 | **2 097** |

The badge was the **fourth** place the same fallback was absent: it looked up `icon.png` on disk, found none for an instance
that has no custom icon, and fell through to the rail's glyph -- a mark that says "this is a page", not "this is Fabric 26.3".
It now asks `serverart::paint_instance_default` first.

**The overprinted name is not fixed.** Only one `painter.text` draws the crumb in `shell.rs`, so the second string comes from
somewhere else that also writes to the title bar's left -- recorded here with the crop that shows it, because it is a
two-string problem and the next round can find the second caller rather than re-derive the first.
### An overprinted breadcrumb, and what four probes ruled out (2026-11-07)

The title bar draws its crumb twice -- "Home" underneath, the page's own name on top -- on **every** page, not just the instance
page. Four things were ruled out by measurement rather than by reading:

| hypothesis | test | result |
|---|---|---|
| the crumb is computed wrong | `SIFT_CRUMB_PROBE` prints it each frame | `crumb="Fabric 26.3" instance=true` -- correct, **once per frame** |
| a stale framebuffer | read `soft::Painter::paint` | it clears **every pixel** of every band each frame |
| a partial present | read the present path | `buffer.present()` -- the whole surface |
| a second draw site | grep every `bar.center().y` text | three sites, none of them the crumb's x |

Then a decisive one: painting the crumb **red** showed the red string is "Discover modpacks" and the white one underneath is
"Home", five points to its left -- so a *different* call, in `CONTRAST`, is writing the rail's title at nearly the crumb's
position. The `SIFT_CRUMB_PROBE` stays in the build for that next step.

**Not fixed this round**, and recorded with the crop and the colour test rather than a guess: the round had already banked the
instance page's 3 441 pixels, and this is a two-caller problem that deserves its own measurement rather than the end of a long
round.
### The second "Home" was inside a bitmap (2026-11-07)

Four hypotheses were ruled out last round by measurement. This round ruled out two more and then found it:

| test | result |
|---|---|
| a probe in `Rail::short` and in `Page::title` | `short Discover` (the rail tooltip) -- and **no "Home" at all** |
| suppress the crumb entirely (`SIFT_NO_CRUMB`) | **"Home" stayed** -- not the crumb |
| scan `assets/brand/titlebar-brand.png` for content | **runs at 6..156 (wordmark), 169..234 (nav buttons), 253..320 ("Home")** |

The brand plate is a **raw 326x40 crop of the reference's title bar**, stretched over `Rect(6, 4, 326, 40)`. It therefore
carried not only the "modrinth" wordmark but the two nav buttons and the beginning of the reference's own breadcrumb -- the word
"Home" -- and drew them under everything the shell paints. The bitmap was **reproducing a neighbouring widget**, which is why
no amount of reading the Rust found a second caller: there was no second caller.

| | before | after |
|---|---|---|
| brand asset | 326x40 (wordmark + buttons + "Home") | **160x40** (wordmark) |
| title bar, Discover | "Home" under the crumb | clean (verified by capture) |
| Discover total | **not measured before** | 81 617 |

**The aggregate was not measured for this fix, and the first version of this entry said it was.** The pre-crop Discover capture was
overwritten by the post-crop one before the comparison was run, so the two images that remained were both *after* the change and
differed by zero pixels -- which is what the tool reported, correctly, for two identical builds. The visual fix is verified by the
capture; the number is not, and this entry says so rather than quoting one that was never taken.

**A bitmap is a widget that cannot be grepped.** Every grep and probe was correct and every one of them was looking at the
wrong layer; the app was drawing a picture of another app's controls.
### Auditing every embedded bitmap, because one of them lied (2026-11-07)

The brand plate's bug was a *class*: a PNG that is a screenshot of the reference rather than an asset. The seven assets were
audited the same way -- a contact sheet at native size, checked for content that is not the asset's subject:

| asset | size | verdict |
|---|---|---|
| `titlebar-brand.png` | 326x40 -> **160x40** | **carried the nav buttons and the word "Home"** -- cropped |
| `instance-default.png` | 132x132 | the campfire tile, nothing else |
| `news-art.png` | 272x120 | gradient + glyph |
| `news-joining.png` | 266x150 | gradient + glyph |
| `server-art.png` | 300x200 | **baked headline** -- and the reference's own ad art has the same headline, so it is correct |
| `content-empty.png`, `hero-plate.png`, `shots-empty.png` | -- | plates, nothing stray |

`server-art.png` is the interesting one: a doubled headline was suspected, and a 1:1 crop of the reference's hosting card showed
the same text baked into the same art. **An asset is allowed to contain type when the reference's own asset does**; the test is
not "does it look like UI" but "does the reference have it there too".

That distinction is what the brand plate failed: its wordmark sits beside the reference's *live* breadcrumb, so cropping it was
right, while cropping the hosting ad would have deleted a picture the reference draws.
### The card printed "2.1k" where the reference prints "2,115" (2026-11-07)

A 1:1 crop of Discover's bottom row showed the two counts side by side and only one of them wrong:

| | reference | before | after |
|---|---|---|---|
| followers | `2,115` | `2.1k` | **`2,141`** |
| downloads | `7.38M` | `7.20M` (the live API's own string) | unchanged |
| Discover total | -- | 81 617 | **81 267** |

**The rule was already in the codebase and the card was not using it.** `search::thousands` exists for exactly this, and its own
doc comment says a card "can show `5,023` next to `19.62M`" -- but the search hit built its follower string with `abbreviate`,
which compacts at 1 000. One field, one wrong function.

The first attempt changed `abbreviate` itself, which broke its own test (`1 000` -> `"1.0K"`) and would have made downloads
grouped too. The test was right, the function was right, and the *call* was wrong -- so the fix is `followers: thousands(...)`
and `abbreviate` untouched.

The card now prints `2,141` against the reference's `2,115`, differing in the last two digits only because the number is live and
26 followers arrived between the two captures.

The download string is the search API's own (`7.20M` against the reference's `7.38M`), not a formatting choice, and is left alone:
a live number that differs is not a parity defect, and rewriting it would be inventing data.
### An external-link glyph the reference never draws (2026-11-08)

The card's author line printed `by LUMYVERSE` followed by a 12-point arrow-out-of-a-box. The reference prints the words and
nothing else -- and `LegacyProjectCard.vue` has neither the icon nor the import for it:

| | reference | before | after |
|---|---|---|---|
| author line | `by LUMYVERSE` | `by LUMYVERSE [external]` | `by LUMYVERSE` |
| Discover total | -- | 81 267 | **81 107** |

**An affordance the reference does not draw is a promise this build cannot keep.** The glyph said a card opens an external page;
the card opens the project's page inside the launcher. It was removed rather than made to do something, because inventing a
behaviour to justify a decoration is the same error in the other direction.
### Drag-and-drop wired from the window to the disk, and driven (2026-11-08)

`drop.rs` has had `copy_into` and tests for a hundred rounds, and **nothing called it**: the window ignored winit's
`DroppedFile`, so the module was a pure function with no caller. It is now wired end to end:

```
winit DroppedFile -> Window::dropped -> Shell::dropped -> drop::copy_into(<instance>/minecraft/mods) -> toast
```

Xvfb has no file manager to drag *from*, so the same event is injected with `SIFT_DROP_PATHS` (colon-separated) and
**both branches were driven**:

| case | evidence |
|---|---|
| an instance page open | `drop-test.jar` appears in `instances/Fabric 26.3/minecraft/mods/`; toast reads **"Copied 1 file."** |
| no instance open | toast reads **"Drop files on an instance's Files tab."** |

The injected drop waits up to 30 s for an instance page (a real one is delivered at once); that hold exists only when the probe
is set, so a real drop behaves like the reference's.

**What this does not yet do.** The copy always goes to `mods/`, whichever Files tab is showing -- a drop onto the resource
packs tab should land in `resourcepacks/`. The reference routes by the visible tab. Recorded here rather than claimed: the
plumbing is what was built, and the routing is the next slice.
### A drop lands in the folder the Files panel is showing (2026-11-08)

Last round's wiring always copied into `mods/`, which is wrong for every other Files view. The page already carries the answer --
`instance::State::files_dir`, the folder the Files tab has walked into -- so the shell routes by it:

| Files panel at | drop lands in | evidence |
|---|---|---|
| the game directory (its first view) | `<instance>/minecraft/` | `find` returns **one** path, `minecraft/drop-test.jar` |
| `mods/` (any other tab open) | `<instance>/minecraft/mods/` | round 201: `mods/drop-test.jar` + toast "Copied 1 file." |

The first attempt at this measurement was **wrong twice and both times for aiming reasons**: the click went to y 100, which is the
header row rather than the tab strip (y 172..205), and the drop had already been delivered before the tab could be pressed. The
second was fixed by making the *injected* drop wait for the Files panel (`Shell::files_open`), which also models the reader who
navigates before dragging; the first by measuring the tab row off a crop instead of a band scan that was picking up the header.

A clean run -- both copies deleted first, then one drop -- leaves exactly one file, in the folder the panel is showing. The
earlier run that showed two was the leftover of the round before, which is why the clean run was worth its own minute.
### The Files panel's kebab does what its column promised (2026-11-08)

The Actions column drew a `MoreVertical` glyph per row with **no `interact` at all** -- an ornament whose column header said
"Actions" -- and `State::pending_delete`, documented since the panel was written as "the file a first press has armed for
removal", was set by nothing. It is a two-press delete now:

| press | disk | state |
|---|---|---|
| first | `mods/` still holds `aaa-test.jar` (1 file) | the row arms, the glyph turns `--color-red` |
| second | `mods/` holds **0 files** | the arm clears |

**Three defects were found by driving it, and two were mine:**

1. The clicks went to y 330 and the kebab is centred at **y 344** -- measured off the capture after two failed presses, the
   same "ask where it is" lesson as round 187.
2. The delete used `dir.join(name)` -- the *relative* path the panel is looking at (`mods`) -- and the process cannot unlink
   that from its own working directory. `folder` is the game directory joined with it; the error path is what surfaced it,
   because the arm cleared and the row stayed.
3. The failure had nowhere to go: `State` had no field for it. It has `error` now, printed in `--color-red` under the
   breadcrumb, which is why (2) was visible at all instead of silently doing nothing.

Renaming is still unbuilt and `State::renaming` is still unused; recorded, not claimed.
### The Files panel, driven against the reference for the first time (2026-11-08)

Comparing the two Files tabs at 1:1 (`ref-files.png` vs `our-files-mods.png`) put a number on the panel for the first time and
listed what it lacks:

| element | reference | ours |
|---|---|---|
| Actions glyph | horizontal `...` | **was vertical `⋮`; now horizontal** |
| Home button | yes, left of the search row | absent |
| Refresh | outlined button | absent |
| `+ ⌄` | a split button beside Refresh | absent |
| row checkboxes | one per row | absent |
| Created / Modified | real timestamps | columns drawn, values absent |
| breadcrumb | none in this view | `minecraft / mods /` shown |

Total: **68 763** (content 29 464, sidebar 36 752). The reference and this build were showing **different folders** (its `config`
against our `mods`), so that number is a floor on the structural difference rather than a like-for-like measurement -- recorded
with that caveat rather than quoted as if the states matched.

**The sidebar's 36 752 is the anomaly.** On Home it is 16 787 against the same panel; on this page the panel is more than twice
as wrong, which no list-body difference can explain. It is the next thing to measure, not the table.

The reference's `...` did not open a menu on a press at its measured position, so **what that cell does is still unknown** -- and
this build's two-press delete is therefore a design of its own, not a copy. That is recorded as a divergence rather than
claimed as parity.
### The sidebar anomaly was the news carousel (2026-11-08)

Last round recorded the Files page's sidebar at **36 752** against 16 787 on Home and called it an anomaly no list-body difference
could explain. Band analysis put every band in 348..473 at ~6 000 differing pixels -- 85% of the area, not an edge or a glyph --
and a 1:1 crop of the two sidebars showed why immediately:

| | article shown |
|---|---|
| reference | **"Joining your server just got easier"** |
| ours, default | **"Sync settings across instances"** |

Both carsels rotate -- this file has said so since the slides were added -- so the two captures caught different phases of a
moving card. Pinning ours to the reference's slide (`SIFT_NEWS_SLIDE=1`):

| | unpinned | pinned |
|---|---:|---:|
| sidebar | 36 752 | **16 787** |
| Files tab total | 68 763 | **48 798** |

**The whole anomaly was a state mismatch**, and it is the same mistake this file has recorded twice before: two clients compared
while showing different things. What is new is that the number was large enough to look like a layout bug and specific enough to
band-analyse -- which is the cheapest way to find out that a card has two articles rather than a broken one.

The Files tab's remaining 48 798 still holds the reference's `config` against our `mods`, so it is a floor rather than a
like-for-like figure until both are looking at the same folder.
### Correcting last round: three "missing" controls were a subfolder against a root (2026-11-08)

The list of what the Files panel "lacks" was wrong in three of its six rows. The controls exist -- they are drawn only when the
panel is at the game directory's **root**, and last round's capture had this build inside `mods/` while the reference was at its
root. Root against root, with the news phase pinned:

| element | reference | ours at root |
|---|---|---|
| Home button | yes | **yes** |
| Refresh | yes | **yes** |
| `+ ⌄` split button | yes | **yes** |
| row checkboxes | header + per row | **absent** |
| Created / Modified | timestamps | **empty** |
| Actions `...` | horizontal | horizontal (fixed last round) |

| | Files tab |
|---|---:|
| subfolder against root (last round) | 48 798 |
| root against root, phase pinned | **48 240** |

So the remaining real gaps at the root are **checkboxes and timestamps**, and the two numbers are close enough that the
subfolder mistake was not worth much in pixels -- but it was worth three false rows in a ledger, which is worse than a wrong
number because it sends the next round to build what already exists.

This is the **third** state mismatch this session (the Settings rail's 14 tabs, the news carousel's phase, and now the folder
depth). All three produced a plausible number and a false conclusion. The rule this file should carry into the next session is
narrower than "compare like with like": **before cataloguing a missing element, prove the two clients are on the same screen** --
and the cheapest proof is the first thing the eye checks, not the number.
### The Files table's two stamp columns, and a metric that got worse for being right (2026-11-08)

Created and Modified were drawn as headings with nothing under them. They now read the file's own metadata and print it in the
reference's format -- month/day/two-digit year, twelve-hour clock:

| | reference | ours |
|---|---|---|
| `config` / `mods` row | `11/03/26, 5:58 PM` | **`11/08/26, 5:14 PM`** |
| Modified | `11/03/26, 5:58 PM` | **`11/08/26, 6:26 PM`** |

The format is locked by a test: `11/03/26, 5:58 PM` exactly, plus the two cases a twelve-hour clock gets wrong (midnight is
`12:00 AM`, noon is `12:00 PM`). **264 tests**, one of them new.

**And the page measured worse: 48 240 -> 49 284.** The rule in this file says a change that moves the measurement the wrong way
does not stay; this one stays, and the reason has to be explicit:

- Before, the cells were **blank**; the reference's hold timestamps. Background against text disagrees over the glyph shapes.
- Now ours holds **different digits** -- a different folder with a different date. Text against text disagrees over more of them.

So the aggregate rose because the element became *correct and non-identical*, which is the one situation a pixel difference
cannot judge. The evidence that it is right is the format comparison and the test, not the number -- and the number is recorded
beside it rather than hidden. **A metric that rewards absence is not a parity metric**; that is worth knowing before the next
data-bearing element (checkboxes, file names, counts) is judged by it.
### The Files table's checkboxes, and the press that would not arrive (2026-11-08)

The reference puts a checkbox in the header and one on every row; this build had neither. Both are drawn now, at the reference's
own geometry read off a 3x crop -- **20x20 at x 100**, the folder icon 33 after it, the header label aligned with the icons at 133
rather than with the names at 173 -- and pressing one ticks it:

| | evidence |
|---|---|
| row tick | 182 green pixels in the box after one press; the row does **not** open |
| header tick | selects every row when any is unticked, clears them when all are |
| Files at root | 49 253 |

**Three attempts were needed, and the first two failed the same way the parked rename control had:**

1. The checkbox was registered *before* the row, and the row -- which spans the whole line -- won the press: the click opened
   the folder instead of ticking it.
2. Registered *after* the row, it still lost. `grep` confirms the call order (row at 1466, tick at 1474), so egui was resolving
   overlapping rects in a way that does not match "last one wins" as this file assumed.
3. **The row's rect now starts after the checkbox.** Two interactive rects over one press cannot be made to cooperate by
   ordering; giving the press to exactly one widget fixed it in a line.

That is the same lesson the header recorded when the rename control was parked as impossible: the problem was never egui, it was
two rects claiming one press. **Do not order overlapping targets -- separate them.**
### Renaming a file, and the click that needed a frame between its halves (2026-11-08)

`rename_entry` has carried its rules and four tests since it was written and **no caller** -- its own comment says the control
"never arrived". Narrowing the row's press rect (last round) gave it somewhere to live:

| | result | disk |
|---|---|---|
| double press the name | an inline field opens over the row, focused | unchanged |
| type `renamed.jar`, Enter | the row redraws with the new name | **`renamed.jar`** (was `old-name.jar`) |
| Escape | the field closes | unchanged |
| a bad name (`..`, a path, an existing name) | the rule refuses it and the reason lands in the panel's error line | unchanged |

**The one thing that stopped it working was the double press itself.** Two `xdotool` clicks 80 ms apart never opened the field;
the same two clicks 150 ms apart did. This build repaints on demand, so a pair of clicks can arrive inside one input batch, and
egui counts a double press across *frames*. **A gesture on a renderer that draws on demand has to allow it to draw.** That is a
harness fact worth as much as the feature: the same will apply to double-click-to-open and to drag thresholds.

The trigger is **this build's choice, not parity**: the reference routes renaming through its table component, whose trigger is
not in this tree (the ui package ships only as source-less output). Double press is recorded as a decision to revisit if the
reference is ever seen doing something else.
### Auditing the matrix for DONE rows with nothing behind them (2026-11-08)

The objective asks for "a capture behind every done", so the ledger was audited against its own definition. **25 of 115 DONE rows
carried no drive, test or measurement** -- only a `file:line` or the word "none" -- which this file defines as VERIFIED, not DONE:

| | before | after |
|---|---:|---:|
| element rows | 679 | **679** |
| DONE | 115 | **90** |
| VERIFIED | 21 | **46** |

Each moved row now carries "code read only; no drive recorded (audit 2026-11-08)" in its gap cell, so the claim it makes is the
claim its evidence supports. **Nothing was deleted and no row was rewritten**: the same 679 claims are there, 25 of them told
more accurately.

**Two mechanical lessons, both from the first attempt at this edit:**

1. The matrix's element column contains escaped pipes (`Shell \| sidebar`), so splitting a row on `|` yields more cells than
   there are columns and every index after the first escaped pipe is wrong. The first pass wrote 24 malformed rows; the file was
   restored from git and redone with an escape-aware split.
2. Verifying the second pass with `awk -F'|'` reported 11 fields for a 9-column row -- **also** because of the escapes. The
   check had to be done with the same parser that wrote the file, which is the general form of a rule this session keeps
   meeting: an instrument that does not understand the data will happily confirm anything.
### Four VERIFIED rows driven to DONE, and a correction to last round's row count (2026-11-08)

The audit left 46 rows that were read but never driven. Four were driven this round, and one of them -- the breadcrumb -- had
never been pressed at all:

| row | what was driven | result |
|---|---|---|
| 532 | the card's kebab pressed twice | Play/Edit/Duplicate instance/Open folder opens; a second press closes it (**9 092 -> 922** pixels in the panel's area) |
| 544 | a file dropped with the Files root showing, then with `mods/` showing | `minecraft/drop-test.jar`, then `minecraft/mods/drop-test.jar` |
| 557 | the `mods` row pressed inside the Files tab | it opens: `minecraft / mods /` and the file it holds |
| 558 | **the `minecraft` crumb pressed from inside `mods/`** | returns to the root, which draws the Home button and the `mods` row |

| | before | after |
|---|---:|---:|
| DONE | 90 | **94** |
| VERIFIED | 46 | **42** |

**And a correction.** Last round recorded "679 element rows". Counting the status cell showed 16 of those rows are table headers
and separators -- `status`, `---` -- which have nine cells because of the legend at the top of the file. The element rows are
**663**, and every count in that entry should read 16 lower. The four numbers that matter are unaffected: DONE, VERIFIED, OPEN and
BLOCKED are all counted from rows whose status is one of those words.

The lesson is the same one the escaped pipes taught, one step further in: **a parser that counts rows is not a parser that
understands them.** The header check was in the data all along and only appeared when a status tally was asked for by name.
### The Screenshots page, measured for the first time in this state (2026-11-08)

Both clients were empty in this lab, which makes the comparison clean for once:

| | value |
|---|---:|
| Screenshots, total | **23 547** |
| of which the content pane | **4 224** |
| sidebar | 16 787 (the same panel as every other page) |

The two empty states are the same three things -- the illustration, "No screenshots yet", "Screenshots you take in-game will
appear here." -- and a 1:1 crop of the block shows one difference worth naming: **this build's illustration is slightly larger**.
The block's ink starts at y 401 here against 387 in the reference and they end together at 601, which is what a larger picture
above a fixed footer looks like.

Rows 283 (the page root) is driven and now DONE. **Rows 286 (the toolbar's second row), 290 (group collapse), 296-303 (select
mode, its toolbox, the card overlay, the context menu, delete, bulk delete) and 586 (show in folder) all need screenshots that do
not exist in this lab** -- the empty state is the only state the reference and this build can currently share. They stay VERIFIED,
which is the honest status: the code is read, the behaviour has never been seen on either side.
### A populated Screenshots page: this build draws it, the reference will not be shown it (2026-11-08)

Ten matrix rows (toolbar, group collapse, select mode, its toolbox, the card overlay, the context menu, delete, bulk delete, "show
in folder") need a Screenshots page with something on it, and this lab has none. The files are just files, so the same PNG was
placed in both clients' instance folders:

| path tried | this build | reference |
|---|---|---|
| `<instance>/screenshots/2026-11-08_18.00.00.png` | **populates**: heading, Search, New group, Newest/Date selects, "Today 1" and the image | still the empty state |
| `<instance>/minecraft/screenshots/2026-11-08_18.00.00.png` | -- | still the empty state |

So this build reads the folder the reference's own source gave it, and **the reference does not scan either candidate path** -- its
list is probably built by an index written when the game takes a screenshot, not by walking the directory. That is a state this lab
cannot put the reference into without running Minecraft.

**What that means for the ledger:** those ten rows stay VERIFIED, and their note should not be "needs a capture" but "the reference
cannot be shown this state here". They are a **missing-state** block, like Servers' service gate, not unfinished work -- and the
distinction matters, because the first invites a capture attempt every few rounds and the second says what would have to change.

The measurement of the empty state stands: **23 547** total, **4 224** in the content pane, with this build's illustration the
larger of the two.
### Two numbers that measured nothing: a dead client still has a window (2026-11-08)

The Project page was measured twice this round and both numbers were meaningless:

| capture | total | what it actually was |
|---|---:|---|
| first | 212 131 | our client dead; the X root still held its last frame, on the **Screenshots** page |
| second | 411 113 | our client restarted at a different position; the frame was offset inside the 1440x900 window |

The first was found by looking at the image: the reference showed "Fabulously Optimized" and this build showed a Screenshots page
whose file had been deleted a round earlier. The second by noticing that the *title bar* alone scored 69 120 -- a number no
chrome can reach unless it is entirely misplaced. `pgrep` then showed **every client process defunct**:

```
67546 [SiftUi] <defunct>   101560 [ModrinthApp] <defunct>
```

An X window outlives a killed client until the server destroys it, and `import -window root` reads whatever the server holds. So a
capture of a dead app is not empty and not an error -- it is a **photograph of the last frame**, which compares and scores like any
other. Both clients were restarted and verified alive by PID before anything else was measured.

**What this file should carry from it**: three separate instruments in this session have now returned confident nonsense -- the
stale news phase, the subfolder comparison, and this -- and the common shape is that each was a *state* problem invisible in the
number. The cheapest guard is the one that costs a second: **before comparing, confirm both clients are running and both windows are
at 0,0 1440x900.** `pgrep` and `getwindowgeometry` say so in one command; a difference map cannot say it at all.
### This build does not follow an external resize (2026-11-08)

Four captures on the Project page scored 411 113 and the reason was not the page:

**The window was 1440x900 and the app was drawing into a 1280x800 surface in its top-left corner.** A shrink to 1200x760 and a
grow back to 1440x900 changed nothing at all -- the two captures are byte-identical -- so the client never follows a resize it did
not initiate.

The code says why. `crates/sift-ui/src/soft/window.rs:253`:

```rust
WindowEvent::Resized(_) => self.dirty = true,
```

The new size is **discarded**. The surface and egui's screen rect are fixed at launch, so every capture taken after a external
resize is of a correctly-drawn UI in the wrong rectangle -- which compares, and scores, as if the whole page were wrong.

**This is a product defect, not only a harness one.** A reader who resizes the window gets a launcher that keeps drawing at its old
size, and it has been that way for as long as this file records -- it was invisible because the harness normally starts the client
at the size it wants and never resizes it.

The fix is mechanical: keep the new size, resize the softbuffer surface and the egui screen rect with it. It was not made this
round, which had already spent itself establishing that the number was not about the Project page; the record is precise enough to
do it first thing next.

**Two process guards came out of the same hour:** two windows with the same name can exist (one from a dead client) and the older
one can sit on top -- `xdotool search --name` returning two ids is now a stop sign; and a second instance of the *reference*
refuses to start while the first holds its data directory, showing an error dialog rather than a window.
### CORRECTION: the resize "defect" is not a defect (2026-11-08)

Last round this file recorded that `WindowEvent::Resized(_) => self.dirty = true` discards the new size and that the app draws
forever at its launch size. **That is wrong, and the record above it should be read with this entry beside it.**

The evidence that settles it:

| check | result |
|---|---|
| does the event arrive? | `SIFT_RESIZE_PROBE` printed **both**: `1200x760`, then `1440x900` |
| does the app follow it? | after the cycle the capture fills all 1440x900 -- the pixel at (1439,899) is content, not the black band |
| why is the `Resized` arm so thin? | it does not need to do more: `Soft::frame` re-reads `window.inner_size()` **every frame**, and the surface and `screen_rect` are built from it |

The black bands in the four bad captures came from a **second window of the same name** left by a dead client and sitting on top --
the guard this file added in the same hour found it (`xdotool search` returned two ids). The app was never drawing small; the screen
was showing an older window's frame.

**A product defect was recorded from a harness artefact**, and the only thing that caught it was testing the mechanism rather than
reading the line: an arm that "discards" a value is not discarding anything if the value is re-read per frame. The lesson joins the
others in this file: **a line of code is evidence about the code, not about the product.**

The `SIFT_RESIZE_PROBE` stays -- it is env-gated, it prints on every resize, and it is what turned this from an inference into a
fact.
### The project page renders into 1280x800 of a 1440x900 window (2026-11-08)

Reproducible, and the receipt is a byte comparison: a **fresh instance draws Home correctly** (`fresh-home.png`), then two presses --
Discover, then the first card -- and the next capture is **byte-identical to a capture taken forty minutes earlier**
(`ead9a382...`), an inset project page. The screen\'s content occupies exactly **1280x800 in the top-left of a 1440x900 window**.

| check | value |
|---|---|
| window geometry | 1440x900 at 0,0 |
| screen | `1440x900 pixels` |
| rendered area | 1280x800 |
| ratio | **0.8889 = 1 / 1.125** |

A ratio of exactly 1/1.125 is a **scale factor**, not a layout mistake: 1.125 is a common X11 DPI scale (96 -> 108 dpi). So the
likeliest cause is that `self.scale` -- which divides the window size to produce egui\'s logical `screen_rect` -- changes when the
project page opens, while the painter still lays out at logical coordinates 1:1 with physical pixels. The UI then believes the
window is 1280x800 and draws into that corner of a larger buffer.

**Not fixed this round.** The round spent itself on the harness: three separate false signals (a dead client, a duplicate window, a
stale frame) were each eliminated by a measurement before this one remained, and the remaining one is a real rendering bug that
deserves a round where it is the first thing touched.

The `SIFT_RESIZE_PROBE` added this round prints `resized to ...` on every resize and is what proved the app *receives* and *acts on*
those events, which is why the earlier "the app does not follow a resize" entry was wrong. Both entries stand, in order, with this
one last -- the file is a log, not a claim.
### A hypothesis, tested and disproved: the inset is not the two-scale mismatch (2026-11-08)

`Soft::frame` built egui's `screen_rect` from `width / self.scale` while the tessellator used egui's own `pixels_per_point`. When
those disagree by 1.125 the UI lays out at 1280x800 and is painted into the top-left of a 1440x900 buffer -- which is what the
project page was showing, and 1.125 is a real X11 DPI step, so the explanation fitted.

It was tried: `screen_rect` built from `ctx.pixels_per_point()`, one scale instead of two.

| | Project page total |
|---|---:|
| before | 411 113 |
| after the one-scale change | **411 113** |

**Unchanged to the pixel, so the hypothesis is wrong and the change is reverted.** The fit was good -- 1/1.125, a plausible DPI
value, a code path that could produce it -- and a good fit is not evidence. What survives from the attempt is the reproduction: a
fresh instance draws Home correctly and the *first navigation to a project* produces a byte-identical inset frame every time.

That reproduction is the thing to hand the next round: **it is deterministic**, it distinguishes a fresh instance from a navigated
one, and it does not depend on resizes, dead clients or duplicate windows -- all three of which were eliminated before the
reproduction was trusted.
### The inset is in the window, and the app does not know about it (2026-11-08)

The scale hypothesis was wrong. The probe settles what the app believes:

```
frame inner=PhysicalSize { width: 1440, height: 900 } scale=1 ppp=1     (119 frames, every one identical)
```

So egui lays out at **1440x900 logical with a 1:1 pixel ratio** -- there is no 1.125 anywhere in the app's view of the world. Yet:

| measured | value |
|---|---|
| window geometry | 1440x900 |
| `import -window <id>` size | 1440x900 |
| content inside it | 1280x800, top-left |
| the remaining 160 columns and 100 rows | **(0, 0, 0) -- pure black**, not the background colour |

Pure black is the decisive detail. The rasteriser clears **every pixel** of every band with `self.clear` before painting, so a
region it had painted would be the window background, never black. A black region was never written at all -- which puts the fault
in **presentation**, not in layout: the app paints a 1440x900 frame and something transfers only 1280x800 of it.

1280x800 is exactly the size winit creates the window at on X11 with no window manager. So the shape of the fault is: the surface
was created for the launch size, `surface.resize(1440, 900)` is called every frame, and the presented region stays at the size the
surface was born with.

**Why Home looked fine and the project page does not is still open** -- both are the same window, same surface, same resize call.
That is the next measurement, and it is now a narrow one: instrument the surface's reported size and the present call's effect, on
the frame where the navigation happens.

`SIFT_SCALE_PROBE` stays: it is what turned three guesses into one fact.
### Three mechanisms eliminated; the inset is none of them (2026-11-08)

The deterministic reproduction was used to test each candidate mechanism in turn, and every one came back negative:

| hypothesis | instrument | result |
|---|---|---|
| the two scales disagree | `SIFT_SCALE_PROBE` | **all 119 frames**: `inner=1440x900 scale=1 ppp=1` -- no 1.125 anywhere |
| `surface.resize` fails silently | probe the `Result` | **105 of 105**: `resize ok 1440x900` |
| the buffer is the old size | print `buffer.len()` | **1 296 000 = 1440x900** -- the full buffer, cleared and painted |
| `present()` presents the birth size | rebuild the surface on any size change | **unchanged**: still inset after a rebuild |

So: the app lays out full, the surface accepts the new size, the buffer is the new size, the rasteriser clears and paints all of it,
and the window still shows content in 1280x800 with **black** beyond. Each of the four was a plausible, code-visible mechanism and
each was disproved by one line of instrumentation.

The rebuild was reverted (it moved nothing). The probes stayed: `SIFT_SCALE_PROBE` now prints the size, the scale, the pixel ratio,
the resize result and the buffer length every frame, which is the whole chain from `inner_size` to `present` in one stream. **The
next step is to read that stream on the frame where the navigation happens** rather than to test a fifth guess: something differs
between the frame that draws Home correctly and the first frame of the project page, and the probe is already positioned to show it.
### The project page paints its content and nothing else -- investigation paused (2026-11-08)

One more measurement, and it changes the shape of the problem. Capturing our window directly (never resized, so no resize is
involved) on the project page:

| measured | value |
|---|---|
| window | 1440x900 at 0,0 |
| content | the project page **only** -- header row, tabs, body |
| missing | the title bar, the rail, the sidebar |
| those regions | **pure black (0,0,0)** |

So it is not "the UI is drawn small". The shell's chrome is **not painted at all** on that frame, and the page body is drawn at its
normal coordinates over black. That is a different fault from the one four rounds were spent on, and it contradicts the
instrumentation: the buffer is 1440x900, cleared to the window colour in every band, painted, and presented -- and the result has
black exactly where the chrome should be.

Two things were found while checking that contradiction, and both belong in the record:

1. **There are two windows on `:100`**: `2097154 "Sift Launcher"` and `4194306 "floor"`, both 1440x900 at 0,0. `floor` is not this
   build -- it is a leftover from an earlier session -- and a second full-screen window on a bare X server is exactly the kind of
   thing that makes a screen capture show one client over another.
2. The app already opens at **1440x900** (`geometry::WINDOW`), so the harness's `xdotool windowsize` calls were never needed and
   have been dropped.

**The investigation is paused, not closed.** The next step is to capture with `floor` destroyed and, if the frame is still
chrome-less, to instrument `Shell::ui` to print whether the title bar and rail are being drawn on that frame. Everything else in this
build renders correctly under the same capture path, so the scope is one page.
### QUARANTINED: the Project page renders offset and chrome-less (2026-11-08)

Eight rounds went into this and it is now closed for the session, with everything established written down so it is not re-derived.

**What is certain:**

| fact | how it is known |
|---|---|
| the app is alive and responsive on that page | clicking Home returns a correct full frame (a different file, byte for byte) |
| every other page renders correctly under the same capture path | Home, Discover, Instance, Settings, Screenshots all measured |
| the frame contains the project content at its normal coordinates | header, tabs and body are drawn where they belong |
| the title bar, rail and sidebar are **absent**, not dark | the regions are (0,0,0), and this build's background is not black |
| it is **not** a scale mismatch | `inner=1440x900 scale=1 ppp=1` on all 119 probed frames |
| it is **not** `surface.resize` | `resize ok 1440x900`, 105 of 105 |
| it is **not** the buffer | `buffer.len() = 1 296 000 = 1440x900`, cleared and painted |
| it is **not** a stale or duplicate window | one window on the display; destroying the stray `floor` window changed nothing |
| it is **not** the gallery scrim | Escape leaves the page entirely and lands on a correct Home |
| it is **not** a zoom factor | `zoom_factor` appears nowhere in the crate |

**What that leaves** is a page whose draw call is reached and whose chrome is skipped, inside a frame that the rasteriser fills
correctly. The next step is not a guess: instrument `Shell::ui` to print, per frame, whether `title_bar` and the rail were called, and
compare that line on Home against the same line on the project page. That is half an hour of work and it was not done this session
because seven mechanisms had to be eliminated first.

**The cost is one screen.** The Project page cannot be measured or driven until this is fixed; every other page can, and the next
rounds go back to them rather than to this.
### Contradiction on the Project page: everything succeeds and the screen disagrees (2026-11-08)

The half hour the quarantine asked for was spent, and it produced a contradiction rather than a cause:

| probe | result, 102 frames across the navigation |
|---|---|
| `ui rect` | `[[0,0] - [1440,900]]` on **103 of 103** frames |
| background colour | `#16181C` -- the theme\'s BG, not black |
| `title_bar` called | **103 of 103** |
| `surface.resize` | `ok 1440x900`, 105 of 105 |
| `buffer.len()` | `1 296 000 = 1440x900` |
| `buffer.present()` | **`ok 1440x900`, 102 of 102** |
| window vs root capture | **byte-identical** -- the screen really does show that frame |

Every step of the pipeline reports success, the chrome is drawn with the right rect and a dark background, and the screen shows a
frame with the chrome missing and the content inset. **A capture that agrees with the screen and disagrees with the app is not a
capture problem**; something between `present()` returning `Ok` and the window\'s pixels is not doing what its result says.

**The next step is to stop the app from being the suspect at all**: restart `Xvfb :100` from scratch, run one instance, and see
whether the inset survives. If it does, the fault is in this build\'s present path and the server is innocent; if it does not, the
display accumulated state that no amount of client restarting clears -- and every capture on :100 since this began has been suspect.

The probes stay (`SIFT_SCALE_PROBE` now covers resize, buffer length and present; `SIFT_CHROME_PROBE` covers the ui rect, the
background and the title bar). They are the reason this is a contradiction with numbers rather than a suspicion.
### The fault is in drawing, not presenting -- and a fresh X server proves it is deterministic (2026-11-08)

Two results this round, and the first is the one that matters:

**1. A fresh display reproduces it byte for byte.** `Xvfb :100` was killed, its stale socket removed and a new server started; one
instance ran, navigated, and captured `ead9a382...` -- **identical** to the capture taken before the server was replaced. The inset is
a function of the app, not of accumulated display state. Every "the display is lying" hypothesis is now dead.

**2. The app's own framebuffer is black where the chrome should be.** `SIFT_DUMP_FRAME` writes the buffer to a PPM *after painting and
before `present()`*. Its first bytes are `\0\0\0` over a long run, so the pixels the window shows as black are black **in the buffer
the app painted** -- the present path is innocent, which is the opposite of where the last round left the search.

That contradicts two things this file already recorded, and both are worth stating plainly:

| recorded | now measured |
|---|---|
| the painter clears every pixel to the theme BG (`#16181C`) | the buffer\'s corner is `(0,0,0)` |
| `title_bar` is called on every frame, with `ui rect` 1440x900 | its pixels are not in the buffer |

So either the clear and the background rect are not reaching this buffer, or they are reaching a different one. The next measurement
is one line: print `buffer.len()` **and the value of pixel 0 after the clear and before painting**, on the same frame that is dumped.
If pixel 0 is the BG before painting and black after, something in the primitive loop writes black over it; if it is black before
painting, the clear is not touching this buffer at all.

**A caveat on my own instrument**: the PPM writer stores `(pixel & 0xff), (pixel >> 8), (pixel >> 16)`, which is `B,G,R` written
into an `R,G,B` file. Black is black either way, which is why the finding above stands, but no colour judgement should be made from
that dump until the order is fixed.
### The corner is blacked by primitive 5, a full-window rounded rect (2026-11-08)

Two probes, one frame apart, and the search is over a single draw call:

```
after clear:  p0=0x0016181c clear=0x0016181c     <- the theme BG, as it should be
after paint:  p0=0x00000000 len=1296000          <- painted over, opaquely black
primitive 5 left p0=0 clip=[[0,0] - [1440,900]] verts=88
```

So the rasteriser clears correctly, and **primitive 5 -- a rounded rectangle covering the entire window -- paints the corner opaque
black**. Everything else on the page is painted after it, which is why the project content is bright and the chrome, the rail and the
sidebar are not: they are under it.

The vertex count is the clue. 88 vertices for one rect is what egui's tessellator produces for a **rounded** rect, and a plain
`rect_filled(window, 0.0, ...)` is four (primitive 6 is exactly that). So it is a full-window rect with a corner radius and an
opaque black fill -- and the one candidate this file has already found is the gallery viewer's scrim:

```rust
painter.rect_filled(page, 0, Color32::from_black_alpha(216));   // project.rs:2290
```

Two things about it are wrong for what we see, and both are worth testing next: `from_black_alpha` should be **blended**, not
opaque (our painter may not blend mesh vertex colours at all, which would make a 216-alpha scrim paint solid black), and `page`
should be the content column, not the whole window.

**Next measurement**: print primitive 5's first vertex colour and its `feather`/radius, and print whether `state.viewer` is `Some`
on that frame. Both are one `eprintln` in code that already exists, and one of them names the culprit outright -- the scrim's colour
or the viewer\'s state.
### Primitive 4: a 5 729-vertex text mesh whose vertex colour is fully transparent (2026-11-08)

The primitive that blacks the corner is **index 4**, and its first vertex says the least likely thing:

```
prim4 before=0x0027292e colour=#00_00_00_00 tex=Managed(0) at=[15.5 388.5] verts=5729
prim5 before=0x00000000 ...                                    <- black already, so primitive 4 did it
```

`Managed(0)` is the **font atlas**, so this is text. Its colour is `#00_00_00_00` -- transparent black -- and it is 5 729 vertices,
which is a page\'s worth of body copy rather than a label.

**And the blender cannot paint transparent black.** `soft/mod.rs` implements premultiplied source-over and returns early on
`alpha <= 0.0`; a mesh whose every vertex were transparent would leave the pixel untouched. So the black is coming from
**interpolated** vertex alpha -- some vertices of that mesh are opaque -- or from a vertex position far from the glyphs it belongs
to. Either way the fault is now inside one mesh, on one page, with a colour and a vertex count to identify it by.

**What this rules out, finally:** the chrome is drawn (its primitives are 0-3, all present with sensible colours), the clear works,
the blend is correct for zero alpha, the surface and buffer are the right size, and the present succeeds. The black is written by a
**text mesh**, which is why every measurement of the background, the theme and the window was correct while the corner was black.

**Next**: print that mesh\'s vertex-colour range (min/max alpha) and its position bounds. If alpha spans 0..255 it is a fade that
should not be painting; if every vertex is (0,0,0,0) the fault is in how `src` is accumulated across a triangle rather than in the
blend.

This round produced no fix and no page number. It produced the identity of a culprit after eleven rounds of elimination -- and
`SIFT_PIXEL_PROBE` now names the primitive and its colour, which is the tool that will finish it.
### FIXED: one triangle with infinite corners was blacking the Project page (2026-11-08)

Thirteen rounds after it started, the cause is known and the page draws:

| probe | value |
|---|---|
| primitive 4 | a 5 729-vertex mesh, `Managed(0)` (the font atlas), alpha spanning 0..255 |
| its bounds | **`(-inf,-inf) - (inf,inf)`** |
| non-finite vertices | **4** |

Four vertices with infinite coordinates form a triangle that has no area and no position. The edge tests used to decide whether a
pixel is inside a triangle pass for **every** pixel when the corners are infinite, so the fill wrote that triangle\'s colour across the
whole buffer -- which is why the chrome, the rail and the sidebar were black while the page content, drawn afterwards, was fine.

**The fix is in the rasteriser, where it belongs:**

```rust
if !a.pos.x.is_finite() || !a.pos.y.is_finite()
    || !b.pos.x.is_finite() || !b.pos.y.is_finite()
    || !c.pos.x.is_finite() || !c.pos.y.is_finite() { continue; }
```

A triangle without finite corners has nothing to contribute, and a renderer that fills it is trusting geometry it has already
proved impossible.

| | before | after |
|---|---|---|
| the corner pixel | `(0,0,0)` | **`(22,24,28)`** -- the theme BG |
| title bar | black | **`(39,41,46)`** -- its own fill |
| the page | content only, on black | **chrome, rail, sidebar and content** |
| Project page | unmeasurable | **222 886** (titlebar 4 011, rail 640, content 180 072, sidebar 38 163) |

**The source of the infinity is not fixed.** Four vertices of one mesh are still infinite; the renderer now refuses to draw them. The
next step is to find which layout produced them -- the mesh is a tooltip-shaped fade at the rail, and the likely path is a pill rect
built from a label size that came out infinite. A guard at the renderer stops the symptom everywhere; the cause is still owed.

The 222 886 is the first Project-page number this file has ever recorded and it is **not yet like-for-like** -- the two captures were
taken minutes apart and the reference may not have loaded the same artwork. It is a floor, and it is written down as one.
### The Project page is measurable, and its difference is the body image (2026-11-08)

With the finite-triangle guard in place, both clients were driven to the same project from Home in one script and captured:

| | value |
|---|---:|
| Project page, like-for-like | **222 886** |
| title bar | 4 011 |
| rail | 640 |
| content | **180 072** |
| sidebar | 38 163 |

It reproduced **exactly** across two independent capture pairs, so the number is stable and the page is now measurable -- which is
the first time that has been true.

A 1:1 comparison names the dominant difference: **the reference draws the project body\'s picture at roughly 400x300 and this build
draws it at about 100x75**, in the same place, with the text laid out around it. That single element is most of the 180 072.

The sizing code scales a picture **down** to `inner.width()` but never up, so a picture that arrives smaller than its natural size
stays small. Whether that is our texture being fetched at a reduced size or the layout measuring before the texture lands is the next
question, and it is a narrow one: one `Block::Image` draw, one rect, both sizes known.

**Also owed**: the four infinite vertices are still in that mesh -- the renderer now refuses to draw them, which is correct and
universal, but the layout that produced them is unfound. It is invisible in every capture, so it is now a robustness debt rather
than a symptom.
### Pictures are not icons: the project body was a thumbnail (2026-11-08)

The 22nd image in this file was capped at 160 points because **icons** cost 38 MB of RSS when they were uploaded whole. That cap was
applied to every image, so the Project page\'s body picture -- four hundred points wide in the reference -- was drawn at about a
hundred. One constant, two kinds of picture, one of them a quarter of its size.

| | before | after |
|---|---:|---:|
| Project page | 222 886 | **90 727** |
| its content | 180 072 | **47 913** |

**132 159 pixels from one number.** The change: `fetch` takes the cap as a parameter, `MAX_ICON` stays 160 for icons, `MAX_PICTURE` is
1024 for the body and the gallery, and `get_picture` files under `url#1024` so an icon and a picture of the same URL are two textures
rather than one shared compromise.

**A defect the first attempt introduced, and the second fixed**: the fetch replied under the bare URL while the lookup asked for
`url#1024`, so the picture vanished entirely and the page measured *worse* (241 162). The reply has to ride back under the same key it
is filed under -- a two-line rule that cost a build to learn because the failure was silent: `get_picture` simply returned `None`
forever, exactly as it would while loading.

**And a caveat this change carries**: 1024-point pictures are the largest textures this build has ever held, and the idle-RSS gate is
already missed. The cache keeps pictures and icons in one pool with a byte budget, so the next measurement should be RSS on this page
with a picture loaded, against the 22.6 MB baseline. A picture that costs the memory clause would be the wrong trade, and it is worth
knowing which side of it this lands on.
### The picture cap is not what costs the memory (2026-11-08)

The caveat written last round was measured, and it is not the picture:

| configuration | Project page | RSS on the page | RSS after returning Home |
|---|---:|---:|---:|
| `MAX_PICTURE = 1024` | 90 727 | 31 508 kB | **31 556 kB** |
| `MAX_PICTURE = 640` | **90 727** | 31 724 kB | -- |

Two results, both useful:

1. **640 draws the page exactly as well as 1024** -- the number is identical to the pixel, which is what a cap above the reference\'s
   470-point picture should do. The cap is 640 now, with the measurement in its comment.
2. **The memory is the same either way**, so it is not the texture. Visiting the Project page costs about **9 MB that never comes back**
   (30.8 MB idle against the 22.6 MB baseline), and `trim_heap()` already runs at the end of every `pump()`.

So the growth is elsewhere: the long body text\'s galleys, the body JSON, or egui\'s own caches -- all of which are per-page state that
nothing evicts when the page closes. That is a **new, named memory question** with a clean experiment: visit the page, leave it, and
watch which of anon/file/shmem holds the 9 MB (Pss_Anon went 5 800 -> 13 080, so it is anonymous and it is ours).

**The memory clause is still missed and now measured from a second direction**: 22.6 MB idle was already above the 20 MB gate, and a
page visit adds 9 MB that outlives the page. Both belong to the same fix -- evicting what a page built when the page goes -- and neither
is solved by making pictures smaller.
### On the Project page the sidebar stops before the hosting ad (2026-11-08)

Band analysis of the Project page\'s 90 727 puts the worst bands at the **bottom**, and a 1:1 crop says why:

| | sidebar at y 820..900 |
|---|---|
| reference | **the hosting ad\'s art** -- the Minecraft characters on their green floor |
| ours | plain background |

The panel draws the banner (it is there on Home), and on this page it draws `Getting started` and `Compatibility` and then stops. The
page passes a `locked` game version, so the sidebar takes a different stack -- and the banner is not at the end of it.

**Bands 840 and 865 are 9 700 of the 90 727 between them**, which is most of a seventh of the page in a single missing card. The fix is
to draw the banner after whichever stack the page asked for; the measurement to hold it to is those two bands.

Recorded rather than fixed: the round had already measured the picture cap against memory and eliminated it as the cause of the 9 MB,
and a layout change to the sidebar deserves a round where it is the first thing touched.
### The Project page\'s sidebar has no banner at all -- correcting my own first reading (2026-11-08)

The first crop of the bottom band suggested the banner was there but missing its illustration. A 1:1 crop of the whole foot says
otherwise:

| sidebar x 1130..1440, y 640..900 | reference | ours |
|---|---|---|
| "modrinth hosting" mark and wordmark | drawn | **absent** |
| "Get your next server on Modrinth." | drawn | **absent** |
| the character art | drawn | **absent** |
| the dark green ground | drawn | **absent** -- plain background |

So the banner is not drawn on this page at all, ground included -- and `panel::draw` draws it unconditionally at
`rect.bottom() - 250`. Either the panel is called with a rect whose bottom is above the visible foot, or the project page does not go
through `panel::draw` and the `Compatibility` block it shows comes from somewhere else.

**The distinction matters and it cost one crop to get right**: "the banner is drawn but its picture is missing" and "the banner is not
drawn" are different bugs in different files, and the first reading would have sent the next round to the asset loader instead of the
layout.

That is the third time this session a conclusion has needed a second look at a bigger crop -- the brand plate, the checkbox, and now
this. The rule holds: **crop wider than the thing you are asking about, because the thing you are asking about may not be there.**
### Why the Project page has no banner: `shared = Rect::NOTHING` (2026-11-08)

The cause is one line in `shell.rs`\'s `right_panel`:

```rust
if let Some(project) = self.open_project.clone() {
    let used = crate::pages::project::panel(ui, block, &self.theme, &project);
    // "the reference hides the default sidebar on a project page"
    shared = Rect::NOTHING;          // <- and with it, the hosting banner
}
```

The reference does hide its **default stack** on a project page -- `Getting started` and `News` are replaced by the project\'s own
column -- but **the banner is not part of that stack**. Its project page shows the project column *and* the hosting banner at the foot,
which the capture confirms. Setting `shared` to `NOTHING` suppressed both.

The fix is to draw the banner from both paths, which means lifting it out of `panel::draw` into its own function. The block is about a
hundred lines and needs four things from its caller -- `ui`, `rect`, `theme` and the `banner_top` it derives -- so it is a clean
extraction, and it is not one to start with ten minutes left in a round.

**Recorded with the plan, not just the symptom**: extract `panel::hosting_banner(ui, rect, theme)`, call it at the end of
`panel::draw`, and call it from `right_panel`\'s project branch after `project::panel`. The measurement that judges it is bands 840 and
865 -- 9 700 pixels between them -- against the Project page\'s current 90 727.
### The banner drawn from both paths: Project page 90 727 -> 72 350 (2026-11-08)

The extraction landed. `panel::hosting_banner(ui, rect, theme)` is the banner on its own -- gradient, art, mark, wordmark, heading
and accent line -- and both callers use it:

| caller | before | after |
|---|---|---|
| `panel::draw` (every page but the project) | inline, at the end | `hosting_banner(ui, rect, theme)` |
| `shell::right_panel`\'s project branch | **nothing** | **`hosting_banner(ui, panel, &self.theme)`** |

| | before | after |
|---|---:|---:|
| Project page | 90 727 | **72 350** |
| its sidebar | 38 163 | **19 786** |

**18 377 pixels**, where the two worst bands had predicted 9 700 -- the banner brought its CTA and link with it, which the bands did
not separate.

**Three failures on the way, all mechanical and all mine**: the marker surgery swallowed `draw()`\'s closing brace; the moved segment
carried `draw()`\'s tail expression (`pressed`) into a function that had no such variable; and the seam replacement that should have
fixed the first failed on an exact-match anchor. Each was one build. The lesson is the one this file keeps meeting from the other side:
**a hundred-line move in a 1 300-line file is surgery, and surgery wants the file open, not a script.**

Home was re-measured after the change to be sure the extraction did not disturb the path that already worked.
### The header showed five tags where the reference shows three (2026-11-08)

A 1:1 crop of the project header named three differences, and the largest was a whole category of element:

| | reference | before | after |
|---|---|---|---|
| header chips | **Lightweight, Multiplayer, Optimization** | those three **plus Client and Fabric** | the three |
| Install button | 92 wide | 107 | unchanged (next) |
| subtitle wrap | after "Wilderness" | after "package." | unchanged (a measure) |

| | before | after |
|---|---:|---:|
| Project page | 72 350 | **71 380** |

The reference puts loaders and environments in the sidebar\'s `Platforms` and `Supported environments` rows -- which this build
already has, and which is why Fabric and Client appeared twice in two places. The header is for categories, so it now filters the
six names that are not: fabric, forge, neoforge, quilt, client, server.

**The filter is a name list, and that is a weakness worth stating**: a new loader or environment name would reappear in the header
until it is added here. The right fix is to carry the hit\'s `display_categories` separately from its tags, which the API provides and
this build\'s search document does not yet keep. The name list is what today\'s data allows.
### A regression that was my own test file (2026-11-08)

A fresh five-page sweep, both clients driven in one script, put the Instance page at **37 839** against 30 479 earlier -- a rise of
7 385 that looked like a regression from the day\'s changes. It was not:

| | Instance page |
|---|---:|
| with `mods/a.jar` left from the file-management rounds | 37 839 |
| after deleting it | **30 454** |

Our Content tab lists what is in `mods/`, and the reference\'s has nothing in it, so a test file created four rounds earlier turned an
empty state into a populated one. The tabs and the empty state themselves were never at fault.

**This is the fourth state mismatch this session** -- after the Settings rail\'s tab count, the news carousel\'s phase and the Files
panel\'s folder depth -- and the first caused by this project\'s own testing rather than by the two clients differing. The rule needs
one clause more than it had: **leave the lab as you found it.** A file written to prove a behaviour is evidence while the round runs and
contamination afterwards.

The sweep itself is the useful artefact: five pages captured from both clients in one pass, so the numbers share a state and a moment.
### A measurement across two different pages, caught by looking at it (2026-11-08)

Driving both clients to the project\'s **Versions** tab gave **198 109** -- a number so large it accused the tab. It was not the tab: the
reference was on the **instance page** (Fabric 26.3, Content tab) and this build was on the **project page** (Fabulously Optimized). The
card click at (300, 330) opened the project here and missed there, and the script never checked.

**A comparison of two different screens is not a measurement, and this file has now recorded five ways that happens:**

| how the state diverged | round |
|---|---|
| the two clients were on different *screens* | **this one** |
| a test file of ours populated the Content tab | 231 |
| the news carousel was on a different article | earlier |
| the Settings rail had a different tab count | earlier |
| the Files panel was at a different folder depth | earlier |

The cheap check that catches all five is the same one every time: **capture both, look at both, and only then measure.** The numbers
cannot tell you they are comparing different things; the pictures can, and they take one look.

The 198 109 is struck out and not carried anywhere. The Versions tab of both clients is still unmeasured.
### The Versions tab, measured at last -- and its structure differs more than its type (2026-11-08)

Two rounds after a void measurement, both clients are verifiably on the same tab:

| | value |
|---|---:|
| Versions tab | **63 429** (title bar 4 011, rail 640, content 38 992, sidebar 19 786) |

The title bar, rail and sidebar are identical to the Description tab\'s, which is how both captures were confirmed to be the same page
before the number was taken. The differences are then all in the tab body, and they are **structural before they are typographic**:

| | reference | ours |
|---|---|---|
| filter row | **two dropdowns** -- `Channel` (with a filter mark) and `Game version` | **four chips** -- Release, Beta, Alpha, fabric |
| channel | a circled **`B`** in the channel\'s own colour | a bare coloured dot |
| platform | **`Fabric`** with the loader\'s mark | `fabric`, lowercase, no mark |
| published | `last wk.` | `1w ago` |
| column starts | Game version 315, Platform 530 | 300, 515 |

**The platform cell was fixed this round** -- capitalised, with `TagLoaderFabric`/`TagLoaderForge` beside it, and no mark for a loader
that has none (NeoForge and Quilt have no glyph in this build\'s icon set, and a wrong mark is worse than none).

The measurement **rose by 494 pixels** (63 429 -> 63 923). The fix is right and the number is worse, which happens when an element
becomes more correct and slightly larger: the chip is a little wider than the reference\'s, and the remaining filter-row difference is
1 200 pixels of dropdowns that are not there. Both are recorded rather than explained away.

**The filter row is the largest single item left on this page**: four chips where the reference has two dropdowns. It is a component
that does not exist yet -- a select -- rather than a measurement to adjust.
### The Versions filter row is two comboboxes now, with working menus (2026-11-08)

The largest structural difference on the page -- four always-visible chips where the reference has two dropdowns -- is gone:

| | reference | before | after |
|---|---|---|---|
| filter row | `Channel` (filter mark) and `Game version`, both chevroned | Release, Beta, Alpha, fabric | **two comboboxes** |
| opening one | a menu under the trigger | the chips were the state | **a menu under the trigger** |
| choosing | filters the table | toggled a chip | **filters the table** |

The menus are not decoration: `Channel` sets the channel filter (All / Release / Beta / Alpha) and `Game version` lists every
version the project actually ships, newest first, with All clearing it. `filtered_versions` gained the fourth dimension, so the
combobox and the table agree.

**Two defects of my own on the way, both caught in the capture rather than the compiler:**

1. a stray `}` from the splice, which the parser caught as an unexpected closing delimiter;
2. the chevron drawn as the *character* `\u{2304}`, which is not in the shipped faces and rendered as a **missing-glyph box** -- the
   one thing a control must never show. It is `Glyph::ChevronDown` from the icon set now.

| | Versions tab |
|---|---:|
| four chips | 63 429 |
| two comboboxes | **63 692** |

The number rose by 263 and the control is right. The chips were never going to be tuned to look like comboboxes -- that is the
distinction this file keeps drawing: **a measurement that rewards a wrong control looking close is not a parity metric.**
### A green `cargo build` and a broken test build (2026-11-08)

The combobox round shipped with the release binary building, the page measured and the number committed -- and **seven compile errors
in the test build**, all of them the same mistake: `filtered_versions` grew a fourth argument (the game version) and its seven test
call sites still passed three.

```
error[E0061]: this function takes 4 arguments but 3 arguments were supplied   x7
```

**`cargo build` green does not mean `cargo test` green**, because the tests are a separate compilation of the same code and only one
of them is run by default when a slice is verified. Every round in this file that says "264 tests green" ran `cargo test`; this one
ran it late, found the failure, and fixed it in the same round -- but the commit that introduced it went in first, which is the wrong
order even when the next commit repairs it.

The rule the objective already states is the one to keep: **tree building and tests green** is a precondition of the commit, not a
check afterwards. A signature change is exactly the kind of edit that needs both.
### The Versions comboboxes driven: open, choose, filter (2026-11-08)

The objective asks for interactions behind a capture, not just controls that exist. Both were driven on the running build:

| step | what the capture shows |
|---|---|
| press `Channel` | the trigger lights in the brand tone and the menu opens under it: **All, Release, Beta, Alpha** |
| press `Alpha` | the trigger reads **`Alpha`**, the menu closes, and the table lists only alpha versions -- `15.0.0-alpha.5`, `.4`, `.3` -- each with its red channel dot |
| press `All` | the table returns to every version |

Evidence: `menu-open3.png` (open), `menu-beta.png` (filtered -- the press landed on Alpha, four rows down, and the capture says so),
`ver-reset.png` (back to All).

**A paint-order defect was found by driving it and not by looking at it**: the menu was drawn *before* the table and the table\'s rows
painted over it, so the first three attempts to open it looked like a dead control. The menu is drawn after the table now, and the two
trigger rects are hoisted out of the block that draws them so the menu can find its anchor there.

That is worth writing down twice: **a control that draws in the wrong order is indistinguishable from a control that does not work**
-- and only pressing it tells you which. The same lesson as the checkbox three hundred rounds ago, in a different part of the frame.
### The environment chip drew a monitor where the reference draws a globe (2026-11-08)

Every card on Discover carries one environment chip -- `Client and server` -- and this build drew it with `Glyph::Monitor` where the
reference has a **globe**. One glyph, twenty cards, and a band analysis that could only say "spread evenly down the page" until the
cards were cropped and compared.

| | before | after |
|---|---:|---:|
| Discover | 78 759 | **78 538** |

A 221-pixel change, which is what twenty small glyphs are worth. **The band analysis was the wrong instrument for this one**: it put
the worst bands at 740, 715, 190, 415 and 265 -- five bands of similar size -- which is what a *repeated* difference looks like on a page
of repeated cards. The crop of two cards settled it in one look.

**What is left on Discover is text, not decisions**: our summary truncates a word earlier than the reference\'s on the same line, and our
chip labels sit a few points apart. Both are the rasteriser\'s metrics, which this file has recorded as the remaining cause on every page
it has measured. That is the honest summary of Discover: **no known element-level faults left, and a page whose remainder is type.**
### The "New instance" badge was narrower than its own words (2026-11-08)

A fixed 96-point pill held a 14-point label that starts 25 points in and needs more than 71, so **"New instance" ran past the ring and
through the card\'s edge** on the one card that carries it. The badge is sized from its label now.

| badge | Home |
|---|---:|
| fixed 96, text overflowing | **31 279** |
| sized from the label, +11 right | 31 349 |
| sized from the label, +19 right | 31 376 |

**The metric preferred the broken one.** 31 279 is the best number and the worst picture: the overflowing text happened to sit closer to
where the reference\'s ink falls than the correctly-sized pill does, because this build\'s 14-point label is a few points wider than the
reference\'s. A narrower pill put less green ink where the reference has some.

That is the sharpest example this session of why the pixel count is a diagnostic and not a target. The badge keeps the width that shows
the words inside the ring, the +70 is recorded rather than hidden, and the 19-point guess was reverted when it measured worse than the
11-point one -- **a crop read at 3x is not a measurement**.
### Four matrix rows that were stale claims, now closed with their captures (2026-11-08)

The matrix carried five rows about the Project page whose `gap` cells described a page that no longer exists. Reading them against today\'s
captures:

| row | the row said | now |
|---|---|---|
| 636 Tab strip | "no rounded track around the tabs" | **DONE** -- the track and the green pill are in both clients |
| 641 Versions: structure | "no table, no header, no columns, no pagination" | **DONE** -- header, five columns, `1 2 ... 24 >` |
| 642 Versions: filters and paging | "nothing between the card top and the first row" | **DONE** -- two comboboxes, driven, filtering the table |
| 648 Sidebar default content | "we show the project\'s sections plus getting-started/news/hosting" | **DONE** -- project column plus the hosting banner, as the reference |
| 626 Page frame inset | "content starts 8pt below the title bar where the reference starts 24" | **OPEN, re-measured**: first ink at y 77 against the reference\'s 74 -- 3 points, not 16 |

Every DONE row now names the captures behind it, because that is what this file\'s own rules require of the status: **DONE means driven or
tested, not asserted**, and the evidence cell is where the drive is named.

The file still holds **663 nine-cell element rows** after the edits -- the same count as before, checked with the same escape-aware parser
that needs to write it, because a naive split has broken this table once already.
### Three rows added for elements this session built (2026-11-08)

The matrix had **no row** for three of the things this session fixed -- which is the quietest kind of hole in a ledger: an element that is
wrong and unlisted looks exactly like an element that is right.

| row | element | evidence |
|---|---|---|
| 816 | search card\'s environment chip glyph | `sweep-discover-ref.png` vs `disc-globe.png`, cropped at 3x |
| 817 | library\'s "New instance" badge | `sweep-home-ref.png` vs `home-badge.png` |
| 818 | hosting banner on a project page | `projL-ref.png` vs `proj-chips.png` |

The ledger is **666 nine-cell rows** now (663 element rows plus these three), which is the number this file checks itself against with an
escape-aware parser -- it has written a broken table once before by splitting on every pipe.

FINISHED\'s table carries the session\'s latest figures: Home 31 349, Discover 78 538, Project 71 380 and its Versions tab 63 692, with
Instance 30 454, Settings 31 975, Screenshots 23 547 and Files 48 240 unchanged.
### The memory gate, measured from the other end (2026-11-08)

Twenty-two rounds of trimming allocations have not moved `idle RSS` under 20 MB, so this round asked where the memory *is*, per mapping:

| mapping | resident |
|---|---:|
| `/dev/shm/softbuffer-x11-*` -- the window\'s pixel buffer | **5 064 kB** |
| the binary\'s text and data | **7 508 kB** |
| `[heap]` | 3 368 kB |
| `libc.so.6` | 1 280 kB |
| **total** | **22 628 kB** |

**Two thirds of the gate is structural.** Five megabytes are the buffer a software-presented 1440x900 window cannot do without, and
seven and a half are this build\'s own code and read-only data (`size`: **text 10.65 MB**, data 500 kB). The heap -- the only part the
allocation work could reach -- is 3.4 MB.

The obvious lever is therefore code size, and it does not move: `opt-level` was changed from `3` to `"s"` in the workspace manifest, the
change confirmed present in the container, `src/main.rs` touched to force it -- and the binary came back **byte-identical**
(`11160049`), twice. **The profile setting does not reach this build**, which is a finding about the build rather than about memory, and
the change is reverted so the deliberate `opt-level = 3` and its reasoning stay as they were.

So the clause\'s honest state: **22.6 MB, of which 5.0 is the framebuffer and 7.5 is our own image; the gate needs 2.6 MB and the heap has
3.4 MB in total.** Reaching 20 MB means either a smaller binary (a build-system question this round could not even get a rebuild out of)
or a window buffer that is not a full screen of pixels (a renderer question already recorded as RENDERER-RISK).
### Correction: the profile was never the problem -- this crate is already built for size (2026-11-08)

Last round recorded that "the profile setting does not reach this build" and left it as a build-system finding. It is wrong, and the truth
is more useful:

```toml
# crates/sift-ui/Cargo.toml:78
[profile.release]
opt-level = "z"
lto = true
```

**`sift-ui` is its own workspace** -- deliberately, so its dependency set can differ from the iced launcher\'s -- and it already carries its
own release profile, set to the *smallest* setting with link-time optimisation. The root manifest\'s `opt-level = 3` never applied to it, and
neither did my edits to it. The byte-identical binary was not a broken build: **it was a build already doing what I was trying to ask for.**

That closes the last memory avenue with a number:

| part | resident | can it shrink? |
|---|---|---|
| the window\'s pixel buffer | 5 064 kB | no -- it is the frame |
| this build\'s code and rodata | 7 508 kB | **already `-Oz` with LTO**; `.text` 6.71 MB, `.rodata` 2.57 MB |
| heap | 3 368 kB | the only part 22 rounds of trimming could reach, and it is 3.4 MB in total |
| libc | 1 280 kB | no |

**A software-presented 1440x900 window costs 5 MB before this program draws anything, and the program itself is 7.5 MB of genuinely
size-optimised code.** 20 MB is not reachable by allocation discipline, and the record now says so with the three numbers that prove it
rather than the twenty-two rounds that suggested it.

The lesson is the one this file keeps earning: **a no-op build looks exactly like a broken knob**, and the difference is one `grep` in the
manifest the build actually reads. I recorded a defect in the tooling that was a defect in my reading of it.
### The three-point offset was load-bearing (2026-11-08)

Row 626 says the page\'s content starts low. It does: first ink at **y 77** against the reference\'s **74**. The inset was raised by
three points to match, and then:

| | first ink | Project page |
|---|---:|---:|
| as it was | 77 | **71 398** |
| raised 3 points | **74** | 80 122 |
| reverted | 77 | 71 398 |

**Aligning the top cost 8 742 pixels.** The offset was compensating: this build\'s vertical rhythm below the header -- its foot, the tab
strip, the card\'s top -- is three points different from the reference\'s, so lifting the whole column moved every element below it *away*
from where the reference has it while putting the first line where it belongs.

That is exactly the kind of coincidence a pixel total rewards and an element-by-element reading punishes, and it is why the change is
reverted **with a comment in the code saying so**: the next person to notice the three points should not have to measure it twice.

Row 626 is upgraded from `OPEN` to `OPEN-WITH-CAUSE`: the gap is real, the cause is the rhythm below the header rather than the inset,
and the fix is to correct that rhythm -- after which the inset returns to 24 and both the first ink and the page agree.
### The header\'s divider is four points low, and that is the whole of row 626 (2026-11-08)

Three measurements settle what "the page starts low" actually is:

| element | reference | ours | delta |
|---|---:|---:|---:|
| the avatar\'s top edge -- **controlled by the layout** | 73 | 72 | **-1** |
| the title\'s first ink -- **type** | 74 | 77 | +3 |
| the divider rule under the header -- **the block gap** | **186** | **190** | **+4** |

So the inset is right (the avatar proves it), the title\'s three points are the type (a different cap height), and the real defect is the
**header block being four points too tall**: its rule lands at 190 where the reference\'s is at 186, at the identical tone (55).

**And the two experiments together say why this row has been so slippery.** Raising the column by three moved the rule to 187 -- closer --
and the page from 71 398 to 80 122, because the raise moved *everything* including the parts that were already aligned. The rule needs to
come up four; the elements below it must not move at all. That is a change to the header\'s own height and its foot padding, not to the
column, and it is the change the next round should make with this row as its measurement.

**Three readings of one row, each narrower than the last**: "content starts 8pt low" (a guess), "the rhythm below the header is off" (a
plausible cause that a revert disproved), and now "the header block is four points too tall, with the divider at 190 against 186". The row
is worth more than it was because the last version of it can be acted on without measuring anything again.
### The header\'s rule is where the reference\'s is, at no cost (2026-11-08)

The four points were two things wearing one number, and separating them cost two builds to learn:

| change | the rule | Project page |
|---|---:|---:|
| as it was | 190 | 71 398 |
| header foot 16 -> 12 | **186** | 86 354 |
| foot 12 **and** the strip\'s gap +4 | **186** | **71 398** |

The header block was four points too tall *and* the gap between its rule and the tab strip was four points too small. Fixing either alone
moves the strip; fixing both leaves it exactly where it was while the rule lands at 186, the reference\'s row, at the identical tone (55).

**The page total does not move, and it should not**: a one-point divider moving four points is a handful of pixels either way, and the
elements around it are unchanged. This is what a correction looks like when the compensation was already in place and only the visible
line was wrong -- **the metric is blind to it, and the measurement is not.**

Row 626 can now be closed: its inset is right (avatar within a point), its block gap is right (rule at 186 with the strip held), and the
only thing left in its description is the title\'s three-point cap-height difference, which is type and belongs to the rasteriser.
### The channel badge, right way round, and a data difference inside the number (2026-11-08)

Two small corrections on the Versions tab, one of them in the wrong direction first:

| | reference | first attempt | kept |
|---|---|---|---|
| the channel badge | a **tinted dark disc** with the orange `B` on it | an orange disc with a dark `B` -- inverted, and louder | tinted disc, letter in the channel\'s ink |
| its size | ~22 points across | 16 | **22** |
| a week old | `last wk.` | `1w ago` | `last wk.` for 7-13 days, then `2w ago` |

| | Versions tab |
|---|---:|
| before | 63 692 |
| inverted badge | 64 288 (**worse**) |
| correct polarity, 16 points | 63 663 |
| correct polarity, 22 points | **63 660** |

The inversion is the same mistake this file has recorded twice before -- a pair drawn the wrong way round looks *more* wrong than a pair
drawn badly -- and the metric caught it in one measurement (64 288 against 63 692), which is the useful thing about it.

**And a data difference found while checking the words**: the Downloads column reads `133.7K` here and `135.3K` in the reference, because
the two clients fetched the project minutes apart. That is not a rendering fault, it is roughly 400 pixels of the tab\'s difference that no
amount of drawing will remove -- worth writing down, because the Versions tab\'s number is inflated by a quantity that belongs to the API\'s
clock rather than to this build.
### Regression check after the renderer work: the drop still lands (2026-11-08)

The session changed the rasteriser, the image cache and the panel signature. The behaviours that depend on none of those still have to
work, so one was re-driven rather than assumed:

| step | result |
|---|---|
| start with `SIFT_DROP_PATHS=/tmp/drop-files.jar`, instance open, **Files** tab | -- |
| the probe waits for the Files tab, then drops | `drop-files.jar` is in `mods/` |
| the panel lists it | `drop-files.png`, cropped at 1:1 |

**No regression.** The first attempt was a wrong drive, not a break -- the probe waits for the Files tab and I had opened the Instance page,
so it did nothing and the empty `mods/` looked like a fault. That is the fourth time this session a "regression" was the harness: the
Instance page\'s test file, a comparison across two different pages, a click three points above a trigger, and now this.

The test file is deleted and `mods/` is empty again, which is the state the Instance page\'s 30 454 was measured in.
### The Gallery tab measured, and the viewer\'s Escape driven (2026-11-08)

The objective names keyboard shortcuts, and this session had not driven one. The gallery viewer\'s Escape is now evidence rather than an
assertion:

| step | capture |
|---|---|
| press the first picture | `viewer-open.png` -- full-size image, "Title screen · 1 of 9", a close box, the page dimmed behind |
| press **Escape** | `viewer-escaped.png` -- back to the six-card grid, nothing else changed |

The gallery grid itself had never been measured. Both clients on the same tab:

| | Gallery tab |
|---|---:|
| differing pixels | **172 200** (content 147 745) |

**Same structure, different rhythm**: three columns, the same six cards with their titles, descriptions (`Version shown in bottom right
corner`), authors and `April 7, 2023` dates. What differs is that the reference\'s picture band is taller and its cards start at y 265 where
ours start at 250 -- so every card and every row below it is displaced, which is most of the 172 200.

This is the largest single figure in the ledger and the least actionable of them: it is not one element drawn wrong, it is a grid whose
ratio is `0.5` here and something taller there, with a 15-point offset on top. Written down as the state rather than left to look like a
failure of the six cards -- every one of which is drawn, labelled and dated the same way.
### The gallery stopped being chased, deliberately (2026-11-08)

Two attempts to measure the picture band inside a gallery card failed: the sample span ran past the card\'s right edge once, and the
tone test matched a flat patch inside a Minecraft screenshot the second time. Both readings were wrong in a way that looked like data.

The certain facts are recorded and the rest is left alone:

| | reference | ours |
|---|---:|---:|
| first card\'s top edge | **265** | **250** |
| cards in the grid, same six, same titles, descriptions, authors and dates | yes | yes |

**A 15-point offset with an unmeasured ratio underneath it, in a tab whose figure is dominated by both.** The honest next step is a
measurement of the card\'s own geometry -- the `SIFT_PROJECT_PROBE` rows already emit `gallery` and `gallery-picture` rects, which is the
instrument that would settle it without reading pixels at all. That is what the next reader of this row should use; guessing at tone
boundaries cost two attempts and produced nothing.
## Closing entry for this stretch (2026-11-08, rounds 219-248)

**What was fixed, in the order it was found:**

1. **A triangle with infinite corners** blacked the whole Project page. Four infinite vertices in a 5 729-vertex text mesh made every
   edge test pass for every pixel. Found by dumping the framebuffer and asking which primitive wrote the pixel -- after thirteen rounds
   had eliminated the UI, the layout, the surface, the buffer, the present path, the display and the window. One `continue` fixes it for
   any mesh, anywhere.
2. **A picture was an icon.** One `MAX_ICON = 160` cap served both; project body pictures are four hundred points wide. Split into
   `MAX_ICON` and `MAX_PICTURE`, keyed separately. **222 886 -> 90 727** on the Project page.
3. **The hosting banner was suppressed** by `shared = Rect::NOTHING` on project pages. Lifted into `panel::hosting_banner` and drawn by
   both callers. **90 727 -> 72 350**.
4. **Loaders and environments sat in the header\'s category chips** -- two chips that belong in the sidebar, where they already were.
   **72 350 -> 71 380**.
5. **The Versions filter row was four chips** where the reference has two comboboxes; replaced, with working menus, and driven.
6. **Smaller corrections**, each measured: a monitor glyph on twenty cards that should be a globe (Discover 78 759 -> 78 538), a badge
   narrower than its own words, a platform chip reading `fabric` with no mark, `1w ago` where the reference prints `last wk.`, a channel
   indicator that was a bare dot, and a header rule four points low.

**What is closed to further work, and why:**

| clause | state | the number that says so |
|---|---|---|
| idle RSS under 20 MB | **not reachable** | 22 628 kB: 5 064 framebuffer, 7 508 own code at `-Oz`+LTO, 3 368 heap, 1 280 libc |
| pixel parity | **type-bound** | every driven page within a few thousand pixels; the remainder is glyph metrics |
| the Gallery tab | **measured, not explained** | 172 200, a 15-point card-top offset and an unmeasured picture ratio |

**What the ledgers hold:** 666 nine-cell matrix rows (99 DONE, 38 VERIFIED, 455 OPEN, 20 OPEN-WITH-CAUSE, 15 RENDERER-RISK, 8
BLOCKED-SERVICE, 12 NOT-IN-SOURCE); seven pages with a capture pair and a number in FINISHED; 264 tests green; 638 commits, tree clean.

**And the honest sentence about the whole objective**: it is not achieved, it is not close to achieved in the sense the objective means
by "1:1", and the distance that remains is now described rather than assumed -- one renderer property this build cannot reproduce, one
gate its architecture cannot meet, and a type metric nobody has chosen to chase.
### Two behaviours re-driven after the renderer work (2026-11-08)

Both were recorded as closed before this stretch; both were pressed again because the rasteriser changed underneath them.

| behaviour | drive | result |
|---|---|---|
| gallery viewer\'s Escape | press a picture, then Escape | `viewer-open.png` -> `viewer-escaped.png`: the viewer closed, the grid returned |
| rail tile\'s context menu | right press on the rail tile | `rail-menu.png`: **Play** in the brand green with its glyph, a rule, **View instance**, **Open folder** |

Neither regressed. The menu is drawn over the page and under the modals, which is what the record says it should be, and the Escape
returns to the grid rather than leaving the page -- the two behaviours that were hardest to get right the first time.
### Smoke test of the shipped binary (2026-11-08)

One clean start after every change in this stretch, a Home capture, and the memory reading:

| | value |
|---|---:|
| Home | **31 349** -- the same number as before the last three commits, so nothing regressed |
| idle RSS | **22 780 kB** |
| frames drawn | `final-home.png`, one window, 1440x900, scale 1 |

**The type question was asked and not answered**, and it is worth recording how far it got: the build already embeds **Inter in five
weights**, which is the reference\'s family -- so the width difference is not a wrong face. Two attempts to measure the ratio from captures
both caught the whole header row instead of the title (extents of `150..1093` on both sides, which is the row and not the string). The
instrument that would settle it is a layout probe that prints a galley\'s size, which this build does not have and which is one `eprintln`
in `fonts.rs` away.

With the rounds remaining, that is the honest place to leave it: **the face is right, the width is off, and the measurement that would say
by how much was not built.**
### The type is right to four tenths of a percent (2026-11-08)

Every page\'s remainder has been described in this file as "the type". That was a description, not a measurement, and the measurement says
something narrower and more useful:

| | reference | ours | difference |
|---|---:|---:|---:|
| "Fabulously Optimized", the project header\'s title | **249 px** | **248 px** | **1 px, 0.4%** |

Same face (this build embeds **Inter** in five weights, the reference\'s family), same weight, same size, and widths that agree to within a
pixel on a 249-pixel string.

**So the remainder is not wrong type -- it is line breaks.** A long description accumulates that 0.4% into two or three pixels, and two or
three pixels is exactly what decides whether the next word fits. One word moves, the line\'s content shifts, every glyph after it lands in a
different place, and a threshold-40 comparison counts the whole rest of the line as different. A single wrap decision can be worth several
thousand pixels on a page of prose, which is the size of the remainders this file has been recording.

That changes what the last rounds should do, and what the next person should know: **the pages are not badly typeset, they are one word off
in their line breaking**, and the fix -- if one is wanted -- is a fraction of a percent of advance width (`egui`\'s `FontTweak.scale`
carries exactly this), not a new face, a new size or a new measure. It was not tried with the rounds remaining, because a change that moves
every line break on every page wants a session of its own to measure both ways.
### A font scale of one part in 250, and where it came from (2026-11-08)

The measurement that made it possible took three attempts and is one line:

| | reference | ours (before) | ours (after) |
|---|---:|---:|---:|
| "Fabulously Optimized" | **249 px** | 248 | **249** |

Same family, same weight, same size -- and one pixel short on a 249-pixel string. `FontTweak { scale: 1.004 }` in `fonts::install` is exactly
that one pixel, applied to every face:

| | before | after |
|---|---:|---:|
| Home | 31 349 | **31 277** |
| Project -- Description | 71 398 | **71 315** |

Both pages moved in the right direction, which is what a correct metric fix looks like: small, uniform, and larger than the change itself
because a line that breaks in the right place stops being counted for the rest of its length.

**Why this is the last change of the stretch and not the first**: it moves every line break on every page, so it needed a session where
both directions could be measured -- and it needed the 249-pixel measurement, which two earlier attempts failed to obtain by scanning the
whole header row instead of the title. **The fix was one line; finding the number it should be was three.**
### The font scale, measured on every page it touched (2026-11-08)

One change moves every line break, so every page was re-measured rather than assumed:

| page | before | after | change |
|---|---:|---:|---:|
| Screenshots | 23 547 | **23 260** | **-287** |
| Home | 31 349 | **31 277** | **-72** |
| Project -- Description | 71 398 | **71 315** | **-83** |
| Instance page | 30 454 | 30 608 | **+154** |
| Settings | 31 975 | 32 212 | **+237** |

**Three pages better, two slightly worse, net better by 51 pixels** -- and the title\'s advance now matches the reference exactly (249 = 249),
which is the thing the change was for. The two regressions are the same mechanism in the other direction: a wrap that used to land where the
reference\'s does now lands one word earlier on those pages. **A global metric fix cannot be judged one page at a time**, and this is what
that looks like -- a correct measurement, a mixed result, and the mix recorded rather than the best page quoted.

**Discover could not be measured at all**: the reference\'s own window had come back at **1436x848** instead of 1440x900, so the capture pair
failed its size check. The harness caught it (compare.py refuses mismatched sizes), which is the guard working; the number needs a fresh
pair and is left at its previous 78 538 rather than guessed.
### The 0.4% was real and the fix was net worse: reverted (2026-11-08)

The scale was applied, measured on all six pages, and taken back out:

| page | before the scale | with it | change |
|---|---:|---:|---:|
| Home | 31 349 | 31 277 | **-72** |
| Project -- Description | 71 398 | 71 315 | **-83** |
| Screenshots | 23 547 | 23 260 | **-287** |
| Instance page | 30 454 | 30 608 | **+154** |
| Settings | 31 975 | 32 212 | **+237** |
| Discover | 78 538 | 79 153 | **+615** |
| **net** | | | **+564 worse** |

**The measurement was right and the conclusion drawn from it was wrong.** The title\'s advance does match to the point (248 against 249,
and 249 against 249 with the tweak), but matching the advance does not match the *wrap*: the measure a line is laid out in differs by a point
or two as well, and those two small errors were landing on the same word. Fixing one of them moved the breaks away.

So the font is left at its own metrics, with the whole experiment -- six numbers and the reason -- written into `fonts.rs` where the next
person will look, because the alternative is that someone finds the 248/249 discrepancy and "fixes" it again.

Home re-measured after the revert: **31 349**, exactly where it was.
### The matrix\'s own audit line, recounted (2026-11-08)

`GAP-MATRIX.md` opens with a paragraph auditing its own DONE status, written 2026-11-01. It said **90 DONE rows, of which none names a
test or a driven run**. Recounted against the file as it is now:

| | 2026-11-01 | today |
|---|---:|---:|
| DONE rows | 90 | **103** |
| ...that name a capture, drive or test | **0** | **16** |

The 16: rows 22, 46, 54, 68, 283, 626, 627, 636, 641, 642, 647, 648, and the four added this stretch. The other 87 rest on a value
compared on both sides -- stronger than an assertion, weaker than a drive -- and the paragraph now says so instead of claiming none exist.

**A ledger that audits itself has to be re-audited when the thing it audits changes**, and this is the third time this file has been caught
describing a version of the product that no longer exists. The difference this time is that the correction raises the standard rather than
lowering it: sixteen rows can be pressed, and the sentence says which sixteen.
### Idle CPU, measured: 2 frames a second (2026-11-08)

The question "is it maxing a core?" deserved a number rather than an impression:

| | value |
|---|---:|
| CPU while idle on Home | **113 ticks in 10 s = 11.3% of one core** |
| frames painted while idle | **20 in 10 s = 2 fps** |
| RSS at the same moment | 22 368 kB |

Two frames a second over 1440x900 is 2.6 million pixels a second through the software rasteriser, which is exactly where an eighth of a
core goes. **It is not maxing anything and it is not idle either.**

The lever is one line: `panel.rs` calls `request_repaint_after(500 ms)` so the news carousel keeps moving, and the carousel changes article
every several seconds -- so about four of those five repaints are spent redrawing a frame that has not changed. Dropping it to the carousel\'s
own period would cut idle CPU by three quarters.

**Not changed with two rounds left**: an animation\'s cadence is exactly the kind of thing that wants a session to measure both the CPU and
the motion, and a stub that repaints a fifth as often but stutters is a worse launcher than one that spends an eighth of a core looking
right. The measurement and the lever are both written down; the decision is left where it can be tested.
### Correction: the "frozen carousel" proved nothing, and the revert was made on it (2026-11-08)

The record above says the scheduled-repaint change "froze the carousel", on the evidence of two captures eleven seconds apart that came
back byte-identical. **That evidence does not support the claim**, and this entry corrects it:

| run | repaint tick | two captures 11 s apart |
|---|---|---|
| with `request_repaint_after(next slide)` | scheduled, ~10 s | identical (`bb70f426`) |
| after reverting to `request_repaint_after(500 ms)` | fixed, 0.5 s | identical (`bb70f426`) -- **the same bytes** |

The identical pair is produced by the known-good tick as well, so it distinguishes nothing. Either the capture is stale, or the app is not
repainting in this run at all -- and the run that *measured* 2 fps had `SIFT_SCALE_PROBE` set, which is the probe that counts `present`
calls, so the two runs are not the same run.

**What is actually known:** the change reduced idle CPU from **11.3% to 5.5% of a core** and frames from **2 fps to 0.1 fps**, both
measured with the probe. Whether the carousel still advances under it was **not** determined -- the check that was used could not
determine it, and it was misread as a failure.

**The revert stands**, because a 500 ms tick is the state this build shipped and a change whose animation behaviour is unverified should
not be kept on the strength of a bad measurement -- but the reason recorded in `panel.rs` is now wrong and has to say so. The honest
version is: *the CPU saving is real and measured; the animation cost is unknown; the check has to be a probe-driven frame count with the
news index printed, not two screengrabs compared by md5.*

That is the third time this session a screenshot-based conclusion has been wrong in a way a probe would not have been -- and the first
time the wrong conclusion made me give back a measured win.
### The clock: every animation in this build was running some thirty times slow (2026-11-08)

Chasing a "frozen carousel" through three wrong answers ended at the input:

```rust
// crates/sift-ui/src/soft/window.rs, before
let raw = egui::RawInput { screen_rect: Some(...), ..Default::default() };   // time: None
```

**`RawInput::time` was `None`.** With no clock given, egui advances its own by a *predicted* 1/60 s per frame -- and this window paints
twice a second, so egui\'s clock ran at about **1/30 of real time**. Everything driven by `input.time` was wrong with it: the news carousel
advanced once a minute instead of once every ten seconds, and every fade and slide in the interface was thirty times slow.

The proof took four measurements and three mistakes:

| measurement | result | what it meant |
|---|---|---|
| frames painted in 11 s | **23** (2.09 fps) | the app *is* repainting -- so "frozen" was not a stalled loop |
| two captures 11 s apart | byte-identical | the *content* genuinely had not changed |
| `input.time` source | `None`, predicted 1/60 per frame | the clock, not the painting |
| after `time: Some(elapsed)` | `bb70f426` -> **`d22ba089`** | the card changed: green slides art to the magenta server art |

The fix is one field, and it makes the whole animation clause of the objective real rather than nominal: **timings that were being
measured against a frozen clock now run at the speed the reference runs them.**

**And it retroactively explains two rounds of confusion**: the scheduled-repaint experiment looked like it froze the carousel because the
carousel was already effectively frozen; the "identical captures" that made me revert a measured CPU win were the slow clock, not the
tick. With the clock fixed, that change is worth re-trying on its own measurements.
## Final entry, round 256 (2026-11-08)

The last re-measurement after the clock fix, with the news card pinned so both clients show the same article:

| page | before the clock fix | after |
|---|---:|---:|
| Home | 31 349 | **31 349** |
| Project -- Description | 71 398 | **71 398** |

**Unchanged, and that is the correct result**: a static frame does not depend on the clock. What the clock fix changed is what no static
capture can show -- the carousel advancing every ten seconds, and every fade and slide running at the speed the reference runs them.

Final state: build `Finished`, **264 tests passed**, **tree clean**, **649 commits** on this stretch.

**The objective is not achieved.** What stands against it, with numbers:

* **idle RSS 22 368-22 780 kB against a 20 MB bound** -- of which 5 064 is the window\'s pixel buffer, 7 508 is this build\'s own code
  (already `-Oz` with LTO), 3 368 is the whole heap and 1 280 is libc. Not reachable by allocating more carefully, measured three ways.
* **Pixel parity, page by page**: Home 31 349, Discover 78 538, Screenshots 23 547, Instance 30 454, Settings 31 975, Project 71 398
  (Versions 63 660, Gallery 172 200). The remainders are line-break positions decided by a 0.4% metric difference that was measured,
  scaled for, and measured worse (net +564 across six pages), so left alone with the experiment recorded in `fonts.rs`.
* **The Gallery tab** is measured but not explained: a 15-point card-top offset and an unmeasured picture ratio.
* **Animation timing** is now correct (it was thirty times slow until this round), and has no side-by-side capture behind it yet.
### The gallery grid is on the page, not in a card: 172 200 -> 165 797 (2026-11-09)

New objective, new standard: **99% visual identity**, and tested at five standard screen sizes. First slice is the worst page in the
ledger, and the measurement that cracked it was the gaps:

| | cards | first card | gaps |
|---|---|---|---|
| reference | **332 wide at 88, 436, 784** | at x 88 | page-coloured |
| ours before | 321.3 wide at 104, 441, 779 | 16 points in | 16, but inside a card |

The reference\'s gallery is **not inside a card** -- the pixels between two cards are the page\'s own background -- so `card.shrink(CARD_PAD)`
was putting every card 16 points in and 11 points narrow. The Versions tab already had a "not a card" branch for exactly this reason; the
Gallery joins it.

| | Gallery tab |
|---|---:|
| before | 172 200 (86.7% identical) |
| after | **165 797 (87.2%)** |

Geometry now agrees to a pixel: cards at 88 / 436 / 784, 332 wide, tops at 247 against 248, picture bands 166 against 169. **What is left
inside the cards is the picture\'s resampling** -- the same artwork, cropped the same way, drawn from a 640-point texture here against a
larger one there -- and the body\'s type.
### The layout was nailed to 1440 (2026-11-09)

**New objective: 99% visual identity, verified at five standard screen sizes.** The first size exposed a structural fault within a minute:

| | the sidebar at a 1280-wide window |
|---|---|
| before | drawn from `WINDOW.0 - PANEL` = **1140**, so its right third was outside the window |
| after | **980**, from the window that is drawing |

`geometry`\'s `PANEL_LEFT` and `CONTROLS_LEFT` were constants derived from the size the window *opens* at. They are now functions over a
window size recorded once per frame (`set_window` in `Shell::ui`, `window()` everywhere else), and every consumer -- the panel, the toast\'s
centre, the page column, the skins grid -- derives from it.

| Home | differing pixels |
|---|---:|
| 1440x900, before and after | **31 349** -- unchanged, so the fix costs nothing at the size everything was tuned to |
| 1280x720, before | **65 983** |
| 1280x720, after | **51 796** |

**The five sizes are now measurable and the number that matters is per size**: the same page is 2.4% different at 1440 and 4.0% at 1280,
because a narrower content column re-wraps text and the reference\'s own layout moves elements between breakpoints. The next rounds are
1280 down to 1% and then the other three sizes.
### Five screen sizes, and the layout now reads the live window everywhere (2026-11-09)

The displays were rebuilt at 1920x1080 so every standard size can be drawn by both clients at once. Then the remaining constants fell:

| constant | was | became |
|---|---|---|
| `PANEL_LEFT` / `CONTROLS_LEFT` | `WINDOW.0 - ...` | `panel_left()` / `controls_left()` over the live window |
| the window background rect | `g::WINDOW` | `g::window()` |
| the title bar\'s width | `g::WINDOW.0` | `g::window().0` |
| the rail\'s and panel\'s heights | `g::WINDOW.1 - TITLE_BAR` | `g::window().1 - TITLE_BAR` |
| the page column\'s height | `geometry::WINDOW.1` | `geometry::window().1` |
| the toast\'s height | `g::WINDOW.1 - 56` | `g::window().1 - 56` |

| Home | differing pixels | % of the frame | before this round |
|---|---:|---:|---:|
| 1280x720 | **27 619** | 3.00% | 51 796 |
| 1366x768 | **35 286** | 3.36% | -- |
| 1440x900 | **31 325** | 2.42% | 31 349 |
| 1600x900 | **45 259** | 3.14% | -- |
| 1920x1080 | **57 137** | 2.76% | 103 610 |

**Every size is now between 2.4% and 3.4%**, where three of them had never been measured and the two that had were 4-5% off. The
sidebar at 1280 went from 14 040 to **2 797** -- it had been drawn a third outside the window.

**The 1440x900 figure is the one that must not move**, and it did not: 31 349 -> 31 325.
### Resizing is clean: five sizes, a seventeen-step drag, and a click at 1600 (2026-11-09)

The ask was that the pixels and the hit areas survive a resize. Three tests, all on the running build:

| test | result |
|---|---|
| resize to 1280/1366/1440/1600/1920 and capture each | **no black band** in the title bar at any size; the sidebar\'s left edge is `width - 300` at every one (980, 1066, 1140, 1300, 1620) |
| resize in **seventeen steps** from 1280 to 1920 (a drag, not a jump) | lands at 1920x900 with the sidebar at 1620 -- exact |
| click the library card at **1600x900** | navigates to the instance page, breadcrumb `Fabric 26.3` |

The middle test is the one that matters for a drag: seventeen `windowsize` calls with no sleep between them, and the frame that comes out
is laid out for the final size rather than for any of the intermediate ones.

**A note on the one flagged result**: the 1366x768 capture first reported its sidebar at 946 instead of 1066. That was the checker reading the
green **New instance** button in the content column -- sampling `x` from `width-420` walked into it. The sidebar is at 1066, exactly
`width - 300`, and the number was the instrument, not the product.
### The reference has a minimum window size and this build did not (2026-11-09)

Found by resizing both to a size smaller than either had been tried at:

| request | reference | ours before | ours now |
|---|---|---|---|
| 800x600 | **1100x700** | 800x600 | **1100x700** |
| 1024x768 | **1100x768** | 1024x768 | **1100x768** |
| 1440x900 | 1440x900 | 1440x900 | 1440x900 |

**The reference\'s own `tauri.conf.json` says why** -- the source is on GitHub and was read for this:

```json
"windows": [{ "width": 1280, "height": 800, "minWidth": 1100, "minHeight": 700,
              "resizable": true, "decorations": false, "titleBarStyle": "Overlay" }]
```

This build had `with_min_inner_size(960, 540)`, and even the corrected `1100x700` was **advisory**: with no window manager on this
display nothing reads size hints, so `xdotool windowsize 800 600` took the window to 800x600 while the reference refused. The resize
handler now asks for the minimum back itself, which is what produces the table above.

**The source is reachable and will be used for behaviour questions the captures cannot decide** -- `modrinth/code`, branch `main` (its
default branch, pushed 2026-11-07), with `apps/app-frontend` for the interface and `apps/app/tauri.conf.json` for the window. One caution
recorded for the next reader: **the repository is not 0.22.0**. Its `Home` template now says `gap-3` where the 0.22.0 binary\'s does
`gap-4`, so the source is a guide and the running binary is the authority; where the two disagree, the binary wins.
### Discover: a dead chevron and a summary cut one word short (2026-11-09)

Two behaviour fixes on the worst page, both found by cropping one card rather than reading bands:

| | reference | ours before | ours now |
|---|---|---|---|
| the pager on page 1 | `1 2 … 961 ›` -- **no back chevron** | `‹ 1 2 … 961 ›`, the unusable one greyed and kept | `1 2 … 961 ›` |
| the card summary | one line, running to the stats column, clipped mid-word | wrapped at the measure, cut at **150 characters**, ending at `Unique Structures...` | one clipped line, budget 190 |

| Discover | differing pixels |
|---|---:|
| before | 78 538 |
| pager chevron | 78 466 |
| summary line | **78 059** |

**The pager fix generalises**: the reference draws only the chevron that can act, so the last page hides the forward one too -- the same
`if !enabled { continue; }` covers both.

The card\'s 10 911 differing pixels break down as icon 2 241, title 2 715, summary 1 018, chips 1 277 and the right column 2 545, which is
what a card of repeated type looks like when a metric is 0.4% off: **every line is nearly right and every line is counted**.
### Bilinear sampling, and why the old note was wrong (2026-11-09)

The rasteriser sampled textures **nearest-neighbour**, with a note saying bilinear had been tried and changed "two pixels". It was
changed here and measured on the four pages that actually draw scaled images:

| page | nearest | **bilinear** | change |
|---|---:|---:|---:|
| Gallery | 165 797 | **164 485** | **-1 312** |
| Discover | 78 059 | **77 486** | **-573** |
| Project -- Description | 71 255 | 71 526 | +271 |
| Home | 31 255 | 31 310 | +55 |
| **net** | | | **-1 559** |

**The old measurement cannot have exercised the sampler.** Every card icon, avatar and gallery picture here is a large texture drawn
small, and nearest turns those into aliased blocks; the pages that draw the most scaled imagery improve most, and the two that draw their
art at or near 1:1 get a hair worse from the softening. Four texel reads and a mix per pixel buys 1 559 pixels, and the note that
discouraged it has been replaced with this table.

**What bilinear did not fix**: the hosting banner mark still draws as a striped square where the reference has clean rings, so that is
not aliasing. It is the next thing to look at, along with the wordmark, which this build paints all in green where the reference sets
the word "modrinth" in white and "hosting" in green.
### The app was drawing its own stand-in logo (2026-11-09)

The hosting banner\'s mark was a **bright green square** and the wordmark was one green string. The reference draws Modrinth\'s
concentric rings and sets "modrinth" in white with "hosting" in green. Both are now right, and the logo is the real one:

| | before | after |
|---|---|---|
| the mark | a generated stand-in, a green square | **`packages/assets/branding/logo.svg` from `modrinth/code`**, rasterised at 512 |
| its size / position in the banner | 20 points at x 1152 | **16 points at x 1154** (reference: 16 at 1154) |
| its tone | brand green `(27,217,106)` | **`(63,196,118)`**, sampled from the reference\'s own banner |
| the wordmark | `modrinth hosting`, all green | **`modrinth` white, `hosting` green** |

| Home | differing pixels |
|---|---:|
| before | 31 310 |
| wordmark split | 31 133 |
| real logo + matched mark | **31 401** |

**The last step made the total 229 worse and the picture right.** The generated square happened to cover a different set of the
reference\'s ink pixels than the true rings do; the wordmark now agrees to a point vertically and five horizontally, and the mark is the
mark. That is the same trade this file has recorded before -- a metric that rewards a wrong shape because it overlaps -- and the state
that can be pressed and compared wins.

The logo is Modrinth\'s own asset, taken from the open-source repository the objective points at, with its provenance in a comment in
`assets/brand/logo.svg`.
### The text floor, measured three ways (2026-11-09)

The 99% target needs 12 960 differing pixels on a 1440x900 frame. Home sits at 31 401. This round asked whether the remainder is
something this build can fix, and measured the same regions three ways:

| region | area | differing | share |
|---|---:|---:|---:|
| blank card padding (no ink at all) | 8 400 | **22** | **0.26%** |
| the words "Getting started" | 3 200 | **789** | **24.7%** |
| the same heading, ink rows | -- | **identical**: 84..98 in both | -- |

So **the layout, the box, the size and the position of the text all match, and a quarter of the glyph pixels still differ**. That is the
rasteriser: Chrome and `ab_glyph` put different coverage in the same box.

Two attempts to close it, both measured and both reverted:

| attempt | belief | result |
|---|---|---|
| damp glyph coverage to 0.92 | our text averages 107.6 brightness where the reference averages 99.8 | **Home 31 401 -> 32 713**, worse |
| scale every face by 1.004 | the advance is 248 against 249 | net **+309** across five pages, worse |

The damping failed for a reason worth keeping: the brightness gap is in the **edges**, not in the glyph cores, so scaling all coverage
makes the strokes lighter than the reference rather than matching them.

**What this means for the objective, stated plainly**: a page whose ink covers 10% of its area cannot get below ~2.5% differing, because
that 10% differs by a quarter no matter what this build does. 1% is reachable only on pages that are almost entirely image and chrome --
the Gallery is the closest and it is at 12.7%. The number is a property of having a different text engine, not a list of things left to do.

**The work that remains can still move it**: every non-text difference is measurable and fixable, and this file has the list. But the
floor has to be reported rather than chased.
### Letter positions: the drift is real, and it is the advance (2026-11-09)

The 24.7% figure for a text row is not soft edges -- it is letters in different places. Measured stem by stem on the words "Getting
started" (16-point, semibold), the reference\'s stems and this build\'s:

```
ref  : 1167 1179 1190 1197 1203 1207 1213 1217 1231 1241 1247 1257 1265 1272
ours : 1167 1179 1189 1195 1200 1205 1210 1214 1229 1238 1243 1252 1260 1265
delta:   0    0    1    2    3    2    3    3    2    3    4    5    5    7
```

**Same start, same ink height, and a drift of seven points by the fourteenth stem.** That is not antialiasing and not a box; it is the
accumulated advance being a little short, glyph by glyph.

The heading was set in Inter 600; the reference\'s is heavier, so it is **700** now:

| | Home | drift by stem 14 |
|---|---:|---:|
| semibold 600 | 31 401 | **7 points** |
| **bold 700** | **31 339** | **5 points** |

It helped and it did not cure, and the reason is worth stating: the remaining five points over fourteen glyphs is about a third of a point
per glyph -- **sub-pixel advance**, which is what a layout engine rounds and a browser does not. The same measurement on the 24-point
project title found a difference of one point in 249, so the error scales with the size of the type, not with the string.

**This is the floor from the inside**: the ink is in the right place to within a pixel and a half, and every stem after the first drifts
by a fraction of one. Nothing in this build can round the same way as Chrome.
### The Add filter control: right shape, two wrong attempts, reverted (2026-11-09)

The library\'s filter row differs structurally and I could see it plainly:

| | reference | ours |
|---|---|---|
| the row | two dropdown chips, a standalone **funnel**, a vertical rule, then an **outlined `+ Add filter`** | three identical chips, the third with a chevron |

Two attempts to draw it, both measured, both worse than leaving it:

| attempt | Home |
|---|---:|
| three chips (as it was) | **31 339** |
| rule + outlined button, third chip removed | 31 547 (**+208**) |
| ... plus the standalone funnel, button moved 14 points right | 31 581 (**+242**) |

Both were reverted. The reason is not that the shape is wrong -- it is right -- but that **I could not measure the reference\'s button** to
place it: two bbox probes over the region both returned their own crop edges (`455..699`), which is the same instrument failure this file
has recorded before, and a hand-guessed rectangle that is 10 points out costs more than a wrong chip that happens to overlap.

**The way to do this one is not a probe at all**: the reference sizes that button from its own contents (`px-3 py-1` plus a 16-point plus, a
gap and the label), so the width should be *derived from the laid-out text* the way every other button in this build is, not guessed from a
crop. That is a half-hour job with the right instrument, and it is written down here instead of being attempted a third time blind.
### The gallery\'s crop is right, and my eyes were not (2026-11-09)

The Gallery\'s pictures looked about ten per cent more zoomed than the reference\'s, which reads exactly like a `cover` fit gone wrong. A
probe inside `cover_uv` says otherwise:

```
cover_uv texture [350.0 197.0] into [332.0 166.0] scale 0.94857144 shown [1.0 0.9]
```

**A 350x197 source (16:9) covered into a 332x166 box shows 90% of its height and all of its width** -- and the reference, covering a
full-size 16:9 source into the same box, shows the same 88.8%. The crops agree to a pixel; what differed was the two screenshot crops I
was comparing, taken at y 265 and y 267.

**Two wrong instruments in one round**: a bbox probe that kept returning its own crop edges, and a crop pair whose offsets differed by the
amount I was trying to explain. Both are the same mistake in different clothes -- measuring a thing without first pinning the frame it is
measured in. The probe was worth building: it turned "looks zoomed" into two numbers that agree.

The Gallery\'s 164 485 is therefore its **text**, like every other page, and the picture\'s softness against the reference is the difference
between downscaling a 350-point texture and downscaling a 1920-point one -- which the 20 MB memory clause forbids us from matching.
### The library page had no context menu, and its "+ New instance" button was dead (2026-11-09)

Found by pressing: a right press on the library background changed **nothing** in this build, where the reference opens a one-item
menu anchored at the pointer. Measured by diffing each client against its own idle frame:

| | reference | ours before |
|---|---|---|
| change after a right press at (150,350) | menu at **(149,349)-(327,407)** | **no change at all** |
| its content | `+ New instance` | -- |

It is implemented now, and pressing it exposed a second, older bug:

| step | result |
|---|---|
| right press on the library page | menu opens at the pointer, panel 178x58, `+ New instance` (`pgmenu-our.png`) |
| Escape | closes (`pgmenu-esc.png`) |
| pressing the row | **opens the "Create instance" dialog** (`pgmenu-created.png`) |

**The dialog did not open the first time.** `state.open_create` was set by the toolbar\'s `+ New instance` button and by the new menu, and
**nothing in the tree ever read it** -- the toolbar\'s button had been dead for as long as it existed, and the only working route was the
hero\'s own `Action::Create`. The flag is now consumed at the end of `list`, which fixes both: one field, two buttons, three rounds of
"the press did nothing" that were a press doing nothing.
### The filter row\'s chips were painted, never interactive (2026-11-09)

Pressing them did nothing, and the reason was in the code: the loop that draws "Last played", "Custom group" and "Add filter" had
**no `interact` call at all**. They were decoration that looked like controls. The sort chip now opens the reference\'s menu:

| | reference | ours |
|---|---|---|
| the menu | seven rows under the chip, current one green: Name, Last played, Hours played, Date created, Date modified, Loader, Game version | **same seven, same order, same green row** (`sort3-our.png`) |
| choosing a row | chip\'s label becomes the key | **chip reads `Name`** (`sort-pick2.png`) |

**Three bugs were stacked on one press**, and each was found by driving rather than reading:

1. the chip had no interaction;
2. the menu was drawn beside the chip, so the first card **painted over it** -- the clicks worked and nothing appeared;
3. the same click that opened the menu **closed it in the same frame** (the chip is outside the panel), and once it did draw, a row
   press **fell through to the card underneath** and opened the instance instead of choosing the sort. The panel now claims its own
   rectangle.

The label also drew empty at first: `#[derive(Default)]` gives a bare `&'static str` the empty string, so the chip showed only its glyph
and chevron. It is an `Option` now, with `None` meaning the reference\'s default, `Last played`.

**Not done**: the order of the cards is unchanged. With one instance there is nothing to reorder, so the comparator would be untestable --
the menu, the selection and the chip are real and driven, and the ordering is the next slice, with a second instance to prove it.
### The sort chip orders the cards, proved with two instances (2026-11-09)

The menu opened and the chip changed; the list did not. Now it does, and the proof needed a second instance:

| the library | `Last played` | `Name` |
|---|---|---|
| Fabric 26.3 (stamped `lastLaunchTime`) and Aardvark (never played) | **Fabric first** (`ord-last2.png`) | **Aardvark first** (`ord-name2.png`) |

The two keys disagree, so the order is being computed rather than looked up -- and the same pair in the first attempt agreed, because
**neither instance had a last-played time**: the default sort fell through to its name tie-break and both orders looked identical. The
instance was not "sorted wrong", it was not being told apart.

The comparator is `ordered(&mut shown, key)` with two tests: one asserts that two instances whose name order and last-played order
disagree come out in opposite orders, the other that a key with no data behind it (`Hours played`) leaves the default order alone.
**Three of the seven keys have no data in `Entry`** -- hours played, date created, date modified -- and they fall back to the
reference's own default rather than inventing an order; that is written in the function, not left to be discovered.

The lab was put back as it was found: Aardvark removed and the stamp deleted from Fabric's cfg, so every capture measures one instance
again.
### The second chip: five groups, named after the reference\'s (2026-11-09)

The "Custom group" chip was the third dead control in the filter row (after the sort chip and the add-filter chip, which was dead by
drawing rather than by wiring). It opens the reference\'s menu now:

| | reference | ours |
|---|---|---|
| the menu | 187 wide, five rows: Custom group, Instance type, Loader, Game version, No grouping | **same five, same order, current in green** (`grp-our.png` vs `ref-groupmenu.png`) |
| choosing | the chip takes the row\'s name | **chip reads `No grouping`** (`grp-pick.png`) |

**What this does not yet do**: it does not group. The chip\'s label and the menu\'s green row are real and driven, but the cards are
still one row with no section headers, so `populated -- group header` (row 32) and `empty -- empty group message` (row 53) stay OPEN --
correctly. A menu row that changes a label and nothing else is exactly the kind of control this round was spent finding; the difference
is that this one is written down as half-done instead of counted as done.
### Grouping draws a header now, and moves the grid to make room (2026-11-09)

The second chip named a grouping and drew nothing. The header is drawn now, and the cards move down by its height:

| | reference (grouped by Instance type) | ours |
|---|---|---|
| the header | chevron, the group\'s name, its count in a pill, a rule under it | **same**, "Custom" and "1" (`gh2-our.png` vs `ref-grouped.png`) |
| the cards | start below the rule | **start below the rule**, and pressing one still opens the instance (`gh-click.png`) |

Two mistakes were made and both were visible immediately in a crop: the header was **drawn over the filter chips** (the grid\'s top was
subtracted twice), and the count pill was placed at a fixed 100 points, which ran into any name longer than "Custom". The pill follows the
laid-out name now.

**What is still wrong, and is now its own OPEN row**: the header is drawn **once for the whole list** and names the first entry. Two
loaders would share one header where the reference draws two sections. That is recorded as a new row rather than left implied by a
"done" on the row above it -- the row that closed is the header itself, which was verified against the reference.
### Grouping draws one section per group, and three mistakes came out of it (2026-11-09)

With a Vanilla instance beside the Fabric one, grouping by Loader now draws **two sections**, each with its own chevron, name, count
in a pill and rule, and each restarting the grid in column 0 (`two-groups3.png`). The default page is unchanged by any of it:
**Home still measures 31 339** with the same per-slab split, because the ungrouped path passes no shift.

Three separate mistakes on the way, each visible only when driven:

| mistake | what it looked like | why |
|---|---|---|
| the drawing loop still used the **global** index | the second group\'s card appeared in **column 1** | only the probe\'s call site had been converted |
| the tables were declared **after** the probe | a compile error, then a probe reading an empty table | blocks spliced in the wrong order |
| a group\'s cards started **at** its header | the card painted over the heading -- the header was there, underneath | the shift was the group\'s offset, without its own header\'s height |

The last one is the same failure as the sort menu two rounds ago, in a different costume: **a thing drawn correctly and then covered by
the next thing drawn**. It is worth naming because the fix is always the same -- give the covered element its own space -- and because
the symptom (nothing appears) is identical to the symptom of not drawing it at all.
### The group chevron folds its section (2026-11-09)

It was drawn and did nothing -- the exact shape this sequence of rounds keeps finding. Now:

| | before the press | after |
|---|---|---|
| the Vanilla section | chevron down, "Vanilla 1", its card under it | **chevron right, no cards, the subsection folded** |
| the Fabric section | below it | **moved up** into the space the fold freed |

The press target is the whole header, not the 16-point glyph, because the reference\'s row is the control. A folded group takes only
its header\'s height in the layout, so the sections below shift by exactly 208 + 12 rather than leaving a hole.

The default page is unchanged by any of it: **Home still measures 31 339**, same per-slab split.

**Still open**: the chevron\'s rotation is a different glyph rather than a rotated one (they agree at 16 points), and there is no
animation on the fold -- the reference\'s sections open and close over ~150 ms, which this build does not do anywhere yet.
### The first animation in this build, and it was found by pressing (2026-11-09)

Until now the fold was a jump: the cards vanished at a midpoint and everything below teleported. It is a 150 ms tween now, which is
what the reference\'s sections take, and the evidence is a capture **40 ms into it**:

| | 40 ms after the press | settled |
|---|---|---|
| the folded section | its card **clipped part way**, the chevron already pointing right | gone, header only |
| the section below | **part way up** | at the top |

The layout reads a per-group progress `0..1`, the section\'s height is `rows * 220 * (1 - progress)`, and its cards are drawn through a
painter clipped to that band, so they slide out rather than disappear. Frames are requested while any progress is short of its target,
which is also why the mid-fold capture could catch it at all: an idle page paints at 2 fps and would have jumped straight to the end.

**A fold now belongs to its key.** It was keyed by group name alone, so a "Fabric" folded under `Loader` would arrive folded under
`Game version`, where the name means something else; the set is cleared when the grouping changes.

This is the **only** animation in the build. The reference has fades on menus and page transitions, and this work has not touched them:
the mechanism is now here (progress, dt, repaint-while-moving) and the next one is cheaper than this was.
### The five-size sweep, taken again -- and the captures that were lying (2026-11-09)

The objective asks for verification at five sizes. Re-running it produced a result that was too good, and finding out why was the
useful part.

**The trap.** A set of captures named `rs-<WxH>.png` had been filed as reference captures. Comparing our current build against them
gave **1 827 differing pixels at every single size** -- an identical total for five different frames, which is not a thing that happens.
They are **this build\'s own earlier captures** (from a resize sweep of *our* app). The comparison was our app against itself, and the
1 827 is what eight rounds of UI work moved.

**The real sweep**, against the reference captured live at each size minutes ago:

| size | differing | pixels | share |
|---|---:|---:|---:|
| 1280x720 | 46 230 | 921 600 | **5.02%** |
| 1366x768 | 50 525 | 1 049 088 | **4.82%** |
| 1440x900 | 51 333 | 1 296 000 | **3.96%** |
| 1600x900 | 59 722 | 1 440 000 | **4.15%** |
| 1920x1080 | 73 558 | 2 073 600 | **3.55%** |

These are **worse than the figures this file carried** (27 619 / 35 286 / 31 255 / 45 259 / 57 062) because those were taken against
earlier states of the reference and of this build, and the earlier numbers were never re-taken. The current table is the one to quote.

**Two things were ruled out as causes on the way**, each by measuring rather than by guessing:

| candidate | test | answer |
|---|---|---|
| the reference\'s news carousel rotating between captures | three captures 4 s apart, compared pairwise | **0 differing** -- it is static |
| the news artwork drawn in the wrong colour | five pixels probed in both | **~6/255 apart** -- same gradient, ours a shade brighter |

What is left is the text, which is what every previous round found: the sidebar\'s 16 818 at 1440x900 is the news card\'s title and body,
whose line breaks differ by a word.
### Correction: the sweep two paragraphs up compared a grouped reference (2026-11-09)

The table above is **wrong and is superseded by this one**. Its reference captures were taken with the reference still grouped by
`Instance type` -- a state left over from driving its Custom group menu earlier in the session. A grouped reference and an ungrouped
build differ by every group header, every shifted card and a rule across the content column, which is most of what the numbers were
measuring.

It was found by looking at the band with the most differences and seeing a **"Custom 1" header** in the reference crop that this build
does not draw -- rather than by re-reading the numbers, which looked plausible.

**The reference was reset to `Custom group` and all five sizes re-taken**, both sides ungrouped:

| size | differing | pixels | share |
|---|---:|---:|---:|
| 1280x720 | 31 775 | 921 600 | **3.45%** |
| 1366x768 | 39 444 | 1 049 088 | **3.76%** |
| 1440x900 | 35 479 | 1 296 000 | **2.74%** |
| 1600x900 | 49 413 | 1 440 000 | **3.43%** |
| 1920x1080 | 61 295 | 2 073 600 | **2.96%** |

These agree with the shape of the older figures (27.6k / 35.3k / 31.3k / 45.3k / 57.1k) to within the drift of nine rounds of work, which
is the check the wrong table failed: it was 40-50% higher everywhere, and nothing in those nine rounds did that.

**The rule this breaks is the oldest one in this file**: leave the lab as found, and check both screens before measuring. The reference
was left grouped, and the sweep was taken without looking at it.
### The filter row, measured exactly, drawn exactly, and worse (2026-11-09)

The content column\'s worst bands were the filter row, so it was measured from a clean live reference by ink runs per column:

| element | reference (x) | this build before |
|---|---|---|
| chip 1 "Last played" | **88..252** (165 wide) | 88..205 (118) |
| chip 2 "Custom group" | **255..453** (199) | 214..377 (164) |
| third control | a **funnel at 464**, then an **outlined `+ Add filter` to 630** | a third 122-wide chip |
| chip label ink, rows | **306..320** (15) | 308..319 (12) |

All of it was rebuilt to those numbers -- chips 165 and 199 wide with 16-point labels, a funnel at 464, an outlined button to 630 -- and the
result measured **worse, twice**:

| | before | after |
|---|---:|---:|
| Home at 1440x900 | 32 265 | **32 735** |
| the filter row\'s own band (80..700 x 284..332) | 3 186 | **3 724** |
| the cards band, as a control | 2 388 | **2 388** |

The cards band is identical to the pixel, so nothing leaked; the regression is inside the row that was made more accurate. Its chip labels now
sit on **exactly** the reference\'s ink rows (306..320 both) and its chips have **exactly** the reference\'s widths -- and the band still scores
538 worse. So the remaining difference is in what is *inside* those correctly-sized chips: the glyph, its inset, the chevron\'s, and the
outlined button\'s border landing on x 629 where the reference\'s is on 630. Matching every dimension that can be measured from ink runs
does not match the drawing inside them, and this round did not have the instrument to see that.

**Reverted**, and the tree is back to 32 265 with the cards band unchanged. The measurements above are the useful part and are recorded so the
next attempt starts from them rather than from a crop.

**Two lab lessons from the same round**, both about the reference\'s own state:

- a chip that has been clicked keeps a **magenta focus ring** -- (255,160,255) around the chip -- through clicks elsewhere and pointer moves,
  so a capture taken after driving its menus is not the default state;
- the ring clears only on a **restart** of the reference, which needed its real process name (`ModrinthApp`, not `AppRun`); several zombie
  entries make `pgrep` look like it is still running when the live one is a different pid.

`refclean2.png` is the clean 1440 reference from that restart: it differs from `h1440-ref.png` by **925 pixels**, which is the check that it is
the same state and worth comparing against.
### The filter row: a control that is visibly wrong and measurably better (2026-11-09)

At 4x the chip interiors are plainly different, and every difference was then measured and corrected:

| | reference | this build |
|---|---|---|
| label ink | **306..320**, sampled **pure white (255,255,255)** | 308..319, **(176,186,197)** |
| chip height / top | **40 points from y 288** | 32 from y 290 |
| chip widths | **165 and 199** | 118 and 164 |
| third control | funnel at 464, outlined button to 630 | a filled 122-wide chip |

All of it was applied -- 16-point white labels, 40-point chips at 288, the reference\'s widths -- and it measured **worse a third time**:

| attempt | filter-row band (80..700 x 284..332) | Home total |
|---|---:|---:|
| original (visibly wrong) | **3 186** | **32 265** |
| widths + 16-point labels (grey) | 3 724 | 32 735 |
| + white labels, 40-point chips | **3 785** | **32 876** |

Each correction moved the drawing *towards* the reference and the band *away* from it. The explanation is in the geometry: the reference\'s
chip fills are (52,54,60) on a (22,24,28) page, a 30-point step -- **under the 40 threshold**. Where this build draws no chip, the reference\'s
chip does not count as a difference. Where this build draws a chip that is 32 tall and the reference\'s is 40, the *overlap* is free and only
the 8-point fringe counts. **Making the chip the right size enlarges the region where two slightly-different fills must agree**, and a fill
that is 2-3 points off -- invisible at this threshold on its own -- becomes a counted difference across the whole enlarged area.

So the threshold-40 metric **cannot see** this row\'s most obvious errors (grey vs white text is 79 points, and does count -- but the text
occupies a fraction of the chip) while it **punishes** fixing the ones it cannot see. Three attempts is enough to establish that: the row is
reverted, and it is recorded here as a place where the objective\'s own measurement and its objective\'s own intent disagree.

**What would settle it**: a per-element comparison (the GAP-MATRIX\'s own idea) that checks the chip\'s colour, height and type against the
reference directly, rather than a whole-frame pixel count that cannot see them. That is a different instrument from `compare.py` and this
round did not build it.
### A per-element instrument, and the row it settles (2026-11-09)

`tools/parity/element.py` compares **one rectangle** between a reference and a build: its fill, its brightest pixel (the ink), the rows that
carry ink, and the rows its fill covers. It answers what a whole-frame threshold-40 count cannot: what colour is it, and how big.

Its first verdict, on the sort chip:

| property | reference | before | after |
|---|---|---|---|
| fill | (52,54,60) | (52,54,60) | (52,54,60) |
| ink | **(255,255,255)** | (176,186,197) -- off by **79** | **off by 0** |
| ink rows | 305..320 (16) | 306..319 (14) | **305..320** |
| chip fill rows | 296..327 | 291..320 (-5) | 297..326 (+-1) |

and on the control after the two chips:

| | reference | the old filled chip | the outlined button now |
|---|---|---|---|
| brightest | (255,255,255) | **off by 105** (grey) | **off by 0** |
| ink rows | 293..330 (19) | 310..312 (3) | 296..327 (15) |
| extent | 288..331 | 288..331 | 288..331 |

**And the whole-frame count went up, not down: 32 265 -> 32 864.** The arithmetic says why, and it is worth writing down because it
explains four reverted rounds:

- this build\'s chip fill is (52,54,60) on a (22,24,28) page: a **30-point** step, under the threshold of 40, so a chip drawn where the
  reference has none **does not count**;
- the reference\'s outlined button draws a 1-point border at (66,68,74): a **44-point** step, over the threshold, so drawing the *correct*
  border **does count** -- and counts against the alternative, which was a filled chip that overlapped the same area invisibly.

So `compare.py` scores a filled chip above the reference\'s own outlined button, and always will at threshold 40. **The correct drawing is
kept**, both numbers are recorded, and the row is now judged by the instrument that can see it. The whole-frame figure is the one the
objective names, so it is quoted honestly: **it is 599 worse, and it is right**.

What that implies for the objective: any element whose fill step from the page is under 40 cannot be measured by the frame count, and
**19** of the surfaces in this theme are within 40 of the page -- so the frame figure has a floor set by what it cannot see, independent of
how close the drawing gets.
### The same row at three thresholds, and a correction (2026-11-09)

The filter row\'s correction was kept while the frame count said it was worse. Both captures were then measured against the same clean
reference at three thresholds:

| threshold | three chips (wrong shape) | funnel + outlined button (correct) | winner |
|---|---:|---:|---|
| **40** -- the objective\'s | **32 265** | 32 864 | the wrong one, by **599** |
| 20 | 54 562 | **50 298** | the right one, by **4 264** |
| 10 | 90 901 | **86 356** | the right one, by **4 545** |

So the correction is a real improvement of about **4 300 pixels**, and threshold 40 inverts the ranking. That is not an argument about
which number to like: it is the same measurement with the threshold moved, and the ordering flips.

**Correction to the previous entry.** It said "19 of the surfaces in this theme are within 40 of the page". Counted from `theme.rs` for
the dark palette -- the first count was polluted by the light one, whose page tone is (235,235,235) -- there are **12 distinct tone steps**
under 40: 0, 4, 7, 10, 13, 15, 16, 18, 20, 22, 28 and 32, shared across about twenty token names. The claim was directionally right and
numerically wrong, and it is the number a reader would have quoted.

**What to use from here**: `tools/parity/element.py` for any single element (it sees colour to the point), `compare.py` at 40 for the
objective\'s figure as written, and `compare.py --threshold 20` when the question is "is this frame getting closer", because at 20 the
measure is monotone with the drawing being right on the rows this file has tested.
### The five-size table, third time, from a restarted reference (2026-11-09)

The two earlier tables in this file were both contaminated. The first compared a grouped reference with an ungrouped build; the second
was taken from a reference that still carried the **focus ring** a click leaves on a chip, which costs about 2 600 pixels at every size.

The reference was restarted, confirmed clean at the ring\'s own pixel, and all five sizes re-captured. The current build:

| size | differing @40 | share | differing @20 | share |
|---|---:|---:|---:|---:|
| 1280x720 | **29 153** | **3.16%** | 46 319 | 5.03% |
| 1366x768 | **36 825** | **3.51%** | 54 110 | 5.16% |
| 1440x900 | **32 863** | **2.54%** | 50 298 | 3.88% |
| 1600x900 | **46 795** | **3.25%** | 66 583 | 4.62% |
| 1920x1080 | **57 752** | **2.79%** | 98 380 | 4.74% |

These are the lowest figures this file has recorded at all five sizes, and they include the corrected filter row whose 40-threshold cost
(+599 at 1440) is visible in the table as the difference between the two thresholds: at 20 the same frame is 4 264 better than the
uncorrected one.

**The rule, now with a third instance behind it**: *restart the reference and check a known pixel before any sweep*. A click leaves the
ring, a click also leaves a grouping, and both survive everything short of a restart. Every "the numbers moved" episode in this file is
one of those two.
### The pointer is a state too, and the rail is not where this file thought (2026-11-09)

Comparing Discover at 1440 gave **80 453**, then **78 984** after nothing but moving the reference\'s pointer from (50,860) to (700,880) --
the same place this build\'s pointer was parked. The 1 469-pixel difference was a **hover state** in the reference\'s sidebar, on an element
this build was not hovering because the pointer was somewhere else.

That is the fourth member of the same family as the grouped reference and the focus ring: **the pointer position is part of the state**, and
two captures taken with it in different places are not comparable. The rule now reads: restart the reference, check the ring pixel, park
both pointers on the same neutral point (700,880), and only then capture.

Clean Discover, both thresholds:

| page | @40 | @20 | previously recorded |
|---|---:|---:|---:|
| Discover modpacks | **78 984** | 119 597 | 77 486 / 78 059 |

in line with what was recorded, so the recent work did not move it.

**The rail map, measured rather than assumed.** Clicking the reference\'s rail at y 123 gives *Discover modpacks* and at y 178 gives the
*Skin selector* -- which is not the order this file had been driving (Home, Discover, Screenshots). Each page needs its icon identified
by pressing it and reading the title, not by counting down a list:

| rail y (1440x900) | page |
|---|---|
| 123 | Discover modpacks |
| 178 | Skin selector |

The rest are unmapped and will be found the same way, one press at a time.
### The sidebar costs 16 816 on every page, and the pages beneath it are closer than their totals say (2026-11-09)

Screenshots, measured with the pointer parked on both sides and the reference restarted:

| page | total @40 | titlebar | rail | **content** | **sidebar** |
|---|---:|---:|---:|---:|---:|
| Home | 32 863 | 1 693 | 397 | 13 957 | 16 816 |
| Screenshots | **23 573** | 2 107 | 426 | **4 224** | **16 816** |

**The sidebar is the same 16 816 on both pages, to the pixel.** It is shared chrome: the "Getting started" card, the news card and the hosting
ad are drawn identically whichever page is open, so its difference is a **fixed cost carried by every page measurement** in this file. Of
Screenshots\' 23 573, the page\'s own content is 4 224 -- 0.8% of the content slab, which is the closest any page-area has measured.

That reframes the whole table: the numbers this file has been comparing page-to-page are dominated by a constant, and the pages are being
judged on the smaller part. A 4 224 content difference and a 13 957 one are the real page figures; 23 573 and 32 863 are those plus the
same sidebar.

**The sidebar was then audited with `element.py`, box by box** (`shots-ref.png` against `shots-our.png`):

| element | fill | brightest | ink rows | extent |
|---|---|---|---|---|
| "Getting started" heading | off by 1 | **off by 0** | **off by 0** | **off by 0** |
| news title | off by 1 | off by 1 | -- | **off by 0** |
| news body | off by 1 | **off by 0** | +0/-1 | **off by 0** |
| first step row\'s text | off by 1 | **off by 107** | -- | -7 bottom |

So the sidebar\'s colours and positions match to a point or two: its 16 816 is **text**, wrapped a word differently, exactly as the earlier
floor work found. The one outlier is the first step row, whose text is 107 darker than the reference\'s -- that row is the struck-through
"Create first instance", and this build dims it more than the reference does. That is a real, fixable difference and it is the next thing
here.

**The rail map**, extended by pressing: y 123 Discover modpacks, y 178 Skin selector, **y 205 Screenshots**. The icons read, top to bottom:
play, compass, t-shirt, picture, two stacked rectangles, then the instance\'s own icon, then `+`.
### The completed step, dimmed to the reference\'s own tone (2026-11-09)

`element.py` said the first checklist row\'s ink was **107 too light**, and the wide box said the whole row had nothing brighter than the
reference\'s (61,68,69). Both were the same thing seen twice: this build drew a finished step in `SECONDARY` (150,162,176) where the
reference uses something near `SURFACE_5` (66,68,74) -- for the label **and** the tick\'s disc, not just the label.

| | reference | before | after |
|---|---|---|---|
| label ink (brightest) | (61,68,69) | (150,162,176) -- **off by 107** | **(66,75,73) -- off by 7** |
| the disc | dim | `SECONDARY` | `SURFACE_5` |

**And the frame did not move at all**: Home is 32 864 at threshold 40 and 50 298 at 20, identical to before. One row\'s ink and one 8-point
disc are about two hundred pixels, most of which were already counted on both sides. The element instrument improved; the frame metric
could not see it. That is the same asymmetry this file recorded for the filter row, in the opposite direction.

**What is not verified**: the disc\'s position. Sampling its expected pixel (1192,114) in this build returns the row\'s fill, so the disc is
drawn somewhere else -- the 2x crop shows a light disc where the reference has a dim one, and after the change it should be dim in both.
Either the disc moved with the label\'s new inset, or that pixel belongs to the label\'s first letter. One probe would settle it and this round
ran out before it was pointed at the right coordinates.
### Two "Getting started" cards, and six points that would not move (2026-11-09)

Last round\'s note said the ticked row\'s ink was 107 too light and that the disc\'s position was unverified. Both were wrong in the same way:
**the measurement box straddled two rows.** Scanning the row\'s area for its brightest pixels showed the culprit at y **135** -- the *second*
row\'s unticked ring -- not in the completed row at all. With the box corrected the completed label matched to 7, before any change.

Then the real difference: sampling a column down the card gave the reference\'s completed label at **y 114** and this build\'s at **120**.

**And there are two implementations of that card.** `panel.rs` has `getting_started_card` (used by a project page) and a second, larger
routine that Home actually draws. Three edits went into the first one and changed nothing on screen -- a no-op build looks exactly like a
broken knob, and this is the second time this file has recorded that.

The right one uses `plan.steps_first` for its rows:

| attempt | what moved | Home @40 |
|---|---:|---:|
| baseline | -- | **32 864** |
| `steps_first - 6` (rows up, news held still) | the label reached y 114, **exactly the reference\'s** | 33 372 (+508) |
| reverted the rows, moved the label to `row.center().y - 7` | **nothing moved** -- it is another label call | 33 331 (+467) |
| reverted everything | -- | **32 864** |

So aligning the label\'s ink to the reference\'s exact row costs 508 pixels, because the row\'s **fill** moves with it and the fill was what
matched. The label sits six points lower inside its row here than in the reference, and only the label can move -- third time in this file
that a metric prefers a misaligned element to an aligned one.

**Reverted.** The tree is at 32 864 / 50 298, the numbers it had before the round, and the six-point offset is recorded rather than left as a
half-made change in a card with two implementations.
### The trap has a name now (2026-11-09)

Two rounds were spent editing a function that Home never calls. Both implementations are live -- Home\'s is inside `panel::draw`, the
project page\'s is its own function -- and the comment beside the project one claimed they were shared, which is exactly the sentence that
made the mistake reasonable.

| | before | after |
|---|---|---|
| the function | `getting_started_card` | **`project_getting_started_card`** |
| its doc | none | says which page draws it, that Home\'s is a **different implementation**, and that two rounds were lost to this |
| the call site\'s comment | "the card itself is the shared one in `panel`, so the project and Home sidebars cannot drift apart again" | states that they are two implementations **and they can drift** |

Home is unchanged by the rename: **32 864** at threshold 40, the same per-slab split, and 266 tests pass.

**Not done**: the rename makes the trap visible, it does not remove it. The two cards still compute their row positions from different
expressions (`card.top() + 52` against `plan.steps_first`), so they can still drift; the honest fix is one row-drawing routine called by
both, and that is a refactor rather than a rename. What this round buys is that the next person reading either one knows there are two.
### A test for the two cards, and the two things it found immediately (2026-11-09)

The rename made the duplicate visible; a test makes it **pinned**. Both cards\' row geometry is now constants -- Home\'s
`HEADING_LINE + HEADING_TO_BODY`, the project card\'s `PROJECT_STEPS_TOP` and `PROJECT_ROW_PITCH` -- so an edit to either fails a test
instead of hiding in a page nobody screenshotted.

It failed on its first run, twice over:

| | Home | project card | reference |
|---|---:|---:|---|
| first row below the card\'s top | 39 | **52** | -- |
| row pitch | 48 | **44** | **48** |

The pitch was the fixable one. Measured from the reference\'s own sidebar -- three text bands at y 141, 189 and 237 -- its pitch is **48**,
which is what Home already used; the project card was the only thing in the build pacing its rows 44 apart. It is 48 now, and the test
asserts both that the two agree **and** that the number is the reference\'s.

The first-row offset is left as it was, now with the test saying so out loud: Home starts at 39, the project card at 52, and the reference
has not been measured for that page yet. Pinning a divergence is not fixing it, but it cannot now be mistaken for agreement.

Home is unchanged: **32 864** at threshold 40, same per-slab split. **267 tests** pass, one of them new.
### The constants were dead, and the test said they were fine (2026-11-09)

Last round\'s entry ended with a risk: "a passing test may be read as agreement". It was worse than that -- the test asserted two
constants that **nothing read**. `PROJECT_STEPS_TOP` and `PROJECT_ROW_PITCH` were defined, the test compared them, and the drawing
still said `card.top() + 58.0 + index as f32 * 44.0`, unchanged. Measured on the running build: the project sidebar\'s rows were still
44 apart after a commit that claimed 48.

Both are wired in now, and the test asserts the **drawing expression itself** (`card.top() + PROJECT_STEPS_TOP`,
`index as f32 * PROJECT_ROW_PITCH`) rather than the constants, so a dead constant cannot pass again.

The pitch fix is verified on screen this time:

| | reference | before | after |
|---|---|---|---|
| the project sidebar\'s row bands | 141, 189, 237 | 186, 230 (**44** apart) | 184, 232 (**48** apart) |

**What is still wrong**: the rows are 43 points too low (184 against 141) while both cards\' headings sit at y 85. So the offset is not the
heading and not the pitch -- it is the distance between them inside the card, which on this page is 99 here against 57 in the reference.
`PROJECT_STEPS_TOP` is 52 and the heading inset is 13, which should put the first row about 39 below the heading, not 99. Something between
the heading and the row loop is adding ~40 points that this round did not find.

The card\'s height was left at its tested value: changing it with the pitch broke the project page\'s own test at 204 against the
reference\'s 178, so the height and the pitch are now pinned separately and the height\'s own 26-point error is recorded here rather than
folded into this change.
### The project sidebar is 7.6% different all over, and the "43 points" was a bad scan (2026-11-09)

Last round\'s entry said the project sidebar\'s checklist rows sat "43 points too low", from a scan that found text bands at 184 and 232
against the reference\'s 141 and 189. On screen the two sidebars look closely matched, and a direct measurement says why the scan was wrong:

| measurement | value |
|---|---|
| the project sidebar\'s differing pixels (1140..1440 x 40..900) | **19 559 of 258 000 -- 7.6%** |
| where they are | the **whole column**: bbox (13,11)-(298,691), worst 20-row bands at offsets 160, 200, 280, 360 and 500 |

So the differences are spread across every card -- Getting started, Compatibility, the links -- and not concentrated in the checklist\'s rows at
all. The band scan that produced "43" restricted itself to x 1210..1340 and a brightness threshold that caught the second and third rows in one
image and the Compatibility card\'s chips in the other; the two numbers were measuring different objects, which is the third time in this file
(after the two-rows box and the crop offsets) that a narrow window has produced a confident, wrong number.

**Corrected**: the checklist rows\' position on the project page is **not** known to be 43 out. What is known is that the page\'s sidebar as a
whole differs by 7.6%, distributed across its cards, and that is the number to work from. The pitch of 48 is verified separately, by band
spacing, and stands.

The lesson is the same one this file keeps relearning from a new angle: **measure the thing you name, at the size of the thing you name**. A
window narrower than the element produces a number about part of it, and a number about part of an element is indistinguishable from a
measurement of the element until somebody crops it.
### The card attributed, measured at 1:1, and a height the counts and the crop disagree about (2026-11-09)

Last round said the project sidebar\'s 7.6% was "spread across every card". Attributed by contiguous runs of differing rows, it is not:

| run (y) | differing | what it is |
|---|---:|---|
| **52..309** | **8 185** | the Getting started card -- **42% of the column\'s total** |
| 311..336 | 1 659 | the Compatibility heading |
| 686..702 | 1 508 | the links row |

so the card is the thing to fix after all, and the correction is recorded rather than left standing. The mask of that region showed the
answer plainly: **every line of the card doubled**, heading and all three rows, which is one offset applied to the whole card\'s contents.

Measured at 1:1 against the reference -- card 54..272, first row 124, rows 34 tall, 48 apart:

| | reference | this build before | after |
|---|---|---|---|
| first row below the card\'s top | 70 | 52 | **70** |
| row height | 34 | 40 | **34** |
| row pitch | 48 | 44 | **48** |
| the column\'s difference | -- | 19 559 | **18 664** (-895) |

**The card\'s height is the one number the two instruments disagree about.** The crop says the reference\'s card is 218 tall (54..272); the
count prefers 234 by 1 017 pixels, because the cards below move with it and land closer. The count is taken, the crop\'s number is
recorded, and the constant carries a comment saying which is which -- this file has been wrong about crops more often than about counts,
but it has been wrong about both, and 1 017 pixels is not an argument.

Tests were updated to the measured numbers rather than the old ones: 70 for the first row, 234 for the returned height, and the geometry
test now asserts all of them, including that the drawing expressions mention the constants.
### The 20 MB gate, measured again, and the image cache ruled out (2026-11-09)

The gate had not been measured in a while. Built fresh, idle at 1440x900 with the pointer parked:

| | value |
|---|---|
| **VmRSS** | **22 500 kB -- 22.5 MB**, against a 20 MB gate |
| RssAnon | 5 592 kB |
| VmData | 18 068 kB |

Stable across three samples two seconds apart, so it is an idle floor rather than a warm-up.

**The image cache was the obvious lever and it is not the one.** Decoded textures cost cap*cap*4 bytes in anonymous memory, and the caps
were 640 for pictures and 160 for icons -- 1.6 MB and 100 kB per entry. Lowering them to 384 and 128 saved **112 kB**: 22 500 -> 22 388.

That number says the cache holds almost nothing at idle on this page -- a handful of icons -- and that the 5.6 MB of anonymous memory is
elsewhere: egui\'s own tessellation and texture atlas, the process\'s allocator, and the five-megabyte framebuffer that the software backend
needs for a 1440x900 window in 32-bit colour. Reverted, because 112 kB is not worth a single icon being resampled.

The gate therefore stands where it stood: **2.5 MB over, with the two biggest line items being the framebuffer this backend requires and the
code itself.** Neither is reachable by configuration; both were measured, and the numbers are here so the next round does not spend its time
on the cache again.
### The card\'s height: three instruments, three answers, none of them settled (2026-11-09)

This file has now recorded the project card\'s height as 218, then 234, then 214, and the running build has been each of those in turn.
Here is what each instrument actually measured, and why none of them can be trusted for this number:

| instrument | said | why it is not the card |
|---|---|---|
| a 1:1 crop | 218 | I read the bottom edge by eye against a dark page; the card\'s ring is 17 points from the page and the crop\'s last rows are the third **row\'s** ring |
| long light runs in a column | 215 | the scan finds the rows\' rings as readily as the card\'s -- on this build it reported the bottom at 269, which is inside the card |
| the whole-column pixel count | 234 | prefers whichever height puts the *other* cards closest, and cannot see a fill 17 points from the page at threshold 40 |

**All three are proxies.** The card\'s own rect is not visible as an edge distinct from its rows\' rings; the only reliable way to get it is to ask
the program, which is what `SIFT_HOME_PROBE` does for cards on Home and what nothing does for the sidebar\'s cards on a project page.

So the height is left at the value the column count prefers (**234 returned, 218 inner**) -- not because it is right, but because it is the best
measured state and because oscillating between three proxies has cost three rounds. The build is reverted to exactly that state.

**The instrument to build next**: a probe that prints the sidebar\'s card rects the way `SIFT_HOME_PROBE` prints the grid\'s. Then this number is
read rather than inferred, and the crop/ring/count argument ends.
### The probe that ends the argument, and what it says (2026-11-09)

`SIFT_PANEL_PROBE=1` prints the sidebar\'s cards as they are laid out, the way `SIFT_HOME_PROBE` prints Home\'s grid. On a project page:

```
panel card getting-started y=68.0 h=234.0 width=252.0
```

So this build draws the card at **y 68, 234 tall, 252 wide** -- which is the 234 the pixel count preferred, and neither the 218 of my crop nor
the 214 of my ring scan. Both of those were readings of a *neighbouring* row\'s ring, as the third entry in this file suspected.

**The instrument is the deliverable.** Three rounds went to crops, ring runs and column counts disagreeing about a number that the program knows
exactly and would have printed on request. The lesson is not new here -- "the rect comes from the program, never off a capture" is rule one of this
file\'s own drives -- but it had only been applied to Home\'s grid, never to the sidebar\'s cards.

**Still open**: the reference\'s card rect. Its pixels give a top ring at 48 and a bottom somewhere near 262, and our probe says ours is 68..302, so
the card is 20 lower and 20 taller than the reference\'s -- two round numbers that make one another plausible, and neither verified against the
reference\'s own program, which this build cannot run. What it can do is read the reference\'s edges where they are *unambiguous*: the card\'s ring
is a 1-point line above a 17-point fill, and a scan for a full-width run of that ring colour, rather than any light run, would find only it.
### The card\'s edges, found by the page background between cards (2026-11-09)

The ring scan found only the card\'s **top** edge (y 48, one full-width run); its bottom is rounded and never reaches full width, and the fill
sampled mid-card lands on a *row* rather than the card. What worked was the page background itself: scanning a column for runs of (22,24,28)
finds exactly the gaps between cards.

| | this build |
|---|---|
| page background above the card | **49..67** |
| the card, drawn | **68..269** (202 tall) |
| page background below it | **270..285** |
| what the probe said was laid out | y 68, **234** tall |

So the probe\'s 234 is the card plus the space it reserves; the card itself is drawn 202. The reference\'s card runs from y 54 to about 262
-- its rows sit at 124, 172 and 220 in both, so the rows agree while the card around them does not.

**The offset is the page\'s, not the card\'s**: `panel()` starts at `rect.top()`, and the rect the shell hands a project page starts at 68 where the
reference\'s sidebar content starts at 54. That is a 14-point difference above the first card, affecting every card on the page, and it is where
the next change belongs -- not in the card\'s own height, which the program says is right for where it is placed.

**Not changed this round**: the offset, because it comes from the shell\'s rect and that rect is shared with Home, whose sidebar currently
matches. Changing it without measuring Home first is how the last three rounds went.
### The project sidebar: stopped, at its best-measured state (2026-11-09)

Six rounds have now been spent on this column. The tally, all against the same reference capture and the same 258 000-pixel slab:

| change | difference |
|---|---:|
| the state as found (card at y 68, 202 drawn, rows 52 below, pitch 44) | 19 559 |
| rows to the reference\'s pitch of 48 and 34 tall, first row 70 below | **18 664** |
| card height 218 returned instead of 234 | 19 681 |
| card height 202 | 19 681 |
| the card moved up 14 to the reference\'s own top of 54 | 19 595 |

**Every change that moved the card towards where I measured the reference\'s card to be scored worse**, and the one change that helped was
the rows inside it. That is a pattern, not noise: the reference\'s card is not where I keep measuring it. The page-background scan that located
this build\'s card exactly (68..269) returns **no runs at all** on the reference\'s column, so the reference\'s cards do not have a
page-coloured gap between them and the method does not transfer.

**Decision: the column is left as it measures best** -- rows at the reference\'s pitch, card where the count prefers it -- and this is the last
round that will move it without a new instrument. What is missing is the reference\'s own card rect, and the only ways to get it are a probe
inside an app this build does not control, or an edge that is unambiguous in its pixels. Neither exists yet; three proxies have been tried and
all three disagreed.

Six rounds for one card, on a page whose total is 19 595 of a 1.3-megapixel frame -- 1.5%. The time was not well spent, and the record says so
rather than presenting the churn as progress.
### Escape did nothing, and now clears the selection (2026-11-09)

Driven with the card\'s ring at the coordinates the program prints (`home row=0 ... card=[[88,329]-[256,537]]`, ring centre 210,388):

| step | what changed |
|---|---|
| a press on the ring | **(199,377)-(221,399)** -- the 22-point circle, and nothing else |
| `Escape` | **nothing at all** |

So the ring toggled correctly and Escape was not wired to the selection. It is now: the same 22-point box changes back, and the frame after
Escape is **pixel-identical** to the frame before the press (`f0.png` against `f2.png`: no difference at threshold 40).

That is the whole behaviour -- tick, change your mind, one key, nothing left ticked -- and it closes the matrix row that recorded the
reference doing it.

**A lab note from the same drive**: the reference was still sitting on a *project page* from a previous round, so hunting for its ring on
Home found a "How to install" button instead. The reference is not reset between rounds and does not reset itself; the only way to know which
screen it is on is to look at it.
### A card press means two things, and the reference decides by selection (2026-11-09)

Driving the reference showed what happens when a card is ticked: its whole content band changes, because selecting anything brings up a
**selection bar** (row 155) -- "1 selected | Clear" on the left, "New group | Delete" on the right. That is a separate gap. What this round
took from the same drive is the rule underneath it (`instance-card.vue:148-154`):

> a press on a card **opens the instance**, unless a selection is live or Shift/Ctrl/Cmd is held -- then it **toggles that card**.

This build opened the instance every time, which throws away every tick the reader has made. Now:

| step | what changed |
|---|---|
| tick a card | (199,377)-(221,399), the ring |
| press the card body **with the tick live** | **the same 22x22 box -- the tick came off, the page did not move** |

So a reader can tick, then untick by pressing the card, and never leaves Home by accident. The modifier half of the rule is in the same
expression but **untested**: the library has one instance, and an extension needs two.
### The selection bar, drawn for the first time (2026-11-09)

Selecting a card in the reference brings up a bar at the foot of the content column. This build had no bar at all: ticking a ring changed
22 points of pixels and nothing else.

Measured from the reference\'s own capture (`r1.png`): 64 points tall, the content\'s width less 32, sitting 80 above the content\'s floor, fill
(39,41,46). Drawn now with its left group -- **"N selected" | rule | "Clear"** -- and its right group -- **"New group" and "Delete"**, the second
in the destructive red, each with the reference\'s glyph.

| what | state |
|---|---|
| the bar, its count, its fill and its rules | drawn (`bar4.png` against `r1.png`) |
| `Clear` | **live** -- empties the selection, which is what its label says |
| `Delete` | **live for a single selection**: it opens the same confirmation dialog the kebab\'s Delete does |
| `Delete` with several ticked | **deliberately not wired** -- there is no bulk confirmation flow here, and one press removing several folders is not a thing to add without one |
| `New group` | drawn, **not wired**: no group modal exists in this build |

The glyphs were drawn on top of their labels first, because both were placed by eye; they are now placed from the label\'s laid-out width,
which is the same fix the filter row needed and for the same reason.
### Both of the bar\'s live actions, driven (2026-11-09)

The selection bar was drawn last round with two of its four labels wired. Both were pressed this round:

| press | what happened |
|---|---|
| **Clear** (with one card ticked) | the frame afterwards is **pixel-identical** to the one before the tick -- the tick came off, the bar went, nothing else moved |
| **Delete** (with one card ticked) | the **"Delete instance"** confirmation opened: its warning, the instance\'s own row, `Cancel` and the red `Delete instance` |

So the bar is not decoration: a reader can tick, clear, or commit to deleting, and the last of those goes through the same confirmation the
kebab\'s Delete does rather than removing a folder on one press.

**A hand-over note for the next round**: the bar\'s `Delete` rect is `bar.right()-96 .. bar.right()-24`, which is 72 wide and sits 24 in from
the bar\'s end. The red label is about 45 wide, so the rect covers the label and its glyph with a little slack -- but `New group`\'s label ends
at `bar.right()-120`, only 24 from the Delete rect\'s left edge, and a press in that 24-point gap does nothing at all.
### The selection ring was a fifth of the size it should be (2026-11-09)

The reference draws a **24-point circle inside a 50-point hit area** (`instance-card.vue:363-388`). This build drew 22 in 22 -- the circle a point
small and the target a fifth of the area, so a press anywhere near the ring missed it.

| | reference | before | after |
|---|---|---|---|
| the circle | 24 | 22 | **24** (measured: the white disc runs x 198..221 in the selected frame) |
| the hit area | 50 | 22 | **50** |
| a press 22 points from the centre | selects | **missed** | **selects** (`k0.png` -> `k3` sequence) |

The check glyph went from 14 to 16 with it (`CheckIcon size-4`).

**Not done**: the 200 ms opacity fade. The reference fades the ring in (`opacity-0 ... group-hover:opacity-100`,
`transition-opacity duration-200 ease-out`); this build draws it at once, as it draws every hover state. That is recorded in the row rather than
claimed, and it is the same missing mechanism as the menu fades.
### A card\'s right press showed the page\'s menu, and now shows nothing (2026-11-09)

Driving a right press on a card at (150,400) opened **"+ New instance"** -- the page\'s own menu -- because the page handler took every press
inside the content rect. The reference gives the card its own menu at the pointer (`instance-group/index.vue:645-648`).

The page handler is now **guarded by the cards\' rects**: off a card it still opens "+ New instance" (driven: a press at (700,600) changes
(718,623)-(848,635) at threshold 40; the panel\'s own fill is only 17 points from the page, so the panel does not count and the text does), and
on a card the page menu no longer appears.

**Two attempts to draw the card\'s own menu failed, and the card now opens nothing:**

| attempt | result |
|---|---|
| `card_response.secondary_clicked()` sets the popover\'s anchor | **no change at all** -- a widget that senses only clicks does not report secondary clicks |
| raw `pointer.secondary_clicked()` plus `card.contains(at)`, anchoring the row popover with a **zero-size rect** | the press changed only the ring\'s own 42x117 area; the popover never drew |

So `menu_popover` does not render from a zero-size anchor. The fix is to give it a real one -- the pointer plus the menu\'s own
`bottom-start` placement -- rather than to keep guessing. Both failures are written down so the next attempt starts from them.
### The card\'s own menu, at the pointer, and why it took three attempts (2026-11-09)

A right press on a card opened the **page\'s** "+ New instance" menu. It now opens the card\'s own verb list at the pointer, placed
`bottom-start` as the reference\'s `ContextMenu.vue:32-33` is: **Play, Edit, Duplicate instance, Open folder, Delete**.

Three things had to be true, and each one was missing in turn:

| what was missing | how it showed | fix |
|---|---|---|
| the page handler took the press first | "+ New instance" over a card | the page handler skips presses inside a card\'s rect |
| `menu.open` was never called | setting the anchor alone drew **nothing** -- `menu_popover` returns early while `open_for` is `None` | the press opens the menu |
| the kebab\'s block overwrote the anchor | the menu drew **under the card**, anchored to the kebab | `Menu::opened_at` remembers the pointer, and the kebab\'s block honours it |

And one more that only the capture could show: with a zero-size anchor, `anchor.right() - width` put the panel **off the window\'s left
edge**, because the placement assumes a right-aligned trigger. A pointer-opened menu now places its left edge at the pointer instead.

The first two of those are the same class of failure this file keeps recording -- **a control that looks wired and is not**, because the
thing it depends on was never called. The third is a familiar one too: two code paths writing one field, and the later one winning silently.
### The card menu\'s rows act, and a test for the anchor it uses (2026-11-09)

Opening the card menu at (150,400) and pressing **Edit** -- its second row -- opened the **"Edit instance"** dialog with the name field,
`Cancel` and `Save` (`edit-row.png`). So the menu is not a new code path with its own verbs: its rows are the kebab\'s, and they reach
the same dialogs.

**A test for the risk the last round left open.** `opened_at` decides where a menu appears, and two things write it: a right press on a
card, and the kebab. If it survived a close, the next kebab menu would appear wherever the last right press had been -- a fault that would
read as a rendering bug. `a_pointer_anchor_does_not_outlive_its_menu` asserts that opening the kebab\'s menu starts with no pointer
anchor, that closing forgets one, and that opening again starts clean. **268 tests** pass.
### The placement that broke, now a function with tests (2026-11-09)

Two rounds ago a card\'s menu drew **off the window\'s left edge**, because the panel aligned to `anchor.right() - width` and a
right-pressed anchor has no width. The fix was two lines inline; the number that broke had no test, and the same shape of mistake is one
edit away.

It is `menu_placement(anchor, bounds, size, pointer_opened, top)` now, with three assertions:

| case | expected |
|---|---|
| a right press at x 150 | the panel\'s **left** edge is 150 (the old code produced **-50**) |
| a right press near the right edge | the panel clamps to the page\'s right edge |
| the kebab\'s menu | its **right** edge stays on the kebab |

**269 tests** pass, and the running build still opens the card menu at the pointer with all five rows (`w1.png`) -- the refactor is
behaviour-preserving, which is the least a refactor owes.

**Also cleaned**: the comment block that carried the old inline maths had literal backslashes in it (`\'`), left by the escaping on a
previous insert. They were inside `//` so the build never noticed, but they were noise in the one place a reader looks for why the number is
what it is.
### The other half of the placement, and what "behaviour-preserving" means here (2026-11-09)

Last round\'s risk was specific: `menu_placement` covered the left edge and the caller still computed `top`, untested. Both halves are in
the function now -- below the anchor when it fits, above it when it does not, never above the page -- with two more assertions:

| case | expected |
|---|---|
| the kebab\'s menu, room below | top = kebab\'s bottom + the menu distance |
| a press at y 860, no room below | top = pointer − distance − height, and **not above the page** |

**269 tests** pass, and the running build\'s card menu is **byte-identical** to the one captured before the refactor -- the same PNG
hash, not just the same appearance. That is the strongest form "behaviour-preserving" takes in this file: two captures of the same state that
differ by nothing at all.

The vertical flip is still **not driven** -- the library has one card and it sits mid-page, so the flip needs a card near the floor. The
test is arithmetic, not evidence from the screen, and it is written here as such.
### The search field: Escape, a clear button, and why two `interact`s did not work (2026-11-09)

One drive covered two rows and found a third thing missing.

| step | what happened |
|---|---|
| click the field, type `Fabric` | the field\'s text appears at (127,262)-(168,271); the list is unchanged, correctly |
| `Escape` | **the query clears and focus drops** -- typing "zzz" after it changes **nothing** |
| the field with text in it | **no clear button at all**, where the reference has one |

The clear button is drawn now: an X at the field\'s right edge, only while there is something to clear, and pressing it restores the
`Search` placeholder.

**Getting the press to land took three tries, and the reason is worth keeping:**

| attempt | result |
|---|---|
| a second `interact` on the X\'s rect, registered **before** the field\'s | the field took the press |
| the same `interact`, registered **after** the field\'s | the field took the press again |
| the field\'s own handler checks whether the press was inside the X | **it clears** |

egui resolves one press to one widget, and the field\'s rect contains the X\'s -- so whichever order they are registered in, one of them
loses and it is not the one drawn on top. A single handler deciding by position is not a workaround for that; it is the only arrangement in
which two targets can share one rect.

The probe prints the field\'s rect now (`field=[[88,248]-[837,284]]`) -- the press had been aimed at a guess for two attempts, because
nothing printed where the field was.
### The search field\'s placeholder was the wrong tone, and the tool said so about the wrong pixel (2026-11-09)

`element.py` on the field box reported "brightest off by 26, ink rows 3 short", and I took the placeholder to be the subject. It was not: the
brightest pixel in that box is the **magnifier glyph**, which is `SECONDARY` in both builds and was never in question. Narrowing the box to
the text alone gave the real numbers:

| | reference | before | after |
|---|---|---|---|
| text tone | (111,119,130) | (150,162,176) -- **off by 39** | **(114,123,135) -- off by 5** |
| text ink rows | 12 | 9 | **11** |
| the field\'s fill | -- | -- | **off by 0** |

Home at 1440x900 went **32 864 -> 32 818**, which is what a placeholder two shades right is worth: 46 pixels, none of them loud.

**The mistake worth keeping**: an element-level tool reports the extremes of whatever box it is given, and a box containing two elements answers
for the brighter one. Every "off by N" is about one pixel, and which pixel is the reader\'s job -- the third time in this file that a window
wider than the thing produced a confident number about a different thing.
### The field\'s glyph and inset, to the pixel (2026-11-09)

Boxing the glyph and the text separately -- the lesson from last round -- gave numbers that converged in two passes:

| | reference | first | after |
|---|---|---|---|
| the magnifier | (102,257)-(119,274) | 14x14, 2 points in | **18x18, the same box** |
| the text\'s first column | 129 | 127 | **129** |

and Home at 1440x900 came down **32 864 -> 32 781** across the whole field (placeholder tone, glyph size, text inset), which is 83 pixels for
three measured corrections -- a small number for the work, and the honest scale of what an element this size can be worth.

The passes were needed because each fix moved the other: enlarging the glyph pushed its box a point left, and the text had to move a point
back to compensate. That is ordinary for adjacent elements, and it is why the box has to be re-measured after every change rather than
extrapolated from the first.
### The field\'s state, Enter, and focus -- three checks that closed three questions (2026-11-09)

Last round\'s risk was that the field\'s numbers had been tuned against a reference capture taken mid-focus. They had not:

| | `refclean2` | a fresh capture now |
|---|---|---|
| field fill | (52,54,60) | (52,54,60) |
| glyph tone | (111,119,130) | (111,119,130) |

so the placeholder, glyph and inset figures stand.

**Enter does nothing** -- on either side. Typing `Fabric` and pressing Return changed **no pixel** in the reference and none here. That is a
behaviour matched by *not* having a behaviour, which is the kind that is easy to get wrong by adding a sensible feature nobody asked for.

**There is no focus ring.** The reference\'s field border reads (66,68,74) at (88,266) and at (600,248) whether it has the keyboard or not --
identical to the byte. This build draws exactly that, so "the field\'s focus ring" was a thing I had worried about that does not exist.
### The objective, clause by clause, with each clause\'s own number (2026-11-09)

`docs/FINISHED.md` now opens with a table that answers the objective directly rather than describing the work. Its contents, in brief:

- **99% identity: not met.** Best page 2.54%; the floor is the text rasteriser, measured three ways.
- **Five sizes: met.** All five captured against a restarted reference with both pointers parked.
- **Matrix rows: partial.** 192 of 733 DONE.
- **RSS: not met.** 22 500 kB against 20 MB, with the breakdown and the cache experiment that ruled the cache out.
- **Build and tests: met.**
- **Committed and measured: met.** 709 commits.

Two of the six cannot be reached from here, and the document says so in the same table as the four that can. That is the honest form of
"finished" this project\'s own rules ask for: the bar is stated, the measurements are attached, and the shortfall is named with its cause
rather than left as a number to be improved.
### A page comparison that was two different pages (2026-11-09)

Refreshing the Screenshots number gave **113 657** against a longstanding 23 573. The capture said why: the reference was showing
**Settings** -- a key press made while probing the rail had opened it, and the Screenshots capture was of its backdrop.

The tell was not subtle once looked at: content alone differed by 76 438 pixels, which is not a tone or a glyph, it is a page. The rule this file
already carries -- **check a known pixel before any sweep** -- caught it at the first comparison rather than in a table three rounds later.

After Escape the reference was on **Servers**, not Screenshots: the rail\'s y 245 is Servers and y 205 is Screenshots, and the probe had
pressed 245 expecting the picture icon. Pressing 205 and reading the title first gave the true figure, **23 573**, identical to the number
recorded months of rounds ago -- so the page has not regressed, and my "improvement" was a measurement of the wrong window.

Three presses were needed to get one comparison: 245 (Servers), 820 (Settings, which then had to be dismissed), then 205 (Screenshots,
confirmed by cropping the title bar and reading it). The crop-and-read step is what turned a passing comparison into a checked one.
### The rail, mapped by pressing every icon and reading the title (2026-11-09)

Last round\'s wrong press meant the rail map I had been using was not trustworthy. It is now measured: each y was pressed, and the title bar
cropped and read -- eight crops stacked into one image (`/tmp/rail.png`) so they could be compared at once.

| y | title bar says | icon |
|---:|---|---|
| 65 | Home | play |
| 123 | Discover modpacks | compass |
| 178 | Skin selector | t-shirt |
| 205 | Screenshots | picture |
| 245 | *(still Screenshots)* | **nothing** -- empty rail |
| 290 | **Servers** | stacked rectangles |
| 365 | **Fabric 26.3** | the green instance icon |
| 820 | Settings | opens a blurred backdrop over the page |

**Two of my own notes were wrong and are corrected here:** y 245 is empty space, not the Servers icon; and y 365 is the **instance page**, not
the `+` that creates one. The `+` sits between them and is not at either address I had written down.

That explains the round before last: pressing 245 left Screenshots showing (no page change at all), which I read as "the reference is stuck",
then Escape took the *previous* page -- Servers -- which I read as "245 is Servers". Both readings came from a single press on nothing, and
the title crop would have said so at the time.

**The rule this adds**: a rail press is not evidence of a destination unless the destination is read back. Position memory is what produced
two mislabelled captures and a 113 657-pixel comparison of two different pages.
### Settings, compared as the modal it is (2026-11-09)

Both clients show Settings as a **modal over Home** with a blurred backdrop, and the two are the same object: a title, a close X, three nav
groups (DISPLAY, ACCOUNT, INSTANCE DEFAULTS), the "Color theme" heading over four cards (Dark, Light, OLED, Sync with system), then "Sync theme
across devices" with its toggle, and a footer with the mark and the version string.

`compare.py` puts the pair at **32 469** (titlebar 1 547, rail 344, content 28 687, sidebar 1 891). Three differences are visible without
measuring further, and each is a real one rather than a tone:

| difference | what it means |
|---|---|
| the dialog is ~28 points taller and ~20 higher here | the card column or the footer padding is larger |
| a **scrollbar** is drawn on this build\'s dialog | the content overflows where the reference\'s fits -- the reference has no scrollbar in this state |
| an **"Advanced options"** hint is drawn at the foot | the reference does not carry it in this state |

The scrollbar is the useful one: it is the measurement telling me the dialog is too tall in a way the height number alone would not. A dialog
that fits does not need a bar.
### Settings: the nav scrolls on both sides, and only one draws a hover plate (2026-11-09)

Three deltas were named from the Settings comparison. Chasing them produced one fix, one correction and one near-miss.

**The near-miss first.** The extra text at the dialog\'s foot reads **"Advanced rendering"** -- a nav row, not a hint. The tempting reading
("the reference has no such item, delete it") would have removed a real tab: the reference\'s own key is
`settings.display.sidebar.advanced-rendering.title`, so the item exists on both sides. What the reference lacks is the **scrollbar**, and
that is an overlay-scrollbar convention, not an absent list.

**The correction.** The page\'s own header comment says "scrolling it is the next thing this page owes". Driving the wheel over the reference\'s
rail and over this build\'s changed both by the same region -- (273,220)-(468,698) against (272,218)-(490,695) -- and both landed on the same
rows: Profile, Social, Privacy, INSTANCE DEFAULTS (Java and memory, Window, Launch hooks), SYSTEM (Resources, Troubleshooting). So the nav
**does** scroll, and the comment is stale.

**The fix.** Cropping the two after-states side by side showed the difference that matters: the reference puts a plate under the row the
pointer is on, and this build drew those rows flat. The plate is drawn now -- `BUTTON_BG` under a non-selected row that contains the pointer,
with the selected row keeping its brand wash. Driven: with the pointer at (300,490) the row under it changes (272,464)-(490,504), a 218x40
plate.
### Two claims withdrawn, one difference measured (2026-11-09)

Last round named three differences in the Settings dialog from a downscaled composite. Measured at full resolution, **two of the three were
wrong**:

| claim | how it was checked | verdict |
|---|---|---|
| "a scrollbar is drawn on this build\'s dialog" | cropped both dialogs\' right edges at 1:1 (`edge.png`) | **withdrawn** -- neither side has one. `settings.rs` contains no `ScrollArea`; the line I saw was the plate\'s own border, at 1:2 scale |
| "the dialog is ~20 higher and ~28 taller" | longest run of the plate\'s fill per row, both captures | **withdrawn** -- the plate\'s top is **115 on both** and the first 84 rows are identical in length |
| "an `Advanced options` hint is drawn" | cropped the foot at 1:1 | **withdrawn** -- it reads "Advanced rendering", a nav row that both sides have (`settings.display.sidebar.advanced-rendering.title`) |

**What is actually different**: at x 1000, 1020, 1040 and 1060 the reference\'s plate fill ends at y **198** while this build\'s continues to y **447**. Same
top, so not a height matter -- **this build\'s plate is wider**, by enough that the reference has left the content well before those columns.

The lesson is the one this file keeps re-learning at a different scale each time: **a downscaled composite is a picture, not a measurement**. Three
plausible differences, all three read from one 540-point-wide thumbnail of a 1440-point window, and all three wrong in a way that each had a
believable story attached.
### The Settings plate is the same rectangle, and that took four wrong answers to establish (2026-11-09)

Four differences were named for this dialog across two rounds, and **all four were withdrawn by measurement**:

| claim | measurement | verdict |
|---|---|---|
| a scrollbar is drawn | cropped both right edges at 1:1 | no bar on either side |
| ~20 higher, ~28 taller | longest fill run per row | top at **115 on both**, first 84 rows identical |
| an extra `Advanced options` hint | cropped the foot at 1:1 | it is `Advanced rendering`, a nav row both sides have |
| **this build\'s plate is wider** | the plate\'s own top rows | **identical**: 270..1169 at y 116, 257..1182 from y 140 |

The bottom corners agree too -- both plates end between y 784 and 786, and their rounded corners differ by at most 2 points. **The plate is the
same rectangle on both sides**: x 257..1182, y 115..784. Every one of the 32 469 differing pixels is interior -- type, spacing, a card\'s tone --
and none of it is the frame.

**The measurement technique that finally worked**, after three that did not: read the plate\'s extent from **its own edge rows**, where its content
cannot interfere. Scanning a row *through* the dialog cannot distinguish "plate" from "empty plate" -- `(39,41,46)` is both -- so a row where the
reference happens to hold a card and this build holds space reads as a width difference that is not there. Which is what I reported, twice, with
increasing confidence.

**The count**: four confident differences, four measurements, four withdrawals. The first two came from a thumbnail, the third from reading a
clipped label, the fourth from a scan whose method could not answer the question asked of it. Each had a story that fit.
### Where the Settings difference actually lives: inside the plate, and it is type (2026-11-09)

The plate being identical left the question of where 32 469 pixels differ. Masking the plate\'s own rect answers it:

| region | differing pixels at threshold 40 |
|---|---:|
| inside the plate (257..1182, 115..784) | **27 256** |
| outside it -- the blurred Home behind | **10 189** |

And the plate\'s contents check out at the landmarks: the "Color theme" heading inks rows 231..269 on **both** sides, and the first card\'s top
edge is 240 at x 620 and 245 at x 560 on **both**. So the difference is not layout, not the frame, not the cards\' geometry -- it is the text inside
them, which is the same rasteriser difference every page has.

**The useful number is the backdrop\'s.** Home alone differs by 32 781 pixels; the same page seen through Settings\' blur differs by only
**10 189**, because the blur suppresses glyph detail. That is a direct measurement of how much of every page\'s difference is text: blur it and
two thirds of the disagreement disappears.

It also means a modal comparison is not a page comparison: the number includes whatever is behind the plate, attenuated by the blur, so it
understates the dialog and overstates nothing. Reading it as "the Settings dialog differs by 32 469" -- which the table did for many rounds -- was
wrong in both directions at once.
### The sidebar is half of Home, and its news card is 33 points narrow (2026-11-09)

Home\'s 32 781 splits 13 875 content and **16 816 sidebar**. The sidebar is banded here by 40-row slices, and the largest single band is
**rows 568-607 with 3 697 differing pixels** -- the news card\'s paragraph.

At 1:1 the cause is not tone, it is **where the lines break**:

| | line 1 of the paragraph |
|---|---|
| reference | "Share your server with friends **directly**" |
| this build | "Share your server with friends" |

Measuring the two cards\' right edges at y 600 gives **x 1378 for the reference and x 1345 here** -- the news card is **33 points narrower**, so the
same words do not fit. The type itself is the same width per character (about 6.5 points a glyph on both), so the size is right and the box is
wrong.

**A hypothesis tried and measured out**: `COPY_SIZE` is 14 while its own doc comment says "12 points, from the reference\'s measurement". Setting it
to 12 made the sidebar **worse by 140** (16 816 -> 16 956), so 14 stands and the comment is the stale artefact -- the second time this session a
comment and its constant disagreed and the constant was right.

**The fix is the card\'s width**, which the wrapping measure is derived from (`column(rect).width() - 20`). That is next round\'s work, and it is
worth 3 697 pixels in one band rather than the 50-100 the last four rounds have averaged.
### The news wrap: the box was already the reference\'s, and widening it is worse (2026-11-09)

Last round left the question open -- is the news card narrow, or its column? -- with the note that two causes would need two fixes. A probe
settles it:

```
news column=[[1153.0 48.0] - [1428.0 900.0]] measure=255
```

The column is **275** wide and the measure is **255** -- exactly the reference\'s own measure, as the code\'s comment claims. The plates agree
too: at y 120, 300 and 700 both sides\' sidebar spans 1140..1439 identically. So neither the card nor the column is narrow, and the earlier
"33 points" was text ink at y 600, not a card edge -- a third reading of the same row, and the third one to be wrong.

**What is left is the typeface.** At the same 255 points and the same nominal 14-point size, this build\'s face needs more width per word than
the reference\'s, so "directly" does not fit on line 1. Widening the box to the column\'s full 275 should then pull the reference\'s line break
back -- and it does, and it is worse:

| measure | sidebar | Home |
|---|---:|---:|
| 255 (the reference\'s) | **16 816** | **32 781** |
| 275 (the column\'s full width) | 17 681 | 33 646 |

Widening fits more words per line but shifts every news card\'s date, art and height, and costs 865 pixels overall. 255 is the measured
optimum, and the probe is kept so the next round that wonders about this number can read it rather than infer it -- the third time this session a
printed value has ended an argument that measurements from captures could not.

**The band\'s 3 697 pixels are therefore the text engine\'s**, not a layout error -- the same cause as every other page\'s floor, concentrated in
one card because this card has four lines of it.
### The "wrap" that was two different articles (2026-11-09)

Two rounds attributed the news card\'s 3 697 pixels to a narrower box, then to a wider typeface. Both were wrong, and the probe that disproved the
second one also disproves the first:

```
news line width=252.40625 (reference ink 253) column_width=275
```

This engine lays out "Share your server with friends directly" at **252.4 points** where the reference inks **253**. The faces are within a fifth
of a percent of each other; the box is the reference\'s 255; nothing about the type or the measure explains a wrap.

**Because it is not the same sentence.** At 1:1, rows 548-620:

| | the card\'s copy |
|---|---|
| reference | "Share your server with friends directly / in the app. When they accept, we\'ll / create an instance for them that\'s ..." |
| this build | "Keep game options, servers, resource / packs, and more the same across / your instances." |

**Different articles.** The reference\'s first card is the hosting promo; this build\'s is a different item, dated "September 7, 2026". Every
"wrap difference" measured across two rounds was the line breaks of two different sentences, and the 3 697-pixel band is mostly that, not a
rendering difference at all.

Three slides were tried (`SIFT_NEWS_SLIDE=0/1/2`) and all three differ from the reference by about 9 500 pixels in that band, so the ordering is
not a one-off: this build\'s bundled feed does not lead with the article the reference\'s live feed leads with.

**What this changes.** Home\'s 32 781 is not all rendering: some of it is different words. Any page whose content comes from a live service --
news, the project page\'s body, download counts -- carries the same contamination, and the numbers for those pages should not be read as though
they measured the interface.

**The pattern, fourth time this session**: a difference attributed to a mechanism, measured, and found to be a state or content mismatch. The
probe that prints our own computed value is what ended it; nothing in the captures could, because two different sentences look exactly like two
different renderings of one sentence.
### The "Jump in" caret: off by twelve points and a shade, invisible to the metric (2026-11-09)

Measuring four labels\' ink extents to test the rasteriser attribution turned up one that was not the rasteriser:

| | ink extent |
|---|---|
| reference "Jump in" + caret | x 89..205 |
| this build | x 88..187 |

At 4x the cause is plain: the reference\'s caret sits **twelve points right** of the last glyph, in the heading\'s own white; this build drew it
hard against the "n" in `SECONDARY`, a shade below the text. Moving it to `rect.left() + 104` and `CONTRAST` matches both.

**The number did not move**: 32 781 -> 32 784, three pixels, against a 40-point threshold that cannot see a caret\'s tone or twelve points of
gap. The change is correct and the metric is blind to it -- which is the standing reason this project measures with crops as well as counts,
and the reason a flat number is not evidence that nothing happened.

**Three of the four labels measured were clean**: the "Library" heading\'s ink box, the card name\'s column and the sidebar\'s "News" heading all
land within a point or two across 1 000-odd ink pixels each. So the rasteriser attribution holds for those, and this round is the first in five
that tested it rather than assuming it -- the difference it found was real, small, and in the one place nobody had looked.
### The library card\'s plate is 18 points high and 11 short; its artwork is right (2026-11-09)

A region of `RAISED_BG` below the filter chips -- 3 450 pixels with no counterpart in the reference -- led to the card, and the card\'s own
extents settle it:

| | plate | height |
|---|---|---:|
| reference (x 95, y run) | 358..564 | **207** |
| this build | 340..535 | **196** |

**The artwork starts at y 367 on both sides.** So the thing that is wrong is the plate around it: this build\'s card begins **18 points higher**
and ends **29 earlier**, and the art sits at the reference\'s own row inside it. That is why the difference did not show as a misplaced picture --
the picture is where it belongs and the box around it is not.

Two consequences worth naming before the fix: the card\'s top padding here is **27 points** where the reference\'s is **9**, and its bottom padding
is **29 points short** of the reference\'s. A card that is 207 tall in the reference and 196 here also means the grid\'s row pitch is wrong, and
a second row would drift further -- with one instance in the library, that has never been visible.

**Three wrong turns got here**, each recorded rather than hidden: the chips\' fill (measured identical at (52,54,60)), the chevron band (no
glyph ink above threshold 90, so nothing to measure), and the card\'s top edge at x 100 (which caught the chip row above it). The measurement
that worked scanned a column **inside the card but right of its icon** and split the rows into runs.
### The card box is measurably wrong, and moving it alone costs 121 pixels (2026-11-09)

At the card\'s middle column -- where the rounded corner cannot interfere -- the two plates are:

| | plate at x 150 | height |
|---|---|---:|
| reference | 354..568 | **215** |
| this build | 329..536 | **208** |

and **the artwork starts at y 367 on both**. So the reference\'s card begins 25 lower than this build\'s and is 7 taller, with its tile 11 below
its top edge where this build\'s is 36.

The fix was applied as three numbers -- `ROW_TOP` 281 -> 306, `CARD_H` 208 -> 215, the tile\'s inset 36 -> 11 -- and measured **worse**:

| change | content | Home |
|---|---:|---:|
| baseline | 13 878 | **32 784** |
| the three numbers | 13 999 | 32 905 |
| and the name\'s gap 8 -> 3 | 14 022 | 32 928 |

**All four reverted**, back to 32 784 exactly.

The reason is in the reference\'s own card: its name ink starts at **y 500**, only three points below the tile\'s 497, and its plate continues
215 points to 568. That is not a taller version of this build\'s card -- the reference\'s card carries its text *inside* a plate that has room for
more lines than this build draws, a "last played" line among them. Moving the box without those lines puts every one of them in the wrong place,
which is what the two worse measurements are.

So the card\'s box cannot be matched before its interior is: the height is a consequence of the lines, not a number to set. The measurement stands
and the fix waits for the interior -- recorded here so the next round does not try the three numbers again, which is what this file is for.
### The card\'s height, taken alone, is worth 4 pixels; its top is not a number (2026-11-09)

Reading the interior at 2x before touching numbers again -- the lesson last round wrote down -- showed the real defect plainly: this build\'s grey
loader line ran **into the plate\'s bottom edge**, where the reference leaves padding under it.

The three numbers were separated this time:

| change | Home |
|---|---:|
| baseline | 32 784 |
| **`CARD_H` 208 -> 215 alone** | **32 780** |
| `CARD_H` 215 and the name\'s gap 8 -> 3 | 32 803 |
| `CARD_H` 215, `ROW_TOP` +25 and the tile inset 36 -> 11 | 32 905 |

So the height is worth 4 pixels and the **top is not a number to set**: the card\'s top edge is 25 points above the reference\'s, and moving it --
with the tile inset compensated, so the artwork stays at 367 on both -- costs 121. The reference\'s card is taller *and lower*, and its extra height
comes from a line this build does not draw.

**Kept**: `CARD_H` 215. **Reverted**: the gap and the top. Home 32 784 -> **32 780**, and the padding under the loader line is now visible at 2x.

Four attempts at this card across two rounds have produced 4 pixels and three reverts. The honest reading is that the card\'s geometry is a
consequence of a content difference -- one line, one height -- and no arrangement of the box alone will match it.
**The test caught the change, and was right to.** `a_rows_targets_are_on_the_card_and_clear_of_each_other` asserts the card is 208 tall,
with a comment saying the number is the measurement and the test follows it rather than the constant. Changing the constant to 215 failed it,
which is the guard working: a height is a claim about the reference, and the claim now has a new value with its own measurement behind it
(354..568 at x 150). Updated to 215 with that reason; **269 tests** pass.
### The card: a coherent set of offsets, and why it is still wrong (2026-11-09)

The missing third line does not exist. At the same absolute rows the reference\'s card carries exactly what this build\'s does -- a name, a loader
line, then padding -- so the extra 29 points below the name are padding, not content, and the difference looked like three transposable offsets:

| | reference (card 354..568) | this build (card 329..544) |
|---|---|---|
| art | 367..497 | 367..495 |
| name ink | 515..527 | 505..515 |
| loader ink | 535..547 | 518..528 |
| plate end | 568 | 544 |

Read as offsets from the card\'s top: the reference puts its tile 13 down and its name 18 below the tile; this build puts its tile 36 down and its
name 8 below. Setting all three to the reference\'s -- `ROW_TOP` +25, the tile\'s inset 36 -> 13, the name\'s gap 8 -> 18 -- was measured at
**33 595, worse by 811**, and reverted.

**Why a set that matches the reference\'s own numbers is worse**: `card.min + 13` and `tile.bottom() + 18` are offsets in *this* engine, and this
engine\'s glyph boxes are not the browser\'s. A name drawn 18 below a tile that was placed 13 below the card puts its ink somewhere the arithmetic
does not predict, because egui puts a glyph two to four points into its line box depending on size -- the same asymmetry the copy\'s comment records
for the news card ("+1 and the -4"). The reference\'s offsets cannot be copied; they have to be solved against this engine\'s line boxes.

**Stopped here, deliberately.** Five attempts at this card across three rounds: 4 pixels gained, four reverts, one test corrected. The card sits at
a measured optimum of 32 780 with `CARD_H` 215, and every further move I can justify from a capture makes it worse. Getting it right needs the
line-box model written down first -- one paragraph of arithmetic, then one attempt -- rather than a sixth guess at an offset.
### The rail tooltip was white with dark text -- the inverse of the reference (2026-11-09)

Two rounds of "the rail does nothing on hover" turned out to be the reference sitting behind a Settings modal. With it closed and Home shown,
the tooltip is there -- and so is this build's, as the **opposite object**:

| | reference | this build, before |
|---|---|---|
| the plate | `RAISED_BG` (39,41,46) | **`CONTRAST` -- white** |
| the label | white, 15 point | **`ACCENT_CONTRAST` -- dark**, 12 point |
| the arrow | a left-pointing arrow at the plate's edge, aimed at the icon | **none** |
| the offset | 31 points right of the rail | 6 |

Now drawn as the reference draws it, and driven on both sides: the Servers icon shows **"Modrinth Hosting"** in both (`tt5-rail.png`,
`tt6-servers.png`) -- same string, same design, right of the icon.

**Rows 213, 430 and 505 closed.** The timing (300 ms to show, 400 to hide, per `providers/tooltip.ts`) was already in the code and is recorded
rather than timed; the geometry spec in row 507 (8-point offset, flip, arrow, default `top`) is matched for the rail only, so it stays open.
### The title bar tooltips, and the layer that hid them (2026-11-09)

The reference title-bar tooltips are real and match the rail design, with the placement flipped: the plate sits **below** the button and its
arrow points **up**. Hovering the download ring shows **"Download manager"** -- the pill top edge at y 49, which is the button bottom (41)
plus 8.

This build had none. A reusable `tooltip_below` now draws the plate, and one call site uses it. Two things had to be right and the first was not:

| attempt | result |
|---|---|
| draw it from the title bar own painter | **nothing visible** -- the title bar is painted before the page, and the content column covers y 49..85 |
| the same call on an `Order::Foreground` layer | **the pill draws** (`otb2-on.png`), plate top 49, label "Download manager" |

The rail tooltip never had this problem because it is drawn after the pages, in the shell last pass -- which is why the same design in the same
file behaved differently in two places.

**Row 514 stays open**: it asks for eight title-bar tooltips and this round delivered one. The mechanism is now one function call per target, so
the remaining seven are calls rather than a new system.
### Which tooltips this lab can reach at all (2026-11-09)

After the rail's tooltip was matched, the rest of the cluster was attacked by hovering every target the reference has, and most of them answer
with nothing because the *state* that carries them is not reachable here. Recorded so the next round does not re-hover them:

| target | hover result |
|---|---|
| rail buttons | **tooltip** -- matched and closed this session |
| title-bar download ring (x 974) | **tooltip**, "Download manager" -- now drawn here too |
| title-bar Update pill (x 1040) | hover state only, **no tooltip** |
| title-bar status pill (x 1200, 1250) | **nothing at all** |
| instance card play disc (x 206, y 470) | the card's hover affordance, **no tooltip** |
| instance card at (236, 470) and (240, 520) | the same affordance, **identical box** each time |

Row 514 asks for **eight** title-bar tooltips -- "downloads, view instance, stop, view logs". The buttons for view-instance, stop and view-logs
appear in the title bar only when an instance is running or installing, and this lab cannot put the reference into that state: its own instance is
one this build created, and the reference has never launched it. Hovering the three positions that exist gives one tooltip and two nothings.

Row 510 (card actions: Loading/Installing, Stop, Repair, Play, Select/Delete) is the same shape: those labels belong to states -- installing,
running, repairing -- that the reference is not in and that this lab cannot induce without a Minecraft account it does not have.

So of the 16 open tooltip rows, **four are reachable now** (the rail's three, closed, plus the download ring, drawn) and the rest are behind
states. That is a bigger statement than "not done": it says the cluster cannot be closed from here at all, and the rows should carry that cause
rather than staying OPEN as though a drive would close them.
### A populated Screenshots page, on both sides, for the first time (2026-11-09)

Every screenshots comparison in this project has been of **two empty pages**, because the reference had never taken a screenshot. It does not
have to be that way: the page reads `/tmp/ref107/ModrinthApp/profiles/<instance>/screenshots`, and a file copied in there makes the page populate.

**Two wrong folders first.** The instance folder under `/tmp/ref107/SiftLauncher/instances/...` looks like the obvious home, and is where this
build keeps them; putting a PNG there changed nothing -- the page still said No screenshots yet. The reference reads its *profile* copy, found
with `find /tmp/ref107 -iname '*screenshot*'`.

With one file in both instances the two pages show: a heading, a search field, a New group button, the sort chips (Newest, Date), a group header
reading Today 1, and one card carrying the image with its filename over the instance name -- on both sides.

`compare.py`: **39 083** -- content 19 734, sidebar 16 816, titlebar 2 107, rail 426 -- against **23 573** for the two empty pages. The populated
state differs *more*, not less, and the extra 15 510 is exactly what a populated page adds: the card\'s own image scaling, its caption overlay and
the group header. The empty-state figure was flattering precisely because it had almost nothing to compare.

**This paragraph first said 26 180.** That number was written before the comparison was run -- a guess dressed as a measurement, in the same
entry that quotes `compare.py` as its source. The command was in the same program and its output printed one line later. Corrected here rather
than silently: it is the exact failure this file has recorded four times, and the first time it happened in a sentence about my own measurement.

**The lab now differs from before**: both instances hold `2026-11-09_12.00.00.png`, so the empty-state figures in FINISHED.md are no longer
reproducible without deleting it. Both states are recorded here so either can be restored.
### The Screenshots page was padded twice: 24 points, 11 338 pixels (2026-11-09)

With both pages populated for the first time, the page head was compared at 1:1 and the whole of this build's content sat **24 points right** of the
reference's: heading, search field and sort chips, all of it, by exactly one `PAGE_PADDING`.

The cause is a double padding. The rect this page is handed is already the content column, inset 24 from the gutter -- the reference's own heading
starts at 88, which is that inset -- and the page then added `PAGE_PADDING` again, putting it at 112.

| | content | total |
|---|---:|---:|
| before | 19 734 | 39 083 |
| after four `rect.left() + PAGE_PADDING` -> `rect.left()` | **8 396** | **27 745** |

**11 338 pixels from one constant**, which is more than the previous ten rounds combined and more than every measurement of glyph tone, caret
position and card height put together. The empty-state page had hidden it completely: an empty page has no list to misalign, and its centred
illustration is centred in whatever rect it is given, padded or not.

That is the argument for populating the reference rather than comparing empty states -- the same round that made the page reachable found the
largest defect in it, and no amount of measuring the empty version would have.
### What is left on the populated Screenshots page (2026-11-09)

After the double padding came out (39 083 -> 27 745), the page's remaining content difference splits cleanly:

| region | differing pixels |
|---|---:|
| inside the card's image area (92..500, 275..492) | **2 166** |
| elsewhere in the content band (200..620) | **3 688** |

The caption is not among them: its ink box is (96,496)-(587,542) here against (95,496)-(588,542) in the reference, and the instance line below it
matches the same way. So the image area's 2 166 is the screenshot **being resampled** to the card's size -- a 1440x900 capture drawn into a
400-point box, where this renderer's bilinear sampling and the reference's GPU filtering disagree on every interpolated pixel. That is the same
class as the glyphs: a different sampler, not a misplaced box.

**The double padding is not everywhere.** The same check on three other pages, by the first ink column at the page head:

| page | reference | this build |
|---|---:|---:|
| Discover | 110 | 111 |
| Skins | 88 | 89 |
| Servers | (no ink in reach) | 88 |

so it was Screenshots' own, and the other pages were not carrying it. Two matched within a point, which is the check worth repeating on any page
that looks "a shade off to the right" -- and cheap, since it needs only the first ink column rather than a populated page.
### The Skin selector: layout matches, the skins are stand-ins (2026-11-09)

`compare.py` on this page reads **135 516** -- the largest single page figure in the project, and the one most likely to be a shift. It is not.
The heading and the "Saved skins" label land within a point:

| | reference | this build |
|---|---|---|
| "Skin selector" ink | (89,78)-(419,95) | (89,77)-(419,94) |
| "Saved skins" ink | (412,82)-(550,96) | (412,83)-(551,97) |

and the structure is the same on both: the big model at the left, the Saved skins card with "+ Add skin / Drag and drop", the "Defaults skins"
heading, a row of four, a second row, "Drag to rotate", "Edit skin", the demo-account banner and "Sign in to Modrinth".

The difference is the **art**. Splitting it: 13 891 pixels inside the big model's area, **144 609** everywhere else -- the nine default skins,
which the reference draws from real skin textures and this build draws as flat coloured stand-ins, plus the 3D model's shading against a flat
silhouette.

**So this page cannot reach 1% without those textures.** It is a content difference, not a layout one, and it is the same shape as the news
card's different article: the interface is in the right place and the thing inside it is different. Recorded rather than attempted, because
drawing nine skin textures by hand would be inventing assets to make a number smaller -- the fake-measurement this project forbids, in its most
literal form.
### The Files tab listed one folder because the instance had one (2026-11-09)

The instance page reads **30 471** (content 11 114) -- one of the closer pages -- and its Files tab read **49 056** with a list that looked wrong:

| | rows |
|---|---|
| reference | config, crash-reports, datapacks, mods, resourcepacks, saves |
| this build | **mods** |

The columns, the header, the search row, Refresh and the kebab all matched; the list was one row against six. That is not a rendering fault -- this
lab's instance had `mods` and nothing else, because nothing in this session had ever created the others. The reference's profile folder carries all
six, and creating the same six in `/tmp/matchdata/SiftLauncher/instances/Fabric 26.3/minecraft` made the list identical in content and order.

`compare.py` went **up**, 49 056 -> 57 726, and that is the honest direction: six populated rows hold more to compare than one, and the columns
they differ in are the mtimes -- 11/03/26 5:58 PM in the reference, 11/09/26 3:32 PM here, because the folders were made today. Two instances of the
same app on different days cannot have the same file dates.

**So the usable number for this tab is the structure, not the count**: same six folders, same order, same five columns, same control row. The
record should say that rather than quote 57 726 as though it measured the table's design.

**The lab changed again**: this instance now has six folders where it had one, so every earlier Files figure (including 48 240 in FINISHED.md) is
for a state that no longer exists.
### Where the Screenshots chrome still differs, band by band (2026-11-09)

After the double padding came out, the chrome's 3 688 differing pixels were banded by 20-row slices:

| rows | differing | what is there |
|---|---:|---|
| 120-139 | **1 316** | the search field |
| 160-179 | **810** | the sort chips |
| 80-99 | **783** | the heading |
| 180-199 | 468 | the chips' foot |
| 220-239 | 413 | the group header |
| 260-279 | 375 | the card's top |

**The heading band was 783 and looked like a clean 24-point error** -- the reference's heading ink starts at x 88, and something in this build's
band starts at 64. Moving the heading to `rect.left() + PAGE_PADDING` was measured at **28 510, worse by 765**, and reverted: 27 745 is restored.

So the ink at x 64 in that band is **not the heading** -- the heading was already on the reference's column, and the other thing in the row is what
differs. That is the fourth time in this file that an ink box wider than its subject has answered for the wrong element, and the second time in
two rounds that the "obvious" 24-point shift was already correct.

**What that leaves**: the search field's 1 316 is the largest single band, and it is measurable -- its plate, its placeholder and its magnifier can
each be boxed separately, as the library's field was. That is the next piece of this page, and it is a search for *which* element rather than for
an offset.
### The search field was 48 points narrow, and the plate said so (2026-11-09)

Boxing the plate of the field, instead of the 20-row band that contains it, gave the answer in one measurement:

| | plate fill | plate rows | plate columns |
|---|---|---|---|
| reference | (52,54,60) | 117..150 | **89..1114** |
| this build | (52,54,60) | 117..150 | **89..1066** |

Fill and rows already matched; the plate ended **48 points early**, and 48 is `2 x PAGE_PADDING` -- the width still subtracted the padding that
the position no longer added. Removing it from the width closes the gap exactly: the plate now runs 89..1114 on both sides.

| | content | total |
|---|---:|---:|
| before | 8 396 | 27 745 |
| after | **7 646** | **26 995** |

**The band had said 1 316 differing pixels in rows 120-139 and could not say which element they were in.** The box said it in one line: same
fill, same rows, different right edge. That is the second time in two rounds that boxing one element beat banding a row, and the cause is the same
as the padding bug one round earlier -- a rect that is already padded, used as though it were not.
### The chips are 3 and 6 points narrow; the heading is not a size error (2026-11-09)

Boxing the two sort-chip plates, and the heading, after the search field:

| | reference | this build |
|---|---|---|
| first chip plate | 89..218 (129) | 89..215 (126) |
| second chip plate | 233..339 (106) | 228..328 (100) |
| the gap between them | 15 | 13 |

so the chips are **3 and 6 points narrow and the gap 2**, and the row ends 11 points short. Small, and in the same family as the field -- widths
derived from a text measure rather than a set number -- but not the same cause, and not worth a build on their own.

**The heading looked like an 11-point size error and is not.** Its ink measured 26 rows here against the reference's 16, which reads as a font one
step too large; at 1:1 the two are the same size and weight, and the 26 was the ink threshold catching antialiasing on a bold face at a different
subpixel phase. The same threshold that made the search field's 1 316 visible made this look like something it is not.

**The remaining 7 646 on this page** therefore sits in: the chips' 11 points of width, whatever the group header is doing (413 in rows 220-239),
the image's 2 166, and the ordinary type difference on every string. Each is measurable; none of them is a 750-pixel constant like the last two.
### Five sizes, re-run on the current build (2026-11-09)

The objective asks for verification at five standard sizes, and the numbers behind that clause predate most of this session. Re-run on Home, both
clients resized and the pointers parked:

| size | differing pixels | share of the frame |
|---|---:|---:|
| 1280x720 | **28 171** | 3.05% |
| 1366x768 | **35 839** | 3.41% |
| 1440x900 | **31 876** | 2.46% |
| 1600x900 | **45 815** | 3.18% |
| 1920x1080 | **57 677** | 2.78% |

**The per-slab split is only meaningful at 1440.** `compare.py` hard-codes its slabs for 1440x900, and the sweep shows what that does at other
sizes: the sidebar term reads 16 816 at 1440 and **1 196** at 1920, which is not the sidebar getting better but the slab covering content instead.
The totals are sound; the columns are not, and should be read at 1440 only.

**A suspicion tested and dropped.** That swing looked like the news carousel rotating between captures -- which would have made every sidebar
figure a race. The reference captured three times over 24 seconds changes by **0** pixels in the sidebar, so it does not rotate, and the article
difference recorded earlier is a real difference in which article is shown rather than a phase.
### The measuring tool was wrong at four of the five sizes (2026-11-09)

`compare.py`'s four slabs were literals -- `(0,0,1440,48)`, `(0,48,64,900)`, `(64,48,1140,900)`, `(1140,48,1440,900)` -- so at any other size it
measured its sidebar inside the content column and its content inside the rail. The tell was a sidebar term reading 16 816 at 1440 and **1 196**
at 1920 for the same page, which I had annotated as a slab artefact rather than fixed.

They are derived now: 48 points of title bar, a 64-point rail, and a sidebar that is a **fixed 300-point panel at the right**, not a fraction of
the width. The 1440 numbers are unchanged to the pixel (31 876 total, sidebar 16 816), which is the check that the derivation agrees with the
literals at the size they were written for.

**The sweep, re-split:**

| size | total | titlebar | content | sidebar |
|---|---:|---:|---:|---:|
| 1280x720 | 28 171 | 2 854 | 14 863 | **10 057** |
| 1366x768 | 35 839 | 2 729 | 22 618 | **10 095** |
| 1440x900 | 31 876 | 1 715 | 12 948 | **16 816** |
| 1600x900 | 45 815 | 2 995 | 25 606 | **16 817** |
| 1920x1080 | 57 677 | 3 237 | 26 363 | **27 680** |

**The sidebar grows with the window**: 10 057 at 1280, 16 816 at 1440, 27 680 at 1920. The panel itself is a fixed 300 points, so what is growing
is not its size but what it holds -- the news card is taller and its art larger at these widths, and this build's card wraps its copy differently
and scales its art differently. That is one earlier verdict (different article) plus a new one (different scaling), and the tool could not have told
them apart while it was measuring the wrong columns.

**Every per-slab figure in this file before today is 1440-only.** The totals stand; the columns at other sizes did not.
### A test for the tool that produces every number (2026-11-09)

`compare.py` had none, and the defect it just acquired a test for had been live since the first sweep. `--selftest` now asserts, at five sizes,
that the four slabs cover exactly the frame, sit inside it, and do not overlap:

| size | covered | frame | verdict |
|---|---:|---:|---|
| 1280x720, 1366x768, 1440x900, 1600x900, 1920x1080 -- derived | full | full | **ok** |
| 1920x1080 -- the old literals | 1 296 000 | 2 073 600 | **FAIL** |

The old literals pass at 1440 to the pixel (1 296 000 of 1 296 000) and fail at 1920 by three quarters of a million, which is exactly the shape of
the bug: correct where it was written, silently wrong everywhere else, and producing numbers that look like measurements.

The slabs moved into `slabs_for(width, height)` so the selftest exercises the same function the measurement does -- a test of a copy of the
arithmetic would have passed while the tool stayed wrong.

**Run it before a sweep.** `python3 tools/parity/compare.py --selftest` takes no captures and prints one line.
### The element tool now says when its box holds more than one thing (2026-11-09)

`element.py` has reported both a dominant colour and an extreme since it was written; my error was reading the extreme as the subject, twice --
once taking a magnifier glyph for a placeholder, once taking a chip's plate for its neighbour. The tool could not have stopped that, because it
never said how much of the box the dominant colour was.

It does now: `fill share`, plus a warning when it is under half --

```
  fill share ref 0.77
             our 0.78
```

for the field box, which is one element with its glyph and placeholder inside it, and

```
  WARNING    the ref box is not one element: its most common colour is only 33% of it, so `brightest` and `ink rows` answer for whatever else is in there
```

for a box straddling three.

**`--selftest`** builds both cases as synthetic images and checks the detection: one element with ink reports that element's fill and its brightest
pixel, and a three-way box reports a share under half. The first version of the test used a 50/50 split, which sits *exactly* on the threshold and
asserted a warning that correctly did not fire -- the test was wrong, not the tool, and it is now a three-way split.
### Two points of chip padding, 409 pixels (2026-11-09)

The chips were measured across eight rows each, taking the union so no glyph could break the run:

| | chip 1 | chip 2 | gap |
|---|---|---|---|
| reference | 88..221 | 230..342 | 9 |
| before | 88..218 | 227..334 | 9 |

so the chips were **3 and 5 points narrow with the gap already right**, and the width is `2 * padding + 2 * glyph + two gaps + the label measure`.
The label measures cannot be shared between two different strings, so padding is the only knob that moves both: 10 -> 12.

| | chip 1 | chip 2 | content | total |
|---|---|---|---:|---:|
| after | 88..222 | 231..342 | **7 237** | **26 586** |

Within a point on both, and the second chip's right edge exact. **409 pixels for two points of padding**, which is the best exchange rate since
the double padding -- and the same method: box the element, trust its fill, change one number, measure.

**The chips were 3 and 5 narrow because two different strings measured narrow**, and a shared constant cannot fix two different errors exactly. 12
splits the difference: chip 1 ends a point late, chip 2 a point early, and the row's total width lands on the reference's.
### Two fixes on the Screenshots page: chips then heading (2026-11-09)

| fix | evidence | content | total |
|---|---|---:|---:|
| chips 3 and 5 points narrow -> `SELECT_PADDING` 10 -> 12 | plates 88..221/230..342 against 88..218/227..334 | 7 646 -> **7 237** | 27 745 -> 26 586 |
| group heading `TEXT_PRIMARY` -> `BASE` | reference label is (176,186,197), exactly `BASE`; ours was (255,255,255) | 7 237 -> **7 133** | 26 586 -> **26 482** |

Both were found by boxing one element and reading its **fill**, the method that has worked every time this session: the chip plates gave edges to
the point, and the heading tone answered to (176,186,197), which is a theme token rather than a mixture. After the change the heading brightest
pixel is (176,186,197) on both sides and its ink box is the same 120..239 x 222..247.

**513 pixels for two numbers**, and both are now at the reference values rather than near them -- `SELECT_PADDING` splits two label measures that
cannot both be right, and `BASE` is exact.

The page has gone **39 083 -> 26 482** over four rounds, 12 601 of it from the double padding and the rest from these two. What is left is 7 133
of content, of which 2 166 is the image resample and the remainder is type on a page with a heading, two chips, a search field and a group header.
### Discover: the emoji in the API text, and two smaller differences (2026-11-09)

Discover re-measured on the current lab: **78 363** (content 58 360, sidebar 17 138) -- the largest populated page, and the second largest in the
project after the Skin selector.

At 1:1 the first two cards are otherwise the same object: same name, same author line, same summary, and the same five tag chips (Client,
Lightweight, Multiplayer, Optimization, Fabric). Three differences are visible:

| difference | what it is |
|---|---|
| the reference draws a **download glyph and a compact count** in front of the second card summary; this build prints **`????`** | characters this build has no glyph for -- emoji in the project summary, which is user-supplied API text |
| the author line has an **external-link mark** after the name in the reference | this build omits it |
| the card art | the reference shows real modpack icons, this build shows them too -- the art itself matches |

**The `????` is a real limitation, not a bug to fix this round.** `ab_glyph` has no emoji fallback, so any character outside the bundled faces
renders as a replacement box or a question mark. The reference uses the platform font stack and falls back to the system's emoji font. Closing it
would mean bundling an emoji face -- several megabytes, against a 20 MB RSS gate and a build that is already 2.5 MB over it. That is a trade this
project has already made once, for the text rasteriser, and the answer is the same: record the cause, do not pretend a knob exists.

The external-link mark is **not** in that category: it is a 16-point glyph this build already has (`Glyph::ExternalLink` or similar), simply not
drawn on this line. That is next round's work, and it is a call rather than a font.
### The external-link mark: measured, and why the first placement missed (2026-11-09)

The glyph exists in the reference and not in this build. Measuring it exactly, by taking the ink runs across the first card byline row:

| | value |
|---|---|
| the byline last ink column | **611** |
| the glyph box | **(619,249)-(632,262)** -- 14x14 |
| the gap between them | **8 points** |

**The first attempt drew it at `by_at.x + by.size().x + 13` and cost 171 pixels** rather than saving any: a 14x14 glyph can only cost that much
if none of its 196 pixels lands where the reference has one, so both axes were wrong at once.

Two reasons, both from the same habit of trusting a laid-out width:

- `by.size().x` is the galley **advance**, which carries side bearings and any trailing space; the reference gap is measured from the last **ink**
  column, 611. The advance can sit several points past it.
- `by_at.y + by.size().y / 2.0` centres on the **line box**, which for a 16-point body puts the glyph's centre several points above where the ink is --
  the reference glyph centre is at 255.5, and its own text's ink centre is not the line box's.

So the fix is to place it from ink, not from the layout: find the byline galley's ink end (or lay the string out with a trailing mark so the measure
carries it) and centre vertically on the text's ink rows, which for this card are 249..262's neighbours.

**Also recorded**: the same note that denied the glyph had been in the file for many rounds, specific enough to sound checked. It was not.
### The external-link mark is drawn, and the number goes up (2026-11-09)

The reference draws a 14x14 square-with-an-arrow 8 points after its byline (ink ends x 611, glyph 619..632). This build now draws the same
glyph after its own byline, and the page scores **78 534 against 78 363 without it** -- 171 pixels worse.

**Kept, deliberately.** At 10x the two marks are the same design; the reason ours lands wrong is that our byline is about ten points wider than
the reference's, so the mark follows the text further right. The metric is measuring our text width, which is a separate defect, and deleting a real
affordance to make a count smaller is precisely the fake measurement this project forbids.

**A correction to last round.** I had concluded the placement was wrong because 171 pixels is what a fully misplaced 196-pixel glyph would cost.
That was sound arithmetic on a wrong premise: the glyph's *shape* differs from the reference's by enough to cost nearly all of its own pixels even
when correctly placed, so the cost says nothing about position. The 10x crop says the position is right.

**The note that denied it** -- claiming `LegacyProjectCard.vue` prints `by {author}` and nothing else -- had stood for many rounds. It was specific,
cited a file, and was never checked. A subagent audit of the whole tree found nine more of the same shape; they are listed in the next entry.
### Two of the audit's claims, adjudicated against the running reference (2026-11-09)

A subagent listed ten comments that assert something *negative* about the reference ("the reference has no X"). Two were settled by driving
the reference, and they came out in opposite directions:

**Claim 4, supported.** `instance.rs:228` says the Screenshots tab is conditional on `showScreenshotsTabInInstances`. The reference's Settings
-> **Features** carries three toggles in a row -- "Show Worlds tab in instances", "Show Files tab in instances", "Show Screenshots tab in
instances" -- and only the third is **off** by default, while its neighbours are on. That is exactly the arrangement the claim describes, and it is
the first time it has been checked against a capture rather than against a cited line.

**Claim 5, refuted.** `settings.rs:2032` says "a signed-out body would be a state the reference does not have". The reference's Settings ->
**Profile**, signed out, shows an **illustration of a green robot**, "Modrinth account required", "Sign in with a Modrinth account to customize your
public profile.", and a green **Sign in** button with an arrow glyph. The claim is wrong: the reference has a signed-out body, and this build
already draws most of it.

**What the comparison found that no claim mentioned**: this build's Profile tab draws the same heading, copy and button, and **no illustration**.
That is a real gap on a tab that is now reachable in both clients, and it is worth more than either adjudication.

**Also visible in the same pair**: this build's Settings rail draws a scrollbar at its right edge where the reference's draws none -- the overlay
convention recorded earlier, still unfixed.
### The Create instance dialog: a ring at the right place, and two claims settled (2026-11-09)

The audit's claim 8 said the reference's instance-type cards carry **no chevron**. The dialog, opened in both clients, has four cards -- Custom
setup, Start from a mod or modpack, Upload a modpack, Import instance -- each an icon, a title and a subtitle, and **no chevron on either side**.
The claim holds, and the glyph that was removed was never there to remove.

What the same pair did find was the **search field**, which the reference focuses on open and rings in brand green. This build drew the plate with
no border at all. The ring is the one element in the dialog that is unconditionally drawn, so it makes a better landmark than the text:

| | brand ring box |
|---|---|
| reference | (461,255)-(978,298) |
| before | (456,220)-(983,259) -- 35 high, 10 wide, 4 short |
| after | **(461,255)-(978,298)** |

Three numbers and one stroke: the offset 104 -> 139, a five-point inset on each side, and a height of 44 rather than 40, with a two-point
`--color-brand` ring. The dialog plates themselves already matched -- (441,116)-(998,784) against (441,117)-(998,782) -- so the mismatch was
entirely inside a frame that was right.
### Three attempts to open the reference card menu, and a hover that did nothing (2026-11-09)

Adjudicating audit claim 3 ("the reference never deletes on the menu press") needs the menu open first. On the reference's Home, with the title
bar read back to confirm the page:

| attempt | result |
|---|---|
| press the card foot at (238,540), where this build puts its kebab | **no menu** |
| right-click the card at (170,450) | **no menu** -- the card is unchanged in the capture |
| hover the card at (170,450) after moving in from (700,870) | **no change anywhere on screen** -- `hover changed: None` at threshold 30 |

The third is the surprising one. This build draws three hover affordances on a library card -- a play pill, a kebab and, on the art, a selection
ring -- and all three were built by *driving the reference* in earlier rounds. The capture here shows the reference's card with none of them, at rest
or under the pointer.

**Not recorded as a fact, and here is why.** "Nothing happened" is the one result this session has repeatedly misread: the rail tooltip looked
absent twice and was there both times, once behind a settings modal and once the pointer was in the wrong place. The candidate explanations here are
that the pointer did not register (an `xdotool mousemove` without a subsequent motion event), that the instance's state hides the affordances
(never played, never installed), or that earlier rounds drove a *different* page and attributed it to Home.

**What would settle it**: a hover with two motions (move away, move back in), a capture of the card in a state where the instance has been played,
and a re-read of the earlier round's capture rather than its prose. Until then, claim 3 stays open and the three affordances keep the evidence they
were built on.
### The jump-in strip and the getting-started card, looked at rather than measured (2026-11-09)

Both at 1:1, after checking the two clients were on the same page (the first pair this round was not -- the reference was on Home and this build
on Files, which is obvious in the image and would have been a meaningless number from a diff).

**The jump-in strip** is the same object on both: the heading with its caret, a dashed row, the instance tile, the name in semibold and the green
"New instance" pill with its sparkle. Ours runs about four points wider -- the same trailing difference the name has everywhere -- and nothing else
differs at 1:1.

**The getting-started card** carries the same three rows in the same order with the same states, including the struck-through "Create first instance".
Sampling a row through the card at y 90: the sidebar's tinted background is (25,35,32) in the reference and (25,35,33) here -- **within one unit**, which
is the closest any large surface in this project has come.

**What the sample did find** is at the panel's right edge: x 1432..1436 reads **(66,68,74) in the reference and the background colour here**. That is the
reference's **sidebar scrollbar**, drawn as an overlay thumb because its sidebar overflows. This build draws none.

It is the mirror of an earlier finding -- the Settings rail drawing a scrollbar where the reference draws none -- so the two are not one policy
applied inconsistently: the reference draws an overlay thumb **when a panel overflows**, and this build's panels differ in whether they overflow and
whether they draw. That is a behaviour, not a tone, and it belongs with the other animation work rather than with the pixel fixes.
### The reference window was 1440x625, and two rounds of comparisons were between different sizes (2026-11-09)

Measuring the element at the sidebar's right edge printed the capture's size in passing: **`(1440, 625)`** for the reference against 1440x900
for this build. The reference's window had been left at a different height by the five-size sweep and had not been reset, so the last two rounds'
side-by-sides -- the jump-in strip, the getting-started card, the sidebar row sample -- were crops of windows of different heights.

Some of those observations survive, because they were made on elements pinned to the top (the strip, the card's first rows, the heading). The
**sidebar sample at y 90 does not survive in the way I read it**: with a 625-point window the sidebar's own content overflows, which is why the
reference had a 6x220 thumb at x 1432..1437 -- and at 900 it may not overflow at all.

**The rule, stated properly this time**: every capture pair needs its **size** checked as well as its **title**. `compare.py` refuses a mismatch,
but crops and pastes do not, so a whole round can be spent looking at two windows that are not the same shape. The check costs one line:
`Image.open(path).size` on both.

After resetting both to 1440x900 and re-capturing, Home reads **31 876** (titlebar 1 715, rail 397, content 12 948, sidebar 16 816) -- the figure
this session has been quoting, which is itself evidence that the earlier numbers came from the right size and only the recent *visual* pairs did
not.

**The scrollbar question, narrowed**: at 625 the reference's sidebar overflows and carries a 6-point overlay thumb at its right edge; at 900 it may
not. This build draws none at either size. Whether ours *should* needs one more capture at 625 -- and a wheel test that actually reaches the panel,
which the one I ran did not (both clients returned `None`).
### The sidebar's 6x97 element, measured and not yet explained (2026-11-09)

At 1440x700 (the window manager refuses 625 and clamps to 700) the reference carries a SURFACE_5 element that this build does not:

| property | value |
|---|---|
| columns | **1432..1437** -- 6 points, four in from the window's edge |
| rows | **51..147** -- 97 points, beginning just below the title bar |
| colour | (66,68,74), the theme's `SURFACE_5` |

A thumb that occupies 97 of a 652-point panel implies content about 4.3 times the visible height, which nothing in the sidebar is. So it is probably
**not** a scrollbar thumb, and the three candidates -- an overlay thumb for a longer list than the panel shows, a window-level scrollbar from the
web view itself, or a divider belonging to something not yet identified -- cannot be separated from one capture.

**What is settled**: the element is real, it is in the reference, it is absent here, and it is 6 by 97 at a measured position. What is not settled is
what draws it, and drawing something 6 by 97 to match a number without knowing that would be exactly the fabricated affordance this project's rules
forbid.

**Both windows are back at 1440x900**, verified by reading the capture sizes rather than assuming.
### A capture tool that refuses a bad pair (2026-11-09)

Three rounds lost to a pair that was not a pair -- a modal on one side, two different pages, and a reference window at 1440x625 against
1440x900 -- and each time the fault was visible only after a number had been produced from it. The checks were known and were run afterwards.

`tools/parity/pair.py` runs the whole sequence and refuses to hand back a pair that fails them:

```
python3 tools/parity/pair.py NAME [--size 1440x900] [--rail 65]
```

It resizes and moves both windows, optionally presses the same rail y on both, parks both pointers at the same place, captures, and then:

| check | on failure |
|---|---|
| both captures the same size | **refuses**, exit 2 -- a number from two sizes is meaningless |
| the size is the one asked for | warns (the window manager clamps: 1440x625 comes back as 1440x700) |
| the breadcrumbs agree | warns, and saves both strips for a look |

**The breadcrumb threshold is calibrated, not guessed.** The same page -- both clients on Home -- reads **419 of 23 600 (1.8%)** purely from glyph
rendering, so the warning fires at **15%**, the point at which the words themselves must differ. The first version warned at 200 and fired on a
correct pair, which is how the calibration was found.

The tool prints its two capture paths and the breadcrumb count on success, so a round can start from a pair it did not have to trust by hand.
### Discover, re-measured through the new tool (2026-11-09)

`python3 tools/parity/pair.py discV --rail 123` produced a pair it could vouch for -- both windows 1440x900, both on Discover (breadcrumbs 4.8%
apart, under the 15% floor) -- and `compare.py` reads **79 146** (titlebar 2 438, rail 427, content 59 142, sidebar 17 139).

That replaces 78 363, which came from a pair assembled by hand. The 783-point difference is not a regression: it is what a verified pair reads
where the earlier one may have been a moment or a state apart. From here the table quotes only pairs the tool produced.
### The news copy is 13 points, not 14 -- and 13.5 is worse than both (2026-11-09)

Looking at the two sidebar news cards at 1:1, with both showing the **same article** for the first time, showed our text heavier and wrapping
differently. Counting lit pixels over the same boxes: the headline differs by 6% (1 145 against 1 083) and the copy by **20%** (3 403 against
2 824). A headline that is close and a copy that is not points at one size, not at the face.

Four sizes measured, each on a verified pair:

| `COPY_SIZE` | sidebar | Home |
|---|---:|---:|
| 12 | 16 956 | 32 921 |
| **13** | **16 565** | **31 625** |
| 13.5 | 17 236 | 32 296 |
| 14 (was) | 16 817 | 31 877 |

**13, and the non-integer is worse than both neighbours** -- which is not noise: at 13.5 the copy wraps to a different set of lines than at either
integer, and the wrap costs more than the size gains. The optimum here is not a value to interpolate towards; it is one of two integers, and the
measurement picks it.

**252 pixels, and they are honest ones**: the copy is now nearer the reference in weight, which is what the 20% ink difference said was wrong.

This is also the first round where every number came from `pair.py` -- both windows the same size, both on the same page, both verified before
the comparison ran.
### The sidebar's two levers, both now at their measured optimum (2026-11-09)

The news card's copy and its measure were the last two knobs on the largest recurring cost in the project -- 16 565 on every page. Both have been
swept and both are at a turning point:

| lever | values measured | best |
|---|---|---|
| `COPY_SIZE` | 12 / **13** / 13.5 / 14 | **13** (16 565) |
| measure inset | **-20** / -27 | **-20** (16 565; -27 gives 16 700) |

**Why narrowing did not help**, though the diagnosis said it should: at 13 points this build fits one more word per line than the reference, so a
narrower box should force the reference's break. It does -- and the sidebar gets worse, because the break moves *and* all four lines shift with it,
and the shifts cost more than the break gains. The wrap is a symptom of a face that is a few per cent narrower per character, and neither the size
nor the box is the place to fix a face.

**So the sidebar's remainder is the text engine's**, recorded as such with both sweeps behind it rather than as an assertion. Home is **31 625**
with the sidebar at 16 565 of it -- and that figure appears on every page, which is why it has had four rounds.
### The face is within one per cent -- which weakens four of my own verdicts (2026-11-09)

Four times this session I concluded that a difference was "the text engine" and stopped: the news card, the card's box, the chips' padding and the
cinema of small offsets. All four rested on **one** comparison, the news line at 252.4 against 253. So I printed more, from the running build:

| string | our advance | reference ink | difference |
|---|---:|---:|---:|
| "Screenshots" at 24 semibold | **146.06** | **147** | **-0.6%** |
| "Share your server with friends directly" at 13 | 234.38 | -- | (253 at 14, so -0.2%) |
| "Jump in" at 24 | 91.44 | ~81 for the text alone | not comparable yet |
| "Today" at 16 | 47.19 | masked by its count badge | not comparable yet |

**The face is within one per cent of the reference's, and the advance/ink distinction makes it closer still** -- an advance carries side bearings
that ink does not, so our ink is a point or two narrower than 146. Against 147 that is a difference of about one per cent.

**So "the engine did it" is weakly supported, and I have used it as a stopping argument four times.** One per cent of a 147-point heading is 1.5
points, which cannot explain a copy that breaks a word later or a card that is 25 points off. Those need explanations of their own -- a measure
interacting with a trailing space, a line box, a different wrap rule -- and calling them "the engine" closed the investigation before it started.

**What stands**: the numbers in the table (our advances, printed by the build) and the reference inks (measured from captures). **What does not**:
the inference from them to "the face explains the remaining differences". The next round that meets a text-position difference should assume the
face is *close* and look for a mechanical cause first.
### The "copy is 20% heavier" was the link, not the copy (2026-11-09)

Last round's diagnosis for the news card -- "the copy is 20% heavier in ink, so the size is wrong" -- does not survive a check of the weight it
implied. `fonts::body` is **inter-400**, the regular face, which is what the reference's copy uses; there is no heavier weight to be wrong about.

And the box the ink was counted in, **(1150..1425, 536..620)**, contains the bright purple **"Upgrade to Modrinth+"** row, which sits under the copy
and crosses the same rows in both captures at slightly different heights. Purple link glyphs count as "lit" exactly like text does, so the count was
the link's position as much as the copy's weight.

**What survives**: the totals. `COPY_SIZE` 13 measures 16 565 against 14's 16 817, and 13.5 measures 17 236 -- three verified pairs, three numbers,
and the choice stands on them rather than on the ink explanation.

**What does not**: "the copy is heavier, so shrink it". The right lesson is not that the size is 13 but that an ink count is only meaningful when
the box holds one thing -- the same rule `element.py` now warns about, broken here by a box drawn around a paragraph and a link together.
### The RSS gate is met: 13 804 kB, not 22 500 (2026-11-09)

Chasing the last 2.5 MB, I measured the running build directly rather than quoting the table, and it came back **13 804 kB** -- below the 20 MB
gate with six megabytes to spare.

| measurement | value |
|---|---|
| fresh from start | 13 640 kB |
| after visiting six pages | 13 768 kB |
| after 45 s idle | 13 768 kB |
| on Discover with its artwork loaded | **13 804 kB** |
| back on Home | 13 804 kB |
| `RssAnon` / `RssFile` | 2 464 / 6 240 kB -- 2 500 kB anonymous in total |

**The 22 500 kB in `FINISHED.md` is withdrawn.** I cannot reproduce it, and I cannot say what produced it: the earlier note said only "22 500 kB, stable",
with a breakdown (`RssAnon 5 592`, `VmData 18 068`) that the current build does not match either. It may have been a different build, or a wrapper
process; the honest statement is that the number is not this program's.

**What the current numbers say about the design**: the heap is 2.5 MB, not the 5.2 MB a 1440x900 RGBA framebuffer would cost, so the software
backend is not holding a full-screen RGBA buffer; the file-backed share is 6.2 MB against an 11 MB binary, so less than two thirds of the code is
ever touched. Both are better than the assumptions the earlier analysis was built on -- including my own "the framebuffer and the code" verdict
two rounds ago, which was resting on a measurement I had not taken.

**So one of the two clauses this project recorded as unreachable is met.** The other -- 99% identity -- has the measurements behind it that it always
had. That is the position the objective should be judged from, and the table now says so.
### RSS is flat across all five sizes (2026-11-09)

The gate does not name a size, so it was measured at all five, on the live process, resizing between reads:

| size | `VmRSS` | `RssAnon` |
|---|---:|---:|
| 1280x720 | 13 720 kB | 2 504 kB |
| 1366x768 | 13 720 kB | 2 504 kB |
| 1440x900 | 13 720 kB | 2 504 kB |
| 1600x900 | 13 720 kB | 2 504 kB |
| 1920x1080 | 13 720 kB | 2 504 kB |

**Identical to the kilobyte at every size, including 1920x1080 where the window is 2.25 times the area of 1280x720.** So the software backend does
not hold a screen-sized buffer at all: the heap is 2.5 MB whatever the window is, and the drawing must be going straight into whatever surface the
presenter hands it. That also explains why the earlier "5.2 MB framebuffer" analysis was wrong -- there is no such buffer in this process.

Two measurement errors were made getting here, both worth recording because they are the kind that produce confident nonsense: `pgrep -x` returned
nothing while the process was alive, and `ps aux | grep` returned the **wrapper shell** (1 476 kB, `RssAnon` 4 kB) ahead of the program. The first
read of the five sizes was of a `bash` process, and it reported a flat 1 476 kB across all five -- which is exactly the shape of the right answer
and entirely wrong. `ps -C SiftUi -o pid=,stat=` with the zombies filtered is what works.
### The library card: every number measured, and the fix still measures worse (2026-11-09)

Measured on a verified pair, with the plate identified by its own fill and the artwork by its own green:

| | reference | this build |
|---|---|---|
| card plate | **354..568** | **329..544** |
| artwork (green runs at x 150) | **367..409, 468..481** | **367..409, 468..481** |
| the name | 515 | 505 |

So the artwork matches to the pixel, the plate is **25 points higher**, and the text block is **10 points higher inside it**. The obvious fix -- plate
+25, tile inset 36 -> 11 to keep the art still, name gap 8 -> 18 to bring the text down -- was applied and measured **13 763 of content against
12 948, worse by 815**.

That is the third arrangement tried for this card, and all three are worse despite matching the reference's measured positions. Something in the
premise is wrong, and the candidates are now narrow: either the plate's measured extent is of a different element than the card (the fill scan at
x 95 found the **chip row**, not the plate, until the x was moved), or the card's plate is not what the difference is being counted on.

**What is not in doubt**: the artwork aligns exactly, and the totals are reproducible (**31 625** with the file restored from HEAD). What is in doubt
is the identity of the 25-point band, and the next attempt should settle *that* -- by cropping the reference's card's top edge at 4x and looking at
what is actually between y 329 and 354 -- before moving anything again.

**A process failure worth naming**: while reverting, a line-replacement matched a `+ 25.0` belonging to a different expression and broke the build at
home.rs:1248. `git checkout` restored the file, and the working tree is clean. Two builds were lost to that, on a round where three were already
spent on a fix that did not work.
### Why matching the library card's plate never paid: the metric cannot see it (2026-11-09)

Six builds went into this card across three rounds, and the contradiction -- the plate is measurably 25 points high, and moving it onto the
reference's own position measures worse every time -- has a plain explanation that I should have reached three rounds ago:

| element | fill against the page | step | counts at threshold 40? |
|---|---|---|---|
| the card plate | (39,41,46) against (22,24,28) | **17** | **no** -- only its 1-px border does |
| its border stroke | (66,68,74) against (22,24,28) | 44 | yes, ~1 px per row |
| the name's glyphs | (255,255,255) against (22,24,28) | 233 | yes, every ink pixel |

So moving the plate can gain at most its border -- a couple of hundred pixels -- while moving any *text* by ten points costs a thousand. Every
arrangement that matched the plate and took the text with it lost more than it gained, and the one that matched the plate and held the text still
(+125) still gained less than the border cost at its new place.

**This is the same blindness that the project has recorded for the filter row and the field's placeholder tone**, now working in the opposite
direction: a difference that is obvious in a 4x crop is worth almost nothing to the count, and the count is what the objective asks for at threshold
40. Both statements are true and they point opposite ways, which is why the card has resisted six attempts.

**What to do instead**: match the *text* inside the card (name at 515, loader at 535) and let the plate land wherever it lands. The plate's
position is worth ~200 px and the text's is worth ~1 000; fixing them in the order of what the metric can see is the only order that pays. That is
the next attempt's shape, and it is a different experiment from the six that failed.
### The card's text first, then its foot: 269 pixels (2026-11-09)

The explanation from the previous entry -- the plate's fill is 17 points from the page and cannot pay for moving any text -- gives a different
experiment, and it worked on the first try:

| change | content | Home |
|---|---:|---:|
| baseline | 12 948 | 31 625 |
| **the name and loader 10 points down** (gap 8 -> 18), plate untouched | **12 699** | **31 376** |
| and the plate grown to 239 so its foot lands on the reference's 568 | **12 679** | **31 356** |

**-269 for two numbers**, after six builds of moving the plate had made it worse every time. The order matters exactly as the arithmetic said it
would: the text is worth a thousand pixels and the plate a couple of hundred, so moving the text first is the only sequence where the gains are not
swamped by the losses.

The card now has 38 points above its artwork against the reference's 13, and 20 below its loader against the reference's 21. The bottom matches;
the top is still 25 out, and closing that would mean moving the plate up into the text -- which is the experiment that failed six times.
### Five sizes, re-swept through the verified tool (2026-11-09)

Every size captured with `pair.py`, so both windows were confirmed the same size and both clients the same page before any number was taken:

| size | total | share | titlebar | content | sidebar |
|---|---:|---:|---:|---:|---:|
| 1280x720 | **29 888** | 3.24% | 2 854 | 16 581 | 10 056 |
| 1366x768 | **36 268** | 3.46% | 2 729 | 23 047 | 10 095 |
| 1440x900 | **31 356** | 2.42% | 1 715 | 12 679 | 16 565 |
| 1600x900 | **45 469** | 3.16% | 2 995 | 25 513 | 16 564 |
| 1920x1080 | **57 864** | 2.79% | 3 237 | 26 932 | 27 298 |

Against the previous sweep (28 171 / 35 839 / 31 876 / 45 815 / 57 677) every size is within about 600 and 1440 is 520 better, which is the card
and copy work of the last few rounds showing up at the one size it was tuned at.

**The sidebar's 10 056 at 1280 against 27 298 at 1920** is the news card again: at narrow widths it wraps to more lines but its artwork is smaller,
and at 1920 the artwork is large enough that the two builds' scaling differs across a bigger area. The panel is a fixed 300 points, so this is all
content, not layout.
### The 1920 sidebar is the artwork, not the text (2026-11-09)

The sidebar's difference grows with the window -- 10 056 at 1280, 16 565 at 1440, **27 298 at 1920** -- and the panel is a fixed 300 points, so it
had to be content. Banded at 1920, by 60-row slices over x 1620..1920:

| rows | differing | what is there |
|---|---:|---|
| 660-719 | **11 053** | the news artwork |
| 720-779 | **8 371** | the news artwork, lower half |
| 540-659 | 5 820 | the card's title and copy |
| everything else | ~2 000 | headings, banner, spacing |

**Two thirds of the 27 298 is the promo image**, and the share it takes grows with the window because the image is drawn larger: at 1920 the card is
390 points wide against 270 at 1440, so the same source is scaled over twice the area.

That is a **different mechanism from every text finding so far** -- it is the sampler, not the glyphs. This build draws through a software
rasteriser whose sampling is bilinear (`crates/sift-ui/src/soft/mod.rs`); the reference's is the compositor's. On a downscale of this ratio the two
filters disagree on most pixels of a photograph and on none of a flat fill, which is exactly the shape the bands show.

**The testable next step is a better filter, not a better layout**: a box or area average for large downscales is what a compositor does, and it is a
few lines in the sampler. If the artwork's 19 424 drops, the mechanism is confirmed; if it does not, the difference is the source image and the filter
is not the answer.
### The no-results state was drawn, 160 points too low (2026-11-09)

Driving a query that matches nothing into both clients showed the reference's **No matching screenshots** state and, in this build, an empty page.
The code had the copy and the branch (`visible.is_empty()` -> `empty_state`), so the strings were right and nothing appeared.

A probe settled it in one build: `visible=0`, `list_rect=[[88,208]-[1116,876]]`, so the state *was* being drawn -- **centred**. Measured on the
reference, the block is not centred at all:

| | reference | this build, centred |
|---|---|---|
| illustration | rows **273..383** | 411..509 |
| heading | rows **439..460** | ~530 |
| body | rows **473..484** | ~554 |

In a 668-point list area a centred 262-point block starts at 411 -- **138 points below the reference's** -- and with the state that low it sat under
the fold of a crop and read as absent.

**The fix is the source's own geometry**: the block is anchored **76 points below the list top**, with an 110-point illustration, a 56-point gap, a
21-point heading line, 13, and an 11-point body line. The 76 rather than 65 came from the probe: the list area actually starts at **197**, not the 208
the page's padding suggests, so 65 left everything eleven points high.

**Measured after**: the no-results screen at 1440x900 reads **39 559** (content 20 461, sidebar 16 565). Both clients draw the same illustration,
the same heading and the same body at the same height, where one of them previously drew nothing.

**Why this state had never been caught**: every earlier capture of this page was of a populated library or the *empty library* state, which is a
different branch and which sits correctly. The no-results state needs a query that matches nothing, and no round had typed one.
### The dropdown's chosen row is a green plate, not a tick (2026-11-09)

Driving the Screenshots page's sort dropdown on both clients, and then its group-by dropdown on the reference to check whether the style was the
sort menu's or the component's:

| | reference | this build, before |
|---|---|---|
| the chosen row's plate | **(46,94,72)**, a dark green | the same (52,54,60) as the rest |
| the chosen row's label | **brand green** | `CONTRAST` |
| a mark at the row's right end | **none** | a `Check` glyph |

The sample is the evidence: three points inside the chosen row all read (46,94,72) and two points inside an unchosen row all read (52,54,60), with the
menu's own plate at (52,54,60) -- so the green is the row's, not the menu's.

`open_list` is shared by every dropdown on the page, so the fix is in one place: fill the chosen row with (46,94,72), set its label in `theme.BRAND`,
and drop the `Check` glyph. The hover fill still applies to the other rows and no longer competes with the selection, since a chosen row is never
overpainted by a hover.

**Still different, and now recorded rather than fixed**: the reference's sort chip reads **Newest ^** -- its chevron is the *direction* -- where this
build reads **Newest v**, the same dropdown chevron every other chip carries. Its Date chip carries v in both, so the sort chip's chevron is a
special case: up for newest-first, down for oldest-first. The glyph is hardcoded in the shared chip helper and needs a parameter; that is the next
round's first item, with `sort-ref.png` and `sort2-our.png` behind it.
### The sort chip's chevron is a direction, and an index of where things live (2026-11-09)

The chip now reads **Newest ^** against Date's **v**, matching the reference: the sort chip's chevron is the sort *direction*, not a dropdown
indicator. `select` gained a `chevron: Glyph` parameter, the sort call passes `ChevronUp` for `Sort::Newest` and `ChevronDown` otherwise, the
group call passes `ChevronDown`, and the helper draws whatever it is given. Both chips verified at 2x against the reference's own.

**`docs/COMPONENTS.md`** now records where each shared element lives, verified by grep. Writing it took one command and immediately caught two
things I had wrong from memory: `menu_placement` is in `home.rs`, not `shell.rs`, and `open_list` -- the dropdown renderer that cost most of the
previous round to find -- is in `screenshots.rs` with no module of its own.
### The group menu: read from the reference's own source, not its UI (2026-11-09)

Driving the group-by dropdown failed three times (the open sort menu swallowed the clicks, twice producing byte-identical captures), so this
went to the source instead -- which is in the workspace, 2 044 `.vue` files under `.scratch/modrinth-0.22/`. The screenshots page is not where the
comments said: `pages/instance/screenshots/index.vue` is an 11-line wrapper, and the real component is
**`components/ui/screenshots-page/index.vue`**, 1 708 lines.

From it, verbatim:

```js
type ScreenshotGroupBy = 'custom' | 'instance' | 'date' | 'none'
const sortOptions = [newest, oldest, name]
const groupOptions = [
  { value: 'custom',   label: 'Custom group' },
  ...(isGlobal ? [{ value: 'instance', label: 'Instance' }] : []),
  { value: 'date',     label: 'Date' },
  { value: 'none',     label: 'No grouping' },
]
```

**What matches**: every label -- including "No grouping", which I had doubted -- and the three sort options.

**What does not**:

| | reference | this build |
|---|---|---|
| order | **Custom first**, then Instance, Date, None | Instance, Date, None, then the groups |
| custom entries | **one**, `custom` | **one per group**, appended, each labelled "Group" |
| `instance` offered | **only when `isGlobal`** | always |

So the menu's *shape* differs, not just its order: the reference has a single "Custom group" mode and groups by whichever groups exist, while this
build lists each group as its own mode. That is a state-model difference rather than a drawing one -- the chip's label, the stored value and the
grouping function all key off it -- so it is the next round's work rather than a line changed at the end of this one.

**The general lesson**: three attempts to drive a dropdown cost a round and produced nothing; reading the component took three commands. The source
has been in the workspace the whole time and answers questions a capture cannot -- the *conditional* (isGlobal) and the exact string set among them.
### The group mode is one entry that groups by all of them (2026-11-09)

The state-model difference recorded last round is fixed, on the reference's own shape rather than a guess:

| | before | after |
|---|---|---|
| the mode's value | `Custom(group_id)` -- a payload | **`Custom`** -- a unit mode |
| the select's entries | instance, date, none, then one per group | **custom, instance, date, none** |
| a group's entry label | "Group", one per group | **"Custom group"**, once |
| the grid under `custom` | filtered to that one group | **one section per group, then "Ungrouped"** |

The sections follow the source exactly: `custom:${group.id}` titled with the group's name, and a final `custom:ungrouped` titled
**Ungrouped** holding every shot whose assignment names no live group. Shots arrive already in the reader's sort order, so each
section keeps it and no second sort is needed.

**Two tests changed with the model**, which is the point of having them: `a_custom_group_holds_only_its_assigned_shots` became
`the_custom_mode_sections_every_group_then_lists_the_rest_ungrouped` and now asserts both a group's section and the ungrouped one; and
the select's test asserts the four labels in the reference's order. **269 pass.**

**Captured**: the open menu reads Custom group / Instance / Date / No grouping, Date selected on the green plate, with "Custom group"
on the chip behind it.

**Not done, and it is in the source**: `instance` is offered only when `isGlobal`. This build's Screenshots page *is* the global one, so
the condition is satisfied today -- but there is no instance-scoped variant of this page to satisfy it for, so the flag has nothing to
switch. Recording it as an open question rather than implementing a condition with no branch to guard.
### A fabricated element that was not there, and what showed me (2026-11-09)

Reading `toolbar.vue` (87 lines, the whole component) gave the page's row structure precisely:

```html
row 1: <Input icon=Search placeholder="Search" clearable /> + <Button><SquarePlusIcon /> New group</Button>
row 2: <Combobox sort prefix=ArrowUpDown dropdown-min-width=160px />
       <Combobox group prefix=LayoutGrid dropdown-min-width=160px />
```

Grepping the component directory for a filter control returned only the empty-state copy ("Try a different search or filter.") and
JavaScript `.filter()` calls -- **no filter UI anywhere in the reference's screenshots page**. But this build appeared to carry a
funnel and an "Add filter" control beside the two chips, which would have been a fabricated element of exactly the kind the rules
forbid.

**It was not.** Capturing the whole frame instead of a crop showed the page's toolbar as Search, New group, Newest ^, Date -- correct,
matching the source line for line -- and the "Add filter" sitting *inside the screenshot card's picture*. The fixture for this page is a
capture of this app's own window, so the thumbnail contains a complete miniature Home page: its chips, its "Add filter", its "New group"
pill and its news card. At 1:1 in a crop, none of that is distinguishable from UI.

**Two things this cost and two it taught**:

* the hypothesis was wrong, and it was caught before anything was deleted -- the round's only value is the negative result;
* the capture that settled it took one command, and it is the same command that would have prevented the confusion: **look at the frame,
  not the region**, when the region's content is unfamiliar.

**The toolbar is now confirmed against the source rather than assumed**: row 1 is the search field and the New group button, row 2 is the
two comboboxes with their arrow and grid prefixes, and there is no third control. That also confirms the menu order fixed last round sits
under the right chip.

**A hazard worth writing down for later rounds**: a fixture that depicts the interface is a bad fixture for comparing the interface. Every
future crop of this page has to remember that roughly a third of it is a picture of the app.
### The card's second line is a choice, not a pair (2026-11-09)

`card.vue`'s footer, verbatim:

```html
<div v-tooltip="screenshot.file_name" class="truncate text-sm font-semibold">{{ screenshot.file_name }}</div>
<div class="truncate text-xs text-secondary">
  {{ showInstanceName ? screenshot.instance_name : formatTime(screenshot.created_at) }}
</div>
```

and `index.vue:1574` sets `:show-instance-name="isGlobal && groupBy !== 'instance'"`.

**So the second line is either the instance or the time, never both** -- and on a global page grouped by date it is the **instance alone**.
Measured on both clients' cards at rest:

| | second line |
|---|---|
| reference | **Fabric 26.3** |
| this build | **Fabric 26.3 · Nov 9, 2026, 3:18 PM** |

This build formats both at `instance_and_date` (`screenshots.rs:1120`), which the doc comment there says correctly is the *preview's*
description for a global screenshot (`index.vue:668-673`) -- a different element from the card's second line. The card is drawing the
preview's string.

**The conditions to carry**, since they are three-valued rather than one:

* `isGlobal && groupBy !== 'instance'` -> the instance name;
* otherwise -> the formatted time, in the reference's `dateStyle: 'medium', timeStyle: 'short'` spelling, which `stamp_label` already matches;
* the card's own `v-tooltip` on the first line is the file name, which this build also has.

**Not fixed in this round**: the card's footer is drawn through a shared helper that I did not locate before the round ran out, and a
text substitution in this file has already broken a build once. The next round starts by finding where the card's subtitle string is
passed in (`grep` for the caller of whatever draws `file_name` at `screenshots.rs:2720`) and changes it there, with the capture pair
`cardhover-ref.png` / `cardhover-our.png` as the before and after.
### The card's second line now says one thing, and the card's buttons say the wrong things (2026-11-09)

**Fixed**: the card's second line was drawing `instance_and_date`, the string the *preview* uses, where `card.vue:240` draws
`showInstanceName ? instance_name : formatTime(created_at)`. The flag is threaded as `show_instance` from the page's `group_by` through
`draw_card` to `draw_overlay`, set to `group_by != GroupBy::Instance` -- the reference's `isGlobal && groupBy !== 'instance'` with the
global half true by construction.

Verified on both clients' cards at rest: the reference reads `2026-11-09_12.00.00.png` / **Fabric 26.3**, and so does this build.

**Found while verifying, and not fixed because fixing it needs machinery this build does not have**: the card's three hover actions are
the wrong three.

| | reference (`card.vue:248-275`) | this build (`screenshots.rs:2977-2982`) |
|---|---|---|
| first | `EditIcon` -- edit | `Action::Open` (a picture glyph) |
| second | `ClipboardCopyIcon` -- **copy** | **`Action::Delete`** (a trash glyph) |
| third | `MoreIcon` | `Action::More` |

**The middle control is destructive where the reference's copies.** Drawing a clipboard glyph over a delete would be worse than the current
mismatch -- it would promise a copy and perform a deletion -- so the glyphs stay until a copy action exists. That is real work: copying a
file to the desktop clipboard through X11 selection ownership, with the failure path the reference has (the icon becomes a check).

What the reference's *edit* does is also unestablished: the card's own click already opens the viewer, so the pencil is something else --
most likely a rename, which `card.vue` would pass up as `emit('edit')`. `index.vue`'s handler for `@edit` is the next thing to read.

**So the card now matches on its text and is recorded as wrong on its actions** -- which is the honest state, and better than matching the
icons while the behaviour underneath lies.
### The card's buttons are now copy and more, and the copy is unproven (2026-11-09)

The reference's three actions are `editScreenshot`, `copyScreenshot` and `showScreenshotOptions`, and reading them settled what each does:

* **copy** reads the screenshot's PNG and writes it to the clipboard as `image/png`, then marks the card copied for **2 000 ms**, during
  which the glyph is a check (`markScreenshotCopied`);
* **edit** opens the *viewer* at that index and calls `imageViewer.edit(index)` -- it is an edit mode inside the preview, not a rename;
* **more** opens the options menu.

This build had `[Image, Trash, More]` wired to `[Open, Delete, More]`, so its middle control deleted where the reference copies.

**Changed**:

| | before | after |
|---|---|---|
| buttons | image, trash, kebab | **clipboard-copy, kebab** |
| actions | open, delete, more | **copy, more** |
| copy | -- | `arboard` writes the decoded RGBA to the clipboard; the glyph becomes a check for two seconds |
| delete from the card | yes | **no** -- it stays in the card's menu, where the reference keeps it |

The edit button is **absent rather than mislabelled**: this build has no edit mode inside the viewer, and a pencil that opens a plain preview
would promise something it does not do. Two honest buttons and one missing is a better state than three buttons of which one lies.

**Not verified, and stated plainly**: the copy was never executed. The click that was meant to press it landed on the card instead and opened
the viewer -- the screenshot of that moment is the viewer, not a crash -- so `arboard`'s write is untested on this X server, and so is the
two-second check. The next round presses the button by its measured rect (the copy slot is `action_button_rect(rect, 0)`, which at the card's
current box is its own rect from the frame probe) and reads the clipboard back with `xclip -t image/png -o`.

**Also worth recording**: `arboard` was added as a dependency, which is the first thing this project has added to the binary in a while. It
built, but its cost to the 13 720 kB RSS figure has not been re-measured, and the RSS gate is the one clause that is currently met.
### The RSS gate holds until one screenshot is opened (2026-11-09)

Adding `arboard` for the card's copy button, I re-measured RSS as promised, and it read **28 108 kB** against the 13 720 kB certified two
rounds ago. The dependency is not the cause, and the numbers say so:

| | before | after |
|---|---:|---:|
| binary | 11 098 056 | **11 344 192** (+246 kB) |
| `RssAnon` | 2 464 kB | **10 840 kB** (+8 376 kB) |
| `RssFile` | 6 240 kB | 12 248 kB |

A 246-kilobyte binary cannot cost eight megabytes of heap. The heap growth is the **image cache**: this session opened the full-size viewer once,
which decodes a 1440x900 screenshot to RGBA -- 5.2 MB -- and uploads a texture of the same size. Closing the viewer, leaving the page and
returning to Home all leave it in place (28 112, 28 152), so a single preview makes the process permanently ~2x its idle size.

**So the gate's figure has a condition it did not have before**: 13 720 kB is the size of a process that has not opened a picture. The gate says
"idle RSS under 20 MB", and 28 MB after one preview is not idle in the strict sense -- but a launcher whose memory doubles the first time a
reader looks at a screenshot is not meeting the spirit of it either.

**The fix is eviction, not a smaller cache**: the decoded image and its texture should be dropped when nothing has drawn them for a frame or two.
The cache is `crates/sift-ui/src/images.rs`; the question is whether it keys on use-per-frame or keeps everything it has ever loaded, and the
measurement above says it is the latter.

**Recorded as a regression against a met gate** -- the first time this session has broken one. It is not caused by the copy button, but it was
found by checking the copy button's cost, which is the argument for re-measuring after any dependency change rather than assuming.
### Thumbnails decoded at thumbnail size -- and the gate is still broken (2026-11-09)

The image cache held a screenshot whole: a 1440x900 file decoded to 5.2 MB of bitmap and uploaded as 5.2 MB of texture, to fill a card 510
points wide. `get_file` now has a capped sibling and the card asks for **640 on the longest side**, a separate cache entry from the original so
the viewer can still open the full one.

| state | before the cap | after |
|---|---:|---:|
| the grid | 27 800 kB | **24 920 kB** |
| the viewer open | 27 812 kB | 38 840 kB |
| back on Home, viewer closed | 27 844 kB | 38 840 kB |

**320 kB on the grid, and the gate is still not met.** Two things are wrong and one of them is not this fix:

1. **The baseline has grown from 13 720 kB to 22 504 kB** since round 348, with the app on Home and no screenshot ever opened. That is not
   the cache, not the thumbnails and -- measured directly -- not `arboard`: a build with the dependency removed read 27 564 kB against 27 800
   with it. What it is, is unattributed, and attributing it means bisecting builds, which costs rounds this loop does not have.
2. **The viewer's 14 MB is never given back.** Opening one picture takes the process to 38 840 kB and closing it, leaving the page and
   returning to Home all leave it there. The 8 MiB budget did not evict it, which says the eviction runs on insert and not on use, or that
   something still holds the handle.

**So `FINISHED.md`'s RSS row must be corrected again**: it currently claims MET at 13 720 kB, and that figure is not this build's. The clause is
**NOT MET** on the current tree, with the numbers above, and the two causes named but not yet fixed.

**This is the second time a met gate has been recorded on a number that stopped being true**, and both times the correction came from a
re-measurement prompted by something else entirely. The lesson is not about RSS: it is that a gate's row needs the date and the build it was
measured on, because a figure without them silently becomes a claim about code that no longer exists.
### The RSS "regression" was the environment, and one build proved it (2026-11-09)

Two rounds were spent suspecting the copy button, then the image cache, of an eight-megabyte growth. The experiment that settled it is the one
I said cost too much: **build the older tree and measure it in the same session**.

| tree | state | `VmRSS` | `RssAnon` | `RssFile` |
|---|---|---:|---:|---:|
| round 348's build, measured then | Home, fresh | **13 720** | 2 464 | 6 240 |
| commit `6e05c3c`, rebuilt today | Home, fresh | **22 472** | 5 604 | 11 804 |
| the current tree | Home, fresh | **22 504** | 5 608 | ~16 900 |

**The old tree reads the same as the new one**, so no code changed this. What moved is `RssFile`: 6 240 kB against an 11 MB binary meant only
about half the executable was resident when the gate was certified, and 11 804 means essentially all of it is now. That is the kernel's page-cache
and mapping behaviour under the container's current memory state, not anything in this project.

**So the correct statement of the gate is conditional, and that is what `FINISHED.md` now says**: the clause was met on this lab as it was running in
round 348 and is not met as it runs now, with the tree unchanged in the relevant respect. A gate whose number depends on the host's page-cache
pressure is a gate that cannot be asserted from one measurement -- it needs the date, the build and the host state, and only the first two are
recorded by anything in this repository.

**What remains a real defect**: opening one picture adds 14 MB and no path gives it back -- not closing the viewer, not leaving the page. That one
is the code's, and it is not explained by any of the above.

**Cost of the detour**: three builds and most of two rounds, against the one build that would have answered it at the start. The instinct to
blame the most recent change was wrong twice -- first arboard, then the cache -- and both times the reasoning was 'it changed just before the
number did', which is not evidence.
### Trimming the allocator after a synchronous decode: 5 MB of the 14 (2026-11-09)

The viewer's 14 MB survived three explanations. Eviction was the second: the cache budget was cut to 4 MiB, below one full-size
texture, so that the next grid insert would drop it -- and memory went **up**, 38 840 to 45 668 kB at the viewer, because the budget
then evicted the grid's thumbnails too and they were decoded again on the next frame. Reverted to 8 MiB.

The third explanation was right. `fetch` -- the asynchronous path -- calls `trim_heap()` when a reply lands, and `trim_heap` is
`malloc_trim(0)` on Linux. The **synchronous** file decode, which is what the viewer uses, never called it, so the 5 MB bitmap that
became the texture stayed in the allocator as a free arena. Adding the same trim after any synchronous decode of a megabyte or more:

| state | before | after |
|---|---:|---:|
| grid | 24 920 kB | 25 036 kB |
| viewer open | 38 840 kB | **33 828 kB** |
| viewer closed | 38 840 kB | 33 872 kB |
| back on Home | 38 840 kB | 33 924 kB |

**Five megabytes back at the peak, and the plateau is unchanged**: closing the viewer still leaves the process about 9 MB above the grid,
because the texture itself -- 5.2 MB -- is a cache entry under the 8 MiB budget and nothing evicts an entry that is under budget. Evicting
it means removing that path's entry when the viewer closes, which needs a `forget` on the cache and a call at the close handler; that is the
next step and it is small.

**The general lesson, which is not about this viewer**: a cache that only evicts on insert cannot release the thing the reader just stopped
using. Whatever the budget, the last-used entry stays until something else arrives to push it out.
### The texture is released; RSS never follows it (2026-11-09)

`SIFT_SOFT_MEM` prints the software backend's own texture total, which is the measurement this needed all along:

| state | backend textures | RSS |
|---|---:|---:|
| the grid | **3 019 kB, 7 textures** | 24 964 kB |
| the viewer open | **8 081 kB, 8 textures** (+5 062) | 33 776 kB |
| viewer closed, still on the page | 8 081 kB, 8 | 33 788 kB |
| back on the grid | **3 019 kB, 7** | 39 296 kB |

**So the `forget_file` fix works**: the full-size texture leaves the backend when the cache drops it, and the 5 062 kB it was holding is
gone by the backend's own count. What does **not** happen is RSS coming down with it -- it went *up*, from 33.8 to 39.3 MB, on a frame that
released five megabytes of texture.

**That settles what the RSS plateau is**: not live textures. The software backend accounts for every byte it holds and the count is small
(3-8 MB) and honest, so the ~23 MB of RSS on an idle Home is **allocator retention and fragmentation**, not images. `malloc_trim` is
already called on the path that frees the texture and it does not give the pages back, which is consistent with the freed blocks being small
or interleaved rather than one mmapped arena.

**Two consequences**:

1. Future memory work should quote **backend texture bytes alongside RSS**, because they are different quantities and only one of them is
   under this program's control. A probe already existed and two rounds were spent without it.
2. Chasing RSS by changing caches is the wrong instrument: the previous round's 4 MiB experiment moved RSS by seven megabytes in the wrong
   direction while the texture accounting barely changed.

**What is still true**: `forget_file` is a real fix -- a preview no longer pins its picture for the life of the process -- and the viewer's
texture now goes when the reader leaves the page, even though the operating system is told nothing about it.
### The environment chip's glyph follows its value (2026-11-09)

Discover's difference is spread evenly across every row -- 4 000 to 10 000 in each 80-row band, with no band standing out -- which is the
signature of something repeated on every card rather than one bad element. At 1:1 the repeated thing was the **environment chip's icon**: the
reference draws a **monitor** for a client-only project, and this build drew a **globe**.

The comment above that line asserted the opposite, that the reference carries the world mark for both cases and a monitor only as a fallback
this build never drew. It is wrong in the way the codebase's negative comments keep being wrong: half right. The globe *is* the mark for
"Client and server"; the monitor is the mark for a single side. Both are visible in the same reference capture, one card apart.

The glyph is now chosen with the label: both sides -> globe, client only -> monitor, otherwise the globe. Verified at 2x against the reference:
both read a monitor followed by Client.

**The measurement barely moved: Discover 79 146 -> 79 115, thirty-one pixels.** That is not a contradiction; it is the metric being blind in a
way worth recording. A 14-point glyph inside a chip shares most of its ink with any other 14-point glyph of similar mass, so at threshold 40 the
two icons differ by about one and a half pixels per card. The defect was real, visible at 1:1, and worth fixing; the count could not have found
it and cannot be used to justify it.

**Which is the counterexample to the last several rounds**: fixing what the number points at has been the productive loop, and here the number
pointed at a region while the defect needed an eye on a single element. Both instruments, in that order -- band the region, then look inside it.
### Two OPEN rows were already implemented (2026-11-09)

The matrix has 443 OPEN rows and the project has a history of six of them being complete and merely unreached. This round tested whether that
is a pattern: **138 OPEN rows claim their element is absent**, and two of the first four examined were wrong.

| row | claim | what the code says | verdict |
|---|---|---|---|
| 42 | "no selection circle" | the circle, its 50-point hit area and its selected fill are all drawn (`screenshots.rs`: `CHECK_HIT = 50.0` and the overlay's check) | **VERIFIED** |
| 103 | "group and its search absent" | the group is drawn at `panel.rs:1065-1079`; only the 16rem scroll panel and its search Input are missing | **OPEN-WITH-CAUSE** |

**Row 42 was self-contradicting**, which is how it was caught: its *states and animation* cell already described our circle in detail -- "24-point circle
in a 50-point hit area, drawn on the art's top-right; a press 22 points from the centre still selects" -- while its *gap* cell said "no selection
circle". A row whose two cells disagree is a row nobody re-read after the work landed, and the states cell is the one with the measurement in it.

**Row 103 is a partial**, which is the more common shape: the group exists, the scroll panel and search do not. Converting it all the way to VERIFIED
would have been the same error in the other direction, so it went to OPEN-WITH-CAUSE with the file and line that names what is there and a sentence
naming what is not.

**The rate is the problem.** Two rows took about twenty minutes, most of it finding the code the row is about, and there are 138 absence-claims and
54 rounds left -- 405 rows at 7 a round. The next round should build the search instead of doing it by hand: a script that takes each candidate
row's element name and looks for its keywords in `crates/sift-ui/src`, printing the candidate `file:line` for a read rather than a grep by hand.
### A finder for already-implemented rows, and six more closed (2026-11-09)

The hand search cost twenty minutes for two rows. `tools/parity/rows.py` replaces it: for every OPEN row it pulls the **quoted literals**
out of the reference cells and greps `crates/sift-ui/src` for them. Reference copy is verbatim from 0.22.0, so a literal that exists in our
source means the element that shows it very probably exists too.

```
python3 tools/parity/rows.py --limit 14     14 candidates from 228 OPEN rows in a few seconds
```

**It found candidates and it also found its own trap.** Row 309's "Export ZIP" matched at `screenshots.rs:85` -- inside a doc comment that
says Export ZIP is *not* implemented. A comment about an absence contains the absent feature's name, so a hit must be read before it is
believed, and the tool's job is to make the reading cheap rather than to skip it. Every hit below was checked for a *draw* call, not just a
string.

| row | element | the code |
|---|---|---|
| 17 | hero title | `home.rs:844` -- "Welcome to Modrinth", drawn |
| 18 | hero description | `home.rs:852` -- "Ready to start playing?", drawn |
| 118 | Discover, no results | `shell.rs:3597` -- the exact sentence, drawn |
| 266 | skins demo account bar | `skins.rs:1990` -- the button, from `SIGN_IN_MICROSOFT` |
| 304 | screenshots empty state | `screenshots.rs:1316` -- through `empty_state` |
| 305 | screenshots no-results | `screenshots.rs:1413` -- driven and measured this session |

**Six rows in one pass against two by hand**, and the counts moved from 56/443 to **62/437**. The remaining lead list is printed by the tool and
each pass costs seconds, so the limit is reading rather than finding.
### The literal signal is exhausted, and it says so (2026-11-09)

After the first six, `rows.py` returns **nine** candidates from 440 OPEN rows, and eight of them are **comments**:

| row | literal | where it matched |
|---|---|---|
| 12 | Hide already installed | a doc comment about the wording being wrong |
| 55 | New instance | a doc comment describing the toolbars press |
| 212 | Modrinth App {version} | a module doc listing the reference keys |
| 248 | Reset to default | a doc comment introducing the row |
| 259 | Drag to rotate | a module doc citing the reference line |
| 279 | Default skins | a doc comment about the strip columns |
| 309 | Export ZIP | a doc comment saying Export ZIP is NOT built |
| 598 | Sift Launcher | an unrelated string in `servers.rs` |

One was real: **row 97**, the locked game-version pill, drawn at `panel.rs:1079` with the loader interpolated. Converted.

**So the finder has drained what it can find.** Its signal is a verbatim string, and the strings that remain are the ones our code mentions only
in order to say it does not have them. That is not a failure of the tool -- seven rows came out of it in one round against two by hand -- but it
sets the ceiling for this method, and the next pass needs a different signal: the reference element names and their Code tokens (`StopCircle`,
`ScrollablePanel`, `EmptyState`), searched for as identifiers rather than sentences.

**Counts after this round**: 164 DONE, **63 VERIFIED**, 436 OPEN, 38 OPEN-WITH-CAUSE.
### The false-negative vein is nearly dry, and that is the finding (2026-11-09)

Two rounds of hunting rows that were already implemented, with the search scoped to the file each row is about:

| pass | signal | candidates | real |
|---|---|---:|---:|
| 1 | verbatim literals | 14 from 228 rows | 7 |
| 2 | identifiers (`StopCircle` -> `stop_circle`), scoped by page | 7 from 439 rows | 2 |

The second pass's five false leads are instructive: `empty_state` exists in `screenshots.rs` and not in `instance.rs`, so an unscoped name hit is
evidence of nothing -- which is why the area-to-file map went in before the numbers were trusted. What is left after both passes is mostly rows
whose literal appears **only in a comment saying the feature is missing**, which is the opposite of a lead.

**So the hypothesis is answered**: this project's six wrongly-recorded rows were real but rare, and the 436 that remain are mostly genuinely open.
Nine rows were closed across two rounds (42, 97, 103, 17, 18, 118, 266, 304, 305, plus 129 and 616 today), which is 2 per cent of the OPEN list,
not the third the hand-work suggested.

**Two of the eleven were rows that contradicted themselves** -- row 42's states cell described our circle while its gap said none, and row 616's
ours cell said "in use: yes, central" while its status said OPEN. That is a cheaper signal than any grep: a row whose cells disagree needs a read,
and it says where to start. The next pass should look for those first, because they are self-identifying.
### The copy button froze the window; it no longer does, and the clipboard still gets nothing (2026-11-09)

Pressing the card's copy button with `arboard::Clipboard::new()` on the frame thread left the window **drawn black with the process alive and
the log empty** -- a UI thread parked inside the clipboard library. That is worse than the wrong glyph it replaced: a button that appears to
work and kills the window is a defect a reader would report as a crash.

**Fixed**: the bytes are read on the frame thread (a local file, milliseconds) and the clipboard work happens on a worker. Pressing copy now
leaves the window drawn -- 65 dark samples of 12 298 across the page area, against a black frame before.

**Not fixed**: the clipboard still reads zero bytes. Two attempts:

1. `set_image` from the worker and drop the handle -- an X11 selection belongs to the client that set it and is served on demand, so dropping
   `Clipboard` released ownership before anything could paste.
2. Hold it with `std::mem::forget(clipboard)` -- ownership survives, and `xclip -t image/png -o` still returns an empty file.

So the failure is upstream of ownership: either `Clipboard::new()` fails on this display, or `set_image` does, and the code cannot currently
tell the two apart because both are swallowed. **The next step is a probe, not a fix**: an `eprintln!` on each `Result` in the worker, which
says in one build whether there is a clipboard at all on `:100`.

**What this round established**: the freeze was real and is gone; the clipboard write was never the freeze and is still unexplained. Two
builds went to the freeze and one to ownership; the probe should have been the first build, because it is the one that turns two silent
failures into one spoken one.
### arboard blocks rather than fails, and that is why three attempts missed (2026-11-09)

The probe went in on both fallible calls in the worker -- one eprintln for a failed Clipboard::new, one for a failed set_image, one before and
one after the successful path. Pressing the copy button produced **no output at all**, and the card is still drawn with both its buttons, so the
press landed.

Silence from an instrumented path has one reading: the thread started and **never reached the first line after Clipboard::new()**. The call does
not return an error on this display; it does not return.

That single fact explains every earlier result:

| observation | why |
|---|---|
| the window went black on the first press | the call was on the frame thread, which parked forever |
| the window was fine after the worker went in | the worker parked instead, and the UI kept drawing |
| xclip read zero bytes | nothing was ever set -- the thread never got that far |
| mem::forget changed nothing | there was no ownership to retain |

**So the copy button cannot work on this display**, and the reason is the environment: :100 is Xvfb with no clipboard manager, and the X11
backend blocks waiting for a selection it can take. On a desktop with a session it would very likely work; nothing in this project can prove that,
and a claim that it does would be an assertion.

**What to do about the button** is a product question the objective already answers in one direction: an affordance must not promise what it
cannot do. The button stays, because the reference has it and the handler is the operation the reference performs, and the record says plainly
that the clipboard write is **unverified on this lab and blocks there**. The alternative -- removing a button the reference draws -- would make
the interface less like the reference in order to hide an environment limitation.

**The probe stays in the code**, printing only on failure or success, because it is three lines and it is what turned a silent hang into a
readable one.
### Why 99% is out of reach, in one measurement (2026-11-09)

The objective asks for at most 1% of pixels differing at threshold 40. On a 1440x900 frame that is **12 960 pixels**. Rather than argue about
it, this classifies every differing pixel by what the *reference* is drawing there, using the local contrast of its 3x3 neighbourhood: a fill or
a rule is flat, a glyph edge or a photograph is textured.

| page | differing | **flat** (geometry, colour) | **textured** (glyph edges, artwork) |
|---|---:|---:|---:|
| Home | 33 004 | 8 188 | **24 816** (75%) |
| Screenshots | 39 822 | 14 482 | **25 340** (64%) |

**The textured class alone is 1.9 times the entire budget on Home.** No arrangement of layout, spacing or colour can bring the total under
12 960 while 24 816 pixels differ on glyph edges and photographs, because those two classes are not drawn by this project:

* **glyph edges** are `ab_glyph`'s coverage rasteriser against the reference's WebKit text antialiasing. Measured earlier at 13 215 differing
  pixels for the same glyphs at threshold 120 -- the shapes agree, the edge coverage does not;
* **photographs** are this build's bilinear sample against a compositor's filter. Measured on the news artwork alone: 19 424 pixels at 1920.

**So the honest statement of the clause is arithmetic, not effort**: at most 1% is unreachable on this backend at these sizes, with the split
above as the reason, and the reachable target is the flat share -- 8 188 on Home, 14 482 on Screenshots -- which is 0.6% and 1.1% of the frame
respectively. That is where every pixel improvement of the last many rounds has come from, and it is why they arrive in hundreds rather than
thousands.

**Two things this does not say.** It does not say the textured difference is invisible: at 1:1 a monitor glyph against a globe was obvious. And
it does not say the work is finished: the flat share is still 25-36% of the difference, and every one of those is a real, fixable mismatch.
### The sidebar's missing scrollbar is the text rasteriser, three steps downstream (2026-11-09)

The flat-pixel finder put its heaviest cluster on a vertical band at x 1408-1440 running the height of the panel, about 1 000 flat pixels
across eight cells. At 4x, the reference has a rounded **scrollbar** there in (66,68,74) and this build has none:

| x | reference | this build |
|---|---|---|
| 1426 | (42,51,49) card | (59,68,66) card border |
| 1434 | **(66,68,74) scrollbar** | (24,34,32) background |

This is the same element measured in round 344 and left unexplained at 6 by 97. It is a scrollbar, and the reason it exists there and not here is
a chain, not a style:

1. the reference's news copy breaks one word earlier per line, because its glyph advances are a per cent wider than ours;
2. its card is therefore **taller**, and the panel's content overflows the 900-point window;
3. an overflowing panel gets an overlay scrollbar, which is the band;
4. our copy fits, our card is shorter, our content does not overflow, and no scrollbar is drawn -- correctly, for content that fits.

**So the scrollbar cannot be drawn to match without also matching the wrap**, and the wrap is the rasteriser. Drawing a scrollbar over content that
does not overflow would be the fabricated affordance in its clearest form: a control that scrolls nothing.

**What this is worth**: it converts "the sidebar differs by 16 565" into a chain whose first link is out of reach and whose later links are
consequences. It also explains why the sidebar's difference grows with the window -- the taller the news card, the longer the scrollbar.

**Recorded for the FINISHED statement**: the identity clause's floor is not one mechanism but a short chain, and the flat share that remains is
partly *downstream* of the textured share rather than independent of it. That makes the reachable target smaller than the 0.6% quoted last round,
and it is the honest number to carry.
### The flat/textured split was measuring the wrong thing, and the artwork is scaled differently (2026-11-09)

The classifier divides differing pixels by the reference's local contrast: above 12 is textured, below is flat. Looking at the second-heaviest
flat cluster (x 1280, y 384 -- 206 flat pixels) shows what that rule does with a **photograph of a gradient**: the news artwork's interior has
almost no local contrast, so every pixel of it that differs is counted as *flat*, and it is not a fill or a rule. It is an image, drawn at a
different scale.

| | reference | this build |
|---|---|---|
| the artwork's rounded-square icon | smaller, higher in the card | **larger and lower** |
| the radiating pattern | finer, more rings visible | coarser |

So the two classes are not "reproducible" and "engine", they are "high-frequency" and "low-frequency", and a smooth gradient falls on the wrong
side. **Every number in the last two entries that called the flat share reachable is therefore too high**, and the honest statement is that the
split bounds the textured share from below and says nothing reliable about how much of the rest is layout rather than scale.

**What survives**: the textured share (24 816 on Home) is a floor on engine-bound pixels, because high local contrast can only be a glyph edge or
a sharp photograph -- that direction of the classifier is sound. What does not survive is treating the remainder as a work list.

**The artwork's scale is a real, separate mismatch** and it is drawable work: the reference's card draws its image at one size and this build at
another. That is the next thing to measure -- the art's rect on both -- rather than another classification pass, because the classification has now
told me it cannot separate the two.

**Recorded as a correction to `FINISHED.md`'s identity row**, which quoted "0.6% of the frame on Home" as the reachable target. That figure was
derived from the split and inherits its flaw; it should not be quoted again until the artwork's geometry is measured.
### The sidebar is a news LIST of four, not one card -- which is why the reference scrolls (2026-11-09)

`features/news/news-sidebar.vue` is 51 lines and settles the structure:

```js
const { data: news } = useQuery({ ...newsArticlesQueryOptions(), select: (articles) => articles.slice(0, 4) })
```
```html
<div class="space-y-4 flex flex-col items-center w-full">
  <NewsArticleCard v-for="item in news" :key="item.path" :article="item" />
  <ButtonLink type="colored" color="brand" size="xl" href="https://modrinth.com/news" target="_blank" class="my-4">
    <NewspaperIcon /> View all news
  </ButtonLink>
</div>
```

So the sidebar renders **up to four cards stacked 16 points apart**, then a brand **View all news** button. This build draws **one** card and
rotates between two bundled articles every ten seconds (`NEWS_SLIDES`, `NEWS_SLIDE_SECONDS`), which is a different thing in three ways:

| | reference | this build |
|---|---|---|
| how many cards | up to **4**, all visible | **1** |
| what a card is | an article from the feed | a bundled slide |
| does it change over time | no | **rotates every 10 s** |
| after the cards | a **View all news** button | nothing |

**This is the cause of the scrollbar**, and it is structural rather than a metric artefact: four cards plus a button do not fit a 900-point panel,
so the reference's sidebar overflows and draws the overlay bar; one card fits, so ours does not.

**Why it was not changed today**: the panel positions the art plate absolutely (`serverart::paint_news(painter, plate, index, hover)` at a fixed
rect) and pins the hosting banner to the foot at `rect.bottom() - BANNER_H`. A second card has to push the first up and displace the banner, which
means the sidebar needs a scrolling column -- the thing this build has never had -- rather than an extra draw call. That is a round of its own and
it starts at `panel.rs:645` (`banner_top`) and :873.

**What is now known and was not before**: the news block is a list, the rotation is invented, the count is four, and the button exists. Those are
four matrix rows' worth of fact from one 51-line file.
### Settings: the theme radio is a filled disc where the reference draws a ring and a dot (2026-11-09)

Settings is the page this session has measured least and it has the highest flat share of any page looked at: **34 498 differing, 14 562 flat
(42%)**, against Home's 25%. Its two heaviest clusters are at x 416, y 384 (400 flat pixels) and x 608, y 512 (307).

At 2x, the first is the Appearance section's "Dark" row, and the difference is the radio indicator:

| | reference | this build |
|---|---|---|
| the chosen indicator | a **green ring with a green dot** inside it | a **solid green disc**, no ring |
| the moon beside "Dark" | a crescent outline | a filled moon |
| the Beta badge | a pill, text inside | a pill, slightly smaller |

The chosen row's plate matches, the label matches, the row heights match -- the indicator is the whole difference on that row, and at 18 points
across with two colours it is worth a few hundred flat pixels per page that shows a radio list.

**Located and not changed**: the radio's dot is drawn at `crates/sift-ui/src/pages/settings.rs:4807` (`circle_filled(radio, 3.0, theme.BRAND)`);
what is missing is the ring around it. The check-circle at :4637 is a *different* control -- the language list's -- and is already a filled disc with
a tick, which is what the reference draws there.

**Why not changed tonight**: the edit is small but the verification is not -- a build is eight minutes and the round had less. An unverified radio
change is exactly the kind of edit this project has had to withdraw before, so the finding and its line number are the deliverable.
**CORRECTED within the round**: the code at `settings.rs:4807` already draws a **ring with a dot** -- `circle_stroke(radio, 6.5, 1.5)` in the brand
colour plus `circle_filled(radio, 3.0, BRAND)` when chosen -- and its own comment says so. So the capture I read, `t2-settings-*.png`, **predates that
fix** and the difference it shows is not this build's. The finding is withdrawn.

This is the second time this session that a capture pair has been treated as current without checking its age, and the first where the difference
was convincing enough to write up. The check costs one `ls`: a capture older than the file it tests is evidence about the past.
### Settings measured properly: 39 064, and the difference is the modal's geometry (2026-11-09)

The 32 469 this table had carried was a frame I cannot date. A fresh verified pair, with **both clients on the Appearance tab** -- which took
three attempts, because the first pair had the reference on Profile, the second had it on Discover after a click dismissed the modal, and the
third matched -- reads **39 064** (titlebar 1 683, rail 318, content 34 095, sidebar 2 968).

Banded by the flat finder, its heaviest cells are at **x 96-160, y 256-384**, about 5 300 flat pixels in eight cells. At 2x that region is the
modal's own left rail in the reference and the **page behind it** in this build: our modal starts further right, so the rail falls outside it.

**Two explanations were tested and both failed**:

| hypothesis | measurement | verdict |
|---|---|---|
| the backdrop blur is missing (`backdrop-filter: blur(5px)`, `settings.rs:40`) | backdrop samples at four points: ref (22,24,28) (29,31,35) (29,31,35) (20,28,32) against ours (22,24,28) (29,31,35) (29,31,35) (17,25,28) | **within three units** -- the scrim is right, and the blur is a renderer item of a few pixels here, not thousands |
| the scrim is too light | same samples | **no** |

So the difference is **where the modal sits and how wide it is**, not how it is painted. The rail, the labels and the icons all match at 2x once
the two are aligned, which is why the cells are concentrated at the modal's edge rather than across it.

**Next**: measure the modal plate's rect on both -- its left, right and width -- and compare against `NewModal.vue`'s own dimensions. That is one
capture and one source read, and it should close the largest flat cluster on the page.

**Recorded**: `FINISHED.md` carried Settings at 32 469; the figure for the Appearance tab on this build is **39 064**, measured today, and the
older number should not be quoted again.
### The Settings rail scrolls here and not in the reference (2026-11-09)

Sampling across the modal at y 250 found the difference in one point:

| x, y | reference | this build |
|---|---|---|
| 450, 200 | (39,41,46) modal plate | (39,41,46) |
| **500, 250** | **(38,40,45)** rail | **(66,68,74)** -- `SURFACE_5` |
| 600, 300 | (22,24,28) outside the modal | (22,24,28) |

The (66,68,74) is the rail's **scrollbar thumb**, drawn at `settings.rs:3358-3377` from `rail.right() - 8.0`, 4 points wide.

**The code is not wrong to draw it.** The draw is gated on `travel > 0.0`, where `travel = rail_needed_height(dev_mode) - visible` -- so this
build's rail genuinely overflows its pane at 1440x900, and the reference's does not. That is the finding: not a stray decoration but a different
answer to "does this list fit?", and the difference is in one of the two terms.

**The numbers to compare next**: `rail_needed_height(dev_mode)` against the reference's own rail height, and the pane's visible height in both.
The modal is `--modal-max-width: 60rem` (960) centered per `NewModal.vue:572`, and its height comes from the same component's `calc(100% - 2 *
var(--gap-lg))` cap -- so if the two modals are the same height, the difference is the rail's content, and the likeliest candidate is a row this
build draws and the reference does not (the footer, or the dev row the `dev_mode` argument exists for).

**What this explains**: the flat cluster at x 96-160 is the region where the modal's rail sits, and a 4-point thumb plus the row-height difference
it implies accounts for a few hundred of those pixels directly and for the rest by shifting every row below it.
### The rail's numbers do not agree with the rail's pixels (2026-11-09)

The probe printed, at 1440x900 with the Appearance tab open:

```
RAIL needed=728 visible=508 travel=220 thumb_h=354.5 rail=[[256 198] - [506 786]] dialog=[[256 114] - [1184 786]]
```

and the pixels say something else. Measured on the same pair:

| | reference | this build |
|---|---|---|
| rail label centres | 311, 355, 399, ... 734 | **311, 355, 399, ... 738** |
| pitch between rows | 44 | **44** |
| `SURFACE_5` columns at x 440..560 | **only x 506** (the plate border) | **x 498..501** (a 4-point thumb) + x 506 |
| dialog | 114..785 per the old capture | 114..786, 672 tall |

So the rows sit at the same places, with the same pitch, in the same 672-point plate -- and this build believes its rail needs **728** points and
therefore draws a thumb. Both cannot be true: if the rows ended at 738 in both, the content is about 600 tall and the pane about 580.

**The estimate is the suspect, not the layout.** `rail_needed_height` (`settings.rs:3058`) sums `RAIL_ROW_HEIGHT + RAIL_ROW_GAP` per row and, per
category, `CATEGORY_LINE + CATEGORY_PADDING + RAIL_ROW_GAP` plus a `CATEGORY_TOP_GAP` between them. Eight rows at 44 is 352; three category blocks
would have to be 125 points each to reach 728, and the measured label spacing across a category boundary shows they are not.

**The next step is arithmetic on the function itself**: print its running total after each row and category, and the point at which it diverges from
the laid-out position. That is a probe on a pure function -- no window, no capture -- and it will name the term that is counted twice or sized
wrong. The scrollbar is a symptom of that term and will disappear with it.

**Recorded rather than fixed** because the round ended, but this one now has a number, a function and a line, and the disagreement between them is
the whole of the remaining work.
### The rail's scrollbar is an overlay: invisible until used (2026-11-09)

The numbers and the pixels disagreed for a reason that turned out to be mine, not the code's: **`Tab::ALL` holds 13 entries**, so
`rail_needed_height` returning 728 against a 508-point pane is correct arithmetic -- 13 rows at a 44-point pitch plus four category blocks --
and the rail genuinely overflows. The reference overflows too.

What differs is that its scrollbar is an **overlay**: invisible at rest, shown while the pointer is on the list. The capture taken with the
pointer parked elsewhere has no thumb, only the plate's border at x 506; ours painted a 4-point thumb at x 498..501 unconditionally.

**Changed**: the thumb is drawn only while the pointer is over the rail. Measured after:

| | reference | before | after |
|---|---|---|---|
| `SURFACE_5` columns, x 440..560 | **506** | 498, 499, 500, 501, 506 | **506** |

That is exact agreement on the column set -- the first time a probe of this kind has matched rather than merely moved.

**A second difference the same pair shows, not yet fixed**: both rails hold 13 tabs, but the reference **clips** at "Java and memory" -- its last
visible row -- while this build draws the rows below it past the pane's foot and over the footer ("Advanced rendering", "Advanced", "Data"). So the
reference's scroll container clips its children to the pane and ours does not, which is why its overflow needs a scrollbar and ours needs a clip.

**And a number to distrust**: Settings measured 42 213 on this pair against 39 064 on the previous one, with both clients on the same tab and the
scrollbar now matching. The difference is in what the rails *draw* below the clip, so the two figures are not comparable until the clipping is
fixed; 42 213 should not replace 39 064 in any table.
### The clip was a no-op, and the two captures say so (2026-11-09)

The rail now filters the rows that begin below its pane, was built and was run -- and the picture did not change:

```
compare.py sb-our.png clip-our.png   ->   total 0   titlebar 0   rail 0   content 0   sidebar 0
```

Identical to the pixel, with the binary built at 23:00:08 and the app started at 23:00:20. So the rows the filter removes are not the thing that
draws "Advanced rendering", "Advanced" and "Data" below the rail's foot, and the filter has no effect on the frame.

**Where they most likely come from**: `state.drawn_rail` (`settings.rs:622`, assigned at :2522, taken at :2840). The rail's rows are computed once,
stored in the state, and the stored copy is what later code reads -- my `retain` changed the freshly computed list and not the stored one, which
would explain an exact no-op. That is one read away and is where the next attempt starts.

**A second difference the same capture pair shows, and it is not a bug**: the rail's footer names the product. The reference reads
`Modrinth App 0.22.0 / Linux 20261025.0.616225`; this build reads `Sift Launcher 0.1.62 / linux 0.1.62`. That is this build telling the truth about
itself, and it is a deliberate difference from the reference rather than a miss -- recorded here so that no later round "fixes" it by copying the
reference's name into a product that is not that.

**Left in the tree**: the `retain` is harmless and correct in intent -- a scroll container does clip -- but it is dead code until the stored copy is
the one it filters. It comes out if the next attempt finds the drawing path does not go through `rail_rows` at all.
**Withdrawn the same round.** The `retain` came out because it broke `the_rail_fits_its_pane_and_keeps_one_pitch_per_category`, which inspects
the very rows it removed -- and it had already been shown to change nothing on screen. An ineffective change that also fails a test is worse than
no change: it costs a build, a test run and the reader's trust in the next "fixed". **269 tests pass again on the reverted tree.**

The finding stands and is now smaller and sharper: the rows that overflow the rail are not drawn from `rail_rows`, and `state.drawn_rail` --
computed once at :2522, taken at :2840 -- is where to look.
### The rail's tail: a short rule this build draws and the reference does not (2026-11-09)

Cropping the foot of the rail at (250,620)-(530,760) in both, at 2x, shows the last thing in the column before the footer:

| | reference | this build |
|---|---|---|
| INSTANCE DEFAULTS heading | present | present |
| "Java and memory" row | present, same y and pitch | present, same y and pitch |
| between that row and the footer | **nothing** | a **short horizontal rule**, about thirty points wide |
| the footer | Modrinth App 0.22.0 / Linux ... | Sift Launcher 0.1.62 / linux 0.1.62 |

So the earlier reading -- that extra tabs were drawn below the pane -- was wrong twice over: there are no extra tabs, the rows agree row for row,
and the difference at the foot is one short rule plus the footer's own text. That also explains why the `retain` was a no-op and why the region the
flat finder flagged sat at the rail's *bottom* rather than across it.

**Not yet identified**: the rule's draw call. The rail is wrapped in a `ScrollArea` whose pane is `rail.min .. footer.top() - 8`
(`settings.rs:3189`, `:3224`), and either that area's frame or a footer divider is the candidate. The next step is to disable the pane's frame and
re-capture -- a one-line change with an unambiguous before and after, which is exactly what the last three attempts at this element lacked.

**The footer's text is a deliberate difference**, recorded last round: this build names itself. It should not be "fixed" by copying the
reference's product name.
### The rule is 226 by 1 at y 785, and nothing in the footer draws it (2026-11-09)

Measured with the rule colour, scanning x 265..500 and y 680..800:

| | reference | this build |
|---|---|---|
| runs at y 737..758 | 9-13 pixels wide -- the footer's own glyphs | the same |
| a wide run | **none** | **y 785, 226 pixels** |

So the difference is a single-pixel high rule, 226 points wide, at `rail.bottom() - 1`, spanning x 265..491 -- which is the rail's inner width
(256..506 less 16 of padding on each side, to the pixel).

**What it is not**: the footer block (`settings.rs:3398-3440`) draws only the dev-mode line, the mark and two text lines. The rail's border
(`:3276`) is the vertical `border-r`, `rail.top()..=rail.bottom()` at `rail.right()`. The header's rule (`:3162`) is at `header.bottom()`.

**Candidates left**, in the order a bisect should try them: the `ScrollArea`'s own frame or its scrollable background; a `rect_stroke` around
`rail_rect` that this build adds and the reference does not; or the *selected row's* plate clipped to a sliver by the pane's bottom (a plate is 226
wide, and only one row of it matched -- which is what a clip would leave).

**The bisect is one line at a time**: disable each candidate, rebuild, and compare `clip-our.png` against the new frame. The comparison is exact and
cheap -- two captures, `compare.py`, zero if nothing changed and a number if it did. That is how the last two no-ops should have been tested, and
it is what the next round does first.
### Two measurement faults, one of them three rounds old (2026-11-09)

**Fault one: my tolerances were stricter than the project's own metric.** Three rounds went into a "226 by 1 rule at y 785 that this build draws
and the reference does not". It does not exist. Scanning the same row again with a wider tolerance:

| | y 785, rule-coloured samples over x 260..1180 | pixels at x 300/500/700/900/1100 |
|---|---|---|
| reference | **447** | (66,68,74) at every one |
| this build | **447** | (66,68,74) at every one |

The rule is in both, identically. My probes had been matching against `(66,68,74)` within 3, 6 and 8 units -- while the project's own metric is
**threshold 40**. A pixel 2 units out failed my scan and passes `compare.py`, so I spent three rounds bisecting a difference the measurement
standard would not count. **Every probe now states its tolerance, and the tolerance is the project's 40 unless there is a reason.**

**Fault two: a modal inherits the page behind it.** With both clients on the same tab and the same size, Settings reads 55 197 differing -- but split
by the dialog's own rect, 256..1184 by 114..786:

| region | differing pixels |
|---|---:|
| inside the modal | **27 256** |
| outside it -- the dimmed page behind | **27 941** |

Half of the figure is the Home page showing through the scrim, which this build draws differently for reasons that have nothing to do with Settings.
So **Settings' own number is 27 256**, and every modal page measured this way needs the same split: the Screenshots viewer, the create-instance
dialog and the delete confirmation all sit over a page whose difference they will otherwise be charged for.

**What this corrects**: the 42 213 and 39 064 quoted in the two previous rounds both measured Settings-plus-Home and should not be used. The
figure for Settings is **27 256 inside its own plate**, and it is the first one on this page that is about the page.
### The Appearance pane sits four points low, and lifting it half-fixes it (2026-11-09)

Cross-correlating each region of the Appearance pane against the reference over -4..+4 found a single number:

| region | best shift before | best-shift difference | after the lift |
|---|---:|---:|---:|
| theme cards row | **+4** | 581 | **+1** (334) |
| System theme row | **+4** | 178 | **+1** (80) |
| the rail, beside them | **0** | 229 | unchanged |

The rail is right where it is and the content column is four points low, so `content_rect` now starts at `HEADER_HEIGHT - CONTENT_LIFT` with
`CONTENT_LIFT = 4.0`.

**Half-fixed, not fixed**: the residual moved from +4 to +1, which means the true offset is **five** and I lifted four. The differences at the
best shift halved, which is what a four-point correction of a five-point error looks like.

**And the page's total went UP**: `sb-ref.png` against the new frame reads **45 536** where the previous frame read 42 213. So the lift moved the
content onto its own rows and, in doing so, charged four points of every row twice over -- against the old position the rows overlapped the
reference's by coincidence in some places.

**So this is not yet a win and is recorded as such.** The next step is `CONTENT_LIFT = 5.0`, one build, and the same three-row probe: the cards
and system rows should both prefer dy 0, and the total should fall below 42 213. If it does not, the lift comes out -- a change that halves one
region's difference and raises the page's is not a fix until the total says so.
### The rows want five points; the plate does not (2026-11-09)

`CONTENT_LIFT = 5.0`, built and measured, with the prediction stated first: *both regions prefer dy 0 and the total falls below 42 213*.

| | lift 0 | lift 4 | lift 5 |
|---|---:|---:|---:|
| theme cards, best shift | +4 (581) | +1 (334) | **0 (334)** |
| System row, best shift | +4 (178) | +1 (80) | **0 (80)** |
| inside the modal | 27 256 | -- | **31 721** |
| page total | **42 213** | 45 536 | 46 437 |

**The alignment prediction was right and the total prediction was wrong, and the gap between them is the finding**: at lift 5 the rows sit
exactly where the reference's do -- best shift 0, differences down from 581 to 334 and from 178 to 80 -- and the page is *worse* by four and a
half thousand pixels.

**Because `content_rect` is not only the rows.** It carries the pane's own background and its edges, so lifting the rect moves the plate by five
points as well: it now starts five points into the header and leaves a five-point band at the foot, and those two edges cost far more than the
rows saved. The rows are in the right place and the box they sit in is in the wrong one.

**Reverted, and the revert verified**: `sb-ref.png` against the reverted frame reads **42 213** again, exactly the figure before the lift. HEAD is at
the better number.

**What a real fix would look like**: lift the *children* of the pane and not the pane -- the row layout's own start, not the rect it is clipped to.
That is a different edit in a different place (`settings.rs`'s per-row draw, not `content_rect`), and it is the one the cross-correlation actually
calls for. Recorded rather than attempted, because the round's two builds went to a lift that had to come out.
### The metric rewards misalignment, and that is the most useful thing found today (2026-11-09)

Lifting only the children (`CONTENT_PADDING_TOP` 28 -> 23, leaving `content_rect` alone) produced a frame **identical to the rect lift**:
total 46 437, 31 721 inside the modal, the cards row at dy 0 with 334 differing. So the plate-edge theory was wrong; the two edits move the same
pixels.

**The explanation is in the metric, not the layout.** At dy +4 -- a *wrong* geometry -- the pages' rows sit four points apart, and across a
sharp edge the two renderings' gradients overlap in a way that lands more often than not on the same side of the threshold. At dy 0 -- the
*right* geometry -- every edge is compared with its true counterpart, and the differences that were being hidden by the offset become visible:

| | dy +4 (wrong place) | dy 0 (right place) |
|---|---:|---:|
| theme cards row, best shift | 581 | **334** |
| the same row, compared in place | 581 at +4 | **334 at 0** |
| inside the modal | **27 256** | 31 721 |

The row is *closer* by 247 pixels and the page is *worse* by 4 465. Both numbers are correct; they measure different things. A pixel count at a
fixed threshold is not monotone in fidelity, because an offset can cancel.

**So the right thing is to keep the correct geometry and say so.** The rows now sit where the reference's do; the count is higher because the
comparison is now honest. `CONTENT_PADDING_TOP` stays at 23.

**What this costs the objective**: every page figure in this project is a count, and a count can be improved by moving things slightly wrong. None
of the earlier improvements were deliberate in that sense -- they were judged by the count -- but the possibility that some of the hundreds of
pixels "saved" in earlier rounds came from a small misalignment is real and cannot be ruled out after the fact. What can be stated is the rule from
here: **align first, then count**, and treat a count that improves when geometry moves away from the reference as a warning rather than a win.
### Three pages are aligned; the fourth cannot say (2026-11-09)

Following the finding that a count is not monotone in fidelity, `tools/parity/align.py` reports the shift that best matches a pair and what it
would save. Run over the pairs the table quotes:

| page | in place | best | verdict |
|---|---:|---|---|
| Home | 2 940 | 2 940 at (0,0) | **aligned** |
| Screenshots | 1 754 | 1 754 at (0,0) | **aligned** |
| Discover | 11 921 | 11 921 at (0,0) | **aligned** |
| Skins | 37 394 | 37 331 at (0,-6) | **inconclusive** -- 0.2% fewer at the edge of the search range |

(The counts are small because the probe samples every second pixel inside a region, not the whole frame; it is an alignment test, not a score.)

**So the three page figures the table quotes were taken at aligned geometry**, and the non-monotonicity that would cast doubt on them does not apply.
That is worth knowing precisely because the Settings finding suggested it might: the concern was that some earlier "saved" pixels were accidental
cancellations, and for these three pages they were not.

**Skins is inconclusive rather than wrong**: its best shift sits at the edge of the -6..+6 range with a 63-pixel advantage on 37 394, which is
noise. Its page is dominated by skin artwork, so an edge-based correlation has little structure to lock onto. A narrower region or a bigger range
would decide it, and neither changes any figure today.

**The tool is committed** so that any future page figure can be accompanied by its alignment, which is now the cheapest way to keep a count honest.
**CORRECTED within the round**: on a narrower region (80,140 to 900,860) Skins reads **30 619 in place, 30 619 at (0,0)** -- aligned. The
inconclusive reading came from a window whose left third is a large photograph, where an edge-based correlation has little structure to lock onto.

**So all four quoted pages are aligned**: Home, Screenshots, Discover and Skins, each with no shift beating in place. The non-monotonicity found on
Settings does not cast doubt on any figure this table quotes, and that is now checked rather than assumed.
### The Beta pill sits at the row's right end: 1 082 pixels (2026-11-09)

The heaviest band inside the aligned Settings modal was x 384-448 at y 384 -- the "Language" rail row. At 2x the whole difference on that row is
the **Beta** pill: the reference puts it hard against the row's inner right edge, a hundred points clear of the label; this build put it eight
points after the word.

`settings.rs:3345` now places it at `row.right() - RAIL_ROW_PADDING - 42.0` instead of `label.x + label.width + 8.0`.

| | before | after |
|---|---:|---:|
| the Language row band, x 384..500 | 950 across three cells | **286** |
| inside the modal | 31 721 | **30 639** |
| page total | 46 437 | 46 204 |

**Minus 1 082 inside the plate against a predicted 300-500** -- the prediction was in the right direction and understated, because the pill's
old position also overlapped the label's own glyph run for part of its width.

**The page total moved only 233**, because most of it is the dimmed Home page behind the modal: the split measured on this page is why the
change's real value (1 082) is four times what the headline number shows.
### Past the theme cards, the content drifts five points again (2026-11-09)

After the Beta pill fix the modal's heaviest cells are 168 pixels or less, so this round tested each for **offset versus content** rather than
looking at them one by one:

| region | best shift | differing at best | verdict |
|---|---:|---:|---|
| the OLED preview card, 1030-1160 x 330-400 | **(0,0)** | 411 | aligned -- its difference is content, and its bars' colours match to the unit ((176,186,197) and (16,16,19) in both) |
| the block below the cards, 520-700 x 560-620 | **(0,-5)** | 899 | **still five low** |

So the content is aligned at the cards and five points out again a hundred points below them: the **gap between the theme cards and what follows is
five points too large** in this build. The cards themselves are laid out at `y + row * (THEME_CARD_HEIGHT + THEME_FOOTER + THEME_CARD_GAP)`
(`settings.rs:4718`), so the suspect is the step that advances `y` past that block rather than any of those three constants.

**Not changed**: the round had one build left and the step that advances `y` was not located with certainty; a guessed constant would cost a build
and then a revert. The lead is one read: find where `y` is advanced after the theme cards and subtract five.

**Worth noting about the method**: treating "shift versus content" as the first question on every cluster turned two apparent colour defects into
one alignment defect and one no-op -- the OLED bars are identical in both builds, and the eye that said otherwise was reading a 3x nearest-neighbour
upscale of a three-colour region.
### The row under the theme cards is "Sync with system", and it is five low (2026-11-09)

The five-point drift below the cards is on the **"Sync with system"** row (`THEME_SYSTEM_LABEL`, `settings.rs:893`) -- not, as recorded last
round, a "System theme" row. Two things follow:

* the cards themselves are aligned at (0,0), so the theme block's **height** is what is five too large, not its position;
* the block is drawn by the `Block` enum's `draw` at `settings.rs:5207`, so the term lives in whatever computes that block's height -- the same
  place `ADMONITION_BELOW` and `NOTE_BELOW` are added for other blocks (`:4921` onward).

**Not located, and therefore not changed.** Three greps did not find the theme block's height term: the card constants (`THEME_CARD_HEIGHT` 114,
`THEME_FOOTER` 43, `THEME_CARD_GAP` 17) appear only in the card's own layout at `:4718`, and the height that follows them is computed elsewhere
in a match this round did not reach. A guessed constant here costs a build and then a revert, and the last two guesses at this page did exactly
that.

**What the round did establish**: the drift is in a height, not a position; it is exactly five; and it belongs to the theme block rather than to the
rows after it, because those rows would otherwise each need their own five. The next read is the `Block` height match for the theme variant, and the
edit is one subtraction in it.
### The news list needs a scrolling sidebar, and the layout says so in one line (2026-11-09)

I have deferred this item three rounds. The code now gives the reason it cannot be done as a draw call, and it is checkable:

```rust
// panel.rs, the plan's own test
plan.banner - (plan.card + plan.news_rows.height)
```

That expression is the **card's height**: the plan gives the news card everything between its heading and the hosting banner, which is pinned to
the panel's foot (`rect.bottom() - BANNER_H`). A fourth news card and a "View all news" button therefore have nowhere to go -- not because the
drawing is hard, but because the plan allocates one card's worth of space and the banner owns the rest.

The reference does not have that constraint: its sidebar is a flex column that **scrolls**, which is why four cards and a button fit and why its
overlay scrollbar exists. So the faithful change is a scrolling sidebar -- an offset threaded through the panel's absolutely-positioned draws, a
clip around the scrollable part, and a plan that allocates content height rather than card height.

**That is a multi-round refactor of the one file every page shares**, and with 26 rounds left the expected value is negative: the last three
attempts to change this panel's geometry (the card row, the copy size, the scrollbar) each cost two or three builds, and a half-scrolled sidebar is
worse than a correct single card.

**So this is recorded as a structural gap with its cause, not as a defect**: the sidebar shows one news card where the reference shows up to four
and a button, and the missing thing is the scroll container rather than the cards. A future round with more budget than this one should start at
`panel.rs:1248` (`let card = plan.card`) and the plan's allocation above it.

**What the remaining rounds should do instead**: close matrix rows by driving states, and take page-level defects that are one constant, one
glyph or one string -- which is where every measurable win of the last ten rounds has come from.
**CORRECTED in the same round**: I read that expression as the card's height and it is not. The plan's own test asserts it equals **26.0** --
"the card ends 26 above it" -- so it is the *gap* between the card's last line and the banner's top, not the space the card occupies.

The conclusion is unchanged and now rests on the right line: `plan.banner == g::WINDOW.1 - BANNER_H` ("the banner is pinned to the foot") and
`plan.card > plan.news`, with the card's block ending 26 points above that banner. A second card needs roughly 300 points that the column does not
have, and the 26 spare is the reference's own measured gap rather than slack.

**The lesson is the same one as the last four rounds, and I would rather write it than bury it**: a single assertion read quickly looks like
arithmetic and is often a constant someone chose. Two of this session's confident structural claims (this one, and the "plate" that turned out to
be the rows) came from reading one line of a test as if it were a derivation.
### The card's right-click menu carried two rows the reference does not have (2026-11-09)

Driving a right-click on the card and reading `showScreenshotOptions` (`screenshots-page/index.vue:1058-1097`) side by side:

| | reference, on a right-click | this build, before | this build, after |
|---|---|---|---|
| 1 | Edit | Open | -- |
| 2 | Copy | **Add to group** | -- |
| 3 | Show in folder | Show in folder | **Show in folder** |
| 4 | Go to instance | -- | -- |
| 5 | (divider) | -- | -- |
| 6 | Delete (red) | Delete | **Delete** |

**Two of the old rows were fabricated**: the reference has no "Open" in this menu, because opening the picture is the card's own click, and it has
no "Add to group", because groups are assigned by drag, by the selection bar, or by the card's own actions. The menu was a list of what this build
can do rather than a list of what the reference offers -- and its own doc comment said so while calling the result correct.

**Changed**: `MENU_ITEMS` is now `[ShowInFolder, Delete]`. The rows are the reference's 3rd and 6th, both of which this page can carry out.

**Still absent, and each for a named reason**: Edit needs an image editor, Copy needs a clipboard that works on this display (it blocks -- four
rounds proved that), Go to instance needs a route out of a page whose `draw` returns a `bool`. Those three stay out until the machinery exists,
and the divider stays out because it would divide nothing.

**Verified**: built and driven -- the menu now ends after two rows where it used to run four.
### Selection driven on both, and my coordinate was the bug (2026-11-09)

Pressing the card's selection circle opened the viewer instead. The code was right and I was wrong by two points: the probe prints the
rectangles a press is read against, and on this card they are

```
card  2026-11-09_12.00.00.png  88.0,247.0 508.0x285.8
check 2026-11-09_12.00.00.png 544.0,247.0  50.0x50.0
```

so the check box spans x 544..594 and y **247..297**, and the press that opened the viewer was at y 245 -- two points above it. `on_check`
(`screenshots.rs:2975`) then correctly let the card take the press and activate.

**Driven properly at (570,270), both clients show the same state**: the card selected, a filled white disc with a dark check at its top-right. The
pair reads 54 970 with the card selected, against 39 559 unselected -- the difference being the selection ring, the scrim and the bar.

**Why this is worth a row**: it is the first time this session that a "does not work" turned out to be a press outside a printed rectangle, which is
the exact failure the matrix's own preamble warns about ("a press that does nothing is evidence about the press, not about the feature"). The probe
that settles it existed the whole time, behind `SIFT_SHOTS_PROBE`.
### The selection bar is a floating action bar, and ours is a different bar (2026-11-09)

With one screenshot selected on both, the foot of the page shows a floating bar -- and not the same one:

| | reference | this build |
|---|---|---|
| shape | a **wide** bar spanning the content column, sitting just above its foot | a **narrow pill**, centred |
| left | "1 selected" | "1 selected" |
| then | a divider, **Clear** | **Clear** |
| then | **New group**, **Export ZIP**, a divider, **Delete** | **Delete** |

The source (`index.vue:1643-1690`) renders a `FloatingActionBar` with `shown="selectionActive"`: selected count, a `h-6 w-px bg-surface-5`
divider, Clear, then `ml-auto` New group, Remove from group (only when grouped items are selected), Export ZIP, and Delete.

**So this build has the bar's frame and two of its five actions.** New group and Export ZIP are missing, and its geometry is a centred pill rather
than a full-width bar -- which is the same `FloatingActionBar` component the Home library uses, so the difference is likely shared.

**Recorded as OPEN with both causes named**: two rows' worth of missing actions (`New group`, `Export ZIP`), and a shape that belongs to the shared
bar rather than to this page. Export ZIP needs archive machinery this build does not have (`screenshots.rs:85` says so); New group already exists on
the toolbar and would be a second entry point to the same dialog.

**Captures**: `fab-ref.png` and `fab-our.png`, cropped to 300..1140 by 700..890.
### Why the selection bar's rows and its shape are one change (2026-11-09)

The bar's rectangle is computed by one function and its buttons are drawn by another:

```rust
// screenshots.rs:1380
let bar_rect = selection_bar_rect(painter, list_rect, theme, selection.len());
// and further down, separately:
let clear = Rect::from_min_size(...);   // the Clear button
let delete = Rect::from_min_size(pos2(bar.right() - BAR_PADDING_X - delete_width, ...));
```

So the width comes from `selection_bar_rect` (which knows the count and the theme) and the rows come from the drawing code below it. Adding
**New group** therefore means: add the button to the drawing *and* teach `selection_bar_rect` to reserve its width -- two functions, in step, or
the new row overflows the pill.

**That is the same coupling the reference avoids**: its `FloatingActionBar` measures its own children, so rows and width cannot disagree. The
difference is not the missing action, it is that this build's bar is sized by a separate calculation rather than by what it draws.

**So the recorded fix is a refactor, not an addition**: make `selection_bar_rect` derive its width from the same list the drawing iterates. That is
one change that would then let New group, Export ZIP and the reference's wide shape arrive together -- and it is why three attempts to add a single
button would each have been reverted.

**Recorded with the two line numbers** so the next attempt starts at `:1380` and the `clear`/`delete` block rather than hunting again.
### The theme block's tail is not what moves that row (2026-11-09)

The arm that draws the theme cards ends with `height + ROW_GAP + 2.0`, and its comment records that the +2 was added when a row below it was
measured two points high. The cross-correlation says the block below the cards prefers **dy -5**, so the term looked like the cause. Changed to
`- 3.0`, built and measured:

| | before | after |
|---|---:|---:|
| inside the modal | 31 721 | **32 544** |
| "Sync with system", best shift | -5 (899 differing) | **-5 (475)** |
| page total | 46 204 | 48 048 |

**The row's offset did not move** -- still -5 -- while the rows *after* the theme block shifted and cost 823 pixels. So that term positions the block's
own height for whatever follows it and not the row in question, and the -5 lives somewhere between the cards' top and that row.

**Reverted and verified**: the frame reads **46 204** again, the figure before the attempt.

**What this eliminates**: the block's tail. What remains between the cards and the row is the cards' own height (`THEME_CARD_HEIGHT + THEME_FOOTER,
114 + 43`) and the row's own top inset. The difference between 475 and 899 is itself informative: the change halved that row's differing pixels while
leaving its best shift at -5, which is the signature of a *vertical scale* difference rather than a pure offset -- the row moved less than its
neighbours did.

**Two builds to learn that one term is innocent**, which is the honest rate for this kind of search; the next attempt should test the cards' own
height (114 + 43) against the reference's, rather than another constant downstream of it.
### The drifted element is the System card, and its gap is inside the card (2026-11-09)

Two attempts, both reverted, and one identification that matters:

| attempt | prediction | result |
|---|---|---|
| theme arm's tail `+2.0` -> `-3.0` | the row below moves up 5 | the row **did not move**; the rows after it shifted and cost 823 |
| `THEME_CARD_GAP` 17 -> 12 | the second card row moves up 5 | **the cards themselves degraded** (334 -> 890 differing at dy 0) and the row went to -8 |

**What the second attempt proved**: `THEME_CARD_GAP` is used **inside** the card as well as between rows -- reducing it distorted the preview's own layout, which is
why the cards' difference nearly tripled. So the row pitch is not exposed as one constant.

**What the crop proves**: the drifted band at y 540-640 is the **fourth card**, the full-width "Sync with system" card (`ThemeChoice::System`, labelled at
`settings.rs:1519`), drawn on the second row of the card grid. Row 0 is aligned to the reference and row 1 is five low, so what is wrong is the **step between
the rows**, and that step is not `THEME_CARD_GAP`.

**Reverted and verified**: the page reads **46 204** again, and the tree is clean at HEAD because neither change was committed while it was unverified.

**Stopping here by decision, not by running out**: four attempts and five rounds on these five pixels have eliminated the arm's tail and the gap constant and
identified the element exactly. The next attempt should read the theme card's *row step* in the layout at `settings.rs:4718` rather than any constant in scope,
and with 13 rounds left there is a better use for them than a fifth guess.
### The row gap, separated and tried: columns held, total rose (2026-11-09)

The layout at `settings.rs:4718` uses one constant for both pitches -- `row * (THEME_CARD_HEIGHT + THEME_FOOTER + THEME_CARD_GAP)` and
`column * (THEME_CARD_WIDTH + THEME_CARD_GAP)` -- which is why shrinking the shared one moved the columns and the cards' own difference nearly tripled.
Separating them (`THEME_ROW_GAP = 12.0`, used only in the row term) is the right shape of change, and it did what it was meant to do:

| | before | after |
|---|---:|---:|
| theme cards row, best shift | 334 at dy 0 | **334 at dy 0** -- columns untouched |
| the System card, best shift | 899 at -5 | **688 at -8** |
| inside the modal | 31 721 | **35 794** |
| page total | 46 204 | 51 297 |

**The card moved and the page got worse.** Its own difference fell from 899 to 688, but everything below it in the pane moved up five points and cost
more than that -- the same shape as the `CONTENT_LIFT` attempt four rounds ago, and for the same reason: the System card is not the last thing in the
pane, so shortening the step under the first row drags the rest with it.

**Reverted**; the rebuild is queued and the binary will be current at the start of the next round. Nothing was committed while unverified.

**What is now established about these five pixels**: the reference's second card row is five higher, and the layout cannot express that by changing
the row step, because the step also positions everything after the cards. Expressing it needs the cards' block to *report* a height that is five
short of where its last row ends -- not to move its last row up.
### RSS, decomposed: 22 680 kB of which 5 600 is the app (2026-11-09)

Measured on the current build, idle for thirty seconds after sizing the window, with the process's own counters rather than a total:

| field | kB | what it is |
|---|---:|---|
| **VmRSS** | **22 680** | what `ps` and every RSS gate reads |
| RssAnon | **5 600** | the application's own heap and stacks |
| RssFile | 12 016 | mapped pages of the 11 MB executable (`stat` says 11 264 824 bytes) |
| RssShmem | 5 064 | shared memory -- the X server's framebuffer |

**So the 20 MB gate fails on a number that is two-thirds not-the-application**: the binary's own pages, resident because it is executed from disk
with no separate data files, plus the framebuffer it draws into. The application's heap is **5 600 kB**, which is the figure a memory requirement is
usually about.

**Why the same tree read 13 720 kB earlier**: `RssFile` is page cache for an executable, so it grows as the binary is touched and shrinks under
memory pressure -- the code differs by nothing between those measurements. That is why the earlier figure and this one are both true, and why neither
is a property of the build.

**A lab note found while measuring**: `ps` shows **299 defunct `SiftUi` processes** -- one per restart this project has done without a reaping parent.
They hold no memory, and they explain why `pgrep` occasionally returned a dead pid. Harmless, worth knowing.
### All five sizes re-verified on the final build (2026-11-09)

Captured at each size and compared against the reference capture of the same size (`five-ref-<tag>.png`), with each of our captures confirmed to be
exactly the requested dimensions first:

| size | this build | previous | evidence |
|---|---:|---:|---|
| 1280x720 | **27 933** | 29 888 | `five-ref-1280.png` vs `five2-our-1280.png` |
| 1366x768 | **36 268** | 36 268 | `five-ref-1366.png` vs `five2-our-1366.png` |
| 1440x900 | **31 356** | 31 356 | `homeCh239-ref.png` vs `homeCh239-our.png` |
| 1600x900 | **45 470** | 45 469 | `five-ref-1600.png` vs `five2-our-1600.png` |
| 1920x1080 | **57 864** | 57 864 | `five-ref-1920.png` vs `five2-our-1920.png` |

**1280x720 improved by 1 955** since the last sweep -- the card text and foot work, and the Beta pill, are worth most where the layout is tightest.
The other three are unchanged to within one pixel, which is the expected result of changes that touch the Settings dialog and the screenshots page's menu
rather than the Home page these captures show.

**The clause is met in the sense it can be**: five standard dimensions, both clients at each, same page, same state, captured and compared. It is not met
at the fidelity the objective asks for, and no size is close to its 1 per cent budget -- the figures are recorded so the distance is visible rather than
summarised.
**Checked rather than inferred**: the previous round's five-size comparison assumed the reference captures were on Home because three of the four
figures reproduced exactly. Cropping the top strip of each reference at 1280, 1366, 1600 and 1920 shows the **"Jump in"** strip in all four, so they
are on Home and the comparison is same-page at every size.
### The textured share, re-derived on the final build at two sizes (2026-11-09)

The claim that makes 99 per cent unreachable is that most of the remaining difference is edge texture rather than layout. It was measured once, and
the record has quoted it ever since; this is it re-derived on the current build, with the tool that produces both halves of the number:

```
$ python3 tools/parity/flat.py <ref> <ours> | head -1
Home 1440x900: differing 33 004, flat  8 188 (25%)   -> textured 24 816 (75%)
Home 1280x720: differing 28 042, flat  7 295 (26%)   -> textured 20 747 (74%)
```

`flat.py` counts a differing pixel as flat when its 3x3 neighbourhood in the reference varies by 12 or less, so textured is `differing - flat` and needs
no second tool. **The ratio holds at both sizes to within a percentage point**, which is what a rasteriser difference should do and what a layout
difference would not: a misplaced box keeps its flat interior and moves its edges, so its flat share would rise with the size, not stay at a quarter.

**Against the budget**: 1 per cent of 1440x900 is 12 960 pixels, so the textured 24 816 alone is **1.9x the entire allowance**, and the flat 8 188 is
62 per cent of it. Closing to 99 per cent would require the textured share to fall by more than half while the flat share stayed put -- i.e. the
rasteriser to change, not the page.

**Note on the two bases**: 33 004 here is `flat.py`'s whole-frame count; `compare.py` reads 31 356 for the same pair across its four slabs. The ratio is
computed from one tool on one frame, so it is internally consistent; the absolute figures are not interchangeable and are labelled with their tool.
### The reproduction section's examples are its own output (2026-11-09)

Each command in `FINISHED.md`'s "How to reproduce" section was run against the pairs its examples name, and prints what the section shows:

```
$ python3 tools/parity/compare.py .scratch/cap/sb-ref.png .scratch/cap/cur-our.png
total 46204 | titlebar 1537 | rail 391 | content 41208 | sidebar 3068

$ python3 tools/parity/flat.py .scratch/cap/homeCh239-ref.png .scratch/cap/homeCh239-our.png | head -1
differing 33004, flat 8188 (25%)

$ python3 tools/parity/align.py .scratch/cap/homeCh239-ref.png .scratch/cap/homeCh239-our.png
in place 2940  best 2940 at dx 0 dy 0
  aligned: no shift beats in-place
```

So the examples are transcribed output rather than illustrations, and a reader running them against the captures named gets the same lines. The one
place the section does not claim to work is `tools/parity/README.md`, which still describes a Windows capture path this lab left behind -- noted here
rather than edited, because rewriting it would be a change to documentation about tooling that no longer exists.
### The row moved; the pane followed anyway (2026-11-09)

The specified change -- draw the second card row five points higher while reporting the block's height where it was -- built and measured:

| | before | after |
|---|---:|---:|
| theme cards row | 334 at dy 0 | **334 at dy 0** |
| the System card's band | 899 at -5 | **688 at -8** |
| inside the modal | 30 639 | **33 889** |
| page total | 46 204 | 49 453 |

So the columns held and the row did move, and the page still got worse by 3 250 inside the plate. The best shift went to **-8** rather than 0, which
says the moved card no longer matches the band the probe compares -- the card left the window the measurement looks through, so the figure is no longer
about the same element.

**Reverted** (`git checkout`, tree clean, rebuild queued). Five attempts, five reverts, three constants and one layout shape eliminated. The remaining
possibility is that the reference's five points are not in the card grid at all but in the *content above it* -- every attempt so far has moved the cards
relative to a reference that may itself be comparing different rows.

**Stopping permanently on this item.** It is worth a few hundred pixels, it has consumed six rounds, and every measurement has been consistent with more
than one explanation. Recording it as OPEN-WITH-CAUSE in the matrix with this round's table is the honest end of it.
### Discover's pagination row sits 27 points right (2026-11-09)

Discover is the worst page in the table (82 179 differing) and 82 per cent of it is card artwork, so the clusters are only useful where they are not
artwork. The two heaviest are x 960 y 192 (644) and x 928 y 192 (513), and at 3x they are the **pagination row**: filled circle, `2`, an ellipsis, and
in the reference a final `9`.

Measured by the brand-green ink on that row:

| | green pixels at y 190..215 | centre |
|---|---|---:|
| reference | x **941..946** (the ring's left edge; the filled centre is covered by the numeral) | **943** |
| this build | x **952..987** (the whole disc) | **970** |

**Ours is 27 points right**, and the reference's row ends with a `9` that this build does not draw at all -- visible in the 3x crop as both the offset
and the missing last page.

**Not changed**: the round's build went to the card-row attempt, which was reverted. The lead is the pagination's own placement -- whether it is centred
on a rect that differs, or right-aligned to a different edge -- and the missing final page is a second, separate defect in how the page count is chosen.

**Why this is worth a row**: Discover has never been examined cluster by cluster in this session, and its flat share is only 18 per cent, which means
almost every non-artwork difference it carries is still unlisted.
### Discover's pager is right-aligned to the row, not to the row's inner edge (2026-11-09)

The 27-point offset found last round is located:

```rust
// shell.rs:2459
fn draw_pager(&mut self, ui: &mut Ui, right_edge: f32, y: f32, tag: &str)
// and its one call, shell.rs:3430
self.draw_pager(ui, row.right(), row.center().y, "top");
```

The pager is laid out from `row.right()` inwards, and the reference's sits in a padded container -- the comment at `:2537` already records that its
page 1 reads `1 2 ... 961 >`, i.e. it is a container-wrapped pager rather than a bare run of buttons. So the fix is the edge it is measured from, not
the list it draws: ours starts 27 points further right because it is anchored to the row's outer edge.

**Not built this round**: the round's build went to a reverted attempt and there was no rebuild time left after locating this. The change is one
argument at `:3430` (or one inset inside `draw_pager`), and the measurement to check it against is the brand-green ink centre: **943 in the reference,
970 here**.

**Also unverified from the earlier crop**: whether the reference's trailing `9`/`961` and its final chevron are both drawn here. The comment at `:2537`
says the chevron logic was considered; the capture says the reference's row ends with a numeral after the ellipsis and this build's does not, which
would be a second defect in the same widget.
### The pager inset could not be verified, so it was reverted (2026-11-09)

The change built (`PAGER_INSET = 27.0`, applied at `shell.rs:3430` as `row.right() - Self::PAGER_INSET`) and the verification failed for a reason that has
nothing to do with the change: **this build cannot be put into the reference capture's state.**

| attempt | result |
|---|---|
| click rail y 160 | landed on **Skins** -- wrong row, compared 205 535 against a Discover reference |
| click rail y 62 | **Discover with no query**, no results, no pager -- no green ink to measure |
| click at (500,62), type `sodium`, Return | **identical frame** (132 862), so the text never went into the field |

So the diff between the reference capture and this app is not just the pager: the reference's frame has a populated search, and reproducing it needs a
working text field and a network answer -- neither of which this round could drive.

**Reverted**, because a change whose effect was never observed is a change I cannot tell from a no-op. The tree is clean, the rebuild is queued, and the
finding stands as written last round: the pager is anchored to `row.right()` where the reference's sits 27 points in, with the ink measurement (943
against 970) as the check to run once a populated Discover can be reached.

**One thing learned about the lab**: the rail's Discover row is at y **62**, not the 205 used earlier in the session -- 205 is Screenshots. Three
captures were spoilt by that in this session alone.
### WITHDRAWN: the pager was never 27 points out (2026-11-09)

The "Discover's pagination row sits 27 points right" finding (rounds 407-408) is **wrong**, and the fix I wrote for it would have introduced a 27-point error.

With Discover finally in the reference's state -- populated, which took mapping the rail with `SIFT_TITLE_PROBE` (Home, Discover at **y 120**, Skins,
Screenshots, Servers; the 205 used all session is Screenshots) -- the brand-green ink on the current-page pill measures:

| | green pixels | centre |
|---|---|---:|
| reference | 941..946 | **943** |
| this build | 925..960 | **942.5** |

**Half a point apart.** The earlier measurement compared this build on Discover-with-no-results against a reference with results, where the pager's width
and therefore its start differ because the page count differs. The offset was in the pair, not in the widget.

**So `PAGER_INSET` was reverted for the right reason and the finding it came from is withdrawn.** Two rounds went to it: one to measure an invalid pair,
one to build and revert a change that would have broken a working alignment.

**Discover now compares at 79 574** against the reference's 82 179 for the same state -- the closest this page has been, and the first figure for it that
was taken on both clients in the same state.

**The lesson, third time in ten rounds**: a finding is only a finding once both frames are known to show the same thing. Every invalid pair this session
produced a confident, precise, wrong number -- 27 points, 10 189 pixels outside a plate, a 226-wide rule -- and each cost at least one round.
### Discover's heaviest flat cell is the checklist, and the two frames disagree about which step is done (2026-11-09)

With both frames finally in the same state, Discover reads **81 444 differing (17% flat)** and its heaviest flat cell is x 1184 y 128 (224), which at 2x
is the **Getting started** checklist: the reference strikes through "Create first instance" and dims it, this build shows the same row ticked but with
plain bright text and no line.

**The code already does both.** `panel.rs:955-1016` is explicit: a completed row draws the check instead of the radio, dims the label to
`#6d777c` ("sampled from the brightest pixel of the label's own box"), and then draws a line through it when `done`. So a row that is ticked but not
struck is not a state this code can produce -- which means the two frames are showing **different steps**, and the difference is in which step each
client considers complete, not in how a complete one is drawn.

**That is what the evidence supports and no more**: `done_row = index == 0 && library_has_instance()`, so the first step's state depends on whether the
library holds an instance. The reference's sidebar was populated from its own data directory; ours from `/tmp/matchdata`. Different libraries, different
first step, and the checklist is the one part of the page that reflects that.

**Not changed**, because changing the drawing would have made a working path wrong. **What would settle it**: run both clients against the same
library contents, or capture the checklist on a page where neither has an instance -- neither is available in the rounds left, which is why this is
recorded rather than fixed.
### CORRECTED: both draw the strike; the checklist sits nine points apart (2026-11-09)

Last round concluded the two frames showed different *steps*, because this build's row looked ticked but not struck. That was wrong, and the measurement is
simple: a horizontal bright run inside the checklist's rows -- the strike itself -- is present in **both**, 111 pixels wide in the reference and 112 here.

| | strike's y | width |
|---|---:|---:|
| reference | **148** | 111 |
| this build | **139** | 112 |

So the line is drawn in both and is the same length; the **checklist sits nine points higher here**. The `done` path (`panel.rs:1026-1044`) is running
correctly in both, and nothing about the strike needed changing.

**What this means for the cell**: the 224 differing pixels at x 1184 y 128 are a **nine-point offset in the right panel's Getting started card**, not a
missing decoration -- the same family as the Settings card-row drift, and the same caution applies: a difference that looks like a missing style is often
a displaced element, and the tell is that both frames carry the "missing" ink at different coordinates.

**Withdrawn**: "the two clients read different libraries, so the checklist shows a different first step". The libraries may still differ, but this cell is
not evidence of it, and the caveat added to `FINISHED.md` last round should be read as unproven rather than established.
### The checklist's rows now sit where the reference's do: strike at y 148 in both (2026-11-09)

`STEPS_TOP_NUDGE = 9.0`, added only to the steps' start (`panel.rs:1550`) because `HEADING_TO_BODY` is shared with other panels -- the lesson from
the card gap, where changing a shared constant distorted the cards.

| measurement | before | after | reference |
|---|---:|---:|---:|
| the strike's y in the first row | 139 | **148** | **148** |
| the News heading's ink | 300 | **307** | 313 |
| Discover, four slabs | 79 574 | 79 695 | -- |

**The rows are aligned exactly** and the section below moved most of the way to where it belongs. The page total rose by 121 (0.15 per cent),
which by this project's rule is a warning rather than a win -- but the target of this change was never the total: Discover's count is 83 per cent
artwork, and a nine-point shift in one card moves the artwork comparison in ways that have nothing to do with the card. The direct measurement is the
strike's y, and it now matches to the pixel in both clients.

**Kept, on that basis, and recorded with both numbers** so the judgement can be re-examined rather than assumed: aligned rows and six points still
to find in the section gap below the card, against a total that moved 0.15 per cent in the wrong direction.
### `NEWS_HEADING_DROP` is a no-op on the heading that is out (2026-11-09)

The News heading reads **307** against the reference's **313**, and the constant that carries it looked like `NEWS_HEADING_DROP = 29.0`
(`panel.rs:222`), used at `:1243` as `plan.news + NEWS_HEADING_DROP`. Changed to 35, built and measured:

| | before | after |
|---|---:|---:|
| the News heading's ink | 307 | **307** |
| Discover, four slabs | 79 695 | **79 695** |

**Identical**, so that constant is not on the path of the heading this probe finds -- either another heading is drawn above it in the same
x-range, or the pager draws `NEWS` somewhere else as well. Reverted; the tree is clean and the binary rebuilt from HEAD.

**What the probe measures**: the first row in x 1165..1330, y 300..360 whose bright pixels exceed twelve. On this page the sidebar holds *two*
headings in that column -- `News` and, above the category list on Discover, `Category` -- so a six-point difference in one and a six-point difference in
the other are the same measurement to this test. That is the flaw, and the next attempt should print both rectangles rather than inferring from ink.
### The nudge belonged on the rows, not the plan -- and the suite proved it (2026-11-09)

The checklist fix from two rounds ago moved the rows to the right place and **broke something else**: the suite failed,

```
panel::tests::a_section_heading_sits_twenty_below_the_section_above_it
assertion failed: and the card ends 26 above it
  left: 17.0   right: 26.0
```

because `steps_first` was nudged, and everything planned from it moved too: the card's bottom, the News art and the banner gap. So the change traded a
correct checklist for a nine-point error in the artwork below it -- which is exactly what the earlier comment in this file warned about ("shifting the
whole news section down by 20 measured far worse").

**Fixed properly**: `steps_first` is back to the plan's value and `STEPS_TOP_NUDGE` is added in two places that are *drawn* rather than planned -- the
row positions and the card's own bottom edge:

```rust
row_top(plan.steps_first, index) + STEPS_TOP_NUDGE            // the rows
plan.steps_first + 3.0 * ROW_PITCH - ROW_GAP + 13.0 + STEPS_TOP_NUDGE   // the card's foot
```

**269 tests pass**, including the one that caught it, and the row positions are unchanged from the version that measured the strike at y 148 in both
clients. **The release build is queued and the on-screen confirmation of the artwork's return is not yet taken** -- this entry will not claim it until it
is.

**Worth stating plainly**: the wrong version was committed and reported as a fix, and the only reason it did not stand is that the suite ran. That is the
clause working exactly as intended, and it is the second time this session a test has caught a change I had already called correct.
### Verified on screen: the checklist aligns and Discover drops 1 933 (2026-11-09)

The corrected nudge -- rows and the card's foot moved, the plan left alone -- built and captured against the reference in the same state:

| measurement | before any nudge | after the wrong one | after this one | reference |
|---|---:|---:|---:|---:|
| the strike's y | 139 | 148 | **148** | 148 |
| Discover, four slabs | 79 574 | 79 695 | **77 641** | -- |
| the sidebar slab | 17 139 | 17 207 | **15 153** | -- |

**So the checklist's rows are aligned to the pixel and the artwork kept its place**, and the page fell **1 933** where the wrong version had raised it by
121. The sidebar slab fell 1 986, which is where almost all of it lands, as it should for a change inside the right panel.

**This is the shape a fix should have**: a direct measurement that matches exactly (the strike's y), a page total that falls, and the slab that owns the
element carrying the whole of the movement. Two rounds produced it -- one wrong, caught by the suite, and one right -- and the difference between them was
whether the nudge was applied where the drawing happens or where the plan does.
### The "News heading six points out" was a toggle label, and the label was wrong (2026-11-09)

Cropping the band the probe called the News heading (x 1160..1400, y 290..340) shows it is **not a heading at all**: it is the
`Hide already installed` switch, and the two clients read different words --

| | label |
|---|---|
| reference | `Hide already installed` |
| this build | `Hide content already installed` |

So three rounds of "the heading is six points low" were measuring a label that was a different length. The probe took the first bright row in that band,
which was this switch's text in both clients -- the y difference was the glyph heights of two different strings, not a misplaced heading.

**The string was wrong, and the reference says so twice**: `Browse.vue:574` has `defaultMessage: 'Hide already installed'` and
`en-US/index.json:258` reads `"Hide already installed"`. The constant now matches, and the comment above it -- which claimed this build used the *shorter*
wrong string -- is corrected, since it described the opposite of the line beneath it.

| | before | after |
|---|---:|---:|
| Discover, four slabs | 77 641 | **77 462** |
| the sidebar slab | 15 153 | **14 974** |

**Both labels read identically in the capture**, and the whole 179-pixel gain lands in the sidebar slab, which is where the label lives.
### The panel fixes were worth 1 986 on Home as well (2026-11-09)

The checklist and switch fixes were made against Discover, and Home carries the same right panel. Re-measured on the final build:

| | before the panel work | after |
|---|---:|---:|
| Home, four slabs | 31 356 | **29 370** |
| the sidebar slab | 16 565 | **14 579** |

**So the gain is 1 986 and it lands entirely in the sidebar** -- the same 1 986 that appeared on Discover, because it is the same slab on both pages.
That is what a shared-panel fix should look like, and it is the largest single improvement of the last twenty rounds.

**Final state of this tree**: 269 tests pass, the release binary is built from HEAD, the working tree is clean at 912 commits, and every figure quoted
in `FINISHED.md` was re-measured on this build rather than carried forward.
### The sidebar has no clip, so its copy draws through the banner (2026-11-10)

Resuming after the round cap. The sidebar's news block is the largest remaining structural gap, and the first thing found is smaller and more concrete
than the card count: **`panel.rs` contains no `with_clip_rect` at all.**

```
$ grep -n "with_clip_rect\|set_clip_rect\|clip_rect" crates/sift-ui/src/panel.rs
(nothing)
```

So the news card's fourth copy line is drawn wherever it lands, and at 1440x900 it lands **on top of the pinned hosting banner** -- in the reference
the same line is cut by the banner's own top edge. The crop at (1150..1440, 270..700) shows both side by side: the reference's "managed by you." ends
above the banner, while this build's "your server..." runs into it.

**The fix is one clip**, and it is also the precondition for the card count: a scrolling or overflowing news list is only safe once the area is clipped at
`banner_top`, which the plan already computes (`panel.rs:880`, `panel.bottom() - BANNER_H`). The shape is

```rust
let painter = painter.with_clip_rect(Rect::from_min_max(
    pos2(column.left(), plan.news - 24.0),
    pos2(column.right(), banner_top),
));
```

wrapping the news block's art, title, copy and date -- all of which currently call `painter` directly.

**Also established this round, so it is not tried again**: the copy's line breaks cannot be matched by narrowing the measure. `panel.rs:826-828` records
the measurement -- "at -27 ... the sidebar worsens to 16 700 from 16 565: the break moves, but every one of the four lines shifts with it".
### The banner overlap is the reference's own behaviour; the clip was a no-op (2026-11-10)

Two findings, both from measurement rather than the crop that prompted them:

**One, the copy is already aligned.** The news copy's first ink row is **566 in the reference and 565 here** -- one point apart, which is the same
sub-pixel difference the rest of the page shows. The crop that suggested a 15-point offset was read against the wrong baseline.

**Two, the reference overlaps the banner too.** Its fourth line, "managed by you.", is struck through by "Upgrade to Modrinth+" in exactly the way this
build's is. So there was no clipping defect to fix: both clients draw the sidebar's text over the promo, and the clip I added at `banner_top` (626) was
above all the content, so it changed **nothing** -- Home read 29 370 both before and after, to the pixel.

**Reverted.** The clip is a correct thing to have when the news list grows past the pane, and it belongs in that change rather than before it, where it
has no observable effect and therefore cannot be told from a no-op.

**What the news-list work therefore needs**: more cards, not a clip. At 1440x900 this build draws one article and the reference draws one and a half --
because the reference's list continues below the fold, and the fold is the banner. A second card drawn where the plan puts the first plus one card's
height would land inside the banner's own area, which is when the clip earns its place.
### The second card is drawn and immediately clipped away (2026-11-10)

Implemented the loop the last round specified: two articles, the second at `plan.card + rows.height + 16`, both inside a clip at `banner_top`. It built
and ran, and **nothing appeared**: the two-card frame is identical to the one-card frame apart from 47 pixels, and the sidebar slab moved 14 579 ->
14 626.

**Why**: the second card starts at `plan.card + rows.height + NEWS_CARD_GAP`, which is at or below `banner_top`, so the clip that was needed to keep it
out of the banner removes all of it. The two changes cancel.

**What that proves**: this build's first card is **taller than the reference's**. At 1440x900 the reference fits one and a half cards above its banner,
which is only possible if its card is shorter than the space this build's one card occupies. So the news list has never been a card-count problem --
the count is a consequence of the card's height, and until the height matches, adding cards will keep drawing them into the clip.

**Measured, for the next attempt**: the card's own rows come from `news_rows(copy_height, title_height)` (`panel.rs:1508`) -- art 150, the title's
measured two lines, the copy's measured height, and `DATE_LINE`. The reference's article card is shorter than 260 points at this width; what to find is
which of those four terms is over-tall, by measuring the reference's date line against this build's (`news_rows` puts it at `copy + copy_height +
COPY_TO_DATE`, and the copy's own ink starts within a point in both).

**Reverted** -- a change that moves 47 pixels in the wrong direction and shows nothing has no claim to stay.
### The news card's internals are eight points too tall, not the section (2026-11-10)

Measured first this time, as the previous round's note required. The article's art plate is a saturated band, so it can be found by colour:

| | art band | copy's first ink row | art -> copy |
|---|---|---:|---:|
| reference | **370..479** | 566 | **196** |
| this build, section gap 28 | **362..471** | 565 | **203** |
| this build, section gap 36 | **370..479** | 573 | 203 |

**So the art is eight points high and the copy is already in place** -- the distance between them is 203 where the reference's is 196. Raising the section gap
moved the art exactly onto the reference's band (370..479 in both, to the pixel) and simultaneously pushed the copy eight points low, which cost **622**
pixels: Home 29 370 -> 29 992.

**The correct change is therefore two terms, not one**: the card's top moves down eight (so the art lands at 370) and the art-to-title gap shrinks by the same
eight (so the copy stays at 566). Any change that moves only one of them trades the art against the copy, which is what the last two rounds did in opposite
directions.

**The exact terms**: `plan.card` sets the art's top; `ART_TO_TITLE` then `TITLE_TO_COPY` (with the title's measured height) set the copy. The eight belongs
between the art and the title, and the card's own top needs to follow the section rather than lead it.

**Reverted and recorded rather than attempted**, because the two-term change needs its own build and verification and this round's went to establishing
which term is wrong -- the thing the last three rounds guessed at.
### The news card, fixed as a pair: Home 29 370 -> 28 521 (2026-11-10)

The two-term change from last round's diagnosis, applied together and measured on both numbers:

| term | before | after |
|---|---:|---:|
| `NEWS_HEADING_TO_CARD` | 33 | **41** |
| `ART_TO_TITLE` | 24 | **16** |

| measurement | before | after | reference |
|---|---:|---:|---:|
| the article's art band | 362..471 | **370..479** | **370..479** |
| the copy's first ink row | 565 | **565** | 566 |
| Home, four slabs | 29 370 | **28 521** | -- |
| the sidebar slab | 14 579 | **13 730** | -- |

**The art is now on the reference's band to the pixel and the copy did not move**, which is exactly what the pair was chosen to do: the card's top
rose eight and its art-to-title gap fell eight, so the art moved and everything after it stayed. Home fell **849**, all of it in the sidebar.

**One prediction was wrong and the suite caught it.** I changed the test's card-to-banner assertion from 26 to 18 on the reasoning that a card whose top
moved down eight would end eight lower. It does not: `rows.height` is derived from the copy, which did not move, so the card's *height* fell by the same
eight and the gap stayed at **26**. The assertion is restored to 26 with that reasoning recorded, and 269 tests pass.

**So this is the first change since the checklist to improve a measurement and the total at once**, and the reason is that it corrected two terms
whose errors were opposite. Every earlier attempt on this card moved one of them.
### The biggest remaining gap is reachable: the lab has network, and the skins come over it (2026-11-10)

The matrix's largest cluster is **skins (23 OPEN rows)**, and the page's largest single difference is its artwork: nine real skin textures against
this build's flat stand-ins, **144 609 pixels**. The question was whether that art is obtainable, and the answer is yes.

**The data directories do not have them** -- `/tmp/ref107` holds only caches and a screenshot, and the extracted AppImage contains no file with
`skin` in its name.

**But the lab has a network**, which nothing in this session had established:

```
$ getent hosts textures.minecraft.net
2603:1061:14:64::1  mr-afd-azuredge.tm-azurefd.net  textures.minecraft.net ...
$ curl -s -o /tmp/s.png -w "%{http_code}" <a skin texture url>
200   (1 469 bytes)
```

**Where the app gets its list**: `Skins.vue` calls `@/helpers/skins.ts`, and that helper's only import is `invoke` from `@tauri-apps/api/core` -- so the
catalogue and the textures are fetched by the app's **Rust** side, not by a URL the frontend names. The next step is the reference's Rust source for the
command `skins.ts` invokes, which will name the endpoint.

**Why this matters more than another sidebar point**: 144 609 pixels sit on one page's artwork, and every other open item in this session has been worth
hundreds. If the endpoint serves the same nine defaults, fetching and drawing them is the largest single improvement still available -- and it is also the
kind of work that a rasteriser cannot undo, unlike the glyph edges that cap the rest.
### The stand-ins are procedural art, not missing files (2026-11-10)

With the reference's 45 textures committed, the page's drawing path is now known exactly:

```rust
// skins.rs:1775
fn skin_texture(ctx, state, name) -> Option<TextureHandle> {
    let index = SKINS.iter().position(|skin| skin.name == name)?;
    if index >= ART.len() { return None; }   // "more skins than this build authors art for"
    art_texture(ctx, state, index)           // procedurally authored art, not a skin file
}
```

So the flat characters are **authored in `ART`**, not the result of missing assets -- which is why the page has never shown a real skin. Two changes follow, and
they are separable:

1. **The texture**: `skin_texture` should load `assets/skins/NN-<name>.png` (committed last round, 45 files, 130 535 bytes) instead of `art_texture`.
2. **The drawing**: `default_card` (`skins.rs:1407`) currently blits the whole sheet into a `height/2`-wide box -- the entire 64x64 texture squashed into a
   portrait column. A front view is a composition of the sheet's own faces: head `(8,8)-(16,16)`, body `(20,20)-(28,32)`, right arm `(44,20)-(48,32)`,
   left arm `(36,52)-(40,64)` for 64x64 skins, with the legs at `(4,20)` and `(20,52)`.

**Which to do first**: the drawing, on the existing stand-in art. Changing the texture first puts a real skin through a squash that has never been
evaluated for it, and the page's 144 609-pixel difference is in the *shape* of the character (a tall flat plate) as much as in its colours. The
measurement to beat is the same on both: the page's four-slab total, 135 498 at last count.
### Real skins through the existing compositor made the page worse (2026-11-10)

Wired the 45 baked textures into `art_texture`: decoded with `image::load_from_memory`, composed through the same `compose_front`, uploaded instead of
`ART[index].atlas()` when the catalogue had a skin. It built and ran, and the page went from **135 498 to 147 250** -- **11 752 worse**.

Two causes, visible in the crop:

1. **The grid repeats one skin.** `SKINS` is this build's own eight-name sample list; the baked catalogue's names are the reference's (`Mr. Pack -
   Asexual`, ...). Indexing the baked array by `SKINS`' position therefore draws the first files of *one section* in every cell -- three neighbours in the
   grid are the same character. The reference's grid shows its `Default skins` section, which is a different set at a different offset.
2. **The composed view is squashed.** `default_card` draws it into a `height/2`-wide box (`skins.rs:1414`), which was sized for the authored atlas's
   aspect. `compose_front` produces its own `VIEW_W x VIEW_H`, and a real skin's front view is not that ratio -- hence the narrow, blurred columns.

**Reverted** (`skins.rs`, `main.rs` restored, the generated module deleted); the assets themselves stay committed because they are the reference's own art and
will be needed by whichever attempt gets the two causes right. The build is queued.

**So the order I argued for last round was right and I did the opposite**: the drawing's aspect should have been fixed first, on art whose shape was already
known, and only then the texture swapped. Doing both at once made the result unattributable -- and the crop shows both faults at once, which is exactly the
confusion the staging was meant to avoid.

**Next attempt, in order**: (1) `default_card` draws the composed view at `compose_front`'s own aspect, measured against the reference's cell shape;
(2) map the catalogue properly -- the reference's `Default skins` section first, by name, not by index into this build's list.
### The skins grid is the reference's eight defaults, and ours is not (2026-11-10)

Two facts that settle the mapping, both read out of the reference's own asset file:

**The visible section is `Default skins`**, whose entries are, in order: **Alex, Ari, Efe, Kai, Makena, Noor, Steve, Sunny** -- each shipped twice,
once `Classic` and once `Slim`, which is why the 45 baked files contain duplicate names. The reference's grid at 1440x900 shows eight of them.

**Our `SKINS` list is eight sample names of this build's own invention**, so no index into the baked catalogue can line up with it: index 0 of the
baked set is `Mr. Pack - Asexual`, a pride skin, which is why the attempt last round drew three near-identical dark characters and scored 11 752 worse.

**And the aspect was never the problem.** `VIEW_W = 16`, `VIEW_H = 32` (`skins.rs:1535`) and `default_card` draws into `height/2 x height` -- the
same 1:2. What the reference's cells show is a **3D model rendered at high resolution**; what this build draws is a **16x32 sprite** scaled up, which
is inherently blocky and cannot match a shaded render whatever texture is behind it. So the artwork swap is worth doing for *colour* fidelity and
cannot by itself close 144 609 pixels.

**The change the next attempt should make, in one edit**: replace `SKINS`' eight names with the reference's eight defaults (classic variants), keep
the labels, and map each name to its baked texture by name rather than by index. That is a data change in one array plus the loader already written
and reverted -- no drawing code, so its effect is attributable.

**What to expect**: the grid's characters change from invented ones to the reference's own, which should fix the *colours* and leave the *shape*
difference (sprite against render) in place. The page's number will move; by how much is the measurement.
### The skins grid now shows the reference's own eight defaults: 135 498 -> 132 429 (2026-11-10)

The change the last two rounds prepared, done as one data edit plus the loader written and reverted before it:

* `SKINS`' eight invented names (`Overgrown`, `Copper Patina`, ...) became the reference's `Default skins` section -- **Alex, Ari, Efe, Kai,
  Makena, Noor, Steve, Sunny**;
* `skin_texture` looks each name up in the baked catalogue (`crate::skins_assets::by_name`) and composes that front view, falling back to the authored
  atlas only for names the catalogue does not carry.

| | before | after |
|---|---:|---:|
| the Skins page, four slabs | 135 498 | **132 429** |
| its content slab | 119 165 | **116 256** |

**Minus 3 069**, and the grid shows the reference's characters rather than invented ones: an orange Alex, a blue Steve, a purple Ari, where before
there were near-identical dark pride skins. The crop at (560..1120, 300..620) puts them side by side.

**What it does not fix, and cannot**: the reference renders a **3D model** and this build composes a **16x32 sprite** scaled up. The characters are now
the right ones and the shapes are still flat -- which is why 3 069 of 144 609 came back and not more. That ceiling is a renderer, not a texture, and it
is the honest end of this page's artwork.
### All five sizes re-measured after the sidebar work: every one improved (2026-11-10)

| size | before this stretch | now | change |
|---|---:|---:|---:|
| 1280x720 | 27 933 | **25 099** | -2 834 |
| 1366x768 | 36 268 | **33 456** | -2 812 |
| 1440x900 | 29 370 | **28 521** | -849 |
| 1600x900 | 45 470 | **42 635** | -2 835 |
| 1920x1080 | 57 864 | **55 027** | -2 837 |

Every capture was size-checked before it was compared, and the Home page was opened afresh for the sweep.

**Four of the five moved by almost exactly the same amount** (2 810-2 837), which is what a fixed-width sidebar's improvement should do: the news card
and the checklist live in a column of constant width, so its contribution is the same at every window size. The odd one out is 1440x900 at 849, and
that is because its figure had already been re-taken after the news-card fix while the others were still quoted from before it.

**So the honest reading of this stretch**: the sidebar work is worth about **2 800 pixels per screen** at four of the five standard sizes, and the
skins work is worth 3 069 on its own page. Neither moves the 99 per cent clause, whose budget at 1280x720 is 9 216 pixels: even the best screen
(1280x720 at 25 099) is 2.7 times it.
### The instance row opens the instance page in the reference and does nothing here (2026-11-10)

Drove the same press in both clients -- (170, 145) on Home's "Jump in" strip, inside the one instance's row -- and captured each afterwards:

| | after the press |
|---|---|
| reference | the **instance page**: its icon, "Fabric 26.3", "Fabric 26.3  •  Never played", and the Content / Files / Worlds / Logs / Share tabs |
| this build | **Home, unchanged**: "Jump in", the same instance row, "Library" below |

The pair reads 77 138, which is not a visual difference but two different pages.

**So either the row is not pressable here or the press missed its rectangle.** That question has a one-line answer that I did not have time to run: the
pages already print their press rectangles under a probe (`SIFT_HOME_PROBE` on this page), and the rule this session learned the hard way is that a press
which does nothing is evidence about the press until a printed rect says otherwise.

**Recorded rather than guessed**, because the two explanations imply opposite work: a missing handler is a behaviour to implement, and a missed rect is a
coordinate to fix. The next round should print Home's rects, press the printed one, and only then say which it is.

**What the pair does establish**: the two clients' Home pages at 1440x900 were within 32 715 of each other immediately before the press, so the rows are
drawn alike; the divergence is in what happens when one is pressed.
### The "Jump in" strip is drawn and never made pressable (2026-11-10)

Last round's ambiguity is resolved, and it was not a missed coordinate:

* `SIFT_HOME_PROBE` prints this build's instance rectangles -- `card=[[88,329]-[256,568]]`, `play`, `kebab` -- and **no strip row at all**;
* pressing the **library card** at (120,400) **navigates** to the instance page, whose header matches the reference's (icon, name, loader line, and the
  Content / Files / Worlds / Logs / Share tabs);
* pressing the **strip row** at (170,145) does nothing.

**So the row is drawn and never registered for input.** `home.rs:1132-1300` draws the strip, its icon, its name, its badge, its meta line and its Play
button with `painter` calls only -- there is no `ui.interact` anywhere in that block. The library card does the opposite: `ui.interact(card,
egui::Id::new(("card", &instance.id)), Sense::click())` at `:1924` and reports `Action::Open(instance.id.clone())` at `:1955`.

**The fix is two lines, mirroring the card**: an `interact` on the strip's rect with an id keyed by the instance, and `Action::Open` on a press. It needs one
care -- the strip's Play button and its badge sit inside that rect, so the press must not fire from them, which is why the card's own handler sits *after*
its children's.

**Recorded rather than attempted**: the round ran out after establishing the cause, and this is the kind of two-line change that must be built and driven
to be worth anything -- the reference navigates on that press, so the test is a capture of our instance page after pressing the strip.

**Worth noting what kind of defect this is**: the strip is *visually* right and *behaviourally* dead, which no pixel count can find. It is the first
clear example this session of the objective's "every interaction" clause failing while the fidelity clause passes, and there are likely to be others among
the 434 OPEN rows that only a drive can expose.
### The two-line fix is not two lines: `list`'s action is declared 500 lines later (2026-11-10)

Added the `interact` on the strip's rect with `Action::Open`, exactly as planned, and it did not compile:

```
src/pages/home.rs:1171:13: error[E0425]: cannot find value `action` in this scope
```

**Because the strip is inside `fn list(...) -> Action` (`home.rs:1089`), and that function's own `let mut action = Action::None;` is at line
**1654** -- five hundred lines after the strip. The `action` at 1032 belongs to a different function. And the strip's rect is a `let` inside the
`if let Some(recent)` block, so it is out of scope by the time the function's action exists.

**So the fix needs one of two shapes**, neither of which is two lines:

1. a `let mut jump_in: Option<String> = None;` declared before the strip, set by the press, and applied to `list`'s `action` after line 1654;
2. moving the strip's block (or its press) to where the action lives -- a much larger change to a 570-line function.

**Shape 1 is the honest one** and is about four lines: the declaration, the assignment, and the application. **Reverted** rather than half-wired, and
recorded with the line numbers so the next attempt does not rediscover this.

**What this says about the defect's cost**: the strip is dead because the function that draws it collects its actions in one place at the end, and the strip
was added to the drawing side without a matching entry on the collecting side. That is a plausible reason for a whole class of these: anything drawn early
in a long function and pressed early has no action to set. The probes can find them; the fixes are each a small threading job.
### The "Jump in" strip opens the instance now (2026-11-10)

The threading fix, in the shape `toggled` already used in the same function:

```rust
let mut jump_in: Option<String> = None;          // beside `toggled`, before the strip is drawn
// in the strip block, before its icon and Play, so those children still win their own rects:
if ui.interact(strip, egui::Id::new(("jump-in", &recent.id)), Sense::click()).clicked() {
    jump_in = Some(recent.id.clone());
}
// where the function applies `toggled`:
if let Some(id) = jump_in { action = Action::Open(id); }
```

**Driven and verified**: a press at (170,145) on Home's strip now opens the instance page -- icon, "Fabric 26.3", "Fabric 26.3  •  Never
played", and the Content / Files / Worlds / Logs / Share tabs -- where before the press did nothing at all.

**This was a behaviour defect with no pixel signature.** The strip was drawn correctly and was completely dead; no count on any page could have found it,
and it took a driven press in both clients plus the page's own printed rectangles to establish. It is the clearest example this session of the objective's
interaction clause failing while its fidelity clause passed -- and the first of its class to be fixed.

**Four rounds for four lines**, and the three that found it were worth more than the one that wrote it: the press that did nothing, the probe that
showed no strip rect existed, and the grep that showed `action` was 500 lines downstream.
### The dead-press scan finds nothing, and that is the finding (2026-11-10)

Wrote `tools/parity/deadpress.py` to find the class named at round 436 -- functions that name an `Action`, paint, and register no press:

```
$ python3 tools/parity/deadpress.py
0 candidates: functions that name an Action, paint, and register no press
```

**Zero, and the reason matters**: the strip lived inside `fn list`, which *does* register presses (the card, the kebab). A function-level scan
cannot see a dead row inside a live function, which is exactly where this one was. The tool is committed with that limitation written into its own
docstring rather than left to mislead someone into thinking the class is empty.

**What would find it** is a runtime sweep -- press each rectangle the page prints and diff the frame before and after, flagging the ones where nothing
changes and the reference does something. That is one probe per page and one capture per rect, which is affordable for the pages that have probes
(Home, Screenshots, Settings, Project, Skins) and not for the ones that do not.

**Measured while here: the instance page's active tab matches.** On `inst2-ref.png` against `strip-fixed.png`, both in the same state:

| property | reference | this build |
|---|---:|---:|
| active pill y extent | 174..209 | 174..209 |
| active pill height | **36** | **36** |
| active pill width | 121 | 116 |

So the instance page's tab strip has the same geometry as the category chips to the pixel, and the same 4-5 point width difference -- which is the
label's own measure and appears on every `px-4` pill in the app. That is one font-metric cause behind several rows rather than several causes.
### The runtime sweep works, and it found two dead switches (2026-11-10)

Pressing each rectangle `SIFT_SETTINGS_PROBE` prints, capturing before and after, and comparing the two frames:

| press | frame change | reading |
|---|---:|---|
| theme card `Dark` | **0** | already selected -- correct, and the control that shows the test works |
| theme card `Light` | 846 127 | switches the theme |
| row `Sync theme across devices` | **0** | **dead** |
| row `Advanced rendering` | **0** | **dead** |
| row `System window frame` | 632 547 | flips its switch |

**So two of the five targets do nothing when pressed**, and the probe reports their controls with state (`Switch { on: false }` and
`Switch { on: true }`), which means they are drawn as live switches and read as dead ones. That is the same class as the "Jump in" strip: a control that
pixels cannot distinguish from a working one.

**The method is now a loop, not a ritual**: a list of presses, a before and an after, and a diff. The zero that matters is the one on a control the
reference makes pressable; the zero on an already-selected card is the control that proves the test can tell the difference.

**Not fixed yet**: both switches need wiring, and "Advanced rendering" may have no behaviour behind it in this build at all -- which would make it a
deliberate omission rather than a defect, and should be recorded as such rather than silently wired to nothing.
### One of the two dead switches is a defect; the other matches the reference (2026-11-10)

Ran the same sweep against the reference, after finding its own rows (its "Sync theme across devices" switch sits at y **679**, its
"Advanced rendering" at 780 and is visibly dimmed):

| press | this build | reference | reading |
|---|---:|---:|---|
| `Sync theme across devices` | **0** | **1 147** | **a real defect here** -- the reference flips its switch and this build does not |
| `Advanced rendering` | **0** | **0** | **not a defect** -- the reference's switch is disabled and does nothing either |

**So the sweep's two zeros had different meanings**, which is exactly why the reference has to be pressed and not inferred. `Advanced rendering` should
be drawn **disabled** -- the reference's label is dimmed and ours is not -- and its row should close against that rather than against a working switch.

**And a layout difference fell out of the same look**: the reference's `Sync theme across devices` row is at y **679** where this build's is at **643**,
36 points higher. The two dialogs' content is otherwise aligned at the top (the theme cards match to the pixel), so the 36 is somewhere between the cards
and this row -- the same family as the five-point drift already recorded there, but six times larger and easier to find.

**Not fixed**: both. The switch needs its press wired to the state the probe already prints, and `Advanced rendering` needs the reference's dimmed
treatment; the 36 points need one measurement of the row above it before anything moves.
### CORRECTION: the Sync switch is disabled on purpose, and the reference is signed in (2026-11-10)

Last round I called `Sync theme across devices` a real defect because the reference flips its switch and this build does not. Reading the code that
refuses the press:

```rust
// settings.rs:3996
Control::Switch { on } => {
    // A sync switch is disabled while signed out: it keeps the value its row declares, refuses the press,
    // and says why on hover (`layout.vue:60-75`).
    if let Some(tip) = sync_switch_tooltip(label) {
        ...state.hover_tip = Some((rect, tip));
        Control::Switch { on }        // unchanged: the press is refused
    } else { ... }
```

**So the dead press is the designed behaviour for a signed-out client**, and the reference's 1 147-pixel flip happened because its client is signed in -- its
demo account -- not because of anything this build lacks. The sweep compared two clients in different account states, which is the same class of error as
the pager at 205 and the checklist at y 145, and it is the third time a matching account state was the unstated premise.

**Withdrawn**: "`Sync theme across devices` is a real defect here". The row matches the reference's own signed-out behaviour, and `layout.vue:60-75` is
the source it was ported from.

**What survives from the sweep**: `Advanced rendering` does nothing in either client, and the reference dims its label while this build does not -- a
visual difference in the disabled treatment, not a behavioural one. And the reference's row sits at y **679** against this build's **643**, which no account
state explains.

**The lesson, in its sharpest form yet**: a behavioural comparison is only valid if both clients are in the same *state*, and state includes who is signed
in. The lab has no signed-in account for this build, so every account-dependent control will look dead here and be live there.
### The rows under the theme cards were 36 points high: now at 679 in both (2026-11-10)

`height + ROW_GAP + 2.0` became `+ 38.0` -- the same term attempted at round 383, which then measured 5 points too far and was reverted. This time the
target came from pressing both clients rather than from an ink probe: the reference's `Sync theme across devices` switch is at **y 679**, this build's was
at **643**, and the theme cards above them match to the pixel.

| | before | after | reference |
|---|---:|---:|---:|
| `Sync theme across devices` row | 643 | **679** | **679** |
| Settings, four slabs | 46 204 | **46 115** | -- |
| its sidebar slab | 3 068 | **2 359** | -- |

**And a scare worth recording**: the first measurement after the change read **829 926**. The row had landed exactly on 679 and the page had become
unrecognisable -- because my own sweep two rounds earlier had pressed the **Light** theme card, and this build persists the choice. The client was in light
mode and the reference in dark. Pressing `Dark` and re-measuring gave 46 115.

**So the sweep changes state, and state persists.** Every probe-driven press in this lab is a mutation: the theme, the switches, the selection. A later
measurement is only comparable if the state it was taken in is known, which is the same lesson as the signed-out switch, arriving from the other side.
### `pair.py` now refuses a theme mismatch, and the threshold is measured (2026-11-10)

The scare at round 441 -- a 20x apparent regression caused by the client being in light mode after my own sweep pressed the Light card -- is now a
refusal rather than a number. `pair.py` compares the two frames' own mean brightness after the size and breadcrumb checks:

```
$ python3 tools/parity/pair.py <name>
mean brightness: ref 43.0, ours 42.7
```

and refuses at a difference over 25, with the reason printed. The threshold is not a guess -- the two cases it has to separate are:

| pair | ref | ours | difference | verdict |
|---|---:|---:|---:|---|
| the scare (client in light) | 43.0 | 147.3 | **104.3** | REFUSED |
| the good pair (both dark) | 43.0 | 42.7 | **0.3** | accepted |

**So the instrument separates them by two orders of magnitude**, and 25 sits an order of magnitude above the noise a real layout change introduces.

**What it protects against is not a bug but a class of them**: every press this lab makes is a mutation that persists -- the theme, the switches, the
selection -- so a measurement taken after a sweep is a measurement of the sweep as much as of the page. Sizes and breadcrumbs both passed during the scare;
neither could see it. Brightness can.
### `Advanced rendering`: near-invisible in the reference, and not worth a round (2026-11-10)

The dimming fix I expected to be one line is not worth making, and the reason is where the row sits. At 2x, the reference's `Advanced rendering` label
is **barely legible** -- it is the last row of the plate, at the fold, and its disabled treatment leaves it a few points above the background. This build's
row is at y **804** after the +36 correction, which puts it below the crop band entirely, so the two are not even showing the same part of the plate.

**A colour probe could not resolve it either**: sampling the brightest pixel in the band returned **(66, 68, 74) -- the background** -- for both clients,
which is what a label a few points above its own ground looks like to a max() over a box.

**So the row is recorded as OPEN-WITH-CAUSE rather than fixed**: the reference draws it dimmed because its switch is disabled, this build draws it undimmed
or off-crop, and the difference is worth a handful of pixels on a row nobody can read. Spending a build on it would be the fourth round in a row on this
dialog's spacing, and the dialog is now at 46 115 with its rows aligned to the pixel.

**What would justify revisiting it**: a capture of the plate scrolled to its foot, which neither client currently produces -- the Settings modal does not scroll
to this row in either build at 1440x900.
### The Gallery grid is ten points low, not fifteen (2026-11-10)

The project page's Gallery is the largest single page figure in the table (172 200), and the cause recorded for it was a "15-point card-top offset". Measured
by cross-correlating the grid region (x 80..700, y 200..700) against the reference over -24..+24:

| shift | differing pixels |
|---:|---:|
| **-10** | **5 215** |
| -12 | 5 297 |
| -8 | 5 631 |

So the grid is **ten** points out, and at that shift it is 5 215 -- meaning the grid's *own* contribution at correct alignment is small and most of the
172 200 is elsewhere (the page's chrome and the sidebar's 19 804).

**Where the ten points are**: `gallery()` draws into a `ScrollArea` whose content starts at `inner`, so the offset is the **tab content's start** rather
than anything inside the grid. That is the same family as the Settings dialog's rows (36), the news card (8) and the checklist (9): this build's tab
content begins a few points below the reference's on every page that has tabs.

**Not changed**: the fix should be one constant -- the tab content's inset -- and changing it would move the Gallery, Description, Versions and Files tabs
together, so it wants one measurement that covers all four rather than a nudge to this grid.

**What the number says about the page**: correcting the grid's offset would be worth at most a few thousand of the 172 200, because at its best alignment
the grid still differs by 5 215. The remaining ~140 000 is the page's chrome, the sidebar and the artwork -- which is the same rasteriser ceiling as every
other page, not a layout error.
### There is no shared content inset: plain tabs are 2 out, scrolling tabs 12 to 16 (2026-11-10)

Measured all four project tabs the same way -- cross-correlating the content region (x 90..700, y 200..700) against the reference over -20..+20:

| tab | best shift | differing at best | kind |
|---|---:|---:|---|
| Description | **+2** | 1 226 | plain text content |
| Versions | **+2** | 804 | plain text content |
| Gallery | **-12** | 2 899 | inside a `ScrollArea` |
| Files | **-16** | 758 | inside a `ScrollArea` |

**So the "one content inset" hypothesis is wrong, and the split is informative**: the two plain tabs agree to the point at +2, and the two that live in
scroll areas are 12 and 16 points out -- an order of magnitude larger. The difference is the scroll container, not the page.

**What that means for a fix**: there is no constant to change once. The plain tabs are a 2-point matter (below the threshold worth touching at this
budget); the scrolling ones need their content's own top measured -- `gallery()` and the files tab both draw into `egui::ScrollArea::vertical()` over a
`ui.new_child`, and the first allocation inside a scroll area starts below the area's own top by whatever spacing the child ui carries.

**And what it means for the page's 172 200**: at their best shifts the four tabs differ by 1 226, 804, 2 899 and 758 -- so the content of the project page is
**within about 5 700 pixels of the reference in total**, and the remaining ~167 000 is the page chrome, the sidebar (19 804) and artwork. The largest
figure in the table is not a content defect.
### The Settings table row was a figure from before its own last change (2026-11-10)

Writing the ceiling-versus-content table, I quoted Settings' inside-plate figure as 30 639 -- which was measured **before** the +36 row correction. Re-measured
on the current capture:

| | before the +36 | now |
|---|---:|---:|
| Settings, four slabs | 46 204 | **46 115** |
| inside its plate | 30 639 | **31 311** |

**So the row correction improved the page total by 89 and made the plate itself 672 worse**, which is the same trade the news card made at round 404: moving an
element onto the reference's position can increase the count where the move changes how two renderings' edges overlap. The page total is the arbiter for a page,
and the plate figure is the honest one for the dialog; they disagree here, and both are recorded.

**The table now carries 31 311 with that trade written into its cell**, rather than the older number that flattered the change.
### The Screenshots page has no dead rectangles among its five probed targets (2026-11-10)

Pressed each rectangle `SIFT_SHOTS_PROBE` prints, with a before and after capture and a diff between them:

| target | frame change | reading |
|---|---:|---|
| `new group` | 726 549 | opens its dialog |
| `sort` | 11 189 | opens its list |
| `group` | 18 299 | opens its list |
| `GroupBy` title | 108 624 | collapses the group |
| the card's `check` | **0**, then **1 998** | **live** -- the zero was my own fault |

**The check's first zero was a state artefact, not a defect.** The press before it collapsed the group, so the card moved and the coordinate printed a moment
earlier no longer pointed at it. Re-expanding and pressing the same printed rect gave 1 998 -- the selection ring and the scrim.

**So the sweep has now been run on two pages and found exactly one real defect** (the "Jump in" strip), one false alarm caused by comparing different
account states, and one false alarm caused by my own mutation of the page. That is the instrument's real hit rate, and it is worth knowing: the runtime
sweep is good at confirming interactions work and poor at finding the ones that do not, because most zeros have a mundane cause.

**What it does prove**, and did not before: these five interactions on this page were driven, captured and measured rather than assumed -- which is what the
objective asks for. The matrix rows for them can close on this evidence.
### The sweep's evidence does not close the rows it looks like it closes (2026-11-10)

Five interactions on the Screenshots page were driven and measured last round. The obvious next step was to mark their matrix rows VERIFIED, and that step is
wrong. Reading what the rows actually assert:

| row | it asserts | what the sweep measured |
|---|---|---|
| 173 `group section header` | `chevron rotate-180 transition-all 300ms` (`section.vue:125`) -- the *timing* of the chevron's turn | that pressing the header changes the frame by 108 624 pixels |
| 287 `toolbar defaults and persistence` | `sort` defaults to `newest`, `groupBy` to `date`, **kept in storage keys and restored on the next visit** | that the two dropdowns open |
| 294 `card, box` | `aspect-video ... border-surface-5 bg-surface-2 shadow-sm`, hover `brightness-110`, selected `!border-contrast` | that the check press toggles a selection |

**So the rows are at a finer grain than the instrument.** A press that does something is a much weaker claim than a chevron that turns in 300 milliseconds, a
toolbar that remembers itself across visits, or a border that changes colour when selected. Marking them VERIFIED on this evidence would be the same error as
the "Sync theme" switch -- a real observation, attached to a claim it does not support.

**What would close each**: a capture pair 150 ms apart during the collapse (for the transition), a restart between two captures (for the persistence), and a
selected-versus-unselected crop (for the border). All three are cheap and none was done.

**So the ledger stays as it was**, and the sweep's contribution is recorded as what it is: five interactions confirmed live at a coarse grain, which is the
objective's "driven" clause satisfied for those five and not its "every state" clause.
### A selected card's border is white in the reference and near-black here (2026-11-10)

Row 294 asserts `selected !border-contrast`. Measured on the card's left edge at y 400, x 86..95:

| | x86 | x87 | x88 | x89 | x90 and right |
|---|---:|---:|---:|---:|---:|
| reference, selected | 22,24,28 | 22,24,28 | **(255,255,255)** | 40,43,47 | 40,43,47 (fill) |
| this build, selected | 235,235,235 | 235,235,235 | **(5,6,9)** | 12,14,20 | 24,25,28 (fill) |

**So the reference draws a bright 1-point border on a selected card and this build draws a near-black 2-point one.** The code intends the reference's:
`screenshots.rs:3166-3176` strokes `theme.CONTRAST` at twice `STROKE_WIDTH` when `selected`. The intent and the pixels disagree, so one of three things is true
and this round did not distinguish them:

1. the card measured is not the card `selected` refers to (the check press changed the frame by 1 998 and the scrim appeared, so *something* selected);
2. the scrim is painted over the border despite the comment saying the border is last;
3. `theme.CONTRAST` is not the near-white this build assumes for this widget.

**What settles it**: sample the same edge on a card the probe prints as selected, and print the theme's `CONTRAST` value in the same frame -- both one-liners, both
next round.

**Why it is worth a row of its own**: a selected card is a *common* state on this page, and a 2-point near-black border against a 1-point white one is visible at
1:1 without a threshold. It is the kind of difference the 99 per cent clause actually cares about, unlike the textured edges that dominate the counts.
### A probe print resolves the selected border, and raises a bigger question (2026-11-10)

Added `SIFT_SHOTS_BORDER_PROBE`, which prints the selected card's rect and the theme's `CONTRAST` in the same frame:

```
card selected rect=[[88.0 247.0] - [596.0 532.8]] contrast=#1A_20_2C_FF
left edge x86..95 at y400: (235,235,235) (235,235,235) (26,32,44) (26,32,44) (54,56,61) ...
```

**So the border is drawn exactly as the code says** -- `(26,32,44)` is `#1A202C`, the printed `CONTRAST`, at double width in x88..89. Nothing is painting over it.
The defect is not in the card.

**The theme values say something else is wrong**: `theme.rs:456` defines `DARK` with `CONTRAST: #FFFFFF`, and `theme.rs:632` defines `LIGHT` with
`CONTRAST: #1A202C`. The frame printed **the light preset's value**, and the page renders on a dark ground -- a light-theme ink on a dark-theme background.

**Two readings, and this round cannot separate them**:

1. the app is genuinely in `LIGHT` and something else paints the dark ground -- in which case every `CONTRAST` label on every page is dark-on-dark and the
   pages only look right where the text uses a different field;
2. the theme in use is `DARK` and this widget is handed a `LIGHT` instance -- in which case the bug is local to the card.

**What separates them is one print**: the same probe printing `theme.BG` and `theme.SECONDARY` beside `CONTRAST`. If `BG` is light, reading 1 is right and the
finding is much larger than a border; if `BG` is dark, reading 2 is right and the fix is wherever the two themes are mixed.

**Which is why this is the right place to stop the round**: the selected border is a 1-point difference, and the question it has opened is whether the app's
theme fields are consistent at all. That deserves the round it will take, not the last ten minutes of this one.
### The theme persisted to Light, and the selected-border finding is withdrawn (2026-11-10)

The probe print that resolved the border question read:

```
contrast=#1A_20_2C  bg=#EB_EB_EB  secondary=#48_4D_54  raised=#F8_F8_F8
```

**`bg=#EBEBEB` is light.** Sampling the live page confirmed it: **(235,235,235)** where the reference has (39,41,46). The client had been running in the
**light theme** -- the choice my sweep made two rounds earlier had persisted across every restart since.

**So the selected card's border finding is withdrawn.** `#1A202C` is the *correct* ink for a light page; the reference's (255,255,255) is the correct ink for a
dark one. The comparison was two different themes, and the 2-point near-black border I called a defect is a light-theme border on a light-theme card. That is the
fourth time this session a pair has been invalid for a state reason -- the pager, the checklist, the signed-out switch, and now the theme.

**Restored**: pressed `Dark`, and the page reads **(29,31,35)**, the dark family the reference is in.

**What this costs**: the screenshots sweeps in rounds 447-451 ran in the light theme. Their *diffs* remain valid -- a press that changes a light frame by 726 549
pixels is still a press that worked -- but no pixel figure from them can be compared with a dark reference.

**And what it argues for**: `pair.py`'s brightness guard exists because of the first version of this mistake and would have refused every one of these captures. It
is not used by the sweeps, which drive xdotool and compare directly. The guard needs to move into `compare.py` as well, so that no comparison anywhere can
silently straddle two themes.
### `compare.py` now refuses two themes, so the trap is closed everywhere (2026-11-10)

`pair.py` has refused a light/dark pair since round 442; the sweeps and every ad-hoc measurement use `compare.py`, which did not. It does now, with the same
measurement behind the threshold:

```
$ python3 tools/parity/compare.py <dark ref> <dark ours>      -> total 46115 | titlebar 1537 | rail 391 | ...
$ python3 tools/parity/compare.py <dark ref> <light ours>     -> REFUSED: mean brightness 43.0 against 191.1
```

**Both halves are load-bearing**: a guard that refuses good pairs would be worse than none, and a guard that accepts bad ones is none. The pair that measured
46 115 is the same pair every Settings figure this session comes from; the pair it refuses is the capture that produced the withdrawn border finding.

**So the class is closed at the instrument**: there are exactly two comparison tools in `tools/parity`, and both now refuse a theme mismatch before printing a
number. A future sweep that presses the Light card can no longer produce a figure that looks like a page difference -- it produces a refusal with the two
brightness values in it.

**And the probe from last round is committed with it**: `SIFT_SHOTS_BORDER_PROBE` prints the selected card's rect, `CONTRAST`, `BG`, `SECONDARY` and
`RAISED_BG`, which is what turned a border reading into a theme question in one line.
### Three pages re-measured in a known dark state (2026-11-10)

After the theme withdrawal, the three pages whose figures came from sweeps were re-captured with the client confirmed dark at the moment of capture
(`bg` sampled from the frame itself, and `compare.py`'s new guard as a second check):

| page | background | figure | against the table |
|---|---|---:|---|
| Discover | (39,41,46) | **77 465** | 77 462 -- unchanged to within three pixels |
| Skins | dark (sampled over artwork) | **132 429** | 132 429 -- identical |
| Screenshots | (22,24,28) | **61 007** | the table says 26 483, and **the pair is not comparable** |

**Discover and Skins are confirmed**: the sweep's mutation did not reach their figures, and both reproduce. **Screenshots is not**: at the same size and
theme, the two clients differ by the group's own label -- the reference says "Today 1" and this build says "Yesterday 1" -- because the screenshot file was
taken on an earlier date than the reference's copy. The 34 000-pixel difference between the two figures is that label and the layout it shifts, and it is a
**data** difference of the same kind the Files tab carries in its mtimes.

**So the table's Screenshots figure is kept with that caveat rather than replaced**, and the honest statement is that this build and the reference hold
different files with different dates -- not that the page disagrees with the reference.
### The news slide is a fifth non-rendering state, worth 20 542 pixels (2026-11-10)

Re-measuring Home after the theme restore gave **49 063** against the table's 28 521, with the sidebar at 34 272 instead of 13 730. The cause was in the
launch line, not the page: that session had been started without `SIFT_NEWS_SLIDE=1`, so the sidebar's article was cycling every ten seconds and had rotated
away from the one the reference shows.

| launch | Home | sidebar slab |
|---|---:|---:|
| without the pin | 49 063 | 34 272 |
| with `SIFT_NEWS_SLIDE=1` | **28 521** | **13 730** |

**So 20 542 pixels -- 70 per cent of the whole sidebar difference -- is which article is showing.** The table's figure is the pinned one, and the state section in
`FINISHED.md` now lists the article alongside the theme, the account, the screenshot dates and the instance folder dates as something that must match before a
number means anything.

**That is five state differences now, and only one of them (the theme) is under the instruments' control.** The other four -- account, screenshot files, folder
dates and the article -- are properties of the two clients' data, and a figure that straddles them is measuring data.
### The pin matters on Home and not on Discover or Skins (2026-11-10)

Re-measured both pages with `SIFT_NEWS_SLIDE=1` set, since yesterday's figures were taken without it:

| page | unpinned | pinned | reading |
|---|---:|---:|---|
| Discover | 77 465 | **77 465** | **identical** -- its sidebar draws the category list, not News (`panel.rs`'s `else` branch), so the article never shows |
| Skins | 132 429 | **132 428** | one pixel -- its News card is present but the article had not rotated |

**So the pin is worth 20 542 pixels on Home and nothing on Discover**, which is not obvious and is exactly the kind of thing a figure should say. The state table in
`FINISHED.md` records the article as a state that must match; this round showed that on two of three pages it already did.

**All three figures now stand in a fully-specified state**: dark theme (background sampled from the frame), the news slide pinned, the same page on both
clients, both windows 1440x900. Home **28 521**, Discover **77 465**, Skins **132 428**.
### The instance page is missing its header divider (2026-11-10)

Row 126 asserts a `border` under the instance page's header. Measured by scanning for a straight, neutral-coloured run across x 200..1100:

| | rule rows found |
|---|---|
| reference | **y 152** (225 of 225 sampled columns matched) |
| this build | **none** anywhere in y 130..320 |

and the crop at (80..900, 130..175) shows it: a faint line across the reference's header foot and nothing at all in this build's.

**So this is a missing element, not a misplaced one** -- the first of that kind found by measurement in some time. The instance page draws its icon (64x64 at x
88..151 in both, verified the same round), its name, its loader line and its tab strip, and omits the rule the reference puts under them.

**What closing it needs**: the header's own closure in `pages/instance.rs` -- one `hline` at the header's foot in the reference's rule colour (`SURFACE_5`-
family, since the reference's line is a few points above its background), then the same scan to confirm y 152 appears.

**Row 127 closes on this round's evidence**: the instance icon is 64x64 at x 88..151 in both, with a one-pixel difference at its top edge from antialiasing.
### The instance header has its rule now: y 152 in both (2026-11-10)

`header()` in `pages/instance.rs` now draws an `hline` at `avatar.bottom() + 16.0` in the theme's `SURFACE_5` -- the reference's own offset, since its avatar is
64 points tall from y 72 and its rule is at 152.

| | rule rows found by the scan | columns matched |
|---|---|---:|
| reference | y **152** | 225 of 225 |
| this build, before | none in y 130..320 | 0 |
| this build, after | y **152** | **225 of 225** |

**So the missing element is drawn and lands on the reference's row to the pixel.** Row 126 closes on this evidence, giving the matrix its 69th VERIFIED row.

**Why this one is worth more than its pixels**: a rule under a header is a structural line -- its absence makes the instance page read as one undivided
surface instead of a header above content. It cost no count to find (the scan is ten lines) and its fix is six. The rows that assert a single measurable
property are the ones this budget should have been closing all along.
### The instance header's cluster: padding and icon verified, the metadata line within two rows (2026-11-10)

Three rows of the same cluster measured in one sitting, against the same pair of captures:

| row | property | reference | this build | verdict |
|---|---|---|---|---|
| 125 | header block inset (avatar from the content edge) | 48 | **48** | **VERIFIED** |
| 127 | instance icon | 64x64 at x 88..151 | 64x64 at x 88..151 | **VERIFIED** (last round) |
| 126 | rule under the header | y 152, 225 columns | y 152, 225 columns | **VERIFIED** (last round, after the fix) |
| 130/131 | the metadata line (playtime, last played) | ink rows 96..128 | 96..126 | **within two rows**, not closed -- the reference's line runs 2 lower |
| 132 | metadata separators | 906 bright pixels in the line | 873 | **within 4 per cent**, not closed |

**So the header cluster is now five rows decided: three verified, two measured to within a couple of pixels and left open rather than claimed.** The two open
ones need their separator's own rectangle rather than a line-wide ink count, because the 33-pixel difference could be the bullet's size, the space around it, or
one glyph's rasterisation.

**What the cluster cost**: one build (the rule) and about twenty minutes of measurement for five rows. That is the rate the ledger needs -- a hundred rows in
twenty rounds -- and it is only available where the rows assert a single property against a capture that already exists.
### Escape and click-outside both close the modal: rows 194 and 195 (2026-11-10)

Two behavioural rows decided by driving the running app, with no build needed:

| action | frame change | centre pixel | corner pixel | reading |
|---|---:|---|---|---|
| before either (modal open) | -- | (52,54,60) the plate | (17,25,29) the scrim | the dialog is up |
| **Escape** | 126 644 | (22,24,28) the page | (39,41,46) the page's own ground | **closed** |
| **click outside** | 126 644 | (22,24,28) the page | (39,41,46) | **closed** |

**The two produce the identical number because they produce the identical frame** -- both remove the plate and the scrim and leave the page in the same state, which is
itself a small confirmation that neither leaves a residue behind.

**And the samples prove it rather than the count**: a count of 126 644 alone could be the dialog *opening*; the centre pixel going from the plate's (52,54,60) to
the page's (22,24,28), and the corner from the dimmed (17,25,29) to the undimmed (39,41,46), says which way it went.

**Cost**: one round, no build. That is the cheapest kind of row in the ledger and the instance header's cluster was the second-cheapest -- the expensive rows are
the ones that assert a transition's *timing*, and those are the ones worth leaving open rather than pretending.
### The library card's title and its meta line overlap (2026-11-10)

Found while trying to reach a context menu: the reference window had been left at 1248x262 (so the pair was refused by the size check), and the only capture that
came out of the round was this build's own -- which showed, at 4x, the card's title **"Fabric 26.3"** and the meta line's first field **"Fabric 26.3"** drawn on
top of each other, overlapping by about half a line's height.

**It is unmistakable at 4x and almost invisible at 1:1**, which is why it has survived every page count for the whole session: two strings of the same text
overlapping by a few points cost a few hundred pixels on a page whose figure is 28 521.

**Where it is**: the library card's two text rows in `pages/home.rs` -- the name under the artwork and the meta line beneath it. The crop is
`ctx2-our.png` at (85..300, 495..545), and the overlap is visible in the top third of it.

**Not fixed**: the round ended while locating it, and this is the kind of change that must be built and re-cropped to be worth anything -- the test is the same
crop showing two separated lines.

**What it argues for, again**: a page's total ranks this defect below a thousand invisible sub-pixel differences. The only instruments that find it are a crop at 4x
and an eye, and the only rule that would have caught it earlier is looking at every card's text at 4x once.
### The card's title and meta line are separated: 26 -> 38 (2026-11-10)

The overlap found at 4x last round was two draws eight points apart where a 15-point line needs about twenty:

```rust
painter.text(pos2(card.left() + 10.0, tile.bottom() + 18.0), ..., &instance.name, fonts::semibold(15.0), ...);
painter.text(pos2(card.left() + 10.0, tile.bottom() + 26.0), ..., loader, fonts::body(13.0), ...);   // inside the name's line box
```

The meta line is now at **38**, twenty below the name's top. Verified by the same 4x crop as the finding: the title reads in white semibold with the grey
loader line cleanly beneath it, where before the two overlapped by half their height.

| | before | after |
|---|---:|---:|
| Home, four slabs | 28 521 | **28 467** |

**The page total moved by 54 pixels** -- two tenths of one per cent. Two overlapping strings of the same text cost almost the same as two separated ones at threshold 40 -- which is precisely why this
defect survived every count for the whole session, and why the 4x crop was the only instrument that could see it.

**So this is the clearest case yet of the count and the clone disagreeing**: a visible, unarguable improvement in the product, invisible to the metric the objective
is written against. Both facts are true, and the record says both.
### Three text clusters inspected at 2-3x: none has the card's overlap (2026-11-10)

After the library card's title was found printed through its own meta line, the other text-bearing clusters on Home were inspected the same way:

| cluster | crop | result |
|---|---|---|
| the "Jump in" strip | (150..600, 100..200) at 2x | clean -- name, green badge and meta line are three separated rows |
| the sidebar checklist | (1160..1430, 100..290) at 2x | clean -- three rows, the first ticked and dimmed. Its strike is thin and dim, but **round 413 measured it at y 148 in both clients**, so the crop's impression that it was missing is superseded by a number |
| the news card | (1155..1430, 510..620) at 3x | clean -- two-line title, four-line copy, no overlap |

**So the card's overlap was specific to the card**, not a pattern across the page's text. That is worth knowing: the defect looked like it might be a family (a spacing constant shared by several widgets), and three crops say it was one draw with its own offset.

**And the checklist is the case where a crop is the weaker instrument**: at 2x the strike is invisible, at 1 pixel in a colour two steps from the text, and only the scan at y 148 could decide it. The rule is not "crops beat counts" but "each defect has one instrument that can see it" -- overlapping text needs a crop, a 1-point rule needs a scan, and a placement needs a cross-correlation.
### Settings' rows inspected closely: clean, and the copy matches word for word (2026-11-10)

Cropped the dialog's lower rows at 1.5x (`set463.png`, x 520..1180, y 620..780):

* `Sync theme across devices` in semibold, its description in two lines breaking after **"keep a"**, the switch at the row's right -- **the same break and the same
  words as the reference's own capture**, which reads "Use this theme everywhere you're signed in. Turn this off to keep a / separate theme on this device."
* No overlap, no clipping, no stray glyphs.

**So the Settings dialog, which is the most measured surface in the project, also passes the eye test** -- the fourth text cluster inspected this way and the third to
be clean. The one defect found by this method remains the library card's title, and it was a single draw with its own offset rather than a family.

**The method's tally, honestly**: four clusters inspected, one real defect, three clean. That is a worse hit rate than the row-by-row ledger (which has closed
row after row) and a far better one than the page counts for *visible* defects (which found none of them). It is worth keeping in the rotation but not worth
spending the remaining rounds on wholesale.
### The rail's rows agree in position; the rows that ask for a count do not close (2026-11-10)

Tried to close rows 207 (row pitch) and 208 (row count at rest) from the dark pair already captured, by finding the rail's rows in both frames.

**Text ink bands**: the reference yields 15 bands and this build 16, but the bands split on glyph gaps, so the counts are not row counts. Their **centres**
mostly coincide: 225, 304, 311, 353, 443, 483, 527, 566, 573, 617 and 658 appear in **both**, with the remaining bands differing by 1 to 5 points.

**Row plates**: a scan for the rows' own fill in a column inside the rail found **no bands at all** in either frame, so that approach is wrong for this rail --
either the fill is not in the colour window I chose or the column I sampled is outside the rows.

**So neither row closes.** What can be said is narrower and still useful: **eleven of the sixteen ink centres are identical to the point**, which is consistent with
the same row pitch and the same row count, and is not the same as measuring either. The rows stay OPEN rather than being marked VERIFIED on a resemblance.

**What would decide them**: the rail's row rectangles, which `SIFT_SETTINGS_PROBE` prints for the *content* but not for the rail; one eprintln in the rail's own
loop would give the pitch and the count directly, the way the Home probe did for its cards.
### The rail prints its own rows: thirteen, at a 44-point pitch (2026-11-10)

One `eprintln` in `rail_rows()` under `SIFT_RAIL_ROWS`, and the two rows that three rounds of inference could not decide are decided in one launch:

```
rail rows=13
rail tab=Appearance    rect=272.0,246.0 218.0x40.0
rail tab=Features      rect=272.0,290.0 218.0x40.0
rail tab=Behavior      rect=272.0,334.0 218.0x40.0
rail tab=Language      rect=272.0,378.0 218.0x40.0
...
```

* **row 207 (44-point pitch)**: 290 - 246 = **44**, and every consecutive row inside a category is 44 apart. **VERIFIED.**
* **row 208 (thirteen rows at rest)**: `rail rows=13`. **VERIFIED.**

**And the rects show the category gaps plainly** -- 378 to 464 is 86 (a heading plus its spacing), 552 to 638 the same, 770 to 856 the same -- which is the
structure row 205 asserts and does not yet have a number for.

**What this cost**: one eprintln and one build, after three rounds of glyph counts and plate scans that produced nothing. The lesson is the one this budget keeps
relearning at a price: **when a claim is about what the program lays out, print what the program lays out.**
### The selected rail row's brand text is dimmer than the reference's (2026-11-10)

Row 209 asserts `bg-[color-mix(in srgb, var(--color-brand) 30%, transparent)] text-brand`. Sampling the selected row (`Appearance`, y 246..286) at three points
inside it, and the category heading above it:

| | reference | this build |
|---|---|---|
| selected row, brightest ink | **(27,217,106)** | **(31,157,85)** |
| selected row, mid | (28,204,101) | (29,183,94) |
| selected row, right | (35,95,64) | (35,94,64) |
| the heading's ink | (150,162,176) | (150,162,176) |

**The heading matches exactly** -- row 205's `text-secondary` is (150,162,176) in both, which closes that part of it. **The selected row's ink does not**: the
reference reaches full brand green and this build peaks 60 points below it in the green channel.

**And the constant is right**: `theme.rs:512` gives `DARK.BRAND` as **(0x1b,0xd9,0x6a)** = (27,217,106), which is precisely the reference's value. So the label is
not being drawn in `BRAND` -- it is being drawn in something dimmer, or blended with the row's fill.

**Not fixed**: the ink's source is in the rail's row drawing, which the matrix cites at `:2704` but which my read of that line found to be a different widget (a
folder dialog). The next attempt should search the rail's own draw for where the label's colour is chosen rather than trusting the row's citation.

**What it would look like**: a selected row whose label is as green as the reference's, which is visible at 1:1 and is exactly the kind of difference the count
cannot rank -- a handful of glyphs in a 46 000-pixel page.
### The selected rail row's label is brand green now: (27,217,106) in both (2026-11-10)

`TabbedModal.vue:189` gives a selected row `text-brand`, and this build drew `theme.BUTTON_TEXT_SELECTED` -- an ink meant for text *on* a brand-filled
control, which is a different job.

| | reference | before | after |
|---|---|---|---|
| selected row, brightest brand ink | **(27,217,106)** | (31,157,85) | **(27,217,106)** |
| Settings, four slabs | -- | 46 115 | 46 135 |

**Exact match on the ink**, and the page moved by 20 pixels -- which is the third time this stretch that a visible fix has been invisible to the count.

**The constant was never wrong**: `DARK.BRAND` is (0x1b,0xd9,0x6a) = (27,217,106), the reference's own value. The defect was one line choosing an on-brand ink
for off-brand text, which is why measuring the constant alone would have cleared a row that was not correct.

**Row 209 closes**, and with it the rail's cluster: 205's heading ink, 207's pitch, 208's count and 209's selected row are all measured now.
### The Beta pill matches, and row 211 was stale (2026-11-10)

Row 211 read "not implemented -- no Beta pill beside Language", which stopped being true at round 386 when the pill was moved to the row's right end.
Measured now:

| | reference | this build |
|---|---|---|
| pill box | x 433..474, y 388..407 (42x20) | x 432..473, y 388..407 (42x20) |
| fill | (37,71,56) | (36,76,58) |
| ink | **(27,217,106)** | **(27,217,106)** |

**Same size, same row, same ink to the unit, fill within five** -- and a one-point x offset that is the pill's own left edge rounding. **Row 211 VERIFIED.**

**A stale row is its own kind of defect**: a reader trusting the ledger would conclude a whole element was missing when it had been drawn and measured for eighty
rounds. The matrix is a record of what has been proved, and this row had drifted out of date without anyone noticing -- which is why the rows that assert a single
measurable property benefit from being re-measured rather than trusted.
### The ledger maps the reference, not this build (2026-11-10)

A staleness sweep over the matrix's 240 DONE and VERIFIED rows, checking whether their citations still resolve:

```
rows with our-source citations: 0
rows citing a missing file:      0
DONE/VERIFIED total:           240
```

**Not one row cites this build.** Every citation is to the reference -- `packages/ui/...`, `apps/app-frontend/...` -- so the ledger records *what Modrinth
does* and a status, but not *where this build does it*. Two consequences follow, and both matter for the objective's "a capture behind every done" clause:

1. **A DONE row cannot be checked for staleness from the row itself.** The row 211 case -- stale for eighty rounds, claiming a pill was missing that had been
   drawn and measured -- had no way of being caught by reading the matrix. It was caught by looking at the screen.
2. **The captures are the only evidence**, and they are files in `.scratch/cap/` whose names are not recorded in the rows. A reader can see the claim and its
   status but not the pair that decided it.

**So the ledger is a map of the reference with a progress column, not a map of the clone.** That is a legitimate thing to have built, and it is not what the
objective asked for: a row per element with a capture behind every *done* needs, at minimum, the capture's name in the row.

**What would fix it**: add the capture name to the 240 rows' evidence column. That is mechanical, needs no measurement, and can be done for the rows whose
pairs still exist -- which is most of them, since the session's captures are all in one directory.
### The rail row hovers, and the reference's own hover could not be captured (2026-11-10)

Row 210 asserts `hover:bg-surface-...` on an unselected rail row. Driven: pointer away, capture, pointer over `Features` (y 290..330), capture:

| | fill at three points |
|---|---|
| this build, pointer away | (39,41,46) at all three |
| this build, pointer over | **(52,54,60)** at all three |
| this build vs the reference, un-hovered | (39,41,46) in both |

**So the hover is implemented and raises the row by 13 points on every channel**, and the un-hovered row matches the reference exactly.

**What is missing is the reference's own hover value**: two attempts to capture it produced **1248x262** images -- the reference window was never
resized to 1440x900, so the probe's co-ordinates fell outside the frame and raised rather than returning a colour. **CORRECTED after checking the file**: the
captures were valid PNGs at the wrong size, not corrupt ones, and the failure was the window size it has been three times before. Row 210 therefore closes on "hover exists with fill
(52,54,60)" and **not** on "the same fill as the reference", which is a weaker claim than the row asks for and is recorded as such.

**This is the third kind of evidence gap this session has produced** -- after the rows with no capture and the rows citing only the reference -- and it is the
most ordinary: a capture that silently failed. The check that would have caught it is the one `compare.py` already does on size and brightness, applied to every
capture at the moment it is taken rather than at the moment it is used.
### The reference window cannot be resized, and `getwindowgeometry` does not notice (2026-11-10)

Wrote `tools/parity/drive.py` to resize both clients and *prove* it before capturing, because four rounds were lost to a reference window at 1248x262. It
reports:

```
ref: wanted 1440x900, got 1440x900 -- ok
ours: wanted 1440x900, got 1440x900 -- ok
```

and then the captures are:

```
hoverfix-ref.png   (1248, 262)
hoverfix-ours.png  (1440, 900)
```

**So `xdotool windowsize` sets a geometry that `xdotool getwindowgeometry` reads back correctly and that `import -window` ignores** -- the reference
application enforces its own surface size, and the window manager's idea of it is not the thing the capture sees. The guard I built would have passed a
mismatched pair, which is worse than no guard.

**What this explains**: every failed reference capture this session -- the hover, the context menu, the Home comparison -- was this, and each time I
diagnosed it afterwards as a stray window or a corrupt file. It is neither: it is an application that will not be resized, plus a read-back that agrees with the
request instead of with the pixels.

**The check that would work**: compare the capture's *own* dimensions to the requested size, after the fact. That is exactly what `capture.py` and `pair.py`
already do on their outputs, and it is what `drive.py` should assert -- on the PNG, not on xdotool.
### `drive.py` now refuses on the PNG, and the quoted pairs all pass an audit (2026-11-10)

Two things this round, and the second is the important one.

**One: the guard asserts on the capture, not the geometry.** `drive.py` now reads its own output's dimensions and refuses rather than reporting "ok",
because the window manager's read-back agrees with the request while `import` disagrees with both:

```
ref: wanted 1440x900, got 1440x900 -- ok
  captured guard3-ref.png but it is (1248, 262) -- REFUSED
ours: wanted 1440x900, got 1440x900 -- ok
  captured guard3-ours.png (1440x900)
REFUSED: a window did not take the size, so no pair from here is comparable
```

**Two: every pair behind a quoted figure passes the audit.** All seven checked -- Home's two, Discover, Skins, Settings, the instance page and
Screenshots -- are **(1440, 900) on both sides**:

| pair | reference | ours | verdict |
|---|---|---|---|
| Home | 1440x900 | 1440x900 | OK |
| Discover | 1440x900 | 1440x900 | OK |
| Skins | 1440x900 | 1440x900 | OK |
| Settings | 1440x900 | 1440x900 | OK |
| Instance | 1440x900 | 1440x900 | OK |
| Screenshots | 1440x900 | 1440x900 | OK |

**So the table's figures are not affected by the resize problem** -- it struck only the drives attempted this round and the rounds immediately before,
whose captures were never quoted. That is worth stating plainly, because the alternative -- a table of numbers taken from 1248x262 references -- would have
invalidated the whole record.

**What remains true**: the reference window is stuck at 1248x262 now, so any *new* comparison against it is blocked until whatever sets that size is cleared.
The five-size sweep and the page figures were all taken before it happened, and the audit says so with pixels rather than with memory.
### The reference window was wedged, and a restart unblocked it (2026-11-10)

Four rounds were lost to a reference window that captured at 1248x262 while `xdotool getwindowgeometry` reported it at 1440x900. It was not a stored
preference -- its data directory has no window-size JSON -- and it was not the application refusing to resize: **a restart fixed it**. The reference now opens at
1280x800 and takes 1440x900 on request, and `drive.py` confirms it:

```
ref:  wanted 1440x900, got 1440x900 -- ok
  captured unblock-ref.png (1440x900)
ours: wanted 1440x900, got 1440x900 -- ok
  captured unblock-ours.png (1440x900)
```

**So the diagnosis in the last round was right about the symptom and wrong about the cause**: the geometry read-back does disagree with the capture, but only
while the window is wedged -- a state a restart clears. The guard is still the right instrument, because it catches the wedged case in one line rather than four
rounds; what it is not is evidence that the application cannot be resized at all.

**And the hover row closes completely**: with both windows at 1440x900, the reference's hovered rail row reads **(52,54,60)**, which is exactly what this build
measured at the same coordinates last round. Row 210's earlier "weaker claim" becomes the full one.

**The operational lesson**, worth writing down because it cost four rounds: when `xdotool windowsize` does not take, **restart the client** before concluding
anything about its geometry -- and check the capture's own size, not the window manager's.
### The library card's context menu has five of nine items, and one the reference lacks (2026-11-10)

First comparison made possible by the window unblock: right-clicking the library card on Home at 1440x900, both clients verified at that size by `drive.py`.
Both open a menu; the lists differ materially.

| # | reference | this build |
|---|---|---|
| 1 | Play | Play |
| 2 | **Add to favorites** | -- |
| 3 | **Add content** | -- |
| 4 | **View instance** | -- |
| 5 | Duplicate instance | Duplicate instance |
| 6 | Open folder | Open folder |
| 7 | **Copy path** | -- |
| 8 | **Edit icon** (with a submenu chevron) | -- |
| 9 | Delete | Delete |
| -- | -- | **Edit** (not in the reference's list) |

**So the menu carries five of the reference's nine rows, omits four, and adds one it does not have.** The reference also groups them with dividers after rows 2, 3
and 8; this build draws none in the crop.

**What each missing row needs**: *Add to favorites* and *Add content* are actions this build has no notion of; *View instance* duplicates the card's own press;
*Copy path* is a clipboard write (blocked on this display, as the screenshot copy is); *Edit icon* is a submenu this build has no art for. **Edit** is this build's
own row -- it opens the edit dialog, which the reference reaches from the card's kebab instead.

**Recorded rather than fixed**: five rows of menu work is not a five-round job, and the honest close is that the card's menu is a known, measured divergence with
its item-by-item gap written down.
### The library card is 93.6 per cent identical, and its hover state matches (2026-11-10)

Second comparison from the healthy lab: the card hovered with the pointer over it, both windows verified at 1440x900.

| region | differing pixels | share |
|---|---:|---:|
| the card alone (x 88..260, y 345..590) | **2 704 of 42 140** | **6.4%** |
| Home, four slabs, with the card hovered | 28 866 | 2.2% of the frame |

**So the card is the closest-matching element measured in this project** -- 93.6 per cent identical *including its artwork*, which is the part that has capped
every other page. The crop shows why: the green tile, the play pill at its bottom-right, the selection ring at its top-right and the two text lines under it all
land in the same places.

**With one correction to the record**: the card's title and meta line are separated now (round 461), and this pair includes that fix -- the meta line was
overlapping the title in every earlier capture of this element.

**And no kebab appears in either client on hover**, which is what the round set out to find. The reference's card menu is opened by a right press on the card
(measured last round: nine items to this build's five), not by a hover affordance.

**What this is worth**: a positive verification of the page's most-repeated element, at a figure that is an order of magnitude better than any page total. The
page reads 28 866 because the sidebar, the chrome and the type are in it; the card itself is nearly right, and that is the kind of statement the table could
never make.
### Discover's per-element table is invalid: the two clients show different results (2026-11-10)

Attempted the same element-by-element measurement on Discover, which came out much worse than Home's:

| element | identical |
|---|---:|
| title bar | 99.9% |
| nav rail | 91.5% |
| category chips | 89.3% |
| first card row | 84.2% |
| sidebar panel | 81.2% |

and the page pair reads **123 024** where earlier Discover pairs read 77 465.

**Those numbers are not a fidelity measurement.** The two clients are showing **different result sets** -- Discover's content is whatever the last query
returned, and the reference was restarted this round (so it re-fetched its defaults) while this build still holds the state from earlier in the session. A card row
of different cards differs on most of its pixels for reasons that have nothing to do with rendering.

**So the table is recorded as a failed pair, not a result**, and it is the cleanest example of the rule this session learned repeatedly: a figure is only about
the page when both clients show the same content, and on a search-driven page that means the same query and the same results -- which needs the reference and this
build to be given the same search by hand, not merely to be on the same page.

**What survives**: the title bar at 99.9% is content-independent (it is the same on every page), and it reproduces Home's figure exactly. The rest of the row is
unusable.
### Final state, measured (2026-11-10)

| | value |
|---|---|
| commits | **979** |
| tests | **269 pass, 0 fail** |
| tree | clean |
| release binary | 11 403 416 bytes, built from HEAD |
| matrix | **164 DONE, 77 VERIFIED, 422 OPEN** |
| docs | GATES 9 622 lines, FINISHED 612, GAP-MATRIX 5 731 |
| tools | 17 parity scripts |
| idle RSS, this launch | **31 584 kB** (was 22 680 on a fresh launch; `RssFile` grows as the binary is touched) |

**The last figure is worth its own note**: RSS rose from 22 680 to 31 584 across a long session without a code change, which is the page-cache behaviour already
documented -- the same executable resident in more of its pages. The application's own heap (`RssAnon`) was 5 600 kB when measured directly, and that number has
not moved.

**And the per-element table is the honest summary of the clone**: Home's seven elements run 90.9 to 99.9 per cent identical, the whole frame 97.8, and the
ordering follows texture rather than layout. Nothing in the page is wrong in the way a misplacement is wrong; everything carries glyph edges and, where there are
photographs, resampling.
### The 50 per cent idle CPU was the reference's renderer, not this build (2026-11-10)

Reported by the user: near 50 per cent CPU at idle with nothing heavy running. Measured with `top -bn1`, which reports *instantaneous* usage rather than `ps`'s
lifetime average:

| process | %CPU | note |
|---|---:|---|
| **WebKitWebProcess (the reference)** | **199.3** | 71 GB VIRT, 4h45m old -- the runaway |
| Xvfb | 25.3 | drawing for it |
| **SiftUi (this build)** | **12.7** | idle on its last page |
| ModrinthApp | 9.5 | the reference's host process |

**Killing the runaway WebKit process took idle from 77 per cent used to 46**, and Xvfb to **0 ticks in 5 seconds** -- it had been redrawing for the reference's
spinning renderer.

**The reference's renderer is the bug**: a WebKit process at 199 per cent with a wedged window, which is the same wedged state that captured at 1248x262 for four
rounds. It was left running while I measured pixels, and it was spinning the whole time.

**This build's own idle is ~21 per cent** (104 ticks in 5 s), which is also wrong but is a separate question: `main.rs:169-176` requests a repaint only while a
dialog is open or `shell.ui()` reports an animation in flight, so an idle frame should sleep. What keeps it awake is being traced -- the page it was left on
(Discover) may be reporting an animation every frame, or the software backend may be redrawing regardless.

**Operationally**: the reference app should not be left running between comparisons. It is a memory-heavy Electron/WebKit client that spins when its window is
unfocused or wedged, and this lab has had it open for days.
### Fixed: 77 per cent busy down to 17. What is left is this build's own repaint (2026-11-10)

After killing the reference's runaway WebKit process, measured from `/proc/stat` over six seconds rather than from `top`, whose own 26 per cent was inflating the
reading:

| | before | after |
|---|---:|---:|
| machine busy | ~77% | **17%** |
| Xvfb | 25% CPU | **0 ticks in 5 s** |
| this build, on Discover | 12.7% (top), 20.5% (ticks) | **20.5% of a core** |
| this build, on Home | -- | **13.7% of a core** |

**So the 50 per cent the user saw was the reference application**, left running for days with a wedged window: a WebKitWebProcess at 199 per cent and 71 GB of
virtual address space. Killing it took the machine from 77 per cent busy to 17. **The reference should not be left running between comparisons.**

**What remains is this build's own idle cost**, which is real and page-dependent: **13.7 per cent of a core on Home and 20.5 per cent on Discover**, against an
Xvfb that is now at zero. `main.rs:169-176` requests a repaint only while a dialog is open or `shell.ui()` reports an animation in flight, so whatever keeps it
awake is in what `shell.ui()` returns -- two different pages both report an animation, and the difference between them (13.7 against 20.5) is the size of the
page's own draw.

**Not fixed**: the app still does not reach zero when idle. The path is clear -- find which branch of `shell.ui()` keeps returning `animating = true` on a still
page -- and it is the same class of problem `docs/LIGHTNESS.md` was written about.
### Idle CPU is zero: the half-second tick became a wait for the next slide (2026-11-10)

The application burned **13.7 per cent of a core on Home and 20.5 on Discover** while doing nothing. The cause was one line in `panel.rs`:

```rust
ui.ctx().request_repaint_after(std::time::Duration::from_millis(500));   // 2 fps, forever
```

A fixed half-second tick redraws the whole window -- a full software rasterisation -- twice a second, for a carousel that changes every ten. The file's own comment
recorded a previous attempt to fix it, reverted because its evidence was worthless: *"two captures eleven seconds apart came back byte-identical ... it
distinguishes nothing. Re-try it with a probe that prints the slide index each time the news card draws."*

**So that is what this round did.** `SIFT_SLIDE_PROBE` prints the index on every draw, and the tick became a wait for the next cut:

```rust
if pinned.is_none() {
    let until = (NEWS_SLIDE_SECONDS - seconds % NEWS_SLIDE_SECONDS).max(0.05);
    ui.ctx().request_repaint_after(std::time::Duration::from_secs_f64(until));
}
```

| | before | after |
|---|---:|---:|
| CPU over 6 s, still page | 82-123 ticks (13.7-20.5% of a core) | **0 ticks** |
| the carousel | advanced (unmeasured) | **0, 1, 0, 1, 0 over 25 s** -- still advances |

**Both halves proven this time**: the CPU is zero and the slide index changes every ten seconds, which is exactly the evidence the earlier attempt lacked.

**And a pinned slide now costs nothing at all** -- with `SIFT_NEWS_SLIDE` set, no repaint is scheduled, so a capture session no longer keeps the window awake.
### Every page idles at zero: the caret and the toast stopped polling (2026-11-10)

After the panel's tick was replaced by a wait for the next slide, a sweep of all five pages still found three drawing on a still screen:

| page | before | after |
|---|---:|---:|
| Home | 0 ticks / 5 s | **0** |
| Discover | 0 | **0** |
| Skins | 9 (1.8% of a core) | **0** |
| Screenshots | 7 (1.4%) | **0** |
| Servers | 10 (2.0%) | **0** |

**Two polls were responsible, both in `screenshots.rs`:**

1. **The search caret** asked for a frame every 500 ms whether or not the field had the caret. It is now gated on `state.search_focused`, the same flag the
   caret is drawn from -- so an unfocused field costs nothing.
2. **The toast** asked for a frame every 100 ms while it was up -- ten full-page redraws for a plate whose only change is that it disappears. It now asks for
   `current.until - seconds`, the deadline it was polling for.

**The Skins and Servers figures closed with them**, which was not obvious before the fix: both pages draw the same shell, and the shell carries the toast and
the search field. So one page's poll was three pages' idle cost.

**The whole ladder, in order**: 77 per cent machine-busy to 17 (the reference's runaway WebKit at 199 per cent of a core), then this build's 13.7-20.5 per cent
to zero (a half-second repaint tick, then a caret and a toast). **The application now uses no measurable CPU on any of its five pages when nothing is happening.**
### Settings element by element, and what RSS is made of now (2026-11-10)

The same treatment given to Home, on the Settings pair (`sb-ref.png` against `ink-fixed.png`, both 1440x900):

| element | differing | of | identical |
|---|---:|---:|---:|
| title bar | 72 | 11 520 | **99.4%** |
| content plate | 21 612 | 528 000 | **95.9%** |
| rail rows | 4 262 | 78 480 | **94.6%** |
| rail footer | 0 | 26 160 | **100% -- a trivial match**: both sides are uniform background there, so this measures nothing |

**So the ordering is the same as Home's**: the chrome is within a point of perfect, the content is within five, and the rail -- which carries the most type per
square point -- is the worst at 94.6. Nothing here is misaligned; it is all glyph edges.

**And RSS, on a long-lived instance** (five pages visited, Skins' 3D models built, the screenshots list loaded):

| | bytes |
|---|---:|
| VmRSS | 34 640 kB |
| RssAnon (the application's own heap) | **16 008 kB** |
| RssFile (the binary and its assets, page cache) | 13 568 kB |
| RssShmem | 5 064 kB |

**The anonymous heap has tripled since it was measured at 5 600 kB**, which is what visiting every page costs: the skin atlases, the 3D model, the screenshot
thumbnails. **It is under the 20 MB bar on its own** (16 008), and the total is not (34 640) because two thirds of it is file-backed pages that the kernel owns and
can evict.

**What the gate would need**: either a smaller working set -- evict the page caches the tabs no longer show -- or the acknowledgement, already in `FINISHED.md`,
that `ps`'s RSS counts 19 MB of things this process does not allocate.
### Signed in as msedgemc, and the sidebar checklist does not notice (2026-11-11)

Reported by the user: signed in as **msedgemc**. Captured this build at 1440x900 on the Servers page, signed in:

| element | state in the capture |
|---|---|
| `Create first instance` | **ticked and struck** -- correct, an instance exists |
| `Sign in to Minecraft` | **empty circle** |
| `Sign in to Modrinth` | **empty circle** |
| rail, bottom | a logout affordance is drawn |

**So the two sign-in rows stay unticked while the account is signed in.** Either the checklist reads a different flag from the one the sign-in sets, or the client is
not actually holding a session and the logout affordance is static -- the two readings are indistinguishable from the pixels, and I did **not** restart the client to
settle it, because a session held in memory would be lost by the restart the probe needed.

**The account is not in `app-settings.json`** either: `/tmp/matchdata/SiftLauncher/app-settings.json` holds chips, dev_mode, selects, sliders, switches, texts and
theme, and no account field. So whatever holds the session is runtime state or a separate store -- which is why the checklist may be reading a durable flag that
nothing wrote.

**What would decide it, without a restart**: the reference's own checklist behaves the same way when signed in, and the app's account dialog (if the rail's avatar
opens one) would print the username. A click on the avatar opened nothing in this capture, which is itself a finding worth chasing separately.

**Recorded, not fixed.** The matrix has a row for the checklist's four states; this adds the case that matters most -- signed in, and the row still asks you to sign
in.
### Push needs a credential in this environment, not a browser session (2026-11-11)

The user is signed in to GitHub; the push still fails, because a browser session is not a credential any process here can read:

```
$ git push origin master
fatal: could not read Username for 'https://github.com': No such device or address
```

What was checked, and all of it absent:

| path | state |
|---|---|
| SSH keys (`~/.ssh/*.pub`) | none |
| `GITHUB_TOKEN` / `GH_TOKEN` in the environment | unset |
| `~/.git-credentials` | absent |
| `git config credential.helper` | unset |
| `gh auth status` | **not logged into any host** (the binary is present) |

**So the repository is complete and unpushed -- 937 commits on `master` against `origin` https://github.com/msedge-projects/SiftLauncher.git --** and the one
thing missing is a credential: either `gh auth login` (device flow) in this environment, or a token exported for the session.

**The work is not blocked by it.** The commits are the record; the push is a transport detail that will take one command once a credential exists.
### Pushed: 938 commits on GitHub (2026-11-11)

The push had been failing all session with `could not read Username for 'https://github.com'`. The cause was not a missing credential but an empty `$HOME` in
the shell: `gh` was looking for its config in a home directory that did not resolve, so `gh auth status` reported no accounts while two were logged in at
`/root/.config/gh/hosts.yml`.

**Two accounts are logged in on this machine**, which is what the user asked:

| account | status |
|---|---|
| **MSedgeMC** | logged in, was active |
| **msedge-projects** | logged in, now active |

The first push attempt with `MSedgeMC` active was refused: `Permission to msedge-projects/SiftLauncher.git denied to MSedgeMC` (403). Switching to
`msedge-projects` -- the account that owns the repository -- and pushing with `gh` as the credential helper succeeded:

```
$ GH_CONFIG_DIR=/root/.config/gh git -c credential.helper='!gh auth git-credential' push origin master
   8140954..79e9b61  master -> master
```

**So the record is off this machine**: every commit from the session's start to `79e9b61` is on GitHub, and the 'all commits are local' caveat that has ended
every report since the beginning is now false and should not be repeated.
### The repo is private now; a private repo under MSedgeMC could not be pushed to (2026-11-11)

The user asked for the launcher to live in a **private** repository under **MSedgeMC**, not under the project account.

**What was achieved**: `msedge-projects/SiftLauncher` was **PUBLIC** and is now **PRIVATE**, holding all 988 commits. That is the private repo; it is just
owned by the project account rather than the user's.

**What could not be achieved, and the measurements**:

| attempt | result |
|---|---|
| create `MSedgeMC/SiftLauncher` (private) | **worked** |
| push master to it | `remote rejected: failed` |
| push **one** commit, after seeding a README | `remote rejected` -- so it is not size |
| permissions on that repo | `admin: true, push: true`, private, size 0 |
| push the same pack to the project account | **succeeded** |
| `gh repo transfer` | not a command in this gh version |
| `POST /repos/{o}/{r}/transfer` with `new_owner` | returned the repo **unchanged** |

**So every write to a freshly created repository under the personal account is rejected server-side while the same content pushes cleanly to the project
account** -- a property of the account or its state, not of this repository. Two loose ends: an empty `MSedgeMC/SiftLauncher-seed` remains (the token lacks the
`delete_repo` scope to remove it), and the transfer needs to be done from the web UI if the personal account is where it must live.
### The repository is public at msedge-projects/SiftLauncher, 990 commits (2026-11-11)

At the user's direction the project account's repository is **PUBLIC** (it had been made private during the attempt to move it to the personal account):

**https://github.com/msedge-projects/SiftLauncher**

| | value |
|---|---|
| visibility | **PUBLIC** |
| commits, local | **990** |
| commits, remote (fetched) | **990** |
| HEAD, both | `b2f4e21` |
| unpushed | 0 |

**Reconciled rather than assumed**: the API's paging header suggested 1 909 commits, which disagreed with the local count; a `git fetch` then `rev-list --count`
gives **990 on both sides at the same SHA**, and the API's default-branch head is that same commit. The paging figure was an artefact of counting with `per_page=1`
across a repository whose refs include more than the default branch.

**So the record is public, current, and identical on both sides** -- the state the objective asks for ('every slice committed'), and the first time in this session
that has been true of the remote.
### A smaller texture cache makes RSS worse: measured, reverted (2026-11-11)

The image cache is already bounded -- **8 MiB or 96 entries**, whichever comes first -- and a page walk showed Discover accounting for one full budget
(`+6 412 kB` of anonymous heap) and Screenshots for thumbnails (`+2 116 kB`). The obvious move was to halve it, so it was halved and the same walk repeated:

| after visiting | 8 MiB / 96 entries | **4 MiB / 48 entries** |
|---|---:|---:|
| Home, fresh | 22 348 / 5 552 | 22 596 / 5 556 |
| Discover | 30 404 / 11 964 | 31 668 / **13 040** |
| Skins | 30 672 / 12 100 | 31 848 / 13 156 |
| Screenshots | 32 792 / 14 216 | 33 956 / 15 264 |
| Servers | 33 104 / **14 516** | 34 256 / **15 564** |

(VmRSS / RssAnon, kB.)

**The smaller cache uses a megabyte more**, at every page, because evicting sooner means decoding again: a re-decoded texture is allocated, uploaded, freed, and the
allocator keeps the arena. Less cache, more churn, more resident memory -- the opposite of the intent.

**So the change is reverted**, and the finding is worth keeping: the cache was already tuned at the point where its budget is smaller than the churn it would save.
The RSS gate's remaining gap is not in this cache.

**Idle CPU stayed at 0 ticks** across the same walk, so neither setting regressed the lightness work.
### The card menu's `View instance` is drawn and works (2026-11-11)

The card's context menu had four of the reference's nine rows. `View instance` is the first added, because it needs no new state: the reference's own
`InstanceItem.vue` opens the instance page from the card's press, and this build already has that verb.

| | before | after |
|---|---|---|
| menu rows | Play, Edit, Duplicate instance, Open folder, Delete (5) | Play, **View instance**, Edit, Duplicate instance, Open folder, Delete (6) |
| glyph | -- | `Glyph::Eye`, the reference's `EyeIcon` |

**Verified by driving it**: pressing the new row changed **59 667 pixels** of content and landed on the instance page -- 64x64 icon, `Fabric 26.3`, and the
metadata line `Fabric 26.3 - Never played`. So the row is not only drawn, it is wired to the verb it names.

**Still missing from the reference's nine**: `Add to favorites` and `Add content` (no state for either here), `Copy path` (a clipboard write; the display has
blocked clipboard writes before), and `Edit icon` (a submenu with no art). **And this build draws an `Edit` row the reference does not have** -- it opens the
rename dialog, which the reference reaches from the card's kebab instead.
### The tests a new menu row shifts, and the fact I pushed before reading them (2026-11-11)

Adding `View instance` as the menu's second row broke **two** tests, and the commit went up before either was read -- `267 passed; 2 failed` is what the
push carried. That is a process failure worth naming: the suite was not consulted before the push, only after.

The failures were all index or list assertions the new row legitimately shifts:

| test | what it asserted | now |
|---|---|---|
| `the_menu_labels_are_the_reference_strings` | the exact five labels | six, with `View instance` second |
| `..._type_ahead_...` | `d` -> 2, `de` -> 4, `e` -> 1 | `d` -> **3**, `de` -> **5**, `e` -> **2** |

**All three type-ahead indices moved by exactly the one row inserted above them** -- which is the assertion doing its job: it pins positions, so a menu that
grows cannot silently change where a keystroke lands.

**Fixed, and the suite is green at 269 again**, with the corrected indices pushed as a follow-up commit.
### `Copy path` is in the card menu and copies the right folder (2026-11-11)

The second of the four missing menu rows, and the one that works without a file manager. Pressed on the card's menu, it puts the instance's folder on the
clipboard and says so:

> **Copied /tmp/matchdata/SiftLauncher/instances/Fabric 26.3.**

| | before | after |
|---|---|---|
| menu rows | Play, View instance, Edit, Duplicate, Open folder, Delete (6) | Play, View instance, Duplicate, Open folder, **Copy path**, Edit, Delete (7) |
| order | not the reference's | reference order, with this build's `Edit` where the reference has `Edit icon` |
| glyph | -- | `Glyph::Link` |

**Wired through a new `Action::CopyPath`**, handled beside `Action::OpenFolder` -- the same `instance_dir` lookup, written to the shell's clipboard and
announced in the toast, which is the pattern the folder action already used when no file manager is present.

**The tests caught the insertion this time before the push**: the menu-labels test and all three type-ahead indices moved again (`d` -> 2, `de` -> 6,
`e` -> 5), and the `names()` test required the new variant to name its instance -- which is the test doing exactly what its comment says it exists for.

**Still missing**: `Add to favorites` and `Add content` (no state for either in this build), `Edit icon` (a submenu with no art).
### Favourites begin at the core: the reference stores a group, this build a flag (2026-11-11)

The two menu rows still missing need state, so the reference's own source was read rather than guessed at:

```ts
const isFavorite = item.instance.group_ids.includes(FAVORITES_GROUP_ID)   // use-library.ts:1275
export const FAVORITES_GROUP_ID = 'group:favorites'                       // helpers/instance-groups.ts:3
```

**Favourites are a group membership**, toggled by rewriting `group_ids`, and the row's label flips between `Add to favorites` and `Remove from
favorites` (`use-library.ts:1298-1314`). **Driving that source also corrected the menu the captures gave:** the reference's list begins with `Stop`
*when the instance is playing* and `Play` when it is not -- the two are mutually exclusive -- and `Add content` is drawn only while `canAddContent`
(`!quarantined && !link && !installing`). This build always draws `Play`, which a still capture could not show.

**This slice, in the core**: `Instance::favorite()` and `Instance::set_favorite(bool)`, keeping the flag in `instance.cfg` as `favorite=true`.
That is not the reference's storage -- it has no group index here to join -- but it is per-instance, it survives a restart, and it produces the same
visible behaviour.

**Core suite green: 230 + 8 tests.** The menu row itself is the next slice, because its label depends on the instance's state and the menu's
`label()` currently returns a `&'static str` with no state to read.
### `Add to favorites` is drawn, toggles, and flips its own label (2026-11-11)

The row the reference builds out of a group membership, implemented as a per-instance flag and verified end to end:

| | before | after |
|---|---|---|
| menu rows | Play, View instance, Duplicate, Open folder, Copy path, Edit, Delete (7) | Play, **Add to favorites**, View instance, Duplicate, Open folder, Copy path, Edit, Delete (8) |
| the row | -- | star glyph, second position, which is the reference's |

**Driven, twice**:

1. Pressing `Add to favorites` produced the toast **"Added Fabric 26.3 to favorites."** -- so the flag reached `instance.cfg`.
2. Reopening the menu showed **`Remove from favorites`** in the same position -- so the page's own copy flipped, and the label reads it.

**How it is wired**: `State::favorites` is read from each `instance.cfg` when the instance set changes (the same refresh that reads `saves/`), the row's
press flips that copy and returns `Action::SetFavorite(id, on)`, and the shell writes the flag through `sift_core`'s `Instance::set_favorite`. The page owns
what is drawn; the shell owns what is on disk.

**`label_with(favorite)` is the one state-dependent label in the menu** -- every other row keeps `label()`, and the test that pins the labels still pins
them, with the favourite row asserting its default spelling.
### `drive.py` reported five verified captures it never wrote (2026-11-11)

A five-size sweep was run with the guard written earlier. It printed, for every size:

```
1280x720   both verified
1366x768   both verified
1440x900   both verified
1600x900   both verified
1920x1080  both verified
```

**and no file exists for any of them.** `ls -t` in both `.scratch/cap` (host) and `/scratch/.scratch/cap` (container) shows the newest entries are the
favourite-row captures from the previous round; a `find` for `sweep1280x720*` returns nothing on either side.

So the guard that was written to stop mismatched pairs is **reporting success for captures it did not make**, which is worse than the fault it was
written to catch: the earlier `xdotool getwindowgeometry` lied about a size, and now this lies about a file.

**The likely fault**, to be checked rather than assumed: `drive.py` runs its PIL probe as a *host* process on a path with `/scratch/` stripped, and reads
`stdout.split()` without checking the exit code -- so a probe that fails to open the file may still leave `probe` non-empty from something else, and the
code that turns that into `(w, h)` is not the check it looks like. The fix is to make the probe **exit non-zero** when it cannot open the file and to treat
a non-zero exit as REFUSED, rather than parsing whatever came back.

**No five-size figure is quoted from this run.** The table in `FINISHED.md` stands as the last measured one, and this round adds no number to it.
### Five sizes, re-measured with a guard that can fail (2026-11-11)

`drive.py` was rewritten so that every step checks its own exit status and the pair is REFUSED unless **the PNG's own dimensions** match the size
requested -- and `--selftest` proves the check fires:

```
$ python3 tools/parity/drive.py --selftest
selftest ok: a missing file reads as None, so an unwritten capture cannot pass
```

With both clients on Home at every size:

| size | differing | title bar | rail | content | sidebar |
|---|---:|---:|---:|---:|---:|
| 1280x720 | **25 938** | 2 832 | 397 | 15 486 | 7 223 |
| 1366x768 | **34 174** | 2 707 | 397 | 23 788 | 7 282 |
| 1440x900 | **29 274** | 1 693 | 397 | 13 454 | 13 730 |
| 1600x900 | **43 440** | 2 973 | 397 | 26 340 | 13 730 |
| 1920x1080 | **55 768** | 3 215 | 397 | 27 695 | 24 461 |

**Every pair passed `PAIR OK` on its own pixels**, which is the difference between this table and the one attempted two rounds ago: that run reported five
verified sizes and wrote nothing at all.

**The figures are 200-800 higher than the last quoted table** (25 099 / 33 456 / 28 467 / 42 635 / 55 027). The difference is state, not regression: the
sidebar's news article rotates every ten seconds and the two clients are not synchronised to it, so the sidebar column carries whichever plate each was
showing. That is the same 20 542-pixel article effect documented in `FINISHED.md`, and the reason `SIFT_NEWS_SLIDE` exists.

**The rail is 397 at every size**, which is the one column that does not depend on content -- and the best evidence in the table that the frame's fixed parts
are close.
### Pinning the news plate removes the ambiguity, and the figures come back (2026-11-11)

The carousel has **two** plates, so the mismatch was binary. Each plate was pinned in turn and compared against the reference's sidebar at 1440x900:

| our pinned plate | differing in the sidebar (x 1150..1430, y 300..640) |
|---|---:|
| 0 | 44 482 of 95 200 |
| **1** | **8 074 of 95 200** |

**So the reference is showing plate 1**, and with this build pinned to the same one every size re-measures as:

| size | matched plate | mismatched (last round) | difference |
|---|---:|---:|---:|
| 1280x720 | **25 109** | 25 938 | +829 |
| 1366x768 | **33 346** | 34 174 | +828 |
| 1440x900 | **28 444** | 29 274 | +830 |
| 1600x900 | **42 611** | 43 440 | +829 |
| 1920x1080 | **54 939** | 55 768 | +829 |

**The mismatch is a constant 829 pixels at every size**, which is what a fixed-size sidebar plate should cost -- and it confirms the earlier table
(25 099 / 33 456 / 28 467 / 42 635 / 55 027) was taken with matched plates: the new figures sit within 100 of it at every size.

**So the ambiguity is now measured rather than explained**: 829 pixels, binary, and removable in one environment variable. Every future figure should state
which plate it was taken on, and `SIFT_NEWS_SLIDE=1` is the value that matches the reference as it currently stands.
### `drive.py` names the news plate on every run, and proves it can tell them apart (2026-11-11)

The carousel rotates every ten seconds in the reference, so any figure is only comparable while both clients show the same plate. Rather than
remembering which plate a session was on, the guard now reports it:

```
$ python3 tools/parity/drive.py --size 1440x900 --tag platecheck3
ref: .scratch/cap/platecheck3-ref.png (1440x900)
ours: .scratch/cap/platecheck3-ours.png (1440x900)
reference news plate: 1
PAIR OK
```

**And the check is tested, not assumed.** `--selftest` now proves both properties:

```
selftest ok: a missing file reads as None, so an unwritten capture cannot pass
selftest ok: each plate signature reads back as its own index
```

The second is the one that matters: a check that answers *something* is worthless -- it must answer **0 for a plate-0 capture and 1 for a plate-1
capture**, which is exactly what it does when handed the two full captures. The first attempt at this selftest fed a crop where a full frame was
expected and correctly reported `None`, which is how the mistake in the test itself was caught.

**So the ambiguity that has been explained in prose for two rounds is now a line of output**, and a table taken on the wrong plate can no longer be
recorded without its run saying so.
### The card's meta line sat two points high: measured at the band, fixed (2026-11-11)

Rows 130-132 had been left OPEN as "within two rows", which is a guess dressed as a measurement. Reading the ink bands of the card's two text lines, in
both clients at 1440x900:

| band | reference | before | after |
|---|---|---|---|
| title | 518..529 | **518..528** | 518..528 |
| meta | **541..551** | 539..547 | **541..549** |

**The title's top was already identical** at 518. **The meta line was two points high**, so its offset went from 38 to 40 -- the same two points.

| | before | after |
|---|---:|---:|
| Home, 1440x900 | 28 444 | **28 402** |

**And what remains is not position**: the reference's meta band is 11 rows tall and this build's is 9, with both starting at 541. That is glyph weight or size,
not placement, and it is now the only difference in that line.

**So "within two rows" became a number, and the number was a defect.** The two rows of slack were the offset, and closing them moved the page by 42 pixels
-- smaller than the estimate written before the measurement, which is the usual order of things here.
### The card's type was a point small: 16/14 now, and both bands match (2026-11-11)

The remaining difference in the card's text was ink height -- 11 rows against the reference's 12 for the title, 9 against 11 for the meta. The reference's own
classes say why (`InstanceItem.vue:249, 260`): `text-base font-semibold` for the title and `text-sm text-secondary` for the meta, which are **16 and 14
points**. This build drew 15 and 13.

| | before | after | reference |
|---|---|---|---|
| title band | 518..528 (11 rows) | **518..529 (12)** | 518..529 (12) |
| meta band | 539..547 (9) | **541..551 (11)** | 541..551 (11) |

**Both bands are now identical to the reference**, and the meta's offset needed one more point (40 -> 41) once its line box grew.

**And the page got worse**:

| | total, 1440x900 |
|---|---:|
| 15/13 points, meta at 38 | 28 444 |
| 15/13 points, meta at 40 | 28 402 |
| **16/14 points, meta at 41** | **28 454** |

**Matching the reference's type raised the count by 52 pixels.** Larger glyphs have more edges, and every edge is a place this rasteriser and the browser
disagree. The element is now exactly right and the number says it got worse -- the fourth clear instance of counts not being monotone in fidelity, and the
cleanest: two font sizes chosen to match the reference, verified by ink bands, and a page figure that prefers the wrong ones.
### The checklist's type matches, though its class name says otherwise (2026-11-11)

After the card's title was found a point small, the same check was run on the sidebar. The reference's checklist is `text-base` with `font-medium leading-5`
rows (`features/onboarding/checklist.vue:92, 122`), which reads as 16 points -- and this build draws those rows at **13**.

**The pixels say they match anyway:**

| element | reference band | this build |
|---|---|---|
| checklist row 1 | 141..152 (12 rows) | 142..153 (12 rows) |
| checklist row 2 | 189..200 + 202..203 | 190..201 + 203..204 |
| sidebar heading | 90..95 + 97..98 | 90..98 |

**Twelve rows of ink on both sides**, with this build's a single point lower. So the two font stacks reach the same ink height from different nominal sizes --
the reference's 16px web font and this build's 13-point face have nearly the same cap height -- and changing 13 to 16 on the strength of the class name would
have made the element **three points too large**.

**Which is the limit of last round's method, stated plainly**: reading the reference's classes fixed the card because the card's ink was also wrong by a row; it
would have broken the checklist, whose ink was already right. **The source gives the intent, the band gives the result, and only the band can decide** -- so the
order is: read the class, measure the band, and change nothing when the band already agrees.

**The one-point offset in the checklist** is left as it is: measured, recorded, smaller than the tolerance of the comparison being used to judge it.
### The completed checklist row is 54 points too bright, and its neighbour's ring is faceted (2026-11-11)

A single-pass ink profile over both captures found the sidebar disagreeing at 22 row-bands, the worst at **y 142..155 (delta 87)** -- the checklist's first row.
Cropped at 3x, two differences are visible at once and both measure:

| | reference | this build |
|---|---|---|
| brightest pixel in the completed row (x 1240..1300, y 140..160) | **(96,107,113)** | **(150,162,176)** = `SECONDARY` |
| pixels above sum 260 in the row | 436 | 667 |
| the empty row's ring | a thin, smooth circle | a faceted, scalloped circle |

**So the completed row carries an ink 54 points too bright**, even though the label and the strike are both drawn in `SURFACE_5` (66,68,74). Something in that row
-- the check glyph's plate, or the strike's own edge -- is reaching `SECONDARY`, and the *brightest* pixel is the right instrument to find it because a mean over
the row would hide one bright element among dim ones.

**And the ring's facets are the rasteriser**, not a parameter: the code draws `circle_stroke(radius 7.0, 1.5 points, SECONDARY)`, which is the same
instruction the reference's CSS border-shape issues. The browser antialiases a fractional stroke; this renderer tessellates a polygon and feathers it, and at
radius 7 the facets are visible at 3x.

**Both recorded, neither fixed.** The first needs the bright element located -- one more probe, reading *where* the (150,162,176) pixel is; the second is the
renderer class already documented 19 times in the matrix as RENDERER-RISK.
### Corrected: the completed row is brighter by *coverage*, not by a stray pixel (2026-11-11)

Last round's claim -- "the completed row carries an ink 54 points too bright" -- came from the single brightest pixel in the band, which is the wrong
statistic for a line of type. Measuring the band's mean instead (`x 1200..1370, y 141..154`):

| | mean sum | pixels above sum 300 |
|---|---:|---:|
| reference | **212.9** | 423 |
| this build | **244.4** | 708 |

**So the row is brighter, but not because anything is the wrong colour.** The label and the strike are both drawn in `SURFACE_5`, whose sum is 208 --
within four of the reference's mean of 212.9. This build's 244.4 is *above its own ink colour*, which means its glyphs cover more of the band: heavier
antialiasing, more lit pixels per stroke, 67 per cent more of them above the threshold.

**That is a rasteriser property, not a style bug**, and it belongs beside the faceted ring rather than in a list of things to change. The earlier single-pixel
reading would have sent me looking for a `SECONDARY` fill that does not exist in that row -- the exact error this project has made before, and the reason the
mean is now the recorded figure.

**What it costs the page**: text that covers more area also differs more, which is the same mechanism that made matching the reference's font sizes *raise* the
count. Both findings point the same way: this renderer's glyphs are heavier than the browser's, and no colour or size change fixes coverage.
### The profile's two classes, separated: one real geometry defect, the rest is coverage (2026-11-11)

Pairing ink bands by index in both captures was meant to tell fixable geometry from the renderer's coverage. It did -- but only for the content column, because
the sidebar's band counts differ (20 against 21) and every pair after the first divergence compares different elements: the -49, -80 and -203 deltas are
misalignment, not layout.

**The sidebar's extra band is the coverage finding again**: our completed checklist row's ink crosses the 110 threshold and the reference's does not, so this
build reports a band the reference has no matching band for. That is the same 244.4-against-212.9 mean measured two rounds ago, and it is not a structural
difference.

**The content column has 8 bands on both sides, so its pairs are real**, and two of them disagree:

| band | reference | this build | reading |
|---|---|---|---|
| 2 | y161..171 (11 rows) | y158..166 (**9 rows**) | the `Jump in` strip's meta line: a point small, and 3 rows high as a consequence |
| 4 | y260..273 (14 rows) | y262..270 (9 rows) | the strip's title: also short by five rows |

**Cropped and read at 2x**, the strip's meta line is visibly smaller than the reference's -- the same class of defect the card's title had, in a different element.

**So the method works when the band counts match**, and the fix is to pair by *position* rather than by index when they do not. Recorded here rather than rebuilt
this round: the three bands that matter are named, with their numbers.
### The `Jump in` strip's meta line: 14 points and 44 down, band identical (2026-11-11)

The band profile had named it: reference y161..171 (11 rows), this build y158..166 (9 rows). The reference's own meta runs are `text-sm`, which is 14 points,
and this drew 13 at `strip.top() + 40`.

| | before | after | reference |
|---|---|---|---|
| strip meta band | 158..166 (9 rows) | **161..171 (11)** | 161..171 (11) |
| Home, 1440x900 | 28 454 | **28 388** | -- |

**And here the count went the right way**: -66 pixels, where matching the card's type had raised it by 52. So the coverage penalty is real but not universal -- when
the element also *moves*, the layout gain outweighs the extra ink, and the two effects can be told apart only by measuring both.

**Three elements in a row have now been a point small** (the card's title, its meta, and this). The pattern is no longer a coincidence to note: this build's
`body(13.0)` is the wrong default wherever the reference says `text-sm`, and the remaining 13-point text on the page is where the next defects are.
### The search placeholder is 5 rows short, and the file that draws it is not the one named `search` (2026-11-11)

The band profile leaves exactly one divergent band on the content column: **ref y260..273 (14 rows), ours y262..270 (9 rows)**. Cropped, it is the library
toolbar's search field, and its placeholder is visibly larger in the reference.

`Input.vue:26,55` gives the size -- `size == 'small' ? 'text-sm' : 'text-base'`, so **16 points** -- and `widgets/search.rs:256` draws a placeholder at 13. That
was changed to 16, rebuilt, and re-measured: **the band did not move and the page total did not move** (28 388 both times), so that call site is not the one
drawing this field. The change is reverted rather than left in unverified.

**And the first attempt at verifying it captured a dead window**: the two `SiftUi` processes were `defunct` zombies from 1 November, so `import` returned the
last content of a destroyed window and the comparison was between two identical stale images. That is the third capture this session taken from a window whose
process was gone -- after the wedged reference and the five sizes that were never written -- and it is the same class of failure each time: **nothing checks that
the thing being photographed is alive.**

**So the band stays OPEN** with its numbers, and two things are needed: the real draw site for the toolbar placeholder, and a liveness check in `drive.py` before
it captures anything.
### `drive.py` refuses a dead client, and the refusal is proven (2026-11-11)

Three captures this session came from windows whose processes were gone -- a wedged reference, five sizes never written, and a zombie from 1 November whose
dead window still returned its last frame. `xdotool` finds the window either way; only the process table tells them apart.

So the guard now requires, before it resizes anything:

```
$ python3 tools/parity/drive.py --size 1440x900 --tag dead
ours: no live SiftUi process -- REFUSED, a zombie still owns a window and still draws its last frame   [exit 4]
```

and with the client restarted it passes as before:

```
1440x900 -> ref ... (1440x900) | ours ... (1440x900) | reference news plate: 1 | PAIR OK
1280x720 -> ref ... (1280x720) | ours ... (1280x720) | PAIR OK
```

**The check is `ps` filtered on a non-zombie state**, plus `xdotool search --onlyvisible` for a mapped window -- because a process can be alive with its window
unmapped, which is the wedged case. Both are selftested:

```
selftest ok: a missing file reads as None, so an unwritten capture cannot pass
selftest ok: each plate signature reads back as its own index
selftest ok: liveness says alive for bash and dead for a name nothing runs
```

**So the guard now answers three questions before a pair exists**: is the process alive, is its window mapped, and is the file it wrote the size it claims.
Each answer can be no, and each was made to say no once before being trusted.
### The search placeholder: 15 -> 16 points, and why 18 was rejected (2026-11-11)

The band profile's last divergent band was the library toolbar's `Search` placeholder: reference **y260..273 (14 rows)**, this build y262..270 (9 rows) at
15 points. The real draw site is `pages/home.rs:1419` (not `widgets/search.rs`, which was tried first and moved nothing).

| size | band | Home, 1440x900 |
|---|---|---:|
| 15 points | 262..270 (9 rows) | -- |
| **16 points** | 262..272 (11 rows) | **28 368** |
| 18 points | 261..272 (12 rows) | 28 411 |

**16 is kept**: it is the field's own size in the reference's source (`Input.vue:26,55` -- default inputs are `text-base`), it gives the lowest count of the
three, and 18 buys one row of geometry for 43 pixels.

**And 14 rows is not reachable with this font at any sane size.** The checklist taught the same lesson from the other direction: this build's 13-point face
matches the reference's 16px ink, so their type is not the same shape as ours. Chasing the band by nominal size would end in a placeholder three points too
wide for its field -- the element would match a profile and stop matching its own box.

**So the band stays 3 rows short**, recorded with the three sizes measured rather than a fourth guess.
### The sidebar paired by position; the checklist's 14 changed nothing (2026-11-11)

Index pairing broke on the sidebar because its band counts differ (24 reference, 25 here). Pairing **by position within six rows** instead gives a usable
account, and the differences are legible:

| band | reference | this build | reading |
|---|---|---|---|
| 189..202 | 14 rows | 12 rows | checklist row, clear: a point small |
| 237..250 | 14 rows | 12 rows | the same |
| 566..634 | 11, 11, 10, 15 rows | 9, 9, 9, 19 | the news card's title and copy: short, with one line taller |
| 671..706 | 7, 16 rows | 9, 20 | the news copy: taller |
| 720..732 | 13 rows | 7 rows | short by six |

**And the checklist's clear rows were changed from 13 to 14 points, rebuilt, and re-measured: nothing moved.** The source has `fonts::body(14.0)`, the
binary is from 06:35, a live process is running, and both bands are still 12 rows with the page total unchanged at 28 368.

**So one of two things is true**, and they need different rounds: either this font rasterises 13 and 14 identically (in which case the checklist rows are a
coverage difference that no size will fix), or those rows are not drawn by the call that was edited (in which case the size is still wrong somewhere else).
**The test that separates them is a deliberately oversized value** -- 16 or 18 -- and seeing whether the band moves at all.

**The 14 is kept**: it is closer to the reference's intent and demonstrably harmless, but it is recorded as **unverified**, not as a fix.
### Three sizes, no movement: the checklist's draw site is still not found (2026-03-11)

The checklist's clear rows measure 12 rows of ink where the reference has 14. Three sizes were tried in the call at `panel.rs:533`, each rebuilt and
re-measured against a guard-certified pair:

| size | band | page total |
|---|---|---:|
| 13 | 12 rows | 28 368 |
| 14 | 12 rows | 28 368 |
| **18** | **12 rows** | **28 368** |

**An eighteen-point label cannot render at the same height as a thirteen-point one.** So that call is not drawing what the capture shows -- it belongs to the
*project* page's sidebar, whose rows are laid out at `panel.rs:486` (`card.top() + PROJECT_STEPS_TOP + index * PROJECT_ROW_PITCH`), and Home's checklist is
drawn somewhere else.

**What this cost, and what it bought**: three builds and three captures to prove a negative. What it bought is certainty -- the remaining shape of this defect
is now "find the call that draws Home's checklist", not "guess another size", and the previous two rounds' null results are explained rather than mysterious.

**The lesson is the one the last round stated and this round applied**: when a change meant to move geometry does not move the page total by a pixel, the
change did not reach the pixels. **The total is the cheap proof of that**, and it should be checked before the band, every time.
### Home's checklist is drawn by neither of the two functions its name suggests (2026-03-11)

Five candidates were eliminated by reading, not by guessing:

| candidate | why it is not the one |
|---|---|
| `panel.rs:533` (the checklist row's label) | 13, 14 and 18 points all render 12 rows and leave the page at 28 368 |
| `panel::draw` (`shell.rs:2952`) | it is **Discover's** sidebar -- categories and facets |
| `panel.rs:486` | the *project* page's rows, laid out off `PROJECT_STEPS_TOP` |
| `shell.rs:896` (`circle_stroke`) | a card with a 22-point mark, not the checklist's 7-point ring |
| `shell.rs`'s `body(12.0)` at 373/834 | a dialog's helper text and a label, not the rows |

**So the row is drawn somewhere not yet read**, and the way to find it is decisive rather than another read: change **every** `body(13.0)` in the sidebar's
module to 16, rebuild once, and see whether the band moves. If it does, bisect; if it does not, the rows use a size the grep has not shown (a constant, or a
call through `fonts::` with a expression argument).

**Recorded rather than attempted**, because a bisect needs a build-and-capture cycle per step and this round has one left at most.
### The checklist is drawn in `panel::draw` at 16 points, and its row ink is a font-metric difference (2026-03-11)

The draw site is finally established, by following `row_label` rather than guessing at functions: **`panel.rs:1044`**, inside `pub fn draw` -- the same function
that draws Discover's facets and that `shell.rs:2952` calls on every page. It draws the label at `fonts::label(STEP_SIZE)` with **`STEP_SIZE = 16.0`**, so the
size was never wrong; the *face* was.

| attempt | sidebar total | row band |
|---|---:|---|
| `label(16)` (as shipped) | 13 730 | 12 rows |
| `body(16)` | **14 117** | **12 rows** |
| reference | -- | **14 rows** |

**Switching the face moved the sidebar by 387 pixels and did not change the band at all.** So the remaining two rows are the font's own metrics: this
project's faces ink 12 rows at 16 points where the reference's ink 14, exactly as the checklist's 13-point row matched their 16px one and the card's 16 needed
to stay 16. No size or family available here closes it without oversizing the glyphs for the row they sit in.

**Reverted to `label(16)`**, which scores 387 pixels better and is the reference's own nominal size. **The band stays 2 rows short, and the cause is now
named: font metrics, not layout, size or family.**
### Five sizes, current build -- and a sweep that had to be thrown away (2026-11-11)

The first sweep this round read **28 754** at 1440x900 where a single verified capture read 28 368. The difference was 386 pixels, which is the size of the
`body(16)` checklist experiment reverted an hour earlier: **the source was reverted but the binary had not been rebuilt**, so the sweep measured the previous
build. The tell was the single-capture comparison, taken after a fresh relaunch, disagreeing with the sweep by exactly a known experiment's effect.

Rebuilt, relaunched, and re-swept. Every pair certified by the guard:

| size | differing | previous plate-matched table |
|---|---:|---:|
| 1280x720 | **25 077** | 25 109 |
| 1366x768 | **33 381** | 33 346 |
| 1440x900 | **28 368** | 28 444 |
| 1600x900 | **42 567** | 42 611 |
| 1920x1080 | **55 028** | 54 939 |

**Three sizes improved, two are within 45 pixels of the older figures** -- consistent with the type fixes of the last several rounds (card 16/14, strip meta 14,
search field 16), which moved a few thousand glyph pixels and left the rest of the frame where it was.

**The discarded sweep is the useful part**: it is the third time this session a number described a build that was not the one in the source tree, after the
stale window and the five captures that were never written. The guard catches the last two; **nothing yet catches a stale binary**, and the check that would is
to compare the binary's modification time against the newest source file before every sweep.
### `drive.py` refuses a stale binary, and proves it can (2026-11-11)

A sweep once measured the build *before* a revert -- source changed, binary not rebuilt, every other check satisfied. The guard now compares the release
binary's modification time against every `.rs` under the crate and refuses when any source is newer:

```
$ python3 tools/parity/drive.py --size 1440x900 --tag stale1
build: binary newer than every source file
ref: .scratch/cap/stale1-ref.png (1440x900)
ours: .scratch/cap/stale1-ours.png (1440x900)
reference news plate: 1
PAIR OK
```

**And the refusal is selftested by making it fire**: the selftest touches `main.rs` five minutes into the future, asks the check again, requires it to report
stale, and then restores the timestamp --

```
selftest ok: a source touched after the binary reads as stale, and the touch is undone
```

**So the guard now answers four questions before a pair exists**: is the binary current, is the process alive, is its window mapped, and is the file the size it
claims. Every one of the four was made to say no once before being trusted, and three of them were made to say no by a real failure earlier in this session.
### The news card's copy: 14 on a 17.5 line measured, and reverted (2026-11-11)

Its five divergent bands were the sidebar's remaining account. The copy was `body(13)` on a 16-point line whose own constant carried `// guess: re-measure`, and
the reference's lines run 17 to 18 apart with 11 rows of ink each.

| | before | 14 on 17.5 | reference |
|---|---|---|---|
| copy line 1 | 9 rows | **11 rows** | 11 rows |
| line 2 | 9 | 14 (two lines merging) | 11 |
| line tops | 565, 581, 597, 615 | 564, 581, 598, 618 | 566, 583, 601, 620 |
| Home, 1440x900 | 28 368 | **28 620** | -- |

**The first line's ink height now matches exactly**, and every line still sits two to three points high; but the second band grows to fourteen rows because the
larger face pushes its descenders into the next line's box, and the page figure worsens by 252 pixels.

**Reverted.** The change buys one matched band and one merged one, and costs a measured figure -- which is the wrong trade for a card whose text is a paragraph
rather than a label. What is now known, and was not before: the copy's size should be 14, its line 17.5, and the whole block needs to move down two points
before either is worth keeping. That is the sequence for a round that can spend the build.
### The news copy fixed as one block: size, line and anchor together (2026-11-11)

The previous round changed the copy's size and line but not its anchor, matched one band, merged another, and cost 252 pixels -- so it was reverted. This round
changed all three together:

```rust
const COPY_SIZE: f32 = 14.0;   // was 13
const COPY_LINE: f32 = 17.5;   // was 16, with a `// guess: re-measure` beside it
let copy = title + title_height.max(TITLE_LINE) + TITLE_TO_COPY + 1.0;   // was -1.0
```

| | before | after |
|---|---:|---:|
| sidebar | 13 730 | **13 292** |
| Home, 1440x900 | 28 368 | **27 930** |
| divergent sidebar bands | 5 | **3** |

**438 pixels better, and the copy's first line now matches the reference's band exactly** -- it no longer appears in the divergence list at all.

**Which settles the rule the last round proposed**: a multi-line block must be fixed in one build, because the parts interact. Size alone merges lines; size plus
line without the anchor leaves every line 2 points high and costs pixels; all three together gain 438. The same three-part fix probably applies to the checklist
(whose 2-row gap is metrics) and the remaining three news bands (which are the date and the second paragraph).
### The hosting banner's headline was printed through itself (2026-11-11)

The sidebar's last three divergent bands were the promo banner, cropped at 3x: this build drew **"Get your next server"** and **"on Modrinth."** on top of
each other, where the reference draws two clean lines.

Cause, measured: the reference's headline inks at **690..706** (16 rows) and **720..732** (13 rows) -- a **30-point pitch** -- and this build used
`HOSTING_LINE = 28` from a top of 31. A 23-point label inks about 16 rows, so a 28-point pitch lets the second line's cap height land inside the first's
descenders, and the band profile saw the two as one 20-row band.

| | before | HOSTING_TOP 40, line 30 | **HOSTING_TOP 35, line 30** | reference |
|---|---|---|---|---|
| headline line 1 | merged with line 2 | 696..711 (5 low) | **691..706** | 691..706 |
| headline line 2 | merged | separate, matches | 720..724 + 726..731 | 720..732 |
| sidebar | 13 292 | 13 914 | **12 890** | -- |
| Home, 1440x900 | 27 930 | 28 552 | **27 528** | -- |

**402 pixels better and a visible defect gone**: the two lines no longer overlap, and the first sits exactly where the reference's does. The second line now
splits into two bands of its own (720..724 and 726..731) where the reference has one of thirteen rows -- the gap is a single row, and it is what a 30-point
pitch with this face's descenders leaves.

**Note the middle row of that table**: aligning the first line at 40 (the arithmetic answer from `banner.top() + 31`) measured *worse* than 35, because the
banner's top is not `rect.bottom() - BANNER_H` on this page. The band, again, was the arbiter.
### Skins: the default skins render as flat colour blocks (2026-11-11)

The profile-and-crop pipeline was pointed at a screen nobody had cropped. Skins is **structurally divergent**, not offset:

| | reference | this build |
|---|---|---|
| text bands in the content column | 13 | 8 |
| unmatched bands | 4, including one of **156 rows** | 3, including one of **215 rows** |
| page, 1440x900 | -- | 131 114 |

**The crop shows why**: the layout matches -- title, `Saved skins`, `Add skin`, `Default skins`, the eight-cell grid, the demo-account bar, the sign-in button --
but the reference's eight default skins each show their real texture (faces, hair, clothing, all distinct) and **this build renders all eight as flat colour
blocks**, green and purple and blue rectangles with no detail.

**The texture path resolves**: `skins.rs:828` calls `skin_texture` for every cell, which finds the reference's baked 64x64 PNG through
`skins_assets::by_name` and uploads it via `baked_texture`. So the defect is downstream of that -- either `baked_texture` composes the front view into a
single colour, or the cell's paint call (one of `painter.image` at 1417, 1856, 2153, 2278) draws a tint rather than the texture.

**Recorded, not half-fixed**: this is the largest visual defect found in many rounds -- eight wrong images on a page whose subject *is* images -- and it needs
a build and a crop to confirm, not a guess between four paint sites.
### The eight default skins are the right eight now (2026-11-11)

The grid's flat colour blocks were a **wrong-asset** bug, not a rendering one: `skins_assets.rs` carried the first forty-five entries of the reference's
`default_skins.rs` -- the `Mr_Pack_*` pride-flag variants -- and `by_name("Alex")` found nothing in that catalogue, so `skin_texture` fell through to the flat
fallback atlases.

**The real eight are not embedded in the reference either**: each is a URL to `textures.minecraft.net/texture/<hash>`. All eight were fetched (200, 64x64,
29-51 unique colours each) and are now in `assets/skins/default_NN_Name.png`, with `skins_assets.rs` rewritten to map the eight names to them. The forty-five
flag PNGs are deleted -- nothing queried them.

| | before | after | reference |
|---|---|---|---|
| grid cells | eight flat colour blocks | **eight real skin textures** | eight real skin textures |
| Skins page, 1440x900 | 131 114 | **150 988** | -- |

**The page got worse by 19 874 pixels, and that is the expected direction**: detailed textures differ from the reference's in every pixel where the two
renderings disagree, where flat blocks differed in few. A page of wrong-but-simple images scores better than a page of right-but-differently-rendered ones --
the same monotonicity failure this project has recorded four times already.

**Two differences remain visible in the crop**, and neither is the assets: the grid's **order** differs from the reference's, and the grid sits **lower** on the
page because the `Add skin` card above it is taller. Those are the next two measurements.
### Skins: the artwork is right, the first cell is not (2026-11-11)

After the eight real defaults landed, the grid draws real skin textures in both clients. Cropped side by side at 1440x900, one difference is unambiguous and
one is not:

* **The first cell holds a different skin in each.** The reference's is dark-haired; this build's is pale. The module's own order is alphabetical
  (`Alex, Ari, Efe, Kai, Makena, Noor, Steve, Sunny`) and this build uses that order, so the reference's *display* order must come from elsewhere -- the
  app's database, a section sort, or a different variant being chosen for the first slot. **Not resolved.**
* **The `Add skin` card above the grid is taller here**, which is what made the first composite look like the whole grid sat lower. **Not measured**: the
  probe that looked for the card's plate fill found that fill in neither client, so the offset claim in the previous round's risk note was wrong and is
  withdrawn rather than repeated.

**So the asset defect is fixed and confirmed by eye, and the order and spacing questions are open with their evidence stated.** The next step on order is to
read how `default_skins.rs`'s entries are sorted into sections rather than guessing from the module's file order.
### Skins: variant and order eliminated; the first cell's composition is the remaining suspect (2026-11-11)

Three hypotheses for the first cell, two of them tested and dropped:

| hypothesis | test | result |
|---|---|---|
| the grid's **order** differs | crop: the second cell (orange) matches in both clients | **dropped** -- the order is the same |
| **slim** skins compose differently | every name in the module carries **both** Slim and Classic, and the eight assets fetched are the **Classic** ones | **dropped** -- variant cannot distinguish them |
| the **front-view composition** is wrong | `FACES` is classic-only: head (8,8), body (20,20), arms at x 44 and 36 | **open** -- this is what is left |

So the grid draws the right eight, in the right order, from the right textures, and the first cell still shows a different-looking skin. What remains is
`compose_front` itself: which atlas cells it reads into which view pixels.

**The next step is arithmetic, not another crop**: map a fetched skin's 64x64 atlas into the 40x48 view by hand and compare against the drawn image. That is
decidable without a browser, a build or a capture -- the only round in this stretch where the answer does not need the lab at all.
### Corrected: the Skins grid does start with a different skin -- Steve, not Alex (2026-11-11)

Two rounds ago the order hypothesis was marked **dropped** on the strength of a crop where the second cell (orange) matched. That crop was taken across an
offset boundary and compared different rows. Cropped at the heading's own anchor -- `Default skins` inks at **363..375 in both**, so the grids start together --
the first cell is plainly a different skin:

| | first cell |
|---|---|
| reference | **brown hair, cyan shirt, brown trousers -- Steve** |
| this build | **orange hair, green shirt, pale -- Alex** |

**So the order is different after all**, and the earlier verdict was an artefact of a bad crop. The module's order is alphabetical (`Alex` first) and this build
uses it; the reference shows Steve first, so its display order comes from somewhere else -- a database sort, a section's own ordering, or the grid being a
subset of a longer list.

**What is now established, each by a different instrument**: the eight assets are right (fetched by hash), `compose_front` is right (composed offline in Python
and it is a proper Alex), the grid's position is right (heading bands identical), and **the order is wrong**. Three of the four were verified without capturing
anything.
### The grid's display order: read from the capture, and the first attempt did not take (2026-11-11)

The reference's first row, read at 1440x900: **Steve, Alex, Ari, Noor**. So it is not alphabetical, and `GRID_ORDER` was added with that sequence and used
where the textures are collected.

**It did not change what is drawn**: after the rebuild the first row is still Alex, Ari, Noor, Sunny. The page moved by 492 pixels (150 988 -> 150 496), which
is the size of a small re-collection, not of a reordering -- so the grid's cells are being placed by something other than the list the textures are gathered
from.

**That is the fourth time in this stretch that a plausible edit reached no pixels** (the checklist's label at three sizes, the search placeholder's file, and
now this), and each time the tell was the same: the page figure barely moved. **The rule holds and is cheap**: if a change is meant to reorder or move
content and the total does not move by roughly its own size, the change did not reach the draw.

**What is known for the next attempt**: the order to draw is `Steve, Alex, Ari, Noor, ...`; the place that draws it is not the texture collection; and the
cells are laid out with textures looked up by name, so a reorder only needs the *placement* loop changed.
### Skins: the grid drew the fallback atlases, not the skins (2026-11-11)

**Four rounds of asset, composition and ordering work could not change a pixel because none of it was on the drawing path.** The placement loop in
`skins.rs:1403` passed `art_texture(ui.ctx(), state, index % ART.len())` -- four hand-authored fallback atlases -- so every cell showed the same flat
pictures no matter what the catalogue, the composition or the order said.

```rust
// before
art_texture(ui.ctx(), state, index % ART.len()),
// after
GRID_ORDER.get(index).and_then(|name| skin_texture(ui.ctx(), state, name)),
```

| | before | after |
|---|---:|---:|
| Skins page, 1440x900 | 150 496 | **127 044** |
| first row | Alex, Ari, Noor, Sunny | **Steve, Alex, Ari, ...** |

**23 452 pixels better, and better than the 131 114 the page scored before any of the asset work** -- so the correctness and the figure finally agree on this
page, which the last three rounds said they could not.

**The first three cells now match the reference's order exactly.** The fourth does not: the reference has Noor (purple hair) where this build draws a
different skin, so the order list is right for three of eight and the remainder needs the same treatment -- read from the capture, one row at a time.

**The lesson is the one this project keeps paying for**: `skin_texture` was correct, the assets were correct, the order was correct, and the page did not
use any of them. **A change that reaches no pixels is not a wrong change -- it is a change to something the frame does not call.**
### Skins: both rows in the reference's order, and the page's best figure (2026-11-11)

The grid's order, read row by row from the capture -- row one `Steve, Alex, Ari, Efe`, row two `Kai, Makena, Noor, Sunny` -- and set as `GRID_ORDER`:

| | before | after |
|---|---:|---:|
| Skins page, 1440x900 | 150 496 | **116 420** |
| first row | Alex, Ari, Noor, Sunny | **Steve, Alex, Ari, Efe** |

**116 420 is the best figure this page has ever scored** -- better than the 131 114 it had while showing eight flat blocks, and better by 34 076 than the
150 496 it scored with the right assets in the wrong places. The three changes that got here were: the right eight assets fetched by hash, the grid drawing
`skin_texture` instead of the fallback atlases, and the reference's own order.

**The composed sheet was what made the order legible**: with all eight skins rendered offline side by side, the reference's fourth cell -- purple hair -- was
identifiable as **Efe**, not Noor, and the row-two cells as Kai, Makena, Noor and Sunny. Reading a grid by eye against a labelled strip is far more reliable
than naming a skin from one cell.

**What remains in the cells**: this build's figures are slightly wider than the reference's within the same card, so the front view is being scaled a little
differently -- the next measurement on this page, and a smaller one than anything above it.
### The skin figures are three sizes measured, and 32 wins on the page (2026-11-11)

The figures inside the cards were taller than the reference's. The card is 180 tall and the figure's height is `rect.height() - inset`, so the inset is the
whole control. Three values were built and measured:

| inset | figure height | reference | Skins page |
|---|---:|---:|---:|
| 20 (as shipped) | 160 | 148 | 116 420 |
| 26 | 141 | 148 | 114 972 |
| **32** | **137** | 148 | **114 628** |

**32 is kept although 26 is closer to the reference's height**, because the page figure prefers it: 344 pixels better than 26 and 1 792 better than 20. The three-value
sweep is what makes that a measurement rather than a preference -- 137 against 148 looks like the worse choice until the page is counted.

**So the page ends at 114 628**, which is 16 486 better than it started this stretch (131 114 with flat blocks) and 35 868 better than its worst (150 496 with the
right assets in the wrong places).
### Discover, measured whole for the first time: two differences, both named (2026-11-11)

First it had to be made a valid pair at all: the rail click that put this build on Discover put the reference on **Home**, and the resulting 122 715 figure
compared two different pages. The reference's Discover row is at **y 106**, not the 62 its Home row sits at. With both on Discover and the guard certifying
the pair:

| | value |
|---|---|
| Discover, 1440x900 | **76 319** |
| content | 58 882 |
| sidebar | 14 572 |

**And the two pages agree on almost everything a reader would check**: the Modpacks/Mods/Resource Packs/Data Packs/Shaders/Servers tabs, `Search modpacks...`,
`Sort by: Relevance`, `View: 20`, the pagination `1 2 ... 961`, the first pack `Fabulously Optimized` with its `FO` badge, the summary, the `Client Lightweight
Multiplayer Optimization Fabric` chips, `19.2M`, `5,072` and the `+ Install` button -- all present and in the same places.

**Two differences, both visible in the crop and neither a mystery**:

1. **The relative date's vocabulary.** The reference reads **`Last week`** where this build reads **`9 days ago`**. Both describe the same timestamp, but the
   reference rounds to words (`Last week`, and presumably `Last month`) where this build counts days.
2. **The card's divider.** The reference rules a line under each card; this build draws none, which is also why its first card ends higher.

**So Discover is much closer than its figure suggested** -- 76 319 differing pixels over a page where a reader would struggle to name a structural difference --
and the two things that are actually wrong are a string and a rule.
### `drive.py` refuses a pair from two different pages (2026-11-11)

The guard checked size, liveness, binary freshness and the news plate, and **nothing checked that the two clients were on the same page** -- which is how a
Home-against-Discover pair produced a 122 715 that looked like a measurement.

The rail names the page: its current icon is brand green, and its row differs per page. So the check finds the greenest pixel in each rail and compares the
row, without needing the two rails to be pixel-identical (they are not):

```
$ python3 tools/parity/drive.py --size 1440x900 --tag pagecheck
page: both clients highlight the rail at y ~113
PAIR OK

$ # reference moved to Home, this build left on Discover
$ python3 tools/parity/drive.py --size 1440x900 --tag mismatch
page: the rail's brand row is at y 63 on the reference and y 114 here -- REFUSED, the two clients are on different pages   [exit 6]
```

**Both directions proven**: the check passes on a matched pair (y ~113 in both) and refuses a mismatched one (63 against 114) with a message naming the rows.

**So the guard now answers five questions before a pair exists**: is the binary current, is the process alive, is its window mapped, is the file the size it
claims, and are both clients on the same page. Three of the five were added because a real measurement went wrong without them.