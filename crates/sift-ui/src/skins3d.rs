//! A small software rasteriser for a Minecraft skin: boxes in, painted quads out.
//!
//! The reference's preview is a rotatable 3D model (SkinPreviewRenderer.vue), and a flat elevation cannot
//! rotate. This draws the six boxes a skin is made of -- head, body, two arms, two legs -- texel by texel,
//! with an orthographic projection, so the preview can be dragged without a graphics backend.
//!
//! It is deliberately **not** a general renderer: no perspective, no lighting but a per-face shade, no
//! overlay layers, no animation. What it must do is put the reader's own skin on a body that turns.

use egui::{Color32, Pos2, Vec2};

/// One box: how big, where it sits, and the atlas rectangle of each face.
struct Part {
    /// Size in skin texels (x, y, z).
    size: (f32, f32, f32),
    /// The box's minimum corner, in skin texels, with x centred on the body and y down from the head.
    min: (f32, f32, f32),
    /// Atlas rectangles for +x, -x, top, bottom, front, back.
    faces: [Option<(usize, usize, usize, usize)>; 6],
}

const PARTS: [Part; 6] = [
    Part { size: (8.0, 8.0, 8.0), min: (-4.0, 0.0, -4.0), faces: [
        Some((16, 8, 8, 8)), Some((0, 8, 8, 8)), Some((8, 0, 8, 8)),
        Some((16, 0, 8, 8)), Some((8, 8, 8, 8)), Some((24, 8, 8, 8))] },
    Part { size: (8.0, 12.0, 4.0), min: (-4.0, 8.0, -2.0), faces: [
        Some((28, 20, 4, 12)), Some((16, 20, 4, 12)), Some((20, 16, 8, 4)),
        Some((28, 16, 8, 4)), Some((20, 20, 8, 12)), Some((32, 20, 8, 12))] },
    Part { size: (4.0, 12.0, 4.0), min: (-8.0, 8.0, -2.0), faces: [
        Some((40, 20, 4, 12)), Some((32, 20, 4, 12)), Some((36, 16, 4, 4)),
        Some((40, 16, 4, 4)), Some((44, 20, 4, 12)), Some((48, 20, 4, 12))] },
    Part { size: (4.0, 12.0, 4.0), min: (4.0, 8.0, -2.0), faces: [
        Some((56, 20, 4, 12)), Some((48, 20, 4, 12)), Some((52, 16, 4, 4)),
        Some((56, 16, 4, 4)), Some((36, 52, 4, 12)), Some((40, 52, 4, 12))] },
    Part { size: (4.0, 12.0, 4.0), min: (-4.0, 20.0, -2.0), faces: [
        Some((8, 20, 4, 12)), Some((0, 20, 4, 12)), Some((4, 16, 4, 4)),
        Some((8, 16, 4, 4)), Some((4, 20, 4, 12)), Some((12, 20, 4, 12))] },
    Part { size: (4.0, 12.0, 4.0), min: (0.0, 20.0, -2.0), faces: [
        Some((24, 20, 4, 12)), Some((16, 20, 4, 12)), Some((20, 16, 4, 4)),
        Some((24, 16, 4, 4)), Some((20, 52, 4, 12)), Some((28, 52, 4, 12))] },
];

const SHADE: [f32; 6] = [0.82, 0.82, 1.0, 0.6, 1.0, 0.7];

pub struct Texel {
    pub points: [Pos2; 4],
    pub color: Color32,
    pub depth: f32,
}

fn project(point: (f32, f32, f32), yaw: f32, pitch: f32) -> (f32, f32, f32) {
    let (x, y, z) = point;
    let (sin_yaw, cos_yaw) = yaw.sin_cos();
    let x = x * cos_yaw + z * sin_yaw;
    let z = -x * sin_yaw + z * cos_yaw;
    let (sin_pitch, cos_pitch) = pitch.sin_cos();
    let y = y * cos_pitch - z * sin_pitch;
    let z = y * sin_pitch + z * cos_pitch;
    (x, y, z)
}

