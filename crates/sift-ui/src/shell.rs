//! The window's furniture: title bar, rail, right panel, and the content column.
//!
//! **Laid out from measured rectangles, not from a responsive tree.** The parity
//! requirement is +/-2 px against the Modrinth app, so every element is placed with the
//! numbers in [`crate::geometry`] and painted into an explicit rect. A layout engine that
//! decides its own gaps would be re-deciding, every frame, something the captures already
//! answered.
//!
//! **Repaint only when something moves.** An idle window draws nothing: the frame loop
//! exists while a hover, a scroll glide or a transition is running and stops when it lands.
//! That is the `idle CPU 0%` budget in `docs/LIGHTNESS.md`, and it is the one place this
//! crate differs most from the slice it grew out of, which asked for a frame every vsync
//! whether or not anything had changed.

use egui::{Align2, Color32, Rect, Sense, Stroke, Ui, Vec2};

use crate::geometry as g;
use crate::icons::{self, Glyph};
use crate::theme::Theme;

/// The Minecraft version a quick-created instance gets.
///
/// The reference asks in a modal; until that modal exists, a version the launcher can resolve
/// is better than none, and it is one constant to change when the modal lands.
const DEFAULT_MC_VERSION: &str = "1.21.4";

/// A rail destination. The reference has five, separated from the utility group below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rail {
    /// The welcome screen and the instance list.
    Home,
    /// Find modpacks and content.
    Discover,
    /// Skins: the player's own and the catalogue's.
    Skins,
    /// Screenshots taken in any instance.
    Screenshots,
    /// Servers the launcher knows about.
    Servers,
}

impl Rail {
    /// Every destination, in the reference's order.
    pub const ALL: [Rail; 5] = [
        Rail::Home,
        Rail::Discover,
        Rail::Skins,
        Rail::Screenshots,
        Rail::Servers,
    ];

    /// The name the reference prints in the page header.
    pub fn title(self) -> &'static str {
        match self {
            Rail::Home => "Home",
            Rail::Discover => "Discover modpacks",
            Rail::Skins => "Skins",
            Rail::Screenshots => "Screenshots",
            Rail::Servers => "Servers",
        }
    }

    /// The name the rail's tooltip shows on hover, which is shorter than the page title.
    pub fn short(self) -> &'static str {
        match self {
            Rail::Discover => "Discover",
            // The reference's tooltip and heading both say `Skin selector`.
            Rail::Skins => "Skin selector",
            // The reference names its hosting product here instead of the page's heading, which
            // is `Servers`. Ours is Sift Hosting, as every other string in this build is.
            Rail::Servers => "Sift Hosting",
            other => other.title(),
        }
    }

    /// The glyph the reference draws for this destination.
    pub fn glyph(self) -> Glyph {
        match self {
            Rail::Home => Glyph::Play,
            Rail::Discover => Glyph::Compass,
            Rail::Skins => Glyph::Shirt,
            Rail::Screenshots => Glyph::Image,
            Rail::Servers => Glyph::ServerStack,
        }
    }
}

/// What the shell is showing, and what is moving.
/// The four ways the reference's create dialog offers to make an instance
/// (`CreateInstanceModal.vue`, captured at `.scratch/cap/ref-create4.png`): the title, the line
/// under it, and whether the row opens a further step rather than finishing here.
///
/// The loader and game version live one level deeper, behind `Custom setup` -- that is the row
/// this launcher's single-press Vanilla instance replaced, and it is why a Fabric instance cannot
/// exist yet.
impl Shell {
    /// The reference's create dialog (`.scratch/cap/ref-create4.png`): a 576x350 plate with a
    /// title and a close button, a project search under "Already know what you want to play?",
    /// an "or", and the four instance types.
    ///
    /// Drawn on a foreground layer so it sits over every page, which is where a modal belongs
    /// and is why this is a method taking the context rather than part of `ui`.
    pub fn draw_create(&mut self, ctx: &egui::Context, screen: egui::Rect) {
        let theme = &self.theme;
        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new("create-dialog"),
        ));
        painter.rect_filled(screen, 0.0, egui::Color32::from_black_alpha(120));
        let plate = egui::Rect::from_center_size(screen.center(), egui::vec2(576.0, 350.0));
        // A modal has to be dismissable: the reference closes on Escape and on a click outside
        // the plate. Without this the dialog is a trap, which is what the capture showed --
        // every row and the close mark drawn, none of them answering.
        let (pointer, clicked, escaped) = ctx.input(|input| {
            (
                input.pointer.interact_pos(),
                input.pointer.any_click(),
                input.key_pressed(egui::Key::Escape),
            )
        });
        let settled = ctx.input(|input| input.time) > self.create_opened + 0.15;
        if escaped || (settled && clicked && pointer.is_some_and(|point| !plate.contains(point))) {
            self.create_open = false;
            ctx.request_repaint();
            return;
        }
        painter.rect_filled(plate, 12.0, theme.RAISED_BG);
        painter.rect_stroke(
            plate,
            12.0,
            egui::Stroke::new(1.0, theme.SURFACE_5),
            egui::StrokeKind::Inside,
        );
        let left = plate.left() + 16.0;
        let right = plate.right() - 16.0;
        painter.text(
            egui::pos2(left, plate.top() + 18.0),
            egui::Align2::LEFT_TOP,
            "Create instance",
            crate::fonts::semibold(18.0),
            theme.CONTRAST,
        );
        painter.text(
            egui::pos2(right, plate.top() + 18.0),
            egui::Align2::RIGHT_TOP,
            "✕",
            crate::fonts::body(16.0),
            theme.SECONDARY,
        );
        painter.hline(
            left..=right,
            plate.top() + 48.0,
            egui::Stroke::new(1.0, theme.SURFACE_5),
        );
        // Step one: what "Custom setup" leads to. The reference's next screen asks for a name, a
        // loader and a game version; this draws the three rows it will fill, so the step exists
        // and can be walked before any of them takes input.
        if self.create_step == 1 {
            for (index, (label, value)) in [
                ("Name", "Name your instance"),
                ("Loader", "Fabric"),
                ("Game version", "1.21.4"),
            ]
            .into_iter()
            .enumerate()
            {
                let row = egui::Rect::from_min_size(
                    egui::pos2(left, plate.top() + 62.0 + index as f32 * 62.0),
                    egui::vec2(right - left, 52.0),
                );
                painter.text(
                    egui::pos2(row.left(), row.top()),
                    egui::Align2::LEFT_TOP,
                    label,
                    crate::fonts::semibold(13.0),
                    theme.CONTRAST,
                );
                let field = egui::Rect::from_min_size(
                    egui::pos2(row.left(), row.top() + 20.0),
                    egui::vec2(row.width(), 28.0),
                );
                painter.rect_filled(field, 8.0, theme.BUTTON_BG);
                painter.text(
                    egui::pos2(field.left() + 10.0, field.center().y),
                    egui::Align2::LEFT_CENTER,
                    value,
                    crate::fonts::body(13.0),
                    theme.SECONDARY,
                );
            }
            return;
        }
        painter.text(
            egui::pos2(left, plate.top() + 62.0),
            egui::Align2::LEFT_TOP,
            "Already know what you want to play?",
            crate::fonts::semibold(14.0),
            theme.CONTRAST,
        );
        let search = egui::Rect::from_min_size(
            egui::pos2(left, plate.top() + 84.0),
            egui::vec2(right - left, 28.0),
        );
        painter.rect_filled(search, 8.0, theme.BUTTON_BG);
        painter.text(
            egui::pos2(search.left() + 10.0, search.center().y),
            egui::Align2::LEFT_CENTER,
            "Search mods, modpacks, and more...",
            crate::fonts::body(13.0),
            theme.SECONDARY,
        );
        painter.text(
            egui::pos2(plate.center().x, plate.top() + 120.0),
            egui::Align2::CENTER_TOP,
            "or",
            crate::fonts::body(12.0),
            theme.SECONDARY,
        );
        painter.text(
            egui::pos2(left, plate.top() + 142.0),
            egui::Align2::LEFT_TOP,
            "Choose instance type",
            crate::fonts::semibold(14.0),
            theme.CONTRAST,
        );
        let close = egui::Rect::from_min_size(
            egui::pos2(plate.right() - 40.0, plate.top() + 12.0),
            egui::vec2(24.0, 24.0),
        );
        if settled && clicked && pointer.is_some_and(|point| close.contains(point)) {
            self.create_open = false;
            ctx.request_repaint();
            return;
        }
        for (index, (title, description, _)) in INSTANCE_TYPES.iter().enumerate() {
            let row = egui::Rect::from_min_size(
                egui::pos2(left, plate.top() + 166.0 + index as f32 * 42.0),
                egui::vec2(right - left, 38.0),
            );
            if settled && clicked && pointer.is_some_and(|point| row.contains(point)) {
                self.create_choice = Some(index);
                if index == 0 {
                    self.create_step = 1;
                }
                ctx.request_repaint();
            }
            painter.rect_filled(row, 10.0, theme.SURFACE_4);
            if self.create_choice == Some(index) {
                painter.rect_stroke(
                    row,
                    10.0,
                    egui::Stroke::new(1.0, theme.BRAND),
                    egui::StrokeKind::Inside,
                );
            }
            painter.text(
                egui::pos2(row.left() + 14.0, row.top() + 7.0),
                egui::Align2::LEFT_TOP,
                *title,
                crate::fonts::semibold(13.0),
                theme.CONTRAST,
            );
            painter.text(
                egui::pos2(row.left() + 14.0, row.top() + 22.0),
                egui::Align2::LEFT_TOP,
                *description,
                crate::fonts::body(11.0),
                theme.SECONDARY,
            );
        }
    }
}

pub const INSTANCE_TYPES: [(&str, &str, bool); 4] = [
    (
        "Custom setup",
        "Start from scratch by picking a loader and game version",
        true,
    ),
    (
        "Start from a mod or modpack",
        "Choose a project and we'll use its latest version",
        true,
    ),
    (
        "Upload a modpack",
        "Install a modpack from an .mrpack file on your device",
        false,
    ),
    (
        "Import instance",
        "Import an instance from Prism, CurseForge, or similar.",
        false,
    ),
];

pub struct Shell {
    /// Whether the reference's create dialog is open. The rail's plus sets it, as the
    /// reference's does, where this launcher used to create a Vanilla instance on the press.
    pub create_open: bool,
    /// When it opened, on the frame clock. The press that opens the dialog is also a click
    /// outside its plate, so the dismissal has to know it is not the same press.
    pub create_opened: f64,
    /// Which instance type the reader has chosen, if any. The four rows answer a press by
    /// remembering it; the flows behind them come one at a time.
    pub create_choice: Option<usize>,
    /// Which step of the create flow is showing: 0 chooses an instance type, 1 is the custom
    /// setup the first row leads to. The reference's other three rows lead elsewhere and are
    /// each their own step when they exist.
    pub create_step: u8,
    /// The selected rail destination.
    pub rail: Rail,
    /// The hovered rail destination, if any. A press animates between the two, which is
    /// why this is state rather than a per-frame read.
    hovered: Option<Rail>,
    /// fastframe's scroll state: the wheel step, the touchpad scale and the glide.
    scrolling: fastframe_scroll::Scrolling,
    /// The colour scheme, from the generated tokens.
    pub theme: Theme,
    /// Project icons, fetched off the frame thread and cached as textures.
    icons: crate::images::Images,
    /// The one game this shell starts, and the worker that reports on it.
    game: crate::game::Game,
    /// The pack install behind the Discover cards' Install buttons, and its worker.
    install: crate::install::Install,
    /// The instances on disk, read when the shell starts and when the rail returns to Home.
    instances: Vec<crate::data::Entry>,
    /// The Home page's own state: which row's kebab popover is open, and the `saves/` names
    /// it read. Kept here beside the other pages' states so the popover survives the frames
    /// that also draw the rail and the panel.
    home: crate::pages::home::State,
    /// The project whose page is open, if any. `None` means the rail's own page is showing.
    open_project: Option<crate::pages::project::Project>,
    /// The open project's tab.
    project_state: crate::pages::project::State,
    /// The Discover search field: its text, its focus and its blink phase.
    search: crate::widgets::search::Search,
    /// The network search behind that field.
    results: crate::search::Search,
    /// Which page of the results the reader is on, counted from zero.
    page: u32,
    /// The question the page belongs to, so a new one starts at the first page.
    asked: String,
    /// The categories the API lists for the current project type, empty until they arrive.
    categories: Vec<String>,
    /// Whether the category list has been asked for.
    categories_asked: bool,
    /// Whether the category list shows all of them or just the first few.
    categories_expanded: bool,
    /// Whether results whose project is already installed are hidden.
    hide_installed: bool,
    /// The worker's answer, until it is taken.
    categories_rx: Option<std::sync::mpsc::Receiver<Vec<String>>>,
    /// The chosen category as a facet group, e.g. `["categories:adventure"]`; `None` is no
    /// filter at all.
    facet: Option<String>,
    /// Which of the tab strip's kinds the search is for: `modpack`, `mod`, `resourcepack`,
    /// `datapack`, `shader` or `plugin`.
    project_type: &'static str,
    /// The API's sort index: `relevance`, `downloads`, `follows`, `newest`, `updated`.
    sort_index: &'static str,
    /// The API's page size, which the reference shows as `View: 20`.
    page_size: u32,
    /// Which select's popover is open: 0 for the sort, 1 for the page size. The reference's
    /// selects are popovers with their options listed; a press that only cycles the value
    /// cannot show a reader what the options are.
    open_select: Option<usize>,
    /// The skins page's own state.
    skins: crate::pages::skins::State,
    /// The screenshots page's own state.
    screenshots: crate::pages::screenshots::State,
    /// The servers page's own state.
    servers: crate::pages::servers::State,
    /// A one-line message shown after an action, until the next one.
    toast: Option<String>,
    /// The instance whose page is open, if any.
    open_instance: Option<crate::pages::instance::Instance>,
    /// The open instance page's selected tab.
    instance_state: crate::pages::instance::State,
    /// The Microsoft account, and the worker that talks to it.
    account: crate::account::Account,
    /// Whether the stored account has been read, which happens once.
    restored_account: bool,
    /// Whether the About modal is showing.
    about_open: bool,
    /// The settings modal: open flag and selected tab.
    settings: crate::pages::settings::State,
    /// The theme the window is drawn in, so a change can be noticed once rather than every frame.
    theme_choice: crate::pages::settings::ThemeChoice,
}

