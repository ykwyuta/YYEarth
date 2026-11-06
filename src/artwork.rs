//! Generated artwork shared by the WebGL and Direct3D editions.
//! JPEGs are embedded so offline builds need no image server or asynchronous loading.
use crate::gfx::{Bitmap, Gfx, Texture};

const TEACHER_IMAGE: &[u8] = include_bytes!("../assets/artwork/ms-toki.jpg");

const IMAGES: &[&[u8]] = &[
    include_bytes!("../assets/artwork/stromatolite.jpg"),
    include_bytes!("../assets/artwork/microbial-mat.jpg"),
    include_bytes!("../assets/artwork/treptichnus.jpg"),
    include_bytes!("../assets/artwork/anabarites.jpg"),
    include_bytes!("../assets/artwork/archaeocyath.jpg"),
    include_bytes!("../assets/artwork/lingulella.jpg"),
    include_bytes!("../assets/artwork/redlichia.jpg"),
    include_bytes!("../assets/artwork/halkieria.jpg"),
    include_bytes!("../assets/artwork/haikouichthys.jpg"),
    include_bytes!("../assets/artwork/fuxianhuia.jpg"),
    include_bytes!("../assets/artwork/isoxys.jpg"),
    include_bytes!("../assets/artwork/leanchoilia.jpg"),
    include_bytes!("../assets/artwork/hallucigenia.jpg"),
    include_bytes!("../assets/artwork/amplectobelua.jpg"),
    include_bytes!("../assets/artwork/kerygmachela.jpg"),
    include_bytes!("../assets/artwork/tamisiocaris.jpg"),
    include_bytes!("../assets/artwork/anomalocaris.jpg"),
    include_bytes!("../assets/artwork/opabinia.jpg"),
    include_bytes!("../assets/artwork/wiwaxia.jpg"),
    include_bytes!("../assets/artwork/marrella.jpg"),
    include_bytes!("../assets/artwork/pikaia.jpg"),
    include_bytes!("../assets/artwork/ottoia.jpg"),
    include_bytes!("../assets/artwork/aysheaia.jpg"),
    include_bytes!("../assets/artwork/olenoides.jpg"),
    include_bytes!("../assets/artwork/vauxia.jpg"),
    include_bytes!("../assets/artwork/sidneyia.jpg"),
    include_bytes!("../assets/artwork/nectocaris.jpg"),
    include_bytes!("../assets/artwork/burgessomedusa.jpg"),
    include_bytes!("../assets/artwork/cambroraster.jpg"),
    include_bytes!("../assets/artwork/paradoxides.jpg"),
    include_bytes!("../assets/artwork/olenus.jpg"),
    include_bytes!("../assets/artwork/agnostus.jpg"),
    include_bytes!("../assets/artwork/climactichnites.jpg"),
    include_bytes!("../assets/artwork/conodont.jpg"),
    include_bytes!("../assets/artwork/radiolarian.jpg"),
    include_bytes!("../assets/artwork/glass-sponge.jpg"),
    include_bytes!("../assets/artwork/gondwana.jpg"),
    include_bytes!("../assets/artwork/laurentia.jpg"),
    include_bytes!("../assets/artwork/eruption.jpg"),
    include_bytes!("../assets/artwork/barren-land.jpg"),
    include_bytes!("../assets/artwork/title.jpg"),
    include_bytes!("../assets/artwork/shallow-sea.jpg"),
    include_bytes!("../assets/artwork/deep-sea.jpg"),
];

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
            textures: (0..IMAGES.len()).map(|_| None).collect(),
            attempted: vec![false; IMAGES.len()],
            teacher: None,
        }
    }

    /// Decode and upload once, only when an image first becomes visible.
    pub fn texture(&mut self, gfx: &Gfx, index: usize) -> Option<&Texture> {
        if index >= IMAGES.len() { return None; }
        if !self.attempted[index] {
            self.attempted[index] = true;
            if let Ok(image) = image::load_from_memory_with_format(IMAGES[index], image::ImageFormat::Jpeg) {
                let rgba = image.to_rgba8();
                let pixels: Vec<u32> = rgba.as_raw().chunks_exact(4)
                    .map(|p| u32::from_le_bytes([p[0], p[1], p[2], p[3]])).collect();
                self.textures[index] = gfx.create_texture(rgba.width(), rgba.height(), &pixels).ok();
            }
        }
        self.textures[index].as_ref()
    }

    pub fn draw(&mut self, gfx: &mut Gfx, index: usize, bounds: [f32; 4], alpha: f32, fit: Fit) -> bool {
        let Some(texture) = self.texture(gfx, index) else { return false; };
        draw(gfx, texture, bounds, alpha, fit);
        true
    }

    /// Dialog portraits are drawn in the UI pass (Canvas2D / Direct2D).
    pub fn draw_teacher(&mut self, gfx: &Gfx, [x0, y0, x1, y1]: [f32; 4]) -> bool {
        if self.teacher.as_ref().map(|(generation, _)| *generation) != Some(gfx.d2d_gen) {
            let Ok(image) = image::load_from_memory_with_format(TEACHER_IMAGE, image::ImageFormat::Jpeg) else { return false; };
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
    #[test]
    fn every_guide_entry_has_decodable_artwork() {
        assert_eq!(IMAGES.len(), crate::life::n_entries() + 3);
        for bytes in IMAGES {
            let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Jpeg).expect("valid artwork");
            assert!(image.width() >= 768 && image.height() >= 480);
        }
    }

    #[test]
    fn teacher_portrait_is_square_and_decodable() {
        let image = image::load_from_memory_with_format(TEACHER_IMAGE, image::ImageFormat::Jpeg).expect("valid teacher portrait");
        assert_eq!(image.width(), image.height());
        assert!(image.width() >= 512);
    }
}
