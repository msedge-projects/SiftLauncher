//! Where things are, in the reference's own pixels.
//!
//! Every number here was measured from a capture of Modrinth App 0.22.0 at 1440x900
//! with `tools/parity/probe.py`, and every one of them is also stated in the reference's
//! own stylesheet. Where the two agree, both are quoted, because the stylesheet says what
//! the author intended and the capture says what the engine drew.
//!
//! The shell is a grid, in the reference's own words (`src/App.vue`):
//!
//! ```css
//! .app-grid-layout, .app-contents { --top-bar-height: 3rem; --left-bar-width: 4rem; --right-bar-width: 300px }
//! .app-grid-layout { grid-template: "status status" "nav dummy" 1fr / auto 1fr; background: var(--color-raised-bg) }
//! .app-contents    { left: var(--left-bar-width); top: var(--top-bar-height);
//!                    background: var(--color-bg); border-top-left-radius: var(--radius-xl);
//!                    position: absolute; bottom: 0; right: 0; display: grid;
//!                    grid-template-columns: 1fr 0 }
//! .app-contents.sidebar-enabled { grid-template-columns: 1fr 300px }
//! .app-sidebar { width: 300px; background: var(--brand-gradient-bg) }
//! ```
//!
//! So the window is: a 48 px raised strip across the top holding the wordmark and the
//! window controls; a 64 px raised rail down the left; and one rounded surface filling
//! everything else, split into a scrolling viewport and a 300 px sidebar whose background
//! is a green-tinted gradient rather than a flat colour. The page inside the viewport
//! carries its own `px-6` padding, which is why the surface's left edge is at 64 and a
//! page's text starts at 88.
//!
//! Re-measure with:
//!
//! ```text
//! python3 tools/parity/probe.py row .scratch/cap/ref-home.png 450
//! python3 tools/parity/probe.py col .scratch/cap/ref-home.png 700
//! ```

/// The window the captures are taken at.
pub const WINDOW: (f32, f32) = (1440.0, 900.0);

/// `--top-bar-height: 3rem`, and row 700 of the reference capture: the raised strip runs
/// 0..47 and the content surface begins at 48.
pub const TITLE_BAR: f32 = 48.0;

/// `--left-bar-width: 4rem`: the raised rail occupies x 0..63.
pub const RAIL: f32 = 64.0;

/// The rail's own `p-[0.5rem]`: an 8 px gutter either side of a 48 px button.
pub const RAIL_PAD: f32 = 8.0;

/// `NavButton`'s `w-12 h-12`: the round target the icon sits in.
pub const RAIL_BUTTON: f32 = 48.0;

/// The rail's `gap-[0.25rem]` between buttons.
pub const RAIL_GAP: f32 = 4.0;

/// One rail button plus the gap under it: the pitch between two icon centres.
pub const RAIL_PITCH: f32 = RAIL_BUTTON + RAIL_GAP;

/// The first button's top edge **from the rail's top**: the rail is `pt-0`, so it is 0 and
/// the active plate's first row is the first row under the title bar (capture: 48..95).
pub const RAIL_FIRST: f32 = 0.0;

/// The content surface's left edge: `left: var(--left-bar-width)`.
pub const CONTENT_LEFT: f32 = RAIL;

/// `--right-bar-width: 300px`, the sidebar's own `width: 300px`.
pub const PANEL: f32 = 300.0;

/// The sidebar's left edge in window coordinates.
// **The window is not always 1440 wide.** This module used to derive the sidebar edge from the constant
// `WINDOW.0`, which is where the window *opens* -- so at 1280 the sidebar was still drawn at 1140 and its right
// third fell outside the window. The size the frame is actually drawing is recorded once per frame.
static WINDOW_SIZE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Record the size the frame is drawing, before anything asks for a derived edge.
pub fn set_window(width: f32, height: f32) {
    let packed = ((width as u32 as u64) << 32) | height as u32 as u64;
    WINDOW_SIZE.store(packed, std::sync::atomic::Ordering::Relaxed);
}

/// The size the frame is drawing: the live window, or the size it opens at before the first frame sets it.
pub fn window() -> (f32, f32) {
    match WINDOW_SIZE.load(std::sync::atomic::Ordering::Relaxed) {
        0 => WINDOW,
        packed => ((packed >> 32) as u32 as f32, (packed & 0xffff_ffff) as u32 as f32),
    }
}

/// The sidebar edge, from the window that is drawing.
pub fn panel_left() -> f32 {
    window().0 - PANEL
}

/// A page's own `px-6` inside the surface: text starts 24 px in from the surface's edge,
/// which is why the reference's first column of text begins at x 88.
pub const CONTENT_INSET: f32 = 24.0;

/// Where a page's text column starts in window coordinates.
pub const TEXT_LEFT: f32 = CONTENT_LEFT + CONTENT_INSET;