impl Default for Shell {
    fn default() -> Shell {
        Shell {
            // The create dialog starts closed: the reference opens on Home with nothing over it.
            create_open: false,
            create_opened: 0.0,
            create_choice: None,
            create_step: 0,
            // The reference opens on Home, and a launcher that opens on a catalogue looks
            // like a store rather than a launcher.
            rail: Rail::Home,
            hovered: None,
            scrolling: fastframe_scroll::Scrolling::default(),
            theme: crate::theme::dark(),
            icons: crate::images::Images::default(),
            game: crate::game::Game::default(),
            install: crate::install::Install::default(),
            instances: crate::data::load().0,
            home: crate::pages::home::State::default(),
            open_project: None,
            project_state: crate::pages::project::State::default(),
            search: crate::widgets::search::Search::default(),
            results: crate::search::Search::default(),
            page: 0,
            asked: String::new(),
            facet: None,
            categories: Vec::new(),
            categories_asked: false,
            categories_expanded: false,
            hide_installed: false,
            categories_rx: None,
            project_type: "modpack",
            sort_index: "relevance",
            page_size: 20,
            open_select: None,
            skins: crate::pages::skins::State::default(),
            screenshots: crate::pages::screenshots::State::default(),
            servers: crate::pages::servers::State::default(),
            toast: None,
            open_instance: None,
            instance_state: crate::pages::instance::State::default(),
            account: crate::account::Account::default(),
            restored_account: false,
            about_open: false,
            settings: crate::pages::settings::State::default(),
            theme_choice: crate::pages::settings::ThemeChoice::Dark,
        }
    }
}

/// One colour laid over another at \`alpha\`, the way a CSS \`rgba()\` background does over
/// the element behind it.
pub fn tint(base: Color32, over: (u8, u8, u8), alpha: f32) -> Color32 {
    let mix = |b: u8, o: u8| -> u8 {
        (b as f32 * (1.0 - alpha) + o as f32 * alpha).round().clamp(0.0, 255.0) as u8
    };
    Color32::from_rgb(
        mix(base.r(), over.0),
        mix(base.g(), over.1),
        mix(base.b(), over.2),
    )
}

/// A vertical two-stop gradient, which is what \`linear-gradient(0deg, a 0%, b 100%)\` is.
///
/// egui has no gradient primitive, so this is a quad whose two top vertices carry one
/// colour and two bottom vertices the other: one draw call, and the interpolation is the
/// GPU's own, which is the same arithmetic a browser does.
pub fn vertical_gradient(painter: &egui::Painter, rect: Rect, top: Color32, bottom: Color32) {
    let mut mesh = egui::Mesh::default();
    let base = mesh.vertices.len() as u32;
    mesh.colored_vertex(rect.left_top(), top);
    mesh.colored_vertex(rect.right_top(), top);
    mesh.colored_vertex(rect.left_bottom(), bottom);
    mesh.colored_vertex(rect.right_bottom(), bottom);
    mesh.add_triangle(base, base + 1, base + 2);
    mesh.add_triangle(base + 1, base + 2, base + 3);
    painter.add(egui::Shape::mesh(mesh));
}

impl Shell {
    /// Draw one frame. Returns whether anything is still animating, which the caller uses
    /// to decide whether to ask for another frame.
    pub fn ui(&mut self, ui: &mut Ui) -> bool {
        let ctx = ui.ctx().clone();
        // First thing in the frame, before any scroll area reads the input.
        self.scrolling.apply(&ctx);
        // Take the launch worker's latest word before any of the window reads it.
        self.game.poll();
        // The account a previous run left, once: the profile shows immediately and its token is
        // renewed behind it.
        if !self.restored_account {
            self.restored_account = true;
            self.account.restore();
        }
        // The sign-in worker's, which announces the code the moment Microsoft prints one.
        let was = self.account.state().clone();
        self.account.poll();
        // The launch reads the account when it starts, so it is told here rather than at the
        // press: a sign-in that finished one frame before a Play must be the one that launches.
        self.game.set_account(
            self.account
                .launch_identity(),
        );
        if self.account.state() != &was {
            self.toast = Some(match self.account.state() {
                crate::account::State::Waiting { code, uri } => {
                    format!("To sign in, enter {code} at {uri}")
                }
                crate::account::State::SignedIn { name, .. } => {
                    format!("Signed in as {name}.")
                }
                crate::account::State::Failed(reason) => format!("Sign-in failed: {reason}"),
                // Neither state arrives as a change here: the press says `Starting`, and
                // `Idle` is only ever set by signing out.
                crate::account::State::Starting | crate::account::State::Idle => String::new(),
            });
        }
        // The pack install's word, and the sentence it owes when one ends. The toast names
        // the instance a finished install created, and the list is re-read so Home shows it:
        // a failed install can leave a folder behind too, which is why both outcomes do.
        if let Some(line) = self.install.poll() {
            self.toast = Some(line);
            self.instances = crate::data::load().0;
        }

        let window = Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(g::WINDOW.0, g::WINDOW.1));
        ui.painter().rect_filled(window, 0, self.theme.BG);

        self.title_bar(ui);
        // The surface under every page, before the chrome that borders it.
        self.content_surface(ui);
        let mut rail_animating = false;
        rail_animating |= self.rail_column(ui);
        rail_animating |= self.right_panel(ui);

        let content = if let Some(instance) = self.open_instance.clone() {
            let rect = crate::pages::Page::Instance.rect();
            ui.painter().rect_filled(rect, 0, self.theme.BG);
            let theme = self.theme;
            let result = crate::pages::instance::draw(
                ui,
                rect,
                &theme,
                &mut self.icons,
                &mut self.instance_state,
                &instance,
            );
            let _ = result;
            false
        } else if let Some(project) = &self.open_project {
            let rect = crate::pages::Page::Project.rect();
            ui.painter().rect_filled(rect, 0, self.theme.BG);
            // The borrow of `project` and the mutable borrows the page needs are disjoint
            // fields of `self`, which is why the project is taken out first.
            let open = self.open_project.clone();
            match open {
                Some(project) => {
                    let theme = self.theme;
                    let result = crate::pages::project::draw(
                        ui,
                        rect,
                        &theme,
                        &mut self.icons,
                        &mut self.project_state,
                        &project,
                    );
                    let _ = project;
                    result
                }
                None => false,
            }
        } else if self.rail == Rail::Screenshots {
            let rect = crate::pages::Page::Screenshots.rect();
            ui.painter().rect_filled(rect, 0, self.theme.BG);
            let theme = self.theme;
            let result = crate::pages::screenshots::draw(
                ui,
                rect,
                &theme,
                &mut self.icons,
                &mut self.screenshots,
            );
            result
        } else if self.rail == Rail::Servers {
            let rect = crate::pages::Page::Servers.rect();
            ui.painter().rect_filled(rect, 0, self.theme.BG);
            let theme = self.theme;
            let result = crate::pages::servers::draw(
                ui,
                rect,
                &theme,
                &mut self.icons,
                &mut self.servers,
            );
            result
        } else if self.rail == Rail::Skins {
            // The skins page owns its own state; the shell keeps it beside the others so the
            // selection survives a trip to another page.
            let rect = crate::pages::Page::Skins.rect();
            ui.painter().rect_filled(rect, 0, self.theme.BG);
            let theme = self.theme;
            let result = crate::pages::skins::draw(
                ui,
                rect,
                &theme,
                &mut self.icons,
                &mut self.skins,
            );
            result
        } else if self.rail == Rail::Home {
            let rect = crate::pages::Page::Home.rect();
            ui.painter().rect_filled(rect, 0, self.theme.BG);
            let theme = self.theme;
            let action = crate::pages::home::draw(
                ui,
                rect,
                &theme,
                &mut self.icons,
                &self.instances,
                &mut self.home,
            );
            // The page reports intent; the shell owns the paths, the disk and the process
            // table, so it is the shell that acts.
            self.act(action);
            false
        } else {
            self.content_column(ui)
        };

        // The About modal is drawn over the whole window, last, so nothing paints on top of
        // it. It is static while open -- no tween, no hover -- so it owes no frame debt: an
        // open modal that asked for a repaint every vsync would be the one thing in this
        // window failing the idle-CPU budget. What it does owe is a frame when it closes, so
        // the page underneath is not left half-covered by a stale buffer.
        // Both modals are static while open, so neither owes a frame: what they owe is one
        // repaint on the frame they close, so the page underneath is not left half-covered.
        let was_about = self.about_open;
        let was_settings = self.settings.open;
        crate::pages::about::draw(ui, Self::window_rect(), &self.theme, &mut self.about_open);
        crate::pages::settings::draw(ui, Self::window_rect(), &self.theme, &mut self.settings);
        // The theme cards choose; the shell owns the theme. A press arrives as `chosen_theme`,
        // the stored choice arrives in `theme` with the first load, and one expression covers
        // both -- the earlier pair of `if`s left the window dark on a press, because the press
        // had already set both fields equal.
        let requested = self.settings.chosen_theme.take().unwrap_or(self.settings.theme);
        if requested != self.theme_choice {
            self.theme_choice = requested;
            let chosen = match requested {
                crate::pages::settings::ThemeChoice::Dark => crate::theme::dark(),
                crate::pages::settings::ThemeChoice::Light => crate::theme::light(),
                crate::pages::settings::ThemeChoice::Oled => crate::theme::oled(),
                // `Sync with system` has no system preference to ask on this build, so it keeps
                // the scheme it was drawn in; the choice is remembered and shown.
                crate::pages::settings::ThemeChoice::System => self.theme,
            };
            if chosen != self.theme {
                self.theme = chosen;
                ui.ctx().request_repaint();
            }
        }
        if was_about != self.about_open || was_settings != self.settings.open {
            ui.ctx().request_repaint();
        }

        // The surface's shadow last of the page layer: it is an overlay in the reference
        // too, so the page underneath cannot paint over it.
        self.content_shadow(ui);

        // The reference labels the destination under the pointer with a small pill beside it.
        // Here rather than with the rail for the toast's own reason: the rail is painted before
        // the content column, so a tooltip drawn there is covered by the page.
        // `providers/tooltip.ts:35-36` waits 300 ms before showing and 400 ms before hiding;
        // the pill itself enters over 0.1 s from scale 0.9 with ease-out
        // (`assets/styles/classes.scss:644-665`). The delay is the part a reader notices, so
        // it is the part wired here; the 400 ms hide delay is not, and the pill leaves on
        // the frame the pointer does.
        let held = self
            .hovered
            .map(|_| crate::motion::held_for(ui.ctx(), egui::Id::new("rail-tooltip"), true))
            .unwrap_or(None);
        let tooltip_due = held.map(|ms| ms >= 300.0).unwrap_or(false);
        if let Some(destination) = self.hovered.filter(|_| tooltip_due) {
            let painter = ui.painter();
            let index = Rail::ALL
                .iter()
                .position(|candidate| *candidate == destination)
                .unwrap_or(0) as f32;
            let y = g::TITLE_BAR + g::RAIL_FIRST + index * g::RAIL_PITCH + g::RAIL_BUTTON / 2.0;
            let label = painter.layout_no_wrap(
                destination.short().to_owned(),
                crate::fonts::body(12.0),
                self.theme.ACCENT_CONTRAST,
            );
            let pill = egui::Rect::from_min_size(
                egui::pos2(g::RAIL_PAD + g::RAIL_BUTTON + 6.0, y - label.size().y / 2.0 - 5.0),
                egui::vec2(label.size().x + 16.0, label.size().y + 10.0),
            );
            painter.rect_filled(pill, 6, self.theme.CONTRAST);
            painter.galley(
                egui::pos2(pill.left() + 8.0, pill.top() + 5.0),
                label,
                self.theme.ACCENT_CONTRAST,
            );
        }

