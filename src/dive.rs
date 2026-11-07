//! 観察の場面：カプセルで海に潜る（または陸・干潟に降りる）。横から見た断面に生きものが暮らす。
//!
//! 座標は m（横 x、下向き y）。海面が y=0。深い海は縦方向を縮めて描く（`compress`）。
//! 海底は奥行きのある帯（`BAND`）として描き、海底の生きものは帯の中の奥行き z（0=奥、1=手前）に置く。

use crate::art;
use crate::earth::*;
use crate::gfx::{Gfx, Texture};
use crate::life::*;
use crate::units::{self, Pose, Units};
use crate::rng::{fbm, value_noise, Rng};

/// 海底の帯の奥行き（画面の縦、m）
pub const BAND: f32 = 0.7;
const FSTEP: f32 = 0.25;
pub const CAP_R: f32 = 0.2;

/// 実際の深さ（m）→ 場面の y
pub fn compress(d: f32) -> f32 {
    if d <= 12.0 {
        d
    } else {
        12.0 + (d - 12.0) * 0.08
    }
}

/// 場面の y → 実際の深さ（m）
pub fn decompress(y: f32) -> f32 {
    if y <= 12.0 {
        y
    } else {
        12.0 + (y - 12.0) / 0.08
    }
}

pub struct Creature {
    pub sp: usize,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub vx: f32,
    pub vy: f32,
    pub facing: f32,
    pub pitch: f32,
    pub phase: f32,
    pub seed: u32,
    pub tx: f32,
    pub ty: f32,
    pub wait: f32,
    /// 巣穴から出ている度合い（0〜1）
    pub out: f32,
    pub flee: f32,
    pub scale: f32,
    pub track_acc: f32,
}

pub struct Rock {
    pub x: f32,
    pub z: f32,
    pub r: f32,
    pub tone: f32,
}

pub struct Mote {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub s: f32,
}

pub struct Capsule {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub facing: f32,
    pub tilt: f32,
}

pub struct Scene {
    pub hab: Habitat,
    /// 実際の水深（m）
    pub depth: f32,
    pub w: f32,
    /// 海底（帯の奥の縁）の y
    floor: Vec<f32>,
    pub creatures: Vec<Creature>,
    pub rocks: Vec<Rock>,
    pub motes: Vec<Mote>,
    /// 這い跡（x, z, 向き）
    pub tracks: Vec<(f32, f32, f32)>,
    pub cap: Capsule,
    pub t: f32,
    /// 噴火の近さ（0〜1）
    pub volcanic: f32,
    /// Local strength of the Chicxulub impact, independent of volcanoes and lava.
    pub impact: f32,
    /// 酸素の少ない暗い海
    pub anoxic: bool,
    /// 陸の植物の広がり（0〜1）
    pub veg: f32,
    /// 氷河におおわれた陸
    pub icy: bool,
    /// 乾いた砂漠
    pub arid: bool,
    pub lat: f32,
    pub seed: u32,
    rng: Rng,
    /// その場にいる生きもの（種類）
    pub present: Vec<usize>,
    /// 溶岩の流れ（x0, x1, z）
    pub lava: Vec<(f32, f32, f32)>,
}

#[derive(Clone, Copy)]
pub struct View {
    pub cx: f32,
    pub cy: f32,
    /// 画面の 1m あたりの画素（UI 座標）
    pub ppm: f32,
    pub w: f32,
    pub h: f32,
}

impl View {
    pub fn x0(&self) -> f32 {
        self.cx - self.w * 0.5 / self.ppm
    }
    pub fn x1(&self) -> f32 {
        self.cx + self.w * 0.5 / self.ppm
    }
    pub fn y0(&self) -> f32 {
        self.cy - self.h * 0.5 / self.ppm
    }
    pub fn y1(&self) -> f32 {
        self.cy + self.h * 0.5 / self.ppm
    }
    pub fn to_world(&self, sx: f32, sy: f32) -> (f32, f32) {
        ((sx - self.w * 0.5) / self.ppm + self.cx, (sy - self.h * 0.5) / self.ppm + self.cy)
    }
}

fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    let t = t.clamp(0.0, 1.0);
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

fn c4(c: [f32; 3], a: f32) -> [f32; 4] {
    [c[0], c[1], c[2], a]
}

/// 写真に写った生きもの（種類、点数）
pub struct ShotResult {
    pub subjects: Vec<(usize, f32)>,
    /// 陸の景色の写真か
    pub landscape: bool,
    /// 噴火が写ったか
    pub eruption: bool,
}

/// 図鑑に収録できる写りなら 1、そうでなければ 0（星の段階はやめ、収録したかどうかだけにした）
pub fn stars_of(score: f32) -> u8 {
    if score >= 30.0 {
        1
    } else {
        0
    }
}

impl Scene {
    pub fn new(earth: &Earth, spot: &Spot, t: f32, seed: u32) -> Scene {
        let mut rng = Rng::new(seed as u64 ^ 0x9e37_79b9);
        let hab = spot.hab;
        let w = 30.0;
        let n = (w / FSTEP) as usize + 2;
        let near_site = |s: usize| deg_dist(earth.site_lonlat(s, t), (spot.lon, spot.lat)) < 7.0;
        let cliff = hab == Habitat::Shelf && near_site(S_BURGESS) && t <= SITES[S_BURGESS].t0 + 0.5 && t >= SITES[S_BURGESS].t1 - 0.5;
        let strength = |impact| ERUPTIONS.iter()
            .filter(|er| er.is_impact() == impact && t <= er.t0 + 1e-3 && t >= er.t1 - 1e-3)
            .map(|er| (1.0 - deg_dist(earth.site_lonlat(er.site, t), (spot.lon, spot.lat)) / 9.0).clamp(0.0, 1.0))
            .fold(0.0f32, f32::max);
        let volcanic = strength(false);
        let impact = strength(true);
        let land = hab == Habitat::Land;
        let icy = land && glacial(t) && spot.lat < -55.0;
        let arid = land && !icy && arid(t, spot.lat);
        let veg = if icy { 0.0 } else if arid { veg_at(t) * 0.25 } else { veg_at(t) };
        let anoxic = near_site(S_ALUM) && t <= SITES[S_ALUM].t0 + 0.5 && hab.is_water();
        let base = match hab {
            Habitat::Land => -0.4,
            Habitat::Tidal => 0.35,
            _ => compress(spot.depth),
        };
        let rough = match hab {
            Habitat::Land => 0.8,
            Habitat::Tidal => 0.3,
            Habitat::Shallow => 0.6,
            Habitat::Shelf => 0.9,
            Habitat::Deep => 1.5,
        };
        let mut floor = Vec::with_capacity(n);
        for i in 0..n {
            let x = i as f32 * FSTEP;
            let mut y = base + (fbm(x * 0.09, 3.7, seed, 4) - 0.5) * rough * 2.0 + (value_noise(x * 1.2, 1.3, seed + 7) - 0.5) * 0.06;
            if cliff && x < 7.0 {
                // 海底の崖：左端に向かって急に浅くなる
                let k = ((7.0 - x) / 3.0).clamp(0.0, 1.0);
                y -= k * k * (base - 3.0).max(0.0) * 0.8;
            }
            floor.push(y);
        }
        let cap_x = w * 0.5;
        let cap_y = match hab {
            Habitat::Land => -1.3,
            Habitat::Tidal => -0.9,
            Habitat::Deep => 5.0,
            _ => (base - 1.1).max(0.7),
        };
        let mut s = Scene {
            hab,
            depth: spot.depth,
            w,
            floor,
            creatures: Vec::new(),
            rocks: Vec::new(),
            motes: Vec::new(),
            tracks: Vec::new(),
            cap: Capsule { x: cap_x, y: cap_y, vx: 0.0, vy: 0.0, facing: 1.0, tilt: 0.0 },
            t: 0.0,
            volcanic,
            impact,
            anoxic,
            veg,
            icy,
            arid,
            lat: spot.lat,
            seed,
            rng: Rng::new(seed as u64),
            present: Vec::new(),
            lava: Vec::new(),
        };
        // 岩と小石
        let nr = match hab {
            Habitat::Land => 22,
            Habitat::Deep => 8,
            _ => 16,
        };
        for _ in 0..nr {
            s.rocks.push(Rock { x: rng.range(0.0, w), z: rng.range(0.0, 1.0), r: rng.range(0.04, 0.28) * if hab == Habitat::Land { 1.5 } else { 1.0 }, tone: rng.f32() });
        }
        s.rocks.sort_by(|a, b| a.z.partial_cmp(&b.z).unwrap());
        for _ in 0..140 {
            s.motes.push(Mote { x: rng.range(0.0, 1.0), y: rng.range(0.0, 1.0), z: rng.range(0.3, 1.0), s: rng.range(0.6, 1.6) });
        }
        if volcanic > 0.25 && !hab.is_water() {
            let k = (2.0 + volcanic * 4.0) as usize;
            for _ in 0..k {
                let x0 = rng.range(0.0, w - 4.0);
                s.lava.push((x0, x0 + rng.range(1.5, 4.5), rng.range(0.15, 0.9)));
            }
        }
        // 生きもの
        for (i, sp) in SPECIES.iter().enumerate() {
            if !lives_here(sp, earth, spot, t) {
                continue;
            }
            s.present.push(i);
            let a = abundance(sp, earth, spot, t) * if volcanic > 0.5 { 0.4 } else { 1.0 };
            let count = ((a * 1.8 * rng.range(0.7, 1.3)).round() as usize).max(1).min(18);
            for _ in 0..count {
                s.spawn(i);
            }
        }
        s
    }

