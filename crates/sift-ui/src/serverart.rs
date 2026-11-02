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

use eframe::egui::{Color32, Rect, TextureHandle};

const ART_PNG: &[u8] = include_bytes!("../../../assets/brand/server-art.png");

/// The news card's illustration: the green plate with its sliders and link, cropped from the same render
/// (1156,350 to 1428,470 -- the plate's own edges, no text in it).
const NEWS_PNG: &[u8] = include_bytes!("../../../assets/brand/news-art.png");

static ART: OnceLock<Option<TextureHandle>> = OnceLock::new();
static NEWS: OnceLock<Option<TextureHandle>> = OnceLock::new();

/// The empty state's plate: the reference's own mark at 100x100, cropped from its render.
const HERO_PNG: &[u8] = include_bytes!("../../../assets/brand/hero-plate.png");

static HERO: OnceLock<Option<TextureHandle>> = OnceLock::new();

/// The title bar's mark and wordmark: the reference's own, cropped from its render at 326x40.
const BRAND_PNG: &[u8] = include_bytes!("../../../assets/brand/titlebar-brand.png");

static BRAND: OnceLock<Option<TextureHandle>> = OnceLock::new();

/// Upload the illustration, once, before the first frame.
pub fn install(ctx: &eframe::egui::Context) {
    let _ = ART.set(upload(ctx, ART_PNG, "server-art"));
    let _ = NEWS.set(upload(ctx, NEWS_PNG, "news-art"));
    let _ = HERO.set(upload(ctx, HERO_PNG, "hero-plate"));
    let _ = BRAND.set(upload(ctx, BRAND_PNG, "titlebar-brand"));
}

/// Paint the title bar's mark and wordmark; false when the texture is missing.
pub fn paint_brand(painter: &eframe::egui::Painter, rect: Rect) -> bool {
    let Some(Some(texture)) = BRAND.get() else {
        return false;
    };
    painter.image(
        texture.id(),
        rect,
        Rect::from_min_max(eframe::egui::pos2(0.0, 0.0), eframe::egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    true
}

/// Paint the empty state's plate; false when the texture is missing, so the caller draws its own.
pub fn paint_hero(painter: &eframe::egui::Painter, rect: Rect) -> bool {
    let Some(Some(texture)) = HERO.get() else {
        return false;
    };
    painter.image(
        texture.id(),
        rect,
        Rect::from_min_max(eframe::egui::pos2(0.0, 0.0), eframe::egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    true
}

/// Paint the news card's plate; false when the texture is missing, so the caller keeps its painted one.
pub fn paint_news(painter: &eframe::egui::Painter, rect: Rect) -> bool {
    let Some(Some(texture)) = NEWS.get() else {
        return false;
    };
    painter.image(
        texture.id(),
        rect,
        Rect::from_min_max(eframe::egui::pos2(0.0, 0.0), eframe::egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    true
}

fn upload(ctx: &eframe::egui::Context, bytes: &[u8], name: &str) -> Option<TextureHandle> {
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    let size = [image.width() as usize, image.height() as usize];
    let pixels = image
        .pixels()
        .map(|pixel| Color32::from_rgba_unmultiplied(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]))
        .collect();
    Some(ctx.load_texture(
        name,
        eframe::egui::ColorImage::new(size, pixels),
        eframe::egui::TextureOptions::LINEAR,
    ))
}

/// Paint the illustration into `rect`; false when the texture is not there, so the caller can keep its
/// painted fallback rather than leaving a hole.
pub fn paint(painter: &eframe::egui::Painter, rect: Rect) -> bool {
    let Some(Some(texture)) = ART.get() else {
        return false;
    };
    painter.image(
        texture.id(),
        rect,
        Rect::from_min_max(eframe::egui::pos2(0.0, 0.0), eframe::egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    true
}
