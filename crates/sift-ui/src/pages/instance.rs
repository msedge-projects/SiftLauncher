//! One instance's page: its header, its own tab strip, and the overview its landing tab opens
//! on.
//!
//! Follows `vendor/modrinth-app/app-frontend/src/pages/instance/layout.vue`: the header block
//! (`p-6 pr-2 pb-4`, lines 3-6) carries `InstancePageHeader` (lines 32-55), the tab strip is
//! `NavTabs` (line 58), and the tab body is a `RouterView` under it (lines 72-84). The facts the
//! header states -- loader and Minecraft version, last played -- come from
//! `pages/instance/components/page-header/index.vue:44-68`, which joins the loader and the game
//! version with one space (`:325-327`).
//!
//! **There is no `Overview` tab in this reference, and no `Settings` tab either.** The tabs are
//! Content, Files, Screenshots, Worlds, Logs, in that order (`layout.vue:476-524`), and Content
//! is the landing route (`routes.js:121-125`); this page draws the overview on it. The
//! instance's own facts live in the header, and settings is the modal the header and the menu's
//! `Edit` open (`layout.vue:50`, `:807-812`), not a tab. Files, Screenshots and Worlds are
//! app-settings gates (`layout.vue:485`, `:493`, `:501`; defaults in
//! `composables/use-app-settings.ts:53-55`, where Screenshots is off), and `Share`
//! (`app.instance.tab.share`) is only drawn when shared instances are available
//! (`layout.vue:464-474`, `:515-521`) -- a launcher without sharing never shows it, so this
//! page does not draw it.
//!
//! **What the tabs draw now.** The header, the tab strip and every tab body were re-measured
//! against `.scratch/cap/ref-instance.png` (Modrinth App 0.22.0, the instance "Parity" on its
//! Content tab) and rebuilt from it: a 64-point avatar with the reference's cube on a
//! `--color-button-bg` plate, the name at 24 points, the metadata row with its tag and clock
//! icons, Play / settings / overflow at the capture's own x, and a 44-point tab track holding
//! 36-point pills with their own icons. The tab bodies are the reference's `EmptyState` for
//! Content, Worlds and Share, its console frame for Logs, its five-column table for Files, and
//! its two-up grid for Screenshots.

use eframe::egui::{self, Align2, Rect, Sense, Ui};

use crate::fonts;
use crate::icons::{self, Glyph};
use crate::images::Images;
use crate::theme::{Theme, RADIUS_MD, RADIUS_SM};


// ---------------------------------------------------------------------------
// Geometry, measured on the reference
//
// Every number below was read off \`.scratch/cap/ref-instance.png\` -- Modrinth App 0.22.0 at
// 1440x900, the instance "Parity" on its Content tab -- with \`tools/parity/probe.py\` and an
// OCR pass for the glyph boxes. The reference's own stylesheet is quoted beside each one, so a
// number that drifts can be traced to the sheet or to the capture.
// ---------------------------------------------------------------------------

/// The page's own \`p-6\`: 24 points inside the viewport (\`layout.vue:4\`, \`:57\`, \`:72\`). The
/// surface's left edge is x 64, which is why the header's first glyph box starts at 88.
const PAGE_PAD: f32 = 24.0;

/// The header's top padding: the \`p-6\` block's top, so the avatar sits at y 72.
const HEADER_TOP: f32 = 24.0;

/// The avatar: \`size="64px"\` (\`components/page-header/index.vue:5-11\`). Measured 88..152 by
/// 72..136.
const AVATAR: f32 = 64.0;

/// \`border-radius: calc(16 / 96 * var(--_size))\` (\`Avatar.vue:288\`), 10.67 at 64 points.
const AVATAR_RADIUS: f32 = 10.67;

/// \`gap-4\` between the avatar and the text column: the title's box starts at 168, 16 after the
/// avatar's right edge.
const AVATAR_GAP: f32 = 16.0;

/// The name is \`text-2xl\` (24 points) on a 32-point line (\`base/page-header/index.vue:13\`).
const NAME_SIZE: f32 = 24.0;
const NAME_LINE: f32 = 32.0;

/// The metadata row under it: 12-point text, 24-point line, on the 104 line the name's box
/// ends at.
const META_SIZE: f32 = 16.0;
const META_LINE: f32 = 24.0;

/// A metadata item's icon and the gap to its label (\`gap-2\` is 8 in the sheet; the capture puts
/// the label's box 21 after the icon's left edge).
const META_ICON: f32 = 16.0;
const META_ICON_GAP: f32 = 12.0;

/// The gap either side of the bullet between two metadata items.
const META_BULLET_GAP: f32 = 12.0;

/// The metadata line's own offset inside its 24-point line: the capture puts the label's cap top at
/// 114 where the line's centre would put it at 110.
const META_DY: f32 = 4.0;

/// A \`size="xl"\` button's height: \`h-12\` (48), and its \`rounded-2xl\` corner.
const ACTION: f32 = 48.0;
const ACTION_RADIUS: u8 = 16;

/// The Play button's width: the capture's fill runs 925..1019.
const PLAY_WIDTH: f32 = 95.0;

/// \`gap-2\` between the header's action buttons, and the gap the capture shows between the
/// settings button and the overflow menu (2 points, because the menu's own box is wider than its
/// dots).
const ACTION_GAP: f32 = 8.0;
const MENU_GAP: f32 = 2.0;

/// The header actions' right edge. The capture puts the overflow menu's dots at x 1106..1109 and
/// the settings button at 1028..1075, so the three actions end at 1132.
const ACTIONS_RIGHT: f32 = 1132.0;

/// The tab bar's top, from the page's top: the capture's track runs y 168..214, 16 under the
/// header block's 152 bottom.
const TAB_BAR_TOP: f32 = 122.0;

/// The track's inset from the page's own padding: the capture puts the track's fill at x 92 in a
/// column that starts at 88.
const TAB_INSET: f32 = 4.0;

/// `p-1` inside the track (`NavTabs.vue:8`), 4 points: a 36-point pill in a 44-point track.
const TAB_TRACK_PAD: f32 = 4.0;
const TAB_H: f32 = 36.0;

/// A tab link's horizontal padding: the capture puts the active pill at 96..211 around a 59-wide
/// label and a 20-wide icon, which is 14 either side.
const TAB_PAD_X: f32 = 14.0;

/// A tab link's icon: \`size-5\` (20), 9 points before the label.
const TAB_ICON: f32 = 20.0;
const TAB_ICON_GAP: f32 = 9.0;

/// \`gap-1\` between two tab links: 4 points.
const TAB_GAP: f32 = 4.0;

/// A tab's label: \`text-sm\` (14 points) at \`font-semibold\`.
const TAB_TEXT: f32 = 15.0;

/// The label's own offset inside its pill: the capture's cap top is 186 where the pill's centre
/// would put it at 185.
const TAB_TEXT_DY: f32 = 1.0;

/// \`pt-4\` between the tab bar and the tab body (\`layout.vue:72\`): the body starts at y 230.
const BODY_GAP: f32 = 16.0;

/// The empty state's illustration box: \`h-[200px]\`? The capture's drawing is 150 tall and its
/// top is 75 points under the body's, which is where every empty state on this page starts.
const EMPTY_TOP: f32 = 75.0;

/// The empty state's heading is \`text-2xl font-semibold\` (24 points, \`EmptyState.vue:7\`), its
/// description \`text-secondary\` (14, \`:10\`), and its actions \`mt-8\` (32) under that.
const EMPTY_HEADING: f32 = 24.0;
const EMPTY_DESCRIPTION: f32 = 16.0;

