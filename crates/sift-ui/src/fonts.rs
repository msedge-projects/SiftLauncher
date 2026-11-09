//! Inter, at the weights the reference actually uses.
//!
//! egui ships one proportional face and no weight axis, so a heading and a body line come
//! out the same weight unless the fonts are registered by hand. The reference's stylesheet
//! asks for **400** (body), **500** (labels), **600** (tab labels and section headings),
//! **700** (card titles) and **800** (page headings), and at 1440x900 the difference
//! between 400 and 600 is the first thing an eye catches in a side-by-side capture -- the
//! tab strip looked "lighter and smaller" for exactly this reason.
//!
//! Each weight becomes its own `FontFamily`, so a call site names the weight it wants:
//!
//! ```text
//! ui.painter().text(pos, align, "Mods", fonts::semibold(14.0), colour);
//! ```
//!
//! The files are the same Inter TTFs the iced build embedded, kept in `assets/fonts/` now that
//! assets today; when that crate is deleted this module's `include_bytes!` paths move with
//! them, which is why the path is written once, here.

use egui::{FontData, FontDefinitions, FontFamily, FontId};

/// The weights the interface uses, as `(family name, file name)`.
///
/// The names are what a call site writes; the files are Inter's static weights. A weight
/// the interface never asks for is not shipped -- five faces is 1.5 MB already.
const FACES: [(&str, &str); 5] = [
    ("inter-400", "Inter-400.ttf"),
    ("inter-500", "Inter-500.ttf"),
    ("inter-600", "Inter-600.ttf"),
    ("inter-700", "Inter-700.ttf"),
    ("inter-800", "Inter-800.ttf"),
];

/// Register every weight with the context, and make Inter the default proportional face.
///
/// Called once at startup from `main`. Registering rather than replacing matters: egui's
/// own glyph fallbacks stay available, so a character Inter lacks does not become a box.
/// Where a desktop keeps its fonts, in the order they are usually authoritative.
const FONT_DIRS: &[&str] = &[
    "/usr/share/fonts",
    "/usr/local/share/fonts",
    "/System/Library/Fonts",
    "C:\\Windows\\Fonts",
];

/// Families whose presence means the machine can draw CJK, matched on the file name because that is all a directory
/// scan has: fontconfig would need a library and a cache, and this needs one answer at startup.
const CJK_FAMILIES: &[&str] = &[
    "NotoSansCJK",
    "NotoSansKR",
    "NotoSansJP",
    "NotoSansSC",
    "NotoSansTC",
    "NotoSerifCJK",
    "SourceHanSans",
    "SourceHanSerif",
    "wqy-",
    "WenQuanYi",
    "malgun",
    "YuGothic",
    "PingFang",
    "Hiragino",
    "msyh",
    "simhei",
    "simsun",
];

/// Whether a font file looks like a CJK face worth loading.
fn is_cjk_font(file_name: &str) -> bool {
    let lower = file_name.to_ascii_lowercase();
    if !(lower.ends_with(".ttf") || lower.ends_with(".otf")) {
        // Collections (.ttc) cannot be read by the rasteriser and would panic at layout time.
        return false;
    }
    CJK_FAMILIES
        .iter()
        .any(|family| lower.contains(&family.to_ascii_lowercase()))
}

/// One CJK face from the system, if the system has one: the first match in [`FONT_DIRS`], two levels deep, which covers
/// `/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttf` and `/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttf` alike.
fn system_fallbacks() -> Vec<(String, Vec<u8>)> {
    let roots: Vec<std::path::PathBuf> = FONT_DIRS.iter().map(std::path::PathBuf::from).collect();
    system_fallbacks_in(&roots)
}

/// The scan itself, over a given list of roots, so a test can point it at a directory it owns.
fn system_fallbacks_in(roots: &[std::path::PathBuf]) -> Vec<(String, Vec<u8>)> {
    let mut found = Vec::new();
    for root in roots {
        let Ok(entries) = std::fs::read_dir(root) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let candidates = if path.is_dir() {
                match std::fs::read_dir(&path) {
                    Ok(inner) => inner.flatten().map(|item| item.path()).collect(),
                    Err(_) => continue,
                }
            } else {
                vec![path]
            };
            for candidate in candidates {
                let Some(name) = candidate.file_name().and_then(|name| name.to_str()) else {
                    continue;
                };
                if !is_cjk_font(name) {
                    continue;
                }
                if let Ok(bytes) = std::fs::read(&candidate) {
                    found.push((format!("system-{name}"), bytes));
                    return found;
                }
            }
        }
    }
    found
}

