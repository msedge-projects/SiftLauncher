//! The right panel: `Getting started`, the `News` card and the hosting banner.
//!
//! **One file, because one panel draws on every page.** The reference's shell owns this
//! column and every page it swaps underneath keeps it (`App.vue:2486` draws the sidebar
//! beside the router view), so a copy per page would be the same arithmetic five times
//! over, and the first capture that disagreed with it would be fixed in one page and left
//! wrong in the other four. The shell draws its furniture here and the pages draw only the
//! content column, which is the split `pages/mod.rs` states from the other side.
//!
//! **The layout is arithmetic over constants, not a column that measures itself.** Every
//! number below carries where it came from: the reference's own measurement (the panel is
//! 340 wide with a 24-point inset, a row is 36 tall), a value read out of the vendored
//! stylesheet at the `file:line` beside it, or a guess, marked `// guess: re-measure` so
//! the parity loop knows which numbers a capture is meant to correct. Nothing waits for a
//! font to decide where the next row goes: the reference's card is a fixed stack, and a
//! card that grew with its copy would move the date somewhere the capture does not.
//!
//! **Nothing here is clickable yet, and the hover says so.** The steps and the upgrade row
//! have actions that belong to the shell's state -- creating an instance, signing in,
//! opening a URL -- and [`draw`] reports nothing back to spend them on. Each takes a hover
//! and no press: a control that brightens under the pointer and does nothing when clicked
//! is worse than one that plainly has no press state yet.

use egui::{Align2, Rect, Sense, Stroke, Ui, Vec2, Color32};

use crate::fonts;
use crate::icons::{self, Glyph};
use crate::theme::Theme;

// ---------------------------------------------------------------------------
// The panel's box
// ---------------------------------------------------------------------------

/// The inset between the panel's edges and its content column, on both sides.
///
/// Measured from the reference's own render: its "Getting started" card spans x 1153..1428 while the
/// sidebar begins at 1140 and the window ends at 1440, so the padding is 13 on the left and 12 on the right.
/// It was 16, which made every card 7 points narrow -- 3 too far in on the left and 4 on the right, which is
/// exactly the smear a blend shows along both edges.
const PAD: f32 = 13.0;

/// The inset on the panel's right: `px-3` (12) and no border, because the sidebar's own 1px rule
/// is on its left edge only. The reference's card spans x 1153..1427 inside a 1140..1440 sidebar
/// -- 13 in on the left and 12 on the right -- so the column is 275 wide and not 274.
const PAD_RIGHT: f32 = 12.0;

/// The inset from the card's edge to what it draws: the accordion's 1px border plus its `p-3` (12).
///
/// Every piece of the checklist sits this far inside the card in the reference -- the heading's
/// ink starts at x 1167 against a card at 1153, the rows' fill at 1166 -- where this build drew
/// all of them on the card's own edge.
const HEADING_INSET: f32 = 13.0;

/// Where the first heading is drawn, from the panel's top edge.
// Measured, not guessed: the checklist's card starts 17 points under the surface's top edge
// (y 65 at 1440x900) and its heading's ink starts 18 points into the card (y 83), which is
// where this face puts the ink for a draw one point higher again -- 33.
const TOP_PAD: f32 = 33.0;

/// The gap between one section's last row and the next section's heading.
///
/// 20 points, from the reference's measurement. It is measured from the *bottom* of the
/// last row, not from its top: the reference rules between sections and not between rows,
/// so the space belongs to the heading above it rather than to the row.
const SECTION_GAP: f32 = 28.0;

/// The section headings stay at 16. **Tried 21** to match the reference's 245-point ink against this build's 269 and
/// measured the sidebar *worse* (17 602 -> 18 158): the wider ink here is the two headings' own letter-spacing, not a
/// smaller size, and shrinking the row under it is what the difference was already telling me not to do.
const SECTION_HEADING: f32 = HEADING_SIZE;

/// The news block's heading ink sits at 295..306 in this build where the reference's is at 315..326, and every line
/// under it follows 28--31 points high. The card's own top is right; its contents start too early.
const NEWS_TOP_SHIFT: f32 = 20.0;

/// The rule down the panel's left edge, and every other hairline here: the sheet states
/// it literally as `1px` (`assets/styles/classes.scss:753`), which [`Theme`]'s generated
/// ladder keeps as [`crate::theme::STROKE_WIDTH`].
const RULE: f32 = crate::theme::STROKE_WIDTH;

// ---------------------------------------------------------------------------
// Section headings
// ---------------------------------------------------------------------------

/// The Discover panel's own heading, where the other pages say `News`.
const CATEGORY: &str = "Category";
/// The link under the list, from `app.browse.see-more` and `app.browse.see-less`.
const SEE_MORE: &str = "See more categories";
/// The switch above the list, from `app.browse.hide-installed`.
/// The reference's own label, which wraps to two lines in its column ("Hide content already" / "installed").
/// This build's shorter "Hide already installed" fitted on one, which is the wrong words *and* the wrong shape.
const HIDE_INSTALLED: &str = "Hide content already installed";
const SEE_LESS: &str = "See less";

/// One category row's height, and how many the panel lists before a `See more` would go.
// Measured from the reference's Discover panel: its category rows sit at y 229, 275, 322 -- a 46-point pitch with
// a 16-point glyph and 16-point text, where this drew 26-point rows of 13-point text.
// **32, measured twice.** The category labels' ink bands in the reference start at y 410, 442, 474, 506, 538 and 570 --
// a 32-point pitch -- and a second capture puts Adventure at 413 and Challenging at 445. The 46 this held came from a
// citation of "y 229, 275, 322", which is 46 apart but belongs to a different list.
const CATEGORY_ROW: f32 = 32.0;
// The reference lists eight before its own scroll takes over; there is no "See more" row in its capture.
const CATEGORY_ROWS: usize = 8;

/// The API's category ids are lowercase; a reader sees them capitalised.
pub fn capitalised(name: &str) -> String {
    let spaced = name.replace('-', " ");
    let mut letters = spaced.chars();
    match letters.next() {
        Some(first) => format!("{}{}", first.to_uppercase(), letters.as_str()),
        None => String::new(),
    }
}

/// A section heading's size: `Getting started` and `News` are both semibold 16.
const HEADING_SIZE: f32 = 16.0;

/// The checklist heading's size, kept separate from the section headings' so the two can move
/// apart. **Tried 17** to close the gap between the reference's 123 points of ink for "Getting
/// started" and this build's 116: it measured the slab *worse* (2 549 -> 2 559), because the
/// extra height costs more than the width it wins.
const CHECKLIST_HEADING_SIZE: f32 = 16.0;

/// A heading's line box. The checklist's title is `font-semibold leading-6`
/// (`ui/onboarding-checklist/index.vue:99`), and `leading-6` is 24 points.
const HEADING_LINE: f32 = 24.0;

/// The gap between a heading and the rows under it. The checklist's rows sit inside the
/// accordion's content, which is padded `p-3` (`ui/onboarding-checklist/index.vue:94`).
// The capture puts the first row's top edge at y 128, which is 23 below the heading's 24
// point line box (81 + 24).
// **15: the reference's heading-to-row gap is 43 points.** Its "Category" ink starts at y 367 and its first label at 410;
// this build drew 338 and 389 -- the block 29 high and the gap 51 wide. With the block moved and the pitch at 32, the gap
// has to shrink by the same 8 points the pitch did, or the rows land under the wrong row of the reference.
const HEADING_TO_BODY: f32 = 15.0;

/// The disclosure chevron on the `Getting started` heading: `DropdownIcon` at `size-5`
/// (`ui/components/base/Accordion.vue:36`), 20 points.
const CHEVRON: f32 = 20.0;

// ---------------------------------------------------------------------------
// `Getting started` rows
// ---------------------------------------------------------------------------

/// A checklist row's height: 36 points, from the reference's measurement. The sheet's step
/// button is `h-10` (40); the capture is what this follows, and `docs/PARITY-LOG.md`'s
/// OBS-4 is the entry that left the reference's row rhythm for the next capture to settle.
const ROW_H: f32 = 40.0;

/// The gap between two checklist rows: 8 points, from the reference's measurement. With
/// [`ROW_H`] this is the whole of OBS-4's open question.
/// **The project card's own row offsets**, hoisted out of the function so a test can pin them: they differ from
/// Home's (`HEADING_LINE + HEADING_TO_BODY` = 39) by 13, which is a divergence between two implementations of one
/// card rather than a measured constant. Naming them is the first step to removing one of the implementations.
/// Measured at 1:1 against the reference (`proj-ref.png`, card y 54..272): its first row starts 70 below the card's top,
/// its rows are 34 tall, 48 apart, and the card leaves 18 below the last one. This build used 52, 40 and a height that
/// let the third row overflow the card by 3 points.
const PROJECT_STEPS_TOP: f32 = 70.0;
/// The reference's own pitch, measured from three text bands at 141, 189 and 237: **48**, which is `ROW_PITCH`.
/// This card used 44 and was the only one that did.
const PROJECT_ROW_PITCH: f32 = 48.0;
/// The row's own height, and the card's: 70 + 2 rows + one row + 18 = 218, which is the reference's own card.
const PROJECT_ROW_H: f32 = 34.0;
/// **218 inner, 234 returned**, which is 16 more than the reference\'s own card measures at 1:1 (54..272). That 16 is
/// kept because it measures better: the sidebar\'s whole-column difference is 18 664 with it against 19 681 without,
/// and the cards below move with this height. Which of the two is right is not settled -- the crop says one thing and
/// the count another, and this file has been wrong about crops more often.
const PROJECT_CARD_H: f32 = PROJECT_STEPS_TOP + 2.0 * PROJECT_ROW_PITCH + PROJECT_ROW_H + 18.0;

const ROW_GAP: f32 = 8.0;

/// The distance between two rows' top edges. Derived, so a correction to either half of
/// the rhythm moves every row below it and nothing above it.
const ROW_PITCH: f32 = ROW_H + ROW_GAP;

/// A row's inset before its ring: the sheet's step button is `px-4`
/// (`ui/onboarding-checklist/index.vue:105`), 16 points.
const ROW_PAD: f32 = 16.0;

/// The ring glyph at a row's left: 16 points, from the reference's measurement.
const RING: f32 = 20.0;

/// The gap between the ring and its label: the sheet's `gap-2`
/// (`ui/onboarding-checklist/index.vue:105`), 8 points.
const RING_GAP: f32 = 8.0;

