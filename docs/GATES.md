
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