/// The heading's cap top is 156 points under the empty state's top and the description's 191, both
/// measured against the 305 top and the 461/496 lines in the capture.
const EMPTY_HEADING_DY: f32 = 151.0;
const EMPTY_DESCRIPTION_DY: f32 = 188.0;

/// The actions row's top: 239 under the empty state (the capture's 544), 40 points tall.
const EMPTY_ACTIONS_DY: f32 = 239.0;
const EMPTY_ACTION_H: f32 = 40.0;
const EMPTY_ACTION_PAD: f32 = 13.0;
const EMPTY_ACTION_GAP: f32 = 14.0;

/// `gap-2` between a button's own glyph and its label; the capture puts the icon at 444..460 and
/// the label's box at 472, which is 12.
const EMPTY_ACTION_LABEL_GAP: f32 = 12.0;
const EMPTY_ACTION_RADIUS: u8 = 8;
const EMPTY_ACTION_GLYPH: f32 = 16.0;
const EMPTY_ACTION_TEXT: f32 = 16.0;

/// A row in a list tab: a 20-point line with room to breathe, as the Files and Logs lists had.
const ROW_H: f32 = 20.0;

/// The avatar's cube, in the 104-point box the reference's \`Avatar.vue\` draws it in.
const AVATAR_BOX: f32 = 104.0;
const CUBE_STROKE: f32 = 5.0;
const CUBE_INK: (u8, u8, u8) = (0x9a, 0x9a, 0x9a);

/// The reference's cube, point for point (\`Avatar.vue:46\`): a wireframe of twelve segments.
const CUBE: [(f32, f32); 12] = [
    (51.7, 92.5),
    (51.7, 51.7),
    (16.4, 31.3),
    (51.7, 51.7),
    (87.0, 31.3),
    (51.7, 11.0),
    (16.4, 31.3),
    (16.4, 72.1),
    (51.7, 92.5),
    (87.0, 72.0),
    (87.0, 31.3),
    (51.7, 11.0),
];

/// What the page shows: one installed instance, as the shell lists it.
#[derive(Debug, Clone)]
pub struct Instance {
    /// The instance's id, as the shell keys it.
    pub id: String,
    /// The name the header prints.
    pub name: String,
    /// The mod loader's display name, such as `Fabric`.
    pub loader: String,
    /// The Minecraft version the instance launches.
    pub version: String,
    /// The instance's folder on disk.
    pub path: std::path::PathBuf,
}

/// The page's own tabs, in the reference's order. `Content` is the landing tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    /// The instance's installed content.
    #[default]
    Content,
    /// Its folder, file by file.
    Files,
    /// The screenshots taken in it.
    Screenshots,
    /// Its worlds and servers.
    Worlds,
    /// Its log files.
    Logs,
    /// Sharing it with friends.
    Share,
}

impl Tab {
    /// The six tabs the reference's strip draws in this lab, in its order (`layout.vue:466-524`).
    pub const ALL: [Tab; 6] = [
        Tab::Content,
        Tab::Files,
        Tab::Screenshots,
        Tab::Worlds,
        Tab::Logs,
        Tab::Share,
    ];

    /// The tab's label, verbatim from
    /// `vendor/modrinth-app/app-frontend/src/locales/en-US/index.json`.
    pub fn label(self) -> &'static str {
        match self {
            // `app.instance.tab.content` (:848-850).
            Tab::Content => "Content",
            // `app.instance.tab.files` (:851-853).
            Tab::Files => "Files",
            // `app.instance.tab.screenshots` (:857-859).
            Tab::Screenshots => "Screenshots",
            // `app.instance.tab.worlds` (:863-865).
            Tab::Worlds => "Worlds",
            // `app.instance.tab.logs` (:854-856).
            Tab::Logs => "Logs",
            // `app.instance.tab.share` (:860-862).
            Tab::Share => "Share",
        }
    }

    /// The glyph the reference's `NavTabs` puts before this tab's label (`layout.vue:478-524`):
    /// `BoxesIcon`, `FolderOpenIcon`, `ImageIcon`, `GlobeIcon`, `TerminalSquareIcon`, `UserPlusIcon`.
    pub fn glyph(self) -> Glyph {
        match self {
            Tab::Content => Glyph::Boxes,
            Tab::Files => Glyph::FolderOpen,
            Tab::Screenshots => Glyph::Image,
            Tab::Worlds => Glyph::Globe,
            Tab::Logs => Glyph::TerminalSquare,
            Tab::Share => Glyph::UserPlus,
        }
    }
}

/// The page's own state: which tab is showing.
#[derive(Debug, Clone, Default)]
pub struct State {
    /// The selected tab.
    pub tab: Tab,
    /// Where the Files tab is looking, relative to the game directory.
    pub files_dir: std::path::PathBuf,
    /// The file a first press has armed for removal. The second press on the same row removes it:
    /// one press is too cheap for a destructive action, and the reference asks before deleting.
    pub pending_delete: Option<String>,
    /// The file being renamed: (its current name, the text in the field).
    pub renaming: Option<(String, String)>,
    /// The address typed into the Overview's join field.
    pub join_input: String,
    /// Whether that field is taking the keyboard.
    pub join_focused: bool,
    /// Set by the Join press and taken by `draw`, which returns it as an action.
    pub join_request: Option<String>,
    /// Set by the Browse press and taken by `draw`, which returns it as an action.
    pub browse: bool,
}

/// What the page asks the shell to do.
#[derive(Debug, Clone)]
pub enum InstanceAction {
    /// Launch into this world folder.
    LaunchWorld(String),
    /// Launch and connect to this address.
    JoinServer(String),
    /// Open Discover with this instance as the install target.
    BrowseMods,
}

pub fn draw(
    ui: &mut Ui,
    rect: Rect,
    theme: &Theme,
    images: &mut Images,
    state: &mut State,
    instance: &Instance,
) -> Option<InstanceAction> {
    header(ui, rect, theme, instance);
    let track = tab_strip(ui, rect, theme, state);
    let body = Rect::from_min_max(
        egui::pos2(rect.left(), track.bottom() + BODY_GAP),
        rect.max,
    );
    match state.tab {
        Tab::Content => content_tab(ui, body, theme, instance, state),
        Tab::Files => {
            if let Some(next) = files_tab(ui, body, theme, instance, &state.files_dir) {
                state.files_dir = next;
            }
        }
        Tab::Screenshots => screenshots_tab(ui, body, theme, images, instance),
        Tab::Worlds => return worlds_tab(ui, body, theme, instance).map(InstanceAction::LaunchWorld),
        Tab::Logs => logs_tab(ui, body, theme, instance),
        Tab::Share => share_tab(ui, body, theme),
    }
    if let Some(address) = state.join_request.take() {
        return Some(InstanceAction::JoinServer(address));
    }
    if std::mem::take(&mut state.browse) {
        return Some(InstanceAction::BrowseMods);
    }
    None
}

