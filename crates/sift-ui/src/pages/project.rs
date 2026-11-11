
//! A project's page: the header, the tab strip, the three tabs' contents, and the compatibility
//! block its sidebar carries.
//!
//! Follows `vendor/modrinth-app/app-frontend/src/pages/project/{Index,Description,Versions,
//! Gallery}.vue` and the components they render, with the layout classes read out of each:
//!
//! * `Index.vue:48` — the page column is `flex flex-col gap-4 p-6`: a 24-point inset and a
//!   16-point gap between the header, the tab strip and the tab's card.
//! * `ui/src/components/base/page-header/index.vue:59-63` — the header carries
//!   `border-b border-divider` and `pb-4`; its row is `flex flex-wrap items-start gap-4` with a
//!   96-point avatar (`ProjectPageHeader.vue:8-15`), the title at
//!   `text-2xl font-semibold leading-none text-contrast`, the summary at body size with
//!   `max-w-[44rem] line-clamp-2`, and the metadata row at `gap-x-[1.625rem] gap-y-2`.
//! * `Index.vue:138-170` — the actions are a `Button` at `size="xl"` (48 points tall,
//!   `rounded-2xl px-3.5 text-base font-extrabold`, a 24-point glyph) and a quiet
//!   `TeleportOverflowMenu` at `size="xl"`, which is a 48-point square.
//! * `ui/src/components/base/NavTabs.vue:7-13,24` — the tab strip is
//!   `relative flex w-fit rounded-full bg-bg-raised p-1 text-sm font-bold` with a
//!   `border border-surface-4`, one `px-4 py-2` link per tab (36 points tall) and a
//!   `bg-button-bgSelected` slider that slides with an 80 ms `cubic-bezier(0.4,0,0.2,1)`.
//! * `packages/assets/styles/classes.scss:499-508` — a `Card` is `padding: 1rem`,
//!   `background: var(--surface-3)`, `border-radius: var(--radius-lg)` = 16 and a 1-point
//!   `--surface-4` border.
//! * `Versions.vue` + `ui/src/components/project/ProjectPageVersions.vue:560-615` — the
//!   versions tab is a `Table`: `overflow-hidden rounded-2xl border border-surface-4`, a
//!   `bg-surface-3` header row `h-12` tall, `h-14` body rows each with a `border-t
//!   border-surface-4`, columns channel `4.5rem`, Version, Game version, Platform, Published
//!   `12%`, Downloads `12%` and actions `1%`. The channel is a `VersionChannelIndicator`, the
//!   version number is `font-medium text-contrast`, game versions and platforms are
//!   `TagItem` chips, published is `formatRelativeTime` and downloads are compact.
//! * `Gallery.vue:91-120` — the gallery is
//!   `grid-template-columns: repeat(auto-fill, minmax(20rem, 1fr)); gap: 1rem`, each item a
//!   `Card` whose image is `aspect-ratio: 2/1; object-fit: cover`, whose body is `p-4` with
//!   the title and description, and whose foot is a `CalendarIcon` and the date.
//!
//! **What this page cannot carry out.** The install button, the kebab menu and a version row
//! all need an action the page cannot return: `draw` answers a `bool` (the frame debt) and the
//! shell owns the installer, the menus and the routes (`shell.rs:812-833`). They are drawn at
//! the reference's geometry and left inert there rather than moved into this page, where the
//! press could not reach the shell either.
//!
//! **What is modelled from the API and not from a fetch.** `Project` carries the fields the
//! live endpoints return — `GET /v2/project/{id}` for the header, the gallery and the
//! sidebar's details, and `GET /v2/project/{id}/version` for the versions table — with the
//! sample's values. Nothing here fetches; installing a project is the shell's Discover path.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::UNIX_EPOCH;

use egui::{self, Align2, Color32, Rect, Sense, Stroke, Ui, Vec2};
// The launcher's own paths and wire, so a project document comes through the same client,
// cache and retry policy every other metadata question does.
use sift_core::paths::{platform_data_root, SiftPaths, System};
use sift_game::wire::Wire;

use crate::fonts;
use crate::icons::{self, Glyph};
use crate::images::Images;
use crate::theme::{Theme, STROKE_WIDTH};

// ---------------------------------------------------------------------------
// The page's geometry
// ---------------------------------------------------------------------------

/// The page column's inset: `p-6` (`Index.vue:48`).
const PAGE_PADDING: f32 = 24.0;

/// The menu's width under the kebab: a `ContextMenu` popover, wide enough for its longest row.
const MENU_WIDTH: f32 = 200.0;

/// A menu row's height.
const MENU_ROW: f32 = 32.0;

/// `p-1` inside the menu's plate.
const MENU_PAD: f32 = 4.0;

/// The gap between the header, the tabs and the card: `gap-4` (`Index.vue:48`).
const BLOCK_GAP: f32 = 16.0;

/// The header's avatar: `size="96px"` (`ProjectPageHeader.vue:8-15`).
const AVATAR: f32 = 96.0;

/// `gap-4` between the avatar and the text column (`page-header/index.vue:3-4`).
const HEADER_GAP: f32 = 16.0;

/// The title: `text-2xl` (`page-header/index.vue:12`).
const TITLE_SIZE: f32 = 24.0;

/// `leading-none` on the title.
const TITLE_LINE: f32 = 24.0;

/// The summary's size: the page's body size (`page-header/index.vue:20-22,65`).
const SUMMARY_SIZE: f32 = 16.0;

/// The summary's line height: `leading-6`.
const SUMMARY_LINE: f32 = 24.0;

/// `max-w-[44rem]` on the summary (`page-header/index.vue:20`).
const SUMMARY_MAX: f32 = 704.0;

/// `gap-1.5` between the title and the summary (`page-header/index.vue:15`).
const SUMMARY_GAP: f32 = 6.0;

/// `line-clamp-2` (`page-header/index.vue:65`).
const SUMMARY_LINES: usize = 2;

/// `gap-x-[1.625rem]` in the metadata row (`metadata/index.vue:2-5`).
const META_GAP: f32 = 26.0;

/// The metadata item's icon: `size-5` (`page-header-metadata-item.vue:77-80`).
const META_ICON: f32 = 20.0;

/// `gap-2` inside a metadata item.
const META_CONTENT_GAP: f32 = 8.0;

/// The metadata item's text: body size at `font-medium leading-none`.
const META_TEXT: f32 = 16.0;

/// The bullet between metadata items (`page-header-metadata-item.vue:3-7`).
const META_BULLET: f32 = 4.0;

/// `pb-4` under the header's row (`page-header/index.vue:63`).
// 12, not 16: the rule under the header landed at y 190 where the reference\'s is at 186, at the identical
// tone, while the header\'s own contents were within a point of the reference\'s. The block was four points too
// tall and its foot is where the four were.
const HEADER_PAD_BOTTOM: f32 = 12.0;

/// A `TagItem`'s text: `text-sm` (`base/TagItem.vue:18`).
const TAG_TEXT: f32 = 14.0;

/// `px-2` on a `TagItem`.
const TAG_PAD_X: f32 = 8.0;

/// `py-1` on a `TagItem`, over `leading-none`.
const TAG_PAD_Y: f32 = 4.0;

/// A tag's icon: `svg 16` (`base/TagItem.vue:18`).
const TAG_GLYPH: f32 = 16.0;

/// `gap-1` between a tag's icon and its label.
const TAG_GAP: f32 = 4.0;

/// The height a `TagItem` resolves to: 14-point text on a 20-point line, 4 above and below,
/// with its 1-point border inside.
const TAG_HEIGHT: f32 = 26.0;

/// `h-12` on a `size="xl"` button (`ButtonFrame.vue:33`).
const INSTALL_HEIGHT: f32 = 48.0;

/// `rounded-2xl` on a `size="xl"` button.
const INSTALL_RADIUS: u8 = 16;

/// `px-3.5` on a `size="xl"` button: 14 points of padding. The capture shows 16 between the
/// fill's left edge (946) and the glyph's box (962); the extra two are the glyph's own box
/// meeting the padding, so the value stays the sheet's and the glyph box does the rest.
const INSTALL_PAD: f32 = 14.0;

/// `gap-2` on a `size="xl"` button.
const INSTALL_GAP: f32 = 8.0;

/// `svg size-6` on a `size="xl"` button. The install button's own glyph, measured at 20: the
/// reference's download arrow occupies x 962..982 inside a button whose fill starts at 946, so
/// the 24-point box is not what it draws. A 24-point box left the arrow 4 points wider than the
/// reference's on every capture.
const INSTALL_GLYPH: f32 = 20.0;

/// The kebab's `ellipsis-vertical`, which is a 24-point box whatever the install button's glyph
/// is; the two used to share one constant because they were both 24.
const KEBAB_GLYPH: f32 = 24.0;

/// `text-base font-extrabold` on a `size="xl"` button.
const INSTALL_TEXT: f32 = 16.0;

/// An icon-only `size="xl"` button: `w-12` (`ButtonFrame.vue:41`).
const KEBAB: f32 = 48.0;

/// `p-1` on the tab strip's track (`NavTabs.vue:8`).
const TAB_TRACK_PAD: f32 = 4.0;

/// `px-4` on a tab link (`NavTabs.vue:24`).
const TAB_PAD_X: f32 = 16.0;

/// `py-2` on a tab link.
const TAB_PAD_Y: f32 = 8.0;

/// `text-sm` on the tab strip (`NavTabs.vue:8`).
const TAB_TEXT: f32 = 14.0;

/// `text-sm`'s line height: 20 points, which is what makes a link 36 tall.
const TAB_LINE: f32 = 20.0;

/// The slider's slide: `80ms cubic-bezier(0.4, 0, 0.2, 1)` (`NavTabs.vue:260-273`).
const SLIDER_SECONDS: f64 = 0.08;

/// A card's padding: `padding: 1rem` (`classes.scss:499-508`).
const CARD_PAD: f32 = 16.0;

/// The top padding, which the sheet sets differently: pt-3.
const CARD_PAD_TOP: f32 = 12.0;

/// mb-4 under every card but the last.
const CARD_GAP: f32 = 16.0;

/// rounded-2xl on a card.
const CARD_RADIUS: u8 = 16;

/// A card's heading: text-lg font-semibold = 18 points, on a 28-point line.
const SIDEBAR_HEADING: f32 = 18.0;
const CARD_HEADING_LINE: f32 = 28.0;

/// gap-3 between a card's heading and its content.
const CARD_CONTENT_GAP: f32 = 12.0;

/// A line of the sidebar's body text: 16 points on a 24-point line.
const SIDEBAR_BODY: f32 = 16.0;
const SIDEBAR_LINE: f32 = 24.0;

/// A chip's height in the sidebar, and the gap beside it.
const SIDEBAR_CHIP_HEIGHT: f32 = 26.0;
const SIDEBAR_CHIP_GAP: f32 = 4.0;

/// A card's corner: `--radius-lg` (`classes.scss:502`).

/// A table header cell's height: `h-12` (`base/Table.vue:24-36`).
const TABLE_HEADER: f32 = 48.0;

/// A table row's height: `h-14` (56) plus its own `border-t`, which the reference renders as a
/// 61-point pitch -- its row rules run 362, 423, 484, 545, 606 (ref-project-versions.png).
const TABLE_ROW: f32 = 61.0;

/// `first:pl-4 last:pr-4` on a table cell (`base/Table.vue:24-36`).
const TABLE_PAD: f32 = 16.0;

/// `pr-2` between a cell's content and the next one (`base/Table.vue:24-36`).
const TABLE_GAP: f32 = 8.0;

/// The channel column: `width: '4.5rem'` (`ProjectPageVersions.vue:561-566`).
const CHANNEL_WIDTH: f32 = 72.0;

/// `max-w-[12rem]` on the chip columns (`ProjectPageVersions.vue:578-590`).
const CHIP_COLUMN: f32 = 192.0;

/// `width: 12%` on the Published and Downloads columns.
const DATE_COLUMN: f32 = 0.12;

/// `width: 1%` on the actions column.
const ACTIONS_COLUMN: f32 = 48.0;

/// `repeat(auto-fill, minmax(20rem, 1fr))` (`Gallery.vue:91-96`).
const GALLERY_MIN: f32 = 320.0;

/// `gap: 1rem` in the gallery grid.
const GALLERY_GAP: f32 = 16.0;

/// `aspect-ratio: 2/1` on a gallery image (`Gallery.vue:105-110`).
const GALLERY_ASPECT: f32 = 0.5;

/// `p-4` on a gallery item's body (`Gallery.vue:112-115`).
const GALLERY_BODY_PAD: f32 = 16.0;

/// A gallery item's title: `h3`, which the sheet draws at 1.125rem semibold.
const GALLERY_TITLE: f32 = 18.0;

/// A gallery item's description: body size, `text-secondary`.
const GALLERY_BODY: f32 = 16.0;

/// The gallery foot's date: `text-sm text-secondary`.
const GALLERY_DATE: f32 = 14.0;

/// The gallery foot's calendar icon.
const GALLERY_ICON: f32 = 16.0;

// ---------------------------------------------------------------------------
// The model
// ---------------------------------------------------------------------------

/// A calendar date, as `GET /v2/project/{id}` writes one in `published`/`updated` and
/// `GET /v2/project/{id}/version` writes one in `date_published`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    /// The civil year.
    pub year: i32,
    /// The month, 1-12.
    pub month: u8,
    /// The day of the month.
    pub day: u8,
}

/// The long month names `Intl.DateTimeFormat` prints for `month: 'long'`.
const MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June", "July", "August", "September",
    "October", "November", "December",
];

/// The short month names `dateStyle: 'medium'` prints.
const MONTHS_SHORT: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

/// Seconds in a day: 24 * 60 * 60.
const SECONDS_PER_DAY: i64 = 86_400;

impl Date {
    /// The date as `useFormatDateTime({ year: 'numeric', month: 'long', day: 'numeric' })`
    /// prints it, which is `October 20, 2026` in en-US.
    fn long(self) -> String {
        let month = MONTHS
            .get(usize::from(self.month.saturating_sub(1)))
            .copied()
            .map_or("", |name| name);
        format!("{month} {}, {}", self.day, self.year)
    }

    /// Days from 1970-01-01 to this date: Hinnant's `days_from_civil`.
    ///
    /// The same twelve lines the screenshots page carries for its week buckets. Two pages with
    /// one date each is not worth a module, and a shared helper would have to live in a third
    /// file this change is not allowed to add.
    fn day_number(self) -> i64 {
        let month = i64::from(self.month);
        let year = i64::from(self.year) - i64::from(month <= 2);
        let era = if year >= 0 { year } else { year - 399 } / 400;
        let year_of_era = year - era * 400;
        let month_index = (month + 9) % 12;
        let day_of_year = (153 * month_index + 2) / 5 + i64::from(self.day) - 1;
        let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
        era * 146_097 + day_of_era - 719_468
    }
}

/// The release channel of a version, which is Labrinth's `version_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// A stable release.
    Release,
    /// A beta.
    Beta,
    /// An alpha.
    Alpha,
}

impl Channel {
    /// The word a tooltip and the table's reader would use.
    fn label(self) -> &'static str {
        match self {
            Channel::Release => "Release",
            Channel::Beta => "Beta",
            Channel::Alpha => "Alpha",
        }
    }
}

/// One release, as `GET /v2/project/{id}/version` returns it.
#[derive(Debug, Clone)]
pub struct Version {
    /// `version_number`: the version's own name.
    pub number: String,
    /// `name`: the longer label the row's tooltip carries.
    pub name: String,
    /// `game_versions`: the Minecraft versions the file runs on.
    pub game_versions: Vec<String>,
    /// `loaders`: the platforms the file is built for.
    pub loaders: Vec<String>,
    /// `version_type`.
    pub channel: Channel,
    /// `date_published`.
    pub published: Date,
    /// `downloads`.
    pub downloads: u64,
}

/// One gallery image, as `project.gallery` returns it.
#[derive(Debug, Clone)]
pub struct Image {
    /// `url`: the thumbnail the grid draws.
    pub url: String,
    /// `title`.
    pub title: String,
    /// `description`.
    pub description: String,
    /// `created`.
    pub created: Date,
}

/// One block of the description, as the project's markdown wrote it.
///
/// The reference renders the body as HTML (`markdown-body`), so a heading is a heading and an
/// image is a picture rather than a line of text. Four kinds cover everything the pages in this
/// catalogue actually use: running text, a heading, a quoted line and an image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    /// Running text, its inline markers removed.
    Text(String),
    /// A `###` heading or above, without its hashes.
    Heading(String),
    /// A `>` quote's line, without its marker.
    Quote(String),
    /// An image, by URL.
    Image(String),
}