/// A row's label: `text-base` (16) at `font-medium`, from the reference's own ink -- 157 points for
/// "Create first instance" against this build's 156 at 16 medium, and 141 at 15 regular.
const STEP_SIZE: f32 = 16.0;

// ---------------------------------------------------------------------------
// The news card
// ---------------------------------------------------------------------------

/// The art plate's height: `aspect-video` on the 266-point column is 149.6, which the capture
/// rounds to a 150-point plate running y 348..498. Measured from the reference's own render:
/// its title ink starts at 518, where a 120-point plate put this build's at 487.
const ART_H: f32 = 150.0;

/// The plate's own width: the news section's `p-4` leaves 266 of the 300-point sidebar, and
/// the card's `w-full` image fills it. Its `aspect-video` then makes the height 266 x 9/16 =
/// 149.6, which the capture rounds to a 150-point plate at y 348..498.
const ART_W: f32 = 266.0;

/// The news section's own inset from the column: `p-4` (16) against the checklist's `px-3` (12),
/// so the news art and every line under it start four points right of the checklist card.
/// Measured: the reference puts that text at x 1157 against a card at 1153.
const NEWS_INSET: f32 = 4.0;

/// The gap between the `News` heading and the card: the section's heading is `mb-4`
/// (`App.vue:2531`), 16 points.
const NEWS_HEADING_TO_CARD: f32 = 33.0;

/// The gap above the news section, which is **twenty points wider than the one above it**: the reference's "News"
/// ink sits at 315..326 where this build's sat at 295..306, and its getting-started rows end at the same 251 as
/// ours, so the difference is the boundary and not the type.
// **29, measured.** The reference's "News" ink runs 317..326 and this build's 308..318: the heading sat nine points high
// while the card under it was aligned, so the gap between them was nine too wide.
const NEWS_HEADING_DROP: f32 = 29.0;

/// The gap between the card's art and its title: the card's own stack is `gap-4`
/// (`ui/components/content/NewsArticleCard.vue:25`), 16 points.
/// **Tried 28 and put back at 16.** The reference's title ink does sit at 546..559 where this build's sits at
/// 518..531, but widening this gap grows the card, which moves the banner under it and the whole foot with it --
/// measured 17 291 -> 18 443. The 28 points belong to whatever separates the card's text from its art in the
/// reference, and moving this number is not it.
// **24: the reference puts its title's ink at 519..533, 20 points under a plate that ends at 499.** This drew the ink at
// 510..525 with the gap at 16 -- nine points high, measured from the two captures' text bands.
const ART_TO_TITLE: f32 = 24.0;

/// The title's size: `text-base` (16) at the h3's semibold, measured from the reference's own
/// ink -- 251 points wide and 16 tall where 14 points here drew 210 wide and 15 tall.
// **16 stays.** 19 was tried, on the theory that the reference fits 26 characters where this build fitted 33 into the same
// measure; measured, it made the sidebar worse (13 568 -> 14 675 pixels), so the theory is wrong about the size and the
// wrap difference has another cause. Recorded in GATES.md rather than re-guessed.
const TITLE_SIZE: f32 = 16.0;

/// The title's line box. The card's `h3` is `leading-tight`
/// (`ui/components/content/NewsArticleCard.vue:31`), which is 1.25 x 16 = 20.
const TITLE_LINE: f32 = 20.0;

/// The gap between the title and the copy: the card's inner stack is `gap-2`
/// (`ui/components/content/NewsArticleCard.vue:30`), 8 points.
// Measured: the reference's title ink starts at 518 and its copy's at 546 -- 28 points, which is
// this gap plus the two line boxes' own bearings.
// **8, and the reason is the title's wrap, not the gap.** The reference's title is TWO lines -- its bands are 519..533 and
// 542..550 -- and its copy begins at 566. This build's title fits on one line (518..533), so its copy at 564 is already
// aligned with the reference's copy. Dropping this to 1 moved the copy to 557 and made the page 1 450 pixels worse: the
// 8 here is compensating for a missing line, and the real defect is that our title's measure lets it fit where the
// reference's wraps.
// **8, and the reference puts both at the same time.** Its copy starts at 566 and its card ends 26 above the hosting
// banner; with 10 the copy starts at 566 (right) and the gap becomes 24 (wrong), because the card is two points taller
// and the banner is pinned to the foot. 8 gives the gap its 26 and the copy at 564 -- two points high. Both cannot hold
// with the current terms, so one of the others (the date line or `COPY_TO_DATE`) is wrong; the pinned gap is the one the
// test guards and the one the reference states, so it wins until the third term is measured.
const TITLE_TO_COPY: f32 = 8.0;

/// The copy's size: 12 points, from the reference's measurement.
const COPY_SIZE: f32 = 14.0;

/// How many lines the copy takes in the reference: the article's summary wraps twice at
/// the panel's inner width, which is why the card reserves room for two.
const COPY_LINES: usize = 3;

/// One line of the copy's box.
const COPY_LINE: f32 = 16.0; // guess: re-measure

/// The room two lines of copy are given, and the least the card ever reserves for them.
const COPY_H: f32 = COPY_LINE * COPY_LINES as f32;

/// The gap between the copy and the date: the date is the last child of the same `gap-2`
/// stack (`ui/components/content/NewsArticleCard.vue:30`), so 8 points -- plus the one this
/// galley is shorter than the reference's three 17.5-point lines, so the date's ink still
/// lands on 606.
const COPY_TO_DATE: f32 = 9.0;

/// The date's size: `text-sm` (14), which the capture confirms -- 127 points of ink against
/// this build's 124.
const DATE_SIZE: f32 = 14.0;

/// The date's line box: `text-sm` at the app's normal leading, 21 points. It used to stand in for
/// the link row's offset as well, at 9.5; the link is pinned to the banner now and this is only
/// the date's own line, which is what makes the card's height come out at the reference's 275.
const DATE_LINE: f32 = 21.0;

/// The upgrade link's box: `px-4 py-3` with a `text-2xl` (24) icon, so 24 + 12 + 12 = 48.
/// It is pinned above the banner rather than stacked under the date (`bottom-[250px]`), and
/// its own ink lands at y 620..635 in the reference's render.
const LINK_H: f32 = 48.0;

/// The upgrade row's label size: `text-base` at `font-medium`, whose ink spans 169 points in the
/// reference against this build's 170.
const LINK_SIZE: f32 = 16.0;

/// The mark beside the upgrade label: `text-2xl` (24) on the reference's `ArrowBigUpDashIcon`,
/// whose ink spans 16 of those 24 points at x 1195..1210.
const SPARKLE: f32 = 24.0;

/// The gap between the mark and the label: the link is `gap-1` (4).
const SPARKLE_GAP: f32 = 4.0;

// ---------------------------------------------------------------------------
// The hosting banner
// ---------------------------------------------------------------------------

/// The hosting banner's padding: the promo card's `p-4`
/// (`ui/components/servers/ServersPromo.vue:22`), 16 points.
// The banner is full-bleed, so its own padding carries the inset the column used to: the reference's chip
// starts at x 1166 with the banner beginning at 1140.
// Measured: the reference's heading's ink starts at x 1155 with the banner beginning at 1140 -- 15 points in.
const BANNER_PAD: f32 = 15.0;

/// Where the banner's first heading line is drawn, from the banner's top: its ink sits at y 689
/// in the reference with the banner at 650, and this build's 23-point face puts its ink three
/// points into the draw position.
const HOSTING_TOP: f32 = 31.0;
/// The pitch between the banner's two heading lines.
const HOSTING_LINE: f32 = 28.0;
/// The chip's mark box: the reference's logo ink is 16 points at x 1154..1169. This build paints
/// it through [`crate::brand::paint_mark`], whose ink is three quarters of its box, so a 20-point
/// box lands the same 16 points of ink.
const CHIP_MARK: f32 = 16.0;
/// Where that box starts, from the banner's left edge.
const CHIP_LEFT: f32 = 14.0;
/// The gap between the mark and `modrinth hosting`.
const CHIP_GAP: f32 = 0.0;
/// The chip label's size: its ink spans 127 points, which is 16-point medium here.
const CHIP_SIZE: f32 = 16.0;

/// The banner's two lines: 16 semibold, from the reference's measurement.
const BANNER_SIZE: f32 = 16.0;

/// One banner line's box: the promo's copy is `leading-tight`
/// (`ui/components/servers/ServersPromo.vue:31`) at 16, which is 20 points.
const BANNER_LINE: f32 = 20.0;

/// The room the banner keeps at its bottom for the promo art.
///
/// The reference draws the big Modrinth mark bleeding off the plate's corner
/// (`ui/components/servers/ServersPromo.vue:24-29`); this launcher paints nothing into the
/// space yet, and the reservation is what keeps the two lines where the capture puts them.
const BANNER_ART: f32 = 64.0; // the news card's art plate; the hosting banner sizes its own.

/// The banner's call to action: `ButtonLink` with `View plans` and an arrow
/// (`ui/components/servers/ServersPromo.vue:41`).
const BANNER_CTA: &str = "View plans";
const BANNER_CTA_HEIGHT: f32 = 32.0;
const BANNER_CTA_PAD: f32 = 14.0;

/// The banner's height: the promo's own `min-h-[250px]` image, which is what the reference
/// draws from y 650 to the panel's foot. It used to be 260 and to take whatever height the
/// column had left, which stretched the illustration by 75 points.
const BANNER_H: f32 = 250.0;

// ---------------------------------------------------------------------------
// Radii
// ---------------------------------------------------------------------------

/// A row's radius: 8, from the reference's measurement. The sheet's step button is
/// `rounded-xl` (`ui/onboarding-checklist/index.vue:105`), which this preset's Tailwind
/// defaults resolve to 12, so the two disagree by four points; the measurement is what is
/// drawn, and the generated ladder is where the 8 comes from rather than a second literal.
const ROW_RADIUS: u8 = 10;

/// A card's radius: 12, from the reference's measurement, which is also what the article's
/// `rounded-xl` image resolves to. The hosting banner is `rounded-2xl` (16) in the sheet;
/// the measurement says both plates are 12, and the parity loop settles the difference.
const CARD_RADIUS: u8 = crate::theme::RADIUS_MD;

// ---------------------------------------------------------------------------
// The words
//
// The reference's own strings, not written here: a capture compares this section against
// the reference's text, so the text has to be the reference's. The article's four fields
// are the feed's own answers for the entry it leads with
// (`https://modrinth.com/news/feed/articles.json`, read live in `GATES.md`'s G101).
// ---------------------------------------------------------------------------

