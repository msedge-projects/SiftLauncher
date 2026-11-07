//! The sidebar's hosting illustration: the reference's own art, embedded.
//!
//! Cropped from the reference's 1440x900 render (`ab-ref-home.png`, 1150,742 to 1440,900 -- the region under
//! the card's heading and above nothing else), so the picture is the one the reference draws rather than an
//! approximation of it. This build used to draw three crates and a torch out of rectangles, which a side-by-side
//! capture showed instantly.
//!
//! Embedded for the same reason the mark is: the exe is the product, and art read from a path beside it
//! disappears the moment the binary is moved.

use std::sync::OnceLock;

use egui::{Color32, Rect, TextureHandle};

const ART_PNG: &[u8] = include_bytes!("../../../assets/brand/server-art.png");

/// The news card's illustration: the green plate with its sliders and link, cropped from the same render
/// (1156,350 to 1428,470 -- the plate's own edges, no text in it).
const NEWS_PNG: &[u8] = include_bytes!("../../../assets/brand/news-art.png");

static ART: OnceLock<Option<TextureHandle>> = OnceLock::new();
/// The **second** news slide's plate, cropped from the same reference render (1156,350 to 1428,470) at the moment
/// its carousel showed "Joining your server just got easier". One plate for two slides was the sidebar's whole
/// difference: 272x120 is 32 640 pixels, which is the size of the slab that would not go away.
const NEWS_JOINING_PNG: &[u8] = include_bytes!("../../../assets/brand/news-joining.png");

static NEWS: OnceLock<Option<TextureHandle>> = OnceLock::new();
static NEWS_JOINING: OnceLock<Option<TextureHandle>> = OnceLock::new();

/// The empty state's plate: the reference's own mark at 100x100, cropped from its render.
const HERO_PNG: &[u8] = include_bytes!("../../../assets/brand/hero-plate.png");

static HERO: OnceLock<Option<TextureHandle>> = OnceLock::new();

/// The title bar's mark and wordmark: the reference's own, cropped from its render at 326x40.
const BRAND_PNG: &[u8] = include_bytes!("../../../assets/brand/titlebar-brand.png");

static BRAND: OnceLock<Option<TextureHandle>> = OnceLock::new();

/// The screenshots page's empty-state illustration: two photo frames, cropped from the reference's render at
/// 228x98 (488,380 to 716,478 of it).
const SHOTS_PNG: &[u8] = include_bytes!("../../../assets/brand/shots-empty.png");

static SHOTS_EMPTY: OnceLock<Option<TextureHandle>> = OnceLock::new();

/// The instance page's "No content installed" illustration: an open box with a face, cropped from the
/// reference's own render at 93x99 (475,302 to 568,401 of it).
const CONTENT_PNG: &[u8] = include_bytes!("../../../assets/brand/content-empty.png");

static CONTENT_EMPTY: OnceLock<Option<TextureHandle>> = OnceLock::new();

/// Upload the illustration, once, before the first frame.
pub fn install(ctx: &egui::Context) {
    // **Only what every frame draws.** The title bar's mark is on screen from the first frame; the rest of these plates
    // belong to pages the reader may never open, and each is a decoded RGBA image resident for the session. They upload
    // on first use instead -- six plates, about 660 kB of texture the idle window no longer holds.
    let _ = BRAND.set(upload(ctx, BRAND_PNG, "titlebar-brand"));
}

/// The plate for one of the lazily-uploaded illustrations: upload on first ask, then answer from the cache.
///
/// Returns `None` when the texture is missing, which every caller already treats as "draw nothing".
fn lazy(
    painter: &egui::Painter,
    lock: &'static OnceLock<Option<TextureHandle>>,
    png: &'static [u8],
    name: &'static str,
) -> Option<&'static TextureHandle> {
    let held = lock.get_or_init(|| upload(painter.ctx(), png, name));
    held.as_ref()
}

/// Paint the instance page's "No content installed" illustration; false when the texture is missing.
pub fn paint_content_empty(painter: &egui::Painter, rect: Rect) -> bool {
    let Some(texture) = lazy(painter, &CONTENT_EMPTY, CONTENT_PNG, "content-empty") else {
        return false;
    };
    painter.image(
        texture.id(),
        rect,
        Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    true
}

/// Paint the screenshots empty state's illustration; false when the texture is missing.
pub fn paint_shots_empty(painter: &egui::Painter, rect: Rect) -> bool {
    let Some(texture) = lazy(painter, &SHOTS_EMPTY, SHOTS_PNG, "shots-empty") else {
        return false;
    };
    painter.image(
        texture.id(),
        rect,
        Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    true
}

/// Paint the title bar's mark and wordmark; false when the texture is missing.
pub fn paint_brand(painter: &egui::Painter, rect: Rect) -> bool {
    let Some(texture) = lazy(painter, &BRAND, BRAND_PNG, "titlebar-brand") else {
        return false;
    };
    painter.image(
        texture.id(),
        rect,
        Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    true
}

/// Paint the empty state's plate; false when the texture is missing, so the caller draws its own.
pub fn paint_hero(painter: &egui::Painter, rect: Rect) -> bool {
    let Some(texture) = lazy(painter, &HERO, HERO_PNG, "hero-plate") else {
        return false;
    };
    painter.image(
        texture.id(),
        rect,
        Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    true
}

/// Paint the news card's plate; false when the texture is missing, so the caller keeps its painted one.
pub fn paint_news(painter: &egui::Painter, rect: Rect, slide: usize) -> bool {
    // Each slide carries its own picture, so the plate follows the article rather than standing in for both.
    // The article's own plate, uploaded the first time that article is drawn.
    let plate = if slide == 1 {
        lazy(painter, &NEWS_JOINING, NEWS_JOINING_PNG, "news-joining")
    } else {
        lazy(painter, &NEWS, NEWS_PNG, "news-art")
    };
    let Some(texture) = plate else {
        return false;
    };
    painter.image(
        texture.id(),
        rect,
        Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    true
}

fn upload(ctx: &egui::Context, bytes: &[u8], name: &str) -> Option<TextureHandle> {
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image
        .pixels()
        .map(|pixel| Color32::from_rgba_unmultiplied(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]))
        .collect();
    Some(ctx.load_texture(
        name,
        egui::ColorImage::new(size, pixels),
        egui::TextureOptions::LINEAR,
    ))
}

/// Paint the illustration into `rect`; false when the texture is not there, so the caller can keep its
/// painted fallback rather than leaving a hole.
pub fn paint(painter: &egui::Painter, rect: Rect) -> bool {
    let Some(texture) = lazy(painter, &ART, ART_PNG, "server-art") else {
        return false;
    };
    painter.image(
        texture.id(),
        rect,
        Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    true
}
