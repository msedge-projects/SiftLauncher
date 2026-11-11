//! **The reference's eight default skins, by name.**
//!
//! These are the textures the app's own `Default skins` grid shows. They are not embedded in the reference's source either:
//! `default_skins.rs` lists them as URLs to Mojang's texture CDN keyed by hash, and each was fetched from
//! `textures.minecraft.net/texture/<key>` and stored here so the grid draws without a network.
//!
//! **What was here before was the wrong set.** An earlier extraction took the first forty-five entries of that file -- the
//! `Mr_Pack_*` pride-flag variants -- none of which the page asks for, so `by_name` never matched and the grid fell through to
//! the flat fallback atlases. That is what eight solid colour blocks looked like.

/// Each default skin's 64x64 PNG, keyed by the name the grid asks for.
pub static BAKED: [(&str, &[u8]); 8] = [
    ("Alex", include_bytes!("../assets/skins/default_01_Alex.png")),
    ("Ari", include_bytes!("../assets/skins/default_02_Ari.png")),
    ("Efe", include_bytes!("../assets/skins/default_03_Efe.png")),
    ("Kai", include_bytes!("../assets/skins/default_04_Kai.png")),
    ("Makena", include_bytes!("../assets/skins/default_05_Makena.png")),
    ("Noor", include_bytes!("../assets/skins/default_06_Noor.png")),
    ("Steve", include_bytes!("../assets/skins/default_07_Steve.png")),
    ("Sunny", include_bytes!("../assets/skins/default_08_Sunny.png")),
];

/// The bytes of a named default skin, if this build carries it.
pub fn by_name(name: &str) -> Option<&'static [u8]> {
    BAKED.iter().find(|(n, _)| *n == name).map(|(_, bytes)| *bytes)
}
