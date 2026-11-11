//! The skins page: the `Skin selector` heading, its toolbar, and the grid of skin tiles.
//!
//! Follows the vendored app's skins route and the components it renders:
//!
//! * `app-frontend/src/pages/Skins.vue` — the container is
//!   `skin-layout box-border grow p-4` (line 1085), the heading is
//!   `h1.m-0.text-2xl.font-bold.flex.items-center.gap-2` carrying
//!   `appMessages.skinSelectorLabel` (lines 1087-1089), the left column is the
//!   `SkinPreviewRenderer` under a sticky wrapper (lines 1086-1239), and the right column is
//!   the list with `pt-2` (lines 1241-1261). The route's own stylesheet splits the page
//!   `minmax(0, 1fr) minmax(0, 2.5fr)` with `gap: 2.5rem` (lines 1340-1348), collapsing to one
//!   column under 700 points (`@media (max-width: 700px)`, line 1345).
//! * `.../components/ui/skin/VirtualSkinSectionList.vue` — the section list. Its geometry is
//!   a block of constants: the tile aspect `31 / 40` (lines 47-48), the grid gap `12`
//!   (line 49), the first section's `pt-1` and the rest `pt-6` (line 366),
//!   `SKIN_SECTION_SPACING` 24 (line 51), the header's 28 points (line 52), the content's
//!   `pt-2` (line 375), and the card width as `(listWidth - gaps) / columnCount`
//!   (lines 130-137) with `cardHeight = cardWidth * 40 / 31` (lines 139-141). The section
//!   header is a `DropdownIcon.size-6` plus `text-xl font-semibold leading-7 text-primary`
//!   in a `gap-[6px]` row (lines 373-388). The add-skin control is the Saved-skins grid's
//!   **first tile**, a `SkinLikeTextButton` at `aspect-[31/40] rounded-[20px]`
//!   (lines 426-443).
//! * `ui/src/components/skin/SkinButton.vue` — the card itself: `aspect-ratio: 31 / 40`,
//!   `border: 1px solid var(--surface-4)`, `border-radius: 20px`,
//!   `background: var(--surface-3)` (lines 153-162), with its action row and any text laid
//!   over the bottom at `bottom-3 px-3` (lines 118-123).
//! * `ui/src/components/skin/SkinLikeTextButton.vue` — the dropzone tile: a dashed
//!   `border-surface-5` edge at `rounded-[20px]` on `bg-surface-2` (lines 58-66), a
//!   `size-8` glyph, `gap-4` to the text block, `text-base font-semibold leading-6` over
//!   `text-sm font-medium leading-5 text-primary`, and `gap-0.5` between the two lines
//!   (lines 81-95).
//! * `.../components/ui/skin/EditSkinModal.vue` — where the model lives. The variants are
//!   the literals `'CLASSIC'` and `'SLIM'` (line 41), their labels are
//!   `app.skins.modal.arm-style-wide` and `app.skins.modal.arm-style-slim`
//!   (lines 44-45, 185-192), and the modal opens on `variant = ref<SkinModel>('CLASSIC')`
//!   (line 249), which is also what it resets to (lines 351, 367). So the page's default
//!   model is the classic one, not slim.
//! * `app-frontend/src/locales/en-US/index.json` — every string below, key by key.
//!
//! **What this page does not have, because the reference does not either.** The skins page
//! has no search field, no model tabs and no `Upload` button in a toolbar: the only upload
//! affordance in the source is the add-skin tile inside the Saved-skins grid
//! (`VirtualSkinSectionList.vue:426-443`), which opens a file input (`Skins.vue:826-847`).
//! It also has no empty state at all — with nothing saved, the section simply renders the
//! add-skin tile and no prose. And its `Skin` type carries no author (`helpers/skins.ts:6`
//! names a skin, `:18` its variant; the author line this page draws is this page's own).
//! The toolbar, the two empty states and the author line are therefore this launcher's
//! additions on top of the source's structure, and every string they print is copied from
//! the locale rather than invented — see the `Strings` block for where each one comes from.
//!
//! **The grid arithmetic is the library page's, by instruction.** The skins source measures
//! its own list column and switches *fixed* column counts by window width
//! (`VirtualSkinSectionList.vue:114-128`: three by default, four at 1300, five at 1750 and
//! six at 2050). This page instead mirrors
//! `pages/library.rs`'s `grid_layout` and the expressions behind it —
//! `gap = remSize * 0.75`, `columnCount = max(1, floor((gridWidth + gap) / (minWidth + gap)))`,
//! `cardWidth = (gridWidth - gap * (n - 1)) / n`, and the
//! `windowWidth < 1280 ? 8rem : 10rem` minimum — from
//! `instance-group/index.vue:121-134`, citing that same file for the breakpoint rule as the
//! brief asks. The card's *height* comes from the skin tile's own aspect instead of the
//! instance card's `+ 3.375rem`: `cardHeight = cardWidth * 40 / 31`
//! (`VirtualSkinSectionList.vue:139-141`), because a 31/40 skin tile is the one proportion
//! the source is unambiguous about.
//!
//! **What the column split costs.** The grid therefore resolves inside the list column,
//! not across the whole route: at 1440 the route's box is 1044 points, its 2.5fr column
//! 717.14 of them, and the library's rule takes four 170.29-point columns -- the same four
//! the reference's own list lays out (its first card starts at x 407, its last ends at 1123).
//! The reference's list column is 715.7 wide against this page's 717.14; that 1.4 points is
//! the whole cost of taking the library's rule over the source's fixed column counts, and it
//! lands as about a point on each card's right edge.
//!
//! **The grid at 1440.** The shell's content column is 64..1140, the route pads it `p-4` to
//! 80..1124, the window is not below 1280 so the 160-point minimum applies, and the list's
//! 717.14 points give `floor((717.14 + 12) / (160 + 12)) = 4` columns of
//! `(717.14 - 3 * 12) / 4 = 170.29` points, each `170.29 * 40 / 31 = 219.72` tall --
//! the 170 by 219 the reference's default cards measure. The 8rem branch is the 960-point
//! window's: the same furniture leaves 564 points of route, 374.29 of list, and two
//! 181.14-point cards.
//!
//! **The page box is the route's, not the shell's 24-point inset.** `Page::rect()` starts
//! at x 88 because the shell's pages pad themselves by `CONTENT_INSET`; `Skins.vue:1085`
//! pads the route itself with `p-4` (16), so the reference's columns start at x 80 and
//! its heading -- eight points further in, inside the sticky wrapper's `p-2` -- at
//! x 88. [`page_box`] recomputes the box from the content column for exactly
//! that reason: taking `rect` and insetting it again put this page's columns 24 points
//! right of the reference's, and the capture measured it.
//!
//! **Faces from a local file, not from a renderer.** The source's whole left column is a 3D
//! `SkinPreviewRenderer` (`Skins.vue:1093-1237`), and each card's face is a baked skin image
//! (`BakedSkinButton.vue:27-45`). This launcher has no renderer and no skin service, so the
//! page reads PNGs from `<platform data root>/skins/` (see `crate::skins`) and draws the one
//! face every skin format agrees on: the head's front at atlas `(8,8,8,8)` and its overlay at
//! `(40,8,8,8)`, scaled into the card's picture plate and into the preview column. The grid
//! falls back to the eight sample rows only while the folder is empty; the moment a file is
//! there, the saved skins are the section's content. There is still no 3D renderer: the
//! preview draws the composed front view at the reference's size, and the hint under it is
//! the reference's own Drag to rotate line (see [`draw_preview`]).
//!
//! [`draw`] returns `false`: nothing on this page animates, and the search plate is a
//! painted stand-in for the widget another agent owns (see [`search_plate`]).

use std::path::PathBuf;

use egui::{self, Align2, Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};

use crate::fonts;
use crate::icons::{self, Glyph};
use crate::skins::SavedSkin;
use crate::theme::Theme;

// ---------------------------------------------------------------------------
// Page and toolbar geometry
// ---------------------------------------------------------------------------

/// The page's inset: `p-6` on the library's route container
/// (`app-frontend/src/pages/Index.vue:96`) = 1.5rem, which is
/// `geometry::CONTENT_INSET` and every other page's inset. The skins route itself pads
/// `p-4` (`Skins.vue:1085`); the module docs say why this page takes the 24 anyway.
const PAGE_PADDING: f32 = 24.0;

/// The gap between the heading, the toolbar and the section header: `gap-3`
/// (`ui/library/index.vue:267`), i.e. `--gap-md` 0.75rem (`variables.scss:197`).
const SECTION_GAP: f32 = 12.0;

/// The heading's size: `text-2xl` (`Skins.vue:1087`) = 1.5rem at the 16px root.
const HEADING_SIZE: f32 = 24.0;

/// The heading's line box: `text-2xl` resolves to a 2rem line, the same size the library's
/// heading uses (`leading-8` in `WelcomeScreen.vue:95`).
const HEADING_LINE: f32 = 32.0;

/// The height of every toolbar control. `Combobox`'s trigger and the search field are both
/// `h-9` at `md`/`standard` (`ui/src/components/base/ButtonFrame.vue:31`,
/// `InputFrame.vue:47`), and the library's toolbar uses the same 36.
const CONTROL_HEIGHT: f32 = 36.0;

/// The gap between two controls in the row: `gap-2` (`ui/library/library-toolbar/index.vue:30`),
/// i.e. `--gap-sm` 0.5rem (`variables.scss:196`).
const CONTROL_GAP: f32 = 8.0;

/// The search plate's width when the row has room for it. The reference has no search field
/// on this page, so there is no source width to copy.
// guess: re-measure once the interactive field lands and has a designed width.
const SEARCH_WIDTH: f32 = 320.0;

/// The narrowest the search plate may be squeezed to, so the tabs and the Upload button
/// win the row rather than the plate eating them.
// guess: re-measure -- chosen to keep the magnifier and a few characters legible.
const SEARCH_MIN_WIDTH: f32 = 120.0;

/// The plate's corner: `--radius-sm` (`variables.scss:202`), which is the radius the crate's
/// own search field paints (`widgets/search.rs:181`).
const SEARCH_PLATE_RADIUS: u8 = 8;

/// The magnifier's box: 12 points, centred 18 in from the plate's left edge
/// (`widgets/search.rs:185-193`), which is the same glyph the shell's search field draws.
const SEARCH_GLYPH: f32 = 12.0;

/// How far in from the plate's left edge the magnifier's centre sits
/// (`widgets/search.rs:188-192`): 18 for the glyph's centre, 6 for its radius, 10 of gap.
const SEARCH_GLYPH_INSET: f32 = 18.0;

/// The placeholder's left inset (`widgets/search.rs:195-197`).
const SEARCH_TEXT_INSET: f32 = 34.0;

/// The placeholder's size: the search field's own `fonts::body(13.0)`
/// (`widgets/search.rs:218`).
const SEARCH_TEXT_SIZE: f32 = 13.0;

/// A toolbar control's corner: `rounded-xl` on a `md` `ButtonFrame` (`ButtonFrame.vue:31`),
/// i.e. `--radius-md` 12 (`variables.scss:203`) -- the library's selects use the same.
const CONTROL_RADIUS: u8 = 12;

/// A model tab's inset inside the segmented control: 2 points, matching the library's
/// grid/list toggle (`pages/library.rs:467-468`).
const SEGMENT_INSET: f32 = 2.0;

/// A model tab's height inside the 36-point control: 36 - 2 * 2, the library toggle's own
/// 32-point segment (`pages/library.rs:467-468`).
const SEGMENT_HEIGHT: f32 = 32.0;

/// A model tab's corner: the 12-point control radius less the 2-point inset
/// (`pages/library.rs:469`).
const SEGMENT_RADIUS: u8 = 10;

/// A model tab's horizontal padding.
// guess: re-measure -- the reference has no model tabs on this page to measure.
const SEGMENT_PADDING: f32 = 12.0;

/// A model tab's label size: `text-sm` at semibold, the weight the library's selects use
/// (`pages/library.rs:413`).
const SEGMENT_TEXT_SIZE: f32 = 14.0;

/// The Upload button's horizontal padding, `px-3` on a `md` `ButtonFrame`
/// (`ButtonFrame.vue:31`).
const BUTTON_PADDING: f32 = 12.0;

/// The Upload button's glyph: `[&>svg]:size-5` on a `md` `ButtonFrame`
/// (`ButtonFrame.vue:31`), read as 18 here so the 36-point control keeps 9 points above and
/// below it.
const BUTTON_GLYPH: f32 = 18.0;

/// The gap between the Upload button's glyph and its label: `gap-1.5` (`ButtonFrame.vue:31`).
const BUTTON_GAP: f32 = 6.0;

/// The Upload button's label size, `text-sm` semibold like the reference's own buttons
/// (`pages/library.rs:413`).
const BUTTON_TEXT_SIZE: f32 = 14.0;

// ---------------------------------------------------------------------------
// Section header
// ---------------------------------------------------------------------------

/// The section header's line box: `leading-7` (`VirtualSkinSectionList.vue:388`) = 1.75rem.
const SECTION_HEADER_LINE: f32 = 28.0;

/// The section title's size: `text-xl` (`VirtualSkinSectionList.vue:388`).
const SECTION_HEADING_SIZE: f32 = 20.0;

/// The section title's leading glyph: `size-6` on the `DropdownIcon`
/// (`VirtualSkinSectionList.vue:385`).
const SECTION_HEADER_GLYPH: f32 = 24.0;

/// The gap between that glyph and the title: `gap-[6px]`
/// (`VirtualSkinSectionList.vue:374`).
const SECTION_HEADER_GAP: f32 = 6.0;

/// The space between the header and the grid: the accordion's `content-class="pt-2"`
/// (`VirtualSkinSectionList.vue:375`).
const SECTION_CONTENT_SPACING: f32 = 8.0;

// ---------------------------------------------------------------------------
// Grid geometry -- the library's expressions, the skin tile's aspect
// ---------------------------------------------------------------------------

/// The grid gap: `gap = remSize * 0.75` with `remSize` 16
/// (`instance-group/index.vue:121-122`), which is also this page's `SKIN_GRID_GAP` of 12
/// (`VirtualSkinSectionList.vue:49`). Both sources state 12.
const GRID_GAP: f32 = 12.0;

/// The minimum card width at or above the 1280 breakpoint: `remSize * 10`
/// (`instance-group/index.vue:124`, the `windowWidth < 1280 ? 8 : 10` ternary).
const MIN_CARD_AT_1440: f32 = 160.0;

/// The minimum card width below 1280: `remSize * 8` (`instance-group/index.vue:124`).
const MIN_CARD_BELOW_1280: f32 = 128.0;

/// The breakpoint in that ternary: the grid narrows its minimum at this window width
/// (`instance-group/index.vue:124`).
const NARROW_BREAKPOINT: f32 = 1280.0;

/// The skin tile's width in its own aspect ratio (`VirtualSkinSectionList.vue:47`).
const SKIN_ASPECT_WIDTH: f32 = 31.0;

/// The skin tile's height in its own aspect ratio (`VirtualSkinSectionList.vue:48`).
const SKIN_ASPECT_HEIGHT: f32 = 40.0;

/// The grid's minimum height: `min-h-[45px]` (`instance-group/index.vue:611`), repeated in
/// that file's `Math.max(45, ...)` height calculation (`:135-142`).
const GRID_MIN_HEIGHT: f32 = 45.0;

/// The card's radius: `rounded-[20px]` (`Skins.vue:428`, `SkinButton.vue:156`), which is
/// `--radius-xl` (`variables.scss:205`).
const CARD_RADIUS: u8 = 20;

