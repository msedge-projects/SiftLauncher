# GATES

Measured gates, with the numbers and the command that produced them. A gate is a fact about a build on this
machine, not a target to argue about later.

## The rendering backend gate

> A software rasteriser ships if and only if it holds **<= 20 MB idle RSS** and **<= 16 ms p95** at 1440x900.

### What is actually running

`eframe` with `glow` (OpenGL). The lab runs Xvfb with no GPU, so Mesa serves that GL through **llvmpipe** -- the
software rasteriser this gate is about. The build under test is therefore already the software path.

### Measured, 1440x900, empty library, after twelve seconds idle

| quantity | value | how |
|---|---:|---|
| idle RSS, renderer process | **48.4 MB** (49 544 kB) after twelve seconds idle | `awk '/VmRSS/' /proc/<pid>/status` |
| RSS while being driven | 111.3 MB at its peak | same, sampled during the interaction pass |
| frame interval p95 | **13.02 / 13.12 / 14.87 ms** over three 30-frame windows | `SIFT_FRAME_LOG` summaries |
| frame body (CPU) p95 | 0.14 -- 0.16 ms | same lines |

### Verdict

| bound | required | measured | result |
|---|---:|---:|---|
| idle RSS | <= 20 MB | 111.3 MB | **FAIL** |
| frame p95 | <= 16 ms | 13.0 -- 14.9 ms | **PASS** |

**The software rasteriser does not ship under this gate as written**, because the memory bound is missed by a
factor of two and a half (48.4 against 20). The time bound is met with room to spare, and the CPU cost per frame is two orders of magnitude
under it.

### What the 111 MB is, and why the bound is not reachable here

- Roughly half of it is the GL driver's own allocations for a 1440x900 default framebuffer and its back buffers
  under llvmpipe, which the app does not own; the peak of 111 MB during a drive is those buffers plus the icon
  textures arriving.
- The rest is the Rust binary's mapping, the font atlas (five Inter weights embedded, 1.5 MB of TTF), the texture
  cache for project icons and skin atlases, and egui's retained meshes.
- A 20 MB ceiling is below what any GL-backed desktop window reaches in this environment: even a bare
  `Xvfb` + `glxgears` sits in the tens of megabytes. Nothing in the application's own code can be removed to
  reach 20 MB while it still draws a 1440x900 frame through OpenGL.

Recorded rather than reinterpreted: the gate is failed, and the reason is the bound, not a measurement error.
