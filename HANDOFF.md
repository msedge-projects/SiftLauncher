# Handoff — 2026-11-02, evening

## The tree
- HEAD: see `git log -1`. **286 commits** since 8140954. Nothing pushed (no credentials).
- `cargo test --release` in `/scratch/crates/sift-ui`: **236 passed, 0 failed**.
- Library crates: sift-core 230, sift-net 307, sift-game 153, sift-loader 33.

## Proved today, by driving rather than reading
| feature | proof |
|---|---|
| Join from the Servers page | picker -> Instance 2 -> java carried `--quickPlayMultiplayer purpleprison.net:25565` |
| Duplicate instance | "Instance 2 copy" on disk (27 files), its cfg renamed, a new Library row, toast |
| Rename an instance | `name=Renamed Pack` in the instance's cfg, the row re-sorted |
| "Installed" state | a project already in the library says so on its card and its page |
| New server dialog | press opens the modal with Name/Address, Cancel and "+ New server" |
| Ctrl+1..5 | reaches the rail's pages and leaves an open instance page |
| Discover | "Discover content" label, six page sizes, chevrons, the reference's sort labels |
| Skins | saved files now draw the composed 16x32 character, not a head |
| File rename | inline TextEdit over the row, committed through rename_entry |

## Two retractions, both recorded in place
1. **The Servers page**: the *reference itself* answers "Modrinth App update required", so this build's server list
   was never comparable against a running reference. Its rows are OPEN-WITH-CAUSE; Join, Copy IP and the New
   server dialog stay DONE because they were driven.
2. **"The reference has no Screenshots page"**: wrong. It does; my presses had hit the divider and the "+".
   Corrected with a measured table of rail positions.

## The lab, as it now behaves
- `screen = content + 22` on :99; `tools/parity/lab.sh click` takes **screen** coordinates.
- Launch: `docker exec -d sift-lab bash -lc 'cd /scratch/crates/sift-ui && setsid nohup env DISPLAY=:99 ./target/release/SiftUi >/dev/null 2>&1 </dev/null &'`
- `pkill -9 -f target/release/SiftUi` and `pkill -f java` before every drive; a Minecraft window steals every click.
- The reference runs with `tools/parity/lab.sh ref` (it takes the whole display, so swap, don't share).
- Keys reach the app only when a window manager is running; :99 has none, so typing works through egui's own focus
  but xdotool key needs the pointer over the window.

## What is left, in the order I would take it
1. **The rail's geometry** has never been probed against the reference: our icons start at content y 48 and the
   reference's start lower. Measure both with a pixel scan and correct.
2. **Skins' 3D preview** — the reference rotates a model; this build draws a flat elevation.
3. The 466 OPEN rows in `docs/GAP-MATRIX.md`, most of them measurement work per page.
4. The project page's Follow/Save/Report rows need an account service this build has no client for.