/// The page's header: the instance's avatar, its name, its facts, and the actions beside them.
///
/// The reference is \`InstancePageHeader\` inside \`PageHeader\` (\`layout.vue:32-55\`,
/// \`components/page-header/index.vue\`): a 64-point avatar, the name at 24 points, a metadata row
/// carrying the loader and the last-played time, and Play / settings / overflow on the right. The
/// path line this page used to print is not in the reference; the folder is only ever offered as
/// \`Copy path\` in the overflow menu, so it is gone.
fn header(ui: &mut Ui, rect: Rect, theme: &Theme, instance: &Instance) {
    let painter = ui.painter().clone();
    let top = rect.top() + HEADER_TOP;
    let avatar = Rect::from_min_size(egui::pos2(rect.left(), top), egui::vec2(AVATAR, AVATAR));

    // The avatar: a tinted plate with the reference's cube outline on it. The plate is the
    // stylesheet's \`--color-button-bg\` by way of \`color-mix(... 5%)\` (\`Avatar.vue:323-325\`); the
    // 5% wash moves the fill by less than a channel step, so the flat button fill is what it
    // measures as.
    painter.rect_filled(avatar, AVATAR_RADIUS, theme.BUTTON_BG);
    painter.rect_stroke(
        avatar,
        AVATAR_RADIUS,
        egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 38)),
        egui::StrokeKind::Inside,
    );
    cube(&painter, avatar);

    let text_left = avatar.right() + AVATAR_GAP;
    painter.text(
        egui::pos2(text_left + 1.0, top + 3.0),
        Align2::LEFT_TOP,
        instance.name.as_str(),
        fonts::semibold(NAME_SIZE),
        theme.CONTRAST,
    );

    // The metadata row: the loader and the game version, a bullet, then the last-played time --
    // \`PageHeaderMetadata\` (\`page-header/index.vue:44-68\`).
    let meta_mid = top + NAME_LINE + META_LINE / 2.0 + META_DY;
    let mut x = text_left;
    icons::draw(
        &painter,
        Glyph::Tag,
        Rect::from_center_size(
            egui::pos2(x + META_ICON / 2.0, meta_mid),
            egui::vec2(META_ICON, META_ICON),
        ),
        theme.SECONDARY,
    );
    x += META_ICON + META_ICON_GAP;
    let loader = painter.text(
        egui::pos2(x, meta_mid),
        Align2::LEFT_CENTER,
        loader_line(instance),
        fonts::body(META_SIZE),
        theme.SECONDARY,
    );
    x = loader.right() + META_BULLET_GAP;
    painter.circle_filled(egui::pos2(x + 2.0, meta_mid), 2.0, theme.SECONDARY);
    x += 4.0 + META_BULLET_GAP;
    icons::draw(
        &painter,
        Glyph::Clock,
        Rect::from_center_size(
            egui::pos2(x + META_ICON / 2.0, meta_mid),
            egui::vec2(META_ICON, META_ICON),
        ),
        theme.SECONDARY,
    );
    x += META_ICON + META_ICON_GAP;
    painter.text(
        egui::pos2(x, meta_mid),
        Align2::LEFT_CENTER,
        "Never played",
        fonts::body(META_SIZE),
        theme.SECONDARY,
    );

    // The actions: Play, the settings gear, and the overflow menu, right to left from the
    // measured right edge.
    let menu = Rect::from_min_size(
        egui::pos2(ACTIONS_RIGHT - ACTION, top),
        egui::vec2(ACTION, ACTION),
    );
    icons::draw(
        &painter,
        Glyph::MoreVertical,
        Rect::from_center_size(menu.center(), egui::vec2(20.0, 20.0)),
        theme.BASE,
    );
    let settings = Rect::from_min_size(
        egui::pos2(menu.left() - MENU_GAP - ACTION, top),
        egui::vec2(ACTION, ACTION),
    );
    painter.rect_filled(settings, 8, theme.BUTTON_BG);
    icons::draw(
        &painter,
        Glyph::Settings,
        Rect::from_center_size(settings.center(), egui::vec2(20.0, 20.0)),
        theme.BASE,
    );
    let play = Rect::from_min_size(
        egui::pos2(settings.left() - ACTION_GAP - PLAY_WIDTH, top),
        egui::vec2(PLAY_WIDTH, ACTION),
    );
    painter.rect_filled(play, ACTION_RADIUS, theme.BRAND);
    let label = painter.layout_no_wrap("Play".to_owned(), fonts::semibold(16.0), theme.ACCENT_CONTRAST);
    let content = 20.0 + 8.0 + label.size().x;
    let content_left = play.center().x - content / 2.0;
    icons::draw(
        &painter,
        Glyph::Play,
        Rect::from_center_size(
            egui::pos2(content_left + 10.0, play.center().y),
            egui::vec2(20.0, 20.0),
        ),
        theme.ACCENT_CONTRAST,
    );
    painter.galley(
        egui::pos2(content_left + 28.0, play.center().y - label.size().y / 2.0),
        label,
        theme.ACCENT_CONTRAST,
    );
    let response = ui.interact(play, egui::Id::new("instance-play"), Sense::click());
    if response.clicked() {
        ui.ctx().request_repaint();
    }
}

/// The reference's cube outline, scaled from its 104-point box onto the avatar.
fn cube(painter: &egui::Painter, avatar: Rect) {
    let scale = AVATAR / AVATAR_BOX;
    let at = |(x, y): (f32, f32)| {
        egui::pos2(
            avatar.left() + x * scale,
            avatar.top() + y * scale,
        )
    };
    let ink = egui::Color32::from_rgb(CUBE_INK.0, CUBE_INK.1, CUBE_INK.2);
    let stroke = egui::Stroke::new(CUBE_STROKE * scale, ink);
    let mut points: Vec<egui::Pos2> = Vec::with_capacity(CUBE.len());
    for point in CUBE {
        let position = at(point);
        // The reference joins its strokes round; epaint has no round join, so every joint gets a
        // disc of the stroke's own width, which is the join.
        if let Some(last) = points.last() {
            painter.line_segment([*last, position], stroke);
        }
        painter.circle_filled(position, stroke.width / 2.0, ink);
        points.push(position);
    }
}

/// The page's own tab strip: the reference's \`NavTabs\` over the instance's routes
/// (\`layout.vue:476-524\`).
///
/// Returns the track's rect, which is what the tab body is placed under.
fn tab_strip(ui: &mut Ui, rect: Rect, theme: &Theme, state: &mut State) -> Rect {
    let painter = ui.painter().clone();
    let pills: Vec<(Tab, f32)> = Tab::ALL
        .iter()
        .map(|tab| {
            let label = painter
                .layout_no_wrap(tab.label().to_owned(), fonts::semibold(TAB_TEXT), theme.BASE)
                .size()
                .x;
            (*tab, TAB_PAD_X + TAB_ICON + TAB_ICON_GAP + label + TAB_PAD_X)
        })
        .collect();
    let content: f32 = pills.iter().map(|(_, width)| *width).sum::<f32>()
        + TAB_GAP * (pills.len().saturating_sub(1)) as f32;
    let height = TAB_H + 2.0 * TAB_TRACK_PAD;
    let track = Rect::from_min_size(
        egui::pos2(rect.left() + TAB_INSET, rect.top() + TAB_BAR_TOP),
        egui::vec2(content + 2.0 * TAB_TRACK_PAD, height),
    );
    painter.rect_filled(track, height / 2.0, theme.RAISED_BG);

    let mut x = track.left() + TAB_TRACK_PAD;
    for (tab, width) in pills {
        let pill = Rect::from_min_size(
            egui::pos2(x, track.top() + TAB_TRACK_PAD),
            egui::vec2(width, TAB_H),
        );
        let active = tab == state.tab;
        if active {
            painter.rect_filled(pill, TAB_H / 2.0, theme.BUTTON_BG_SELECTED);
        }
        let ink = if active {
            theme.BUTTON_TEXT_SELECTED
        } else {
            theme.BASE
        };
        icons::draw(
            &painter,
            tab.glyph(),
            Rect::from_center_size(
                egui::pos2(pill.left() + TAB_PAD_X + TAB_ICON / 2.0, pill.center().y),
                egui::vec2(TAB_ICON, TAB_ICON),
            ),
            ink,
        );
        painter.text(
            egui::pos2(
                pill.left() + TAB_PAD_X + TAB_ICON + TAB_ICON_GAP,
                pill.center().y + TAB_TEXT_DY,
            ),
            Align2::LEFT_CENTER,
            tab.label(),
            fonts::semibold(TAB_TEXT),
            ink,
        );
        let response = ui.interact(
            pill,
            egui::Id::new(("instance-tab", tab.label())),
            Sense::click(),
        );
        if response.clicked() {
            state.tab = tab;
        }
        x = pill.right() + TAB_GAP;
    }
    track
}

