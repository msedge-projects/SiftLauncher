//! The screenshots page: the `Screenshots` heading, the toolbar's search field and two
//! selects, and the grouped grid of screenshot cards.
//!
//! Follows `vendor/modrinth-app/app-frontend/src/pages/Screenshots.vue` and the component it
//! renders, `.../components/ui/screenshots-page/`:
//!
//! * `pages/Screenshots.vue` — the route's container is `box-border h-full p-6` (line 26)
//!   and it passes `show-heading`, so the heading is on.
//! * `.../screenshots-page/index.vue` — the page root is `flex h-full w-full flex-col gap-3`
//!   and carries `justify-center` while there is nothing to list. The heading is
//!   `m-0 text-2xl font-bold text-contrast` — `font-bold`, where the library's heading is
//!   `font-semibold` — and heading *and* toolbar exist only when the list is non-empty. The
//!   stored defaults are `sort = 'newest'` and `groupBy = 'date'`.
//! * `.../screenshots-page/toolbar.vue` — two rows: a search `Input` with a `New group`
//!   button, then the sort and group `Combobox`es. **There is no view toggle**: the grid/list
//!   pair lives in the library's own toolbar, and this toolbar does not import it.
//! * `.../screenshots-page/group.vue` — the grid is `grid-cols-1 gap-3 sm:grid-cols-2
//!   2xl:grid-cols-4` with `min-h-[45px]`.
//! * `.../screenshots-page/section.vue` — the group header is an `h-10` row with
//!   `border-b border-surface-5`, a 20-point `DropdownIcon` that turns `rotate-180` while
//!   open, the title at `text-base font-semibold text-primary` lifting to `text-contrast` on
//!   hover, and a `TagItem` count; the grid sits `mt-2.5` under it. The whole header is the
//!   Accordion's open/close target.
//! * `.../screenshots-page/card.vue` — the card is `aspect-video overflow-hidden rounded-xl
//!   border border-surface-5 bg-surface-2` with the thumbnail filling it, `object-cover` and
//!   fading in over 350 ms. The caption is the *hover overlay* at the card's foot:
//!   `inset-x-0 bottom-0 ... bg-gradient-to-t from-surface-1 to-transparent p-3 pt-[120px]`,
//!   the file name at `text-sm font-semibold`, the second line at `text-xs text-secondary` —
//!   the instance name, or the date when the list is one instance's — and the edit/copy/more
//!   `IconButton`s on the right, `translate-y-1 opacity-0` until the card is hovered. The card
//!   itself is `role=button` and activates the viewer; its top-right corner holds a 50-point
//!   hit box for a 24-point selection circle that appears on hover, fills `bg-contrast` and
//!   draws a check when selected, and marks the card `aria-pressed`.
//!
//! **The layout is arithmetic over the source's own constants.** `screenshots-page/index.vue`
//! names `SCREENSHOT_GRID_GAP = 12`, `SCREENSHOT_GROUP_SPACING = 12`,
//! `SCREENSHOT_GROUP_HEADER_HEIGHT = 40`, `SCREENSHOT_GROUP_CONTENT_SPACING = 10`,
//! `SCREENSHOT_GRID_MIN_HEIGHT = 45` and `FALLBACK_SCREENSHOT_CARD_WIDTH = 320`. The column
//! count is `windowWidth >= 1536 ? 4 : windowWidth >= 640 ? 2 : 1`, a card's width is the
//! list's width less the gaps over that count, its height is `width * 9 / 16`, and a group's
//! block is `header + 10 + grid + 12`. Every one of those is a constant below with its line,
//! and [groups], [grid_layout] and [block_height] are the pure functions the tests pin.
//!
//! **The grid at 1440.** The shell's page rect is 1028 points wide (`pages/mod.rs`), so the
//! page's 24-point insets leave 980 for the list. 1440 is below the 1536 breakpoint, so the
//! source shows **two** columns: two 484-point cards and one 12-point gap close the 980
//! exactly, and a card is 272.25 tall (`484 * 9 / 16`). The 1536 window takes the
//! four-column branch.
//!
//! **The grouping is the source's date rule** (`getDateGroup`, `index.vue:980-1009`): a
//! screenshot is bucketed by the day it was taken into `Today`, `Yesterday`, `This week`,
//! `This month` and then the fallback `formatMonth`, i.e. "October 2026". The first four rungs
//! are a function of the *reader's clock*, so [groups] takes `now` as an argument and the page
//! passes the wall clock; a test can then pin every rung without waiting for Tuesday. The
//! source's other grouping modes are `custom`, `instance` and `none`; `instance` and `none`
//! need nothing but the shots already in hand and are built, `custom` needs stored group ids
//! this build does not keep.
//!
//! **The list is the disk.** [read_shots] walks each instance's `screenshots/` folder, one
//! level deep, for `*.png` and `*.jpg`; a file's modification time is its [Stamp], in UTC. A
//! reader with no screenshots gets the empty state, not a sample.
//!
//! **What the page does now, and did not before.** The cards draw their thumbnails from the
//! paths the scan found; the caption is a hover overlay; a card can be selected, singly or with
//! Shift, from its own corner checkbox or by a press while a selection exists; the selection
//! carries a floating bar with its count, Clear and Delete; a right-click (or the caption's
//! more button) opens the card's menu; and a delete is armed by one press and carried out by
//! the second — the pattern the instance page's Screenshots tab and its Files rows use, because
//! it is the same destructive press on the same kind of file and this build already has a
//! settled way to ask.
//!
//! **What this page cannot do yet.**
//!
//! * **Custom groups**, and therefore the toolbar's New group *action*: a group is a stored id
//!   attached to each screenshot (`screenshotGroupsQueryOptions`), and this build keeps no such
//!   store. The group-by select offers the three modes that need none, and the New group button
//!   answers with a toast naming what is missing rather than doing nothing.
//! * **Edit and Copy image** in the card's menu: the first opens `ImageViewerEditor`'s editor
//!   (`index.vue:1439-1478`), the second writes a PNG to the system clipboard through tauri's
//!   clipboard plugin. This build has neither an image editor nor an image clipboard — egui's
//!   clipboard carries text — so the menu carries the rows it can carry out.
//! * **Go to instance**: the reference routes to `/instance/{id}`, and this page's `draw`
//!   returns a `bool` rather than an action (`shell.rs:838-845`), so a page cannot ask the
//!   shell for a route. The row is left out rather than drawn dead.
//! * **Export ZIP**, drag-and-drop between groups, and the viewer's own actions: no archive
//!   writer is wired to a save dialog, and there is no drop target without custom groups.
//! * The empty state's illustration: `NoImagesIllustration` is a 200-point SVG asset this build
//!   does not carry, so the icon set's `Images` glyph stands in on its box.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use egui::{self, Align2, Color32, Rect, Sense, Stroke, Ui, Vec2};

use crate::fonts;
use crate::icons::{self, Glyph};
use crate::images::Images;
use crate::theme::{Theme, STROKE_WIDTH};

// ---------------------------------------------------------------------------
// Page and toolbar geometry
// ---------------------------------------------------------------------------

/// The route container's inset: `p-6` on `pages/Screenshots.vue:26`, i.e. 1.5rem = 24 points
/// at the reference's 16px root (`assets/styles/variables.scss:199`).
const PAGE_PADDING: f32 = 24.0;

/// The gap between the heading, the toolbar and the list: `gap-3` on the page root
/// (`screenshots-page/index.vue:1482`), i.e. `--gap-md` 0.75rem (`variables.scss:197`).
const SECTION_GAP: f32 = 12.0;

/// The heading's size: `text-2xl` (`index.vue:1486`).
const HEADING_SIZE: f32 = 24.0;

/// `text-2xl`'s line height: 2rem.
const HEADING_LINE: f32 = 32.0;

/// A toolbar control's height: `InputFrame`'s `h-9` (`InputFrame.vue:47`) and `ButtonFrame`'s
/// `md: h-9` (`ButtonFrame.vue:31`).
const CONTROL_HEIGHT: f32 = 36.0;

/// `gap-2` between the toolbar's controls (`toolbar.vue:37,50`).
const CONTROL_GAP: f32 = 8.0;

/// `gap-2` between the toolbar's two rows (`toolbar.vue:35-36`).
const TOOLBAR_ROW_GAP: f32 = 8.0;

/// `rounded-xl` on the toolbar's controls (`InputFrame.vue:47`, `ButtonFrame.vue:31`).
const CONTROL_RADIUS: u8 = 12;

/// `min-w-[16rem]` on the search field's wrapper (`toolbar.vue:41`).
const SEARCH_MIN_WIDTH: f32 = 256.0;

/// `px-3` on the input's frame (`InputFrame.vue:47`).
const SEARCH_PADDING: f32 = 12.0;

/// The input's leading glyph: `size-5` (`Input.vue:12`).
const SEARCH_GLYPH: f32 = 20.0;

/// `gap-2` between the input's glyph and its text (`InputFrame.vue:47`).
const SEARCH_INNER_GAP: f32 = 8.0;

/// The input's text: `text-base` (`Input.vue:51`).
const SEARCH_TEXT_SIZE: f32 = 16.0;

/// `px-2.5` on a `md` button (`ButtonFrame.vue:31`).
const BUTTON_PADDING: f32 = 10.0;

/// `gap-1.5` between a button's glyph and its label (`ButtonFrame.vue:31`).
const BUTTON_GAP: f32 = 6.0;

/// `[&>svg]:size-5` on a `md` button (`ButtonFrame.vue:31`).
const BUTTON_GLYPH: f32 = 20.0;

/// `text-base font-semibold` on a button's label (`ButtonFrame.vue:31`).
const BUTTON_TEXT_SIZE: f32 = 16.0;

/// `px-2.5` on a `Combobox`'s trigger, which is a `ButtonFrame` (`Combobox.vue:80`).
// **12, not 10.** Measured on the populated page: the two chips run 88..221 and 230..342 in the reference
// against 88..218 and 227..334 here -- 3 and 5 points narrow, with the gap between them already matching. The
// label measures differ per string (this face is narrower than Inter for both), so padding is the only shared knob.
const SELECT_PADDING: f32 = 12.0;

/// The combobox's prefix and suffix glyphs at `size-5` (`toolbar.vue:60,74`).
const SELECT_GLYPH: f32 = 20.0;

/// `gap-1.5` inside the trigger (`ButtonFrame.vue:31`).
const SELECT_INNER_GAP: f32 = 6.0;

/// `gap-1.5` again, between the label and the chevron (`Combobox.vue:88-93`).
const SELECT_SUFFIX_GAP: f32 = 6.0;

/// The selected label at `text-base font-semibold` (`Combobox.vue:80`).
const SELECT_LABEL_SIZE: f32 = 16.0;

/// A `TagItem`'s text: `text-sm` (`base/TagItem.vue:18`).
const BADGE_TEXT_SIZE: f32 = 14.0;

/// `px-2` on a `TagItem` (`base/TagItem.vue:18`).
const BADGE_PADDING: f32 = 8.0;

/// `py-1` over `leading-none` plus the 1-point rules: 24 points tall (`base/TagItem.vue:18`).
const BADGE_HEIGHT: f32 = 24.0;

/// `rounded-full` on a `TagItem`.
const BADGE_RADIUS: u8 = 12;

// ---------------------------------------------------------------------------
// The grid
// ---------------------------------------------------------------------------

/// `SCREENSHOT_GRID_GAP = 12` (`index.vue:127`), `gap-3` on the grid (`group.vue:210`).
const GRID_GAP: f32 = 12.0;

/// `SCREENSHOT_GROUP_SPACING = 12` (`index.vue:128`).
const GROUP_SPACING: f32 = 12.0;

/// `SCREENSHOT_GROUP_HEADER_HEIGHT = 40` (`index.vue:129`), `h-10` (`section.vue:113`).
const GROUP_HEADER_HEIGHT: f32 = 40.0;

/// `SCREENSHOT_GROUP_CONTENT_SPACING = 10` (`index.vue:130`), `mt-2.5` (`section.vue:164`).
const GROUP_CONTENT_SPACING: f32 = 10.0;

/// `SCREENSHOT_GRID_MIN_HEIGHT = 45` (`index.vue:132`), `min-h-[45px]` (`group.vue:211`).
const GRID_MIN_HEIGHT: f32 = 45.0;

/// `FALLBACK_SCREENSHOT_CARD_WIDTH = 320` (`index.vue:133`).
const FALLBACK_CARD_WIDTH: f32 = 320.0;

/// The four-column breakpoint: `2xl` is 1536 (`index.vue:522-526`).
const WIDE_BREAKPOINT: f32 = 1536.0;

/// The two-column breakpoint: `sm` is 640 (`index.vue:522-526`).
const NARROW_BREAKPOINT: f32 = 640.0;

/// `2xl:grid-cols-4`.
const WIDE_COLUMNS: usize = 4;

/// `sm:grid-cols-2`.
const NARROW_COLUMNS: usize = 2;

/// `grid-cols-1`.
const SLIM_COLUMNS: usize = 1;

/// `aspect-video` (`card.vue:172`).
const CARD_ASPECT: f32 = 9.0 / 16.0;

/// `rounded-xl` on the card (`card.vue:172`).
const CARD_RADIUS: u8 = 12;

/// `p-3` on the caption overlay (`card.vue:233`).
const OVERLAY_PADDING: f32 = 12.0;

/// `gap-2` between the caption's text and its actions (`card.vue:233`).
const OVERLAY_GAP: f32 = 8.0;

/// The height of the caption's scrim: `pt-[120px]` (`card.vue:233`), capped to the card.
const OVERLAY_FADE: f32 = 120.0;

/// The scrim's top band: `from-surface-1 to-transparent` sampled at three heights.
const SCRIM_ALPHA_TOP: u8 = 85;
/// The scrim's middle band.
const SCRIM_ALPHA_MIDDLE: u8 = 170;
/// The scrim's bottom band, solid.
const SCRIM_ALPHA_BOTTOM: u8 = 255;

/// The caption's file name: `text-sm` (`card.vue:236`).
const CAPTION_SIZE: f32 = 14.0;

/// `text-sm`'s line height: 1.25rem (`card.vue:236`).
const CAPTION_LINE: f32 = 20.0;

/// The caption's second line: `text-xs` (`card.vue:239`).
const DATE_SIZE: f32 = 12.0;

/// `text-xs`'s line height: 1rem.
const DATE_LINE: f32 = 16.0;

/// An `IconButton` at its default `md` size: `w-9` = 36 points (`ButtonFrame.vue:39`).
const ACTION_SIZE: f32 = 36.0;

/// `gap-1` between the caption's action buttons (`card.vue:245`).
const ACTION_GAP: f32 = 4.0;

/// The glyph inside an action button (`[&>svg]:size-5`, `ButtonFrame.vue:31`).
const ACTION_GLYPH: f32 = 20.0;

/// The group header's chevron: `size-5` (`section.vue:124`).
const HEADER_GLYPH: f32 = 20.0;

/// `gap-2` between the header's chevron, its title and its count (`section.vue:113`).
const HEADER_GAP: f32 = 8.0;

/// The group title: `text-base` (`section.vue:144`).
const GROUP_TITLE_SIZE: f32 = 16.0;

// ---------------------------------------------------------------------------
// Selection: the checkbox and the floating bar
// ---------------------------------------------------------------------------

/// The selection checkbox's hit box: `size-[50px]` (`card.vue:196`).
const CHECK_HIT: f32 = 50.0;

/// `pt-4` inside that hit box (`card.vue:196`).
const CHECK_TOP_PAD: f32 = 16.0;

/// `right-0.5` = 0.125rem = 2 points (`card.vue:196`).
const CHECK_RIGHT_INSET: f32 = 2.0;

/// The circle itself: `size-6` (`card.vue:206`).
const CHECK_DISC: f32 = 24.0;

/// `border-2` on the empty circle (`card.vue:207`).
const CHECK_BORDER: f32 = 2.0;

/// `CheckIcon size-4` (`card.vue:213`).
const CHECK_GLYPH: f32 = 16.0;

/// The floating bar's plate: `rounded-[20px]` (`FloatingActionBar.vue:238`).
const BAR_RADIUS: u8 = 20;

/// `px-3` on the bar (`FloatingActionBar.vue:238`).
const BAR_PADDING_X: f32 = 12.0;

/// `py-2.5` on the bar (`FloatingActionBar.vue:238`).
const BAR_PADDING_Y: f32 = 10.0;

/// `gap-1.5` between the bar's items (`FloatingActionBar.vue:238`).
const BAR_GAP: f32 = 6.0;

/// The bar's own buttons are `Button`s at `md`: `h-9` (`ButtonFrame.vue:31`).
const BAR_BUTTON_HEIGHT: f32 = 36.0;

/// The bar's divider: `h-6 w-px` (`index.vue:1659,1685`).
const BAR_DIVIDER_HEIGHT: f32 = 24.0;

/// `p-4` between the bar and the page's foot (`FloatingActionBar.vue:238`).
const BAR_MARGIN: f32 = 16.0;

/// The count's `px-4 py-2.5 text-base font-semibold tabular-nums` (`index.vue:1652`).
const BAR_COUNT_SIZE: f32 = 16.0;

/// The margins either side of the bar's divider: §mx-1§ on a §h-6 w-px§ rule
/// (§index.vue:1659,1685§).
const BAR_DIVIDER_MARGIN: f32 = 4.0;

/// The strip a card wears while its delete waits: tall enough for one line of §text-xs§
/// inside the caption it replaces.
const ARMED_STRIP_HEIGHT: f32 = 36.0;

// ---------------------------------------------------------------------------
// The card's menu and the dropdowns
// ---------------------------------------------------------------------------

/// A menu row's height: `ButtonFrame`'s `md` control again, with the sheet's own padding.
const MENU_ROW: f32 = 32.0;

/// `p-1` inside a `ContextMenu`'s plate.
const MENU_PAD: f32 = 4.0;

/// A menu's floor width; `dropdown-min-width="160px"` is the source's (`toolbar.vue:52`).
const MENU_MIN_WIDTH: f32 = 160.0;

/// A menu plate's corner: `rounded-xl`, the sheet's popover radius.
const MENU_RADIUS: u8 = 10;

/// The gap between a trigger and the plate it opens.
const MENU_GAP: f32 = 4.0;

/// A menu row's inner padding.
const MENU_TEXT_PADDING: f32 = 10.0;

/// A menu row's label size.
const MENU_TEXT_SIZE: f32 = 14.0;

/// A menu row's glyph.
const MENU_GLYPH: f32 = 16.0;

/// How long a toast stays: the reference's notifications clear after about two seconds.
const TOAST_SECONDS: f64 = 2.0;

/// The toast's plate padding.
const TOAST_PADDING_X: f32 = 12.0;
/// The toast's plate padding, vertically.
const TOAST_PADDING_Y: f32 = 8.0;

/// The toast's text size: `text-sm`.
const TOAST_TEXT_SIZE: f32 = 14.0;

/// The empty state's illustration box: `h-[200px] w-auto` (`EmptyState.vue:4`).
const ILLUSTRATION_HEIGHT: f32 = 200.0;

/// The stand-in glyph drawn inside that box.
const ILLUSTRATION_GLYPH: f32 = 64.0;

/// `gap-1.5` between the heading and the description (`EmptyState.vue:3`).
const EMPTY_TEXT_GAP: f32 = 6.0;

/// The description's line: `text-base` at `leading-6`.
const EMPTY_BODY_LINE: f32 = 24.0;

/// The description's size (`EmptyState.vue:9`).
const EMPTY_BODY_SIZE: f32 = 16.0;

// ---------------------------------------------------------------------------
// The page's words
// ---------------------------------------------------------------------------

/// `app.screenshots.heading`.
const HEADING: &str = "Screenshots";

