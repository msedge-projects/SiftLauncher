
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