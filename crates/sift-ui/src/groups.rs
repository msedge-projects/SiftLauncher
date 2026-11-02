//! The reader's own screenshot groups.
//!
//! The reference lets a shot be filed under a group the reader names (`Screenshots.vue`, "New group"). The
//! store is one JSON file beside the launcher's other state: groups by id and name, and which shot belongs to
//! which. A shot the reader has not filed belongs to no group, which is a state and not an error.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// One group: a stable id and the name the reader gave it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    pub id: String,
    pub name: String,
}

/// The whole store.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Groups {
    pub groups: Vec<Group>,
    /// Shot file name -> group id.
    pub assignments: BTreeMap<String, String>,
}

impl Groups {
    /// A file name for the store, under the launcher's data root.
    pub fn path(root: &Path) -> PathBuf {
        root.join("screenshot-groups.json")
    }

    /// Read the store; a missing or unreadable file is an empty one, because a first run has no groups and a
    /// broken file must not stop the screenshots page from opening.
    pub fn load(path: &Path) -> Self {
        let Ok(text) = std::fs::read_to_string(path) else {
            return Self::default();
        };
        Self::from_json(&text)
    }

    /// Write the store, creating its folder.
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, self.to_json())
    }

    /// A group id that is not taken: the name, slugged, with a counter when it is.
    pub fn unique_id(&self, name: &str) -> String {
        let base: String = name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
            .collect();
        let base = base.trim_matches('-').to_owned();
        let base = if base.is_empty() { "group".to_owned() } else { base };
        if !self.groups.iter().any(|group| group.id == base) {
            return base;
        }
        let mut n = 2;
        loop {
            let candidate = format!("{base}-{n}");
            if !self.groups.iter().any(|group| group.id == candidate) {
                return candidate;
            }
            n += 1;
        }
    }

    /// Add a group and return its id. A blank name is refused: a group with no name is a row nobody can
    /// recognise in the list.
    pub fn add(&mut self, name: &str) -> Option<String> {
        let name = name.trim();
        if name.is_empty() {
            return None;
        }
        let id = self.unique_id(name);
        self.groups.push(Group { id: id.clone(), name: name.to_owned() });
        Some(id)
    }

    /// File a shot under a group, or clear it when `group` is `None`.
    pub fn assign(&mut self, shot: &str, group: Option<&str>) {
        match group {
            Some(id) if self.groups.iter().any(|entry| entry.id == id) => {
                self.assignments.insert(shot.to_owned(), id.to_owned());
            }
            _ => {
                self.assignments.remove(shot);
            }
        }
    }

    /// The group a shot belongs to, if any.
    pub fn group_of(&self, shot: &str) -> Option<&Group> {
        let id = self.assignments.get(shot)?;
        self.groups.iter().find(|group| &group.id == id)
    }

    fn to_json(&self) -> String {
        let groups: Vec<String> = self
            .groups
            .iter()
            .map(|group| format!(r#"{{"id":{},"name":{}}}"#, quote(&group.id), quote(&group.name)))
            .collect();
        let assigned: Vec<String> = self
            .assignments
            .iter()
            .map(|(shot, group)| format!("{}:{}", quote(shot), quote(group)))
            .collect();
        format!(
            "{{\"groups\":[{}],\"assignments\":{{{}}}}}",
            groups.join(","),
            assigned.join(",")
        )
    }

    fn from_json(text: &str) -> Self {
        // A hand-rolled reader rather than a serde pass: the file is two arrays of short strings, and a
        // dependency for that would be the third one this module added for no gain.
        let mut store = Self::default();
        if let Some(position) = text.find("\"groups\":[") {
            let body = &text[position + 10..];
            let body = &body[..body.find(']').unwrap_or(body.len())];
            // Each group is one object; its fields are the same \"key\":\"value\" pairs the assignments use.
            for chunk in body.split('}') {
                let fields = pairs(chunk);
                let get = |key: &str| {
                    fields
                        .iter()
                        .find(|(name, _)| name == key)
                        .map(|(_, value)| value.clone())
                };
                if let (Some(id), Some(name)) = (get("id"), get("name")) {
                    store.groups.push(Group { id, name });
                }
            }
        }
        if let Some(start) = text.find("\"assignments\":{") {
            let body = &text[start + 15..];
            let body = &body[..body.find('}').unwrap_or(body.len())];
            for (shot, group) in pairs(body) {
                store.assignments.insert(shot, group);
            }
        }
        store
    }
}

/// Every `"key":"value"` pair in an object body.
fn pairs(body: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find('"') {
        let inner = &rest[start + 1..];
        let Some(middle) = inner.find("\":\"") else { break };
        let key = &inner[..middle];
        let after = &inner[middle + 3..];
        let Some(end) = after.find('"') else { break };
        out.push((key.to_owned(), after[..end].to_owned()));
        rest = &after[end + 1..];
    }
    out
}

/// A JSON string literal, with the two characters that matter escaped.
fn quote(text: &str) -> String {
    format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
}

/// One field of a JSON object body, as a string.
fn field(body: &str, key: &str) -> Option<String> {
    let start = body.find(&format!("\"{key}\":"))? + key.len() + 3;
    let rest = &body[start..];
    let rest = rest.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_group_gets_a_slug_and_a_name() {
        let mut groups = Groups::default();
        assert_eq!(groups.add("Best of 2026").as_deref(), Some("best-of-2026"));
        assert_eq!(groups.groups[0].name, "Best of 2026");
    }

    #[test]
    fn a_repeated_name_gets_its_own_id() {
        let mut groups = Groups::default();
        groups.add("Raids");
        assert_eq!(groups.add("Raids").as_deref(), Some("raids-2"));
    }

    #[test]
    fn a_blank_name_is_refused() {
        let mut groups = Groups::default();
        assert_eq!(groups.add("   "), None);
        assert!(groups.groups.is_empty());
    }

    #[test]
    fn a_shot_can_be_filed_and_cleared() {
        let mut groups = Groups::default();
        let id = groups.add("Raids").expect("group");
        groups.assign("shot.png", Some(&id));
        assert_eq!(groups.group_of("shot.png").map(|g| g.name.as_str()), Some("Raids"));
        groups.assign("shot.png", None);
        assert!(groups.group_of("shot.png").is_none());
    }

    #[test]
    fn filing_into_a_group_that_does_not_exist_clears_it() {
        let mut groups = Groups::default();
        groups.assign("shot.png", Some("nope"));
        assert!(groups.group_of("shot.png").is_none());
    }

    #[test]
    fn the_store_round_trips_through_its_file() {
        let dir = std::env::temp_dir().join(format!("sift-groups-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("dir");
        let path = Groups::path(&dir);
        let mut groups = Groups::default();
        let id = groups.add("Raids").expect("group");
        groups.assign("shot one.png", Some(&id));
        groups.save(&path).expect("save");
        let read = Groups::load(&path);
        assert_eq!(read, groups);
    }

    #[test]
    fn a_missing_file_is_an_empty_store() {
        let path = std::env::temp_dir().join("sift-groups-missing/never.json");
        assert!(Groups::load(&path).groups.is_empty());
    }
}
