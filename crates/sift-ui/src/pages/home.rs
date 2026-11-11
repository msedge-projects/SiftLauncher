//! The welcome screen.
//!
//! Measured from the reference's own Home at 1440x900 (`out/home-idle-modrinth.png`, cropped
//! at 2x): the content column carries a centred hero -- a 64-point mark plate, the heading
//! at 24, a subtitle at 14, a green `+ Create an instance` button and a keyboard hint -- and,
//! pinned near the bottom, the `Escaping another launcher?` prompt with its own button.
//!
//! **Two states, one page.** With instances installed the reference replaces the hero with
//! the instance list; the hero is what a fresh install shows. This draws the hero, which is
//! the state a new player sees and the state the parity capture compares, and the list comes
//! in the slice that ports instance cards.
//!
//! **The row menu.** Every row carries the reference's kebab (`InstanceItem.vue:308-314`, a
//! quiet overflow button around `MoreVerticalIcon`) and a press opens the popover
//! [`menu_popover`] draws. Its rows are the library's instance actions
//! (`use-library.ts:1322-1377`), labelled with the reference's own strings -- see
//! [`MenuItem::label`] for the key behind each one. The menu's `Play` is the same
//! [`Action::Play`] the row's pill sends, so it launches for real -- what the reference's own
//! context menu does (`InstanceItem.vue:203-217`) -- and the shell answers a second press
//! while a launch is out with a toast. `Edit`, `Duplicate`, `Open folder` and `Delete` report
//! their own verbs to the shell. **`Delete` asks first**: the press opens the reference's
//! confirmation (`ConfirmDeleteInstanceModal.vue`), and `Action::Delete` -- the verb that
//! removes a folder -- leaves this page only when that dialog's own button answers.
//!
//! **The worlds under the list.** The reference's `Jump in` section
//! (`app.home.jump-back-in.title`, `RecentWorldsList.vue:47`) lists recently played worlds
//! with the instance each belongs to. This page draws them from each instance's `saves/`
//! folder names only -- see [`read_worlds`] for what that can and cannot say, and why a world
//! row's press is a toast rather than a launch.

use egui::{Align2, Rect, Sense, Stroke, Ui, Vec2, Color32};

use crate::data::Entry;
use crate::fonts;
use crate::icons::{self, Glyph};
use crate::images::Images;
use crate::theme::Theme;

// ---------------------------------------------------------------------------
// The row menu: geometry and type
//
// Every number names the rule it came from. The panel is the reference's menu panel --
// `rounded-[14px] bg-surface-3 ring-1 ring-surface-5` (`button-menu.ts:21`) around a
// `flex flex-col p-2` column (`ButtonMenuPanel.vue:70`) -- and a row is `min-h-10 px-3
// py-2 gap-2 rounded-[10px] text-base font-semibold text-contrast` (`button-menu.ts:15`).
// ---------------------------------------------------------------------------

/// The kebab's hit target: the reference's quiet overflow button is a `size-9` icon button
/// (`InstanceItem.vue:308-314`); 28 points is the 16-point glyph with a pointer's worth of
/// plate around it, not a measurement -- re-measure against the capture.
// guess: re-measure
const KEBAB_SIZE: f32 = 28.0;

/// The glyph inside the kebab (`MoreVerticalIcon`, `InstanceItem.vue:313`).
// guess: re-measure
const KEBAB_GLYPH: f32 = 16.0;

/// The kebab's hover and open plate: `--radius-sm`, 8 points (`variables.scss:202`).
const KEBAB_RADIUS: u8 = 8;

/// The gap between the play pill and the kebab: `gap-1` on the row's action group
/// (`InstanceItem.vue:283`), 0.25rem = 4 points at the reference's 16px root
/// (`variables.scss:195`).
const ROW_ACTION_GAP: f32 = 4.0;

/// A menu row's height: `min-h-10` (`button-menu.ts:15`), 2.5rem = 40 points.
const MENU_ITEM_HEIGHT: f32 = 40.0;

/// A menu row's horizontal padding: `px-3` (`button-menu.ts:15`), 0.75rem = 12 points.
const MENU_ITEM_PADDING: f32 = 12.0;

/// The gap between a menu row's glyph and its label: `gap-2` (`button-menu.ts:15`), 8 points.
const MENU_ITEM_GAP: f32 = 8.0;

/// A menu row's glyph. The reference sizes a row's icon at `size-5` for trailing actions
/// (`ButtonMenuItem.vue:20`); 16 keeps the label's leading edge where the smaller glyphs of
/// this shell already sit -- re-measure before trusting the two points.
// guess: re-measure
const MENU_ITEM_GLYPH: f32 = 20.0;

/// The panel's inset around its rows: `p-2` (`ButtonMenuPanel.vue:70`), 8 points.
const MENU_PANEL_PADDING: f32 = 8.0;

/// The panel's corner radius: `rounded-[14px]` (`button-menu.ts:21`).
const MENU_RADIUS: u8 = 14;

/// A menu row's corner radius: `rounded-[10px]` (`button-menu.ts:15`).
const MENU_ROW_RADIUS: u8 = 10;

/// How far below the kebab the panel opens: `distance: 8` on the reference's floating
/// panels (`FloatingPanel.vue:22`).
const MENU_DISTANCE: f32 = 8.0;

// ---------------------------------------------------------------------------
// The list: one row's targets
// ---------------------------------------------------------------------------

/// Where the first row's card starts, down from the page's top edge: the heading line is 24
/// at 24 and the row starts below it -- the number the shell's Home capture was aimed with.
/// Where the first card's row starts inside the page: the reference's own capture puts the first card's top at
/// **y 335** in the window (page top 48), so 287. Driving its library (`docs/captures/ref-lib-real.png`) showed this
/// build drawing the heading, the toolbar and the cards about 28 points higher than the reference does.
// **Six points up.** The reference's first card starts at y 329 and this build's at 335; the icon below it was 13 points
// high for a second reason (its own offset), and the two together made the whole card row look vertically shifted.
const ROW_TOP: f32 = 281.0;

/// The step between two rows: a 96-point card and a 12-point gap.
const ROW_HEIGHT: f32 = 96.0;
const ROW_STEP: f32 = 108.0;

/// How far the play pill's centre sits in from the card's right edge, and how big it is.
/// The reference's card shows the pill in a `size-12` slot 6 points off the tile's corner
/// (`instance-card-view.vue:66-71`); on this row it is the same 40-point circle.
const PLAY_INSET: f32 = 60.0;
const PLAY_SIZE: f32 = 40.0;

/// A dashed rounded rectangle: the reference's "Jump in" strip is drawn with a dashed border, and epaint has no
/// dash pattern of its own, so this walks the four edges in 6-on/4-off steps.
pub fn dashed_rect(painter: &egui::Painter, rect: Rect, radius: f32, ink: egui::Color32) {
    let stroke = egui::Stroke::new(1.0, ink);
    let step = 6.0;
    let gap = 4.0;
    let mut x = rect.left() + radius;
    while x < rect.right() - radius {
        let end = (x + step).min(rect.right() - radius);
        painter.line_segment([egui::pos2(x, rect.top()), egui::pos2(end, rect.top())], stroke);
        painter.line_segment(
            [egui::pos2(x, rect.bottom()), egui::pos2(end, rect.bottom())],
            stroke,
        );
        x = end + gap;
    }
    let mut y = rect.top() + radius;
    while y < rect.bottom() - radius {
        let end = (y + step).min(rect.bottom() - radius);
        painter.line_segment([egui::pos2(rect.left(), y), egui::pos2(rect.left(), end)], stroke);
        painter.line_segment(
            [egui::pos2(rect.right(), y), egui::pos2(rect.right(), end)],
            stroke,
        );
        y = end + gap;
    }
}

/// An instance's folder on disk, from the entry's own id.
fn recent_path(entry: &Entry) -> std::path::PathBuf {
    crate::data::instances_dir().join(&entry.id)
}

/// The three targets on one row: the card, its play pill and its kebab.
///
/// A function rather than the same three expressions spelled twice: the probe below and the
/// tests aim at exactly the rects the drawing registers, which is the only way a printed
/// number and a press can be the same number.
fn row_targets(rect: Rect, index: usize) -> (Rect, Rect, Rect) {
    row_targets_at(rect, index, 0.0)
}

/// The same grid, pushed down by a group header. The default is `0.0`, which is the ungrouped page
/// and every measurement already recorded; a grouped page passes the header's height, and both the
/// drawing and the press handling read it from here so a click cannot land on the card above.
fn row_targets_at(rect: Rect, index: usize, shift: f32) -> (Rect, Rect, Rect) {
    // **A grid, as the reference draws it.** Driving its Home page (display :107) showed a wall of cards: the
    // first plate starts at x 88 and is 168 wide with the pack's icon filling its top and the name and
    // loader+version under it. This build drew one full-width row per instance with the Play pill and the kebab
    // inside it, which is a different object on a different page.
    const CARD_W: f32 = 168.0;
    // **215, not 208.** Measured on the reference's own card at x 150: 354..568. At 208 this build's grey loader line ran into the
    // plate's bottom edge, which is what the 2x crop shows; the extra 7 is the padding under it.
    // **239, so the plate foot lands on the reference's 568.** The card starts at 329 here and 354 there; 329 + 239 = 568 gives the
    // same bottom edge, and 20 points of padding under the loader against the reference's 21.
    const CARD_H: f32 = 239.0;
    const GAP: f32 = 12.0;
    let columns = ((rect.width() + GAP) / (CARD_W + GAP)).floor().max(1.0) as usize;
    let column = index % columns;
    let row = index / columns;
    let card = Rect::from_min_size(
        egui::pos2(
            rect.left() + column as f32 * (CARD_W + GAP),
            rect.top() + ROW_TOP + shift + row as f32 * (CARD_H + GAP),
        ),
        egui::vec2(CARD_W, CARD_H),
    );
    // The old row's constants are still what `ROWS_SHOWN` and the tests read; the grid card is the shape the
    // reference draws, and the plate below uses this rect.
    // The two actions sit over the card's foot, where a card of this size can carry them.
    // **A circle over the art's bottom-right, not a pill at the card's foot.** The reference's hovered card shows a
    // 30-point green disc on the picture itself; this drew a rounded pill at the card's foot, where it sat over the name.
    let play = Rect::from_center_size(
        egui::pos2(card.right() - 50.0, card.top() + 142.0),
        Vec2::splat(42.0),
    );
    let kebab = Rect::from_center_size(
        egui::pos2(card.right() - 20.0, card.bottom() - 24.0),
        Vec2::splat(KEBAB_SIZE),
    );
    (card, play, kebab)
}

/// The narrowest the panel gets before its widest label widens it. The reference's panel is
/// `w-fit` (`ButtonMenuPanel.vue`'s class list), so this floor exists only so a short menu
/// does not look like a tooltip -- re-measure against the capture.
// guess: re-measure
const MENU_MIN_WIDTH: f32 = 168.0;

// ---------------------------------------------------------------------------
// The delete confirmation: geometry, type and copy
//
// The reference never deletes on the menu's press. `use-library.ts:1373-1376` stores the id
// and calls `confirmDeleteModal.value?.show()`, so the destructive half of the action is a
// second, deliberate press. The dialog is `ConfirmDeleteInstanceModal`
// (`apps/app-frontend/src/components/ui/modal/ConfirmDeleteInstanceModal.vue`): a
// `max-width: 500px` `NewModal` with `fade="danger"`, a critical `Admonition`, the instance
// it names, and `Cancel` beside a red `Delete instance`.
//
// Its anatomy is `NewModal`'s (`packages/ui/src/components/modal/NewModal.vue`): a
// `bg-bg-raised rounded-2xl border border-solid border-surface-5` body (:38), a `p-6` header
// with a `text-2xl font-semibold text-contrast` title (:44-46) over a 1 px `surface-5` rule,
// a `p-6` content column (:118) and a `p-4` action row (:130). The two buttons are the design
// system's `md` (`ButtonFrame.vue:31`).
// ---------------------------------------------------------------------------

/// The plate's width: `max-width="500px"` on the reference's dialog
/// (`ConfirmDeleteInstanceModal.vue:2`).
const CONFIRM_WIDTH: f32 = 500.0;

/// The plate's corner radius: `rounded-2xl`, 1rem (`NewModal.vue:38`).
const CONFIRM_RADIUS: u8 = 16;

/// The header's, the content column's and the action row's padding: `p-6` twice and `p-4`
/// (`NewModal.vue:45, 118, 130`).
const CONFIRM_PAD: f32 = 24.0;
const CONFIRM_ACTIONS_PAD: f32 = 16.0;

/// The header's title: `text-2xl font-semibold` -- 24 on a 32-point line (`NewModal.vue:44`).
const CONFIRM_TITLE_SIZE: f32 = 24.0;
const CONFIRM_TITLE_LINE: f32 = 32.0;

/// The close button: `IconButton` at `md`, a 36-point square with a 20-point glyph
/// (`IconButton.vue`, `ButtonFrame.vue:31`).
const CONFIRM_CLOSE: f32 = 36.0;

/// The admonition: `grid grid-cols-[1.5rem_minmax(0,1fr)_auto] rounded-2xl border p-4` with a
/// 24-point severity glyph and `gap-x-2`, its header `text-lg font-semibold leading-6` and
/// its body `leading-tight` (`Admonition.vue:2-47`).
const ADMONITION_PAD: f32 = 16.0;
const ADMONITION_ICON: f32 = 24.0;
const ADMONITION_GAP: f32 = 8.0;
const ADMONITION_HEADER_SIZE: f32 = 18.0;
const ADMONITION_HEADER_LINE: f32 = 24.0;
const ADMONITION_BODY: f32 = 16.0;
const ADMONITION_BODY_LINE: f32 = 20.0;

/// The instance card under the admonition: `flex items-center gap-2.5 rounded-[20px] p-3` with
/// a `40px` avatar, the name `font-semibold`, the loader `text-sm font-medium text-secondary`
/// (`ConfirmDeleteInstanceModal.vue:22-40`).
const CARD_PAD: f32 = 12.0;

/// The height a group header takes above the grid: the reference's row is a 24-point chevron and label
/// over a rule, measured at 36 from the header's top to the first card.
const GROUP_HEADER_H: f32 = 36.0;
const CARD_RADIUS: u8 = 20;
const CARD_AVATAR: f32 = 40.0;
const CARD_GAP: f32 = 10.0;
const LABEL_SIZE: f32 = 16.0;
const LABEL_LINE: f32 = 24.0;
const LOADER_SIZE: f32 = 14.0;
const LOADER_LINE: f32 = 20.0;

/// A button at the design system's `md`: `h-9 rounded-xl px-2.5 gap-1.5 text-base font-semibold
/// [&>svg]:size-5` (`ButtonFrame.vue:31`).
const ACTION_HEIGHT: f32 = 36.0;
const ACTION_RADIUS: u8 = 12;
const ACTION_PAD: f32 = 10.0;
const ACTION_GAP: f32 = 6.0;
const ACTION_ICON: f32 = 20.0;
const ACTION_TEXT: f32 = 16.0;
const ACTION_GAP_BETWEEN: f32 = 8.0;

/// The dialog's copy, verbatim from `ConfirmDeleteInstanceModal.vue`'s `defineMessages`
/// (:138-175); the key is beside each string so a drifting word can be traced to the reference.
/// The one-instance branch of each plural is the one this page can reach.
const CONFIRM_TITLE: &str = "Delete instance"; // app.instance.confirm-delete.header (:140-143)
const CONFIRM_ADMONITION_HEADER: &str = "This action cannot be undone"; // :146-152
const CONFIRM_ADMONITION_BODY: &str =
    "All data for your instance will be permanently deleted, including your worlds, configs, and all installed content."; // :153-157
const CONFIRM_INSTANCES_LABEL: &str = "Instance"; // app.instance.confirm-delete.instances-label, one (:164-168)
const CONFIRM_CANCEL: &str = "Cancel"; // commonMessages.cancelButton (`common-messages.ts:51-54`)
const CONFIRM_DELETE: &str = "Delete instance"; // app.instance.confirm-delete.delete-button (:169-172)

// ---------------------------------------------------------------------------
// The worlds section: geometry and type
// ---------------------------------------------------------------------------

/// Where an instance's worlds are, relative to its root: `worlds.ts:222` joins
/// `<gameRoot>/saves/<world>`, and this layout's game root is the instance's `minecraft/`
/// folder (`sift_core::instance::Instance::game_root`). The `.minecraft/` spelling is what an
/// instance imported from another launcher can use, so both are tried, in that order.
const SAVES_DIRS: [&str; 2] = ["minecraft/saves", "saves"];

/// How many instance rows are drawn. The reference caps its `Jump in` section at five items
/// (`MAX_JUMP_BACK_IN = 5`, `RecentWorldsList.vue:67`) and shows the rest in the library; six
/// rows is this page's own list height, not that cap -- re-measure against the window.
// guess: re-measure
const ROWS_SHOWN: usize = 6;

/// How many worlds one instance contributes. Three per instance keeps a five-instance page
/// near the reference's five-item section without hiding the other instances' worlds --
/// re-measure once the section is compared frame to frame.
// guess: re-measure
const WORLDS_PER_INSTANCE: usize = 3;

/// The gap between the last instance row and the worlds section.
// guess: re-measure
const WORLD_SECTION_GAP: f32 = 24.0;

/// The worlds heading's size: one step above the row text, the same step the reference uses
/// for a section header inside a card.
// guess: re-measure
const WORLD_HEADING_SIZE: f32 = 24.0;

/// The worlds heading's line box, which is what the rows start below.
// guess: re-measure
const WORLD_HEADING_LINE: f32 = 32.0;

/// A world row's height.
// guess: re-measure
const WORLD_ROW_HEIGHT: f32 = 80.0;

/// The gap between two world rows.
// guess: re-measure
const WORLD_ROW_GAP: f32 = 4.0;

/// A world row's corner radius: `--radius-sm`, 8 points (`variables.scss:202`).
const WORLD_ROW_RADIUS: u8 = 8;

/// The world row's glyph centre, in from the row's left edge.
// guess: re-measure
const WORLD_GLYPH_INSET: f32 = 20.0;

/// The glyph in a world row. **Not `Glyph::Map`**: this icon table has no `map.svg` -- it has
/// `map-pin.svg` ([`Glyph::MapPin`], a location pin) and `world.svg` ([`Glyph::World`], the
/// globe), and the globe is the set's own mark for a world, so that is the one drawn.
const WORLD_GLYPH: f32 = 16.0;

/// Where a world row's label starts.
// guess: re-measure
const WORLD_TEXT_INSET: f32 = 40.0;

/// A world row's label size.
// guess: re-measure
const WORLD_TEXT_SIZE: f32 = 13.0;

/// What the reader asked for by pressing something on this page.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Action {
    /// Nothing was pressed.
    #[default]
    None,
    /// `+ Create an instance`, or the `N` key: make one.
    Create,
    /// `Import from launcher`: find another launcher's instances.
    Import,
    /// Play this instance, by its id: the row's pill, or the menu's `Play`.
    Play(String),
    /// The row's own press: open the instance's page (`InstanceItem.vue`). A different verb
    /// from the menu's `Edit` on purpose -- a card press that re-opened the rename dialog was
    /// how the dialog used to be thrown away mid-typing.
    Open(String),
    /// The menu's `Edit`: the instance's own name, in the shell's rename dialog.
    Edit(String),
    /// The menu's `Duplicate instance`: copy the instance's folder.
    Duplicate(String),
    /// The menu's `Open folder`: show the instance's folder in the file manager.
    OpenFolder(String),
    /// The menu's `Copy path`: put the instance's folder on the clipboard.
    CopyPath(String),
    /// The menu's `Add to favorites` / `Remove from favorites`: persist the instance's flag.
    SetFavorite(String, bool),
    /// The menu's `Delete`: remove the instance's folder from disk.
    Delete(String),
    /// A world row: launch `world` inside `instance`. Quick-play is not wired, so the shell
    /// answers with a toast naming what is missing rather than launching.
    OpenWorld {
        /// The instance the world belongs to, by id.
        instance: String,
        /// The world's folder name inside `saves/`, already cleaned.
        world: String,
    },
}