    fn spawn(&mut self, sp: usize) {
        let spd = &SPECIES[sp];
        let x = self.rng.range(2.0, self.w - 2.0);
        let z = self.rng.range(0.05, 0.95);
        let f = self.floor_at(x);
        let top = self.water_top();
        let y = match spd.beh {
            Beh::Swim => self.rng.range((top + 0.3).max(f - 2.6), (f - 0.25).max(top + 0.4)).min(f - 0.2),
            Beh::Drift => {
                let lo = (top + 0.3).max(f - 4.0);
                let hi = (f - 0.5).max(lo + 0.2);
                self.rng.range(lo, hi)
            }
            Beh::NearFloor => f - self.rng.range(0.1, 0.7),
            _ => f + z * BAND,
        };
        let seed = self.rng.next_u32();
        self.creatures.push(Creature {
            sp,
            x,
            y,
            z,
            vx: 0.0,
            vy: 0.0,
            facing: if self.rng.chance(0.5) { 1.0 } else { -1.0 },
            pitch: 0.0,
            phase: self.rng.range(0.0, 100.0),
            seed,
            tx: x,
            ty: y,
            wait: self.rng.range(0.0, 3.0),
            out: if spd.beh == Beh::Burrow { self.rng.range(0.4, 1.0) } else { 1.0 },
            flee: 0.0,
            scale: self.rng.range(0.85, 1.15),
            track_acc: 0.0,
        });
    }

    /// 海底の奥の縁の y
    pub fn floor_at(&self, x: f32) -> f32 {
        let fx = (x / FSTEP).clamp(0.0, (self.floor.len() - 2) as f32);
        let i = fx as usize;
        let t = fx - i as f32;
        self.floor[i] * (1.0 - t) + self.floor[i + 1] * t
    }

    /// 水の上端（陸では空の上の方まで動ける）
    pub fn water_top(&self) -> f32 {
        match self.hab {
            Habitat::Land => -4.5,
            Habitat::Tidal => -3.0,
            _ => 0.25,
        }
    }

    pub fn has_water(&self) -> bool {
        self.hab.is_water()
    }

    /// 明るさ（0〜1）。深いほど暗く、酸素の少ない海は濁って暗い
    pub fn ambient_at(&self, y: f32) -> f32 {
        if !self.has_water() || y < 0.0 {
            return 1.0;
        }
        let d = decompress(y);
        let k = if self.anoxic { 45.0 } else { 115.0 };
        (-d / k).exp().max(0.0)
    }

    /// 深い海で、海底のすぐ上（または海面のすぐ下）へ一気に移る
    pub fn jump(&mut self, to_floor: bool) {
        let x = self.cap.x;
        self.cap.y = if to_floor { self.floor_at(x) - 1.0 } else { self.water_top() + 0.5 };
        self.cap.vx = 0.0;
        self.cap.vy = 0.0;
    }

    pub fn cap_speed(&self) -> f32 {
        (self.cap.vx * self.cap.vx + self.cap.vy * self.cap.vy).sqrt()
    }

    /// カプセルを動かす（ax, ay：-1〜1 の入力。target：指・マウスで示した場所）
    pub fn update(&mut self, dt: f32, ax: f32, ay: f32, target: Option<(f32, f32)>, boost: bool) {
        self.t += dt;
        let c = &mut self.cap;
        let (mut ix, mut iy) = (ax, ay);
        if let Some((tx, ty)) = target {
            let (dx, dy) = (tx - c.x, ty - c.y);
            let d = (dx * dx + dy * dy).sqrt();
            if d > 0.15 {
                let k = (d / 1.2).min(1.0);
                ix += dx / d * k;
                iy += dy / d * k;
            }
        }
        let l = (ix * ix + iy * iy).sqrt();
        if l > 1.0 {
            ix /= l;
            iy /= l;
        }
        let deep = decompress(c.y.max(0.0)) > 30.0;
        let top = match self.hab {
            Habitat::Land => -4.5,
            Habitat::Tidal => -3.0,
            _ => 0.25,
        };
        let vmax = if boost { 3.4 } else { 1.2 } * if deep && boost { 2.5 } else { 1.0 };
        let acc = 4.0;
        c.vx += (ix * vmax - c.vx) * (acc / vmax.max(1.0) * dt).min(1.0) * 1.6;
        c.vy += (iy * vmax - c.vy) * (acc / vmax.max(1.0) * dt).min(1.0) * 1.6;
        if ix.abs() < 0.05 && iy.abs() < 0.05 {
            let k = (1.0 - dt * 2.5).max(0.0);
            c.vx *= k;
            c.vy *= k;
        }
        c.x += c.vx * dt;
        c.y += c.vy * dt;
        if c.vx > 0.2 {
            c.facing = 1.0;
        } else if c.vx < -0.2 {
            c.facing = -1.0;
        }
        c.tilt += ((c.vx * 0.05).clamp(-0.25, 0.25) - c.tilt) * (dt * 4.0).min(1.0);
        // 場面の端・海面・海底
        let w = self.w;
        c.x = c.x.clamp(1.0, w - 1.0);
        let cx = c.x;
        if c.y < top {
            c.y = top;
            c.vy = c.vy.max(0.0);
        }
        let fl = self.floor_at(cx) + BAND * 0.45 - CAP_R;
        let fl = fl.min(self.floor_at(cx - 0.4) + BAND * 0.45 - CAP_R).min(self.floor_at(cx + 0.4) + BAND * 0.45 - CAP_R);
        let c = &mut self.cap;
        if c.y > fl {
            c.y = fl;
            c.vy = c.vy.min(0.0);
        }
        self.update_life(dt);
        // 漂う粒
        for m in &mut self.motes {
            m.y += dt * 0.004 * m.z;
            m.x += dt * 0.002 * (m.y * 40.0).sin();
            if m.y > 1.0 {
                m.y -= 1.0;
            }
        }
    }