/// `app.screenshots.sort.newest`.
const SORT_NEWEST: &str = "Newest";

/// `app.screenshots.sort.oldest`.
const SORT_OLDEST: &str = "Oldest";

/// `app.screenshots.sort.name`.
const SORT_NAME: &str = "Name";

/// `app.screenshots.group.instance`.
const GROUP_INSTANCE: &str = "Instance";

/// `app.screenshots.group.date`.
const GROUP_DATE: &str = "Date";

/// `app.screenshots.group.none`.
const GROUP_NONE: &str = "No grouping";

/// `app.screenshots.group.today`.
const TODAY: &str = "Today";

/// `app.screenshots.group.yesterday`.
const YESTERDAY: &str = "Yesterday";

/// `app.screenshots.group.this-week`.
const THIS_WEEK: &str = "This week";

/// `app.screenshots.group.this-month`.
const THIS_MONTH: &str = "This month";

/// `app.screenshots.group.all-screenshots`.
const ALL_SCREENSHOTS: &str = "All screenshots";

/// `app.screenshots.search`.
const SEARCH_PLACEHOLDER: &str = "Search";

/// `app.screenshots.group.new`.
const NEW_GROUP: &str = "New group";

/// `commonMessages.clearButton`.
const CLEAR: &str = "Clear";

/// `commonMessages.deleteLabel`.
const DELETE: &str = "Delete";

/// `index.vue:1058-1097`'s "Show in folder".
const SHOW_IN_FOLDER: &str = "Show in folder";

/// The menu's first row: the source's "Edit screenshot" opens the viewer, and this build's
/// viewer only views, so the row says what it does.
const OPEN: &str = "Open";

/// The menu's group row. The reference puts a screenshot in a group from this menu; the store that holds the
/// groups is here, so the row is the way in.
const ADD_TO_GROUP: &str = "Add to group";

/// `app.screenshots.delete-success`.
const DELETE_SUCCESS: &str = "Screenshot deleted";

/// `app.screenshots.empty-heading`.
const EMPTY_HEADING: &str = "No screenshots yet";

/// `app.screenshots.empty-description`.
const EMPTY_BODY: &str = "Screenshots you take in-game will appear here.";

/// `app.screenshots.no-results-heading`.
const NO_RESULTS_HEADING: &str = "No matching screenshots";

/// `app.screenshots.no-results-description`.
const NO_RESULTS_BODY: &str = "Try a different search or filter.";

/// `app.screenshots.error-heading`.
const ERROR_HEADING: &str = "Failed to load screenshots";

/// `commonMessages.retryButton`.
const RETRY: &str = "Retry";

/// The month names `Intl.DateTimeFormat` prints for `month: 'long'`.
const MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June", "July", "August", "September",
    "October", "November", "December",
];

/// The short month names for `dateStyle: 'medium'`.
const MONTHS_SHORT: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];


// ---------------------------------------------------------------------------
// The model
// ---------------------------------------------------------------------------

/// The list's order: the `sort` the page stores (`index.vue:149`), whose options are the three
/// `app.screenshots.sort.*` strings (`index.vue:390-394`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
    /// Newest first, the stored default.
    Newest,
    /// Oldest first.
    Oldest,
    /// By file name, ascending — `localeCompare` on the ASCII names the game writes is byte
    /// order (`index.vue:414`).
    Name,
}

impl Default for Sort {
    fn default() -> Sort {
        Sort::Newest
    }
}

impl Sort {
    /// The three sorts, in the order the select lists them (`index.vue:390-394`).
    pub const ALL: [Sort; 3] = [Sort::Newest, Sort::Oldest, Sort::Name];

    /// The label the sort select shows for this value.
    pub fn label(self) -> &'static str {
        match self {
            Sort::Newest => SORT_NEWEST,
            Sort::Oldest => SORT_OLDEST,
            Sort::Name => SORT_NAME,
        }
    }
}

/// What the list is grouped by: the source's `groupBy` (`index.vue:150`, options at
/// `:395-400`).
///
/// `Custom group` is the fourth mode and is not here: it reads a store of group ids that this
/// build does not keep (see the module note). The three below need nothing but the shots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupBy {
    /// One heading per day-bucket or month: the stored default.
    Date,
    /// One of the reader's own groups: the grid shows the screenshots assigned to it, under its own name. The
    /// reference offers these beside Date, Instance and None, which is what the store added this session is for.
    Custom(String),
    /// One heading per instance the screenshot came from.
    Instance,
    /// One list with no heading at all (`index.vue:1510` hides the header, and `group.vue`
    /// draws the grid without one).
    None,
}

impl Default for GroupBy {
    fn default() -> GroupBy {
        GroupBy::Date
    }
}

impl GroupBy {
    /// The modes the select offers, in the source's order without `custom`.
    /// The modes this build offers, before the reader's own groups: the store's are appended by `with_groups`.
    pub const ALL: [GroupBy; 3] = [GroupBy::Instance, GroupBy::Date, GroupBy::None];

    /// The select's list: the three stored modes, then one entry per custom group.
    pub fn with_groups(groups: &[crate::groups::Group]) -> Vec<GroupBy> {
        let mut all = GroupBy::ALL.to_vec();
        for group in groups {
            all.push(GroupBy::Custom(group.id.clone()));
        }
        all
    }

    /// The label the group select shows for this value.
    pub fn label(&self) -> &str {
        match self {
            GroupBy::Date => GROUP_DATE,
            GroupBy::Instance => GROUP_INSTANCE,
            GroupBy::None => GROUP_NONE,
            GroupBy::Custom(_) => "Group",
        }
    }
}

/// One row of a card's menu, in the order the source lists them (`index.vue:1058-1097`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    /// Show the screenshot full-size.
    Open,
    /// Put this screenshot in one of the reader's groups.
    AddToGroup,
    /// Reveal the file in the desktop's file manager.
    ShowInFolder,
    /// Remove the file from the disk.
    Delete,
}

/// The rows a card's menu carries, in order.
///
/// The source's menu is Edit, Copy image (both only for a `contextmenu` open), Show in folder,
/// Go to instance, a divider, Delete. Three of those need machinery this build does not have —
/// an image editor, an image clipboard, and a route out of a page whose `draw` returns a
/// `bool` — so the menu carries every row this page can carry out, and each one does something
/// when pressed.
const MENU_ITEMS: [Item; 4] = [
    Item::Open,
    Item::AddToGroup,
    Item::ShowInFolder,
    Item::Delete,
];

impl Item {
    /// The row's label.
    fn label(self) -> &'static str {
        match self {
            Item::Open => OPEN,
            Item::AddToGroup => ADD_TO_GROUP,
            Item::ShowInFolder => SHOW_IN_FOLDER,
            Item::Delete => DELETE,
        }
    }

    /// The row's glyph.
    fn glyph(self) -> Glyph {
        match self {
            Item::Open => Glyph::Image,
            Item::AddToGroup => Glyph::SquarePlus,
            Item::ShowInFolder => Glyph::FolderOpen,
            Item::Delete => Glyph::Trash,
        }
    }

    /// Whether the row is destructive, which is the source's `tone: 'red'`.
    fn danger(self) -> bool {
        matches!(self, Item::Delete)
    }
}

/// The page's own state: what the toolbar has selected, what the search field holds, the list
/// read from disk, and everything a visit has done to it since.
#[derive(Debug, Clone)]
pub struct State {
    /// The selected order. Defaults to the source's stored `newest`.
    pub sort: Sort,
    /// The reader's own groups and their assignments, read with the screenshots. The select lists them and the grid
    /// can group by one.
    pub groups: crate::groups::Groups,
    /// The group list a card's menu opened, while it is open.
    group_menu: Option<GroupMenu>,
    /// The new-group dialog's name while it is open; `None` closes it.
    pub new_group: Option<String>,
    /// Where the toolbar's "New group" button was drawn this frame. The press is read against this at the end of
    /// the frame with the page's own pointer, because an `interact` registered there loses to the page-wide handlers
    /// the body registers after it -- which is why the button looked dead.
    new_group_button: Option<Rect>,
    /// What the list is grouped by. Defaults to the source's stored `date`.
    pub group_by: GroupBy,
    /// The query the search field holds, as typed.
    pub search: String,
    /// Whether the search field has the keyboard. The field is the page's own: it reads key
    /// events while this is set, the way the servers page's field does.
    search_focused: bool,
    /// What the last scan of the instances folder found.
    shots: Vec<Shot>,
    /// Whether the folder has been read once.
    scanned: bool,
    /// Why the last scan failed, when it did. Drawn as the error state (`index.vue:1506-1517`).
    error: Option<String>,
    /// The cards the reader has selected, by the path that names them.
    selected: BTreeSet<PathBuf>,
    /// The open card menu: which card, and where its top-left corner goes.
    menu: Option<Menu>,
    /// A delete that has been asked for once and waits for its second press.
    armed: Option<Arm>,
    /// Set on the frame the armed control itself took the press, so the arm is not cleared by
    /// the press that consumed it.
    armed_touched: bool,
    /// The sentence a finished action owes, and when it should leave.
    toast: Option<Toast>,
    /// Groups the reader has folded away, by id (`collapsedGroups`, `index.vue`).
    collapsed: BTreeSet<String>,
    /// Whether the sort select's list is open.
    sort_open: bool,
    /// Whether the group select's list is open.
    group_open: bool,
    /// The picture being looked at, as its index in the list the cards came from.
    viewer: Option<usize>,
}

impl Default for State {
    fn default() -> State {
        State {
            sort: Sort::default(),
            groups: crate::groups::Groups::load(&crate::groups::Groups::path(&crate::data::instances_dir())),
            group_menu: None,
            new_group: None,
            new_group_button: None,
            group_by: GroupBy::default(),
            search: String::new(),
            search_focused: false,
            shots: Vec::new(),
            scanned: false,
            error: None,
            selected: BTreeSet::new(),
            menu: None,
            armed: None,
            armed_touched: false,
            toast: None,
            collapsed: BTreeSet::new(),
            sort_open: false,
            group_open: false,
            viewer: None,
        }
    }
}

impl State {
    /// The query to filter with, or `None` when the field asks for nothing.
    ///
    /// Whitespace is not a query, for the reason `widgets::search::Search::query` gives: a
    /// single space is what a reader leaves behind after deleting a word, and treating it as
    /// a query would empty the list instead of restoring it.
    pub fn query(&self) -> Option<&str> {
        let trimmed = self.search.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    }

    /// Whether this card is in the selection.
    fn is_selected(&self, path: &Path) -> bool {
        self.selected.contains(path)
    }

    /// Add the card to the selection, or take it out — `toggleScreenshotSelection`
    /// (`index.vue:1029-1041`), which is a plain set toggle.
    fn toggle(&mut self, path: &Path) {
        if !self.selected.remove(path) {
            self.selected.insert(path.to_owned());
        }
    }

    /// Drop anything the list no longer holds: a selection never outlives the file it names,
    /// and an armed delete never points at a file that has gone.
    fn prune(&mut self) {
        let live: BTreeSet<PathBuf> = self.shots.iter().map(|shot| shot.path.clone()).collect();
        self.selected.retain(|path| live.contains(path));
        if let Some(Arm::Card(path)) = &self.armed {
            if !live.contains(path) {
                self.armed = None;
            }
        }
        if self
            .menu
            .as_ref()
            .is_some_and(|menu| !live.contains(&menu.target))
        {
            self.menu = None;
        }
    }

    /// The cards the selection holds, in the list's own order.
    fn selection(&self, visible: &[Shot]) -> Vec<Shot> {
        visible
            .iter()
            .filter(|shot| self.is_selected(&shot.path))
            .cloned()
            .collect()
    }

    /// Say one sentence at the foot of the page for a couple of seconds.
    fn say(&mut self, now: f64, text: impl Into<String>) {
        self.toast = Some(Toast {
            text: text.into(),
            until: now + TOAST_SECONDS,
        });
    }
}

/// An open card menu: the card it belongs to and the point it was opened at.
#[derive(Debug, Clone)]
struct Menu {
    target: PathBuf,
    anchor: egui::Pos2,
}

/// The list of groups a card can be put in, opened from the menu's "Add to group" row.
#[derive(Debug, Clone)]
struct GroupMenu {
    /// The card this list was opened for.
    target: PathBuf,
    /// Where the list is drawn: the menu's own top-left, so it opens under the row that asked for it.
    anchor: egui::Pos2,
}

/// A destructive press waiting for its confirmation.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Arm {
    /// One card, named by its path.
    Card(PathBuf),
    /// Every card in the selection.
    Selection,
}

/// A sentence the page owes the reader, and the instant it stops being true.
#[derive(Debug, Clone)]
struct Toast {
    text: String,
    until: f64,
}

/// A wall-clock date and time, in the form the game writes into a screenshot's name
/// (`2026-10-21_14.32.07.png`).
///
/// A civil stamp rather than an epoch count: the source reads `created_at` through dayjs
/// (`index.vue:981`) and prints it through `Intl.DateTimeFormat` (`card.vue:46`), and both
/// are local-calendar operations. A stamp needs no timezone, no dependency and no clock to
/// sort or to print, and `Ord` over the field order below **is** chronological order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Stamp {
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

/// One screenshot, as far as this page reads it: the instance it belongs to, its file name
/// (which is what the source's search matches on, `index.vue:408`), when it was taken, and
/// where the file is — the path is what the thumbnail, the menu and the delete are made of.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Shot {
    instance: String,
    file_name: String,
    taken: Stamp,
    path: PathBuf,
}

/// One heading's worth of screenshots.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Group {
    /// The bucket's id: `date:today`, `date:week:YYYY-MM-DD`, `date:month:YYYY-MM`,
    /// `instance:<name>` or `all` (`index.vue:980-1009`).
    id: String,
    /// The heading as printed.
    title: String,
    /// Whether the group draws a heading at all (`groupBy === 'none'` does not).
    header: bool,
    /// The shots, in the order the sort put them.
    shots: Vec<Shot>,
}

// ---------------------------------------------------------------------------
// The rules, as pure functions
// ---------------------------------------------------------------------------

/// The source's search (`index.vue:402-411`): the query is trimmed, lowercased, and a shot
/// matches when its **file name or its instance name** contains it. No query is every shot,
/// which is what makes an untouched field show the whole list.
fn filtered(shots: &[Shot], query: Option<&str>) -> Vec<Shot> {
    let needle = match query {
        Some(text) => text.to_lowercase(),
        None => return shots.to_vec(),
    };
    shots
        .iter()
        .cloned()
        .filter(|shot| {
            shot.file_name.to_lowercase().contains(&needle)
                || shot.instance.to_lowercase().contains(&needle)
        })
        .collect()
}

/// The source's order for the shots themselves (`index.vue:413-418`): by file name for
/// `Name`, and by creation time for the two date orders, either end being a reversal of the
/// other. The sort is stable, as JavaScript's `Array.prototype.sort` is.
fn in_sort_order(shots: &[Shot], sort: Sort) -> Vec<Shot> {
    let mut ordered = shots.to_vec();
    match sort {
        Sort::Name => ordered.sort_by(|a, b| a.file_name.cmp(&b.file_name)),
        Sort::Newest => ordered.sort_by(|a, b| b.taken.cmp(&a.taken)),
        Sort::Oldest => ordered.sort_by(|a, b| a.taken.cmp(&b.taken)),
    }
    ordered
}

/// A screenshot's date bucket, which is the rung of `getDateGroup` it lands on
/// (`index.vue:980-1009`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bucket {
    /// Taken today.
    Today,
    /// Taken yesterday.
    Yesterday,
    /// Earlier this week: dayjs's `isSame(now, 'week')`, a Sunday-to-Saturday week in en-US.
    ThisWeek,
    /// Earlier this month.
    ThisMonth,
    /// Anything older: the month's own name, `formatMonth`.
    Month,
}

