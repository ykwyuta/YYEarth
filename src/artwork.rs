//! Generated artwork shared by the WebGL and Direct3D editions.
//! JPEGs are embedded so offline builds need no image server or asynchronous loading.
//!
//! 画像は名前（slug）で引く。build.rs が assets/artwork/*.jpg を探して表を作るので、
//! 新しい画像は同じ名前で置いてビルドし直すだけでよい。無い画像は手描き（art.rs）で代わりに描く。
use crate::gfx::{Bitmap, Gfx, Texture};

#[allow(dead_code)]
mod gen {
    include!(concat!(env!("OUT_DIR"), "/assets_gen.rs"));
}
pub use gen::{ARTWORK, UNITS};

/// その名前の図鑑画像の番号
pub fn find(slug: &str) -> Option<usize> {
    ARTWORK.iter().position(|a| a.0 == slug)
}

/// その名前の観察用スプライトの番号
pub fn find_unit(slug: &str) -> Option<usize> {
    UNITS.iter().position(|a| a.0 == slug)
}

#[derive(Clone, Copy)]
pub enum Fit { Contain, Cover }

pub struct Artwork {
    textures: Vec<Option<Texture>>,
    attempted: Vec<bool>,
    teacher: Option<(u32, Bitmap)>,
}

impl Artwork {
    pub fn new() -> Self {
        Self {
            textures: (0..ARTWORK.len()).map(|_| None).collect(),
            attempted: vec![false; ARTWORK.len()],
            teacher: None,
        }
    }

    /// Decode and upload once, only when an image first becomes visible.
    pub fn texture(&mut self, gfx: &Gfx, index: usize) -> Option<&Texture> {
        if index >= ARTWORK.len() { return None; }
        if !self.attempted[index] {
            self.attempted[index] = true;
            if let Ok(image) = image::load_from_memory_with_format(ARTWORK[index].1, image::ImageFormat::Jpeg) {
                let rgba = image.to_rgba8();
                let pixels: Vec<u32> = rgba.as_raw().chunks_exact(4)
                    .map(|p| u32::from_le_bytes([p[0], p[1], p[2], p[3]])).collect();
                self.textures[index] = gfx.create_texture(rgba.width(), rgba.height(), &pixels).ok();
            }
        }
        self.textures[index].as_ref()
    }

    pub fn texture_slug(&mut self, gfx: &Gfx, slug: &str) -> Option<&Texture> {
        let i = find(slug)?;
        self.texture(gfx, i)
    }

    /// 画像があれば描いて true。無ければ何もせず false（呼ぶ側が手描きで代わりに描く）
    pub fn draw(&mut self, gfx: &mut Gfx, slug: &str, bounds: [f32; 4], alpha: f32, fit: Fit) -> bool {
        let Some(index) = find(slug) else { return false; };
        let Some(texture) = self.texture(gfx, index) else { return false; };
        draw(gfx, texture, bounds, alpha, fit);
        true
    }

    /// Dialog portraits are drawn in the UI pass (Canvas2D / Direct2D).
    pub fn draw_teacher(&mut self, gfx: &Gfx, [x0, y0, x1, y1]: [f32; 4]) -> bool {
        if self.teacher.as_ref().map(|(generation, _)| *generation) != Some(gfx.d2d_gen) {
            let Some(i) = find("ms-toki") else { return false; };
            let Ok(image) = image::load_from_memory_with_format(ARTWORK[i].1, image::ImageFormat::Jpeg) else { return false; };
            let rgba = image.to_rgba8();
            // The bitmap API expects premultiplied BGRA; JPEG pixels are opaque.
            let pixels: Vec<u32> = rgba.as_raw().chunks_exact(4)
                .map(|p| u32::from_le_bytes([p[2], p[1], p[0], 255])).collect();
            let Some(bitmap) = gfx.create_bitmap(rgba.width(), rgba.height(), &pixels) else { return false; };
            self.teacher = Some((gfx.d2d_gen, bitmap));
        }
        if let Some((_, bitmap)) = &self.teacher {
            gfx.draw_bitmap(bitmap, x0, y0, x1, y1);
            true
        } else {
            false
        }
    }
}

/// Fit a photograph without stretching or cutting off the specimen.
pub fn draw(gfx: &mut Gfx, texture: &Texture, [x0, y0, x1, y1]: [f32; 4], alpha: f32, fit: Fit) {
    let (bw, bh) = (x1 - x0, y1 - y0);
    if bw <= 0.0 || bh <= 0.0 { return; }
    let aspect = texture.w as f32 / texture.h as f32;
    match fit {
        Fit::Contain => {
            let dw = bw.min(bh * aspect);
            let dh = dw / aspect;
            let (cx, cy) = ((x0 + x1) * 0.5, (y0 + y1) * 0.5);
            gfx.draw_textured(texture, cx - dw * 0.5, cy - dh * 0.5,
                cx + dw * 0.5, cy + dh * 0.5, [1.0, 1.0, 1.0, alpha], true);
        }
        Fit::Cover => {
            let screen_aspect = bw / bh;
            let (u, v) = if screen_aspect > aspect {
                (0.0, (1.0 - aspect / screen_aspect) * 0.5)
            } else {
                ((1.0 - screen_aspect / aspect) * 0.5, 0.0)
            };
            gfx.draw_textured_quad(texture, [(x0,y0), (x1,y0), (x1,y1), (x0,y1)],
                [(u,v), (1.0-u,v), (1.0-u,1.0-v), (u,1.0-v)],
                [1.0, 1.0, 1.0, alpha], true);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::life::{PHENOMS, SPECIES};

    /// 置いてある画像はすべて読める（無いものは手描きで代わりに描くので、無くてもよい）
    #[test]
    fn embedded_artwork_decodes() {
        for (slug, bytes) in ARTWORK {
            let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Jpeg).expect(slug);
            assert!(image.width() >= 512 && image.height() >= 480, "{}", slug);
        }
    }

    /// カンブリア紀の図鑑の画像はそろっている
    #[test]
    fn cambrian_entries_have_artwork() {
        for sp in SPECIES.iter().filter(|s| s.t0 > crate::earth::PERIODS[1].t0) {
            assert!(find(sp.slug).is_some(), "{}", sp.slug);
        }
        for p in ["gondwana", "laurentia", "eruption", "barren-land", "title", "shallow-sea", "deep-sea"] {
            assert!(find(p).is_some(), "{}", p);
        }
        assert!(PHENOMS.iter().all(|p| !p.slug.is_empty()));
    }

    /// 名前の重なりがない
    #[test]
    fn slugs_are_unique() {
        let mut v: Vec<&str> = SPECIES.iter().map(|s| s.slug).chain(PHENOMS.iter().map(|p| p.slug)).collect();
        let n = v.len();
        v.sort();
        v.dedup();
        assert_eq!(v.len(), n);
    }

    #[test]
    fn teacher_portrait_is_square_and_decodable() {
        let i = find("ms-toki").unwrap();
        let image = image::load_from_memory_with_format(ARTWORK[i].1, image::ImageFormat::Jpeg).expect("valid teacher portrait");
        assert_eq!(image.width(), image.height());
        assert!(image.width() >= 512);
    }
}