    fn update_life(&mut self, dt: f32) {
        let (cx, cy) = (self.cap.x, self.cap.y);
        let cs = self.cap_speed();
        let top = self.water_top();
        let n = self.creatures.len();
        for i in 0..n {
            let sp = self.creatures[i].sp;
            let spd = &SPECIES[sp];
            let fl = {
                let x = self.creatures[i].x;
                self.floor_at(x)
            };
            let r1 = self.rng.f32();
            let r2 = self.rng.f32();
            let r3 = self.rng.f32();
            let w = self.w;
            let cr = &mut self.creatures[i];
            let (dx, dy) = (cr.x - cx, cr.y - cy);
            let dist = (dx * dx + dy * dy).sqrt();
            cr.phase += dt;
            match spd.beh {
                Beh::Swim | Beh::NearFloor | Beh::Drift => {
                    let near_floor = spd.beh == Beh::NearFloor;
                    let mut speed = spd.speed * 0.6;
                    let shy = spd.shy * 0.5;
                    if shy > 0.0 && dist < shy && (cs > 0.45 || dist < shy * 0.45) && cr.flee <= 0.0 {
                        // 驚いて逃げる
                        cr.flee = 2.5;
                        let ang = dy.atan2(dx) + (r1 - 0.5) * 0.8;
                        cr.tx = (cr.x + ang.cos() * 3.5).clamp(1.0, w - 1.0);
                        cr.ty = cr.y + ang.sin() * 1.2;
                        cr.wait = 0.0;
                    }
                    if cr.flee > 0.0 {
                        cr.flee -= dt;
                        speed *= 3.2;
                    }
                    let (ddx, ddy) = (cr.tx - cr.x, cr.ty - cr.y);
                    let d = (ddx * ddx + ddy * ddy).sqrt();
                    if cr.wait > 0.0 {
                        cr.wait -= dt;
                        cr.vx *= (1.0 - dt * 1.5).max(0.0);
                        cr.vy *= (1.0 - dt * 1.5).max(0.0);
                    } else if d < 0.3 {
                        cr.wait = if spd.beh == Beh::Drift { 0.0 } else { r2 * 3.0 + 0.5 };
                        let span = if near_floor { 1.6 } else { 3.0 };
                        cr.tx = (cr.x + (r1 - 0.5) * span * 2.0).clamp(1.0, w - 1.0);
                        cr.ty = if near_floor {
                            fl - 0.1 - r3 * 0.6
                        } else if spd.beh == Beh::Drift {
                            cr.y + (r3 - 0.5) * 0.8
                        } else {
                            (cr.y + (r3 - 0.5) * 1.6).max(fl - 2.8)
                        };
                    } else {
                        let k = (speed / d.max(0.01)).min(4.0);
                        cr.vx += (ddx * k - cr.vx) * (dt * 2.0).min(1.0);
                        cr.vy += (ddy * k - cr.vy) * (dt * 2.0).min(1.0);
                    }
                    if spd.beh == Beh::Drift {
                        cr.vy += (cr.phase * 2.6).sin() * 0.03 * dt;
                    }
                    cr.x += cr.vx * dt;
                    cr.y += cr.vy * dt;
                    cr.x = cr.x.clamp(1.0, w - 1.0);
                    let lo = top + 0.2;
                    let hi = fl - 0.08;
                    if cr.y < lo {
                        cr.y = lo;
                        cr.ty = cr.ty.max(lo + 0.5);
                    }
                    if cr.y > hi {
                        cr.y = hi;
                        cr.ty = cr.ty.min(hi - 0.3);
                    }
                    if cr.vx.abs() > 0.03 && spd.beh != Beh::Drift {
                        cr.facing = cr.vx.signum();
                    }
                    let target_pitch = if cr.vx.abs() + cr.vy.abs() > 0.05 { (-cr.vy / (cr.vx.abs() + 0.2)).clamp(-0.5, 0.5) } else { 0.0 };
                    cr.pitch += (target_pitch - cr.pitch) * (dt * 3.0).min(1.0);
                }
                Beh::Crawl | Beh::FlatCrawl => {
                    if cr.wait > 0.0 {
                        cr.wait -= dt;
                        cr.vx = 0.0;
                    } else {
                        let ddx = cr.tx - cr.x;
                        if ddx.abs() < 0.05 {
                            cr.wait = r2 * 4.0 + 1.0;
                            cr.tx = (cr.x + (r1 - 0.5) * 1.4).clamp(0.8, w - 0.8);
                            cr.z = (cr.z + (r3 - 0.5) * 0.3).clamp(0.05, 0.95);
                        } else {
                            cr.vx = ddx.signum() * spd.speed * 0.6;
                            cr.facing = ddx.signum();
                            cr.x += cr.vx * dt;
                            if spd.beh == Beh::FlatCrawl {
                                cr.track_acc += spd.speed * dt;
                            }
                        }
                    }
                    cr.y = fl + cr.z * BAND;
                }
                Beh::Burrow => {
                    let scared = dist < spd.shy * 0.5 && cs > 0.25;
                    if scared {
                        cr.out = (cr.out - dt * 3.0).max(0.0);
                        cr.wait = 2.5 + r1 * 2.0;
                    } else if cr.wait > 0.0 {
                        cr.wait -= dt;
                    } else {
                        cr.out = (cr.out + dt * 0.35).min(1.0);
                    }
                    cr.y = fl + cr.z * BAND;
                }
                Beh::Sessile => {
                    cr.y = fl + cr.z * BAND;
                }
            }
            if cr.track_acc > 0.06 {
                cr.track_acc = 0.0;
                let (x, z, f) = (cr.x, cr.z, cr.facing);
                self.tracks.push((x, z, f));
                if self.tracks.len() > 400 {
                    self.tracks.remove(0);
                }
            }
        }
    }

    /// 生きものの位置での明るさ（ライトを含む）
    pub fn light_at(&self, x: f32, y: f32, light: bool) -> f32 {
        let a = self.ambient_at(y);
        if light {
            let (lx, ly) = (self.cap.x + self.cap.facing * 0.4, self.cap.y);
            let d = ((x - lx).powi(2) + (y - ly).powi(2)).sqrt();
            let l = (1.0 - d / 3.2).clamp(0.0, 1.0);
            (a + l * 0.95).min(1.0)
        } else {
            a
        }
    }

    /// 写真の採点
    pub fn shoot(&self, v: &View, light: bool) -> ShotResult {
        let (vx0, vy0, vx1, vy1) = (v.x0(), v.y0(), v.x1(), v.y1());
        let (vw, vh) = (vx1 - vx0, vy1 - vy0);
        let (vcx, vcy) = (v.cx, v.cy);
        let mut best: Vec<(usize, f32)> = Vec::new();
        let blur = if self.cap_speed() > 1.0 { 0.85 } else { 1.0 };
        for cr in &self.creatures {
            let spd = &SPECIES[cr.sp];
            if spd.beh == Beh::Burrow && cr.out < 0.35 {
                continue;
            }
            let len = spd.disp_len() * cr.scale;
            let [bx0,by0,bx1,by1] = units::bounds(cr.sp, cr.x, cr.y, len, cr.out);
            let (hw,hh) = ((bx1-bx0)*0.5, (by1-by0)*0.5);
            let (cx,cy) = ((bx0+bx1)*0.5,(by0+by1)*0.5);
            if cx < vx0 || cx > vx1 || cy < vy0 || cy > vy1 {
                continue;
            }
            let ix = ((cx + hw).min(vx1) - (cx - hw).max(vx0)).max(0.0);
            let iy = ((cy + hh).min(vy1) - (cy - hh).max(vy0)).max(0.0);
            let frac = (ix * iy) / (4.0 * hw * hh).max(1e-6);
            let size = ((2.0 * hw) / vw).max((2.0 * hh) / vh * 0.8);
            let size_pts = (size / 0.36).clamp(0.0, 1.0).powf(0.8) * 55.0;
            let dcx = ((cx - vcx) / (vw * 0.5)).abs();
            let dcy = ((cy - vcy) / (vh * 0.5)).abs();
            let center_pts = (1.0 - dcx.max(dcy)).max(0.0) * 25.0;
            let inside = frac * 20.0;
            let lit = self.light_at(cx, cy, light);
            let score = (size_pts + center_pts + inside) * (0.2 + 0.8 * lit.min(1.0).powf(0.7)) * blur;
            match best.iter_mut().find(|b| b.0 == cr.sp) {
                Some(b) => b.1 = b.1.max(score),
                None => best.push((cr.sp, score)),
            }
        }
        best.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let landscape = self.hab == Habitat::Land;
        let eruption = self.impact > 0.25 || (self.volcanic > 0.25 && self.lava.iter().any(|&(a, b, _)| b > vx0 && a < vx1));
        ShotResult { subjects: best, landscape, eruption }
    }

