//! Copying an instance, for the library menu's `Duplicate instance`.
//!
//! Two rules the file system will not enforce: a copy must not write into the original (a recursive copy
//! that follows a link or reuses a name destroys the thing it was asked to preserve), and the new folder
//! must not collide with an existing one.

use std::path::Path;

/// A folder name for a copy of `base` that is not in `existing`.
///
/// The first copy is "<base> copy", the second "<base> copy 2" -- the same shape the install path uses
/// when a pack is installed twice, so a reader sees one convention rather than two.
pub fn unique_id(base: &str, existing: &[String]) -> String {
    let first = format!("{base} copy");
    if !existing.iter().any(|id| id == &first) {
        return first;
    }
    let mut n = 2;
    loop {
        let candidate = format!("{first} {n}");
        if !existing.iter().any(|id| id == &candidate) {
            return candidate;
        }
        n += 1;
    }
}

/// Copy `from` into `to`, recursively, and return how many entries were written.
pub fn copy_tree(from: &Path, to: &Path) -> std::io::Result<usize> {
    std::fs::create_dir_all(to)?;
    let mut written = 0;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        let target = to.join(entry.file_name());
        if kind.is_dir() {
            written += copy_tree(&entry.path(), &target)?;
        } else if kind.is_file() {
            std::fs::copy(entry.path(), &target)?;
            written += 1;
        }
        // A symlink is skipped rather than followed: following one can walk out of the instance and, on
        // a loop, copy for ever.
    }
    Ok(written)
}

/// Give the copy its own display name.
///
/// The copy's `instance.cfg` still names the original, so without this the library shows two rows
/// called the same thing and the reader cannot tell which is which.
pub fn rename_copy(dir: &std::path::Path, name: &str) -> std::io::Result<()> {
    let cfg = dir.join("instance.cfg");
    let text = std::fs::read_to_string(&cfg)?;
    let mut out = String::with_capacity(text.len() + name.len());
    let mut replaced = false;
    for line in text.lines() {
        if line.starts_with("name=") {
            out.push_str(&format!("name={name}"));
            replaced = true;
        } else {
            out.push_str(line);
        }
        out.push('\n');
    }
    if !replaced {
        out.push_str(&format!("name={name}\n"));
    }
    std::fs::write(&cfg, out)
}

#[cfg(test)]
mod tests {
    use super::{copy_tree, rename_copy, unique_id};

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("sift-dup-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch");
        dir
    }

    #[test]
    fn the_first_copy_is_named_copy() {
        assert_eq!(unique_id("Instance 2", &[]), "Instance 2 copy");
        assert_eq!(
            unique_id("Instance 2", &["Instance 2".to_owned()]),
            "Instance 2 copy"
        );
    }

    #[test]
    fn later_copies_count_up() {
        let existing = vec!["A copy".to_owned(), "A copy 2".to_owned()];
        assert_eq!(unique_id("A", &existing), "A copy 3");
    }

    #[test]
    fn a_tree_is_copied_whole() {
        let dir = scratch("tree");
        let from = dir.join("from");
        std::fs::create_dir_all(from.join("minecraft/mods")).expect("dirs");
        std::fs::write(from.join("instance.cfg"), b"cfg").expect("write");
        std::fs::write(from.join("minecraft/mods/a.jar"), b"jar").expect("write");
        let to = dir.join("to");
        let written = copy_tree(&from, &to).expect("copy");
        assert_eq!(written, 2);
        assert_eq!(std::fs::read(to.join("instance.cfg")).expect("read"), b"cfg");
        assert!(to.join("minecraft/mods/a.jar").is_file());
    }

    #[test]
    fn the_copy_can_be_given_its_own_name() {
        let dir = scratch("rename");
        std::fs::create_dir_all(&dir).expect("dir");
        std::fs::write(dir.join("instance.cfg"), b"InstanceType=OneSix\nname=Instance 2\n").expect("write");
        rename_copy(&dir, "Instance 2 copy").expect("rename");
        let text = std::fs::read_to_string(dir.join("instance.cfg")).expect("read");
        assert!(text.contains("name=Instance 2 copy"));
        assert!(text.contains("InstanceType=OneSix"));
        assert!(!text.contains("name=Instance 2\n"));
    }

    #[test]
    fn the_original_is_left_alone() {
        let dir = scratch("alone");
        let from = dir.join("from");
        std::fs::create_dir_all(&from).expect("dir");
        std::fs::write(from.join("a.txt"), b"a").expect("write");
        let to = dir.join("to");
        copy_tree(&from, &to).expect("copy");
        std::fs::write(to.join("a.txt"), b"changed").expect("write");
        assert_eq!(std::fs::read(from.join("a.txt")).expect("read"), b"a");
    }
}
