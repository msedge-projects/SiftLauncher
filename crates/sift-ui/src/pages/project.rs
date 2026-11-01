
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

use std::time::UNIX_EPOCH;

use eframe::egui::{self, Align2, Color32, Rect, Sense, Stroke, Ui, Vec2};

use crate::fonts;
use crate::icons::{self, Glyph};
use crate::images::Images;
use crate::theme::{Theme, STROKE_WIDTH};

// ---------------------------------------------------------------------------
// The page's geometry
// ---------------------------------------------------------------------------

/// The page column's inset: `p-6` (`Index.vue:48`).
const PAGE_PADDING: f32 = 24.0;

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
const HEADER_PAD_BOTTOM: f32 = 16.0;

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

/// `px-3.5` on a `size="xl"` button.
const INSTALL_PAD: f32 = 14.0;

/// `gap-2` on a `size="xl"` button.
const INSTALL_GAP: f32 = 8.0;

/// `svg size-6` on a `size="xl"` button.
const INSTALL_GLYPH: f32 = 24.0;

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

/// A card's corner: `--radius-lg` (`classes.scss:502`).
const CARD_RADIUS: u8 = 16;

/// A table header cell's height: `h-12` (`base/Table.vue:24-36`).
const TABLE_HEADER: f32 = 48.0;

/// A table row's height: `h-14` (`base/Table.vue:100-110`).
const TABLE_ROW: f32 = 56.0;

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
#[derive(Debug, Clone, Copy)]
pub struct Version {
    /// `version_number`: the version's own name.
    pub number: &'static str,
    /// `name`: the longer label the row's tooltip carries.
    pub name: &'static str,
    /// `game_versions`: the Minecraft versions the file runs on.
    pub game_versions: &'static [&'static str],
    /// `loaders`: the platforms the file is built for.
    pub loaders: &'static [&'static str],
    /// `version_type`.
    pub channel: Channel,
    /// `date_published`.
    pub published: Date,
    /// `downloads`.
    pub downloads: u64,
}

/// One gallery image, as `project.gallery` returns it.
#[derive(Debug, Clone, Copy)]
pub struct Image {
    /// `url`: the thumbnail the grid draws.
    pub url: &'static str,
    /// `title`.
    pub title: &'static str,
    /// `description`.
    pub description: &'static str,
    /// `created`.
    pub created: Date,
}

/// What the page shows. The Discover card carries the same fields, plus what a full project
/// page needs.
#[derive(Clone)]
pub struct Project {
    /// The project's name.
    pub title: &'static str,
    /// The team that publishes it.
    pub author: &'static str,
    /// The one-line summary the header shows.
    pub summary: &'static str,
    /// The longer description, already broken into paragraphs.
    pub body: &'static [&'static str],
    /// The loader and environment tags the header shows.
    pub tags: &'static [(&'static str, Glyph)],
    /// Game versions the project supports, newest first.
    pub game_versions: &'static [&'static str],
    /// The download count, abbreviated.
    pub downloads: &'static str,
    /// The follower count, abbreviated.
    pub followers: &'static str,
    /// The icon's URL.
    pub icon: &'static str,
    /// The project's links, by the catalogue's `project.about.links.*` labels.
    pub links: &'static [&'static str],
    /// The team, as `(name, role)`.
    pub creators: &'static [(&'static str, &'static str)],
    /// When it was first published.
    pub published: Date,
    /// When it last changed.
    pub updated: Date,
    /// The licence it ships under.
    pub license: &'static str,
    /// The gallery, in the order the reference shows it.
    pub gallery: &'static [Image],
    /// The releases, newest first.
    pub versions: &'static [Version],
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
}

impl Default for State {
    fn default() -> State {
        State {
            tab: None,
            slider: None,
            viewer: None,
        }
    }
}