/// What the page shows. The Discover card carries the same fields, plus what a full project
/// page needs.
#[derive(Clone)]
pub struct Project {
    /// The project's name.
    pub title: String,
    /// The team that publishes it.
    pub author: String,
    /// The one-line summary the header shows.
    pub summary: String,
    /// The longer description, already broken into paragraphs. Empty until the project endpoint
    /// has answered.
    pub body: Vec<String>,
    /// The same description as blocks -- headings, quotes and images kept as themselves -- for
    /// the tab that draws it. Empty means "draw `body` as paragraphs", which is what a project
    /// with no document yet falls back to.
    pub blocks: Vec<Block>,
    /// The loader and environment tags the header shows.
    pub tags: Vec<(String, Glyph)>,
    /// Game versions the project supports, newest first, from the `version` endpoint.
    pub game_versions: Vec<String>,
    /// The download count, abbreviated.
    pub downloads: String,
    /// The follower count, abbreviated.
    pub followers: String,
    /// The icon's URL.
    pub icon: String,
    /// The project's links, by the catalogue's `project.about.links.*` labels.
    pub links: Vec<String>,
    /// The team, as `(name, role)`.
    pub creators: Vec<(String, String)>,
    /// When it was first published, when the API said.
    pub published: Option<Date>,
    /// When it last changed, when the API said.
    pub updated: Option<Date>,
    /// The licence it ships under.
    pub license: String,
    /// The project's slug, which its Modrinth URL ends with (`modrinth.com/{type}/{slug}`). A
    /// search hit has no slug, and Modrinth resolves the id in the same position.
    pub slug: String,
    /// Labrinth's `project_type`: `mod`, `modpack`, `resourcepack`, `shader`, `datapack`.
    pub kind: String,
    /// The gallery, in the order the reference shows it.
    pub gallery: Vec<Image>,
    /// The releases, newest first.
    pub versions: Vec<Version>,
}

/// The page's tabs. `Description` is the default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    /// The project's long description.
    Description,
    /// Its released files.
    Versions,
    /// Its images.
    Gallery,
}

impl Tab {
    /// Every tab the page knows, in the reference's order.
    pub const ALL: [Tab; 3] = [Tab::Description, Tab::Versions, Tab::Gallery];

    /// The tab's label, from `app.project.tab.*`.
    pub fn label(self) -> &'static str {
        match self {
            Tab::Description => "Description",
            Tab::Versions => "Versions",
            Tab::Gallery => "Gallery",
        }
    }
}

/// The tabs a project actually offers.
///
/// The reference shows Description always, Versions unless the project is a server
/// (`Index.vue:174-192`) and Gallery only while `gallery.length > 0`; `NavTabs` renders
/// nothing at all under two links (`NavTabs.vue:3`), which a project with neither a gallery
/// nor a versions list is.
pub fn tabs(project: &Project) -> Vec<Tab> {
    let mut out = vec![Tab::Description, Tab::Versions];
    if !project.gallery.is_empty() {
        out.push(Tab::Gallery);
    }
    out
}

/// The page's own state: which tab is showing, where the tab strip's slider is, and which
/// gallery image is open full size.
#[derive(Debug, Clone)]
pub struct State {
    /// The selected tab.
    pub tab: Option<Tab>,
    /// The slider's left edge, in the tab strip's own coordinates, and where it is going.
    slider: Option<(f32, f32, f64)>,
    /// The gallery image being looked at, by its index in the grid.
    pub viewer: Option<usize>,
    /// The versions page the reader is on, counting from one.
    pub page: usize,
    /// The channel chips the reader has toggled on (Release/Beta/Alpha).
    pub channels: Vec<Channel>,
    /// The loader chips the reader has toggled on.
    pub loaders: Vec<&'static str>,
    /// **Which filter menu is open.** The reference\'s Versions tab has two comboboxes over the table -- Channel and
    /// Game version -- and this build drew four always-visible chips instead, which is a different control and a wider
    /// row.
    pub open_filter: Option<FilterKind>,
    /// The game version the reader has narrowed to, if any.
    pub game_version: Option<String>,
    /// A press the page took and could not act on, waiting for the shell.
    pressed: Option<Press>,
    /// Whether the header's menu is open.
    menu: bool,
    /// Whether the library already carries this project, as the last draw found it.
    installed: bool,
    /// When it opened, so the press that opened it cannot dismiss it on the same frame.
    menu_opened: f64,
}

impl Default for State {
    fn default() -> State {
        State {
            tab: None,
            slider: None,
            viewer: None,
            page: 1,
            channels: Vec::new(),
            loaders: Vec::new(),
            open_filter: None,
            game_version: None,
            pressed: None,
            menu: false,
            installed: false,
            menu_opened: 0.0,
        }
    }
}

/// A press the page took but cannot act on.
///
/// The page draws the reference's Install button and kebab menu, and the work behind both is the
/// shell's: the installer, the menus and the routes all live there, and this page's `draw`
/// returns a `bool` rather than an action (`shell.rs:812-833`). So the press is recorded here
/// and the shell takes it -- which is one line on the shell's side and no new seam:
///
/// ```ignore
/// if let Some(press) = self.project_state.take_press() { self.press_install(&project) }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Press {
    /// The header's Install button.
    Install,
    /// The header's kebab menu.
    More,
}

impl State {
    /// Take the last press, if the page is holding one.
    pub fn take_press(&mut self) -> Option<Press> {
        self.pressed.take()
    }

    /// The tab that is showing, clamped to the tabs this project offers.
    pub fn current(&self, project: &Project) -> Tab {
        let offered = tabs(project);
        let wanted = self.tab.unwrap_or(Tab::Description);
        if offered.contains(&wanted) {
            wanted
        } else {
            Tab::Description
        }
    }
}


// ---------------------------------------------------------------------------
// The rules, as pure functions
// ---------------------------------------------------------------------------

/// formatRelativeTime for the Published column, over dayjs's own ladder: under 26 days is a day
/// count, then a month, then months, then a year, then years.
///
/// dayjs measures in milliseconds and this measures in days, so the two differ only inside the
/// first day — where dayjs says "a few seconds ago" and this says "today", which is what a date
/// without a time can honestly say.
fn relative(published: Date, now: Date) -> String {
    let days = now.day_number() - published.day_number();
    match days {
        i64::MIN..=0 => "today".to_owned(),
        1 => "yesterday".to_owned(),
        2..=25 => format!("{days} days ago"),
        26..=45 => "a month ago".to_owned(),
        46..=319 => format!("{} months ago", days / 30),
        320..=547 => "a year ago".to_owned(),
        _ => format!("{} years ago", days / 365),
    }
}

/// The versions table's own relative date: `2d ago`, `2w ago`, `last mo` -- the short form the
/// reference's table prints where its header prints `2 days ago` (ref-project-versions.png: the
/// Published column reads 2d, 2d, 2w, 3w, 3w, 3w, last mo, last mo, last mo down the rows).
fn relative_short(published: Date, now: Date) -> String {
    let days = now.day_number() - published.day_number();
    match days {
        i64::MIN..=0 => "today".to_owned(),
        1 => "yesterday".to_owned(),
        2..=6 => format!("{days}d ago"),
        // The reference prints "last wk." for the week just gone, then weeks counted.
        7..=13 => "last wk.".to_owned(),
        14..=27 => format!("{}w ago", days / 7),
        28..=45 => "last mo".to_owned(),
        46..=319 => format!("{}mo ago", days / 30),
        320..=547 => "1y ago".to_owned(),
        _ => format!("{}y ago", days / 365),
    }
}

/// A metadata count with its noun, singular for exactly one: `5K followers`, `1 download`.
///
/// The value arrives already abbreviated -- it is the search hit's own `downloads` string on a
/// page opened from a card -- so only the noun is added here.
fn count(value: &str, noun: &str) -> String {
    if value.trim().is_empty() {
        return String::new();
    }
    if value.trim() == "1" {
        format!("{value} {noun}")
    } else {
        format!("{value} {noun}s")
    }
}

/// The compact form the Downloads column prints: formatCompactNumber, which is 1.2K, 19.8M,
/// 1.1B -- and `7,847` for a number under ten thousand, which the reference leaves grouped
/// rather than turning into `7.8K` (ref-project-versions.png, the 14.1.2 row).
fn compact(count: u64) -> String {
    if count < 10_000 {
        return grouped(count);
    }
    scaled(count)
}

/// The same number always abbreviated, which is what the header's two counts are: the
/// reference prints `5K followers` where its table would print `5,037`.
fn short_count(count: u64) -> String {
    scaled(count)
}

/// One decimal place, rounded, with a trailing `.0` dropped: `19.8M`, `155K`.
fn scaled(count: u64) -> String {
    for (size, suffix) in [(1_000_000_000_u64, "B"), (1_000_000, "M"), (1_000, "K")] {
        if count >= size {
            let tenths = (count as f64 / (size as f64 / 10.0)).round() as u64;
            let whole = tenths / 10;
            let fraction = tenths % 10;
            return if fraction == 0 {
                format!("{whole}{suffix}")
            } else {
                format!("{whole}.{fraction}{suffix}")
            };
        }
    }
    count.to_string()
}

/// A count with its thousands separated: `7,847`.
fn grouped(count: u64) -> String {
    let digits = count.to_string();
    let mut out = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

/// The gallery's column count and one item's width: repeat(auto-fill, minmax(20rem, 1fr)) over
/// the space the card leaves, at gap: 1rem (Gallery.vue:91-96).
fn gallery_grid(inner_width: f32) -> (usize, f32) {
    if inner_width <= 0.0 {
        return (1, GALLERY_MIN);
    }
    // auto-fill fits as many 320-point tracks as the row holds, and a track may grow.
    let columns =
        (((inner_width + GALLERY_GAP) / (GALLERY_MIN + GALLERY_GAP)).floor() as usize).max(1);
    let width = (inner_width - GALLERY_GAP * (columns.saturating_sub(1)) as f32) / columns as f32;
    (columns, width)
}

/// The versions a set of chips leaves: a version passes when it matches **every dimension that
/// has a chip on** and, inside one dimension, any of its values — the source's
/// `hasAnySelected(version.game_versions, selected)` per dimension, AND-ed across them
/// (`ProjectPageVersions.vue:700-712`).
///
/// A dimension with no chip toggled is not a filter, which is what makes an untouched filter row
/// show the whole table.
fn filtered_versions<'a>(
    versions: &'a [Version],
    channels: &[Channel],
    loaders: &[&str],
    game: Option<&str>,
) -> Vec<&'a Version> {
    versions
        .iter()
        .filter(|version| {
            (channels.is_empty() || channels.contains(&version.channel))
                && (loaders.is_empty()
                    || version
                        .loaders
                        .iter()
                        .any(|loader| loaders.contains(&loader.as_str())))
                && game.is_none_or(|wanted| version.game_versions.iter().any(|name| name == wanted))
        })
        .collect()
}

/// `mb-3` under the filter row (`ProjectPageVersions.vue:2`).
const VERSION_FILTER_GAP: f32 = 16.0;

/// Where the Versions tab's filter row starts, from the card edge the other tabs are bordered
/// at: the reference's own 3 points.
const VERSIONS_INSET: f32 = 3.0;

/// The filter row's own height: the reference's quiet selects are 28 points tall
/// (ref-project-versions.png, the `Channel` control's box at y 268..296).
const FILTER_HEIGHT: f32 = 28.0;

/// One page of releases: `pageSize` (`ProjectPageVersions.vue:687`).
const PAGE_SIZE: usize = 20;

/// A pager button: `!min-w-9` is 36 points square and round.
const PAGER_BUTTON: f32 = 36.0;

/// `gap-1` between two pager buttons.
const PAGER_GAP: f32 = 4.0;

/// The versions table's column widths, in the order the cells are drawn: channel, version, game
/// versions, platforms, published, downloads, actions.
///
/// ProjectPageVersions.vue:560-615 gives the channel 4.5rem, the chip columns max-w-[12rem],
/// published and downloads 12% each, actions 1%, and leaves the version number the rest.
fn table_columns(width: f32) -> [f32; 7] {
    // Measured off the reference's own table rather than derived from the sheet's percentages:
    // its header labels start at 162, 396, 613, 779 and 903 and its row action sits at 1072, so
    // the columns are 72 / 234 / 217 / 166 / 124 / 159 / 56 across a 1028-point table. The sheet
    // gives the channel 4.5rem and leaves the rest to `table-layout: auto`, which sizes the two
    // chip columns to their content -- so the proportions it states are not the ones it draws.
    const MEASURED: [f32; 7] = [72.0, 234.0, 217.0, 166.0, 124.0, 159.0, 56.0];
    let mut total = 0.0;
    for column in MEASURED {
        total += column;
    }
    let scale = width / total;
    let mut out = [0.0; 7];
    for (index, column) in MEASURED.iter().enumerate() {
        out[index] = column * scale;
    }
    out
}

/// The wall clock, as the page's [Date]: the Published column is relative to the reader's day.
fn today() -> Date {
    let seconds = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since_epoch| since_epoch.as_secs() as i64);
    civil_from_days(seconds / SECONDS_PER_DAY)
}

/// Hinnant's civil_from_days, the inverse of [Date::day_number].
pub(crate) fn civil_from_days(days: i64) -> Date {
    let shifted = days + 719_468;
    let era = if shifted >= 0 { shifted } else { shifted - 146_096 } / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    if month <= 2 {
        year += 1;
    }
    Date {
        year: year.min(i64::from(i32::MAX)) as i32,
        month: month as u8,
        day: day as u8,
    }
}

// ---------------------------------------------------------------------------
// The page
// ---------------------------------------------------------------------------

