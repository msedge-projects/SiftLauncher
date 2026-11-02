# Handoff — 2026-11-02, final state of this run

## The tree
- **297 commits** since 8140954 on local `master`. Nothing pushed: no credentials in this environment.
- `cargo test --release` in `/scratch/crates/sift-ui`: **242 passed, 0 failed**.
- Library crates: sift-core 230, sift-net 307, sift-game 153, sift-loader 33.
- No panics across a final pass over all five pages, the create dialog, Settings and the account button.

## Proved this run, by driving rather than reading
| feature | the evidence |
|---|---|
| Server join from the Servers page | java carried `--quickPlayMultiplayer purpleprison.net:25565` |
| Duplicate instance | "Instance 2 copy" on disk (27 files), its cfg renamed, a new row |
| Rename an instance | `name=Renamed Pack` in the cfg, the row re-sorted |
| "Installed" state | a project already in the library says so on its card and its page |
| New server dialog | opens, fills, saves |
| Ctrl+1..5 | reaches every page and leaves an open instance page |
| Discover's header | "Discover content", six page sizes, chevrons, the reference's sort labels |
| **Add mods -> filtered browse -> install** | pager 956 -> 39 and 3 898 -> 478 for a Fabric 26.3 instance; `fabric-api-0.165.0+26.3.jar` landed in Instance 2 with no picker |
| Per-tab search placeholder | "Search mods..." on the Mods tab |
| Skin preview | a software-rendered model that turns on drag; saved files draw as characters |
| Settings | side-by-side with the reference's own captures: same tabs, groups, theme cards, foot |

## Retractions recorded in place, not quietly dropped
1. The reference's Servers page shows **"Modrinth App update required"**, so this build's server list was never
   comparable against a running reference. Its rows are OPEN-WITH-CAUSE; Join, Copy IP and New server stay DONE
   because they were driven.
2. "The reference has no Screenshots page" -- **wrong**; it does. My presses had hit the divider and the "+".
3. "Our rail starts higher than the reference's" -- **wrong**; both are at 72/124/176/228/280 with a 52-point pitch.
4. "The gear sits 30 points high" -- **wrong**; 816/868 against 815/867.

Each was a number read off a capture instead of measured. The rule that would have prevented all four is already
at the top of `docs/GAP-MATRIX.md`: print it from the code or scan it from the pixels.

## What is still open, with its cause
- **Discover is not faceted by category chips the way the reference's sidebar is** (the chips exist; the panel's
  category press is one facet at a time, as the reference's is).
- The pager now appears at both ends of the results, as the reference's does (`draw_pager` with a tag per strip).
- **Screenshots' "New group"** answers with a toast that says groups are not built.
- **Follow / Save / Report** on a project need an account service this build has no client for.
- **The Servers page** cannot be compared until Modrinth's hosting accepts this client version.
- **466 OPEN rows** in `docs/GAP-MATRIX.md`, most of them per-element measurement work.

## The lab, as it now behaves
- `screen = content + 22` on :99; `tools/parity/lab.sh click` takes **screen** coordinates.
- Launch: `docker exec -d sift-lab bash -lc 'cd /scratch/crates/sift-ui && setsid nohup env DISPLAY=:99 ./target/release/SiftUi >/dev/null 2>&1 </dev/null &'`
- `pkill -9 -f target/release/SiftUi` and `pkill -f java` before every drive; a Minecraft window steals every click.
- The reference runs with `tools/parity/lab.sh ref` and takes the whole display.
- Print the widget's rect (every page has a probe behind an env var) before pressing: twenty presses have failed on
  aim this run, none on logic.
