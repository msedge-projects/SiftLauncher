//! `mmc-pack.json` — the component (patch) list of an instance.
//!
//! Mirrors `launcher/minecraft/PackProfile.cpp` (develop):
//!
//! * Root object: `{"components": [...], "formatVersion": 1}`; any other
//!   `formatVersion` is a hard parse error.
//! * Component fields on **write** (present only when meaningful):
//!   `uid` (always), `version` (non-empty), `dependencyOnly`, `important`,
//!   `disabled` (true), `cachedVersion`, `cachedName` (non-empty),
//!   `cachedRequires` / `cachedConflicts` (non-empty), `cachedVolatile`
//!   (true).
//! * On **read**, `cachedVolatile` is taken from the key `volatile` — a
//!   live asymmetry in Prism's own code that we reproduce verbatim so both
//!   launchers behave identically on the same files.
//! * Duplicate `uid` entries: the first wins, later ones are ignored
//!   ("Ignoring duplicate component entry").
//! * Missing `uid` in a component: hard error (`Json::requireString`).
//! * Output uses `QJsonDocument::Indented` shape: 4-space indent,
//!   alphabetical keys, trailing newline (see [`crate::json`]).

use crate::error::{Error, Result};
use crate::json;
use std::path::Path;

/// A metadata requirement (`Meta::Require`): serialized as
/// `{"uid": ..., "equals": ..., "suggests": ...}` with empty members
/// omitted (`meta/JsonFormat.cpp`).
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Require {
    /// Required component uid.
    pub uid: String,
    /// Exact version requirement (`equals` key).
    pub equals_version: String,
    /// Suggested version (`suggests` key).
    pub suggests: String,
}

impl Require {
    /// Build a requirement that only names a uid.
    pub fn uid(uid: impl Into<String>) -> Require {
        Require {
            uid: uid.into(),
            ..Default::default()
        }
    }

    /// Parse from a JSON object; missing members are empty.
    pub fn from_json(value: &serde_json::Value) -> Option<Require> {
        let obj = value.as_object()?;
        let uid = obj.get("uid")?.as_str()?.to_string();
        let equals_version = obj
            .get("equals")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let suggests = obj
            .get("suggests")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        Some(Require {
            uid,
            equals_version,
            suggests,
        })
    }

    /// Serialize per `Meta::serializeRequires` (empty members omitted).
    pub fn to_json(&self) -> serde_json::Value {
        let mut obj = serde_json::Map::new();
        obj.insert("uid".into(), serde_json::Value::String(self.uid.clone()));
        if !self.equals_version.is_empty() {
            obj.insert(
                "equals".into(),
                serde_json::Value::String(self.equals_version.clone()),
            );
        }
        if !self.suggests.is_empty() {
            obj.insert(
                "suggests".into(),
                serde_json::Value::String(self.suggests.clone()),
            );
        }
        serde_json::Value::Object(obj)
    }
}

/// Mod loader kinds recognized by Prism (`Component::KNOWN_MODLOADERS`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModLoader {
    /// NeoForge (`net.neoforged`).
    NeoForge,
    /// Forge (`net.minecraftforge`).
    Forge,
    /// Fabric (`net.fabricmc.fabric-loader`).
    Fabric,
    /// Quilt (`org.quiltmc.quilt-loader`).
    Quilt,
    /// LiteLoader (`com.mumfrey.liteloader`).
    LiteLoader,
}

impl ModLoader {
    /// The component uid for this loader.
    pub fn uid(self) -> &'static str {
        match self {
            ModLoader::NeoForge => "net.neoforged",
            ModLoader::Forge => "net.minecraftforge",
            ModLoader::Fabric => "net.fabricmc.fabric-loader",
            ModLoader::Quilt => "org.quiltmc.quilt-loader",
            ModLoader::LiteLoader => "com.mumfrey.liteloader",
        }
    }

    /// Recognize a loader by uid.
    pub fn from_uid(uid: &str) -> Option<ModLoader> {
        match uid {
            "net.neoforged" => Some(ModLoader::NeoForge),
            "net.minecraftforge" => Some(ModLoader::Forge),
            "net.fabricmc.fabric-loader" => Some(ModLoader::Fabric),
            "org.quiltmc.quilt-loader" => Some(ModLoader::Quilt),
            "com.mumfrey.liteloader" => Some(ModLoader::LiteLoader),
            _ => None,
        }
    }

    /// Uids that conflict with this loader (`knownConflictingComponents`).
    pub fn conflicting_uids(self) -> &'static [&'static str] {
        match self {
            ModLoader::NeoForge => &[
                "net.minecraftforge",
                "net.fabricmc.fabric-loader",
                "org.quiltmc.quilt-loader",
            ],
            ModLoader::Forge => &[
                "net.neoforged",
                "net.fabricmc.fabric-loader",
                "org.quiltmc.quilt-loader",
            ],
            ModLoader::Fabric => &[
                "net.minecraftforge",
                "net.neoforged",
                "org.quiltmc.quilt-loader",
            ],
            ModLoader::Quilt => &[
                "net.minecraftforge",
                "net.neoforged",
                "net.fabricmc.fabric-loader",
            ],
            ModLoader::LiteLoader => &[],
        }
    }
}