/// Draw the page's content column. Returns whether another frame is owed.
pub fn draw(
    ui: &mut Ui,
    rect: Rect,
    theme: &Theme,
    images: &mut Images,
    state: &mut State,
    project: &Project,
    installed: bool,
) -> bool {
    ui.painter().rect_filled(rect, 0, theme.BG);
    // The project this frame draws: what the shell handed over, plus whatever the project's own
    // document has added since. One request per open page, and the frame that finds it ready
    // draws the header, the tabs and the panel from it.
    let loaded = merged(ui.ctx(), project);
    let project = &loaded;
    state.installed = installed;
    if rect.width() <= 2.0 * PAGE_PADDING || rect.height() <= 2.0 * PAGE_PADDING {
        return false;
    }

    // The page column: flex flex-col gap-4 p-6 (Index.vue:48).
    //
    // Horizontally the rect a page is handed is *already* the padded column -- the shell's
    // `Page::rect` starts at `TEXT_LEFT` = 88 and ends at the sidebar's 24-point margin, which
    // is what the reference's `p-6` produces inside its 64-to-1140 viewport. Adding the inset
    // again put the header's avatar at 112 and the card's edges at 112 and 1092 where the
    // reference has 88 and 1116 (measured: ref-project.png y 300 runs 89..1114, this build's
    // 113..1090). So only the top inset is applied here.
    //
    // The bottom is the window's own edge: the reference's page scrolls, and its card is cut by
    // the window rather than stopping 24 points above it (ref-project.png y 890 is still card).
    // **The three-point offset is deliberate, and the reason is measured.** Raising the column by 3 puts the page\'s
    // first ink at y 74, exactly the reference\'s -- and takes the page from 71 380 to **80 122**, because the rhythm
    // below the header (its foot, the tab strip, the card\'s top) is 3 points different from the reference\'s. The
    // offset was compensating for that; aligning the top without fixing the rhythm moves every element below it.
    let column = Rect::from_min_max(
        egui::pos2(rect.left(), rect.top() + PAGE_PADDING),
        egui::pos2(rect.right(), rect.bottom()),
    );

    // Every rectangle a press is read against, for `SIFT_PROJECT_PROBE`.
    let mut probe_rows: Vec<(String, Rect)> = vec![("column".to_owned(), column)];
    let mut owed = false;

    // ---- The header, with its rule and its 16-point foot ----
    let header_height = header(ui, column, theme, images, project, state, &mut probe_rows);
    let rule_y = column.top() + header_height + HEADER_PAD_BOTTOM;
    ui.painter().rect_filled(
        Rect::from_min_size(
            egui::pos2(column.left(), rule_y),
            egui::vec2(column.width(), STROKE_WIDTH),
        ),
        0,
        theme.DIVIDER,
    );

    // The header's menu, over the page, under the kebab it was opened from.
    owed |= header_menu(ui, column, theme, project, state);

    // ---- The tab strip ----
    let offered = tabs(project);
    // The four points come back here: the rule belongs at 186 and the strip was already where the reference has
    // it, so the header\'s foot gave up four and the rule\'s gap took them. Moving both (the column raise, or the
    // foot alone) measured worse by 15 000 pixels than doing neither.
    let strip_top = rule_y + STROKE_WIDTH + BLOCK_GAP + 4.0;
    let strip = if offered.len() > 1 {
        let (strip, sliding) =
            tab_strip(ui, column, strip_top, theme, state, project, &offered, &mut probe_rows);
        owed |= sliding;
        Some(strip)
    } else {
        None
    };

    // ---- The tab's card ----
    let card_top = match strip {
        Some(strip) => strip.bottom() + BLOCK_GAP,
        None => strip_top,
    };
    let card = Rect::from_min_max(
        egui::pos2(column.left(), card_top),
        egui::pos2(column.right(), column.bottom()),
    );
    if card.height() < 1.0 {
        return owed;
    }

    // **The Versions tab is not a card.** `Versions.vue:3` is a plain `flex flex-col gap-3`
    // column: the filter row sits on the page's own background and the table is its own rounded
    // box, which is why the reference's filter row starts at y 268 where its page card would have
    // been bordered at 265, and why the table's border is at 313 (ref-project-versions.png).
    // **The Gallery is not a card either**: its gaps are page-coloured in the reference, and a `shrink(CARD_PAD)`
    // put every card 16 points in and 11 narrow (321.3 against the reference's 332).
    if matches!(state.current(project), Tab::Versions | Tab::Gallery) {
        let inner = Rect::from_min_max(
            egui::pos2(column.left(), card_top + VERSIONS_INSET),
            egui::pos2(column.right(), column.bottom()),
        );
        match state.current(project) {
            Tab::Versions => versions(ui, inner, theme, project, state),
            Tab::Gallery => gallery(ui, inner, theme, project, images, state, &mut probe_rows),
            _ => {}
        }
    } else {
        ui.painter().rect_filled(card, CARD_RADIUS, theme.SURFACE_3);
        ui.painter().rect_stroke(
            card,
            CARD_RADIUS,
            Stroke::new(STROKE_WIDTH, theme.SURFACE_4),
            egui::StrokeKind::Inside,
        );

        let inner = card.shrink(CARD_PAD);
        match state.current(project) {
            Tab::Description => description(ui, inner, theme, project, images),
            _ => {}
        }
    }

    probe_rows.push(("card".to_owned(), card));

    // The gallery's viewer is drawn last, over everything the page owns.
    owed |= viewer(ui, rect, theme, images, state, project);
    probe(ui, &probe_rows);
    if std::env::var_os("SIFT_PROJECT_PROBE").is_some() {
        if let Some(press) = state.pressed {
            eprintln!("project-probe holding press={press:?}");
        }
    }
    owed
}

/// Set `SIFT_PROJECT_PROBE` to print every rectangle a press is read against, on any frame the
/// pointer clicks. The same instrument the settings, servers and screenshots pages carry.
fn probe(ui: &Ui, rows: &[(String, Rect)]) {
    if std::env::var_os("SIFT_PROJECT_PROBE").is_none() {
        return;
    }
    if !ui.input(|input| input.pointer.any_click()) {
        return;
    }
    let pointer = ui.input(|input| input.pointer.interact_pos());
    eprintln!("project-probe pointer={pointer:?}");
    for (name, rect) in rows {
        eprintln!(
            "project-probe {name:<22} {:.1},{:.1} {:.1}x{:.1}",
            rect.left(),
            rect.top(),
            rect.width(),
            rect.height()
        );
    }
}

/// The header's row: the avatar, the name, the summary and the metadata, with the install
/// button and the menu at the right. Returns the height the header used, before its rule.
fn header(
    ui: &mut Ui,
    column: Rect,
    theme: &Theme,
    images: &mut Images,
    project: &Project,
    state: &mut State,
    probe_rows: &mut Vec<(String, Rect)>,
) -> f32 {
    let painter = ui.painter();
    let avatar = Rect::from_min_size(column.min, Vec2::splat(AVATAR));
    painter.rect_filled(avatar, 16, theme.BUTTON_BG);
    if let Some(texture) = images.get(ui.ctx(), &project.icon) {
        painter.image(
            texture.id(),
            avatar,
            Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    }

    let text_left = avatar.right() + HEADER_GAP;
    // The actions are laid out from the right first, so the summary knows what width it has.
    // The reference puts the kebab at the column's right edge and the button to its left:
    // its kebab's box ends at 1116 and the green fill runs 946..1060 (ref-project.png y 95).
    // This had them the other way round -- button flush right, kebab to its left -- so every
    // control in the header's action corner was 48 points out of place.
    let install_width = install_width(painter, theme);
    let kebab = Rect::from_min_size(
        egui::pos2(column.right() - KEBAB, column.top()),
        Vec2::splat(KEBAB),
    );
    let install = Rect::from_min_size(
        egui::pos2(kebab.left() - INSTALL_GAP - install_width, column.top()),
        egui::vec2(install_width, INSTALL_HEIGHT),
    );

    // The title: text-2xl font-semibold leading-none text-contrast.
    painter.text(
        egui::pos2(text_left, column.top()),
        Align2::LEFT_TOP,
        project.title.as_str(),
        fonts::semibold(TITLE_SIZE),
        theme.CONTRAST,
    );

    // The summary: max-w-[44rem], line-clamp-2, at the body size.
    let summary_top = column.top() + TITLE_LINE + SUMMARY_GAP;
    let summary_width = (kebab.left() - HEADER_GAP - text_left)
        .min(SUMMARY_MAX)
        .max(1.0);
    let summary = clamp_to_lines(
        painter,
        &project.summary,
        fonts::body(SUMMARY_SIZE),
        theme.SECONDARY,
        summary_width,
        SUMMARY_LINES,
    );
    let summary_height = summary.size().y;
    painter.galley(
        egui::pos2(text_left, summary_top),
        summary,
        theme.SECONDARY,
    );

    // The metadata row under the summary: two number items with a bullet between them, then
    // the category tags.
    let meta_top = summary_top + summary_height + META_CONTENT_GAP;
    let mut x = text_left;
    // "19.8M downloads", "5K followers": the reference's metadata items carry the noun, and
    // the project struct carries the number alone.
    let downloads = count(&project.downloads, "download");
    let followers = count(&project.followers, "follower");
    for (index, (glyph, value)) in [
        (Glyph::Download, downloads.as_str()),
        (Glyph::Heart, followers.as_str()),
    ]
    .into_iter()
    .enumerate()
    {
        icons::draw(
            painter,
            glyph,
            Rect::from_min_size(
                egui::pos2(x, meta_top + (TAG_HEIGHT - META_ICON) / 2.0),
                Vec2::splat(META_ICON),
            ),
            theme.SECONDARY,
        );
        let label = painter.layout_no_wrap(value.to_owned(), fonts::label(META_TEXT), theme.BASE);
        painter.galley(
            egui::pos2(x + META_ICON + META_CONTENT_GAP, meta_top + (TAG_HEIGHT - META_TEXT) / 2.0),
            label.clone(),
            theme.BASE,
        );
        x += META_ICON + META_CONTENT_GAP + label.size().x;
        if index == 0 {
            // The bullet the sheet puts between two metadata items.
            painter.circle_filled(
                egui::pos2(x + META_GAP / 2.0, meta_top + TAG_HEIGHT / 2.0),
                META_BULLET / 2.0,
                theme.SECONDARY,
            );
            x += META_GAP;
        }
    }
    x += META_GAP;
    // **Categories only, as the reference shows them.** The hit\'s tag list also carries loaders (Fabric, Forge) and
    // environments (Client, Server); the reference puts those in the sidebar\'s Platforms and Supported environments
    // rows and keeps the header for categories. Drawing all five put two chips in the header that are not there.
    const NOT_A_CATEGORY: [&str; 6] = ["fabric", "forge", "neoforge", "quilt", "client", "server"];
    for (tag, glyph) in project
        .tags
        .iter()
        .filter(|(tag, _)| !NOT_A_CATEGORY.contains(&tag.to_lowercase().as_str()))
    {
        let chip = tag_rect(painter, egui::pos2(x, meta_top), theme, tag.as_str(), Some(*glyph));
        x = chip.right() + TAG_GAP;
    }

    // The install button and the kebab menu. Both are drawn at the reference's size; the work
    // behind them is the shell's, so a press is recorded for it to take rather than dropped.
    // A project the library already carries says so, and its button leads to the instance instead of
    // starting a second copy of the same pack.
    primary_button(
        painter,
        install,
        theme,
        if state.installed { "Installed" } else { "Install" },
        if state.installed { Glyph::Folder } else { Glyph::Download },
    );
    if ui
        .interact(install, egui::Id::new("project-install"), Sense::click())
        .clicked()
    {
        state.pressed = Some(Press::Install);
        ui.ctx().request_repaint();
    }
    if ui
        .interact(kebab, egui::Id::new("project-more"), Sense::click())
        .clicked()
    {
        state.menu = !state.menu;
        // The same press that opens the menu is also "a press anywhere else" on this frame, which
        // closed it before it drew. The frame clock is the guard.
        state.menu_opened = ui.input(|input| input.time);
        ui.ctx().request_repaint();
    }
    icons::draw(
        painter,
        Glyph::EllipsisVertical,
        Rect::from_center_size(kebab.center(), Vec2::splat(KEBAB_GLYPH)),
        theme.SECONDARY,
    );

    probe_rows.push(("install".to_owned(), install));
    probe_rows.push(("kebab".to_owned(), kebab));

    // The header's own height: the taller of the avatar and the text column.
    let text_height = TITLE_LINE + SUMMARY_GAP + summary_height + META_CONTENT_GAP + TAG_HEIGHT;
    AVATAR.max(text_height)
}

/// The header's menu: the rows the reference's kebab carries that this build can carry out
/// (Index.vue:376-400).
///
/// The reference's list is Follow (disabled, "Coming soon"), Save (disabled, "Coming soon"),
/// Open in Modrinth, a divider and Report. Follow and Save are account state this build has no
/// client for, and Report is a page this launcher does not have. What is left is the project's
/// own link twice: open it, or copy it. Both are real, and a row that copied nothing would be
/// the one outcome a menu must not have.
fn header_menu(
    ui: &mut Ui,
    column: Rect,
    theme: &Theme,
    project: &Project,
    state: &mut State,
) -> bool {
    if !state.menu {
        return false;
    }
    let rows = [
        ("Open in Modrinth", Glyph::External),
        ("Copy link", Glyph::ClipboardCopy),
    ];
    let plate = Rect::from_min_size(
        egui::pos2(column.right() - MENU_WIDTH, column.top() + KEBAB + 4.0),
        egui::vec2(MENU_WIDTH, rows.len() as f32 * MENU_ROW + 2.0 * MENU_PAD),
    );
    let painter = ui.painter();
    painter.rect_filled(plate, 10, theme.SUPER_RAISED_BG);
    painter.rect_stroke(
        plate,
        10,
        Stroke::new(STROKE_WIDTH, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );

    let url = format!("https://modrinth.com/{}/{}", project.kind, project.slug);
    let mut chosen = None;
    for (index, (label, glyph)) in rows.into_iter().enumerate() {
        let row = Rect::from_min_size(
            egui::pos2(
                plate.left() + MENU_PAD,
                plate.top() + MENU_PAD + index as f32 * MENU_ROW,
            ),
            egui::vec2(plate.width() - 2.0 * MENU_PAD, MENU_ROW),
        );
        let response = ui.interact(row, egui::Id::new(("project-menu", index)), Sense::click());
        if response.hovered() {
            ui.painter().rect_filled(row, 8, theme.SURFACE_4);
        }
        icons::draw(
            ui.painter(),
            glyph,
            Rect::from_center_size(
                egui::pos2(row.left() + TAG_PAD_X + TAG_GLYPH / 2.0, row.center().y),
                Vec2::splat(TAG_GLYPH),
            ),
            theme.BASE,
        );
        ui.painter().text(
            egui::pos2(row.left() + TAG_PAD_X + TAG_GLYPH + TAG_GAP, row.center().y),
            Align2::LEFT_CENTER,
            label,
            fonts::body(TAG_TEXT),
            theme.BASE,
        );
        if response.clicked() {
            chosen = Some(index);
        }
    }

    // A press anywhere else puts the menu away.
    let (pointer, clicked) = ui.input(|input| {
        (input.pointer.interact_pos(), input.pointer.primary_clicked())
    });
    let settled = ui.input(|input| input.time) > state.menu_opened + 0.15;
    if settled && clicked && !pointer.is_some_and(|at| plate.contains(at)) {
        state.menu = false;
    }

    match chosen {
        Some(0) => {
            state.menu = false;
            open_url(&url);
            true
        }
        Some(1) => {
            state.menu = false;
            ui.ctx().copy_text(url);
            true
        }
        _ => false,
    }
}

/// Ask the desktop to open a URL, the way openUrl does in the reference (Index.vue:404-408).
fn open_url(url: &str) {
    let opened = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn()
    } else if cfg!(target_os = "macos") {
        std::process::Command::new("open").arg(url).spawn()
    } else {
        std::process::Command::new("xdg-open").arg(url).spawn()
    };
    let _ = opened;
}

/// Lay text out and, if it takes more rows than the sheet's clamp allows, drop words from its
/// end until an ellipsis fits: line-clamp-2 (page-header/index.vue:65).
fn clamp_to_lines(
    painter: &egui::Painter,
    text: &str,
    font: egui::FontId,
    ink: Color32,
    width: f32,
    lines: usize,
) -> std::sync::Arc<egui::Galley> {
    let full = painter.layout(text.to_owned(), font.clone(), ink, width);
    if full.rows.len() <= lines {
        return full;
    }
    let mut words: Vec<&str> = text.split_whitespace().collect();
    while !words.is_empty() {
        words.pop();
        let mut candidate = words.join(" ");
        candidate.push('…');
        let clamped = painter.layout(candidate, font.clone(), ink, width);
        if clamped.rows.len() <= lines {
            return clamped;
        }
    }
    painter.layout("…".to_owned(), font, ink, width)
}

/// The width an Install button resolves to: its two paddings, its glyph, its gap and its label.
fn install_width(painter: &egui::Painter, theme: &Theme) -> f32 {
    let label = painter.layout_no_wrap(
        "Install".to_owned(),
        fonts::semibold(INSTALL_TEXT),
        theme.BUTTON_TEXT_SELECTED,
    );
    2.0 * INSTALL_PAD + INSTALL_GLYPH + INSTALL_GAP + label.size().x
}

/// A Button at size xl and color brand: h-12 gap-2 rounded-2xl px-3.5 text-base font-extrabold
/// with a 24-point glyph (ButtonFrame.vue:33).
fn primary_button(painter: &egui::Painter, rect: Rect, theme: &Theme, label: &str, glyph: Glyph) {
    // `color="green"` is the brand fill with the accent's own contrast ink on it
    // (`--color-brand` #1bd96a under `--color-accent-contrast` #000). This drew
    // `button-bg-selected` -- the translucent *highlight* tint -- under `button-text-selected`,
    // which inverted the pair: a 114x48 block of (36,85,61) with green text where the reference
    // has (27,217,106) with black text (ref-project.png x 946..1060, y 72..119).
    painter.rect_filled(rect, INSTALL_RADIUS, theme.BRAND);
    icons::draw(
        painter,
        glyph,
        Rect::from_center_size(
            egui::pos2(
                rect.left() + INSTALL_PAD + INSTALL_GLYPH / 2.0,
                rect.center().y,
            ),
            Vec2::splat(INSTALL_GLYPH),
        ),
        theme.ACCENT_CONTRAST,
    );
    painter.text(
        egui::pos2(
            rect.left() + INSTALL_PAD + INSTALL_GLYPH + INSTALL_GAP,
            rect.center().y,
        ),
        Align2::LEFT_CENTER,
        label,
        fonts::semibold(INSTALL_TEXT),
        theme.ACCENT_CONTRAST,
    );
}

/// A TagItem: px-2 py-1 leading-none rounded-full text-sm border surface-5 bg-button-bg
/// text-secondary, gap-1, svg 16 (base/TagItem.vue:18-19). Returns the rect it used.
fn tag_rect(
    painter: &egui::Painter,
    min: egui::Pos2,
    theme: &Theme,
    label: &str,
    glyph: Option<Glyph>,
) -> Rect {
    let text = painter.layout_no_wrap(label.to_owned(), fonts::label(TAG_TEXT), theme.SECONDARY);
    let glyph_width = if glyph.is_some() {
        TAG_GLYPH + TAG_GAP
    } else {
        0.0
    };
    let rect = Rect::from_min_size(
        min,
        egui::vec2(text.size().x + glyph_width + 2.0 * TAG_PAD_X, TAG_HEIGHT),
    );
    let radius = (TAG_HEIGHT / 2.0) as u8;
    painter.rect_filled(rect, radius, theme.BUTTON_BG);
    painter.rect_stroke(
        rect,
        radius,
        Stroke::new(STROKE_WIDTH, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );
    if let Some(glyph) = glyph {
        icons::draw(
            painter,
            glyph,
            Rect::from_center_size(
                egui::pos2(rect.left() + TAG_PAD_X + TAG_GLYPH / 2.0, rect.center().y),
                Vec2::splat(TAG_GLYPH),
            ),
            theme.SECONDARY,
        );
    }
    painter.galley(
        egui::pos2(
            rect.left() + TAG_PAD_X + glyph_width,
            rect.center().y - text.size().y / 2.0,
        ),
        text,
        theme.SECONDARY,
    );
    rect
}


// ---------------------------------------------------------------------------
// The tab strip
// ---------------------------------------------------------------------------

/// The tab strip: a rounded track with a slider under the active tab (`NavTabs.vue:7-13,63-72`).
/// Returns the track's rect and whether the slider is still moving.
fn tab_strip(
    ui: &mut Ui,
    column: Rect,
    top: f32,
    theme: &Theme,
    state: &mut State,
    project: &Project,
    offered: &[Tab],
    probe_rows: &mut Vec<(String, Rect)>,
) -> (Rect, bool) {
    let painter = ui.painter();
    let pill_height = TAB_LINE + 2.0 * TAB_PAD_Y;
    let track_height = pill_height + 2.0 * TAB_TRACK_PAD;
    let widths: Vec<f32> = offered
        .iter()
        .map(|tab| {
            painter
                .layout_no_wrap(tab.label().to_owned(), fonts::semibold(TAB_TEXT), theme.BASE)
                .size()
                .x
                + 2.0 * TAB_PAD_X
        })
        .collect();
    let content_width: f32 = widths.iter().sum();
    let track = Rect::from_min_size(
        egui::pos2(column.left(), top),
        egui::vec2(content_width + 2.0 * TAB_TRACK_PAD, track_height),
    );
    painter.rect_filled(track, track_height / 2.0, theme.RAISED_BG);
    painter.rect_stroke(
        track,
        track_height / 2.0,
        Stroke::new(STROKE_WIDTH, theme.SURFACE_4),
        egui::StrokeKind::Inside,
    );

    let current = state.current(project);
    let mut target = track.left() + TAB_TRACK_PAD;
    let mut pill_width = 0.0;
    let mut x = target;
    for (tab, width) in offered.iter().zip(&widths) {
        if *tab == current {
            target = x;
            pill_width = *width;
        }
        x += width;
    }

    // The slide: 80 ms cubic-bezier(0.4, 0, 0.2, 1) (`NavTabs.vue:260-273`), sampled with a
    // smoothstep, which is the same curve's shape over the same window.
    let now = ui.ctx().input(|input| input.time);
    let (slider_left, sliding) = match state.slider {
        Some((from, to, started)) if (to - target).abs() < 0.5 => {
            let t = (((now - started) / SLIDER_SECONDS).clamp(0.0, 1.0)) as f32;
            let eased = t * t * (3.0 - 2.0 * t);
            (from + (to - from) * eased, t < 1.0)
        }
        _ => {
            state.slider = None;
            (target, false)
        }
    };
    if sliding {
        ui.ctx().request_repaint();
    }
    let pill = Rect::from_min_size(
        egui::pos2(slider_left, track.top() + TAB_TRACK_PAD),
        egui::vec2(pill_width, pill_height),
    );
    ui.painter()
        .rect_filled(pill, pill_height / 2.0, theme.BUTTON_BG_SELECTED);

    probe_rows.push(("tab-track".to_owned(), track));

    // The labels, over the slider.
    let mut x = track.left() + TAB_TRACK_PAD;
    for (tab, width) in offered.iter().zip(&widths) {
        let hit = Rect::from_min_size(
            egui::pos2(x, track.top() + TAB_TRACK_PAD),
            egui::vec2(*width, pill_height),
        );
        probe_rows.push((format!("tab {}", tab.label()), hit));
        let response = ui.interact(
            hit,
            egui::Id::new(("project-tab", tab.label())),
            Sense::click(),
        );
        if response.clicked() && *tab != current {
            state.slider = Some((slider_left, x, now));
            state.tab = Some(*tab);
            ui.ctx().request_repaint();
        }
        ui.painter().text(
            hit.center(),
            Align2::CENTER_CENTER,
            tab.label(),
            fonts::semibold(TAB_TEXT),
            if *tab == current {
                theme.BUTTON_TEXT_SELECTED
            } else {
                theme.BASE
            },
        );
        x += width;
    }

    (track, sliding)
}

// ---------------------------------------------------------------------------
// The Description tab
// ---------------------------------------------------------------------------

/// One line of the description: a run of text, or a row of inline pictures.
enum Line {
    /// A laid-out paragraph, heading or quote.
    Text(std::sync::Arc<egui::Galley>, Color32, f32),
    /// The pictures that share one line, at the size each of them draws at.
    Pictures(Vec<(String, Vec2)>),
}

/// The gap under a paragraph: a markdown `p`'s own margin.
const PARAGRAPH_GAP: f32 = 16.0;

/// The gap under a heading: `h3`'s own `margin-bottom`, which is half a paragraph's. Six rather
/// than eight: at eight every line after the first heading of the reference's body landed two
/// points low (measured: ref 839/863, this build 841/865).
const HEADING_GAP: f32 = 6.0;

/// A paragraph's line height: `line-height: 1.5` on a 16-point body, so a row is 24 points
/// where the font alone gives 19.
const BODY_LINE: f32 = 24.0;

/// A heading's line height, for its 20-point type.
const HEADING_LINE: f32 = 28.0;

/// The room an inline picture leaves under itself: the line box is the picture plus the font's
/// descender, which is why the reference's badge row ends at 315 and its next paragraph starts
/// at 338 rather than 331.
const PICTURE_DESCENDER: f32 = 6.0;

/// A heading's size: the reference's `.markdown-body h3`, 1.25 times the body's 16.
const HEADING_SIZE: f32 = 20.0;

/// How far a quote is inset from the body's own left edge.
const QUOTE_INDENT: f32 = 16.0;

/// Lay a block of text out at the reference's own line height.
///
/// `Painter::layout` packs a row to the font's own height -- 19 points for the 16-point body --
/// where the reference's markdown sets `line-height: 1.5`, so its rows are 24 apart. The extra
/// five points per row is the whole of the difference between a paragraph of two lines landing
/// at y 839 and y 863 and one landing at 818 and 837.
fn paragraph(
    painter: &egui::Painter,
    text: &str,
    font: egui::FontId,
    ink: Color32,
    width: f32,
    line: f32,
) -> std::sync::Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::single_section(
        text.to_owned(),
        egui::TextFormat {
            font_id: font,
            color: ink,
            line_height: Some(line),
            ..Default::default()
        },
    );
    job.wrap.max_width = width;
    painter.layout_job(job)
}

/// The description tab: the rendered markdown body inside the card, and nothing else.
///
/// The reference's tab is `Card` wrapping `ProjectPageDescription` (`Description.vue:1-5`); the
/// action row this page used to draw above the body was invented, and is gone.
///
/// The body is drawn from `Project::blocks`: paragraphs at the body size, headings at
/// `HEADING_SIZE`, quotes inset and muted, and pictures at the size the file itself is -- which is
/// what the reference's `markdown-body img` does, and the reason a pack's badge row is 34 points
/// tall rather than a full-width block.
fn description(ui: &mut Ui, inner: Rect, theme: &Theme, project: &Project, images: &mut Images) {
    let blocks: Vec<Block> = if project.blocks.is_empty() {
        project
            .body
            .iter()
            .cloned()
            .map(Block::Text)
            .collect()
    } else {
        project.blocks.clone()
    };
    if blocks.is_empty() {
        // A card carries no description; the project endpoint does. Saying so is the honest
        // state, and it is what the reference shows while its own query is pending.
        empty_tab(ui, inner, theme, "No description yet.");
        return;
    }

    // Measure first: the run's own height is what the card has to scroll, and a picture's size
    // is only known once its texture is in hand.
    let painter = ui.painter();
    let mut laid: Vec<Line> = Vec::new();
    let mut run: Vec<(String, Vec2)> = Vec::new();
    let mut run_width = 0.0;
    for block in &blocks {
        let picture = match block {
            Block::Image(url) => Some(url),
            _ => None,
        };
        if let Some(url) = picture {
            let size = images
                .get_picture(ui.ctx(), url)
                .map(|texture| texture.size_vec2())
                .unwrap_or(Vec2::ZERO);
            let size = if size.x > inner.width() {
                size * (inner.width() / size.x)
            } else {
                size
            };
            if size.x > 0.0 {
                if !run.is_empty() && run_width + size.x > inner.width() {
                    laid.push(Line::Pictures(std::mem::take(&mut run)));
                    run_width = 0.0;
                }
                run_width += size.x + 4.0;
                run.push((url.clone(), size));
            }
            continue;
        }
        if !run.is_empty() {
            laid.push(Line::Pictures(std::mem::take(&mut run)));
            run_width = 0.0;
        }
        match block {
            Block::Text(text) => laid.push(Line::Text(
                paragraph(
                    painter,
                    text,
                    fonts::body(SUMMARY_SIZE),
                    theme.BASE,
                    inner.width(),
                    BODY_LINE,
                ),
                theme.BASE,
                PARAGRAPH_GAP,
            )),
            Block::Heading(text) => laid.push(Line::Text(
                paragraph(
                    painter,
                    text,
                    fonts::semibold(HEADING_SIZE),
                    theme.CONTRAST,
                    inner.width(),
                    HEADING_LINE,
                ),
                theme.CONTRAST,
                HEADING_GAP,
            )),
            Block::Quote(text) => laid.push(Line::Text(
                paragraph(
                    painter,
                    text,
                    fonts::body(SUMMARY_SIZE),
                    theme.SECONDARY,
                    inner.width() - QUOTE_INDENT,
                    BODY_LINE,
                ),
                theme.SECONDARY,
                PARAGRAPH_GAP,
            )),
            Block::Image(_) => {}
        }
    }
    if !run.is_empty() {
        laid.push(Line::Pictures(run));
    }

    let mut total = 0.0;
    for line in &laid {
        total += match line {
            Line::Text(galley, ..) => galley.size().y,
            Line::Pictures(pictures) => pictures
                .iter()
                .map(|(_, size)| size.y)
                .fold(0.0_f32, f32::max)
                + PICTURE_DESCENDER,
        } + match line {
            Line::Text(_, _, gap) => *gap,
            Line::Pictures(_) => PARAGRAPH_GAP,
        };
    }

    let place = |painter: &egui::Painter, images: &mut Images, ctx: &egui::Context, left: f32, top: f32, lines: &[Line]| {
        let mut y = top;
        for line in lines {
            match line {
                Line::Text(galley, ink, gap) => {
                    painter.galley(egui::pos2(left, y), galley.clone(), *ink);
                    y += galley.size().y + gap;
                }
                Line::Pictures(pictures) => {
                    let mut x = left;
                    let mut height = 0.0_f32;
                    for (url, size) in pictures {
                        let rect = Rect::from_min_size(egui::pos2(x, y), *size);
                        if let Some(texture) = images.get_picture(ctx, url) {
                            painter.image(
                                texture.id(),
                                rect,
                                Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                                Color32::WHITE,
                            );
                        }
                        x += size.x + 4.0;
                        height = height.max(size.y);
                    }
                    y += height + PICTURE_DESCENDER + PARAGRAPH_GAP;
                }
            }
        }
    };

    if total > inner.height() {
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner));
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(&mut child, |ui| {
                let (area, _) =
                    ui.allocate_exact_size(egui::vec2(inner.width(), total), Sense::hover());
                let painter = ui.painter();
                place(painter, images, ui.ctx(), area.left(), area.top(), &laid);
            });
        return;
    }
    let left = inner.left();
    let top = inner.top();
    let painter = ui.painter();
    place(painter, images, ui.ctx(), left, top, &laid);
}
// ---------------------------------------------------------------------------
// The Versions tab
// ---------------------------------------------------------------------------

