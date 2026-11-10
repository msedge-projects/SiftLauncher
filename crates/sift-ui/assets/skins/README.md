# The reference's default skin textures

Forty-five 64x64 PNGs, extracted from Modrinth App 0.22.0's own source, where they are embedded as base64 data URLs in
`packages/app-lib/src/api/minecraft_skins/assets/default/default_skins.rs` (1 249 lines). They are Mojang's bundled skins and
Modrinth ships them with the app, so drawing them is reproducing the reference's own artwork rather than approximating it.

Extracted once with a regex over the `texture_key` / `name` / `data:image/png;base64,` triples, and named by their catalogue names
so the page can order them as the reference's sections do (`Default skins`, `Modrinth Pride`, `MINECON Earth 2017`, ...).

The Skins page draws these as flat stand-ins today, which is the page's largest single difference (144 609 pixels). A front view of a
Minecraft character is its texture's own faces -- head 8,8 to 16,16; body 20,20 to 28,32; arms and legs at their UV rectangles -- so
a paper-doll composition from these files is the next step, not a 3D renderer.