/// One entry of `mmc-pack.json` (`Component`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Component {
    /// Component uid (required).
    pub uid: String,
    /// Requested version (empty = whatever the metadata provides).
    pub version: String,
    /// `dependencyOnly`: pulled in by another component, cannot be disabled.
    pub dependency_only: bool,
    /// `important`: user-requested core component (cannot be removed).
    pub important: bool,
    /// `disabled`: excluded from resolution and launch.
    pub disabled: bool,
    /// Cached resolved version (`cachedVersion`).
    pub cached_version: String,
    /// Cached display name (`cachedName`).
    pub cached_name: String,
    /// Cached requirements (`cachedRequires`).
    pub cached_requires: Vec<Require>,
    /// Cached conflicts (`cachedConflicts`).
    pub cached_conflicts: Vec<Require>,
    /// Cached volatility. NOTE: read from the `volatile` key and written as
    /// `cachedVolatile`, exactly like Prism.
    pub cached_volatile: bool,
}

impl Component {
    /// `important` components are non-removable (`Component::isRemovable`).
    pub fn is_removable(&self) -> bool {
        !self.important
    }

    /// `dependencyOnly` components cannot be disabled. Note `important`
    /// only controls removability: an important component can still be
    /// toggled (but disabling is ineffective, see `is_enabled`).
    pub fn can_be_disabled(&self) -> bool {
        !self.dependency_only
    }

    /// Effective enabled state (`Component::isEnabled`): disabling an
    /// `important` or `dependencyOnly` component has no effect.
    pub fn is_enabled(&self) -> bool {
        !self.disabled || self.important || self.dependency_only
    }

    /// Serialize per `componentToJsonV1` (field presence rules above).
    pub fn to_json(&self) -> serde_json::Value {
        let mut obj = serde_json::Map::new();
        obj.insert("uid".into(), serde_json::Value::String(self.uid.clone()));
        if !self.version.is_empty() {
            obj.insert(
                "version".into(),
                serde_json::Value::String(self.version.clone()),
            );
        }
        if self.dependency_only {
            obj.insert("dependencyOnly".into(), serde_json::Value::Bool(true));
        }
        if self.important {
            obj.insert("important".into(), serde_json::Value::Bool(true));
        }
        if self.disabled {
            obj.insert("disabled".into(), serde_json::Value::Bool(true));
        }
        if !self.cached_version.is_empty() {
            obj.insert(
                "cachedVersion".into(),
                serde_json::Value::String(self.cached_version.clone()),
            );
        }
        if !self.cached_name.is_empty() {
            obj.insert(
                "cachedName".into(),
                serde_json::Value::String(self.cached_name.clone()),
            );
        }
        if !self.cached_requires.is_empty() {
            // Prism stores requirements in a `RequireSet` ordered by uid, so
            // serialization is sorted by uid regardless of insertion order.
            let mut reqs = self.cached_requires.clone();
            reqs.sort_by(|a, b| a.uid.cmp(&b.uid));
            obj.insert(
                "cachedRequires".into(),
                serde_json::Value::Array(reqs.iter().map(Require::to_json).collect()),
            );
        }
        if !self.cached_conflicts.is_empty() {
            let mut confs = self.cached_conflicts.clone();
            confs.sort_by(|a, b| a.uid.cmp(&b.uid));
            obj.insert(
                "cachedConflicts".into(),
                serde_json::Value::Array(confs.iter().map(Require::to_json).collect()),
            );
        }
        if self.cached_volatile {
            obj.insert("cachedVolatile".into(), serde_json::Value::Bool(true));
        }
        serde_json::Value::Object(obj)
    }

    /// Parse per `componentFromJsonV1` (read `volatile`, not
    /// `cachedVolatile`). Missing `uid` is an error, matching
    /// `Json::requireString`.
    pub fn from_json(value: &serde_json::Value, path: &Path) -> Result<Component> {
        let obj = value
            .as_object()
            .ok_or_else(|| Error::json(path, "Component must be an object"))?;
        let uid = obj
            .get("uid")
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::MissingField {
                path: path.to_path_buf(),
                field: "uid",
            })?
            .to_string();
        let get_bool = |key: &str| obj.get(key).and_then(|v| v.as_bool()).unwrap_or(false);
        let get_str = |key: &str| {
            obj.get(key)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string()
        };
        let get_reqs = |key: &str| {
            obj.get(key)
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(Require::from_json)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default()
        };
        Ok(Component {
            uid,
            version: get_str("version"),
            dependency_only: get_bool("dependencyOnly"),
            important: get_bool("important"),
            disabled: get_bool("disabled"),
            cached_version: get_str("cachedVersion"),
            cached_name: get_str("cachedName"),
            cached_requires: get_reqs("cachedRequires"),
            cached_conflicts: get_reqs("cachedConflicts"),
            // Prism quirk: reads "volatile", writes "cachedVolatile".
            cached_volatile: get_bool("volatile"),
        })
    }
}

/// The component list of an instance (`PackProfile`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PackProfile {
    components: Vec<Component>,
}

