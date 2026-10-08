//! Core data model and launch metadata.

/// The data root used by SiftLauncher on this machine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataRoot {
    pub path: std::path::PathBuf,
}

impl DataRoot {
    pub fn new(path: impl Into<std::path::PathBuf>) -> Self {
        Self { path: path.into() }
    }
}
