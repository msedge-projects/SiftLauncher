
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