//! The reference's own default skin textures, embedded.
//!
//! Forty-five 64x64 PNGs extracted from Modrinth App 0.22.0 -- it ships them as base64 data URLs in
//! `packages/app-lib/src/api/minecraft_skins/assets/default/default_skins.rs`. Baked in because the page composes a front
//! view once and caches it, and a skin cannot change while the app is running.

/// Every baked skin, named by the catalogue's own name. Duplicates are the `Classic` and `Slim` variants of one skin; the
/// first in the catalogue wins, which is the `Classic` one in every case.
pub static BAKED: [(&str, &[u8]); 45] = [
    ("Mr_Pack_Asexual", include_bytes!("../assets/skins/01-Mr_Pack_Asexual.png")),
    ("Mr_Pack_Bisexual", include_bytes!("../assets/skins/02-Mr_Pack_Bisexual.png")),
    ("Mr_Pack_Genderfluid", include_bytes!("../assets/skins/03-Mr_Pack_Genderfluid.png")),
    ("Mr_Pack_Intersex", include_bytes!("../assets/skins/04-Mr_Pack_Intersex.png")),
    ("Mr_Pack_Lesbian", include_bytes!("../assets/skins/05-Mr_Pack_Lesbian.png")),
    ("Mr_Pack_MLM", include_bytes!("../assets/skins/06-Mr_Pack_MLM.png")),
    ("Mr_Pack_Nonbinary", include_bytes!("../assets/skins/07-Mr_Pack_Nonbinary.png")),
    ("Mr_Pack_Pride", include_bytes!("../assets/skins/08-Mr_Pack_Pride.png")),
    ("Mr_Pack_Transgender", include_bytes!("../assets/skins/09-Mr_Pack_Transgender.png")),
    ("Mr_Pack", include_bytes!("../assets/skins/10-Mr_Pack.png")),
    ("Christina", include_bytes!("../assets/skins/11-Christina.png")),
    ("Dani", include_bytes!("../assets/skins/12-Dani.png")),
    ("Fishy", include_bytes!("../assets/skins/13-Fishy.png")),
    ("Hal", include_bytes!("../assets/skins/14-Hal.png")),
    ("Hedwig", include_bytes!("../assets/skins/15-Hedwig.png")),
    ("Hex", include_bytes!("../assets/skins/16-Hex.png")),
    ("Jellyfish_Girl", include_bytes!("../assets/skins/17-Jellyfish_Girl.png")),
    ("Loge", include_bytes!("../assets/skins/18-Loge.png")),
    ("Paw_Prints", include_bytes!("../assets/skins/19-Paw_Prints.png")),
    ("Penguin_Onesie", include_bytes!("../assets/skins/20-Penguin_Onesie.png")),
    ("Pig_Onesie", include_bytes!("../assets/skins/21-Pig_Onesie.png")),
    ("Tim", include_bytes!("../assets/skins/22-Tim.png")),
    ("Valorie", include_bytes!("../assets/skins/23-Valorie.png")),
    ("Darian", include_bytes!("../assets/skins/24-Darian.png")),
    ("Edson", include_bytes!("../assets/skins/25-Edson.png")),
    ("Eshe", include_bytes!("../assets/skins/26-Eshe.png")),
    ("Esperanza", include_bytes!("../assets/skins/27-Esperanza.png")),
    ("Flores", include_bytes!("../assets/skins/28-Flores.png")),
    ("Gaston", include_bytes!("../assets/skins/29-Gaston.png")),
    ("Greta", include_bytes!("../assets/skins/30-Greta.png")),
    ("Gubbis", include_bytes!("../assets/skins/31-Gubbis.png")),
    ("Javier", include_bytes!("../assets/skins/32-Javier.png")),
    ("Kellen", include_bytes!("../assets/skins/33-Kellen.png")),
    ("Mats", include_bytes!("../assets/skins/34-Mats.png")),
    ("Nuru", include_bytes!("../assets/skins/35-Nuru.png")),
    ("Pim", include_bytes!("../assets/skins/36-Pim.png")),
    ("Qamar", include_bytes!("../assets/skins/37-Qamar.png")),
    ("Raven", include_bytes!("../assets/skins/38-Raven.png")),
    ("Ren", include_bytes!("../assets/skins/39-Ren.png")),
    ("Riggs", include_bytes!("../assets/skins/40-Riggs.png")),
    ("Stella", include_bytes!("../assets/skins/41-Stella.png")),
    ("Tula", include_bytes!("../assets/skins/42-Tula.png")),
    ("Valorie", include_bytes!("../assets/skins/43-Valorie.png")),
    ("Violet", include_bytes!("../assets/skins/44-Violet.png")),
    ("Big_Foot", include_bytes!("../assets/skins/45-Big_Foot.png")),
];

/// The baked texture for a catalogue name, if the reference carries one.
pub fn by_name(name: &str) -> Option<&'static [u8]> {
    BAKED.iter().find(|(n, _)| *n == name).map(|(_, bytes)| *bytes)
}