/// One control in the pager.
enum Pager {
    /// A page number.
    Page(usize),
    /// The ellipsis the reference draws where the pages skip, which is its rotated
    /// \`EllipsisVerticalIcon\`.
    Gap,
    /// The previous page's chevron.
    Previous,
    /// The next page's chevron.
    Next,
}

/// The reference's \`Pagination\` (\`ui/src/components/base/Pagination.vue\`): a chevron back while
/// there is a page behind, the current page as a filled round pill, its neighbours, an ellipsis
/// where the list skips, the last page, and a chevron on. Every button is \`!min-w-9\` -- 36 points
/// square and round -- with \`gap-1\` between them, and the run is right-aligned on the row it
/// shares with the filters, which is what \`justify-between\` does in the sheet.
///
/// Returns the page a press asked for, if any.
fn pagination(
    ui: &mut Ui,
    right: f32,
    centre: f32,
    page: usize,
    count: usize,
    theme: &Theme,
) -> Option<usize> {
    if count <= 1 {
        return None;
    }
    // The list its own \`computed\` builds: first, gap, previous, current, next, gap, last.
    let mut items: Vec<Pager> = Vec::new();
    let (first, last) = (1usize, count);
    let previous = page.saturating_sub(1);
    let next = page + 1;
    if page > 1 {
        items.push(Pager::Previous);
    }
    if previous > first {
        items.push(Pager::Page(first));
    }
    if previous > first + 1 {
        items.push(Pager::Gap);
    }
    if page > 1 && previous >= first {
        items.push(Pager::Page(previous));
    }
    items.push(Pager::Page(page));
    if next <= last {
        items.push(Pager::Page(next));
    }
    if next + 1 < last {
        items.push(Pager::Gap);
    }
    if next < last {
        items.push(Pager::Page(last));
    }
    if page < last {
        items.push(Pager::Next);
    }

    let width = items.len() as f32 * PAGER_BUTTON + (items.len() - 1) as f32 * PAGER_GAP;
    let mut x = right - width;
    let mut wanted = None;
    for item in &items {
        let rect = Rect::from_center_size(
            egui::pos2(x + PAGER_BUTTON / 2.0, centre),
            Vec2::splat(PAGER_BUTTON),
        );
        let painter = ui.painter();
        match item {
            Pager::Page(number) => {
                let current = *number == page;
                if current {
                    painter.rect_filled(rect, PAGER_BUTTON / 2.0, theme.BRAND_HIGHLIGHT);
                }
                painter.text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    number.to_string(),
                    // **16, not TAB_TEXT.** The reference's selected numeral inks 14 rows and this drew 10, which also made
                    // the right-aligned row 27 points narrower and pushed every item to its right.
                    fonts::semibold(16.0),
                    if current {
                        theme.BUTTON_TEXT_SELECTED
                    } else {
                        theme.BASE
                    },
                );
                if ui
                    .interact(rect, egui::Id::new(("versions-page", *number)), Sense::click())
                    .clicked()
                {
                    wanted = Some(*number);
                }
            }
            Pager::Gap => {
                painter.text(
                    rect.center(),
                    Align2::CENTER_CENTER,
                    "…",
                    fonts::semibold(16.0),
                    theme.BASE,
                );
            }
            Pager::Previous => {
                icons::draw(
                    painter,
                    Glyph::ChevronLeft,
                    Rect::from_center_size(rect.center(), Vec2::splat(16.0)),
                    theme.BASE,
                );
                if ui
                    .interact(rect, egui::Id::new("versions-page-prev"), Sense::click())
                    .clicked()
                {
                    wanted = Some(page - 1);
                }
            }
            Pager::Next => {
                icons::draw(
                    painter,
                    Glyph::ChevronRight,
                    Rect::from_center_size(rect.center(), Vec2::splat(16.0)),
                    theme.BASE,
                );
                if ui
                    .interact(rect, egui::Id::new("versions-page-next"), Sense::click())
                    .clicked()
                {
                    wanted = Some(page + 1);
                }
            }
        }
        x += PAGER_BUTTON + PAGER_GAP;
    }
    wanted
}