/// One row of the kebab's popover.
///
/// The order and the labels are the library's instance actions
/// (`app-frontend/src/components/ui/library/use-library.ts:1322-1377`), which is where the
/// reference keeps the per-instance menu a card's kebab opens. Each label is the reference's
/// own string; [`MenuItem::key`] is the locale key it comes from
/// (`app-frontend/src/locales/en-US/index.json`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    /// Launch the instance.
    Play,
    /// Toggle the instance's favourite flag. The label depends on the instance's state, which is why
    /// it is the one row drawn through `label_with` rather than `label`.
    Favorite,
    /// Open the instance's page -- what a press on the card itself does (`InstanceItem.vue`).
    /// The reference lists it fourth, after `Add content`; those two are not built here.
    ViewInstance,
    /// Open the instance's settings page.
    Edit,
    /// Copy the instance.
    Duplicate,
    /// Show the instance's folder.
    OpenFolder,
    /// Put the instance's folder path on the clipboard (`app.instance.action.copy-path`).
    CopyPath,
    /// Delete the instance.
    Delete,
}

impl MenuItem {
    /// Every row, in the order the popover draws them: play first, then the instance's own
    /// actions, the folder, and delete last.
    pub const ALL: [MenuItem; 8] = [
        MenuItem::Play,
        MenuItem::Favorite,
        MenuItem::ViewInstance,
        MenuItem::Duplicate,
        MenuItem::OpenFolder,
        MenuItem::CopyPath,
        MenuItem::Edit,
        MenuItem::Delete,
    ];

    /// **The label for an instance whose favourite state is known.** Only one row changes its text, and the
    /// reference flips exactly that one (`use-library.ts:1298-1308`).
    pub fn label_with(self, favorite: bool) -> &'static str {
        match self {
            MenuItem::Favorite if favorite => "Remove from favorites",
            other => other.label(),
        }
    }

    /// The label, verbatim. The key it is read from is [`MenuItem::key`], and each comment
    /// below carries the line of `app-frontend/src/locales/en-US/index.json` that states it.
    pub fn label(self) -> &'static str {
        match self {
            // `app.instance.action.play` -- "Play" (:566-568)
            MenuItem::Play => "Play",
            // `app.instance.action.edit` -- "Edit" (:560-562)
            // `app.instance.action.view-instance`
            MenuItem::Favorite => "Add to favorites",
            MenuItem::ViewInstance => "View instance",
            // `app.instance.action.copy-path`
            MenuItem::CopyPath => "Copy path",
            MenuItem::Edit => "Edit",
            // `app.library.instance.action.duplicate` -- "Duplicate instance" (:1133-1135)
            MenuItem::Duplicate => "Duplicate instance",
            // `app.instance.action.open-folder` -- "Open folder" (:563-565)
            MenuItem::OpenFolder => "Open folder",
            // `app.library.instance.action.delete` -- "Delete" (:1130-1132)
            MenuItem::Delete => "Delete",
        }
    }

    /// The locale key [`MenuItem::label`] is read from, so a string that drifts can be
    /// traced back to the reference's own copy of it. Read by the tests, which is where a
    /// label and its key are compared: the popover draws [`MenuItem::label`] alone.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn key(self) -> &'static str {
        match self {
            MenuItem::Play => "app.instance.action.play",
            MenuItem::Favorite => "app.instance.action.add-to-favorites",
            MenuItem::ViewInstance => "app.instance.action.view-instance",
            MenuItem::CopyPath => "app.instance.action.copy-path",
            MenuItem::Edit => "app.instance.action.edit",
            MenuItem::Duplicate => "app.library.instance.action.duplicate",
            MenuItem::OpenFolder => "app.instance.action.open-folder",
            MenuItem::Delete => "app.library.instance.action.delete",
        }
    }

    /// The glyph the reference puts beside the label: `PlayIcon`, `EditIcon`,
    /// `ClipboardCopyIcon`, `FolderOpenIcon` and `TrashIcon` (`use-library.ts:1288-1370`).
    pub fn glyph(self) -> Glyph {
        match self {
            MenuItem::Play => Glyph::Play,
            MenuItem::Favorite => Glyph::Star,
            MenuItem::ViewInstance => Glyph::Eye,
            MenuItem::CopyPath => Glyph::Link,
            MenuItem::Edit => Glyph::Edit,
            MenuItem::Duplicate => Glyph::ClipboardCopy,
            MenuItem::OpenFolder => Glyph::FolderOpen,
            MenuItem::Delete => Glyph::Trash,
        }
    }

    /// What a press on this row asks the shell to do, for the instance `id`.
    ///
    /// Every item names a verb; there is no row that reports "nothing", which is the test
    /// below. Which of those verbs the shell can carry out is the shell's business --
    /// [`Action::Edit`] and [`Action::Duplicate`] exist so the press can be answered out
    /// loud instead of being dropped on the floor.
    pub fn action(self, id: &str) -> Action {
        let id = id.to_owned();
        match self {
            MenuItem::Play => Action::Play(id),
            MenuItem::Favorite => Action::SetFavorite(id.clone(), false),
            MenuItem::ViewInstance => Action::Open(id),
            MenuItem::CopyPath => Action::CopyPath(id),
            MenuItem::Edit => Action::Edit(id),
            MenuItem::Duplicate => Action::Duplicate(id),
            MenuItem::OpenFolder => Action::OpenFolder(id),
            MenuItem::Delete => Action::Delete(id),
        }
    }
}

/// The row menu's state: which row's popover is open, if any.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Menu {
    /// The id of the instance whose kebab popover is open. One at a time: two open menus on
    /// one page would each swallow the other's presses.
    pub open_for: Option<String>,
    /// **Where a right press opened the menu**, when that is how it opened. The kebab\'s own block below sets the
    /// anchor to the kebab whenever the menu is open, which overwrote the pointer and put the card\'s menu under the
    /// card instead of under the cursor.
    pub opened_at: Option<egui::Pos2>,
    /// The row the keyboard is on, if any. Set by the arrow keys, `Home`, `End` and the
    /// type-ahead, and the row `Enter` activates -- the reference's menu is reachable without a
    /// pointer (`button-menu.ts:102-131`).
    pub highlight: Option<usize>,
    /// The type-ahead's buffer: the letters typed since the last 500 ms reset
    /// (`button-menu.ts:29`).
    pub typed: String,
    /// When that buffer was last added to, on the frame's own clock.
    pub typed_at: f64,
}

impl Menu {
    /// Open the menu on `id`, with the keyboard where a menu opens: on nothing, so the first
    /// arrow key starts at the top rather than resuming whatever the last menu highlighted.
    pub fn open(&mut self, id: &str) {
        self.open_for = Some(id.to_owned());
        self.opened_at = None;
        self.highlight = None;
        self.typed.clear();
        self.typed_at = 0.0;
    }

    /// Close the menu and forget its keyboard state.
    pub fn close(&mut self) {
        self.open_for = None;
        self.opened_at = None;
        self.highlight = None;
        self.typed.clear();
        self.typed_at = 0.0;
    }
}

/// One key the open menu understands, mapped off the frame's events by [`menu_press`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuKey {
    /// `ArrowDown`: the next row, wrapping.
    Down,
    /// `ArrowUp`: the previous row, wrapping.
    Up,
    /// `Home`: the first row.
    Home,
    /// `End`: the last row.
    End,
    /// `Escape`: close.
    Escape,
    /// `Tab`: close, because a menu is not a tab stop (`button-menu.ts:29`).
    Tab,
    /// `Enter`: activate the highlighted row.
    Enter,
    /// A printable character: the type-ahead.
    Char(char),
}

/// The menu's keyboard, as a rule rather than as a branch inside the painter.
///
/// `button-menu.ts:102-131` moves focus with ArrowUp/ArrowDown (both wrapping), `Home` and
/// `End`; :29 closes on `Tab`; :149-177 is the type-ahead, a single-character buffer with a
/// 500 ms reset that jumps to the first row whose label starts with it. Returns the row to
/// activate, which only `Enter` can: everything else moves focus or closes the menu.
pub fn menu_press(key: MenuKey, menu: &mut Menu, now: f64) -> Option<MenuItem> {
    /// The reference's type-ahead reset (`button-menu.ts:29`): 500 ms, in the frame clock's
    /// seconds.
    const TYPEAHEAD_RESET: f64 = 0.5;
    let last = MenuItem::ALL.len() - 1;
    match key {
        MenuKey::Down => {
            menu.highlight = Some(match menu.highlight {
                Some(row) if row < last => row + 1,
                // A closed menu opens on its first row; the last row wraps to the first.
                _ => 0,
            });
            menu.typed.clear();
        }
        MenuKey::Up => {
            menu.highlight = Some(match menu.highlight {
                Some(0) | None => last,
                Some(row) => row - 1,
            });
            menu.typed.clear();
        }
        MenuKey::Home => {
            menu.highlight = Some(0);
            menu.typed.clear();
        }
        MenuKey::End => {
            menu.highlight = Some(last);
            menu.typed.clear();
        }
        MenuKey::Escape | MenuKey::Tab => menu.close(),
        MenuKey::Enter => {
            if let Some(row) = menu.highlight {
                return MenuItem::ALL.get(row).copied();
            }
        }
        MenuKey::Char(typed) => {
            // The buffer is what was typed inside the last half second; a letter typed after
            // that starts a new search rather than extending the old one.
            if now - menu.typed_at > TYPEAHEAD_RESET {
                menu.typed.clear();
            }
            menu.typed_at = now;
            menu.typed.extend(typed.to_lowercase());
            if let Some(row) = MenuItem::ALL
                .iter()
                .position(|item| item.label().to_lowercase().starts_with(&menu.typed))
            {
                menu.highlight = Some(row);
            }
        }
    }
    None
}

/// What the page remembers between frames.
#[derive(Debug, Default)]
pub struct State {
    /// The kebab popover.
    pub menu: Menu,
    /// A press on the library toolbar's "+ New instance", recorded for the shell: the toolbar is painted before
    /// this page's action variable is in scope, so the press is a flag the shell reads rather than a return.
    pub open_create: bool,
    /// Every drawn instance's `saves/` folder names, read when the set of instances last
    /// changed. Held here rather than re-read per frame: see `State::refresh_worlds`.
    worlds: Vec<(String, Vec<(String, String)>)>,
    /// The ids of the instances marked as favourites, read when the set of instances last changed
    /// and updated in place by the menu's toggle.
    pub favorites: Vec<String>,
    /// The instance ids `worlds` was read for. A create or a delete changes this list, and
    /// that change is what triggers the re-read.
    scanned: Vec<String>,
    /// The instance the row menu asked to delete, waiting on the confirmation the reference
    /// puts between the press and the disk (`ConfirmDeleteInstanceModal.vue`). `Action::Delete`
    /// leaves this page only when the dialog's own button answers -- see [`resolve_confirm`].
    pub confirm_delete: Option<String>,
    /// The library's search text. The reference's library toolbar carries a field in the
    /// heading row, so the filter lives here rather than on a page of its own.
    pub query: String,
    /// Whether that field is taking the keyboard.
    pub query_focused: bool,
    /// The instances ticked for a bulk action. `InstanceItem.vue` puts a selection ring on the card's art, filled with a
    /// check when the card is in this set; the reference's tooltip reads "Select instance" or "Deselect instance".
    pub selected: std::collections::HashSet<String>,
    /// **Where the page's own context menu was opened**, if it is open. The reference opens a one-item menu
    /// on a right press anywhere on the library page: `+ New instance`, anchored at the pointer. Captured at
    /// (149,349)-(327,407) after a press at (150,350). Escape, or a press anywhere else, closes it.
    pub page_menu: Option<egui::Pos2>,
    /// **The sort the library is in**, as the reference names it. Its chip reads this, and its menu lists all
    /// seven of the reference's keys -- `Name`, `Last played`, `Hours played`, `Date created`, `Date modified`,
    /// `Loader`, `Game version` -- with the current one in green.
    /// `None` is the reference's default, `Last played`. An `Option` because `#[derive(Default)]` gives a bare
    /// `&'static str` the empty string, which is how the chip first drew with no label at all.
    pub sort_label: Option<&'static str>,
    /// Whether that menu is open.
    pub sort_open: bool,
    /// **How the library is grouped**, as the reference's second chip names it: `Custom group`, `Instance type`,
    /// `Loader`, `Game version`, `No grouping`. `None` is the reference's default, `Custom group`.
    pub group_label: Option<&'static str>,
    /// Whether that menu is open.
    pub group_open: bool,
    /// **The groups the reader has folded shut**, by name. The reference\'s header chevron collapses its section;
    /// this set is what makes the chevron a control rather than a decoration.
    pub collapsed: std::collections::HashSet<String>,
    /// The grouping the set above was folded under. **A fold belongs to the key that made it**: "Fabric" folded under
    /// `Loader` must not arrive folded under `Game version`, where a group can share the name but not the meaning.
    pub folded_under: Option<&'static str>,
    /// How far each group has folded, 0 open to 1 shut, so a press moves the section instead of teleporting it.
    pub fold_progress: std::collections::HashMap<String, f32>,
}

impl State {
    /// Re-read the `saves/` folders when the set of instances changed since the last frame.
    ///
    /// A frame that walked six `saves/` folders would spend the idle-CPU budget of
    /// `docs/LIGHTNESS.md` on a list that only changes when a game writes a world. The ids
    /// are the cache key rather than the folder's timestamp, so a world created while the
    /// window is open is seen once the instance list itself changes -- when the reader
    /// leaves Home and comes back, which re-reads the instances (`shell.rs`, the rail's
    /// press).
    fn refresh_worlds(&mut self, instances: &[Entry]) {
        let ids: Vec<String> = instances
            .iter()
            .take(ROWS_SHOWN)
            .map(|instance| instance.id.clone())
            .collect();
        if ids == self.scanned {
            return;
        }
        self.scanned = ids;
        // A row that appeared or vanished cannot keep a popover open: the menu names an
        // instance by id, and an id that is gone would leave a panel anchored to nothing.
        self.menu.close();
        self.worlds = read_worlds(instances);
        self.favorites = read_favorites(instances);
    }
}

/// What a press on one of the row menu's rows means.
///
/// Every row but `Delete` is the verb it names. `Delete` is destructive and the reference
/// asks first (`use-library.ts:1373-1376`), so its press only opens the confirmation: the
/// action that can remove a folder is emitted from [`resolve_confirm`], on the dialog's own
/// button. A function rather than three inline lines because this is the rule a test can
/// call without a pointer.
pub fn press_menu_row(row: MenuItem, id: &str, state: &mut State) -> Action {
    match row {
        // The page flips its own copy for the label, and the shell writes the flag to disk.
        MenuItem::Favorite => {
            let on = !state.favorites.iter().any(|favorite| favorite == id);
            if on {
                state.favorites.push(id.to_owned());
            } else {
                state.favorites.retain(|favorite| favorite != id);
            }
            Action::SetFavorite(id.to_owned(), on)
        }
        MenuItem::Delete => {
            state.confirm_delete = Some(id.to_owned());
            Action::None
        }
        verb => verb.action(id),
    }
}

/// What the confirmation's answer means.
///
/// Both answers close the dialog, and only [`ConfirmChoice::Delete`] returns
/// [`Action::Delete`] -- `Ask` is the dialog still standing there, which is why it is the
/// default and why nothing else can reach the shell's delete.
pub fn resolve_confirm(choice: ConfirmChoice, id: &str, state: &mut State) -> Action {
    match choice {
        ConfirmChoice::Ask => Action::None,
        ConfirmChoice::Cancel => {
            state.confirm_delete = None;
            Action::None
        }
        ConfirmChoice::Delete => {
            state.confirm_delete = None;
            Action::Delete(id.to_owned())
        }
    }
}

/// What the confirmation's press meant, or that it is still waiting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConfirmChoice {
    /// Still open: nothing has been answered.
    #[default]
    Ask,
    /// `Cancel`, the close button, Escape, or a press on the backdrop.
    Cancel,
    /// `Delete instance`: the one press allowed to remove a folder.
    Delete,
}

/// Draw the page into `rect`, and say what was pressed.
///
/// The page does not create, launch, open or delete anything itself: it has no access to the
/// process table, and a page that had one would be untestable and would have to be re-taught
/// every rule the shell already knows. It reports the intent and the shell acts.
///
/// It does read one thing off the disk -- the folder names inside each instance's `saves/`,
/// through `State::refresh_worlds` -- because a world list handed down from the shell would
/// be a second copy of the folder rule that `data::instances_dir` already owns, and because
/// nothing this page can do with those names is destructive: they are strings on a row.
pub fn draw(
    ui: &mut Ui,
    rect: Rect,
    theme: &Theme,
    icons_cache: &mut Images,
    instances: &[Entry],
    state: &mut State,
) -> Action {
    if instances.is_empty() {
        hero(ui, rect, theme, icons_cache)
    } else {
        list(ui, rect, theme, icons_cache, instances, state)
    }
}