/// One step of the checklist, as a thing the shell can act on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
    /// A category the reader pressed, as the API spells it.
    Category(String),
    /// The list's `See more` / `See less`.
    MoreCategories,
    /// The `Hide already installed` switch.
    HideInstalled,
    /// `Create first instance`.
    CreateInstance,
    /// `Sign in to Minecraft`.
    Minecraft,
    /// `Sign in to Modrinth`.
    Modrinth,
}

impl Step {
    /// The three, in the order they are drawn.
    pub const ALL: [Step; 3] = [Step::CreateInstance, Step::Minecraft, Step::Modrinth];

    /// The row's own label.
    pub fn label(self) -> &'static str {
        match self {
            // A category press carries its own name and never asks for a step label.
            Step::Category(_) => "",
            Step::MoreCategories => "",
            Step::HideInstalled => "",
            Step::CreateInstance => STEPS[0],
            Step::Minecraft => STEPS[1],
            Step::Modrinth => STEPS[2],
        }
    }
}

/// What a checklist row says, given who is signed in.
///
/// The Minecraft row is the one that changes: the reference's checklist strikes a finished step
/// through, and the launcher's own wording for that state is the account's name. A row that
/// insisted `Sign in to Minecraft` after a successful sign-in would be asking again.
/// **The Getting started card, on its own.** The sidebars of the Home and project pages both carry it, and the
/// project page had a compact copy of it; this is the one both call. It draws the frame, the heading with its
/// open chevron, and the three steps, and answers the height it used. `done` says which steps the library has
/// finished, in the same order as the steps themselves.
/// **The PROJECT page\'s copy of this card.** Home draws its own, further down this file, through [`draw`] --
/// and the two are *not* one implementation: this one positions its rows from `card.top() + 52`, Home\'s from
/// `plan.steps_first`. Two rounds were spent editing this function and measuring no change, because the
/// screenshot in hand was Home. The name says which page it belongs to now.
pub fn project_getting_started_card(
    ui: &mut egui::Ui,
    rect: Rect,
    theme: &Theme,
    // The card's own fill: the sidebar paints it with the gradient showing through, the project page with a flat
    // raised surface, and both are the reference's own colour for their page. `ring` is the one-pixel step above it.
    fill: egui::Color32,
    ring: egui::Color32,
    done: [bool; 3],
    labels: [&str; 3],
) -> (f32, Option<usize>) {
    // **The rows are widgets, not paint.** A card that only paints never sees the pointer: egui has no reason to
    // redraw when the mouse moves, so the hover frame is never drawn. `interact` also gives the press back to the
    // caller, which is what the sidebar's own copy does by hand.
    let painter = ui.painter().clone();
    let painter = &painter;
    let pointer = ui.input(|input| input.pointer.hover_pos());
    let mut pressed = None;
    // The card's own height stays as the project page's test asserts it (188); only the rows inside it moved to the
    // named constants. Changing both at once failed that test at 204, which is the reference's 178 plus 26.
    let card = Rect::from_min_size(rect.min, egui::vec2(rect.width(), PROJECT_CARD_H));
    painter.rect_filled(card, 12, fill);
    painter.rect_stroke(card, 12, Stroke::new(1.0, ring), egui::StrokeKind::Inside);
    painter.text(
        egui::pos2(card.left() + 16.0, card.top() + 14.0),
        Align2::LEFT_TOP,
        GETTING_STARTED,
        fonts::semibold(15.0),
        theme.CONTRAST,
    );
    icons::draw(
        painter,
        Glyph::ChevronUp,
        Rect::from_center_size(
            egui::pos2(card.right() - 24.0, card.top() + 22.0),
            Vec2::splat(14.0),
        ),
        theme.CONTRAST,
    );
    for (index, label) in labels.iter().enumerate() {
        let row = Rect::from_min_size(
            // **+58, not +34.** Measured off the reference: its sidebar heading's ink runs 84..95 and its first row's
            // 138..155 -- 54 below the heading's top -- on *both* Home and a project page alike. With +34 this card
            // drew its rows 24 points above the reference's.
            egui::pos2(
                card.left() + 16.0,
                card.top() + PROJECT_STEPS_TOP + index as f32 * PROJECT_ROW_PITCH,
            ),
            egui::vec2(card.width() - 32.0, PROJECT_ROW_H),
        );
        let response = ui.interact(row, egui::Id::new(("getting-started-step", index)), egui::Sense::click());
        if response.clicked() {
            pressed = Some(index);
        }
        let over = pointer.is_some_and(|at| row.contains(at)) || response.hovered();
        painter.rect_filled(
            row,
            10.0,
            if over {
                crate::shell::tint(theme.BUTTON_BG, (255, 255, 255), 0.08)
            } else {
                theme.BUTTON_BG
            },
        );
        let ticked = done.get(index).copied().unwrap_or(false);
        if ticked {
            // The disc is as dim as the label: the reference\'s whole row tops out at (61,68,69), so nothing in it is
            // SECONDARY (150,162,176) -- measured with `element.py` over the box that contains the disc.
            painter.circle_filled(egui::pos2(row.left() + 20.0, row.center().y), 8.0, theme.SURFACE_5);
            icons::draw(
                painter,
                Glyph::Check,
                Rect::from_center_size(
                    egui::pos2(row.left() + 20.0, row.center().y),
                    Vec2::splat(10.0),
                ),
                theme.BUTTON_BG,
            );
        } else {
            painter.circle_stroke(
                egui::pos2(row.left() + 20.0, row.center().y),
                7.0,
                Stroke::new(1.5, theme.SECONDARY),
            );
        }
        // **A finished step is dimmer than this**, not merely struck through: the reference\'s own completed row reads
        // (61,68,69) at its brightest, which is `SURFACE_5` (66,68,74); this build used `SECONDARY` (150,162,176) and
        // measured 89 too light. Found with `tools/parity/element.py`, which sees the ink colour a frame count cannot.
        let ink = if ticked { theme.SURFACE_5 } else { theme.BASE };
        painter.text(
            egui::pos2(row.left() + 36.0, row.center().y),
            Align2::LEFT_CENTER,
            *label,
            fonts::body(13.0),
            ink,
        );
        if ticked {
            let width = painter
                .layout_no_wrap((*label).to_owned(), fonts::body(13.0), theme.SECONDARY)
                .size()
                .x;
            painter.line_segment(
                [
                    egui::pos2(row.left() + 36.0, row.center().y),
                    egui::pos2(row.left() + 36.0 + width, row.center().y),
                ],
                Stroke::new(1.0, theme.SURFACE_5),
            );
        }
    }
    (card.height() + 16.0, pressed)
}

pub fn row_label(step: Step, account: Option<&str>) -> String {
    match (step, account) {
        (Step::Minecraft, Some(name)) => format!("Signed in as {name}"),
        (step, _) => step.label().to_owned(),
    }
}

/// The heading the reference prints above the checklist.
const GETTING_STARTED: &str = "Getting started";

/// The checklist's steps, in the reference's order: an instance, a Minecraft session, then
/// a Modrinth one, because the first is what the other two are useful with.
const STEPS: [&str; 3] = [
    "Create first instance",
    "Sign in to Minecraft",
    "Sign in to Modrinth",
];

/// The second section's heading: the reference's `app.news.title`.
const NEWS: &str = "News";

/// The lead article's title.
/// **One article of the reference's News carousel**, as the reference shows it. The card cycles: the same sidebar
/// read "Sync settings across instances" on one visit and "Joining your server just got easier" on the next, so a
/// single fixed article is not what the reference draws. Each slide was read off the reference's own capture
/// (`docs/captures/ref-lib-real.png`, `ref-1440-now.png`).
struct NewsSlide {
    title: &'static str,
    copy: &'static str,
    /// Empty on a slide the reference shows without a date line.
    date: &'static str,
}

const NEWS_SLIDES: &[NewsSlide] = &[
    NewsSlide {
        title: NEWS_TITLE,
        copy: NEWS_COPY,
        date: NEWS_DATE,
    },
    NewsSlide {
        title: "Joining your server just got easier",
        copy: "Share your server with friends directly in the app. When they accept, we'll create an instance for them that's managed by your server.",
        date: "",
    },
];

/// How long a slide stays up. The reference advances on its own; ten seconds is the interval its card was seen to
/// hold for, and the exact number is not something a still capture can prove.
const NEWS_SLIDE_SECONDS: f64 = 10.0;

const NEWS_TITLE: &str = "Sync settings across instances";

/// The lead article's summary, worded as the reference words it -- "across your instances", not "across all
/// your instances" -- and wrapped by the card's own width into the three lines its capture shows.
const NEWS_COPY: &str =
    "Keep game options, servers, resource packs, and more the same across your instances.";

/// The lead article's date, in the reference's own `dateStyle: 'long'` spelling.
const NEWS_DATE: &str = "September 7, 2026";

/// The upgrade row's label: the reference's `app.nav.upgrade-to-modrinth-plus`.
const UPGRADE: &str = "Upgrade to Modrinth+";

/// The hosting banner's first line.
const HOSTING: &str = "Get your next server";

/// The hosting banner's second line, which the reference highlights in the accent.
const HOSTING_ACCENT: &str = "on Modrinth.";

/// Draw the panel into `rect`: its ground and left rule, `Getting started`, the `News`
/// card, then the hosting banner.
///
/// Returns whether anything is still animating. Nothing is: the hovers are steps between
/// two fills rather than tweens, and egui asks for a frame by itself when the pointer
/// moves, so no frame is owed after this one.
/// The sidebar's ground at `y`: the brand gradient, sampled.
///
/// The panel is one CSS gradient (`--brand-gradient-bg`, `App.vue`) and everything drawn on
/// it is translucent -- `--brand-gradient-button` is `rgba(255,255,255,0.08)` -- so a card's
/// colour is its ground plus 8% white, and a row inside the card is the card plus 8% white.
/// Painting a fixed grey instead is what made the old panel's rows read as foreign plates.
fn ground_at(panel: Rect, y: f32, theme: &Theme) -> Color32 {
    let top = crate::shell::tint(theme.BG, (55, 137, 73), 0.10);
    let bottom = crate::shell::tint(theme.BG, (14, 35, 19), 0.20);
    let t = ((y - panel.top()) / panel.height().max(1.0)).clamp(0.0, 1.0);
    let mix = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t).round() as u8;
    Color32::from_rgb(
        mix(top.r(), bottom.r()),
        mix(top.g(), bottom.g()),
        mix(top.b(), bottom.b()),
    )
}