impl PackProfile {
    /// Parse `mmc-pack.json` text.
    pub fn from_text(text: &str, path: &Path) -> Result<PackProfile> {
        let root: serde_json::Value =
            serde_json::from_str(text).map_err(|e| Error::json(path, e.to_string()))?;
        let obj = root
            .as_object()
            .ok_or_else(|| Error::json(path, "root must be an object"))?;
        let version = obj
            .get("formatVersion")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| Error::MissingField {
                path: path.to_path_buf(),
                field: "formatVersion",
            })?;
        if version != 1 {
            return Err(Error::format(
                path,
                format!("Invalid component file version, expected 1 (found {version})"),
            ));
        }
        let items = obj
            .get("components")
            .and_then(|v| v.as_array())
            .ok_or_else(|| Error::MissingField {
                path: path.to_path_buf(),
                field: "components",
            })?;
        let mut components: Vec<Component> = Vec::with_capacity(items.len());
        for item in items {
            let comp = Component::from_json(item, path)?;
            if components.iter().any(|c| c.uid == comp.uid) {
                continue; // "Ignoring duplicate component entry"
            }
            components.push(comp);
        }
        Ok(PackProfile { components })
    }

    /// Load from disk.
    pub fn load(path: &Path) -> Result<PackProfile> {
        let text = crate::util::read_text(path)?;
        PackProfile::from_text(&text, path)
    }

    /// Serialize exactly like `savePackProfile`.
    pub fn to_text(&self) -> String {
        let mut root = serde_json::Map::new();
        root.insert(
            "components".into(),
            serde_json::Value::Array(self.components.iter().map(Component::to_json).collect()),
        );
        root.insert("formatVersion".into(), serde_json::Value::from(1));
        json::to_document_string(&serde_json::Value::Object(root))
            .unwrap_or_else(|_| "{\"components\":[],\"formatVersion\":1}\n".to_string())
    }

    /// Save to disk atomically.
    pub fn save(&self, path: &Path) -> Result<()> {
        crate::util::atomic_write(path, self.to_text().as_bytes())
    }

    /// Components in file order (this is the resolution order basis).
    pub fn components(&self) -> &[Component] {
        &self.components
    }

    /// Mutable components in file order.
    pub fn components_mut(&mut self) -> &mut [Component] {
        &mut self.components
    }

    /// Lookup by uid.
    pub fn get(&self, uid: &str) -> Option<&Component> {
        self.components.iter().find(|c| c.uid == uid)
    }

    /// Mutable lookup by uid.
    pub fn get_mut(&mut self, uid: &str) -> Option<&mut Component> {
        self.components.iter_mut().find(|c| c.uid == uid)
    }

    /// Insert at `index`; silently ignored when the uid already exists
    /// (`PackProfile::insertComponent`) or when the uid is empty.
    pub fn insert(&mut self, index: usize, component: Component) {
        if component.uid.is_empty() || self.components.iter().any(|c| c.uid == component.uid) {
            return;
        }
        let index = index.min(self.components.len());
        self.components.insert(index, component);
    }

    /// Append a component.
    pub fn append(&mut self, component: Component) {
        self.insert(self.components.len(), component);
    }

    /// Remove a component by uid (only when removable, like
    /// `PackProfile::remove`). Returns whether anything was removed.
    pub fn remove(&mut self, uid: &str) -> bool {
        match self.get(uid) {
            Some(c) if c.is_removable() => {
                self.components.retain(|c| c.uid != uid);
                true
            }
            _ => false,
        }
    }

    /// Set (or create) a component version (`PackProfile::setComponentVersion`).
    /// Returns true when the profile changed.
    pub fn set_version(&mut self, uid: &str, version: &str, important: bool) -> bool {
        if let Some(c) = self.get_mut(uid) {
            c.version = version.to_string();
            c.important = important;
            true
        } else {
            self.append(Component {
                uid: uid.to_string(),
                version: version.to_string(),
                important,
                ..Default::default()
            });
            true
        }
    }

    /// Recognized mod loader for a uid, if any.
    pub fn modloader_of(uid: &str) -> Option<ModLoader> {
        ModLoader::from_uid(uid)
    }

    /// Enabled mod loaders present in the profile
    /// (`PackProfile::getModLoaders`).
    pub fn mod_loaders(&self) -> Vec<ModLoader> {
        self.components
            .iter()
            .filter(|c| c.is_enabled())
            .filter_map(|c| ModLoader::from_uid(&c.uid))
            .collect()
    }

    /// A minimal vanilla profile: one component, `net.minecraft` pinned to
    /// `version`, marked important.
    ///
    /// Prism writes a second slot beside it -- `org.lwjgl3` -- because its own
    /// `net.minecraft` file has no LWJGL entries at all: it serves them as a
    /// component of their own and names that component in a `requires`. Mojang's
    /// file keeps the libraries (56 of them for 1.21.4, plain and natives both),
    /// so a reader of piston has nothing to put in that slot -- it would name the
    /// same jars a second time, under the two naming schemes. An instance that
    /// already carries the slot, written by Prism or by a build of this launcher
    /// from before it read piston, resolves it as a component with nothing behind
    /// it instead of failing ([`crate::resolve`]), and Prism adds the slot back
    /// for itself when it opens a profile whose `net.minecraft` requires one.
    pub fn vanilla(version: &str) -> PackProfile {
        let mut p = PackProfile::default();
        p.append(Component {
            uid: "net.minecraft".into(),
            version: version.to_string(),
            important: true,
            ..Default::default()
        });
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GOLDEN: &str = concat!(
        "{\n",
        "    \"components\": [\n",
        "        {\n",
        "            \"cachedName\": \"Minecraft\",\n",
        "            \"cachedVersion\": \"1.21.1\",\n",
        "            \"important\": true,\n",
        "            \"uid\": \"net.minecraft\"\n",
        "        },\n",
        "        {\n",
        "            \"cachedName\": \"Fabric Loader\",\n",
        "            \"cachedVersion\": \"0.16.5\",\n",
        "            \"important\": true,\n",
        "            \"uid\": \"net.fabricmc.fabric-loader\"\n",
        "        }\n",
        "    ],\n",
        "    \"formatVersion\": 1\n",
        "}\n"
    );

    #[test]
    fn save_matches_prism_byte_for_byte() {
        let mut p = PackProfile::default();
        p.append(Component {
            uid: "net.minecraft".into(),
            important: true,
            cached_version: "1.21.1".into(),
            cached_name: "Minecraft".into(),
            ..Default::default()
        });
        p.append(Component {
            uid: "net.fabricmc.fabric-loader".into(),
            important: true,
            cached_version: "0.16.5".into(),
            cached_name: "Fabric Loader".into(),
            ..Default::default()
        });
        assert_eq!(p.to_text(), GOLDEN);
    }

    #[test]
    fn load_round_trips_and_enforces_presence_rules() {
        let p = PackProfile::from_text(GOLDEN, Path::new("t")).unwrap();
        assert_eq!(p.components().len(), 2);
        let mc = p.get("net.minecraft").unwrap();
        assert_eq!(mc.version, "");
        assert_eq!(mc.cached_version, "1.21.1");
        assert!(mc.important && !mc.disabled && !mc.dependency_only && !mc.cached_volatile);

        // save -> load identity
        let again = PackProfile::from_text(&p.to_text(), Path::new("t")).unwrap();
        assert_eq!(p, again);
    }

    #[test]
    fn volatile_read_quirk_is_reproduced() {
        // Prism writes "cachedVolatile" but READS "volatile": a file with
        // "cachedVolatile": true must parse as false, and one with
        // "volatile": true must parse as true.
        let written = PackProfile::from_text(
            "{\"formatVersion\":1,\"components\":[{\"uid\":\"a\",\"cachedVolatile\":true}]}",
            Path::new("t"),
        )
        .unwrap();
        assert!(!written.get("a").unwrap().cached_volatile);
        let legacy = PackProfile::from_text(
            "{\"formatVersion\":1,\"components\":[{\"uid\":\"a\",\"volatile\":true}]}",
            Path::new("t"),
        )
        .unwrap();
        assert!(legacy.get("a").unwrap().cached_volatile);
        // ...and the written form uses cachedVolatile again
        assert!(legacy.to_text().contains("\"cachedVolatile\": true"));
        assert!(!legacy.to_text().contains("\"volatile\": true"));
    }

    #[test]
    fn duplicate_uid_first_wins() {
        let p = PackProfile::from_text(
            "{\"formatVersion\":1,\"components\":[{\"uid\":\"a\",\"version\":\"1\"},{\"uid\":\"a\",\"version\":\"2\"}]}",
            Path::new("t"),
        )
        .unwrap();
        assert_eq!(p.components().len(), 1);
        assert_eq!(p.get("a").unwrap().version, "1");
    }

    #[test]
    fn wrong_format_version_and_missing_uid_are_errors() {
        let err = PackProfile::from_text("{\"formatVersion\":2,\"components\":[]}", Path::new("t"))
            .unwrap_err();
        assert!(matches!(err, Error::Format { .. }));
        let err = PackProfile::from_text(
            "{\"formatVersion\":1,\"components\":[{\"version\":\"1\"}]}",
            Path::new("t"),
        )
        .unwrap_err();
        assert!(matches!(err, Error::MissingField { field: "uid", .. }));
        let err = PackProfile::from_text("not json", Path::new("t")).unwrap_err();
        assert!(matches!(err, Error::Json { .. }));
        assert!(matches!(
            PackProfile::from_text("[]", Path::new("t")),
            Err(Error::Json { .. })
        ));
    }

    #[test]
    fn requires_serialize_with_equals_and_suggests_keys() {
        let mut c = Component {
            uid: "x".into(),
            ..Default::default()
        };
        c.cached_requires = vec![
            Require {
                uid: "org.lwjgl3".into(),
                suggests: "3.3.2".into(),
                equals_version: String::new(),
            },
            Require {
                uid: "net.minecraft".into(),
                equals_version: "1.21.1".into(),
                suggests: String::new(),
            },
        ];
        let json = c.to_json();
        let reqs = json["cachedRequires"].as_array().unwrap();
        // sorted because serde_json Map is a BTreeMap: keys alphabetical per object
        assert_eq!(reqs[0]["uid"], "net.minecraft");
        assert_eq!(reqs[0]["equals"], "1.21.1");
        assert!(reqs[0].get("suggests").is_none());
        assert_eq!(reqs[1]["uid"], "org.lwjgl3");
        assert_eq!(reqs[1]["suggests"], "3.3.2");
        assert!(reqs[1].get("equals").is_none());
    }

    #[test]
    fn component_semantics_match_prism() {
        let mut c = Component {
            uid: "x".into(),
            important: true,
            ..Default::default()
        };
        assert!(!c.is_removable());
        assert!(c.can_be_disabled()); // important but not dependencyOnly
        c.disabled = true;
        assert!(c.is_enabled()); // important => cannot be disabled effectively
        c.important = false;
        assert!(!c.is_enabled());
        c.dependency_only = true;
        assert!(c.is_enabled()); // dependencyOnly => cannot be disabled
    }

    #[test]
    fn profile_mutations_follow_prism_rules() {
        let mut p = PackProfile::vanilla("1.21.1");
        assert_eq!(p.components().len(), 1);
        assert_eq!(p.components()[0].uid, "net.minecraft");
        // important components are not removable
        assert!(!p.remove("net.minecraft"));
        // a non-important component is removable
        p.append(Component {
            uid: "custom.thing".into(),
            ..Default::default()
        });
        assert!(p.remove("custom.thing"));
    }

    #[test]
    fn insert_ignores_duplicates_and_empty_uids() {
        let mut p = PackProfile::default();
        p.append(Component {
            uid: "a".into(),
            ..Default::default()
        });
        p.append(Component {
            uid: "a".into(),
            version: "2".into(),
            ..Default::default()
        });
        assert_eq!(p.components().len(), 1);
        p.insert(
            0,
            Component {
                uid: String::new(),
                ..Default::default()
            },
        );
        assert_eq!(p.components().len(), 1);
        p.insert(
            99,
            Component {
                uid: "b".into(),
                ..Default::default()
            },
        ); // clamps to end
        assert!(p.get("b").is_some());
        p.insert(
            0,
            Component {
                uid: "c".into(),
                ..Default::default()
            },
        );
        assert_eq!(p.components()[0].uid, "c");
    }

    #[test]
    fn set_version_creates_or_updates() {
        let mut p = PackProfile::default();
        assert!(p.set_version("net.minecraft", "1.20.4", true));
        assert_eq!(p.get("net.minecraft").unwrap().version, "1.20.4");
        assert!(p.set_version("net.minecraft", "1.21.1", false));
        let c = p.get("net.minecraft").unwrap();
        assert_eq!(c.version, "1.21.1");
        assert!(!c.important);
    }

    #[test]
    fn mod_loaders_and_conflicts_match_known_table() {
        assert_eq!(
            ModLoader::from_uid("net.fabricmc.fabric-loader"),
            Some(ModLoader::Fabric)
        );
        assert_eq!(ModLoader::from_uid("unknown"), None);
        assert_eq!(
            ModLoader::NeoForge.conflicting_uids(),
            &[
                "net.minecraftforge",
                "net.fabricmc.fabric-loader",
                "org.quiltmc.quilt-loader"
            ]
        );
        assert!(ModLoader::LiteLoader.conflicting_uids().is_empty());
        let mut p = PackProfile::default();
        p.append(Component {
            uid: "net.fabricmc.fabric-loader".into(),
            ..Default::default()
        });
        p.append(Component {
            uid: "net.minecraftforge".into(),
            disabled: true,
            ..Default::default()
        });
        assert_eq!(p.mod_loaders(), vec![ModLoader::Fabric]);
    }

    // ---- the file on disk -------------------------------------------------

    /// `Instance::create` writes the first profile an instance has: it saves
    /// `PackProfile::vanilla(version)`, and this is that file read back through
    /// the parser that will read it for the rest of the instance's life. The
    /// bytes are spelled out because they are the contract with Prism's own
    /// reader, not an implementation detail.
    #[test]
    fn a_created_instance_writes_the_profile_this_module_reads_back() {
        let dir = tempfile::tempdir().unwrap();
        let instance = crate::instance::Instance::create(dir.path(), "Round Trip", "1.21.1")
            .expect("instance fixture");
        let loaded = PackProfile::load(&instance.mmc_pack_path()).expect("the profile it wrote");
        assert_eq!(loaded, PackProfile::vanilla("1.21.1"));
        assert_eq!(loaded.components().len(), 1);
        let component = &loaded.components()[0];
        assert_eq!(component.uid, "net.minecraft");
        assert_eq!(component.version, "1.21.1");
        assert!(component.important);
        assert!(component.cached_version.is_empty() && component.cached_name.is_empty());
        assert!(component.cached_requires.is_empty() && component.cached_conflicts.is_empty());
        assert_eq!(
            std::fs::read_to_string(instance.mmc_pack_path()).unwrap(),
            concat!(
                "{\n",
                "    \"components\": [\n",
                "        {\n",
                "            \"important\": true,\n",
                "            \"uid\": \"net.minecraft\",\n",
                "            \"version\": \"1.21.1\"\n",
                "        }\n",
                "    ],\n",
                "    \"formatVersion\": 1\n",
                "}\n"
            )
        );
    }

    /// A profile with no components still writes the document Prism reads: the
    /// two keys, in that shape, with an inline empty array and a trailing
    /// newline.
    #[test]
    fn an_empty_profile_writes_the_empty_document() {
        let text = PackProfile::default().to_text();
        assert_eq!(text, "{\n    \"components\": [],\n    \"formatVersion\": 1\n}\n");
        // and the document it wrote loads back as the profile it was
        assert_eq!(
            PackProfile::from_text(&text, Path::new("t")).unwrap(),
            PackProfile::default()
        );
    }

    /// Unknown keys are not an error: a component file is written by other
    /// launchers and other versions of this one, and a reader that refused a key
    /// it does not model would refuse the whole instance. Writing is a
    /// normalisation, so the unknown keys are gone from the output.
    #[test]
    fn unknown_fields_are_ignored_on_the_root_and_on_a_component() {
        let p = PackProfile::from_text(
            r#"{"formatVersion":1,"somethingElse":{"a":1},
                "components":[{"uid":"net.minecraft","version":"1.21.1","icon":"grass",
                               "name":"Minecraft","cachedIcon":"grass"}]}"#,
            Path::new("t"),
        )
        .expect("unknown fields are not an error");
        assert_eq!(p.components().len(), 1);
        assert_eq!(p.get("net.minecraft").unwrap().version, "1.21.1");
        let written = p.to_text();
        assert!(!written.contains("icon"), "{written}");
        assert!(!written.contains("somethingElse"), "{written}");
    }

    /// Every field but `uid` is optional on read, and a component that only
    /// names one is one this launcher can resolve.
    #[test]
    fn a_component_with_only_a_uid_reads_as_all_defaults() {
        let p = PackProfile::from_text(
            r#"{"formatVersion":1,"components":[{"uid":"a"}]}"#,
            Path::new("t"),
        )
        .unwrap();
        assert_eq!(
            p.components()[0],
            Component {
                uid: "a".into(),
                ..Default::default()
            }
        );
    }

    /// Only `uid` is required. Everything else is read leniently -- a number
    /// where a string is expected is not a string, and a string where a boolean
    /// is expected is not a boolean -- so a file written by something careless
    /// costs the field, not the file.
    #[test]
    fn a_field_of_the_wrong_type_reads_as_absent() {
        let p = PackProfile::from_text(
            r#"{"formatVersion":1,"components":[{
                "uid":"a","version":3,"important":"true","disabled":1,
                "cachedVersion":null,"cachedRequires":{},"cachedConflicts":"x",
                "volatile":"yes"}]}"#,
            Path::new("t"),
        )
        .expect("a component whose fields are the wrong type");
        let c = p.get("a").unwrap();
        assert_eq!(c.version, "");
        assert!(!c.important && !c.disabled && !c.cached_volatile);
        assert_eq!(c.cached_version, "");
        assert!(c.cached_requires.is_empty() && c.cached_conflicts.is_empty());
    }

    /// One bad entry in a cached requirement list costs that entry, not the
    /// file: the list is a cache, and a cache that cannot be read is empty.
    #[test]
    fn requirements_that_are_not_objects_are_dropped_not_failed() {
        let p = PackProfile::from_text(
            r#"{"formatVersion":1,"components":[{"uid":"a",
                "cachedRequires":[{"uid":"one"},null,{"equals":"1.0"},"two",{"uid":7},
                                  {"uid":"two","equals":"2.0"}],
                "cachedConflicts":[{"uid":"three","suggests":"3.0"}]}]}"#,
            Path::new("t"),
        )
        .unwrap();
        let c = p.get("a").unwrap();
        assert_eq!(
            c.cached_requires,
            vec![
                Require::uid("one"),
                Require {
                    uid: "two".into(),
                    equals_version: "2.0".into(),
                    suggests: String::new(),
                },
            ]
        );
        assert_eq!(
            c.cached_conflicts,
            vec![Require {
                uid: "three".into(),
                equals_version: String::new(),
                suggests: "3.0".into(),
            }]
        );
    }

    /// A component that is not an object stops the parse with the message
    /// Prism's own reader gives, and a `uid` that is not a string is a missing
    /// field rather than an empty one.
    #[test]
    fn a_component_that_is_not_an_object_fails_the_parse() {
        let err =
            PackProfile::from_text(r#"{"formatVersion":1,"components":[1]}"#, Path::new("t"))
                .unwrap_err();
        assert!(matches!(err, Error::Json { .. }), "{err:?}");
        assert!(
            err.to_string().contains("Component must be an object"),
            "{err}"
        );

        for uid in [r#"null"#, r#"7"#, r#"["a"]"#] {
            let err = PackProfile::from_text(
                &format!(r#"{{"formatVersion":1,"components":[{{"uid":{uid}}}]}}"#),
                Path::new("t"),
            )
            .unwrap_err();
            assert!(
                matches!(err, Error::MissingField { field: "uid", .. }),
                "{uid} -> {err:?}"
            );
        }
    }

    /// The root is a document with a *numeric* version and an *array* of
    /// components. A quoted `"1"` is a string, and the number it looks like is
    /// not the number Prism writes.
    #[test]
    fn the_root_needs_a_numeric_version_and_an_array_of_components() {
        let missing_version =
            PackProfile::from_text(r#"{"components":[]}"#, Path::new("t")).unwrap_err();
        assert!(matches!(
            missing_version,
            Error::MissingField {
                field: "formatVersion",
                ..
            }
        ));

        let quoted =
            PackProfile::from_text(r#"{"formatVersion":"1","components":[]}"#, Path::new("t"))
                .unwrap_err();
        assert!(matches!(
            quoted,
            Error::MissingField {
                field: "formatVersion",
                ..
            }
        ));

        for components in [r#"{}"#, r#""x""#, r#"null"#] {
            let err = PackProfile::from_text(
                &format!(r#"{{"formatVersion":1,"components":{components}}}"#),
                Path::new("t"),
            )
            .unwrap_err();
            assert!(
                matches!(
                    err,
                    Error::MissingField {
                        field: "components",
                        ..
                    }
                ),
                "{components} -> {err:?}"
            );
        }

        for root in ["[]", "null", "1", "\"text\""] {
            let err = PackProfile::from_text(root, Path::new("t")).unwrap_err();
            assert!(matches!(err, Error::Json { .. }), "{root} -> {err:?}");
            assert!(
                err.to_string().contains("root must be an object"),
                "{root} -> {err}"
            );
        }
    }

    /// The version error is the message a reader sees when a file from a newer
    /// launcher is opened, so it is asserted word for word rather than by
    /// variant alone. Zero and negative are unsupported in exactly the same way.
    #[test]
    fn the_unsupported_version_message_names_the_version_found() {
        let err =
            PackProfile::from_text(r#"{"formatVersion":2,"components":[]}"#, Path::new("t"))
                .unwrap_err();
        assert!(matches!(err, Error::Format { .. }), "{err:?}");
        let message = err.to_string();
        assert!(
            message.contains("Invalid component file version, expected 1 (found 2)"),
            "{message}"
        );
        assert!(message.contains("unsupported format"), "{message}");

        for version in ["0", "-1"] {
            let err = PackProfile::from_text(
                &format!(r#"{{"formatVersion":{version},"components":[]}}"#),
                Path::new("t"),
            )
            .unwrap_err();
            assert!(matches!(err, Error::Format { .. }), "{version} -> {err:?}");
        }
    }

    /// The field-presence rules, both ways round: a bare component writes its
    /// uid alone, a full one writes every key, and the full one reads back with
    /// the one field that is deliberately asymmetric.
    #[test]
    fn every_component_field_is_written_only_when_it_means_something() {
        let bare = Component {
            uid: "bare".into(),
            ..Default::default()
        };
        let keys: Vec<String> = bare
            .to_json()
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();
        assert_eq!(keys, ["uid"]);

        let full = Component {
            uid: "full".into(),
            version: "1.0".into(),
            dependency_only: true,
            important: true,
            disabled: true,
            cached_version: "1.0".into(),
            cached_name: "Full".into(),
            cached_requires: vec![Require::uid("r")],
            cached_conflicts: vec![Require::uid("c")],
            cached_volatile: true,
        };
        let json = full.to_json();
        let keys: Vec<String> = json.as_object().unwrap().keys().cloned().collect();
        assert_eq!(
            keys,
            [
                "cachedConflicts",
                "cachedName",
                "cachedRequires",
                "cachedVersion",
                "cachedVolatile",
                "dependencyOnly",
                "disabled",
                "important",
                "uid",
                "version",
            ]
        );

        // "cachedVolatile" is written and "volatile" is read, so the flag does
        // not survive its own file -- Prism's asymmetry, reproduced.
        let mut expected = full.clone();
        expected.cached_volatile = false;
        assert_eq!(
            Component::from_json(&json, Path::new("t")).unwrap(),
            expected
        );

        // an empty string is absence for the string fields, and false is absence
        // for the flags
        let quiet = Component {
            uid: "quiet".into(),
            version: String::new(),
            important: false,
            ..Default::default()
        };
        assert_eq!(quiet.to_json(), serde_json::json!({"uid": "quiet"}));
    }

    /// Save then load, with the requirements in the other order: the file is the
    /// same document either way, because writing sorts them by uid the way
    /// Prism's `RequireSet` does, and nothing else changes.
    #[test]
    fn a_profile_round_trips_through_a_file_with_its_requirements_sorted() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("mmc-pack.json");
        let mut profile = PackProfile::vanilla("1.21.1");
        profile.append(Component {
            uid: "net.fabricmc.fabric-loader".into(),
            version: "0.16.5".into(),
            cached_version: "0.16.5".into(),
            cached_name: "Fabric Loader".into(),
            cached_requires: vec![
                Require::uid("org.omni"),
                Require {
                    uid: "net.minecraft".into(),
                    equals_version: "1.21.1".into(),
                    suggests: String::new(),
                },
            ],
            cached_conflicts: vec![Require {
                uid: "optifine".into(),
                equals_version: String::new(),
                suggests: "1.0".into(),
            }],
            ..Default::default()
        });
        profile.save(&path).unwrap();

        let loaded = PackProfile::load(&path).unwrap();
        assert_eq!(loaded.to_text(), profile.to_text());
        assert_eq!(
            loaded
                .components()
                .iter()
                .map(|c| c.uid.as_str())
                .collect::<Vec<_>>(),
            ["net.minecraft", "net.fabricmc.fabric-loader"]
        );
        let loader = loaded.get("net.fabricmc.fabric-loader").unwrap();
        assert_eq!(
            loader
                .cached_requires
                .iter()
                .map(|r| r.uid.as_str())
                .collect::<Vec<_>>(),
            ["net.minecraft", "org.omni"]
        );
        assert_eq!(loader.cached_requires[0].equals_version, "1.21.1");
        assert_eq!(loader.cached_conflicts[0].suggests, "1.0");

        // the file is the indented document, not a compact one
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.starts_with("{\n    \"components\": [\n"), "{text}");
        assert!(text.ends_with("}\n"), "{text}");
    }

    /// A missing file and an unwritable folder are both reported with the path
    /// they are about -- a save never invents the folders above it.
    #[test]
    fn a_missing_file_is_an_io_error_that_names_it() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("mmc-pack.json");
        let err = PackProfile::load(&path).unwrap_err();
        assert!(matches!(err, Error::Io { .. }), "{err:?}");
        assert!(err.to_string().contains("mmc-pack.json"), "{err}");

        let nested = dir.path().join("no").join("such").join("mmc-pack.json");
        let err = PackProfile::vanilla("1.21.1").save(&nested).unwrap_err();
        assert!(matches!(err, Error::Io { .. }), "{err:?}");
    }

    /// `dependencyOnly` stops a component being disabled, not removed; only
    /// `important` controls removal, and a uid that is not there removes
    /// nothing.
    #[test]
    fn removing_a_dependency_only_component_is_allowed_and_a_missing_one_is_not() {
        let mut p = PackProfile::default();
        p.append(Component {
            uid: "dep".into(),
            dependency_only: true,
            ..Default::default()
        });
        p.append(Component {
            uid: "core".into(),
            important: true,
            ..Default::default()
        });
        assert!(p.remove("dep"));
        assert!(p.get("dep").is_none());
        assert!(!p.remove("core"));
        assert!(p.get("core").is_some());
        assert!(!p.remove("never-there"));
    }

    /// The mutable accessors are what resolution and the GUI write through:
    /// `get_mut` edits in place, `components_mut` is the same slice, and a uid
    /// that is not there has no place to edit.
    #[test]
    fn get_mut_edits_the_component_in_place() {
        let mut p = PackProfile::vanilla("1.21.1");
        p.get_mut("net.minecraft").unwrap().disabled = true;
        assert!(p.get("net.minecraft").unwrap().disabled);
        assert!(p.get_mut("nope").is_none());

        p.components_mut()[0].cached_name = "Minecraft".into();
        assert_eq!(p.get("net.minecraft").unwrap().cached_name, "Minecraft");
    }

    /// Insertion is positional, duplicate uids are ignored wherever they are
    /// inserted, and an index past the end is clamped rather than panicking.
    #[test]
    fn insert_keeps_the_order_it_is_given() {
        let mut p = PackProfile::default();
        for uid in ["a", "c"] {
            p.append(Component {
                uid: uid.into(),
                ..Default::default()
            });
        }
        p.insert(
            1,
            Component {
                uid: "b".into(),
                ..Default::default()
            },
        );
        assert_eq!(
            p.components()
                .iter()
                .map(|c| c.uid.as_str())
                .collect::<Vec<_>>(),
            ["a", "b", "c"]
        );

        p.insert(
            0,
            Component {
                uid: "c".into(),
                version: "9".into(),
                ..Default::default()
            },
        );
        assert_eq!(p.components().len(), 3);
        assert_eq!(p.get("c").unwrap().version, "");
    }

    /// The five loader uids are the ones Prism recognizes, each round-trips
    /// through `from_uid`, and each names the loaders it conflicts with and
    /// never itself.
    #[test]
    fn every_loader_uid_round_trips_and_each_names_its_rivals() {
        let loaders = [
            ModLoader::NeoForge,
            ModLoader::Forge,
            ModLoader::Fabric,
            ModLoader::Quilt,
            ModLoader::LiteLoader,
        ];
        for loader in loaders {
            assert_eq!(ModLoader::from_uid(loader.uid()), Some(loader));
            assert!(!loader.conflicting_uids().contains(&loader.uid()));
        }
        assert_eq!(
            ModLoader::Forge.conflicting_uids(),
            &[
                "net.neoforged",
                "net.fabricmc.fabric-loader",
                "org.quiltmc.quilt-loader"
            ]
        );
        assert_eq!(
            ModLoader::Fabric.conflicting_uids(),
            &[
                "net.minecraftforge",
                "net.neoforged",
                "org.quiltmc.quilt-loader"
            ]
        );
        assert_eq!(
            ModLoader::Quilt.conflicting_uids(),
            &[
                "net.minecraftforge",
                "net.neoforged",
                "net.fabricmc.fabric-loader"
            ]
        );
        assert_eq!(PackProfile::modloader_of("net.minecraft"), None);
        assert_eq!(
            PackProfile::modloader_of("org.quiltmc.quilt-loader"),
            Some(ModLoader::Quilt)
        );

        // a disabled loader is a loader the profile does not have -- but an
        // important one cannot be disabled in the first place, so it still is
        let mut p = PackProfile::default();
        p.append(Component {
            uid: "com.mumfrey.liteloader".into(),
            disabled: true,
            ..Default::default()
        });
        p.append(Component {
            uid: "net.minecraftforge".into(),
            disabled: true,
            important: true,
            ..Default::default()
        });
        assert_eq!(p.mod_loaders(), vec![ModLoader::Forge]);
    }

    /// A requirement without a uid is not a requirement; the two optional members
    /// are the empty string when absent, and are not written when empty.
    #[test]
    fn a_requirement_needs_a_uid_and_omits_the_members_it_does_not_have() {
        assert_eq!(
            Require::uid("a").to_json(),
            serde_json::json!({"uid": "a"})
        );
        let both = Require {
            uid: "a".into(),
            equals_version: "1".into(),
            suggests: "2".into(),
        };
        assert_eq!(
            both.to_json(),
            serde_json::json!({"uid": "a", "equals": "1", "suggests": "2"})
        );
        assert_eq!(
            Require::from_json(&serde_json::json!({"equals": "1"})),
            None
        );
        assert_eq!(Require::from_json(&serde_json::json!({"uid": 1})), None);
        assert_eq!(Require::from_json(&serde_json::json!("a")), None);
        let bare = Require::from_json(&serde_json::json!({"uid": "a"})).unwrap();
        assert_eq!(bare, Require::uid("a"));
        assert!(bare.to_json().get("equals").is_none());
        assert!(bare.to_json().get("suggests").is_none());
    }

    /// A cached name is whatever the metadata said, and metadata names carry
    /// quotes, backslashes and non-ASCII text. Writing goes through serde_json,
    /// which escapes what JSON requires and leaves UTF-8 alone, and the file has
    /// to read back as the same string.
    #[test]
    fn a_cached_name_with_json_metacharacters_round_trips() {
        let name = "Say \"hello\" \\ ünïcoded\tname";
        let mut profile = PackProfile::vanilla("1.21.1");
        profile.get_mut("net.minecraft").unwrap().cached_name = name.into();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("mmc-pack.json");
        profile.save(&path).unwrap();

        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("\\\"hello\\\""), "{text}");
        assert!(text.contains("\\\\"), "{text}");
        assert!(text.contains("\\t"), "{text}");
        assert!(text.contains("ünïcoded"), "{text}");
        assert_eq!(
            PackProfile::load(&path)
                .unwrap()
                .get("net.minecraft")
                .unwrap()
                .cached_name,
            name
        );
    }
}