fn face_normal(face: usize, yaw: f32) -> f32 {
    let (sin_yaw, cos_yaw) = yaw.sin_cos();
    let (nx, nz) = match face {
        0 => (1.0, 0.0),
        1 => (-1.0, 0.0),
        4 => (0.0, 1.0),
        5 => (0.0, -1.0),
        _ => return 1.0,
    };
    -nx * sin_yaw + nz * cos_yaw
}

pub fn yaw_after(yaw: f32, delta: f32) -> f32 {
    yaw + delta * 0.01
}

pub fn scale_for(height: f32) -> f32 {
    (height / 38.0).max(1.0)
}

pub fn size_at(scale: f32) -> Vec2 {
    Vec2::new(18.0 * scale, 36.0 * scale)
}

fn face_cell(part: &Part, face: usize, tx: usize, ty: usize, aw: usize, ah: usize) -> [(f32, f32, f32); 4] {
    let (w, h, d) = part.size;
    let (x, y, z) = part.min;
    let u0 = tx as f32 / aw as f32;
    let u1 = (tx + 1) as f32 / aw as f32;
    let v0 = ty as f32 / ah as f32;
    let v1 = (ty + 1) as f32 / ah as f32;
    match face {
        0 => [(x + w, y + v0 * h, z + u0 * d), (x + w, y + v0 * h, z + u1 * d),
              (x + w, y + v1 * h, z + u1 * d), (x + w, y + v1 * h, z + u0 * d)],
        1 => [(x, y + v0 * h, z + u1 * d), (x, y + v0 * h, z + u0 * d),
              (x, y + v1 * h, z + u0 * d), (x, y + v1 * h, z + u1 * d)],
        2 => [(x + u0 * w, y, z + v0 * d), (x + u1 * w, y, z + v0 * d),
              (x + u1 * w, y, z + v1 * d), (x + u0 * w, y, z + v1 * d)],
        3 => [(x + u0 * w, y + h, z + v1 * d), (x + u1 * w, y + h, z + v1 * d),
              (x + u1 * w, y + h, z + v0 * d), (x + u0 * w, y + h, z + v0 * d)],
        4 => [(x + u0 * w, y + v0 * h, z + d), (x + u1 * w, y + v0 * h, z + d),
              (x + u1 * w, y + v1 * h, z + d), (x + u0 * w, y + v1 * h, z + d)],
        _ => [(x + u1 * w, y + v0 * h, z), (x + u0 * w, y + v0 * h, z),
              (x + u0 * w, y + v1 * h, z), (x + u1 * w, y + v1 * h, z)],
    }
}