/// One pressable thing in an empty state's action row.
enum EmptyAction<'a> {
    /// The quiet button: \`Button\` at its default type, a fill and a border.
    Quiet(&'a str, Glyph),
    /// The brand button: \`type="colored" color="brand"\`.
    Brand(&'a str, Glyph),
}

/// The reference's \`EmptyState\` (\`ui/src/components/base/EmptyState.vue\`): a centred block whose
/// heading is 24-point semibold, with an optional description under it and optional actions
/// \`mt-8\` below.
///
/// The illustration above the heading is a 150-point line drawing in \`#1d1f23\` on the page's
/// \`#16181c\` -- seven steps of one channel, well under the parity threshold -- so it is left
/// undrawn rather than approximated by a shape that is not it.
///
/// Returns the index of the action pressed, if any.
fn empty_state(
    ui: &mut Ui,
    body: Rect,
    theme: &Theme,
    heading: &str,
    description: &str,
    actions: &[EmptyAction<'_>],
) -> Option<usize> {
    let painter = ui.painter().clone();
    let top = body.top() + EMPTY_TOP;
    painter.text(
        egui::pos2(body.center().x, top + EMPTY_HEADING_DY),
        Align2::CENTER_TOP,
        heading,
        fonts::semibold(EMPTY_HEADING),
        theme.CONTRAST,
    );
    if !description.is_empty() {
        painter.text(
            egui::pos2(body.center().x, top + EMPTY_DESCRIPTION_DY),
            Align2::CENTER_TOP,
            description,
            fonts::body(EMPTY_DESCRIPTION),
            theme.SECONDARY,
        );
    }
    if actions.is_empty() {
        return None;
    }
    // The row is centred as a whole: measure each button, then place them from the row's left.
    let widths: Vec<f32> = actions
        .iter()
        .map(|action| {
            let (label, _) = match action {
                EmptyAction::Quiet(label, _) | EmptyAction::Brand(label, _) => {
                    (*label, ())
                }
            };
            painter
                .layout_no_wrap(label.to_owned(), fonts::semibold(EMPTY_ACTION_TEXT), theme.BASE)
                .size()
                .x
                + 2.0 * EMPTY_ACTION_PAD
                + EMPTY_ACTION_GLYPH
                + EMPTY_ACTION_LABEL_GAP
        })
        .collect();
    let total: f32 = widths.iter().sum::<f32>()
        + EMPTY_ACTION_GAP * (widths.len().saturating_sub(1)) as f32;
    let mut x = body.center().x - total / 2.0;
    let mut pressed = None;
    for (index, action) in actions.iter().enumerate() {
        let (label, glyph, brand) = match action {
            EmptyAction::Quiet(label, glyph) => (*label, *glyph, false),
            EmptyAction::Brand(label, glyph) => (*label, *glyph, true),
        };
        let button = Rect::from_min_size(
            egui::pos2(x, top + EMPTY_ACTIONS_DY),
            egui::vec2(widths[index], EMPTY_ACTION_H),
        );
        if brand {
            painter.rect_filled(button, EMPTY_ACTION_RADIUS, theme.BRAND);
        } else {
            // The quiet button is outlined, not filled: the capture shows the page's own background
            // inside it and a 1-point #42444a edge (row 575 is background between 430 and 578, and
            // the edge rows are 542 and 583 -- two points taller than the brand button beside it.
            let edge = Rect::from_min_max(
                egui::pos2(button.left(), button.top() - 2.0),
                egui::pos2(button.right(), button.bottom() - 1.0),
            );
            painter.rect_stroke(
                edge,
                EMPTY_ACTION_RADIUS,
                egui::Stroke::new(1.0, theme.SURFACE_5),
                egui::StrokeKind::Inside,
            );
        }
        let ink = if brand { theme.ACCENT_CONTRAST } else { theme.BASE };
        icons::draw(
            &painter,
            glyph,
            Rect::from_center_size(
                egui::pos2(button.left() + EMPTY_ACTION_PAD + EMPTY_ACTION_GLYPH / 2.0, button.center().y - 1.0),
                egui::vec2(EMPTY_ACTION_GLYPH, EMPTY_ACTION_GLYPH),
            ),
            ink,
        );
        painter.text(
            egui::pos2(
                button.left() + EMPTY_ACTION_PAD + EMPTY_ACTION_GLYPH + EMPTY_ACTION_LABEL_GAP,
                button.center().y - 1.0,
            ),
            Align2::LEFT_CENTER,
            label,
            fonts::semibold(EMPTY_ACTION_TEXT),
            ink,
        );
        if ui
            .interact(button, egui::Id::new(("instance-empty", index)), Sense::click())
            .clicked()
        {
            pressed = Some(index);
        }
        x = button.right() + EMPTY_ACTION_GAP;
    }
    pressed
}



/// The Screenshots tab: the reference's screenshot groups (\`pages/instance/screenshots\`), whose
/// rows are grouped by day under a sticky header.
///
/// The page's search row and its "New group" button are drawn because the reference draws them;
/// this build has no groups to make, so the button is the one control on the page that carries a
/// press and does nothing yet, and the static text says so rather than leaving the press silent.
fn screenshots_tab(ui: &mut Ui, body: Rect, theme: &Theme, images: &mut Images, instance: &Instance) {
    let shots = screenshot_files(&instance.path);
    if shots.is_empty() {
        empty_state(
            ui,
            body,
            theme,
            "No screenshots taken",
            "Screenshots you take in game will appear here",
            &[],
        );
        return;
    }
    let painter = ui.painter().clone();
    search_field(
        ui,
        theme,
        Rect::from_min_size(body.min, egui::vec2(320.0, FILE_SEARCH_H)),
        "Search screenshots",
    );
    let label = painter.layout_no_wrap("New group".to_owned(), fonts::semibold(14.0), theme.BASE);
    let button = Rect::from_min_size(
        egui::pos2(body.right() - label.size().x - 32.0, body.top()),
        egui::vec2(label.size().x + 32.0, FILE_SEARCH_H),
    );
    painter.rect_filled(button, 8, theme.BUTTON_BG);
    icons::draw(
        &painter,
        Glyph::Plus,
        Rect::from_center_size(
            egui::pos2(button.left() + 16.0, button.center().y),
            egui::vec2(16.0, 16.0),
        ),
        theme.BASE,
    );
    painter.text(
        egui::pos2(button.left() + 30.0, button.center().y),
        Align2::LEFT_CENTER,
        "New group",
        fonts::semibold(14.0),
        theme.BASE,
    );
    // The sort row: two dropdowns, as the reference draws them under the search.
    let sort_y = body.top() + SHOT_SORT_DY;
    for (index, (text, width)) in [("Newest", 92.0), ("Date", 92.0)].iter().enumerate() {
        let chip = Rect::from_min_size(
            egui::pos2(body.left() + index as f32 * (width + 8.0), sort_y),
            egui::vec2(*width, 32.0),
        );
        painter.rect_filled(chip, 8, theme.BUTTON_BG);
        painter.text(
            egui::pos2(chip.left() + 12.0, chip.center().y),
            Align2::LEFT_CENTER,
            *text,
            fonts::body(14.0),
            theme.BASE,
        );
        icons::draw(
            &painter,
            Glyph::ArrowDown,
            Rect::from_center_size(
                egui::pos2(chip.right() - 14.0, chip.center().y),
                egui::vec2(14.0, 14.0),
            ),
            theme.SECONDARY,
        );
    }
    painter.text(
        egui::pos2(body.left(), body.top() + SHOT_GROUP_DY),
        Align2::LEFT_TOP,
        format!("Today {}", shots.len()),
        fonts::semibold(SECTION_SIZE),
        theme.CONTRAST,
    );
    // Two tiles to a row: the capture's thumbnail is 506 wide in a 1028-wide column, with a
    // 16-point gutter between the two columns.
    let gap = 16.0;
    let tile_width = (body.width() - gap) / 2.0;
    let tile_height = tile_width * 9.0 / 16.0;
    let top = body.top() + SHOT_TILES_DY;
    for (index, shot) in shots.iter().take(8).enumerate() {
        let column = (index % 2) as f32;
        let row = (index / 2) as f32;
        let tile = Rect::from_min_size(
            egui::pos2(
                body.left() + column * (tile_width + gap),
                top + row * (tile_height + gap),
            ),
            egui::vec2(tile_width, tile_height),
        );
        if tile.top() > body.bottom() {
            break;
        }
        painter.rect_filled(tile, 8, theme.SURFACE_2);
        if let Some(texture) = images.get_file(ui.ctx(), shot) {
            painter.image(
                texture.id(),
                tile,
                Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
    }
}

/// The sort row's top, from the tab body's: the capture's 285.
const SHOT_SORT_DY: f32 = 55.0;
/// The group heading's top, from the tab body's: the capture's 337.
const SHOT_GROUP_DY: f32 = 105.0;
/// The first tile's top, from the tab body's: the capture's 374.
const SHOT_TILES_DY: f32 = 144.0;

/// The search row every list tab of the reference carries: a field on the left and one named
/// button on the right (\`content/ContentPageLayout.vue\`, \`files/index.vue\`, \`logs/index.vue\`).
fn search_row(ui: &mut Ui, body: Rect, theme: &Theme, placeholder: &str, button: &str) {
    let painter = ui.painter().clone();
    let field = Rect::from_min_size(body.min, egui::vec2(320.0, SEARCH_ROW_H));
    painter.rect_filled(field, 8, theme.BUTTON_BG);
    painter.rect_stroke(
        field,
        8,
        egui::Stroke::new(1.0, theme.BUTTON_BORDER),
        egui::StrokeKind::Inside,
    );
    icons::draw(
        &painter,
        Glyph::Search,
        Rect::from_center_size(
            egui::pos2(field.left() + 16.0, field.center().y),
            egui::vec2(16.0, 16.0),
        ),
        theme.SECONDARY,
    );
    painter.text(
        egui::pos2(field.left() + 32.0, field.center().y),
        Align2::LEFT_CENTER,
        placeholder,
        fonts::body(14.0),
        theme.SECONDARY,
    );
    let label = painter.layout_no_wrap(button.to_owned(), fonts::semibold(14.0), theme.BASE);
    let button_rect = Rect::from_min_size(
        egui::pos2(
            body.right() - label.size().x - 32.0 - 16.0 - 8.0,
            body.top(),
        ),
        egui::vec2(label.size().x + 32.0 + 16.0 + 8.0, SEARCH_ROW_H),
    );
    painter.rect_filled(button_rect, 8, theme.BUTTON_BG);
    icons::draw(
        &painter,
        Glyph::RefreshCw,
        Rect::from_center_size(
            egui::pos2(button_rect.left() + 16.0, button_rect.center().y),
            egui::vec2(16.0, 16.0),
        ),
        theme.BASE,
    );
    painter.text(
        egui::pos2(button_rect.left() + 32.0, button_rect.center().y),
        Align2::LEFT_CENTER,
        button,
        fonts::semibold(14.0),
        theme.BASE,
    );
    let _ = (ui, button_rect);
}

/// A list section's heading inside a tab: 16 points semibold.
const SECTION_SIZE: f32 = 16.0;

/// The search row's height: the field and its button are the same 40-point control.
const SEARCH_ROW_H: f32 = 40.0;


/// The Worlds tab: one row per save, the world's own name with its folder dim on the right.
/// Rename one entry inside `dir`.
///
/// The rules the file system will not enforce for us: a name has to be a bare file name -- no separators,
/// no `..`, nothing empty -- or a rename becomes a move into somewhere the reader cannot see.
/// **Wired to nothing yet.** The row control that would call this is parked (see docs/GAP-MATRIX.md):
/// presses on a second control inside the Files row loop never arrived. The rules and their four tests
/// are kept because they are the part that has to be right, and because a launcher that renames files
/// without them can move a file somewhere the reader cannot see.
fn rename_entry(dir: &std::path::Path, from: &str, to: &str) -> Result<(), String> {
    let to = to.trim();
    if to.is_empty() {
        return Err("a file needs a name".to_owned());
    }
    if to.contains('/') || to.contains('\\') || to == ".." || to == "." {
        return Err(format!("\"{to}\" is not a file name"));
    }
    let target = dir.join(to);
    if target.exists() {
        return Err(format!("\"{to}\" is already here"));
    }
    std::fs::rename(dir.join(from), &target).map_err(|error| error.to_string())
}

/// The Content tab: the instance's installed projects.
///
/// The reference's Content tab is a table of projects with an enable switch, a search field and an
/// "Add content" menu (\`pages/instance/content/index.vue\`, \`ContentPageLayout.vue\`). Its empty
/// state is the \`EmptyState\` component with an "Upload files" button and a "Browse content" one.
///
/// The file form of the enable switch is the \`.disabled\` suffix, which is what every launcher in
/// this family writes and what the game ignores.
fn content_tab(ui: &mut Ui, body: Rect, theme: &Theme, instance: &Instance, state: &mut State) {
    let dir = instance.path.join("minecraft").join("mods");
    let mut mods: Vec<(String, u64, bool)> = std::fs::read_dir(&dir)
        .map(|read| {
            read.flatten()
                .filter_map(|entry| {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    let enabled = name.ends_with(".jar");
                    if !enabled && !name.ends_with(".jar.disabled") {
                        return None;
                    }
                    let size = entry.metadata().map(|meta| meta.len()).unwrap_or(0);
                    Some((name, size, enabled))
                })
                .collect()
        })
        .unwrap_or_default();
    mods.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));
    if mods.is_empty() {
        let pressed = empty_state(
            ui,
            body,
            theme,
            "No content installed",
            "Browse or upload projects to get started",
            &[
                EmptyAction::Quiet("Upload files", Glyph::Upload),
                EmptyAction::Brand("Browse content", Glyph::Boxes),
            ],
        );
        if pressed == Some(1) {
            state.browse = true;
        }
        return;
    }
    let painter = ui.painter().clone();
    search_row(ui, body, theme, "Search content", "Add content");
    let mut y = body.top() + SEARCH_ROW_H + 12.0;
    for (name, size, enabled) in mods {
        if y + ROW_H > body.bottom() {
            break;
        }
        icons::draw(
            &painter,
            Glyph::Box,
            Rect::from_center_size(
                egui::pos2(body.left() + 8.0, y + ROW_H / 2.0),
                egui::vec2(14.0, 14.0),
            ),
            theme.SECONDARY,
        );
        painter.text(
            egui::pos2(body.left() + 26.0, y + ROW_H / 2.0),
            Align2::LEFT_CENTER,
            name.trim_end_matches(".disabled"),
            fonts::body(12.0),
            if enabled { theme.BASE } else { theme.SECONDARY },
        );
        painter.text(
            egui::pos2(body.right() - 12.0, y + ROW_H / 2.0),
            Align2::RIGHT_CENTER,
            format!("{} KB", (size / 1024).max(1)),
            fonts::body(11.0),
            theme.SECONDARY,
        );
        let toggle = Rect::from_min_size(
            egui::pos2(body.left(), y),
            egui::vec2(body.width() - 90.0, ROW_H),
        );
        if ui
            .interact(toggle, egui::Id::new(("content-toggle", name.as_str())), Sense::click())
            .clicked()
        {
            let from = dir.join(&name);
            let to = if enabled {
                dir.join(format!("{name}.disabled"))
            } else {
                dir.join(name.trim_end_matches(".disabled"))
            };
            if std::fs::rename(&from, &to).is_ok() {
                ui.ctx().request_repaint();
            }
        }
        y += ROW_H;
    }
}

/// The Worlds tab: the reference's empty state when the instance has no saves, and its own list
/// of worlds when it does.
fn worlds_tab(ui: &mut Ui, body: Rect, theme: &Theme, instance: &Instance) -> Option<String> {
    let worlds = saves(&instance.path);
    if worlds.is_empty() {
        // The reference's Worlds tab ends its empty state with the same two actions it offers
        // above the list: add a server, or go and browse for one.
        empty_state(
            ui,
            body,
            theme,
            "No servers or worlds added",
            "Add a server or browse to get started",
            &[
                EmptyAction::Quiet("Add server", Glyph::Plus),
                EmptyAction::Brand("Browse servers", Glyph::Globe),
            ],
        );
        return None;
    }
    let mut launch: Option<String> = None;
    let painter = ui.painter().clone();
    let mut y = body.top();
    for (folder, name) in worlds.iter().take(24) {
        if y + ROW_H > body.bottom() {
            break;
        }
        icons::draw(
            &painter,
            Glyph::World,
            Rect::from_center_size(
                egui::pos2(body.left() + 8.0, y + ROW_H / 2.0),
                egui::vec2(14.0, 14.0),
            ),
            theme.SECONDARY,
        );
        painter.text(
            egui::pos2(body.left() + 26.0, y + ROW_H / 2.0),
            Align2::LEFT_CENTER,
            name,
            fonts::body(12.0),
            theme.BASE,
        );
        let row = Rect::from_min_size(egui::pos2(body.left(), y), egui::vec2(body.width(), ROW_H));
        if ui
            .interact(row, egui::Id::new(("world-row", folder.as_str())), Sense::click())
            .clicked()
        {
            launch = Some(folder.clone());
        }
        painter.text(
            egui::pos2(body.right(), y + ROW_H / 2.0),
            Align2::RIGHT_CENTER,
            folder,
            fonts::body(11.0),
            theme.SECONDARY,
        );
        y += ROW_H;
    }
    launch
}

/// The Share tab: the reference's empty state (\`pages/instance/share/index.vue\`).
///
/// This build cannot share an instance -- there is no account to share it with -- so the tab
/// states what it would hold rather than inventing an invitation flow.
fn share_tab(ui: &mut Ui, body: Rect, theme: &Theme) {
    empty_state(
        ui,
        body,
        theme,
        "No friends invited",
        "You can share this instance with your friends!",
        &[EmptyAction::Brand("Invite friends", Glyph::UserPlus)],
    );
}


/// The Files tab: the instance's game folder as the reference's table
/// (\`pages/instance/files/index.vue\`): a search field, a refresh button, and one row per entry
/// with its size and timestamps.
///
/// The reference's columns start at fixed points -- Name 133, Size 474, Created 622, Modified 830,
/// Actions 1037 -- and its rows are 61 points apart from the first row's 330 top. Both are taken
/// from the capture.
fn files_tab(
    ui: &mut Ui,
    body: Rect,
    theme: &Theme,
    instance: &Instance,
    dir: &std::path::Path,
) -> Option<std::path::PathBuf> {
    let mut descend: Option<std::path::PathBuf> = None;
    let folder = instance.path.join("minecraft").join(dir);
    let entries = folder_entries(&folder);
    let painter = ui.painter().clone();
    if dir.as_os_str().is_empty() {
        search_field(
            ui,
            theme,
            Rect::from_min_size(egui::pos2(body.left(), body.top()), egui::vec2(320.0, FILE_SEARCH_H)),
            "Search files",
        );
        let refresh = Rect::from_min_size(
            egui::pos2(body.left() + 332.0, body.top()),
            egui::vec2(88.0, FILE_SEARCH_H),
        );
        painter.rect_filled(refresh, 8, theme.BUTTON_BG);
        icons::draw(
            &painter,
            Glyph::RefreshCw,
            Rect::from_center_size(
                egui::pos2(refresh.left() + 18.0, refresh.center().y),
                egui::vec2(16.0, 16.0),
            ),
            theme.BASE,
        );
        painter.text(
            egui::pos2(refresh.left() + 32.0, refresh.center().y),
            Align2::LEFT_CENTER,
            "Refresh",
            fonts::semibold(14.0),
            theme.BASE,
        );
    } else {
        // A folder deeper than the root keeps a way back: the crumbs are its press targets.
        let mut x = body.left();
        let crumbs: Vec<(String, std::path::PathBuf)> = {
            let mut out: Vec<(String, std::path::PathBuf)> = Vec::new();
            out.push(("minecraft".to_owned(), std::path::PathBuf::new()));
            let mut acc = std::path::PathBuf::new();
            for part in dir.iter() {
                acc = acc.join(part);
                out.push((part.to_string_lossy().into_owned(), acc.clone()));
            }
            out
        };
        for (label, target) in crumbs {
            let text = painter.layout_no_wrap(label.clone(), fonts::body(12.0), theme.BRAND);
            let width = text.size().x + 16.0;
            let hit = Rect::from_min_size(egui::pos2(x, body.top()), egui::vec2(width, FILE_SEARCH_H));
            painter.text(
                egui::pos2(hit.left() + 8.0, hit.center().y),
                Align2::LEFT_CENTER,
                label,
                fonts::body(12.0),
                theme.BRAND,
            );
            if ui
                .interact(hit, egui::Id::new(("files-crumb", x as i32)), Sense::click())
                .clicked()
            {
                descend = Some(target);
            }
            x += width;
            painter.text(
                egui::pos2(x, hit.center().y),
                Align2::LEFT_CENTER,
                "/",
                fonts::body(12.0),
                theme.SECONDARY,
            );
            x += 10.0;
        }
    }

    // The column headings, at the capture's own x.
    let head_y = body.top() + FILE_HEAD_DY;
    for (label, x, right) in [
        ("Name", FILE_COL_NAME, false),
        ("Size", FILE_COL_SIZE, false),
        ("Created", FILE_COL_CREATED, false),
        ("Modified", FILE_COL_MODIFIED, false),
        ("Actions", FILE_COL_ACTIONS, false),
    ] {
        let _ = right;
        painter.text(
            egui::pos2(x, head_y),
            Align2::LEFT_TOP,
            label,
            fonts::semibold(14.0),
            theme.BASE,
        );
    }
    if entries.is_empty() {
        return None;
    }
    for (index, (name, is_dir, size)) in entries.iter().take(24).enumerate() {
        let top = body.top() + FILE_FIRST_ROW_DY + index as f32 * FILE_ROW_PITCH;
        if top + FILE_ROW_PITCH > body.bottom() {
            break;
        }
        icons::draw(
            &painter,
            if *is_dir { Glyph::Folder } else { Glyph::File },
            Rect::from_center_size(
                egui::pos2(FILE_COL_NAME - 16.0, top + 20.0),
                egui::vec2(16.0, 16.0),
            ),
            theme.SECONDARY,
        );
        painter.text(
            egui::pos2(FILE_COL_NAME, top + 20.0),
            Align2::LEFT_CENTER,
            name,
            fonts::semibold(14.0),
            theme.CONTRAST,
        );
        painter.text(
            egui::pos2(FILE_COL_SIZE, top + 20.0),
            Align2::LEFT_CENTER,
            if *is_dir { "0 items".to_owned() } else { format_size(*size) },
            fonts::body(14.0),
            theme.SECONDARY,
        );
        let row = Rect::from_min_size(
            egui::pos2(body.left(), top),
            egui::vec2(body.width(), FILE_ROW_PITCH),
        );
        if *is_dir
            && ui
                .interact(row, egui::Id::new(("file-row", name.as_str())), Sense::click())
                .clicked()
        {
            descend = Some(dir.join(name.as_str()));
        }
        icons::draw(
            &painter,
            Glyph::MoreVertical,
            Rect::from_center_size(
                egui::pos2(FILE_COL_ACTIONS + 8.0, top + 20.0),
                egui::vec2(16.0, 16.0),
            ),
            theme.SECONDARY,
        );
    }
    descend
}

/// A filled search field with the magnifier and a placeholder, as the reference's rows open with.
fn search_field(ui: &mut Ui, theme: &Theme, field: Rect, placeholder: &str) {
    let painter = ui.painter().clone();
    painter.rect_filled(field, 8, theme.BUTTON_BG);
    icons::draw(
        &painter,
        Glyph::Search,
        Rect::from_center_size(
            egui::pos2(field.left() + 16.0, field.center().y),
            egui::vec2(16.0, 16.0),
        ),
        theme.SECONDARY,
    );
    painter.text(
        egui::pos2(field.left() + 32.0, field.center().y),
        Align2::LEFT_CENTER,
        placeholder,
        fonts::body(14.0),
        theme.SECONDARY,
    );
    let _ = ui;
}

/// The Files search row's height.
const FILE_SEARCH_H: f32 = 40.0;
/// The table's heading row, from the body's top (the capture's 296).
const FILE_HEAD_DY: f32 = 66.0;
/// The first row's top, from the body's top (the capture's 330).
const FILE_FIRST_ROW_DY: f32 = 100.0;
/// The pitch between two rows: the capture's 61.
const FILE_ROW_PITCH: f32 = 61.0;
/// The table's column lefts, from the window's left edge.
const FILE_COL_NAME: f32 = 165.0;
const FILE_COL_SIZE: f32 = 474.0;
const FILE_COL_CREATED: f32 = 622.0;
const FILE_COL_MODIFIED: f32 = 830.0;
const FILE_COL_ACTIONS: f32 = 1037.0;


/// The Logs tab: the reference's console (\`pages/instance/logs/index.vue\`).
///
/// The reference draws a search field, a "Live Log" switch on the right, a filter row of Error /
/// Warn / Info chips, and a bordered console. With no instance running its console holds the
/// centred "Start your instance in the top right to start receiving live logs!" block; when the
/// game has written a log this page shows its tail inside the same border.
fn logs_tab(ui: &mut Ui, body: Rect, theme: &Theme, instance: &Instance) {
    let painter = ui.painter().clone();
    search_field(
        ui,
        theme,
        Rect::from_min_size(body.min, egui::vec2(320.0, FILE_SEARCH_H)),
        "Search logs",
    );
    let live = Rect::from_min_size(
        egui::pos2(body.right() - 138.0, body.top()),
        egui::vec2(138.0, FILE_SEARCH_H),
    );
    painter.rect_filled(live, 8, theme.BUTTON_BG);
    painter.text(
        egui::pos2(live.left() + 16.0, live.center().y),
        Align2::LEFT_CENTER,
        "Live Log",
        fonts::semibold(14.0),
        theme.BASE,
    );
    // The filter chips: Error, Warn, Info, with a divider between each pair.
    let chips_y = body.top() + 46.0;
    let mut x = body.left();
    for (label, colour) in [
        ("Error", theme.RED),
        ("Warn", theme.ORANGE),
        ("Info", theme.SECONDARY),
    ] {
        painter.circle_filled(egui::pos2(x + 4.0, chips_y + 12.0), 4.0, colour);
        painter.text(
            egui::pos2(x + 14.0, chips_y + 12.0),
            Align2::LEFT_CENTER,
            label,
            fonts::body(14.0),
            theme.BASE,
        );
        x += 92.0;
    }
    painter.text(
        egui::pos2(body.right(), chips_y + 12.0),
        Align2::RIGHT_CENTER,
        "Expand",
        fonts::body(14.0),
        theme.BASE,
    );
    // The console's frame, and its contents.
    let console = Rect::from_min_max(
        egui::pos2(body.left(), body.top() + CONSOLE_DY),
        egui::pos2(body.right(), body.bottom()),
    );
    painter.rect_filled(console, 12, theme.RAISED_BG);
    for line in ["Start your instance in the top right to start", "receiving live logs!"] {
        painter.text(
            egui::pos2(console.left() + 24.0, console.top() + 40.0 + 28.0 * f32::from(line.starts_with("receiving"))),
            Align2::LEFT_TOP,
            line,
            fonts::body(14.0),
            theme.BASE,
        );
    }
    if let Some(text) = log_text(&instance.path) {
        let fits = (((console.height() - 80.0) / ROW_H).floor() as usize).max(1);
        let mut y = console.top() + 40.0;
        for line in tail_lines(&text, fits) {
            painter.text(
                egui::pos2(console.left() + 24.0, y),
                Align2::LEFT_TOP,
                line,
                fonts::body(11.0),
                theme.SECONDARY,
            );
            y += ROW_H;
        }
    }
    let _ = ui;
}

/// The console frame's top, from the body's top: the capture's filter row ends at 310 and the
/// frame begins at 350.
const CONSOLE_DY: f32 = 120.0;


/// The instance's screenshots, newest first: the game names them by date.
fn screenshot_files(path: &std::path::Path) -> Vec<std::path::PathBuf> {
    for candidate in ["minecraft/screenshots", "screenshots"] {
        let folder = path.join(candidate);
        if !folder.is_dir() {
            continue;
        }
        let Ok(read) = std::fs::read_dir(&folder) else {
            continue;
        };
        let mut shots: Vec<std::path::PathBuf> = read
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| {
                matches!(
                    path.extension().and_then(|extension| extension.to_str()),
                    Some("png" | "jpg" | "jpeg")
                )
            })
            .collect();
        shots.sort();
        shots.reverse();
        return shots;
    }
    Vec::new()
}

/// The instance's worlds: `(folder, name)`, the name from `level.dat` and the folder when it has
/// none -- the same fallback the game uses.
fn saves(path: &std::path::Path) -> Vec<(String, String)> {
    for candidate in ["minecraft/saves", "saves"] {
        let saves = path.join(candidate);
        if !saves.is_dir() {
            continue;
        }
        let Ok(read) = std::fs::read_dir(&saves) else {
            continue;
        };
        let mut worlds: Vec<(String, String)> = read
            .flatten()
            .filter(|entry| entry.file_type().map(|kind| kind.is_dir()).unwrap_or(false))
            .filter_map(|entry| {
                let folder = entry.file_name().to_str()?.to_owned();
                let name = crate::nbt::world_name(&entry.path()).unwrap_or_else(|| folder.clone());
                Some((folder, name))
            })
            .collect();
        worlds.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()));
        return worlds;
    }
    Vec::new()
}