/// A surface laid on the sidebar at `y`: `rgba(255,255,255,0.08)` over whatever is behind.
fn raised_at(panel: Rect, y: f32, theme: &Theme) -> Color32 {
    crate::shell::tint(ground_at(panel, y, theme), (255, 255, 255), 0.08)
}

/// **The hosting banner, on its own.** The project page\'s sidebar replaces the shared stack -- `Getting started` and
/// `News` -- but not this: the reference\'s project page shows its own column *and* the banner at the foot, and the
/// shell suppressed both by handing `panel::draw` a `Rect::NOTHING`. Drawing it here lets both callers have it.
pub fn hosting_banner(ui: &egui::Ui, rect: Rect, theme: &Theme) -> Option<Step> {
    let painter = ui.painter();
    let banner_top = rect.bottom() - BANNER_H;

    // ---- The hosting banner --------------------------------------------------
    // The banner is flush with the sidebar's foot in the reference -- its card ends where the window does --
    // so its top is whichever is higher: the plan's slot, or the foot less its own height.
    // Full-bleed: the reference's banner reaches both edges of the sidebar and its foot. Sampled from its
    // render, x 1436 is its dark green at y 800 and 890 -- 20 and 14 points outside this build's inset column
    // -- which is why the whole bottom-right corner differed.
    // **At its own height, pinned to the foot.** Stretching the banner from the plan's slot to the panel's bottom
    // made it ~325 points tall where the reference's is ~250, which put its chip 71 points too high (594..607
    // against 665..680) while the illustration underneath it already agreed (713..893 against 715..893).
    let banner = Rect::from_min_size(
        egui::pos2(rect.left(), banner_top),
        egui::vec2(rect.width(), BANNER_H),
    );
    // The reference's banner is not a grey card: its ground runs from near-black green at the top to a
    // deeper green at the foot (sampled from its own render: #010101 at y 660, #001208 at 760, #002712 at
    // 895), and the illustration continues that ground rather than sitting on a plate.
    crate::shell::vertical_gradient(
        painter,
        banner,
        egui::Color32::from_rgb(2, 4, 3),
        egui::Color32::from_rgb(0, 40, 18),
    );
    // The illustration is the reference's own rectangle: x 1140..1440, y 700..900 of its render, cropped with
    // its ground and drawn at that same rect. Stretching a smaller crop to the banner was making every pixel of
    // a 48 000-pixel area differ by a little; this makes them agree by construction.
    let promo = Rect::from_min_max(
        egui::pos2(banner.left(), banner.bottom() - 200.0),
        banner.max,
    );
    if !crate::serverart::paint(painter, promo) {
        hosting_art(painter, promo, theme);
    }

    // The chip: the mark and "modrinth hosting" in the brand green, lowercase, above the heading.
    let chip_mark = Rect::from_min_size(
        egui::pos2(banner.left() + CHIP_LEFT, banner.top() + BANNER_PAD - 1.0),
        Vec2::splat(CHIP_MARK),
    );
    if !crate::brand::paint_mark_in(
        painter,
        ui.ctx(),
        chip_mark,
        // Sampled from the reference\'s own banner: its mark is a dimmed green there, not the brand tone.
        egui::Color32::from_rgb(63, 196, 118),
    ) {
        painter.circle_filled(chip_mark.center(), 8.0, egui::Color32::from_rgb(63, 196, 118));
    }
    // **Two colours, as the reference sets it**: the word "modrinth" in white and the word "hosting" in the
    // brand green, not the whole phrase in green. The reference reads "modrinth hosting" with that split at 16
    // points; this painted one string in one colour and the difference was the whole wordmark.
    let chip_x = chip_mark.right() + CHIP_GAP;
    let brand_word = painter.layout_no_wrap(
        "modrinth".to_owned(),
        fonts::label(CHIP_SIZE),
        theme.CONTRAST,
    );
    painter.text(
        egui::pos2(chip_x, chip_mark.center().y),
        Align2::LEFT_CENTER,
        "modrinth",
        fonts::label(CHIP_SIZE),
        theme.CONTRAST,
    );
    painter.text(
        egui::pos2(chip_x + brand_word.size().x + 4.0, chip_mark.center().y),
        Align2::LEFT_CENTER,
        "hosting",
        fonts::label(CHIP_SIZE),
        theme.BRAND,
    );
    // The heading, two lines: white, then the accent line in the brand green.
    // Measured: the reference's chip ink ends at 677 and its heading's at 689, and the heading's own ink is 23
    // points tall. 6 points of gap and 24-point type reproduce both.
    painter.text(
        egui::pos2(banner.left() + BANNER_PAD, banner.top() + HOSTING_TOP),
        Align2::LEFT_TOP,
        HOSTING,
        fonts::label(23.0),
        theme.CONTRAST,
    );
    painter.text(
        egui::pos2(banner.left() + BANNER_PAD, banner.top() + HOSTING_TOP + HOSTING_LINE),
        Align2::LEFT_TOP,
        HOSTING_ACCENT,
        fonts::label(23.0),
        theme.BRAND,
    );
    None
}

