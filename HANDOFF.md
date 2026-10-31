# HANDOFF — where the parity work stands (2026-10-31, session 5)

Read this first, then `docs/GAP-MATRIX.md` (625 rows, seven verification passes), then
`docs/TOOLING.md` (what this environment can actually do — the tool list is larger than the
one the model is handed).

## What is true right now

* 17 commits on `master` on top of `8140954`, **none pushed**: this environment has no git
  credentials (`git push` → could not read Username). Nothing is tagged; no release exists.
* 166 tests pass; `cargo build --release` is green; the tree is clean.
* The capture lab works and is committed: `tools/parity/lab.sh` (container `sift-lab`, Xvfb
  :99, both apps at 1440x900) and `tools/parity/probe.py` (colour runs, OCR boxes, bounding
  boxes, tile RMS). `.scratch/cap/` holds `ref-*` and `ours-*` pairs; `.scratch/` is
  git-ignored, so those captures do not travel with the repo.
* The reference does not need to be re-derived: `.scratch/modrinth-0.22/` is the 0.22.0 tree
  and `.scratch/dl/squashfs-root` is the extracted AppImage the lab launches.

## What has been matched against captures, and holds

The shell (title bar 48, rail 64 with 48-point circles on a 52 pitch, the content surface with
its rounded corner and inset shadow, the sidebar's gradient and its 1px brand-gradient border),
the action bar's two pills and the 44-point window controls, the Home hero's rhythm, the
sidebar's card and rows, Discover's column at x88/1028 with its pill bar, 48-point search field,
sort row, 36-point pager and 142-point cards, and the Settings modal's 84-point header, 40-point
rail rows, theme cards at 198x158 with their mock windows.

## The one known bug, with its evidence

A settings row advances by whichever of its control and its text is **shorter**, so at the
source's 18/16 type the next row draws inside the description. Probe output is in
`docs/GAP-MATRIX.md` under "The loop probe": the block loop prints `gap=0.0` after that row
while the next block is 56 points lower, so the advance happens somewhere other than the arm's
return — and a strict search finds no `y +=` or `gap =` inside the arm. **Next step: put the
probe on both sides of the arm call (print y before and after `gap = match *block`), which will
name the line.** Until then the type stays at 14 over 12, which is self-consistent.

## What to do next, in order

1. The advance bug above, then raise the row type on all thirteen tabs and re-capture.
2. The stray 1px rule at y 673 through the Appearance body text, and the missing section divider
   at y 746.
3. The settings rail: 3px scrollbar thumb at x 497..499, labels 62x11 (ours 50x9).
4. Discover: the sidebar's category rows (four plain rows at a 26 pitch against eight with
   icons at 32, plus the Upgrade row), the hosting promo (268x136 solid against 297x272
   full-bleed), and the card's missing update clock.
5. The Instance page — 62 rows, untouched, the only page with a file manager.

## How not to repeat this session's mistakes

* A commit message is not evidence. One commit here claimed a column had moved while the
  capture still showed it in the old place; the fourth verification pass caught it.
* A capture taken before the app has settled is indistinguishable from broken code: the pager
  "disappeared" once because the feed had not answered yet.
* Raise type sizes only after the layout that holds them is proved to measure; the alternative
  is a font problem that is not a font problem.