        // Last, because the content column and every page paint their own background:
        // a toast drawn before them is a toast covered up.
        // The toast: one line, bottom-centre over the content column. A message that reported
        // an action and vanished at the frame's end would be invisible.
        if let Some(message) = self.toast.clone() {
            let painter = ui.painter();
            let galley =
                painter.layout_no_wrap(message, crate::fonts::body(13.0), self.theme.CONTRAST);
            let plate = Rect::from_center_size(
                egui::pos2((g::CONTENT_LEFT + g::PANEL_LEFT) / 2.0, g::WINDOW.1 - 56.0),
                egui::vec2(galley.size().x + 32.0, 36.0),
            );
            painter.rect_filled(plate, 8, self.theme.SUPER_RAISED_BG);
            painter.galley(
                egui::pos2(
                    plate.center().x - galley.size().x / 2.0,
                    plate.center().y - galley.size().y / 2.0,
                ),
                galley,
                self.theme.CONTRAST,
            );
        }

        // An icon in flight is a reason to draw the next frame, exactly like a hover.
        rail_animating
            || content
            || self.icons.loading()
            || self.results.loading()
            || self.game.busy()
            || self.install.busy()
            // The sign-in worker's next word arrives on its own thread, so the window has to
            // keep drawing to notice it.
            || self.account.busy()
    }

    /// The one surface every page sits on.
    ///
    /// \`.app-contents\` in the reference is a single element: \`--color-bg\`,
    /// \`border-top-left-radius: var(--radius-xl)\` (1.25rem = 20 px), stretching from the
    /// rail to the window's right edge and from the title bar to the floor, with
    /// \`box-shadow: inset 1px 1px 15px #0003, inset 1px 1px 1px #ffffff3b\` along its top and
    /// left edges. It is the seam against the chrome -- there is no rule under the title bar
    /// and none beside the rail -- so a launcher that draws those rules instead gets a
    /// visibly different frame around every page.
    ///
    /// Only the top-left corner is rounded: the other three meet the window's edges.
    fn content_surface(&self, ui: &mut Ui) {
        let surface = Rect::from_min_max(
            egui::pos2(g::CONTENT_LEFT, g::TITLE_BAR),
            egui::pos2(g::WINDOW.0, g::WINDOW.1),
        );
        let painter = ui.painter();
        painter.rect_filled(surface, 0, self.theme.BG);

        // Cut the corner: fill the square back in with the chrome's own colour, then put
        // the surface's colour back as a disc inscribed in it.
        let r = g::RADIUS_XL;
        // The square's outer half is chrome; the disc that carves it is centred on the
        // square's bottom-right, so its arc is the corner's arc. (Centring it on the square
        // itself would put a 20 px disc where a 10 px quarter belongs, which shows up as a
        // notch with a straight diagonal edge.)
        let corner = Rect::from_min_size(surface.min, Vec2::splat(r));
        painter.rect_filled(corner, 0, self.theme.RAISED_BG);
        painter.circle_filled(corner.right_bottom(), r, self.theme.BG);

    }

    /// The surface's inset shadow, painted **after** the page.
    ///
    /// The reference draws it as `.app-contents:before`, a fixed overlay at `z-index: 1`:
    /// above the viewport's content, below the modals. Painting it with the surface instead
    /// would let the first page that fills its own rect with `--color-bg` erase it, which is
    /// exactly what the Home page does.
    fn content_shadow(&self, ui: &mut Ui) {
        let surface = Rect::from_min_max(
            egui::pos2(g::CONTENT_LEFT, g::TITLE_BAR),
            egui::pos2(g::WINDOW.0, g::WINDOW.1),
        );
        let painter = ui.painter();
        let r = g::RADIUS_XL;
        // 15 px of #0003 under a 1 px #ffffff3b highlight. Drawn as bands of falling alpha
        // because egui has no inner-shadow primitive; 15 bands on two edges is cheaper than
        // a per-pixel pass and indistinguishable at these alphas.
        for i in 0..15 {
            let t = i as f32 / 15.0;
            let alpha = (1.0 - t) * (1.0 - t) * 51.0;
            let colour = Color32::from_rgba_unmultiplied(0, 0, 0, alpha as u8);
            let inset = i as f32;
            painter.hline(
                (surface.left() + r)..=surface.right(),
                surface.top() + inset + 0.5,
                Stroke::new(1.0, colour),
            );
            painter.vline(
                surface.left() + inset + 0.5,
                (surface.top() + r)..=surface.bottom(),
                Stroke::new(1.0, colour),
            );
        }
        let highlight = Color32::from_rgba_unmultiplied(255, 255, 255, 59);
        painter.hline(
            (surface.left() + r)..=surface.right(),
            surface.top() + 0.5,
            Stroke::new(1.0, highlight),
        );
        painter.vline(
            surface.left() + 0.5,
            (surface.top() + r)..=surface.bottom(),
            Stroke::new(1.0, highlight),
        );
    }

    /// The top bar, at the reference's own 3rem.
    ///
    /// Measured on the running client at 1440x900 (\`.scratch/cap/ref-home.png\`): the strip
    /// is 48 px of \`--color-raised-bg\` with **no rule under it** -- the seam is the content
    /// surface's rounded top-left corner, not a border. The wordmark sits at x 12..162,
    /// y 10..37; back and forward are 28 px outlined squares at x 176 and 212; the route's
    /// icon starts at 262 and its name at 283; the status line ends at 1278, six pixels
    /// before the window controls, which are 48 + 48 + 60 wide.
    fn title_bar(&mut self, ui: &mut Ui) {
        let bar = Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(g::WINDOW.0, g::TITLE_BAR));
        let painter = ui.painter();
        painter.rect_filled(bar, 0, self.theme.RAISED_BG);

        // The mark and the product name. The reference draws its wordmark as an SVG
        // \`h-7\` tall; this launcher has no wordmark asset, so the name is set in the
        // semibold face at the same height rather than stretched from the mark.
        let mark = Rect::from_min_size(
            egui::pos2(12.0, (g::TITLE_BAR - 28.0) / 2.0),
            Vec2::splat(28.0),
        );
        if !crate::brand::paint_mark(painter, ui.ctx(), mark) {
            painter.rect_filled(mark, 7, self.theme.BRAND);
        }
        painter.text(
            egui::pos2(mark.right() + 8.0, bar.center().y),
            Align2::LEFT_CENTER,
            "Sift Launcher",
            crate::fonts::semibold(16.0),
            self.theme.CONTRAST,
        );

        // Back and forward: \`IconButton type="outlined"\` with \`!h-7 !w-7 !border\`, which is
        // a 28 px square with a one-pixel \`--surface-4\` border and a 16 px glyph. Both are
        // disabled on a page with no history, and the reference dims the glyph to 20%.
        for (index, glyph) in [Glyph::ChevronLeft, Glyph::ChevronRight]
            .into_iter()
            .enumerate()
        {
            let button = Rect::from_min_size(
                egui::pos2(
                    g::BREADCRUMB_ARROWS + index as f32 * (g::ARROW_BUTTON + g::ARROW_GAP),
                    (g::TITLE_BAR - g::ARROW_BUTTON) / 2.0,
                ),
                Vec2::splat(g::ARROW_BUTTON),
            );
            painter.rect_stroke(
                button,
                8,
                Stroke::new(1.0, self.theme.BUTTON_BG),
                egui::StrokeKind::Inside,
            );
            let ink = self.theme.SECONDARY.gamma_multiply(0.2);
            icons::draw(
                painter,
                glyph,
                Rect::from_center_size(button.center(), Vec2::splat(16.0)),
                ink,
            );
        }

        // The route: the page's own glyph, then its name on the same baseline.
        let badge = Rect::from_min_size(
            egui::pos2(g::BREADCRUMB_BADGE, (g::TITLE_BAR - 16.0) / 2.0),
            Vec2::splat(16.0),
        );
        icons::draw(painter, self.rail.glyph(), badge, self.theme.SECONDARY);
        // A project page's breadcrumb is \`rail > project\`, which is why the name comes
        // from the open project when there is one.
        let crumb = match &self.open_project {
            Some(project) => format!("{}  >  {}", self.rail.title(), project.title),
            None => self.rail.title().to_owned(),
        };
        painter.text(
            egui::pos2(badge.right() + 5.0, bar.center().y),
            Align2::LEFT_CENTER,
            crumb,
            crate::fonts::body(14.0),
            self.theme.CONTRAST,
        );

        // The action bar's two pills, measured off the capture: the Update pill spans
        // x 999..1096 and the status pill x 1110..1285, both 34 px tall (h-8 plus the ring)
        // and fully rounded. Update is brand text on a brand ring; the status pill is
        // --color-secondary text after an 8 px dot, on a --surface-5 ring.
        let pill_top = (g::TITLE_BAR - 34.0) / 2.0;
        let update = Rect::from_min_size(egui::pos2(999.0, pill_top), egui::vec2(98.0, 34.0));
        painter.rect_stroke(
            update,
            17,
            Stroke::new(1.0, self.theme.BRAND),
            egui::StrokeKind::Inside,
        );
        icons::draw(
            painter,
            Glyph::Download,
            Rect::from_center_size(
                egui::pos2(update.left() + 20.0, update.center().y),
                Vec2::splat(16.0),
            ),
            self.theme.BRAND,
        );
        painter.text(
            egui::pos2(update.left() + 42.0, update.center().y),
            Align2::LEFT_CENTER,
            "Update",
            crate::fonts::semibold(14.0),
            self.theme.BRAND,
        );

        let status = Rect::from_min_size(egui::pos2(1110.0, pill_top), egui::vec2(176.0, 34.0));
        painter.rect_stroke(
            status,
            17,
            Stroke::new(1.0, self.theme.SURFACE_5),
            egui::StrokeKind::Inside,
        );
        painter.circle_filled(
            egui::pos2(status.left() + 12.0, status.center().y),
            4.0,
            self.theme.SECONDARY,
        );
        painter.text(
            egui::pos2(status.left() + 25.0, status.center().y),
            Align2::LEFT_CENTER,
            self.game.status().to_string(),
            crate::fonts::body(14.0),
            self.theme.SECONDARY,
        );

        // height; the reference's hover is a circle the size of the button, and the close
        // button's hover turns it red (\`.titlebar-button.close.hover:before\`).
        // Three 44 px buttons whose icon centres the capture puts at 1327.5, 1371.5 and
        // 1415.5: the last one ends two pixels inside the window edge. The stylesheet says
        // `width: 3rem` and a 3.75rem close button; the running client disagrees, and the
        // render is the spec.
        let widths = [44.0, 44.0, 44.0];
        let mut x = 1306.0;
        for (index, glyph) in [Glyph::Minus, Glyph::Square, Glyph::X]
            .into_iter()
            .enumerate()
        {
            let width = widths[index];
            let button = Rect::from_min_size(egui::pos2(x, 0.0), egui::vec2(width, g::TITLE_BAR));
            x += width;
            let response = ui.interact(
                button,
                egui::Id::new(("window-control", index)),
                Sense::click(),
            );
            let closing = index == 2;
            if response.hovered() {
                let plate = if closing { self.theme.RED } else { self.theme.BUTTON_BG };
                painter.rect_filled(button, 0, plate);
            }
            let ink = if response.hovered() {
                self.theme.ACCENT_CONTRAST
            } else {
                self.theme.SECONDARY
            };
            icons::draw(
                painter,
                glyph,
                Rect::from_center_size(button.center(), Vec2::splat(20.0)),
                ink,
            );
        }
    }

    /// The rail's buttons, at the reference's own pitch.
    ///
    /// The rail is one flex column, not two groups: \`App.vue\` stacks Home, Discover, Skins,
    /// Screenshots, Hosting, the create button, Settings and the account button with
    /// \`gap-[0.25rem]\` inside \`p-[0.5rem] pt-0\`. The account button is pinned to the
    /// bottom (\`mt-auto\` in the reference's markup), which the capture agrees with: icon
    /// centres land at 71, 123, 175, 227, 279, 331 from the top, and the last two at 815
    /// and 867 -- nine pixels above the window's bottom edge for a 48 px button and 8 px
    /// of padding.
    ///
    /// The active plate is \`--color-button-bg-selected\`, which in dark mode is
    /// \`rgba(27, 217, 106, 0.25)\`; over \`--color-raised-bg\` that resolves to #24553d,
    /// which is the colour the capture measures behind the first icon.
    ///
    /// Returns whether a hover is still moving, so the caller can keep the frame loop
    /// alive only while something is in flight.
    fn rail_column(&mut self, ui: &mut Ui) -> bool {
        let rail = Rect::from_min_size(
            egui::pos2(0.0, g::TITLE_BAR),
            egui::vec2(g::RAIL, g::WINDOW.1 - g::TITLE_BAR),
        );
        let painter = ui.painter();
        painter.rect_filled(rail, 0, self.theme.RAISED_BG);

        let mut animating = false;
        let mut hovered = None;
        for (index, destination) in Rail::ALL.into_iter().enumerate() {
            let button = Rect::from_min_size(
                egui::pos2(
                    g::RAIL_PAD,
                    g::TITLE_BAR + g::RAIL_FIRST + index as f32 * g::RAIL_PITCH,
                ),
                Vec2::splat(g::RAIL_BUTTON),
            );
            let response = ui.interact(button, egui::Id::new(("rail", index)), Sense::click());
            if response.hovered() {
                hovered = Some(destination);
            }
            if response.clicked() {
                self.rail = destination;
                // The rail is the way out of a project or an instance page, so a rail press
                // closes both.
                self.open_project = None;
                self.open_instance = None;
                // Coming home re-reads the folder: an instance may have been created,
                // imported or deleted while the reader was elsewhere.
                if destination == Rail::Home {
                    self.instances = crate::data::load().0;
                }
            }

            let selected = destination == self.rail;
            // \`rounded-full\` on a 48 px button is a 24 px radius: a circle, not a plate.
            if selected {
                painter.circle_filled(button.center(), g::RAIL_BUTTON / 2.0, self.theme.BUTTON_BG_SELECTED);
            } else if response.hovered() {
                painter.circle_filled(button.center(), g::RAIL_BUTTON / 2.0, self.theme.BUTTON_BG);
            }
            // The active icon is `--color-button-text-selected`, which in dark mode is
            // `--color-brand` itself: the capture's play triangle is #1bd96a on the
            // #24553d plate. A resting icon is `--color-base` (#b0bac5), not the dimmer
            // `--color-secondary` -- that is the difference between the rail reading as a
            // set of destinations and reading as disabled ones.
            let ink = if selected {
                self.theme.BUTTON_TEXT_SELECTED
            } else if response.hovered() {
                self.theme.CONTRAST
            } else {
                self.theme.BASE
            };
            icons::draw(
                painter,
                destination.glyph(),
                Rect::from_center_size(button.center(), Vec2::splat(24.0)),
                ink,
            );
        }

        // The create button continues the same column: it is not a page, it makes one, and
        // in the reference it is a NavButton with a \`+\` after Hosting.
        // The switcher's own rule: `h-px w-8 bg-surface-5` immediately above the create
        // button. The capture puts a 32 px line of #42444a at y 320, centred in the rail.
        painter.hline(
            (g::RAIL / 2.0 - 16.0)..=(g::RAIL / 2.0 + 16.0),
            320.0,
            Stroke::new(1.0, self.theme.SURFACE_5),
        );

        // The create button is not the next slot in the column: it lives inside the
        // reference's `QuickInstanceSwitcher`, under a 32 px divider row
        // (`h-px w-8` inside `gap-1` and `py-2`), which the capture puts at y 348. Its
        // icon centre measures 356, so the button starts 25 px below the last destination's
        // slot -- `RAIL_CREATE_DROP` is that distance and nothing else.
        const RAIL_CREATE_DROP: f32 = 25.0;
        let create = Rect::from_min_size(
            egui::pos2(
                g::RAIL_PAD,
                g::TITLE_BAR
                    + g::RAIL_FIRST
                    + Rail::ALL.len() as f32 * g::RAIL_PITCH
                    + RAIL_CREATE_DROP,
            ),
            Vec2::splat(g::RAIL_BUTTON),
        );
        let create_response = ui.interact(create, egui::Id::new("rail-create"), Sense::click());
        if create_response.hovered() {
            painter.circle_filled(create.center(), g::RAIL_BUTTON / 2.0, self.theme.BUTTON_BG);
        }
        icons::draw(
            painter,
            Glyph::Plus,
            Rect::from_center_size(create.center(), Vec2::splat(24.0)),
            if create_response.hovered() {
                self.theme.CONTRAST
            } else {
                self.theme.SECONDARY
            },
        );
        if create_response.clicked() {
            // The reference opens its create dialog here rather than making an instance on the
            // press; the four instance types are the dialog's, and the quick-create default it
            // replaced was the only path this launcher had.
            self.create_open = true;
            self.create_choice = None;
            self.create_opened = ui.input(|input| input.time);
            // The window only redraws while something is in flight, so the frame that opens
            // the dialog has to ask for the one that draws it.
            ui.ctx().request_repaint();
        }

        // The bottom pair: Settings and the account, \`mt-auto\`, eight pixels off the floor.
        for (index, glyph) in [Glyph::Settings, Glyph::LogIn].into_iter().enumerate() {
            let button = Rect::from_min_size(
                egui::pos2(
                    g::RAIL_PAD,
                    rail.bottom() - g::RAIL_PAD - g::RAIL_BUTTON - (1 - index) as f32 * g::RAIL_PITCH,
                ),
                Vec2::splat(g::RAIL_BUTTON),
            );
            let response = ui.interact(button, egui::Id::new(("utility", index)), Sense::click());
            if response.hovered() {
                painter.circle_filled(button.center(), g::RAIL_BUTTON / 2.0, self.theme.BUTTON_BG);
            }
            icons::draw(
                painter,
                glyph,
                Rect::from_center_size(button.center(), Vec2::splat(24.0)),
                if response.hovered() {
                    self.theme.CONTRAST
                } else {
                    self.theme.BASE
                },
            );
            if response.clicked() {
                match index {
                    0 => self.settings.open = true,
                    _ => self.about_open = true,
                }
            }
        }

        // A hover that changed keeps the loop alive for the transition; once the hover
        // settles there is nothing left to draw.
        if hovered != self.hovered {
            self.hovered = hovered;
            animating = true;
        }
        if self.hovered.is_none() {
            crate::motion::held_for(ui.ctx(), egui::Id::new("rail-tooltip"), false);
        }
        animating
    }

    /// The whole window, which is what a modal covers.
    fn window_rect() -> Rect {
        Rect::from_min_size(
            egui::pos2(0.0, 0.0),
            egui::vec2(g::WINDOW.0, g::WINDOW.1),
        )
    }

    /// The toolbar's `first - last` range, from the search's own answering.
    ///
    /// `None` when nothing has been searched: a range with no search behind it is a number
    /// invented by the launcher, which is what the hardcoded `1 - 946` was.
    fn result_range(&self) -> Option<String> {
        match self.results.state() {
            crate::search::State::Ready { hits, total, .. } => {
                if hits.is_empty() {
                    // A search that matched nothing says so; there is no range to print.
                    return Some("No results".to_owned());
                }
                let first = self.page as u64 * self.page_size as u64 + 1;
                let last = first + hits.len() as u64 - 1;
                Some(format!("{first} - {last} of {total}"))
            }
            _ => None,
        }
    }

    /// How many pages the last answer needs, counted from one.
    fn result_pages(&self) -> u64 {
        match self.results.state() {
            crate::search::State::Ready { total, .. } => {
                let size = self.page_size.max(1) as u64;
                ((total + size - 1) / size).max(1)
            }
            _ => 1,
        }
    }

    /// Do what a page's press asked for.
    ///
    /// Every page's verbs land in this one list: a launcher's actions are few and shared (make
    /// one, open one, run one), and a page that grew its own copy would be where the rules
    /// diverged.
    fn act(&mut self, action: crate::pages::home::Action) {
        use crate::pages::home::Action;
        match action {
            Action::None => {}
            Action::Create => self.create_instance(),
            Action::Import => {
                // The button is honest about not doing it yet rather than silently doing
                // nothing, which reads as a broken press.
                self.toast = Some("Importing from another launcher is not wired yet.".to_owned());
            }
            Action::Play(id) => {
                // One game at a time (`game.rs:113-124`): a press while a launch is out is
                // otherwise a no-op, and a press that silently does nothing reads as broken
                // -- so the second press says why instead of being swallowed. The menu's
                // Play and the row's pill both land here, which is what keeps them one verb.
                if self.game.busy() {
                    self.toast = Some(
                        "A launch is already running: this launcher starts one game at a time."
                            .to_owned(),
                    );
                } else {
                    self.game.start(&id);
                }
            }
            Action::Edit(id) => {
                // The reference opens the instance's own page (`pages/instance/layout.vue:182`).
                self.open_instance_page(&id);
            }
            Action::Duplicate(id) => {
                self.toast = Some(format!(
                    "Duplicating {} is not wired yet: nothing copies an instance's folder.",
                    self.instance_name(&id)
                ));
            }
            Action::OpenFolder(id) => self.open_instance_folder(&id),
            Action::Delete(id) => self.delete_instance(&id),
            Action::OpenWorld { instance, world } => {
                // One world's launch: the engine turns the world into
                // `--quickPlaySingleplayer`, which the game has understood since 1.7.
                self.game.start_world(&instance, &world);
            }
        }
    }

    /// Install one card's pack, or say why the press did nothing.
    ///
    /// **One install at a time**, like the one game: a second worker would import a second
    /// pack into the same instances folder while the first is still filling it, and the
    /// engine's own transfer ceiling is one queue. The refusal is said rather than swallowed,
    /// because a press that silently does nothing reads as a broken button.
    fn press_install(&mut self, card: &Card) {
        if self.install.busy() {
            self.toast = Some(
                "An install is already running: this launcher installs one pack at a time."
                    .to_owned(),
            );
        } else {
            // The title is the instance's name: `install_pack_archive`'s own rule is that a
            // reader wants the project's name rather than the archive's file stem.
            self.install.start(&card.id, &card.title);
        }
    }

    /// The name a reader knows an instance by, or its id when the list no longer holds it.
    ///
    /// A toast that said `Instance 4` when the row read `Fabric 1.21` would be about a
    /// different thing than the press, so every message the row menu produces goes through
    /// here.
    /// Open an instance's page, from the row the shell already has.
    ///
    /// The page takes plain values rather than a `data::Entry` or a live `Instance`, so this is
    /// where the two are joined: the entry supplies the name, loader and version it read from
    /// the pack profile, and the path is the folder the entry's id names.
    fn open_instance_page(&mut self, id: &str) {
        let Some(entry) = self.instances.iter().find(|entry| entry.id == id) else {
            self.toast = Some(format!("No instance called {id} is on disk any more."));
            return;
        };
        self.open_instance = Some(crate::pages::instance::Instance {
            id: entry.id.clone(),
            name: entry.name.clone(),
            loader: entry.loader.clone(),
            version: entry.version.clone(),
            path: crate::data::instances_dir().join(&entry.id),
        });
        self.open_project = None;
    }

    fn instance_name(&self, id: &str) -> String {
        self.instances
            .iter()
            .find(|entry| entry.id == id)
            .map(|entry| entry.name.clone())
            .unwrap_or_else(|| id.to_owned())
    }

    /// The folder one instance lives in.
    ///
    /// `Entry::id` is the folder's own name (`data.rs:21-23`), and `data::instances_dir` is
    /// the root the list was read from, so the path is the two joined -- the same tree the
    /// launch and the create already open.
    fn instance_dir(id: &str) -> std::path::PathBuf {
        crate::data::instances_dir().join(id)
    }

    /// Show an instance's folder in the platform's file manager.
    ///
    /// **`explorer`, and Windows-only for now.** `explorer` is Windows' own file manager and
    /// takes the directory as a plain argument, which is what the reference's
    /// `showInstanceInFolder` ends up doing through Tauri's `openPath`. The launcher ships a
    /// Windows exe and nothing here builds for another target, so the unix branch is the
    /// fallback that keeps the call honest rather than a path that has been run:
    /// `xdg-open` takes the same single argument, and macOS's `open` is not handled because
    /// no build targets it. A spawn (not a wait) is deliberate: the file manager outlives
    /// this process, and blocking a frame on it would freeze the window until it closed.
    fn open_instance_folder(&mut self, id: &str) {
        let path = Self::instance_dir(id);
        let opener = if cfg!(windows) { "explorer" } else { "xdg-open" };
        match std::process::Command::new(opener).arg(&path).spawn() {
            Ok(_) => self.toast = Some(format!("Opened {}.", path.display())),
            Err(error) => {
                self.toast = Some(format!(
                    "Could not open {} with {opener}: {error}",
                    path.display()
                ));
            }
        }
    }

    /// Remove an instance's folder from disk, unless a launch is alive.
    ///
    /// **The refusal is the shell's, not the row's.** `Game` holds one launch and keeps a
    /// status, not the id it was asked for (`game.rs:83-92`), so any live launch refuses
    /// every delete. Deleting the instance a game is running would take its `saves/` out
    /// from under a process still writing to them, which is the one mistake this launcher
    /// must not make; refusing a delete on an unrelated instance is the safe side of that
    /// line, and the toast says which rule refused it.
    fn delete_instance(&mut self, id: &str) {
        if self.game.busy() {
            self.toast = Some(format!(
                "Not deleting {} while a launch is running.",
                self.instance_name(id)
            ));
            return;
        }
        let path = Self::instance_dir(id);
        match std::fs::remove_dir_all(&path) {
            Ok(()) => {
                self.toast = Some(format!("Deleted {}.", self.instance_name(id)));
                // The list is what the page draws, so it is re-read here rather than on a
                // later frame: a deleted row left on screen would be a ghost the reader
                // could press again.
                self.instances = crate::data::load().0;
            }
            Err(error) => {
                self.toast = Some(format!(
                    "Could not delete {}: {error}",
                    path.display()
                ));
            }
        }
    }

    /// Create an instance with a name nobody has used, then re-read the list.
    ///
    /// The name and the version are the launcher's choices until the reference's create modal
    /// exists: a default that works beats a button that does nothing, and the instance it makes
    /// is a real one in the real folder.
    fn create_instance(&mut self) {
        let root = crate::data::instances_dir();
        let mut index = self.instances.len() + 1;
        while index < 64 {
            let name = format!("Instance {index}");
            match sift_core::instance::Instance::create(&root, &name, DEFAULT_MC_VERSION) {
                Ok(instance) => {
                    let made = instance
                        .root()
                        .file_name()
                        .map(|name| name.to_string_lossy().into_owned())
                        .unwrap_or(name);
                    self.toast = Some(format!("Created {made}."));
                    self.instances = crate::data::load().0;
                    return;
                }
                // A collision is `unique_dir_name`'s job, but trying the next number costs
                // nothing and the alternative is a message the reader cannot act on.
                Err(_) => index += 1,
            }
        }
        self.toast = Some("Could not create an instance: no free name.".to_owned());
    }

    /// The right panel: the page's own sections first, then the shared getting-started,
    /// news and hosting block.
    ///
    /// The reference teleports page sections into this panel (`<Teleport
    /// to="#sidebar-teleport-target">`), which is why the page draws first and the shared
    /// stack is then painted into whatever rect is left: a panel with one implementation and a
    /// hole at the top for the page, rather than a second copy of the shared sections per page.
    fn right_panel(&mut self, ui: &mut Ui) -> bool {
        let panel = Rect::from_min_size(
            egui::pos2(g::PANEL_LEFT, g::TITLE_BAR),
            egui::vec2(g::PANEL, g::WINDOW.1 - g::TITLE_BAR),
        );

        let mut shared = panel;
        if let Some(project) = self.open_project.clone() {
            let block = Rect::from_min_size(
                egui::pos2(panel.left() + 24.0, panel.top() + 20.0),
                egui::vec2(g::PANEL - 48.0, 400.0),
            );
            let used = crate::pages::project::panel(ui, block, &self.theme, &project);
            shared = Rect::from_min_max(
                egui::pos2(panel.left(), panel.top() + 20.0 + used + 16.0),
                panel.max,
            );
        }

        let signed_in = self
            .account
            .session()
            .map(|(name, _)| name.to_owned());
        let step = crate::panel::draw(
            ui,
            shared,
            &self.theme,
            signed_in.as_deref(),
            // Discover's own sidebar: the list is loaded once and kept, so it must be handed to
            // the panel only on the page that owns it.
            if self.rail == Rail::Discover {
                &self.categories
            } else {
                &[]
            },
            self.facet.as_deref(),
            self.categories_expanded,
            self.hide_installed,
        );
        let pressed = step.is_some();
        if let Some(step) = step {
            self.panel_step(step);
        }
        pressed
    }

    /// Do what a checklist row asked for.
    fn panel_step(&mut self, step: crate::panel::Step) {
        match step {
            // A category press is a filter, not a step: it replaces whatever was chosen, or
            // clears it when the same one is pressed again.
            crate::panel::Step::HideInstalled => {
                self.hide_installed = !self.hide_installed;
            }
            crate::panel::Step::MoreCategories => {
                self.categories_expanded = !self.categories_expanded;
            }
            crate::panel::Step::Category(facet) => {
                self.facet = if self.facet.as_deref() == Some(facet.as_str()) {
                    None
                } else {
                    Some(facet)
                };
                self.page = 0;
                self.results.forget();
            }
            crate::panel::Step::CreateInstance => self.create_instance(),
            crate::panel::Step::Minecraft => self.start_sign_in(),
            // Modrinth's own account is a separate service with its own flow; nothing here
            // can start it, so the row says so rather than doing nothing.
            crate::panel::Step::Modrinth => {
                self.toast = Some(
                    "Signing in to Sift needs its own OAuth flow, which this launcher has no                      client for yet."
                        .to_owned(),
                );
            }
        }
    }

    /// Ask Microsoft for a code and put it where the reader can see it.
    fn start_sign_in(&mut self) {
        match self.account.state() {
            crate::account::State::SignedIn { name, .. } => {
                self.toast = Some(format!("Already signed in as {name}."));
            }
            crate::account::State::Failed(reason) => {
                // A fresh press after a failure should try again, not repeat the failure.
                self.toast = Some(format!("Trying again after: {reason}"));
                self.account = crate::account::Account::default();
                self.account.start();
            }
            _ if self.account.busy() => {
                self.toast = Some("A sign-in is already in progress.".to_owned());
            }
            _ => {
                self.account.start();
                // Said now, not when the worker's first word arrives: a request to Microsoft
                // can take half a minute, and silence reads as a button that does nothing.
                self.toast = Some(
                    "Asking Microsoft for a sign-in code…".to_owned(),
                );
            }
        }
    }

    /// The tabs, the search row and the scrolling list. Returns whether a glide is running.
    fn content_column(&mut self, ui: &mut Ui) -> bool {
        // The page column, not the surface: the reference's Discover content starts at
        // x 88, y 72 -- the surface's edge plus its own p-6 -- and is 1028 wide, which the
        // capture measures as 88..1115 with the scrollbar overlaying the padding. Drawing at
        // the surface's edge put every control 24 points left of the reference and gave the
        // strip 622 points where it has 636.
        let content = Rect::from_min_size(
            egui::pos2(g::TEXT_LEFT, g::TITLE_BAR + g::CONTENT_INSET),
            egui::vec2(
                g::PANEL_LEFT - g::CONTENT_INSET - g::TEXT_LEFT,
                g::WINDOW.1 - g::TITLE_BAR - 2.0 * g::CONTENT_INSET,
            ),
        );
        ui.painter().rect_filled(
            Rect::from_min_max(
                egui::pos2(g::CONTENT_LEFT, g::TITLE_BAR),
                egui::pos2(g::PANEL_LEFT, g::WINDOW.1),
            ),
            0,
            self.theme.BG,
        );

        let painter = ui.painter();
        // The tab strip, measured on the running client (`.scratch/cap/ref-discover.png`):
        // a **w-fit** pill bar at x 88..725, y 74..122 -- 48 tall, radius 24, one
        // `--color-raised-bg` plate -- with six 36-tall pills six points in and four apart.
        // It is not a full-width strip: the reference's bar is only as wide as its pills,
        // which is why a full-width one reads as a toolbar instead of a tab bar.
        const KINDS_PRE: [&str; 6] = [
            "Modpacks",
            "Mods",
            "Resource Packs",
            "Data Packs",
            "Shaders",
            "Servers",
        ];
        // The plate is as wide as its pills: measure them first, then draw it, then draw
        // them. A full-width plate was the old shape and it read as a toolbar.
        let mut strip_width = 12.0;
        for label in KINDS_PRE {
            let galley = painter.layout_no_wrap(
                label.to_owned(),
                crate::fonts::semibold(14.0),
                self.theme.BASE,
            );
            strip_width += galley.size().x + 32.0 + 4.0;
        }
        let strip = Rect::from_min_size(
            egui::pos2(content.left(), content.top()),
            egui::vec2(strip_width - 4.0, 46.0),
        );
        painter.rect_filled(strip, 23, self.theme.RAISED_BG);
        painter.rect_stroke(
            strip,
            23,
            Stroke::new(1.0, self.theme.SURFACE_4),
            egui::StrokeKind::Inside,
        );
        let mut x = strip.left() + 6.0;
        // The reference's six kinds. `Servers` has no search facet of its own -- it is a
        // destination, not a project type -- so it keeps the plugins facet the API does have
        // and says so here rather than quietly searching nothing.
        const KINDS: [(&str, &str); 6] = [
            ("Modpacks", "modpack"),
            ("Mods", "mod"),
            ("Resource Packs", "resourcepack"),
            ("Data Packs", "datapack"),
            ("Shaders", "shader"),
            ("Servers", "plugin"),
        ];
        for (index, (label, kind)) in KINDS.into_iter().enumerate() {
            let galley = painter.layout_no_wrap(
                label.to_owned(),
                crate::fonts::semibold(14.0),
                // The active pill's lettering is `--color-accent-contrast`: laying it out in
                // the brand colour, as this did, is green text on a green fill and reads as an
                // empty pill.
                if kind == self.project_type {
                    self.theme.BUTTON_TEXT_SELECTED
                } else {
                    self.theme.CONTRAST
                },
            );
            let pill = Rect::from_min_size(
                egui::pos2(x, strip.top() + 5.0),
                egui::vec2(galley.size().x + 32.0, 36.0),
            );
            let selected = kind == self.project_type;
            // The active pill is the brand itself with `--color-accent-contrast` lettering
            // (`NavTabs.vue`), not a translucent plate: the capture's active pill is #1bd96a
            // with near-black text.
            // The plate is `--color-button-bg-selected` -- #24553d over the raised strip --
            // and the lettering is `--color-button-text-selected` (#1bd96a). The pill region
            // of the capture is 2189 pixels of #24553d and its glyphs are #1bd96a, so the
            // bright-green-fill reading was wrong.
            if selected {
                painter.rect_filled(pill, 18, self.theme.BUTTON_BG_SELECTED);
            }
            // The pill is a target: pressing it switches the kind the search asks for, which is
            // the whole reason the strip exists rather than being a label.
            if ui
                .interact(pill, egui::Id::new(("kind-tab", index)), Sense::click())
                .clicked()
                && !selected
            {
                self.project_type = kind;
                // The list belongs to the kind: a new one asks again.
                self.categories_asked = false;
                self.categories.clear();
                // A new kind is a new search; dropping the answered query makes the request
                // below fire even when the text has not changed.
                self.results.forget();
            }
            painter.galley(
                egui::pos2(pill.center().x - galley.size().x / 2.0, pill.center().y - galley.size().y / 2.0),
                galley.clone(),
                if selected {
                    self.theme.ACCENT_CONTRAST
                } else {
                    self.theme.BASE
                },
            );
            x = pill.right();
        }

        // The search field is a row of its own and the full width of the column: the capture
        // puts it at x 88..1113, y 126..174 -- 48 tall (`size="large"`), radius 14,
        // `--surface-4` inside a one-pixel `--surface-5` ring. The sort and view selects and
        // the pager sit on the *next* row, at y 182 and 36 tall.
        let field = Rect::from_min_size(
            egui::pos2(content.left(), content.top() + 54.0),
            egui::vec2(content.width(), 48.0),
        );
        let row = Rect::from_min_size(
            egui::pos2(content.left(), content.top() + 110.0),
            egui::vec2(content.width(), 36.0),
        );
        // The field is a widget with state now: focus, a caret that blinks and a clear button.
        // Its return value is frame debt -- true while it is focused, because the caret is
        // the one thing in this window that moves without being asked to.
        let mut typing = self.search.ui(ui, field, &self.theme);
        // The field's text drives the network search: a changed query starts one, and a
        // reply already in flight is collected here. Both are cheap when nothing changed.
        // A press on the search field closes any open popover, the way a click outside one
        // does. The field's own focus handling is the widget's business.
        if ui.input(|i| i.pointer.any_pressed())
            && self.open_select.is_some()
            && !ui.rect_contains_pointer(field)
        {
            // Keep it open only while the press is inside the select or its popover; the
            // coarse test is the field, and the selects close themselves on selection.
            let _ = &self.open_select;
        }
        // The category list, asked for once for the kind in hand: it is a fixed list per project
        // type, so a second question would be the same answer bought twice.
        if !self.categories_asked {
            self.categories_asked = true;
            let (sender, receiver) = std::sync::mpsc::channel();
            self.categories_rx = Some(receiver);
            let kind = self.project_type.to_owned();
            let _ = std::thread::Builder::new()
                .name("sift-categories".to_owned())
                .spawn(move || {
                    let _ = sender.send(load_categories(&kind));
                });
        }
        if let Some(receiver) = &self.categories_rx {
            if let Ok(list) = receiver.try_recv() {
                self.categories = list;
                self.categories_rx = None;
                // The panel drew before this answer arrived, and an idle window schedules no
                // frame of its own -- without this the list waits for a stray press.
                ui.ctx().request_repaint();
            }
        }
        // A facet without a query is a question too -- it is what a category press asks -- so this
        // is not nested inside the field's `Option`.
        let asked_query = self.search.query().map(str::to_owned).unwrap_or_default();
        // The empty box is the reference's default feed -- `Browse.vue:1201` requests it when
        // the page mounts -- so asking is not gated on text. A bare block rather than a
        // condition, because "always" is what this is.
        {
            let query = asked_query;
            // A different question starts at the first page: the offset is part of the request
            // now, so paging and then typing would ask for page three of a search with one page.
            let question = format!(
                "{query}|{}|{}|{}|{:?}",
                self.project_type, self.sort_index, self.page_size, self.facet
            );
            if self.asked != question {
                self.asked = question;
                self.page = 0;
            }
            let facets: Vec<String> = self.facet.iter().map(|name| name.clone()).collect();
            self.results.request(
                &query,
                self.project_type,
                self.sort_index,
                self.page_size,
                self.page.saturating_mul(self.page_size),
                &facets,
            );
        }
        self.results.poll();
        // `search.ui` took `ui` mutably, which ended the painter borrow taken above; the
        // painter is re-acquired here, and the compiler is what said so.
        let painter = ui.painter();
        // The two selects: `Sort by: Relevance` and `View: 20`.
        //
        // A press cycles the value rather than opening a dropdown. That is deliberate for this
        // slice: the values are few and the whole point is that the *search* answers the new
        // order, which a painted menu would not prove. The dropdown is the next slice, and the
        // select's own label already reads from the value it will set.
        const SORTS: [(&str, &str); 5] = [
            ("Relevance", "relevance"),
            ("Downloads", "downloads"),
            ("Follows", "follows"),
            ("Newest", "newest"),
            ("Updated", "updated"),
        ];
        const SIZES: [u32; 4] = [10, 20, 50, 100];
        let select_w = 150.0;
        let mut select_rects = [Rect::NOTHING; 2];
        for index in 0..2 {
            let select = Rect::from_min_size(
                egui::pos2(
                    content.left() + if index == 0 { 0.0 } else { 256.0 + 8.0 },
                    row.top(),
                ),
                egui::vec2(if index == 0 { 256.0 } else { 144.0 }, 36.0),
            );
            // `Sort by: ` in `--color-base` then the value in `--color-contrast` semibold 16:
            // two spans, not one 13-point string, which is what the capture shows.
            let (prefix, value) = if index == 0 {
                let name = SORTS
                    .iter()
                    .find(|(_, value)| *value == self.sort_index)
                    .map(|(name, _)| *name)
                    .unwrap_or("Relevance");
                ("Sort by: ", name.to_owned())
            } else {
                ("View: ", self.page_size.to_string())
            };
            painter.rect_filled(select, 12, self.theme.SURFACE_4);
            painter.rect_stroke(
                select,
                12,
                Stroke::new(1.0, self.theme.SURFACE_5),
                egui::StrokeKind::Inside,
            );
            let prefix_galley =
                painter.layout_no_wrap(prefix.to_owned(), crate::fonts::body(16.0), self.theme.BASE);
            let value_width = prefix_galley.size().x;
            painter.galley(
                egui::pos2(select.left() + 12.0, select.center().y - prefix_galley.size().y / 2.0),
                prefix_galley,
                self.theme.BASE,
            );
            painter.text(
                egui::pos2(select.left() + 12.0 + value_width, select.center().y),
                Align2::LEFT_CENTER,
                value,
                crate::fonts::semibold(16.0),
                self.theme.CONTRAST,
            );
            icons::draw(
                painter,
                Glyph::ChevronDown,
                Rect::from_center_size(
                    egui::pos2(select.right() - 16.0, select.center().y),
                    Vec2::splat(12.0),
                ),
                self.theme.SECONDARY,
            );
            let response =
                ui.interact(select, egui::Id::new(("search-select", index)), Sense::click());
            if response.clicked() {
                // Toggle: pressing the open select's own button closes it, which is what a
                // popover does everywhere else in the reference.
                self.open_select = if self.open_select == Some(index) {
                    None
                } else {
                    Some(index)
                };
            }
            // Where this select sits, remembered so its popover can be drawn after the list:
            // egui paints in call order, and a popover drawn here would sit *under* the cards
            // that the scroll area paints later.
            select_rects[index] = select;
        }
        // The result count, right-aligned on the same row.
        //
        // It reads the search's own total now instead of the reference's `1 - 946`. When the
        // box is empty there is no search, and the reference's number is what a first capture
        // happened to show rather than a fact about anything -- so an empty box says nothing
        // rather than a stale count.
        // The page controls, where the reference puts its numbered strip. `// owed:` that strip
        // -- the numbers with an ellipsis and the last page -- is not drawn; what is here pages,
        // which is the part a reader needs. Laid out from the right edge so the parts cannot run
        // into each other whatever the numbers say.
        if let Some(range) = self.result_range() {
            let pages = self.result_pages();
            let middle = row.center().y;
            let right = row.right();
            // An empty search says so where the strip would be; there are no pages to turn.
            if range == "No results" {
                ui.painter().text(
                    egui::pos2(right, middle),
                    Align2::RIGHT_CENTER,
                    range,
                    crate::fonts::body(12.0),
                    self.theme.SECONDARY,
                );
            } else {
                let next = egui::Rect::from_center_size(
                    egui::pos2(right - 12.0, middle),
                    egui::Vec2::splat(22.0),
                );
                let mut x = next.left() - 6.0;
                for item in pager_items(self.page + 1, pages).into_iter().rev() {
                    let label = match item {
                        Some(number) => number.to_string(),
                        None => "…".to_owned(),
                    };
                    let ink = if item == Some(u64::from(self.page) + 1) {
                        self.theme.BUTTON_TEXT_SELECTED
                    } else {
                        self.theme.SECONDARY
                    };
                    let galley = ui.painter().layout_no_wrap(
                        label.clone(),
                        crate::fonts::body(16.0),
                        ink,
                    );
                    let width = if item.is_some() { PAGER_CELL } else { galley.size().x + 8.0 };
                    // A 36x36 cell at radius 18 with a 16-point numeral: the reference's
                    // current-page disc measures 924..959 by 182..217, and this used to be a
                    // 22-tall plate with 12-point text.
                    let cell = egui::Rect::from_center_size(
                        egui::pos2(x - width / 2.0, middle),
                        egui::vec2(width, PAGER_CELL),
                    );
                    if item == Some(u64::from(self.page) + 1) {
                        ui.painter().rect_filled(cell, 18, self.theme.BUTTON_BG_SELECTED);
                    }
                    ui.painter().galley(
                        egui::pos2(cell.center().x - galley.size().x / 2.0, middle - galley.size().y / 2.0),
                        galley,
                        ink,
                    );
                    if let Some(number) = item {
                        if number != u64::from(self.page) + 1
                            && ui
                                .interact(cell, egui::Id::new(("page", number)), egui::Sense::click())
                                .clicked()
                        {
                            self.page = (number - 1) as u32;
                            self.results.forget();
                            ui.ctx().request_repaint();
                        }
                    }
                    x = cell.left() - 2.0;
                }
                // The `-` sits just left of where the numbers stopped, whatever they were.
                let prev = egui::Rect::from_center_size(
                    egui::pos2(x - 10.0, middle),
                    egui::Vec2::splat(22.0),
                );
                for (rect, step, enabled) in [
                    (prev, -1_i64, self.page > 0),
                    (next, 1, (self.page as u64 + 1) < pages.max(1)),
                ] {
                    let ink = if enabled {
                        self.theme.BASE
                    } else {
                        self.theme.SECONDARY
                    };
                    ui.painter().text(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        if step < 0 { "<" } else { ">" },
                        crate::fonts::body(13.0),
                        ink,
                    );
                    if enabled
                        && ui
                            .interact(rect, egui::Id::new(("page-step", step)), egui::Sense::click())
                            .clicked()
                    {
                        self.page = (self.page as i64 + step).max(0) as u32;
                        self.results.forget();
                        ui.ctx().request_repaint();
                    }
                }
            }
        }

        // The list, via fastframe's scrolling: 120 points per notch rather than egui's 40.
        let list = Rect::from_min_max(
            egui::pos2(content.left(), row.bottom() + 12.0),
            content.max,
        );
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(list));
        let mut owed = typing;
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(&mut child, |ui| {
                // The reference's own six results first, then repeats so the list is long
                // enough to scroll: a capture compares anatomy, not the catalogue.
                // Real results when the search has them; the reference's own six cards while
                // the box is empty, so the page is never a blank rectangle.
                // The projects this launcher installed, for the switch below. An instance
                // installed before the id was recorded simply never matches.
                let installed: Vec<&str> = self
                    .instances
                    .iter()
                    .filter_map(|entry| entry.project.as_deref())
                    .collect();
                let live: Vec<Card> = match self.results.state() {
                    crate::search::State::Ready { hits, .. } => hits
                        .iter()
                        .filter(|hit| !self.hide_installed || !installed.contains(&hit.id.as_str()))
                        .map(|hit| Card {
                            // The project's id, which is what the Install button is keyed by
                            // and what the version request needs; a card that dropped it
                            // could only offer a button that had nothing to install.
                            id: hit.id.clone(),
                            title: hit.title.clone(),
                            author: hit.author.clone(),
                            summary: hit.summary.clone(),
                            // The categories then the loaders, which is the order the
                            // reference draws a search card's tags in.
                            tags: hit
                                .categories
                                .iter()
                                .map(|category| (category.clone(), category_glyph(category)))
                                .chain(
                                    hit.loaders
                                        .iter()
                                        .map(|loader| (loader.clone(), Glyph::Box)),
                                )
                                .collect(),
                            downloads: hit.downloads.clone(),
                            followers: hit.followers.clone(),
                            // The search API has no "updated" field, and the row is omitted
                            // rather than drawn empty.
                            updated: String::new(),
                            icon: hit.icon.clone(),
                        })
                        .collect(),
                    _ => Vec::new(),
                };
                let results: &[Card] = &live;
                // No results is a state, not a reason to invent six: the reference draws a
                // centred line while the feed is out and "No results found for your query!"
                // when it comes back empty. The fabricated cards that used to stand here were
                // the single most misleading thing in this page.
                if results.is_empty() {
                    let message = match self.results.state() {
                        crate::search::State::Loading => "Loading…",
                        crate::search::State::Failed { .. } => {
                            "The search could not be reached. Check the connection."
                        }
                        _ => "No results found for your query!",
                    };
                    let centre = egui::pos2(
                        ui.max_rect().center().x,
                        ui.max_rect().top() + 120.0,
                    );
                    ui.painter().text(
                        centre,
                        Align2::CENTER_CENTER,
                        message,
                        crate::fonts::body(16.0),
                        self.theme.SECONDARY,
                    );
                }
                let mut opened = false;
                // No modulo: an empty list is a state drawn above, not a list to repeat, and
                // `index % 0` is a panic rather than a card.
                for index in 0..results.len() {
                    let result = &results[index];
                    // The button's word comes from the install, not from the card: a card
                    // cannot know whether its own project is the one being installed.
                    let label = self.install.label(&result.id);
                    match card(ui, &self.theme, index, result, &mut self.icons, label) {
                        Press::None => {}
                        Press::Card => opened = true,
                        Press::Install => self.press_install(result),
                    }
                }
                if opened {
                    // Any card opens the sample project for now; the real one carries the
                    // project's own slug once search is wired.
                    self.open_project = Some(crate::pages::project::sample());
                }
            });
        // The open popover, last, so it covers the list rather than the other way round. Its
        // items are interacted here too: an item registered before the cards would lose the
        // click to whichever card overlaps it.
        if let Some(open) = self.open_select {
            let select = select_rects[open];
            if select != Rect::NOTHING {
                let row_h = 32.0;
                let labels: Vec<(String, bool)> = if open == 0 {
                    SORTS
                        .iter()
                        .map(|(name, value)| ((*name).to_owned(), *value == self.sort_index))
                        .collect()
                } else {
                    SIZES
                        .iter()
                        .map(|size| (size.to_string(), *size == self.page_size))
                        .collect()
                };
                let plate = Rect::from_min_size(
                    egui::pos2(select.left(), select.bottom() + 4.0),
                    egui::vec2(select.width(), row_h * labels.len() as f32 + 8.0),
                );
                ui.painter().rect_filled(plate, 8, self.theme.SUPER_RAISED_BG);
                for (row, (label, chosen)) in labels.iter().enumerate() {
                    let item = Rect::from_min_size(
                        egui::pos2(plate.left() + 4.0, plate.top() + 4.0 + row as f32 * row_h),
                        egui::vec2(plate.width() - 8.0, row_h),
                    );
                    if *chosen {
                        ui.painter().rect_filled(item, 6, self.theme.BUTTON_BG_SELECTED);
                    }
                    ui.painter().text(
                        egui::pos2(item.left() + 10.0, item.center().y),
                        Align2::LEFT_CENTER,
                        label,
                        crate::fonts::body(13.0),
                        if *chosen {
                            self.theme.BUTTON_TEXT_SELECTED
                        } else {
                            self.theme.BASE
                        },
                    );
                    if ui
                        .interact(item, egui::Id::new(("search-option", open, row)), Sense::click())
                        .clicked()
                    {
                        if open == 0 {
                            if let Some((_, value)) = SORTS.get(row) {
                                self.sort_index = value;
                            }
                        } else if let Some(size) = SIZES.get(row) {
                            self.page_size = *size;
                        }
                        self.open_select = None;
                        // A different order or page size is a different question: forget the
                        // answered query so the request below asks it again.
                        self.results.forget();
                    }
                }
            }
        }

        owed |= typing;
        typing = owed;
        typing
    }
}