/// The card's padding: the reference insets its card contents by `p-3`/`px-3`/`bottom-3`
/// (`SkinButton.vue:120`), i.e. `--gap-md` (`variables.scss:197`).
const CARD_PADDING: f32 = 12.0;

/// The gap between the card's name, author and tag rows: `gap-1` on the library card's text
/// column (`instance-card-view.vue:74`).
const CARD_TEXT_GAP: f32 = 4.0;

/// The name's line box: `leading-5` (`instance-card-view.vue:80`) = 20 points.
const NAME_LINE: f32 = 20.0;

/// The name's size: the library card's 17-point title (`pages/library.rs:612`), which is
/// one point above the source's `text-base`.
const NAME_SIZE: f32 = 17.0;

/// The author's line box: `leading-[18px]` on the library card's meta row
/// (`instance-card-view.vue:87`).
const AUTHOR_LINE: f32 = 18.0;

/// The author's size: the library card's 13-point meta text (`pages/library.rs:629`).
const AUTHOR_SIZE: f32 = 13.0;

/// The model tag's height. The reference's `TagItem` is `px-2 py-1 text-sm`
/// (`instance-group/index.vue:571-576`), which is 28 points; this tag is deliberately
/// smaller so it stays a tag on a 180-point card.
// guess: re-measure against a capture once the tiles carry artwork.
const TAG_HEIGHT: f32 = 20.0;

/// The model tag's horizontal padding (`px-2` on the same `TagItem`).
const TAG_PADDING: f32 = 8.0;

/// The model tag's text size: `text-xs` (`--font-size-xs`, `variables.scss` / `defaults.scss:24`).
const TAG_TEXT_SIZE: f32 = 12.0;

/// The model tag's corner: half of its 20-point height, i.e. the reference's pill.
const TAG_RADIUS: u8 = 10;

// ---------------------------------------------------------------------------
// The add-skin dropzone tile
// ---------------------------------------------------------------------------

/// The dropzone's glyph: `size-8` on the `PlusIcon` (`VirtualSkinSectionList.vue:439`,
/// `SkinLikeTextButton.vue:85`).
const DROPZONE_GLYPH: f32 = 32.0;

/// The space between that glyph and the text block: `gap-4`
/// (`SkinLikeTextButton.vue:82`).
const DROPZONE_GAP: f32 = 16.0;

/// `Add skin`'s line box: `leading-6` (`SkinLikeTextButton.vue:89`).
const DROPZONE_TITLE_LINE: f32 = 24.0;

/// `Add skin`'s size: `text-base` (`SkinLikeTextButton.vue:89`).
const DROPZONE_TITLE_SIZE: f32 = 16.0;

/// The space between the tile's two lines: `gap-0.5` (`SkinLikeTextButton.vue:88`).
const DROPZONE_TEXT_GAP: f32 = 2.0;

/// `Drag and drop`'s line box: `leading-5` (`SkinLikeTextButton.vue:92`).
const DROPZONE_SUBTITLE_LINE: f32 = 20.0;

/// `Drag and drop`'s size: `text-sm` (`SkinLikeTextButton.vue:92`).
const DROPZONE_SUBTITLE_SIZE: f32 = 14.0;

/// The dropzone's dash length, for the `border-dashed` edge
/// (`SkinLikeTextButton.vue:58`). CSS's default dash is proportional to the border width;
/// this is a visible dash on a 180-point tile.
// guess: re-measure against a capture of the reference's dashed tile.
const DASH_LENGTH: f32 = 6.0;

/// The gap between two dashes.
// guess: re-measure -- CSS gives no number for the default dash gap.
const DASH_GAP: f32 = 5.0;

/// How far past the corner radius the dashes start, so they do not cut the rounded corner.
// guess: re-measure.
const DASH_CORNER_INSET: f32 = 3.0;

// ---------------------------------------------------------------------------
// The no-matches block
// ---------------------------------------------------------------------------

/// The magnifier above the no-matches line: `size-6`, the section header's own glyph size.
const NO_MATCHES_GLYPH: f32 = 24.0;

/// The space between that glyph and the line: `gap-3` (`--gap-md`, `variables.scss:197`).
const NO_MATCHES_GAP: f32 = 12.0;

/// The no-matches line's box: `text-sm` with `leading-5`.
const NO_MATCHES_LINE: f32 = 20.0;

/// The no-matches line's size: `text-sm`.
const NO_MATCHES_SIZE: f32 = 14.0;

// ---------------------------------------------------------------------------
// Strings -- every one copied from the vendored locale
// ---------------------------------------------------------------------------

/// `app.skins.title` -- "Skin selector" (`locales/en-US/index.json:3284-3286`), read through
/// `appMessages.skinSelectorLabel` (`app-frontend/src/utils/app-messages.ts:4-7`) and printed
/// by the page's own `h1` (`Skins.vue:1087-1089`). The rail calls the destination "Skins"
/// (`shell.rs:51`); the page prints the source's word.
const HEADING: &str = "Skin selector";

/// `app.skins.section.saved-skins` -- "Saved skins" (`locales/en-US/index.json:3266-3268`),
/// the title of the first section in the list (`VirtualSkinSectionList.vue:56-60`).
const SECTION_TITLE: &str = "Saved skins";

/// The preview column's strings, all from the catalogue: the button under the preview
/// (`app.skins.preview.edit-button`, :3219-3221), and the account bar at its foot
/// (`app.skins.demo.title` :3153-3155, `.description` :3150-3152, `app.skins.sign-in.button`
/// :3279-3281).
const PREVIEW_EDIT: &str = "Edit skin";
const DEMO_TITLE: &str = "Editing with a demo account";
const DEMO_DESCRIPTION: &str = "Sign in to your Minecraft account to save and apply skins!";
const SIGN_IN_MICROSOFT: &str = "Sign in to Microsoft";

/// "Drag to rotate" -- the hint the reference prints under the model. It is not a locale
/// string: `SkinPreviewRendererImpl.vue:17` carries it as literal English, so this page takes
/// it from there rather than inventing a key.
const DRAG_TO_ROTATE: &str = "Drag to rotate";

// ---------------------------------------------------------------------------
// The route's two columns, and the preview column inside the first of them
// ---------------------------------------------------------------------------

/// The route's own inset: `p-4` on `.skin-layout` (`Skins.vue:1085`) = 1rem. The page's
/// content box is the shell's content column inset by this, not by [`PAGE_PADDING`]: the
/// reference draws its columns from x 80 and its heading from x 88, where a 24-point inset
/// inside the shell's 88-point page rect would put them at 112 and 113.
const ROUTE_PADDING: f32 = 16.0;

/// The sticky wrapper's padding, `p-2` on `Skins.vue:1086` -- the 8 points between the
/// route's box and the heading.
const HEADER_PADDING: f32 = 8.0;

/// The route's columns: `grid-template-columns: minmax(0, 1fr) minmax(0, 2.5fr)` with
/// `gap: 2.5rem` (`Skins.vue:1340-1348`). Kept as two shares so the split is `share /
/// (PREVIEW_SHARE + LIST_SHARE)` of whatever the gap leaves.
const PREVIEW_SHARE: f32 = 1.0;
const LIST_SHARE: f32 = 2.5;
const COLUMN_GAP: f32 = 40.0;

/// `ml-5` on the renderer's wrapper (`Skins.vue:1091`): the model sits 20 points inside the
/// column the heading is 8 inside.
const PREVIEW_INDENT: f32 = 20.0;

/// `mt-4` between the heading and the renderer (`Skins.vue:1091`).
const PREVIEW_GAP: f32 = 16.0;

/// `h-[calc(80vh-1rem)]` on the renderer (`Skins.vue:1091`): 720 - 16 at this window.
const PREVIEW_HEIGHT: f32 = 704.0;

/// Below this column width the preview is dropped rather than squeezed: the reference collapses
/// to one column under 700 points of page (`Skins.vue:1345`), and the model needs room beside
/// the list's cards before it is worth drawing.
const PREVIEW_MIN_WIDTH: f32 = 220.0;

/// Where the character's head-top sits below the renderer's top, and how many points one skin
/// texel is worth. Measured from `.scratch/cap/ref-skins.png`: the character is 157 by 299
/// points with its head at y 290, in a renderer whose box is y 120 to 824 -- and a skin is 32
/// texels from head to foot.
const MODEL_HEAD_TOP: f32 = 170.0;
const MODEL_TEXEL: f32 = 9.34;

/// The "Drag to rotate" row's 24-point line and the "Edit skin" button's 40-point box,
/// measured the same way: y 652 and y 700 against a renderer whose top is 120.
const DRAG_ROW_TOP: f32 = 532.0;
const EDIT_TOP: f32 = 580.0;

/// The glyph both rows carry: `size-5` (`SkinPreviewRendererImpl.vue:15`,
/// `ButtonFrame.vue:38`).
const ROW_GLYPH: f32 = 20.0;

/// `gap-1.5` between the drag row's glyph and its label.
const ROW_GAP: f32 = 6.0;

/// The drag row's label: `text-base font-medium leading-6`
/// (`SkinPreviewRendererImpl.vue:17`).
const ROW_TEXT_SIZE: f32 = 16.0;
const ROW_LINE: f32 = 24.0;

/// The edit button: `type="base" size="lg"` -- `h-10 rounded-[14px] px-4 gap-2 text-base`
/// (`ButtonFrame.vue:38`), with `bg-surface-4` and its `inset 0 0 0 1px var(--surface-5)`
/// ring (`ButtonFrame.vue:172-176`).
const EDIT_HEIGHT: f32 = 40.0;
const EDIT_PADDING: f32 = 16.0;
const EDIT_GAP: f32 = 8.0;
const EDIT_RADIUS: u8 = 14;
const ACTION_TEXT_SIZE: f32 = 16.0;

/// The account bar: `mx-auto w-full max-w-5xl ... rounded-[20px] border border-surface-5
/// bg-surface-3 p-4` in a `p-4 pt-0` sticky wrapper at the page's foot
/// (`Skins.vue:1263-1268`).
const ACCOUNT_BAR_MAX: f32 = 1024.0;
const ACCOUNT_BAR_HEIGHT: f32 = 86.0;
const ACCOUNT_BAR_MARGIN: f32 = 15.0;
const ACCOUNT_BAR_PADDING: f32 = 16.0;
const ACCOUNT_BAR_RADIUS: u8 = 20;

/// `InfoIcon.size-6` and the `gap-3` beside it (`Skins.vue:1269-1270`).
const ACCOUNT_ICON: f32 = 24.0;
const ACCOUNT_GAP: f32 = 12.0;

/// The bar's two lines: `text-lg font-semibold leading-6` over `text-base leading-6`, with
/// `gap-1` between them (`Skins.vue:1272-1278`). Our body face draws its ink 3 points into a
/// 24-point line where the reference's draws it 6.5, so the gap is the source's 4 plus the 4
/// that puts the second line's ink on the capture's (rows 850-864).
const ACCOUNT_TITLE_SIZE: f32 = 18.0;
const ACCOUNT_LINE: f32 = 24.0;
const ACCOUNT_TEXT_GAP: f32 = 8.0;
const ACCOUNT_BODY_SIZE: f32 = 16.0;

/// The sign-in button: a `md` frame -- `h-9 rounded-xl px-2.5 gap-1.5 text-base
/// font-semibold`, `[&>svg]:size-5` (`ButtonFrame.vue:34`), filled with `--button-color`
/// (`:61`).
const SIGN_IN_HEIGHT: f32 = 36.0;
const SIGN_IN_PADDING: f32 = 10.0;
const SIGN_IN_GAP: f32 = 6.0;
const SIGN_IN_RADIUS: u8 = 12;

/// The logo the reference's sign-in button carries (`WindowsIcon`): four squares.
const SIGN_IN_LOGO: f32 = 20.0;

/// Where the list column's first section header starts, relative to the route's top. The
/// source's list column starts at the same row top as the heading and pads itself `pt-2`
/// (`Skins.vue:1241`); this page's header draws its ink 13 points into its box where the
/// reference's draws it 6, so the strip starts 4.5 points lower to land the header's ink on
/// the reference's -- both captures put it at rows 82-97.
const LIST_TOP_OFFSET: f32 = 4.5;

/// `SKIN_SECTION_SPACING` in the source: the 24 points between two sections
/// (`VirtualSkinSectionList.vue:51`).
const SKIN_SECTION_SPACING: f32 = 24.0;

/// The Default skins strip's columns: the reference lays its eight catalogue skins out four to
/// a row (`VirtualSkinSectionList.vue:114-128` at this window's list width).
const DEFAULTS_COLUMNS: usize = 4;

/// `app.skins.section.default-skins` — "Default skins" (`locales/en-US/index.json:3248-3250`),
/// the second section of the reference's right-hand column.
const DEFAULT_SECTION_TITLE: &str = "Default skins";

/// `app.skins.add-button` -- "Add skin" (`locales/en-US/index.json:3131-3133`). The reference
/// prints it on the grid's add tile (`VirtualSkinSectionList.vue:61-64,441`); this page
/// prints it on that tile and, as the brief's `Upload` control, on the toolbar button.
const ADD_SKIN: &str = "Add skin";

/// `app.skins.add-button.drag-and-drop` -- "Drag and drop"
/// (`locales/en-US/index.json:3134-3136`), the add tile's subtitle
/// (`VirtualSkinSectionList.vue:65-68,442`).
const DRAG_AND_DROP: &str = "Drag and drop";

/// `app.skins.modal.arm-style-wide` -- "Wide" (`locales/en-US/index.json:3182-3184`). It is
/// the label the reference puts on its `CLASSIC` variant (`EditSkinModal.vue:41-45`), which
/// is the model this page calls [`Model::Classic`].
const MODEL_CLASSIC_LABEL: &str = "Wide";

/// `app.skins.modal.arm-style-slim` -- "Slim" (`locales/en-US/index.json:3179-3181`).
const MODEL_SLIM_LABEL: &str = "Slim";

/// `app.screenshots.no-results-description` -- "Try a different search or filter."
/// (`locales/en-US/index.json:1568-1570`). The `app.skins.*` block has no empty-state key at
/// all (the page never renders one), so the no-matches state borrows the app's own
/// noun-neutral wording for a filtered grid -- the screenshots page is its other searchable
/// grid and pairs this line with a heading whose noun ("screenshots") does not fit here,
/// which is why only the description is used.
const NO_MATCHES: &str = "Try a different search or filter.";

/// "Search skins..." -- the brief's placeholder for the toolbar's plate. It is **not** a
/// locale string: the reference has no skins search field, and its nearest placeholder keys
/// are `app.library.search.placeholder` / `app.screenshots.search`, both of which read
/// "Search" (`locales/en-US/index.json:1193-1195`, `:1577-1579`). This plate is a painted
/// stand-in for the interactive field a sibling agent owns, so the copy lives here until
/// that field brings its own.
const SEARCH_PLACEHOLDER: &str = "Search skins...";

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

/// Which arm style a skin is drawn for. The variants are the reference's own
/// `SkinModel` literals, `'CLASSIC'` and `'SLIM'` (`EditSkinModal.vue:41`), and their labels
/// are the locale's "Wide" and "Slim" -- Minecraft's classic model is the wide-armed one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Model {
    /// `'CLASSIC'` -- the wide-armed model, and the reference's default.
    Classic,
    /// `'SLIM'` -- the narrow-armed model.
    Slim,
}