/// The last `count` lines of a log, which is what a log pane shows.
///
/// A real `latest.log` reaches megabytes and the pane can show a screenful, so the read is bounded
/// by the answer rather than by the file.
fn tail_lines(text: &str, count: usize) -> Vec<&str> {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines.len().saturating_sub(count);
    lines[start..].to_vec()
}

/// A byte count as the words a file browser uses: `842 B`, `1.4 KB`, `12.1 MB`, `1.2 GB`.
fn format_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    let value = bytes as f64;
    if value < KB {
        format!("{bytes} B")
    } else if value < KB * KB {
        format!("{:.1} KB", value / KB)
    } else if value < KB * KB * KB {
        format!("{:.1} MB", value / (KB * KB))
    } else {
        format!("{:.1} GB", value / (KB * KB * KB))
    }
}

/// The instance folder's entries, folders first then files, each with its size.
fn folder_entries(path: &std::path::Path) -> Vec<(String, bool, u64)> {
    let Ok(read) = std::fs::read_dir(path) else {
        return Vec::new();
    };
    let mut entries: Vec<(String, bool, u64)> = read
        .flatten()
        .filter_map(|entry| {
            let name = entry.file_name().to_str()?.to_owned();
            let metadata = entry.metadata().ok()?;
            let is_dir = metadata.is_dir();
            Some((name, is_dir, if is_dir { 0 } else { metadata.len() }))
        })
        .collect();
    // Folders first, then names, which is how every file browser a reader has used orders them.
    entries.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.to_lowercase().cmp(&b.0.to_lowercase())));
    entries
}