    /// 地図の上での経緯度ではなく、場面の文字の説明
    pub fn depth_text(&self, y: f32) -> String {
        match self.hab {
            Habitat::Land => tf!("地上 {:.0}m", "{:.0} m above ground", "地面以上 {:.0}米", (self.floor_at(self.cap.x) - y).max(0.0)),
            Habitat::Tidal => {
                if y < 0.0 {
                    tf!("干潟の上 {:.1}m", "{:.1} m above the flat", "潮坪上方 {:.1}米", -y)
                } else {
                    tf!("水深 {:.1}m", "Depth {:.1} m", "水深 {:.1}米", y)
                }
            }
            _ => tf!("水深 {:.0}m", "Depth {:.0} m", "水深 {:.0}米", decompress(y.max(0.0))),
        }
    }

    // ------------------------------------------------------------ 描画

    pub fn render(&self, g: &mut Gfx, v: &View, light: bool, hide_capsule: bool, backdrop: Option<&Texture>, units: &mut Units) {
        let (sw, sh) = (v.w, v.h);
        // 場面の座標 → 画面
        let sx = 2.0 * v.ppm / sw;
        let sy = -2.0 * v.ppm / sh;
        g.set_xform(sx, sy, -v.cx * sx, -v.cy * sy);
        let (x0, y0, x1, y1) = (v.x0() - 1.0, v.y0() - 1.0, v.x1() + 1.0, v.y1() + 1.0);
        self.render_back(g, x0, y0, x1, y1);
        if let Some(texture) = backdrop {
            // Distant scenery sits behind the interactive seabed and animals.
            // Restrict marine photographs to the water below the surface.
            let by0 = if self.has_water() { y0.max(0.0) } else { y0 };
            crate::artwork::draw(g, texture, [x0, by0, x1, y1], 0.32, crate::artwork::Fit::Cover);
        }
        self.render_floor(g, x0, y0, x1, y1, units);
        // 奥の生きもの → カプセル → 手前の生きもの
        let mut order: Vec<(f32, usize)> = self.creatures.iter().enumerate().filter(|(_, c)| c.x > x0 - 2.0 && c.x < x1 + 2.0).map(|(i, c)| (self.depth_key(c), i)).collect();
        order.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut cap_drawn = false;
        for &(k, i) in &order {
            if !cap_drawn && k > 0.5 {
                if !hide_capsule {
                    self.render_capsule(g, light, units);
                }
                cap_drawn = true;
            }
            self.render_creature(g, &self.creatures[i], light, units);
        }
        if !cap_drawn && !hide_capsule {
            self.render_capsule(g, light, units);
        }
        self.render_front(g, x0, y0, x1, y1, light);
    }

    fn depth_key(&self, c: &Creature) -> f32 {
        match SPECIES[c.sp].beh {
            Beh::Swim | Beh::Drift | Beh::NearFloor => 0.5 + (c.seed % 100) as f32 * 0.0001,
            _ => c.z,
        }
    }

    fn water_cols(&self) -> ([f32; 3], [f32; 3]) {
        let warm = (1.0 - self.lat.abs() / 60.0).clamp(0.0, 1.0);
        let top = mix([0.20, 0.52, 0.66], [0.26, 0.66, 0.72], warm);
        let deep = [0.02, 0.06, 0.12];
        if self.anoxic {
            (mix(top, [0.22, 0.30, 0.26], 0.6), [0.02, 0.03, 0.03])
        } else if self.volcanic.max(self.impact) > 0.2 {
            (mix(top, [0.42, 0.40, 0.34], self.volcanic.max(self.impact) * 0.6), deep)
        } else {
            (top, deep)
        }
    }

    fn sky_cols(&self) -> ([f32; 3], [f32; 3]) {
        if self.impact > 0.2 {
            return ([0.16, 0.19, 0.25], [0.90, 0.57, 0.27]);
        }
        if self.icy {
            return ([0.52, 0.64, 0.80], [0.86, 0.90, 0.94]);
        }
        if self.arid && self.volcanic <= 0.2 {
            return ([0.36, 0.56, 0.84], [0.90, 0.80, 0.66]);
        }
        if self.volcanic > 0.2 {
            let k = self.volcanic;
            (mix([0.42, 0.62, 0.82], [0.30, 0.18, 0.14], k), mix([0.76, 0.84, 0.88], [0.86, 0.44, 0.24], k))
        } else {
            ([0.26, 0.50, 0.82], [0.72, 0.82, 0.90])
        }
    }

