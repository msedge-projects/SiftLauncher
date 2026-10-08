//! Core data model and launch metadata for SiftLauncher.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// The root folder that owns versions, libraries, assets, and accounts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataRoot {
    pub path: PathBuf,
}

impl DataRoot {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn versions_dir(&self) -> PathBuf {
        self.path.join("versions")
    }

    pub fn libraries_dir(&self) -> PathBuf {
        self.path.join("libraries")
    }

    pub fn assets_dir(&self) -> PathBuf {
        self.path.join("assets")
    }
}

/// A downloadable file entry from Mojang metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Download {
    pub sha1: String,
    pub size: u64,
    pub url: String,
    #[serde(default)]
    pub path: Option<String>,
}

/// One rule within a library or launch-argument block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Rule {
    pub action: RuleAction,
    #[serde(default)]
    pub os: Option<OsRule>,
    #[serde(default)]
    pub features: HashMap<String, bool>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuleAction {
    Allow,
    Deny,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OsRule {
    pub name: Option<String>,
    pub version: Option<String>,
    pub arch: Option<String>,
}

/// Evaluate a set of rules in order; later specific rules win.
pub fn rules_match(rules: &[Rule], features: &HashMap<String, bool>) -> bool {
    if rules.is_empty() {
        return true;
    }
    let mut allowed = false;
    for rule in rules {
        let mut applies = true;
        if let Some(os) = &rule.os {
            applies &= current_os_matches(os);
        }
        if !rule.features.is_empty() {
            applies &= rule
                .features
                .iter()
                .all(|(name, expected)| features.get(name).copied().unwrap_or(false) == *expected);
        }
        if applies {
            allowed = rule.action == RuleAction::Allow;
        }
    }
    allowed
}

fn current_os_matches(rule: &OsRule) -> bool {
    match rule.name.as_deref() {
        Some(name) => name == current_os_name(),
        None => true,
    }
}

fn current_os_name() -> &'static str {
    if cfg!(windows) {
        "windows"
    } else if cfg!(target_os = "macos") {
        "osx"
    } else {
        "linux"
    }
}

/// A Minecraft library artifact, possibly a native classifier.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Library {
    pub name: String,
    #[serde(default)]
    pub downloads: Option<LibraryDownload>,
    #[serde(default)]
    pub rules: Vec<Rule>,
    #[serde(default)]
    pub natives: HashMap<String, String>,
    #[serde(default)]
    pub extract: Option<Extract>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LibraryDownload {
    pub artifact: Option<Download>,
    #[serde(default)]
    pub classifiers: HashMap<String, Download>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Extract {
    pub exclude: Vec<String>,
}

/// Launch arguments are a mix of plain strings and conditional objects.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Argument {
    Value(String),
    Object {
        #[serde(default)]
        rules: Vec<Rule>,
        value: ArgumentValue,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ArgumentValue {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Arguments {
    #[serde(default)]
    pub game: Vec<Argument>,
    #[serde(default)]
    pub jvm: Vec<Argument>,
}

/// The per-version JSON metadata served by piston-meta.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VersionInfo {
    pub id: String,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    #[serde(rename = "type", default)]
    pub kind: String,
    #[serde(rename = "releaseTime", default)]
    pub release_time: String,
    #[serde(default)]
    pub assets: String,
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
    pub downloads: HashMap<String, Download>,
    pub libraries: Vec<Library>,
    #[serde(default)]
    pub arguments: Option<Arguments>,
    #[serde(rename = "minecraftArguments", default)]
    pub minecraft_arguments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    #[serde(rename = "totalSize", default)]
    pub total_size: u64,
    pub url: String,
}

/// The object map inside an asset index file.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetsIndex {
    pub objects: HashMap<String, AssetObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssetObject {
    pub hash: String,
    pub size: u64,
}

/// The public version manifest that lists every known Minecraft version.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManifestV2 {
    pub latest: HashMap<String, String>,
    pub versions: Vec<ManifestVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ManifestVersion {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
}

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("invalid json: {0}")]
    Json(#[from] serde_json::Error),
}

impl VersionInfo {
    pub fn from_json(text: &str) -> Result<Self, CoreError> {
        Ok(serde_json::from_str(text)?)
    }
}

impl ManifestV2 {
    pub fn from_json(text: &str) -> Result<Self, CoreError> {
        Ok(serde_json::from_str(text)?)
    }

    pub fn find(&self, id: &str) -> Option<&ManifestVersion> {
        self.versions.iter().find(|version| version.id == id)
    }
}

/// Flatten conditional arguments into plain strings.
pub fn resolve_arguments(args: &[Argument], features: &HashMap<String, bool>) -> Vec<String> {
    let mut out = Vec::new();
    for arg in args {
        match arg {
            Argument::Value(value) => out.push(value.clone()),
            Argument::Object { rules, value } => {
                if !rules.is_empty() && !rules_match(rules, features) {
                    continue;
                }
                match value {
                    ArgumentValue::One(value) => out.push(value.clone()),
                    ArgumentValue::Many(values) => out.extend(values.iter().cloned()),
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_round_trips() {
        let input = r#"{
          "latest": {"release": "1.20.1"},
          "versions": [
            {
              "id": "1.20.1",
              "type": "release",
              "url": "https://example.com/1.20.1.json",
              "time": "2023-06-12T00:00:00Z",
              "releaseTime": "2023-06-12T00:00:00Z"
            }
          ]
        }"#;
        let manifest = ManifestV2::from_json(input).unwrap();
        assert_eq!(manifest.find("1.20.1").unwrap().kind, "release");
    }

    #[test]
    fn arguments_resolve() {
        let args = vec![
            Argument::Value("--width".into()),
            Argument::Object {
                rules: vec![Rule {
                    action: RuleAction::Allow,
                    os: Some(OsRule {
                        name: None,
                        version: None,
                        arch: None,
                    }),
                    features: Default::default(),
                }],
                value: ArgumentValue::One("1280".into()),
            },
        ];
        assert_eq!(
            resolve_arguments(&args, &Default::default()),
            vec!["--width", "1280"]
        );
    }
}