/// The versions tab: the reference's `Table` (`ProjectPageVersions.vue:560-615`).
fn versions(ui: &mut Ui, inner: Rect, theme: &Theme, project: &Project, state: &mut State) {
    if project.versions.is_empty() {
        empty_tab(ui, inner, theme, "No versions yet.");
        return;
    }
    let now = today();

    // The filter row: one chip per channel and per loader the project actually has, toggled on
    // and off (`ProjectPageVersions.vue:2-28`, the `VersionFilterControl` above the table), with
    // the reference's own `Pagination` at its right-hand end.
    let mut filter_row = Rect::from_min_size(inner.min, egui::vec2(inner.width(), FILTER_HEIGHT));
    // Hoisted: the open menu is drawn after the table, so its anchors outlive the block that drew the triggers.
    let mut channel_rect = Rect::NOTHING;
    let mut version_rect = Rect::NOTHING;
    {
        let painter = ui.painter();
        // **Two comboboxes, as the reference has them** (`ProjectPageVersions.vue:2-28`): `Channel`, with the filter
        // mark, and `Game version`. This row was four always-visible chips, which is a different control and a wider
        // row; the chips\' semantics are kept inside the menus.
        let mut x = filter_row.left();
        let chip_y = filter_row.top() + (FILTER_HEIGHT - TAG_HEIGHT) / 2.0;
        let channel_label = match state.channels.len() {
            0 => "Channel".to_owned(),
            1 => state.channels[0].label().to_owned(),
            count => format!("{count} channels"),
        };
        channel_rect = filter_trigger(
            painter,
            egui::pos2(x, chip_y),
            theme,
            Some(Glyph::Filter),
            &channel_label,
            matches!(state.open_filter, Some(FilterKind::Channel)),
        );
        if ui
            .interact(channel_rect, egui::Id::new("version-channel"), Sense::click())
            .clicked()
        {
            state.open_filter = match state.open_filter {
                Some(FilterKind::Channel) => None,
                _ => Some(FilterKind::Channel),
            };
            ui.ctx().request_repaint();
        }
        x = channel_rect.right() + TAG_GAP;
        let version_label = state
            .game_version
            .clone()
            .unwrap_or_else(|| "Game version".to_owned());
        version_rect = filter_trigger(
            painter,
            egui::pos2(x, chip_y),
            theme,
            None,
            &version_label,
            matches!(state.open_filter, Some(FilterKind::Version)),
        );
        if ui
            .interact(version_rect, egui::Id::new("version-game"), Sense::click())
            .clicked()
        {
            state.open_filter = match state.open_filter {
                Some(FilterKind::Version) => None,
                _ => Some(FilterKind::Version),
            };
            ui.ctx().request_repaint();
        }
        filter_row = Rect::from_min_size(
            egui::pos2(inner.left(), inner.top()),
            egui::vec2(inner.width(), FILTER_HEIGHT),
        );
    }

    let filtered = filtered_versions(
        &project.versions,
        &state.channels,
        &state.loaders,
        state.game_version.as_deref(),
    );
    let pages = filtered.len().div_ceil(PAGE_SIZE).max(1);
    if state.page > pages {
        state.page = pages;
    }
    // The pager, at the right-hand end of the filter row (`Pagination`, right-aligned there
    // because the row is `justify-between`). A press turns the page and nothing else.
    if let Some(wanted) = pagination(
        ui,
        filter_row.right(),
        filter_row.center().y,
        state.page,
        pages,
        theme,
    ) {
        state.page = wanted;
        ui.ctx().request_repaint();
    }
    let shown: Vec<&Version> = filtered
        .into_iter()
        .skip((state.page - 1) * PAGE_SIZE)
        .take(PAGE_SIZE)
        .collect();
    let table_top = filter_row.bottom() + VERSION_FILTER_GAP;
    if shown.is_empty() {
        empty_tab(
            ui,
            Rect::from_min_max(
                egui::pos2(inner.left(), table_top),
                egui::pos2(inner.right(), inner.bottom()),
            ),
            theme,
            "No versions match these filters.",
        );
        return;
    }
    let table_height = TABLE_HEADER + shown.len() as f32 * TABLE_ROW;
    let table = Rect::from_min_size(
        egui::pos2(inner.left(), table_top),
        egui::vec2(inner.width(), table_height),
    );

    // The list scrolls under the filter row, so the scroll area starts where the table does:
    // anchored at `inner` it drew the table over the chips, at the top of the tab (measured:
    // the table's border at 270 where the reference has it at 313).
    let list = Rect::from_min_max(egui::pos2(inner.left(), table_top), inner.max);
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(list));
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            let (area, _) =
                ui.allocate_exact_size(egui::vec2(table.width(), table_height), Sense::hover());
            let table = Rect::from_min_size(area.min, table.size());
            let painter = ui.painter();
            // overflow-hidden rounded-2xl border border-surface-4 (base/Table.vue:2-8).
            painter.rect_filled(table, CARD_RADIUS, theme.SURFACE_3);
            painter.rect_stroke(
                table,
                CARD_RADIUS,
                Stroke::new(STROKE_WIDTH, theme.SURFACE_4),
                egui::StrokeKind::Inside,
            );

            let columns = table_columns(table.width());
            let labels = [
                "",
                "Version",
                "Game version",
                "Platform",
                "Published",
                "Downloads",
                "",
            ];
            // The header row: h-12, bg-surface-3, cells at pr-2 first:pl-4 last:pr-4.
            let header = Rect::from_min_size(table.min, egui::vec2(table.width(), TABLE_HEADER));
            let mut x = table.left();
            for (index, (label, width)) in labels.iter().zip(&columns).enumerate() {
                let cell = Rect::from_min_size(
                    egui::pos2(x, header.top()),
                    egui::vec2(*width, TABLE_HEADER),
                );
                if !label.is_empty() {
                    painter.text(
                        egui::pos2(cell.left() + if index == 0 { TABLE_PAD } else { 0.0 }, cell.center().y),
                        Align2::LEFT_CENTER,
                        *label,
                        fonts::semibold(TAG_TEXT),
                        theme.SECONDARY,
                    );
                }
                x += width;
            }
            // The rule under the header.
            painter.rect_filled(
                Rect::from_min_size(
                    egui::pos2(table.left(), header.bottom() - STROKE_WIDTH),
                    egui::vec2(table.width(), STROKE_WIDTH),
                ),
                0,
                theme.SURFACE_4,
            );

            for (row, version) in shown.iter().enumerate() {
                let top = header.bottom() + row as f32 * TABLE_ROW;
                let band = Rect::from_min_size(
                    egui::pos2(table.left(), top),
                    egui::vec2(table.width(), TABLE_ROW),
                );
                // Each body row carries a border-t border-surface-4 (base/Table.vue:100-110).
                painter.rect_filled(
                    Rect::from_min_size(
                        egui::pos2(band.left(), band.top()),
                        egui::vec2(band.width(), STROKE_WIDTH),
                    ),
                    0,
                    theme.SURFACE_4,
                );

                let mut x = band.left();
                // The channel: a VersionChannelIndicator, which is a dot in the channel's ink.
                let channel = Rect::from_min_size(
                    egui::pos2(x, band.top()),
                    egui::vec2(columns[0], TABLE_ROW),
                );
                // **A circled letter, as the reference draws it** (`VersionChannelIndicator`): the channel\'s ink
                // under the channel\'s initial in the page\'s background tone. This was a bare 5-point dot, which
                // says "a channel" but not which one at a glance.
                // **A tinted disc with the letter in the channel\'s ink** -- not the inverse. The first version drew
                // the disc in the ink and the letter dark, which is a loud badge where the reference has a quiet one
                // (its `B` is the orange and its ground is that orange at a fifth over the row).
                let (ink, rgb) = match version.channel {
                    Channel::Release => (theme.GREEN, (0x1b, 0xd9, 0x6a)),
                    Channel::Beta => (theme.ORANGE, (0xe0, 0x8b, 0x3e)),
                    Channel::Alpha => (theme.RED, (0xd0, 0x3a, 0x3a)),
                };
                painter.circle_filled(
                    channel.center(),
                    11.0,
                    crate::shell::tint(theme.BG, rgb, 0.30),
                );
                painter.text(
                    channel.center(),
                    Align2::CENTER_CENTER,
                    &version.channel.label()[..1],
                    fonts::semibold(12.0),
                    ink,
                );
                x += columns[0];

                // The version number: font-medium text-contrast.
                let name = Rect::from_min_size(
                    egui::pos2(x, band.top()),
                    egui::vec2(columns[1], TABLE_ROW),
                );
                painter.text(
                    egui::pos2(name.left(), name.center().y),
                    Align2::LEFT_CENTER,
                    &version.number,
                    fonts::label(SUMMARY_SIZE),
                    theme.CONTRAST,
                );
                x += columns[1];

                // The game versions, as chips, clipped to the column's 12rem.
                let games = Rect::from_min_size(
                    egui::pos2(x, band.top()),
                    egui::vec2(columns[2], TABLE_ROW),
                );
                chip_run(
                    painter,
                    games,
                    theme,
                    &version.game_versions,
                );
                x += columns[2];

                // The platforms, the same way.
                let platforms = Rect::from_min_size(
                    egui::pos2(x, band.top()),
                    egui::vec2(columns[3], TABLE_ROW),
                );
                chip_run(painter, platforms, theme, &version.loaders);
                x += columns[3];

                // Published: relative time, font-medium.
                let published = Rect::from_min_size(
                    egui::pos2(x, band.top()),
                    egui::vec2(columns[4], TABLE_ROW),
                );
                painter.text(
                    egui::pos2(published.left(), published.center().y),
                    Align2::LEFT_CENTER,
                    relative_short(version.published, now),
                    fonts::label(TAG_TEXT),
                    theme.BASE,
                );
                x += columns[4];

                // Downloads: compact, font-medium.
                let downloads = Rect::from_min_size(
                    egui::pos2(x, band.top()),
                    egui::vec2(columns[5], TABLE_ROW),
                );
                painter.text(
                    egui::pos2(downloads.left(), downloads.center().y),
                    Align2::LEFT_CENTER,
                    compact(version.downloads),
                    fonts::label(TAG_TEXT),
                    theme.BASE,
                );
            }
        });
        // The open menu: the same toggles the chips carried, and none of them until it is opened.
        if let Some(open) = state.open_filter {
            let anchor = match open {
                FilterKind::Channel => channel_rect,
                FilterKind::Version => version_rect,
            };
            let items: Vec<String> = match open {
                FilterKind::Channel => ["All", "Release", "Beta", "Alpha"]
                    .iter()
                    .map(|name| (*name).to_owned())
                    .collect(),
                FilterKind::Version => {
                    let mut names: Vec<String> = Vec::new();
                    for version in &project.versions {
                        for name in &version.game_versions {
                            if !names.contains(name) {
                                names.push(name.clone());
                            }
                        }
                    }
                    names.sort_by(|a, b| b.cmp(a));
                    let mut all = vec!["All".to_owned()];
                    all.extend(names);
                    all
                }
            };
            if let Some(picked) = filter_menu(ui, anchor, inner, theme, &items) {
                match open {
                    FilterKind::Channel => match picked.as_str() {
                        "All" => state.channels.clear(),
                        name => {
                            state.channels.clear();
                            if let Some(channel) = [Channel::Release, Channel::Beta, Channel::Alpha]
                                .into_iter()
                                .find(|channel| channel.label() == name)
                            {
                                state.channels.push(channel);
                            }
                        }
                    },
                    FilterKind::Version => {
                        state.game_version = if picked == "All" { None } else { Some(picked) };
                    }
                }
                state.open_filter = None;
                ui.ctx().request_repaint();
            }
        }

}

/// A run of `TagItem` chips, one row, clipped to the column they are given.

/// Which of the Versions tab\'s two comboboxes is open.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FilterKind {
    Channel,
    Version,
}

/// One `Combobox` trigger: `h-9 rounded-xl px-2.5` with an optional 20-point prefix and a chevron
/// (`ButtonFrame.vue:31`, `Combobox.vue:80-93`). Returns the rect it used.
fn filter_trigger(
    painter: &egui::Painter,
    min: egui::Pos2,
    theme: &Theme,
    glyph: Option<Glyph>,
    label: &str,
    open: bool,
) -> Rect {
    let galley = painter.layout_no_wrap(
        label.to_owned(),
        fonts::semibold(TAG_TEXT),
        theme.CONTRAST,
    );
    let prefix = if glyph.is_some() { TAG_HEIGHT } else { 0.0 };
    let width = 2.0 * 10.0 + prefix + galley.size().x + 6.0 + 14.0;
    let rect = Rect::from_min_size(min, egui::vec2(width, TAG_HEIGHT));
    painter.rect_filled(
        rect,
        (TAG_HEIGHT / 2.0) as u8,
        if open { theme.BUTTON_BG_SELECTED } else { theme.BUTTON_BG },
    );
    let mut x = rect.left() + 10.0;
    if let Some(glyph) = glyph {
        icons::draw(
            painter,
            glyph,
            Rect::from_center_size(
                egui::pos2(x + TAG_HEIGHT / 2.0, rect.center().y),
                Vec2::splat(TAG_HEIGHT),
            ),
            theme.BASE,
        );
        x += TAG_HEIGHT;
    }
    painter.text(
        egui::pos2(x, rect.center().y),
        Align2::LEFT_CENTER,
        label,
        fonts::semibold(TAG_TEXT),
        if open { theme.BUTTON_TEXT_SELECTED } else { theme.CONTRAST },
    );
    // The suffix row is a 20-point chevron from the icon set, not a font character: `⌄` is not in the shipped
    // faces and drew as a missing-glyph box, which is the one thing a control must never show.
    icons::draw(
        painter,
        Glyph::ChevronDown,
        Rect::from_center_size(
            egui::pos2(rect.right() - 18.0, rect.center().y),
            Vec2::splat(16.0),
        ),
        theme.BASE,
    );
    rect
}

/// The menu under an open trigger: one row per item, the pressed one returned. Rows are hit-tested
/// against the pointer rather than registered, so nothing has to be remembered between frames.
fn filter_menu(
    ui: &mut Ui,
    anchor: Rect,
    page: Rect,
    theme: &Theme,
    items: &[String],
) -> Option<String> {
    let painter = ui.painter();
    let row_h = 32.0;
    let pad = 6.0;
    let width = anchor.width().max(160.0);
    let height = items.len() as f32 * row_h + 2.0 * pad;
    let mut min = egui::pos2(anchor.left(), anchor.bottom() + 6.0);
    if min.y + height > page.bottom() {
        min.y = (anchor.top() - 6.0 - height).max(page.top());
    }
    let menu = Rect::from_min_size(min, egui::vec2(width, height));
    painter.rect_filled(menu, 8, theme.SUPER_RAISED_BG);
    let pointer = ui.ctx().pointer_interact_pos();
    let mut picked = None;
    for (index, item) in items.iter().enumerate() {
        let row = Rect::from_min_size(
            egui::pos2(menu.left() + pad, menu.top() + pad + index as f32 * row_h),
            egui::vec2(menu.width() - 2.0 * pad, row_h),
        );
        let hovered = pointer.is_some_and(|at| row.contains(at));
        if hovered {
            painter.rect_filled(row, 6, theme.RAISED_BG);
        }
        painter.text(
            egui::pos2(row.left() + 8.0, row.center().y),
            Align2::LEFT_CENTER,
            item,
            fonts::label(TAG_TEXT),
            theme.CONTRAST,
        );
        if hovered && ui.input(|input| input.pointer.any_click()) {
            picked = Some(item.clone());
        }
    }
    picked
}
/// The mark a loader carries in a table row, by the name the API gives it. An unknown loader gets none rather than a
/// wrong one, which is the same rule the category rows follow.
fn loader_glyph(name: &str) -> Option<crate::icons::Glyph> {
    use crate::icons::Glyph;
    match name.to_lowercase().as_str() {
        "fabric" => Some(Glyph::TagLoaderFabric),
        "forge" => Some(Glyph::TagLoaderForge),
        _ => None,
    }
}