impl Model {
    /// Both models, in the order the reference's radio buttons show them
    /// (`EditSkinModal.vue:41`).
    pub const ALL: [Model; 2] = [Model::Classic, Model::Slim];

    /// The model's tab label, from the locale (see the two label strings above).
    pub fn label(self) -> &'static str {
        match self {
            Model::Classic => MODEL_CLASSIC_LABEL,
            Model::Slim => MODEL_SLIM_LABEL,
        }
    }
}

/// The reference opens the editor on `CLASSIC` (`EditSkinModal.vue:249`) and resets to it
/// (`:351`, `:367`), so the page opens on the classic tab. Slim is deliberately *not* the
/// default: the source does not say it is.
impl Default for Model {
    fn default() -> Model {
        Model::Classic
    }
}

/// The page's own state: which model tab is selected, and what has been typed into the
/// search plate.
pub struct State {
    /// The selected model tab.
    pub model: Model,
    /// The search query as typed, untrimmed. The trimming belongs to [`State::query`], once,
    /// at the point of use -- the same rule `widgets/search.rs:254-267` states for the
    /// shell's field.
    pub query: String,
    /// Whether the search field is taking the keyboard.
    pub query_focused: bool,
    /// The sample skin the preview column shows when no saved file is selected.
    pub selected: &'static str,
    /// The saved skin the preview column shows, if the reader picked one.
    ///
    /// A path rather than a name: two folders can hold files with the same stem, and the
    /// selection has to survive the grid being filtered by the query.
    pub selected_file: Option<PathBuf>,
    /// Full-body views of the saved files, composed once and kept: the reference shows a character,
    /// not a face, and composing it every frame would decode a PNG on the UI thread each time.
    body_textures: std::collections::HashMap<PathBuf, egui::TextureHandle>,
    /// The decoded atlas of each saved file, kept for the 3D preview: the renderer samples texels, which a
    /// GPU texture cannot give it back.
    atlases: std::collections::HashMap<PathBuf, Vec<u8>>,
    /// How far the reader has dragged the preview round, in radians.
    pub yaw: f32,
    /// The skins found in the launcher's `skins/` folder, in the order the reader sees them.
    pub files: Vec<SavedSkin>,
    /// Whether the folder has been read at least once. An empty list is re-read on later
    /// draws, so the first skin a reader drops in appears without a restart.
    scanned: bool,
    /// The authored skins' composed views, uploaded once each.
    textures: Vec<Option<egui::TextureHandle>>,
}

impl std::fmt::Debug for State {
    /// A texture handle has no `Debug`, so the cache is reported by how much of it is filled.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("skins::State")
            .field("model", &self.model)
            .field("query", &self.query)
            .field("selected", &self.selected)
            .field("selected_file", &self.selected_file)
            .field("files", &self.files.len())
            .field(
                "textures",
                &self.textures.iter().filter(|slot| slot.is_some()).count(),
            )
            .finish()
    }
}

impl Default for State {
    /// The tab the reference opens on, no query, and the first skin in the preview.
    fn default() -> Self {
        State {
            model: Model::default(),
            query: String::new(),
            query_focused: false,
            selected: SKINS[0].name,
            selected_file: None,
            body_textures: std::collections::HashMap::new(),
            atlases: std::collections::HashMap::new(),
            yaw: 0.0,
            files: Vec::new(),
            scanned: false,
            textures: Vec::new(),
        }
    }
}

impl State {
    /// The query to filter with, or `None` when the field asks for nothing. Whitespace is
    /// not a query: a space left behind after deleting a word should restore the grid, not
    /// empty it.
    pub fn query(&self) -> Option<&str> {
        let trimmed = self.query.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    }
}

// ---------------------------------------------------------------------------
// Sample data
// ---------------------------------------------------------------------------

/// One skin tile's worth of data.
///
/// The reference's `Skin` has a name, a source, a texture, a variant, an optional cape and an
/// equipped flag (`helpers/skins.ts:6-20`) and **no author**; the author line this page draws
/// is this launcher's addition, so the field is filled with plausible filler rather than
/// attributed to anyone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Skin {
    /// The skin's name.
    pub name: &'static str,
    /// Who made it. Not a real attribution -- see the type's note.
    pub author: &'static str,
    /// The arm style it is drawn for.
    pub model: Model,
}

/// Eight sample skins, so a capture shows a grid rather than an empty one.
///
/// Five are wide-armed and three slim, which is what makes the default tab a full row of
/// five at 1440 and the Slim tab a shorter one. No licence, file, download or texture URL is
/// recorded here: the reference's skins are either Mojang's or the player's own upload, and
/// neither is this page's to invent.
/// **The order the grid draws them in**, which is not alphabetical. Measured from the reference's own grid at 1440x900:
/// its first row is Steve, Alex, Ari, Noor. The catalogue in `default_skins.rs` is alphabetical, so the display order comes
/// from the app, not from the file -- this is that order, read off the capture.
pub const GRID_ORDER: [&str; 8] = ["Steve", "Alex", "Ari", "Efe", "Kai", "Makena", "Noor", "Sunny"];

pub const SKINS: [Skin; 8] = [
    Skin {
        name: "Alex",
        author: "fernlight",
        model: Model::Classic,
    },
    Skin {
        name: "Ari",
        author: "ingot",
        model: Model::Classic,
    },
    Skin {
        name: "Efe",
        author: "wickburn",
        model: Model::Classic,
    },
    Skin {
        name: "Kai",
        author: "flurry",
        model: Model::Classic,
    },
    Skin {
        name: "Makena",
        author: "graveljaw",
        model: Model::Classic,
    },
    Skin {
        name: "Noor",
        author: "moonpetal",
        model: Model::Slim,
    },
    Skin {
        name: "Steve",
        author: "voidsalt",
        model: Model::Slim,
    },
    Skin {
        name: "Sunny",
        author: "northlight",
        model: Model::Slim,
    },
];

/// Which empty state the grid shows, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EmptyState {
    /// Nothing uploaded yet and nothing searched for: the source's own state, which is the
    /// add-skin dropzone tile (`VirtualSkinSectionList.vue:426-443`) and no prose.
    NoSkins,
    /// A query that matches none of the tiles on the selected tab.
    NoMatches,
}

/// The tiles to show, in the reference's order: the selected model, then the query matched
/// case-insensitively against the name and the author.
///
/// **This is not a search engine.** It is a substring test over the eight constants above --
/// no index, no other fields, no catalogue, no network -- and it exists so the two empty
/// states are reachable from the UI instead of being dead code. A real one replaces it.
fn visible_skins(model: Model, query: Option<&str>) -> Vec<Skin> {
    SKINS
        .iter()
        .filter(|skin| skin.model == model)
        .filter(|skin| match query {
            None => true,
            Some(query) => {
                contains_ignore_case(skin.name, query) || contains_ignore_case(skin.author, query)
            }
        })
        .copied()
        .collect()
}

/// The saved skins matching the query, in file order.
///
/// **The model tab does not filter these.** A PNG does not record whether its arms are wide
/// or slim -- the model is account metadata in the reference, not a pixel in the texture --
/// so hiding a file that is visibly on disk because the tab says "Slim" would be a worse
/// lie than showing it under both tabs. The sample grid above still follows the tab.
fn visible_saved(files: &[SavedSkin], query: Option<&str>) -> Vec<SavedSkin> {
    files
        .iter()
        .filter(|skin| match query {
            None => true,
            Some(query) => contains_ignore_case(&skin.name, query),
        })
        .cloned()
        .collect()
}

/// Whether `haystack` contains `needle`, ignoring ASCII case. `to_ascii_lowercase` rather
/// than `to_lowercase`: the sample data is ASCII, and the ASCII fold is the one that cannot
/// change a string's length.
fn contains_ignore_case(haystack: &str, needle: &str) -> bool {
    haystack
        .to_ascii_lowercase()
        .contains(needle.to_ascii_lowercase().as_str())
}

/// The empty state the grid is in, given what the field says and how many tiles survived
/// both filters. `None` means the grid has tiles to draw.
///
/// The source has only the first state, and it is not prose: with nothing saved, the
/// Saved-skins section still renders its add tile. The second state does not exist in the
/// reference at all -- see [`NO_MATCHES`] for why the line under it is borrowed rather than
/// invented.
fn empty_state(query: Option<&str>, matches: usize) -> Option<EmptyState> {
    if matches > 0 {
        return None;
    }
    match query {
        None => Some(EmptyState::NoSkins),
        Some(_) => Some(EmptyState::NoMatches),
    }
}

// ---------------------------------------------------------------------------
// The page
// ---------------------------------------------------------------------------

/// Draw the skins page into `rect`. Returns whether another frame is owed: nothing here
/// animates yet, so it is always `false`.
pub fn draw(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    theme: &crate::theme::Theme,
    _images: &mut crate::images::Images,
    state: &mut State,
) -> bool {
    // Every sample skin's view, uploaded on first use and read by name below: the grid
    // closure cannot borrow the state mutably, and a view does not change while the page is
    // up.
    let textures: Vec<(&'static str, Option<egui::TextureHandle>)> = GRID_ORDER
        .iter()
        .map(|name| (*name, skin_texture(ui.ctx(), state, name)))
        .collect();

    // The page owns its rect (`pages/mod.rs`'s seam); paint the page colour first so the
    // grid sits on `--color-bg` even if the caller did not clear the column.
    ui.painter().rect_filled(rect, 0, theme.BG);

    if rect.width() <= 2.0 * PAGE_PADDING || rect.height() <= 2.0 * PAGE_PADDING {
        return false;
    }

    // The folder is read on the first draw, not on every frame: a scan is filesystem I/O on
    // the UI thread, and a list of saved skins is stable for the whole visit. An empty list
    // is re-read on later draws, because that is the reader who drops in their first skin
    // and comes back to find it.
    // shortcut: the folder is scanned on the first draw and while empty, not once it has
    // files, until a file watcher can report a new PNG.
    // A dropped PNG is a new skin: the tile promises "Drag and drop", so the page takes the files
    // the window hands it and rescans when any of them was one.
    let dropped: Vec<std::path::PathBuf> = ui.ctx().input(|input| {
        input
            .raw
            .dropped_files
            .iter()
            .map(|file| file.path().to_path_buf())
            .collect()
    });
    if !dropped.is_empty() {
        let taken = crate::skins::accept_drop(&crate::skins::skins_dir(), &dropped);
        if taken > 0 {
            state.files = crate::skins::list();
            state.scanned = true;
            ui.ctx().request_repaint();
        }
    }

    if !state.scanned || state.files.is_empty() {
        state.files = crate::skins::list();
        state.scanned = true;
    }
    // A selection that named a deleted file falls back to the first one, so the preview
    // always has a face to show while the folder has any.
    if state
        .selected_file
        .as_ref()
        .is_none_or(|path| !state.files.iter().any(|skin| &skin.path == path))
    {
        state.selected_file = state.files.first().map(|skin| skin.path.clone());
    }

    let content = page_box(rect);
    let toolbar_bottom = heading_and_toolbar(ui, content, theme, state);
    if toolbar_bottom >= rect.bottom() - PAGE_PADDING {
        return false;
    }

    // The route's two columns: the preview takes the 1fr, the list the 2.5fr, and the grid
    // resolves inside what is left.
    let column = preview_column_width(content.width());
    let show_preview = column >= PREVIEW_MIN_WIDTH;
    let list_left = if show_preview {
        content.right() - list_column_width(content.width())
    } else {
        content.left() - HEADER_PADDING
    };
    if show_preview {
        let renderer = Rect::from_min_size(
            egui::pos2(
                content.left() + HEADER_PADDING + PREVIEW_INDENT,
                toolbar_bottom + PREVIEW_GAP,
            ),
            egui::vec2(
                column - 2.0 * HEADER_PADDING - PREVIEW_INDENT,
                PREVIEW_HEIGHT,
            ),
        );
        draw_preview(ui, renderer, theme, state);
    }

    let list_top = content.top() + LIST_TOP_OFFSET;
    let grid_top = section_header(ui, list_left, list_top, theme, SECTION_TITLE);
    // The list's box runs to the window's foot: the account bar is sticky over the scrolling
    // list, so the grid passes under it rather than stopping short of it.
    let grid_rect = Rect::from_min_max(
        egui::pos2(list_left, grid_top),
        egui::pos2(content.right(), rect.bottom()),
    );
    if grid_rect.width() < 1.0 || grid_rect.height() < 1.0 {
        return false;
    }

    // `gridWidth` in the source is the measured content box of the grid
    // (`useElementSize`, `instance-group/index.vue:92`), which is the page rect minus its
    // insets here. The second argument is the source's `windowWidth` (`useWindowSize()`,
    // `:93`): the `windowWidth < 1280 ? 8 : 10` ternary picks the minimum before the
    // columns are counted, so passing 948 at 1440 would take the 8rem branch the source
    // never takes there. egui 0.36 calls the window's content rectangle
    // `Context::content_rect`.
    let grid = grid_layout_in(grid_rect.width(), ui.ctx().content_rect().width());
    // Owned: `state.query()` borrows the page's own state, and the texture cache below needs it mutably.
    let query: Option<String> = state.query().map(str::to_owned);
    // **Saved files win the Saved grid.** The eight `SKINS` rows are sample data so a
    // capture taken on a machine with no skins still shows a grid. The moment the folder has
    // a PNG, the real saved skins take the section and the samples stay where they belong,
    // in the Default skins strip under it. The model tab filters the samples only: a file
    // does not record its arm style (see `visible_saved`).
    let using_files = !state.files.is_empty();
    // Owned, so the texture cache below can be filled while this list is alive.
    let visible_files: Vec<SavedSkin> = visible_saved(&state.files, query.as_deref());
    let samples = visible_skins(state.model, query.as_deref());
    // **Only files count as saved.** The eight catalogue skins are sample data for the Default strip, and
    // counting them here made the Saved grid list "Overgrown", "Copper Pattina" and the rest as if the reader had
    // saved them -- the reference, with nothing saved, shows one thing in that section: the add dropzone.
    let visible_count = visible_files.len();

    // The saved skins' 64x64 atlases, through the icon cache: decoded once and sampled
    // nearest, because a skin is pixel art (see `Images::get_file_nearest`). A file that
    // will not decode is `None` and keeps the glyph its card has always drawn.
    // Built before the grid's closure, which borrows `state` read-only: composing a body needs a
    // mutable borrow for its cache.
    let saved_textures: Vec<Option<egui::TextureHandle>> = {
        let mut textures = Vec::with_capacity(visible_files.len());
        for skin in visible_files.iter() {
            textures.push(body_texture(ui.ctx(), &mut state.body_textures, &skin.path));
        }
        textures
    };

    let probing = std::env::var_os("SIFT_SKINS_PROBE").is_some();
    if probing {
        eprintln!(
            "skins: page={rect:?} preview={} grid={grid_rect:?} files={} visible={visible_count} columns={} card={}x{}",
            show_preview, state.files.len(), grid.columns, grid.card_width, grid.card_height
        );
    }

    // Where the Default skins section starts: the bottom of whatever the saved section drew.
    let below_grid;

    match empty_state(query.as_deref(), visible_count) {
        // The source's own state for "nothing saved": the Saved-skins grid's first tile is
        // the add dropzone (`VirtualSkinSectionList.vue:426-443`), drawn where the first
        // card would be. There is no prose because the reference has none.
        Some(EmptyState::NoSkins) => {
            let painter = ui.painter().clone();
            let tile =
                Rect::from_min_size(grid_rect.min, egui::vec2(grid.card_width, grid.card_height));
            dropzone_tile(&painter, tile, theme);
            below_grid = tile.bottom();
        }
        // A filtered grid with no survivors: this state is this launcher's own, since the
        // reference has no search on this page.
        Some(EmptyState::NoMatches) => {
            let painter = ui.painter().clone();
            no_matches(&painter, grid_rect, theme);
            below_grid = grid_rect.top() + GRID_MIN_HEIGHT;
        }
        None => {
            let columns = grid.columns.max(1);
            // The reference's Saved-skins grid holds the add dropzone as its first tile
            // (`VirtualSkinSectionList.vue:426-443`), so the reader's own skins start at the
            // second slot whether the list is empty or not.
            let slots = visible_count + 1;
            let rows = slots.div_ceil(columns);
            let grid_height =
                (rows as f32 * (grid.card_height + GRID_GAP) - GRID_GAP).max(GRID_MIN_HEIGHT);
            below_grid = grid_rect.top() + grid_height;

            // The source scrolls the whole page; this page owns only the column below its
            // section header, so the scroll area starts at the grid.
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(grid_rect));
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(&mut child, |ui| {
                    // Reserve the full grid height so the scroll area knows what is below
                    // the fold, then paint the cards at the source's absolute positions
                    // (`instance-group/index.vue:164-171`).
                    let (area, _) = ui.allocate_exact_size(
                        egui::vec2(grid_rect.width(), grid_height),
                        Sense::hover(),
                    );
                    let painter = ui.painter().clone();
                    for slot in 0..slots {
                        let column = slot % columns;
                        let row = slot / columns;
                        let card = Rect::from_min_size(
                            area.min
                                + egui::vec2(
                                    column as f32 * (grid.card_width + GRID_GAP),
                                    row as f32 * (grid.card_height + GRID_GAP),
                                ),
                            egui::vec2(grid.card_width, grid.card_height),
                        );
                        // Slot zero is the dropzone; the reader's own skins start at the
                        // second slot (`VirtualSkinSectionList.vue:426-443`).
                        let Some(index) = slot.checked_sub(1) else {
                            dropzone_tile(&painter, card, theme);
                            continue;
                        };
                        if using_files {
                            let Some(skin) = visible_files.get(index) else {
                                dropzone_tile(&painter, card, theme);
                                continue;
                            };
                            // The card is the selection control, so it has to be a target:
                            // the reference's `SkinButton` is a button with `aria-pressed`
                            // (`SkinButton.vue:80`), and the preview above is what it
                            // selects into.
                            let id = egui::Id::new(("skin-card", skin.path.clone()));
                            let response = ui.interact(card, id, Sense::click());
                            if response.clicked() {
                                state.selected_file = Some(skin.path.clone());
                                // The preview was drawn before the grid this frame; ask for
                                // the frame that draws the face the reader just chose.
                                ui.ctx().request_repaint();
                            }
                            let selected = state.selected_file.as_ref() == Some(&skin.path);
                            draw_saved_card(
                                &painter,
                                card,
                                theme,
                                skin,
                                saved_textures.get(index).and_then(|texture| texture.as_ref()),
                                selected,
                                response.hovered(),
                            );
                            if probing {
                                eprintln!(
                                    "skins: card slot={slot} name={:?} rect={card:?} face={:?} selected={selected}",
                                    skin.name,
                                    square_in(card_face_rect(card, false))
                                );
                            }
                        } else if let Some(skin) = samples.get(index) {
                            let texture = textures
                                .iter()
                                .find(|(name, _)| *name == skin.name)
                                .and_then(|(_, texture)| texture.as_ref());
                            draw_sample_card(&painter, card, theme, skin, texture);
                        } else {
                            dropzone_tile(&painter, card, theme);
                        }
                    }
                });
        }
    }

    // The Default skins section rides under the Saved skins one, in the same columns: the
    // reference's list is one scroll box holding both sections
    // (`VirtualSkinSectionList.vue:355-380`).
    defaults_section(ui, list_left, below_grid, grid, theme, state);

    // The account bar is the last thing painted: it is `sticky bottom-0 z-20` over the list
    // (`Skins.vue:1263`).
    account_bar(ui.painter(), content, theme);

    false
}