/// `--radius-xl: 1.25rem`: the content surface's rounded top-left corner.
pub const RADIUS_XL: f32 = 20.0;

/// The left edge of the back button: the wordmark ends at 162, `ml-2` (8) is 170, and the
/// capture puts the outlined square at 176..203.
pub const BREADCRUMB_ARROWS: f32 = 176.0;

/// The back/forward button's `!h-7 !w-7`.
pub const ARROW_BUTTON: f32 = 28.0;

/// `gap-2` between the two arrows.
pub const ARROW_GAP: f32 = 8.0;

/// Where the route's own icon starts on the title bar: capture, x 262..277.
/// **256, measured.** The reference's breadcrumb icon starts at x 256 and its name at 278; this build put the icon at 262
/// and the name at 283. The badge is only ever drawn with the name five points to its right, so one constant moves both.
pub const BREADCRUMB_BADGE: f32 = 256.0;

/// The three window controls: two 3rem buttons and the 3.75rem close button.
pub const WINDOW_CONTROLS: f32 = 48.0 * 2.0 + 60.0;

/// Where the window controls begin, measured from the right edge.
/// The window controls edge, from the window that is drawing.
pub fn controls_left() -> f32 {
    window().0 - WINDOW_CONTROLS
}

/// The third window control is wider: `.window-controls .titlebar-button:last-child`
/// carries `width: 3.75rem`.
pub const CLOSE_BUTTON: f32 = 60.0;

/// The tab strip's rounded container begins below the title bar.
pub const TAB_CONTAINER_TOP: f32 = 48.0;

/// The tab container's height: 36-point pills inside 8-point padding.
pub const TAB_CONTAINER: f32 = 52.0;

/// One tab pill's height.
pub const TAB_PILL: f32 = 36.0;

/// The search row's height: the field and the two selects are the same 40-point control.
pub const SEARCH_ROW: f32 = 40.0;

/// The tab strip's height: the label sits at row 105 and the strip's bottom rule is the
/// next horizontal edge below it.
pub const TAB_STRIP: f32 = 36.0;

/// A result card's height.
/// Measured from the reference's Discover: its first card's frame runs y 117..262 -- 145 points -- where
/// this build's ran 112..254.
/// The card's height. The reference's first card's Install ring starts at 247 with this build's at 246, so the
/// top is right; its pitch is 154 against this build's 157, and a 142-point card measured *worse* (141 077
/// against 139 591), so the three points are inside the card's own layout and not in this constant.
/// Measured from the card frames themselves (the page ground against the card's fill at x 600): the reference's
/// run 230..371, 384..525, 538..679 -- 141 tall on a 154 pitch -- where this build's ran 144 tall on 160.
pub const CARD: f32 = 141.0;

/// The gap between two result cards.
/// The gap the layout adds between cards, on top of egui's own item spacing: 141 + 9 + 4 gives the reference's
/// 154-point pitch between card frames (230, 384, 538, 692 in its capture).
pub const CARD_GAP: f32 = 10.0;

/// Where the first tab's label starts, inside the tab strip.
pub const FIRST_TAB_LABEL: f32 = 196.0;

#[cfg(test)]
mod tests {
    use super::*;

    /// The grid has to add up, or every page inherits the error.
    #[test]
    fn the_columns_do_not_overlap_and_reach_the_window_edge() {
        assert_eq!(CONTENT_LEFT, 64.0, "--left-bar-width: 4rem");
        assert_eq!(TEXT_LEFT, 88.0, "the surface's edge plus the page's px-6");
        assert_eq!(panel_left(), 1140.0, "1440 - 300");
        assert_eq!(panel_left() + PANEL, WINDOW.0, "the sidebar ends at the window edge");
        assert_eq!(controls_left(), 1284.0, "48 + 48 + 60 from the right edge");
    }

    /// The rail stacks 48 px buttons four pixels apart starting at the surface's top.
    #[test]
    fn the_rail_pitch_is_the_button_plus_the_gap() {
        assert_eq!(RAIL_PITCH, 52.0);
        assert_eq!(RAIL_PAD * 2.0 + RAIL_BUTTON, RAIL, "8 + 48 + 8 is the rail's 64");
        assert!(TITLE_BAR + RAIL_FIRST + 7.0 * RAIL_PITCH < WINDOW.1, "seven buttons fit");
    }

    /// The title bar's contents have to fit between the wordmark and the controls.
    #[test]
    fn the_title_bar_contents_do_not_collide() {
        assert!(BREADCRUMB_ARROWS > 162.0, "the arrows start after the wordmark");
        assert!(
            BREADCRUMB_ARROWS + ARROW_BUTTON * 2.0 + ARROW_GAP < BREADCRUMB_BADGE,
            "the badge follows the forward arrow"
        );
        assert!(BREADCRUMB_BADGE < controls_left(), "the breadcrumb is left of the controls");
    }
}