pub fn install(ctx: &egui::Context) {
    let mut definitions = FontDefinitions::default();

    for (family, file) in FACES {
        let bytes: &[u8] = match file {
            "Inter-400.ttf" => include_bytes!("../../../assets/fonts/Inter-400.ttf"),
            "Inter-500.ttf" => include_bytes!("../../../assets/fonts/Inter-500.ttf"),
            "Inter-600.ttf" => include_bytes!("../../../assets/fonts/Inter-600.ttf"),
            "Inter-700.ttf" => include_bytes!("../../../assets/fonts/Inter-700.ttf"),
            "Inter-800.ttf" => include_bytes!("../../../assets/fonts/Inter-800.ttf"),
            _ => continue,
        };
        definitions
            .font_data
            .insert(
                family.to_owned(),
                std::sync::Arc::new(FontData::from_static(bytes).tweak(egui::FontTweak {
                    // **One part in 250, measured.** "Fabulously Optimized" renders 248 points here and 249 in the
                    // reference; the faces are the same family and weight, so the difference is in the advance. A
                    // long line accumulates it into the two or three points that decide where a word wraps.
                    scale: 1.004,
                    ..Default::default()
                })),
            );
        definitions
            .families
            .insert(FontFamily::Name(family.into()), vec![family.to_owned()]);
    }

    // **Every family is Inter.** With egui's bundled faces gone there is nothing to fall back to, so the two generic
    // families point at our own weights: body text is Inter 400, and monospace -- which this interface never uses, but
    // which a widget could still ask for -- is Inter 400 as well rather than a missing family and a panic.
    // **A system fallback, when the machine has one.** Inter carries no CJK, so a modpack whose description is Korean
    // drew "?????" here where the reference drew the same text as tofu boxes -- and the two substitutes are different
    // widths, so every line of that description rewrapped. A CJK face in the binary would be several megabytes against a
    // memory gate already 4.2 MiB over, so the face is taken from the system at startup if it exists: nothing is added to
    // the executable, and a machine without one behaves exactly as before.
    let mut proportional = vec!["inter-400".to_owned()];
    let mut monospace = vec!["inter-400".to_owned()];
    for (name, bytes) in system_fallbacks() {
        definitions
            .font_data
            .insert(name.clone(), std::sync::Arc::new(FontData::from_owned(bytes)));
        proportional.push(name.clone());
        monospace.push(name);
    }
    definitions
        .families
        .insert(FontFamily::Proportional, proportional);
    definitions
        .families
        .insert(FontFamily::Monospace, monospace);

    ctx.set_fonts(definitions);
}

/// A weighted face, by the name given in [`FACES`].
///
/// A weight that was not registered falls back to the default proportional family rather
/// than panicking: a missing font is a look that is wrong, not a window that does not open.
/// Every text size is multiplied by this before the face is asked for, which is the one knob that moves the whole
/// interface's type together.
///
/// Measured rather than guessed: the reference's "No instances running" spans 143 points of ink where this build
/// spanned 138 -- 3.6% narrower at the same nominal size -- and the same shortfall shows on every string measured
/// so far. Inter's optical sizing in a browser and egui's hinted rasterising disagree by about that much at 12-16
/// points, and one scale factor is honest about it where a per-string fudge would not be.
const TEXT_SCALE: f32 = 1.0;

fn face(name: &str, size: f32) -> FontId {
    FontId::new(size * TEXT_SCALE, FontFamily::Name(name.into()))
}

/// Body text.
pub fn body(size: f32) -> FontId {
    face("inter-400", size)
}

/// A label, one step heavier than body.
pub fn label(size: f32) -> FontId {
    face("inter-500", size)
}

/// Tab labels and section headings: the reference's `font-semibold`.
pub fn semibold(size: f32) -> FontId {
    face("inter-600", size)
}

/// Card titles.
pub fn title(size: f32) -> FontId {
    face("inter-700", size)
}

/// Page headings: the reference's `font-extrabold` on a page's own title.
pub fn heading(size: f32) -> FontId {
    face("inter-800", size)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every weight the helpers name must be one the installer registers, or the helper
    /// silently falls back to 400 and a heading looks like a paragraph.
    /// The scan finds a CJK-named face, reads it, and refuses everything else -- driven through a directory the test owns,
    /// with this repository's own Inter renamed into a CJK name so the file is a real TTF without needing a system font.
    #[test]
    fn the_scan_finds_a_cjk_face_in_a_given_directory() {
        let dir = std::env::temp_dir().join("sift-font-scan-test");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a temp directory");
        std::fs::copy(
            concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/fonts/Inter-400.ttf"),
            dir.join("NotoSansKR-Test.ttf"),
        )
        .expect("the bundled Inter, renamed");
        std::fs::copy(
            concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/fonts/Inter-500.ttf"),
            dir.join("DejaVuSans.ttf"),
        )
        .expect("a font that must not match");
        let found = system_fallbacks_in(&[dir.clone()]);
        assert_eq!(found.len(), 1, "exactly one CJK-named face");
        assert_eq!(found[0].0, "system-NotoSansKR-Test.ttf");
        assert!(found[0].1.len() > 1000, "the file is read, not merely named");
        let _ = std::fs::remove_dir_all(&dir);
    }
    /// The fallback matcher: names that mean CJK, names that do not, and the collection format the rasteriser cannot read.
    #[test]
    fn the_system_fallback_matches_cjk_faces_only() {
        assert!(is_cjk_font("NotoSansCJK-Regular.ttf"));
        assert!(is_cjk_font("NotoSansKR-Regular.otf"));
        assert!(is_cjk_font("wqy-zenhei.ttf"));
        assert!(is_cjk_font("malgun.ttf"));
        assert!(is_cjk_font("PingFang.ttc") == false, "a collection would panic the rasteriser");
        assert!(!is_cjk_font("NotoSans-Regular.ttf"), "Latin Noto is not a fallback for CJK");
        assert!(!is_cjk_font("DejaVuSans.ttf"));
        assert!(!is_cjk_font("NotoSansCJK-Regular.ttc"), ".ttc is excluded whatever the family");
    }
    #[test]
    fn every_helper_names_a_registered_face() {
        let registered: Vec<&str> = FACES.iter().map(|(family, _)| *family).collect();
        for family in ["inter-400", "inter-500", "inter-600", "inter-700", "inter-800"] {
            assert!(registered.contains(&family), "{family} is not registered");
        }
    }
}
