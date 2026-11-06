//! Alpha sprites with species-specific mesh animation, shared by both editions.
use crate::art;
use crate::gfx::{Gfx, Texture, Vertex};
use crate::artwork::{find_unit, UNITS};
use crate::life::{Art, Beh, SPECIES};

pub struct Units {
    textures: Vec<Option<Texture>>,
    attempted: Vec<bool>,
    mesh: Vec<Vertex>,
    /// 生きものごとのスプライトの番号（無いものは手描き）
    species: Vec<Option<usize>>,
    capsule: Option<usize>,
    rock: Option<usize>,
    sand: Option<usize>,
}

#[derive(Clone, Copy)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub len: f32,
    pub facing: f32,
    pub pitch: f32,
    pub time: f32,
    pub activity: f32,
    pub out: f32,
}

/// The same bounds feed drawing, culling and photo scoring.
pub fn bounds(sp: usize, x: f32, y: f32, len: f32, out: f32) -> [f32; 4] {
    let (ew, eh) = art::extent(sp);
    let (mut w, mut h) = match find_unit(SPECIES[sp].slug) {
        Some(i) => {
            let (_, _, iw, ih) = UNITS[i];
            let w = len * ew * 1.12;
            (w, w * ih as f32 / iw as f32)
        }
        // スプライトが無いものは手描きの絵の大きさ
        None => (len * ew, len * eh.max(0.25)),
    };
    let cap = len * eh.max(0.25) * 1.65;
    if h > cap { w *= cap / h; h = cap; }
    let grounded = matches!(SPECIES[sp].beh, Beh::Sessile | Beh::Crawl | Beh::FlatCrawl | Beh::Burrow);
    if SPECIES[sp].beh == Beh::Burrow { h *= out.clamp(0.0, 1.0); }
    let top = if grounded { y - h } else { y - h * 0.5 };
    [x - w * 0.5, top, x + w * 0.5, top + h]
}

/// Keep hard shells steady while tails, limbs and soft tissues move.
fn deform(art: Art, beh: Beh, u: f32, v: f32, t: f32, activity: f32) -> (f32, f32) {
    if beh == Beh::Sessile || matches!(art, Art::Trace | Art::Mat | Art::Stromatolite) {
        return (u, v);
    }
    let wave = (t * (4.0 + activity * 3.0) - u * 10.0).sin();
    match art {
        Art::Fish(_) | Art::Nectocaris | Art::Kerygmachela | Art::Jawless(_) | Art::Placoderm(_)
        | Art::Shark(_) | Art::LobeFin(_) | Art::Tully | Art::Tetrapod(7) =>
            (u, v + wave * (1.0-u).powi(2) * 0.16),
        Art::Cephalopod(_) | Art::Graptolite => {
            // 触手・枝の先だけがゆれる
            let tip = ((u - 0.6) / 0.4).clamp(0.0, 1.0);
            (u, v + wave * tip * 0.05)
        }
        Art::Griffinfly => {
            // はねを上下に羽ばたく（体の中心から離れるほど大きく）
            let flap = (t * 22.0).sin();
            (u, 0.5 + (v - 0.5) * (1.0 - 0.45 * flap.abs()))
        }
        Art::Jelly => {
            let pulse = (t * 3.6).sin();
            let tentacles = ((v - 0.38) / 0.62).clamp(0.0, 1.0);
            (0.5 + (u-0.5) * (1.0 + pulse * 0.10) + wave * tentacles * 0.035,
             v + pulse * (1.0-v) * 0.045)
        }
        Art::Worm | Art::Slug | Art::Aysheaia =>
            (u + wave * 0.018, v + wave * 0.045 * (0.35+activity*0.65)),
        Art::Radiolarian => (u + (t*0.9).sin()*0.012, v + (t*1.1).cos()*0.012),
        _ => {
            let legs = ((v - 0.44) / 0.56).clamp(0.0, 1.0);
            let strength = 0.2 + 0.8 * activity;
            let flap = if matches!(art, Art::Radiodont(_) | Art::Opabinia) { 1.0 } else { strength };
            (u + wave * legs * 0.032 * flap, v + wave * legs * 0.065 * flap)
        }
    }
}

impl Units {
    pub fn new() -> Self {
        Self { textures: (0..UNITS.len()).map(|_|None).collect(),
            attempted: vec![false;UNITS.len()], mesh: Vec::with_capacity(18*6*6),
            species: SPECIES.iter().map(|s| find_unit(s.slug)).collect(),
            capsule: find_unit("capsule"), rock: find_unit("rock"), sand: find_unit("seabed-sand") }
    }

    fn load(&mut self, g: &Gfx, index: usize) {
        if self.attempted[index] { return; }
        self.attempted[index] = true;
        if let Ok(image) = image::load_from_memory_with_format(UNITS[index].1, image::ImageFormat::Png) {
            let rgba = image.to_rgba8();
            let pixels: Vec<u32> = rgba.as_raw().chunks_exact(4)
                .map(|p|u32::from_le_bytes([p[0],p[1],p[2],p[3]])).collect();
            self.textures[index] = g.create_texture(rgba.width(), rgba.height(), &pixels).ok();
        }
    }