/// One result card, drawn to the reference's anatomy.
///
/// Measured from a 2.2x crop of the reference (`card.jpg`), in window pixels from the
/// card's own top-left: a 64-point icon tile at 16,16; the title at 96,16 with the author
/// beside it and a link glyph after; a two-line summary at 96,44; a row of tag pills at
/// 96,104; the stats right-aligned on that same row; and the install button at the card's
/// top-right corner. The card itself is 148 tall with a 12-point gap, which is what the
/// reference's pitch between two card tops measures.
fn card(
    ui: &mut Ui,
    theme: &Theme,
    index: usize,
    result: &Card,
    icons: &mut crate::images::Images,
    install_label: &str,
) -> Press {
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, g::CARD), Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, 12, theme.RAISED_BG);

    // The icon tile. The reference draws the project's icon here; until images load, a
    // plate in the accent carries the same weight and the same 12-point radius.
    // The reference's card is 1028x142 at radius 16 with a one-pixel ring, a 100-point icon
    // sixteen points in from the corner, and a 20-point title starting 129 in.
    painter.rect_stroke(
        rect,
        16,
        Stroke::new(1.0, theme.SURFACE_4),
        egui::StrokeKind::Inside,
    );
    let tile = Rect::from_min_size(rect.min + egui::vec2(16.0, 16.0), Vec2::splat(100.0));
    painter.rect_filled(tile, 12, theme.BUTTON_BG);
    // The project's own icon once it has arrived; the plate until then, and for ever if the
    // fetch failed. Rounded corners are the tile's: a texture drawn into a square rect inside
    // a rounded plate keeps the plate's corner showing, which is what the reference does.
    if let Some(texture) = icons.get(ui.ctx(), &result.icon) {
        let inner = tile.shrink(2.0);
        painter.image(
            texture.id(),
            inner,
            Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    }

    // The title, the author and the link glyph, on one baseline.
    let title = painter.layout_no_wrap(
        result.title.clone(),
        crate::fonts::title(20.0),
        theme.CONTRAST,
    );
    let title_at = egui::pos2(rect.left() + 129.0, rect.top() + 16.0);
    painter.galley(title_at, title.clone(), theme.CONTRAST);
    let by = painter.layout_no_wrap(
        format!("by {}", result.author),
        crate::fonts::body(16.0),
        theme.SECONDARY,
    );
    let by_at = egui::pos2(title_at.x + title.size().x + 10.0, title_at.y + 3.0);
    painter.galley(by_at, by.clone(), theme.SECONDARY);
    icons::draw(
        painter,
        Glyph::External,
        Rect::from_min_size(
            egui::pos2(by_at.x + by.size().x + 8.0, by_at.y + 2.0),
            Vec2::splat(12.0),
        ),
        theme.SECONDARY,
    );

    // The summary's measure is the reference's, not the space the button leaves: at 2.2x
    // its first card breaks after "in a", which is a ~440-point line, so the second line
    // exists for the same reason it does there.
    // The summary starts on the title's own left edge (129 in from the card) and runs to the
    // stats column, in 16-point text: it used to start 96 in, which put its first line under
    // the 100-point icon, and be set at 13, which made it smaller than the author line.
    let summary_width = (rect.width() - 129.0 - 260.0).max(120.0);
    let summary_text = ellipsise(&result.summary, 150);
    let summary = painter.layout(
        summary_text,
        crate::fonts::body(16.0),
        theme.SECONDARY,
        summary_width,
    );
    painter.galley(egui::pos2(rect.left() + 129.0, rect.top() + 48.0), summary, theme.SECONDARY);

    // The tag pills, each with its own glyph, then the stats on the same row. The row stops
    // where the stats begin and reports what it could not show as a `+N` chip, which is what the
    // reference draws -- without it the pills ran under the download count.
    let mut x = rect.left() + 96.0;
    let tag_limit = rect.right() - TAG_ROW_RESERVE - (rect.left() + 96.0);
    let widths: Vec<f32> = result
        .tags
        .iter()
        .map(|tag| {
            painter
                .layout_no_wrap(tag.0.clone(), crate::fonts::body(11.0), theme.SECONDARY)
                .size()
                .x
                + 30.0
        })
        .collect();
    let fitted = tags_that_fit(&widths, 6.0, tag_limit);
    for (index, tag) in result.tags.iter().enumerate() {
        if index >= fitted {
            break;
        }
        let text = painter.layout_no_wrap(tag.0.clone(), crate::fonts::body(11.0), theme.SECONDARY);
        let pill_width = text.size().x + 30.0;
        let pill = Rect::from_min_size(
            egui::pos2(x, rect.top() + 104.0),
            egui::vec2(pill_width, 24.0),
        );
        painter.rect_filled(pill, 12, theme.BUTTON_BG);
        icons::draw(
            painter,
            tag.1,
            Rect::from_center_size(
                egui::pos2(pill.left() + 14.0, pill.center().y),
                Vec2::splat(11.0),
            ),
            theme.SECONDARY,
        );
        painter.galley(
            egui::pos2(pill.left() + 24.0, pill.center().y - text.size().y / 2.0),
            text.clone(),
            theme.SECONDARY,
        );
        x = pill.right() + 6.0;
    }
    if fitted < result.tags.len() {
        let rest = result.tags.len() - fitted;
        let chip = painter.layout_no_wrap(
            format!("+{rest}"),
            crate::fonts::body(11.0),
            theme.SECONDARY,
        );
        let plate = Rect::from_min_size(
            egui::pos2(x, rect.top() + 104.0),
            egui::vec2(chip.size().x + 20.0, 24.0),
        );
        painter.rect_filled(plate, 12, theme.BUTTON_BG);
        painter.galley(
            egui::pos2(plate.left() + 10.0, plate.center().y - chip.size().y / 2.0),
            chip,
            theme.SECONDARY,
        );
    }

    // The stats and the install button, right-aligned: downloads, followers, updated.
    // The reference's Install is an **outlined** brand button -- 94x36, radius 12, a
    // one-pixel `--color-brand` ring, brand lettering, no fill -- with a plus glyph before
    // the word. A filled plate with dark text reads as the pressed state of something else.
    let install = Rect::from_min_size(
        egui::pos2(rect.right() - 16.0 - 94.0, rect.top() + 16.0),
        egui::vec2(94.0, 36.0),
    );
    painter.rect_stroke(
        install,
        12,
        Stroke::new(1.0, theme.BRAND),
        egui::StrokeKind::Inside,
    );
    let install_text = painter.layout_no_wrap(
        install_label.to_owned(),
        crate::fonts::semibold(14.0),
        theme.BRAND,
    );
    let glyph_w = 16.0;
    let gap = 6.0;
    let content_w = glyph_w + gap + install_text.size().x;
    let start_x = install.center().x - content_w / 2.0;
    icons::draw(
        painter,
        Glyph::Plus,
        Rect::from_center_size(
            egui::pos2(start_x + glyph_w / 2.0, install.center().y),
            Vec2::splat(glyph_w),
        ),
        theme.BRAND,
    );
    painter.galley(
        egui::pos2(start_x + glyph_w + gap, install.center().y - install_text.size().y / 2.0),
        install_text,
        theme.BRAND,
    );
    // The stats are two right-aligned lines beside the summary: downloads then followers on
    // the first, the update clock on the second. The capture puts the pair at y 306..321 and
    // the clock at 336..353, both ending at x 1098, in 14-point text -- they used to be one
    // 11-point line on the tag row, which is where the tags belong and the stats do not.
    let mut counts_right = rect.right() - 16.0;
    let counts_y = rect.top() + 62.0;
    // Drawn right to left, so the followers are laid down first and the downloads end up to
    // their left -- which is the order the reference reads: "19.66M  heart 5,028".
    for (glyph, value) in [
        (Glyph::Heart, result.followers.as_str()),
        (Glyph::Download, result.downloads.as_str()),
    ] {
        if value.is_empty() {
            continue;
        }
        let label = painter.layout_no_wrap(
            value.to_owned(),
            crate::fonts::body(14.0),
            theme.SECONDARY,
        );
        painter.galley(
            egui::pos2(counts_right - label.size().x, counts_y - label.size().y / 2.0),
            label.clone(),
            theme.SECONDARY,
        );
        icons::draw(
            painter,
            glyph,
            Rect::from_center_size(
                egui::pos2(counts_right - label.size().x - 12.0, counts_y),
                Vec2::splat(14.0),
            ),
            theme.SECONDARY,
        );
        counts_right -= label.size().x + 12.0 + 14.0 + 16.0;
    }
    if !result.updated.is_empty() {
        let label = painter.layout_no_wrap(
            result.updated.clone(),
            crate::fonts::body(14.0),
            theme.SECONDARY,
        );
        let y = rect.top() + 92.0;
        painter.galley(
            egui::pos2(rect.right() - 16.0 - label.size().x, y - label.size().y / 2.0),
            label.clone(),
            theme.SECONDARY,
        );
        icons::draw(
            painter,
            Glyph::Calendar,
            Rect::from_center_size(
                egui::pos2(rect.right() - 16.0 - label.size().x - 12.0, y),
                Vec2::splat(14.0),
            ),
            theme.SECONDARY,
        );
    }

    ui.add_space(g::CARD_GAP);
    // The whole card is the hit target; the reference navigates on a press anywhere on it.
    // Keyed by row, not by title: the same project can legitimately appear twice in a list
    // (two sources, a repeat), and duplicate widget ids make egui paint a clash warning and
    // box the card in red.
    let card = ui.interact(rect, egui::Id::new(("result-card", index)), Sense::click());
    // The button is registered after the card, so it sits above it: egui hands a press to the
    // last target that claimed the point, and a press on Install must not also open the
    // project. Answered first for the same reason -- the card's own response is the one a
    // changed hit-test order would otherwise turn into a page navigation.
    let button = ui.interact(
        install,
        egui::Id::new(("result-install", index)),
        Sense::click(),
    );
    if button.clicked() {
        Press::Install
    } else if card.clicked() {
        Press::Card
    } else {
        Press::None
    }
}