    fn render_back(&self, g: &mut Gfx, x0: f32, y0: f32, x1: f32, y1: f32) {
        // 空
        if y0 < 0.0 || !self.has_water() {
            let (top, hor) = self.sky_cols();
            let yh = if self.has_water() { 0.0 } else { self.floor_at((x0 + x1) * 0.5) - 0.5 };
            // 画面の上端から地平線まで色を変える（地平より下は地面が覆う）
            let sy0 = y0.min(yh - 4.0);
            let k = ((yh - y0) / 10.0).clamp(0.3, 1.0);
            let top = mix(hor, top, k);
            g.rect_vgrad(x0, sy0, x1, yh, c4(top, 1.0), c4(hor, 1.0));
            if self.impact > 0.2 {
                let hx = x0 + (x1-x0)*0.45;
                let hy = yh-0.4;
                let r = 0.45+(self.t*2.0).sin()*0.04;
                g.line((hx-3.0, hy-3.0), (hx, hy), 0.22, [1.0, 0.65, 0.18, self.impact]);
                g.radial(hx, hy, r*3.0, 32, [1.0, 0.95, 0.72, self.impact], [1.0, 0.40, 0.08, 0.0]);
                g.circle(hx, hy, r, 24, [1.0, 0.98, 0.88, self.impact]);
                let wave = 0.8+(self.t*0.4).fract()*2.5;
                g.ring(hx, yh, wave, wave-0.06, 48, [1.0, 0.87, 0.62, self.impact*0.6]);
            }
            if !self.has_water() {
                g.rect(x0, yh, x1, y1, c4(hor, 1.0));
            }
            // 太陽
            let sunx = x0 + (x1 - x0) * 0.78;
            if self.volcanic < 0.6 {
                g.radial(sunx, yh - 3.2, 1.6, 24, [1.0, 0.96, 0.82, 0.7], [1.0, 0.9, 0.7, 0.0]);
                g.circle(sunx, yh - 3.2, 0.35, 20, [1.0, 0.98, 0.9, 0.95]);
            }
            // 遠くの山（陸）や雲
            if !self.has_water() || self.hab == Habitat::Tidal {
                for layer in 0..3 {
                    let k = layer as f32;
                    // 遠いほど空の色にかすむ
                    let col = mix(hor, self.hill_col(), 0.25 + k * 0.22);
                    let par = 0.15 + 0.2 * k;
                    let amp = 0.7 + k * 0.3;
                    let mut x = x0 - 2.0;
                    let base = yh + 2.0;
                    while x < x1 + 2.0 {
                        let wx = x - (x0 + x1) * 0.5 * par;
                        let hgt = |xx: f32| 0.3 + fbm(xx * 0.12, k * 7.0, self.seed + 3, 4) * amp * 2.0 - k * 0.25;
                        let (h0, h1) = (hgt(wx), hgt(wx + 1.0));
                        g.tri((x, base), (x + 1.0, base), (x + 1.0, yh - h1), c4(col, 1.0));
                        g.tri((x, base), (x + 1.0, yh - h1), (x, yh - h0), c4(col, 1.0));
                        x += 1.0;
                    }
                }
            }
            if self.volcanic > 0.25 && !self.has_water() {
                // 遠くの火山：溶岩を噴き上げる
                for k in 0..2 {
                    let vx = x0 + (x1 - x0) * (0.25 + k as f32 * 0.5) - (x0 + x1) * 0.5 * 0.1;
                    let base = yh + 0.3;
                    let (hgt, wid) = (1.25 - k as f32 * 0.35, 2.0 - k as f32 * 0.5);
                    let col = mix(hor, [0.18, 0.12, 0.12], 0.6);
                    g.tri((vx - wid, base), (vx + wid, base), (vx + 0.25, base - hgt), c4(col, 1.0));
                    g.tri((vx - wid, base), (vx + 0.25, base - hgt), (vx - 0.25, base - hgt), c4(col, 1.0));
                    let top = (vx, base - hgt);
                    g.radial(top.0, top.1, 0.8, 20, [1.0, 0.5, 0.15, 0.7 * self.volcanic], [1.0, 0.3, 0.1, 0.0]);
                    for j in 0..14 {
                        let ph = ((self.t * 0.7 + j as f32 * 0.137 + k as f32 * 0.31) % 1.0) as f32;
                        let ang = (j as f32 * 2.3).sin() * 0.7;
                        let sp = 1.2 + (j as f32 * 1.7).cos() * 0.35;
                        let px = top.0 + ang * sp * ph;
                        let py = top.1 - sp * ph + 1.6 * ph * ph;
                        g.circle(px, py, 0.025 + 0.025 * (1.0 - ph), 8, [1.0, 0.55 + 0.3 * (1.0 - ph), 0.15, 0.9 * (1.0 - ph * 0.6)]);
                    }
                    // 山肌を流れる溶岩
                    g.line(top, (vx + wid * 0.35, base), 0.04, [1.0, 0.45, 0.1, 0.8]);
                    g.line((top.0 - 0.1, top.1), (vx - wid * 0.5, base), 0.03, [1.0, 0.45, 0.1, 0.7]);
                }
            }
            if self.volcanic > 0.2 {
                // 噴煙
                for i in 0..8 {
                    let fx = x0 + (x1 - x0) * (0.1 + i as f32 * 0.12);
                    let h = (self.t * 0.2 + i as f32).sin() * 1.5;
                    g.radial(fx, yh - 4.0 - h * 0.4, 2.5 + i as f32 % 3.0, 20, [0.24, 0.20, 0.20, 0.5 * self.volcanic], [0.3, 0.24, 0.22, 0.0]);
                }
            }
        }
        // 水
        if self.has_water() {
            let (top, deep) = self.water_cols();
            let wy0 = y0.max(0.0);
            if y1 > 0.0 {
                // 深さに応じた色（いくつかの帯に分けてグラデーション）
                let steps = 8;
                for i in 0..steps {
                    let a = wy0 + (y1 - wy0) * i as f32 / steps as f32;
                    let b = wy0 + (y1 - wy0) * (i + 1) as f32 / steps as f32;
                    let ca = mix(deep, top, self.ambient_at(a).powf(0.6));
                    let cb = mix(deep, top, self.ambient_at(b).powf(0.6));
                    g.rect_vgrad(x0, a, x1, b, c4(ca, 1.0), c4(cb, 1.0));
                }
                // 光の筋
                if y0 < 30.0 {
                    for i in 0..7 {
                        let bx = ((i as f32 * 9.7 + self.seed as f32 * 0.37) % self.w) + (self.t * 0.15 + i as f32).sin() * 0.6;
                        let a = 0.07 * (1.0 + (self.t * 0.5 + i as f32 * 2.0).sin()) * 0.5 + 0.02;
                        let len = 22.0;
                        g.tri3((bx, 0.0), (bx + 1.6, 0.0), (bx + 6.0, len), [1.0, 1.0, 0.95, a], [1.0, 1.0, 0.95, a], [1.0, 1.0, 0.95, 0.0]);
                        g.tri3((bx, 0.0), (bx + 6.0, len), (bx + 3.0, len), [1.0, 1.0, 0.95, a], [1.0, 1.0, 0.95, 0.0], [1.0, 1.0, 0.95, 0.0]);
                    }
                }
                // 海面
                if y0 < 0.6 && self.hab != Habitat::Tidal {
                    let mut x = x0;
                    while x < x1 {
                        let w0 = (x * 1.3 + self.t * 1.8).sin() * 0.06;
                        let w1 = ((x + 0.5) * 1.3 + self.t * 1.8).sin() * 0.06;
                        g.tri((x, w0), (x + 0.5, w1), (x + 0.5, w1 + 0.08), [0.9, 1.0, 1.0, 0.5]);
                        g.tri((x, w0), (x + 0.5, w1 + 0.08), (x, w0 + 0.08), [0.9, 1.0, 1.0, 0.5]);
                        x += 0.5;
                    }
                }
            }
        }
    }

    /// 遠くの山の色（岩・森・雪・砂漠）
    fn hill_col(&self) -> [f32; 3] {
        if self.icy {
            [0.88, 0.92, 0.96]
        } else if self.arid {
            [0.74, 0.46, 0.30]
        } else {
            mix([0.50, 0.36, 0.30], [0.26, 0.38, 0.24], self.veg)
        }
    }

    fn sediment_cols(&self) -> ([f32; 3], [f32; 3]) {
        match self.hab {
            Habitat::Land => {
                if self.volcanic > 0.3 {
                    ([0.24, 0.20, 0.20], [0.14, 0.12, 0.12])
                } else if self.icy {
                    ([0.92, 0.95, 0.98], [0.70, 0.76, 0.84])
                } else if self.arid {
                    ([0.86, 0.58, 0.36], [0.62, 0.38, 0.24])
                } else {
                    // 植物が広がると、むき出しの岩から湿った土と下草の色へ
                    (mix([0.66, 0.46, 0.32], [0.36, 0.40, 0.22], self.veg), mix([0.46, 0.30, 0.22], [0.28, 0.22, 0.16], self.veg))
                }
            }
            Habitat::Tidal => (mix([0.74, 0.66, 0.50], [0.46, 0.50, 0.32], self.veg * 0.7), mix([0.52, 0.44, 0.34], [0.34, 0.30, 0.22], self.veg * 0.7)),
            Habitat::Shallow => ([0.82, 0.76, 0.60], [0.56, 0.50, 0.40]),
            Habitat::Shelf => {
                if self.anoxic {
                    ([0.20, 0.20, 0.20], [0.10, 0.10, 0.11])
                } else {
                    ([0.50, 0.50, 0.46], [0.30, 0.30, 0.30])
                }
            }
            Habitat::Deep => ([0.30, 0.30, 0.32], [0.14, 0.14, 0.16]),
        }
    }

    /// 明るさを色に掛ける（深い所は暗い。ライトは別に重ねる）
    fn shade(&self, c: [f32; 3], y: f32) -> [f32; 3] {
        let (top, _) = self.water_cols();
        if !self.has_water() || y < 0.0 {
            return c;
        }
        // 暗さは最後にまとめて重ねるので、ここでは水の色を少しまぜるだけ
        let a = self.ambient_at(y);
        mix(c, mix(top, [0.0, 0.0, 0.0], 0.5), 0.25 * (1.0 - a) + 0.08)
    }