    pub fn creature(&mut self, g: &mut Gfx, sp: usize, pose: Pose) -> bool {
        if SPECIES[sp].beh == Beh::Burrow && pose.out < 0.025 { return true; }
        let Some(index) = self.species[sp] else { return false; };
        self.load(g, index);
        let Some(texture) = &self.textures[index] else { return false; };
        let [x0,y0,x1,y1] = bounds(sp, pose.x, pose.y, pose.len, pose.out);
        let (w,h) = (x1-x0,y1-y0);
        let grounded = matches!(SPECIES[sp].beh, Beh::Sessile | Beh::Crawl | Beh::FlatCrawl | Beh::Burrow);
        let pitch = if grounded { 0.0 } else { -pose.pitch };
        let (sn,cs) = pitch.sin_cos();
        let vertex = |u:f32,v:f32| {
            let (du,dv)=deform(SPECIES[sp].art,SPECIES[sp].beh,u,v,pose.time,pose.activity.clamp(0.0,1.0));
            let (lx,ly)=((du-0.5)*w,(dv-0.5)*h);
            Vertex { x:pose.x + (lx*cs-ly*sn)*pose.facing,
                y:(y0+y1)*0.5 + lx*sn+ly*cs, u, v, c:[1.0,1.0,1.0,1.0] }
        };
        self.mesh.clear();
        for row in 0..6 {
            for col in 0..18 {
                let (u0,u1)=(col as f32/18.0,(col+1) as f32/18.0);
                let (v0,v1)=(row as f32/6.0,(row+1) as f32/6.0);
                self.mesh.extend_from_slice(&[vertex(u0,v0),vertex(u1,v0),vertex(u1,v1),
                    vertex(u0,v0),vertex(u1,v1),vertex(u0,v1)]);
            }
        }
        g.draw_textured_mesh(texture,&self.mesh);
        true
    }

    pub fn capsule(&mut self, g:&mut Gfx, x:f32,y:f32,r:f32,facing:f32,tilt:f32) -> bool {
        let Some(index)=self.capsule else { return false; };
        self.load(g,index);
        let Some(texture)=&self.textures[index] else { return false; };
        let w=r*3.4;
        let h=w*texture.h as f32/texture.w as f32;
        let (sn,cs)=tilt.sin_cos();
        let p=|u:f32,v:f32|(x+(u*cs-v*sn)*facing,y+u*sn+v*cs);
        g.draw_textured_quad(texture,[p(-w*0.5,-h*0.5),p(w*0.5,-h*0.5),p(w*0.5,h*0.5),p(-w*0.5,h*0.5)],
            [(0.0,0.0),(1.0,0.0),(1.0,1.0),(0.0,1.0)],[1.0,1.0,1.0,1.0],true);
        true
    }

    pub fn rock(&mut self,g:&mut Gfx,x:f32,y:f32,r:f32,tone:f32) -> bool {
        let Some(index)=self.rock else { return false; };
        self.load(g,index);
        let Some(texture)=&self.textures[index] else { return false; };
        let w=r*2.6;
        let h=w*texture.h as f32/texture.w as f32;
        let tint=0.78+tone*0.22;
        let uv=if tone>0.5 {[(1.0,0.0),(0.0,0.0),(0.0,1.0),(1.0,1.0)]}
            else {[(0.0,0.0),(1.0,0.0),(1.0,1.0),(0.0,1.0)]};
        g.draw_textured_quad(texture,[(x-w*0.5,y-h),(x+w*0.5,y-h),(x+w*0.5,y),(x-w*0.5,y)],
            uv,[tint,tint,tint,1.0],true);
        true
    }

    pub fn seabed(&mut self,g:&mut Gfx,mesh:&[Vertex]) -> bool {
        let Some(index)=self.sand else { return false; };
        self.load(g,index);
        let Some(texture)=&self.textures[index] else { return false; };
        g.draw_textured_mesh(texture,mesh);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assets_are_real_alpha_sprites() {
        for &(slug,bytes,w,h) in UNITS.iter() {
            let image=image::load_from_memory_with_format(bytes,image::ImageFormat::Png).unwrap().to_rgba8();
            assert_eq!(image.dimensions(),(w,h),"{}",slug);
            if slug!="seabed-sand" { assert!(image.pixels().any(|p|p[3]==0),"{}",slug); }
            assert!(image.pixels().any(|p|p[3]>220),"{}",slug);
        }
        for slug in ["capsule","rock","seabed-sand"] { assert!(find_unit(slug).is_some()); }
    }
    #[test]
    fn attached_life_stays_still_but_swimming_tails_move() {
        for sp in SPECIES.iter().filter(|sp|sp.beh==Beh::Sessile) {
            assert_eq!(deform(sp.art,sp.beh,0.2,0.8,0.0,1.0),deform(sp.art,sp.beh,0.2,0.8,1.0,1.0));
        }
        assert_ne!(deform(Art::Fish(0),Beh::Swim,0.1,0.5,0.0,1.0),deform(Art::Fish(0),Beh::Swim,0.1,0.5,0.5,1.0));
    }
    #[test]
    fn ground_contact_and_photo_bounds_match_sprite_footprint() {
        for (i,sp) in SPECIES.iter().enumerate() {
            let b=bounds(i,4.0,7.0,sp.disp_len(),1.0);
            assert!(b[2]>b[0] && b[3]>b[1]);
            if matches!(sp.beh,Beh::Sessile|Beh::Crawl|Beh::FlatCrawl|Beh::Burrow) { assert!((b[3]-7.0).abs()<0.00001); }
        }
    }
}