/// What a press on a result card asked for.
///
/// One value rather than two flags: a frame delivers one press to one target, and the button
/// and the card are both hit targets on the same rect.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Press {
    /// Nothing was pressed.
    None,
    /// The card itself: open the project page.
    Card,
    /// The Install button.
    Install,
}

/// What a result card shows. The reference's list carries these fields and no others.
///
/// Owned rather than borrowed: half the cards come from a search reply, whose strings the
/// page does not own, and a card that borrowed them would tie the list to the reply's
/// lifetime for no gain.
pub struct Card {
    /// The project's stable id (`"1KVo5zza"`), which is what `version_url` takes and what the
    /// install button is keyed by.
    pub id: String,
    /// The project's name.
    pub title: String,
    /// The team that published it.
    pub author: String,
    /// One or two sentences of description.
    pub summary: String,
    /// The categories and loader, as (label, glyph).
    pub tags: Vec<(String, Glyph)>,
    /// The download count, already abbreviated the way the reference shows it.
    pub downloads: String,
    /// The follower count, abbreviated the same way.
    pub followers: String,
    /// When it last changed, in the reference's phrasing; empty when the API has no such
    /// field, which is the case for a search hit.
    pub updated: String,
    /// The project icon's URL.
    pub icon: String,
}

/// How many of a card's tags fit before `limit`, and how many are left over.
///
/// A tag that does not fit is not shown and everything from it on is counted in the chip, which is
/// what the reference's `+N` reports.
fn tags_that_fit(widths: &[f32], gaps: f32, limit: f32) -> usize {
    let mut x = 0.0;
    for (index, width) in widths.iter().enumerate() {
        if x + width > limit {
            return index;
        }
        x += width + gaps;
    }
    widths.len()
}