// ---------------------------------------------------------------------------
// Heading and toolbar
// ---------------------------------------------------------------------------

/// Draw the heading and the toolbar row into the route's content box. Returns the y where the
/// heading (or the toolbar) ends.
///
/// The heading is `h1.m-0.text-2xl.font-bold` inside the sticky wrapper's `p-2`
/// (`Skins.vue:1086-1089`) -- note `font-bold`, 700, where the library's heading is
/// `font-semibold` (`library/index.vue:268`), which is why this one is `fonts::title` and that
/// one is `fonts::semibold`.
fn heading_and_toolbar(ui: &mut Ui, content: Rect, theme: &Theme, state: &mut State) -> f32 {
    // An owned clone, not a borrow: `model_tabs` needs `&mut Ui` to take the clicks, and a
    // `&Painter` borrowed from `ui` would still be alive while it does.
    let painter = ui.painter().clone();

    let heading_at = egui::pos2(content.left() + HEADER_PADDING, content.top());
    painter.text(
        heading_at,
        Align2::LEFT_TOP,
        HEADING,
        fonts::title(HEADING_SIZE),
        theme.CONTRAST,
    );

    // The reference's own capture, with no saved skins, has **no controls in the header at all** -- no search
    // field, no Wide/Slim tabs, no "+ Add skin": the only way to add one is the card inside "Saved skins". Its
    // toolbar appears once there is something to search or filter, so this draws it on the same condition.
    if state.files.is_empty() {
        return heading_at.y + HEADING_LINE;
    }
    let row = Rect::from_min_size(
        egui::pos2(heading_at.x, heading_at.y + HEADING_LINE + SECTION_GAP),
        egui::vec2(content.width() - HEADER_PADDING, CONTROL_HEIGHT),
    );

    // The two measured controls first, then whatever is left over for the search plate. The
    // reference has no search field here, so there is no width to copy; a flat 320 would
    // overlap the tabs on a 960-point window.
    let tabs_width = model_tabs_width(&painter);
    let upload_width = upload_button_width(&painter);
    let search_width = SEARCH_WIDTH
        .min((row.width() - tabs_width - upload_width - 2.0 * CONTROL_GAP).max(SEARCH_MIN_WIDTH));

    // **No search field.** The reference's Skin selector has none -- its own 1440x900 capture shows the heading,
    // then Saved skins, with nothing between them -- and this build invented a plate for it. The query state stays
    // for the filter the page may grow, but nothing is drawn for it.
    let _ = search_width;
    if state.query_focused {
        for event in ui.input(|i| i.events.clone()) {
            match event {
                egui::Event::Text(text) => {
                    state.query.push_str(&text);
                    ui.ctx().request_repaint();
                }
                egui::Event::Key { key: egui::Key::Backspace, pressed: true, .. } => {
                    state.query.pop();
                    ui.ctx().request_repaint();
                }
                egui::Event::Key { key: egui::Key::Escape, pressed: true, .. } => {
                    state.query_focused = false;
                }
                _ => {}
            }
        }
    }
    model_tabs(
        ui,
        &painter,
        // The tabs now start at the row's own left edge: with no search plate there is nothing to sit after.
        egui::pos2(row.left(), row.top()),
        theme,
        state,
    );
    upload_button(&painter, row.right(), row.top(), theme);

    row.bottom() + SECTION_GAP
}

/// The search plate: the raised fill, the magnifier, and the placeholder in
/// `--color-secondary`.
///
/// **Only a plate.** The interactive field belongs to `crate::widgets::search::Search`,
/// which draws the same plate, glyph, placeholder and caret and takes the keyboard
/// (`widgets/search.rs:74-252`). That widget is a sibling's work; this function paints the
/// control's resting look so the row reads right in a capture, and the day the field lands
/// its `ui` call replaces this one at the same rect.
fn search_plate(painter: &egui::Painter, rect: Rect, theme: &Theme, text: &str) {
    painter.rect_filled(rect, SEARCH_PLATE_RADIUS, theme.RAISED_BG);
    icons::draw(
        painter,
        Glyph::Search,
        Rect::from_center_size(
            egui::pos2(rect.left() + SEARCH_GLYPH_INSET, rect.center().y),
            Vec2::splat(SEARCH_GLYPH),
        ),
        theme.SECONDARY,
    );

    // Clipped rather than wrapped: this is a one-line control, and a hint that outgrows it
    // should disappear at the plate's edge rather than spill over the tabs beside it.
    let text_rect = Rect::from_min_max(
        egui::pos2(rect.left() + SEARCH_TEXT_INSET, rect.top()),
        egui::pos2(rect.right() - SEARCH_GLYPH_INSET, rect.bottom()),
    );
    let clipped = painter.with_clip_rect(text_rect);
    // The typed query replaces the placeholder once there is one -- the same rule the servers plate
    // follows, so the two fields look alike in every state.
    let shown = if text.is_empty() { SEARCH_PLACEHOLDER } else { text };
    let galley = clipped.layout_no_wrap(
        shown.to_owned(),
        fonts::body(SEARCH_TEXT_SIZE),
        theme.SECONDARY,
    );
    clipped.galley(
        egui::pos2(text_rect.left(), rect.center().y - galley.size().y / 2.0),
        galley,
        theme.SECONDARY,
    );
}

/// The two-segment model control. The selected segment is the brand wash a selected tab
/// uses (`BUTTON_BG_SELECTED`, `--color-brand-highlight`), and a press anywhere on a segment
/// selects that model.
fn model_tabs(
    ui: &mut Ui,
    painter: &egui::Painter,
    min: Pos2,
    theme: &Theme,
    state: &mut State,
) -> Rect {
    let control = Rect::from_min_size(min, egui::vec2(model_tabs_width(painter), CONTROL_HEIGHT));
    painter.rect_filled(control, CONTROL_RADIUS, theme.BUTTON_BG);

    let mut x = control.left() + SEGMENT_INSET;
    for model in Model::ALL {
        let width = tab_width(painter, model);
        let segment = Rect::from_min_size(
            egui::pos2(x, control.top() + SEGMENT_INSET),
            egui::vec2(width, SEGMENT_HEIGHT),
        );
        let selected = state.model == model;
        if selected {
            painter.rect_filled(segment, SEGMENT_RADIUS, theme.BUTTON_BG_SELECTED);
        }
        painter.text(
            segment.center(),
            Align2::CENTER_CENTER,
            model.label(),
            fonts::semibold(SEGMENT_TEXT_SIZE),
            if selected {
                theme.BUTTON_TEXT_SELECTED
            } else {
                theme.BASE
            },
        );

        // The whole segment is the hit target, so a press anywhere on it switches models.
        let response = ui.interact(
            segment,
            egui::Id::new(("skins-model-tab", model.label())),
            Sense::click(),
        );
        if response.clicked() {
            state.model = model;
        }

        x = segment.right();
    }

    control
}

/// The width the model control needs: two segments plus the outer inset either side.
fn model_tabs_width(painter: &egui::Painter) -> f32 {
    let mut width = 2.0 * SEGMENT_INSET;
    for model in Model::ALL {
        width += tab_width(painter, model);
    }
    width
}

/// One model tab's width: its label plus the segment's padding either side.
fn tab_width(painter: &egui::Painter, model: Model) -> f32 {
    text_width(painter, model.label(), fonts::semibold(SEGMENT_TEXT_SIZE)) + 2.0 * SEGMENT_PADDING
}

/// The Upload button, drawn at rest and right-aligned on the row: the reference's brand
/// button (`type="colored" color="brand"`, the same fill the welcome screen's action uses,
/// `WelcomeScreen.vue:106`).
///
/// **Painted, not wired.** Opening a file picker, decoding a PNG and normalising its texture
/// is the reference's `processSkinFileBuffer` (`Skins.vue:973-988`) and is not this file's;
/// the button exists so the toolbar is complete and the seam is visible.
fn upload_button(painter: &egui::Painter, right: f32, top: f32, theme: &Theme) -> Rect {
    let rect = Rect::from_min_size(
        egui::pos2(right - upload_button_width(painter), top),
        egui::vec2(upload_button_width(painter), CONTROL_HEIGHT),
    );
    painter.rect_filled(rect, CONTROL_RADIUS, theme.BRAND);
    icons::draw(
        painter,
        Glyph::Upload,
        Rect::from_center_size(
            egui::pos2(
                rect.left() + BUTTON_PADDING + BUTTON_GLYPH / 2.0,
                rect.center().y,
            ),
            Vec2::splat(BUTTON_GLYPH),
        ),
        theme.ACCENT_CONTRAST,
    );
    painter.text(
        egui::pos2(
            rect.left() + BUTTON_PADDING + BUTTON_GLYPH + BUTTON_GAP,
            rect.center().y,
        ),
        Align2::LEFT_CENTER,
        ADD_SKIN,
        fonts::semibold(BUTTON_TEXT_SIZE),
        theme.ACCENT_CONTRAST,
    );
    rect
}

/// The width the Upload button needs: its padding, glyph, gap and label.
fn upload_button_width(painter: &egui::Painter) -> f32 {
    2.0 * BUTTON_PADDING
        + BUTTON_GLYPH
        + BUTTON_GAP
        + text_width(painter, ADD_SKIN, fonts::semibold(BUTTON_TEXT_SIZE))
}

/// One string's advance, so a control can be sized around its own label. The ink does not
/// affect a galley's width, which is why a placeholder colour is enough.
fn text_width(painter: &egui::Painter, text: &str, font: egui::FontId) -> f32 {
    painter
        .layout_no_wrap(text.to_owned(), font, Color32::WHITE)
        .size()
        .x
}

// ---------------------------------------------------------------------------
// The section header
// ---------------------------------------------------------------------------

/// The Saved-skins header: the accordion's chevron and its title. The list is always open on
/// this page, so the glyph is drawn in its open orientation (`rotate-180` on the
/// `DropdownIcon`, `VirtualSkinSectionList.vue:384-387`). Returns the y where the grid
/// begins.
fn section_header(ui: &Ui, left: f32, top: f32, theme: &Theme, title: &str) -> f32 {
    let painter = ui.painter();
    let glyph = Rect::from_min_size(
        egui::pos2(
            left,
            top + (SECTION_HEADER_LINE - SECTION_HEADER_GLYPH) / 2.0,
        ),
        Vec2::splat(SECTION_HEADER_GLYPH),
    );
    icons::draw(painter, Glyph::ChevronDown, glyph, theme.CONTRAST);
    painter.text(
        egui::pos2(
            glyph.right() + SECTION_HEADER_GAP,
            top + SECTION_HEADER_LINE / 2.0,
        ),
        Align2::LEFT_CENTER,
        title,
        fonts::semibold(SECTION_HEADING_SIZE),
        theme.CONTRAST,
    );
    top + SECTION_HEADER_LINE + SECTION_CONTENT_SPACING
}

