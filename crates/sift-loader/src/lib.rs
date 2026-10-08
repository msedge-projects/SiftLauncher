//! Loader and importer logic.

/// Supported Minecraft loader kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoaderKind {
    Vanilla,
    Forge,
    Fabric,
    NeoForge,
    Quilt,
}