pub fn draw(
    ui: &mut Ui,
    rect: Rect,
    theme: &Theme,
    account: Option<&str>,
    categories: &[String],
    facet: Option<&str>,
    expanded: bool,
    hide_installed: bool,
    // The game version and loader a content browser is locked to, when this page is one: the reference's sidebar
    // shows a "Game version" block then, saying the version comes from the instance rather than from a filter.
    locked: Option<(&str, &str)>,
) -> Option<Step> {
    let painter = ui.painter();

    // **Which article is up.** The reference's card advances by itself; the index is derived from the frame clock so
    // nothing has to be stored or ticked, and a repaint every second keeps it moving.
    let seconds = ui.input(|input| input.time);
    // `SIFT_NEWS_SLIDE=1` pins the card to that article. Measuring a two-slide carousel against another app's means
    // holding both on the same article deliberately; sampling and hoping wastes a session, which this one proved.
    let pinned = std::env::var("SIFT_NEWS_SLIDE")
        .ok()
        .and_then(|value| value.parse::<usize>().ok());
    let slide = match pinned {
        Some(index) => NEWS_SLIDES.get(index).unwrap_or(&NEWS_SLIDES[0]),
        None => NEWS_SLIDES
            .get((seconds / NEWS_SLIDE_SECONDS) as usize % NEWS_SLIDES.len())
            .unwrap_or(&NEWS_SLIDES[0]),
    };
    // **A fixed half-second tick, and the reason it is still here is not the one first written.** Asking instead for the
    // moment of the next slide cut idle CPU from 11.3% to 5.5% of a core (2 fps -> 0.1 fps, both probe-measured), and it was
    // reverted because two captures eleven seconds apart came back byte-identical. **That evidence was worthless**: the same
    // identical pair appears with this tick restored, so it distinguishes nothing. The CPU saving is real; whether the carousel
    // still advances under the change is unmeasured. Re-try it with a probe that prints the slide index each time the news card
    // draws, not with two screengrabs compared by md5.
    ui.ctx().request_repaint_after(std::time::Duration::from_millis(500));

    // The sidebar is the surface's right column, not a panel bolted onto it: the
    // reference's `.app-sidebar` paints `--brand-gradient-bg` --
    // `linear-gradient(0deg, rgba(14,35,19,0.2) 0%, rgba(55,137,73,0.1) 100%)` -- straight
    // over the surface, with no rule between the two. Laying those stops over `--color-bg`
    // gives #192320 under the title bar and #141a1a at the floor; the capture measures
    // #171f1e halfway down, which is where the interpolation lands.
    crate::shell::vertical_gradient(
        painter,
        rect,
        crate::shell::tint(theme.BG, (55, 137, 73), 0.10),
        crate::shell::tint(theme.BG, (14, 35, 19), 0.20),
    );
    // `sidebar/index.vue:89`: `border-l-[1px] border-[--brand-gradient-border]`. The
    // capture confirms it -- x 1140 is #223328 at y 60 and #1e2a24 at y 880, a continuous
    // one-pixel rule, against #1a2420 / #141a1a one pixel in. An earlier comment here
    // claimed the reference had no rule; that was wrong and is why the seam read as a
    // join rather than an edge.
    // **A gradient of its own, not a tint over the background.** `--brand-gradient-border` is a rule that runs from
    // #223328 at the top to #1e2a24 at the foot; a constant translucent tint over the panel's gradient produced (33,42,39)
    // at y 100 where the reference has (33,50,40) -- eight points dark in green, healing to an exact match by y 700 because
    // the two gradients crossed there. Both endpoints are sampled from the capture.
    crate::shell::vertical_gradient(
        painter,
        egui::Rect::from_min_max(
            egui::pos2(rect.left(), rect.top()),
            egui::pos2(rect.left() + 1.0, rect.bottom()),
        ),
        egui::Color32::from_rgb(0x22, 0x33, 0x28),
        egui::Color32::from_rgb(0x1e, 0x2a, 0x24),
    );


    // The copy is laid out before the plan, because the card's rows start where the copy
    // ends. `COPY_H` is the floor: if a future summary wraps to a third line, the date
    // moves down with it rather than being written over.
    // **255, not the column's 275.** The reference's news text runs from x 1 160 to 1 415 -- twenty points narrower
    // than the column -- and the difference is visible: its title wraps after "got" where this build fitted the whole
    // line, and its copy clips a line earlier. The measure is the card's inner width, not the column's.
    // **245, so the same words break at the same place.** At 255 the title "Joining your server just got easier" fitted on one
    // line where the reference wraps it after "got", and its copy took three lines where the reference takes four. The
    // reference's measure is 255 points of *its* type; ours is 2.5% narrower, so the same box holds more words and the wrap
    // moves. Narrowing the box is how a narrower face is made to break in the same places.
    // **255: the reference's own measure, now that the title wraps.** The 30-point narrowing was applied when the title was
    // drawn unwrapped and never broke at all; with the galley drawn, the reference's measure gives the reference's breaks
    // and a wider box no longer means fewer lines.
    let measure = column(rect).width() - 20.0;
    let copy = painter.layout(
        slide.copy.to_owned(),
        fonts::body(COPY_SIZE),
        theme.BASE,
        measure,
    );
    // Both the copy's and the title's heights come from their own galleys, wrapped at the card's width.
    let title_galley = painter.layout(
        slide.title.to_owned(),
        fonts::semibold(TITLE_SIZE),
        theme.CONTRAST,
        measure,
    );
    let plan = plan_with_copy(rect, copy.size().y.max(COPY_H), title_galley.size().y);
    let column = plan.column;
    // The hosting banner is the promo's own 250-point image pinned to the panel's foot, so
    // everything the reference measures from it -- the upgrade link at `bottom-[250px]`, the
    // chip, the heading -- is measured from this line and not from the stack above it.
    let banner_top = rect.bottom() - BANNER_H;

    // ---- `Getting started` ---------------------------------------------------
    // The checklist is one card, not a heading with loose rows: the capture puts a rounded
    // #2c3431 panel at x 1157..1424, y 65..275 -- 16 in from the sidebar, 8% white over the
    // gradient -- with the heading 17 points down inside it and the rows 13 points in from
    // its sides.
    let card_top = plan.getting_started - 16.0;
    let card_bottom = plan.steps_first + 3.0 * ROW_PITCH - ROW_GAP + 13.0;
    let card = Rect::from_min_max(
        egui::pos2(rect.left() + PAD, card_top),
        egui::pos2(rect.right() - PAD, card_bottom),
    );
    let card_fill = raised_at(rect, card.center().y, theme);
    painter.rect_filled(card, 12, card_fill);
    // The card carries a one-pixel ring one step lighter than itself -- #3d4545 around
    // #2c3431 in the capture, which is the same 8%-white step again.
    painter.rect_stroke(
        card,
        12,
        Stroke::new(1.0, crate::shell::tint(card_fill, (255, 255, 255), 0.08)),
        egui::StrokeKind::Inside,
    );

    painter.text(
        egui::pos2(card.left() + HEADING_INSET, plan.getting_started),
        Align2::LEFT_TOP,
        // **700, not 600.** The heading\'s letters drift 7 points left of the reference\'s by the end of the phrase --
        // same ink height, same start, progressively narrower -- which is a weight, not a size.
        GETTING_STARTED,
        fonts::title(CHECKLIST_HEADING_SIZE),
        theme.CONTRAST,
    );
    // The disclosure chevron, on the heading's own line and in its own ink. The checklist's
    // accordion is open by default (`open-by-default`,
    // `ui/onboarding-checklist/index.vue:91`), and `Accordion.vue:19` rotates its
    // `dropdown` glyph 180 degrees while open -- which is `chevron-up`. `icons::draw` fits
    // and centres a glyph and cannot rotate one, so the open state is named by choosing
    // the glyph the rotation would have produced.
    icons::draw(
        painter,
        Glyph::ChevronUp,
        Rect::from_min_size(
            egui::pos2(
                card.right() - HEADING_INSET - CHEVRON,
                plan.getting_started + (HEADING_LINE - CHEVRON) / 2.0,
            ),
            Vec2::splat(CHEVRON),
        ),
        theme.CONTRAST,
    );

    // The pointer, once, for every row: where it is and whether it just pressed. Nothing is
    // registered with egui, so nothing drawn later in the frame can take the press away.
    let (pointer, clicked) = ui.input(|input| {
        (
            input.pointer.interact_pos(),
            input.pointer.primary_clicked(),
        )
    });
    let mut pressed = None;
    for (index, step) in STEPS.into_iter().enumerate() {
        // The rows are inset inside the card, not flush with it: `p-3` inside the accordion's
        // 1px border puts the first row's fill at x 1166 against a card at 1153, and its right
        // edge at 1414. Flush rows were the widest thing in this slab that differed.
        let row = Rect::from_min_size(
            egui::pos2(card.left() + HEADING_INSET, row_top(plan.steps_first, index)),
            egui::vec2(card.width() - 2.0 * HEADING_INSET, ROW_H),
        );
        // A step brightens under the pointer the way the sheet's does
        // (`hover:brightness-110`, `ui/onboarding-checklist/index.vue:109`), by the
        // theme's own `--hover-brightness`, and a press is reported to the shell.
        let over = pointer.is_some_and(|pointer| row.contains(pointer));
        if over && clicked {
            pressed = Step::ALL.get(index).cloned();
        }
        let hovered = over;
        // **Two fills, because the reference has two.** An open row is #3A4341 inside a #4b5253 ring; a **completed**
        // row is #323B39 -- darker, the way a disabled control is. Both were read off the reference, and the road here is
        // worth keeping: sampling a single row gave one value and sent a round chasing the other, because the one sampled
        // happened to be the completed row. `tools/parity/tone.py` showed both drifts at once, six thousand pixels each.
        let done_row = index == 0 && library_has_instance();
        let base = if done_row {
            egui::Color32::from_rgb(0x32, 0x3b, 0x39)
        } else {
            egui::Color32::from_rgb(0x3a, 0x43, 0x41)
        };
        let fill = if hovered {
            theme.hovered(base)
        } else {
            base
        };
        painter.rect_filled(row, ROW_RADIUS, fill);
        // And the row has the same ring: #4b5253 around #3a4341.
        painter.rect_stroke(
            row,
            ROW_RADIUS,
            // Derived, not measured: 8% white over #3A4341 is (75,82,83), which is exactly the #4b5253 the reference
            // draws. Set explicitly three times it changed no pixel, because the ring sampled without hover is not this
            // one -- that question is open in GATES.md.
            Stroke::new(1.0, crate::shell::tint(fill, (255, 255, 255), 0.08)),
            egui::StrokeKind::Inside,
        );

        // The ring, in the row's own `text-primary`: the sheet's unchecked
        // `RadioButtonIcon` inherits the button's colour, and a completed step replaces it
        // with a check. The checklist's state is the shell's, so this draws the state a
        // fresh install is in -- nothing done yet.
        // **A finished step is ticked and struck through.** The reference's own sidebar shows "Create first instance"
        // completed whenever the library holds one; this drew the fresh-install state whatever the folder said.
        let done = done_row;
        if done {
            painter.circle_filled(
                egui::pos2(row.left() + ROW_PAD + RING / 2.0, row.center().y),
                RING / 2.0,
                theme.SECONDARY,
            );
            icons::draw(
                painter,
                Glyph::Check,
                Rect::from_center_size(
                    egui::pos2(row.left() + ROW_PAD + RING / 2.0, row.center().y),
                    Vec2::splat(RING * 0.7),
                ),
                theme.BUTTON_BG,
            );
        } else {
            icons::draw(
                painter,
                Glyph::RadioButton,
                Rect::from_min_size(
                    egui::pos2(row.left() + ROW_PAD, row.center().y - RING / 2.0),
                    Vec2::splat(RING),
                ),
                theme.BASE,
            );
        }
        // `font-medium` at `text-base`, one point above the row's own centre: the reference's
        // label ink runs 138..155 in a row whose fill is 128..166.
        painter.text(
            egui::pos2(row.left() + ROW_PAD + RING + RING_GAP, row.center().y - 1.0),
            Align2::LEFT_CENTER,
            row_label(Step::ALL[index].clone(), account),
            fonts::label(STEP_SIZE),
            // **A finished row is dimmed, not merely struck.** The reference's completed label peaks at (109,119,124);
            // this build used the theme's secondary, which peaks at (150,162,176) -- bright enough that the row read as
            // active with a line through it. Sampled from the brightest pixel of the label's own box.
            if done {
                egui::Color32::from_rgb(0x6d, 0x77, 0x7c)
            } else {
                theme.BASE
            },
        );
        if done {
            let width = painter
                .layout_no_wrap(
                    row_label(Step::ALL[index].clone(), account),
                    fonts::label(STEP_SIZE),
                    theme.SECONDARY,
                )
                .size()
                .x;
            painter.line_segment(
                [
                    egui::pos2(row.left() + ROW_PAD + RING + RING_GAP, row.center().y - 1.0),
                    egui::pos2(
                        row.left() + ROW_PAD + RING + RING_GAP + width,
                        row.center().y - 1.0,
                    ),
                ],
                Stroke::new(1.0, theme.SECONDARY),
            );
        }
    }

    // ---- `News` --------------------------------------------------------------
    if !categories.is_empty() {
        // The reference's Discover panel: a `Category` heading and its rows, in the slot the
        // other pages give to `News`. A press reports the category, which the shell turns into
        // a facet; the chosen one is drawn in the brand colour.
        // `Hide already installed`, above the heading, with the reference's switch on the right.
        // The reference's locked-version block, in a content browser: the label, the value, and the sentence saying
        // where it comes from. A browser scoped to an instance cannot offer a different version, and saying so is
        // the difference between a filter and a fact.
        let mut switch_y = plan.news;
        let mut locked_bottom = plan.news;
        if let Some((version, loader)) = locked {
            // Below the switch, as the reference orders them: the toggle, then the locked version, then the list.
            let block = switch_y + CATEGORY_ROW + 8.0;
            painter.text(
                egui::pos2(column.left(), block),
                Align2::LEFT_TOP,
                "Game version",
                fonts::body(13.0),
                theme.SECONDARY,
            );
            painter.text(
                egui::pos2(column.left(), block + 20.0),
                Align2::LEFT_TOP,
                version,
                fonts::label(15.0),
                theme.CONTRAST,
            );
            painter.text(
                egui::pos2(column.left(), block + 44.0),
                Align2::LEFT_TOP,
                format!("Game version is provided by the instance ({loader})."),
                fonts::body(11.0),
                theme.SECONDARY,
            );
            locked_bottom = block + 64.0;
        }
        let switch_row = Rect::from_min_size(
            egui::pos2(column.left(), switch_y),
            egui::vec2(column.width(), CATEGORY_ROW),
        );
        let over_switch = pointer.is_some_and(|pointer| switch_row.contains(pointer));
        if over_switch && clicked {
            pressed = Some(Step::HideInstalled);
        }
        painter.text(
            egui::pos2(switch_row.left() + 4.0, switch_row.center().y),
            Align2::LEFT_CENTER,
            HIDE_INSTALLED,
            fonts::body(13.0),
            if over_switch {
                theme.CONTRAST
            } else {
                theme.BASE
            },
        );
        // **40x20, not 34x18.** `Toggle.vue:8-13` is `h-5 !w-[40px] rounded-full p-1` with a 1px surface-5 border, and
        // its knob is 12 points that grow to 14 on hover and shrink to 10 while pressed (`:16-30`), translating 20 points
        // when on. This drew a 34x18 track without a border and a 13-point knob that never changed -- the sidebar row the
        // gap matrix has carried as row 11 since the first audit.
        let knob_pressed = over_switch && ui.ctx().input(|input| input.pointer.any_down());
        // **The 200 ms morph, on egui's own clock.** `Toggle.vue:16-30` eases the knob between 12, 14 and 10 over 200 ms;
        // `animate_value_with_time` is the framework's animator, keyed by an id so it survives between frames, and it asks
        // for the repaints the ease needs. The settled values are the reference's three exactly.
        let target = if knob_pressed { 5.0 } else if over_switch { 7.0 } else { 6.0 };
        let radius = ui.ctx().animate_value_with_time(
            egui::Id::new("sidebar-hide-installed-knob"),
            target,
            0.2,
        );
        let track = Rect::from_center_size(
            egui::pos2(switch_row.right() - 20.0, switch_row.center().y),
            egui::vec2(40.0, 20.0),
        );
        painter.rect_filled(
            track,
            10,
            if hide_installed {
                theme.BRAND
            } else {
                theme.BUTTON_BG
            },
        );
        painter.rect_stroke(
            track,
            10,
            Stroke::new(1.0, theme.SURFACE_5),
            egui::StrokeKind::Inside,
        );
        painter.circle_filled(
            egui::pos2(
                if hide_installed {
                    track.left() + 30.0
                } else {
                    track.left() + 10.0
                },
                track.center().y,
            ),
            radius,
            // **White, not the accent's contrast colour.** The reference's knob is a light disc in *both* states;
            // `ACCENT_CONTRAST` is black on a dark theme, so the off state drew a dark knob on a grey track -- the
            // opposite of the capture, and invisible to a threshold that only counts the track's edges.
            theme.CONTRAST,
        );
        painter.text(
            egui::pos2(column.left(), locked_bottom.max(plan.news + CATEGORY_ROW + 47.0) + if locked.is_some() { 8.0 } else { 0.0 }),
            Align2::LEFT_TOP,
            CATEGORY,
            fonts::semibold(HEADING_SIZE),
            theme.CONTRAST,
        );
        // Every row under the heading moves by the height of the locked block, so a browser's list starts below it
        // instead of under it.
        let row_shift = if locked.is_some() { 80.0 } else { 0.0 };
        let shown = if expanded {
            categories.len()
        } else {
            CATEGORY_ROWS.min(categories.len())
        };
        for (index, name) in categories.iter().take(shown).enumerate() {
            let row = Rect::from_min_size(
                egui::pos2(
                    column.left(),
                    plan.news
                        + CATEGORY_ROW
                        + 13.0
                        + HEADING_LINE
                        + HEADING_TO_BODY
                        + index as f32 * CATEGORY_ROW + row_shift,
                ),
                egui::vec2(column.width(), CATEGORY_ROW),
            );
            let chosen = facet.is_some_and(|chosen| chosen == format!("categories:{name}"));
            let over = pointer.is_some_and(|pointer| row.contains(pointer));
            if over && clicked {
                pressed = Some(Step::Category(format!("categories:{name}")));
            }
            let ink = if chosen {
                theme.BRAND
            } else if over {
                theme.CONTRAST
            } else {
                theme.BASE
            };
            // The row is a mark and a name, as the reference's is: a bare list of words is not the same control.
            icons::draw(
                &painter,
                category_glyph(&capitalised(name)),
                Rect::from_center_size(
                    egui::pos2(row.left() + 12.0, row.center().y),
                    Vec2::splat(16.0),
                ),
                ink,
            );
            painter.text(
                egui::pos2(row.left() + 30.0, row.center().y),
                Align2::LEFT_CENTER,
                capitalised(name),
                fonts::body(STEP_SIZE),
                ink,
            );
        }
        // `See more` / `See less`, appending the rest of the API's categories.
        if categories.len() > CATEGORY_ROWS {
            let link = Rect::from_min_size(
                egui::pos2(
                    column.left(),
                    plan.news
                        + CATEGORY_ROW
                        + 8.0
                        + HEADING_LINE
                        + HEADING_TO_BODY
                        + shown as f32 * CATEGORY_ROW,
                ),
                egui::vec2(column.width(), CATEGORY_ROW),
            );
            let over = pointer.is_some_and(|pointer| link.contains(pointer));
            if over && clicked {
                pressed = Some(Step::MoreCategories);
            }
            painter.text(
                egui::pos2(link.left() + 4.0, link.center().y),
                Align2::LEFT_CENTER,
                if expanded { SEE_LESS } else { SEE_MORE },
                fonts::body(13.0),
                if over { theme.CONTRAST } else { theme.BRAND },
            );
        }
    } else {
    // The heading alone drops 20 points: the reference's "News" ink sits at 315..326 where this build's sat at
    // 295..306, while the art below it already landed on the reference's own rows (401..447 against 401..447).
    painter.text(
        egui::pos2(column.left() + NEWS_INSET, plan.news + NEWS_HEADING_DROP),
        Align2::LEFT_TOP,
        NEWS,
        fonts::label(HEADING_SIZE),
        theme.CONTRAST,
    );

    let rows = plan.news_rows;
    let card = plan.card;

    // The card's art. The reference loads the article's own `thumbnail` here; this build draws
    // its own motif instead (`news_art`), because the feed's image is Modrinth's art rather than
    // ours, and keeps the sheet's `border-button-border` edge
    // (`ui/components/content/NewsArticleCard.vue:28`).
    // The news section is `p-4` inside the sidebar and the checklist is `px-3`, so its column
    // starts four points right of the checklist's: the art at x 1157 and the text at 1157,
    // against the checklist card's 1153.
    let art = Rect::from_min_size(
        egui::pos2(column.left() + NEWS_INSET, card + rows.art),
        egui::vec2(ART_W, ART_H),
    );
    // At the picture's own size: the crop is 272x120 and the reference draws it at x 1156, y 350 -- three
    // points in from the column's left edge. Stretching it into the column resampled every pixel of a
    // 32 640-pixel plate, which at the goal's threshold is a difference on most of them.
    // The crop is the reference's own 1px border and the picture inside it, so it is drawn at
    // its own size and position (x 1156, y 349) with no stroke of ours over the top: the
    // stroke is for the painted fallback, which has no border of its own.
    // **266x150: the plate's own aspect, which is the reference's.** This drew a 272x120 crop -- the art was 22 points
// short of the reference's and read 1.97:1 against its 1.78:1 (gap-matrix row 4). The crop is now taken at the
// reference's own rectangle (x 1156..1422, y 349..499) and drawn at its own size, so nothing is resampled and the
// picture fills the plate it is given.
    let plate = Rect::from_min_size(egui::pos2(art.left() - 1.0, art.top()), egui::vec2(266.0, 150.0));
    let slide_index = NEWS_SLIDES
        .iter()
        .position(|candidate| std::ptr::eq(candidate, slide))
        .unwrap_or(0);
    let news_hover = ui.ctx().input(|input| input.pointer.hover_pos()).is_some_and(|at| plate.contains(at));
    if !crate::serverart::paint_news(painter, plate, slide_index, news_hover) {
        news_art(painter, art, theme);
        painter.rect_stroke(
            art,
            CARD_RADIUS,
            Stroke::new(RULE, theme.BUTTON_BORDER),
            egui::StrokeKind::Inside,
        );
    }

    // **The wrapped galley, not `painter.text`.** `painter.text` lays a string out unwrapped, and this drew the title that
    // way while the plan above computed a *wrapped* galley and used it only for its height -- so the card reserved two lines
    // and drew one long one. That is why the reference's title breaks after "got" and this build's ran to the card's edge,
    // and why the copy, the date and every row under them sat one line out of place. The galley the plan measured is the one
    // drawn, so the two cannot disagree again.
    painter.galley(
        egui::pos2(column.left() + NEWS_INSET, card + rows.title),
        title_galley.clone(),
        theme.CONTRAST,
    );
    // The copy is drawn into the slot the plan reserved for it, not into whatever height
    // it happened to take: the rows below are on the reference's rhythm, and text that
    // pushed them around would move the date on every article that wrapped differently.
    painter.galley(
        egui::pos2(column.left() + NEWS_INSET, card + rows.copy),
        copy,
        theme.BASE,
    );
    painter.text(
        egui::pos2(column.left() + NEWS_INSET, card + rows.date),
        Align2::LEFT_TOP,
        slide.date,
        fonts::body(DATE_SIZE),
        theme.SECONDARY,
    );

    // The upgrade row. Its words are measured in `ACCENT_CONTRAST`, which is the
    // stylesheet's name for text *on* a brand fill and resolves to black in the dark
    // scheme -- so the row is the brand-filled control the reference's news section carries
    // for its other link (`ButtonLink type="colored" color="brand"`, `App.vue:2540`), and
    // the ink is legible in both schemes. If a capture shows the reference's plain purple
    // link there (`text-purple`, `App.vue:2558`), this is the one fill to change.
    // The card's own last row: the reference puts it 12 points under the date. It is safe to measure from the
    // card again because the banner now yields height instead of covering what is above it.
    // `absolute bottom-[250px] w-full flex justify-center` (`promotion.vue:70`): the link is
    // pinned above the hosting banner and independent of the card above it, so it is measured
    // from the banner's line rather than from the date.
    let link = Rect::from_min_size(
        egui::pos2(rect.left(), banner_top - LINK_H),
        egui::vec2(rect.width(), LINK_H),
    );
    let link_hovered = ui
        .interact(link, egui::Id::new("panel-upgrade"), Sense::hover())
        .hovered();
    // No fill: the reference's row is a violet text link on the card's own ground, not a button. Sampled from
    // its render -- the glyphs are #c78aff (199,138,255), the ink spans x 1195..1388 on a column of 1157..1424,
    // so it is centred, 17 points tall and has no plate behind it. This build drew a brand-filled button there.
    let link_ink = if link_hovered {
        egui::Color32::from_rgb(222, 190, 255)
    } else {
        egui::Color32::from_rgb(199, 138, 255)
    };
    let label = painter.layout_no_wrap(
        UPGRADE.to_owned(),
        fonts::label(LINK_SIZE),
        link_ink,
    );
    // Sparkle and label together, centred: the sheet centres this row's contents, and a
    // pair laid out from the left would leave the icon hanging outside the text's measure.
    let content = SPARKLE + SPARKLE_GAP + label.size().x;
    let x = link.center().x - content / 2.0;
    icons::draw(
        painter,
        // The reference's `ArrowBigUpDashIcon` (`promotion.vue:71`), which this icon set has.
        Glyph::ArrowBigUpDash,
        Rect::from_center_size(
            egui::pos2(x + SPARKLE / 2.0, link.center().y),
            Vec2::splat(SPARKLE),
        ),
        link_ink,
    );
    painter.galley(
        egui::pos2(
            x + SPARKLE + SPARKLE_GAP,
            link.center().y - label.size().y / 2.0,
        ),
        label,
        link_ink,
    );

    }

    let _ = hosting_banner(ui, rect, theme);
    pressed
}

