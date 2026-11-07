//! The software backend: egui drawn by this crate's own rasteriser, straight into the X buffer.
//!
//! **Why this exists.** Through OpenGL on a machine without a GPU driver -- Mesa's llvmpipe -- the launcher measured
//! 111.3 MiB of resident memory, and 53 MiB of that was `libLLVM` and `libgallium` loaded to compile shaders for a CPU.
//! The measured floor for a minimal CPU-rasterised window in this container is 16.7 MiB (`tools/mem-spike`). The gate is
//! 20 MB, so the renderer had to change rather than the number.
//!
//! **How it avoids a second buffer.** The spike drew into a `tiny_skia::Pixmap` and copied it into the presentable
//! buffer, which costs two full-screen allocations: 4.94 MiB each at 1440x900. This writes *into* the presentable buffer
//! itself, so the launcher pays for one.
//!
//! The rasteriser is deliberately small: egui hands over triangles with a colour and a texture coordinate per vertex,
//! and that is exactly what is implemented -- barycentric fill, premultiplied source-over blending, nearest sampling.
//! No paths, no gradients beyond what egui tessellates, no shaders, nothing to compile at run time.

pub mod window;

pub use window::run;

use std::collections::HashMap;

use egui;
use egui::{Color32, Pos2, TextureId};

/// One texture egui has uploaded: the pixels, and how they are laid out.
struct Texture {
    /// The upload's own image, shared rather than copied: egui has already allocated these pixels, and a private
    /// `Vec<Color32>` beside them is the same picture twice -- 4 bytes a pixel for a font atlas that is already resident.
    image: std::sync::Arc<egui::ColorImage>,
    /// Bytes this texture occupies, for the probe below.
    bytes: usize,
}

/// Everything egui has asked this backend to hold, and the rasteriser that consumes it.
pub struct Painter {
    textures: HashMap<TextureId, Texture>,
    /// The colour the frame starts from. Cleared each frame because the shell does not paint every pixel.
    clear: Color32,
}

impl Painter {
    pub fn new(clear: Color32) -> Self {
        Self {
            textures: HashMap::new(),
            clear,
        }
    }