fn chip_run(painter: &egui::Painter, column: Rect, theme: &Theme, labels: &[String]) {
    let painter = painter.with_clip_rect(column);
    let mut x = column.left();
    for label in labels {
        // **The reference\'s platform chip is a name, not an id.** It reads `Fabric`, capitalised, with the loader\'s
        // own mark; this column printed the API\'s `fabric` bare, which reads as a filter value rather than a platform.
        let pretty = match label.chars().next() {
            Some(first) => first.to_uppercase().collect::<String>() + &label[first.len_utf8()..],
            None => label.clone(),
        };
        let chip = tag_rect(
            &painter,
            egui::pos2(x, column.center().y - TAG_HEIGHT / 2.0),
            theme,
            pretty.as_str(),
            loader_glyph(label),
        );
        x = chip.right() + TAG_GAP;
        if x > column.right() {
            break;
        }
    }
}


// ---------------------------------------------------------------------------
// The Gallery tab
// ---------------------------------------------------------------------------

/// The gallery: repeat(auto-fill, minmax(20rem, 1fr)) (`Gallery.vue:91-96`), each item a card
/// whose image is 2:1 and whose body and foot carry the title, the description and the date.
fn gallery(
    ui: &mut Ui,
    inner: Rect,
    theme: &Theme,
    project: &Project,
    images: &mut Images,
    state: &mut State,
    probe_rows: &mut Vec<(String, Rect)>,
) {
    if project.gallery.is_empty() {
        empty_tab(ui, inner, theme, "No images yet.");
        return;
    }
    let (columns, width) = gallery_grid(inner.width());
    let image_height = width * GALLERY_ASPECT;
    let title_line = 24.0;
    let description_line = 24.0;
    let body_height = GALLERY_BODY_PAD + title_line + TAG_GAP
        + 2.0 * description_line
        + GALLERY_BODY_PAD;
    let foot_height = 24.0 + GALLERY_BODY_PAD;
    let item_height = image_height + body_height + foot_height;
    let rows = project.gallery.len().div_ceil(columns);
    let content_height = rows as f32 * item_height
        + rows.saturating_sub(1) as f32 * GALLERY_GAP;

    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner));
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(&mut child, |ui| {
            let (area, _) = ui.allocate_exact_size(
                egui::vec2(inner.width(), content_height),
                Sense::hover(),
            );
            for (index, image) in project.gallery.iter().enumerate() {
                let column = index % columns;
                let row = index / columns;
                let item = Rect::from_min_size(
                    egui::pos2(
                        area.left() + column as f32 * (width + GALLERY_GAP),
                        area.top() + row as f32 * (item_height + GALLERY_GAP),
                    ),
                    egui::vec2(width, item_height),
                );
                let painter = ui.painter();
                painter.rect_filled(item, CARD_RADIUS, theme.SURFACE_3);
                painter.rect_stroke(
                    item,
                    CARD_RADIUS,
                    Stroke::new(STROKE_WIDTH, theme.SURFACE_4),
                    egui::StrokeKind::Inside,
                );

                // The image: aspect-ratio 2/1, object-fit cover.
                let picture = Rect::from_min_size(
                    item.min,
                    egui::vec2(item.width(), image_height),
                );
                probe_rows.push((format!("gallery {index}"), item));
                probe_rows.push((format!("gallery-picture {index}"), picture));
                let clipped = painter.with_clip_rect(picture);
                match images.get_picture(ui.ctx(), &image.url) {
                    Some(texture) => {
                        let uv = cover_uv(texture.size_vec2(), picture.size());
                        clipped.image(texture.id(), picture, uv, Color32::WHITE);
                    }
                    None => {
                        clipped.rect_filled(picture, 0, theme.BUTTON_BG);
                    }
                }

                // The body: p-4 with the title and the description.
                let body = Rect::from_min_size(
                    egui::pos2(item.left() + GALLERY_BODY_PAD, picture.bottom() + GALLERY_BODY_PAD),
                    egui::vec2(item.width() - 2.0 * GALLERY_BODY_PAD, body_height),
                );
                painter.text(
                    body.min,
                    Align2::LEFT_TOP,
                    image.title.as_str(),
                    fonts::semibold(GALLERY_TITLE),
                    theme.CONTRAST,
                );
                let description = clamp_to_lines(
                    &painter,
                    &image.description,
                    fonts::body(GALLERY_BODY),
                    theme.SECONDARY,
                    body.width(),
                    2,
                );
                painter.galley(
                    egui::pos2(body.left(), body.top() + title_line + TAG_GAP),
                    description,
                    theme.SECONDARY,
                );

                // The foot: a CalendarIcon and the date (`Gallery.vue:105-120`).
                let foot_y = item.bottom() - GALLERY_BODY_PAD - 20.0;
                icons::draw(
                    &painter,
                    Glyph::Calendar,
                    Rect::from_min_size(
                        egui::pos2(item.left() + GALLERY_BODY_PAD, foot_y + 2.0),
                        Vec2::splat(GALLERY_ICON),
                    ),
                    theme.SECONDARY,
                );
                painter.text(
                    egui::pos2(
                        item.left() + GALLERY_BODY_PAD + GALLERY_ICON + TAG_GAP,
                        foot_y + 10.0,
                    ),
                    Align2::LEFT_CENTER,
                    image.created.long(),
                    fonts::body(GALLERY_DATE),
                    theme.SECONDARY,
                );

                // A press anywhere on the item opens it (`Gallery.vue:4-6`).
                if ui
                    .interact(
                        item,
                        egui::Id::new(("gallery-item", index)),
                        Sense::click(),
                    )
                    .clicked()
                {
                    state.viewer = Some(index);
                    ui.ctx().request_repaint();
                }
            }
        });
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