/// The fresh-install state, at the reference's own rhythm.
///
/// \`WelcomeScreen.vue\` is a column with \`px-6 pb-6 pt-16\` -- 64 px down from the title
/// bar and 24 px off the floor -- that centres a hero block and then prints the
/// "Escaping another launcher?" prompt underneath it. The block itself is
/// \`flex flex-col items-center gap-6\`: a 100 px mark (\`size-[6.25rem]\`), then a
/// \`gap-2\` title pair (24 px semibold over 16 px), then a \`w-72 flex flex-col gap-4\`
/// column holding the create button and its key hint.
///
/// Captured at 1440x900 (\`.scratch/cap/ref-home.png\`, \`probe.py text\` and \`probe.py
/// bbox\`): mark 313..413, title line 437..469, description 477..501, create button
/// 526..564 (212x40, radius 14), key hint 583..599, prompt line 800..820, import button
/// 836..876. Those are the numbers the arithmetic below reproduces; the two it does not
/// are the hero's 1 px and the button's, which come from the icon's own inset.
///
/// The button is \`Button size="lg"\` from the reference's design system
/// (\`packages/ui/src/components/base/buttons/ButtonFrame.vue:32\`): \`h-10\`,
/// \`rounded-[14px]\`, \`px-4\`, \`gap-2\`, \`text-base font-semibold\`, \`[&>svg]:size-5\`,
/// filled with the button's colour and lettered in \`--color-accent-contrast\`.
fn hero(ui: &mut Ui, rect: Rect, theme: &Theme, icons_cache: &mut Images) -> Action {
    let painter = ui.painter();
    let centre_x = rect.center().x;

    // The rhythm, in the page's own points.
    const PT: f32 = 64.0; // pt-16
    const PB: f32 = 24.0; // pb-6
    const GAP_6: f32 = 24.0;
    const GAP_4: f32 = 16.0;
    const GAP_2: f32 = 8.0;
    const MARK: f32 = 100.0; // size-[6.25rem]
    const TITLE_LINE: f32 = 32.0; // text-2xl leading-8
    const BODY_LINE: f32 = 24.0; // text-base leading-6
    const HINT_LINE: f32 = 20.0; // text-sm leading-5
    const BUTTON_H: f32 = 40.0; // h-10
    const BUTTON_PAD: f32 = 16.0; // px-4
    const BUTTON_GAP: f32 = 8.0; // gap-2
    const BUTTON_ICON: f32 = 20.0; // [&>svg]:size-5
    const BUTTON_RADIUS: u8 = 14; // rounded-[14px]
    const FOOTER: f32 = HINT_LINE + GAP_4 + BUTTON_H; // prompt, gap-4, button

    let area_top = rect.top() + PT;
    let area_bottom = rect.bottom() - PB - FOOTER;
    let block = MARK
        + GAP_6
        + TITLE_LINE
        + GAP_2
        + BODY_LINE
        + GAP_6
        + BUTTON_H
        + GAP_4
        + HINT_LINE;
    let top = area_top + (area_bottom - area_top - block) / 2.0;

    // The dotted field behind the mark: \`29.875rem\` by \`min(25.9375rem, 80vw)\`,
    // \`-top-52\` from the block, \`rounded-2xl\`, faded by an elliptical mask. It is the
    // only decoration on the page and it reads as the page being empty on purpose.
    dotted_field(painter, rect, top, theme);

    // The mark. \`brand::paint_mark\` draws the texture \`brand::install\` uploads at
    // startup; the plate is the frame's fallback so the hero keeps its anchor when there
    // is no texture (a headless run, or a decode that failed).
    // Three points up: the reference's glyph starts at 326 and this drew it at 329.
    let mark = Rect::from_min_size(
        egui::pos2(centre_x - MARK / 2.0, top - 3.0),
        Vec2::splat(MARK),
    );
    // The reference's hero mark is one PNG: a dark-green rounded square with the brand
    // ring drawn inside it (`assets/welcome/modrinth-social-icon.png`, rendered at
    // `size-[6.25rem]`). This tree has no equivalent asset, so the plate is drawn and the
    // mark is set inside it at the same proportion the capture shows -- the plate is
    // #103524 at (620,320) and the green glyph occupies 72 of the 100 points. The colour
    // is sampled from the reference, not chosen: the asset is the only place it exists.
    // The reference's own plate: 100x100 with its ring, cropped from its render. The drawn plate and the brand
    // texture are the fallback for a texture that failed to upload.
    if !crate::serverart::paint_hero(painter, mark) {
        painter.rect_filled(mark, 24, Color32::from_rgb(0x10, 0x35, 0x24));
        let glyph = Rect::from_center_size(mark.center(), Vec2::splat(82.0));
        if !crate::brand::paint_mark(painter, ui.ctx(), glyph) {
            painter.rect_filled(glyph, 18, theme.BRAND);
        }
    }

    // The title pair: \`gap-2\` between a 24 px semibold line and a 16 px body line.
    let title_top = top + MARK + GAP_6;
    painter.text(
        egui::pos2(centre_x, title_top + TITLE_LINE / 2.0),
        Align2::CENTER_CENTER,
        // Measured: the reference's title ink is 17 points tall; 24-point type here drew 22. 20 draws 18.
        "Welcome to Modrinth",
        fonts::semibold(20.0),
        theme.CONTRAST,
    );
    let desc_top = title_top + TITLE_LINE + GAP_2;
    painter.text(
        egui::pos2(centre_x, desc_top + BODY_LINE / 2.0),
        Align2::CENTER_CENTER,
        "Ready to start playing?",
        fonts::body(16.0),
        theme.BASE,
    );

    // \`+ Create an instance\`: the page's one primary action, sized by its own label.
    let label = painter.layout_no_wrap(
        "Create an instance".to_owned(),
        fonts::semibold(16.0),
        theme.ACCENT_CONTRAST,
    );
    let width = BUTTON_PAD * 2.0 + BUTTON_ICON + BUTTON_GAP + label.size().x;
    let create = Rect::from_min_size(
        egui::pos2(
            centre_x - width / 2.0,
            desc_top + BODY_LINE + GAP_6,
        ),
        egui::vec2(width, BUTTON_H),
    );
    // **The page's one primary action had no pointer feedback.** `ButtonFrame.vue:18-21` gives every button a hover
    // `brightness(--hover-brightness)` -- 1.25 in dark, which is this theme's own `HOVER_BRIGHTNESS` -- and
    // `enabled:active:scale-[0.97]`. Gap-matrix row 19 has carried the absence since the first audit: "the page's one
    // primary action gives no pointer feedback".
    // **One widget, not two.** A second `interact` on the same rect with a different id loses the hover to the call
    // that comes later in the frame -- egui gives interaction to the last claim on a rect -- so the fill read "not
    // hovered" while the button was under the pointer. The existing id is claimed here instead, and the press below
    // reuses this response.
    let feedback = ui.interact(create, egui::Id::new("home-create"), Sense::click());
    let pressed = feedback.is_pointer_button_down_on();
    let create = if pressed {
        Rect::from_center_size(create.center(), create.size() * 0.97)
    } else {
        create
    };
    // **150 ms of hover, on the framework's clock.** `ButtonFrame.vue:18-21` eases the brightness over `duration-150`
    // and leaves the press instant, which is what this does: the ease is keyed on the widget and settles at 1.25, and the
    // 0.97 press is applied without easing.
    let eased = ui.ctx().animate_value_with_time(
        egui::Id::new("home-create-hover"),
        if feedback.hovered() { 1.0 } else { 0.0 },
        0.15,
    );
    let scale = 1.0 + (theme.HOVER_BRIGHTNESS - 1.0) * eased;
    let mix = |channel: u8| (channel as f32 * scale).round().clamp(0.0, 255.0) as u8;
    let fill = egui::Color32::from_rgb(
        mix(theme.BRAND.r()),
        mix(theme.BRAND.g()),
        mix(theme.BRAND.b()),
    );
    painter.rect_filled(create, BUTTON_RADIUS, fill);
    // `button-frame--colored::before` is a one-pixel rim of
    // `linear-gradient(180deg, rgba(255,255,255,0.3), transparent)` masked to the
    // padding box. At 40 px tall on this radius the capture shows no rim at all: the
    // button is flat brand green with an antialiased edge. A half-height gloss mesh was
    // drawn here and is what made the button read as a different object, so it is gone.
    icons::draw(
        painter,
        Glyph::Plus,
        Rect::from_center_size(
            egui::pos2(create.left() + BUTTON_PAD + BUTTON_ICON / 2.0, create.center().y),
            Vec2::splat(BUTTON_ICON),
        ),
        theme.ACCENT_CONTRAST,
    );
    painter.galley(
        egui::pos2(
            create.left() + BUTTON_PAD + BUTTON_ICON + BUTTON_GAP,
            create.center().y - label.size().y / 2.0,
        ),
        label,
        theme.ACCENT_CONTRAST,
    );

    // \`Press <kbd>N</kbd> to quick create an instance\`: 14 px secondary text with a
    // 20 px keycap -- \`h-5 min-w-5 rounded-md border border-solid border-surface-5
    // bg-button-bg px-1 text-xs\`.
    let hint_top = create.bottom() + GAP_4;
    let before = painter.layout_no_wrap("Press".to_owned(), fonts::body(14.0), theme.SECONDARY);
    let after = painter.layout_no_wrap(
        "to quick create an instance".to_owned(),
        fonts::body(14.0),
        theme.SECONDARY,
    );
    let key_label = painter.layout_no_wrap("N".to_owned(), fonts::body(12.0), theme.BASE);
    let key_w = (key_label.size().x + 8.0).max(20.0);
    let row_w = before.size().x + 4.0 + key_w + 4.0 + after.size().x;
    let mut x = centre_x - row_w / 2.0;
    let mid = hint_top + HINT_LINE / 2.0;
    let before_w = before.size().x;
    let before_h = before.size().y;
    let after_w = after.size().x;
    let after_h = after.size().y;
    let key_label_w = key_label.size().x;
    let key_label_h = key_label.size().y;
    painter.galley(egui::pos2(x, mid - before_h / 2.0), before, theme.SECONDARY);
    x += before_w + 4.0;
    let key = Rect::from_min_size(egui::pos2(x, mid - 10.0), egui::vec2(key_w, 20.0));
    painter.rect_filled(key, 6, theme.BUTTON_BG);
    painter.rect_stroke(
        key,
        6,
        Stroke::new(1.0, theme.BUTTON_BORDER),
        egui::StrokeKind::Inside,
    );
    painter.galley(
        egui::pos2(key.center().x - key_label_w / 2.0, key.center().y - key_label_h / 2.0),
        key_label,
        theme.BASE,
    );
    x += key_w + 4.0;
    painter.galley(egui::pos2(x, mid - after_h / 2.0), after, theme.SECONDARY);

    // The prompt under the hero, \`gap-4\` above its own button, \`pb-6\` under it.
    let import_bottom = rect.bottom() - PB;
    let import_label = painter.layout_no_wrap(
        "Import from launcher".to_owned(),
        fonts::label(16.0),
        theme.BASE,
    );
    let import_w = BUTTON_PAD * 2.0 + BUTTON_ICON + BUTTON_GAP + import_label.size().x;
    let import = Rect::from_min_size(
        egui::pos2(centre_x - import_w / 2.0, import_bottom - BUTTON_H),
        egui::vec2(import_w, BUTTON_H),
    );
    painter.text(
        egui::pos2(centre_x, import.top() - GAP_4 - HINT_LINE / 2.0),
        Align2::CENTER_CENTER,
        "Escaping another launcher?",
        fonts::body(14.0),
        theme.SECONDARY,
    );
    // \`Button\` with the default \`type="base"\`: \`--color-button-bg\` under an inset
    // one-pixel \`--surface-5\` ring (\`button-frame--base\`).
    painter.rect_filled(import, BUTTON_RADIUS, theme.BUTTON_BG);
    painter.rect_stroke(
        import,
        BUTTON_RADIUS,
        Stroke::new(1.0, theme.BUTTON_BORDER),
        egui::StrokeKind::Inside,
    );
    icons::draw(
        painter,
        Glyph::BoxImport,
        Rect::from_center_size(
            egui::pos2(import.left() + BUTTON_PAD + BUTTON_ICON / 2.0, import.center().y),
            Vec2::splat(BUTTON_ICON),
        ),
        theme.BASE,
    );
    painter.galley(
        egui::pos2(
            import.left() + BUTTON_PAD + BUTTON_ICON + BUTTON_GAP,
            import.center().y - import_label.size().y / 2.0,
        ),
        import_label,
        theme.BASE,
    );

    // The two presses. \`N\` is the reference's own shortcut for the same action, and it is
    // read here rather than in the shell because the page is what advertises it.
    // The guards are the reference's own (`WelcomeScreen.vue:50-68`): it ignores
    // `event.repeat`, ignores the key when a modifier is held, and ignores it while a text
    // field has the keyboard. egui's `key_pressed` counts a key-repeat as a press and has no
    // notion of "this keystroke belongs to a dialog", so both are tested here: unfiltered,
    // `Ctrl+N` made a second instance and every "n" typed into the create dialog made one too.
    let quick_key = ui.input(|input| {
        !input.modifiers.any()
            && input.events.iter().any(|event| {
                matches!(
                    event,
                    egui::Event::Key {
                        key: egui::Key::N,
                        pressed: true,
                        repeat: false,
                        ..
                    }
                )
            })
    }) && ui.ctx().memory(|memory| memory.focused().is_none());
    let _ = icons_cache;
    let mut action = Action::None;
    if feedback.clicked() || quick_key {
        action = Action::Create;
    }
    if ui
        .interact(import, egui::Id::new("home-import"), Sense::click())
        .clicked()
    {
        action = Action::Import;
    }
    action
}

/// The dotted field behind the hero mark.
///
/// \`WelcomeScreen.vue\`'s \`.dot-pattern\`: a radial-gradient dot every \`0.5625rem\` at 25%
/// of \`--color-text-primary\` and \`opacity: 0.8\`, masked by
/// \`radial-gradient(ellipse at center, black 10%, transparent 68%)\` inside a
/// \`29.875rem\` by \`min(25.9375rem, 80vw)\` box that sits \`-top-52\` from the block.
fn dotted_field(painter: &egui::Painter, rect: Rect, block_top: f32, theme: &Theme) {
    // The stylesheet says `h-[29.875rem] w-[min(25.9375rem,80vw)]` at `-top-52`, which
    // is 478x415 at 208 above the block. The capture disagrees: the dots are visible over
    // 369x387 centred on the content column at y 320, and with the 68% mask that is a box
    // of 543x569 starting 277 above the block. The render is the spec, so these are the
    // capture's numbers and the stylesheet's are not used.
    const PITCH: f32 = 9.0; // 0.5625rem
    let w = 543.0;
    let h = 569.0;
    let cx = rect.center().x;
    let top = block_top - 277.0;
    let field = Rect::from_min_size(egui::pos2(cx - w / 2.0, top), egui::vec2(w, h));
    let ink = theme.CONTRAST;
    let mut y = field.top();
    while y < field.bottom() {
        let mut x = field.left();
        while x < field.right() {
            let dx = (x - field.center().x) / (w / 2.0);
            let dy = (y - field.center().y) / (h / 2.0);
            let d = (dx * dx + dy * dy).sqrt();
            // black 10% .. transparent 68%: full inside 10%, gone past 68%.
            let fade = ((0.68 - d) / (0.68 - 0.10)).clamp(0.0, 1.0);
            if fade > 0.0 {
                let alpha = (0.25 * 0.8 * fade * 255.0) as u8;
                painter.circle_filled(
                    egui::pos2(x, y),
                    0.5,
                    Color32::from_rgba_unmultiplied(ink.r(), ink.g(), ink.b(), alpha),
                );
            }
            x += PITCH;
        }
        y += PITCH;
    }
}