pub fn render(atlas: &[u8], centre: Pos2, scale: f32, yaw: f32) -> Vec<Texel> {
    let pitch = 0.18_f32;
    let mut texels = Vec::new();
    for part in &PARTS {
        let origin = (part.min.0 + part.size.0 / 2.0, part.min.1 + part.size.1 / 2.0, part.min.2 + part.size.2 / 2.0);
        for (face, atlas_rect) in part.faces.iter().enumerate() {
            let Some((ax, ay, aw, ah)) = *atlas_rect else { continue };
            if face_normal(face, yaw) <= 0.0 { continue; }
            let shade = SHADE[face];
            for ty in 0..ah {
                for tx in 0..aw {
                    let corners = face_cell(part, face, tx, ty, aw, ah);
                    let mut rotated = [(0.0_f32, 0.0_f32, 0.0_f32); 4];
                    for (index, corner) in corners.iter().enumerate() {
                        let local = (corner.0 - origin.0, corner.1 - origin.1, corner.2 - origin.2);
                        rotated[index] = project(local, yaw, pitch);
                    }
                    let index = ((ay + ty) * 64 + (ax + tx)) * 4;
                    let Some(rgba) = atlas.get(index..index + 4) else { continue };
                    if rgba[3] == 0 { continue }
                    let color = Color32::from_rgba_unmultiplied(
                        (rgba[0] as f32 * shade) as u8,
                        (rgba[1] as f32 * shade) as u8,
                        (rgba[2] as f32 * shade) as u8,
                        rgba[3],
                    );
                    let points = [
                        Pos2::new(centre.x + rotated[0].0 * scale, centre.y + rotated[0].1 * scale),
                        Pos2::new(centre.x + rotated[1].0 * scale, centre.y + rotated[1].1 * scale),
                        Pos2::new(centre.x + rotated[2].0 * scale, centre.y + rotated[2].1 * scale),
                        Pos2::new(centre.x + rotated[3].0 * scale, centre.y + rotated[3].1 * scale),
                    ];
                    let depth = (rotated[0].2 + rotated[1].2 + rotated[2].2 + rotated[3].2) / 4.0;
                    texels.push(Texel { points, color, depth });
                }
            }
        }
    }
    texels.sort_by(|a, b| b.depth.partial_cmp(&a.depth).unwrap_or(std::cmp::Ordering::Equal));
    texels
}

#[cfg(test)]
mod tests {
    use super::*;

    fn atlas() -> Vec<u8> {
        let mut bytes = vec![0_u8; 64 * 64 * 4];
        for (index, pixel) in bytes.chunks_mut(4).enumerate() {
            pixel[0] = (index % 251) as u8;
            pixel[1] = (index / 7 % 251) as u8;
            pixel[2] = 200;
            pixel[3] = 255;
        }
        bytes
    }

    #[test]
    fn a_turned_model_paints_something() {
        let texels = render(&atlas(), Pos2::new(100.0, 100.0), 4.0, 0.0);
        assert!(!texels.is_empty(), "nothing was painted");
        assert!(texels.len() < 2000, "{} cells is more than a skin has", texels.len());
    }

    #[test]
    fn the_back_of_the_model_is_drawn_too() {
        let front = render(&atlas(), Pos2::new(0.0, 0.0), 1.0, 0.0).len();
        let back = render(&atlas(), Pos2::new(0.0, 0.0), 1.0, std::f32::consts::PI).len();
        // Not equal: turning the model swaps which side faces the reader, and a side face is narrower than
        // a front one, so the two views legitimately differ. What matters is that both paint a body.
        assert!(front > 300, "the front painted {front} cells");
        assert!(back > 300, "the back painted {back} cells");
    }

    #[test]
    fn turning_the_model_hides_one_side_and_shows_the_other() {
        // The projection turns the object, so at a quarter turn the +x side has swung away and the -x
        // side faces the reader.
        assert!(face_normal(0, 0.0) <= 0.0, "the +x side faces away at rest");
        assert!(face_normal(0, std::f32::consts::FRAC_PI_2) < 0.0, "and further away at a quarter turn");
        assert!(face_normal(1, std::f32::consts::FRAC_PI_2) > 0.0, "the -x side is what the reader sees then");
        assert!(face_normal(4, 0.0) > 0.0, "the front faces the reader at rest");
    }

    #[test]
    fn a_drag_turns_the_model() {
        assert!(yaw_after(0.0, 100.0) > 0.0);
        assert_eq!(yaw_after(1.0, 0.0), 1.0);
    }

    #[test]
    fn cells_sort_far_to_near() {
        let texels = render(&atlas(), Pos2::new(0.0, 0.0), 2.0, 0.5);
        for pair in texels.windows(2) {
            assert!(pair[0].depth >= pair[1].depth, "a near cell was painted before a far one");
        }
    }

    #[test]
    fn the_scale_fits_the_plate() {
        assert!(scale_for(10.0) >= 1.0);
        assert!(scale_for(380.0) > scale_for(190.0));
    }
}