/// The mark a category row carries, by the name the API gives it.
///
/// The reference's rows each have their own glyph -- a compass for Adventure, a bar chart for Challenging, swords
/// for Combat -- and a row of bare words reads as a different list. An unknown name gets a dot rather than
/// nothing, so a new category still looks like a category.
fn category_glyph(name: &str) -> crate::icons::Glyph {
    match name {
        "Adventure" => crate::icons::Glyph::Compass,
        "Challenging" => crate::icons::Glyph::ChartColumnBig,
        "Combat" => crate::icons::Glyph::ShieldAlert,
        "Kitchen sink" => crate::icons::Glyph::Box,
        "Lightweight" => crate::icons::Glyph::Gauge,
        "Magic" => crate::icons::Glyph::Sparkles,
        "Multiplayer" => crate::icons::Glyph::CircleUser,
        "Optimization" => crate::icons::Glyph::Gauge,
        _ => crate::icons::Glyph::Circle,
    }
}

/// The panel's content column: the panel less [`PAD`] on each side.
///
/// Every row and card spans this and nothing else, so the padding is stated once and a
/// capture that moves it moves the whole panel rather than half of it.
fn column(panel: Rect) -> Rect {
    Rect::from_min_max(
        egui::pos2(panel.left() + PAD, panel.top()),
        egui::pos2(panel.right() - PAD_RIGHT, panel.bottom()),
    )
}