    /// Take egui's texture uploads for this frame. Freed ids are dropped.
    pub fn apply(&mut self, delta: &egui::TexturesDelta) {
        for (id, patches) in &delta.set {
            for patch in patches {
                // Every upload is colour: this egui has no separate glyph-mask variant, and the font atlas arrives as a
                // premultiplied colour image like everything else.
                let egui::ImageData::Color(colors) = &patch.image;
                let size = colors.size;
                let bytes = size[0].saturating_mul(size[1]).saturating_mul(4);
                match patch.pos {
                    // A partial update writes into the texture it already has.
                    Some([x, y]) => {
                        if let Some(texture) = self.textures.get_mut(id) {
                            // A patch (the font atlas growing, say) copies once and then writes in place.
                            let image = std::sync::Arc::make_mut(&mut texture.image);
                            for row in 0..size[1] {
                                for col in 0..size[0] {
                                    let (dx, dy) = (x + col, y + row);
                                    if dx < image.size[0] && dy < image.size[1] {
                                        image.pixels[dy * image.size[0] + dx] =
                                            colors.pixels[row * size[0] + col];
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        self.textures.insert(
                            *id,
                            Texture {
                                image: colors.clone(),
                                bytes,
                            },
                        );
                    }
                }
            }
        }
        for id in &delta.free {
            self.textures.remove(id);
        }
    }
    /// How many bytes of texture this backend is holding. Printed when SIFT_SOFT_MEM is set: a number with a name beside
    /// it is the difference between a measurement and a shrug.
    pub fn texture_bytes(&self) -> usize {
        self.textures.values().map(|texture| texture.bytes).sum()
    }

    /// How many textures are held, for the same probe.
    pub fn len(&self) -> usize {
        self.textures.len()
    }

    /// Draw one frame's primitives into an `0x00RRGGBB` buffer of `width` by `height` pixels.
    pub fn paint(
        &self,
        primitives: &[egui::ClippedPrimitive],
        width: usize,
        height: usize,
        buffer: &mut [u32],
    ) {
        let clear = self.clear;
        let packed = pack(clear, clear.a());
        for pixel in buffer.iter_mut() {
            *pixel = packed;
        }
        for clipped in primitives {
            let egui::epaint::Primitive::Mesh(mesh) = &clipped.primitive else {
                continue;
            };
            let texture = self.textures.get(&mesh.texture_id);
            let clip = clipped.clip_rect;
            let x0 = (clip.min.x.round().max(0.0) as usize).min(width);
            let y0 = (clip.min.y.round().max(0.0) as usize).min(height);
            let x1 = (clip.max.x.round().max(0.0) as usize).min(width);
            let y1 = (clip.max.y.round().max(0.0) as usize).min(height);
            if x0 >= x1 || y0 >= y1 {
                continue;
            }
            for triangle in mesh.indices.chunks_exact(3) {
                let (a, b, c) = (
                    &mesh.vertices[triangle[0] as usize],
                    &mesh.vertices[triangle[1] as usize],
                    &mesh.vertices[triangle[2] as usize],
                );
                self.triangle(a, b, c, texture, [x0, y0, x1, y1], width, height, buffer);
            }
        }
    }

    /// One triangle, filled by barycentric weights. Colour, coverage and sampling all interpolate the same way, so a
    /// rounded rectangle's antialiasing, a glyph's coverage and a photograph's pixels come out of one loop.
    #[allow(clippy::too_many_arguments)]
    fn triangle(
        &self,
        a: &egui::epaint::Vertex,
        b: &egui::epaint::Vertex,
        c: &egui::epaint::Vertex,
        texture: Option<&Texture>,
        clip: [usize; 4],
        width: usize,
        height: usize,
        buffer: &mut [u32],
    ) {
        let area = (b.pos.x - a.pos.x) * (c.pos.y - a.pos.y) - (b.pos.y - a.pos.y) * (c.pos.x - a.pos.x);
        if area.abs() < f32::EPSILON {
            return;
        }
        let min_x = a.pos.x.min(b.pos.x).min(c.pos.x).floor().max(clip[0] as f32) as isize;
        let max_x = a.pos.x.max(b.pos.x).max(c.pos.x).ceil().min(clip[2] as f32) as isize;
        let min_y = a.pos.y.min(b.pos.y).min(c.pos.y).floor().max(clip[1] as f32) as isize;
        let max_y = a.pos.y.max(b.pos.y).max(c.pos.y).ceil().min(clip[3] as f32) as isize;
        let min_x = min_x.max(0) as usize;
        let min_y = min_y.max(0) as usize;
        let max_x = (max_x.max(0) as usize).min(width);
        let max_y = (max_y.max(0) as usize).min(height);
        for y in min_y..max_y {
            let py = y as f32 + 0.5;
            for x in min_x..max_x {
                let px = x as f32 + 0.5;
                let w0 = ((b.pos.x - px) * (c.pos.y - py) - (b.pos.y - py) * (c.pos.x - px)) / area;
                let w1 = ((c.pos.x - px) * (a.pos.y - py) - (c.pos.y - py) * (a.pos.x - px)) / area;
                let w2 = 1.0 - w0 - w1;
                if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                    continue;
                }
                let mut src = [
                    a.color.r() as f32 * w0 + b.color.r() as f32 * w1 + c.color.r() as f32 * w2,
                    a.color.g() as f32 * w0 + b.color.g() as f32 * w1 + c.color.g() as f32 * w2,
                    a.color.b() as f32 * w0 + b.color.b() as f32 * w1 + c.color.b() as f32 * w2,
                    a.color.a() as f32 * w0 + b.color.a() as f32 * w1 + c.color.a() as f32 * w2,
                ];
                if let Some(texture) = texture {
                    let u = a.uv.x * w0 + b.uv.x * w1 + c.uv.x * w2;
                    let v = a.uv.y * w0 + b.uv.y * w1 + c.uv.y * w2;
                    // **Nearest sampling.** Bilinear was tried and measured: 73 660 against 73 662 differing pixels, two
                    // pixels, for four texel reads and a mix per pixel. It did not earn its cost, so the simple sampler
                    // stays -- and the note stays with it so the experiment is not run again by accident.
                    let (width_px, height_px) = (texture.image.size[0], texture.image.size[1]);
                    let tx = ((u * width_px as f32) as isize).clamp(0, width_px as isize - 1) as usize;
                    let ty = ((v * height_px as f32) as isize).clamp(0, height_px as isize - 1) as usize;
                    let texel = texture.image.pixels[ty * width_px + tx];
                    // Premultiplied texel times premultiplied vertex: a glyph is white at coverage alpha, so this is its
                    // coverage applied to the label's colour; a picture is its own colour times the tint.
                    src[0] = src[0] * texel.r() as f32 / 255.0;
                    src[1] = src[1] * texel.g() as f32 / 255.0;
                    src[2] = src[2] * texel.b() as f32 / 255.0;
                    src[3] = src[3] * texel.a() as f32 / 255.0;
                }
                let alpha = src[3] / 255.0;
                if alpha <= 0.0 {
                    continue;
                }
                let dst = buffer[y * width + x];
                let dr = ((dst >> 16) & 0xff) as f32;
                let dg = ((dst >> 8) & 0xff) as f32;
                let db = (dst & 0xff) as f32;
                // Source-over with premultiplied colour, which is the form egui hands us.
                let out_r = (src[0] + dr * (1.0 - alpha)).round().clamp(0.0, 255.0) as u32;
                let out_g = (src[1] + dg * (1.0 - alpha)).round().clamp(0.0, 255.0) as u32;
                let out_b = (src[2] + db * (1.0 - alpha)).round().clamp(0.0, 255.0) as u32;
                buffer[y * width + x] = (out_r << 16) | (out_g << 8) | out_b;
            }
        }
    }
}

/// `Color32` is premultiplied; `0x00RRGGBB` wants the result, and the buffer has no alpha channel of its own.
fn pack(color: Color32, _alpha: u8) -> u32 {
    ((color.r() as u32) << 16) | ((color.g() as u32) << 8) | color.b() as u32
}

/// Convert a window position into the point space egui lays out in.
fn point(pos: winit::dpi::PhysicalPosition<f64>, scale: f32) -> Pos2 {
    Pos2::new(pos.x as f32 / scale, pos.y as f32 / scale)
}

/// Map a winit key event onto egui's key set. Letters and digits come from the logical key, so a layout that puts "n"
/// somewhere unexpected still types "n"; the named keys are the set the launcher actually uses.
pub fn map_key(event: &winit::event::KeyEvent) -> Option<egui::Key> {
    use egui::Key;
    use winit::keyboard::{Key as Logical, NamedKey};
    match &event.logical_key {
        Logical::Character(text) => {
            let character = text.chars().next()?;
            Key::from_name(&character.to_uppercase().to_string())
        }
        Logical::Named(named) => Some(match named {
            NamedKey::Escape => Key::Escape,
            NamedKey::Enter => Key::Enter,
            NamedKey::Tab => Key::Tab,
            NamedKey::Space => Key::Space,
            NamedKey::Backspace => Key::Backspace,
            NamedKey::Delete => Key::Delete,
            NamedKey::ArrowLeft => Key::ArrowLeft,
            NamedKey::ArrowRight => Key::ArrowRight,
            NamedKey::ArrowUp => Key::ArrowUp,
            NamedKey::ArrowDown => Key::ArrowDown,
            NamedKey::Home => Key::Home,
            NamedKey::End => Key::End,
            NamedKey::PageUp => Key::PageUp,
            NamedKey::PageDown => Key::PageDown,
            _ => return None,
        }),
        _ => return None,
    }
}