    fn render_floor(&self, g: &mut Gfx, x0: f32, _y0: f32, x1: f32, y1: f32, units: &mut Units) {
        let (sc, deepc) = self.sediment_cols();
        let mut x = (x0 / FSTEP).floor() * FSTEP;
        let bottom = y1 + 2.0;
        while x < x1 {
            let (a, b) = (x.max(0.0), (x + FSTEP).min(self.w + 3.0));
            let (fa, fb) = (self.floor_at(a), self.floor_at(b));
            // 帯の上面（奥ほど暗い）
            let back = self.shade(sc, fa);
            let front = self.shade(mix(sc, [1.0, 1.0, 1.0], 0.08), fa + BAND);
            let ca = c4(mix(back, [0.0, 0.0, 0.0], 0.18), 1.0);
            let cb = c4(front, 1.0);
            g.tri3((a, fa), (b, fb), (b, fb + BAND), ca, ca, cb);
            g.tri3((a, fa), (b, fb + BAND), (a, fa + BAND), ca, cb, cb);
            // 断面
            let s0 = c4(self.shade(mix(sc, deepc, 0.5), fa + BAND), 1.0);
            let s1 = c4(self.shade(deepc, fa + BAND + 3.0), 1.0);
            g.tri3((a, fa + BAND), (b, fb + BAND), (b, bottom), s0, s0, s1);
            g.tri3((a, fa + BAND), (b, bottom), (a, bottom), s0, s1, s1);
            x += FSTEP;
        }
        // Dedicated continuous sand texture, anchored to the world and terrain slope.
        // 陸と干潟では、その時代の地面の色（森の土・雪・赤い砂）に染める
        // （植物のない時代はこれまでどおりの砂の色）
        let tint = {
            let k = if !matches!(self.hab, Habitat::Land | Habitat::Tidal) { 0.0 } else if self.icy || self.arid { 1.0 } else { self.veg };
            let base = [0.80, 0.72, 0.56];
            let to = [(sc[0] / base[0]).min(1.25) * 0.94, (sc[1] / base[1]).min(1.25) * 0.94, (sc[2] / base[2]).min(1.25) * 0.94];
            let c = mix([0.92, 0.94, 0.91], to, k);
            [c[0], c[1], c[2], 0.88 + 0.04 * k]
        };
        // 雪の上では砂の絵を使わず、雪の色で塗る
        let textured = !self.icy && {
            let mut vertices = Vec::new();
            let tile = 4.0;
            let mut start = (x0 / tile).floor() * tile;
            while start < x1 {
                let mut a = start.max(x0);
                let end = (start + tile).min(x1);
                while a < end {
                    let b = (a + FSTEP).min(end);
                    let (fa,fb) = (self.floor_at(a),self.floor_at(b));
                    let (u0,u1) = ((a-start)/tile,(b-start)/tile);
                    let vertex=|x,y,u,v|crate::gfx::Vertex {x,y,u,v,c:tint};
                    vertices.extend_from_slice(&[
                        vertex(a,fa,u0,0.0),vertex(b,fb,u1,0.0),vertex(b,fb+BAND,u1,1.0),
                        vertex(a,fa,u0,0.0),vertex(b,fb+BAND,u1,1.0),vertex(a,fa+BAND,u0,1.0)]);
                    a=b;
                }
                start+=tile;
            }
            units.seabed(g,&vertices)
        };
        // Keep procedural sand detail as a fallback for missing scenery.
        if !textured {
            let mut x = (x0 / 0.18).floor() * 0.18;
            while x < x1 {
                let i = (x / 0.18).round() as i32;
                for k in 0..3 {
                    let h = crate::rng::hash2(i, k, self.seed);
                    if h < 0.45 {
                        continue;
                    }
                    let z = crate::rng::hash2(i, k + 7, self.seed);
                    let px = x + crate::rng::hash2(i, k + 13, self.seed) * 0.18;
                    let y = self.floor_at(px) + z * BAND;
                    let r = (0.008 + h * 0.02) * (0.6 + z * 0.6);
                    let tone = crate::rng::hash2(i, k + 19, self.seed);
                    let c = self.shade(mix(sc, if tone > 0.5 { [1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0] }, 0.18 + (tone - 0.5).abs() * 0.3), y);
                    g.rect(px - r * 1.4, y - r * 0.6, px + r * 1.4, y + r * 0.6, c4(c, 0.9));
                }
                x += 0.18;
            }
        }
        // 地層のすじ
        for k in 1..6 {
            let mut x = (x0 / 1.0).floor();
            let dy = BAND + k as f32 * 0.55;
            let col = c4(self.shade(mix(sc, deepc, 0.3 + k as f32 * 0.1), self.floor_at(x0) + dy), 0.5);
            while x < x1 {
                let w0 = (x * 0.4 + k as f32).sin() * 0.08;
                g.line((x, self.floor_at(x) + dy + w0), (x + 1.0, self.floor_at(x + 1.0) + dy + ((x + 1.0) * 0.4 + k as f32).sin() * 0.08), 0.05, col);
                x += 1.0;
            }
        }
        // 波の跡（浅い海）
        if !textured && matches!(self.hab, Habitat::Shallow | Habitat::Tidal) {
            let mut x = x0.floor();
            while x < x1 {
                for r in 0..3 {
                    let z = 0.2 + r as f32 * 0.3;
                    let y = self.floor_at(x) + z * BAND;
                    g.line((x, y), (x + 0.5, y + 0.04), 0.03, c4(self.shade(mix(sc, [1.0, 1.0, 1.0], 0.2), y), 0.5));
                }
                x += 0.7;
            }
        }
        // 干潟の水たまり
        if self.hab == Habitat::Tidal {
            let mut x = x0.floor();
            while x < x1 {
                let f = self.floor_at(x);
                if f > 0.0 {
                    // 水面（干潟の上に薄く水が乗る）
                    g.rect(x, 0.0, x + 1.0, f + BAND * 0.6, [0.40, 0.70, 0.78, 0.28]);
                }
                x += 1.0;
            }
        }
        // 溶岩
        for &(a, b, z) in &self.lava {
            if b < x0 || a > x1 {
                continue;
            }
            let mid = (a + b) * 0.5;
            g.radial(mid, self.floor_at(mid) + z * BAND, (b - a) * 0.6, 24, [1.0, 0.4, 0.1, 0.3], [1.0, 0.3, 0.1, 0.0]);
            let step = 0.06;
            let mut x = a;
            while x < b {
                let f0 = ((x - a) / (b - a) * std::f32::consts::PI).sin();
                let f1 = ((x + step - a) / (b - a) * std::f32::consts::PI).sin().max(0.0);
                let wob = |xx: f32| (self.t * 1.5 + xx * 3.0).sin() * 0.015;
                let (y0a, y0b) = (self.floor_at(x) + z * BAND + wob(x), self.floor_at(x + step) + z * BAND + wob(x + step));
                let (wa, wb) = (0.03 + 0.07 * f0, 0.03 + 0.07 * f1);
                let hot = [1.0, 0.45 + 0.15 * (self.t * 2.0 + x * 5.0).sin(), 0.08, 0.95];
                g.tri((x, y0a - wa), (x + step, y0b - wb), (x + step, y0b + wb), hot);
                g.tri((x, y0a - wa), (x + step, y0b + wb), (x, y0a + wa), hot);
                let core = [1.0, 0.88, 0.45, 0.95];
                g.tri((x, y0a - wa * 0.35), (x + step, y0b - wb * 0.35), (x + step, y0b + wb * 0.35), core);
                g.tri((x, y0a - wa * 0.35), (x + step, y0b + wb * 0.35), (x, y0a + wa * 0.35), core);
                x += step;
            }
        }
        // 這い跡（はしご状）
        for &(x, z, f) in &self.tracks {
            if x < x0 || x > x1 {
                continue;
            }
            let y = self.floor_at(x) + z * BAND;
            let col = c4(mix(sc, [0.2, 0.15, 0.1], 0.35), 0.7);
            g.line((x - 0.04 * f, y - 0.07), (x + 0.04 * f, y + 0.07), 0.035, col);
            g.line((x - 0.1, y - 0.08), (x + 0.1, y - 0.08), 0.02, col);
            g.line((x - 0.1, y + 0.08), (x + 0.1, y + 0.08), 0.02, col);
        }
        // 岩
        for r in &self.rocks {
            if r.x < x0 - 1.0 || r.x > x1 + 1.0 {
                continue;
            }
            let y = self.floor_at(r.x) + r.z * BAND;
            if units.rock(g,r.x,y,r.r,r.tone) {
                if self.icy {
                    // 岩に積もった雪（平たい帽子）
                    let (cx, cy, rx, ry) = (r.x, y - r.r * 0.62, r.r * 1.05, r.r * 0.26);
                    for k in 0..12 {
                        let a0 = std::f32::consts::PI * (1.0 + k as f32 / 12.0);
                        let a1 = std::f32::consts::PI * (1.0 + (k + 1) as f32 / 12.0);
                        g.tri((cx, cy + ry * 0.4), (cx + a0.cos() * rx, cy + a0.sin() * ry), (cx + a1.cos() * rx, cy + a1.sin() * ry), [0.95, 0.97, 1.0, 0.92]);
                    }
                }
                continue;
            }
            let base = if self.hab == Habitat::Land && !self.icy { mix([0.56, 0.38, 0.28], [0.40, 0.36, 0.34], r.tone) } else { mix(sc, [0.36, 0.34, 0.32], 0.5 + r.tone * 0.4) };
            let c = self.shade(base, y);
            let seed = (r.x * 100.0) as i32;
            // 下が平らな、角ばった石
            let n = 9;
            let mut pts = Vec::with_capacity(n + 1);
            for k in 0..=n {
                let a = std::f32::consts::PI * k as f32 / n as f32;
                let rr = r.r * (0.75 + 0.35 * crate::rng::hash2(seed, k as i32, 5));
                pts.push((r.x - a.cos() * rr * 1.25, y - a.sin() * rr * 0.8));
            }
            let shadow = [0.0, 0.0, 0.0, 0.22];
            g.tri((r.x - r.r * 1.4, y + r.r * 0.12), (r.x + r.r * 1.5, y + r.r * 0.12), (r.x + r.r * 1.2, y - r.r * 0.05), shadow);
            g.tri((r.x - r.r * 1.4, y + r.r * 0.12), (r.x + r.r * 1.2, y - r.r * 0.05), (r.x - r.r * 1.2, y - r.r * 0.05), shadow);
            let ctr = (r.x, y - r.r * 0.25);
            let dark = c4(mix(c, [0.0, 0.0, 0.0], 0.28), 1.0);
            let lit = c4(mix(c, [1.0, 1.0, 1.0], 0.12), 1.0);
            for k in 0..n {
                let col = if k < n / 2 { lit } else { dark };
                g.tri(ctr, pts[k], pts[k + 1], col);
            }
            g.tri(ctr, pts[n], pts[0], dark);
            g.tri(pts[0], pts[n], ctr, c4(c, 1.0));
        }
    }