/// Days from 1970-01-01 to a civil date: Hinnant's `days_from_civil`, the inverse of the
/// `civil_from_days` in [stamp_from_unix] and the only arithmetic the week bucket needs.
fn day_number(year: i32, month: u8, day: u8) -> i64 {
    let month = i64::from(month);
    let year = i64::from(year) - i64::from(month <= 2);
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let month_index = (month + 9) % 12;
    let day_of_year = (153 * month_index + 2) / 5 + i64::from(day) - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// The day of the week a day number falls on: 0 is Sunday. 1970-01-01 was a Thursday, which
/// is the 4 in the sum.
fn weekday(days: i64) -> i64 {
    (days + 4).rem_euclid(7)
}

/// Which rung of `getDateGroup` a shot taken at `taken` lands on, read against `now`.
fn bucket_of(taken: Stamp, now: Stamp) -> Bucket {
    let today = day_number(now.year, now.month, now.day);
    let day = day_number(taken.year, taken.month, taken.day);
    // A timestamp ahead of the clock is not "today", "yesterday" or "this week": every one of
    // the source's isSame checks fails for it and it falls through to the month's own name.
    if day > today {
        return Bucket::Month;
    }
    if day == today {
        return Bucket::Today;
    }
    if day == today - 1 {
        return Bucket::Yesterday;
    }
    // The week runs from the Sunday on or before today.
    let week_start = today - weekday(today);
    if day >= week_start {
        return Bucket::ThisWeek;
    }
    if taken.year == now.year && taken.month == now.month {
        return Bucket::ThisMonth;
    }
    Bucket::Month
}

/// The heading of a month's group: `formatMonth`, i.e. `useFormatDateTime({ month: 'long',
/// year: 'numeric' })` (`index.vue:227,1003`), which prints `October 2026` in en-US.
///
/// A month outside 1-12 cannot come from [stamp_from_unix]; it still answers a string rather
/// than panicking, because a heading that reads oddly is a look that is wrong and a panic is a
/// window that does not open.
fn month_title(year: i32, month: u8) -> String {
    let name = MONTHS
        .get(usize::from(month.saturating_sub(1)))
        .copied()
        .map_or("", |name| name);
    format!("{name} {year}")
}

/// The group a shot belongs to under the selected mode, as `(id, title, header)`.
fn bucket_key(mode: &GroupBy, shot: &Shot, now: Stamp) -> (String, String, bool) {
    match mode {
        // A custom group's bucket is the group itself: the heading is the group's name and every shot that is not
        // assigned to it lands in a "no group" bucket the caller drops.
        GroupBy::Custom(id) => (
            format!("group:{id}"),
            id.clone(),
            true,
        ),
        GroupBy::None => ("all".to_owned(), ALL_SCREENSHOTS.to_owned(), false),
        GroupBy::Instance => (
            format!("instance:{}", shot.instance),
            shot.instance.clone(),
            true,
        ),
        GroupBy::Date => match bucket_of(shot.taken, now) {
            Bucket::Today => ("date:today".to_owned(), TODAY.to_owned(), true),
            Bucket::Yesterday => ("date:yesterday".to_owned(), YESTERDAY.to_owned(), true),
            Bucket::ThisWeek => {
                let day = day_number(shot.taken.year, shot.taken.month, shot.taken.day);
                let start = day - weekday(day);
                let stamp = stamp_from_unix(start * SECONDS_PER_DAY);
                (
                    format!(
                        "date:week:{:04}-{:02}-{:02}",
                        stamp.year, stamp.month, stamp.day
                    ),
                    THIS_WEEK.to_owned(),
                    true,
                )
            }
            Bucket::ThisMonth => (
                format!("date:month:{:04}-{:02}", shot.taken.year, shot.taken.month),
                THIS_MONTH.to_owned(),
                true,
            ),
            Bucket::Month => (
                format!("date:month:{:04}-{:02}", shot.taken.year, shot.taken.month),
                month_title(shot.taken.year, shot.taken.month),
                true,
            ),
        },
    }
}

/// The source's grouping (`index.vue:431-524`, `980-1009`), as a pure function of the shots,
/// the mode, the reader's clock and the sort.
///
/// Shots keep the order they arrive in — the source groups an array it has already sorted by
/// `sort` (`:413-418`). Date groups are then ordered by their **newest** screenshot,
/// descending unless the sort is `Oldest` (`:511-519`); instance groups are ordered by name,
/// as the source's instance list is (`:443`); the no-grouping mode is one group and needs no
/// order.
fn groups(mode: &GroupBy, shots: &[Shot], now: Stamp, sort: Sort, assignments: &std::collections::BTreeMap<String, String>) -> Vec<Group> {
    // A custom group shows only the shots assigned to it.
    let shots: Vec<Shot> = match mode {
        GroupBy::Custom(id) => shots
            .iter()
            .filter(|shot| assignments.get(&shot.file_name).is_some_and(|group| group == id))
            .cloned()
            .collect(),
        _ => shots.to_vec(),
    };
    let shots = shots.as_slice();
    let mut out: Vec<Group> = Vec::new();
    for shot in shots {
        let (id, title, header) = bucket_key(mode, shot, now);
        let index = out.iter().position(|group| group.id == id);
        match index {
            Some(index) => {
                if let Some(group) = out.get_mut(index) {
                    group.shots.push(shot.clone());
                }
            }
            None => out.push(Group {
                id,
                title,
                header,
                shots: vec![shot.clone()],
            }),
        }
    }

    match mode {
        GroupBy::Custom(_) => {}
        GroupBy::Instance => out.sort_by(|a, b| a.title.cmp(&b.title)),
        GroupBy::Date => {
            let newest = |group: &Group| group.shots.iter().map(|shot| shot.taken).max();
            out.sort_by(|a, b| {
                let (left, right) = (newest(a), newest(b));
                match sort {
                    Sort::Oldest => left.cmp(&right),
                    Sort::Newest | Sort::Name => right.cmp(&left),
                }
            });
        }
        GroupBy::None => {}
    }
    out
}

/// The date on a card, as the source prints it in the overlay: `formatTime` is
/// `useFormatDateTime({ dateStyle: 'medium', timeStyle: 'short' })` (`card.vue:46`), which
/// is `Oct 21, 2026, 2:32 PM` in en-US. The hour is the 12-hour clock `Intl` prints, with
/// midnight and noon as `12`.
fn stamp_label(stamp: Stamp) -> String {
    let month = MONTHS_SHORT
        .get(usize::from(stamp.month.saturating_sub(1)))
        .copied()
        .map_or("", |name| name);
    let hour = match stamp.hour % 12 {
        0 => 12,
        other => other,
    };
    let meridiem = if stamp.hour < 12 { "AM" } else { "PM" };
    format!(
        "{month} {}, {}, {hour}:{:02} {meridiem}",
        stamp.day, stamp.year, stamp.minute
    )
}

/// The instance and the date on one line, which is `app.screenshots.preview.instance-and-date`
/// — `{instance} · {date}` (`locales/en-US/index.json:1574`). In the source that line is the
/// preview's description for a global screenshot (`index.vue:668-673`); the card's second
/// line is the bare instance name while the list is one instance's (`card.vue:240`).
fn instance_and_date(shot: Shot) -> String {
    format!("{} · {}", shot.instance, stamp_label(shot.taken))
}

/// The sentence a finished delete owes: `Screenshot deleted` for one, the count for many
/// (`index.vue:1211-1220`).
fn deleted_sentence(count: usize) -> String {
    match count {
        1 => DELETE_SUCCESS.to_owned(),
        other => format!("{other} screenshots deleted"),
    }
}

/// Remove every path from the disk, answering how many went.
///
/// A failure is returned rather than swallowed: a destructive press that half-worked and said
/// nothing is the one outcome it must not have.
fn remove_files(paths: &[PathBuf]) -> Result<usize, String> {
    let mut removed = 0;
    for path in paths {
        match fs::remove_file(path) {
            Ok(()) => removed += 1,
            Err(error) => {
                let name = path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| path.display().to_string());
                return Err(format!("Could not delete {name}: {error}"));
            }
        }
    }
    Ok(removed)
}

/// The resolved grid: the column count and one card's size, in the page's points.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Grid {
    columns: usize,
    card_width: f32,
    card_height: f32,
}

/// The source's column count (`index.vue:522-526`), which is the ladder its grid classes
/// spell out: `grid-cols-1 sm:grid-cols-2 2xl:grid-cols-4` (`group.vue:210`).
fn column_count(window_width: f32) -> usize {
    if window_width >= WIDE_BREAKPOINT {
        WIDE_COLUMNS
    } else if window_width >= NARROW_BREAKPOINT {
        NARROW_COLUMNS
    } else {
        SLIM_COLUMNS
    }
}

/// The source's `screenshotCardWidth` (`index.vue:528-532`) and `screenshotCardHeight`
/// (`:534`): the list's width less the gaps, over the column count — or the 320-point
/// fallback while the list has not been measured.
fn grid_layout(list_width: f32, window_width: f32) -> Grid {
    let columns = column_count(window_width);
    let gaps = GRID_GAP * (columns.saturating_sub(1)) as f32;
    let card_width = if list_width <= 0.0 {
        FALLBACK_CARD_WIDTH
    } else {
        ((list_width - gaps) / columns as f32).max(0.0)
    };
    Grid {
        columns,
        card_width,
        card_height: card_width * CARD_ASPECT,
    }
}

/// One group's block height, from the source's own sum (`index.vue:542-566`): the header, the
/// 10-point gap under it, the grid's height, and the group's 12-point tail.
///
/// A collapsed group is its header and that tail, because the source's accordion unmounts the
/// grid (`group.vue:99-120`); a mode that hides the header contributes no header height.
fn block_height(
    columns: usize,
    card_height: f32,
    count: usize,
    collapsed: bool,
    header: bool,
) -> f32 {
    let head = if header { GROUP_HEADER_HEIGHT } else { 0.0 };
    if collapsed {
        return head + GROUP_SPACING;
    }
    let rows = count.div_ceil(columns.max(1));
    let grid_height = if rows == 0 {
        GRID_MIN_HEIGHT
    } else {
        rows as f32 * card_height + rows.saturating_sub(1) as f32 * GRID_GAP
    };
    head + GROUP_CONTENT_SPACING + grid_height + GROUP_SPACING
}


// ---------------------------------------------------------------------------
// The page
// ---------------------------------------------------------------------------

/// The rects the heading and the toolbar occupy, so a press can be read against them.
struct Chrome {
    /// Where the list starts.
    list_top: f32,
    /// The search field's plate.
    search: Rect,
    /// The New group button.
    new_group: Rect,
    /// The sort select's trigger.
    sort: Rect,
    /// The group select's trigger.
    group: Rect,
}

/// The wall clock, as the page's [Stamp]: the date buckets are the reader's calendar, not the
/// file's, so this is read from the system clock rather than from the list.
fn now_stamp() -> Stamp {
    let seconds = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since_epoch| since_epoch.as_secs() as i64);
    stamp_from_unix(seconds)
}

/// Draw the screenshots page into the rect it owns. Returns whether another frame is owed.
pub fn draw(
    ui: &mut Ui,
    rect: Rect,
    theme: &Theme,
    images: &mut Images,
    state: &mut State,
) -> bool {
    // The page owns its rect (the pages module's seam); paint the page colour first so the
    // cards sit on the background even if the caller did not clear the column.
    ui.painter().rect_filled(rect, 0, theme.BG);

    if rect.width() <= 2.0 * PAGE_PADDING || rect.height() <= 2.0 * PAGE_PADDING {
        return false;
    }

    // The folder is read on the first draw, not on every frame: a scan is filesystem I/O on
    // the UI thread, and the common list is stable for the whole visit. An empty list is
    // re-read on later draws, because that is the reader who takes their first screenshot
    // and comes back to find it.
    // shortcut: the list is scanned on the first draw and while empty, not once it has
    // shots, until a file watcher can report a new screenshot. A delete re-reads it by hand.
    if !state.scanned || (state.shots.is_empty() && state.error.is_none()) {
        match read_shots() {
            Ok(shots) => {
                state.shots = shots;
                state.error = None;
            }
            Err(message) => {
                state.shots.clear();
                state.error = Some(message);
            }
        }
        state.scanned = true;
    }

    // A scan that failed is not an empty folder: the source draws an error state carrying the
    // query's own message and a Retry button (`index.vue:1506-1517`).
    if let Some(message) = state.error.clone() {
        if error_state(ui, rect, theme, &message) {
            state.scanned = false;
            state.error = None;
            ui.ctx().request_repaint();
        }
        return false;
    }
    state.prune();

    // Two clocks: the frame's own, which decides when a toast leaves, and the wall clock,
    // which decides which day-bucket a screenshot falls in.
    let seconds = ui.ctx().input(|input| input.time);
    let today = now_stamp();

    // The whole set and the set the search leaves, kept apart: the source branches on
    // screenshots.length for "nothing yet" and on filteredScreenshots.length for "nothing
    // matched" (index.vue:1515-1525), and the two say different things.
    let visible = in_sort_order(&filtered(&state.shots, state.query()), state.sort);

    if state.shots.is_empty() {
        empty_state(
            ui.painter(),
            rect,
            theme,
            Glyph::Images,
            EMPTY_HEADING,
            EMPTY_BODY,
        );
        return false;
    }

    let total = state.shots.len();
    let chrome = heading_and_toolbar(ui.painter(), rect, theme, state, total);
    // Every rectangle a press is read against, for `SIFT_SHOTS_PROBE`: a drive aims at a
    // measured rect rather than at a guess from a capture.
    let mut probe_rows: Vec<(String, Rect)> = vec![
        ("search".to_owned(), chrome.search),
        ("new-group".to_owned(), chrome.new_group),
        ("sort".to_owned(), chrome.sort),
        ("group".to_owned(), chrome.group),
    ];

    // The toolbar's presses are read from the pointer rather than registered with egui: the
    // list below registers a full-width interaction, and an interaction it could take would
    // never reach a control under it. This is the ordering the servers page records too.
    let (pointer, clicked) = ui.input(|input| {
        (input.pointer.interact_pos(), input.pointer.primary_clicked())
    });
    let pressed = |target: Rect| clicked && pointer.is_some_and(|at| target.contains(at));

    if pressed(chrome.new_group) {
        state.say(seconds, "Screenshot groups are not built yet");
    }
    if pressed(chrome.sort) {
        state.sort_open = !state.sort_open;
        state.group_open = false;
    }
    if pressed(chrome.group) {
        state.group_open = !state.group_open;
        state.sort_open = false;
    }

    let mut owed = search_field(ui, chrome.search, state);

    let list_rect = Rect::from_min_max(
        egui::pos2(rect.left(), chrome.list_top),
        egui::pos2(rect.right(), rect.bottom() - PAGE_PADDING),
    );
    if list_rect.width() < 1.0 || list_rect.height() < 1.0 {
        return owed;
    }
    probe_rows.push(("list".to_owned(), list_rect));

    // gridWidth is the measured content box (useElementSize, index.vue:224), which is this
    // page's list rect; windowWidth is the window's (useWindowSize, :225), and it picks the
    // column count before any width is divided, so it cannot be the list's own width. egui
    // 0.36 calls the window's content rectangle Context::content_rect.
    let grid = grid_layout(list_rect.width(), ui.ctx().content_rect().width());

    let selection = state.selection(&visible);

    // The overlays that will cover the list, so a card under one cannot take the press that
    // belongs to the overlay. They are painted after the list; this is what keeps the two
    // from arguing over the same click.
    let sort_list = dropdown_rect(chrome.sort, Sort::ALL.len(), rect);
    let group_list = dropdown_rect(chrome.group, GroupBy::ALL.len(), rect);
    let menu_plate = state
        .menu
        .as_ref()
        .map(|menu| menu_rect(menu.anchor, MENU_ITEMS.len(), rect));
    let bar_rect = {
        let painter = ui.painter();
        selection_bar_rect(painter, list_rect, theme, selection.len())
    };

    let mut blocked: Vec<Rect> = Vec::new();
    if state.sort_open {
        blocked.push(sort_list);
    }
    if state.group_open {
        blocked.push(group_list);
    }
    if let Some(plate) = menu_plate {
        blocked.push(plate);
    }
    if let Some(bar) = bar_rect {
        blocked.push(bar);
    }
    // The viewer covers the page: a press that dismisses it must not also press the card or the
    // group header underneath, which is what a pass-through click would do.
    if state.viewer.is_some() {
        blocked.push(rect);
    }

    // The state for a search that matches nothing: the toolbar stays, so the query can be
    // cleared, and the list area says why it is empty (index.vue:1520-1524).
    if visible.is_empty() {
        empty_state(
            ui.painter(),
            list_rect,
            theme,
            Glyph::Search,
            NO_RESULTS_HEADING,
            NO_RESULTS_BODY,
        );
    } else {
        let blocks: Vec<(Group, f32)> = groups(&state.group_by, &visible, today, state.sort, &state.groups.assignments)
            .into_iter()
            .map(|group| {
                let height = block_height(
                    grid.columns,
                    grid.card_height,
                    group.shots.len(),
                    state.collapsed.contains(&group.id),
                    group.header,
                );
                (group, height)
            })
            .collect();
        let content_height: f32 = blocks.iter().map(|(_, height)| *height).sum();

        // The source scrolls the whole page; this page owns the column below its toolbar, so
        // the scroll area starts at the list and the heading stays put.
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(list_rect));
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(&mut child, |ui| {
                // Reserve the whole list height so the scroll area knows what is below the
                // fold; the blocks are then painted at the source's absolute positions.
                let (area, _) = ui.allocate_exact_size(
                    egui::vec2(list_rect.width(), content_height),
                    Sense::hover(),
                );
                let mut top = area.top();
                let mut base = 0;
                for (group, height) in &blocks {
                    draw_group(
                        ui,
                        egui::pos2(area.left(), top),
                        list_rect.width(),
                        grid,
                        group,
                        base,
                        theme,
                        images,
                        state,
                        &blocked,
                        &mut probe_rows,
                    );
                    top += *height;
                    base += group.shots.len();
                }
            });
    }

    // ---- The overlays, last so they paint over the list ----

    if state.sort_open {
        let labels: Vec<&str> = Sort::ALL.iter().map(|sort| sort.label()).collect();
        let chosen = Sort::ALL
            .iter()
            .position(|sort| *sort == state.sort)
            .unwrap_or(0);
        if let Some(row) = open_list(ui, sort_list, &labels, chosen, theme, "sort") {
            state.sort = Sort::ALL.get(row).copied().unwrap_or_default();
            state.sort_open = false;
            owed = true;
        } else if clicked
            && !pointer.is_some_and(|at| sort_list.contains(at) || chrome.sort.contains(at))
        {
            // A press outside the plate closes it. The trigger is excluded, or the press that
            // opened the list would close it again in the same frame.
            state.sort_open = false;
        }
    }

    if state.group_open {
        // The select's entries: the three modes and then one per custom group, by its own name.
        let modes = GroupBy::with_groups(&state.groups.groups);
        let labels: Vec<String> = modes
            .iter()
            .map(|mode| match mode {
                GroupBy::Custom(id) => state
                    .groups
                    .groups
                    .iter()
                    .find(|group| &group.id == id)
                    .map(|group| group.name.clone())
                    .unwrap_or_else(|| mode.label().to_owned()),
                other => other.label().to_owned(),
            })
            .collect();
        let labels: Vec<&str> = labels.iter().map(String::as_str).collect();
        let chosen = modes
            .iter()
            .position(|mode| *mode == state.group_by)
            .unwrap_or(0);
        if let Some(row) = open_list(ui, group_list, &labels, chosen, theme, "group") {
            state.group_by = modes.get(row).cloned().unwrap_or_default();
            state.group_open = false;
            owed = true;
        } else if clicked
            && !pointer.is_some_and(|at| group_list.contains(at) || chrome.group.contains(at))
        {
            state.group_open = false;
        }
    }

    if let Some(bar) = bar_rect {
        owed |= selection_bar(ui, bar, theme, state, seconds, selection.len(), &visible);
    }

    if let Some(menu) = state.menu.clone() {
        owed |= card_menu(ui, theme, rect, state, &menu, seconds, &visible);
    }

    // The group list a card's menu opened: one row per group, the card's own group marked, and a row to take it out.
    if let Some(list) = state.group_menu.clone() {
        let rows = state.groups.groups.len() + 1;
        let plate = egui::Rect::from_min_size(
            list.anchor,
            egui::vec2(220.0, rows as f32 * 34.0 + 12.0),
        );
        crate::pages::screenshots::menu_plate(ui.painter(), plate, theme);
        let mut chosen: Option<Option<String>> = None;
        for index in 0..state.groups.groups.len() {
            let row = egui::Rect::from_min_size(
                egui::pos2(plate.left() + 6.0, plate.top() + 6.0 + index as f32 * 34.0),
                egui::vec2(plate.width() - 12.0, 34.0),
            );
            let group = state.groups.groups[index].clone();
            let is_current = state
                .groups
                .assignments
                .get(&list.target.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default())
                .is_some_and(|id| id == &group.id);
            ui.painter().text(
                egui::pos2(row.left() + 10.0, row.center().y),
                Align2::LEFT_CENTER,
                if is_current {
                    format!("\u{2713} {}", group.name)
                } else {
                    group.name.clone()
                },
                fonts::body(13.0),
                if is_current { theme.BRAND } else { theme.BASE },
            );
            if ui
                .interact(row, egui::Id::new(("group-menu", index)), Sense::click())
                .clicked()
            {
                chosen = Some(Some(group.id.clone()));
            }
        }
        let none_row = egui::Rect::from_min_size(
            egui::pos2(plate.left() + 6.0, plate.top() + 6.0 + state.groups.groups.len() as f32 * 34.0),
            egui::vec2(plate.width() - 12.0, 34.0),
        );
        ui.painter().text(
            egui::pos2(none_row.left() + 10.0, none_row.center().y),
            Align2::LEFT_CENTER,
            "Remove from group",
            fonts::body(13.0),
            theme.SECONDARY,
        );
        if ui
            .interact(none_row, egui::Id::new("group-menu-none"), Sense::click())
            .clicked()
        {
            chosen = Some(None);
        }
        if let Some(choice) = chosen {
            let shot = list
                .target
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            state.groups.assign(&shot, choice.as_deref());
            let path = crate::groups::Groups::path(&crate::data::instances_dir());
            let _ = state.groups.save(&path);
            let name = choice
                .as_deref()
                .and_then(|id| state.groups.groups.iter().find(|group| group.id == id))
                .map(|group| group.name.clone());
            state.say(
                seconds,
                &match name {
                    Some(name) => format!("Added to {name}."),
                    None => "Removed from its group.".to_owned(),
                },
            );
            state.group_menu = None;
            owed = true;
        }
    }

    // The viewer is last of all: it is a page-sized overlay, and nothing is drawn over it.
    owed |= viewer(ui, rect, theme, images, state, &visible);

    // A toast outlives the press that made it by a couple of seconds and then leaves.
    owed |= toast(ui, theme, state, seconds, bar_rect);

    if let Some(bar) = bar_rect {
        probe_rows.push(("bar".to_owned(), bar));
    }
    if state.sort_open {
        probe_rows.push(("sort-list".to_owned(), sort_list));
    }
    if state.group_open {
        probe_rows.push(("group-list".to_owned(), group_list));
    }
    if let Some(plate) = menu_plate {
        probe_rows.push(("menu".to_owned(), plate));
    }
    probe(ui, &probe_rows);

    // A stale arm is a destructive press waiting for a second one that may never come; any
    // press that is not on the armed control itself puts it away. This runs after every
    // handler, so the frame's own press has had its chance to set armed_touched.
    if clicked && !state.armed_touched {
        state.armed = None;
    }
    state.armed_touched = false;

    // The new-group dialog, over the page while it is open. Its field is a real `TextEdit` on the page's own `ui`,
    // which is what makes it typeable; the two buttons read the page's raw press like the toolbar's does.
    if let Some(name_now) = state.new_group.clone() {
        let painter = ui.painter().clone();
        painter.rect_filled(rect, 0, egui::Color32::from_black_alpha(140));
        let plate = egui::Rect::from_center_size(rect.center(), egui::vec2(360.0, 176.0));
        painter.rect_filled(plate, 12.0, theme.RAISED_BG);
        painter.rect_stroke(
            plate,
            12.0,
            egui::Stroke::new(1.0, theme.SURFACE_5),
            egui::StrokeKind::Inside,
        );
        painter.text(
            egui::pos2(plate.left() + 20.0, plate.top() + 18.0),
            Align2::LEFT_TOP,
            NEW_GROUP,
            fonts::semibold(16.0),
            theme.CONTRAST,
        );
        let field = egui::Rect::from_min_size(
            egui::pos2(plate.left() + 20.0, plate.top() + 54.0),
            egui::vec2(plate.width() - 40.0, 34.0),
        );
        let mut name = name_now;
        ui.put(
            field,
            egui::TextEdit::singleline(&mut name)
                .id(egui::Id::new("new-group-name"))
                .hint_text("Group name")
                .text_color(theme.CONTRAST)
                .desired_width(field.width()),
        );
        if state.new_group.as_deref() != Some(name.as_str()) {
            state.new_group = Some(name.clone());
        }
        let create = egui::Rect::from_min_size(
            egui::pos2(plate.right() - 20.0 - 96.0, plate.bottom() - 20.0 - 34.0),
            egui::vec2(96.0, 34.0),
        );
        painter.rect_filled(create, 10.0, theme.BRAND);
        painter.text(
            create.center(),
            Align2::CENTER_CENTER,
            "Create",
            fonts::semibold(13.0),
            theme.ACCENT_CONTRAST,
        );
        let cancel = egui::Rect::from_min_size(
            egui::pos2(plate.left() + 20.0, plate.bottom() - 20.0 - 34.0),
            egui::vec2(84.0, 34.0),
        );
        painter.rect_filled(cancel, 10.0, theme.BUTTON_BG);
        painter.text(
            cancel.center(),
            Align2::CENTER_CENTER,
            "Cancel",
            fonts::semibold(13.0),
            theme.BASE,
        );
        if clicked && pointer.is_some_and(|at| create.contains(at)) {
            let trimmed = name.trim().to_owned();
            if !trimmed.is_empty() {
                let path = crate::groups::Groups::path(&crate::data::instances_dir());
                let mut store = crate::groups::Groups::load(&path);
                store.add(&trimmed);
                let _ = store.save(&path);
            }
            state.new_group = None;
            owed = true;
        } else if clicked && pointer.is_some_and(|at| cancel.contains(at)) {
            state.new_group = None;
            owed = true;
        }
    }

    // **"New group" reads the page's own press.** `clicked` and `pointer` are this frame's raw input; testing the
    // button's rect against them cannot be stolen by a later widget, which an `interact` here can.
    if clicked
        && state
            .new_group_button
            .is_some_and(|button| pointer.is_some_and(|at| button.contains(at)))
    {
        state.new_group = Some(String::new());
        owed = true;
    }

    owed
}