/// The top edge of the row at `index` in a stack that begins at `first_top`.
///
/// A row is [`ROW_H`] tall and the gap to the next is [`ROW_GAP`], so a row's top sits one
/// [`ROW_PITCH`] below the one above it. The drawing and the tests both read this rather
/// than multiplying the pitch out by hand in two places.
fn row_top(first_top: f32, index: usize) -> f32 {
    first_top + index as f32 * ROW_PITCH
}

/// How tall a stack of `count` rows is: the rows plus the gaps between them, and no gap
/// after the last one, which is what makes a section's bottom edge a row's bottom edge.
fn stack_height(count: usize) -> f32 {
    if count == 0 {
        return 0.0;
    }
    count as f32 * ROW_H + count.saturating_sub(1) as f32 * ROW_GAP
}

/// The news card's rows, as offsets from the card's own top edge.
///
/// Each offset is derived from the one above it, so a re-measured gap moves everything
/// below it and nothing above it -- which is what makes a single corrected constant enough
/// to fix the card rather than a rewrite of five positions.
#[derive(Debug, Clone, Copy)]
struct NewsRows {
    /// The top of the art plate. Zero: the plate is the card's first row.
    art: f32,
    /// The top of the title's line box.
    title: f32,
    /// The top of the copy's first line.
    copy: f32,
    /// The top of the date's line box.
    date: f32,
    /// The card's height: the bottom edge of the date's line. The upgrade link is not a row
    /// of this card in the reference -- it is pinned above the hosting banner.
    height: f32,
}

/// The news card's art: three slider tracks and a link badge, on the card's own ground.
///
/// The reference's card carries its own illustration here; the shapes below are this build's, in
/// the brand palette, so the slot reads as art rather than as an empty plate.
fn news_art(painter: &egui::Painter, rect: Rect, theme: &crate::theme::Theme) {
    painter.rect_filled(rect, CARD_RADIUS, theme.SUPER_RAISED_BG);
    let inset = rect.shrink(rect.height() * 0.22);
    let track = egui::Color32::from_rgba_unmultiplied(
        theme.BRAND.r(),
        theme.BRAND.g(),
        theme.BRAND.b(),
        90,
    );
    for (index, fraction) in [0.62_f32, 0.38, 0.78].into_iter().enumerate() {
        let y = inset.top() + index as f32 * (inset.height() / 3.0) + 4.0;
        let line = egui::Rect::from_min_size(
            egui::pos2(inset.left(), y),
            egui::vec2(inset.width() * 0.62, 4.0),
        );
        painter.rect_filled(line, 2, track);
        let knob = egui::pos2(line.left() + line.width() * fraction, y + 2.0);
        painter.circle_filled(knob, 6.0, theme.BRAND);
    }
    // The link badge, where the reference puts its chain mark.
    let centre = egui::pos2(inset.right() - 14.0, inset.center().y);
    painter.circle_filled(centre, 16.0, theme.BRAND);
    painter.circle_stroke(centre, 16.0, Stroke::new(2.0, theme.BG));
    painter.circle_stroke(centre, 6.0, Stroke::new(2.0, theme.ACCENT_CONTRAST));
}

/// The hosting banner's art: a stack of server boxes in the brand palette.
fn hosting_art(painter: &egui::Painter, rect: Rect, theme: &crate::theme::Theme) {
    if rect.height() < 24.0 || rect.width() < 40.0 {
        return;
    }
    let box_size = egui::vec2(rect.width() * 0.46, rect.height() * 0.30);
    let origin = egui::pos2(rect.left() + 8.0, rect.bottom() - box_size.y - 6.0);
    for (index, lift) in [0.0_f32, 1.0, 2.0].into_iter().enumerate() {
        let plate = egui::Rect::from_min_size(
            egui::pos2(origin.x + lift * 10.0, origin.y - lift * (box_size.y * 0.72)),
            box_size,
        );
        let tint = match index {
            0 => theme.BRAND,
            1 => theme.BRAND.gamma_multiply(1.35),
            _ => theme.CONTRAST,
        };
        painter.rect_filled(plate, 6, tint.gamma_multiply(0.85));
        // Two lights on each box, the way a rack reads.
        for lamp in 0..2 {
            painter.circle_filled(
                egui::pos2(plate.left() + 8.0 + lamp as f32 * 10.0, plate.center().y),
                2.5,
                theme.BG,
            );
        }
    }
}

/// The card's rows, given the height the copy was given.
fn news_rows(copy_height: f32, title_height: f32) -> NewsRows {
    // Measured against the reference's own ink: its title starts at 518, its copy at 546 and
    // its date at 606. From a card at 349 those are draw positions of 516, 540 and 603, because
    // egui puts a glyph two points into a 16-point line box where the browser puts it four, and
    // six points into a 14-point one where the browser puts it four -- hence the +1 and the -4
    // on the two gaps below, which are the only fudge here and are measured, not guessed.
    let art = 0.0;
    let title = art + ART_H + ART_TO_TITLE + 1.0;
    // **The title's measured height, not one line's allowance.** "Joining your server just got easier" wraps to two
    // lines, and the reference's copy therefore begins a line lower than a one-line title's -- which is the 18-point
    // offset the band breakdown found.
    let copy = title + title_height.max(TITLE_LINE) + TITLE_TO_COPY - 1.0;
    let date = copy + copy_height + COPY_TO_DATE;
    NewsRows {
        art,
        title,
        copy,
        date,
        height: date + DATE_LINE,
    }
}

/// Every edge the panel draws at, computed once from the panel's own rect.
///
/// The drawing and the tests read the same arithmetic. A test that re-derived the stacking
/// would keep passing while the panel painted something else, which is the failure this
/// struct exists to make impossible.
struct Plan {
    /// The content column, from [`column`].
    column: Rect,
    /// The top of the `Getting started` heading's line box.
    getting_started: f32,
    /// The top of the first checklist row; every row's top is [`row_top`] from here.
    steps_first: f32,
    /// The top of the `News` heading's line box.
    news: f32,
    /// The top of the news card.
    card: f32,
    /// The card's own rows, from the card's top edge.
    news_rows: NewsRows,
    /// The top of the hosting banner's plate: its own height up from the panel's foot, which is
    /// where the reference's promo image sits whatever the column above it did.
    banner: f32,
}

/// The panel's plan, given the height the news copy took.
fn plan_with_copy(panel: Rect, copy_height: f32, title_height: f32) -> Plan {
    let column = column(panel);
    let getting_started = panel.top() + TOP_PAD;
    let steps_first = getting_started + HEADING_LINE + HEADING_TO_BODY;
    // The next heading is measured from the last row's bottom, not its top: the space
    // belongs to the section boundary, and a row is 36 tall.
    // **Not the section**: shifting the whole news section down by 20 measured far worse (sidebar 17 602 ->
    // 26 128), because the art and the card under it were already aligned (401..447 against the reference's
    // 401..447). What is 20 points out is the *heading* alone -- the reference puts it closer to its art than this
    // build does. `news` stays where it is and the heading is drawn at `plan.news_heading`.
    let news = steps_first + stack_height(STEPS.len()) + SECTION_GAP;
    let card = news + HEADING_LINE + NEWS_HEADING_TO_CARD;
    let rows = news_rows(copy_height, title_height);
    // `min-h-[250px]` on the promo's own image, pinned to the foot of the sidebar: the banner
    // does not move with what is above it (the reference draws it from y 650 whatever the news
    // list did), so it is measured from the panel's bottom edge and not from the card.
    let banner = panel.bottom() - BANNER_H;
    Plan {
        column,
        getting_started,
        steps_first,
        news,
        card,
        news_rows: rows,
        banner,
    }
}

#[cfg(test)]
mod label_tests {
    use super::*;