    fn render_creature(&self, g: &mut Gfx, c: &Creature, light: bool, units: &mut Units) {
        let spd = &SPECIES[c.sp];
        let len = spd.disp_len() * c.scale;
        let lift = art::base_lift(c.sp);
        let y = match spd.beh {
            Beh::Sessile | Beh::Burrow | Beh::Crawl | Beh::FlatCrawl => c.y - lift * len,
            _ => c.y,
        };
        // 海底の生きものの影
        if matches!(spd.beh, Beh::Sessile | Beh::Crawl | Beh::FlatCrawl | Beh::Burrow) {
            for i in 0..20 {
                let a=i as f32/20.0*std::f32::consts::TAU;
                let b=(i+1) as f32/20.0*std::f32::consts::TAU;
                g.tri((c.x,c.y), (c.x+a.cos()*len*0.44,c.y+a.sin()*len*0.055),
                    (c.x+b.cos()*len*0.44,c.y+b.sin()*len*0.055),[0.0,0.0,0.0,0.20]);
            }
        }
        if matches!(spd.beh, Beh::NearFloor) {
            let fy = self.floor_at(c.x) + BAND * 0.5;
            let a = (0.25 - (fy - c.y) * 0.08).clamp(0.0, 0.25);
            if a > 0.01 {
                g.circle(c.x, fy, len * 0.35, 12, [0.0, 0.0, 0.0, a]);
            }
        }
        // 深い所では色を暗くする代わりに、上から暗さを重ねる（ライトの穴を開けるため）
        let anim = c.phase * if matches!(spd.beh, Beh::Crawl) && c.vx.abs() < 0.001 { 0.15 } else { 1.0 };
        let pitch = if spd.art.dorsal() { 0.0 } else { c.pitch };
        let activity = (c.vx.abs() / spd.speed.max(0.005)).clamp(0.0,1.0);
        let drawn = units.creature(g, c.sp, Pose { x:c.x, y:c.y, len, facing:c.facing,
            pitch, time:anim + (c.seed % 997) as f32 * 0.07, activity, out:c.out });
        if !drawn {
            art::draw(g, c.sp, c.x, y, len, c.facing, pitch, anim, 1.0, c.out, c.seed);
        }
        // 光る放散虫
        if spd.art == Art::Radiolarian && !light {
            g.radial(c.x, y, len * 1.6, 12, [0.6, 0.9, 1.0, 0.18], [0.6, 0.9, 1.0, 0.0]);
        }
    }