/// Set `SIFT_SHOTS_PROBE` to print every rectangle a press is read against, on any frame the
/// pointer clicks. This is the instrument the settings and servers pages use: a drive aims at a
/// measured rectangle, never at a guess read off a capture.
fn probe(ui: &Ui, rows: &[(String, Rect)]) {
    if std::env::var_os("SIFT_SHOTS_PROBE").is_none() {
        return;
    }
    if !ui.input(|input| input.pointer.any_click()) {
        return;
    }
    let pointer = ui.input(|input| input.pointer.interact_pos());
    eprintln!("screenshots-probe pointer={pointer:?}");
    for (name, rect) in rows {
        eprintln!(
            "screenshots-probe {name:<28} {:.1},{:.1} {:.1}x{:.1}",
            rect.left(),
            rect.top(),
            rect.width(),
            rect.height()
        );
    }
}

// ---------------------------------------------------------------------------
// Heading and toolbar
// ---------------------------------------------------------------------------

/// Draw the heading and the toolbar's two rows. Returns the rects its controls occupy and the
/// y where the list begins.
fn heading_and_toolbar(
    painter: &egui::Painter,
    rect: Rect,
    theme: &Theme,
    state: &mut State,
    _total: usize,
) -> Chrome {
    let heading_at = egui::pos2(rect.left(), rect.top() + PAGE_PADDING);
    // h1.m-0.text-2xl.font-bold.text-contrast (index.vue:1486). font-bold is Inter 700, one
    // weight above the library's text-2xl font-semibold heading.
    //
    // The source has no page-level count: its count belongs to each group's header
    // (section.vue:148-150), so no pill is drawn beside the heading.
    painter.text(
        heading_at,
        Align2::LEFT_TOP,
        HEADING,
        fonts::title(HEADING_SIZE),
        theme.CONTRAST,
    );

    // Row 1: the search plate and the New group button (toolbar.vue:36-49), the field flex-1
    // at a 256-point floor and the button content-sized after it.
    let row_top = heading_at.y + HEADING_LINE + SECTION_GAP;
    let button_width = text_button_width(painter, NEW_GROUP, theme);
    // **No PAGE_PADDING here either.** The rect is already the padded column, and subtracting the padding twice made this field
    // 48 points narrow: measured, its plate ran 89..1066 where the reference's runs 89..1114, and 2 x 24 is exactly the gap.
    let search_width = (rect.width() - CONTROL_GAP - button_width).max(SEARCH_MIN_WIDTH);
    let search = Rect::from_min_size(
        egui::pos2(rect.left(), row_top),
        egui::vec2(search_width, CONTROL_HEIGHT),
    );
    search_plate(painter, search, theme, state);
    let new_group = Rect::from_min_size(
        egui::pos2(search.right() + CONTROL_GAP, row_top),
        egui::vec2(button_width, CONTROL_HEIGHT),
    );
    text_button(painter, new_group, NEW_GROUP, Glyph::SquarePlus, theme);
    state.new_group_button = Some(new_group);

    // Row 2: the sort and group Comboboxes, side by side (toolbar.vue:50-85). There is no
    // third control: this toolbar has no view toggle.
    let selects_top = row_top + CONTROL_HEIGHT + TOOLBAR_ROW_GAP;
    let sort = select(
        painter,
        egui::pos2(rect.left(), selects_top),
        theme,
        Glyph::ArrowUpDown,
        state.sort.label(),
        state.sort_open,
    );
    let group = select(
        painter,
        egui::pos2(sort.right() + CONTROL_GAP, selects_top),
        theme,
        Glyph::LayoutGrid,
        state.group_by.label(),
        state.group_open,
    );

    Chrome {
        list_top: selects_top + CONTROL_HEIGHT + SECTION_GAP,
        search,
        new_group,
        sort,
        group,
    }
}

/// The search field's plate and its keyboard.
///
/// InputFrame at its standard size is h-9 rounded-xl px-3 gap-2 border border-surface-5
/// bg-surface-4 (InputFrame.vue:4,47), with a 20-point leading glyph in text-secondary
/// (Input.vue:12), the text at text-base font-medium text-primary and the placeholder in
/// text-secondary (Input.vue:51,55). The field is clearable (toolbar.vue:42), so a non-empty
/// query has the 20-point X at its right end (InputClearButton.vue:4).
///
/// The keyboard is this page's: a press takes the focus, and while it is held every typed
/// character, Backspace and Escape are read from the frame's events. That is the arrangement
/// the servers page's field uses, and the one that page proved accepts typing.
fn search_field(ui: &mut Ui, rect: Rect, state: &mut State) -> bool {
    let response = ui.interact(rect, egui::Id::new("screenshots-search"), Sense::click());
    let clear = Rect::from_center_size(
        egui::pos2(
            rect.right() - SEARCH_PADDING - SEARCH_GLYPH / 2.0,
            rect.center().y,
        ),
        Vec2::splat(SEARCH_GLYPH),
    );

    if response.clicked() {
        // The field is one target, so the clear button inside it is decided by where the
        // press landed rather than by a second overlapping interaction whose ordering would
        // depend on which was registered first.
        let on_clear = ui
            .input(|input| input.pointer.interact_pos())
            .is_some_and(|at| clear.contains(at));
        if on_clear && !state.search.is_empty() {
            state.search.clear();
        }
        state.search_focused = true;
        ui.ctx().request_repaint();
        return true;
    }

    if !state.search_focused {
        return false;
    }

    let mut changed = false;
    for event in ui.input(|input| input.events.clone()) {
        match event {
            egui::Event::Text(text) => {
                // A query is one line: a pasted newline would make the plate's own single
                // line lie about what is being matched.
                let text: String = text.chars().filter(|c| !c.is_control()).collect();
                state.search.push_str(&text);
                changed = true;
            }
            egui::Event::Key {
                key: egui::Key::Backspace,
                pressed: true,
                ..
            } => {
                state.search.pop();
                changed = true;
            }
            egui::Event::Key {
                key: egui::Key::Escape,
                pressed: true,
                ..
            } => {
                state.search_focused = false;
                changed = true;
            }
            _ => {}
        }
    }

    // The caret blinks at 1 Hz, so a frame is owed in half a second whether or not a key
    // arrives. Asking for one keeps the field alive without a repaint every frame.
    ui.ctx()
        .request_repaint_after(std::time::Duration::from_millis(500));
    changed
}