/// Fetch the category list for one project type and return its names, in the API's order.
///
/// Runs on a worker thread: the answer is a fixed list, and a failed fetch simply means the panel
/// shows no categories rather than an error a reader could do anything about.
fn load_categories(project_type: &str) -> Vec<String> {
    let url = sift_net::modrinth::tag_categories_url();
    let path = std::env::temp_dir().join(format!("sift-categories-{project_type}.json"));
    let fetched = sift_net::download_file(&url, &path, std::time::Duration::from_secs(15));
    let Ok(_) = fetched else {
        return Vec::new();
    };
    let Ok(body) = std::fs::read_to_string(&path) else {
        return Vec::new();
    };
    let _ = std::fs::remove_file(&path);
    let parsed = serde_json::from_str::<Vec<sift_net::modrinth::CategoryTag>>(&body);
    let Ok(tags) = parsed else {
        return Vec::new();
    };
    tags.into_iter()
        .filter(|tag| tag.project_type == project_type)
        .map(|tag| tag.name)
        .collect()
}

/// The page numbers a strip shows: the first, the last, the page in hand and one either side of
/// it, with `None` where an ellipsis belongs. One-based, and never a number outside `1..=pages`.
fn pager_items(current: u32, pages: u64) -> Vec<Option<u64>> {
    let pages = pages.max(1);
    let current = u64::from(current).clamp(1, pages);
    let mut wanted: Vec<u64> = Vec::new();
    for candidate in [1, current.saturating_sub(1), current, current + 1, pages] {
        if candidate >= 1 && candidate <= pages && !wanted.contains(&candidate) {
            wanted.push(candidate);
        }
    }
    wanted.sort_unstable();
    let mut items = Vec::new();
    let mut previous: Option<u64> = None;
    for number in wanted {
        if let Some(last) = previous {
            // A gap of exactly one is a number worth showing; anything wider is an ellipsis.
            if number - last == 2 {
                items.push(Some(last + 1));
            } else if number - last > 1 {
                items.push(None);
            }
        }
        items.push(Some(number));
        previous = Some(number);
    }
    items
}