    fn render_capsule(&self, g: &mut Gfx, light: bool, units: &mut Units) {
        let c = &self.cap;
        let (x, y) = (c.x, c.y + (self.t * 1.6).sin() * 0.015);
        let r = CAP_R;
        let f = c.facing;
        // ライトの光
        if light {
            let lx = x + f * r * 0.9;
            g.tri3((lx, y), (lx + f * 2.8, y - 1.1), (lx + f * 2.8, y + 1.1), [1.0, 0.98, 0.84, 0.32], [1.0, 0.98, 0.84, 0.0], [1.0, 0.98, 0.84, 0.0]);
        }
        // 推進の泡
        if self.has_water() && self.cap_speed() > 0.3 {
            for i in 0..5 {
                let k = ((self.t * 3.0 + i as f32 * 0.37) % 1.0) as f32;
                g.circle(x - f * (r + k * 0.6), y + 0.08 - k * 0.2 + (i as f32).sin() * 0.05, 0.02 + k * 0.02, 8, [0.9, 1.0, 1.0, 0.6 * (1.0 - k)]);
            }
        }
        // 陸では下に影
        if !self.has_water() || (self.hab == Habitat::Tidal && y < 0.0) {
            let fy = self.floor_at(x) + BAND * 0.5;
            let a = (0.35 - (fy - y) * 0.04).clamp(0.05, 0.35);
            let n = 18;
            for i in 0..n {
                let a0 = i as f32 / n as f32 * std::f32::consts::TAU;
                let a1 = (i + 1) as f32 / n as f32 * std::f32::consts::TAU;
                g.tri((x, fy), (x + a0.cos() * r * 1.3, fy + a0.sin() * r * 0.3), (x + a1.cos() * r * 1.3, fy + a1.sin() * r * 0.3), [0.0, 0.0, 0.0, a * 0.6]);
            }
            // 浮かぶための光
            g.tri3((x - r * 0.5, y + r * 0.8), (x + r * 0.5, y + r * 0.8), (x, fy), [0.5, 0.9, 1.0, 0.25], [0.5, 0.9, 1.0, 0.25], [0.5, 0.9, 1.0, 0.0]);
        }
        if units.capsule(g,x,y,r,f,c.tilt) { return; }
        let tilt = c.tilt;
        let p = |u: f32, v: f32| -> (f32, f32) {
            let (s, co) = tilt.sin_cos();
            (x + (u * co - v * s) * f, y + (u * s + v * co))
        };
        // 胴（金属の輪）
        let body = [0.86, 0.88, 0.90, 1.0];
        let dark = [0.36, 0.42, 0.50, 1.0];
        let n = 20;
        let ctr = p(0.0, 0.0);
        for i in 0..n {
            let a0 = i as f32 / n as f32 * std::f32::consts::TAU;
            let a1 = (i + 1) as f32 / n as f32 * std::f32::consts::TAU;
            let q0 = p(a0.cos() * r * 1.25, a0.sin() * r);
            let q1 = p(a1.cos() * r * 1.25, a1.sin() * r);
            let col = if a0.sin() > 0.2 { dark } else { body };
            g.tri(ctr, q0, q1, col);
        }
        // 窓
        let wc = p(r * 0.35, -r * 0.1);
        g.circle(wc.0, wc.1, r * 0.62, 20, [0.10, 0.24, 0.34, 1.0]);
        g.circle(wc.0, wc.1, r * 0.52, 20, [0.30, 0.70, 0.82, 0.95]);
        let hl = p(r * 0.48, -r * 0.3);
        g.circle(hl.0, hl.1, r * 0.16, 10, [1.0, 1.0, 1.0, 0.7]);
        // 帯と灯
        let a = p(-r * 1.2, r * 0.05);
        let b = p(r * 0.95, r * 0.05);
        g.line(a, b, 0.03, [0.98, 0.62, 0.20, 1.0]);
        let lamp = p(r * 1.18, 0.0);
        g.circle(lamp.0, lamp.1, 0.035, 10, if light { [1.0, 1.0, 0.8, 1.0] } else { [0.5, 0.5, 0.4, 1.0] });
        // ひれ
        let t0 = p(-r * 1.15, -r * 0.2);
        let t1 = p(-r * 1.55, -r * 0.55);
        let t2 = p(-r * 1.0, -r * 0.55);
        g.tri(t0, t1, t2, [0.98, 0.62, 0.20, 1.0]);
        let u0 = p(-r * 1.15, r * 0.25);
        let u1 = p(-r * 1.5, r * 0.6);
        let u2 = p(-r * 0.95, r * 0.55);
        g.tri(u0, u1, u2, [0.98, 0.62, 0.20, 1.0]);
    }

    fn render_front(&self, g: &mut Gfx, x0: f32, y0: f32, x1: f32, y1: f32, light: bool) {
        // 漂う粒（マリンスノー・砂ぼこり・灰）
        let vw = x1 - x0;
        let vh = y1 - y0;
        let (col, a) = if self.volcanic > 0.2 && !self.has_water() {
            ([0.3, 0.28, 0.28], 0.7)
        } else if self.icy {
            ([1.0, 1.0, 1.0], 0.7)
        } else if self.has_water() {
            ([0.9, 0.95, 0.95], 0.45)
        } else {
            ([0.86, 0.72, 0.56], 0.35)
        };
        for m in &self.motes {
            let px = x0 + ((m.x * 97.0 + x0 * 0.3 * m.z).rem_euclid(1.0)) * vw;
            let py = y0 + ((m.y + y0 * 0.01 * m.z).rem_euclid(1.0)) * vh;
            if self.has_water() && py < 0.0 {
                continue;
            }
            let s = 0.02 * m.s * (0.5 + m.z) * (vw / 16.0).clamp(0.4, 2.0);
            g.circle(px, py, s, 6, c4(col, a * m.z));
        }
        // 深さによる暗さ（ライトのまわりは明るい）
        if self.has_water() {
            let cy = (y0 + y1) * 0.5;
            let amb = self.ambient_at(cy.max(0.0));
            let dark = ((1.0 - amb) * 0.72).clamp(0.0, 0.93);
            if dark > 0.02 {
                let col = [0.0, 0.02, 0.05, dark];
                if light {
                    let (lx, ly) = (self.cap.x + self.cap.facing * 0.8, self.cap.y);
                    let r = 2.9;
                    g.radial(lx, ly, r, 40, [0.0, 0.02, 0.05, 0.0], col);
                    let big = vw.max(vh) * 2.0 + 20.0;
                    g.ring(lx, ly, big, big - r, 40, col);
                } else {
                    g.rect(x0, y0, x1, y1, col);
                }
            }
        }
        if self.volcanic > 0.2 && !self.has_water() {
            g.rect(x0, y0, x1, y1, [0.5, 0.18, 0.08, 0.12 * self.volcanic]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_roundtrip() {
        for d in [0.0, 10.0, 25.0, 30.0, 150.0, 3000.0] {
            assert!((decompress(compress(d)) - d).abs() < 0.01);
        }
    }

    #[test]
    fn chicxulub_is_an_impact_and_never_a_lava_field() {
        let earth = Earth::new();
        let t = 66.025;
        let (lon, lat) = earth.site_lonlat(S_CHICXULUB, t);
        let sc = Scene::new(&earth, &earth.sample(lon, lat, t), t, 42);
        assert!(eruption_at(t).unwrap().is_impact());
        assert!(sc.impact > 0.99);
        assert_eq!(sc.volcanic, 0.0);
        assert!(sc.lava.is_empty());
        let view = View { cx: sc.cap.x, cy: sc.cap.y, ppm: 100.0, w: 1200.0, h: 800.0 };
        assert!(sc.shoot(&view, true).eruption);
    }

    #[test]
    fn scene_has_expected_life() {
        let e = Earth::new();
        let t = 508.0;
        let (lon, lat) = e.site_lonlat(S_BURGESS, t);
        let spot = e.sample(lon, lat, t);
        let mut s = Scene::new(&e, &spot, t, 7);
        assert!(s.present.contains(&crate::mission::SP_MARRELLA));
        assert!(s.present.contains(&crate::mission::SP_ANOMALOCARIS));
        for _ in 0..600 {
            s.update(1.0 / 60.0, 0.5, 0.2, None, false);
        }
        // カプセルが海底にめり込まない
        assert!(s.cap.y <= s.floor_at(s.cap.x) + BAND * 0.45 - CAP_R + 1e-3);
        for c in &s.creatures {
            assert!(c.x.is_finite() && c.y.is_finite());
        }
    }

    #[test]
    fn photo_scores_close_subject_higher() {
        let e = Earth::new();
        let t = 508.0;
        let (lon, lat) = e.site_lonlat(S_BURGESS, t);
        let spot = e.sample(lon, lat, t);
        let s = Scene::new(&e, &spot, t, 11);
        let c = s.creatures.iter().find(|c| c.sp == crate::mission::SP_ANOMALOCARIS).unwrap();
        let near = View { cx: c.x, cy: c.y, ppm: 1600.0, w: 1600.0, h: 900.0 };
        let far = View { cx: c.x + 3.0, cy: c.y, ppm: 100.0, w: 1600.0, h: 900.0 };
        let sn = s.shoot(&near, true).subjects.iter().find(|x| x.0 == c.sp).map(|x| x.1).unwrap_or(0.0);
        let sf = s.shoot(&far, true).subjects.iter().find(|x| x.0 == c.sp).map(|x| x.1).unwrap_or(0.0);
        assert!(sn > sf, "near {} far {}", sn, sf);
    }
}
