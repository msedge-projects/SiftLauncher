//! The pages, one module each.
//!
//! The shell draws the furniture -- title bar, rail, right panel -- and each page draws what
//! goes in the content column. That split is the reference's own: its shell is
//! `app/shell/{title-bar,sidebar}` and its pages are `pages/*`, and keeping the same seam
//! means a page can be ported, compared and signed off without touching the chrome.
//!
//! A page's signature is deliberately plain -- the rect it owns, the theme, and whichever
//! caches it needs -- rather than a handle to the shell. A page that could reach back into
//! the shell would be able to change the rail from inside a list, which is how a UI ends up
//! with navigation nobody can trace.

pub mod about;
pub mod home;
pub mod instance;
pub mod screenshots;
pub mod servers;
pub mod settings;
pub mod skins;
pub mod project;

use egui::Rect;

/// Which page the content column is showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    /// The welcome screen: a hero when there is nothing installed, the instance list
    /// otherwise.
    Home,
    /// Browse and search projects.
    Discover,
    /// One project's page, reached from a card in Discover.
    Project,
    /// One instance's page, reached from a row's Edit.
    Instance,
    /// The installed instances.
    Library,
    /// Skins: the player's own and the catalogue's.
    Skins,
    /// Screenshots taken in any instance.
    Screenshots,
    /// Servers the launcher knows about.
    Servers,
}

impl Page {
    /// The name the reference prints beside the breadcrumb.
    pub fn title(self) -> &'static str {
        if std::env::var_os("SIFT_TITLE_PROBE").is_some() { eprintln!("page title {:?}", self); }
        match self {
            Page::Home => "Home",
            Page::Discover => "Discover modpacks",
            // A project page's breadcrumb is its own name, which the shell supplies.
            Page::Project => "Project",
            Page::Instance => "Instance",
            Page::Library => "Library",
            Page::Screenshots => "Screenshots",
            Page::Servers => "Servers",
            Page::Skins => "Skins",
        }
    }

    /// The page's rect: the text column, not the surface it sits on.
    ///
    /// The reference's `.app-contents` is one rounded surface from x 64 to the window's
    /// right edge, split into a viewport and a 300 px sidebar; the page inside the viewport
    /// carries its own `px-6` (24 px). So the rect a page draws into starts at
    /// `CONTENT_LEFT + CONTENT_INSET` = 88, which is where the capture puts the first
    /// character of every heading, and stops 24 px before the sidebar.
    pub fn rect(self) -> Rect {
        use crate::geometry;
        Rect::from_min_size(
            egui::pos2(
                geometry::TEXT_LEFT,
                // The breadcrumb lives on the title bar; every page starts below it.
                geometry::TITLE_BAR,
            ),
            egui::vec2(
                geometry::panel_left() - geometry::CONTENT_INSET - geometry::TEXT_LEFT,
                geometry::window().1 - geometry::TITLE_BAR,
            ),
        )
    }
}