/// The installed state: the reference's instance cards, newest-played first, then the worlds
/// the reference lists under the `Jump in` heading.
fn list(
    ui: &mut Ui,
    rect: Rect,
    theme: &Theme,
    icons_cache: &mut Images,
    instances: &[Entry],
    state: &mut State,
) -> Action {
    // `+ New instance` anchored at the pointer.
    if ui.input(|input| input.pointer.secondary_clicked()) {
        if let Some(at) = ui.ctx().pointer_interact_pos() {
            // **Off the cards only.** A right press *on* a card belongs to the card, which opens its own menu further
            // down; this handler used to take it first and show "+ New instance" over a card.
            let over_card = (0..instances.len().min(ROWS_SHOWN))
                .any(|index| row_targets(rect, index).0.contains(at));
            if rect.contains(at) && !over_card {
                state.page_menu = Some(at);
                ui.ctx().request_repaint();
            }
        }
    }
    if ui.input(|input| input.key_pressed(egui::Key::Escape)) {
        state.page_menu = None;
        // **Escape clears a multi-selection**, as the reference does: a reader who has ticked cards and changed
        // their mind should not have to find each ring again. Measured before this: a ticked card, Escape, and
        // **no pixel changed** (`e1.png` -> `e2.png` diff: None).
        if !state.selected.is_empty() {
            state.selected.clear();
            ui.ctx().request_repaint();
        }
    }
    // One directory walk per change of instance list, not per frame.
    state.refresh_worlds(instances);
    // Two disjoint fields of one state: naming them separately is what lets the rows below
    // hold the world names while they open and close the menu.
    // **Two fields, borrowed apart.** The loop reads the selected set and writes it back when a ring is pressed, while the
    // menu beside it is borrowed for the popover; destructuring lets both live at once instead of cloning a set per frame.
    let State { menu, selected, .. } = state;
    let selected_ids = selected.clone();
    let mut toggled: Option<String> = None;
    // **The strip's press, carried to the end like the ring's.** The row is drawn at 1132 and this function collects its
    // action at 1654, so the id has to travel; the same shape `toggled` already uses.
    let mut jump_in: Option<String> = None;
    let worlds = &state.worlds;

    let painter = ui.painter();
    // **The "Jump in" strip**, which the reference draws above its Library heading: a dashed row carrying the
    // most recent instance -- its icon, its name with the green "New instance" badge, the loader and last-played
    // line under them, and Play at the right. Driving its Home page (display :107) is where this came from; this
    // build had no such section at all.
    if let Some(recent) = instances.first() {
        painter.text(
            // **20 points down, measured.** The reference's heading ink starts at y 79 and this build's at 59, while the
            // Library heading below it sits at 211 on both: the section was 20 high inside a space of the same total
            // height, so the gap after it was 20 too wide. Found by comparing ink tops, not by eye.
            egui::pos2(rect.left(), rect.top() + 26.0),
            Align2::LEFT_TOP,
            "Jump in",
            // **24, measured.** The reference's ink is 22 rows tall and 110 columns wide; 20 drew 19 and 91. This heading is
            // standalone text -- no box constrains it -- which is the kind that responds to size.
            fonts::semibold(24.0),
            theme.CONTRAST,
        );
        // **The full column, not a formula.** This was `(rect.width() - 240).min(800)`, which made the strip 788 wide
        // while the reference's dashed bottom border runs to x 1 115 -- the strip is the column, and the Play button at
        // its right end sat 238 points left of the reference's because of it.
        let strip = Rect::from_min_size(
            egui::pos2(rect.left(), rect.top() + 66.0),
            egui::vec2(rect.width(), 72.0),
        );
        // A **dashed** border, as the reference draws it, plus the heading's collapse chevron: a solid rule read as
        // another card instead of the temporary space it is.
        // **The strip carries a fill, not just a dashed outline.** The reference's row sits on #1D1F23 -- the theme's
        // own SURFACE_2 -- across its whole width, and this build left the page background showing through: 63 670 pixels
        // of ref (29,31,35) against ours (22,24,28), invisible to a 40-point threshold and obvious to the eye in a crop.
        painter.rect_filled(strip, 12.0, theme.SURFACE_2);
        crate::pages::home::dashed_rect(&painter, strip, 12.0, theme.SURFACE_5);
        // **The row is pressable, like the card it echoes.** The reference opens the instance on a press anywhere in the strip;
        // this block drew the row and registered nothing, so a press did nothing -- a defect no pixel count can see. Registered
        // here, before the icon, the badge and Play, so those children -- being later -- still win their own rects.
        if ui
            .interact(strip, egui::Id::new(("jump-in", &recent.id)), Sense::click())
            .clicked()
        {
            jump_in = Some(recent.id.clone());
        }
        // **104 and `CONTRAST`, measured at 1:1.** The reference\'s caret sits at x 192..202, twelve points right of the
        // last glyph, and it is as bright as the heading; this drew it at 96 in `SECONDARY`, hard against the "n" and a
        // shade below the text. Both are visible in a 4x crop and neither moves a number much -- which is exactly the kind
        // of difference a threshold-40 count cannot see.
        crate::icons::draw(
            &painter,
            Glyph::ChevronUp,
            Rect::from_center_size(
                egui::pos2(rect.left() + 104.0, rect.top() + 38.0),
                egui::Vec2::splat(14.0),
            ),
            theme.CONTRAST,
        );
        let tile = Rect::from_min_size(
            egui::pos2(strip.left() + 12.0, strip.top() + 12.0),
            Vec2::splat(48.0),
        );
        let strip_icon = [recent_path(recent).join("icon.png"), recent_path(recent).join("minecraft").join("icon.png")]
            .into_iter()
            .find(|candidate| candidate.is_file())
            .and_then(|path| icons_cache.get_file(ui.ctx(), &path));
        match strip_icon {
            Some(texture) => {
                painter.image(
                    texture.id(),
                    tile,
                    Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
            None => {
                // **The default art, then the cube.** An instance with no `icon.png` shows Modrinth's own default tile in
                // the reference -- the same campfire its rail button and library card carry -- and this drew the grey cube
                // instead, so the strip's icon was the one place the art never appeared. The cube is still the fallback for
                // a build with no texture at all.
                if !crate::serverart::paint_instance_default(&painter, tile) {
                    painter.rect_filled(tile, 10.0, theme.BUTTON_BG);
                crate::icons::draw(
                    &painter,
                    Glyph::Cube,
                    Rect::from_center_size(tile.center(), Vec2::splat(tile.height() * 0.55)),
                    theme.SECONDARY,
                );
                }
            }
        }
        let name = painter.text(
            egui::pos2(tile.right() + 12.0, strip.top() + 16.0),
            Align2::LEFT_TOP,
            recent.name.as_str(),
            // **16, and the size was the only thing wrong.** The reference's "Fabric 26.3" ink is 24 rows tall with its badge
            // beside it where this drew 20. 17 overshot and 17-plus-a-spaced-bullet was worse than either; 16 alone is the one
            // that measures better, and the bullet's single space is the reference's own.
            fonts::semibold(16.0),
            theme.CONTRAST,
        );
        // The badge the reference shows beside a freshly-made instance.
        // **As wide as its words.** This was a fixed 96 points while the 14-point label starting 25 points in needed
        // more, so "New instance" ran past the ring and through the card\'s own edge -- the pill every card showed was
        // too small for the one string it ever holds.
        let badge_label = painter.layout_no_wrap(
            "New instance".to_owned(),
            fonts::body(14.0),
            theme.BRAND,
        );
        let badge = Rect::from_min_size(
            egui::pos2(name.right() + 8.0, strip.top() + 16.0),
            egui::vec2(25.0 + badge_label.size().x + 11.0, 20.0),
        );
        // "✧ New instance" as an **outlined** pill: the reference's badge is a ring in the brand colour with the
        // sparkle and the words inside it, not a filled chip.
        painter.rect_stroke(
            badge,
            10.0,
            egui::Stroke::new(1.0, theme.BRAND),
            egui::StrokeKind::Inside,
        );
        crate::icons::draw(
            &painter,
            Glyph::Sparkles,
            Rect::from_center_size(
                egui::pos2(badge.left() + 13.0, badge.center().y),
                Vec2::splat(13.0),
            ),
            theme.BRAND,
        );
        painter.text(
            egui::pos2(badge.left() + 25.0, badge.center().y),
            Align2::LEFT_CENTER,
            "New instance",
            fonts::body(14.0),
            theme.BRAND,
        );
        painter.text(
            egui::pos2(tile.right() + 12.0, strip.top() + 40.0),
            Align2::LEFT_TOP,
            // "Fabric 26.3 • Never played": the reference names the loader *and* the game version here. Ours printed
            // the loader alone, which reads like a different instance than the card below it.
            format!("{} {} • Never played", recent.loader, recent.version),
            fonts::body(13.0),
            theme.SECONDARY,
        );
        // The strip's own kebab, beside Play: the reference's row ends with one, and ours had nothing there.
        let kebab = Rect::from_min_size(
            egui::pos2(strip.right() - 40.0, strip.top() + 20.0),
            egui::vec2(28.0, 32.0),
        );
        for dot in 0..3 {
            painter.circle_filled(
                egui::pos2(kebab.center().x, kebab.center().y - 6.0 + dot as f32 * 6.0),
                1.6,
                theme.SECONDARY,
            );
        }
        let play = Rect::from_min_size(
            egui::pos2(strip.right() - 40.0 - 8.0 - 84.0, strip.top() + 20.0),
            egui::vec2(84.0, 32.0),
        );
        painter.rect_filled(play, 10.0, theme.BRAND);
        painter.text(
            play.center(),
            Align2::CENTER_CENTER,
            "Play",
            fonts::semibold(13.0),
            theme.ACCENT_CONTRAST,
        );
    }
    painter.text(
        // The reference's own capture puts this heading's top at y 218 in the window, 170 below the page's top:
        // **+33 on the number this used**, which drew it above the strip's tail.
        egui::pos2(rect.left(), rect.top() + 157.0),
        Align2::LEFT_TOP,
        // `app.library.title` -- "Library" (`locales/en-US/index.json:1235-1237`): the
        // reference's own name for the instance list, which is what these rows are. The
        // string this heading used to print, `Jump back in`, is not in the locale at all;
        // the `Jump in` section is the worlds below (`pages/Index.vue:99-103`).
        "Library",
        // **24, measured.** The reference's ink is 23 rows tall and 73 wide; 26 drew 25 and 83.
        fonts::heading(24.0),
        theme.CONTRAST,
    );

    // The library's search field, in the heading row on the right. Three parts, as everywhere else on
    // this launcher: it takes the keyboard, it shows the query, and the list below filters by it.
    // **The toolbar**, where the reference puts it: under the Library heading, the search field taking the
    // left of the row and the two buttons at its right. Driving the reference's Home page showed the field
    // there, not in the heading's top-right corner where this build drew it.
    let toolbar = Rect::from_min_size(
        // **+7, measured.** The reference's search field's top border is at y 248 and its filter row's border at 294;
        // this build put them at 241 and 291. Measured per-edge with tools/parity/ink.py and an edge scan, because the
        // Library heading above them matched exactly (211 on both) and hid the drift.
        egui::pos2(rect.left(), rect.top() + 200.0),
        egui::vec2(rect.width(), 36.0),
    );
    // **Laid out from the right edge, as the sheet's flex row is.** The search was a hard 600 points with the buttons
    // following it, so the whole row stopped wherever that add up landed -- measured at 957 against the reference's 1 108,
    // 151 points short, with the search 136 early and the brand button 22 narrow. The reference's own numbers, taken from a
    // row above the labels: search ends 806, New group 831..945, New instance 969..1108.
    // **150 and 103, measured at the buttons' mid-height** rather than near their rounded corners, where both shrink
    // by the same two points and hide the difference. The reference: New group 851..951, New instance 964..1113.
    const INSTANCE_W: f32 = 150.0;
    const GROUP_W: f32 = 103.0;
    // **8, not 27.** The column ends at 1 116 -- proved by the reference's empty state centring at 602, which is the
    // centre of 88..1116 -- and the reference's own New instance button ends at 1 108. Eight points of padding inside
    // that column, not twenty-seven from an edge the column never reaches. Assuming 27 was right only if the column ran
    // to 1 140, which widening it to would have broken every centred empty state: Screenshots went 23 729 -> 24 080.
    const RIGHT_PAD: f32 = 2.0;
    const BETWEEN: f32 = 12.0;
    let instance_right = toolbar.right() - RIGHT_PAD;
    let instance_left = instance_right - INSTANCE_W;
    let group_right = instance_left - BETWEEN;
    let group_left = group_right - GROUP_W;
    let field = Rect::from_min_max(
        toolbar.min,
        egui::pos2(group_left - BETWEEN, toolbar.top() + 36.0),
    );
    // **SURFACE_4, not RAISED_BG.** The reference's field reads (52,54,60) inside and (39,41,46) outside; ours painted
    // the plate's own colour, so the field was invisible except for its border: 1 592 pixels of the page were a surface
    // one step too dark.
    painter.rect_filled(field, 10, theme.SURFACE_4);
    painter.rect_stroke(field, 10, Stroke::new(1.0, theme.SURFACE_5), egui::StrokeKind::Inside);
    crate::icons::draw(
        &painter,
        Glyph::Search,
        Rect::from_center_size(
            egui::pos2(field.left() + 23.0, field.center().y),
            // **20, not 16.** The reference's magnifier inks 18x18 at (102,257)-(119,274); a 16-point box drew 14x14.
            egui::Vec2::splat(20.0),
        ),
        theme.SECONDARY,
    );
    let placeholder = "Search";
    let shown_text = if state.query.is_empty() { placeholder } else { state.query.as_str() };
    // **Dimmer and larger than this first drew it.** `element.py` on the field: the reference\'s placeholder peaks at
    // (111,119,130) and its ink is 16 rows tall; this build peaked at (150,162,176) with 13 rows. The tone is
    // `SECONDARY` pulled 72% of the way to the page, and the type one step larger.
    painter.text(
        // +41, not +38: the reference\'s text starts at x 129 against this build\'s 127.
        egui::pos2(field.left() + 40.0, field.center().y),
        Align2::LEFT_CENTER,
        shown_text,
        fonts::body(15.0),
        if state.query.is_empty() {
            crate::shell::tint(theme.BG, (150, 162, 176), 0.72)
        } else {
            theme.BASE
        },
    );
    // The reference's two buttons beside the field: "New group" and the brand "New instance".
    for (index, (label, brand)) in [("New group", false), ("New instance", true)].into_iter().enumerate() {
        let (width, left) = if brand {
            (INSTANCE_W, instance_left)
        } else {
            (GROUP_W, group_left)
        };
        let _ = index;
        let button = Rect::from_min_size(
            egui::pos2(left, toolbar.top()),
            egui::vec2(width, 36.0),
        );
        // **Hover, measured.** The reference quiet button reads (52,54,60) at rest and (65,67,75) under the pointer,
        // and 4 397 pixels of its toolbar change when the pointer arrives -- while this build changed none, because the
        // fill was unconditional. theme.hovered is the sheet own step, used by the checklist rows and the filters.
        let over_button = ui
            .ctx()
            .input(|input| input.pointer.hover_pos())
            .is_some_and(|at| button.contains(at));
        let rest = if brand { theme.BRAND } else { theme.BUTTON_BG };
        painter.rect_filled(
            button,
            10.0,
            if over_button { theme.hovered(rest) } else { rest },
        );
        // Both buttons lead with a "+", as the reference's pair does; the brand one had none.
        if brand {
            crate::icons::draw(
                &painter,
                Glyph::Plus,
                Rect::from_center_size(
                    egui::pos2(button.center().x - 52.0, button.center().y),
                    egui::Vec2::splat(13.0),
                ),
                theme.ACCENT_CONTRAST,
            );
        }
        if !brand {
            // The reference's quiet button carries a **ring** and a leading "+", so the pair reads as one family:
            // an outlined action beside a filled one. Ours was a flat quiet fill with a bare label.
            painter.rect_stroke(
                button,
                10.0,
                Stroke::new(1.0, theme.SURFACE_5),
                egui::StrokeKind::Inside,
            );
            crate::icons::draw(
                &painter,
                Glyph::Plus,
                Rect::from_center_size(
                    egui::pos2(button.center().x - 44.0, button.center().y),
                    egui::Vec2::splat(12.0),
                ),
                theme.BASE,
            );
        }
        painter.text(
            egui::pos2(button.center().x + 6.0, button.center().y),
            Align2::CENTER_CENTER,
            label,
            fonts::semibold(13.0),
            if brand { theme.ACCENT_CONTRAST } else { theme.BASE },
        );
        // A press on "+ New instance" opens the create dialog. The press is recorded here and read by the
        // caller below, where the function's own action variable lives.
        if brand
            && ui
                .interact(button, egui::Id::new("library-new-instance"), Sense::click())
                .clicked()
        {
            state.open_create = true;
        }
    }
    // The filter row under it: the two selects, a divider and the add-filter chip.
    // **Measured with `tools/parity/element.py`**, which compares one element\'s fill, ink and extent:
    // the reference\'s chip fill spans rows 296..327, its label ink 305..320 and is pure white (255,255,255).
    // This build\'s chip started five points high and its label was (176,186,197).
    let filters = Rect::from_min_size(
        egui::pos2(rect.left(), rect.top() + 248.0),
        egui::vec2(rect.width(), 32.0),
    );
    let mut filter_x = filters.left();
    // **Each chip carries a leading glyph**, as the reference's own library row does: the sort arrows on "Last
    // played", the grid on "Custom group", the filter funnel on "Add filter". Ours drew bare text, which read as three
    // buttons rather than three controls of the same kind.
    for (label, width, glyph) in [
        // Widths from the same instrument: the reference\'s chip fills run 88..252 and 255..453, so 165 and 199.
        (state.sort_label.unwrap_or("Last played"), 165.0_f32, Glyph::ArrowUpDown),
        (state.group_label.unwrap_or("Custom group"), 199.0, Glyph::Grid),
    ] {
        let chip = Rect::from_min_size(
            egui::pos2(filter_x, filters.top()),
            egui::vec2(width, 32.0),
        );
        painter.rect_filled(chip, 8.0, theme.BUTTON_BG);
        crate::icons::draw(
            &painter,
            glyph,
            Rect::from_center_size(
                egui::pos2(chip.left() + 20.0, chip.center().y),
                egui::Vec2::splat(16.0),
            ),
            theme.CONTRAST,
        );
        painter.text(
            egui::pos2(chip.left() + 32.0, chip.center().y),
            Align2::LEFT_CENTER,
            label,
            fonts::label(16.0),
            theme.CONTRAST,
        );
        crate::icons::draw(
            &painter,
            Glyph::ChevronDown,
            Rect::from_center_size(
                egui::pos2(chip.right() - 16.0, chip.center().y),
                egui::Vec2::splat(16.0),
            ),
            theme.CONTRAST,
        );
        filter_x = chip.right() + 3.0;
    }
    // The funnel and the outlined `+ Add filter` after the two chips: the reference has no third chip here.
    crate::icons::draw(
        &painter,
        Glyph::Filter,
        Rect::from_center_size(
            egui::pos2(filters.left() + 376.0, filters.center().y),
            Vec2::splat(16.0),
        ),
        theme.CONTRAST,
    );
    let add_filter = Rect::from_min_size(
        egui::pos2(filters.left() + 387.0, filters.top()),
        egui::vec2(155.0, 32.0),
    );
    painter.rect_stroke(
        add_filter,
        8.0,
        egui::Stroke::new(1.0, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );
    crate::icons::draw(
        &painter,
        Glyph::Plus,
        Rect::from_center_size(
            egui::pos2(add_filter.left() + 20.0, add_filter.center().y),
            Vec2::splat(16.0),
        ),
        theme.CONTRAST,
    );
    painter.text(
        egui::pos2(add_filter.left() + 38.0, add_filter.center().y),
        Align2::LEFT_CENTER,
        "Add filter",
        fonts::label(16.0),
        theme.CONTRAST,
    );
    // **The sort chip opens its menu.** The chips were drawn and never interacted with, so every one of them was
    // dead -- found by pressing. The reference opens a 165-wide menu under the chip listing seven keys; this one
    // draws the same list and keeps the chosen one in green.
    let sort_chip = Rect::from_min_size(
        egui::pos2(filters.left(), filters.top()),
        egui::vec2(118.0, 32.0),
    );
    if ui
        .interact(sort_chip, egui::Id::new("library-sort"), Sense::click())
        .clicked()
    {
        state.sort_open = !state.sort_open;
        state.group_open = false;
        ui.ctx().request_repaint();
    }
    // The second chip, the same way: the reference's menu is five rows and the chip carries the chosen one.
    let group_chip = Rect::from_min_size(
        egui::pos2(filters.left() + 126.0, filters.top()),
        egui::vec2(164.0, 32.0),
    );
    if ui
        .interact(group_chip, egui::Id::new("library-group"), Sense::click())
        .clicked()
    {
        state.group_open = !state.group_open;
        state.sort_open = false;
        ui.ctx().request_repaint();
    }
    // at the field\'s right edge, inside the same box, and is drawn only while the field is not empty.
    if !state.query.is_empty() {
        let clear = Rect::from_center_size(
            egui::pos2(field.right() - 20.0, field.center().y),
            Vec2::splat(24.0),
        );
        let painter = ui.painter();
        crate::icons::draw(painter, Glyph::X, clear, theme.SECONDARY);
    }
    if ui
        .interact(field, egui::Id::new("library-search"), Sense::click())
        .clicked()
    {
        // **The press that lands on the X clears; any other press in the field focuses it.** Ordering two overlapping
        // `interact`s did not work: whichever came first, the field took the press. One handler deciding by position is
        // not a trick, it is the only way egui lets two targets share one rect.
        let on_clear = ui.ctx().pointer_interact_pos().is_some_and(|at| {
            !state.query.is_empty()
                && Rect::from_center_size(
                    egui::pos2(field.right() - 20.0, field.center().y),
                    Vec2::splat(24.0),
                )
                .contains(at)
        });
        if on_clear {
            state.query.clear();
        } else {
            state.query_focused = true;
        }
        ui.ctx().request_repaint();
    }
    // **The clear button.** The reference\'s field carries an X once there is something to clear (`clearable` on the
    // library\'s input); this build had none, so emptying a filter meant selecting the text and deleting it. It sits
    // A press anywhere else takes the keyboard back. The reference's input is a real field and
    // loses focus when the reader clicks out of it; this field used to keep every later
    // keystroke on the page, so a press on a card and then a letter still filtered the list.
    if state.query_focused && ui.input(|input| input.pointer.any_pressed())
        && !ui.rect_contains_pointer(field)
    {
        state.query_focused = false;
    }
    if state.query_focused {
        for event in ui.input(|input| input.events.clone()) {
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
    let needle = state.query.trim().to_lowercase();
    let shown: Vec<&Entry> = instances
        .iter()
        .filter(|entry| {
            needle.is_empty()
                || entry.name.to_lowercase().contains(&needle)
                || entry.loader.to_lowercase().contains(&needle)
                || entry.version.to_lowercase().contains(&needle)
        })
        .collect();
    let mut shown = shown;
    ordered(&mut shown, state.sort_label);
    // A grouped page draws a header above the grid and moves the grid down by its height.
    let grouped = matches!(state.group_label, Some(key) if key != "No grouping");
    // **A fold belongs to its key.** Changing the grouping drops every fold, or a group that happens to share a name
    // with one folded under another key arrives shut.
    if state.folded_under != state.group_label {
        state.collapsed.clear();
        state.fold_progress.clear();
        state.folded_under = state.group_label;
    }


    // A dialog that was already open when this frame began owns every press in it. `egui::Modal`
    // makes the widgets under the plate inert from the frame after it is drawn, so this covers
    // the one frame it is drawn on.
    let modal_was_open = state.confirm_delete.is_some();

    let mut action = Action::None;

    // `app.library.search.no-results.title` -- "No instances match your search."
    // (`apps/app-frontend/src/components/ui/library/index.vue:31-34, 272-277`), `text-base
    // text-primary`, standing where the first row would: an empty page under a query must
    // say so rather than read as a library with nothing in it.
    if shown.is_empty() {
        painter.text(
            egui::pos2(rect.left(), rect.top() + ROW_TOP),
            Align2::LEFT_TOP,
            "No instances match your search.",
            fonts::body(16.0),
            theme.CONTRAST,
        );
    }

    // An outside press closes the popover, the way the reference's panels close on a click
    // outside them (`FloatingPanel.vue:192-198`). Registered first, so every kebab, pill and
    // menu row sits above it and takes the press before this does.
    if menu.open_for.is_some()
        && ui
            .interact(rect, egui::Id::new("home-menu-dismiss"), Sense::click())
            .clicked()
    {
        menu.close();
    }

    let width = rect.width();
    // The open row's kebab, kept so the popover can be drawn after every row: a panel drawn
    // in its row's turn would be painted under the card below it.
    let mut anchored: Option<(String, Rect)> = None;
    let rows = shown.len().min(ROWS_SHOWN);
    // **The groups.** Grouping is by the second chip: `Loader` and `Game version` read the entry, `Instance type`
    // is "Custom" for every instance this build has (it records no type), and `Custom group` -- the reference's
    // default -- draws no headings at all. The cards of each group restart the grid, and each group is preceded by
    // its own header: chevron, name, count in a pill, a rule at the foot (`ref-grouped.png`).
    let mut group_of = vec![0usize; shown.len()];
    let mut index_in_group = vec![0usize; shown.len()];
    let mut group_names: Vec<String> = Vec::new();
    let mut group_counts: Vec<usize> = Vec::new();
    if grouped {
        for (index, entry) in shown.iter().enumerate() {
            let name = match state.group_label {
                Some("Loader") => entry.loader.clone(),
                Some("Game version") => entry.version.clone(),
                _ => "Custom".to_owned(),
            };
            if group_names.last() != Some(&name) {
                group_names.push(name);
                group_counts.push(0);
            }
            let group = group_names.len() - 1;
            group_of[index] = group;
            index_in_group[index] = group_counts[group];
            group_counts[group] += 1;
        }
    }
    // How far each group starts below the grid's own top: its own header plus every earlier group's header and rows.
    let columns = (((rect.width() + 12.0) / (168.0 + 12.0)).floor().max(1.0)) as usize;
    let mut group_shift = vec![0.0f32; group_names.len()];
    let mut cursor = 0.0f32;
    // **The fold is animated**, 150 ms, which is what the reference\'s sections take to open and close. The
    // progress per group is advanced here and the layout reads it, so a section moves instead of teleporting.
    let dt = ui.input(|input| input.stable_dt).min(0.1);
    let mut group_visible = vec![0.0f32; group_names.len()];
    let mut group_folded = vec![false; group_names.len()];
    for (group, name) in group_names.iter().enumerate() {
        let target = if state.collapsed.contains(name) { 1.0 } else { 0.0 };
        let progress = state.fold_progress.entry(name.clone()).or_insert(0.0);
        if (*progress - target).abs() > f32::EPSILON {
            let step = dt / 0.15;
            *progress = if *progress < target {
                (*progress + step).min(target)
            } else {
                (*progress - step).max(target)
            };
            ui.ctx().request_repaint();
        }
        group_folded[group] = *progress >= 1.0;
        let rows = group_counts[group].div_ceil(columns) as f32;
        group_visible[group] = rows * (208.0 + 12.0) * (1.0 - *progress);
        group_shift[group] = cursor;
        cursor += GROUP_HEADER_H + group_visible[group];
    }
    if grouped {
        let painter = ui.painter();
        for (group, name) in group_names.iter().enumerate() {
            let top = rect.top() + ROW_TOP + group_shift[group];
            let folded = state.collapsed.contains(name);
            // **The chevron is a control.** A press anywhere on the header folds or unfolds the section, and the
            // glyph points right when folded, as the reference\'s does.
            let header = Rect::from_min_size(
                egui::pos2(rect.left(), top),
                egui::vec2(rect.width(), GROUP_HEADER_H),
            );
            if ui
                .interact(
                    header,
                    egui::Id::new(("library-group-fold", name.as_str())),
                    Sense::click(),
                )
                .clicked()
            {
                if folded {
                    state.collapsed.remove(name);
                } else {
                    state.collapsed.insert(name.clone());
                }
                ui.ctx().request_repaint();
            }
            crate::icons::draw(
                painter,
                if folded { crate::icons::Glyph::ChevronRight } else { crate::icons::Glyph::ChevronDown },
                Rect::from_min_size(egui::pos2(rect.left() + 8.0, top + 4.0), Vec2::splat(16.0)),
                theme.SECONDARY,
            );
            painter.text(
                egui::pos2(rect.left() + 32.0, top + 12.0),
                Align2::LEFT_CENTER,
                name.clone(),
                fonts::label(16.0),
                theme.CONTRAST,
            );
            let name_width = painter
                .layout_no_wrap(name.clone(), fonts::label(16.0), theme.CONTRAST)
                .size()
                .x;
            let badge = Rect::from_min_size(
                egui::pos2(rect.left() + 32.0 + name_width + 10.0, top + 3.0),
                egui::vec2(22.0, 18.0),
            );
            painter.rect_filled(badge, 9.0, theme.BUTTON_BG);
            painter.text(
                badge.center(),
                Align2::CENTER_CENTER,
                format!("{}", group_counts[group]),
                fonts::body(12.0),
                theme.SECONDARY,
            );
            painter.rect_filled(
                Rect::from_min_size(egui::pos2(rect.left(), top + 26.0), egui::vec2(rect.width(), 1.0)),
                0,
                theme.SURFACE_4,
            );
        }
    }
    // Rule 1 of the drive: the rect comes from the program, never off a capture. Off unless
    // `SIFT_HOME_PROBE` is set, and it prints the numbers a press is aimed with.
    if std::env::var_os("SIFT_HOME_PROBE").is_some() {
        eprintln!(
            "home page={rect:?} rows={} field={field:?}",
            shown.len().min(ROWS_SHOWN)
        );
        for (index, instance) in shown.iter().take(ROWS_SHOWN).enumerate() {
            let (card, play, kebab) = row_targets_at(
                rect,
                if grouped { index_in_group[index] } else { index },
                if grouped { group_shift[group_of[index]] } else { 0.0 },
            );
            eprintln!(
                "home row={index} id={:?} card={card:?} play={play:?} kebab={kebab:?}",
                instance.id
            );
        }
    }
    for (index, instance) in shown.iter().copied().take(ROWS_SHOWN).enumerate() {
        // Grouped pages restart the grid inside each group and push it down by the headers above it.
        // **A group's cards start below its own header**, not at it: with the shift alone the first group's cards
        // began exactly where its heading was drawn, and the card painted over the heading.
        let grid_shift = if grouped { group_shift[group_of[index]] + GROUP_HEADER_H } else { 0.0 };
        // A fully folded group draws its header and nothing under it; a folding one is clipped to the height
        // it has left, so its cards slide out of view rather than vanishing at a midpoint.
        if grouped && group_folded[group_of[index]] {
            continue;
        }
        // The card, its pill and its kebab from the one function the probe and the tests read.
        let (card, play, kebab) = row_targets_at(
            rect,
            if grouped { index_in_group[index] } else { index },
            grid_shift,
        );
        // The clipped painter has to outlive the reference, so it is bound before the shadow.
        let clipped;
        let painter: &egui::Painter = if grouped {
            let group = group_of[index];
            let band = rect.top() + ROW_TOP + group_shift[group] + GROUP_HEADER_H;
            clipped = painter.with_clip_rect(Rect::from_min_max(
                egui::pos2(rect.left(), band),
                egui::pos2(rect.right(), band + group_visible[group].max(0.0)),
            ));
            &clipped
        } else {
            painter
        };
        let is_selected = selected_ids.contains(&instance.id);
        // A selected card's plate lifts one step, which is what the reference's capture shows while the ring is ticked.
        painter.rect_filled(
            card,
            12,
            if is_selected { theme.SURFACE_4 } else { theme.RAISED_BG },
        );
        // The reference's card: the pack's icon fills the top of the plate at the card's own radius, with the name
        // and the loader+version under it. This drew a 64-point tile with the rows' text beside it.
        // **132 at (13,15), measured off the reference's own card.** The tile was 152 at (8,8): the icon was twenty points
        // too large and eight off in both axes, which is why the default avatar above looked right but sat wide.
        // **36 down the card, not 15.** With the reference's icon spanning y 365..497 inside a card that starts at 329, the
        // tile sits 36 points below the card's top edge and 13 in from its left.
        let tile = Rect::from_min_size(card.min + egui::vec2(13.0, 36.0), egui::vec2(132.0, 132.0));
        painter.rect_filled(tile, 12, theme.BUTTON_BG);
        // **The instance's own icon.** This drew an empty plate and threw the cache away (`let _ = icons_cache`);
        // Modrinth's library shows each instance's icon, so a row without one is a library that cannot be told
        // apart at a glance. `icon.png` at the instance's root is the launcher's own convention; the folder's
        // `minecraft/icon.png` is where a pack that set its own icon from inside the game leaves it.
        let root = crate::data::instances_dir().join(&instance.id);
        let icon_path = [root.join("icon.png"), root.join("minecraft").join("icon.png")]
            .into_iter()
            .find(|candidate| candidate.is_file());
        let drawn_icon = icon_path.and_then(|path| icons_cache.get_file(ui.ctx(), &path));
        match drawn_icon {
            Some(texture) => {
                let art = tile.shrink(2.0);
                painter.image(
                    texture.id(),
                    art,
                    Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            }
            // No icon: the reference's own default avatar, the twelve-segment cube from `Avatar.vue:46`, drawn
            // scaled into this tile.
            None => {
                // **The reference's default avatar, not a wireframe cube.** Modrinth draws its campfire on the brand plate
                // for an instance with no icon of its own; the grey cube this build invented for the same case was 17 424
                // pixels of a bright, recognisable element being wrong.
                if !crate::serverart::paint_instance_default(&painter, tile) {
                let scale = tile.width() / 104.0;
                let origin = tile.min;
                // The cube is a path, not six independent chords: consecutive points joined trace all eleven
                // segments of the wireframe. Pairing them two at a time drew a broken glyph.
                let cube = crate::pages::instance::CUBE;
                for step in 0..cube.len() - 1 {
                    let (ax, ay) = cube[step];
                    let (bx, by) = cube[step + 1];
                    painter.line_segment(
                        [
                            egui::pos2(origin.x + ax * scale, origin.y + ay * scale),
                            egui::pos2(origin.x + bx * scale, origin.y + by * scale),
                        ],
                        egui::Stroke::new(5.0 * scale, egui::Color32::from_rgb(0x9a, 0x9a, 0x9a)),
                    );
                }
                }
            }
        }

        painter.text(
            egui::pos2(card.left() + 10.0, tile.bottom() + 18.0),
            Align2::LEFT_TOP,
            &instance.name,
            fonts::semibold(15.0),
            theme.CONTRAST,
        );
        let loader = if instance.version.is_empty() {
            instance.loader.clone()
        } else {
            format!("{} {}", instance.loader, instance.version)
        };
        painter.text(
            // **38, not 26: the meta line overlapped the title.** The name is 15-point semibold at `tile.bottom() + 18`, and this
            // was drawn eight points below its top -- inside the name's own line box, so the two strings sat on top of each other.
            // At 1:1 it read as slightly heavy text; at 4x it was unmistakable, and a page count of 28 521 could not see it.
            egui::pos2(card.left() + 10.0, tile.bottom() + 38.0),
            Align2::LEFT_TOP,
            loader,
            fonts::body(13.0),
            theme.SECONDARY,
        );

        // The card's own press opens the instance's page, which the reference does from the same
        // place (`InstanceItem.vue`). Registered before the play pill and the kebab so those two,
        // being later, win the press inside their own rects. It reports `Action::Open`, not
        // `Action::Edit`: the card and the menu row are both over the dialog's plate, and sharing
        // one verb there is what let a press on Save reopen the dialog it meant to save.
        let card_response = ui.interact(card, egui::Id::new(("card", &instance.id)), Sense::click());
        // **A right press on the card opens the card\'s own menu**, at the pointer (`instance-group/index.vue:645-648`:
        // `@contextmenu.prevent.stop` with `placement bottom-start, distance 0`). It used to open the *page\'s*
        // "+ New instance" menu, which is the wrong element: the card has its own verb list.
        // **A right press on a card opens the card\'s own menu at the pointer** (`instance-group/index.vue:645-648`).
        // The first two attempts failed for one reason: `menu_popover` returns early unless `menu.open` has been
        // called, so setting the anchor alone drew nothing. The anchor is a zero-size rect at the pointer, which the
        // popover places above-right of -- the reference places it below-left (`bottom-start`), recorded as a
        // difference rather than fixed here.
        if ui.input(|input| input.pointer.secondary_clicked()) {
            if let Some(at) = ui.ctx().pointer_interact_pos() {
                if card.contains(at) {
                    menu.open(&instance.id);
                    menu.opened_at = Some(at);
                    anchored = Some((instance.id.clone(), Rect::from_min_size(at, Vec2::ZERO)));
                    ui.ctx().request_repaint();
                }
            }
        }
        if card_response.clicked() && matches!(action, Action::None) {
            // **A press with a selection live toggles that card instead of opening it**, and so does a press with
            // Shift, Ctrl or Cmd held (`instance-card.vue:148-154`, `modified-selection.ts:11-19`). Opening an
            // instance while three cards are ticked throws the ticks away; the reference does not, and neither does
            // this now.
            let extend = ui.input(|input| {
                input.modifiers.shift || input.modifiers.command || input.modifiers.ctrl
            });
            if extend || !selected_ids.is_empty() {
                toggled = Some(instance.id.clone());
            } else {
                menu.close();
                action = Action::Open(instance.id.clone());
            }
        }

        // **The pill is a hover state.** The reference's card at rest carries its art, its name and its loader and
        // nothing else; this drew a green pill at the card's foot always, where it sat over the name. It appears under the
        // pointer, which is also when it is a target.
        if ui.rect_contains_pointer(card) {
            painter.circle_filled(play.center(), 21.0, theme.BRAND);
            icons::draw(
                painter,
                Glyph::Play,
                Rect::from_center_size(play.center(), Vec2::splat(18.0)),
                theme.ACCENT_CONTRAST,
            );
        }
        // The pill is a target now: a launcher whose play button is a drawing is not a
        // launcher. The card's own press still does nothing -- the reference opens the
        // instance's page there, and that page does not exist yet.
        if ui
            .interact(play, egui::Id::new(("play", &instance.id)), Sense::click())
            .clicked()
            && matches!(action, Action::None)
        {
            // A launch press closes whatever menu was open: leaving a panel over the list
            // while the game starts would be the last thing between the reader and the row.
            menu.close();
            action = Action::Play(instance.id.clone());
        }

        // The kebab, beside the pill and inside the same right-hand group
        // (`InstanceItem.vue:283-314`). `EllipsisVertical` is the icon set's
        // `ellipsis-vertical.svg`, the three dots the reference's `MoreVerticalIcon` draws.
        let is_open = menu.open_for.as_deref() == Some(instance.id.as_str());
        // **The selection ring.** `InstanceItem.vue` puts a 22-point circle on the art's top-right: an empty ring while the
        // card is hovered, a white disc with a brand check once it is selected -- and the check stays when the pointer
        // leaves, which is how a reader knows what a bulk action would act on.
        {
            let centre = egui::pos2(card.left() + 122.0, card.top() + 59.0);
            // **24 inside 50**, which is what the reference uses: `instance-card.vue:363-388` draws a 24-point circle in a
            // `size-[50px]` hit area. This build drew 22 in 22, so the target was a fifth of the size and the circle a
            // point small. The hit rect is the 50; the circle is drawn on the 24.
            let hit = Rect::from_center_size(centre, Vec2::splat(50.0));
            let ring_hit = ui.interact(hit, egui::Id::new(("select", &instance.id)), Sense::click());
            if is_selected {
                painter.circle_filled(centre, 12.0, theme.CONTRAST);
                crate::icons::draw(
                    painter,
                    Glyph::Check,
                    Rect::from_center_size(centre, Vec2::splat(16.0)),
                    theme.BRAND,
                );
            } else if ui.rect_contains_pointer(card) {
                // The empty ring follows the **card's** hover, not the ring's: the reference shows it as soon as the pointer
                // is anywhere on the card, which is what tells a reader the card can be selected before they aim at it.
                painter.circle_stroke(centre, 11.0, Stroke::new(2.0, theme.CONTRAST));
            }
            if ring_hit.clicked() {
                toggled = Some(instance.id.clone());
            }
        }
        let kebab_response = ui.interact(
            kebab,
            egui::Id::new(("row-menu", &instance.id)),
            Sense::click(),
        );
        // A quiet button: no plate until the pointer is on it or its menu is open
        // (`type="quiet"`, `InstanceItem.vue:309`).
        if kebab_response.hovered() || is_open {
            painter.rect_filled(kebab, KEBAB_RADIUS, theme.BUTTON_BG);
        }
        // **The whole button is the hover state, not just its plate.** The reference's card at rest shows no overflow
        // glyph at all; drawing one in `SECONDARY` left a quiet mark on every card that the capture does not have.
        if kebab_response.hovered() || is_open {
            icons::draw(
                painter,
                Glyph::EllipsisVertical,
                Rect::from_center_size(kebab.center(), Vec2::splat(KEBAB_GLYPH)),
                theme.CONTRAST,
            );
        }
        if kebab_response.clicked() {
            // A second press on the same kebab closes its own popover, which is what the
            // reference's trigger does (`FloatingPanel.vue:184-190`).
            if is_open {
                menu.close();
            } else {
                menu.open(&instance.id);
            }
        }
        if menu.open_for.as_deref() == Some(instance.id.as_str()) {
            // A menu opened by a right press stays where the press was; one opened by the kebab sits at the kebab.
            let anchor = match menu.opened_at {
                Some(at) => Rect::from_min_size(at, Vec2::ZERO),
                None => kebab,
            };
            anchored = Some((instance.id.clone(), anchor));
        }
    }
    // Where the rows stopped: the last card's own bottom plus the 12-point row gap, which is
    // where the worlds section starts.
    let mut y = rect.top() + ROW_TOP + rows as f32 * ROW_STEP;

    // The worlds, under the rows: the reference's `Jump in` section. The heading is only
    // drawn when something is under it -- a section heading over nothing is what a fresh
    // instance would show on every page load.
    let has_worlds = worlds.iter().any(|(_, names)| !names.is_empty());
    if has_worlds {
        y += WORLD_SECTION_GAP;
        painter.text(
            egui::pos2(rect.left(), y),
            Align2::LEFT_TOP,
            // `app.home.jump-back-in.title` -- "Jump in" (`locales/en-US/index.json:482-484`,
            // the message `RecentWorldsList.vue:47` formats).
            "Jump in",
            fonts::semibold(WORLD_HEADING_SIZE),
            theme.CONTRAST,
        );
        y += WORLD_HEADING_LINE;
        for (id, names) in worlds.iter() {
            for (folder, name) in names.iter().take(WORLDS_PER_INSTANCE) {
                let row = Rect::from_min_size(
                    egui::pos2(rect.left(), y),
                    egui::vec2(width, WORLD_ROW_HEIGHT),
                );
                painter.rect_filled(row, WORLD_ROW_RADIUS, theme.RAISED_BG);
                // `world.svg`, not a map pin: the icon table has no `map.svg`, and the globe
                // is the reference's own mark for a world.
                icons::draw(
                    painter,
                    Glyph::World,
                    Rect::from_center_size(
                        egui::pos2(row.left() + WORLD_GLYPH_INSET, row.center().y),
                        Vec2::splat(WORLD_GLYPH),
                    ),
                    theme.SECONDARY,
                );
                // The world's own name from `level.dat`, and the folder when it has none --
                // which is the same fallback the game uses.
                let label = clean_world_name(name);
                painter.text(
                    egui::pos2(row.left() + WORLD_TEXT_INSET, row.center().y),
                    Align2::LEFT_CENTER,
                    &label,
                    fonts::body(WORLD_TEXT_SIZE),
                    theme.BASE,
                );
                if ui
                    .interact(row, egui::Id::new(("world", id, folder)), Sense::click())
                    .clicked()
                    && matches!(action, Action::None)
                {
                    // A world row's press reports back to the shell, which launches that one
                    // world. Any press on the page closes the popover, this one included.
                    menu.close();
                    action = Action::OpenWorld {
                        instance: id.clone(),
                        // The folder, not the label: quick-play names a world by the folder
                        // it lives in, and a label the game cannot resolve reads as a launch
                        // that did nothing.
                        world: folder.clone(),
                    };
                }
                y += WORLD_ROW_HEIGHT + WORLD_ROW_GAP;
            }
        }
    }

    // The popover last, so it lies over every row it covers. egui resolves an overlapping
    // press to the widget registered last, which is what makes a menu row take the click
    // instead of the card's pill underneath it.
    let mut pressed: Option<(String, MenuItem)> = None;
    // **The ring's press, applied once the loop is done.** The rows read a copy of the set; the write lands here, and the
    // frame after this one draws the check.
    if let Some(id) = toggled {
        if !selected.insert(id.clone()) {
            selected.remove(&id);
        }
        ui.ctx().request_repaint();
    }
    // **The strip's press, applied with the ring's.** `Action::Open` is what the library card reports, so a press on the row
    // and a press on the card do the same thing -- which is what the reference does.
    if let Some(id) = jump_in {
        action = Action::Open(id);
    }
    if let Some((id, kebab)) = anchored {
        let favorite = state.favorites.iter().any(|favorite| favorite == &id);
        if let Some(item) = menu_popover(ui, theme, &id, favorite, kebab, rect, menu) {
            menu.close();
            pressed = Some((id, item));
        }
    }
    // `menu` and `worlds` are done with: everything below writes `state`, and a dialog's answer
    // is the only press allowed after this point.
    if let Some((id, item)) = pressed {
        if matches!(action, Action::None) {
            action = press_menu_row(item, &id, state);
        }
    }

    if modal_was_open {
        action = Action::None;
    }
    // **The selection bar.** Selecting anything in the reference puts a bar at the foot of the content column:
    // "N selected | Clear" on the left, "New group | Delete" on the right (`ContentSelectionBar.vue`). Measured from
    // its own capture (`r1.png`): 64 points tall, the content\'s width less 32, fill (39,41,46).
    if !selected_ids.is_empty() {
        let bar = Rect::from_min_size(
            egui::pos2(rect.left() + 16.0, rect.bottom() - 80.0),
            egui::vec2(rect.width() - 32.0, 64.0),
        );
        let painter = ui.painter();
        painter.rect_filled(bar, 14.0, theme.SURFACE_3);
        let count = format!("{} selected", state.selected.len());
        painter.text(
            egui::pos2(bar.left() + 24.0, bar.center().y),
            Align2::LEFT_CENTER,
            count.clone(),
            fonts::label(16.0),
            theme.CONTRAST,
        );
        let count_width = painter
            .layout_no_wrap(count, fonts::label(16.0), theme.CONTRAST)
            .size()
            .x;
        let rule = bar.left() + 24.0 + count_width + 16.0;
        painter.rect_filled(
            Rect::from_min_size(egui::pos2(rule, bar.center().y - 12.0), egui::vec2(1.0, 24.0)),
            0,
            theme.SURFACE_5,
        );
        let clear = Rect::from_min_size(
            egui::pos2(rule + 16.0, bar.center().y - 16.0),
            egui::vec2(64.0, 32.0),
        );
        let clear_over = ui.rect_contains_pointer(clear);
        painter.text(
            egui::pos2(clear.left(), bar.center().y),
            Align2::LEFT_CENTER,
            "Clear",
            fonts::label(16.0),
            if clear_over { theme.CONTRAST } else { theme.SECONDARY },
        );
        if clear_over && ui.input(|input| input.pointer.primary_clicked()) {
            state.selected.clear();
            ui.ctx().request_repaint();
        }
        // **Delete acts on a single selection only**: a bulk delete has no confirmation flow in this build, and
        // removing several folders behind one press is not a thing to add without one. `New group` is drawn and
        // not wired -- the group modal does not exist here.
        painter.text(
            egui::pos2(bar.right() - 24.0, bar.center().y),
            Align2::RIGHT_CENTER,
            "Delete",
            fonts::label(16.0),
            theme.RED,
        );
        // **The glyph goes left of its label, by the label\'s own width.** Placing both by eye drew them on top of each
        // other; `layout_no_wrap` knows how wide "Delete" is.
        let delete_w = painter
            .layout_no_wrap("Delete".to_owned(), fonts::label(16.0), theme.RED)
            .size()
            .x;
        crate::icons::draw(
            ui.painter(),
            Glyph::Trash,
            Rect::from_center_size(
                egui::pos2(bar.right() - 24.0 - delete_w - 16.0, bar.center().y),
                Vec2::splat(16.0),
            ),
            theme.RED,
        );
        painter.text(
            egui::pos2(bar.right() - 120.0, bar.center().y),
            Align2::RIGHT_CENTER,
            "New group",
            fonts::label(16.0),
            theme.CONTRAST,
        );
        let new_w = painter
            .layout_no_wrap("New group".to_owned(), fonts::label(16.0), theme.CONTRAST)
            .size()
            .x;
        crate::icons::draw(
            ui.painter(),
            Glyph::BoxPlus,
            Rect::from_center_size(
                egui::pos2(bar.right() - 120.0 - new_w - 16.0, bar.center().y),
                Vec2::splat(16.0),
            ),
            theme.CONTRAST,
        );
        if state.selected.len() == 1 {
            let delete = Rect::from_min_size(
                egui::pos2(bar.right() - 96.0, bar.center().y - 16.0),
                egui::vec2(72.0, 32.0),
            );
            if ui.rect_contains_pointer(delete) && ui.input(|input| input.pointer.primary_clicked()) {
                if let Some(id) = state.selected.iter().next() {
                    state.confirm_delete = Some(id.clone());
                    ui.ctx().request_repaint();
                }
            }
        }
    }
    // **The two filter-row menus, drawn here so the cards cannot paint over them.** They were drawn beside
    // their chips first and the first card covered them: the clicks worked and the menus were there, underneath.
    let pointer_pos = ui.ctx().pointer_interact_pos();
    if state.group_open {
        const GROUPS: [&'static str; 5] = [
            "Custom group",
            "Instance type",
            "Loader",
            "Game version",
            "No grouping",
        ];
        let primary_clicked = ui.input(|input| input.pointer.primary_clicked());
        let escape = ui.input(|input| input.key_pressed(egui::Key::Escape));
        let row_h = 48.0;
        let menu = Rect::from_min_size(
            egui::pos2(filters.left() + 126.0, filters.top() + 34.0),
            egui::vec2(187.0, row_h * GROUPS.len() as f32 + 12.0),
        );
        // The panel claims its rectangle, or the press that chooses a row falls through to the card underneath.
        let _ = ui.interact(menu, egui::Id::new("library-group-menu"), Sense::click());
        let painter = ui.painter();
        painter.rect_filled(menu, 8.0, theme.SUPER_RAISED_BG);
        let chosen = state.group_label.unwrap_or("Custom group");
        let mut picked: Option<&'static str> = None;
        for (index, label) in GROUPS.iter().enumerate() {
            let row = Rect::from_min_size(
                egui::pos2(menu.left() + 6.0, menu.top() + 6.0 + index as f32 * row_h),
                egui::vec2(menu.width() - 12.0, row_h),
            );
            let current = *label == chosen;
            let hovered = pointer_pos.is_some_and(|p| row.contains(p));
            if current {
                painter.rect_filled(row, 6.0, theme.BUTTON_BG_SELECTED);
            } else if hovered {
                painter.rect_filled(row, 6.0, theme.RAISED_BG);
            }
            painter.text(
                egui::pos2(row.left() + 16.0, row.center().y),
                Align2::LEFT_CENTER,
                *label,
                fonts::label(16.0),
                if current { theme.BRAND } else { theme.CONTRAST },
            );
            if hovered && primary_clicked {
                picked = Some(label);
            }
        }
        if let Some(label) = picked {
            state.group_label = Some(label);
            state.group_open = false;
            ui.ctx().request_repaint();
        } else if primary_clicked
            && !pointer_pos.is_some_and(|p| {
                menu.contains(p)
                    || Rect::from_min_size(
                        egui::pos2(filters.left() + 126.0, filters.top()),
                        egui::vec2(164.0, 32.0),
                    )
                    .contains(p)
            })
        {
            state.group_open = false;
        }
        if escape {
            state.group_open = false;
        }
    }
    if state.sort_open {
        // Read the frame before borrowing a painter: `ui.input` after `ui.painter()` is a second borrow of `ui`.
        let primary_clicked = ui.input(|input| input.pointer.primary_clicked());
        let escape = ui.input(|input| input.key_pressed(egui::Key::Escape));
        const SORTS: [&'static str; 7] = [
            "Name",
            "Last played",
            "Hours played",
            "Date created",
            "Date modified",
            "Loader",
            "Game version",
        ];
        let row_h = 48.0;
        let menu = Rect::from_min_size(
            egui::pos2(filters.left(), filters.top() + 34.0),
            egui::vec2(165.0, row_h * SORTS.len() as f32 + 12.0),
        );
        // **The panel claims its own rectangle**, or the press that chooses a row falls through to the card
        // underneath it: choosing "Name" opened the instance instead of renaming the sort.
        let _ = ui.interact(menu, egui::Id::new("library-sort-menu"), Sense::click());
        // A painter of this block's own: reusing the one bound near the top of `list` would hold a borrow of `ui`
        // across the popover call above it.
        let painter = ui.painter();
        painter.rect_filled(menu, 8.0, theme.SUPER_RAISED_BG);
        let pointer = pointer_pos;
        let mut chosen: Option<&'static str> = None;
        for (index, label) in SORTS.iter().enumerate() {
            let row = Rect::from_min_size(
                egui::pos2(menu.left() + 6.0, menu.top() + 6.0 + index as f32 * row_h),
                egui::vec2(menu.width() - 12.0, row_h),
            );
            let current = *label == state.sort_label.unwrap_or("Last played");
            if current {
                painter.rect_filled(row, 6.0, theme.BUTTON_BG_SELECTED);
            } else if pointer.is_some_and(|p| row.contains(p)) {
                painter.rect_filled(row, 6.0, theme.RAISED_BG);
            }
            painter.text(
                egui::pos2(row.left() + 16.0, row.center().y),
                Align2::LEFT_CENTER,
                *label,
                fonts::label(16.0),
                if current { theme.BRAND } else { theme.CONTRAST },
            );
            if pointer.is_some_and(|p| row.contains(p)) && primary_clicked {
                chosen = Some(label);
            }
        }
        if let Some(label) = chosen {
            state.sort_label = Some(label);
            state.sort_open = false;
            ui.ctx().request_repaint();
        } else if primary_clicked
            && !pointer.is_some_and(|p| {
                // The press that opened it is on the chip, which is outside the panel: without this the same
                // click opens the menu and closes it again in the same frame, and nothing is ever drawn.
                menu.contains(p)
                    || Rect::from_min_size(
                        egui::pos2(filters.left(), filters.top()),
                        egui::vec2(118.0, 32.0),
                    )
                    .contains(p)
            })
        {
            state.sort_open = false;
        }
        if escape {
            state.sort_open = false;
        }
    }
    // **The page's own menu**, over the cards and under the modals: one row, anchored at the press and kept
    // inside the page. The reference's is 178 wide and 58 tall for its single row, and pressing it opens the
    // same creation dialog the toolbar's button does.
    if let Some(at) = state.page_menu {
        let width = 178.0;
        let height = 58.0;
        let min = egui::pos2(
            at.x.min(rect.right() - width - 4.0).max(rect.left() + 4.0),
            at.y.min(rect.bottom() - height - 4.0).max(rect.top() + 4.0),
        );
        let menu_rect = Rect::from_min_size(min, egui::vec2(width, height));
        let painter = ui.painter();
        painter.rect_filled(menu_rect, 8.0, theme.SUPER_RAISED_BG);
        let row = Rect::from_min_size(
            egui::pos2(menu_rect.left() + 6.0, menu_rect.top() + 9.0),
            egui::vec2(width - 12.0, 40.0),
        );
        let hovered = ui.ctx().pointer_interact_pos().is_some_and(|p| row.contains(p));
        if hovered {
            painter.rect_filled(row, 6.0, theme.RAISED_BG);
        }
        crate::icons::draw(
            painter,
            crate::icons::Glyph::Plus,
            Rect::from_center_size(egui::pos2(row.left() + 18.0, row.center().y), Vec2::splat(16.0)),
            theme.SECONDARY,
        );
        painter.text(
            egui::pos2(row.left() + 38.0, row.center().y),
            Align2::LEFT_CENTER,
            "New instance",
            fonts::label(16.0),
            theme.CONTRAST,
        );
        if ui.input(|input| input.pointer.primary_clicked()) {
            if hovered {
                state.open_create = true;
            }
            state.page_menu = None;
        }
    }
    // **The `+ New instance` flag becomes the action here.** The toolbar and the page menu both set
    // `state.open_create`, and nothing read it: the toolbar's own button was dead for the same reason the page
    // menu was, which is how this was found. The action is the only way out of this page to the creation dialog
    // (`main.rs` promotes `Action::Create` into `create_open`).
    if state.open_create {
        state.open_create = false;
        if matches!(action, Action::None) {
            action = Action::Create;
        }
    }
    // The confirmation, last of all: it paints over the chrome on its own layer, and it owns
    // the only press on this page that can take a folder with it.
    if let Some(id) = state.confirm_delete.clone() {
        let entry = instances.iter().find(|entry| entry.id == id);
        let choice = confirm_dialog(ui, theme, entry);
        if choice != ConfirmChoice::Ask {
            action = resolve_confirm(choice, &id, state);
            // The dialog is gone: one frame so the page underneath is not left half-covered.
            ui.ctx().request_repaint();
        } else {
            // The rows under the plate must not act on the press that opened it.
            action = Action::None;
        }
    }

    action
}

/// Draw the open row's popover, and say which row of it was pressed.
///
/// Anchored to the kebab's bottom-right corner -- the reference's panels open `bottom-end` at
/// a distance of 8 (`FloatingPanel.vue:22, 81-103`) -- and shaped like its menu panel:
/// `rounded-[14px] bg-surface-3 ring-1 ring-surface-5` (`button-menu.ts:21`) around a
/// `p-2` column of `min-h-10` rows (`ButtonMenuPanel.vue:70`, `button-menu.ts:15`). The
/// delete row is the reference's red one (`tone: 'red'`, `use-library.ts:1371`).
/// **Where a menu's panel goes.** Two anchors, two placements:
///
/// - a menu the **kebab** opened hangs below-right of the kebab, so its right edge is the anchor's;
/// - a menu a **right press** opened is `bottom-start` at the pointer, as the reference's card context menu is
///   (`ContextMenu.vue:32-33`, placement `bottom-start`, distance 0). Its left edge is the pointer's, clamped
///   into the page -- aligning to `anchor.right() - width` put a zero-size anchor's menu off the window's left edge.
///
/// `top` is decided by the caller: it depends on the panel's height and the space below the anchor.
fn menu_placement(anchor: Rect, bounds: Rect, size: Vec2, pointer_opened: bool) -> Rect {
    // Below the anchor when it fits, above it when it does not, and never above the page.
    let below = anchor.bottom() + MENU_DISTANCE;
    let top = if below + size.y <= bounds.bottom() {
        below
    } else {
        (anchor.top() - MENU_DISTANCE - size.y).max(bounds.top())
    };
    let left = if pointer_opened {
        anchor.left().min(bounds.right() - size.x).max(bounds.left())
    } else {
        anchor.right() - size.x
    };
    Rect::from_min_size(egui::pos2(left, top), size)
}
fn menu_popover(
    ui: &mut Ui,
    theme: &Theme,
    id: &str,
    favorite: bool,
    anchor: Rect,
    bounds: Rect,
    menu: &mut Menu,
) -> Option<MenuItem> {
    // The keyboard, read off the frame's own events and in their order: arrows, Home, End, Tab,
    // Escape, Enter and the type-ahead, which are the reference's (`button-menu.ts:102-131, 29,
    // 149-177`). A menu that can only be driven by a pointer leaves its rows unreachable from
    // the keyboard the kebab that opened it was pressed with.
    let now = ui.input(|input| input.time);
    let mut keys: Vec<MenuKey> = Vec::new();
    for event in ui.input(|input| input.events.clone()) {
        match event {
            egui::Event::Key { key, pressed: true, repeat: false, .. } => match key {
                egui::Key::ArrowDown => keys.push(MenuKey::Down),
                egui::Key::ArrowUp => keys.push(MenuKey::Up),
                egui::Key::Home => keys.push(MenuKey::Home),
                egui::Key::End => keys.push(MenuKey::End),
                egui::Key::Escape => keys.push(MenuKey::Escape),
                egui::Key::Tab => keys.push(MenuKey::Tab),
                egui::Key::Enter => keys.push(MenuKey::Enter),
                _ => {}
            },
            // A printable character is the type-ahead, one character at a time.
            egui::Event::Text(text) => {
                if let Some(typed) = text.chars().next() {
                    keys.push(MenuKey::Char(typed));
                }
            }
            _ => {}
        }
    }
    let mut chosen = None;
    for key in keys {
        if let Some(item) = menu_press(key, menu, now) {
            chosen = Some(item);
        }
        // Escape and Tab close it: the row list must stop drawing this frame, not the next.
        if menu.open_for.is_none() {
            return chosen;
        }
    }

    let painter = ui.painter();

    let painter = ui.painter();
    // The panel's width is its widest row: `w-fit`, with the floor in `MENU_MIN_WIDTH`.
    let mut width = MENU_MIN_WIDTH;
    for item in MenuItem::ALL {
        let label = painter.layout_no_wrap(
            item.label_with(favorite).to_owned(),
            fonts::semibold(crate::theme::FONT_SIZE_NM),
            theme.CONTRAST,
        );
        width = width.max(
            label.size().x + MENU_ITEM_PADDING * 2.0 + MENU_ITEM_GLYPH + MENU_ITEM_GAP,
        );
    }
    let height =
        MENU_PANEL_PADDING * 2.0 + MENU_ITEM_HEIGHT * MenuItem::ALL.len() as f32;
    // Opens downward unless the panel would fall past the page's bottom edge, in which case
    // it opens upward: the flip the reference's floating panels do when there is no room
    // below the trigger (`FloatingPanel.vue:52-69`). A menu that opened off the window would
    // be a menu the reader cannot reach on the last row.
    let panel = menu_placement(anchor, bounds, egui::vec2(width, height), menu.opened_at.is_some());
    // `bg-surface-3` is `RAISED_BG`; the ring is `--surface-5`, one step above it, which is
    // `SURFACE_5` and not `DIVIDER` (`variables.scss:238-239`).
    painter.rect_filled(panel, MENU_RADIUS, theme.RAISED_BG);
    painter.rect_stroke(
        panel,
        MENU_RADIUS,
        Stroke::new(1.0, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );

    if std::env::var_os("SIFT_HOME_PROBE").is_some()
        && ui.input(|input| input.pointer.any_click())
    {
        eprintln!(
            "home click ptr={:?} panel={panel:?} rows={:?}",
            ui.input(|input| input.pointer.interact_pos()),
            MENU_ITEM_HEIGHT
        );
    }
    let mut pressed = None;
    for (index, item) in MenuItem::ALL.into_iter().enumerate() {
        let row = Rect::from_min_size(
            egui::pos2(
                panel.left() + MENU_PANEL_PADDING,
                panel.top() + MENU_PANEL_PADDING + index as f32 * MENU_ITEM_HEIGHT,
            ),
            egui::vec2(width - MENU_PANEL_PADDING * 2.0, MENU_ITEM_HEIGHT),
        );
        let response = ui.interact(
            row,
            egui::Id::new(("row-menu-item", id, index)),
            Sense::click(),
        );
        // The pointer and the keyboard share one highlight: `hover:bg-surface-4` and
        // `focus-visible`'s fill are the same token (`button-menu.ts:17-18`).
        if response.hovered() || menu.highlight == Some(index) {
            painter.rect_filled(row, MENU_ROW_RADIUS, theme.BUTTON_BG);
        }
        // `text-contrast` on every row but the destructive one (`button-menu.ts:15`).
        let ink = if item == MenuItem::Delete {
            theme.RED
        } else {
            theme.CONTRAST
        };
        icons::draw(
            painter,
            item.glyph(),
            Rect::from_center_size(
                egui::pos2(
                    row.left() + MENU_ITEM_PADDING + MENU_ITEM_GLYPH / 2.0,
                    row.center().y,
                ),
                Vec2::splat(MENU_ITEM_GLYPH),
            ),
            ink,
        );
        painter.text(
            egui::pos2(
                row.left() + MENU_ITEM_PADDING + MENU_ITEM_GLYPH + MENU_ITEM_GAP,
                row.center().y,
            ),
            Align2::LEFT_CENTER,
            item.label_with(favorite),
            fonts::semibold(crate::theme::FONT_SIZE_NM),
            ink,
        );
        if response.clicked() {
            pressed = Some(item);
        }
    }
    // `Enter` on a highlighted row is the same press a click makes.
    if pressed.is_some() {
        menu.close();
    }
    pressed.or(chosen)
}

/// Draw the confirmation the menu's `Delete` opens, and say what its press meant.
///
/// **Through `egui::Modal`**, not on the page's own painter. A modal puts its plate on a
/// foreground layer and makes every widget below it inert for as long as it is up; the page's
/// rows are registered with `ui.interact` on the panel layer, and a dialog painted there
/// would lose its presses to them -- the same reason the shell draws its edit dialog.
fn confirm_dialog(ui: &Ui, theme: &Theme, entry: Option<&Entry>) -> ConfirmChoice {
    let ctx = ui.ctx().clone();
    let body_ink = theme.CONTRAST.gamma_multiply(0.85);
    let mut choice = ConfirmChoice::Ask;

    // The one variable-height line, wrapped to the admonition's own measure: everything else
    // is a line box from the sheet, so the plate's height can be known before it is drawn.
    let wrap = CONFIRM_WIDTH
        - CONFIRM_PAD * 2.0
        - ADMONITION_PAD * 2.0
        - ADMONITION_ICON
        - ADMONITION_GAP;
    let body = ui.painter().layout(
        CONFIRM_ADMONITION_BODY.to_owned(),
        fonts::body(ADMONITION_BODY),
        body_ink,
        wrap,
    );
    let admonition_h = ADMONITION_PAD * 2.0
        + (ADMONITION_HEADER_LINE + ADMONITION_GAP + body.size().y).max(ADMONITION_ICON);
    let card_h =
        CARD_PAD * 2.0 + CARD_AVATAR.max(LABEL_LINE + 6.0 + LOADER_LINE);
    let header_h = CONFIRM_PAD * 2.0 + CONFIRM_TITLE_LINE;
    let content_h = CONFIRM_PAD * 2.0 + admonition_h + 24.0 + LABEL_LINE + 8.0 + card_h;
    let actions_h = CONFIRM_ACTIONS_PAD * 2.0 + ACTION_HEIGHT;
    let plate_h = header_h + content_h + actions_h;

    let modal = egui::Modal::new(egui::Id::new("home-confirm-delete"))
        .frame(egui::Frame::NONE)
        // `fade="danger"`: a red-tinted scrim over the page (`NewModal.vue:695-697`).
        .backdrop_color(egui::Color32::from_rgba_unmultiplied(49, 10, 15, 220))
        .show(&ctx, |ui| {
            let painter = ui.painter().clone();
            let (plate, _) = ui.allocate_exact_size(
                egui::vec2(CONFIRM_WIDTH, plate_h),
                Sense::hover(),
            );
            // `bg-bg-raised rounded-2xl border border-solid border-surface-5` (`NewModal.vue:38`).
            painter.rect_filled(plate, CONFIRM_RADIUS, theme.RAISED_BG);
            painter.rect_stroke(
                plate,
                CONFIRM_RADIUS,
                Stroke::new(1.0, theme.SURFACE_5),
                egui::StrokeKind::Inside,
            );

            // The header: the title at 24 on its own 32-point line, and the close button on
            // the right of the same row (`NewModal.vue:42-58`).
            let header_mid = plate.top() + CONFIRM_PAD + CONFIRM_TITLE_LINE / 2.0;
            painter.text(
                egui::pos2(plate.left() + CONFIRM_PAD, header_mid),
                Align2::LEFT_CENTER,
                CONFIRM_TITLE,
                fonts::semibold(CONFIRM_TITLE_SIZE),
                theme.CONTRAST,
            );
            let close = Rect::from_center_size(
                egui::pos2(
                    plate.right() - CONFIRM_PAD - CONFIRM_CLOSE / 2.0,
                    header_mid,
                ),
                Vec2::splat(CONFIRM_CLOSE),
            );
            let close_response =
                ui.interact(close, egui::Id::new("home-confirm-close"), Sense::click());
            if close_response.hovered() {
                painter.rect_filled(close, 8, theme.BUTTON_BG);
            }
            icons::draw(
                &painter,
                Glyph::X,
                Rect::from_center_size(close.center(), Vec2::splat(ACTION_ICON)),
                theme.BASE,
            );
            // The rule under the header, `border-b-[1px] border-surface-5` (`NewModal.vue:45`).
            painter.hline(
                (plate.left() + CONFIRM_RADIUS as f32)..=(plate.right() - CONFIRM_RADIUS as f32),
                plate.top() + header_h + 0.5,
                Stroke::new(1.0, theme.SURFACE_5),
            );

            // The critical admonition: `border-brand-red bg-red-bg` behind a 24-point
            // severity glyph (`Admonition.vue:172-190`; `variables.scss:117`).
            let admonition = Rect::from_min_size(
                egui::pos2(
                    plate.left() + CONFIRM_PAD,
                    plate.top() + header_h + CONFIRM_PAD,
                ),
                egui::vec2(CONFIRM_WIDTH - CONFIRM_PAD * 2.0, admonition_h),
            );
            painter.rect_filled(admonition, CONFIRM_RADIUS, theme.RED.gamma_multiply(0.10));
            painter.rect_stroke(
                admonition,
                CONFIRM_RADIUS,
                Stroke::new(1.0, theme.RED),
                egui::StrokeKind::Inside,
            );
            icons::draw(
                &painter,
                Glyph::CircleAlert,
                Rect::from_min_size(
                    egui::pos2(
                        admonition.left() + ADMONITION_PAD,
                        admonition.top() + ADMONITION_PAD,
                    ),
                    Vec2::splat(ADMONITION_ICON),
                ),
                theme.RED,
            );
            let text_x = admonition.left() + ADMONITION_PAD + ADMONITION_ICON + ADMONITION_GAP;
            painter.text(
                egui::pos2(text_x, admonition.top() + ADMONITION_PAD),
                Align2::LEFT_TOP,
                CONFIRM_ADMONITION_HEADER,
                fonts::semibold(ADMONITION_HEADER_SIZE),
                theme.CONTRAST,
            );
            painter.galley(
                egui::pos2(
                    text_x,
                    admonition.top() + ADMONITION_PAD + ADMONITION_HEADER_LINE + ADMONITION_GAP,
                ),
                body,
                body_ink,
            );

            // The instance it names: `Instance` over a card carrying the instance's own name
            // and loader (`ConfirmDeleteInstanceModal.vue:12-42`).
            let label_y = admonition.bottom() + 24.0;
            painter.text(
                egui::pos2(plate.left() + CONFIRM_PAD, label_y),
                Align2::LEFT_TOP,
                CONFIRM_INSTANCES_LABEL,
                fonts::semibold(LABEL_SIZE),
                theme.CONTRAST,
            );
            let card = Rect::from_min_size(
                egui::pos2(plate.left() + CONFIRM_PAD, label_y + LABEL_LINE + 8.0),
                egui::vec2(CONFIRM_WIDTH - CONFIRM_PAD * 2.0, card_h),
            );
            // `rounded-[20px] border border-transparent bg-surface-2 p-3`
            // (`ConfirmDeleteInstanceModal.vue:29-32`).
            painter.rect_filled(card, CARD_RADIUS, theme.SURFACE_2);
            let avatar = Rect::from_min_size(
                egui::pos2(card.left() + CARD_PAD, card.center().y - CARD_AVATAR / 2.0),
                Vec2::splat(CARD_AVATAR),
            );
            painter.rect_filled(avatar, 12, theme.BUTTON_BG);
            let name_x = avatar.right() + CARD_GAP;
            let (name, loader) = match entry {
                Some(entry) => (
                    entry.name.as_str(),
                    if entry.version.is_empty() {
                        entry.loader.clone()
                    } else {
                        format!("{} {}", entry.loader, entry.version)
                    },
                ),
                // The row can only be gone if the list was re-read between the press and this
                // frame; naming the id is still a true answer to "delete what?".
                None => ("this instance", String::new()),
            };
            let text_top = card.center().y - (LABEL_LINE + 6.0 + LOADER_LINE) / 2.0;
            painter.text(
                egui::pos2(name_x, text_top),
                Align2::LEFT_TOP,
                name,
                fonts::semibold(LABEL_SIZE),
                theme.CONTRAST,
            );
            painter.text(
                egui::pos2(name_x, text_top + LABEL_LINE + 6.0),
                Align2::LEFT_TOP,
                loader,
                fonts::body(LOADER_SIZE),
                theme.SECONDARY,
            );

            // The action row: `Cancel` beside a red `Delete instance`, right-aligned
            // (`ConfirmDeleteInstanceModal.vue:72-86`, `NewModal.vue:130`).
            let (cancel, delete) = action_buttons(&painter, plate, theme);
            if std::env::var_os("SIFT_HOME_PROBE").is_some() {
                eprintln!("home confirm plate={plate:?} cancel={cancel:?} delete={delete:?}");
            }
            for (rect, label, glyph, filled) in [
                (cancel, CONFIRM_CANCEL, Glyph::X, false),
                (delete, CONFIRM_DELETE, Glyph::Trash, true),
            ] {
                let response = ui.interact(
                    rect,
                    egui::Id::new(("home-confirm-action", label)),
                    Sense::click(),
                );
                let hovered = response.hovered();
                if filled {
                    painter.rect_filled(rect, ACTION_RADIUS, theme.RED);
                    // `--hover-brightness: 1.25` in dark (`variables.scss:407`).
                    if hovered {
                        painter.rect_filled(rect, ACTION_RADIUS, brighten(theme.RED, 1.25));
                    }
                } else if hovered {
                    painter.rect_filled(rect, ACTION_RADIUS, theme.BUTTON_BG);
                }
                // `outlined`: `box-shadow: 0 0 0 1px var(--surface-5)` (`ButtonFrame.vue:166`).
                if !filled {
                    painter.rect_stroke(
                        rect,
                        ACTION_RADIUS,
                        Stroke::new(1.0, theme.SURFACE_5),
                        egui::StrokeKind::Inside,
                    );
                }
                let ink = if filled {
                    theme.ACCENT_CONTRAST
                } else {
                    theme.CONTRAST
                };
                let galley = painter.layout_no_wrap(
                    label.to_owned(),
                    fonts::semibold(ACTION_TEXT),
                    ink,
                );
                let content_w =
                    ACTION_ICON + ACTION_GAP + galley.size().x;
                let x = rect.center().x - content_w / 2.0;
                icons::draw(
                    &painter,
                    glyph,
                    Rect::from_min_size(
                        egui::pos2(x, rect.center().y - ACTION_ICON / 2.0),
                        Vec2::splat(ACTION_ICON),
                    ),
                    ink,
                );
                painter.galley(
                    egui::pos2(x + ACTION_ICON + ACTION_GAP, rect.center().y - galley.size().y / 2.0),
                    galley,
                    ink,
                );
                if response.clicked() {
                    choice = if filled {
                        ConfirmChoice::Delete
                    } else {
                        ConfirmChoice::Cancel
                    };
                }
            }

            if close_response.clicked() {
                choice = ConfirmChoice::Cancel;
            }
        });

    if std::env::var_os("SIFT_HOME_PROBE").is_some() {
        eprintln!(
            "home confirm ptr={:?} choice={choice:?}",
            ctx.input(|input| input.pointer.interact_pos())
        );
    }

    // Click-outside and Escape close it, which is what `NewModal`'s `closeOnClickOutside` and
    // `closeOnEsc` do (:18-19, :26-35).
    if modal.backdrop_response.clicked() && choice == ConfirmChoice::Ask {
        choice = ConfirmChoice::Cancel;
    }
    if choice == ConfirmChoice::Ask && ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
        choice = ConfirmChoice::Cancel;
    }
    choice
}

/// The confirmation's two action buttons, laid out from their own labels.
///
/// Both are `md` buttons (`ButtonFrame.vue:31`): `px-2.5 gap-1.5` around a 20-point glyph and
/// a 16-point semibold label, `gap-2` apart, inset `p-4` from the plate's bottom-right.
fn action_buttons(painter: &egui::Painter, plate: Rect, theme: &Theme) -> (Rect, Rect) {
    let width = |label: &str| {
        painter
            .layout_no_wrap(
                label.to_owned(),
                fonts::semibold(ACTION_TEXT),
                theme.CONTRAST,
            )
            .size()
            .x
            + ACTION_PAD * 2.0
            + ACTION_ICON
            + ACTION_GAP
    };
    let cancel_w = width(CONFIRM_CANCEL);
    let delete_w = width(CONFIRM_DELETE);
    let bottom = plate.bottom() - CONFIRM_ACTIONS_PAD;
    let delete = Rect::from_min_size(
        egui::pos2(plate.right() - CONFIRM_ACTIONS_PAD - delete_w, bottom - ACTION_HEIGHT),
        egui::vec2(delete_w, ACTION_HEIGHT),
    );
    let cancel = Rect::from_min_size(
        egui::pos2(delete.left() - ACTION_GAP_BETWEEN - cancel_w, delete.top()),
        egui::vec2(cancel_w, ACTION_HEIGHT),
    );
    (cancel, delete)
}

/// A colour's channels scaled towards white: the reference's `brightness(1.25)` on hover
/// (`ButtonFrame.vue:20`). Clamped, because a channel past 255 is a different bug than a
/// bright button.
fn brighten(colour: Color32, factor: f32) -> Color32 {
    let scale = |channel: u8| ((channel as f32 * factor).min(255.0)) as u8;
    Color32::from_rgb(scale(colour.r()), scale(colour.g()), scale(colour.b()))
}

/// The `saves/` folder names of every drawn instance, in the order the rows are drawn.
///
/// **Folder names are not world names.** The name a player sees is the `LevelName` string
/// inside `saves/<folder>/level.dat`, which is an NBT document: parsing it is owed, and until
/// that exists the folder name is the closest true thing on disk -- with the timestamp a
/// copied world carries cut off by [`clean_world_name`]. A folder that will not open, or a
/// name that is not UTF-8, is skipped rather than reported: a Home page has no way to act on
/// either, and a row that cannot be named is worse than no row.
/// **Order the library the way the sort chip says.** `None` is the reference's default and what the loader
/// already returns: most recently played first, never-played last, ties broken by name.
///
/// Three of the reference's seven keys -- `Hours played`, `Date created`, `Date modified` -- have no data
/// behind them in `Entry`, so they fall back to that same default rather than inventing an order. A menu row
/// that silently sorted by the wrong key would be worse than one that leaves the reference's own order alone.
fn ordered(shown: &mut [&Entry], key: Option<&'static str>) {
    match key {
        Some("Name") => shown.sort_by(|a, b| {
            a.name.to_lowercase().cmp(&b.name.to_lowercase()).then_with(|| a.id.cmp(&b.id))
        }),
        Some("Loader") => shown.sort_by(|a, b| {
            a.loader
                .to_lowercase()
                .cmp(&b.loader.to_lowercase())
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }),
        Some("Game version") => shown.sort_by(|a, b| {
            a.version
                .cmp(&b.version)
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }),
        _ => shown.sort_by(|a, b| {
            b.last_played
                .cmp(&a.last_played)
                .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        }),
    }
}

#[cfg(test)]
mod order_tests {
    use super::*;

    fn entry(id: &str, name: &str, loader: &str, version: &str, last_played: i64) -> Entry {
        Entry {
            id: id.to_owned(),
            name: name.to_owned(),
            loader: loader.to_owned(),
            version: version.to_owned(),
            last_played,
            project: None,
        }
    }

    /// **The sort chip is not decoration**: two instances whose name order and last-played order
    /// disagree must be listed in opposite orders by the two keys. This is the property the drive
    /// proved on screen (`ord-last2.png` has Fabric first, `ord-name2.png` has Aardvark first).
    #[test]
    fn name_and_last_played_disagree() {
        let played = entry("a", "Fabric 26.3", "Fabric", "26.3", 1_794_000_000_000);
        let fresh = entry("b", "Aardvark", "Vanilla", "1.21", 0);
        let mut by_play = vec![&played, &fresh];
        ordered(&mut by_play, None);
        assert_eq!(by_play[0].id, "a", "the played instance leads by default");
        let mut by_name = vec![&played, &fresh];
        ordered(&mut by_name, Some("Name"));
        assert_eq!(by_name[0].id, "b", "Aardvark leads when sorted by name");
    }

    /// A key with no data behind it leaves the default order alone rather than inventing one.
    #[test]
    fn keys_without_data_keep_the_default() {
        let played = entry("a", "Zeta", "Fabric", "26.3", 1_794_000_000_000);
        let fresh = entry("b", "Alpha", "Vanilla", "1.21", 0);
        let mut shown = vec![&played, &fresh];
        ordered(&mut shown, Some("Hours played"));
        assert_eq!(shown[0].id, "a");
        let mut by_loader = vec![&played, &fresh];
        ordered(&mut by_loader, Some("Loader"));
        assert_eq!(by_loader[0].id, "a", "Fabric sorts before Vanilla");
    }
}

/// **The instances marked as favourites**, read from each instance's own `instance.cfg`. The reference keeps this as a
/// group membership (`group:favorites`); this build has no group index for it and stores the flag per instance instead.
fn read_favorites(instances: &[Entry]) -> Vec<String> {
    let root = crate::data::instances_dir();
    instances
        .iter()
        .take(ROWS_SHOWN)
        .filter(|instance| {
            let cfg = root.join(&instance.id).join("instance.cfg");
            std::fs::read_to_string(cfg)
                .map(|text| text.lines().any(|line| line.trim() == "favorite=true"))
                .unwrap_or(false)
        })
        .map(|instance| instance.id.clone())
        .collect()
}

fn read_worlds(instances: &[Entry]) -> Vec<(String, Vec<(String, String)>)> {
    let root = crate::data::instances_dir();
    instances
        .iter()
        .take(ROWS_SHOWN)
        .map(|instance| {
            // The first spelling that exists wins, so an instance with either layout reads.
            let saves = SAVES_DIRS
                .iter()
                .map(|dir| root.join(&instance.id).join(dir))
                .find(|path| path.is_dir());
            let mut names: Vec<(String, String)> = Vec::new();
            if let Some(entries) = saves.map(std::fs::read_dir).and_then(Result::ok) {
                for entry in entries.flatten() {
                    // Only folders are worlds: `saves/` also holds `level.dat` caches and
                    // whatever a mod left behind.
                    let is_folder = entry
                        .file_type()
                        .map(|kind| kind.is_dir())
                        .unwrap_or(false);
                    if is_folder {
                        if let Some(folder) = entry.file_name().to_str() {
                            // The world's own name when `level.dat` has one, and the folder
                            // when it does not: the folder is what the game would show as a
                            // fallback too, so a reader still sees something they recognise.
                            // The folder is what the game launches; the name is what the
                            // row says. Both, because they differ: a world called "Harbour
                            // Build" lives in a folder of any name at all.
                            let named = crate::nbt::world_name(&entry.path())
                                .unwrap_or_else(|| clean_world_name(folder).to_owned());
                            names.push((folder.to_owned(), named));
                        }
                    }
                }
            }
            names.sort();
            (instance.id.clone(), names)
        })
        .collect()
}

/// A world folder's name with a trailing copy timestamp removed.
///
/// Vanilla's "Copy world" and every tool that follows it append `-YYYY-MM-DD_HH-MM-SS`, so
/// `New World-2026-10-25_23-10-14` and `New World` are the same world. Only that exact
/// shape is cut: a world called `Season 2`, a folder whose suffix is a date alone, and a
/// name that would be left empty all keep their full string, because cutting more than the
/// timestamp turns one world's row into another world's name.
pub fn clean_world_name(folder: &str) -> String {
    // The suffix vanilla appends, `YYYY-MM-DD_HH-MM-SS`, is 19 bytes of ASCII plus the one
    // separator before it. The width is vanilla's, not a measurement of this tree: the
    // vendored frontend never formats one, it reads the name out of `level.dat`.
    // guess: re-measure
    const TIMESTAMP_LEN: usize = 19;

    if folder.len() < TIMESTAMP_LEN + 2 {
        return folder.to_owned();
    }
    let separator_at = folder.len() - TIMESTAMP_LEN - 1;
    // The timestamp is ASCII, so a valid suffix always starts on a character boundary. A
    // folder that would split a multi-byte character there has no timestamp to cut.
    if !folder.is_char_boundary(separator_at) {
        return folder.to_owned();
    }
    let separator = folder.as_bytes()[separator_at];
    if separator != b'-' && separator != b'_' {
        return folder.to_owned();
    }
    if !is_copy_timestamp(&folder[separator_at + 1..]) {
        return folder.to_owned();
    }
    let head = &folder[..separator_at];
    if head.is_empty() {
        return folder.to_owned();
    }
    head.to_owned()
}

/// Whether `tail` is exactly the 19-character `YYYY-MM-DD_HH-MM-SS` vanilla writes, with `d`
/// standing for a digit.
fn is_copy_timestamp(tail: &str) -> bool {
    const SHAPE: &[u8] = b"dddd-dd-dd_dd-dd-dd";
    let bytes = tail.as_bytes();
    bytes.len() == SHAPE.len()
        && bytes
            .iter()
            .zip(SHAPE)
            .all(|(byte, expected)| match *expected {
                b'd' => byte.is_ascii_digit(),
                separator => *byte == separator,
            })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The hero is the fresh-install state and the list is everything else: the dispatch is
    /// a plain `is_empty`, so this pins the decision rather than the drawing.
    /// **Escape cancels the delete confirmation.** The dialog is drawn by `confirm_dialog`, which returns its answer,
    /// so this asserts the answer rather than a flag: `Ask` with the key held must come back `Cancel`.
    #[test]
    fn escape_cancels_the_delete_confirmation() {
        let ctx = egui::Context::default();
        crate::fonts::install(&ctx);
        crate::serverart::install(&ctx);
        let theme = crate::theme::dark();
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
        let mut answer = ConfirmChoice::Delete;
        ctx.run_ui(raw, |ui| {
            answer = confirm_dialog(ui, &theme, None);
        });
        assert_eq!(answer, ConfirmChoice::Cancel, "Escape must cancel, never delete");
    }
    #[test]
    fn an_empty_library_is_the_hero() {
        let none: Vec<Entry> = Vec::new();
        assert!(none.is_empty(), "an install with no instances shows the hero");
    }

    /// A copied world's folder carries vanilla's suffix, and the row drops it: the two
    /// folders are one world, and a list that showed the timestamp twice would read as two.
    #[test]
    fn a_world_folder_loses_its_copy_timestamp() {
        assert_eq!(clean_world_name("New World-2026-10-25_23-10-14"), "New World");
        assert_eq!(clean_world_name("New World_2026-10-25_23-10-14"), "New World");
        // A timestamp after non-ASCII text still cuts at the right place: the suffix is
        // ASCII, so its start is a character boundary.
        assert_eq!(clean_world_name("Über-2026-10-25_23-10-14"), "Über");
    }

    /// Everything that is not that exact suffix keeps its whole name. A world called
    /// `Season 2` must not become `Season`, and a folder that is only a timestamp must not
    /// become an empty row.
    #[test]
    fn a_world_name_without_the_suffix_is_untouched() {
        assert_eq!(clean_world_name("Season 2"), "Season 2");
        assert_eq!(clean_world_name("survival"), "survival");
        assert_eq!(clean_world_name("world-2026-10-25"), "world-2026-10-25");
        assert_eq!(clean_world_name("world-2026-10-25_23-10"), "world-2026-10-25_23-10");
        assert_eq!(clean_world_name("2026-10-25_23-10-14"), "2026-10-25_23-10-14");
        assert_eq!(clean_world_name("-2026-10-25_23-10-14"), "-2026-10-25_23-10-14");
        assert_eq!(clean_world_name(""), "");
    }

    /// Every menu row is one verb, and that verb carries the instance the row belongs to.
    /// A row that reported `Action::None` would be the silent press this menu exists to
    /// avoid; two rows reporting the same action would make one of them a lie.
    #[test]
    fn the_menu_reports_exactly_one_action() {
        let mut seen: Vec<Action> = Vec::new();
        for item in MenuItem::ALL {
            let action = item.action("instance-3");
            assert_ne!(
                action,
                Action::None,
                "{} must report something",
                item.label()
            );
            assert!(
                names(&action, "instance-3"),
                "{} must name the row's instance",
                item.label()
            );
            assert!(
                !seen.contains(&action),
                "{} reports an action another row already does",
                item.label()
            );
            seen.push(action);
        }
        assert_eq!(seen.len(), MenuItem::ALL.len());
    }

    /// The popover's rows are the reference's own strings, each beside the key it came from:
    /// a label that drifted from `locales/en-US/index.json` is a parity bug no compiler sees.
    #[test]
    fn the_menu_labels_are_the_reference_strings() {
        let expected = [
            ("Play", "app.instance.action.play"),
            ("Add to favorites", "app.instance.action.add-to-favorites"),
            ("View instance", "app.instance.action.view-instance"),
            ("Duplicate instance", "app.library.instance.action.duplicate"),
            ("Open folder", "app.instance.action.open-folder"),
            ("Copy path", "app.instance.action.copy-path"),
            ("Edit", "app.instance.action.edit"),
            ("Delete", "app.library.instance.action.delete"),
        ];
        assert_eq!(MenuItem::ALL.len(), expected.len());
        for (item, (label, key)) in MenuItem::ALL.into_iter().zip(expected) {
            assert_eq!(item.label(), label);
            assert_eq!(item.key(), key);
        }
    }

    /// **The destructive row asks first.** The menu's `Delete` must not be able to reach
    /// `Action::Delete` on its own press: the reference stores the id and shows a confirmation
    /// (`use-library.ts:1373-1376`), and a launcher that removed a folder on one click would be
    /// one slip away from a data loss no toast can undo.
    #[test]
    fn the_delete_row_asks_before_anything_leaves_the_disk() {
        let mut state = State::default();
        let action = press_menu_row(MenuItem::Delete, "Instance 2", &mut state);
        assert_eq!(action, Action::None, "the menu's Delete only opens the question");
        assert_eq!(
            state.confirm_delete.as_deref(),
            Some("Instance 2"),
            "the question is about the row that was pressed"
        );
    }

    /// The dialog's `Delete instance` is the one press that returns the verb, and it clears the
    /// question as it does; every other row still reports its own verb without asking.
    #[test]
    fn the_confirmation_is_the_only_way_to_the_delete_action() {
        let mut state = State::default();
        state.confirm_delete = Some("Playful".to_owned());
        assert_eq!(
            resolve_confirm(ConfirmChoice::Delete, "Playful", &mut state),
            Action::Delete("Playful".to_owned()),
            "the dialog's own button is what removes the folder"
        );
        assert_eq!(state.confirm_delete, None, "and the question is answered");

        state.confirm_delete = Some("Playful".to_owned());
        assert_eq!(
            resolve_confirm(ConfirmChoice::Cancel, "Playful", &mut state),
            Action::None,
            "Cancel removes nothing"
        );
        assert_eq!(state.confirm_delete, None, "Cancel answers the question too");

        state.confirm_delete = Some("Playful".to_owned());
        assert_eq!(
            resolve_confirm(ConfirmChoice::Ask, "Playful", &mut state),
            Action::None
        );
        assert_eq!(
            state.confirm_delete.as_deref(),
            Some("Playful"),
            "a dialog that is still standing has answered nothing"
        );

        for verb in [MenuItem::Play, MenuItem::Edit, MenuItem::Duplicate, MenuItem::OpenFolder] {
            let mut state = State::default();
            assert_eq!(
                press_menu_row(verb, "Playful", &mut state),
                verb.action("Playful"),
                "{} is not destructive and does not ask",
                verb.label()
            );
            assert_eq!(state.confirm_delete, None, "{} opens no dialog", verb.label());
        }
    }

    /// The dialog's copy is the reference's own, each beside the key it came from: a rebranded
    /// or reworded string is a parity bug no compiler sees. Read from
    /// `ConfirmDeleteInstanceModal.vue:138-175` and `common-messages.ts:51-54`.
    #[test]
    fn the_confirmation_says_what_the_reference_says() {
        let expected = [
            ("Delete instance", "app.instance.confirm-delete.header"),
            (
                "This action cannot be undone",
                "app.instance.confirm-delete.admonition-header",
            ),
            (
                // The body's own line, wrapped into one string.
                "All data for your instance will be permanently deleted, including your worlds, configs, and all installed content.",
                "app.instance.confirm-delete.admonition-body",
            ),
            ("Instance", "app.instance.confirm-delete.instances-label"),
            ("Cancel", "button.cancel"),
            ("Delete instance", "app.instance.confirm-delete.delete-button"),
        ];
        let ours = [
            CONFIRM_TITLE,
            CONFIRM_ADMONITION_HEADER,
            CONFIRM_ADMONITION_BODY,
            CONFIRM_INSTANCES_LABEL,
            CONFIRM_CANCEL,
            CONFIRM_DELETE,
        ];
        assert_eq!(ours.len(), expected.len());
        for (ours, (reference, key)) in ours.into_iter().zip(expected) {
            assert_eq!(ours, reference, "{key} drifted from the reference's string");
        }
    }

    /// The row's three targets are inside each other the way a press needs them to be: the pill
    /// and the kebab sit on the card, the kebab clear of the pill, and two rows never overlap.
    /// The numbers are the ones the probe prints, so an aimed press and the drawing agree.
    #[test]
    fn a_rows_targets_are_on_the_card_and_clear_of_each_other() {
        let page = Rect::from_min_size(
            egui::pos2(88.0, 48.0),
            egui::vec2(1027.0, 828.0),
        );
        let (card, play, kebab) = row_targets(page, 0);
        assert_eq!(card.top(), 48.0 + ROW_TOP);
        // **The card, not the old row.** Driving the reference's Home page showed a grid of 168x208 cards where this
        // build drew 96-point full-width rows; the number here is the measurement, and the test follows it rather
        // than the constant it used to read.
        // **215**: re-measured at the card\'s middle column, where its rounded corner cannot interfere -- the reference\'s
        // plate runs 354..568 at x 150. At 208 this build\'s loader line ran into the plate\'s bottom edge.
        // **239**: the card starts 25 points higher than the reference's here, so it has to be 25 taller for its foot to land
        // on the reference's 568. The plate's own fill is under the threshold; its bottom border is not.
        assert_eq!(card.height(), 239.0);
        assert_eq!(card.width(), 168.0);
        assert!(card.contains_rect(play), "the pill is on the card");
        assert!(card.contains_rect(kebab), "the kebab is on the card");
        // **Diagonal, not side by side.** The play disc sits on the art's lower right (x 185..227) and the kebab at the
        // card's foot (x 222..250), so they share five columns but no rows -- which is how the reference draws them. The
        // invariant that matters is that their *areas* are disjoint, not that one is left of the other; the old assertion
        // was written for the pill this build used to draw at the card's left.
        assert!(
            !play.intersects(kebab),
            "the two targets must not overlap: {play:?} {kebab:?}"
        );
        // The next card is to its right on the same row until the columns run out, and under it after that.
        let (next, _, _) = row_targets(page, 1);
        assert!(
            card.bottom() < next.top() || card.right() < next.left(),
            "one card must not cover the next: {card:?} {next:?}"
        );
    }

    /// **The quick-create key is the reference's, guards and all.** `WelcomeScreen.vue:50-68`
    /// fires on an unmodified, non-repeat `n` and nothing else; this draws the hero with one
    /// synthetic key event and reads what the page reported, so a shortcut that grew a modifier
    /// or a repeat has to fail here rather than in a session where the reader lost a folder.
    #[test]
    fn the_quick_create_key_ignores_modifiers_repeats_and_a_held_keyboard() {
        let ctx = egui::Context::default();
        fonts::install(&ctx);
        let theme = crate::theme::dark();
        let page = Rect::from_min_size(egui::pos2(88.0, 48.0), egui::vec2(1027.0, 828.0));
        let mut images = Images::default();
        let mut hero_action = |modifiers: egui::Modifiers, repeat: bool| {
            let mut action = Action::None;
            let input = egui::RawInput {
                events: vec![egui::Event::Key {
                    key: egui::Key::N,
                    physical_key: None,
                    pressed: true,
                    repeat,
                    modifiers,
                }],
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                ..Default::default()
            };
            let _ = ctx.run_ui(input, |ui| {
                egui::CentralPanel::default().show(ui, |ui| {
                    action = hero(ui, page, &theme, &mut images);
                });
            });
            action
        };
        assert_eq!(
            hero_action(egui::Modifiers::NONE, false),
            Action::Create,
            "a plain n is the reference's shortcut"
        );
        assert_eq!(
            hero_action(egui::Modifiers::CTRL, false),
            Action::None,
            "Ctrl+N belongs to the window manager, not the page"
        );
        assert_eq!(
            hero_action(egui::Modifiers::NONE, true),
            Action::None,
            "a held key is one press, not a stream of instances"
        );
    }

    /// **The menu is reachable from the keyboard.** The reference moves focus with
    /// ArrowUp/ArrowDown (wrapping), `Home` and `End`, activates with `Enter`, closes on
    /// `Escape` and `Tab`, and jumps by type-ahead with a 500 ms reset
    /// (`button-menu.ts:102-131, 29, 149-177`). Each rule is one call here, so a row that went
    /// unreachable again would fail this rather than a drive.
    #[test]
    /// **The pointer anchor belongs to one opening.** A menu opened by a right press places itself at the pointer;
    /// one opened by the kebab places itself at the kebab. If `opened_at` survived a close, the next kebab menu would
    /// appear wherever the last right press was -- a fault that would look like rendering, not state.
    #[test]
    fn a_pointer_anchor_does_not_outlive_its_menu() {
        let mut menu = Menu::default();
        menu.open("a");
        assert_eq!(menu.opened_at, None, "the kebab\'s menu has no pointer anchor");
        menu.opened_at = Some(egui::pos2(10.0, 20.0));
        menu.close();
        assert_eq!(menu.opened_at, None, "closing forgets where it was opened");
        menu.open("b");
        assert_eq!(menu.opened_at, None, "and opening again starts with none");
    }

    /// **The placement that put a menu off the window.** A right-pressed menu is bottom-start at the pointer and
    /// must clamp into the page; the kebab's hangs off its own right edge and is not clamped the same way.
    #[test]
    fn a_pointer_menu_is_clamped_and_a_kebab_menu_hangs_right() {
        let bounds = Rect::from_min_size(egui::pos2(64.0, 48.0), egui::vec2(1076.0, 852.0));
        let size = egui::vec2(200.0, 240.0);
        // A right press at x 150 puts the panel's left edge at 150. The old code used `right() - width`, which
        // produced -50 here -- off the page, and exactly what a capture showed.
        let at = Rect::from_min_size(egui::pos2(150.0, 400.0), Vec2::ZERO);
        let panel = menu_placement(at, bounds, size, true);
        assert_eq!(panel.left(), 150.0, "the pointer's own column");
        let near = Rect::from_min_size(egui::pos2(1120.0, 400.0), Vec2::ZERO);
        let clamped = menu_placement(near, bounds, size, true);
        assert_eq!(clamped.right(), bounds.right(), "clamped to the page's right edge");
        let kebab = Rect::from_min_size(egui::pos2(222.0, 499.0), egui::vec2(28.0, 28.0));
        let hanging = menu_placement(kebab, bounds, size, false);
        // **The vertical flip**, which had no test until this round: a menu with room below hangs below, and one
        // against the floor goes above its anchor instead of off the page.
        assert_eq!(hanging.top(), kebab.bottom() + MENU_DISTANCE, "below the kebab when it fits");
        let near_floor = Rect::from_min_size(egui::pos2(300.0, 860.0), Vec2::ZERO);
        let flipped = menu_placement(near_floor, bounds, size, true);
        assert_eq!(
            flipped.top(),
            near_floor.top() - MENU_DISTANCE - size.y,
            "above the pointer when there is no room below"
        );
        assert!(flipped.top() >= bounds.top(), "and never above the page");
        assert_eq!(hanging.right(), kebab.right(), "right-aligned to the kebab");
    }

    fn the_menu_moves_and_activates_from_the_keyboard() {
        let mut menu = Menu::default();
        menu.open("Parity Test");

        // Down from a fresh menu starts at the top, and the last row wraps to the first.
        assert_eq!(menu.highlight, None, "a menu opens with no row focused");
        menu_press(MenuKey::Down, &mut menu, 0.0);
        assert_eq!(menu.highlight, Some(0));
        for _ in 0..MenuItem::ALL.len() {
            menu_press(MenuKey::Down, &mut menu, 0.0);
        }
        assert_eq!(menu.highlight, Some(0), "Down wraps past the last row");
        menu_press(MenuKey::Up, &mut menu, 0.0);
        assert_eq!(
            menu.highlight,
            Some(MenuItem::ALL.len() - 1),
            "Up wraps back to the last row"
        );
        menu_press(MenuKey::Home, &mut menu, 0.0);
        assert_eq!(menu.highlight, Some(0));
        menu_press(MenuKey::End, &mut menu, 0.0);
        assert_eq!(menu.highlight, Some(MenuItem::ALL.len() - 1));

        // Enter is the only key that reports a row.
        menu_press(MenuKey::Home, &mut menu, 0.0);
        menu_press(MenuKey::Down, &mut menu, 0.0);
        assert_eq!(
            menu_press(MenuKey::Enter, &mut menu, 0.0),
            Some(MenuItem::ALL[1]),
            "Enter activates the focused row"
        );
        let mut fresh = Menu::default();
        fresh.open("Parity Test");
        assert_eq!(
            menu_press(MenuKey::Enter, &mut fresh, 0.0),
            None,
            "Enter with nothing focused activates nothing"
        );

        // Escape and Tab close.
        for key in [MenuKey::Escape, MenuKey::Tab] {
            let mut open = Menu::default();
            open.open("Parity Test");
            menu_press(key, &mut open, 0.0);
            assert_eq!(open.open_for, None, "{key:?} closes the menu");
            assert_eq!(open.highlight, None, "{key:?} forgets the keyboard too");
        }
    }

    /// The type-ahead is a one-letter buffer with a 500 ms reset: "d" lands on the first label
    /// that starts with it, "de" extends it inside the window, and a letter after the window
    /// starts a new search instead of joining the old one.
    #[test]
    fn the_type_ahead_jumps_to_a_label_and_resets_after_half_a_second() {
        let mut menu = Menu::default();
        menu.open("Parity Test");
        menu_press(MenuKey::Char('d'), &mut menu, 10.0);
        assert_eq!(
            menu.highlight,
            Some(3),
            "'d' lands on Duplicate instance, the first label that starts with it"
        );
        menu_press(MenuKey::Char('e'), &mut menu, 10.2);
        assert_eq!(
            menu.highlight,
            Some(7),
            "'de' extends the buffer inside the window, and Delete is what starts with it"
        );
        assert_eq!(menu.typed, "de");
        menu_press(MenuKey::Char('e'), &mut menu, 11.0);
        assert_eq!(
            menu.highlight,
            Some(6),
            "a letter 800 ms later starts a new buffer, and 'e' is Edit"
        );
        assert_eq!(menu.typed, "e");
    }

    /// Whether `action` names the instance it was pressed on. Written as a match over every
    /// variant so a new `Action` cannot be added without this test saying so.
    fn names(action: &Action, id: &str) -> bool {
        match action {
            Action::Play(ours)
            | Action::Open(ours)
            | Action::Edit(ours)
            | Action::Duplicate(ours)
            | Action::OpenFolder(ours)
            | Action::CopyPath(ours)
            | Action::Delete(ours) => ours.as_str() == id,
            Action::SetFavorite(ours, _) => ours.as_str() == id,
            Action::OpenWorld { instance, .. } => instance.as_str() == id,
            Action::None | Action::Create | Action::Import => false,
        }
    }
}