/// The Default skins section, under whatever the Saved skins section drew, and the y below
/// its last row.
///
/// The reference's list is one scroll box holding every section (`VirtualSkinSectionList.vue
/// :355-380`): a section is `pt-6` of space, its header, `pt-2` and then its cards, all in the
/// same columns as the section above. The eight catalogue skins are this build's whole
/// catalogue, so they fill two rows of four.
fn defaults_section(
    ui: &mut Ui,
    left: f32,
    above: f32,
    grid: Grid,
    theme: &Theme,
    state: &mut State,
) -> f32 {
    let header = above + SKIN_SECTION_SPACING;
    if std::env::var_os("SIFT_SKINS_PROBE").is_some() {
        eprintln!("skins: defaults above={above} header={header} card={:?}x{:?}", grid.card_width, grid.card_height);
    }
    let cards = section_header(ui, left, header, theme, DEFAULT_SECTION_TITLE);
    let painter = ui.painter().clone();
    for index in 0..SKINS.len() {
        let column = index % DEFAULTS_COLUMNS;
        let row = index / DEFAULTS_COLUMNS;
        let tile = Rect::from_min_size(
            egui::pos2(
                left + column as f32 * (grid.card_width + GRID_GAP),
                cards + row as f32 * (grid.card_height + GRID_GAP),
            ),
            egui::vec2(grid.card_width, grid.card_height),
        );
        default_card(
            &painter,
            tile,
            theme,
            // **The skin itself, not the fallback art.** This drew `art_texture(index % ART.len())` -- four hand-authored
            // atlases -- so the grid showed the same flat pictures whatever the catalogue said, and changing the assets, the
            // composition or the order could not move a pixel. The texture gathered at the top of this function is the one the
            // page means: `skin_texture` for the name this cell holds.
            GRID_ORDER
                .get(index)
                .and_then(|name| skin_texture(ui.ctx(), state, name)),
            index == 0,
        );
    }
    cards + 2.0 * grid.card_height + GRID_GAP
}

/// One of the catalogue's tiles: the card's plate and the character on it, with the check the
/// reference puts on the skin in use (`SkinButton.vue:153-162`).
fn default_card(
    painter: &egui::Painter,
    rect: Rect,
    theme: &Theme,
    texture: Option<egui::TextureHandle>,
    in_use: bool,
) {
    painter.rect_filled(rect, CARD_RADIUS, theme.RAISED_BG);
    if let Some(texture) = texture {
        let height = rect.height() - 20.0;
        painter.image(
            texture.id(),
            Rect::from_center_size(rect.center(), egui::vec2(height / 2.0, height)),
            Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    }
    if in_use {
        // **A light circle with a dark tick.** The reference's own capture shows a pale disc at the card's top-right
        // corner; this drew the brand green, which reads as a different state rather than the same one.
        let badge = egui::pos2(rect.right() - 14.0, rect.top() + 14.0);
        painter.circle_filled(badge, 9.0, theme.CONTRAST);
        icons::draw(
            painter,
            Glyph::Check,
            Rect::from_center_size(badge, Vec2::splat(11.0)),
            theme.BUTTON_BG,
        );
    }
}

// ---------------------------------------------------------------------------
// The grid
// ---------------------------------------------------------------------------

/// The resolved grid: the column count, one card's size, and the constants the identity
/// `2*padding + columns*cardWidth + (columns-1)*gap == pageWidth` is stated with.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Grid {
    columns: usize,
    card_width: f32,
    card_height: f32,
}

/// The source's column count: `Math.max(1, Math.floor((gridWidth + gap) /
/// (minWidth + gap)))` (`instance-group/index.vue:123-126`).
fn column_count(grid_width: f32, min_width: f32) -> usize {
    let columns = ((grid_width + GRID_GAP) / (min_width + GRID_GAP)).floor();
    if columns < 1.0 {
        1
    } else {
        columns as usize
    }
}

/// `remSize * (windowWidth < 1280 ? 8 : 10)` at `remSize` 16
/// (`instance-group/index.vue:124`).
fn min_card_width(window_width: f32) -> f32 {
    if window_width < NARROW_BREAKPOINT {
        MIN_CARD_BELOW_1280
    } else {
        MIN_CARD_AT_1440
    }
}

/// The route's content box: the shell's content column inset by the route's own `p-4`
/// (`Skins.vue:1085`), from the heading's line top to the window's foot.
///
/// `Page::rect()` is the shell's text column (88..1116): it has already taken the 24-point
/// inset the shell's pages pad themselves with. The reference's skins route pads *itself* 16
/// inside the content column, so its columns start at 80 and its heading -- inside the sticky
/// wrapper's `p-2` -- at 88. Taking `rect` and insetting again would put this page's columns at
/// 112 and its heading at 113, 25 points right of the reference's, which is what the capture
/// measured before this box was recomputed.
fn page_box(rect: Rect) -> Rect {
    Rect::from_min_max(
        egui::pos2(
            crate::geometry::CONTENT_LEFT + ROUTE_PADDING,
            rect.top() + PAGE_PADDING,
        ),
        egui::pos2(
            crate::geometry::panel_left() - ROUTE_PADDING,
            rect.bottom(),
        ),
    )
}

/// The preview column's width: its share of the route box once the gap is taken
/// (`Skins.vue:1340-1348`).
fn preview_column_width(page_width: f32) -> f32 {
    (page_width - COLUMN_GAP) * PREVIEW_SHARE / (PREVIEW_SHARE + LIST_SHARE)
}

/// The list column's width: the rest of the route box.
fn list_column_width(page_width: f32) -> f32 {
    (page_width - COLUMN_GAP) * LIST_SHARE / (PREVIEW_SHARE + LIST_SHARE)
}


/// The same arithmetic over a content width that is not the whole page: the preview column
/// takes its share first, and the grid is resolved inside what is left.
fn grid_layout_in(grid_width: f32, window_width: f32) -> Grid {
    let grid_width = grid_width.max(0.0);
    let columns = column_count(grid_width, min_card_width(window_width));
    let card_width = (grid_width - GRID_GAP * (columns.saturating_sub(1)) as f32) / columns as f32;
    Grid {
        columns,
        card_width,
        card_height: card_height(card_width),
    }
}

/// The skin tile's height at a given width: `cardWidth * 40 / 31`
/// (`VirtualSkinSectionList.vue:139-141`).
fn card_height(card_width: f32) -> f32 {
    card_width * SKIN_ASPECT_HEIGHT / SKIN_ASPECT_WIDTH
}

// ---------------------------------------------------------------------------
// The card
// ---------------------------------------------------------------------------

/// One skin as a 64x64 atlas in RGBA, and the view made from it.
///
/// The atlas is only filled where the front view reads it -- the 3D model the reference draws would
/// need all six faces of every part, which is a renderer rather than a blit.
pub const ATLAS: usize = 64;
/// The composed view: a head over a body with an arm either side and two legs.
pub const VIEW_W: usize = 16;
pub const VIEW_H: usize = 32;

/// A rectangle in the 64x64 atlas, in texels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtlasRect {
    /// Left edge, in texels from the atlas's left.
    pub x: usize,
    /// Top edge, in texels from the atlas's top.
    pub y: usize,
    /// Width in texels.
    pub w: usize,
    /// Height in texels.
    pub h: usize,
}

impl AtlasRect {
    /// The rectangle as the normalised UV corner pair `egui::Painter::image` samples with.
    pub fn uv(self) -> Rect {
        let atlas = ATLAS as f32;
        Rect::from_min_max(
            egui::pos2(self.x as f32 / atlas, self.y as f32 / atlas),
            egui::pos2((self.x + self.w) as f32 / atlas, (self.y + self.h) as f32 / atlas),
        )
    }
}

/// The head's front face in a 64x64 skin: the eight by eight texels the face is drawn from.
pub const HEAD_FRONT: AtlasRect = AtlasRect {
    x: 8,
    y: 8,
    w: 8,
    h: 8,
};

/// The head's overlay (the "hat" layer): a second eight by eight face drawn over the first,
/// where a PNG's alpha lets a hood, a mask or hair sit on top of the skin.
pub const HEAD_OVERLAY: AtlasRect = AtlasRect {
    x: 40,
    y: 8,
    w: 8,
    h: 8,
};

/// The six front faces, as (atlas x, atlas y, width, height, view x, view y).
const FACES: [(usize, usize, usize, usize, usize, usize); 6] = [
    (8, 8, 8, 8, 4, 0),      // head
    (20, 20, 8, 12, 4, 8),   // body
    (44, 20, 4, 12, 0, 8),   // right arm
    (36, 52, 4, 12, 12, 8),  // left arm
    (4, 20, 4, 12, 4, 20),   // right leg
    (20, 52, 4, 12, 8, 20),  // left leg
];

/// Compose one atlas's front view. Pixels the faces do not cover stay transparent.
pub fn compose_front(atlas: &[u8]) -> egui::ColorImage {
    let mut pixels = vec![egui::Color32::TRANSPARENT; VIEW_W * VIEW_H];
    for (ax, ay, aw, ah, vx, vy) in FACES {
        for y in 0..ah {
            for x in 0..aw {
                let source = ((ay + y) * ATLAS + (ax + x)) * 4;
                let Some(pixel) = atlas.get(source..source + 4) else {
                    continue;
                };
                pixels[(vy + y) * VIEW_W + (vx + x)] =
                    egui::Color32::from_rgba_unmultiplied(pixel[0], pixel[1], pixel[2], pixel[3]);
            }
        }
    }
    egui::ColorImage {
        size: [VIEW_W, VIEW_H],
        pixels,
        source_size: egui::vec2(VIEW_W as f32, VIEW_H as f32),
    }
}

/// An authored skin: rows of palette letters, painted onto the faces it needs.
///
/// `.` is transparent, and each letter is one colour from the palette beside it. The rows run head,
/// body, arms, legs -- the order the faces are read in -- so a reader can see the character in the
/// art rather than in a coordinate table.
struct Art {
    palette: &'static [(char, [u8; 4])],
    head: &'static [&'static str],
    body: &'static [&'static str],
    arm: &'static [&'static str],
    leg: &'static [&'static str],
}

impl Art {
    /// Paint this art into a blank 64x64 atlas.
    fn atlas(&self) -> Vec<u8> {
        let mut atlas = vec![0_u8; ATLAS * ATLAS * 4];
        let mut paint = |rows: &[&str], ax: usize, ay: usize| {
            for (y, row) in rows.iter().enumerate() {
                for (x, letter) in row.chars().enumerate() {
                    let Some((_, rgba)) = self.palette.iter().find(|(key, _)| *key == letter)
                    else {
                        continue;
                    };
                    let at = ((ay + y) * ATLAS + (ax + x)) * 4;
                    if let Some(slot) = atlas.get_mut(at..at + 4) {
                        slot.copy_from_slice(rgba);
                    }
                }
            }
        };
        // The six faces, in `FACES` order.
        paint(self.head, 8, 8);
        paint(self.body, 20, 20);
        paint(self.arm, 44, 20);
        paint(self.arm, 36, 52);
        paint(self.leg, 4, 20);
        paint(self.leg, 20, 52);
        atlas
    }
}

/// The skins this build ships, authored here: a green-clad one and a warm one.
const ART: [Art; 4] = [
    Art {
        palette: &[
            ('.', [0, 0, 0, 0]),
            ('h', [0x3a, 0x2a, 0x1c, 255]), // hair
            ('s', [0xd9, 0xa3, 0x7c, 255]), // skin
            ('e', [0x1b, 0x1b, 0x1b, 255]), // eyes
            ('c', [0x1f, 0x9d, 0x55, 255]), // shirt
            ('p', [0x27, 0x3a, 0x5c, 255]), // trousers
            ('b', [0x11, 0x11, 0x11, 255]), // boots
        ],
        head: &[
            "hhhhhhhh", "hhhhhhhh", "hssssssh", "ssssssss", "sesssses", "ssssssss", "ssssssss",
            "ssssssss",
        ],
        body: &[
            "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc",
            "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc",
        ],
        arm: &["cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "ssss",
               "ssss", "ssss"],
        leg: &["pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "bbbb",
               "bbbb", "bbbb"],
    },
    Art {
        palette: &[
            ('.', [0, 0, 0, 0]),
            ('h', [0xb8, 0x5c, 0x1e, 255]), // ginger hair
            ('s', [0xf0, 0xc8, 0xa8, 255]),
            ('e', [0x1b, 0x1b, 0x1b, 255]),
            ('c', [0xe8, 0x7a, 0x2f, 255]), // orange shirt
            ('p', [0x33, 0x4a, 0x38, 255]),
            ('b', [0x1a, 0x1a, 0x1a, 255]),
        ],
        head: &[
            "hhhhhhhh", "hhhhhhhh", "hssssssh", "ssssssss", "sesssses", "ssssssss", "ssssssss",
            "ssssssss",
        ],
        body: &[
            "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc",
            "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc",
        ],
        arm: &["cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "ssss",
               "ssss", "ssss"],
        leg: &["pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "bbbb",
               "bbbb", "bbbb"],
    },
    Art {
        palette: &[
            ('.', [0, 0, 0, 0]),
            ('h', [0x2b, 0x2b, 0x38, 255]),
            ('s', [0xd9, 0xa3, 0x7c, 255]),
            ('e', [0x1b, 0x1b, 0x1b, 255]),
            ('c', [0x2f, 0x5f, 0xc8, 255]),
            ('p', [0x22, 0x22, 0x2c, 255]),
            ('b', [0x11, 0x11, 0x11, 255]),
        ],
        head: &[
            "hhhhhhhh", "hhhhhhhh", "hssssssh", "ssssssss", "sesssses", "ssssssss", "ssssssss",
            "ssssssss",
        ],
        body: &[
            "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc",
            "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc",
        ],
        arm: &["cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "ssss",
               "ssss", "ssss"],
        leg: &["pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "bbbb",
               "bbbb", "bbbb"],
    },
    Art {
        palette: &[
            ('.', [0, 0, 0, 0]),
            ('h', [0x5a, 0x2a, 0x6a, 255]),
            ('s', [0xf0, 0xc8, 0xa8, 255]),
            ('e', [0x1b, 0x1b, 0x1b, 255]),
            ('c', [0x8a, 0x3f, 0xa8, 255]),
            ('p', [0x2d, 0x2d, 0x3a, 255]),
            ('b', [0x1a, 0x1a, 0x1a, 255]),
        ],
        head: &[
            "hhhhhhhh", "hhhhhhhh", "hssssssh", "ssssssss", "sesssses", "ssssssss", "ssssssss",
            "ssssssss",
        ],
        body: &[
            "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc",
            "cccccccc", "cccccccc", "cccccccc", "cccccccc", "cccccccc",
        ],
        arm: &["cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "cccs", "ssss",
               "ssss", "ssss"],
        leg: &["pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "pppp", "bbbb",
               "bbbb", "bbbb"],
    },
];

/// The composed view of the `index`th authored skin, uploaded on first use.
/// The composed view of a baked catalogue skin, uploaded once and read by name.
fn baked_texture(
    ctx: &egui::Context,
    state: &mut State,
    name: &str,
    bytes: &[u8],
) -> Option<egui::TextureHandle> {
    let slot = SKINS.iter().position(|skin| skin.name == name).unwrap_or(0);
    if state.textures.len() < SKINS.len() {
        state.textures.resize_with(SKINS.len(), || None);
    }
    if state.textures[slot].is_none() {
        let decoded = image::load_from_memory(bytes).ok()?.to_rgba8();
        state.textures[slot] = Some(ctx.load_texture(
            format!("skin-baked-{}", slot),
            compose_front(decoded.as_raw()),
            egui::TextureOptions::NEAREST,
        ));
    }
    state.textures[slot].clone()
}

fn art_texture(
    ctx: &egui::Context,
    state: &mut State,
    index: usize,
) -> Option<egui::TextureHandle> {
    if index >= ART.len() {
        return None;
    }
    if state.textures.len() < ART.len() {
        state.textures.resize_with(ART.len(), || None);
    }
    if state.textures[index].is_none() {
        let image = compose_front(&ART[index].atlas());
        state.textures[index] = Some(ctx.load_texture(
            format!("skin-art-{index}"),
            image,
            egui::TextureOptions::NEAREST,
        ));
    }
    state.textures[index].clone()
}