/// A page number's cell in the strip, and how much room each candidate needs.
const PAGER_CELL: f32 = 36.0; // 36x36 cells at radius 18, per the capture
const PAGER_WIDTH: f32 = PAGER_CELL;

/// The room the stats keep at the right of a card's tag row: three stats and their glyphs.
const TAG_ROW_RESERVE: f32 = 260.0;

/// The icon the reference puts beside a category, by the category's own name; anything it does
/// not name gets the plain tag.
fn category_glyph(category: &str) -> Glyph {
    match category {
        "Client" => Glyph::Monitor,
        "Lightweight" => Glyph::Sparkles,
        "Multiplayer" => Glyph::Users,
        "Optimization" => Glyph::Rocket,
        "Adventure" | "Challenging" | "Combat" => Glyph::Compass,
        _ => Glyph::Tag,
    }
}

/// Six results with the reference's own content, so a capture shows the real anatomy rather
/// than lorem ipsum: the first four are the cards the reference's first capture displayed, in
/// its order.
///
/// The icon URLs are the API's own `icon_url` answers for those six projects, fetched with
/// `tools/parity`'s probe rather than constructed: `/data/<id>/icon.png` is not a URL the CDN
/// serves -- it answers 400 on every id -- and the hashed `_96.webp` is.
///
/// Each `id` is read off that same CDN path, which is the only place the sample list carries
/// one: the six are real projects, so their Install buttons install the real packs rather
/// than a button that fails on an id nobody published.
/// Six cards the **tests** use as fixtures. Nothing draws this: an earlier build fell back to
/// it when a search returned nothing, which made an empty page look populated. It is named for
/// what it is so that a future call site cannot mistake it for data.
fn fixture_cards() -> Vec<Card> {
    fn tags(pairs: &[(&str, Glyph)]) -> Vec<(String, Glyph)> {
        pairs.iter().map(|(label, glyph)| ((*label).to_owned(), *glyph)).collect()
    }
    vec![
        Card {
            id: "1KVo5zza".to_owned(),
            title: "Fabulously Optimized".to_owned(),
            author: "Fabulously Optimized".to_owned(),
            summary: "Beautiful graphics, speedy performance and familiar features in a simple package. Wilderness Bound beta!".to_owned(),
            tags: tags(&[
                ("Client", Glyph::Monitor),
                ("Lightweight", Glyph::Sparkles),
                ("Multiplayer", Glyph::Users),
                ("Optimization", Glyph::Rocket),
                ("Fabric", Glyph::Box),
            ]),
            downloads: "19.38M".to_owned(),
            followers: "5,005".to_owned(),
            updated: "5 days ago".to_owned(),
            icon: "https://cdn.modrinth.com/data/1KVo5zza/d8152911f8fd5d7e9a8c499fe89045af81fe816e_96.webp".to_owned(),
        },
        Card {
            id: "l9m9tuPN".to_owned(),
            title: "Zombie Invade 100 Days".to_owned(),
            author: "FlameFire".to_owned(),
            summary: "Same as Forge Labs 100 Days Zombie Apocalypse in new Minecraft.".to_owned(),
            tags: tags(&[
                ("Client and server", Glyph::Users),
                ("Challenging", Glyph::TagCategorySwords),
                ("Forge", Glyph::Box),
            ]),
            downloads: "15.36M".to_owned(),
            followers: "9,659".to_owned(),
            updated: "10 months ago".to_owned(),
            icon: "https://cdn.modrinth.com/data/l9m9tuPN/fefe3f67c37744344d100638452c7bf059d586a1_96.webp".to_owned(),
        },
        Card {
            id: "5FFgwNNP".to_owned(),
            title: "Cobblemon Official Modpack [Fabric]".to_owned(),
            author: "Cobbled Studios".to_owned(),
            summary: "The official modpack of the Cobblemon mod, for Fabric!".to_owned(),
            tags: tags(&[
                ("Client and server", Glyph::Users),
                ("Adventure", Glyph::Compass),
                ("Lightweight", Glyph::Sparkles),
                ("Fabric", Glyph::Box),
            ]),
            downloads: "11.53M".to_owned(),
            followers: "2,732".to_owned(),
            updated: "12 hours ago".to_owned(),
            icon: "https://cdn.modrinth.com/data/5FFgwNNP/e7f9ee2e9d361623847853fe2ddce42f519ee64f.png".to_owned(),
        },
        Card {
            id: "Jkb29YJU".to_owned(),
            title: "COBBLEVERSE - Pokemon Adventure".to_owned(),
            author: "LUMYVERSE".to_owned(),
            summary: "Start a true Pokemon adventure in Minecraft: Cobblemon 1.7.3 [ALL 1025 Pokemon] - Mega Evolutions - Gyms & Badges - Starter Kit.".to_owned(),
            tags: tags(&[("Adventure", Glyph::Compass), ("Fabric", Glyph::Box)]),
            downloads: "7.31M".to_owned(),
            followers: "2,092".to_owned(),
            updated: "Updated today".to_owned(),
            icon: "https://cdn.modrinth.com/data/Jkb29YJU/581ecf54530972afb18a04660afd820f2f24f6c7.png".to_owned(),
        },
        Card {
            id: "4BV47HRn".to_owned(),
            title: "Better MC [FORGE]".to_owned(),
            author: "Shivaxi".to_owned(),
            summary: "The kitchen-sink pack that keeps its focus: quests, dungeons and a world that rewards exploring.".to_owned(),
            tags: tags(&[("Adventure", Glyph::Compass), ("Forge", Glyph::Box)]),
            downloads: "5.02M".to_owned(),
            followers: "1,940".to_owned(),
            updated: "3 days ago".to_owned(),
            icon: "https://cdn.modrinth.com/data/4BV47HRn/7f5ef7cc61b2593609aabef3ed07a866a4f11b73_96.webp".to_owned(),
        },
        Card {
            id: "CfpXILxG".to_owned(),
            title: "All the Mods 10".to_owned(),
            author: "ATMTeam".to_owned(),
            summary: "Every mod that matters, curated and tested, with a quest book to hold it together.".to_owned(),
            tags: tags(&[("Kitchen sink", Glyph::Boxes), ("NeoForge", Glyph::Box)]),
            downloads: "4.88M".to_owned(),
            followers: "1,512".to_owned(),
            updated: "6 days ago".to_owned(),
            icon: "https://cdn.modrinth.com/data/CfpXILxG/832f62314350e237f299f2ee39d90fb7d85b6419_96.webp".to_owned(),
        },
    ]
}

