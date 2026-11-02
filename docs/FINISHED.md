# Definition of finished, and where the work stands

The bar: every screen at **≤2 000 differing pixels** in a 50/50 blend against the reference at 1440x900, both
clients in the same state, with a capture behind every claim.

## Measured, with evidence

| screen | differing pixels | evidence | where the remainder is |
|---|---:|---|---|
| Home, empty | **31 171** | `ab-ref-home.png` vs `ours-home-a.png` | of which only **5 549** survive at a threshold of 160 |
| Discover | **134 450** | `ref-discover-fresh.png` vs `ours-discover25.png` | card icons and their artwork |
| Skins | 185 090 -- **not comparable** | `ref-skins.png` vs `ours-skins-b.png` | the reference's capture has no saved skins and no search row; the two are in different states |
| Screenshots | not measured | -- | needs a fresh capture in the same state |
| Servers | **not comparable** | `ref-servers.png` | the reference refuses its own page: "Modrinth App update required" |
| Settings, 13 tabs | not measured | `ref-settings*.png` | needs one A/B per tab |
| Instance page, 5 tabs | not measured | -- | |
| Project page | not measured | -- | versions pager also unbuilt |

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

323+ commits, 249 tests green, a heartbeat measuring the screen every four minutes in the lab container.