/// The composed view of a named skin, uploaded on first use.
///
/// Filtered nearest, because a skin is pixel art and a smoothed one reads as a different skin.
fn skin_texture(
    ctx: &egui::Context,
    state: &mut State,
    name: &str,
) -> Option<egui::TextureHandle> {
    // **The reference's own texture, by name.** The eight names in `SKINS` are the reference's `Default skins` section, and the
    // baked catalogue carries a 64x64 PNG for each; composing that front view is the same path the authored atlases used, so the
    // drawing is untouched and the change is attributable to the artwork alone.
    if let Some(bytes) = crate::skins_assets::by_name(name) {
        return baked_texture(ctx, state, name, bytes);
    }
    let index = SKINS.iter().position(|skin| skin.name == name)?;
    if index >= ART.len() {
        return None;
    }
    art_texture(ctx, state, index)
}

/// Draw the preview column: the character on the page's own ground, the "Drag to rotate" hint
/// and the Edit skin button under it.
///
/// **A model on the page, not a plate.** The reference's left column is a transparent
/// `TresCanvas` on the route's own background (`Skins.vue:1086-1093`): no raised card, no name
/// line, no filled button. What it holds is the character with its head at y 290, the drag hint
/// at y 652 and a `type="base"` button at y 700 -- all measured from
/// `.scratch/cap/ref-skins.png` against a renderer box of y 120 to 824, which is why the three
/// constants below are offsets into this rect rather than a stack.
///
/// **A figure, from the crate's own data.** The reference draws a rotatable 3D model here
/// (`SkinPreviewRendererImpl.vue:31-78`). This launcher has no graphics backend; the crate does
/// carry a box rasteriser (`crate::skins3d`), but it projects every part about that part's own
/// centre, so a body assembled from it stacks its six boxes on one point instead of standing
/// up. The column therefore draws the composed front view -- the character the cards draw,
/// enlarged to the reference's measurements -- and keeps the rasteriser for an atlas whose
/// composed view is missing.
fn draw_preview(ui: &mut Ui, rect: Rect, theme: &Theme, state: &mut State) {
    // Dragging the column turns the model, which is the reference's own gesture ("Drag to rotate").
    let drag = ui.interact(rect, egui::Id::new("skin-preview-drag"), Sense::drag());
    if drag.dragged() {
        state.yaw = crate::skins3d::yaw_after(state.yaw, drag.drag_delta().x);
        ui.ctx().request_repaint();
    }
    if drag.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
    }

    let painter = ui.painter();
    // The character's own proportions: sixteen texels across, thirty-two down, hung from the
    // top of the head.
    let head = egui::pos2(rect.center().x, rect.top() + MODEL_HEAD_TOP);
    let body = Rect::from_min_size(
        egui::pos2(head.x - VIEW_W as f32 * 0.5 * MODEL_TEXEL, head.y),
        egui::vec2(VIEW_W as f32 * MODEL_TEXEL, VIEW_H as f32 * MODEL_TEXEL),
    );

    if let Some(texture) = preview_texture(ui.ctx(), state) {
        // The composed front view: the whole character in one picture, at the reference's
        // size. It cannot turn, but a turned figure is what `crate::skins3d` cannot draw yet
        // (see this function's note), and the capture is of a character standing still.
        painter.image(
            texture.id(),
            body,
            Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    } else if let Some(atlas) = preview_atlas(state) {
        // No composed view but a readable atlas: the rasteriser still puts the skin's texels
        // on the page, in the boxes it draws them as.
        for texel in crate::skins3d::render(&atlas, head, MODEL_TEXEL, state.yaw) {
            painter.add(egui::Shape::convex_polygon(
                texel.points.to_vec(),
                texel.color,
                egui::Stroke::NONE,
            ));
        }
    } else {
        icons::draw(painter, Glyph::Shirt, body, theme.BASE);
    }
    if std::env::var_os("SIFT_SKINS_PROBE").is_some() {
        eprintln!("skins: preview-head={head:?} body={body:?}");
    }

    drag_hint(painter, rect, theme);
    edit_button(painter, rect, theme);
}

/// The atlas the preview's model is built from: the selected file's if it decodes, otherwise the
/// catalogue sample the reader picked in the Default skins strip.
fn preview_atlas(state: &mut State) -> Option<Vec<u8>> {
    if let Some(path) = state.selected_file.clone() {
        return atlas_of(&mut state.atlases, &path).map(<[u8]>::to_vec);
    }
    let chosen = state.selected;
    SKINS
        .iter()
        .position(|skin| skin.name == chosen)
        .and_then(|index| ART.get(index))
        .map(Art::atlas)
}

/// The composed front view of the selection, for a PNG its atlas read could not decode.
fn preview_texture(ctx: &egui::Context, state: &mut State) -> Option<egui::TextureHandle> {
    if let Some(path) = state.selected_file.clone() {
        return body_texture(ctx, &mut state.body_textures, &path);
    }
    let chosen = state.selected;
    skin_texture(ctx, state, chosen)
}

/// The drag hint: `UnfoldHorizontalIcon` and "Drag to rotate" on one 24-point line
/// (`SkinPreviewRendererImpl.vue:13-20`), centred on the column.
fn drag_hint(painter: &egui::Painter, rect: Rect, theme: &Theme) {
    let font = fonts::label(ROW_TEXT_SIZE);
    let width = ROW_GLYPH + ROW_GAP + text_width(painter, DRAG_TO_ROTATE, font.clone());
    let left = rect.center().x - width / 2.0;
    let top = rect.top() + DRAG_ROW_TOP;
    icons::draw(
        painter,
        Glyph::UnfoldHorizontal,
        Rect::from_min_size(
            egui::pos2(left, top + (ROW_LINE - ROW_GLYPH) / 2.0),
            Vec2::splat(ROW_GLYPH),
        ),
        theme.BASE,
    );
    painter.text(
        egui::pos2(left + ROW_GLYPH + ROW_GAP, top + ROW_LINE / 2.0),
        Align2::LEFT_CENTER,
        DRAG_TO_ROTATE,
        font,
        theme.BASE,
    );
}

/// The edit button: `type="base" size="lg"` -- `h-10 rounded-[14px] px-4 gap-2 text-base`
/// with `bg-surface-4` and the `inset 0 0 0 1px var(--surface-5)` ring its type carries
/// (`ButtonFrame.vue:38,61,172-176`).
fn edit_button(painter: &egui::Painter, rect: Rect, theme: &Theme) {
    let font = fonts::semibold(ACTION_TEXT_SIZE);
    let width = 2.0 * EDIT_PADDING + ROW_GLYPH + EDIT_GAP + text_width(painter, PREVIEW_EDIT, font.clone());
    let button = Rect::from_min_size(
        egui::pos2(rect.center().x - width / 2.0, rect.top() + EDIT_TOP),
        egui::vec2(width, EDIT_HEIGHT),
    );
    painter.rect_filled(button, EDIT_RADIUS, theme.BUTTON_BG);
    painter.rect_stroke(
        button,
        EDIT_RADIUS,
        Stroke::new(crate::theme::STROKE_WIDTH, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );
    let icon = Rect::from_center_size(
        egui::pos2(
            button.left() + EDIT_PADDING + ROW_GLYPH / 2.0,
            button.center().y,
        ),
        Vec2::splat(ROW_GLYPH),
    );
    icons::draw(painter, Glyph::Edit, icon, theme.BASE);
    painter.text(
        egui::pos2(icon.right() + EDIT_GAP, button.center().y),
        Align2::LEFT_CENTER,
        PREVIEW_EDIT,
        font,
        theme.CONTRAST,
    );
}

/// The account bar: "Editing with a demo account" and the sign-in button, sticky at the page's
/// foot and spanning the route's width up to `max-w-5xl` (`Skins.vue:1263-1290`).
///
/// Drawn last, over the list: the reference's bar is `z-20` and the list scrolls under it.
fn account_bar(painter: &egui::Painter, content: Rect, theme: &Theme) {
    let width = content.width().min(ACCOUNT_BAR_MAX);
    let bar = Rect::from_min_size(
        egui::pos2(
            content.center().x - width / 2.0,
            content.bottom() - ACCOUNT_BAR_MARGIN - ACCOUNT_BAR_HEIGHT,
        ),
        egui::vec2(width, ACCOUNT_BAR_HEIGHT),
    );
    painter.rect_filled(bar, ACCOUNT_BAR_RADIUS, theme.RAISED_BG);
    painter.rect_stroke(
        bar,
        ACCOUNT_BAR_RADIUS,
        Stroke::new(crate::theme::STROKE_WIDTH, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );

    // `flex items-start gap-3`: a 24-point info glyph with the two lines beside it.
    let inner = bar.shrink(ACCOUNT_BAR_PADDING);
    icons::draw(
        painter,
        Glyph::Info,
        Rect::from_min_size(inner.min, Vec2::splat(ACCOUNT_ICON)),
        theme.BLUE,
    );
    let text_left = inner.left() + ACCOUNT_ICON + ACCOUNT_GAP;
    painter.text(
        egui::pos2(text_left, inner.top()),
        Align2::LEFT_TOP,
        DEMO_TITLE,
        fonts::semibold(ACCOUNT_TITLE_SIZE),
        theme.CONTRAST,
    );
    painter.text(
        egui::pos2(text_left, inner.top() + ACCOUNT_LINE + ACCOUNT_TEXT_GAP),
        Align2::LEFT_TOP,
        DEMO_DESCRIPTION,
        fonts::body(ACCOUNT_BODY_SIZE),
        theme.BASE,
    );

    // The sign-in button: `type="colored" color="brand"` at the `md` size -- `h-9 rounded-xl
    // px-2.5 gap-1.5`, `bg-[--button-color]`, `text-[var(--color-accent-contrast)]`
    // (`ButtonFrame.vue:34,61`).
    let font = fonts::semibold(ACTION_TEXT_SIZE);
    let width = 2.0 * SIGN_IN_PADDING
        + SIGN_IN_LOGO
        + SIGN_IN_GAP
        + text_width(painter, SIGN_IN_MICROSOFT, font.clone());
    let button = Rect::from_min_size(
        egui::pos2(inner.right() - width, inner.center().y - SIGN_IN_HEIGHT / 2.0),
        egui::vec2(width, SIGN_IN_HEIGHT),
    );
    painter.rect_filled(button, SIGN_IN_RADIUS, theme.BRAND);
    windows_logo(
        painter,
        Rect::from_center_size(
            egui::pos2(
                button.left() + SIGN_IN_PADDING + SIGN_IN_LOGO / 2.0,
                button.center().y,
            ),
            Vec2::splat(SIGN_IN_LOGO),
        ),
        theme.ACCENT_CONTRAST,
    );
    painter.text(
        egui::pos2(
            button.left() + SIGN_IN_PADDING + SIGN_IN_LOGO + SIGN_IN_GAP,
            button.center().y,
        ),
        Align2::LEFT_CENTER,
        SIGN_IN_MICROSOFT,
        font,
        theme.ACCENT_CONTRAST,
    );
}

/// The mark the reference's sign-in button carries (`WindowsIcon`): four squares in a
/// two-by-two block with a two-point gutter.
fn windows_logo(painter: &egui::Painter, rect: Rect, ink: Color32) {
    let square = (rect.width() - 2.0) / 2.0;
    for (column, row) in [(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)] {
        painter.rect_filled(
            Rect::from_min_size(
                egui::pos2(
                    rect.left() + column * (square + 2.0),
                    rect.top() + row * (square + 2.0),
                ),
                Vec2::splat(square),
            ),
            0,
            ink,
        );
    }
}

/// The plate a card leaves for its picture: below the top padding, above the name line.
/// The full-body view of a saved skin, composed from its own atlas and cached.
///
/// A skin file is 64x64; `compose_front` cuts the six front faces out of it and lays them out as a
/// 16x32 character, which is what the reference's preview and its cards show.
fn body_texture(
    ctx: &egui::Context,
    cache: &mut std::collections::HashMap<PathBuf, egui::TextureHandle>,
    path: &std::path::Path,
) -> Option<egui::TextureHandle> {
    if let Some(texture) = cache.get(path) {
        return Some(texture.clone());
    }
    let bytes = std::fs::read(path).ok()?;
    let image = image::load_from_memory(&bytes).ok()?.to_rgba8();
    let (width, height) = image.dimensions();
    let mut atlas = vec![0_u8; ATLAS * ATLAS * 4];
    for y in 0..height.min(ATLAS as u32) {
        for x in 0..width.min(ATLAS as u32) {
            let pixel = image.get_pixel(x, y).0;
            let index = ((y as usize * ATLAS + x as usize) * 4) as usize;
            atlas[index..index + 4].copy_from_slice(&pixel);
        }
    }
    let view = compose_front(&atlas);
    let texture = ctx.load_texture(
        format!("skin-body-{}", path.display()),
        view,
        egui::TextureOptions::NEAREST,
    );
    cache.insert(path.to_path_buf(), texture.clone());
    Some(texture)
}

/// A saved skin's 64x64 RGBA atlas, decoded once and kept.
fn atlas_of<'a>(
    cache: &'a mut std::collections::HashMap<PathBuf, Vec<u8>>,
    path: &std::path::Path,
) -> Option<&'a [u8]> {
    if !cache.contains_key(path) {
        let bytes = std::fs::read(path).ok()?;
        let image = image::load_from_memory(&bytes).ok()?.to_rgba8();
        let (width, height) = image.dimensions();
        let mut atlas = vec![0_u8; ATLAS * ATLAS * 4];
        for y in 0..height.min(ATLAS as u32) {
            for x in 0..width.min(ATLAS as u32) {
                let pixel = image.get_pixel(x, y).0;
                let index = ((y as usize * ATLAS + x as usize) * 4) as usize;
                atlas[index..index + 4].copy_from_slice(&pixel);
            }
        }
        cache.insert(path.to_path_buf(), atlas);
    }
    cache.get(path).map(Vec::as_slice)
}

fn card_face_rect(rect: Rect, has_author: bool) -> Rect {
    let tag_top = rect.bottom() - CARD_PADDING - TAG_HEIGHT;
    let name_top = if has_author {
        tag_top - CARD_TEXT_GAP - AUTHOR_LINE - CARD_TEXT_GAP - NAME_LINE
    } else {
        tag_top - CARD_TEXT_GAP - NAME_LINE
    };
    Rect::from_min_max(
        egui::pos2(rect.left() + CARD_PADDING, rect.top() + CARD_PADDING),
        egui::pos2(rect.right() - CARD_PADDING, name_top - CARD_TEXT_GAP),
    )
}

/// The largest square that fits in `rect`, centred: a face has one side, and the card's
/// picture area is taller than it is wide.
fn square_in(rect: Rect) -> Rect {
    let side = rect.width().min(rect.height()).max(0.0);
    Rect::from_center_size(rect.center(), Vec2::splat(side))
}

/// Draw a 64x64 atlas's head: the front face, then the overlay over it.
///
/// The two rectangles are the skin format's own coordinates -- `(8,8,8,8)` and
/// `(40,8,8,8)` -- and both are scaled into the same square, so the hat layer lands exactly
/// over the face it belongs to. The overlay's transparent texels leave the face beneath
/// showing, which is the whole point of the second layer.
fn draw_face(painter: &egui::Painter, plate: Rect, texture: &egui::TextureHandle) {
    let face = square_in(plate);
    if face.width() < 1.0 {
        return;
    }
    for part in [HEAD_FRONT, HEAD_OVERLAY] {
        painter.image(texture.id(), face, part.uv(), Color32::WHITE);
    }
}