impl State {
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

/// The compact form the Downloads column prints: formatCompactNumber, which is 1.2K, 19.6M,
/// 1.1B.
fn compact(count: u64) -> String {
    let units = [(1_000_000_000_u64, "B"), (1_000_000, "M"), (1_000, "K")];
    for (size, suffix) in units {
        if count >= size {
            let whole = count / size;
            let tenths = count % size * 10 / size;
            return if tenths == 0 {
                format!("{whole}{suffix}")
            } else {
                format!("{whole}.{tenths}{suffix}")
            };
        }
    }
    count.to_string()
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

/// The versions table's column widths, in the order the cells are drawn: channel, version, game
/// versions, platforms, published, downloads, actions.
///
/// ProjectPageVersions.vue:560-615 gives the channel 4.5rem, the chip columns max-w-[12rem],
/// published and downloads 12% each, actions 1%, and leaves the version number the rest.
fn table_columns(width: f32) -> [f32; 7] {
    let fixed = CHANNEL_WIDTH + 2.0 * CHIP_COLUMN + ACTIONS_COLUMN + 4.0 * TABLE_PAD;
    let share = (width - fixed).max(0.0);
    let date = (width * DATE_COLUMN).max(72.0);
    let version = (share - 2.0 * date).max(112.0);
    [
        CHANNEL_WIDTH,
        version,
        CHIP_COLUMN,
        CHIP_COLUMN,
        date,
        date,
        ACTIONS_COLUMN,
    ]
}

/// The wall clock, as the page's [Date]: the Published column is relative to the reader's day.
fn today() -> Date {
    let seconds = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since_epoch| since_epoch.as_secs() as i64);
    civil_from_days(seconds / SECONDS_PER_DAY)
}

/// Hinnant's civil_from_days, the inverse of [Date::day_number].
fn civil_from_days(days: i64) -> Date {
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
) -> bool {
    ui.painter().rect_filled(rect, 0, theme.BG);
    if rect.width() <= 2.0 * PAGE_PADDING || rect.height() <= 2.0 * PAGE_PADDING {
        return false;
    }

    // The page column: flex flex-col gap-4 p-6 (Index.vue:48).
    let column = Rect::from_min_max(
        egui::pos2(rect.left() + PAGE_PADDING, rect.top() + PAGE_PADDING),
        egui::pos2(rect.right() - PAGE_PADDING, rect.bottom() - PAGE_PADDING),
    );

    // Every rectangle a press is read against, for `SIFT_PROJECT_PROBE`.
    let mut probe_rows: Vec<(String, Rect)> = vec![("column".to_owned(), column)];

    // ---- The header, with its rule and its 16-point foot ----
    let header_height = header(ui, column, theme, images, project, &mut probe_rows);
    let rule_y = column.top() + header_height + HEADER_PAD_BOTTOM;
    ui.painter().rect_filled(
        Rect::from_min_size(
            egui::pos2(column.left(), rule_y),
            egui::vec2(column.width(), STROKE_WIDTH),
        ),
        0,
        theme.DIVIDER,
    );

    // ---- The tab strip ----
    let offered = tabs(project);
    let strip_top = rule_y + STROKE_WIDTH + BLOCK_GAP;
    let mut owed = false;
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
    ui.painter().rect_filled(card, CARD_RADIUS, theme.SURFACE_3);
    ui.painter().rect_stroke(
        card,
        CARD_RADIUS,
        Stroke::new(STROKE_WIDTH, theme.SURFACE_4),
        egui::StrokeKind::Inside,
    );

    let inner = card.shrink(CARD_PAD);
    match state.current(project) {
        Tab::Description => description(ui, inner, theme, project),
        Tab::Versions => versions(ui, inner, theme, project),
        Tab::Gallery => gallery(ui, inner, theme, project, images, state, &mut probe_rows),
    }

    probe_rows.push(("card".to_owned(), card));

    // The gallery's viewer is drawn last, over everything the page owns.
    owed |= viewer(ui, rect, theme, images, state, project);
    probe(ui, &probe_rows);
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
    probe_rows: &mut Vec<(String, Rect)>,
) -> f32 {
    let painter = ui.painter();
    let avatar = Rect::from_min_size(column.min, Vec2::splat(AVATAR));
    painter.rect_filled(avatar, 16, theme.BUTTON_BG);
    if let Some(texture) = images.get(ui.ctx(), project.icon) {
        painter.image(
            texture.id(),
            avatar,
            Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
    }

    let text_left = avatar.right() + HEADER_GAP;
    // The actions are laid out from the right first, so the summary knows what width it has.
    let install_width = install_width(painter, theme);
    let install = Rect::from_min_size(
        egui::pos2(column.right() - install_width, column.top()),
        egui::vec2(install_width, INSTALL_HEIGHT),
    );
    let kebab = Rect::from_min_size(
        egui::pos2(install.left() - INSTALL_GAP - KEBAB, column.top()),
        Vec2::splat(KEBAB),
    );

    // The title: text-2xl font-semibold leading-none text-contrast.
    painter.text(
        egui::pos2(text_left, column.top()),
        Align2::LEFT_TOP,
        project.title,
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
        project.summary,
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
    for (index, (glyph, value)) in [
        (Glyph::Download, project.downloads),
        (Glyph::Heart, project.followers),
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
    for (tag, glyph) in project.tags {
        let chip = tag_rect(painter, egui::pos2(x, meta_top), theme, tag, Some(*glyph));
        x = chip.right() + TAG_GAP;
    }

    // The install button and the kebab menu. Both are drawn at the reference's size and left
    // without an action: the installer is the shell's, and this page cannot return one.
    primary_button(painter, install, theme, "Install", Glyph::Download);
    icons::draw(
        painter,
        Glyph::EllipsisVertical,
        Rect::from_center_size(kebab.center(), Vec2::splat(INSTALL_GLYPH)),
        theme.SECONDARY,
    );

    probe_rows.push(("install".to_owned(), install));
    probe_rows.push(("kebab".to_owned(), kebab));

    // The header's own height: the taller of the avatar and the text column.
    let text_height = TITLE_LINE + SUMMARY_GAP + summary_height + META_CONTENT_GAP + TAG_HEIGHT;
    AVATAR.max(text_height)
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
    painter.rect_filled(rect, INSTALL_RADIUS, theme.BUTTON_BG_SELECTED);
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
        theme.BUTTON_TEXT_SELECTED,
    );
    painter.text(
        egui::pos2(
            rect.left() + INSTALL_PAD + INSTALL_GLYPH + INSTALL_GAP,
            rect.center().y,
        ),
        Align2::LEFT_CENTER,
        label,
        fonts::semibold(INSTALL_TEXT),
        theme.BUTTON_TEXT_SELECTED,
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

/// The description tab: the rendered markdown body inside the card, and nothing else.
///
/// The reference's tab is `Card` wrapping `ProjectPageDescription` (`Description.vue:1-5`); the
/// action row this page used to draw above the body was invented, and is gone.
fn description(ui: &mut Ui, inner: Rect, theme: &Theme, project: &Project) {
    let painter = ui.painter();
    let mut heights = Vec::new();
    let mut total = 0.0;
    for paragraph in project.body {
        let galley = painter.layout(
            (*paragraph).to_owned(),
            fonts::body(SUMMARY_SIZE),
            theme.BASE,
            inner.width(),
        );
        total += galley.size().y + PARAGRAPH_GAP;
        heights.push(galley);
    }
    if total > inner.height() {
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner));
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(&mut child, |ui| {
                let (area, _) = ui.allocate_exact_size(
                    egui::vec2(inner.width(), total),
                    Sense::hover(),
                );
                let mut y = area.top();
                for galley in heights {
                    let height = galley.size().y;
                    ui.painter().galley(egui::pos2(area.left(), y), galley, theme.BASE);
                    y += height + PARAGRAPH_GAP;
                }
            });
        return;
    }
    let mut y = inner.top();
    for galley in heights {
        let height = galley.size().y;
        ui.painter()
            .galley(egui::pos2(inner.left(), y), galley, theme.BASE);
        y += height + PARAGRAPH_GAP;
    }
}

/// The gap between two paragraphs of the body: a markdown `p`'s own margin.
const PARAGRAPH_GAP: f32 = 12.0;

// ---------------------------------------------------------------------------
// The Versions tab
// ---------------------------------------------------------------------------

/// The versions tab: the reference's `Table` (`ProjectPageVersions.vue:560-615`).
fn versions(ui: &mut Ui, inner: Rect, theme: &Theme, project: &Project) {
    if project.versions.is_empty() {
        empty_tab(ui, inner, theme, "No versions yet.");
        return;
    }
    let now = today();
    let table_height = TABLE_HEADER + project.versions.len() as f32 * TABLE_ROW;
    let table = Rect::from_min_size(
        inner.min,
        egui::vec2(inner.width(), table_height),
    );

    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner));
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

            for (row, version) in project.versions.iter().enumerate() {
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
                painter.circle_filled(
                    channel.center(),
                    5.0,
                    match version.channel {
                        Channel::Release => theme.GREEN,
                        Channel::Beta => theme.ORANGE,
                        Channel::Alpha => theme.RED,
                    },
                );
                let _ = version.channel.label();
                x += columns[0];

                // The version number: font-medium text-contrast.
                let name = Rect::from_min_size(
                    egui::pos2(x, band.top()),
                    egui::vec2(columns[1], TABLE_ROW),
                );
                painter.text(
                    egui::pos2(name.left(), name.center().y),
                    Align2::LEFT_CENTER,
                    version.number,
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
                    version.game_versions,
                );
                x += columns[2];

                // The platforms, the same way.
                let platforms = Rect::from_min_size(
                    egui::pos2(x, band.top()),
                    egui::vec2(columns[3], TABLE_ROW),
                );
                chip_run(painter, platforms, theme, version.loaders);
                x += columns[3];

                // Published: relative time, font-medium.
                let published = Rect::from_min_size(
                    egui::pos2(x, band.top()),
                    egui::vec2(columns[4], TABLE_ROW),
                );
                painter.text(
                    egui::pos2(published.left(), published.center().y),
                    Align2::LEFT_CENTER,
                    relative(version.published, now),
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
}

/// A run of `TagItem` chips, one row, clipped to the column they are given.
fn chip_run(painter: &egui::Painter, column: Rect, theme: &Theme, labels: &[&'static str]) {
    let painter = painter.with_clip_rect(column);
    let mut x = column.left();
    for label in labels {
        let chip = tag_rect(
            &painter,
            egui::pos2(x, column.center().y - TAG_HEIGHT / 2.0),
            theme,
            label,
            None,
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
                match images.get(ui.ctx(), image.url) {
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
                    image.title,
                    fonts::semibold(GALLERY_TITLE),
                    theme.CONTRAST,
                );
                let description = clamp_to_lines(
                    &painter,
                    image.description,
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
    if let Some(texture) = images.get(ui.ctx(), image.url) {
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
    let painter = ui.painter();
    let mut y = rect.top();
    if project.versions.is_empty() {
        return 0.0;
    }
    painter.text(
        egui::pos2(rect.left(), y),
        Align2::LEFT_TOP,
        "Compatibility",
        fonts::semibold(18.0),
        theme.CONTRAST,
    );
    y += 30.0;
    painter.text(
        egui::pos2(rect.left(), y),
        Align2::LEFT_TOP,
        "Minecraft: Java Edition",
        fonts::body(16.0),
        theme.TEXT_PRIMARY,
    );
    y += 24.0;

    // Version chips, wrapping at the panel's width, at `gap-1` (4) between them.
    let mut x = rect.left();
    for version in project.game_versions {
        let label = painter.layout_no_wrap((*version).to_owned(), fonts::body(16.0), theme.BASE);
        let chip = chip_at(painter, egui::pos2(x, y), theme, &label, 16.0, None);
        if chip.right() > rect.right() {
            x = rect.left();
            y += 26.0;
            let _ = chip_at(painter, egui::pos2(x, y), theme, &label, 16.0, None);
        }
        x += chip.width() + PANEL_CHIP_GAP;
    }
    y += 34.0;

    for (heading, values) in [
        (
            "Platforms",
            project
                .tags
                .iter()
                .filter(|(tag, _)| matches!(*tag, "Fabric" | "Forge" | "NeoForge" | "Quilt"))
                .copied()
                .collect::<Vec<_>>(),
        ),
        (
            "Supported environments",
            project
                .tags
                .iter()
                .filter(|(tag, _)| *tag == "Client-side" || *tag == "Server-side")
                .copied()
                .collect::<Vec<_>>(),
        ),
    ] {
        painter.text(
            egui::pos2(rect.left(), y),
            Align2::LEFT_TOP,
            heading,
            fonts::body(16.0),
            theme.TEXT_PRIMARY,
        );
        y += 24.0;
        let mut chip_x = rect.left();
        for (tag, glyph) in values {
            let label = painter.layout_no_wrap(tag.to_owned(), fonts::body(16.0), theme.BASE);
            let chip = chip_at(
                painter,
                egui::pos2(chip_x, y),
                theme,
                &label,
                TAG_GLYPH,
                Some(glyph),
            );
            chip_x = chip.right() + PANEL_CHIP_GAP;
        }
        y += 32.0;
    }

    // Links are labels with no value yet, and drawn by hand rather than through `section` so
    // the labels are not paired with empty strings.
    painter.text(
        egui::pos2(rect.left(), y),
        Align2::LEFT_TOP,
        "Links", // project.about.links.title
        fonts::semibold(18.0),
        theme.CONTRAST,
    );
    y += 24.0;
    for link in project.links {
        painter.text(
            egui::pos2(rect.left(), y),
            Align2::LEFT_TOP,
            *link,
            fonts::body(16.0),
            theme.LINK,
        );
        y += 24.0;
    }
    y += 8.0;
    y += section(
        painter,
        rect,
        y,
        theme,
        "Creators", // project.about.creators.title
        project.creators,
    );
    let published = project.published.long();
    let updated = project.updated.long();
    y += section(
        painter,
        rect,
        y,
        theme,
        "Details", // label.details
        &[
            // project.about.details.published
            ("Published", published.as_str()),
            // project.about.details.updated
            ("Updated", updated.as_str()),
            // project.license.title
            ("License", project.license),
        ],
    );

    y - rect.top()
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

/// The project the Discover list's first card opens, so the page can be captured against the
/// reference's own first card.
pub fn sample() -> Project {
    Project {
        title: "Fabulously Optimized",
        author: "Fabulously Optimized",
        summary: "Beautiful graphics, speedy performance and familiar features in a simple package. Wilderness Bound beta!",
        body: &[
            "A simple Minecraft modpack focusing on performance and graphics enhancements.",
            "Fabulously Optimized is faster than competing game clients, thanks to modern performance mods like Sodium, Lithium, and others. The bundled mods have been carefully selected to be safe in your worlds and provide noticeable speed benefits on any device.",
        ],
        tags: &[
            ("Lightweight", Glyph::Sparkles),
            ("Multiplayer", Glyph::Users),
            ("Optimization", Glyph::Rocket),
            ("Fabric", Glyph::Box),
            ("Client-side", Glyph::Monitor),
        ],
        game_versions: &["26.3", "26.2", "26.1.x", "1.21.x", "1.20.x", "1.19.x", "1.18.1-1.18.2", "1.17.1", "1.16.5"],
        downloads: "19.6M downloads",
        followers: "5.0K followers",
        icon: "https://cdn.modrinth.com/data/1KVo5zza/d8152911f8fd5d7e9a8c499fe89045af81fe816e_96.webp",
        // project.about.links.{source,issues,wiki,discord}
        links: &["View source", "Report issues", "Visit wiki", "Join Discord"],
        // `GET /v2/project/1KVo5zza`.
        published: Date { year: 2022, month: 2, day: 10 },
        updated: Date { year: 2026, month: 10, day: 20 },
        license: "BSD-3-Clause",
        // `GET /v2/project/1KVo5zza` at `gallery`.
        gallery: &[
            Image {
                url: "https://cdn.modrinth.com/data/1KVo5zza/images/ce41746dbf8f3d2a9661875c18b5667778233ccf_350.webp",
                title: "Title screen",
                description: "The title screen as the pack leaves it, with the panorama and the quick-play buttons untouched.",
                created: Date { year: 2023, month: 3, day: 4 },
            },
            Image {
                url: "https://cdn.modrinth.com/data/1KVo5zza/images/044a5b9cb85394be95656a1ebcfe135466420da9.png",
                title: "Mod menu",
                description: "Every bundled mod in one list, with the ones that change rendering already configured.",
                created: Date { year: 2023, month: 5, day: 19 },
            },
            Image {
                url: "https://cdn.modrinth.com/data/1KVo5zza/images/9d5ddd3392ce89295caa1030ca930f71f5c6b49a_350.webp",
                title: "FPS in high render distance",
                description: "A 32-chunk render distance at a frame rate the vanilla client does not reach.",
                created: Date { year: 2024, month: 1, day: 8 },
            },
            Image {
                url: "https://cdn.modrinth.com/data/1KVo5zza/images/38f70f307f1a0b17d8ed4526d1636317e8c12d1d_350.webp",
                title: "Sunrise behind a hill",
                description: "Shader-ready lighting with the pack's own sky and fog settings.",
                created: Date { year: 2025, month: 7, day: 21 },
            },
        ],
        // `GET /v2/project/1KVo5zza/version?limit=4`, newest first.
        versions: &[
            Version {
                number: "15.0.0-beta.2",
                name: "Wilderness Bound beta 2",
                game_versions: &["26.3"],
                loaders: &["fabric"],
                channel: Channel::Beta,
                published: Date { year: 2026, month: 10, day: 20 },
                downloads: 2_184_302,
            },
            Version {
                number: "15.0.0-beta.1",
                name: "Wilderness Bound beta 1",
                game_versions: &["26.3"],
                loaders: &["fabric"],
                channel: Channel::Beta,
                published: Date { year: 2026, month: 10, day: 11 },
                downloads: 1_406_881,
            },
            Version {
                number: "14.1.1",
                name: "Maintenance release",
                game_versions: &["26.2"],
                loaders: &["fabric"],
                channel: Channel::Release,
                published: Date { year: 2026, month: 10, day: 11 },
                downloads: 8_902_114,
            },
            Version {
                number: "13.6.0",
                name: "Sodium 0.7 update",
                game_versions: &["26.1.2"],
                loaders: &["fabric"],
                channel: Channel::Release,
                published: Date { year: 2026, month: 9, day: 28 },
                downloads: 7_203_559,
            },
        ],
        creators: &[
            ("Robitobi01", "Owner"),
            ("unmojang", "Maintainer"),
            ("trimbe", "Maintainer"),
        ],
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
        without.gallery = &[];
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
        project.gallery = &[];
        let mut state = State::default();
        state.tab = Some(Tab::Gallery);
        assert_eq!(state.current(&project), Tab::Description);
    }

    /// The versions table's columns: the channel is 4.5rem, the two chip columns are the
    /// sheet's 12rem each, and together they never claim more than the card's width.
    #[test]
    fn the_versions_table_columns_fit_the_card() {
        let columns = table_columns(948.0);
        assert_eq!(columns[0], CHANNEL_WIDTH);
        assert_eq!(columns[2], CHIP_COLUMN);
        assert_eq!(columns[3], CHIP_COLUMN);
        let total: f32 = columns.iter().sum();
        assert!(
            total <= 948.0,
            "the columns claim {total} of 948 points, and the table cannot overflow its card"
        );
        assert!(columns[1] > 112.0, "the version column keeps room for its own name");
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

    /// The Downloads column is compact: 999 stays whole, thousands take a K and millions an M,
    /// with one decimal only when it is not zero.
    #[test]
    fn the_download_counts_are_compact() {
        assert_eq!(compact(999), "999");
        assert_eq!(compact(1_000), "1K");
        assert_eq!(compact(1_234), "1.2K");
        assert_eq!(compact(19_600_000), "19.6M");
        assert_eq!(compact(2_184_302), "2.1M");
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
}