/// The gallery's viewer: the picture at full size over the page, with the title and the
/// description under it, closed by Escape, the close button or a press outside the picture.
fn viewer(
    ui: &mut Ui,
    page: Rect,
    theme: &Theme,
    images: &mut Images,
    state: &mut State,
    project: &Project,
) -> bool {
    let Some(index) = state.viewer else {
        return false;
    };
    let Some(image) = project.gallery.get(index) else {
        state.viewer = None;
        return false;
    };

    let painter = ui.painter();
    painter.rect_filled(page, 0, Color32::from_black_alpha(216));
    let box_rect = Rect::from_min_max(
        egui::pos2(page.left() + 48.0, page.top() + 48.0),
        egui::pos2(page.right() - 48.0, page.bottom() - 96.0),
    );
    let mut picture = box_rect;
    if let Some(texture) = images.get_picture(ui.ctx(), &image.url) {
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
    painter.text(
        egui::pos2(box_rect.center().x, box_rect.bottom() + 16.0),
        Align2::CENTER_TOP,
        format!("{}  ·  {} of {}", image.title, index + 1, project.gallery.len()),
        fonts::body(TAG_TEXT),
        theme.TEXT_PRIMARY,
    );

    let close = Rect::from_center_size(
        egui::pos2(page.right() - PAGE_PADDING - 18.0, page.top() + PAGE_PADDING + 18.0),
        Vec2::splat(36.0),
    );
    painter.circle_filled(close.center(), 18.0, theme.SURFACE_3);
    icons::draw(painter, Glyph::X, close.shrink(10.0), theme.CONTRAST);

    if ui
        .interact(close, egui::Id::new("gallery-viewer-close"), Sense::click())
        .clicked()
    {
        state.viewer = None;
        return true;
    }
    let (clicked, pointer, escape, step) = ui.input(|input| {
        let step = if input.key_pressed(egui::Key::ArrowRight) {
            1_i32
        } else if input.key_pressed(egui::Key::ArrowLeft) {
            -1_i32
        } else {
            0_i32
        };
        (
            input.pointer.primary_clicked(),
            input.pointer.interact_pos(),
            input.key_pressed(egui::Key::Escape),
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
    if step != 0 && !project.gallery.is_empty() {
        let next = (index as i32 + step).clamp(0, project.gallery.len() as i32 - 1);
        state.viewer = Some(next as usize);
        return true;
    }
    false
}

/// A tab with nothing in it yet, drawn so the card does not look broken.
fn empty_tab(ui: &mut Ui, inner: Rect, theme: &Theme, message: &str) {
    ui.painter().text(
        inner.center(),
        Align2::CENTER_CENTER,
        message,
        fonts::body(SUMMARY_SIZE),
        theme.SECONDARY,
    );
}


// ---------------------------------------------------------------------------
// The sidebar
// ---------------------------------------------------------------------------

/// The sections this page contributes to the right panel.
///
/// The reference's `ProjectSidebarCompatibility`: the game versions as chips, then the
/// platform, then the supported environment (`ProjectSidebarCompatibility.vue:2-8`), with the
/// h2 at `text-lg` (18) and the h3 at `text-primary !font-normal text-base` (16, normal).
pub fn panel(ui: &mut Ui, rect: Rect, theme: &Theme, project: &Project) -> f32 {
    // The sidebar is drawn from the shell's own copy of the project, so it reads the document
    // through the same cache the page filled.
    let loaded = merged(ui.ctx(), project);
    let project = &loaded;
    let mut y = rect.top();
    let inner_width = rect.width() - 2.0 * CARD_PAD;

    // Each section is a SidebarCard: flex flex-col gap-3 p-4 pt-3, card-shadow mb-4 rounded-2xl
    // bg-surface-3 border border-surface-4, h2 at text-lg font-semibold (SidebarCard.vue:11-18).
    // **Getting started sits above the Compatibility card on this page**, as the reference\'s own project capture
    // shows -- and *only* those two, with no News card and no hosting banner. The card is `panel`\'s
    // `project_getting_started_card`, which is **not** the one Home draws: they are two implementations and they
    // can drift. (This comment claimed they were shared; a round of edits to the wrong one proved otherwise.)
    {
        let has_instance = std::fs::read_dir(crate::data::instances_dir())
            .map(|read| read.flatten().any(|entry| entry.path().is_dir()))
            .unwrap_or(false);
        let (used, _pressed) = crate::panel::project_getting_started_card(
            ui,
            Rect::from_min_size(egui::pos2(rect.left(), y), egui::vec2(rect.width(), 0.0)),
            theme,
            theme.RAISED_BG,
            theme.SURFACE_5,
            [has_instance, false, false],
            ["Create first instance", "Sign in to Minecraft", "Sign in to Modrinth"],
        );
        // **Ask the program, not a crop.** The card's own height is invisible against its rows' rings in a capture:
        // three proxies gave 218, 215 and 234. `SIFT_PANEL_PROBE=1` prints what was actually laid out.
        if std::env::var_os("SIFT_PANEL_PROBE").is_some() {
            eprintln!(
                "panel card getting-started y={y:.1} h={used:.1} width={:.1}",
                rect.width()
            );
        }
        y += used;
    }
    let painter = ui.painter();
    let painter = &painter;
    let cards = [
        ("Compatibility", compatibility_rows(project)),
        ("Links", links_rows(project)),
        ("Creators", creator_rows(project)),
        ("Details", detail_rows(project)),
    ];
    for (title, rows) in cards {
        if rows.is_empty() {
            continue;
        }
        // The rows are placed before the card is drawn, because the card's height is what the
        // placement adds up to -- the same two passes flex-wrap needs.
        let placed = place_rows(painter, &rows, inner_width);
        let height = CARD_PAD_TOP + CARD_HEADING_LINE + CARD_CONTENT_GAP + placed.height + CARD_PAD;
        let card = Rect::from_min_size(
            egui::pos2(rect.left(), y),
            egui::vec2(rect.width(), height),
        );
        painter.rect_filled(card, CARD_RADIUS, theme.SURFACE_3);
        painter.rect_stroke(
            card,
            CARD_RADIUS,
            Stroke::new(STROKE_WIDTH, theme.SURFACE_4),
            egui::StrokeKind::Inside,
        );
        let inner_left = card.left() + CARD_PAD;
        painter.text(
            egui::pos2(inner_left, card.top() + CARD_PAD_TOP),
            Align2::LEFT_TOP,
            title,
            fonts::semibold(SIDEBAR_HEADING),
            theme.CONTRAST,
        );
        let content_top = card.top() + CARD_PAD_TOP + CARD_HEADING_LINE + CARD_CONTENT_GAP;
        for (row, x, row_y) in &placed.rows {
            let _ = rows[*row].draw(
                painter,
                theme,
                inner_left + *x,
                content_top + *row_y,
                inner_width,
            );
        }
        y = card.bottom() + CARD_GAP;
    }

    y - rect.top()
}

/// Where each row goes, and how tall the run is: the result of wrapping the chips at the card's
/// width, which is what flex-wrap does in the sheet.
struct Placed {
    /// The rows, as (index into the source, x offset from the card's content edge, y).
    rows: Vec<(usize, f32, f32)>,
    /// The height the run used.
    height: f32,
}

/// Place a card's rows at the width it will be drawn in.
fn place_rows(painter: &egui::Painter, rows: &[Row], width: f32) -> Placed {
    let mut placed = Vec::new();
    let mut y = 0.0;
    let mut x = 0.0;
    let mut line_height = 0.0_f32;
    for (index, row) in rows.iter().enumerate() {
        match row {
            Row::Break => {
                y += line_height.max(SIDEBAR_CHIP_HEIGHT + SIDEBAR_CHIP_GAP);
                line_height = 0.0;
                x = 0.0;
            }
            Row::Chip(_, _) => {
                let chip_width = row.width(painter);
                if x > 0.0 && x + chip_width > width {
                    y += line_height.max(SIDEBAR_CHIP_HEIGHT + SIDEBAR_CHIP_GAP);
                    line_height = 0.0;
                    x = 0.0;
                }
                placed.push((index, x, y));
                x += chip_width + SIDEBAR_CHIP_GAP;
                line_height = line_height.max(SIDEBAR_CHIP_HEIGHT + SIDEBAR_CHIP_GAP);
            }
            Row::Line(_, _) => {
                if line_height > 0.0 {
                    y += line_height;
                    line_height = 0.0;
                    x = 0.0;
                }
                placed.push((index, 0.0, y));
                y += SIDEBAR_LINE;
            }
        }
    }
    y += line_height;
    Placed { rows: placed, height: y }
}

/// One row inside a sidebar card: a line of label and value, or one chip.
enum Row {
    /// A label and its value, the value after the label.
    Line(String, String),
    /// A chip, drawn at the sheet's chip geometry.
    Chip(String, Option<Glyph>),
    /// A wrap: the rows after this start a new line.
    Break,
}

impl Row {
    /// The chip's width at the sidebar's body size.
    fn width(&self, painter: &egui::Painter) -> f32 {
        match self {
            Row::Chip(label, glyph) => {
                let text =
                    painter.layout_no_wrap(label.clone(), fonts::body(SIDEBAR_BODY), Color32::WHITE);
                text.size().x
                    + if glyph.is_some() {
                        TAG_GLYPH + TAG_GAP
                    } else {
                        0.0
                    }
                    + 2.0 * TAG_PAD_X
            }
            _ => 0.0,
        }
    }

    /// Draw it at a point the layout chose, answering the height it used.
    fn draw(&self, painter: &egui::Painter, theme: &Theme, left: f32, top: f32, width: f32) -> f32 {
        match self {
            Row::Line(label, value) => {
                painter.text(
                    egui::pos2(left, top),
                    Align2::LEFT_TOP,
                    label.as_str(),
                    fonts::body(SIDEBAR_BODY),
                    theme.BASE,
                );
                if !value.is_empty() {
                    painter.text(
                        egui::pos2(left + width_of(painter, label) + 8.0, top),
                        Align2::LEFT_TOP,
                        value.as_str(),
                        fonts::body(SIDEBAR_BODY),
                        theme.SECONDARY,
                    );
                }
                SIDEBAR_LINE
            }
            Row::Chip(label, glyph) => {
                let text =
                    painter.layout_no_wrap(label.clone(), fonts::body(SIDEBAR_BODY), theme.BASE);
                let chip = chip_at(
                    painter,
                    egui::pos2(left, top),
                    theme,
                    &text,
                    SIDEBAR_BODY,
                    *glyph,
                );
                chip.height() + SIDEBAR_CHIP_GAP
            }
            Row::Break => 0.0,
        }
    }
}

/// The width of a label in the sidebar's body size.
fn width_of(painter: &egui::Painter, label: &str) -> f32 {
    painter
        .layout_no_wrap(label.to_owned(), fonts::body(SIDEBAR_BODY), Color32::WHITE)
        .size()
        .x
}

/// The compatibility card's rows: the game, the versions as chips, then the platforms and the
/// environments (ProjectSidebarCompatibility.vue:2-8).
fn compatibility_rows(project: &Project) -> Vec<Row> {
    if project.versions.is_empty() && project.game_versions.is_empty() {
        return Vec::new();
    }
    let mut rows = vec![Row::Line("Minecraft: Java Edition".to_owned(), String::new())];
    for version in &project.game_versions {
        rows.push(Row::Chip(version.clone(), None));
    }
    for (heading, wanted) in [
        ("Platforms", ["fabric", "forge", "neoforge", "quilt"].as_slice()),
        (
            "Supported environments",
            ["client", "server"].as_slice(),
        ),
    ] {
        let chips: Vec<Row> = project
            .tags
            .iter()
            .filter(|(tag, _)| {
                let tag = tag.to_lowercase();
                wanted.iter().any(|name| tag.contains(name))
            })
            .map(|(tag, glyph)| Row::Chip(tag.clone(), Some(*glyph)))
            .collect();
        if chips.is_empty() {
            continue;
        }
        rows.push(Row::Line(heading.to_owned(), String::new()));
        rows.extend(chips);
    }
    rows
}

/// The links card: the catalogue's own labels (ProjectSidebarLinks.vue).
fn links_rows(project: &Project) -> Vec<Row> {
    project
        .links
        .iter()
        .map(|link| Row::Chip(link.clone(), None))
        .collect()
}

/// The creators card: a name per row, with its role.
fn creator_rows(project: &Project) -> Vec<Row> {
    project
        .creators
        .iter()
        .map(|(name, role)| Row::Line(name.clone(), role.clone()))
        .collect()
}

/// The details card: the two dates and the licence, each only when the API gave it
/// (ProjectSidebarDetails.vue).
fn detail_rows(project: &Project) -> Vec<Row> {
    let mut rows = Vec::new();
    if let Some(published) = project.published {
        rows.push(Row::Line("Published".to_owned(), published.long()));
    }
    if let Some(updated) = project.updated {
        rows.push(Row::Line("Updated".to_owned(), updated.long()));
    }
    if !project.license.is_empty() {
        rows.push(Row::Line("License".to_owned(), project.license.clone()));
    }
    rows
}

/// The gap between two chips in the sidebar: `gap-1`.
const PANEL_CHIP_GAP: f32 = 4.0;

/// One chip in the sidebar, at the sheet's chip geometry.
fn chip_at(
    painter: &egui::Painter,
    min: egui::Pos2,
    theme: &Theme,
    label: &std::sync::Arc<egui::Galley>,
    height: f32,
    glyph: Option<Glyph>,
) -> Rect {
    let glyph_width = if glyph.is_some() {
        TAG_GLYPH + TAG_GAP
    } else {
        0.0
    };
    let rect = Rect::from_min_size(
        min,
        egui::vec2(label.size().x + glyph_width + 2.0 * TAG_PAD_X, height + 10.0),
    );
    painter.rect_filled(rect, 8, theme.SUPER_RAISED_BG);
    // **A ring, as the reference's chips have.** Without it the fill is a shade off the card's own and the chips read
    // as a run of loose words -- which is exactly how the Compatibility card's versions looked beside the reference's.
    painter.rect_stroke(
        rect,
        8,
        egui::Stroke::new(1.0, theme.SURFACE_5),
        egui::StrokeKind::Inside,
    );
    if let Some(glyph) = glyph {
        icons::draw(
            painter,
            glyph,
            Rect::from_center_size(
                egui::pos2(rect.left() + TAG_PAD_X + TAG_GLYPH / 2.0, rect.center().y),
                Vec2::splat(TAG_GLYPH),
            ),
            theme.BASE,
        );
    }
    painter.galley(
        egui::pos2(
            rect.left() + TAG_PAD_X + glyph_width,
            rect.center().y - label.size().y / 2.0,
        ),
        label.clone(),
        theme.BASE,
    );
    rect
}

/// One sidebar section: a heading, then a label/value row per entry.
///
/// Returns the height it used, so the next section starts where this one ended.
fn section(
    painter: &egui::Painter,
    rect: Rect,
    top: f32,
    theme: &Theme,
    heading: &str,
    rows: &[(&str, &str)],
) -> f32 {
    let mut y = top;
    painter.text(
        egui::pos2(rect.left(), y),
        Align2::LEFT_TOP,
        heading,
        fonts::semibold(18.0),
        theme.CONTRAST,
    );
    y += 24.0;
    for (label, value) in rows {
        painter.text(
            egui::pos2(rect.left(), y),
            Align2::LEFT_TOP,
            *label,
            fonts::body(16.0),
            theme.BASE,
        );
        if !value.is_empty() {
            painter.text(
                egui::pos2(rect.right(), y),
                Align2::RIGHT_TOP,
                *value,
                fonts::body(16.0),
                theme.SECONDARY,
            );
        }
        y += 24.0;
    }
    y - top + 8.0
}

// ---------------------------------------------------------------------------
// The project document
// ---------------------------------------------------------------------------
//
// A card carries the name, the team, the summary, the counts, the tags and the icon -- the
// search response's own fields. Everything else on this page (the long description, the
// gallery, the releases, the game versions the compatibility block lists) only exists in
// `GET /v2/project/{id}` and `GET /v2/project/{id}/version`, which the reference client asks
// for the moment it opens a project.
//
// So this page asks for them too. The request is a thread -- never the frame -- and its answer
// lands in a cache shared by this module, because the shell owns the `Project` the page is
// handed and the sidebar is drawn from that same copy: one cache read by both is what keeps
// the header, the tabs and the panel describing one project.
//
// Three states per id, and no retry inside a run: `Loading`, then `Ready` or `Failed`. A page
// that retried on every frame would ask the API sixty times a second, and a launcher with no
// connection would spin on a request that cannot answer.

/// What the two document endpoints add to a card's own fields.
#[derive(Debug, Clone, Default)]
struct Document {
    /// The description, as blocks.
    blocks: Vec<Block>,
    /// The gallery.
    gallery: Vec<Image>,
    /// The game versions the project supports, collapsed into the ranges the sidebar shows.
    game_versions: Vec<String>,
    /// The releases, newest first.
    versions: Vec<Version>,
    /// `19.8M`, from the project's own `downloads`.
    downloads: String,
    /// `5K`, from its `followers`.
    followers: String,
    /// The icon's URL, which a card may not have carried.
    icon: String,
}

/// Where one project's document is.
enum Fetched {
    /// A worker is out for it.
    Loading,
    /// The document, once it has answered.
    Ready(Box<Document>),
    /// The request or the parse failed. Not retried this run.
    Failed,
}

/// The documents this build has fetched, by project id.
fn documents() -> &'static Mutex<HashMap<String, Fetched>> {
    static DOCUMENTS: OnceLock<Mutex<HashMap<String, Fetched>>> = OnceLock::new();
    DOCUMENTS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// The document for `id`, starting its fetch the first time the page asks for it.
fn document(ctx: &egui::Context, id: &str) -> Option<Document> {
    if id.trim().is_empty() {
        return None;
    }
    {
        let mut map = match documents().lock() {
            Ok(map) => map,
            Err(_) => return None,
        };
        match map.get(id) {
            Some(Fetched::Ready(document)) => return Some((**document).clone()),
            Some(Fetched::Loading) | Some(Fetched::Failed) => return None,
            None => {}
        }
        map.insert(id.to_owned(), Fetched::Loading);
    }
    let id = id.to_owned();
    let ctx = ctx.clone();
    std::thread::spawn(move || {
        let answer = load_document(&id);
        if let Ok(mut map) = documents().lock() {
            map.insert(
                id,
                match answer {
                    Ok(document) => Fetched::Ready(Box::new(document)),
                    Err(_) => Fetched::Failed,
                },
            );
        }
        // The frame that draws the answer is the one the answer asked for: without this the page
        // would keep showing the card's own fields until something else moved.
        ctx.request_repaint();
    });
    None
}

/// Fetch one project's document: the body, the gallery, the releases and the compatibility
/// fields, off the frame thread.
fn load_document(id: &str) -> Result<Document, String> {
    let paths = SiftPaths::at(platform_data_root(System::current()));
    let wire = Wire::new(paths.meta_dir());
    let body = wire.document(&sift_net::modrinth::project_url(id))?;
    let project: sift_net::modrinth::ModrinthProject =
        serde_json::from_str(&body).map_err(|error| format!("{id}: {error}"))?;
    // The typed model carries what this page's own struct wants. The gallery's `created` stamp
    // is read from the same document's JSON, because the model does not keep it and a gallery
    // with no date would print 1970 under every picture.
    let raw: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);

    let gallery = raw["gallery"]
        .as_array()
        .map(|images| {
            images
                .iter()
                .map(|image| Image {
                    url: image["url"].as_str().unwrap_or_default().to_owned(),
                    title: image["title"].as_str().unwrap_or_default().to_owned(),
                    description: image["description"].as_str().unwrap_or_default().to_owned(),
                    created: image["created"]
                        .as_str()
                        .and_then(date_of)
                        .unwrap_or(Date { year: 1970, month: 1, day: 1 }),
                })
                .collect()
        })
        .unwrap_or_default();

    let mut versions = Vec::new();
    if let Ok(list) = wire.document(&sift_net::modrinth::version_url(id)) {
        if let Ok(entries) = serde_json::from_str::<serde_json::Value>(&list) {
            for entry in entries.as_array().into_iter().flatten() {
                let Some(number) = entry["version_number"].as_str() else {
                    continue;
                };
                versions.push(Version {
                    number: number.to_owned(),
                    name: entry["name"].as_str().unwrap_or_default().to_owned(),
                    game_versions: strings(&entry["game_versions"]),
                    loaders: strings(&entry["loaders"]),
                    channel: match entry["version_type"].as_str() {
                        Some("release") => Channel::Release,
                        Some("beta") => Channel::Beta,
                        _ => Channel::Alpha,
                    },
                    published: entry["date_published"]
                        .as_str()
                        .and_then(date_of)
                        .unwrap_or(Date { year: 1970, month: 1, day: 1 }),
                    downloads: entry["downloads"].as_u64().unwrap_or(0),
                });
            }
        }
    }

    Ok(Document {
        blocks: blocks_of(&project.body),
        gallery,
        game_versions: group_game_versions(&project.game_versions),
        versions,
        downloads: short_count(project.downloads),
        followers: short_count(project.followers),
        icon: raw["icon_url"].as_str().unwrap_or_default().to_owned(),
    })
}

/// A JSON array of strings, as `game_versions` and `loaders` are written.
fn strings(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

/// The date in an ISO-8601 stamp's first ten characters: `2026-10-20T...` is October 20, 2026.
fn date_of(stamp: &str) -> Option<Date> {
    let mut parts = stamp.get(..10)?.split('-');
    let year = parts.next()?.parse().ok()?;
    let month = parts.next()?.parse().ok()?;
    let day = parts.next()?.parse().ok()?;
    Some(Date { year, month, day })
}

/// The game versions collapsed the way the sidebar lists them: one chip per minor series,
/// `1.21.x` for a series that starts at patch zero, `1.18.1-1.18.2` for two versions that do not,
/// and the version itself when it stands alone. Newest first, as the reference prints them.
fn group_game_versions(versions: &[String]) -> Vec<String> {
    let mut parsed: Vec<(i64, i64, i64, String)> = versions
        .iter()
        .filter_map(|version| {
            let mut parts = version.split('.');
            let major = parts.next()?.parse().ok()?;
            let minor = parts.next().unwrap_or("0").parse().unwrap_or(0);
            let patch = parts.next().unwrap_or("0").parse().unwrap_or(0);
            Some((major, minor, patch, version.clone()))
        })
        .collect();
    parsed.sort_by(|a, b| (b.0, b.1, b.2).cmp(&(a.0, a.1, a.2)));

    let mut out = Vec::new();
    let mut index = 0;
    while index < parsed.len() {
        let (major, minor, ..) = parsed[index];
        let mut end = index;
        while end + 1 < parsed.len() && parsed[end + 1].0 == major && parsed[end + 1].1 == minor {
            end += 1;
        }
        if end == index {
            out.push(parsed[index].3.clone());
        } else if parsed[end].2 == 0 {
            out.push(format!("{major}.{minor}.x"));
        } else {
            out.push(format!(
                "{major}.{minor}.{}-{major}.{minor}.{}",
                parsed[end].2, parsed[index].2
            ));
        }
        index = end + 1;
    }
    // A version the numeric parse could not read (a snapshot name, say) is still one the project
    // supports: it goes on the end rather than being dropped.
    for version in versions {
        if !parsed.iter().any(|(.., text)| text == version) {
            out.push(version.clone());
        }
    }
    out
}

/// The markdown body as blocks.
fn blocks_of(body: &str) -> Vec<Block> {
    let mut out = Vec::new();
    for chunk in body.split("\n\n") {
        let lines: Vec<&str> = chunk.lines().filter(|line| !line.trim().is_empty()).collect();
        if lines.is_empty() {
            continue;
        }
        // A chunk of nothing but images is the badge row at the top of most packs' pages.
        if lines
            .iter()
            .all(|line| line.trim().starts_with("[![") || line.trim().starts_with("!["))
        {
            for line in &lines {
                let (urls, _) = split_images(line.trim());
                out.extend(urls.into_iter().map(Block::Image));
            }
            continue;
        }
        if lines.iter().all(|line| line.trim_start().starts_with('>')) {
            for line in &lines {
                let text = inline_text(line.trim_start().trim_start_matches('>').trim());
                if !text.is_empty() {
                    out.push(Block::Quote(text));
                }
            }
            continue;
        }
        if lines[0].trim_start().starts_with('#') {
            let heading = lines[0].trim_start().trim_start_matches('#').trim();
            out.push(Block::Heading(inline_text(heading)));
            continue;
        }
        let joined = lines
            .iter()
            .map(|line| inline_text(line.trim()))
            .collect::<Vec<_>>()
            .join(" ");
        if !joined.trim().is_empty() {
            out.push(Block::Text(joined));
        }
    }
    out
}

/// Split a line into the URLs of its images, and what is left of it.
fn split_images(line: &str) -> (Vec<String>, String) {
    let mut urls = Vec::new();
    let mut rest = String::new();
    let mut index = 0;
    while index < line.len() {
        let tail = &line[index..];
        if let Some(after) = tail.strip_prefix("![") {
            if let Some(close) = after.find("](") {
                let start = close + 2;
                if let Some(end) = after[start..].find(')') {
                    urls.push(after[start..start + end].to_owned());
                    index += 2 + start + end + 1;
                    continue;
                }
            }
        }
        let ch = tail.chars().next().unwrap_or_default();
        rest.push(ch);
        index += ch.len_utf8();
    }
    (urls, rest)
}

/// A line's text with its markdown markers removed: links keep their words, emphasis its
/// letters, and an image disappears (the blocks above have already taken the ones that are a
/// line of their own).
fn inline_text(line: &str) -> String {
    let (_, mut text) = split_images(line);
    let mut out = String::new();
    let mut rest = text.as_str();
    while let Some(start) = rest.find('[') {
        out.push_str(&rest[..start]);
        let after = &rest[start + 1..];
        match after.find(']') {
            Some(end) => {
                out.push_str(&after[..end]);
                let tail = &after[end + 1..];
                // Drop the target of the link, whatever shape it takes: `(url)` or `[1]`.
                let tail = if let Some(stripped) = tail.strip_prefix('(') {
                    stripped.find(')').map_or(stripped, |at| &stripped[at + 1..])
                } else if let Some(stripped) = tail.strip_prefix('[') {
                    stripped.find(']').map_or(stripped, |at| &stripped[at + 1..])
                } else {
                    tail
                };
                rest = tail;
            }
            None => {
                out.push_str(&rest[start..]);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    text = out;
    text.replace("**", "")
        .replace("__", "")
        .replace('*', "")
        .replace('`', "")
        .replace("\\", "")
        .trim()
        .to_owned()
}

/// The page's project with everything its document has added to it.
///
/// Cloned rather than mutated: the shell owns the `Project` the page was handed, and a page that
/// wrote into it would be changing the shell's state from inside a draw.
fn merged(ctx: &egui::Context, project: &Project) -> Project {
    let Some(document) = document(ctx, &project.slug) else {
        return project.clone();
    };
    let mut merged = project.clone();
    if !document.blocks.is_empty() {
        merged.blocks = document.blocks;
    }
    if !document.gallery.is_empty() {
        merged.gallery = document.gallery;
    }
    if !document.game_versions.is_empty() {
        merged.game_versions = document.game_versions;
    }
    if !document.versions.is_empty() {
        merged.versions = document.versions;
    }
    if !document.downloads.is_empty() {
        merged.downloads = document.downloads;
    }
    if !document.followers.is_empty() {
        merged.followers = document.followers;
    }
    if merged.icon.is_empty() {
        merged.icon = document.icon;
    }
    merged
}

/// The project the Discover list's first card opens, so the page can be captured against the
/// reference's own first card.
pub fn sample() -> Project {
    Project {
        title: "Fabulously Optimized".to_owned(),
        author: "Fabulously Optimized".to_owned(),
        summary: "Beautiful graphics, speedy performance and familiar features in a simple package. Wilderness Bound beta!"
            .to_owned(),
        blocks: Vec::new(),
        body: vec![
            "A simple Minecraft modpack focusing on performance and graphics enhancements.".to_owned(),
            "Fabulously Optimized is faster than competing game clients, thanks to modern performance mods like Sodium, Lithium, and others. The bundled mods have been carefully selected to be safe in your worlds and provide noticeable speed benefits on any device."
                .to_owned(),
        ],
        tags: vec![
            ("Lightweight".to_owned(), Glyph::Sparkles),
            ("Multiplayer".to_owned(), Glyph::Users),
            ("Optimization".to_owned(), Glyph::Rocket),
            ("Fabric".to_owned(), Glyph::Box),
            ("Client-side".to_owned(), Glyph::Monitor),
        ],
        game_versions: ["26.3", "26.2", "26.1.x", "1.21.x", "1.20.x", "1.19.x", "1.18.1-1.18.2", "1.17.1", "1.16.5"]
            .iter()
            .map(|version| (*version).to_owned())
            .collect(),
        // The number alone: the header composes "19.6M downloads" from it, the same way it does
        // for a project opened from a card.
        downloads: "19.6M".to_owned(),
        followers: "5.0K".to_owned(),
        icon: "https://cdn.modrinth.com/data/1KVo5zza/d8152911f8fd5d7e9a8c499fe89045af81fe816e_96.webp".to_owned(),
        links: vec![
            "View source".to_owned(),
            "Report issues".to_owned(),
            "Visit wiki".to_owned(),
            "Join Discord".to_owned(),
        ],
        creators: vec![
            ("Robitobi01".to_owned(), "Owner".to_owned()),
            ("unmojang".to_owned(), "Maintainer".to_owned()),
            ("trimbe".to_owned(), "Maintainer".to_owned()),
        ],
        published: Some(Date { year: 2022, month: 2, day: 10 }),
        updated: Some(Date { year: 2026, month: 10, day: 20 }),
        license: "BSD-3-Clause".to_owned(),
        slug: "fabulously-optimized".to_owned(),
        kind: "modpack".to_owned(),
        gallery: vec![
            Image {
                url: "https://cdn.modrinth.com/data/1KVo5zza/images/ce41746dbf8f3d2a9661875c18b5667778233ccf_350.webp".to_owned(),
                title: "Title screen".to_owned(),
                description: "The title screen as the pack leaves it, with the panorama and the quick-play buttons untouched."
                    .to_owned(),
                created: Date { year: 2023, month: 3, day: 4 },
            },
            Image {
                url: "https://cdn.modrinth.com/data/1KVo5zza/images/044a5b9cb85394be95656a1ebcfe135466420da9.png".to_owned(),
                title: "Mod menu".to_owned(),
                description: "Every bundled mod in one list, with the ones that change rendering already configured."
                    .to_owned(),
                created: Date { year: 2023, month: 5, day: 19 },
            },
            Image {
                url: "https://cdn.modrinth.com/data/1KVo5zza/images/9d5ddd3392ce89295caa1030ca930f71f5c6b49a_350.webp".to_owned(),
                title: "FPS in high render distance".to_owned(),
                description: "A 32-chunk render distance at a frame rate the vanilla client does not reach.".to_owned(),
                created: Date { year: 2024, month: 1, day: 8 },
            },
            Image {
                url: "https://cdn.modrinth.com/data/1KVo5zza/images/38f70f307f1a0b17d8ed4526d1636317e8c12d1d_350.webp".to_owned(),
                title: "Sunrise behind a hill".to_owned(),
                description: "Shader-ready lighting with the pack's own sky and fog settings.".to_owned(),
                created: Date { year: 2025, month: 7, day: 21 },
            },
        ],
        versions: vec![
            Version {
                number: "15.0.0-beta.2".to_owned(),
                name: "Wilderness Bound beta 2".to_owned(),
                game_versions: vec!["26.3".to_owned()],
                loaders: vec!["fabric".to_owned()],
                channel: Channel::Beta,
                published: Date { year: 2026, month: 10, day: 20 },
                downloads: 2_184_302,
            },
            Version {
                number: "15.0.0-beta.1".to_owned(),
                name: "Wilderness Bound beta 1".to_owned(),
                game_versions: vec!["26.3".to_owned()],
                loaders: vec!["fabric".to_owned()],
                channel: Channel::Beta,
                published: Date { year: 2026, month: 10, day: 11 },
                downloads: 1_406_881,
            },
            Version {
                number: "14.1.1".to_owned(),
                name: "Maintenance release".to_owned(),
                game_versions: vec!["26.2".to_owned()],
                loaders: vec!["fabric".to_owned()],
                channel: Channel::Release,
                published: Date { year: 2026, month: 10, day: 11 },
                downloads: 8_902_114,
            },
            Version {
                number: "13.6.0".to_owned(),
                name: "Sodium 0.7 update".to_owned(),
                game_versions: vec!["26.1.2".to_owned()],
                loaders: vec!["fabric".to_owned()],
                channel: Channel::Release,
                published: Date { year: 2026, month: 9, day: 28 },
                downloads: 7_203_559,
            },
        ],
    }
}

/// A project page for a card the reader pressed, before the project endpoint has answered.
///
/// The card carries the name, the team, the summary, the tags, the two counts and the icon — the
/// search response's own fields — and nothing else. So those are the page's, and everything that
/// only `GET /v2/project/{id}` has (the description, the gallery, the releases, the links, the
/// creators, the licence and the two dates) starts empty and says so. **An honest empty page
/// beats another project's prose**, which is what showing the sample for every card amounted to.
///
/// The slug is the card's id: Modrinth resolves `modrinth.com/{type}/{id}` the same way it
/// resolves the slug, and a search hit carries no slug.
pub fn from_card(card: &crate::shell::Card) -> Project {
    Project {
        title: card.title.clone(),
        author: card.author.clone(),
        summary: card.summary.clone(),
        body: Vec::new(),
        blocks: Vec::new(),
        tags: card.tags.clone(),
        game_versions: Vec::new(),
        downloads: card.downloads.clone(),
        followers: card.followers.clone(),
        icon: card.icon.clone(),
        links: Vec::new(),
        creators: Vec::new(),
        published: None,
        updated: None,
        license: String::new(),
        slug: card.id.clone(),
        // Discover searches modpacks (the page's own title says so), and the card carries no
        // type of its own; a project page opened from anywhere else keeps the sample's.
        kind: "modpack".to_owned(),
        gallery: Vec::new(),
        versions: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The page defaults to Description, which is what the reference shows on arrival.
    #[test]
    fn a_project_opens_on_its_description() {
        let state = State::default();
        assert_eq!(state.current(&sample()), Tab::Description);
    }

    /// The tabs are in the reference's order and each has a label: a tab that rendered an
    /// empty string would be an invisible target.
    #[test]
    /// The page records the presses it cannot carry out, and the shell can take them.
    #[test]
    fn a_press_the_page_cannot_act_on_waits_for_the_shell() {
        let mut state = State::default();
        assert_eq!(state.take_press(), None, "nothing is pressed on a fresh page");
        state.pressed = Some(Press::Install);
        assert_eq!(state.take_press(), Some(Press::Install));
        assert_eq!(
            state.take_press(),
            None,
            "a press is taken once, not answered twice"
        );
    }

    fn the_tabs_are_named_and_ordered() {
        assert_eq!(Tab::ALL.map(Tab::label), ["Description", "Versions", "Gallery"]);
    }

    /// The gallery tab exists only while the project has a gallery (Index.vue:174-192); a
    /// project without one must not offer a tab that says "No images yet".
    #[test]
    fn the_gallery_tab_appears_only_when_there_are_images() {
        let project = sample();
        assert!(tabs(&project).contains(&Tab::Gallery));

        let mut without = project.clone();
        without.gallery = Vec::new();
        assert_eq!(tabs(&without), vec![Tab::Description, Tab::Versions]);
        assert!(
            !tabs(&without).contains(&Tab::Gallery),
            "a project with no images offers no gallery tab"
        );
    }

    /// A tab selection that this project does not offer falls back to the description rather
    /// than drawing a card with nothing in it.
    #[test]
    fn a_tab_the_project_does_not_offer_falls_back_to_the_description() {
        let mut project = sample();
        project.gallery = Vec::new();
        let mut state = State::default();
        state.tab = Some(Tab::Gallery);
        assert_eq!(state.current(&project), Tab::Description);
    }

    /// The versions table's columns: the channel is 4.5rem, the two chip columns are the
    /// sheet's 12rem each, and together they never claim more than the card's width.
    #[test]
    fn the_versions_table_columns_fit_the_card() {
        // The columns are the reference's own proportions, scaled to whatever width the table is
        // drawn at: nine tenths of the measured 1028 leaves every column a tenth narrow, and the
        // sum is the card's width to within a rounding error.
        let columns = table_columns(948.0);
        let total: f32 = columns.iter().sum();
        assert!(
            (total - 948.0).abs() < 0.01,
            "the columns claim {total} of 948 points"
        );
        assert!(
            (columns[0] - CHANNEL_WIDTH * 948.0 / 1028.0).abs() < 0.01,
            "the channel keeps its measured share: {}",
            columns[0]
        );
        for width in [520.0_f32, 948.0, 1028.0, 1440.0] {
            let scaled = table_columns(width);
            let sum: f32 = scaled.iter().sum();
            assert!((sum - width).abs() < 0.01, "{width}: {sum}");
            assert!(scaled[1] > 100.0, "the version column keeps room for its name");
        }
    }

    /// The gallery is repeat(auto-fill, minmax(20rem, 1fr)) at gap 1rem: two tracks at 948,
    /// three once the card is wide enough for them, one when it is narrower than a track.
    #[test]
    fn the_gallery_grid_reflows() {
        let (columns, width) = gallery_grid(948.0);
        assert_eq!(columns, 2);
        assert!((width - 466.0).abs() < 0.001, "(948 - 16) / 2");

        let (columns, width) = gallery_grid(1100.0);
        assert_eq!(columns, 3, "1100 holds three 320-point tracks and their gaps");
        assert!((width - ((1100.0 - 32.0) / 3.0)).abs() < 0.001);

        let (columns, width) = gallery_grid(300.0);
        assert_eq!(columns, 1, "a card narrower than a track still gets a whole one");
        assert!((width - 300.0).abs() < 0.001);
    }

    /// The Published column is dayjs's relative ladder: a day count under 26 days, then a
    /// month, then months, then a year, then years.
    #[test]
    fn the_relative_dates_follow_the_sources_ladder() {
        let now = Date { year: 2026, month: 11, day: 1 };
        assert_eq!(relative(now, now), "today");
        assert_eq!(relative(Date { year: 2026, month: 10, day: 31 }, now), "yesterday");
        assert_eq!(
            relative(Date { year: 2026, month: 10, day: 20 }, now),
            "12 days ago"
        );
        assert_eq!(
            relative(Date { year: 2026, month: 10, day: 5 }, now),
            "a month ago"
        );
        assert_eq!(
            relative(Date { year: 2026, month: 6, day: 1 }, now),
            "5 months ago"
        );
        assert_eq!(
            relative(Date { year: 2025, month: 12, day: 1 }, now),
            "a year ago"
        );
        assert_eq!(
            relative(Date { year: 2022, month: 2, day: 10 }, now),
            "4 years ago"
        );
    }

    /// The Downloads column: **under ten thousand the number stays grouped**, which is the reference's own rule
    /// (`formatCompactNumber` leaves the 7,847 of its 14.1.2 row whole), and above that it takes a K, an M or a B
    /// with one decimal only when it is not zero.
    #[test]
    fn the_download_counts_are_compact() {
        assert_eq!(compact(999), "999");
        assert_eq!(compact(1_000), "1,000");
        assert_eq!(compact(7_847), "7,847");
        assert_eq!(compact(9_999), "9,999");
        assert_eq!(compact(10_000), "10K");
        assert_eq!(compact(19_600_000), "19.6M");
        assert_eq!(compact(2_184_302), "2.2M", "rounded, not truncated: 2.18M is 2.2M");
        assert_eq!(compact(1_100_000_000), "1.1B");
    }

    /// The civil arithmetic the Published column divides by: the epoch is day zero, a leap day
    /// is one day, and the round trip returns the date it started from.
    #[test]
    fn the_civil_calendar_round_trips() {
        assert_eq!(Date { year: 1970, month: 1, day: 1 }.day_number(), 0);
        assert_eq!(
            Date { year: 2024, month: 2, day: 29 }.day_number()
                - Date { year: 2024, month: 2, day: 28 }.day_number(),
            1
        );
        for date in [
            Date { year: 1970, month: 1, day: 1 },
            Date { year: 2026, month: 10, day: 20 },
            Date { year: 2024, month: 2, day: 29 },
            Date { year: 1999, month: 12, day: 31 },
        ] {
            assert_eq!(civil_from_days(date.day_number()), date);
        }
    }

    /// The filter row: a dimension with no chip on is not a filter, a chip keeps only its own
    /// values, and two dimensions are AND-ed.
    #[test]
    fn the_version_chips_filter_one_dimension_at_a_time() {
        let project = sample();
        let all = filtered_versions(&project.versions, &[], &[], None);
        assert_eq!(all.len(), project.versions.len(), "no chip is the whole table");

        let betas = filtered_versions(&project.versions, &[Channel::Beta], &[], None);
        assert_eq!(betas.len(), 2, "the sample has two betas");
        assert!(betas.iter().all(|version| version.channel == Channel::Beta));

        let releases = filtered_versions(&project.versions, &[Channel::Release], &[], None);
        assert_eq!(releases.len(), 2);

        let none = filtered_versions(&project.versions, &[Channel::Alpha], &[], None);
        assert!(none.is_empty(), "a channel with no versions leaves an empty table");

        let fabric = filtered_versions(&project.versions, &[], &["fabric"], None);
        assert_eq!(fabric.len(), project.versions.len(), "every sample version is Fabric");

        let forge = filtered_versions(&project.versions, &[], &["forge"], None);
        assert!(forge.is_empty(), "a loader the project does not ship leaves nothing");

        let beta_and_fabric = filtered_versions(&project.versions, &[Channel::Beta], &["fabric"], None);
        assert_eq!(
            beta_and_fabric.len(),
            2,
            "two dimensions are AND-ed, so both must match"
        );
    }

    /// A date prints the way `useFormatDateTime({ year, month: 'long', day })` prints it.
    #[test]
    fn the_dates_read_as_the_sheet_prints_them() {
        assert_eq!(
            Date { year: 2022, month: 2, day: 10 }.long(),
            "February 10, 2022"
        );
        assert_eq!(
            Date { year: 2026, month: 10, day: 20 }.long(),
            "October 20, 2026"
        );
    }

    /// The sidebar is four cards, and the height it claims is the cards' own: a project with the
    /// whole endpoint behind it draws more than the 400 points the shell gives the block, and a
    /// card-only project draws nothing rather than four empty frames.
    #[test]
    fn the_sidebar_draws_getting_started_and_the_cards_that_have_rows() {
        let ctx = egui::Context::default();
        crate::fonts::install(&ctx);
        let theme = crate::theme::dark();
        let full = sample();
        let mut drawn = 0.0;
        ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                drawn = panel(
                    ui,
                    Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(252.0, 400.0)),
                    &theme,
                    &full,
                );
            });
        });
        assert!(
            drawn > 400.0,
            "four cards plus Getting started is more than the shell's 400-point block: {drawn}"
        );

        let mut bare = sample();
        bare.versions = Vec::new();
        bare.game_versions = Vec::new();
        bare.links = Vec::new();
        bare.creators = Vec::new();
        bare.published = None;
        bare.updated = None;
        bare.license = String::new();
        let mut bare_height = 1.0;
        ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                bare_height = panel(
                    ui,
                    Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(252.0, 400.0)),
                    &theme,
                    &bare,
                );
            });
        });
        // **Getting started is there whatever the project is**, which the reference's own project capture shows: its
        // sidebar carries the checklist above the Compatibility card, and a project with no links, creators or dates
        // still has that card. The number is the card's own height -- heading, three steps, its gaps.
        assert!(
            (bare_height - 234.0).abs() < 0.5,
            "a card-only project keeps the Getting started card: {bare_height}"
        );
    }
}