/// Cut `text` to a two-line card's worth, with an ellipsis when something was removed.
///
/// A character count rather than a pixel measurement: the summary's font is fixed at 13.5 and
/// its measure at 440, so the two are proportional, and a count is testable without a painter.
fn ellipsise(text: &str, limit: usize) -> String {
    if text.chars().count() <= limit {
        return text.to_owned();
    }
    let mut cut: String = text.chars().take(limit.saturating_sub(1)).collect();
    // Cut at a word boundary when there is one near the limit, so the card does not end on a
    // half word the way a hard slice does.
    if let Some(space) = cut.rfind(' ') {
        if space > limit / 2 {
            cut.truncate(space);
        }
    }
    cut.push('\u{2026}');
    cut
}



#[cfg(test)]
mod tests {
    use super::*;

    /// The strip always offers the ends, the page in hand and its neighbours, and says how wide the
    /// gap is when it skips one.
    #[test]
    fn the_strip_shows_the_first_last_and_the_page_in_hand() {
        assert_eq!(pager_items(1, 9), vec![Some(1), Some(2), None, Some(9)]);
        assert_eq!(pager_items(5, 9), vec![Some(1), None, Some(4), Some(5), Some(6), None, Some(9)]);
        assert_eq!(pager_items(9, 9), vec![Some(1), None, Some(8), Some(9)]);
        // One page is one number, and a gap of one is filled rather than elided.
        assert_eq!(pager_items(1, 1), vec![Some(1)]);
        assert_eq!(pager_items(1, 3), vec![Some(1), Some(2), Some(3)]);
    }

    /// A tag that does not fit stops the row, and the count it leaves is what the `+N` says.
    #[test]
    fn tags_stop_before_the_stats() {
        // Three 100-wide pills with 6 between them: 100 + 6 + 100 = 206 fits two.
        assert_eq!(tags_that_fit(&[100.0, 100.0, 100.0], 6.0, 206.0), 2);
        assert_eq!(tags_that_fit(&[100.0, 100.0, 100.0], 6.0, 318.0), 3);
        // Nothing fits in no room, and an empty row is empty.
        assert_eq!(tags_that_fit(&[100.0], 6.0, 99.0), 0);
        assert_eq!(tags_that_fit(&[], 6.0, 100.0), 0);
    }

    /// A summary inside the limit is returned untouched: a card must not pay for an
    /// ellipsis it does not need.
    #[test]
    fn a_short_summary_is_not_touched() {
        assert_eq!(ellipsise("Lightweight optimization modpack", 150), "Lightweight optimization modpack");
    }

    /// A long one is cut at a word boundary and marked, rather than sliced mid-word: the
    /// reference's cards end on a word.
    #[test]
    fn a_long_summary_is_cut_at_a_word_boundary() {
        let long = "word ".repeat(80);
        let cut = ellipsise(&long, 150);
        assert!(cut.chars().count() <= 150, "the cut respects the limit");
        assert!(cut.ends_with('\u{2026}'), "and says it was cut");
        assert!(!cut.trim_end_matches('\u{2026}').ends_with(' '), "no dangling space");
        assert!(!cut.contains("wor\u{2026}"), "never half a word");
    }

    /// The sample list is what a capture with an empty search box shows, so it must not be
    /// empty and must keep the reference's order.
    #[test]
    fn the_sample_list_carries_the_reference_six() {
        let sample = fixture_cards();
        assert_eq!(sample.len(), 6);
        assert_eq!(sample[0].title, "Fabulously Optimized");
        assert_eq!(sample[5].title, "All the Mods 10");
    }

    /// Every sample card names its project: the Install button is keyed by that id and the
    /// install asks the API for it, so an empty one would be a button that could only fail.
    #[test]
    fn every_sample_card_names_its_project() {
        for card in fixture_cards() {
            assert!(!card.id.is_empty(), "{} carries no project id", card.title);
        }
    }
}

#[cfg(test)]
mod instance_type_tests {
    use super::INSTANCE_TYPES;

    /// All four rows of the capture are here, in its order, and only the first two open a
    /// further step -- the chevron is in the reference on rows one and two.
    #[test]
    fn the_four_types_are_the_captures_four() {
        let titles: Vec<&str> = INSTANCE_TYPES.iter().map(|row| row.0).collect();
        assert_eq!(
            titles,
            [
                "Custom setup",
                "Start from a mod or modpack",
                "Upload a modpack",
                "Import instance"
            ]
        );
        assert!(INSTANCE_TYPES[0].2 && INSTANCE_TYPES[1].2);
        assert!(!INSTANCE_TYPES[2].2 && !INSTANCE_TYPES[3].2);
        // The capture's descriptions are not uniformly punctuated -- the last one ends a
        // sentence and the three above it do not -- so only their presence is asserted, which is
        // what a transcription can honestly claim.
        for (title, description, _) in INSTANCE_TYPES {
            assert!(!description.is_empty(), "{title} has no description");
        }
    }
}
