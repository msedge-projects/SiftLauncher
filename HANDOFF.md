# Handoff — 2026-11-02

## Where the tree stands
d39f207 Take a dropped PNG as a new skin, so the tile's promise is true
- sift-ui: ok. 211 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
- library crates: sift-core 230 tests, sift-net 307, sift-game 153, sift-loader 33 (from the coverage slice)
- Nothing pushed: no credentials in this environment. `master` is 245+ commits ahead of `origin/master`.

## What landed since the last handoff
- **Server join, end to end.** Instance Content tab carries an address field and a Join button; `Game::start_server` writes
  `JoinServerOnLaunch` + its address into the instance for exactly one launch and clears them after; the engine turns them
  into `--quickPlayMultiplayer`. Proven by the java command line: `--quickPlayMultiplayer mc.hypixel.net:25565`.
- **Skins render from disk.** `skins/<name>.png` under the platform data root; head front (8,8,8,8) and overlay
  (40,8,8,8) drawn on cards and in the preview column; dropped PNGs are copied in and rescanned.
- **Project page rebuilt** (owner: subagent): header, tabs, Compatibility sidebar.
- **Screenshots page**: selection, viewer with caption and arrows, groups, filters.
- **Settings**: driven against the reference, six fixes.
- **Ctrl+1..5** reach the rail's pages.
- **Dead Library page deleted** (975 lines behind a `pub mod` nothing called).

## Lab technique that saved this session
1. **Launch with `docker exec -d`.** `... & ` inside `docker exec bash -lc` leaves a zombie and no window.
2. **`pkill -f java` before driving anything.** A launched Minecraft shows a modal "No supported graphics backend"
   window that steals every click; six "the button does nothing" conclusions were this.
3. **`consume_shortcut`, not `key_pressed` + modifier.** The modifier flag is already cleared on the frame the digit arrives.
4. **Commit your own paths immediately.** `git add -A` swept two other agents' work into commits; a `git checkout` of a
   path also wiped an uncommitted edit. Use explicit paths, commit within the same minute.
5. **Print the widget rect.** Fifteen presses have failed on aim; none on logic.

## Open, with causes recorded
- Servers page has no Join button (five mechanisms failed there; the guidance line points at the instance page).
- Files rename: tested core (`rename_entry`, four tests) with no control; the row control is parked.
- Skins: no 3D model, no per-card Edit/Delete, Apply/Reset need the service.
- 466 OPEN rows in `docs/GAP-MATRIX.md`; see its "How to read this file".
