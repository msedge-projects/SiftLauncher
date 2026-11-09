# Where the shared UI lives

A round was spent hunting for the dropdown renderer through `menu.rs`, `popover.rs` and `pages/mod.rs` before finding
`open_list` inside `pages/screenshots.rs`. There is no existing file that answers "where is this drawn", and every shared-
component change starts with that hunt. This is the answer, verified by `grep` rather than memory -- two of the entries
below were in files I would have guessed wrong.

| element | file | symbol |
|---|---|---|
| dropdown popover (sort, group-by) | `crates/sift-ui/src/pages/screenshots.rs` | `open_list` |
| dropdown plate and shadow | same | `menu_plate` |
| chip / select control | same | `select` (suffix chevron is a parameter) |
| empty and no-results states | same | `empty_state` |
| the instance page's own empty state | `crates/sift-ui/src/pages/instance.rs` | `empty_state` (a separate one) |
| popover placement and clamping | `crates/sift-ui/src/pages/home.rs` | `menu_placement` |
| tooltip plate and arrow | `crates/sift-ui/src/shell.rs` | `tooltip_below` |
| rail, title bar, window chrome | `crates/sift-ui/src/shell.rs` | (the shell's own draw pass) |
| sidebar panel, news card, hosting banner | `crates/sift-ui/src/panel.rs` | `draw`, `column` |
| the software rasteriser | `crates/sift-ui/src/soft/mod.rs` | the blend loop; bilinear at the texture sample |
| fonts and the face table | `crates/sift-ui/src/fonts.rs` | `body`, `label`, `semibold`, `title`, `heading` |
| glyph set | `crates/sift-ui/src/icons.rs` | `Glyph`, `draw` |

**Two that were guessed wrong**: `menu_placement` is in `home.rs` (not `shell.rs`), and `open_list` is in `screenshots.rs`
(not a menu module of its own). Both are cases where a component used by many pages lives in the first page that needed it.