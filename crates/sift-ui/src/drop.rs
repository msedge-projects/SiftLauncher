//! Copying files that were dropped onto the Files panel.
//!
//! **Why a module and not four lines in the page.** A drag cannot be driven in this lab's Xvfb without a
//! file manager to drag *from*, so the behaviour has to be provable another way: the copy is a pure
//! function of a directory and a list of paths, and it has tests. The page's job is only to say where the
//! pointer was.

use std::path::{Path, PathBuf};

/// What a drop did, so the caller can say it in one sentence.
#[derive(Debug, PartialEq, Eq)]
pub struct Dropped {
    /// The files that are now in the directory.
    pub copied: Vec<String>,
    /// Files that could not be read, with the reason.
    pub failed: Vec<(String, String)>,
}

/// Copy every dropped file into `dir`.
///
/// A dropped *directory* is refused rather than walked: the reference copies files, and a recursive copy
/// that silently descends a tree is a much larger action than the one a reader asked for.
pub fn copy_into(dir: &Path, paths: &[PathBuf]) -> Result<Dropped, String> {
    if paths.is_empty() {
        return Ok(Dropped { copied: Vec::new(), failed: Vec::new() });
    }
    std::fs::create_dir_all(dir).map_err(|error| format!("could not make {}: {error}", dir.display()))?;
    let mut copied = Vec::new();
    let mut failed = Vec::new();
    for path in paths {
        let Some(name) = path.file_name().map(|n| n.to_string_lossy().into_owned()) else {
            continue;
        };
        if path.is_dir() {
            failed.push((name, "a folder can only be added one file at a time".to_owned()));
            continue;
        }
        match std::fs::copy(path, dir.join(&name)) {
            Ok(_) => copied.push(name),
            Err(error) => failed.push((name, error.to_string())),
        }
    }
    Ok(Dropped { copied, failed })
}

#[cfg(test)]
mod tests {
    use super::copy_into;
    use std::path::PathBuf;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("sift-drop-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        dir
    }

    #[test]
    fn an_empty_drop_does_nothing() {
        let dir = scratch("empty");
        let out = copy_into(&dir, &[]).expect("ok");
        assert!(out.copied.is_empty());
        assert!(out.failed.is_empty());
    }

    #[test]
    fn a_file_lands_under_its_own_name() {
        let root = scratch("one");
        let source = root.join("sodium.jar");
        std::fs::write(&source, b"jar").expect("write");
        let target = root.join("mods");
        let out = copy_into(&target, std::slice::from_ref(&source)).expect("ok");
        assert_eq!(out.copied, vec!["sodium.jar".to_owned()]);
        assert!(target.join("sodium.jar").is_file());
    }

    #[test]
    fn a_folder_is_refused_and_named() {
        let root = scratch("dir");
        let source = root.join("world");
        std::fs::create_dir_all(&source).expect("dir");
        let out = copy_into(&root.join("mods"), std::slice::from_ref(&source)).expect("ok");
        assert!(out.copied.is_empty());
        assert_eq!(out.failed.len(), 1);
        assert_eq!(out.failed[0].0, "world");
    }
}