/// The search field's plate: what the source's Input looks like.
fn search_plate(painter: &egui::Painter, rect: Rect, theme: &Theme, state: &State) {
    painter.rect_filled(rect, CONTROL_RADIUS, theme.SURFACE_4);
    painter.rect_stroke(
        rect,
        CONTROL_RADIUS,
        Stroke::new(STROKE_WIDTH, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );

    // The source lifts the field's focus ring to focus-within:ring-4 ring-brand-shadow
    // (InputFrame.vue:4); a second ring colour is one more token than this theme carries, so
    // the border takes the brand ink instead and the glyph brightens with it.
    if state.search_focused {
        painter.rect_stroke(
            rect,
            CONTROL_RADIUS,
            Stroke::new(STROKE_WIDTH * 2.0, theme.BRAND),
            egui::StrokeKind::Inside,
        );
    }

    icons::draw(
        painter,
        Glyph::Search,
        Rect::from_center_size(
            egui::pos2(
                rect.left() + SEARCH_PADDING + SEARCH_GLYPH / 2.0,
                rect.center().y,
            ),
            Vec2::splat(SEARCH_GLYPH),
        ),
        if state.search_focused {
            theme.CONTRAST
        } else {
            theme.SECONDARY
        },
    );

    let clear = Rect::from_center_size(
        egui::pos2(
            rect.right() - SEARCH_PADDING - SEARCH_GLYPH / 2.0,
            rect.center().y,
        ),
        Vec2::splat(SEARCH_GLYPH),
    );
    let text_left = rect.left() + SEARCH_PADDING + SEARCH_GLYPH + SEARCH_INNER_GAP;
    let empty = state.search.is_empty();
    let text_right = if empty {
        rect.right() - SEARCH_PADDING
    } else {
        clear.left() - SEARCH_INNER_GAP
    };

    let (contents, ink) = if empty {
        (SEARCH_PLACEHOLDER, theme.SECONDARY)
    } else {
        (state.search.as_str(), theme.TEXT_PRIMARY)
    };
    let galley = painter.layout_no_wrap(contents.to_owned(), fonts::label(SEARCH_TEXT_SIZE), ink);
    let text_width = galley.size().x;
    painter
        .with_clip_rect(Rect::from_min_max(
            egui::pos2(text_left, rect.top()),
            egui::pos2(text_right, rect.bottom()),
        ))
        .galley(
            egui::pos2(text_left, rect.center().y - galley.size().y / 2.0),
            galley,
            ink,
        );

    if !empty {
        icons::draw(painter, Glyph::X, clear, theme.SECONDARY);
    }

    // The caret sits after the text and blinks on the frame clock.
    if state.search_focused {
        let now = painter.ctx().input(|input| input.time);
        if (now * 2.0).fract() < 0.5 {
            let caret_x = (text_left + text_width + 1.0).min(text_right);
            painter.rect_filled(
                Rect::from_min_size(
                    egui::pos2(caret_x, rect.center().y - SEARCH_TEXT_SIZE / 2.0),
                    egui::vec2(STROKE_WIDTH, SEARCH_TEXT_SIZE),
                ),
                0,
                theme.CONTRAST,
            );
        }
    }
}

/// The width of a text button with a leading glyph: the frame's two paddings, the glyph, the
/// gap and the label, which is px-2.5 gap-1.5 [&>svg]:size-5 (ButtonFrame.vue:31).
fn text_button_width(painter: &egui::Painter, label: &str, theme: &Theme) -> f32 {
    let galley = painter.layout_no_wrap(
        label.to_owned(),
        fonts::semibold(BUTTON_TEXT_SIZE),
        theme.CONTRAST,
    );
    2.0 * BUTTON_PADDING + BUTTON_GLYPH + BUTTON_GAP + galley.size().x
}

/// A Button at its default type and md size: bg-surface-4 text-contrast with a 1-point inset
/// surface-5 ring (ButtonFrame.vue:45,134-140), h-9 rounded-xl px-2.5 gap-1.5, the glyph in
/// text-primary and the label at text-base font-semibold (ButtonFrame.vue:31).
fn text_button(painter: &egui::Painter, rect: Rect, label: &str, glyph: Glyph, theme: &Theme) {
    painter.rect_filled(rect, CONTROL_RADIUS, theme.SURFACE_4);
    painter.rect_stroke(
        rect,
        CONTROL_RADIUS,
        Stroke::new(STROKE_WIDTH, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );
    icons::draw(
        painter,
        glyph,
        Rect::from_center_size(
            egui::pos2(
                rect.left() + BUTTON_PADDING + BUTTON_GLYPH / 2.0,
                rect.center().y,
            ),
            Vec2::splat(BUTTON_GLYPH),
        ),
        theme.TEXT_PRIMARY,
    );
    painter.text(
        egui::pos2(
            rect.left() + BUTTON_PADDING + BUTTON_GLYPH + BUTTON_GAP,
            rect.center().y,
        ),
        Align2::LEFT_CENTER,
        label,
        fonts::semibold(BUTTON_TEXT_SIZE),
        theme.CONTRAST,
    );
}

/// One Combobox trigger, drawn the way its ButtonFrame resolves at md: h-9 rounded-xl px-2.5
/// gap-1.5 with 20-point glyphs (ButtonFrame.vue:31), a 20-point prefix in text-primary
/// (toolbar.vue:60), the selected label at font-semibold inheriting text-contrast
/// (Combobox.vue:80) and a 20-point down chevron in the suffix row (Combobox.vue:88-93).
/// Returns the rect it used.
fn select(
    painter: &egui::Painter,
    min: egui::Pos2,
    theme: &Theme,
    glyph: Glyph,
    label: &str,
    open: bool,
) -> Rect {
    let label_galley = painter.layout_no_wrap(
        label.to_owned(),
        fonts::semibold(SELECT_LABEL_SIZE),
        theme.CONTRAST,
    );
    let width = 2.0 * SELECT_PADDING
        + SELECT_GLYPH
        + SELECT_INNER_GAP
        + label_galley.size().x
        + SELECT_SUFFIX_GAP
        + SELECT_GLYPH;
    let rect = Rect::from_min_size(min, egui::vec2(width, CONTROL_HEIGHT));
    // An open Combobox lifts its trigger to the raised tone its popover sits on.
    painter.rect_filled(
        rect,
        CONTROL_RADIUS,
        if open { theme.SURFACE_4 } else { theme.BUTTON_BG },
    );
    painter.rect_stroke(
        rect,
        CONTROL_RADIUS,
        Stroke::new(STROKE_WIDTH, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );

    icons::draw(
        painter,
        glyph,
        Rect::from_center_size(
            egui::pos2(
                rect.left() + SELECT_PADDING + SELECT_GLYPH / 2.0,
                rect.center().y,
            ),
            Vec2::splat(SELECT_GLYPH),
        ),
        theme.TEXT_PRIMARY,
    );
    painter.text(
        egui::pos2(
            rect.left() + SELECT_PADDING + SELECT_GLYPH + SELECT_INNER_GAP,
            rect.center().y,
        ),
        Align2::LEFT_CENTER,
        label,
        fonts::semibold(SELECT_LABEL_SIZE),
        theme.CONTRAST,
    );
    icons::draw(
        painter,
        Glyph::ChevronDown,
        Rect::from_center_size(
            egui::pos2(
                rect.right() - SELECT_PADDING - SELECT_GLYPH / 2.0,
                rect.center().y,
            ),
            Vec2::splat(SELECT_GLYPH),
        ),
        theme.CONTRAST,
    );
    rect
}

/// The count pill, which is TagItem's shape: px-2 py-1 text-sm leading-none rounded-full with
/// a 1-point border (base/TagItem.vue:19), in the colours the group header gives it, which are
/// border-surface-3 bg-surface-2 (section.vue:148).
fn count_pill(painter: &egui::Painter, left: f32, centre_y: f32, count: usize, theme: &Theme) {
    let text = count.to_string();
    let galley = painter.layout_no_wrap(
        text.clone(),
        fonts::body(BADGE_TEXT_SIZE),
        theme.SECONDARY,
    );
    let pill = Rect::from_min_size(
        egui::pos2(left, centre_y - BADGE_HEIGHT / 2.0),
        egui::vec2(galley.size().x + 2.0 * BADGE_PADDING, BADGE_HEIGHT),
    );
    painter.rect_filled(pill, BADGE_RADIUS, theme.SURFACE_2);
    painter.rect_stroke(
        pill,
        BADGE_RADIUS,
        Stroke::new(STROKE_WIDTH, theme.SURFACE_3),
        egui::StrokeKind::Inside,
    );
    painter.text(
        pill.center(),
        Align2::CENTER_CENTER,
        text,
        fonts::body(BADGE_TEXT_SIZE),
        theme.SECONDARY,
    );
}


// ---------------------------------------------------------------------------
// Menus: the card's, and the toolbar's two dropdowns
// ---------------------------------------------------------------------------

/// The rect a dropdown's rows occupy under its trigger, clamped into the page.
fn dropdown_rect(anchor: Rect, rows: usize, page: Rect) -> Rect {
    let height = rows as f32 * MENU_ROW + 2.0 * MENU_PAD;
    let width = anchor.width().max(MENU_MIN_WIDTH);
    let mut min = egui::pos2(anchor.left(), anchor.bottom() + MENU_GAP);
    if min.y + height > page.bottom() {
        min.y = (anchor.top() - MENU_GAP - height).max(page.top());
    }
    if min.x + width > page.right() {
        min.x = page.right() - width;
    }
    Rect::from_min_size(min, egui::vec2(width, height))
}

/// The rect a card's menu occupies, growing down and right from the press and clamped into the
/// page so a card at the edge still opens a menu that can be read.
fn menu_rect(anchor: egui::Pos2, rows: usize, page: Rect) -> Rect {
    let height = rows as f32 * MENU_ROW + 2.0 * MENU_PAD;
    let mut min = anchor;
    if min.x + MENU_MIN_WIDTH > page.right() {
        min.x = page.right() - MENU_MIN_WIDTH;
    }
    if min.y + height > page.bottom() {
        min.y = page.bottom() - height;
    }
    Rect::from_min_size(
        egui::pos2(min.x.max(page.left()), min.y.max(page.top())),
        egui::vec2(MENU_MIN_WIDTH, height),
    )
}

/// A menu plate: the raised fill and the surface-5 ring every popover in the sheet carries.
fn menu_plate(painter: &egui::Painter, plate: Rect, theme: &Theme) {
    painter.rect_filled(plate, MENU_RADIUS, theme.SUPER_RAISED_BG);
    painter.rect_stroke(
        plate,
        MENU_RADIUS,
        Stroke::new(STROKE_WIDTH, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );
}

/// Draw an open dropdown and take its press. Returns the row the reader chose.
fn open_list(
    ui: &mut Ui,
    plate: Rect,
    labels: &[&str],
    chosen: usize,
    theme: &Theme,
    salt: &'static str,
) -> Option<usize> {
    menu_plate(ui.painter(), plate, theme);
    let mut picked = None;
    for (index, label) in labels.iter().enumerate() {
        let row = Rect::from_min_size(
            egui::pos2(
                plate.left() + MENU_PAD,
                plate.top() + MENU_PAD + index as f32 * MENU_ROW,
            ),
            egui::vec2(plate.width() - 2.0 * MENU_PAD, MENU_ROW),
        );
        let response = ui.interact(row, egui::Id::new((salt, index)), Sense::click());
        if response.hovered() {
            ui.painter().rect_filled(row, MENU_RADIUS, theme.SURFACE_4);
        }
        ui.painter().text(
            egui::pos2(row.left() + MENU_TEXT_PADDING, row.center().y),
            Align2::LEFT_CENTER,
            *label,
            fonts::body(MENU_TEXT_SIZE),
            if index == chosen {
                theme.CONTRAST
            } else {
                theme.BASE
            },
        );
        if index == chosen {
            icons::draw(
                ui.painter(),
                Glyph::Check,
                Rect::from_center_size(
                    egui::pos2(row.right() - MENU_TEXT_PADDING, row.center().y),
                    Vec2::splat(MENU_GLYPH),
                ),
                theme.TEXT_PRIMARY,
            );
        }
        if response.clicked() {
            picked = Some(index);
        }
    }
    picked
}

/// The open card menu: its rows, and the press that carries one out. Returns whether another
/// frame is owed.
fn card_menu(
    ui: &mut Ui,
    theme: &Theme,
    page: Rect,
    state: &mut State,
    menu: &Menu,
    seconds: f64,
    visible: &[Shot],
) -> bool {
    let plate = menu_rect(menu.anchor, MENU_ITEMS.len(), page);
    menu_plate(ui.painter(), plate, theme);

    let mut chosen = None;
    for (index, item) in MENU_ITEMS.iter().enumerate() {
        let row = Rect::from_min_size(
            egui::pos2(
                plate.left() + MENU_PAD,
                plate.top() + MENU_PAD + index as f32 * MENU_ROW,
            ),
            egui::vec2(plate.width() - 2.0 * MENU_PAD, MENU_ROW),
        );
        let response = ui.interact(row, egui::Id::new(("card-menu", index)), Sense::click());
        if response.hovered() {
            ui.painter().rect_filled(row, MENU_RADIUS, theme.SURFACE_4);
        }
        // The source separates Delete with a rule and a red tone (index.vue:1093-1097); the
        // tone is what carries the meaning here, and the row is the last one in the plate.
        let ink = if item.danger() { theme.RED } else { theme.BASE };
        icons::draw(
            ui.painter(),
            item.glyph(),
            Rect::from_center_size(
                egui::pos2(
                    row.left() + MENU_TEXT_PADDING + MENU_GLYPH / 2.0,
                    row.center().y,
                ),
                Vec2::splat(MENU_GLYPH),
            ),
            ink,
        );
        ui.painter().text(
            egui::pos2(
                row.left() + MENU_TEXT_PADDING + MENU_GLYPH + MENU_TEXT_PADDING,
                row.center().y,
            ),
            Align2::LEFT_CENTER,
            item.label(),
            fonts::body(MENU_TEXT_SIZE),
            ink,
        );
        if response.clicked() {
            chosen = Some(*item);
        }
    }

    // A press anywhere else puts the menu away, which is what a menu is for.
    let (pointer, clicked) = ui.input(|input| {
        (input.pointer.interact_pos(), input.pointer.primary_clicked())
    });
    if clicked && !pointer.is_some_and(|at| plate.contains(at)) {
        state.menu = None;
    }

    match chosen {
        Some(Item::Open) => {
            state.menu = None;
            // The card the menu was opened for, at its place in the list the viewer steps
            // through.
            state.viewer = visible
                .iter()
                .position(|shot| shot.path == menu.target)
                .or(Some(0));
            true
        }
        Some(Item::AddToGroup) => {
            // **The list, not a guess.** With one group the reference's menu offers that one; with several it offers
            // them all, and this build used to assign to the first and say so. The list is the row's real behaviour.
            let anchor = menu.anchor;
            let target = menu.target.clone();
            state.menu = None;
            if state.groups.groups.is_empty() {
                state.say(seconds, "Make a group first.");
                return true;
            }
            state.group_menu = Some(GroupMenu { target, anchor });
            return true;
        }
        Some(Item::ShowInFolder) => {
            state.menu = None;
            reveal_in_folder(state, menu, seconds);
            true
        }
        Some(Item::Delete) => {
            state.menu = None;
            state.armed = Some(Arm::Card(menu.target.clone()));
            state.armed_touched = true;
            true
        }
        None => false,
    }
}

/// Ask the desktop to show the card in its file manager.
///
/// The three commands are the platform's own: explorer /select, on Windows, open -R on macOS,
/// and xdg-open on the directory elsewhere. A machine without one is told so, because a press
/// that silently does nothing is worse than one that explains itself.
fn reveal_in_folder(state: &mut State, menu: &Menu, seconds: f64) {
    let Some(folder) = menu.target.parent() else {
        state.say(seconds, "This screenshot has no folder to show");
        return;
    };
    let spawned = if cfg!(target_os = "windows") {
        std::process::Command::new("explorer")
            .arg(format!("/select,{}", menu.target.display()))
            .spawn()
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("open")
            .arg("-R")
            .arg(&menu.target)
            .spawn()
    } else {
        std::process::Command::new("xdg-open").arg(folder).spawn()
    };
    if spawned.is_err() {
        state.say(seconds, "This system has no file manager to open");
    }
}


// ---------------------------------------------------------------------------
// The selection bar
// ---------------------------------------------------------------------------

/// The bar's count: app.screenshots.selection.selected-count, which is {count} selected
/// (index.vue:254-257).
fn selected_count(count: usize) -> String {
    format!("{count} selected")
}

/// The bar's plate for a selection of this size, or nothing when nothing is selected.
///
/// FloatingActionBar is fixed to the viewport's foot with p-4 around a plate of
/// rounded-[20px] bg-surface-3 border border-surface-5 px-3 py-2.5 (FloatingActionBar.vue:238),
/// and mx-auto with a max width. This page centres the plate in its own list column.
fn selection_bar_rect(
    painter: &egui::Painter,
    list: Rect,
    theme: &Theme,
    count: usize,
) -> Option<Rect> {
    if count == 0 {
        return None;
    }
    let text = |contents: String, font: egui::FontId| {
        painter
            .layout_no_wrap(contents, font, theme.CONTRAST)
            .size()
            .x
    };
    let count_width = text(selected_count(count), fonts::semibold(BAR_COUNT_SIZE)) + 2.0 * 16.0;
    let clear_width = text(CLEAR.to_owned(), fonts::semibold(BUTTON_TEXT_SIZE))
        + 2.0 * BUTTON_PADDING;
    let delete_width = 2.0 * BUTTON_PADDING
        + BUTTON_GLYPH
        + BUTTON_GAP
        + text(DELETE.to_owned(), fonts::semibold(BUTTON_TEXT_SIZE));

    let divider = 2.0 * BAR_DIVIDER_MARGIN + STROKE_WIDTH;
    let width = 2.0 * BAR_PADDING_X
        + count_width
        + BAR_GAP
        + divider
        + BAR_GAP
        + clear_width
        + BAR_MARGIN
        + delete_width;
    let height = BAR_BUTTON_HEIGHT + 2.0 * BAR_PADDING_Y;
    let width = width.min(list.width());
    let min = egui::pos2(
        list.center().x - width / 2.0,
        list.bottom() - height - BAR_MARGIN,
    );
    Some(Rect::from_min_size(min, egui::vec2(width, height)))
}

/// Draw the selection's floating bar and take its two presses. Returns whether another frame
/// is owed.
///
/// The source's bar carries "{count} selected", a divider, Clear, New group, Remove from group
/// (when a selected shot is grouped), Export ZIP, a divider and Delete
/// (index.vue:1645-1707). This build has no groups and no archive writer, so the bar carries
/// the actions it can carry out: Clear and Delete.
fn selection_bar(
    ui: &mut Ui,
    bar: Rect,
    theme: &Theme,
    state: &mut State,
    seconds: f64,
    count: usize,
    visible: &[Shot],
) -> bool {
    let painter = ui.painter();
    painter.rect_filled(bar, BAR_RADIUS, theme.SURFACE_3);
    painter.rect_stroke(
        bar,
        BAR_RADIUS,
        Stroke::new(STROKE_WIDTH, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );

    let mut x = bar.left() + BAR_PADDING_X;
    let centre_y = bar.center().y;

    painter.text(
        egui::pos2(x + 16.0, centre_y),
        Align2::LEFT_CENTER,
        selected_count(count),
        fonts::semibold(BAR_COUNT_SIZE),
        theme.CONTRAST,
    );
    x += 16.0
        + painter
            .layout_no_wrap(
                selected_count(count),
                fonts::semibold(BAR_COUNT_SIZE),
                theme.CONTRAST,
            )
            .size()
            .x
        + 16.0
        + BAR_GAP;

    // The divider before Clear, and the one before Delete.
    painter.rect_filled(
        Rect::from_min_size(
            egui::pos2(x + BAR_DIVIDER_MARGIN, centre_y - BAR_DIVIDER_HEIGHT / 2.0),
            egui::vec2(STROKE_WIDTH, BAR_DIVIDER_HEIGHT),
        ),
        0,
        theme.SURFACE_5,
    );
    x += 2.0 * BAR_DIVIDER_MARGIN + STROKE_WIDTH + BAR_GAP;

    let clear_width = painter
        .layout_no_wrap(CLEAR.to_owned(), fonts::semibold(BUTTON_TEXT_SIZE), theme.CONTRAST)
        .size()
        .x
        + 2.0 * BUTTON_PADDING;
    let clear = Rect::from_min_size(
        egui::pos2(x, centre_y - BAR_BUTTON_HEIGHT / 2.0),
        egui::vec2(clear_width, BAR_BUTTON_HEIGHT),
    );
    let clear_response = ui.interact(clear, egui::Id::new("shots-clear"), Sense::click());
    if clear_response.hovered() {
        painter.rect_filled(clear, CONTROL_RADIUS, theme.SURFACE_4);
    }
    painter.text(
        clear.center(),
        Align2::CENTER_CENTER,
        CLEAR,
        fonts::semibold(BUTTON_TEXT_SIZE),
        theme.CONTRAST,
    );

    // Delete, right-aligned, red while armed.
    let armed = state.armed == Some(Arm::Selection);
    let delete_width = 2.0 * BUTTON_PADDING
        + BUTTON_GLYPH
        + BUTTON_GAP
        + painter
            .layout_no_wrap(DELETE.to_owned(), fonts::semibold(BUTTON_TEXT_SIZE), theme.CONTRAST)
            .size()
            .x;
    let delete = Rect::from_min_size(
        egui::pos2(bar.right() - BAR_PADDING_X - delete_width, centre_y - BAR_BUTTON_HEIGHT / 2.0),
        egui::vec2(delete_width, BAR_BUTTON_HEIGHT),
    );
    let delete_response = ui.interact(delete, egui::Id::new("shots-delete"), Sense::click());
    if armed || delete_response.hovered() {
        painter.rect_filled(delete, CONTROL_RADIUS, theme.RED);
    }
    let delete_ink = if armed { theme.CONTRAST } else { theme.RED };
    icons::draw(
        painter,
        Glyph::Trash,
        Rect::from_center_size(
            egui::pos2(
                delete.left() + BUTTON_PADDING + BUTTON_GLYPH / 2.0,
                delete.center().y,
            ),
            Vec2::splat(BUTTON_GLYPH),
        ),
        if armed { theme.CONTRAST } else { delete_ink },
    );
    painter.text(
        egui::pos2(
            delete.left() + BUTTON_PADDING + BUTTON_GLYPH + BUTTON_GAP,
            delete.center().y,
        ),
        Align2::LEFT_CENTER,
        // The armed label is the question the second press answers.
        if armed {
            format!("{DELETE} {count}?")
        } else {
            DELETE.to_owned()
        },
        fonts::semibold(BUTTON_TEXT_SIZE),
        if armed { theme.CONTRAST } else { delete_ink },
    );

    // The divider before Delete, at its left.
    painter.rect_filled(
        Rect::from_min_size(
            egui::pos2(
                delete.left() - BAR_DIVIDER_MARGIN - STROKE_WIDTH,
                centre_y - BAR_DIVIDER_HEIGHT / 2.0,
            ),
            egui::vec2(STROKE_WIDTH, BAR_DIVIDER_HEIGHT),
        ),
        0,
        theme.SURFACE_5,
    );

    let mut owed = false;
    if clear_response.clicked() {
        state.selected.clear();
        state.armed = None;
        state.armed_touched = true;
        owed = true;
    }
    if delete_response.clicked() {
        state.armed_touched = true;
        if armed {
            let paths: Vec<PathBuf> = state
                .selection(visible)
                .into_iter()
                .map(|shot| shot.path)
                .collect();
            delete_paths(state, &paths, seconds);
        } else {
            state.armed = Some(Arm::Selection);
        }
        owed = true;
    }
    owed
}

/// Remove the paths and re-read the disk, saying what happened.
fn delete_paths(state: &mut State, paths: &[PathBuf], seconds: f64) {
    match remove_files(paths) {
        Ok(count) => {
            for path in paths {
                state.selected.remove(path);
            }
            state.armed = None;
            state.menu = None;
            // The re-scan after a delete can fail too, and says so rather than showing an
            // emptied page as if the files were gone.
            match read_shots() {
                Ok(shots) => {
                    state.shots = shots;
                    state.error = None;
                }
                Err(message) => state.error = Some(message),
            }
            state.scanned = true;
            state.say(seconds, deleted_sentence(count));
        }
        Err(message) => {
            state.armed = None;
            state.say(seconds, message);
        }
    }
}

// ---------------------------------------------------------------------------
// The toast
// ---------------------------------------------------------------------------

/// The sentence the last action owes, at the foot of the page, for a couple of seconds.
fn toast(
    ui: &mut Ui,
    theme: &Theme,
    state: &mut State,
    seconds: f64,
    bar: Option<Rect>,
) -> bool {
    let Some(current) = state.toast.clone() else {
        return false;
    };
    if seconds >= current.until {
        state.toast = None;
        return true;
    }
    let painter = ui.painter();
    let galley = painter.layout_no_wrap(
        current.text.clone(),
        fonts::body(TOAST_TEXT_SIZE),
        theme.CONTRAST,
    );
    let plate = Rect::from_center_size(
        egui::pos2(
            bar.map_or(ui.max_rect().center().x, |bar| bar.center().x),
            bar.map_or(ui.max_rect().bottom() - 32.0, |bar| bar.top() - 16.0)
                - galley.size().y / 2.0,
        ),
        egui::vec2(
            galley.size().x + 2.0 * TOAST_PADDING_X,
            galley.size().y + 2.0 * TOAST_PADDING_Y,
        ),
    );
    painter.rect_filled(plate, MENU_RADIUS, theme.SUPER_RAISED_BG);
    painter.rect_stroke(
        plate,
        MENU_RADIUS,
        Stroke::new(STROKE_WIDTH, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );
    painter.galley(
        egui::pos2(
            plate.left() + TOAST_PADDING_X,
            plate.top() + TOAST_PADDING_Y,
        ),
        galley,
        theme.CONTRAST,
    );
    ui.ctx()
        .request_repaint_after(std::time::Duration::from_millis(100));
    false
}

// ---------------------------------------------------------------------------
// The viewer
// ---------------------------------------------------------------------------

/// One screenshot, full size, over the page: the source's ImageViewerEditor at the index the
/// pressed card holds (index.vue:1439-1478). Esc or a click outside the picture closes it;
/// the arrow keys step through the list the card came from.
fn viewer(
    ui: &mut Ui,
    page: Rect,
    theme: &Theme,
    images: &mut Images,
    state: &mut State,
    visible: &[Shot],
) -> bool {
    let Some(index) = state.viewer else {
        return false;
    };
    let Some(shot) = visible.get(index.min(visible.len().saturating_sub(1))) else {
        state.viewer = None;
        return false;
    };

    let painter = ui.painter();
    painter.rect_filled(page, 0, Color32::from_black_alpha(216));

    // The picture keeps its aspect inside the page's box, less a margin for the caption.
    let box_rect = Rect::from_min_max(
        egui::pos2(page.left() + PAGE_PADDING * 2.0, page.top() + PAGE_PADDING * 2.0),
        egui::pos2(page.right() - PAGE_PADDING * 2.0, page.bottom() - PAGE_PADDING * 3.0),
    );
    let mut picture = box_rect;
    if let Some(texture) = images.get_file(ui.ctx(), &shot.path) {
        let size = texture.size_vec2();
        let scale = (box_rect.width() / size.x).min(box_rect.height() / size.y);
        picture = Rect::from_center_size(box_rect.center(), size * scale);
        painter.image(
            texture.id(),
            picture,
            Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    }

    // The caption under the picture, and the count of where the reader is.
    let caption = format!(
        "{}  ·  {} of {}",
        shot.file_name,
        index + 1,
        visible.len()
    );
    painter.text(
        egui::pos2(box_rect.center().x, box_rect.bottom() + 16.0),
        Align2::CENTER_TOP,
        caption,
        fonts::body(DATE_SIZE),
        theme.TEXT_PRIMARY,
    );

    // The close button, top right of the page.
    let close = Rect::from_center_size(
        egui::pos2(page.right() - PAGE_PADDING - 18.0, page.top() + PAGE_PADDING + 18.0),
        Vec2::splat(36.0),
    );
    painter.circle_filled(close.center(), 18.0, theme.SURFACE_3);
    icons::draw(painter, Glyph::X, close.shrink(10.0), theme.CONTRAST);

    let mut owed = false;
    if ui
        .interact(close, egui::Id::new("shots-viewer-close"), Sense::click())
        .clicked()
    {
        state.viewer = None;
        return true;
    }
    let (clicked, pointer, escape, step) = ui.input(|input| {
        let escape = input.key_pressed(egui::Key::Escape);
        let step = if input.key_pressed(egui::Key::ArrowRight) {
            1_i32
        } else if input.key_pressed(egui::Key::ArrowLeft) {
            -1_i32
        } else {
            0
        };
        (
            input.pointer.primary_clicked(),
            input.pointer.interact_pos(),
            escape,
            step,
        )
    });
    if escape {
        state.viewer = None;
        return true;
    }
    if clicked && !pointer.is_some_and(|at| picture.contains(at)) {
        state.viewer = None;
        return true;
    }
    if step != 0 && !visible.is_empty() {
        let next = (index as i32 + step).clamp(0, visible.len() as i32 - 1);
        state.viewer = Some(next as usize);
        owed = true;
    }
    owed
}


// ---------------------------------------------------------------------------
// The groups and their cards
// ---------------------------------------------------------------------------

/// One group: the source's header row (section.vue:112-154) over its grid (group.vue:208).
///
/// The base is the index of this group's first shot in the list the viewer steps through, so
/// a card press can name its place without a second search.
#[allow(clippy::too_many_arguments)]
fn draw_group(
    ui: &mut Ui,
    min: egui::Pos2,
    width: f32,
    grid: Grid,
    group: &Group,
    base: usize,
    theme: &Theme,
    images: &mut Images,
    state: &mut State,
    blocked: &[Rect],
    probe_rows: &mut Vec<(String, Rect)>,
) {
    let grid_top;

    if group.header {
        let header = Rect::from_min_size(min, egui::vec2(width, GROUP_HEADER_HEIGHT));
        // The rule under the header: border-b border-surface-5 (section.vue:114), inside the
        // 40-point row because the sheet's box-sizing is border-box.
        ui.painter().rect_filled(
            Rect::from_min_size(
                egui::pos2(min.x, header.bottom() - STROKE_WIDTH),
                egui::vec2(width, STROKE_WIDTH),
            ),
            0,
            theme.SURFACE_5,
        );

        // The whole header is the accordion's open/close target (section.vue:117-136).
        let response = ui.interact(
            header,
            egui::Id::new(("shot-group", group.id.as_str())),
            Sense::click(),
        );
        if response.clicked() {
            if !state.collapsed.remove(&group.id) {
                state.collapsed.insert(group.id.clone());
            }
            ui.ctx().request_repaint();
        }
        let open = !state.collapsed.contains(&group.id);

        // DropdownIcon at size-5 text-secondary, rotated 180 degrees while the group is open
        // (section.vue:124-127): the sheet's chevron points down, so an open group's points up.
        icons::draw(
            ui.painter(),
            if open {
                Glyph::ChevronUp
            } else {
                Glyph::ChevronDown
            },
            Rect::from_center_size(
                egui::pos2(min.x + HEADER_GLYPH / 2.0, header.center().y),
                Vec2::splat(HEADER_GLYPH),
            ),
            theme.SECONDARY,
        );

        // The title lifts from text-primary to text-contrast on the header's hover
        // (section.vue:125-126,134).
        let title = ui.painter().text(
            egui::pos2(min.x + HEADER_GLYPH + HEADER_GAP, header.center().y),
            Align2::LEFT_CENTER,
            &group.title,
            fonts::semibold(GROUP_TITLE_SIZE),
            if response.hovered() {
                theme.CONTRAST
            } else {
                theme.TEXT_PRIMARY
            },
        );

        count_pill(
            ui.painter(),
            title.right() + HEADER_GAP,
            header.center().y,
            group.shots.len(),
            theme,
        );

        probe_rows.push((format!("group-title {}", group.title), header));
        if !open {
            return;
        }
        grid_top = header.bottom() + GROUP_CONTENT_SPACING;
    } else {
        grid_top = min.y + GROUP_CONTENT_SPACING;
    }

    // The grid, mt-2.5 under the header (section.vue:164) at gap-3 (group.vue:210), in the
    // source's absolute positions: column-major within a row, rows one card apart.
    let pointer = ui.input(|input| input.pointer.interact_pos());
    let under_overlay = pointer.is_some_and(|at| blocked.iter().any(|rect| rect.contains(at)));

    for (index, shot) in group.shots.iter().enumerate() {
        let column = index % grid.columns.max(1);
        let row = index / grid.columns.max(1);
        let card = Rect::from_min_size(
            egui::pos2(
                min.x + column as f32 * (grid.card_width + GRID_GAP),
                grid_top + row as f32 * (grid.card_height + GRID_GAP),
            ),
            egui::vec2(grid.card_width, grid.card_height),
        );
        if card.width() < 1.0 || card.height() < 1.0 {
            continue;
        }

        let response = ui.interact(
            card,
            egui::Id::new(("shot-card", &shot.path)),
            Sense::click(),
        );
        let hovered = response.hovered() && !under_overlay;
        let selected = state.is_selected(&shot.path);
        let armed = matches!(&state.armed, Some(Arm::Card(path)) if path == &shot.path);

        let hits = draw_card(ui.painter(), card, theme, shot, images, hovered, selected);
        probe_rows.push((format!("card {}", shot.file_name), card));
        probe_rows.push((format!("check {}", shot.file_name), hits.check));
        if let Some(actions) = hits.actions {
            for (slot, index) in actions.iter().zip(0..) {
                probe_rows.push((format!("action{index} {}", shot.file_name), *slot));
            }
        }

        if under_overlay {
            continue;
        }

        // The arming strip is a target of its own and it sits inside the card, so it is read
        // before the card is: a press that confirms a delete must not also open the picture.
        if armed {
            let strip = Rect::from_min_size(
                egui::pos2(card.left(), card.bottom() - ARMED_STRIP_HEIGHT),
                egui::vec2(card.width(), ARMED_STRIP_HEIGHT),
            );
            draw_armed_strip(ui.painter(), strip, theme, &shot.file_name);
            let confirm = ui.interact(
                strip,
                egui::Id::new(("shot-armed", &shot.path)),
                Sense::click(),
            );
            if confirm.clicked() {
                state.armed_touched = true;
                let seconds = ui.ctx().input(|input| input.time);
                delete_paths(state, &[shot.path.clone()], seconds);
                return;
            }
            continue;
        }

        if response.clicked() {
            let shift = ui.input(|input| input.modifiers.shift);
            let on_check = pointer.is_some_and(|at| hits.check.contains(at));
            // activateScreenshot (index.vue:1022-1046): a press toggles when a selection is
            // already active or Shift is held, and otherwise opens the picture.
            if on_check || !state.selected.is_empty() || shift {
                state.toggle(&shot.path);
            } else {
                state.viewer = Some(base + index);
            }
            ui.ctx().request_repaint();
        }

        if response.secondary_clicked() {
            state.menu = Some(Menu {
                target: shot.path.clone(),
                anchor: pointer.unwrap_or_else(|| card.left_top()),
            });
            state.armed = None;
        }

        // The caption's three buttons, which the reference draws only while the card is
        // hovered (card.vue:243-275).
        if let Some(actions) = hits.actions {
            for (slot, action) in [
                (actions[0], Action::Open),
                (actions[1], Action::Delete),
                (actions[2], Action::More),
            ] {
                let pressed = ui
                    .interact(
                        slot,
                        egui::Id::new(("shot-action", &shot.path, action as u8)),
                        Sense::click(),
                    )
                    .clicked();
                if !pressed {
                    continue;
                }
                match action {
                    Action::Open => state.viewer = Some(base + index),
                    Action::Delete => {
                        state.armed = Some(Arm::Card(shot.path.clone()));
                        state.armed_touched = true;
                    }
                    Action::More => {
                        state.menu = Some(Menu {
                            target: shot.path.clone(),
                            anchor: egui::pos2(slot.left(), slot.bottom() + MENU_GAP),
                        });
                    }
                }
                ui.ctx().request_repaint();
            }
        }
    }
}

/// Which of the caption's buttons a press belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    /// Show the screenshot full size.
    Open = 0,
    /// Arm its delete.
    Delete = 1,
    /// Open the card's menu.
    More = 2,
}

/// The rects a drawn card offers as targets.
struct CardHits {
    /// The selection checkbox's 50-point hit box.
    check: Rect,
    /// The three caption buttons, when the caption is drawn.
    actions: Option<[Rect; 3]>,
}

/// The selection checkbox's hit box: size-[50px] at right-0.5 top-0 pt-4 (card.vue:196-197).
fn check_hit(card: Rect) -> Rect {
    Rect::from_min_size(
        egui::pos2(card.right() - CHECK_RIGHT_INSET - CHECK_HIT, card.top()),
        egui::vec2(CHECK_HIT, CHECK_HIT),
    )
}

/// One screenshot card.
///
/// The article is aspect-video min-w-0 overflow-hidden rounded-xl border border-surface-5
/// bg-surface-2 (card.vue:172), filled by the thumbnail and captioned by the hover overlay at
/// its foot (card.vue:232-277).
fn draw_card(
    painter: &egui::Painter,
    rect: Rect,
    theme: &Theme,
    shot: &Shot,
    images: &mut Images,
    hovered: bool,
    selected: bool,
) -> CardHits {
    // The card's own fill, under the picture (card.vue:172).
    painter.rect_filled(rect, CARD_RADIUS, theme.SURFACE_2);

    // The thumbnail, object-cover: the picture keeps its aspect and fills the card, and the
    // excess is cut off. The source asks for a 512-point thumbnail (card.vue:37-41); this
    // decodes the file once and keeps the texture, the trade the instance page makes too.
    if let Some(texture) = images.get_file(painter.ctx(), &shot.path) {
        let uv = cover_uv(texture.size_vec2(), rect.size());
        painter.image(texture.id(), rect, uv, Color32::WHITE);
    } else {
        // With no picture the reference shows bg-button-bg (card.vue:227-231).
        painter.rect_filled(rect, CARD_RADIUS, theme.BUTTON_BG);
    }

    // hover:brightness-110, and the same again when selected: a light wash over the picture,
    // which is what brightness does to a photograph.
    if hovered || selected {
        painter.rect_filled(rect, CARD_RADIUS, Color32::from_white_alpha(18));
    }

    if hovered {
        draw_overlay(painter, rect, theme, shot);
    }

    // The checkbox, revealed on hover and always visible once selected (card.vue:206-214).
    let check = check_hit(rect);
    if hovered || selected {
        let disc = Rect::from_min_size(
            egui::pos2(
                check.center().x - CHECK_DISC / 2.0,
                check.top() + CHECK_TOP_PAD,
            ),
            Vec2::splat(CHECK_DISC),
        );
        if selected {
            painter.circle_filled(disc.center(), CHECK_DISC / 2.0, theme.CONTRAST);
            icons::draw(
                painter,
                Glyph::Check,
                Rect::from_center_size(disc.center(), Vec2::splat(CHECK_GLYPH)),
                theme.SURFACE_1,
            );
        } else {
            painter.circle_stroke(
                disc.center(),
                CHECK_DISC / 2.0 - CHECK_BORDER / 2.0,
                Stroke::new(CHECK_BORDER, theme.TEXT_PRIMARY),
            );
        }
    }

    // The border last, over the scrim: border border-surface-5 (card.vue:172), which the
    // selected state replaces with contrast (card.vue:174).
    painter.rect_stroke(
        rect,
        CARD_RADIUS,
        Stroke::new(
            if selected {
                STROKE_WIDTH * 2.0
            } else {
                STROKE_WIDTH
            },
            if selected {
                theme.CONTRAST
            } else {
                theme.SURFACE_5
            },
        ),
        egui::StrokeKind::Inside,
    );

    let actions = if hovered {
        Some([
            action_button_rect(rect, 0),
            action_button_rect(rect, 1),
            action_button_rect(rect, 2),
        ])
    } else {
        None
    };
    CardHits { check, actions }
}

/// The rect of the n-th caption button, laid out from the card's inner right edge.
fn action_button_rect(card: Rect, index: usize) -> Rect {
    let inner_right = card.right() - OVERLAY_PADDING;
    let step = ACTION_SIZE + ACTION_GAP;
    Rect::from_min_size(
        egui::pos2(
            inner_right - ACTION_SIZE - (2 - index) as f32 * step,
            card.bottom() - OVERLAY_PADDING - ACTION_SIZE,
        ),
        Vec2::splat(ACTION_SIZE),
    )
}

/// The unit rectangle that crops a picture to fill a box without distorting it: object-cover.
fn cover_uv(texture: Vec2, into: Vec2) -> Rect {
    if texture.x <= 0.0 || texture.y <= 0.0 || into.x <= 0.0 || into.y <= 0.0 {
        return Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));
    }
    let scale = (into.x / texture.x).max(into.y / texture.y);
    let shown = Vec2::new(into.x / scale / texture.x, into.y / scale / texture.y);
    Rect::from_min_max(
        egui::pos2((1.0 - shown.x) / 2.0, (1.0 - shown.y) / 2.0),
        egui::pos2((1.0 + shown.x) / 2.0, (1.0 + shown.y) / 2.0),
    )
}

/// The card's foot: the gradient, the file name, the instance-and-date line and the three
/// actions (card.vue:232-277).
///
/// The reference fades this in on hover and slides the actions up 4 points as it arrives, so it
/// is drawn only while the card is hovered.
///
/// The gradient is bg-gradient-to-t from-surface-1 to-transparent with pt-[120px]
/// (card.vue:233). Three stacked bands sample that ramp instead of a per-pixel mesh, and they
/// are clipped to the plate: surface-1 is the page's own colour in both schemes, so the corner
/// triangles the rectangular clip cannot round out do not show.
fn draw_overlay(painter: &egui::Painter, plate: Rect, theme: &Theme, shot: &Shot) {
    let painter = painter.with_clip_rect(plate);

    let fade = OVERLAY_FADE.min(plate.height());
    for (share, alpha) in [
        (1.0, SCRIM_ALPHA_TOP),
        (2.0 / 3.0, SCRIM_ALPHA_MIDDLE),
        (1.0 / 3.0, SCRIM_ALPHA_BOTTOM),
    ] {
        let band = Rect::from_min_max(
            egui::pos2(plate.left(), plate.bottom() - fade * share),
            plate.max,
        );
        painter.rect_filled(band, 0, scrim(theme, alpha));
    }

    let actions_width = 3.0 * ACTION_SIZE + 2.0 * ACTION_GAP;
    let inner_left = plate.left() + OVERLAY_PADDING;
    let inner_right = plate.right() - OVERLAY_PADDING;
    let actions_left = inner_right - actions_width;
    // The actions are dropped rather than overlapped on a card too narrow to hold them and its
    // text: the text is what the caption is for.
    let text_width = if actions_left - OVERLAY_GAP - inner_left > 0.0 {
        actions_left - OVERLAY_GAP - inner_left
    } else {
        inner_right - inner_left
    };

    // Two lines, bottom-aligned: the file name and the instance-and-date line
    // (card.vue:236-241).
    let date_top = plate.bottom() - OVERLAY_PADDING - DATE_LINE;
    clipped_line(
        &painter,
        Rect::from_min_size(
            egui::pos2(inner_left, date_top - CAPTION_LINE),
            egui::vec2(text_width.max(1.0), CAPTION_LINE),
        ),
        &shot.file_name,
        fonts::semibold(CAPTION_SIZE),
        theme.CONTRAST,
    );
    // app.screenshots.preview.instance-and-date: {instance} · {date}
    // (locales/en-US/index.json:1574).
    clipped_line(
        &painter,
        Rect::from_min_size(
            egui::pos2(inner_left, date_top),
            egui::vec2(text_width.max(1.0), DATE_LINE),
        ),
        &instance_and_date(shot.clone()),
        fonts::body(DATE_SIZE),
        theme.SECONDARY,
    );

    // The three quiet IconButtons, carrying bg-surface-2 text-contrast (card.vue:248-275),
    // circular by default (IconButton.vue:29) and 36 points across.
    let glyphs = [Glyph::Image, Glyph::Trash, Glyph::MoreHorizontal];
    for (index, glyph) in glyphs.into_iter().enumerate() {
        let slot = action_button_rect(plate, index);
        painter.circle_filled(slot.center(), ACTION_SIZE / 2.0, theme.SURFACE_2);
        icons::draw(
            &painter,
            glyph,
            Rect::from_center_size(slot.center(), Vec2::splat(ACTION_GLYPH)),
            theme.CONTRAST,
        );
    }
}

/// The strip a card wears while its delete waits for the second press.
fn draw_armed_strip(painter: &egui::Painter, strip: Rect, theme: &Theme, name: &str) {
    painter.rect_filled(strip, 0, theme.RED);
    clipped_line(
        painter,
        strip.shrink2(egui::vec2(OVERLAY_PADDING, 4.0)),
        &format!("Delete {name}? Press again"),
        fonts::semibold(DATE_SIZE),
        theme.CONTRAST,
    );
}

/// One band of the overlay's scrim: surface-1 at alpha, which is the from-surface-1 end of the
/// source's gradient (card.vue:233).
fn scrim(theme: &Theme, alpha: u8) -> Color32 {
    let base = theme.SURFACE_1;
    Color32::from_rgba_unmultiplied(base.r(), base.g(), base.b(), alpha)
}

/// Draw one line of text, clipped to its own box.
///
/// The overlay truncates both of its lines (truncate, card.vue:236,239): a long file name or
/// instance ends at the caption's edge instead of wrapping into the line below it.
fn clipped_line(painter: &egui::Painter, rect: Rect, text: &str, font: egui::FontId, ink: Color32) {
    let galley = painter.layout(text.to_owned(), font, ink, rect.width());
    painter.with_clip_rect(rect).galley(rect.min, galley, ink);
}


// ---------------------------------------------------------------------------
// The empty states
// ---------------------------------------------------------------------------

/// The source's EmptyState (base/EmptyState.vue): a 200-point illustration, the heading at
/// 24/32 semibold text-contrast, 6 points, and the description at 16/24 text-secondary,
/// centred in the area it is given — the page root is justify-center while nothing is listed
/// (index.vue:1483).
fn empty_state(
    painter: &egui::Painter,
    rect: Rect,
    theme: &Theme,
    glyph: Glyph,
    heading: &str,
    body: &str,
) {
    let total = ILLUSTRATION_HEIGHT + HEADING_LINE + EMPTY_TEXT_GAP + EMPTY_BODY_LINE;
    if rect.height() < total || rect.width() < 1.0 {
        return;
    }
    let top = rect.center().y - total / 2.0;
    let centre_x = rect.center().x;

    // The reference's own illustration, at its own 228x98, centred in the box the source gives it
    // (`h-[200px] w-auto`); the icon set's glyph is the fallback for a texture that failed to upload.
    let art = Rect::from_center_size(
        egui::pos2(centre_x, top + ILLUSTRATION_HEIGHT / 2.0),
        egui::vec2(228.0, 98.0),
    );
    if !crate::serverart::paint_shots_empty(painter, art) {
        icons::draw(
            painter,
            glyph,
            Rect::from_center_size(
                egui::pos2(centre_x, top + ILLUSTRATION_HEIGHT / 2.0),
                Vec2::splat(ILLUSTRATION_GLYPH),
            ),
            theme.SECONDARY,
        );
    }

    let heading_y = top + ILLUSTRATION_HEIGHT + HEADING_LINE / 2.0;
    painter.text(
        egui::pos2(centre_x, heading_y),
        Align2::CENTER_CENTER,
        heading,
        fonts::semibold(HEADING_SIZE),
        theme.CONTRAST,
    );
    painter.text(
        egui::pos2(
            centre_x,
            heading_y + HEADING_LINE / 2.0 + EMPTY_TEXT_GAP + EMPTY_BODY_LINE / 2.0,
        ),
        Align2::CENTER_CENTER,
        body,
        fonts::body(EMPTY_BODY_SIZE),
        theme.SECONDARY,
    );
}

/// The error state: `EmptyState type="error"` with the query's message and a Retry button
/// (`index.vue:1506-1517`). Returns whether the reader pressed Retry.
fn error_state(ui: &mut Ui, rect: Rect, theme: &Theme, message: &str) -> bool {
    let painter = ui.painter();
    let retry_height = CONTROL_HEIGHT;
    let total = ILLUSTRATION_HEIGHT + HEADING_LINE + EMPTY_TEXT_GAP + EMPTY_BODY_LINE
        + EMPTY_TEXT_GAP * 2.0
        + retry_height;
    if rect.height() < total || rect.width() < 1.0 {
        return false;
    }
    let top = rect.center().y - total / 2.0;
    let centre_x = rect.center().x;

    icons::draw(
        painter,
        Glyph::CircleAlert,
        Rect::from_center_size(
            egui::pos2(centre_x, top + ILLUSTRATION_HEIGHT / 2.0),
            Vec2::splat(ILLUSTRATION_GLYPH),
        ),
        theme.SECONDARY,
    );
    let heading_y = top + ILLUSTRATION_HEIGHT + HEADING_LINE / 2.0;
    painter.text(
        egui::pos2(centre_x, heading_y),
        Align2::CENTER_CENTER,
        ERROR_HEADING,
        fonts::semibold(HEADING_SIZE),
        theme.CONTRAST,
    );
    painter.text(
        egui::pos2(
            centre_x,
            heading_y + HEADING_LINE / 2.0 + EMPTY_TEXT_GAP + EMPTY_BODY_LINE / 2.0,
        ),
        Align2::CENTER_CENTER,
        message,
        fonts::body(EMPTY_BODY_SIZE),
        theme.SECONDARY,
    );

    // The Retry button: an outlined `Button` under the message.
    let width = text_button_width(painter, RETRY, theme);
    let button = Rect::from_min_size(
        egui::pos2(
            centre_x - width / 2.0,
            heading_y + HEADING_LINE / 2.0 + EMPTY_TEXT_GAP + EMPTY_BODY_LINE + EMPTY_TEXT_GAP * 2.0,
        ),
        egui::vec2(width, retry_height),
    );
    painter.rect_filled(button, CONTROL_RADIUS, theme.SURFACE_4);
    painter.rect_stroke(
        button,
        CONTROL_RADIUS,
        Stroke::new(STROKE_WIDTH, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );
    painter.text(
        button.center(),
        Align2::CENTER_CENTER,
        RETRY,
        fonts::semibold(BUTTON_TEXT_SIZE),
        theme.CONTRAST,
    );
    // The press is read from the pointer, the way the toolbar's are: this page registers its
    // controls that way after the ordering that steals an `interact` from a rect under another.
    ui.input(|input| {
        input.pointer.primary_clicked()
            && input
                .pointer
                .interact_pos()
                .is_some_and(|at| button.contains(at))
    })
}

// ---------------------------------------------------------------------------
// Reading the disk
// ---------------------------------------------------------------------------

/// The folder the game writes into under an instance's root: screenshots/. The reference lists
/// the same folder (instance_list_screenshots, helpers/instance.ts:199-205).
const SCREENSHOTS_DIR: &str = "screenshots";

/// The two formats the page lists: the game writes PNG, and a screenshot folder may also hold
/// JPEGs. Any other file in the folder is ignored.
const SHOT_EXTENSIONS: [&str; 2] = ["png", "jpg"];

/// The game directory inside an instance's root, where Minecraft writes its own folders.
const GAME_DIR: &str = "minecraft";

/// Seconds in a day: 24 * 60 * 60.
const SECONDS_PER_DAY: i64 = 86_400;

/// Seconds in an hour: 60 * 60.
const SECONDS_PER_HOUR: i64 = 3_600;

/// Seconds in a minute.
const SECONDS_PER_MINUTE: i64 = 60;

// The constants below are Howard Hinnant's civil_from_days
// (<https://howardhinnant.github.io/date_algorithms.html#civil_from_days>), which is the
// standard days-to-civil-date algorithm: no table, no dependency, only integer arithmetic.

/// Days from Hinnant's civil epoch, 0000-03-01, to the Unix epoch, 1970-01-01.
const CIVIL_EPOCH_DAYS: i64 = 719_468;

/// Days in one 400-year Gregorian era: 400 * 365 + 97 leap days.
const CIVIL_ERA_DAYS: i64 = 146_097;

/// Days in four common years: 4 * 365, the first term of the year-of-era estimate.
const CIVIL_FOUR_YEAR_DAYS: i64 = 1_460;

/// The century term of that estimate: 146097 / 4 rounded down.
const CIVIL_CENTURY_DAYS: i64 = 36_524;

/// The last day index of an era: 146097 - 1, the estimate's final term.
const CIVIL_ERA_LAST_DAY: i64 = 146_096;

/// A common year: 365 days.
const CIVIL_YEAR_DAYS: i64 = 365;

/// Years in an era.
const CIVIL_ERA_YEARS: i64 = 400;

/// Leap years recur every 4 years...
const CIVIL_LEAP_CYCLE: i64 = 4;

/// ...except every 100, which the era's 400-year rule restores.
const CIVIL_CENTURY_CYCLE: i64 = 100;

/// The March-based month table's stride: five months of 30/31 days make 153.
const CIVIL_MONTH_STRIDE: i64 = 153;

/// The table's numerator: five months per block.
const CIVIL_MONTH_BLOCK: i64 = 5;

/// The + 2 that rounds the month table's division up to the next boundary.
const CIVIL_MONTH_BIAS: i64 = 2;

/// March is month index 0, so a March-November index gains 3 to become a civil month.
const CIVIL_MONTH_OFFSET: i64 = 3;

/// January and February are indices 10 and 11; subtracting 9 makes them 1 and 2.
const CIVIL_YEAR_WRAP: i64 = 9;

/// The first index that is January in the March-based year.
const CIVIL_WRAP_INDEX: i64 = 10;

/// January and February belong to the next civil year.
const CIVIL_LAST_WRAPPED_MONTH: i64 = 2;

/// Day indexes are one-based.
const CIVIL_FIRST_DAY: i64 = 1;

/// A Unix timestamp as the page's local-agnostic [Stamp], in UTC.
///
/// This is Hinnant's civil_from_days after splitting the count into whole days and the time of
/// day. A timestamp before the epoch clamps to it: a file with a 1969 modification time is
/// still a screenshot, and returning a Stamp rather than panicking keeps the page drawable.
fn stamp_from_unix(seconds: i64) -> Stamp {
    let seconds = seconds.max(0);
    let days = seconds / SECONDS_PER_DAY;
    let time = seconds % SECONDS_PER_DAY;

    let shifted = days + CIVIL_EPOCH_DAYS;
    let era = shifted / CIVIL_ERA_DAYS;
    let day_of_era = shifted - era * CIVIL_ERA_DAYS;
    let year_of_era = (day_of_era - day_of_era / CIVIL_FOUR_YEAR_DAYS
        + day_of_era / CIVIL_CENTURY_DAYS
        - day_of_era / CIVIL_ERA_LAST_DAY)
        / CIVIL_YEAR_DAYS;
    let mut year = year_of_era + era * CIVIL_ERA_YEARS;
    let day_of_year = day_of_era
        - (CIVIL_YEAR_DAYS * year_of_era + year_of_era / CIVIL_LEAP_CYCLE
            - year_of_era / CIVIL_CENTURY_CYCLE);
    let month_index = (CIVIL_MONTH_BLOCK * day_of_year + CIVIL_MONTH_BIAS) / CIVIL_MONTH_STRIDE;
    let day = day_of_year - (CIVIL_MONTH_STRIDE * month_index + CIVIL_MONTH_BIAS) / CIVIL_MONTH_BLOCK
        + CIVIL_FIRST_DAY;
    let month = if month_index < CIVIL_WRAP_INDEX {
        month_index + CIVIL_MONTH_OFFSET
    } else {
        month_index - CIVIL_YEAR_WRAP
    };
    if month <= CIVIL_LAST_WRAPPED_MONTH {
        year += 1;
    }

    Stamp {
        year: year.min(i64::from(i32::MAX)) as i32,
        month: month as u8,
        day: day as u8,
        hour: (time / SECONDS_PER_HOUR) as u8,
        minute: (time % SECONDS_PER_HOUR / SECONDS_PER_MINUTE) as u8,
    }
}

/// Every screenshot on disk: each instance's screenshots/ folder, one level deep, in the two
/// formats [SHOT_EXTENSIONS] names. A folder or file that cannot be read is skipped — the same
/// rule the launcher's data loading uses — because one unreadable instance must not hide the
/// rest.
fn read_shots() -> Result<Vec<Shot>, String> {
    let mut shots = Vec::new();
    let instances = match fs::read_dir(crate::data::instances_dir()) {
        Ok(read) => read,
        // No instances folder yet is an ordinary first run, not an error; anything else is a
        // folder that exists and cannot be read, which the page must say rather than draw as an
        // empty one.
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(shots),
        Err(error) => {
            return Err(format!(
                "The screenshots folder could not be read: {error}"
            ))
        }
    };
    for instance in instances.flatten() {
        // The instance root is the Prism-style folder; the game writes into its `minecraft/`
        // child, and an instance made by another launcher may keep `screenshots/` at the root.
        // Both are read, the game's first — the same pair the instance page's Screenshots tab
        // looks in, and the same folder the reference's `instance_list_screenshots` walks.
        let screenshots = match fs::read_dir(instance.path().join(GAME_DIR).join(SCREENSHOTS_DIR)) {
            Ok(read) => read,
            Err(_) => {
                let Ok(read) = fs::read_dir(instance.path().join(SCREENSHOTS_DIR)) else {
                    continue;
                };
                read
            }
        };
        let instance_name = instance.file_name().to_string_lossy().into_owned();
        for file in screenshots.flatten() {
            let path = file.path();
            let extension = path
                .extension()
                .and_then(|ext| ext.to_str())
                .map_or("", |ext| ext);
            if !SHOT_EXTENSIONS
                .iter()
                .any(|known| extension.eq_ignore_ascii_case(*known))
            {
                continue;
            }
            let Ok(metadata) = file.metadata() else {
                continue;
            };
            if !metadata.is_file() {
                continue;
            }
            let Ok(modified) = metadata.modified() else {
                continue;
            };
            // A modification time before the epoch is clamped rather than dropping the file:
            // it is still a screenshot, and stamp_from_unix clamps the same way.
            let seconds = modified
                .duration_since(UNIX_EPOCH)
                .map_or(0, |since_epoch| {
                    since_epoch.as_secs().min(i64::MAX as u64) as i64
                });
            shots.push(Shot {
                instance: instance_name.clone(),
                file_name: file.file_name().to_string_lossy().into_owned(),
                taken: stamp_from_unix(seconds),
                path,
            });
        }
    }
    Ok(shots)
}


#[cfg(test)]
mod tests {
    use super::*;

    /// A shot with only the fields a grouping test reads. The path is the name, which is all
    /// a test needs from it.
    fn shot(instance: &'static str, name: &'static str, year: i32, month: u8, day: u8) -> Shot {
        Shot {
            instance: instance.to_owned(),
            file_name: name.to_owned(),
            taken: Stamp {
                year,
                month,
                day,
                hour: 12,
                minute: 0,
            },
            path: PathBuf::from(name),
        }
    }

    /// The reader's clock for the bucket tests: Wednesday 18 November 2026, whose week began on
    /// Sunday the 15th.
    fn now() -> Stamp {
        Stamp {
            year: 2026,
            month: 11,
            day: 18,
            hour: 18,
            minute: 0,
        }
    }

    /// The shell's page rect at a 1440 window: the surface's left edge to the sidebar's left
    /// edge, less the page's own 24-point inset on both sides.
    fn page_width_at(window_width: f32) -> f32 {
        window_width
            - crate::geometry::RAIL
            - crate::geometry::PANEL
            - 2.0 * crate::geometry::CONTENT_INSET
    }

    /// A scratch folder of real files, so a delete can be proved against the disk rather than
    /// against a counter.
    fn scratch_folder(name: &str) -> PathBuf {
        let folder = std::env::temp_dir().join(format!(
            "sift-screenshots-{}-{name}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&folder);
        fs::create_dir_all(&folder).expect("a scratch folder");
        folder
    }

    /// **Ordering.** The groups run newest first whatever order they arrive in, which is the
    /// source's comparison on each group's newest screenshot (index.vue:511-519).
    /// **A custom group shows its own screenshots, under its own name.** The store's assignments decide which; a
    /// shot that was never assigned belongs to no custom group and must not appear under one.
    /// **Escape closes the viewer, and the arrows step through it.** Three key behaviours live in `viewer` and none had
    /// a test; keys cannot be driven through X here, so a synthesised event is the only verifier there is.
    /// **Escape blurs the search field.** Not a dialog -- the field keeps its text and gives up focus, which is what the
    /// reference's `onKeydown` does -- and the function reports the change so the caller knows a frame is owed.
    #[test]
    fn escape_blurs_the_search_field_without_clearing_it() {
        let ctx = egui::Context::default();
        crate::fonts::install(&ctx);
        let theme = crate::theme::dark();
        let _ = &theme;
        let mut state = State::default();
        state.search = "sunset".to_owned();
        state.search_focused = true;
        let raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1440.0, 900.0))),
            events: vec![egui::Event::Key {
                key: egui::Key::Escape,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        };
        let mut changed = false;
        ctx.run_ui(raw, |ui| {
            let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(260.0, 36.0));
            changed = search_field(ui, rect, &mut state);
        });
        assert!(!state.search_focused, "Escape must give up focus");
        assert_eq!(state.search, "sunset", "and must not clear what was typed");
        assert!(changed, "the caller is told a frame is owed");
    }
    #[test]
    fn the_viewer_closes_on_escape_and_steps_with_the_arrows() {
        let ctx = egui::Context::default();
        crate::fonts::install(&ctx);
        let theme = crate::theme::dark();
        let page = Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1027.0, 828.0));
        let shots = vec![
            Shot {
                instance: "Parity".to_owned(),
                file_name: "a.png".to_owned(),
                taken: Stamp { year: 2026, month: 10, day: 20, hour: 12, minute: 0 },
                path: std::path::PathBuf::from("/tmp/a.png"),
            },
            Shot {
                instance: "Parity".to_owned(),
                file_name: "b.png".to_owned(),
                taken: Stamp { year: 2026, month: 10, day: 21, hour: 12, minute: 0 },
                path: std::path::PathBuf::from("/tmp/b.png"),
            },
        ];
        let key = |key: egui::Key| egui::RawInput {
            events: vec![egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        };

        // Escape closes.
        let mut state = State::default();
        state.viewer = Some(0);
        let mut images = crate::images::Images::default();
        ctx.run_ui(key(egui::Key::Escape), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                viewer(ui, page, &theme, &mut images, &mut state, &shots);
            });
        });
        assert_eq!(state.viewer, None, "Escape must close the viewer");

        // Right arrow steps forward, left steps back, and neither closes it.
        let mut state = State::default();
        state.viewer = Some(0);
        ctx.run_ui(key(egui::Key::ArrowRight), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                viewer(ui, page, &theme, &mut images, &mut state, &shots);
            });
        });
        assert_eq!(state.viewer, Some(1), "the right arrow steps forward");
    }

    #[test]
    fn a_custom_group_holds_only_its_assigned_shots() {
        let shot = |name: &str| Shot {
            instance: "Parity".to_owned(),
            file_name: name.to_owned(),
            taken: Stamp { year: 2026, month: 10, day: 20, hour: 12, minute: 0 },
            path: PathBuf::from(format!("/tmp/{name}")),
        };
        let shots = vec![shot("a.png"), shot("b.png")];
        let mut assignments = std::collections::BTreeMap::new();
        assignments.insert("a.png".to_owned(), "parity-group".to_owned());
        let now = Stamp { year: 2026, month: 10, day: 21, hour: 12, minute: 0 };
        let mode = GroupBy::Custom("parity-group".to_owned());
        let grouped = groups(&mode, &shots, now, Sort::Newest, &assignments);
        assert_eq!(grouped.len(), 1, "one heading for one group");
        assert_eq!(grouped[0].shots.len(), 1, "only the assigned shot");
        assert_eq!(grouped[0].shots[0].file_name, "a.png");
        // And the select offers it beside the three stored modes.
        let listed = GroupBy::with_groups(&[crate::groups::Group {
            id: "parity-group".to_owned(),
            name: "Parity group".to_owned(),
        }]);
        assert_eq!(listed.len(), 4);
        assert_eq!(listed[3], mode);
    }

    #[test]
    fn groups_are_ordered_by_month_with_the_newest_first() {
        let shots = [
            shot("a", "2026-05-02_10.00.00.png", 2026, 5, 2),
            shot("a", "2026-07-19_10.00.00.png", 2026, 7, 19),
            shot("a", "2026-12-24_10.00.00.png", 2026, 12, 24),
            shot("a", "2026-07-01_10.00.00.png", 2026, 7, 1),
        ];
        let out = groups(&GroupBy::Date, &shots, now(), Sort::Newest, &Default::default());
        let titles: Vec<&str> = out.iter().map(|group| group.title.as_str()).collect();
        assert_eq!(
            titles,
            ["December 2026", "July 2026", "May 2026"],
            "December, then July, then May: the input order is not the output's"
        );
        assert_eq!(out[1].shots.len(), 2, "July holds the two shots taken in July");
    }

    /// **Empty groups dropped.** A month with nothing in it is not a group, so a heading is
    /// never drawn over nothing.
    #[test]
    fn a_month_with_no_screenshots_is_not_a_group() {
        let shots = [
            shot("a", "2026-07-19_10.00.00.png", 2026, 7, 19),
            shot("a", "2026-05-02_10.00.00.png", 2026, 5, 2),
        ];
        let out = groups(&GroupBy::Date, &shots, now(), Sort::Newest, &Default::default());
        assert_eq!(out.len(), 2, "July and May, and no June or August between them");
        assert!(
            out.iter().all(|group| !group.shots.is_empty()),
            "every group the rule emits has at least one screenshot"
        );
        assert_eq!(groups(&GroupBy::Date, &[], now(), Sort::Newest, &Default::default()).len(), 0);
    }

    /// **A group with one item is still a group.** The source draws a heading and its count for
    /// a single shot exactly as it does for nine.
    #[test]
    fn a_group_holding_one_screenshot_is_still_a_group() {
        let shots = [
            shot("a", "2026-10-21_14.32.07.png", 2026, 10, 21),
            shot("a", "2026-09-30_16.58.09.png", 2026, 9, 30),
            shot("a", "2026-09-02_08.00.00.png", 2026, 9, 2),
        ];
        let out = groups(&GroupBy::Date, &shots, now(), Sort::Newest, &Default::default());
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].shots.len(), 1, "October's group holds its one shot");
        assert_eq!(out[1].shots.len(), 2);
        assert_eq!(out[0].title, "October 2026");
        assert_eq!(out[0].id, "date:month:2026-10");
    }

    /// **The four relative rungs and the month rung**, on a clock that makes each of them
    /// reachable: today, yesterday, earlier this week, earlier this month, and last month.
    #[test]
    fn the_date_buckets_read_the_way_the_source_buckets_them() {
        let shots = [
            shot("a", "today.png", 2026, 11, 18),
            shot("b", "yesterday.png", 2026, 11, 17),
            shot("c", "monday.png", 2026, 11, 16),
            shot("d", "earlier.png", 2026, 11, 10),
            shot("e", "older.png", 2026, 10, 30),
        ];
        let out = groups(&GroupBy::Date, &shots, now(), Sort::Newest, &Default::default());
        let titles: Vec<&str> = out.iter().map(|group| group.title.as_str()).collect();
        assert_eq!(
            titles,
            [TODAY, YESTERDAY, THIS_WEEK, THIS_MONTH, "October 2026"],
            "the source's five rungs, newest first"
        );
        assert_eq!(out[0].id, "date:today");
        assert_eq!(out[1].id, "date:yesterday");
        assert_eq!(
            out[2].id, "date:week:2026-11-15",
            "the week's id is its first day, as dayjs formats it"
        );
        assert_eq!(out[3].id, "date:month:2026-11");
        assert_eq!(out[4].id, "date:month:2026-10");
    }

    /// Two shots from the same day share one heading, and so do two from the same week.
    #[test]
    fn shots_from_one_day_and_one_week_share_a_group() {
        let shots = [
            shot("a", "morning.png", 2026, 11, 18),
            shot("a", "evening.png", 2026, 11, 18),
            shot("a", "monday.png", 2026, 11, 16),
            shot("a", "sunday.png", 2026, 11, 15),
        ];
        let out = groups(&GroupBy::Date, &shots, now(), Sort::Newest, &Default::default());
        let titles: Vec<(&str, usize)> = out
            .iter()
            .map(|group| (group.title.as_str(), group.shots.len()))
            .collect();
        assert_eq!(titles, [(TODAY, 2), (THIS_WEEK, 2)]);
    }

    /// The shots inside a group keep the order they arrived in, because the source groups an
    /// array it has already sorted (index.vue:413-418).
    #[test]
    fn a_groups_shots_keep_the_order_they_arrived_in() {
        let newest = shot("a", "2026-10-21_14.32.07.png", 2026, 10, 21);
        let oldest = shot("a", "2026-10-02_18.03.12.png", 2026, 10, 2);
        let out = groups(&GroupBy::Date, &[oldest.clone(), newest.clone()], now(), Sort::Newest, &Default::default());
        assert_eq!(
            out[0].shots,
            vec![oldest.clone(), newest.clone()],
            "input order, not re-sorted"
        );

        let by_name = in_sort_order(&[newest.clone(), oldest.clone()], Sort::Name);
        let out = groups(&GroupBy::Date, &by_name, now(), Sort::Newest, &Default::default());
        assert_eq!(
            out[0].shots,
            vec![oldest, newest],
            "a name sort reaches the group's contents"
        );
    }

    /// The source reverses the group order for Oldest and leaves it descending, even for a name
    /// sort, otherwise (index.vue:518).
    #[test]
    fn the_group_order_follows_the_sort() {
        let shots = [
            shot("a", "2026-05-02_10.00.00.png", 2026, 5, 2),
            shot("a", "2026-07-19_10.00.00.png", 2026, 7, 19),
        ];
        let months = |groups: Vec<Group>| -> Vec<String> { groups.iter().map(|g| g.id.clone()).collect() };
        assert_eq!(
            months(groups(&GroupBy::Date, &shots, now(), Sort::Newest, &Default::default())),
            ["date:month:2026-07", "date:month:2026-05"]
        );
        assert_eq!(
            months(groups(&GroupBy::Date, &shots, now(), Sort::Oldest, &Default::default())),
            ["date:month:2026-05", "date:month:2026-07"]
        );
        assert_eq!(
            months(groups(&GroupBy::Date, &shots, now(), Sort::Name, &Default::default())),
            ["date:month:2026-07", "date:month:2026-05"],
            "a name sort keeps the newest group first"
        );
    }

    /// The instance mode is one group per instance, ordered by name, and the no-grouping mode
    /// is one list with no heading.
    #[test]
    fn the_instance_and_no_grouping_modes_group_the_list_their_way() {
        let shots = [
            shot("Vanilla Plus", "b.png", 2026, 10, 21),
            shot("Fabulously Optimized", "a.png", 2026, 10, 22),
            shot("Vanilla Plus", "c.png", 2026, 10, 23),
        ];
        let out = groups(&GroupBy::Instance, &shots, now(), Sort::Newest, &Default::default());
        let titles: Vec<(&str, usize)> = out
            .iter()
            .map(|group| (group.title.as_str(), group.shots.len()))
            .collect();
        assert_eq!(
            titles,
            [("Fabulously Optimized", 1), ("Vanilla Plus", 2)],
            "the source orders instance groups by the instance's name"
        );
        assert_eq!(out[1].id, "instance:Vanilla Plus");

        let flat = groups(&GroupBy::None, &shots, now(), Sort::Newest, &Default::default());
        assert_eq!(flat.len(), 1);
        assert_eq!(flat[0].shots.len(), 3);
        assert!(!flat[0].header, "the no-grouping mode hides the header");
        assert_eq!(flat[0].title, ALL_SCREENSHOTS);
    }

    /// The three sorts, from the source's comparator (index.vue:413-418).
    #[test]
    fn the_three_sorts_order_the_month_s_screenshots() {
        let early = shot("a", "2026-10-02_18.03.12.png", 2026, 10, 2);
        let late = shot("a", "2026-10-21_14.32.07.png", 2026, 10, 21);
        assert_eq!(
            in_sort_order(&[early.clone(), late.clone()], Sort::Newest),
            vec![late.clone(), early.clone()]
        );
        assert_eq!(
            in_sort_order(&[late.clone(), early.clone()], Sort::Oldest),
            vec![early.clone(), late.clone()]
        );
        assert_eq!(
            in_sort_order(&[late.clone(), early.clone()], Sort::Name),
            vec![early, late]
        );
        assert_eq!(Sort::ALL.map(Sort::label), [SORT_NEWEST, SORT_OLDEST, SORT_NAME]);
    }

    /// The group-by select offers the three modes that need no stored groups, in the source's
    /// order with custom left out.
    #[test]
    fn the_group_select_offers_the_modes_this_build_can_group_by() {
        assert_eq!(
            // `ALL` is the three stored modes; `with_groups` appends the reader's own, which a browser's select lists
            // after them.
            GroupBy::ALL.map(|mode| mode.label().to_owned()).to_vec(),
            vec![
                GROUP_INSTANCE.to_owned(),
                GROUP_DATE.to_owned(),
                GROUP_NONE.to_owned()
            ]
        );
        assert_eq!(GroupBy::default(), GroupBy::Date);
    }

    /// The search matches a file name and an instance name, ignores case, and asks for nothing
    /// when it is empty or only spaces (index.vue:402-411, and State::query).
    #[test]
    fn the_search_matches_names_and_instances() {
        let shots = [
            shot("Fabulously Optimized", "2026-10-21_14.32.07.png", 2026, 10, 21),
            shot("Vanilla Plus", "2026-09-30_16.58.09.png", 2026, 9, 30),
        ];
        let all = filtered(&shots, None);
        assert_eq!(all.len(), 2, "an empty query is every screenshot");

        let by_instance = filtered(&shots, Some("vanilla"));
        assert_eq!(by_instance.len(), 1, "an instance name matches, case-insensitively");
        assert_eq!(by_instance[0].instance, "Vanilla Plus");

        let by_file = filtered(&shots, Some("2026-09-30"));
        assert_eq!(by_file.len(), 1);
        assert_eq!(by_file[0].instance, "Vanilla Plus");

        assert_eq!(filtered(&shots, Some("optifine")).len(), 0, "no match is no shot");

        let mut state = State::default();
        assert_eq!(state.query(), None, "a fresh page asks for nothing");
        state.search = "   ".to_owned();
        assert_eq!(state.query(), None, "spaces are not a query");
        state.search = "  vanity  ".to_owned();
        assert_eq!(state.query(), Some("vanity"), "the query is trimmed at its use");
        assert_eq!(filtered(&shots, state.query()).len(), 0);
    }

    /// The month heading and the card's date are the two Intl formats the source asks for:
    /// month long and year numeric, and dateStyle medium with timeStyle short.
    #[test]
    fn the_dates_read_the_way_intl_prints_them() {
        assert_eq!(month_title(2026, 10), "October 2026");
        assert_eq!(month_title(2026, 1), "January 2026");
        assert_eq!(
            stamp_label(Stamp {
                year: 2026,
                month: 10,
                day: 21,
                hour: 14,
                minute: 32,
            }),
            "Oct 21, 2026, 2:32 PM"
        );
        assert_eq!(
            stamp_label(Stamp {
                year: 2026,
                month: 9,
                day: 3,
                hour: 0,
                minute: 5,
            }),
            "Sep 3, 2026, 12:05 AM",
            "midnight is 12 AM, and the minutes keep their leading zero"
        );
        assert_eq!(
            stamp_label(Stamp {
                year: 2026,
                month: 12,
                day: 31,
                hour: 12,
                minute: 0,
            }),
            "Dec 31, 2026, 12:00 PM",
            "noon is 12 PM"
        );
    }

    /// The known timestamp: 1_700_000_000 is 2023-11-14 22:13:20 UTC. [Stamp] keeps whole
    /// minutes, which is all the card's Intl format prints (card.vue:46).
    #[test]
    fn a_known_timestamp_reads_as_its_utc_date() {
        assert_eq!(
            stamp_from_unix(1_700_000_000),
            Stamp {
                year: 2023,
                month: 11,
                day: 14,
                hour: 22,
                minute: 13,
            }
        );
    }

    /// The day numbers the week bucket divides by are the civil calendar's: 1970-01-01 is day
    /// zero and was a Thursday, and 2026-11-18 is a Wednesday.
    #[test]
    fn the_day_numbers_are_the_civil_calendars() {
        assert_eq!(day_number(1970, 1, 1), 0);
        assert_eq!(weekday(0), 4, "the epoch was a Thursday");
        assert_eq!(weekday(day_number(2026, 11, 18)), 3, "a Wednesday");
        assert_eq!(weekday(day_number(2026, 11, 15)), 0, "a Sunday");
    }

    /// A leap day exists in the civil calendar, and the month table must not skip it: the day
    /// 1_709_164_800 is 2024-02-29 00:00 UTC, and the next day is March 1.
    #[test]
    fn a_leap_day_is_still_a_day() {
        assert_eq!(
            stamp_from_unix(1_709_164_800),
            Stamp {
                year: 2024,
                month: 2,
                day: 29,
                hour: 0,
                minute: 0,
            }
        );
        assert_eq!(
            stamp_from_unix(1_709_164_800 + SECONDS_PER_DAY),
            Stamp {
                year: 2024,
                month: 3,
                day: 1,
                hour: 0,
                minute: 0,
            }
        );
    }

    /// The Unix epoch is zero seconds: 1970-01-01 00:00 UTC.
    #[test]
    fn zero_is_the_unix_epoch() {
        assert_eq!(
            stamp_from_unix(0),
            Stamp {
                year: 1970,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
            }
        );
    }

    /// A timestamp before the epoch has no date this page can print, so it clamps to the epoch
    /// instead of wrapping into 1969 or panicking.
    #[test]
    fn a_timestamp_before_the_epoch_clamps_to_it() {
        let epoch = stamp_from_unix(0);
        assert_eq!(stamp_from_unix(-1), epoch);
        assert_eq!(stamp_from_unix(i64::MIN), epoch);
    }

    /// The caption is app.screenshots.preview.instance-and-date, "{instance} · {date}"
    /// (locales/en-US/index.json:1574).
    #[test]
    fn the_caption_puts_the_instance_and_the_date_on_one_line() {
        let shot = shot("Fabulously Optimized", "2026-10-21_14.32.07.png", 2026, 10, 21);
        assert_eq!(
            instance_and_date(shot),
            "Fabulously Optimized · Oct 21, 2026, 12:00 PM"
        );
    }

    /// At 1440 the list is 980 points wide and the 1536 breakpoint has not been reached, so the
    /// source shows two columns of 484. The identity closes exactly: two cards and one gap are
    /// the list's width, and the height comes from the same width.
    #[test]
    fn the_grid_tiles_the_list_at_1440() {
        let page = page_width_at(crate::geometry::WINDOW.0);
        assert_eq!(page, 1028.0, "the shell's page rect at 1440");
        let list = page - 2.0 * PAGE_PADDING;

        let grid = grid_layout(list, crate::geometry::WINDOW.0);
        assert_eq!(grid.columns, 2, "1440 is below the 1536 four-column breakpoint");
        assert!((grid.card_width - 484.0).abs() < 0.001, "(980 - 12) / 2");
        assert!(
            (grid.card_height - 272.25).abs() < 0.001,
            "484 * 9 / 16"
        );

        let used = (grid.columns as f32) * grid.card_width
            + (grid.columns as f32 - 1.0) * GRID_GAP;
        assert!(
            (used - list).abs() < 0.001,
            "columns * card + gaps must be the list width: {used} vs {list}"
        );
    }

    /// At the 1536 breakpoint the ternary takes the other branch: four 236-point cards and
    /// three gaps fill the same 980 points.
    #[test]
    fn the_grid_tiles_the_list_at_1536() {
        let page = page_width_at(crate::geometry::WINDOW.0);
        let list = page - 2.0 * PAGE_PADDING;
        let grid = grid_layout(list, WIDE_BREAKPOINT);
        assert_eq!(grid.columns, WIDE_COLUMNS);
        assert!((grid.card_width - 236.0).abs() < 0.001, "(980 - 36) / 4");
        let used = (grid.columns as f32) * grid.card_width
            + (grid.columns as f32 - 1.0) * GRID_GAP;
        assert!((used - list).abs() < 0.001, "{used} vs {list}");

        assert_eq!(column_count(640.0), NARROW_COLUMNS, "the sm edge is inclusive");
        assert_eq!(column_count(639.0), SLIM_COLUMNS);
    }

    /// A list nobody has measured takes the source's 320-point fallback rather than dividing by
    /// a zero column (index.vue:529).
    #[test]
    fn an_unmeasured_list_falls_back_to_the_source_s_card_width() {
        let grid = grid_layout(0.0, 1440.0);
        assert!((grid.card_width - FALLBACK_CARD_WIDTH).abs() < 0.001);
        assert!((grid.card_height - FALLBACK_CARD_WIDTH * CARD_ASPECT).abs() < 0.001);
    }

    /// The block height is the source's sum (index.vue:542-566): header, 10, grid, 12, and the
    /// grid is rows * card + (rows - 1) * gap. Five shots over two columns is three rows.
    #[test]
    fn a_group_is_as_tall_as_its_rows_plus_its_furniture() {
        let page = page_width_at(crate::geometry::WINDOW.0);
        let grid = grid_layout(page - 2.0 * PAGE_PADDING, crate::geometry::WINDOW.0);
        let height = block_height(grid.columns, grid.card_height, 5, false, true);
        assert!((height - 902.75).abs() < 0.001, "40 + 10 + 840.75 + 12");

        let one_row = block_height(grid.columns, grid.card_height, 2, false, true);
        assert!((one_row - 334.25).abs() < 0.001, "40 + 10 + 272.25 + 12");

        // An empty group is the grid's 45-point minimum, not a zero-height grid.
        let empty = block_height(grid.columns, grid.card_height, 0, false, true);
        assert!((empty - (GROUP_HEADER_HEIGHT + GROUP_CONTENT_SPACING + GRID_MIN_HEIGHT + GROUP_SPACING)).abs() < 0.001);
    }

    /// A collapsed group is its header and its tail, with no grid and no gap under the header;
    /// a group whose header is hidden contributes no header height at all.
    #[test]
    fn a_collapsed_group_is_only_its_header() {
        let page = page_width_at(crate::geometry::WINDOW.0);
        let grid = grid_layout(page - 2.0 * PAGE_PADDING, crate::geometry::WINDOW.0);
        let collapsed = block_height(grid.columns, grid.card_height, 5, true, true);
        assert!((collapsed - (GROUP_HEADER_HEIGHT + GROUP_SPACING)).abs() < 0.001);
        let open = block_height(grid.columns, grid.card_height, 5, false, true);
        assert!(open > collapsed, "folding a group can only make it shorter");

        let headerless = block_height(grid.columns, grid.card_height, 5, false, false);
        let headed = block_height(grid.columns, grid.card_height, 5, false, true);
        assert!((headed - headerless - GROUP_HEADER_HEIGHT).abs() < 0.001);
    }

    /// A selection toggles: once in, once out, and the two presses leave the page as it was.
    #[test]
    fn a_selection_toggles_on_and_off() {
        let mut state = State::default();
        let path = Path::new("/shots/a.png");
        assert!(!state.is_selected(path));
        state.toggle(path);
        assert!(state.is_selected(path), "the first press selects");
        state.toggle(path);
        assert!(!state.is_selected(path), "the second press deselects");
        assert!(state.selected.is_empty());
    }

    /// A selection never outlives the file it names: a shot that has left the disk leaves the
    /// selection, and an armed delete for it is put away.
    #[test]
    fn a_card_that_leaves_the_disk_leaves_the_selection() {
        let mut state = State::default();
        state.shots = vec![shot("a", "kept.png", 2026, 11, 18)];
        let gone = PathBuf::from("gone.png");
        state.selected.insert(gone.clone());
        state.selected.insert(PathBuf::from("kept.png"));
        state.armed = Some(Arm::Card(gone.clone()));

        state.prune();

        assert_eq!(
            state.selected.iter().collect::<Vec<&PathBuf>>(),
            vec![&PathBuf::from("kept.png")]
        );
        assert_eq!(state.armed, None, "an armed delete for a gone file is stale");
    }

    /// Deleting removes every named file from the disk, and a path that is not there is
    /// reported rather than passed over.
    #[test]
    fn deleting_a_selection_removes_each_file() {
        let folder = scratch_folder("delete");
        let first = folder.join("one.png");
        let second = folder.join("two.png");
        fs::write(&first, b"not really a png").expect("a file");
        fs::write(&second, b"not really a png").expect("a file");

        let removed = remove_files(&[first.clone(), second.clone()]).expect("both files go");
        assert_eq!(removed, 2);
        assert!(!first.exists(), "the first file is gone");
        assert!(!second.exists(), "the second file is gone");

        let missing = folder.join("never.png");
        let outcome = remove_files(&[missing.clone()]);
        assert!(outcome.is_err(), "a file that was not there is an error, not a zero");
        assert!(
            outcome.unwrap_err().contains("never.png"),
            "the sentence names the file that could not be deleted"
        );
        let _ = fs::remove_dir_all(&folder);
    }

    /// The sentence a delete owes: one is the source's own string, many are counted
    /// (index.vue:1211-1220).
    #[test]
    fn the_delete_sentence_counts_screenshots() {
        assert_eq!(deleted_sentence(1), DELETE_SUCCESS);
        assert_eq!(deleted_sentence(3), "3 screenshots deleted");
    }

    /// The card's menu carries every row this page can carry out, and nothing else: the four
    /// rows the source has that need an editor, an image clipboard or a route are recorded in
    /// the module note instead of being drawn dead.
    #[test]
    fn the_menu_carries_every_row_this_page_can_carry_out() {
        // The group row sits second, where the reference puts it: Open, group, reveal, then the destructive one
        // last.
        assert_eq!(
            MENU_ITEMS,
            [
                Item::Open,
                Item::AddToGroup,
                Item::ShowInFolder,
                Item::Delete
            ]
        );
        assert!(Item::Delete.danger(), "Delete is the row drawn in red");
        assert!(!Item::Open.danger());
        assert_eq!(Item::ShowInFolder.label(), SHOW_IN_FOLDER);
    }

    /// object-cover crops the longer side: a wide picture in a square box loses its edges, a
    /// tall one loses its top and bottom, and a matching aspect is the whole picture.
    #[test]
    fn cover_crops_without_distorting() {
        // A square picture offered a 2:1 box is scaled until its width fills the box, so the
        // whole width is shown and the top and bottom are cropped away.
        let square = cover_uv(Vec2::new(100.0, 100.0), Vec2::new(200.0, 100.0));
        assert!((square.width() - 1.0).abs() < 0.001, "the full width is shown");
        assert!((square.height() - 0.5).abs() < 0.001, "half the height is cropped");

        let same = cover_uv(Vec2::new(160.0, 90.0), Vec2::new(320.0, 180.0));
        assert_eq!(
            same,
            Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            "a matching aspect is the whole picture"
        );

        // A portrait picture in a landscape box is the same case one step further: the width
        // still fills and more of the height is lost.
        let tall = cover_uv(Vec2::new(90.0, 160.0), Vec2::new(320.0, 180.0));
        assert!((tall.width() - 1.0).abs() < 0.001, "a portrait picture fills the width");
        assert!(tall.height() < 1.0, "and gives up its top and bottom");
        assert!(tall.height() < square.height(), "a taller picture loses more of itself");
    }

    /// The selection bar's count line is the source's own string.
    #[test]
    fn the_selection_bar_says_how_many_are_selected() {
        assert_eq!(selected_count(1), "1 selected");
        assert_eq!(selected_count(12), "12 selected");
    }
}