/// The text of the instance's newest log, or nothing when the game has not run.
fn log_text(path: &std::path::Path) -> Option<String> {
    for candidate in ["minecraft/logs/latest.log", "logs/latest.log"] {
        if let Ok(text) = std::fs::read_to_string(path.join(candidate)) {
            return Some(text);
        }
    }
    None
}


/// The loader and the game version on one line, joined exactly as the reference's `loaderLabel`
/// joins them: `[loader, game_version].filter(Boolean).join(' ')`
/// (`page-header/index.vue:325-327`).
fn loader_line(instance: &Instance) -> String {
    match (instance.loader.is_empty(), instance.version.is_empty()) {
        (true, _) => instance.version.clone(),
        (_, true) => instance.loader.clone(),
        _ => format!("{} {}", instance.loader, instance.version),
    }
}

/// The rows the reference's page header states -- loader and game version, then last played
/// (`page-header/index.vue:44-68`) -- plus the folder the page only offers as `Copy path`
/// (`layout.vue:821-826`). Each label is a catalogue string.
fn info_rows(instance: &Instance) -> [(&'static str, String); 4] {
    [
        // `app.library.filter.loader` (`index.json:1007-1009`).
        ("Loader", instance.loader.clone()),
        // `app.library.filter.game-version` (`index.json:989-991`).
        ("Game version", instance.version.clone()),
        // `instance.last-played` (`index.json:3935-3937`). The value is owed: the shell holds
        // the import's `last_played` stamp, and this page has no field for it yet.
        ("Last played", "\u{2014}".to_owned()),
        // `app.java-detection.columns.path` (`index.json:938-940`) is the catalogue's only
        // "Path" string; the instance page states its folder without a label of its own.
        ("Path", instance.path.display().to_string()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An instance with every field filled, so a test only has to change the one it is about.
    fn instance() -> Instance {
        Instance {
            id: "test-instance".to_owned(),
            name: "Test Instance".to_owned(),
            loader: "Fabric".to_owned(),
            version: "1.21.1".to_owned(),
            path: std::path::PathBuf::from("instances").join("test-instance"),
        }
    }

    /// The tabs are the reference's five, in its order, and each has a label: a tab that
    /// rendered an empty string would be an invisible target.
    #[test]
    fn the_tabs_are_named_and_ordered() {
        assert_eq!(
            Tab::ALL.map(Tab::label),
            // Six, not five: the reference's strip carries a Share tab (`layout.vue:195`,
            // `pages/instance/share/index.vue`), which this build was missing.
            ["Content", "Files", "Screenshots", "Worlds", "Logs", "Share"]
        );
    }

    /// The page opens on Content, which is the reference's landing route.
    #[test]
    fn an_instance_opens_on_its_content_tab() {
        assert_eq!(State::default().tab, Tab::Content);
    }

    /// The header's second line is the loader and the version, one space apart.
    #[test]
    fn the_loader_line_joins_loader_and_version() {
        assert_eq!(loader_line(&instance()), "Fabric 1.21.1");
    }

    /// An empty half is dropped rather than leaving a stray space, as the reference's
    /// `filter(Boolean)` does.
    #[test]
    fn the_loader_line_drops_an_empty_part() {
        let mut no_version = instance();
        no_version.version.clear();
        assert_eq!(loader_line(&no_version), "Fabric");

        let mut no_loader = instance();
        no_loader.loader.clear();
        assert_eq!(loader_line(&no_loader), "1.21.1");
    }

    /// The information rows are labelled and ordered as the reference's header states them,
    /// the path is the instance's own, and last played is still the owed placeholder.
    /// A log pane shows the end of the file, never the start, and never more lines than asked.
    #[test]
    fn a_tail_is_the_end_of_a_log() {
        let text = "one\ntwo\nthree\nfour";
        assert_eq!(tail_lines(text, 2), vec!["three", "four"]);
        assert_eq!(tail_lines(text, 99), vec!["one", "two", "three", "four"]);
        assert_eq!(tail_lines("", 5), Vec::<&str>::new());
    }

    /// A file browser's sizes read as words at every scale it meets.
    #[test]
    fn sizes_read_as_words() {
        assert_eq!(format_size(842), "842 B");
        assert_eq!(format_size(1536), "1.5 KB");
        assert_eq!(format_size(12_700_000), "12.1 MB");
        assert_eq!(format_size(2_147_483_648), "2.0 GB");
    }

    #[test]
    fn the_information_rows_are_labelled_and_ordered() {
        assert_eq!(
            info_rows(&instance()).map(|(label, _)| label),
            ["Loader", "Game version", "Last played", "Path"]
        );
        let rows = info_rows(&instance());
        assert_eq!(rows[2].1, "\u{2014}");
        assert_eq!(rows[3].1, instance().path.display().to_string());
    }
}

#[cfg(test)]
mod rename_tests {
    use super::rename_entry;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("sift-rename-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        dir
    }

    #[test]
    fn a_plain_rename_moves_the_file() {
        let dir = scratch("plain");
        std::fs::write(dir.join("a.txt"), b"x").expect("write");
        rename_entry(&dir, "a.txt", "b.txt").expect("ok");
        assert!(dir.join("b.txt").is_file());
        assert!(!dir.join("a.txt").exists());
    }

    #[test]
    fn an_empty_name_is_refused() {
        let dir = scratch("empty-name");
        std::fs::write(dir.join("a.txt"), b"x").expect("write");
        assert!(rename_entry(&dir, "a.txt", "   ").is_err());
        assert!(dir.join("a.txt").exists());
    }

    #[test]
    fn a_path_is_not_a_name() {
        let dir = scratch("path");
        std::fs::write(dir.join("a.txt"), b"x").expect("write");
        for bad in ["../escape.txt", "sub/a.txt", "..", "."] {
            assert!(rename_entry(&dir, "a.txt", bad).is_err(), "{bad} was accepted");
        }
        assert!(dir.join("a.txt").exists());
    }

    #[test]
    fn an_existing_name_is_refused_rather_than_overwritten() {
        let dir = scratch("clash");
        std::fs::write(dir.join("a.txt"), b"a").expect("write");
        std::fs::write(dir.join("b.txt"), b"b").expect("write");
        assert!(rename_entry(&dir, "a.txt", "b.txt").is_err());
        assert_eq!(std::fs::read(dir.join("b.txt")).expect("read"), b"b");
    }
}