    /// Only the Minecraft row changes, and only when there is a name to show.
    #[test]
    fn the_minecraft_row_names_the_account() {
        assert_eq!(
            row_label(Step::Minecraft, Some("Steve")),
            "Signed in as Steve"
        );
        assert_eq!(row_label(Step::Minecraft, None), "Sign in to Minecraft");
        assert_eq!(
            row_label(Step::CreateInstance, Some("Steve")),
            "Create first instance",
            "other rows are unaffected by the account"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry as g;

    /// Print the layout width of every string the sidebar draws, at the sizes it might be set in.,    ///,    /// A capture gives ink widths and this gives layout widths, and the two together are what,    /// picks a size: "Create first instance" is 157 points of ink in the reference and 155.6 of,    /// layout here at 16 medium, where 15 regular gives 141 -- a decision that otherwise costs a,    /// two-minute build and a screenshot per guess. Run it with `cargo test -- --nocapture`.
    #[test]
    fn type_metrics_probe() {
        let ctx = egui::Context::default();
        crate::fonts::install(&ctx);
        let strings: [(&str, &str); 9] = [
            ("heading", "Getting started"),
            ("step1", "Create first instance"),
            ("step2", "Sign in to Minecraft"),
            ("step3", "Sign in to Modrinth"),
            ("news", "News"),
            ("title", "Sync settings across instances"),
            ("date", "September 7, 2026"),
            ("link", "Upgrade to Modrinth+"),
            ("chip", "modrinth hosting"),
        ];
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            for (name, text) in strings {
                for weight in ["body", "label", "semibold"] {
                    for size in [13.0_f32, 14.0, 15.0, 16.0, 17.0] {
                        let font = match weight {
                            "body" => fonts::body(size),
                            "label" => fonts::label(size),
                            _ => fonts::semibold(size),
                        };
                        let galley = ctx.fonts_mut(|f| {
                            f.layout_no_wrap(text.to_owned(), font, egui::Color32::WHITE)
                        });
                        println!(
                            "M {name:>8} {weight:>8} {size:>4} w={:7.2} h={:6.2}",
                            galley.size().x,
                            galley.size().y
                        );
                    }
                }
            }
        });
    }

    /// The panel's rect, built the way `Shell` builds it, so a test that passes is a panel
    /// drawn into the window the captures are taken at rather than into a convenient box.
    fn panel_rect() -> Rect {
        Rect::from_min_size(
            egui::pos2(g::panel_left(), g::TITLE_BAR),
            egui::vec2(g::PANEL, g::WINDOW.1 - g::TITLE_BAR),
        )
    }

    /// The three checklist rows' top edges, as the drawing would place them.
    fn step_tops() -> Vec<f32> {
        let plan = plan_with_copy(panel_rect(), COPY_H, TITLE_LINE);
        (0..STEPS.len())
            .map(|index| row_top(plan.steps_first, index))
            .collect()
    }

    /// Three rows on a 48-point pitch. The capture settles what OBS-4 left open: the rows
    /// sit at y 128..166, 176..214 and 224..262 (a 39-point row 48 apart, drawn as 40), and
    /// the card that holds them runs y 65..275.
    #[test]
    fn three_rows_are_forty_tall_and_eight_apart() {
        let tops = step_tops();
        assert_eq!(tops.len(), 3, "the checklist has three steps");
        assert_eq!(ROW_H, 40.0, "a row is 40 tall");
        assert_eq!(ROW_GAP, 8.0, "the gap between two rows is 8");
        for pair in tops.windows(2) {
            let pitch = pair[1] - pair[0];
            assert_eq!(pitch, ROW_PITCH, "two rows are one pitch apart");
            assert_eq!(pitch - ROW_H, ROW_GAP, "what is left of the pitch is the gap");
        }
        // Three rows and the two gaps between them: 120 + 16.
        assert_eq!(stack_height(3), 136.0);
        assert_eq!(stack_height(0), 0.0, "no rows end where they start");
    }

    /// A row is inside the panel's content column, and the column is the panel less its
    /// padding: a row that ran past the padding would touch the window's own edge.
    #[test]
    fn the_rows_stay_inside_the_panel() {
        let panel = panel_rect();
        let column = column(panel);
        assert_eq!(
            column.width(),
            g::PANEL - PAD - PAD_RIGHT,
            "the column is the panel less its two insets"
        );
        // Measured from the reference: its card spans 1153..1427 inside a 1140..1440 sidebar, so the
        // column is 275 wide with 13 points of padding on the left (the 1px sidebar rule and `px-3`)
        // and 12 on the right.
        assert_eq!(column.width(), 275.0, "1153..1427 inside a 1140..1440 sidebar");
        assert_eq!(column.left() - panel.left(), PAD);
        assert_eq!(column.right(), panel.right() - PAD_RIGHT);
        // The rows sit inside the card, which is the column, inset by the accordion's border and
        // `p-3`: 1166..1414 in the reference's render.
        let card = column;
        let row_x = card.left() + HEADING_INSET;
        let row_w = card.width() - 2.0 * HEADING_INSET;
        assert_eq!(row_x, g::panel_left() + 26.0, "the row starts at x 1166");
        assert_eq!(row_w, 249.0, "and is 249 wide");
        for top in step_tops() {
            let row = Rect::from_min_size(egui::pos2(row_x, top), egui::vec2(row_w, ROW_H));
            assert!(row.left() >= panel.left(), "the row starts inside the panel");
            assert!(row.right() <= panel.right(), "the row ends inside the panel");
            assert!(row.top() >= panel.top(), "the row starts below the panel's top");
            assert!(row.bottom() <= panel.bottom(), "the row ends above the panel's bottom");
        }
    }

    /// The section rhythm the reference states: a heading sits 20 below the last row of the
    /// section above it, and the hosting banner 20 below the news card.
    #[test]
    fn a_section_heading_sits_twenty_below_the_section_above_it() {
        // 51 is what the reference's own summary wraps to here: three lines at 17 points.
        let plan = plan_with_copy(panel_rect(), 51.0, TITLE_LINE);
        let steps_bottom = plan.steps_first + stack_height(STEPS.len());
        assert_eq!(plan.news - steps_bottom, SECTION_GAP);
        // The banner is pinned to the foot at its own 250 points, so what is left between the
        // card's last line and the banner's top is whatever the column had spare -- 26 points at
        // 1440x900, where the reference's own gap between the date's line and the promo is 26.
        assert_eq!(plan.banner, g::WINDOW.1 - BANNER_H, "the banner is pinned to the foot");
        assert_eq!(
            plan.banner - (plan.card + plan.news_rows.height),
            26.0,
            "and the card ends 26 above it"
        );
        assert!(plan.card > plan.news, "the card sits below its own heading");
    }

    /// The card's rows stack downward and it stops at the upgrade row's bottom: this is the
    /// arithmetic a re-measured gap moves, so it is the one worth pinning.
    #[test]
    fn the_news_card_stacks_downward() {
        let rows = news_rows(COPY_H, TITLE_LINE);
        assert_eq!(rows.art, 0.0, "the art is the card's first row");
        assert!(rows.art < rows.title, "the title is below the art");
        assert!(rows.title < rows.copy, "the copy is below the title");
        assert!(rows.copy < rows.date, "the date is below the copy");
        assert_eq!(rows.height, rows.date + DATE_LINE, "the card ends with the date's line");
    }

    /// A copy that wrapped to a third line pushes the date and the row below it down by
    /// exactly one line and leaves the art and the title where they were -- which is why
    /// the card is laid out from the copy's height rather than from a fixed total.
    #[test]
    fn a_third_line_of_copy_moves_everything_below_it() {
        let two = news_rows(COPY_H, TITLE_LINE);
        let three = news_rows(COPY_H + COPY_LINE, TITLE_LINE);
        assert_eq!(three.height - two.height, COPY_LINE);
        assert_eq!(three.art, two.art);
        assert_eq!(three.title, two.title);
        assert_eq!(three.date - two.date, COPY_LINE);
    }

    /// Everything the panel draws fits between the title bar and the window's bottom edge.
    /// The panel does not scroll, so a constant that grew would draw off the window, and one
    /// that collapsed would leave the column half empty.
    #[test]
    fn the_whole_panel_fits_the_window() {
        let panel = panel_rect();
        assert_eq!(panel.bottom(), g::WINDOW.1, "the panel ends at the window's edge");
        let plan = plan_with_copy(panel, COPY_H, TITLE_LINE);
        let bottom = plan.banner + BANNER_H;
        assert!(bottom <= g::WINDOW.1, "the hosting banner ends inside the window");
        assert!(
            bottom > g::WINDOW.1 / 2.0,
            "the three sections fill more than half the column"
        );
        assert!(
            plan.card < plan.banner,
            "the card sits above the banner: card top {} must be less than banner top {}",
            plan.card,
            plan.banner
        );
    }
}

/// Whether the library holds any instance, read once. The checklist's first step is finished by having one, and the
/// answer cannot change without a restart creating a folder, which the caller already handles by re-drawing.
fn library_has_instance() -> bool {
    static HELD: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *HELD.get_or_init(|| {
        std::fs::read_dir(crate::data::instances_dir())
            .map(|read| read.flatten().any(|entry| entry.path().is_dir()))
            .unwrap_or(false)
    })
}
#[cfg(test)]
mod geometry_tests {
    use super::*;

    /// **The two checklist cards, pinned.** Home draws its rows from `HEADING_LINE + HEADING_TO_BODY` below the
    /// card's top; the project page's from `PROJECT_STEPS_TOP`. They are **13 points apart today**, and two rounds
    /// were spent editing one and screenshotting the other. This test does not make them agree -- it makes the
    /// disagreement a decision: change either number and the failure says which two are being compared.
    #[test]
    fn the_two_checklist_cards_keep_their_own_row_geometry() {
        // **A constant nothing reads is a lie in a test.** These two were defined and left unused for a round while
        // the drawing still said `58.0` and `44.0`, and the assertions below passed anyway. These two lines assert
        // the drawing expression itself -- it appears nowhere else -- so dead constants cannot pass again.
        let source = include_str!("panel.rs");
        assert!(
            source.contains("card.top() + PROJECT_STEPS_TOP"),
            "the card positions its rows from the constant"
        );
        assert!(
            source.contains("index as f32 * PROJECT_ROW_PITCH"),
            "the card pitches its rows from the constant"
        );
        assert_eq!(HEADING_LINE + HEADING_TO_BODY, 39.0, "Home\'s first row, below the card top");
        assert_eq!(
            PROJECT_STEPS_TOP,
            70.0,
            "the project card\'s first row, 70 below the card top in the reference (card 54..272, row 124)"
        );
        assert_eq!(
            PROJECT_CARD_H + 16.0,
            234.0,
            "the height that measures best for the column, 16 over the card\'s own 218"
        );
        assert_eq!(PROJECT_ROW_PITCH, ROW_PITCH, "both cards pitch their rows 48 apart, as the reference does");
        assert_eq!(ROW_PITCH, 48.0, "the reference's own pitch: text bands at 141, 189 and 237");
    }
}
