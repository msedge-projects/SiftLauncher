//! The skins on this machine: `<data root>/skins/*.png`.
//!
//! **Why a folder and not the service.** The reference's skins page is fed by Modrinth's
//! skin service: a saved skin is a texture the account owns, fetched by URL and rendered by
//! a WebGL preview. This launcher has no client for that service, so the page had no
//! texture to draw — every card was a glyph and the preview column a painted plate. A local
//! folder gives the page a source that needs no account and no network, and it is the
//! source the gap matrix names as the buildable path around the block
//! (`docs/GAP-MATRIX.md`, "Skins: BLOCKED-SERVICE, not open").
//!
//! **The convention is one line**: a skin is a PNG whose stem is its name, at
//! `<platform data root>/skins/<name>.png`. The platform data root is the same
//! `sift_core::paths::platform_data_root` that `data::instances_dir` reads, so a skin sits
//! beside the `instances/` folder a reader can already find. The reference's own data
//! directory has no `skins/` folder — this one is this launcher's, and its name is the
//! whole of what a reader has to know.
//!
//! **A 64x64 atlas is the unit.** The page reads the head's front face at (8,8,8,8) and its
//! overlay at (40,8,8,8); the rest of the atlas is body, arms and legs that the page does
//! not draw yet. A file that is not 64x64 is still listed (the stem is still a name) and is
//! left to the image decoder, which draws whatever the atlas's coordinates hold rather than
//! refusing the card: a wrong-sized skin is a visibly wrong face, not a missing one.

use std::path::{Path, PathBuf};

use sift_core::paths::{platform_data_root, System};

/// The folder under the data root that holds saved skins.
pub const SKINS_DIR: &str = "skins";

/// The only format the page lists. Minecraft skins are PNGs, and a JPEG cannot carry the
/// overlay's alpha.
pub const SKIN_EXTENSION: &str = "png";

/// One saved skin: the name its stem gives it, and the file the texture comes from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedSkin {
    /// The file stem, which is what the card prints.
    pub name: String,
    /// The PNG on disk.
    pub path: PathBuf,
}

/// Where saved skins live: `<platform data root>/skins`.
///
/// The platform root rather than `SiftPaths::data_root`: the instances reader next door
/// (`data::instances_dir`) resolves the same root, and two answers to "where is the data"
/// on one machine is the failure this convention is meant to avoid.
pub fn skins_dir() -> PathBuf {
    platform_data_root(System::current()).join(SKINS_DIR)
}

/// Every skin in the launcher's own folder, sorted by name.
pub fn list() -> Vec<SavedSkin> {
    list_in(&skins_dir())
}

/// Every PNG in `dir`, sorted by name.
///
/// A missing folder is an empty list, not an error: the first run has no `skins/` folder,
/// and the page's job with no files is the same as its job with a folder it cannot read.
/// Only regular files are listed; a directory named `foo.png` is not a skin.
pub fn list_in(dir: &Path) -> Vec<SavedSkin> {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut skins: Vec<SavedSkin> = entries
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let extension = path.extension().and_then(|ext| ext.to_str())?;
            if !extension.eq_ignore_ascii_case(SKIN_EXTENSION) {
                return None;
            }
            if !path.is_file() {
                return None;
            }
            let name = path.file_stem()?.to_string_lossy().trim().to_owned();
            if name.is_empty() {
                return None;
            }
            Some(SavedSkin { name, path })
        })
        .collect();
    // Case-insensitive first so `apple` and `Banana` read in the order a person sees
    // them, then the exact name and path so the order is total and a capture is stable.
    skins.sort_by(|a, b| {
        a.name
            .to_lowercase()
            .cmp(&b.name.to_lowercase())
            .then_with(|| a.name.cmp(&b.name))
            .then_with(|| a.path.cmp(&b.path))
    });
    skins
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A scratch folder per test, removed first so a rerun starts clean.
    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("sift-skins-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        dir
    }

    #[test]
    fn a_missing_folder_has_no_skins() {
        let dir = std::env::temp_dir().join(format!("sift-skins-absent-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        assert!(list_in(&dir).is_empty());
    }

    #[test]
    fn every_png_is_a_named_skin() {
        let dir = scratch("list");
        for name in ["steve.png", "Alex.PNG", "armour.v2.png"] {
            std::fs::write(dir.join(name), b"png").expect("write");
        }
        let skins = list_in(&dir);
        let names: Vec<&str> = skins.iter().map(|skin| skin.name.as_str()).collect();
        assert_eq!(names, ["Alex", "armour.v2", "steve"]);
        assert_eq!(skins[0].path, dir.join("Alex.PNG"), "the file keeps its name");
        assert!(skins.iter().all(|skin| skin.path.is_file()));
    }

    #[test]
    fn only_regular_pngs_are_listed() {
        let dir = scratch("filter");
        std::fs::write(dir.join("skin.png"), b"png").expect("write");
        std::fs::write(dir.join("notes.txt"), b"text").expect("write");
        std::fs::write(dir.join("jpeg.jpg"), b"jpg").expect("write");
        std::fs::create_dir_all(dir.join("folder.png")).expect("dir");
        let names: Vec<String> = list_in(&dir).into_iter().map(|skin| skin.name).collect();
        assert_eq!(names, ["skin"], "only the PNG file is a skin");
    }

    #[test]
    fn the_folder_is_under_the_platform_data_root() {
        let dir = skins_dir();
        assert_eq!(dir.file_name().and_then(|name| name.to_str()), Some(SKINS_DIR));
        assert_eq!(
            dir.parent(),
            Some(platform_data_root(System::current()).as_path()),
            "the skins folder sits at the root the instances reader uses"
        );
    }
}