/// Which picture a card's plate carries.
enum CardFace<'a> {
    /// A composed 16x32 front elevation: the authored sample skins.
    View(&'a egui::TextureHandle),
    /// A 64x64 skin atlas: the head's front and overlay are drawn.
    Atlas(&'a egui::TextureHandle),
    /// No texture: the shirt glyph stands in.
    None,
}

/// A sample skin's card: the view, the name, the author and the model.
fn draw_sample_card(
    painter: &egui::Painter,
    rect: Rect,
    theme: &Theme,
    skin: &Skin,
    texture: Option<&egui::TextureHandle>,
) {
    draw_card(
        painter,
        rect,
        theme,
        skin.name,
        Some(skin.author),
        skin.model,
        false,
        false,
        texture.map_or(CardFace::None, CardFace::View),
    );
}

/// A saved skin's card: the face cut from its atlas, its name and its model.
fn draw_saved_card(
    painter: &egui::Painter,
    rect: Rect,
    theme: &Theme,
    skin: &SavedSkin,
    texture: Option<&egui::TextureHandle>,
    selected: bool,
    hovered: bool,
) {
    draw_card(
        painter,
        rect,
        theme,
        &skin.name,
        None,
        Model::Classic,
        selected,
        hovered,
        // A saved file is composed into the same 16x32 front elevation the sample skins use, so its
        // card draws the character rather than a face.
        texture.map_or(CardFace::None, CardFace::View),
    );
}

/// One skin card: a plate with the skin's picture, its name, an optional author and its model.
///
/// Geometry from `SkinButton.vue:153-162` -- the 31/40 box, the 20-point radius and the
/// 1-point edge -- with the contents inset by `CARD_PADDING`. The name, author and tag stack
/// up from the bottom the way the reference's action row sits at `bottom-3`
/// (`SkinButton.vue:118-123`).
#[allow(clippy::too_many_arguments)]
fn draw_card(
    painter: &egui::Painter,
    rect: Rect,
    theme: &Theme,
    name: &str,
    author: Option<&str>,
    model: Model,
    selected: bool,
    hovered: bool,
    face: CardFace<'_>,
) {
    // `hover:bg-surface-4` and the selected edge are the reference's two card states
    // (`SkinButton.vue:55, 173-197`); the press that produces them is the grid's.
    let plate = if hovered {
        theme.SURFACE_4
    } else {
        theme.SUPER_RAISED_BG
    };
    painter.rect_filled(rect, CARD_RADIUS, plate);
    painter.rect_stroke(
        rect,
        CARD_RADIUS,
        if selected {
            Stroke::new(2.0, theme.BRAND)
        } else {
            Stroke::new(crate::theme::STROKE_WIDTH, theme.SURFACE_5)
        },
        egui::StrokeKind::Inside,
    );

    let text_left = rect.left() + CARD_PADDING;
    let content_width = (rect.width() - 2.0 * CARD_PADDING).max(1.0);

    let tag = Rect::from_min_size(
        egui::pos2(text_left, rect.bottom() - CARD_PADDING - TAG_HEIGHT),
        egui::vec2(
            content_width.min(text_width_of(
                painter,
                model.label(),
                fonts::label(TAG_TEXT_SIZE),
                TAG_PADDING,
            )),
            TAG_HEIGHT,
        ),
    );
    let author_top = tag.top() - CARD_TEXT_GAP - AUTHOR_LINE;
    let name_top = match author {
        Some(_) => author_top - CARD_TEXT_GAP - NAME_LINE,
        None => tag.top() - CARD_TEXT_GAP - NAME_LINE,
    };

    let face_rect = card_face_rect(rect, author.is_some());
    if face_rect.width() > 1.0 && face_rect.height() > 1.0 {
        match face {
            // The sample skins' elevation keeps its own 1:2 proportions.
            CardFace::View(texture) => {
                let height = face_rect.height().min(face_rect.width() * 2.0).max(16.0);
                let size = egui::vec2(height / 2.0, height);
                painter.image(
                    texture.id(),
                    Rect::from_center_size(face_rect.center(), size),
                    Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
            // **The face a saved skin's card is about.** Eight texels of head, nearest
            // sampled, scaled into the plate the shirt glyph used to fill.
            CardFace::Atlas(texture) => draw_face(painter, face_rect, texture),
            CardFace::None => {
                let side = (face_rect.height() * 0.5).clamp(16.0, 64.0);
                icons::draw(
                    painter,
                    Glyph::Shirt,
                    Rect::from_center_size(face_rect.center(), Vec2::splat(side)),
                    theme.SECONDARY,
                );
            }
        }
    }

    clipped_line(
        painter,
        Rect::from_min_size(
            egui::pos2(text_left, name_top),
            egui::vec2(content_width, NAME_LINE),
        ),
        name,
        fonts::semibold(NAME_SIZE),
        theme.CONTRAST,
    );
    if let Some(author) = author {
        clipped_line(
            painter,
            Rect::from_min_size(
                egui::pos2(text_left, author_top),
                egui::vec2(content_width, AUTHOR_LINE),
            ),
            author,
            fonts::body(AUTHOR_SIZE),
            theme.SECONDARY,
        );
    }

    // The model tag: the app's pill, filled with the page behind the card so it reads as a
    // cut-out rather than another plate.
    painter.rect_filled(tag, TAG_RADIUS, theme.BG);
    painter.rect_stroke(
        tag,
        TAG_RADIUS,
        Stroke::new(crate::theme::STROKE_WIDTH, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );
    painter.text(
        tag.center(),
        Align2::CENTER_CENTER,
        model.label(),
        fonts::label(TAG_TEXT_SIZE),
        theme.SECONDARY,
    );

    // The reference marks the selected tile with a 24-point contrast disc and a check
    // (`SkinButton.vue:90-96`), over the picture at the top-right corner.
    if selected {
        let badge = egui::pos2(
            rect.right() - CARD_PADDING - 12.0,
            rect.top() + CARD_PADDING + 12.0,
        );
        painter.circle_filled(badge, 12.0, theme.CONTRAST);
        icons::draw(
            painter,
            Glyph::Check,
            Rect::from_center_size(badge, Vec2::splat(15.0)),
            theme.BG,
        );
    }
}

/// A pill's width around its label: the text plus the padding either side.
fn text_width_of(painter: &egui::Painter, text: &str, font: egui::FontId, padding: f32) -> f32 {
    text_width(painter, text, font) + 2.0 * padding
}

/// Draw one line of text, clipped to its own box. A long name ends at the card's edge rather
/// than wrapping into the author line.
fn clipped_line(painter: &egui::Painter, rect: Rect, text: &str, font: egui::FontId, ink: Color32) {
    let galley = painter.layout(text.to_owned(), font, ink, rect.width());
    painter.with_clip_rect(rect).galley(rect.min, galley, ink);
}

// ---------------------------------------------------------------------------
// The empty states
// ---------------------------------------------------------------------------

/// The source's add-skin tile: the `border-dashed border-surface-5 bg-surface-2` plate
/// (`SkinLikeTextButton.vue:58-66`) with the plus glyph, `Add skin` and `Drag and drop`
/// (`:81-95`), at the first cell of the grid.
fn dropzone_tile(painter: &egui::Painter, rect: Rect, theme: &Theme) {
    painter.rect_filled(rect, CARD_RADIUS, theme.SURFACE_2);
    dashed_border(
        painter,
        rect,
        Stroke::new(crate::theme::STROKE_WIDTH, theme.SURFACE_5),
    );

    let stack = DROPZONE_GLYPH
        + DROPZONE_GAP
        + DROPZONE_TITLE_LINE
        + DROPZONE_TEXT_GAP
        + DROPZONE_SUBTITLE_LINE;
    if rect.height() < stack {
        return;
    }
    let mut y = rect.center().y - stack / 2.0;
    icons::draw(
        painter,
        Glyph::Plus,
        Rect::from_center_size(
            egui::pos2(rect.center().x, y + DROPZONE_GLYPH / 2.0),
            Vec2::splat(DROPZONE_GLYPH),
        ),
        theme.CONTRAST,
    );
    y += DROPZONE_GLYPH + DROPZONE_GAP;
    painter.text(
        egui::pos2(rect.center().x, y + DROPZONE_TITLE_LINE / 2.0),
        Align2::CENTER_CENTER,
        ADD_SKIN,
        fonts::semibold(DROPZONE_TITLE_SIZE),
        theme.CONTRAST,
    );
    y += DROPZONE_TITLE_LINE + DROPZONE_TEXT_GAP;
    painter.text(
        egui::pos2(rect.center().x, y + DROPZONE_SUBTITLE_LINE / 2.0),
        Align2::CENTER_CENTER,
        DRAG_AND_DROP,
        fonts::body(DROPZONE_SUBTITLE_SIZE),
        theme.BASE,
    );
}

/// A `border-dashed` edge, four dashes at a time. egui has no dashed stroke on a rounded
/// rectangle, so the dashes are drawn as short segments inset past each corner; the rounded
/// corners themselves are left to the plate's fill.
fn dashed_border(painter: &egui::Painter, rect: Rect, stroke: Stroke) {
    let inset = CARD_RADIUS as f32 + DASH_CORNER_INSET;
    let top = rect.top();
    let bottom = rect.bottom();
    let left = rect.left();
    let right = rect.right();
    dash(
        painter,
        egui::pos2(left + inset, top),
        egui::pos2(right - inset, top),
        stroke,
    );
    dash(
        painter,
        egui::pos2(left + inset, bottom),
        egui::pos2(right - inset, bottom),
        stroke,
    );
    dash(
        painter,
        egui::pos2(left, top + inset),
        egui::pos2(left, bottom - inset),
        stroke,
    );
    dash(
        painter,
        egui::pos2(right, top + inset),
        egui::pos2(right, bottom - inset),
        stroke,
    );
}

/// Draw dashes along one straight edge.
fn dash(painter: &egui::Painter, from: Pos2, to: Pos2, stroke: Stroke) {
    let span = (to - from).length();
    let step = DASH_LENGTH + DASH_GAP;
    if span <= 0.0 || step <= 0.0 {
        return;
    }
    let direction = (to - from) / span;
    let mut along = 0.0;
    while along < span {
        let start = from + direction * along;
        let end = from + direction * (along + DASH_LENGTH).min(span);
        painter.line_segment([start, end], stroke);
        along += step;
    }
}

/// A filtered grid with nothing in it. The line is the app's own wording (see
/// [`NO_MATCHES`]); the magnifier above it is the glyph the search plate already uses, so
/// the state points back at the control that caused it.
fn no_matches(painter: &egui::Painter, rect: Rect, theme: &Theme) {
    let stack = NO_MATCHES_GLYPH + NO_MATCHES_GAP + NO_MATCHES_LINE;
    let mut y = rect.center().y - stack / 2.0;
    icons::draw(
        painter,
        Glyph::Search,
        Rect::from_center_size(
            egui::pos2(rect.center().x, y + NO_MATCHES_GLYPH / 2.0),
            Vec2::splat(NO_MATCHES_GLYPH),
        ),
        theme.SECONDARY,
    );
    y += NO_MATCHES_GLYPH + NO_MATCHES_GAP;
    painter.text(
        egui::pos2(rect.center().x, y + NO_MATCHES_LINE / 2.0),
        Align2::CENTER_CENTER,
        NO_MATCHES,
        fonts::body(NO_MATCHES_SIZE),
        theme.SECONDARY,
    );
}


#[cfg(test)]
mod tests {
    use super::*;

    /// The view is sixteen by thirty-two, and a pixel in it is the pixel the face it came from
    /// holds -- head at the top, body under it, an arm either side, legs at the foot.
    #[test]
    fn the_front_view_is_composed_from_the_faces() {
        let atlas = super::ART[0].atlas();
        let view = super::compose_front(&atlas);
        assert_eq!(view.size, [super::VIEW_W, super::VIEW_H]);

        // The head's top-left corner: atlas (8,8) is the view's (4,0).
        let head = view.pixels[0 * super::VIEW_W + 4];
        assert_ne!(head, egui::Color32::TRANSPARENT, "the head is painted");
        // The body's first pixel: atlas (20,20) is the view's (4,8).
        let body = view.pixels[8 * super::VIEW_W + 4];
        assert_ne!(body, egui::Color32::TRANSPARENT, "the body is painted");
        // A corner no face covers stays transparent.
        assert_eq!(view.pixels[0], egui::Color32::TRANSPARENT, "the corners are empty");
        // The legs reach the last row.
        let foot = view.pixels[(super::VIEW_H - 1) * super::VIEW_W + 4];
        assert_ne!(foot, egui::Color32::TRANSPARENT, "the legs reach the foot");
    }

    /// The route's content box at a window width: the rail and the panel either side of it,
    /// and the route's own `p-4` (`Skins.vue:1085`) inside that.
    fn page_width_at(window_width: f32) -> f32 {
        // The surface's left edge to the sidebar's left edge, less the route's own px-4 on
        // both sides. There is no further gutter: the sidebar is the surface's right
        // column, not a panel with a gap in front of it.
        window_width
            - crate::geometry::RAIL
            - crate::geometry::PANEL
            - 2.0 * ROUTE_PADDING
    }

    /// The route's two columns take the whole content box: the 1fr preview, the 2.5rem gap
    /// and the 2.5fr list are exactly the route's width, and the list still has a real card
    /// in it.
    #[test]
    fn the_preview_column_leaves_the_grid_a_real_width() {
        let page = page_width_at(crate::geometry::WINDOW.0);
        let preview = preview_column_width(page);
        let list = list_column_width(page);
        assert!(list > 0.0, "the grid keeps a positive width");
        assert!(
            (preview + COLUMN_GAP + list - page).abs() < 0.01,
            "column, gap and grid account for the content width exactly"
        );
        assert!(
            (list / preview - LIST_SHARE / PREVIEW_SHARE).abs() < 0.001,
            "the list is two and a half previews, as the source's track sizes say"
        );
        let grid = grid_layout_in(list, 1440.0);
        assert!(
            grid.card_width >= 1.0,
            "a card {} points wide is not a card",
            grid.card_width
        );
    }

    /// Below the breakpoint the column is not drawn, so the grid has the whole page.
    #[test]
    fn a_narrow_page_drops_the_preview_column() {
        let wide = preview_column_width(page_width_at(crate::geometry::WINDOW.0));
        let narrow = preview_column_width(page_width_at(860.0));
        assert!(wide >= PREVIEW_MIN_WIDTH, "1440 keeps the column");
        assert!(narrow < PREVIEW_MIN_WIDTH, "860 does not");
    }

    /// At 1440 the route's box is 1044 points wide: 1fr of it is 286.86 and 2.5fr 717.14,
    /// which is the list the reference's own grid is drawn in (its first card starts at x 407
    /// and its last ends at 1123). The `windowWidth < 1280` test is false, so `minWidth` is
    /// 10rem = 160, not 8rem: `floor((717.14 + 12) / (160 + 12)) = 4` columns of 170.29.
    #[test]
    fn the_grid_tiles_the_page_at_1440() {
        let page = page_width_at(crate::geometry::WINDOW.0);
        assert_eq!(page, 1044.0, "the route's box at 1440");
        assert_eq!(min_card_width(crate::geometry::WINDOW.0), 160.0);

        let list = list_column_width(page);
        assert!((list - 717.142_857).abs() < 0.01, "2.5 / 3.5 of (1044 - 40)");

        let grid = grid_layout_in(list, crate::geometry::WINDOW.0);
        assert_eq!(grid.columns, 4, "floor((717.14 + 12) / (160 + 12))");
        assert!(
            (grid.card_width - 170.285_71).abs() < 0.01,
            "(717.14 - 3 * 12) / 4, got {}",
            grid.card_width
        );

        let used = grid.columns as f32 * grid.card_width
            + (grid.columns as f32 - 1.0) * GRID_GAP;
        assert!(
            (used - list).abs() < 0.001,
            "columns * card + gaps must be the list's width: {used} vs {list}"
        );

        // 170.29 * 40 / 31: the skin tile's own aspect decides the height, not the instance
        // card's `+ 3.375rem`. The reference's default cards are 219.7 points tall.
        assert!(
            (grid.card_height - 219.723).abs() < 0.01,
            "170.29 * 40 / 31, got {}",
            grid.card_height
        );
        assert_eq!(grid.card_height, card_height(grid.card_width));
    }

    /// At 960 the ternary takes the other branch: `minWidth` is 8rem = 128, so
    /// `floor((374.29 + 12) / (128 + 12)) = 2` and the cards are 181.14 wide. The identity
    /// still closes exactly.
    #[test]
    fn the_grid_tiles_the_page_at_960() {
        let page = page_width_at(960.0);
        assert_eq!(page, 564.0, "the same shell furniture at 960");
        assert_eq!(min_card_width(960.0), 128.0);

        let list = list_column_width(page);
        let grid = grid_layout_in(list, 960.0);
        assert_eq!(grid.columns, 2, "floor((374.29 + 12) / (128 + 12))");
        assert!(
            (grid.card_width - 181.142_85).abs() < 0.01,
            "(374.29 - 12) / 2, got {}",
            grid.card_width
        );

        let used = grid.columns as f32 * grid.card_width
            + (grid.columns as f32 - 1.0) * GRID_GAP;
        assert!(
            (used - list).abs() < 0.001,
            "columns * card + gaps must be the list's width: {used} vs {list}"
        );
        assert!(
            (grid.card_height - 233.733).abs() < 0.01,
            "181.14 * 40 / 31, got {}",
            grid.card_height
        );
    }

    /// The ternary is `<`, so the wider minimum starts exactly at 1280 and 1279 keeps the
    /// narrower one. A page that switched at the wrong side would show a column count the
    /// reference never shows.
    #[test]
    fn the_breakpoint_is_the_sources_less_than() {
        assert_eq!(min_card_width(1279.0), MIN_CARD_BELOW_1280);
        assert_eq!(min_card_width(1280.0), MIN_CARD_AT_1440);
    }

    /// The count never drops below one even for a page narrower than a card, and it is a
    /// pure function of the two widths the source's expression names.
    #[test]
    fn a_page_narrower_than_one_card_still_gets_one_column() {
        let grid = grid_layout_in(40.0, 960.0);
        assert_eq!(grid.columns, 1);
        assert!((grid.card_width - 40.0).abs() < 0.001);
    }

    /// The tile's proportion is the source's `31 / 40`, exactly: a 310-point tile is 400
    /// tall, and the height always comes from the width the row was laid out with.
    #[test]
    fn the_skin_tile_keeps_the_sources_aspect() {
        assert_eq!(card_height(310.0), 400.0, "310 * 40 / 31");
        assert!(card_height(1.0) > 0.0);
    }

    /// The reference opens its skin editor on `CLASSIC` (`EditSkinModal.vue:249`), so the
    /// page opens on the classic tab -- not slim.
    #[test]
    fn the_page_opens_on_the_classic_model() {
        let state = State::default();
        assert_eq!(state.model, Model::Classic);
        assert_eq!(Model::default(), Model::Classic);
        assert_eq!(state.query(), None, "an untouched field asks for nothing");
    }

    /// The tabs print the locale's own words, in the reference's own order, and each is a
    /// distinct non-empty label: a tab with no text would be an invisible target.
    #[test]
    fn the_model_tabs_are_labelled_and_ordered() {
        assert_eq!(Model::ALL.map(Model::label), ["Wide", "Slim"]);
        assert_eq!(Model::Classic.label(), MODEL_CLASSIC_LABEL);
        assert_eq!(Model::Slim.label(), MODEL_SLIM_LABEL);
    }

    /// The empty state is chosen by the query, not by the count alone: an empty grid with no
    /// query is the source's add tile, an empty grid with a query is the no-matches line, and
    /// a grid with tiles is neither.
    #[test]
    fn the_empty_state_follows_the_query() {
        assert_eq!(empty_state(None, 0), Some(EmptyState::NoSkins));
        assert_eq!(empty_state(Some("zzz"), 0), Some(EmptyState::NoMatches));
        assert_eq!(empty_state(None, 1), None);
        assert_eq!(empty_state(Some("steve"), 3), None);
    }

    /// The sample grid is eight named, attributed skins -- enough for a capture -- and every
    /// one of them belongs to a model the tabs can show.
    #[test]
    fn the_sample_grid_is_eight_named_skins() {
        assert_eq!(SKINS.len(), 8);
        for skin in SKINS {
            assert!(!skin.name.is_empty(), "every tile needs a name");
            assert!(!skin.author.is_empty(), "every tile needs an author");
            assert!(
                Model::ALL.contains(&skin.model),
                "{} has a model no tab shows",
                skin.name
            );
        }
    }

    /// The model filter splits the sample grid: five wide-armed and three slim, which is why
    /// the default tab fills one 1440 row and the other does not.
    #[test]
    fn the_model_tab_filters_the_sample_grid() {
        assert_eq!(visible_skins(Model::Classic, None).len(), 5);
        assert_eq!(visible_skins(Model::Slim, None).len(), 3);
        assert!(
            visible_skins(Model::Slim, None)
                .iter()
                .all(|skin| skin.model == Model::Slim),
            "the slim tab shows no wide-armed skin"
        );
    }

    /// The query matches the name and the author, ignoring case, and an unmatched query
    /// leaves the grid empty -- which is what puts the no-matches state on screen.
    #[test]
    fn the_query_matches_names_and_authors() {
        // **The eight names are the reference's `Default skins` section**, not invented ones, so the searchable
        // strings here are the catalogue's: `ale` matches `Alex` and nothing else in the classic tab.
        assert_eq!(visible_skins(Model::Classic, Some("ale")).len(), 1);
        assert_eq!(visible_skins(Model::Classic, Some("ALE")).len(), 1);
        assert_eq!(
            visible_skins(Model::Classic, Some("GRAVELJAW")).len(),
            1,
            "the author is searchable too"
        );
        assert_eq!(visible_skins(Model::Classic, Some("zzz")).len(), 0);
        assert_eq!(visible_skins(Model::Slim, Some("sunny")).len(), 1);
        assert_eq!(
            visible_skins(Model::Slim, Some("sunny")),
            vec![SKINS[7]],
            "the match is the skin, not a copy of its fields"
        );
    }

    /// A whitespace-only query is no query, so the grid comes back rather than emptying --
    /// the same rule the shell's search field states (`widgets/search.rs:254-267`).
    #[test]
    fn a_whitespace_query_is_no_query() {
        let blank = State {
            selected: SKINS[0].name,
            query: "   ".to_owned(),
            ..State::default()
        };
        assert_eq!(blank.query(), None);

        let padded = State {
            selected: SKINS[0].name,
            query: "  mossy ".to_owned(),
            ..State::default()
        };
        assert_eq!(padded.query(), Some("mossy"));
        assert_eq!(
            padded.query, "  mossy ",
            "the stored text keeps what was typed"
        );
    }

    /// The two head rectangles are the skin format's own coordinates, expressed as the UV
    /// corners `Painter::image` samples with: an atlas of 64 texels puts the front face at
    /// 0.125..0.25 on both axes and the overlay at 0.625..0.75 across.
    #[test]
    fn the_head_rectangles_map_to_the_atlas_uv() {
        let close = |a: f32, b: f32| (a - b).abs() < 1e-6;
        let front = HEAD_FRONT.uv();
        assert!(close(front.min.x, 0.125), "8 / 64");
        assert!(close(front.min.y, 0.125), "8 / 64");
        assert!(close(front.max.x, 0.25), "16 / 64");
        assert!(close(front.max.y, 0.25), "16 / 64");

        let overlay = HEAD_OVERLAY.uv();
        assert!(close(overlay.min.x, 0.625), "40 / 64");
        assert!(close(overlay.min.y, 0.125), "8 / 64");
        assert!(close(overlay.max.x, 0.75), "48 / 64");
        assert!(close(overlay.max.y, 0.25), "16 / 64");
    }

    /// Both head rectangles are the same eight by eight, inside the atlas, and the overlay is
    /// the second layer over the same face -- not a different part of the skin.
    #[test]
    fn the_overlay_is_the_same_face_at_the_hat_coordinates() {
        assert_eq!((HEAD_FRONT.w, HEAD_FRONT.h), (8, 8));
        assert_eq!((HEAD_OVERLAY.w, HEAD_OVERLAY.h), (8, 8));
        assert_eq!(HEAD_FRONT.w, HEAD_OVERLAY.w);
        assert_eq!(HEAD_FRONT.h, HEAD_OVERLAY.h);
        for rect in [HEAD_FRONT, HEAD_OVERLAY] {
            assert!(rect.x + rect.w <= ATLAS, "head stays inside the atlas");
            assert!(rect.y + rect.h <= ATLAS, "head stays inside the atlas");
        }
        assert_ne!(HEAD_FRONT.x, HEAD_OVERLAY.x, "the overlay is a second layer");
    }

    /// A face is square: in a card's taller-than-wide picture plate the largest square is the
    /// plate's width, centred, so the card never stretches the eight-by-eight head.
    #[test]
    fn a_face_is_the_largest_square_in_its_plate() {
        let tall = Rect::from_min_max(egui::pos2(10.0, 20.0), egui::pos2(110.0, 260.0));
        let face = square_in(tall);
        assert_eq!(face.width(), 100.0);
        assert_eq!(face.height(), 100.0);
        assert_eq!(face.center(), tall.center());

        let wide = Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(300.0, 120.0));
        let face = square_in(wide);
        assert_eq!((face.width(), face.height()), (120.0, 120.0));
        assert_eq!(face.center(), wide.center());
    }

    /// The picture plate ends above the name, and a card with an author line reserves one more
    /// line than a saved skin's card, whose file has no author.
    #[test]
    fn a_saved_card_leaves_the_face_more_room_than_a_sample_card() {
        let card = Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(180.0, 232.26));
        let saved = card_face_rect(card, false);
        let sample = card_face_rect(card, true);
        assert!(saved.height() > sample.height(), "one fewer text line");
        assert!(
            (saved.bottom() - (sample.bottom() + AUTHOR_LINE + CARD_TEXT_GAP)).abs() < 0.001,
            "the name moves up by exactly the author line and its gap"
        );
        assert!(saved.width() > 1.0 && saved.height() > 1.0);
        assert!(saved.top() > card.top(), "the face is inset by the card padding");
        assert!(saved.bottom() < card.bottom(), "the face ends above the tag");
    }

    /// The saved list is filtered by the query but not by the model tab: a PNG does not carry
    /// an arm style, so a file the reader can see is not hidden by the other tab.
    #[test]
    fn the_saved_grid_matches_names_and_ignores_the_model_tab() {
        let files = [
            SavedSkin {
                name: "Overgrown".to_owned(),
                path: PathBuf::from("/skins/Overgrown.png"),
            },
            SavedSkin {
                name: "night-shift".to_owned(),
                path: PathBuf::from("/skins/night-shift.png"),
            },
        ];
        assert_eq!(visible_saved(&files, None).len(), 2);
        assert_eq!(visible_saved(&files, Some("OVER")).len(), 1);
        assert_eq!(visible_saved(&files, Some("night")).len(), 1);
        assert_eq!(visible_saved(&files, Some("zzz")).len(), 0);
        assert_eq!(
            visible_saved(&files, Some("over"))[0],
            files[0].clone(),
            "the match keeps the file it came from"
        );
    }

    /// A real 64x64 PNG is what the page lists, and the head texels a generated one carries
    /// are the texels the UV corners name. This is the source path end to end without a GPU:
    /// write an atlas, list it, decode it, and read one known pixel back through the same
    /// rectangle the painter samples.
    #[test]
    fn a_generated_atlas_lists_and_its_head_pixels_are_where_the_rect_says() {
        let dir = std::env::temp_dir().join(format!("sift-skins-atlas-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        // A 64x64 RGBA atlas with a distinctive head face and a transparent overlay.
        let mut bytes = vec![0_u8; ATLAS * ATLAS * 4];
        for y in 0..8 {
            for x in 0..8 {
                let at = ((8 + y) * ATLAS + 8 + x) * 4;
                bytes[at..at + 4].copy_from_slice(&[255, (x * 32) as u8, (y * 32) as u8, 255]);
            }
        }
        let path = dir.join("Generated.png");
        write_png(&path, ATLAS as u32, ATLAS as u32, &bytes);

        let listed = crate::skins::list_in(&dir);
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].name, "Generated");

        let decoded = image::load_from_memory(&std::fs::read(&path).expect("read"))
            .expect("decode")
            .to_rgba8();
        assert_eq!(decoded.dimensions(), (ATLAS as u32, ATLAS as u32));
        let face = HEAD_FRONT.uv();
        let atlas_x = (face.min.x * ATLAS as f32) as u32;
        let atlas_y = (face.min.y * ATLAS as f32) as u32;
        assert_eq!((atlas_x, atlas_y), (8, 8), "the UV corner is the head's origin");
        let pixel = decoded.get_pixel(atlas_x, atlas_y);
        assert_eq!(pixel.0, [255, 0, 0, 255], "the head's first texel");
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A PNG encoder for the test above: one truecolour-with-alpha image, zlib through
    /// `flate2` (already a dependency), no fixture on disk. Every row is filter 0.
    fn write_png(path: &std::path::Path, width: u32, height: u32, rgba: &[u8]) {
        use std::io::Write;

        fn crc32(bytes: &[u8]) -> u32 {
            let mut crc = 0xffff_ffff_u32;
            for byte in bytes {
                crc ^= u32::from(*byte);
                for _ in 0..8 {
                    crc = if crc & 1 == 1 {
                        0xedb8_8320 ^ (crc >> 1)
                    } else {
                        crc >> 1
                    };
                }
            }
            !crc
        }
        fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
            out.extend_from_slice(&(data.len() as u32).to_be_bytes());
            let mut body = kind.to_vec();
            body.extend_from_slice(data);
            out.extend_from_slice(&body);
            out.extend_from_slice(&crc32(&body).to_be_bytes());
        }

        let mut raw = Vec::with_capacity((width as usize * 4 + 1) * height as usize);
        for row in 0..height as usize {
            raw.push(0);
            raw.extend_from_slice(&rgba[row * width as usize * 4..(row + 1) * width as usize * 4]);
        }
        let mut encoder =
            flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
        encoder.write_all(&raw).expect("deflate");
        let compressed = encoder.finish().expect("deflate finish");

        let mut ihdr = Vec::new();
        ihdr.extend_from_slice(&width.to_be_bytes());
        ihdr.extend_from_slice(&height.to_be_bytes());
        ihdr.extend_from_slice(&[8, 6, 0, 0, 0]);
        let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
        chunk(&mut png, b"IHDR", &ihdr);
        chunk(&mut png, b"IDAT", &compressed);
        chunk(&mut png, b"IEND", &[]);
        std::fs::write(path, png).expect("write png");
    }
}
