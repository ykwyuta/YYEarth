//! 生きものの絵（三角形で手続き的に描く）
//!
//! 体の座標は u（体の長さ方向、頭が +0.5・尾が -0.5）と v（上、海底に平たく置くものは左右）で書き、
//! `Pen` が場面（または画面）の座標に移す。y は下向きが正。

use crate::gfx::Gfx;
use crate::life::{Art, SPECIES};
use crate::rng::hash2;
use std::f32::consts::{PI, TAU};

#[derive(Clone, Copy)]
pub struct Pen {
    pub x: f32,
    pub y: f32,
    /// 体長（座標の単位）
    pub s: f32,
    /// 向き（+1 右、-1 左）
    pub fx: f32,
    /// v 方向の縮め（海底に平たく置くものを斜め上から見る）
    pub sq: f32,
    /// 体の傾き（ラジアン）
    pub rot: f32,
    pub a: f32,
}

impl Pen {
    #[inline]
    pub fn p(&self, u: f32, v: f32) -> (f32, f32) {
        let (sn, cs) = self.rot.sin_cos();
        let (uu, vv) = (u * cs - v * sn, u * sn + v * cs);
        (self.x + uu * self.s * self.fx, self.y - vv * self.s * self.sq)
    }
    /// 平たく置いたものから、画面の上へ h だけ持ち上げた点
    #[inline]
    pub fn up(&self, u: f32, v: f32, h: f32) -> (f32, f32) {
        let (x, y) = self.p(u, v);
        (x, y - h * self.s)
    }
}

#[inline]
fn ca(c: [f32; 3], a: f32) -> [f32; 4] {
    [c[0], c[1], c[2], a]
}

#[inline]
fn mul(c: [f32; 3], k: f32) -> [f32; 3] {
    [c[0] * k, c[1] * k, c[2] * k]
}

#[inline]
fn mix(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t]
}

const WHITE: [f32; 3] = [1.0, 1.0, 1.0];
const INK: [f32; 3] = [0.06, 0.05, 0.05];

/// 楕円（体の座標。ang は体の座標での傾き）
fn ell(g: &mut Gfx, pen: &Pen, u: f32, v: f32, ru: f32, rv: f32, ang: f32, c: [f32; 3], a: f32) {
    let n = 18;
    let (sa, cs) = ang.sin_cos();
    let col = ca(c, a * pen.a);
    let ctr = pen.p(u, v);
    let mut prev = pen.p(u + ru * cs, v + ru * sa);
    for i in 1..=n {
        let th = i as f32 / n as f32 * TAU;
        let (st, ct) = th.sin_cos();
        let (lu, lv) = (ru * ct, rv * st);
        let q = pen.p(u + lu * cs - lv * sa, v + lu * sa + lv * cs);
        g.tri(ctr, prev, q, col);
        prev = q;
    }
}

fn circ(g: &mut Gfx, pen: &Pen, u: f32, v: f32, r: f32, c: [f32; 3], a: f32) {
    let (x, y) = pen.p(u, v);
    g.circle(x, y, (r * pen.s).max(0.0005), 12, ca(c, a * pen.a));
}

/// 多角形（凸に近いもの。最初の点から扇に塗る）
fn poly(g: &mut Gfx, pen: &Pen, pts: &[(f32, f32)], c: [f32; 3], a: f32) {
    if pts.len() < 3 {
        return;
    }
    let col = ca(c, a * pen.a);
    let p0 = pen.p(pts[0].0, pts[0].1);
    for w in pts[1..].windows(2) {
        g.tri(p0, pen.p(w[0].0, w[0].1), pen.p(w[1].0, w[1].1), col);
    }
}

fn seg(g: &mut Gfx, pen: &Pen, u0: f32, v0: f32, u1: f32, v1: f32, w: f32, c: [f32; 3], a: f32) {
    g.line(pen.p(u0, v0), pen.p(u1, v1), (w * pen.s).max(0.0006), ca(c, a * pen.a));
}

/// 先細りの線
fn taper(g: &mut Gfx, pen: &Pen, pts: &[(f32, f32)], w0: f32, w1: f32, c: [f32; 3], a: f32) {
    let n = pts.len();
    if n < 2 {
        return;
    }
    let col = ca(c, a * pen.a);
    for i in 0..n - 1 {
        let t0 = i as f32 / (n - 1) as f32;
        let t1 = (i + 1) as f32 / (n - 1) as f32;
        let (p0, p1) = (pen.p(pts[i].0, pts[i].1), pen.p(pts[i + 1].0, pts[i + 1].1));
        let (dx, dy) = (p1.0 - p0.0, p1.1 - p0.1);
        let l = (dx * dx + dy * dy).sqrt().max(1e-6);
        let (nx, ny) = (-dy / l, dx / l);
        let wa = (w0 + (w1 - w0) * t0) * pen.s * 0.5;
        let wb = (w0 + (w1 - w0) * t1) * pen.s * 0.5;
        let a0 = (p0.0 + nx * wa, p0.1 + ny * wa);
        let b0 = (p0.0 - nx * wa, p0.1 - ny * wa);
        let a1 = (p1.0 + nx * wb, p1.1 + ny * wb);
        let b1 = (p1.0 - nx * wb, p1.1 - ny * wb);
        g.tri(a0, a1, b1, col);
        g.tri(a0, b1, b0, col);
    }
}

/// 平たく置いた体から画面の上へ立つ棘
fn spike_up(g: &mut Gfx, pen: &Pen, u: f32, v: f32, h: f32, w: f32, lean: f32, c: [f32; 3], a: f32) {
    let b0 = pen.p(u - w, v);
    let b1 = pen.p(u + w, v);
    let (tx, ty) = pen.up(u, v, h);
    g.tri(b0, b1, (tx + lean * pen.s * pen.fx, ty), ca(c, a * pen.a));
}

/// 生きものを描く。
/// x, y：体の中心（固着するもの・海底を歩くものは足元）。len：描く長さ。t：動きの時刻。
/// state：巣穴から出ている度合いなど（0〜1）。seed：個体ごとの形の違い
pub fn draw(g: &mut Gfx, sp: usize, x: f32, y: f32, len: f32, facing: f32, pitch: f32, t: f32, alpha: f32, state: f32, seed: u32) {
    let s = &SPECIES[sp];
    let art = s.art;
    let pen = Pen { x, y, s: len, fx: if facing < 0.0 { -1.0 } else { 1.0 }, sq: if art.dorsal() { 0.45 } else { 1.0 }, rot: pitch, a: alpha };
    let (c, c2) = (s.col, s.col2);
    match art {
        Art::Radiodont(k) => radiodont(g, &pen, k, c, c2, t),
        Art::Trilobite { w, spine, pyg, segs } => trilobite(g, &pen, w, spine, pyg, segs, c, c2, t),
        Art::Agnostus => agnostus(g, &pen, c, c2, t),
        Art::Opabinia => opabinia(g, &pen, c, c2, t),
        Art::Hallucigenia => hallucigenia(g, &pen, c, c2, t),
        Art::Aysheaia => aysheaia(g, &pen, c, c2, t),
        Art::Kerygmachela => kerygmachela(g, &pen, c, c2, t),
        Art::Wiwaxia => wiwaxia(g, &pen, c, c2, t),
        Art::Halkieria => halkieria(g, &pen, c, c2, t),
        Art::Marrella => marrella(g, &pen, c, c2, t),
        Art::Leanchoilia => leanchoilia(g, &pen, c, c2, t),
        Art::Fuxianhuia => fuxianhuia(g, &pen, c, c2, t),
        Art::Sidneyia => sidneyia(g, &pen, c, c2, t),
        Art::Isoxys => isoxys(g, &pen, c, c2, t),
        Art::Nectocaris => nectocaris(g, &pen, c, c2, t),
        Art::Fish(k) => fish(g, &pen, k, c, c2, t),
        Art::Jelly => jelly(g, &pen, c, c2, t),
        Art::Radiolarian => radiolarian(g, &pen, c, c2, t),
        Art::Sponge(k) => sponge(g, &pen, k, c, c2, t, seed),
        Art::Archaeocyath => archaeocyath(g, &pen, c, c2, seed),
        Art::Stromatolite => stromatolite(g, &pen, c, c2, seed),
        Art::Mat => mat(g, &pen, c, c2, seed),
        Art::Tubes => tubes(g, &pen, c, c2, seed),
        Art::Brachiopod => brachiopod(g, &pen, c, c2, t),
        Art::Worm => worm(g, &pen, c, c2, t, state),
        Art::Trace => trace(g, &pen, c, c2, seed),
        Art::Slug => slug(g, &pen, c, c2, t),
        Art::Cephalopod(k) => cephalopod(g, &pen, k, c, c2, t),
        Art::Eurypterid(k) => eurypterid(g, &pen, k, c, c2, t),
        Art::Crinoid => crinoid(g, &pen, c, c2, t, seed),
        Art::Coral(k) => coral(g, &pen, k, c, c2, t, seed),
        Art::Graptolite => graptolite(g, &pen, c, c2, t),
        Art::Jawless(k) => jawless(g, &pen, k, c, c2, t),
        Art::Placoderm(k) => placoderm(g, &pen, k, c, c2, t),
        Art::Shark(k) => shark(g, &pen, k, c, c2, t),
        Art::LobeFin(k) => lobefin(g, &pen, k, c, c2, t),
        Art::Tetrapod(k) => tetrapod(g, &pen, k, c, c2, t),
        Art::Plant(k) => plant(g, &pen, k, c, c2, t, seed),
        Art::Arthro(5) => crab(g, &pen, c, c2, t),
        Art::Arthro(k) => arthro(g, &pen, k, c, c2, t),
        Art::Griffinfly => griffinfly(g, &pen, c, c2, t),
        Art::Tully => tully(g, &pen, c, c2, t),
        Art::Fusulinid => fusulinid(g, &pen, c, c2, seed),
        Art::Mollusc(k) => mollusc(g, &pen, k, c, c2, t),
        Art::Starfish => starfish(g, &pen, c, c2, t),
        Art::RayFin => rayfin(g, &pen, c, c2, t),
        Art::Limulid => limulid(g, &pen, c, c2, t),
        Art::Urchin => urchin(g, &pen, c, c2, t),
        Art::Dino(k) => dino(g, &pen, k, c, c2, t),
        Art::Marine(k) => marine(g, &pen, k, c, c2, t),
        Art::Pterosaur(k) => flyer(g, &pen, false, k, c, c2, t),
        Art::Bird(3) => flightless_bird(g, &pen, c, c2, t),
        Art::Bird(5) => penguin(g, &pen, c, c2, t),
        Art::Bird(6) => wading_bird(g, &pen, c, c2, t),
        Art::Bird(k) => flyer(g, &pen, true, k, c, c2, t),
        Art::Mammal(k) => mammal(g, &pen, k, c, c2, t),
        Art::Archosaur(k) => archosaur(g, &pen, k, c, c2, t),
    }
}

/// 描いたときの大きさ（体長に対する、幅・高さ）。写真の判定と図鑑の枠に使う
pub fn extent(sp: usize) -> (f32, f32) {
    match SPECIES[sp].art {
        Art::Radiodont(_) => (1.2, 0.45),
        Art::Trilobite { w, spine, .. } => (1.0 + spine * 0.3, w * 0.6),
        Art::Marrella => (1.0, 0.7),
        Art::Wiwaxia => (1.0, 0.5),
        Art::Hallucigenia => (1.0, 0.45),
        Art::Kerygmachela | Art::Leanchoilia => (1.3, 0.35),
        Art::Jelly => (0.8, 0.8),
        Art::Sponge(_) | Art::Archaeocyath | Art::Tubes | Art::Brachiopod => (0.8, 1.0),
        Art::Stromatolite => (1.1, 0.55),
        Art::Worm => (0.5, 0.7),
        Art::Isoxys => (1.1, 0.5),
        Art::Cephalopod(0) => (1.2, 0.3),
        Art::Cephalopod(_) => (0.95, 0.7),
        Art::Eurypterid(1) => (1.35, 0.6),
        Art::Eurypterid(_) => (1.1, 0.55),
        Art::Crinoid => (0.7, 1.0),
        Art::Coral(_) => (0.9, 1.0),
        Art::Graptolite => (1.0, 0.7),
        Art::Jawless(_) | Art::LobeFin(_) => (1.05, 0.35),
        Art::Placoderm(_) | Art::Shark(_) => (1.1, 0.45),
        Art::Tetrapod(4) => (1.05, 0.75),
        Art::Tetrapod(k) => (1.05, if k == 9 || k == 10 { 0.5 } else { 0.4 }),
        Art::Plant(9) => (1.0, 0.3),
        Art::Plant(_) => (0.8, 1.0),
        Art::Arthro(2) => (1.05, 0.55),
        Art::Arthro(3) => (0.8, 0.45),
        Art::Arthro(5) => (1.1, 0.5),
        Art::Arthro(6) => (1.0, 0.7),
        Art::Arthro(_) => (1.1, 0.3),
        Art::Griffinfly => (1.0, 0.9),
        Art::Tully => (1.2, 0.4),
        Art::Fusulinid => (1.0, 0.3),
        Art::Mollusc(1) => (0.7, 0.6),
        Art::Mollusc(_) => (0.95, 0.75),
        Art::Starfish => (1.0, 1.0),
        Art::RayFin => (1.05, 0.3),
        Art::Limulid => (1.0, 0.75),
        Art::Urchin => (0.9, 0.85),
        Art::Dino(3) => (1.15, 0.7),
        Art::Dino(13) => (1.0, 1.0),
        Art::Dino(2) => (1.05, 0.75),
        Art::Dino(0) | Art::Dino(11) => (1.05, 0.5),
        Art::Dino(1) | Art::Dino(4) => (1.05, 0.6),
        Art::Dino(10) => (1.05, 0.65),
        Art::Dino(_) => (1.05, 0.55),
        Art::Marine(5) => (1.0, 0.5),
        Art::Marine(7) => (1.0, 0.65),
        Art::Marine(_) => (1.15, 0.4),
        Art::Pterosaur(2) => (0.95, 1.0),
        Art::Pterosaur(_) | Art::Bird(_) => (0.9, 1.0),
        Art::Mammal(k) if k >= 4 => (1.2, 1.2),
        Art::Mammal(_) => (1.05, 0.4),
        Art::Archosaur(3) => (1.05, 0.3),
        Art::Archosaur(_) => (1.05, 0.35),
        _ => (1.0, 0.4),
    }
}

/// 足元の高さ（体の座標の v を負にしたもの）。足元を海底に合わせるとき、描く中心をこれだけ上にずらす
pub fn base_lift(sp: usize) -> f32 {
    match SPECIES[sp].art {
        Art::Sponge(_) | Art::Archaeocyath | Art::Tubes | Art::Brachiopod => 0.45,
        Art::Stromatolite => 0.2,
        Art::Crinoid | Art::Coral(_) | Art::Plant(_) | Art::Fusulinid => 0.5,
        Art::Mollusc(1) => 0.45,
        Art::Bird(3) => 0.4,
        Art::Bird(6) => 0.36,
        Art::Mammal(9) => 0.43,
        Art::Mammal(28) => 0.38,
        Art::Arthro(5) => 0.23,
        // 歩く動物は足の裏（v=0）を地面に置く
        _ => 0.0,
    }
}

fn radiodont(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    let dark = mul(c, 0.62);
    let n = 11;
    // 奥側のひれ
    for i in 0..n {
        let u = 0.24 - i as f32 * 0.058;
        let ph = t * 6.0 - i as f32 * 0.7;
        let sz = 1.0 - (i as f32 / n as f32 - 0.35).abs() * 0.8;
        ell(g, pen, u, 0.045, 0.05 * sz + 0.01, 0.07 * sz + 0.02 * ph.sin(), 0.5 + 0.35 * ph.sin(), dark, 0.9);
    }
    // 尾
    match k {
        0 => {
            for (i, &(dv, ang)) in [(0.05, 0.7), (0.0, 0.25), (-0.05, -0.2)].iter().enumerate() {
                let w = (t * 3.0 + i as f32).sin() * 0.06;
                ell(g, pen, -0.46, dv + 0.02, 0.085, 0.022, ang + w, if i == 1 { c2 } else { mul(c2, 0.85) }, 0.95);
            }
        }
        1 => {
            let w = (t * 2.5).sin() * 0.03;
            taper(g, pen, &[(-0.36, 0.0), (-0.52, 0.03 + w), (-0.68, 0.06 + w * 2.0)], 0.025, 0.004, c2, 1.0);
            taper(g, pen, &[(-0.36, -0.01), (-0.52, -0.04 - w), (-0.68, -0.05 - w * 2.0)], 0.025, 0.004, c2, 1.0);
        }
        _ => {
            ell(g, pen, -0.42, 0.0, 0.06, 0.03, 0.0, dark, 1.0);
        }
    }
    // 胴
    ell(g, pen, 0.0, 0.0, 0.35, if k == 3 { 0.07 } else { 0.08 }, 0.0, c, 1.0);
    for i in 0..10 {
        let u = 0.22 - i as f32 * 0.055;
        seg(g, pen, u, 0.07, u - 0.01, -0.07, 0.006, dark, 0.55);
    }
    // 頭
    if k == 3 {
        ell(g, pen, 0.22, 0.03, 0.26, 0.12, 0.08, c2, 1.0);
        ell(g, pen, 0.24, 0.07, 0.20, 0.05, 0.08, mix(c2, WHITE, 0.25), 0.6);
        ell(g, pen, 0.36, 0.0, 0.06, 0.05, 0.0, mul(c2, 0.7), 1.0);
    } else {
        ell(g, pen, 0.31, 0.012, 0.12, 0.085, 0.0, c, 1.0);
        ell(g, pen, 0.33, 0.05, 0.08, 0.03, 0.0, mix(c, WHITE, 0.25), 0.5);
    }
    // 眼（柄の先）
    let (eu, ev) = if k == 3 { (0.12, 0.13) } else { (0.36, 0.14) };
    seg(g, pen, eu - 0.03, ev - 0.06, eu, ev, 0.018, dark, 1.0);
    ell(g, pen, eu + 0.005, ev + 0.005, 0.035, 0.026, 0.0, INK, 1.0);
    circ(g, pen, eu + 0.015, ev + 0.014, 0.008, WHITE, 0.8);
    // 前の付属肢
    let sway = (t * 2.0).sin() * 0.12;
    let (nseg, step0, curl, spine) = match k {
        0 => (13, 0.034, 0.16, 0.035),
        1 => (8, 0.04, 0.12, 0.06),
        2 => (14, 0.034, 0.14, 0.07),
        _ => (7, 0.04, 0.10, 0.09),
    };
    let mut u = 0.41;
    let mut v = -0.03;
    let mut ang = -0.25 + sway;
    let ac = mix(c2, c, 0.3);
    for j in 0..nseg {
        let f = 1.0 - j as f32 / nseg as f32 * 0.55;
        let step = step0 * f;
        let (nu, nv) = (u + ang.cos() * step, v + ang.sin() * step);
        seg(g, pen, u, v, nu, nv, 0.036 * f + 0.008, ac, 1.0);
        // 棘（内側＝下向き）
        let sa = ang - PI * 0.5;
        let sl = spine * f;
        if k == 2 {
            seg(g, pen, nu, nv, nu + sa.cos() * sl, nv + sa.sin() * sl, 0.004, mix(ac, WHITE, 0.4), 0.9);
        } else if k == 3 {
            taper(g, pen, &[(nu, nv), (nu + (sa + 0.5).cos() * sl, nv + (sa + 0.5).sin() * sl)], 0.012, 0.002, mix(ac, WHITE, 0.3), 1.0);
        } else if j % 2 == 0 || k == 1 {
            taper(g, pen, &[(nu, nv), (nu + sa.cos() * sl, nv + sa.sin() * sl)], 0.01, 0.002, mix(ac, WHITE, 0.35), 1.0);
        }
        u = nu;
        v = nv;
        ang -= curl;
    }
    // 手前のひれ
    for i in 0..n {
        let u = 0.24 - i as f32 * 0.058;
        let ph = t * 6.0 - i as f32 * 0.7 + 0.4;
        let sz = 1.0 - (i as f32 / n as f32 - 0.35).abs() * 0.8;
        ell(g, pen, u - 0.01, -0.06, 0.05 * sz + 0.012, 0.06 * sz + 0.025 * ph.sin(), -0.5 - 0.35 * ph.sin(), mix(c, WHITE, 0.12), 0.92);
    }
}

fn trilobite(g: &mut Gfx, pen: &Pen, w: f32, spine: f32, pyg: f32, segs: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    let hw = w * 0.5;
    let dark = mul(c, 0.55);
    let segs = segs.max(2) as usize;
    let th0 = 0.27;
    let th1 = -0.5 + pyg * 2.0 + 0.02;
    let sl = (th0 - th1) / segs as f32;
    // 脚（体の下から少し見える）
    for i in 0..segs {
        let u = th0 - (i as f32 + 0.5) * sl;
        let wv = hw * (1.0 - 0.45 * i as f32 / segs as f32);
        let ph = (t * 7.0 + i as f32 * 0.9).sin() * 0.025;
        for sgn in [-1.0, 1.0] {
            seg(g, pen, u, sgn * wv * 0.9, u + ph, sgn * (wv + 0.07), 0.012, dark, 0.9);
        }
    }
    // 触角
    let aw = (t * 1.5).sin() * 0.03;
    for sgn in [-1.0f32, 1.0] {
        taper(g, pen, &[(0.47, sgn * 0.05), (0.56, sgn * (0.12 + aw)), (0.64, sgn * (0.2 + aw * 2.0))], 0.012, 0.004, dark, 0.9);
    }
    // 頬の棘
    for sgn in [-1.0f32, 1.0] {
        poly(g, pen, &[(0.30, sgn * hw * 0.98), (0.22, sgn * hw * 0.80), (0.27 - spine, sgn * (hw * 1.02 + 0.02))], c, 1.0);
    }
    // 頭（半月）
    let mut cep = vec![(0.27, 0.0)];
    for i in 0..=14 {
        let a = -PI * 0.5 + PI * i as f32 / 14.0;
        cep.push((0.27 + 0.22 * a.cos(), hw * a.sin()));
    }
    poly(g, pen, &cep, c, 1.0);
    // 胸の節
    for i in 0..segs {
        let u = th0 - (i as f32 + 0.5) * sl;
        let wv = hw * (1.0 - 0.45 * i as f32 / segs as f32);
        let cc = if i % 2 == 0 { c } else { mul(c, 0.9) };
        poly(g, pen, &[(u + sl * 0.55, -wv), (u + sl * 0.55, wv), (u - sl * 0.5, wv * 0.97 + 0.01), (u - sl * 0.5, -wv * 0.97 - 0.01)], cc, 1.0);
        seg(g, pen, u - sl * 0.5, -wv, u - sl * 0.5, wv, 0.006, dark, 0.6);
        // 節の先のとげ
        for sgn in [-1.0f32, 1.0] {
            poly(g, pen, &[(u + sl * 0.4, sgn * wv), (u - sl * 0.4, sgn * wv), (u - sl * 1.1, sgn * (wv + 0.03))], cc, 1.0);
        }
    }
    // 尾
    if pyg > 0.0 {
        ell(g, pen, -0.5 + pyg, 0.0, pyg + 0.01, hw * (0.35 + pyg * 1.8), 0.0, c, 1.0);
    }
    // 中軸
    ell(g, pen, (th0 + th1) * 0.5, 0.0, (th0 - th1) * 0.52, hw * 0.26, 0.0, c2, 0.75);
    ell(g, pen, 0.37, 0.0, 0.1, hw * 0.3, 0.0, c2, 0.85);
    ell(g, pen, 0.39, 0.0, 0.06, hw * 0.14, 0.0, mix(c2, WHITE, 0.3), 0.4);
    // 眼
    for sgn in [-1.0f32, 1.0] {
        ell(g, pen, 0.33, sgn * hw * 0.58, 0.05, 0.03, sgn * 0.4, INK, 0.9);
        ell(g, pen, 0.335, sgn * hw * 0.6, 0.02, 0.012, 0.0, mix(c, WHITE, 0.5), 0.6);
    }
}

fn agnostus(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let dark = mul(c, 0.55);
    for i in 0..4 {
        let ph = (t * 9.0 + i as f32).sin() * 0.03;
        for sgn in [-1.0f32, 1.0] {
            seg(g, pen, 0.1 - i as f32 * 0.08, sgn * 0.18, 0.1 - i as f32 * 0.08 + ph, sgn * 0.3, 0.02, dark, 0.9);
        }
    }
    ell(g, pen, 0.24, 0.0, 0.25, 0.26, 0.0, c, 1.0);
    ell(g, pen, -0.24, 0.0, 0.25, 0.26, 0.0, c, 1.0);
    ell(g, pen, 0.0, 0.0, 0.06, 0.22, 0.0, mul(c, 0.85), 1.0);
    for u in [0.24f32, -0.24] {
        ell(g, pen, u, 0.0, 0.2, 0.21, 0.0, mix(c, WHITE, 0.15), 1.0);
        ell(g, pen, u, 0.0, 0.1, 0.07, 0.0, c2, 0.8);
    }
}

fn opabinia(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let dark = mul(c, 0.6);
    let n = 13;
    for i in 0..n {
        let u = 0.24 - i as f32 * 0.05;
        let ph = t * 5.0 - i as f32 * 0.7;
        ell(g, pen, u, 0.04, 0.045, 0.05 + 0.015 * ph.sin(), 0.5 + 0.3 * ph.sin(), dark, 0.9);
    }
    // 尾の扇（赤と青の羽）
    for (i, &(dv, ang)) in [(0.09, 1.0), (0.05, 0.55), (0.01, 0.15)].iter().enumerate() {
        let w = (t * 3.0 + i as f32).sin() * 0.05;
        let col = if i % 2 == 0 { c2 } else { [0.34, 0.46, 0.70] };
        ell(g, pen, -0.42, dv, 0.08, 0.02, ang + w, col, 1.0);
    }
    ell(g, pen, 0.0, 0.0, 0.33, 0.065, 0.0, c, 1.0);
    for i in 0..14 {
        let u = 0.25 - i as f32 * 0.045;
        seg(g, pen, u, 0.06, u, -0.06, 0.005, dark, 0.6);
    }
    ell(g, pen, 0.31, 0.01, 0.09, 0.07, 0.0, c, 1.0);
    // 5つの眼
    for (i, &(eu, ev)) in [(0.26, 0.10), (0.30, 0.125), (0.34, 0.13), (0.37, 0.115), (0.32, 0.09)].iter().enumerate() {
        seg(g, pen, eu, ev - 0.04, eu, ev, 0.012, dark, 1.0);
        circ(g, pen, eu, ev, 0.018 + if i == 4 { 0.0 } else { 0.004 }, INK, 1.0);
        circ(g, pen, eu + 0.005, ev + 0.006, 0.005, WHITE, 0.7);
    }
    // ホースのような口
    let sw = (t * 1.6).sin() * 0.1;
    let mut pts = Vec::new();
    for i in 0..9 {
        let f = i as f32 / 8.0;
        pts.push((0.38 + f * 0.26, -0.03 - f * 0.12 + (f * PI).sin() * 0.04 * (1.0 + sw)));
    }
    taper(g, pen, &pts, 0.03, 0.018, mix(c, c2, 0.2), 1.0);
    let (eu, ev) = pts[8];
    seg(g, pen, eu, ev, eu + 0.05, ev + 0.03 + sw * 0.1, 0.012, c2, 1.0);
    seg(g, pen, eu, ev, eu + 0.05, ev - 0.03, 0.012, c2, 1.0);
    for i in 0..n {
        let u = 0.24 - i as f32 * 0.05;
        let ph = t * 5.0 - i as f32 * 0.7 + 0.5;
        ell(g, pen, u - 0.01, -0.05, 0.045, 0.045 + 0.015 * ph.sin(), -0.5 - 0.3 * ph.sin(), mix(c, WHITE, 0.1), 0.9);
    }
}

fn hallucigenia(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let dark = mul(c, 0.6);
    let body_v = |u: f32| 0.1 + 0.02 * (u * 6.0 + 1.0).sin();
    // 脚
    for i in 0..7 {
        let u = 0.28 - i as f32 * 0.09;
        let ph = (t * 3.0 + i as f32 * 1.1).sin() * 0.025;
        for (k, du) in [(0, -0.008f32), (1, 0.008)] {
            let col = if k == 0 { mul(dark, 0.8) } else { dark };
            taper(g, pen, &[(u + du, body_v(u)), (u + du + ph * 0.5, body_v(u) * 0.5), (u + du + ph, 0.0)], 0.022, 0.01, col, 1.0);
        }
    }
    // 棘
    for i in 0..7 {
        let u = 0.28 - i as f32 * 0.09;
        let bv = body_v(u);
        for (k, lean) in [(0, -0.02f32), (1, 0.02)] {
            let col = if k == 0 { mul(c2, 0.8) } else { c2 };
            poly(g, pen, &[(u - 0.015 + lean, bv), (u + 0.015 + lean, bv), (u - 0.03 + lean * 2.0, bv + 0.24)], col, 1.0);
        }
    }
    // 胴
    let mut u = -0.46;
    while u < 0.42 {
        circ(g, pen, u, body_v(u), 0.032, c, 1.0);
        u += 0.02;
    }
    // 頭
    ell(g, pen, 0.44, body_v(0.44) - 0.01, 0.06, 0.035, -0.3, c, 1.0);
    circ(g, pen, 0.47, body_v(0.44) + 0.0, 0.008, INK, 1.0);
    // 前の触手
    for i in 0..3 {
        let u = 0.36 + i as f32 * 0.025;
        let w = (t * 2.0 + i as f32).sin() * 0.02;
        seg(g, pen, u, body_v(u), u + 0.02 + w, body_v(u) - 0.08, 0.008, dark, 0.9);
    }
}

fn aysheaia(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let v0 = 0.11;
    for i in 0..10 {
        let u = 0.33 - i as f32 * 0.075;
        let ph = (t * 2.5 + i as f32 * 0.9).sin() * 0.02;
        taper(g, pen, &[(u, v0), (u + ph, 0.03), (u + ph + 0.015, 0.0)], 0.05, 0.02, mul(c2, 0.95), 1.0);
        seg(g, pen, u + ph + 0.01, 0.0, u + ph + 0.04, 0.0, 0.01, INK, 0.7);
    }
    ell(g, pen, 0.0, v0 + 0.01, 0.44, 0.075, 0.0, c, 1.0);
    for i in 0..18 {
        let u = 0.38 - i as f32 * 0.045;
        seg(g, pen, u, v0 + 0.075, u, v0 - 0.06, 0.006, c2, 0.55);
    }
    // 頭の付属肢
    let w = (t * 1.8).sin() * 0.03;
    taper(g, pen, &[(0.42, v0 + 0.02), (0.5, v0 + 0.05 + w), (0.56, v0 + 0.02 + w)], 0.03, 0.01, c2, 1.0);
    taper(g, pen, &[(0.42, v0 - 0.01), (0.5, v0 - 0.03 - w), (0.55, v0 - 0.06 - w)], 0.03, 0.01, c2, 1.0);
}

fn kerygmachela(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let dark = mul(c, 0.6);
    let w = (t * 2.0).sin() * 0.03;
    taper(g, pen, &[(-0.28, 0.01), (-0.45, 0.05 + w), (-0.66, 0.08 + w * 2.0)], 0.016, 0.003, c2, 1.0);
    taper(g, pen, &[(-0.28, -0.01), (-0.45, -0.04 - w), (-0.66, -0.05 - w * 2.0)], 0.016, 0.003, c2, 1.0);
    for i in 0..11 {
        let u = 0.22 - i as f32 * 0.05;
        let ph = t * 5.0 - i as f32 * 0.7;
        ell(g, pen, u, 0.04, 0.04, 0.055 + 0.015 * ph.sin(), 0.5 + 0.3 * ph.sin(), dark, 0.9);
    }
    ell(g, pen, 0.0, 0.0, 0.3, 0.06, 0.0, c, 1.0);
    for i in 0..11 {
        let u = 0.22 - i as f32 * 0.05;
        ell(g, pen, u, -0.055, 0.012, 0.025, 0.0, c2, 0.9);
    }
    ell(g, pen, 0.3, 0.0, 0.07, 0.05, 0.0, c, 1.0);
    circ(g, pen, 0.33, 0.03, 0.012, INK, 1.0);
    let sw = (t * 1.4).sin() * 0.04;
    for (k, dv) in [(0, 0.015f32), (1, -0.015)] {
        let col = if k == 0 { mul(c2, 0.8) } else { c2 };
        let tip = (0.62, -0.02 + dv + sw);
        taper(g, pen, &[(0.34, dv), (0.48, dv + sw * 0.5), tip], 0.03, 0.012, col, 1.0);
        for i in 0..5 {
            let f = 0.2 + i as f32 * 0.16;
            let (pu, pv) = (0.34 + (tip.0 - 0.34) * f, dv + (tip.1 - dv) * f);
            seg(g, pen, pu, pv, pu + 0.01, pv - 0.04, 0.005, mix(col, WHITE, 0.3), 0.9);
        }
    }
    for i in 0..11 {
        let u = 0.22 - i as f32 * 0.05;
        let ph = t * 5.0 - i as f32 * 0.7 + 0.5;
        ell(g, pen, u - 0.01, -0.05, 0.04, 0.05 + 0.015 * ph.sin(), -0.5 - 0.3 * ph.sin(), mix(c, WHITE, 0.1), 0.9);
    }
}

fn wiwaxia(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], _t: f32) {
    let row = |g: &mut Gfx, sgn: f32, col: [f32; 3]| {
        for i in 0..8 {
            let u = 0.3 - i as f32 * 0.085;
            let h = 0.22 + 0.12 * (1.0 - ((i as f32 - 3.5) / 3.5).abs());
            spike_up(g, pen, u, sgn * 0.07, h, 0.025, -0.06, col, 1.0);
        }
    };
    row(g, 1.0, mul(c2, 0.75));
    ell(g, pen, 0.0, 0.0, 0.46, 0.24, 0.0, mul(c, 0.8), 1.0);
    for r in 0..5 {
        let v = -0.18 + r as f32 * 0.09;
        let ru = 0.42 * (1.0 - (v / 0.25).powi(2)).max(0.1).sqrt();
        let mut u = -ru;
        let mut k = r;
        while u < ru {
            let col = if k % 2 == 0 { c } else { mix(c, c2, 0.4) };
            ell(g, pen, u, v, 0.05, 0.05, 0.5, col, 1.0);
            ell(g, pen, u + 0.01, v + 0.01, 0.025, 0.02, 0.5, mix(col, WHITE, 0.3), 0.5);
            u += 0.07;
            k += 1;
        }
    }
    row(g, -1.0, c2);
}

fn halkieria(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let wob = (t * 1.5).sin() * 0.01;
    ell(g, pen, 0.0, 0.0, 0.48, 0.18 + wob, 0.0, mul(c, 0.8), 1.0);
    for r in 0..4 {
        let v = -0.13 + r as f32 * 0.085;
        let mut u = -0.38;
        while u < 0.36 {
            ell(g, pen, u, v, 0.035, 0.03, 0.8, if (r + (u * 30.0) as i32 as usize) % 2 == 0 { c } else { mix(c, c2, 0.5) }, 1.0);
            u += 0.05;
        }
    }
    for (u, r) in [(0.36f32, 0.12f32), (-0.38, 0.11)] {
        ell(g, pen, u, 0.0, r, r * 1.05, 0.0, c2, 1.0);
        for k in 1..4 {
            ell(g, pen, u, 0.0, r * (1.0 - k as f32 * 0.22), r * (1.0 - k as f32 * 0.22) * 1.05, 0.0, if k % 2 == 0 { c2 } else { mul(c2, 0.86) }, 1.0);
        }
    }
}

fn marrella(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    // 棘の虹色
    let h = (t * 0.8).sin() * 0.5 + 0.5;
    let iri = mix(c2, [0.86, 0.60, 0.90], h);
    let dark = mul(c, 0.6);
    // 脚とえら
    for i in 0..16 {
        let u = 0.12 - i as f32 * 0.03;
        let ph = (t * 10.0 - i as f32 * 0.6).sin() * 0.03;
        seg(g, pen, u, -0.03, u - 0.02 + ph, -0.13, 0.008, dark, 0.9);
        seg(g, pen, u, -0.02, u - 0.05 + ph, -0.08, 0.01, mix(c, WHITE, 0.3), 0.5);
    }
    // 後ろの棘（奥）
    taper(g, pen, &[(0.17, 0.0), (0.05, -0.07), (-0.15, -0.13), (-0.38, -0.12)], 0.035, 0.005, mul(iri, 0.85), 1.0);
    ell(g, pen, -0.02, 0.0, 0.2, 0.05, 0.0, c, 1.0);
    for i in 0..8 {
        let u = 0.1 - i as f32 * 0.035;
        seg(g, pen, u, 0.04, u, -0.04, 0.004, dark, 0.6);
    }
    ell(g, pen, 0.17, 0.02, 0.06, 0.05, 0.0, c, 1.0);
    // 上の大きな棘
    taper(g, pen, &[(0.2, 0.06), (0.12, 0.2), (-0.08, 0.3), (-0.36, 0.33)], 0.04, 0.005, iri, 1.0);
    taper(g, pen, &[(0.18, 0.05), (0.06, 0.12), (-0.14, 0.14), (-0.4, 0.1)], 0.035, 0.005, mix(iri, WHITE, 0.2), 1.0);
    // 触角
    let w = (t * 2.0).sin() * 0.03;
    taper(g, pen, &[(0.22, 0.02), (0.36, 0.06 + w), (0.52, 0.1 + w * 2.0)], 0.012, 0.003, dark, 1.0);
    taper(g, pen, &[(0.22, 0.0), (0.36, -0.03 - w), (0.5, -0.07 - w)], 0.012, 0.003, dark, 1.0);
}

fn leanchoilia(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let dark = mul(c, 0.6);
    for i in 0..12 {
        let u = 0.25 - i as f32 * 0.05;
        let ph = (t * 8.0 - i as f32 * 0.6).sin() * 0.03;
        seg(g, pen, u, -0.04, u - 0.02 + ph, -0.11, 0.012, dark, 0.9);
    }
    poly(g, pen, &[(-0.32, 0.01), (-0.52, 0.0), (-0.32, -0.02)], c, 1.0);
    ell(g, pen, 0.0, 0.0, 0.34, 0.06, 0.0, c, 1.0);
    for i in 0..12 {
        let u = 0.27 - i as f32 * 0.05;
        seg(g, pen, u, 0.058, u, -0.055, 0.005, dark, 0.6);
        ell(g, pen, u - 0.02, 0.06, 0.028, 0.012, 0.0, mix(c, WHITE, 0.2), 0.6);
    }
    ell(g, pen, 0.33, 0.01, 0.06, 0.05, 0.0, c, 1.0);
    // 大付属肢：付け根と3本のムチ
    taper(g, pen, &[(0.37, -0.01), (0.42, -0.03), (0.46, -0.02)], 0.03, 0.025, c2, 1.0);
    for k in 0..3 {
        let base = 0.25 - k as f32 * 0.25;
        let mut pts = Vec::new();
        for i in 0..10 {
            let f = i as f32 / 9.0;
            let w = (t * 2.5 + k as f32 + f * 4.0).sin() * 0.03 * f;
            pts.push((0.46 + f * 0.36, -0.02 + base * f * 0.5 + (f * PI).sin() * 0.08 * (1.0 - k as f32 * 0.6) + w));
        }
        taper(g, pen, &pts, 0.012, 0.002, mix(c2, WHITE, 0.2), 1.0);
    }
}

fn fuxianhuia(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let dark = mul(c, 0.55);
    for i in 0..14 {
        let u = 0.24 - i as f32 * 0.032;
        let ph = (t * 6.0 + i as f32 * 0.8).sin() * 0.02;
        for sgn in [-1.0f32, 1.0] {
            seg(g, pen, u, sgn * 0.17, u + ph, sgn * 0.25, 0.01, dark, 0.8);
        }
    }
    let aw = (t * 1.4).sin() * 0.03;
    for sgn in [-1.0f32, 1.0] {
        taper(g, pen, &[(0.48, sgn * 0.04), (0.56, sgn * (0.1 + aw)), (0.62, sgn * (0.16 + aw))], 0.012, 0.004, dark, 0.9);
    }
    ell(g, pen, -0.47, 0.0, 0.04, 0.08, 0.0, c2, 1.0);
    for i in 0..5 {
        let u = -0.22 - i as f32 * 0.05;
        ell(g, pen, u, 0.0, 0.03, 0.06 - i as f32 * 0.005, 0.0, if i % 2 == 0 { c } else { mul(c, 0.9) }, 1.0);
    }
    for i in 0..15 {
        let u = 0.26 - i as f32 * 0.032;
        let wv = 0.2 - i as f32 * 0.006;
        ell(g, pen, u, 0.0, 0.022, wv, 0.0, if i % 2 == 0 { c } else { mul(c, 0.9) }, 1.0);
    }
    ell(g, pen, 0.38, 0.0, 0.12, 0.2, 0.0, c, 1.0);
    ell(g, pen, 0.38, 0.0, 0.08, 0.08, 0.0, c2, 0.5);
    for sgn in [-1.0f32, 1.0] {
        ell(g, pen, 0.47, sgn * 0.08, 0.025, 0.02, 0.0, INK, 1.0);
    }
}

fn sidneyia(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let dark = mul(c, 0.55);
    for i in 0..9 {
        let u = 0.2 - i as f32 * 0.045;
        let ph = (t * 5.0 + i as f32).sin() * 0.025;
        for sgn in [-1.0f32, 1.0] {
            seg(g, pen, u, sgn * 0.2, u + ph, sgn * 0.29, 0.015, dark, 0.8);
        }
    }
    for sgn in [-1.0f32, 1.0] {
        taper(g, pen, &[(0.46, sgn * 0.06), (0.56, sgn * 0.12), (0.64, sgn * 0.14)], 0.016, 0.005, dark, 1.0);
    }
    for (dv, ang) in [(0.07f32, 0.4f32), (0.0, 0.0), (-0.07, -0.4)] {
        ell(g, pen, -0.44, dv, 0.07, 0.035, ang, c2, 1.0);
    }
    for i in 0..3 {
        let u = -0.22 - i as f32 * 0.055;
        ell(g, pen, u, 0.0, 0.035, 0.08, 0.0, c, 1.0);
    }
    for i in 0..9 {
        let u = 0.2 - i as f32 * 0.045;
        let wv = 0.23 - i as f32 * 0.008;
        ell(g, pen, u, 0.0, 0.03, wv, 0.0, if i % 2 == 0 { c } else { mul(c, 0.9) }, 1.0);
        for sgn in [-1.0f32, 1.0] {
            poly(g, pen, &[(u + 0.02, sgn * wv * 0.9), (u - 0.02, sgn * wv * 0.9), (u - 0.04, sgn * (wv + 0.02))], c, 1.0);
        }
    }
    ell(g, pen, 0.33, 0.0, 0.14, 0.22, 0.0, c, 1.0);
    ell(g, pen, 0.34, 0.0, 0.08, 0.1, 0.0, c2, 0.6);
    for sgn in [-1.0f32, 1.0] {
        circ(g, pen, 0.44, sgn * 0.06, 0.018, INK, 1.0);
    }
}

fn isoxys(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    // 泳ぐ脚
    for i in 0..9 {
        let u = 0.22 - i as f32 * 0.055;
        let ph = (t * 9.0 - i as f32 * 0.7).sin();
        ell(g, pen, u, -0.15 - 0.02 * ph, 0.02, 0.05, 0.4 * ph, mul(c2, 2.2), 0.8);
    }
    seg(g, pen, 0.32, 0.1, 0.54, 0.15, 0.016, c, 1.0);
    seg(g, pen, -0.32, 0.1, -0.56, 0.15, 0.016, c, 1.0);
    ell(g, pen, 0.0, 0.0, 0.36, 0.17, 0.0, mul(c, 0.82), 1.0);
    ell(g, pen, 0.0, 0.02, 0.33, 0.14, 0.0, c, 1.0);
    for k in 1..4 {
        ell(g, pen, -0.02 * k as f32, 0.02, 0.33 - k as f32 * 0.06, 0.14 - k as f32 * 0.025, 0.0, if k % 2 == 0 { c } else { mul(c, 0.94) }, 1.0);
    }
    seg(g, pen, -0.33, 0.12, 0.33, 0.12, 0.008, mix(c, WHITE, 0.4), 0.6);
    // 大きな眼
    circ(g, pen, 0.38, -0.03, 0.075, c2, 1.0);
    circ(g, pen, 0.39, -0.03, 0.06, INK, 1.0);
    circ(g, pen, 0.405, -0.01, 0.018, WHITE, 0.8);
    let w = (t * 2.0).sin() * 0.03;
    taper(g, pen, &[(0.33, -0.1), (0.42, -0.14 + w), (0.46, -0.1 + w)], 0.02, 0.008, mul(c, 0.7), 1.0);
}

fn nectocaris(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let n = 14;
    for i in 0..n {
        let f = i as f32 / (n - 1) as f32;
        let u = 0.25 - f * 0.68;
        let ph = (t * 7.0 - i as f32 * 0.8).sin();
        let w = 0.1 * (1.0 - (f - 0.35).abs());
        ell(g, pen, u, w + 0.02 + 0.015 * ph, 0.035, 0.03, 0.4 * ph, mix(c2, WHITE, 0.2), 0.7);
        ell(g, pen, u, -w - 0.02 - 0.015 * ph, 0.035, 0.03, -0.4 * ph, mix(c2, WHITE, 0.2), 0.7);
    }
    poly(g, pen, &[(0.32, 0.0), (0.12, 0.1), (-0.1, 0.09), (-0.46, 0.0), (-0.1, -0.09), (0.12, -0.1)], c, 1.0);
    seg(g, pen, 0.28, 0.0, -0.42, 0.0, 0.012, mul(c, 0.75), 0.8);
    ell(g, pen, 0.34, 0.0, 0.06, 0.045, 0.0, c, 1.0);
    for sgn in [-1.0f32, 1.0] {
        let w = (t * 2.2 + sgn).sin() * 0.03;
        taper(g, pen, &[(0.38, sgn * 0.01 - 0.02), (0.48, sgn * 0.03 - 0.05 + w), (0.6, sgn * 0.02 - 0.08 + w * 2.0)], 0.018, 0.005, mul(c, 0.85), 1.0);
    }
    seg(g, pen, 0.36, 0.03, 0.41, 0.09, 0.01, mul(c, 0.8), 1.0);
    circ(g, pen, 0.415, 0.095, 0.02, INK, 1.0);
}

fn fish(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    let n = 18;
    let thick = match k {
        0 => 0.075,
        1 => 0.06,
        _ => 0.045,
    };
    let mut top = Vec::new();
    let mut bot = Vec::new();
    let mut mid = Vec::new();
    for i in 0..n {
        let f = i as f32 / (n - 1) as f32;
        let u = 0.5 - f;
        let wave = (f * 7.0 - t * 9.0).sin() * 0.05 * f;
        let prof = if f < 0.12 { (f / 0.12).sqrt() } else { 1.0 - (f - 0.12) / 0.88 * 0.85 };
        let w = thick * prof;
        mid.push((u, wave));
        top.push((u, wave + w));
        bot.push((u, wave - w));
    }
    // 背びれ・尾びれ
    if k != 1 {
        for i in 4..n {
            let f = i as f32 / (n - 1) as f32;
            let h = if k == 0 { 0.035 } else { 0.02 } * (1.0 - (f - 0.7).abs() * 1.2).max(0.0);
            let (u, v) = top[i];
            seg(g, pen, u, v, u - 0.02, v + h, 0.02, c2, 0.8);
        }
    }
    if k == 2 {
        let (u, v) = mid[n - 1];
        for j in -2..=2 {
            seg(g, pen, u + 0.03, v, u - 0.06, v + j as f32 * 0.025, 0.008, c2, 0.8);
        }
    }
    let col = ca(c, pen.a);
    for i in 0..n - 1 {
        let (a0, a1, b0, b1) = (pen.p(top[i].0, top[i].1), pen.p(top[i + 1].0, top[i + 1].1), pen.p(bot[i].0, bot[i].1), pen.p(bot[i + 1].0, bot[i + 1].1));
        g.tri(a0, a1, b1, col);
        g.tri(a0, b1, b0, col);
    }
    // 筋肉の節（くの字）
    for i in 3..n - 2 {
        let (u, v) = mid[i];
        let w = (top[i].1 - v) * 0.9;
        seg(g, pen, u, v + w, u + 0.02, v, 0.005, mul(c, 0.78), 0.7);
        seg(g, pen, u + 0.02, v, u, v - w, 0.005, mul(c, 0.78), 0.7);
    }
    // えら
    if k == 0 {
        for j in 0..5 {
            let u = 0.36 - j as f32 * 0.025;
            circ(g, pen, u, mid[2].1 - 0.02, 0.008, mul(c2, 0.8), 0.9);
        }
    }
    // 眼
    let er = if k == 2 { 0.022 } else { 0.014 };
    circ(g, pen, 0.45, mid[1].1 + 0.012, er, INK, 1.0);
    circ(g, pen, 0.455, mid[1].1 + 0.018, er * 0.35, WHITE, 0.8);
    if k == 1 {
        for j in 0..2 {
            seg(g, pen, 0.49, mid[0].1 + 0.005 - j as f32 * 0.02, 0.54, mid[0].1 + 0.02 - j as f32 * 0.05, 0.005, c2, 0.9);
        }
    }
}

fn jelly(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let pulse = (t * 2.6).sin();
    let rx = 0.3 * (1.0 + 0.1 * pulse);
    let ry = 0.24 * (1.0 - 0.1 * pulse);
    let p0 = Pen { fx: 1.0, ..*pen };
    // 触手
    for i in 0..16 {
        let f = i as f32 / 15.0;
        let u = -rx + f * rx * 2.0;
        let mut pts = Vec::new();
        for j in 0..6 {
            let d = j as f32 / 5.0;
            pts.push((u * (1.0 - d * 0.2) + (t * 1.6 + f * 5.0 + d * 3.0).sin() * 0.03 * d, -d * 0.3));
        }
        taper(g, &p0, &pts, 0.012, 0.003, c2, 0.5);
    }
    let mut pts = vec![(0.0, 0.0)];
    for i in 0..=16 {
        let a = PI * i as f32 / 16.0;
        pts.push((rx * a.cos(), ry * a.sin()));
    }
    poly(g, &p0, &pts, c, 0.42);
    let mut pts2 = vec![(0.0, ry * 0.2)];
    for i in 0..=16 {
        let a = PI * i as f32 / 16.0;
        pts2.push((rx * 0.7 * a.cos(), ry * 0.2 + ry * 0.65 * a.sin()));
    }
    poly(g, &p0, &pts2, mix(c, WHITE, 0.4), 0.35);
    for i in 0..4 {
        let a = PI * (0.2 + i as f32 * 0.2);
        seg(g, &p0, 0.0, ry * 0.15, rx * 0.8 * a.cos(), ry * 0.8 * a.sin(), 0.012, c2, 0.6);
    }
    seg(g, &p0, -rx, 0.0, rx, 0.0, 0.02, mix(c2, WHITE, 0.3), 0.6);
}

fn radiolarian(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let p0 = Pen { sq: 1.0, ..*pen };
    let r = (t * 0.6).sin() * 0.2;
    for i in 0..10 {
        let a = i as f32 / 10.0 * TAU + r;
        seg(g, &p0, 0.0, 0.0, a.cos() * 0.5, a.sin() * 0.5, 0.03, c2, 0.6);
    }
    circ(g, &p0, 0.0, 0.0, 0.25, c, 0.55);
    circ(g, &p0, 0.0, 0.0, 0.16, mix(c, WHITE, 0.6), 0.6);
    circ(g, &p0, 0.0, 0.0, 0.07, [0.98, 0.72, 0.4], 0.8);
}

fn sponge(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32, seed: u32) {
    let p0 = Pen { fx: 1.0, ..*pen };
    let sway = (t * 0.7 + seed as f32).sin() * 0.03;
    if k == 2 {
        stromatoporoid(g, pen, c, c2, seed);
        return;
    }
    if k == 1 {
        // つぼ形のガラス海綿
        let mut pts = vec![(0.0, -0.45)];
        let prof = |f: f32| 0.06 + 0.22 * (f * PI * 0.85).sin().max(0.0) + f * 0.04;
        for i in 0..=10 {
            let f = i as f32 / 10.0;
            pts.push((prof(f) + sway * f, -0.45 + f * 0.95));
        }
        for i in (0..=10).rev() {
            let f = i as f32 / 10.0;
            pts.push((-prof(f) + sway * f, -0.45 + f * 0.95));
        }
        poly(g, &p0, &pts, c, 0.75);
        for i in 1..10 {
            let f = i as f32 / 10.0;
            let w = prof(f);
            seg(g, &p0, -w + sway * f, -0.45 + f * 0.95, w + sway * f, -0.45 + f * 0.95, 0.008, c2, 0.6);
        }
        for j in -3..=3 {
            let x = j as f32 * 0.06;
            seg(g, &p0, x * 0.4, -0.45, x + sway, 0.48, 0.008, c2, 0.5);
        }
        ell(g, &p0, sway, 0.5, prof(1.0), 0.03, 0.0, mul(c2, 0.6), 0.8);
        return;
    }
    // 枝分かれするバウキシア
    fn branch(g: &mut Gfx, p: &Pen, x: f32, y: f32, ang: f32, len: f32, w: f32, depth: u32, seed: u32, c: [f32; 3], c2: [f32; 3], sway: f32) {
        let a = ang + sway;
        let (ex, ey) = (x + a.sin() * len, y + a.cos() * len);
        taper(g, p, &[(x, y), ((x + ex) * 0.5 + 0.01, (y + ey) * 0.5), (ex, ey)], w, w * 0.8, c, 1.0);
        seg(g, p, x + 0.005, y, ex + 0.005, ey, w * 0.25, c2, 0.5);
        if depth > 0 {
            let h = hash2(seed as i32, depth as i32, 7);
            branch(g, p, ex, ey, ang - 0.35 - h * 0.2, len * 0.72, w * 0.78, depth - 1, seed.wrapping_mul(31).wrapping_add(1), c, c2, sway * 1.2);
            branch(g, p, ex, ey, ang + 0.3 + h * 0.25, len * 0.68, w * 0.78, depth - 1, seed.wrapping_mul(17).wrapping_add(3), c, c2, sway * 1.2);
        } else {
            ell(g, p, ex, ey, w * 0.5, w * 0.25, 0.0, mul(c2, 0.6), 0.9);
        }
    }
    branch(g, &p0, 0.0, -0.45, 0.0, 0.35, 0.11, 2, seed, c, c2, sway);
}

fn archaeocyath(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], seed: u32) {
    let p0 = Pen { fx: 1.0, ..*pen };
    let n = 2 + (seed % 3) as usize;
    for i in 0..n {
        let h = hash2(seed as i32, i as i32, 3);
        let x = (i as f32 - (n - 1) as f32 * 0.5) * 0.28 + (h - 0.5) * 0.08;
        let ht = 0.55 + h * 0.4;
        let w0 = 0.04;
        let w1 = 0.12 + h * 0.06;
        let y0 = -0.45;
        let lean = (h - 0.5) * 0.15;
        let col = mix(c, c2, h * 0.5);
        poly(g, &p0, &[(x - w0, y0), (x + w0, y0), (x + lean + w1, y0 + ht), (x + lean - w1, y0 + ht)], col, 1.0);
        for r in 1..5 {
            let f = r as f32 / 5.0;
            let w = w0 + (w1 - w0) * f;
            let yy = y0 + ht * f;
            for k in -2..=2 {
                circ(g, &p0, x + lean * f + k as f32 * w * 0.38, yy, 0.012, mul(col, 0.65), 0.9);
            }
        }
        ell(g, &p0, x + lean, y0 + ht, w1, 0.035, 0.0, mul(col, 1.1), 1.0);
        ell(g, &p0, x + lean, y0 + ht, w1 * 0.7, 0.022, 0.0, mul(c2, 0.4), 1.0);
    }
}

fn stromatolite(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], seed: u32) {
    let p0 = Pen { fx: 1.0, ..*pen };
    let n = 1 + (seed % 3) as usize;
    for i in 0..n {
        let h = hash2(seed as i32, i as i32, 11);
        let x = if i == 0 { 0.0 } else { (h - 0.5) * 0.8 };
        let w = if i == 0 { 0.5 } else { 0.18 + h * 0.15 };
        let ht = w * (0.6 + h * 0.4);
        for k in 0..6 {
            let f = 1.0 - k as f32 * 0.15;
            let col = if k % 2 == 0 { c } else { mul(c, 0.86) };
            let mut pts = vec![(x, -0.2)];
            for j in 0..=14 {
                let a = PI * j as f32 / 14.0;
                pts.push((x + w * f * a.cos(), -0.2 + ht * f * a.sin()));
            }
            poly(g, &p0, &pts, col, 1.0);
        }
        // 表面の微生物の膜
        let mut prev = (x - w, -0.2);
        for j in 1..=14 {
            let a = PI - PI * j as f32 / 14.0;
            let q = (x + w * a.cos(), -0.2 + ht * a.sin());
            seg(g, &p0, prev.0, prev.1, q.0, q.1, 0.03, c2, 0.9);
            prev = q;
        }
    }
}

fn mat(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], seed: u32) {
    for i in 0..9 {
        let h = hash2(seed as i32, i as i32, 13);
        let h2 = hash2(seed as i32, i as i32, 17);
        let u = (h - 0.5) * 0.8;
        let v = (h2 - 0.5) * 0.6;
        ell(g, pen, u, v, 0.14 + h2 * 0.1, 0.1 + h * 0.08, h * 3.0, if i % 2 == 0 { c } else { c2 }, 0.85);
    }
    for i in 0..6 {
        let h = hash2(seed as i32, i as i32, 19);
        ell(g, pen, (h - 0.5) * 0.7, (hash2(seed as i32, i as i32, 23) - 0.5) * 0.5, 0.05, 0.04, 0.0, mix(c, [0.2, 0.36, 0.18], 0.6), 0.8);
    }
}

fn tubes(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], seed: u32) {
    let p0 = Pen { fx: 1.0, ..*pen };
    for i in 0..6 {
        let h = hash2(seed as i32, i as i32, 29);
        let x = (i as f32 - 2.5) * 0.13 + (h - 0.5) * 0.05;
        let ht = 0.45 + h * 0.45;
        let bend = (h - 0.5) * 0.2;
        let pts = [(x, -0.45), (x + bend * 0.3, -0.45 + ht * 0.5), (x + bend, -0.45 + ht)];
        taper(g, &p0, &pts, 0.05, 0.09, c, 1.0);
        for r in 1..5 {
            let f = r as f32 / 5.0;
            let (rx, ry) = (x + bend * f * f, -0.45 + ht * f);
            seg(g, &p0, rx - 0.03, ry, rx + 0.03, ry, 0.008, c2, 0.7);
        }
        ell(g, &p0, x + bend, -0.45 + ht, 0.045, 0.02, 0.0, mul(c2, 0.5), 1.0);
    }
}

fn brachiopod(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let p0 = Pen { fx: 1.0, ..*pen };
    let open = ((t * 0.5).sin() * 0.5 + 0.5) * 0.06;
    taper(g, &p0, &[(0.0, -0.45), (0.02, -0.3), (0.0, -0.15)], 0.06, 0.05, c2, 1.0);
    ell(g, &p0, -open, 0.12, 0.14, 0.27, 0.1, mul(c, 0.8), 1.0);
    if open > 0.02 {
        ell(g, &p0, 0.0, 0.2, 0.05, 0.16, 0.0, [0.95, 0.85, 0.7], 0.8);
    }
    ell(g, &p0, open, 0.12, 0.14, 0.27, -0.1, c, 1.0);
    for k in 1..4 {
        let f = 1.0 - k as f32 * 0.22;
        ell(g, &p0, open, 0.12 - (1.0 - f) * 0.2, 0.14 * f, 0.27 * f, -0.1, if k % 2 == 0 { c } else { mix(c, WHITE, 0.15) }, 1.0);
    }
}

fn worm(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32, state: f32) {
    let p0 = Pen { fx: pen.fx, sq: 1.0, ..*pen };
    // 巣穴の口
    let ph = Pen { sq: 0.4, ..p0 };
    ell(g, &ph, 0.0, 0.0, 0.14, 0.12, 0.0, [0.12, 0.1, 0.08], 0.9);
    if state <= 0.02 {
        return;
    }
    let len = 0.75 * state;
    let n = 14;
    let mut pts = Vec::new();
    for i in 0..n {
        let f = i as f32 / (n - 1) as f32;
        let d = f * len;
        let bend = (t * 1.3).sin() * 0.25 + 0.35;
        pts.push(((d * bend * 1.2).sin() * d * 0.6, d * 0.9));
    }
    for (i, &(u, v)) in pts.iter().enumerate() {
        let f = i as f32 / (n - 1) as f32;
        circ(g, &p0, u, v - 0.02, 0.06 - f * 0.012, if i % 2 == 0 { c } else { mul(c, 0.9) }, 1.0);
    }
    let (eu, ev) = pts[n - 1];
    // 棘の並んだ吻
    for k in 0..6 {
        let a = k as f32 / 6.0 * TAU + t * 2.0;
        seg(g, &p0, eu, ev, eu + a.cos() * 0.07, ev + a.sin() * 0.07, 0.012, c2, 0.9);
    }
    circ(g, &p0, eu, ev, 0.03, mul(c2, 0.6), 1.0);
}

fn trace(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], seed: u32) {
    let mut u = -0.48;
    let mut v = 0.0;
    let mut k = 0;
    while u < 0.45 {
        let h = hash2(seed as i32, k, 31);
        let nu = u + 0.07 + h * 0.03;
        let nv = if k % 2 == 0 { 0.12 } else { -0.12 } + (h - 0.5) * 0.08;
        taper(g, pen, &[(u, v), ((u + nu) * 0.5, (v + nv) * 0.5 + 0.04), (nu, nv)], 0.05, 0.05, c, 0.9);
        taper(g, pen, &[(u, v + 0.01), ((u + nu) * 0.5, (v + nv) * 0.5 + 0.05), (nu, nv + 0.01)], 0.02, 0.02, mix(c, WHITE, 0.2), 0.4);
        u = nu;
        v = nv;
        k += 1;
    }
    // 掘った虫（先に少しだけ見える）
    circ(g, pen, u, v, 0.04, c2, 0.9);
}

fn slug(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let wave = (t * 2.0).sin() * 0.02;
    ell(g, pen, 0.0, 0.0, 0.48, 0.2 + wave, 0.0, mul(c, 0.8), 1.0);
    ell(g, pen, -0.02, 0.0, 0.42, 0.16 - wave * 0.5, 0.0, c, 1.0);
    for i in 0..7 {
        let u = 0.3 - i as f32 * 0.09;
        seg(g, pen, u, 0.12, u - 0.02, -0.12, 0.012, c2, 0.5);
    }
    for sgn in [-1.0f32, 1.0] {
        seg(g, pen, 0.42, sgn * 0.05, 0.52, sgn * 0.1, 0.02, c2, 1.0);
        circ(g, pen, 0.52, sgn * 0.1, 0.018, INK, 0.9);
    }
}

// ============================================================ オルドビス紀〜ペルム紀
// 画像（assets/units/<slug>.png）が無いあいだの、手描きの代わりの絵

/// 外形の上下（u ごとの厚み）から、くねる魚の体を塗る。中心線と上下の縁を返す
fn body_strip(g: &mut Gfx, pen: &Pen, n: usize, prof: &dyn Fn(f32) -> (f32, f32), wave: &dyn Fn(f32) -> f32, c: [f32; 3]) -> (Vec<(f32, f32)>, Vec<(f32, f32)>, Vec<(f32, f32)>) {
    let (mut top, mut bot, mut mid) = (Vec::new(), Vec::new(), Vec::new());
    for i in 0..n {
        let f = i as f32 / (n - 1) as f32;
        let u = 0.5 - f;
        let w = wave(f);
        let (up, dn) = prof(f);
        mid.push((u, w));
        top.push((u, w + up));
        bot.push((u, w - dn));
    }
    let col = ca(c, pen.a);
    for i in 0..n - 1 {
        let (a0, a1, b0, b1) = (pen.p(top[i].0, top[i].1), pen.p(top[i + 1].0, top[i + 1].1), pen.p(bot[i].0, bot[i].1), pen.p(bot[i + 1].0, bot[i + 1].1));
        g.tri(a0, a1, b1, col);
        g.tri(a0, b1, b0, col);
    }
    (top, bot, mid)
}

fn eye(g: &mut Gfx, pen: &Pen, u: f32, v: f32, r: f32) {
    circ(g, pen, u, v, r, INK, 1.0);
    circ(g, pen, u + r * 0.3, v + r * 0.4, r * 0.35, WHITE, 0.8);
}

/// 尾びれ（上下の葉。hetero > 0 で上の葉が長いサメ型）
fn tail_fin(g: &mut Gfx, pen: &Pen, u: f32, v: f32, h: f32, hetero: f32, c: [f32; 3]) {
    poly(g, pen, &[(u + 0.03, v), (u - 0.1, v + h * (1.0 + hetero)), (u - 0.05, v), (u - 0.09, v - h * (1.0 - hetero * 0.5))], c, 1.0);
}

pub(crate) fn cephalopod(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    let sway = (t * 2.2).sin();
    // 触手
    let head_u = if k == 0 { 0.26 } else { 0.22 };
    for j in 0..6 {
        let jv = (j as f32 - 2.5) * 0.018;
        let ph = t * 3.0 + j as f32;
        let pts = [(head_u + 0.05, jv), (head_u + 0.13, jv * 1.6 + ph.sin() * 0.02), (head_u + 0.22, jv * 2.2 + (ph + 1.0).sin() * 0.035 + sway * 0.01)];
        taper(g, pen, &pts, 0.022, 0.006, mix(c, c2, 0.3), 1.0);
    }
    ell(g, pen, head_u + 0.04, 0.0, 0.06, 0.05, 0.0, mix(c, WHITE, 0.15), 1.0);
    eye(g, pen, head_u + 0.06, 0.02, 0.014);
    if k == 0 {
        // まっすぐな円すいの殻
        let n = 14;
        let mut pts = vec![(-0.5, 0.0)];
        for i in 0..=n {
            let f = i as f32 / n as f32;
            pts.push((-0.5 + f * 0.76, 0.012 + f * 0.075));
        }
        for i in (0..=n).rev() {
            let f = i as f32 / n as f32;
            pts.push((-0.5 + f * 0.76, -0.012 - f * 0.075));
        }
        poly(g, pen, &pts, c, 1.0);
        for i in 1..12 {
            let f = i as f32 / 12.0;
            let u = -0.5 + f * 0.76;
            let w = 0.012 + f * 0.075;
            seg(g, pen, u, w, u + 0.015, -w, 0.004, mul(c, 0.8), 0.7);
            if i % 2 == 0 {
                seg(g, pen, u - 0.02, w * 0.95, u + 0.02, w * 0.95, 0.012, c2, 0.8);
            }
        }
        seg(g, pen, -0.48, 0.0, 0.25, 0.0, 0.008, mul(c, 0.7), 0.4);
    } else {
        // 巻いた殻（アンモノイド）
        let (cu, cv) = (-0.08, 0.0);
        for r in 0..5 {
            let rr = 0.3 - r as f32 * 0.06;
            ell(g, pen, cu + r as f32 * 0.01, cv, rr, rr, 0.0, if r % 2 == 0 { c } else { mix(c, c2, 0.25) }, 1.0);
        }
        for i in 0..14 {
            let a = i as f32 / 14.0 * TAU;
            let (sa, ca_) = a.sin_cos();
            seg(g, pen, cu + ca_ * 0.2, cv + sa * 0.2, cu + ca_ * 0.3, cv + sa * 0.3, 0.008, mul(c2, 0.9), 0.7);
        }
        ell(g, pen, cu + 0.04, cv, 0.04, 0.04, 0.0, mul(c, 0.7), 1.0);
    }
}

pub(crate) fn eurypterid(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    let stroke = (t * 3.0).sin();
    let dark = mul(c, 0.7);
    // 泳ぐための脚（オール）
    for sgn in [-1.0f32, 1.0] {
        let a = 0.25 + stroke * 0.15 * sgn;
        taper(g, pen, &[(0.22, sgn * 0.06), (0.12, sgn * (0.16 + a * 0.1)), (0.02, sgn * (0.2 + a * 0.12))], 0.03, 0.05, dark, 1.0);
        // 歩く脚
        for j in 0..3 {
            let u = 0.36 - j as f32 * 0.04;
            let w = (t * 4.0 + j as f32).sin() * 0.01;
            seg(g, pen, u, sgn * 0.06, u + 0.05 + w, sgn * 0.13, 0.012, dark, 1.0);
            if k == 2 {
                for q in 0..3 {
                    let f = q as f32 / 3.0;
                    let (bu, bv) = (u + (0.05) * f, sgn * (0.06 + 0.07 * f));
                    seg(g, pen, bu, bv, bu + 0.03, bv + sgn * 0.01, 0.005, c2, 0.9);
                }
            }
        }
        if k == 1 {
            // 大きなはさみ
            let open = (t * 1.5).sin() * 0.02 + 0.03;
            taper(g, pen, &[(0.4, sgn * 0.05), (0.52, sgn * 0.12), (0.66, sgn * 0.1)], 0.03, 0.025, c, 1.0);
            seg(g, pen, 0.66, sgn * 0.1, 0.78, sgn * (0.1 + open), 0.014, c2, 1.0);
            seg(g, pen, 0.66, sgn * 0.1, 0.78, sgn * (0.1 - open), 0.014, c2, 1.0);
        }
    }
    // 体の節（後ろほど細い）
    for i in 0..12 {
        let f = i as f32 / 11.0;
        let u = 0.28 - f * 0.66;
        let w = 0.11 * (1.0 - f * 0.75);
        ell(g, pen, u, 0.0, 0.035, w, 0.0, if i % 2 == 0 { c } else { mix(c, c2, 0.3) }, 1.0);
    }
    // 尾のとげ
    let tail = if k == 1 { 0.06 } else { 0.0 };
    poly(g, pen, &[(-0.4, 0.015 + tail), (-0.56, 0.0), (-0.4, -0.015 - tail)], dark, 1.0);
    // 頭の甲羅
    ell(g, pen, 0.38, 0.0, 0.1, 0.11, 0.0, c, 1.0);
    ell(g, pen, 0.39, 0.0, 0.07, 0.08, 0.0, mix(c, WHITE, 0.1), 1.0);
    for sgn in [-1.0f32, 1.0] {
        circ(g, pen, 0.42, sgn * 0.06, 0.016, INK, 1.0);
    }
}

pub(crate) fn crinoid(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32, seed: u32) {
    let p0 = Pen { fx: 1.0, ..*pen };
    let sway = (t * 0.8 + seed as f32 * 0.37).sin() * 0.04;
    let top = 0.12;
    // 茎（節が並ぶ）
    let n = 14;
    for i in 0..n {
        let f0 = i as f32 / n as f32;
        let f1 = (i + 1) as f32 / n as f32;
        let (u0, v0) = (sway * f0 * f0, -0.5 + (top + 0.5) * f0);
        let (u1, v1) = (sway * f1 * f1, -0.5 + (top + 0.5) * f1);
        taper(g, &p0, &[(u0, v0), (u1, v1)], 0.035, 0.035, if i % 2 == 0 { c } else { mul(c, 0.85) }, 1.0);
    }
    // がく
    ell(g, &p0, sway, top + 0.04, 0.06, 0.05, 0.0, mul(c, 0.9), 1.0);
    // 腕（羽のような小枝）
    for j in 0..10 {
        let a = -1.25 + j as f32 / 9.0 * 2.5;
        let wob = (t * 1.6 + j as f32).sin() * 0.06;
        let (ex, ey) = (sway + (a + wob).sin() * 0.3, top + 0.06 + (a + wob).cos() * 0.33);
        let mx = (sway + ex) * 0.5 + a.sin() * 0.03;
        let my = (top + 0.06 + ey) * 0.5 + 0.04;
        taper(g, &p0, &[(sway, top + 0.06), (mx, my), (ex, ey)], 0.025, 0.008, c2, 1.0);
        for q in 1..5 {
            let f = q as f32 / 5.0;
            let (bx, by) = (sway + (ex - sway) * f, top + 0.06 + (ey - top - 0.06) * f);
            seg(g, &p0, bx, by, bx + 0.03 * (a + 1.6).cos(), by + 0.025, 0.006, mix(c2, WHITE, 0.2), 0.7);
        }
    }
}

pub(crate) fn coral(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32, seed: u32) {
    let p0 = Pen { fx: 1.0, ..*pen };
    match k {
        0 => {
            // 角のような単体サンゴ
            let n = 12;
            let mut l = Vec::new();
            let mut r = Vec::new();
            for i in 0..=n {
                let f = i as f32 / n as f32;
                let bend = (f * 1.2).sin() * 0.18;
                let w = 0.02 + f * 0.17;
                l.push((bend - w, -0.5 + f * 0.75));
                r.push((bend + w, -0.5 + f * 0.75));
            }
            let mut pts = l.clone();
            pts.extend(r.iter().rev());
            poly(g, &p0, &pts, c, 1.0);
            for i in 1..n {
                seg(g, &p0, l[i].0, l[i].1, r[i].0, r[i].1, 0.006, mul(c, 0.82), 0.6);
            }
            let (tu, tv) = ((l[n].0 + r[n].0) * 0.5, l[n].1);
            ell(g, &p0, tu, tv, 0.19, 0.04, 0.0, mul(c, 0.7), 1.0);
            for j in 0..9 {
                let a = -1.2 + j as f32 * 0.3 + (t * 1.5 + j as f32).sin() * 0.08;
                seg(g, &p0, tu + a * 0.12, tv, tu + a.sin() * 0.22, tv + 0.1 + a.cos() * 0.08, 0.014, c2, 0.9);
            }
        }
        1 => {
            // 蜂の巣のような群体（ドーム）
            dome(g, &p0, 0.0, -0.5, 0.46, 0.48, mul(c, 0.85), 1.0);
            dome(g, &p0, 0.0, -0.5, 0.42, 0.44, c, 1.0);
            for row in 0..5 {
                let v = -0.44 + row as f32 * 0.08;
                let half = 0.38 * (1.0 - ((v + 0.5) / 0.44).powi(2)).max(0.0).sqrt();
                let mut u = -half + if row % 2 == 0 { 0.0 } else { 0.035 };
                while u < half {
                    circ(g, &p0, u, v, 0.022, mul(c2, 0.9), 0.85);
                    u += 0.07;
                }
            }
        }
        _ => {
            // 枝分かれするコケムシ
            fn br(g: &mut Gfx, p: &Pen, x: f32, y: f32, ang: f32, len: f32, w: f32, d: u32, seed: u32, c: [f32; 3], c2: [f32; 3]) {
                let (ex, ey) = (x + ang.sin() * len, y + ang.cos() * len);
                taper(g, p, &[(x, y), (ex, ey)], w, w * 0.85, c, 1.0);
                for q in 1..4 {
                    let f = q as f32 / 4.0;
                    circ(g, p, x + (ex - x) * f, y + (ey - y) * f, w * 0.2, c2, 0.8);
                }
                if d > 0 {
                    let h = hash2(seed as i32, d as i32, 3);
                    br(g, p, ex, ey, ang - 0.45 - h * 0.2, len * 0.75, w * 0.8, d - 1, seed.wrapping_mul(13).wrapping_add(1), c, c2);
                    br(g, p, ex, ey, ang + 0.4 + h * 0.2, len * 0.75, w * 0.8, d - 1, seed.wrapping_mul(7).wrapping_add(5), c, c2);
                }
            }
            br(g, &p0, 0.0, -0.5, 0.0, 0.3, 0.07, 3, seed, c, c2);
        }
    }
}

pub(crate) fn graptolite(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let sway = (t * 0.9).sin() * 0.1;
    // 浮き（中心）から二本の枝が垂れ下がる
    circ(g, pen, 0.0, 0.2, 0.03, c2, 0.9);
    seg(g, pen, 0.0, 0.2, 0.0, 0.3, 0.006, c2, 0.8);
    for sgn in [-1.0f32, 1.0] {
        let n = 10;
        for i in 0..n {
            let f0 = i as f32 / n as f32;
            let f1 = (i + 1) as f32 / n as f32;
            let p = |f: f32| (sgn * f * 0.48, 0.2 - f * (0.35 + sgn * sway * 0.3) - f * f * 0.1);
            let (a, b) = (p(f0), p(f1));
            seg(g, pen, a.0, a.1, b.0, b.1, 0.018, c, 0.95);
            // 小さな個体の入る杯（歯のように並ぶ）
            poly(g, pen, &[(a.0, a.1), (b.0, b.1), ((a.0 + b.0) * 0.5 + sgn * 0.01, (a.1 + b.1) * 0.5 + 0.035)], mix(c, c2, 0.2), 0.9);
        }
    }
}

pub(crate) fn jawless(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    let wv = |f: f32| (f * 6.0 - t * 8.0).sin() * 0.04 * f * f;
    if k == 0 {
        // サカバンバスピス：前が大きな骨の盾、眼は顔の前、口は開いたまま
        let prof = |f: f32| {
            let h = if f < 0.5 { 0.11 * (1.0 - ((0.25 - f) / 0.25).powi(2) * 0.5) } else { 0.11 * (1.0 - (f - 0.5) / 0.5 * 0.85) };
            (h, h * 0.9)
        };
        let (top, _, mid) = body_strip(g, pen, 18, &prof, &wv, c);
        tail_fin(g, pen, mid[17].0, mid[17].1, 0.06, 0.0, mul(c, 0.8));
        ell(g, pen, 0.22, 0.0, 0.27, 0.115, 0.0, mix(c, c2, 0.3), 1.0);
        for i in 0..6 {
            let u = 0.05 - i as f32 * 0.06;
            seg(g, pen, u, top[8 + i.min(8)].1 * 0.8, u - 0.02, 0.0, 0.006, mul(c, 0.75), 0.7);
        }
        ell(g, pen, 0.47, -0.02, 0.025, 0.03, 0.0, INK, 0.9);
        eye(g, pen, 0.44, 0.045, 0.018);
        eye(g, pen, 0.445, -0.005, 0.012);
    } else {
        let prof = |f: f32| {
            let h = if f < 0.1 { 0.05 * (f / 0.1).sqrt() } else { 0.05 * (1.0 - (f - 0.1) / 0.9 * 0.7) };
            (h, h)
        };
        let (top, _, mid) = body_strip(g, pen, 18, &prof, &wv, c);
        for i in 3..16 {
            let (u, v) = mid[i];
            seg(g, pen, u, top[i].1, u + 0.015, v, 0.004, mul(c, 0.7), 0.7);
        }
        let (u, v) = mid[17];
        poly(g, pen, &[(u + 0.02, v), (u - 0.08, v - 0.08), (u - 0.06, v + 0.02)], mul(c, 0.8), 1.0);
        eye(g, pen, 0.44, mid[1].1 + 0.012, 0.012);
        for i in 0..6 {
            circ(g, pen, 0.36 - i as f32 * 0.02, mid[2].1 - 0.015, 0.006, c2, 0.8);
        }
    }
}

pub(crate) fn placoderm(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    let wv = |f: f32| (f * 6.0 - t * 7.0).sin() * 0.04 * f * f;
    let (hu, hh) = match k {
        1 => (0.36, 0.16),
        2 => (0.42, 0.1),
        _ => (0.4, 0.12),
    };
    let prof = move |f: f32| {
        let h = if f < 0.15 { hh * (0.6 + 0.4 * f / 0.15) } else { hh * (1.0 - (f - 0.15) / 0.85 * 0.88) };
        (h, h * 0.85)
    };
    let (_, _, mid) = body_strip(g, pen, 18, &prof, &wv, c);
    let (u, v) = mid[17];
    tail_fin(g, pen, u, v, hh * 0.6, 0.5, mul(c, 0.8));
    // 背びれ
    poly(g, pen, &[(0.0, mid[8].1 + hh * 0.75), (-0.12, mid[10].1 + hh * 0.95), (-0.14, mid[10].1 + hh * 0.5)], mul(c, 0.85), 1.0);
    // 頭と胸の骨の鎧
    let armour = mix(c2, c, 0.35);
    ell(g, pen, 0.5 - hu * 0.5, 0.01, hu * 0.55, hh * 1.05, 0.0, armour, 1.0);
    seg(g, pen, 0.5 - hu, -hh * 0.8, 0.5 - hu, hh * 0.9, 0.012, mul(armour, 0.7), 0.9);
    seg(g, pen, 0.5 - hu * 0.55, -hh * 0.9, 0.5 - hu * 0.55, hh, 0.008, mul(armour, 0.75), 0.7);
    match k {
        1 => {
            // 刃のようなあご
            let open = ((t * 0.9).sin() * 0.5 + 0.5) * 0.06;
            poly(g, pen, &[(0.5, -0.02), (0.3, -0.05 - open), (0.36, -0.1 - open)], mul(c2, 0.9), 1.0);
            poly(g, pen, &[(0.52, 0.0), (0.48, -0.03), (0.44, 0.01)], WHITE, 0.9);
            eye(g, pen, 0.42, 0.06, 0.022);
        }
        2 => {
            // 細い「腕」の胸びれ
            for sgn in [0.0f32, 1.0] {
                let w = (t * 2.0 + sgn).sin() * 0.03;
                taper(g, pen, &[(0.34, -0.06), (0.22, -0.14 + w), (0.08, -0.1 + w)], 0.03, 0.015, armour, 1.0 - sgn * 0.3);
            }
            eye(g, pen, 0.46, 0.08, 0.012);
            eye(g, pen, 0.43, 0.085, 0.012);
        }
        _ => {
            eye(g, pen, 0.44, 0.05, 0.018);
            seg(g, pen, 0.5, -0.02, 0.38, -0.05, 0.01, INK, 0.7);
        }
    }
}

pub(crate) fn shark(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    let wv = |f: f32| (f * 5.0 - t * 7.0).sin() * 0.03 * f * f;
    let hh = if k == 2 { 0.1 } else { 0.085 };
    let prof = move |f: f32| {
        let h = if f < 0.2 { hh * (f / 0.2).sqrt() } else { hh * (1.0 - (f - 0.2) / 0.8 * 0.82) };
        (h, h * 0.8)
    };
    // お腹は白っぽく
    let (top, bot, mid) = body_strip(g, pen, 18, &prof, &wv, c);
    for i in 2..16 {
        let (u, v) = mid[i];
        seg(g, pen, u, v - 0.01, u - 0.03, (v + bot[i].1) * 0.5 - 0.005, 0.03, c2, 0.5);
    }
    let (u, v) = mid[17];
    poly(g, pen, &[(u + 0.03, v), (u - 0.12, v + 0.16), (u - 0.06, v), (u - 0.1, v - 0.08)], c, 1.0);
    // 背びれ・胸びれ
    let dv = top[6].1;
    if k == 1 {
        // アイロン台のような背びれ
        poly(g, pen, &[(0.14, dv), (0.18, dv + 0.12), (0.02, dv + 0.14), (0.04, dv)], mul(c, 0.8), 1.0);
        ell(g, pen, 0.1, dv + 0.135, 0.09, 0.02, 0.0, mul(c2, 1.0), 1.0);
        ell(g, pen, 0.38, top[2].1 + 0.005, 0.07, 0.015, 0.0, mul(c2, 1.0), 1.0);
    } else {
        poly(g, pen, &[(0.12, dv), (0.0, dv + 0.13), (-0.02, dv)], mul(c, 0.85), 1.0);
    }
    poly(g, pen, &[(0.22, bot[5].1 + 0.01), (0.08, bot[5].1 - 0.1), (0.12, bot[5].1)], mul(c, 0.85), 1.0);
    // えらの切れ込み
    for j in 0..5 {
        let u = 0.33 - j as f32 * 0.025;
        seg(g, pen, u, mid[3].1 + 0.04, u - 0.005, mid[3].1 - 0.04, 0.005, mul(c, 0.6), 0.8);
    }
    eye(g, pen, 0.43, mid[1].1 + 0.025, 0.014);
    if k == 2 {
        // 下あごのうずまきの歯
        let (cu, cv) = (0.38, bot[3].1 - 0.01);
        for r in 0..4 {
            let rr = 0.06 - r as f32 * 0.013;
            for q in 0..10 {
                let a = q as f32 / 10.0 * TAU;
                let (sa, cs) = a.sin_cos();
                poly(g, pen, &[(cu + cs * rr, cv + sa * rr), (cu + cs * (rr + 0.012) - sa * 0.006, cv + sa * (rr + 0.012) + cs * 0.006), (cu + cs * rr - sa * 0.01, cv + sa * rr + cs * 0.01)], WHITE, 0.95);
            }
        }
    }
}

pub(crate) fn lobefin(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    let flat = k == 1;
    let wv = |f: f32| (f * 6.0 - t * if flat { 3.0 } else { 7.0 }).sin() * 0.035 * f * f;
    let hh = if flat { 0.07 } else { 0.085 };
    let prof = move |f: f32| {
        let h = if f < 0.15 { hh * (0.55 + 0.45 * f / 0.15) } else { hh * (1.0 - (f - 0.15) / 0.85 * 0.8) };
        (h, h * 0.9)
    };
    let (_, bot, mid) = body_strip(g, pen, 18, &prof, &wv, c);
    // 肉のついたひれ
    let fins: &[(usize, f32)] = if flat { &[(4, 0.11), (11, 0.06)] } else { &[(4, 0.07), (10, 0.06), (13, 0.05)] };
    for &(i, l) in fins {
        let (u, v) = (mid[i].0, bot[i].1);
        let w = (t * 2.0 + i as f32).sin() * 0.02;
        taper(g, pen, &[(u, v + 0.01), (u - l * 0.4, v - l * 0.7 + w), (u - l, v - l + w)], 0.045, 0.02, mix(c, c2, 0.3), 1.0);
    }
    // うろこ
    for i in 3..15 {
        let (u, v) = mid[i];
        circ(g, pen, u, v + 0.02, 0.012, mix(c, c2, 0.4), 0.5);
        circ(g, pen, u - 0.02, v - 0.02, 0.012, mix(c, c2, 0.4), 0.5);
    }
    // 三つに分かれた尾
    let (u, v) = mid[17];
    poly(g, pen, &[(u + 0.04, v), (u - 0.08, v + 0.07), (u - 0.1, v), (u - 0.08, v - 0.07)], mul(c, 0.85), 1.0);
    if flat {
        ell(g, pen, 0.42, mid[0].1 + 0.01, 0.1, 0.05, 0.0, mix(c, c2, 0.2), 1.0);
        eye(g, pen, 0.44, mid[0].1 + 0.055, 0.014);
    } else {
        eye(g, pen, 0.44, mid[1].1 + 0.02, 0.016);
    }
}

pub(crate) fn tetrapod(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    if k == 11 {
        let points: Vec<_> = (0..=24).map(|i| {
            let u = -0.48 + i as f32 * 0.038;
            (u, 0.11 + (u * 16.0 - t * 2.0).sin() * 0.055)
        }).collect();
        taper(g, pen, &points, 0.008, 0.035, c, 1.0);
        let (u,v) = points[24];
        ell(g, pen, u + 0.035, v, 0.06, 0.035, 0.0, c2, 1.0);
        eye(g, pen, u + 0.06, v + 0.012, 0.009);
        return;
    }
    if k == 12 {
        ell(g, pen, -0.03, 0.13, 0.16, 0.10, 0.0, c, 1.0);
        ell(g, pen, 0.16, 0.16, 0.12, 0.08, 0.0, c, 1.0);
        for side in [-1.0f32, 1.0] {
            let stride = (t * 2.0).sin() * side * 0.01;
            taper(g, pen, &[(-0.10, 0.10), (-0.31, 0.04), (-0.12, 0.015), (-0.35, 0.01 + stride)], 0.05, 0.015, c2, 1.0);
            taper(g, pen, &[(0.12, 0.11), (0.23, 0.01), (0.32, 0.005)], 0.025, 0.012, c2, 1.0);
        }
        eye(g, pen, 0.20, 0.22, 0.02);
        return;
    }
    // 体の長さの中で、胴・頭・尾の割合と脚の長さ
    let (body_h, leg, head_l, tail_l, swim) = match k {
        0 => (0.07, 0.05, 0.14, 0.42, true),
        1 => (0.09, 0.08, 0.16, 0.36, false),
        2 => (0.06, 0.08, 0.12, 0.44, false),
        3 => (0.08, 0.08, 0.2, 0.32, false),
        4 => (0.09, 0.11, 0.14, 0.38, false),
        5 => (0.1, 0.09, 0.2, 0.3, false),
        6 => (0.06, 0.06, 0.12, 0.34, false),
        7 => (0.045, 0.05, 0.2, 0.42, true),
        8 => (0.1, 0.13, 0.2, 0.28, false),
        9 => (0.15, 0.11, 0.12, 0.12, false),
        _ => (0.12, 0.09, 0.14, 0.1, false),
    };
    let walk = t * 3.0;
    let y0 = leg; // 体の下の線（足の裏が v=0）
    let body_u0 = -0.5 + tail_l;
    let body_u1 = 0.5 - head_l;
    let dark = mul(c, 0.72);
    // 奥の脚
    let legs = [body_u1 - 0.05, body_u0 + 0.05];
    for (j, &lu) in legs.iter().enumerate() {
        let ph = walk + j as f32 * PI;
        let sw = if swim { 0.02 } else { ph.sin() * 0.04 };
        taper(g, pen, &[(lu, y0 + 0.02), (lu + sw * 0.5 + 0.01, y0 * 0.5), (lu + sw, 0.0)], 0.045, 0.03, dark, 1.0);
    }
    // 尾
    let n = 8;
    let mut pts = Vec::new();
    for i in 0..=n {
        let f = i as f32 / n as f32;
        let u = body_u0 - f * tail_l;
        let wob = (t * if swim { 6.0 } else { 2.0 } - f * 4.0).sin() * 0.02 * f;
        pts.push((u, y0 + body_h * 0.6 - f * body_h * 0.4 + wob));
    }
    taper(g, pen, &pts, body_h * 1.1, 0.01, c, 1.0);
    if matches!(k, 0 | 1 | 7) {
        // 尾のひれ
        let (u, v) = pts[n / 2];
        poly(g, pen, &[(u + 0.06, v + 0.02), (pts[n].0, pts[n].1 + 0.05), (pts[n].0, pts[n].1 - 0.03), (u + 0.06, v - 0.03)], mix(c, c2, 0.4), 0.8);
    }
    // 帆
    if k == 4 {
        let mut sail = vec![(body_u0 + 0.02, y0 + body_h * 1.6)];
        for i in 0..=8 {
            let f = i as f32 / 8.0;
            let u = body_u0 + 0.02 + f * (body_u1 - body_u0 - 0.04);
            sail.push((u, y0 + body_h * 1.6 + (f * PI).sin() * 0.26));
        }
        sail.push((body_u1 - 0.02, y0 + body_h * 1.6));
        poly(g, pen, &sail, c2, 0.95);
        for i in 1..8 {
            let f = i as f32 / 8.0;
            let u = body_u0 + 0.02 + f * (body_u1 - body_u0 - 0.04);
            seg(g, pen, u, y0 + body_h * 1.6, u, y0 + body_h * 1.6 + (f * PI).sin() * 0.25, 0.008, mul(c2, 0.7), 0.9);
        }
    }
    // 胴
    let bc = (body_u0 + body_u1) * 0.5;
    ell(g, pen, bc, y0 + body_h, (body_u1 - body_u0) * 0.58, body_h, 0.0, c, 1.0);
    ell(g, pen, bc, y0 + body_h * 0.7, (body_u1 - body_u0) * 0.5, body_h * 0.6, 0.0, mix(c, c2, 0.25), 1.0);
    if k == 9 {
        for i in 0..7 {
            let u = body_u0 + 0.04 + i as f32 * (body_u1 - body_u0 - 0.08) / 6.0;
            circ(g, pen, u, y0 + body_h * 1.8, 0.02, c2, 0.9);
        }
    }
    // 頭
    let hu = body_u1 + head_l * 0.45;
    let hv = y0 + body_h * if k == 10 { 0.9 } else { 1.2 };
    match k {
        6 => {
            // ブーメランの頭
            poly(g, pen, &[(body_u1 - 0.02, hv + 0.02), (0.56, hv + 0.08), (hu + 0.04, hv), (0.56, hv - 0.06), (body_u1 - 0.02, hv - 0.02)], mix(c, c2, 0.2), 1.0);
        }
        7 => {
            // 細長いあご
            ell(g, pen, hu - 0.02, hv, head_l * 0.3, body_h * 0.8, 0.0, c, 1.0);
            taper(g, pen, &[(hu, hv), (0.56, hv - 0.01)], 0.04, 0.012, c, 1.0);
            for i in 0..6 {
                let u = hu + 0.02 + i as f32 * 0.02;
                seg(g, pen, u, hv - 0.01, u, hv - 0.025, 0.004, WHITE, 0.9);
            }
        }
        _ => {
            let hr = head_l * 0.55;
            ell(g, pen, hu, hv, hr, body_h * if matches!(k, 3 | 5) { 1.0 } else { 0.85 }, -0.08, c, 1.0);
            seg(g, pen, hu + hr * 0.1, hv - body_h * 0.25, hu + hr, hv - body_h * 0.15, 0.008, INK, 0.7);
            if k == 8 {
                poly(g, pen, &[(hu + hr * 0.55, hv - body_h * 0.2), (hu + hr * 0.45, hv - body_h * 0.2 - 0.07), (hu + hr * 0.35, hv - body_h * 0.2)], WHITE, 1.0);
            }
            if k == 10 {
                poly(g, pen, &[(hu + hr * 0.6, hv - body_h * 0.1), (hu + hr * 1.1, hv - body_h * 0.4), (hu + hr * 0.7, hv - body_h * 0.5)], mul(c2, 0.8), 1.0);
                seg(g, pen, hu + hr * 0.5, hv - body_h * 0.3, hu + hr * 0.55, hv - body_h * 0.9, 0.018, WHITE, 1.0);
            }
            if k == 9 {
                for q in 0..3 {
                    circ(g, pen, hu - hr * 0.3 + q as f32 * 0.02, hv - body_h * 0.6, 0.018, c2, 1.0);
                }
            }
        }
    }
    eye(g, pen, hu + head_l * 0.12, hv + body_h * 0.35, 0.014);
    // 手前の脚
    for (j, &lu) in legs.iter().enumerate() {
        let ph = walk + j as f32 * PI + PI * 0.5;
        let sw = if swim { -0.02 } else { ph.sin() * 0.05 };
        taper(g, pen, &[(lu + 0.02, y0 + 0.03), (lu + sw * 0.5 + 0.03, y0 * 0.5), (lu + sw + 0.02, 0.0)], 0.05, 0.035, mul(c, 0.9), 1.0);
        ell(g, pen, lu + sw + 0.04, 0.0, 0.03, 0.01, 0.0, mul(c, 0.8), 1.0);
    }
}

pub(crate) fn plant(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32, seed: u32) {
    if k >= 12 {
        plant_meso(g, pen, k, c, c2, t, seed);
        return;
    }
    let p0 = Pen { fx: 1.0, ..*pen };
    let sway = (t * 0.6 + seed as f32 * 0.31).sin() * 0.015;
    let h = |i: i32, j: i32| hash2(seed as i32, i * 31 + j, 17);
    match k {
        0 | 1 | 11 => {
            // 細い茎の群れ（クックソニア・ライニア）か、葉のついた茎（バラグワナチア）
            let stems = if k == 0 { 5 } else { 7 };
            for s in 0..stems {
                let x = -0.3 + s as f32 / (stems - 1) as f32 * 0.6 + (h(s, 1) - 0.5) * 0.08;
                let top = 0.1 + h(s, 2) * 0.35;
                let lean = (h(s, 3) - 0.5) * 0.2 + sway * 3.0;
                let mid = (x + lean * 0.4, -0.5 + (top + 0.5) * 0.5);
                taper(g, &p0, &[(x, -0.5), mid], 0.03, 0.022, c, 1.0);
                if k == 11 {
                    let end = (x + lean, top);
                    taper(g, &p0, &[mid, end], 0.022, 0.016, c, 1.0);
                    for q in 0..10 {
                        let f = q as f32 / 10.0;
                        let (bx, by) = (x + lean * f, -0.5 + (top + 0.5) * f);
                        seg(g, &p0, bx, by, bx - 0.05, by + 0.04, 0.008, c2, 0.9);
                        seg(g, &p0, bx, by, bx + 0.05, by + 0.04, 0.008, c2, 0.9);
                    }
                    continue;
                }
                // Y字に分かれる
                for sgn in [-1.0f32, 1.0] {
                    let end = (mid.0 + lean * 0.6 + sgn * 0.08, top);
                    taper(g, &p0, &[mid, end], 0.022, 0.016, c, 1.0);
                    if k == 1 {
                        let e2 = (end.0 + sgn * 0.06, top + 0.1);
                        taper(g, &p0, &[end, e2], 0.016, 0.012, c, 1.0);
                        ell(g, &p0, e2.0, e2.1 + 0.03, 0.022, 0.035, 0.0, c2, 1.0);
                    } else {
                        ell(g, &p0, end.0, end.1 + 0.025, 0.03, 0.025, 0.0, c2, 1.0);
                    }
                }
            }
        }
        9 => {
            // 地面にへばりつく平たい葉状体
            for s in 0..7 {
                let x = -0.4 + s as f32 * 0.13;
                let a = (h(s, 4) - 0.5) * 0.6;
                ell(g, &p0, x, -0.44, 0.1, 0.035, a, if s % 2 == 0 { c } else { c2 }, 1.0);
                seg(g, &p0, x - 0.07, -0.44, x + 0.07, -0.43, 0.006, mul(c, 0.7), 0.6);
            }
        }
        10 => {
            // プロトタキシーテス：太い柱
            taper(g, &p0, &[(0.0, -0.5), (sway, 0.0), (sway * 2.0, 0.42)], 0.2, 0.17, c, 1.0);
            ell(g, &p0, sway * 2.0, 0.42, 0.085, 0.08, 0.0, c, 1.0);
            for i in 0..8 {
                let v = -0.45 + i as f32 * 0.11;
                seg(g, &p0, -0.09, v, 0.09, v + 0.01, 0.006, mul(c2, 1.0), 0.6);
            }
        }
        _ => {
            // 木：幹と樹冠
            let (trunk_w, crown_v) = match k {
                2 => (0.08, 0.0),
                3 => (0.1, 0.25),
                4 => (0.07, 0.3),
                5 => (0.06, -0.1),
                6 => (0.06, 0.32),
                7 => (0.07, 0.05),
                _ => (0.06, -0.05),
            };
            let top = 0.36 + if matches!(k, 4 | 6) { 0.1 } else { 0.0 };
            let tx = sway * 4.0;
            taper(g, &p0, &[(0.0, -0.5), (tx * 0.4, (crown_v - 0.5) * 0.5), (tx, top)], trunk_w * if k == 3 { 1.9 } else { 1.3 }, trunk_w * 0.7, c, 1.0);
            match k {
                4 => {
                    // うろこ模様の幹と、二つに分かれる枝
                    for i in 0..10 {
                        let v = -0.45 + i as f32 * 0.075;
                        let u = tx * ((v + 0.5) / (top + 0.5));
                        poly(g, &p0, &[(u, v - 0.02), (u + 0.025, v), (u, v + 0.02), (u - 0.025, v)], mul(c, 0.8), 0.8);
                    }
                    for sgn in [-1.0f32, 1.0] {
                        let b1 = (tx + sgn * 0.12, top + 0.06);
                        taper(g, &p0, &[(tx, top - 0.02), b1], 0.04, 0.03, c, 1.0);
                        for s2 in [-1.0f32, 1.0] {
                            let b2 = (b1.0 + sgn * 0.06 + s2 * 0.05, b1.1 + 0.07);
                            taper(g, &p0, &[b1, b2], 0.03, 0.02, c, 1.0);
                            ell(g, &p0, b2.0, b2.1 + 0.02, 0.06, 0.035, 0.0, c2, 1.0);
                        }
                    }
                }
                5 => {
                    // 節ごとに輪になった枝
                    for i in 1..9 {
                        let v = -0.5 + i as f32 * 0.11;
                        let u = tx * ((v + 0.5) / (top + 0.5));
                        seg(g, &p0, u - 0.05, v, u + 0.05, v, 0.012, mul(c, 0.7), 0.9);
                        if i > 2 {
                            let l = 0.16 - i as f32 * 0.012;
                            for sgn in [-1.0f32, 1.0] {
                                seg(g, &p0, u, v, u + sgn * l, v + 0.06, 0.01, c2, 0.9);
                            }
                        }
                    }
                }
                6 | 3 => {
                    // てっぺんから垂れる葉（木生シダ）・枝の束（エオスペルマトプテリス）
                    let n = if k == 6 { 9 } else { 12 };
                    for j in 0..n {
                        let a = -1.4 + j as f32 / (n - 1) as f32 * 2.8;
                        let l = if k == 6 { 0.32 } else { 0.2 };
                        let end = (tx + a.sin() * l, top + a.cos().max(-0.2) * l * 0.5 - if a.abs() > 0.8 { 0.1 } else { 0.0 });
                        let mid = ((tx + end.0) * 0.5, top + 0.08);
                        taper(g, &p0, &[(tx, top), mid, end], 0.025, 0.008, c2, 1.0);
                        if k == 6 {
                            for q in 1..6 {
                                let f = q as f32 / 6.0;
                                let (bx, by) = (tx + (end.0 - tx) * f, top + 0.08 * (1.0 - (f - 0.5).abs() * 2.0) + (end.1 - top) * f);
                                seg(g, &p0, bx, by, bx + 0.02, by - 0.04, 0.008, mix(c2, WHITE, 0.1), 0.8);
                            }
                        }
                    }
                }
                _ => {
                    // 水平の枝が段になった樹冠（アーケオプテリス・ワルキア・グロッソプテリス）
                    let tiers = if k == 7 { 4 } else { 6 };
                    for i in 0..tiers {
                        let f = i as f32 / (tiers - 1) as f32;
                        let v = crown_v + (top - crown_v) * f;
                        let w = if k == 7 { 0.3 - (f - 0.4).abs() * 0.25 } else { 0.32 * (1.0 - f * 0.75) };
                        let u = tx * ((v + 0.5) / (top + 0.5));
                        for sgn in [-1.0f32, 1.0] {
                            taper(g, &p0, &[(u, v), (u + sgn * w, v - 0.03)], 0.02, 0.01, mul(c, 0.9), 1.0);
                            if k == 7 {
                                for q in 0..4 {
                                    let bx = u + sgn * w * (0.3 + q as f32 * 0.22);
                                    ell(g, &p0, bx, v + 0.01, 0.025, 0.05, sgn * 0.5, c2, 1.0);
                                }
                            } else {
                                ell(g, &p0, u + sgn * w * 0.55, v + 0.01, w * 0.55, 0.035, 0.0, c2, 0.95);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn arthro(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    if k == 6 {
        let wing = (t * 16.0).sin() * 0.06;
        for side in [-1.0f32, 1.0] {
            ell(g, pen, -0.02, side * (0.15 + wing), 0.17, 0.075, side * 0.4, WHITE, 0.55);
            for j in 0..3 { let u = -0.12 + j as f32 * 0.1; seg(g, pen, u, -0.04, u - 0.05, -0.15, 0.009, INK, 1.0); }
        }
        ell(g, pen, -0.10, 0.0, 0.17, 0.08, 0.0, c2, 1.0);
        for u in [-0.20, -0.10, 0.0] { seg(g, pen, u, -0.055, u, 0.055, 0.025, c, 1.0); }
        circ(g, pen, 0.16, 0.02, 0.07, c, 1.0);
        eye(g, pen, 0.19, 0.04, 0.012);
        return;
    }
    let dark = mul(c, 0.7);
    let walk = t * 8.0;
    match k {
        0 | 1 => {
            // ヤスデ：たくさんの節と脚
            let n = if k == 0 { 14 } else { 18 };
            let h = if k == 0 { 0.05 } else { 0.07 };
            for i in 0..n {
                let f = i as f32 / (n - 1) as f32;
                let u = 0.45 - f * 0.9;
                let v = 0.03 + h + (f * 9.0 - t).sin() * 0.004;
                for q in 0..2 {
                    let sw = (walk - i as f32 * 0.7 + q as f32 * PI).sin() * 0.015;
                    seg(g, pen, u, v - h * 0.6, u + sw, 0.0, 0.008, dark, 1.0);
                }
                ell(g, pen, u, v, 0.9 / n as f32 * 0.65, h, 0.0, if i % 2 == 0 { c } else { mix(c, c2, 0.4) }, 1.0);
                if k == 1 {
                    seg(g, pen, u - 0.02, v + h * 0.8, u + 0.02, v + h * 0.8, 0.01, c2, 0.8);
                }
            }
            seg(g, pen, 0.48, 0.03 + h, 0.55, 0.03 + h * 1.8, 0.008, dark, 1.0);
        }
        2 => {
            // サソリ：はさみと持ち上げた尾
            for q in 0..4 {
                let u = 0.15 - q as f32 * 0.07;
                let sw = (walk + q as f32).sin() * 0.02;
                seg(g, pen, u, 0.05, u + sw - 0.03, 0.0, 0.012, dark, 1.0);
            }
            ell(g, pen, 0.08, 0.07, 0.22, 0.05, 0.0, c, 1.0);
            for i in 0..5 {
                seg(g, pen, 0.1 - i as f32 * 0.06, 0.03, 0.1 - i as f32 * 0.06, 0.11, 0.006, dark, 0.7);
            }
            let mut tail = Vec::new();
            for i in 0..=6 {
                let f = i as f32 / 6.0;
                let a = f * 2.2;
                tail.push((-0.12 - a.sin() * 0.18 + f * 0.05, 0.08 + (1.0 - a.cos()) * 0.16));
            }
            taper(g, pen, &tail, 0.05, 0.03, c, 1.0);
            let (su, sv) = tail[6];
            poly(g, pen, &[(su, sv), (su + 0.06, sv - 0.02), (su + 0.02, sv - 0.05)], c2, 1.0);
            taper(g, pen, &[(0.28, 0.07), (0.38, 0.1), (0.45, 0.07)], 0.03, 0.03, c, 1.0);
            ell(g, pen, 0.5, 0.07, 0.05, 0.03, 0.0, c, 1.0);
        }
        3 => {
            // ワレイタムシ：丸いお腹と8本の脚
            for q in 0..4 {
                for side in [0.0f32, 1.0] {
                    let u = 0.18 - q as f32 * 0.06;
                    let sw = (walk + q as f32 + side * PI).sin() * 0.03;
                    taper(g, pen, &[(u, 0.1), (u + 0.08 - q as f32 * 0.05, 0.2 - side * 0.03), (u + 0.12 - q as f32 * 0.09 + sw, 0.0)], 0.02, 0.012, if side > 0.5 { c } else { dark }, 1.0);
                }
            }
            ell(g, pen, -0.15, 0.13, 0.25, 0.12, 0.0, c, 1.0);
            for i in 0..4 {
                seg(g, pen, -0.32 + i as f32 * 0.1, 0.06, -0.3 + i as f32 * 0.1, 0.2, 0.008, mix(c, c2, 0.5), 0.7);
            }
            ell(g, pen, 0.17, 0.12, 0.11, 0.07, 0.0, mix(c, c2, 0.3), 1.0);
            circ(g, pen, 0.22, 0.16, 0.012, INK, 1.0);
        }
        _ => {
            // ゴキブリの仲間：平たい体とはね、触角
            for q in 0..3 {
                let u = 0.15 - q as f32 * 0.12;
                let sw = (walk + q as f32 * 2.0).sin() * 0.03;
                taper(g, pen, &[(u, 0.06), (u - 0.05, 0.03), (u - 0.08 + sw, 0.0)], 0.025, 0.015, dark, 1.0);
            }
            ell(g, pen, -0.05, 0.1, 0.42, 0.08, 0.0, c, 1.0);
            ell(g, pen, -0.1, 0.12, 0.36, 0.06, 0.05, mix(c, c2, 0.5), 0.9);
            for i in 0..6 {
                seg(g, pen, 0.2 - i as f32 * 0.08, 0.09, 0.15 - i as f32 * 0.08, 0.15, 0.005, mul(c2, 0.8), 0.6);
            }
            ell(g, pen, 0.38, 0.09, 0.07, 0.05, 0.0, mul(c, 0.85), 1.0);
            let wave = (t * 3.0).sin() * 0.03;
            seg(g, pen, 0.43, 0.11, 0.62, 0.2 + wave, 0.006, dark, 1.0);
            seg(g, pen, 0.43, 0.1, 0.6, 0.06 - wave, 0.006, dark, 1.0);
        }
    }
}

pub(crate) fn griffinfly(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let flap = (t * 22.0).sin();
    // 奥のはね
    for (u, l) in [(0.12, 0.42), (0.02, 0.38)] {
        let a = flap * 0.9;
        let (tu, tv) = (u - 0.1, a.cos().abs() * 0.0 + a.sin() * l);
        poly(g, pen, &[(u + 0.03, 0.02), (tu + 0.05, tv + 0.02), (tu - 0.08, tv), (u - 0.03, 0.0)], c2, 0.35);
    }
    // 長い腹
    taper(g, pen, &[(0.1, 0.0), (-0.2, -0.01), (-0.5, 0.0)], 0.06, 0.03, c, 1.0);
    for i in 0..8 {
        let u = 0.05 - i as f32 * 0.065;
        seg(g, pen, u, -0.025, u, 0.025, 0.006, mix(c, c2, 0.3), 0.8);
    }
    ell(g, pen, 0.17, 0.01, 0.09, 0.05, 0.0, mul(c, 1.1), 1.0);
    ell(g, pen, 0.29, 0.02, 0.05, 0.05, 0.0, c, 1.0);
    circ(g, pen, 0.31, 0.04, 0.035, mix(c2, [0.2, 0.6, 0.5], 0.5), 1.0);
    // 手前のはね
    for (u, l) in [(0.14, 0.45), (0.04, 0.4)] {
        let a = -flap * 0.9 + 0.2;
        let tv = -a.sin() * l;
        poly(g, pen, &[(u + 0.03, 0.02), (u - 0.05, tv + 0.02), (u - 0.18, tv), (u - 0.03, 0.0)], c2, 0.55);
        seg(g, pen, u, 0.01, u - 0.1, tv + 0.01, 0.004, mul(c, 0.6), 0.7);
    }
}

pub(crate) fn tully(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let wv = |f: f32| (f * 5.0 - t * 6.0).sin() * 0.03 * f;
    let prof = |f: f32| {
        let h = if f < 0.3 { 0.02 + f / 0.3 * 0.06 } else { 0.08 * (1.0 - (f - 0.3) / 0.7 * 0.6) };
        (h, h)
    };
    let (top, _, mid) = body_strip(g, pen, 16, &prof, &wv, c);
    for i in 4..14 {
        let (u, v) = mid[i];
        seg(g, pen, u, top[i].1, u, v - (top[i].1 - v), 0.005, mul(c, 0.75), 0.7);
    }
    let (u, v) = mid[15];
    poly(g, pen, &[(u + 0.06, v), (u - 0.05, v + 0.09), (u - 0.02, v), (u - 0.05, v - 0.09)], mix(c, c2, 0.3), 0.9);
    // 長い鼻とはさみの口
    let bite = (t * 1.3).sin() * 0.02 + 0.02;
    taper(g, pen, &[(0.42, mid[1].1), (0.5, mid[0].1 - 0.03), (0.58, mid[0].1 - 0.06)], 0.025, 0.012, c, 1.0);
    seg(g, pen, 0.58, mid[0].1 - 0.06, 0.64, mid[0].1 - 0.06 + bite, 0.01, c2, 1.0);
    seg(g, pen, 0.58, mid[0].1 - 0.06, 0.64, mid[0].1 - 0.06 - bite, 0.01, c2, 1.0);
    // 横棒の先の眼
    let ev = mid[4].1 + 0.11;
    seg(g, pen, 0.25, mid[4].1 + 0.06, 0.25, ev, 0.012, c, 1.0);
    circ(g, pen, 0.25, ev, 0.022, INK, 1.0);
}

pub(crate) fn fusulinid(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], seed: u32) {
    let p0 = Pen { fx: 1.0, ..*pen };
    for i in 0..3 {
        let h = hash2(seed as i32, i, 9);
        let (u, v) = (-0.3 + i as f32 * 0.3 + (h - 0.5) * 0.08, -0.42 + h * 0.06);
        let a = (h - 0.5) * 0.5;
        ell(g, &p0, u, v, 0.16, 0.06, a, c, 1.0);
        for q in 1..4 {
            let f = q as f32 / 4.0;
            ell(g, &p0, u, v, 0.16 * f, 0.06 * f, a, if q % 2 == 0 { c } else { c2 }, 0.6);
        }
    }
}

/// 上半分だけの楕円（地面から盛り上がるドーム）
fn dome(g: &mut Gfx, pen: &Pen, u: f32, v: f32, ru: f32, rv: f32, c: [f32; 3], a: f32) {
    let mut pts = vec![(u, v)];
    for i in 0..=16 {
        let th = i as f32 / 16.0 * PI;
        pts.push((u + th.cos() * ru, v + th.sin() * rv));
    }
    poly(g, pen, &pts, c, a);
}

pub(crate) fn stromatoporoid(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], seed: u32) {
    let p0 = Pen { fx: 1.0, ..*pen };
    for layer in 0..6 {
        let f = 1.0 - layer as f32 * 0.14;
        let col = if layer % 2 == 0 { c } else { mix(c, c2, 0.5) };
        dome(g, &p0, (hash2(seed as i32, layer, 2) - 0.5) * 0.04, -0.5, 0.48 * f, 0.55 * f, col, 1.0);
    }
    for i in 0..8 {
        let u = -0.35 + i as f32 * 0.1;
        circ(g, &p0, u, -0.5 + 0.4 * (1.0 - (u / 0.5).powi(2)).max(0.0).sqrt(), 0.012, mul(c2, 0.7), 0.7);
    }
}

pub(crate) fn mollusc(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    if k == 2 {
        ell(g, pen, 0.0, 0.04, 0.32, 0.07, 0.0, c, 1.0);
        for i in 1..7 { let f = i as f32 / 7.0; ell(g, pen, 0.0, 0.05, 0.31 * f, 0.055 * f, 0.0, if i % 2 == 0 { c } else { c2 }, 0.8); }
        return;
    }
    if k == 1 {
        // 二枚貝：海底に立てた殻（少し開く）
        let p0 = Pen { fx: 1.0, ..*pen };
        let open = ((t * 0.6).sin() * 0.5 + 0.5) * 0.04;
        ell(g, &p0, -open, -0.18, 0.3, 0.26, 0.15, mul(c, 0.8), 1.0);
        ell(g, &p0, open, -0.18, 0.3, 0.26, -0.15, c, 1.0);
        for i in 1..5 {
            let f = 1.0 - i as f32 * 0.18;
            ell(g, &p0, open, -0.18 - (1.0 - f) * 0.2, 0.3 * f, 0.26 * f, -0.15, if i % 2 == 0 { c } else { mix(c, c2, 0.4) }, 0.9);
        }
        return;
    }
    // 巻貝：這う足の上に巻いた殻
    let crawl = (t * 1.5).sin() * 0.01;
    ell(g, pen, 0.05 + crawl, 0.04, 0.42, 0.06, 0.0, mix(c2, WHITE, 0.2), 1.0);
    seg(g, pen, 0.38, 0.07, 0.48, 0.16, 0.012, c2, 1.0);
    seg(g, pen, 0.34, 0.07, 0.42, 0.18, 0.012, c2, 1.0);
    for r in 0..4 {
        let rr = 0.3 - r as f32 * 0.07;
        ell(g, pen, -0.05 + r as f32 * 0.03, 0.08 + rr, rr, rr * 0.95, 0.0, if r % 2 == 0 { c } else { mix(c, c2, 0.35) }, 1.0);
    }
}

pub(crate) fn starfish(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let wob = (t * 0.8).sin() * 0.04;
    for k in 0..5 {
        let a = k as f32 / 5.0 * TAU + wob * if k % 2 == 0 { 1.0 } else { -1.0 };
        let (s, co) = a.sin_cos();
        taper(g, pen, &[(0.0, 0.0), (co * 0.25, s * 0.25), (co * 0.48, s * 0.48)], 0.2, 0.04, c, 1.0);
        for q in 1..5 {
            let f = q as f32 / 5.0 * 0.45;
            circ(g, pen, co * f, s * f, 0.018, c2, 0.8);
        }
    }
    circ(g, pen, 0.0, 0.0, 0.09, mix(c, c2, 0.3), 1.0);
}

pub(crate) fn rayfin(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let wv = |f: f32| (f * 6.0 - t * 9.0).sin() * 0.035 * f * f;
    let prof = |f: f32| {
        let h = if f < 0.18 { 0.09 * (f / 0.18).sqrt() } else { 0.09 * (1.0 - (f - 0.18) / 0.82 * 0.8) };
        (h, h * 0.85)
    };
    let (top, bot, mid) = body_strip(g, pen, 18, &prof, &wv, c);
    // うろこの列と、お腹の明るい色
    for i in 2..16 {
        let (u, v) = mid[i];
        seg(g, pen, u, v - 0.01, u - 0.03, (v + bot[i].1) * 0.5, 0.025, c2, 0.45);
        circ(g, pen, u, v + 0.025, 0.01, mix(c, WHITE, 0.25), 0.5);
    }
    // 背びれ・しりびれ・二股の尾びれ
    poly(g, pen, &[(-0.02, top[9].1), (-0.12, top[11].1 + 0.07), (-0.16, top[11].1)], mul(c, 0.8), 1.0);
    poly(g, pen, &[(-0.1, bot[11].1), (-0.2, bot[12].1 - 0.06), (-0.24, bot[13].1)], mul(c, 0.8), 1.0);
    let (u, v) = mid[17];
    poly(g, pen, &[(u + 0.04, v), (u - 0.1, v + 0.12), (u - 0.04, v), (u - 0.09, v - 0.08)], mul(c, 0.85), 1.0);
    eye(g, pen, 0.43, mid[1].1 + 0.02, 0.018);
    seg(g, pen, 0.5, mid[0].1 - 0.01, 0.4, mid[1].1 - 0.035, 0.008, INK, 0.7);
}

pub(crate) fn limulid(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let walk = (t * 6.0).sin() * 0.01;
    // 尾のとげ
    taper(g, pen, &[(-0.2, 0.0), (-0.5, walk)], 0.04, 0.01, mul(c, 0.8), 1.0);
    // 腹の甲羅（トゲつき）
    poly(g, pen, &[(0.05, 0.22), (-0.22, 0.14), (-0.24, -0.14), (0.05, -0.22)], mul(c, 0.9), 1.0);
    for sgn in [-1.0f32, 1.0] {
        for q in 0..3 {
            let u = -0.02 - q as f32 * 0.08;
            seg(g, pen, u, sgn * 0.18, u - 0.03, sgn * 0.26, 0.012, c2, 0.9);
        }
    }
    // 頭の半月形の甲羅
    let mut pts = vec![(0.05, 0.0)];
    for i in 0..=14 {
        let a = -PI * 0.5 + i as f32 / 14.0 * PI;
        pts.push((0.05 + a.cos() * 0.42, a.sin() * 0.34));
    }
    poly(g, pen, &pts, c, 1.0);
    seg(g, pen, 0.05, 0.0, 0.42, 0.0, 0.01, mul(c, 0.7), 0.7);
    for sgn in [-1.0f32, 1.0] {
        circ(g, pen, 0.26, sgn * 0.14, 0.018, INK, 1.0);
    }
}

pub(crate) fn urchin(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    let p0 = Pen { fx: 1.0, ..*pen };
    let wob = (t * 0.7).sin() * 0.05;
    // トゲ（まわりに放射状に）
    for k in 0..22 {
        let a = k as f32 / 22.0 * PI + wob * if k % 2 == 0 { 1.0 } else { -1.0 };
        let (s, co) = a.sin_cos();
        seg(g, &p0, co * 0.2, 0.22 + s * 0.18, co * 0.46, 0.22 + s * 0.42, 0.012, mul(c, 0.8), 1.0);
    }
    // 丸い殻と、つぶつぶの列
    ell(g, &p0, 0.0, 0.22, 0.24, 0.2, 0.0, c, 1.0);
    for r in 0..5 {
        let a = -0.8 + r as f32 * 0.4;
        for q in 1..4 {
            let f = q as f32 / 4.0;
            circ(g, &p0, a.sin() * 0.22 * f, 0.22 + a.cos() * 0.18 * f, 0.016, c2, 0.9);
        }
    }
}

// ============================================================ 中生代
// 歩く動物の共通の形：胴・首と頭・尾・脚（足の裏が v=0）

struct Walker {
    /// 胴の中心と半径（u, v, ru, rv）
    body: (f32, f32, f32, f32),
    /// 首から頭へ（胴から順に）
    neck: Vec<(f32, f32)>,
    neck_w: f32,
    head: (f32, f32, f32, f32),
    tail: Vec<(f32, f32)>,
    tail_w: f32,
    /// 脚（付け根の u, 付け根の v, 太さ）
    legs: Vec<(f32, f32, f32)>,
}

fn walker(g: &mut Gfx, pen: &Pen, w: &Walker, c: [f32; 3], c2: [f32; 3], t: f32, gait: f32) {
    let dark = mul(c, 0.72);
    let step = t * gait;
    // 奥の脚
    for (k, &(lu, lv, lw)) in w.legs.iter().enumerate() {
        let sw = (step + k as f32 * PI).sin() * 0.04 * lv.min(0.4) / 0.25;
        taper(g, pen, &[(lu, lv), (lu + sw * 0.4 + 0.01, lv * 0.5), (lu + sw, 0.0)], lw, lw * 0.7, dark, 1.0);
    }
    taper(g, pen, &w.tail, w.tail_w, 0.01, c, 1.0);
    taper(g, pen, &w.neck, w.neck_w, w.neck_w * 0.6, c, 1.0);
    let (bu, bv, bru, brv) = w.body;
    ell(g, pen, bu, bv, bru, brv, 0.0, c, 1.0);
    ell(g, pen, bu, bv - brv * 0.3, bru * 0.9, brv * 0.6, 0.0, mix(c, c2, 0.25), 1.0);
    let (hu, hv, hru, hrv) = w.head;
    ell(g, pen, hu, hv, hru, hrv, -0.05, c, 1.0);
    seg(g, pen, hu, hv - hrv * 0.35, hu + hru, hv - hrv * 0.2, 0.008, INK, 0.7);
    eye(g, pen, hu + hru * 0.25, hv + hrv * 0.3, 0.012);
    // 手前の脚
    for (k, &(lu, lv, lw)) in w.legs.iter().enumerate() {
        let sw = (step + k as f32 * PI + PI * 0.5).sin() * 0.05 * lv.min(0.4) / 0.25;
        taper(g, pen, &[(lu + 0.02, lv), (lu + sw * 0.4 + 0.03, lv * 0.5), (lu + sw + 0.02, 0.0)], lw * 1.1, lw * 0.8, mul(c, 0.9), 1.0);
        ell(g, pen, lu + sw + 0.04, 0.0, lw * 0.6, 0.012, 0.0, mul(c, 0.8), 1.0);
    }
}

fn curve(a: (f32, f32), b: (f32, f32), bend: f32, n: usize) -> Vec<(f32, f32)> {
    (0..=n)
        .map(|i| {
            let f = i as f32 / n as f32;
            (a.0 + (b.0 - a.0) * f, a.1 + (b.1 - a.1) * f + (f * PI).sin() * bend)
        })
        .collect()
}

pub(crate) fn dino(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    let sway = (t * 1.6).sin() * 0.015;
    match k {
        // 二本足の肉食恐竜：0=小型 1=大型 2=帆のある大型 11=羽毛の小型
        0 | 1 | 2 | 11 => {
            let big = k == 1 || k == 2;
            let hip = if big { 0.26 } else { 0.22 };
            let body = (0.02, hip + 0.06, if big { 0.17 } else { 0.13 }, if big { 0.09 } else { 0.065 });
            let head = if big { (0.4, hip + 0.16, 0.1, 0.06) } else { (0.42, hip + 0.18, 0.06, 0.035) };
            let w = Walker {
                body,
                neck: curve((0.14, hip + 0.09), (head.0 - 0.05, head.1), 0.03, 4),
                neck_w: if big { 0.09 } else { 0.05 },
                head,
                tail: curve((-0.12, hip + 0.07), (-0.5, hip + 0.1 + sway), 0.02, 6),
                tail_w: if big { 0.11 } else { 0.07 },
                legs: vec![(-0.02, hip, if big { 0.07 } else { 0.045 })],
            };
            if k == 2 {
                // 背中の帆
                let mut sail = vec![(-0.14, hip + 0.13)];
                for i in 0..=8 {
                    let f = i as f32 / 8.0;
                    sail.push((-0.14 + f * 0.3, hip + 0.13 + (f * PI).sin() * 0.16));
                }
                sail.push((0.16, hip + 0.13));
                poly(g, pen, &sail, c2, 0.95);
            }
            walker(g, pen, &w, c, c2, t, 3.0);
            // 小さな腕
            seg(g, pen, 0.12, hip + 0.06, 0.18, hip + 0.01, if big { 0.025 } else { 0.018 }, mul(c, 0.85), 1.0);
            if k == 11 {
                // 羽毛（腕と尾のふさ）
                for i in 0..6 {
                    let f = i as f32 / 6.0;
                    seg(g, pen, 0.13 + f * 0.04, hip + 0.06, 0.08 + f * 0.05, hip - 0.04, 0.012, c2, 0.8);
                    let (tu, tv) = (-0.3 - f * 0.18, hip + 0.1);
                    seg(g, pen, tu, tv, tu - 0.02, tv + 0.05, 0.012, c2, 0.8);
                    seg(g, pen, tu, tv, tu - 0.02, tv - 0.04, 0.012, c2, 0.8);
                }
            }
            if big {
                for i in 0..4 {
                    let u = head.0 + 0.02 + i as f32 * 0.018;
                    seg(g, pen, u, head.1 - 0.03, u, head.1 - 0.05, 0.006, WHITE, 0.9);
                }
            }
        }
        // 首の長い竜脚類：3=横にのばす 13=首を高く上げる
        3 | 13 => {
            let hip = 0.2;
            let up = k == 13;
            let head = if up { (0.4, 0.62, 0.035, 0.022) } else { (0.5, 0.32, 0.03, 0.02) };
            let w = Walker {
                body: (-0.02, hip + 0.08, 0.17, 0.1),
                neck: if up { curve((0.12, hip + 0.12), (head.0 - 0.02, head.1), -0.03, 6) } else { curve((0.12, hip + 0.1), (head.0 - 0.02, head.1), 0.05, 6) },
                neck_w: 0.07,
                head,
                tail: curve((-0.17, hip + 0.08), (-0.55, hip - 0.05 + sway), 0.04, 7),
                tail_w: 0.08,
                legs: vec![(-0.1, hip, 0.06), (0.09, hip, 0.06)],
            };
            walker(g, pen, &w, c, c2, t, 1.2);
        }
        // 古い竜脚形類（プラテオサウルス）
        4 => {
            let hip = 0.24;
            let w = Walker {
                body: (0.0, hip + 0.06, 0.15, 0.08),
                neck: curve((0.12, hip + 0.09), (0.36, hip + 0.18), 0.03, 5),
                neck_w: 0.05,
                head: (0.4, hip + 0.19, 0.045, 0.03),
                tail: curve((-0.14, hip + 0.06), (-0.5, hip + 0.04 + sway), 0.03, 6),
                tail_w: 0.07,
                legs: vec![(-0.03, hip, 0.055)],
            };
            walker(g, pen, &w, c, c2, t, 2.0);
            seg(g, pen, 0.1, hip + 0.04, 0.16, hip - 0.1, 0.025, mul(c, 0.85), 1.0);
        }
        // 四足の植物食恐竜：5=剣竜 6=鎧竜 7=大型角竜 8=小型角竜 9=鳥脚類 10=とさかのある鳥脚類
        _ => {
            let (hip, bru, brv, head, neck_u) = match k {
                5 => (0.18, 0.18, 0.1, (0.36, 0.14, 0.05, 0.03), 0.16),
                6 => (0.12, 0.2, 0.08, (0.32, 0.12, 0.06, 0.04), 0.18),
                7 => (0.16, 0.18, 0.1, (0.33, 0.18, 0.1, 0.07), 0.16),
                8 => (0.12, 0.14, 0.07, (0.3, 0.16, 0.07, 0.05), 0.12),
                _ => (0.2, 0.16, 0.08, (0.38, 0.26, 0.06, 0.035), 0.14),
            };
            let w = Walker {
                body: (0.0, hip + brv * 0.8, bru, brv),
                neck: curve((neck_u, hip + brv), (head.0 - 0.03, head.1), 0.02, 4),
                neck_w: brv * 0.9,
                head,
                tail: curve((-bru * 0.9, hip + brv * 0.8), (-0.5, hip + 0.02 + sway), 0.02, 6),
                tail_w: brv * 1.1,
                legs: vec![(-bru * 0.55, hip, 0.05), (bru * 0.55, hip * if k >= 9 { 0.7 } else { 1.0 }, 0.045)],
            };
            if k == 5 {
                // 背中の板
                for i in 0..9 {
                    let f = i as f32 / 8.0;
                    let u = -0.24 + f * 0.42;
                    let v = hip + brv * 1.6 - (f - 0.45).abs() * 0.1;
                    let hgt = 0.06 + (1.0 - (f - 0.45).abs() * 2.0).max(0.0) * 0.07;
                    poly(g, pen, &[(u - 0.025, v - 0.02), (u, v + hgt), (u + 0.025, v - 0.02)], c2, 1.0);
                }
            }
            walker(g, pen, &w, c, c2, t, 1.6);
            match k {
                5 => {
                    for s2 in [0.0f32, 0.05] {
                        seg(g, pen, -0.45 + s2, hip + 0.06, -0.5 + s2, hip + 0.14, 0.012, c2, 1.0);
                    }
                }
                6 => {
                    for i in 0..8 {
                        let u = -0.16 + i as f32 * 0.045;
                        circ(g, pen, u, hip + brv * 1.6, 0.02, c2, 1.0);
                    }
                    ell(g, pen, -0.5, hip + 0.03, 0.04, 0.035, 0.0, c2, 1.0);
                }
                7 | 8 => {
                    // えりかざりと角
                    ell(g, pen, head.0 - head.2 * 0.8, head.1 + head.3 * 0.9, head.2 * if k == 7 { 0.9 } else { 0.6 }, head.3 * 1.6, 0.4, c2, 1.0);
                    let (hu, hv, hr, hh) = head;
                    ell(g, pen, hu, hv, hr, hh, -0.05, c, 1.0);
                    eye(g, pen, hu + hr * 0.2, hv + hh * 0.3, 0.012);
                    poly(g, pen, &[(hu + hr * 0.8, hv - hh * 0.2), (hu + hr * 1.25, hv - hh * 0.5), (hu + hr * 0.9, hv - hh * 0.7)], INK, 0.8);
                    if k == 7 {
                        for du in [0.0f32, 0.04] {
                            poly(g, pen, &[(hu + du - 0.01, hv + hh * 0.6), (hu + du + 0.06, hv + hh * 1.8), (hu + du + 0.02, hv + hh * 0.6)], WHITE, 0.95);
                        }
                        poly(g, pen, &[(hu + hr * 0.7, hv + hh * 0.4), (hu + hr * 0.9, hv + hh * 1.2), (hu + hr * 0.95, hv + hh * 0.4)], WHITE, 0.95);
                    }
                }
                10 => {
                    let (hu, hv, _, hh) = head;
                    taper(g, pen, &[(hu, hv + hh * 0.6), (hu - 0.1, hv + hh * 2.2), (hu - 0.18, hv + hh * 2.6)], 0.03, 0.02, c2, 1.0);
                }
                _ => {}
            }
        }
    }
}

pub(crate) fn marine(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    if k == 8 {
        ell(g, pen, -0.05, 0.02, 0.28, 0.12, 0.0, c, 1.0);
        for u in [-0.20, 0.12] { for side in [-1.0f32, 1.0] {
            taper(g, pen, &[(u, side * 0.05), (u - 0.06, side * 0.15)], 0.045, 0.025, c2, 1.0);
        } }
        taper(g, pen, &[(0.17, 0.03), (0.35, 0.05), (0.43, 0.04)], 0.04, 0.025, c2, 1.0);
        eye(g, pen, 0.39, 0.065, 0.011);
        return;
    }
    if k == 7 {
        dome(g, pen, -0.04, 0.10, 0.30, 0.30, c, 1.0);
        for u in [-0.23, 0.15] { for side in [-1.0f32, 1.0] {
            taper(g, pen, &[(u, 0.12), (u + side * 0.04, 0.025)], 0.06, 0.04, c2, 1.0);
        } }
        ell(g, pen, 0.34, 0.16, 0.10, 0.055, 0.0, c2, 1.0);
        eye(g, pen, 0.38, 0.18, 0.012);
        for u in [-0.19, -0.06, 0.07] { seg(g, pen, u, 0.13, u + 0.02, 0.32, 0.012, c2, 0.8); }
        return;
    }
    let stroke = (t * 2.5).sin();
    let wv = |f: f32| (f * 5.0 - t * 5.0).sin() * 0.03 * f * f;
    match k {
        // 魚竜：イルカのような体
        0 => {
            let prof = |f: f32| {
                let h = if f < 0.12 { 0.03 + f / 0.12 * 0.05 } else if f < 0.45 { 0.08 + (f - 0.12) * 0.1 } else { 0.11 * (1.0 - (f - 0.45) / 0.55 * 0.85) };
                (h, h * 0.85)
            };
            let (top, bot, mid) = body_strip(g, pen, 18, &prof, &wv, c);
            poly(g, pen, &[(0.02, top[9].1), (-0.08, top[9].1 + 0.1), (-0.1, top[10].1)], mul(c, 0.85), 1.0);
            let (u, v) = mid[17];
            poly(g, pen, &[(u + 0.03, v), (u - 0.06, v + 0.14), (u - 0.02, v), (u - 0.06, v - 0.12)], mul(c, 0.85), 1.0);
            taper(g, pen, &[(0.22, bot[5].1 + 0.02), (0.14 + stroke * 0.02, bot[5].1 - 0.08)], 0.05, 0.02, mul(c, 0.8), 1.0);
            taper(g, pen, &[(0.48, mid[0].1), (0.6, mid[0].1 - 0.01)], 0.03, 0.01, c, 1.0);
            eye(g, pen, 0.42, mid[1].1 + 0.02, 0.025);
        }
        // 首長竜（1）・偽竜はひれで泳ぐ。3=首の短い大型
        1 | 3 => {
            let long = k == 1;
            let (bu, bru, brv) = if long { (-0.12, 0.17, 0.08) } else { (-0.05, 0.22, 0.1) };
            for (u, s) in [(bu + bru * 0.5, 1.0f32), (bu - bru * 0.55, -1.0)] {
                let a = stroke * s * 0.06;
                taper(g, pen, &[(u, -brv * 0.3), (u - 0.08, -brv * 0.9 + a), (u - 0.16, -brv * 1.2 + a)], 0.06, 0.02, mul(c, 0.75), 1.0);
            }
            taper(g, pen, &[(bu - bru, 0.0), (-0.5, 0.01 + stroke * 0.01)], brv * 1.2, 0.01, c, 1.0);
            ell(g, pen, bu, 0.0, bru, brv, 0.0, c, 1.0);
            ell(g, pen, bu, -brv * 0.3, bru * 0.9, brv * 0.6, 0.0, mix(c, c2, 0.3), 1.0);
            let neck = if long { curve((bu + bru * 0.8, 0.02), (0.44, 0.08), 0.06 + stroke * 0.01, 8) } else { curve((bu + bru * 0.8, 0.02), (0.28, 0.03), 0.01, 3) };
            taper(g, pen, &neck, brv * 0.8, brv * 0.45, c, 1.0);
            let (hu, hv) = *neck.last().unwrap();
            let hr = if long { 0.045 } else { 0.14 };
            ell(g, pen, hu + hr * 0.6, hv, hr, hr * 0.45, 0.0, c, 1.0);
            seg(g, pen, hu + hr * 0.2, hv - hr * 0.1, hu + hr * 1.5, hv - hr * 0.05, 0.007, INK, 0.7);
            eye(g, pen, hu + hr * 0.4, hv + hr * 0.15, 0.014);
            for i in 0..5 {
                let u = for_teeth(hu, hr, i);
                seg(g, pen, u, hv - hr * 0.08, u, hv - hr * 0.22, 0.005, WHITE, 0.9);
            }
            for (u, s) in [(bu + bru * 0.5, 1.0f32), (bu - bru * 0.55, -1.0)] {
                let a = -stroke * s * 0.06;
                taper(g, pen, &[(u + 0.02, -brv * 0.4), (u - 0.06, -brv * 1.0 + a), (u - 0.14, -brv * 1.3 + a)], 0.07, 0.025, mul(c, 0.9), 1.0);
            }
        }
        // 板歯類：ずんぐりした体と、短い脚・ひれ
        2 => {
            taper(g, pen, &[(-0.2, 0.0), (-0.5, 0.0)], 0.12, 0.01, c, 1.0);
            ell(g, pen, -0.02, 0.0, 0.26, 0.12, 0.0, c, 1.0);
            for i in 0..6 {
                circ(g, pen, -0.18 + i as f32 * 0.07, 0.1, 0.02, c2, 0.9);
            }
            ell(g, pen, 0.3, 0.02, 0.1, 0.07, 0.0, c, 1.0);
            eye(g, pen, 0.32, 0.05, 0.014);
            for u in [0.12f32, -0.16] {
                taper(g, pen, &[(u, -0.08), (u - 0.06 + stroke * 0.02, -0.18)], 0.05, 0.03, mul(c, 0.8), 1.0);
            }
        }
        // モササウルス：トカゲの頭と、ひれ足と尾びれ
        4 => {
            let prof = |f: f32| {
                let h = if f < 0.2 { 0.035 + f / 0.2 * 0.045 } else { 0.08 * (1.0 - (f - 0.2) / 0.8 * 0.75) };
                (h, h * 0.9)
            };
            let (_, bot, mid) = body_strip(g, pen, 20, &prof, &wv, c);
            let (u, v) = mid[19];
            poly(g, pen, &[(u + 0.06, v), (u - 0.05, v - 0.1), (u - 0.02, v), (u - 0.04, v + 0.05)], mul(c, 0.85), 1.0);
            for (i, s) in [(4usize, 1.0f32), (10, -1.0)] {
                let (fu, fv) = (mid[i].0, bot[i].1);
                taper(g, pen, &[(fu, fv), (fu - 0.06, fv - 0.06 + stroke * s * 0.02)], 0.04, 0.02, mul(c, 0.8), 1.0);
            }
            seg(g, pen, 0.5, mid[0].1 - 0.01, 0.36, mid[2].1 - 0.03, 0.01, INK, 0.7);
            for i in 0..5 {
                let u = 0.48 - i as f32 * 0.025;
                seg(g, pen, u, mid[0].1 - 0.012, u, mid[0].1 - 0.03, 0.005, WHITE, 0.9);
            }
            eye(g, pen, 0.4, mid[1].1 + 0.03, 0.015);
        }
        // ウミガメ
        _ => {
            for (u, s) in [(0.14f32, 1.0f32), (-0.18, -1.0)] {
                let a = stroke * s * 0.08;
                taper(g, pen, &[(u, -0.02), (u - 0.1, -0.12 + a), (u - 0.2, -0.16 + a)], 0.07, 0.02, mul(c2, 0.9), 1.0);
            }
            dome(g, pen, -0.02, -0.02, 0.32, 0.2, c, 1.0);
            for i in 0..5 {
                let u = -0.24 + i as f32 * 0.11;
                seg(g, pen, u, 0.02, u + 0.02, 0.14, 0.008, mul(c, 0.7), 0.8);
            }
            ell(g, pen, 0.36, 0.0, 0.08, 0.055, 0.0, c2, 1.0);
            eye(g, pen, 0.39, 0.02, 0.012);
        }
    }
}

fn for_teeth(hu: f32, hr: f32, i: i32) -> f32 {
    hu + hr * 0.5 + i as f32 * hr * 0.2
}

/// 翼竜・鳥：空を飛ぶ（2=地上を歩く大型の翼竜）
pub(crate) fn flyer(g: &mut Gfx, pen: &Pen, bird: bool, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    if !bird && k == 2 {
        // ケツァルコアトルス：たたんだ翼で四つ足で歩く
        let w = Walker {
            body: (-0.05, 0.42, 0.1, 0.08),
            neck: curve((0.03, 0.47), (0.24, 0.78), -0.02, 6),
            neck_w: 0.04,
            head: (0.32, 0.8, 0.12, 0.03),
            tail: vec![(-0.14, 0.42), (-0.2, 0.42)],
            tail_w: 0.03,
            legs: vec![(-0.08, 0.36, 0.03), (0.04, 0.4, 0.035)],
        };
        walker(g, pen, &w, c, c2, t, 1.4);
        poly(g, pen, &[(-0.02, 0.45), (0.04, 0.22), (-0.1, 0.36)], c2, 0.9);
        poly(g, pen, &[(0.28, 0.82), (0.3, 0.92), (0.36, 0.83)], mul(c2, 1.1), 1.0);
        return;
    }
    let flap = (t * if bird { 9.0 } else { 5.0 }).sin();
    let wing = |g: &mut Gfx, s: f32, a: f32| {
        let tip = (0.0 - 0.06 * s, a * 0.48);
        if bird {
            for i in 0..6 {
                let f = i as f32 / 6.0;
                let (fu, fv) = (0.08 - f * 0.2, a * (0.12 + f * 0.36));
                ell(g, pen, fu, fv, 0.09, 0.03, 1.2 * a.signum(), if i % 2 == 0 { c2 } else { mix(c2, c, 0.3) }, 0.95);
            }
        } else {
            poly(g, pen, &[(0.1, 0.0), tip, (-0.18, 0.0)], c2, 0.85);
            seg(g, pen, 0.1, 0.0, tip.0, tip.1, 0.015, mul(c, 0.8), 1.0);
        }
    };
    wing(g, 1.0, 0.55 + flap * 0.45);
    // 胴と尾
    if bird || k == 0 {
        taper(g, pen, &[(-0.08, 0.0), (-0.42, -0.02)], 0.04, 0.01, c, 1.0);
        if bird {
            for i in 0..5 {
                let u = -0.14 - i as f32 * 0.06;
                if k == 2 {
                    continue;
                }
                seg(g, pen, u, 0.0, u - 0.03, 0.05, 0.012, c2, 0.9);
                seg(g, pen, u, 0.0, u - 0.03, -0.05, 0.012, c2, 0.9);
            }
            if k == 2 {
                taper(g, pen, &[(-0.1, 0.0), (-0.45, 0.06), (-0.55, 0.02)], 0.012, 0.03, c2, 1.0);
                taper(g, pen, &[(-0.1, 0.0), (-0.45, -0.04), (-0.55, 0.0)], 0.012, 0.03, c2, 1.0);
            }
        } else {
            poly(g, pen, &[(-0.42, -0.02), (-0.48, 0.03), (-0.5, -0.04)], c2, 1.0);
        }
    }
    ell(g, pen, 0.0, 0.0, 0.12, 0.05, 0.0, c, 1.0);
    // 首と頭
    let big = !bird && k == 1;
    taper(g, pen, &[(0.1, 0.02), (0.2, 0.06)], 0.04, 0.03, c, 1.0);
    ell(g, pen, 0.25, 0.06, if big { 0.14 } else { 0.07 }, 0.03, -0.1, c, 1.0);
    if big {
        poly(g, pen, &[(0.18, 0.08), (0.06, 0.16), (0.2, 0.1)], c2, 1.0);
    }
    if bird {
        poly(g, pen, &[(0.31, 0.07), (0.38, 0.05), (0.31, 0.04)], [0.8, 0.7, 0.4], 1.0);
    }
    eye(g, pen, 0.25, 0.08, 0.012);
    wing(g, -1.0, -(0.35 + flap * 0.45) * 0.6 + 0.2);
    // 足
    seg(g, pen, -0.04, -0.03, -0.1, -0.1, 0.01, mul(c, 0.7), 1.0);
}

/// 小さな哺乳類・キノドン類 0=トガリネズミ形 1=平たい尾で泳ぐ 2=アナグマ形 3=キノドン類
pub(crate) fn mammal(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    if k >= 4 {
        cenozoic_mammal(g, pen, k, c, c2, t);
        return;
    }
    let hip = if k == 2 { 0.12 } else { 0.1 };
    let w = Walker {
        body: (0.0, hip + 0.1, 0.2, 0.11),
        neck: vec![(0.15, hip + 0.13), (0.24, hip + 0.13)],
        neck_w: 0.12,
        head: (0.32, hip + 0.12, 0.12, 0.07),
        tail: if k == 1 { vec![(-0.18, hip + 0.08), (-0.42, hip + 0.04)] } else { curve((-0.18, hip + 0.09), (-0.5, hip + 0.04), 0.05, 5) },
        tail_w: if k == 1 { 0.1 } else { 0.035 },
        legs: vec![(-0.12, hip, 0.05), (0.12, hip, 0.045)],
    };
    walker(g, pen, &w, c, c2, t, 4.0);
    // 毛なみと鼻・ひげ
    for i in 0..8 {
        let u = -0.16 + i as f32 * 0.045;
        seg(g, pen, u, hip + 0.19, u - 0.02, hip + 0.22, 0.008, mix(c, c2, 0.5), 0.8);
    }
    circ(g, pen, 0.45, hip + 0.11, 0.015, INK, 0.9);
    for s in [-1.0f32, 1.0] {
        seg(g, pen, 0.43, hip + 0.1, 0.52, hip + 0.1 + s * 0.03, 0.004, WHITE, 0.6);
    }
    circ(g, pen, 0.27, hip + 0.19, 0.03, mul(c, 0.85), 1.0);
}

/// Cenozoic mammals use distinct silhouettes, also in the guide and photo cards.
fn cenozoic_mammal(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    if k == 15 {
        let flap = (t * 10.0).sin() * 0.13;
        for side in [-1.0f32, 1.0] {
            poly(g, pen, &[(0.03, 0.04), (-0.08, side * (0.50 + flap * 0.3)), (-0.25, side * 0.34), (-0.20, side * 0.13), (-0.35, side * 0.05), (-0.04, -0.04)], c2, 0.9);
            seg(g, pen, 0.03, 0.04, -0.08, side * (0.50 + flap * 0.3), 0.015, c, 1.0);
        }
        ell(g, pen, 0.0, 0.0, 0.14, 0.06, 0.0, c, 1.0);
        circ(g, pen, 0.17, 0.01, 0.055, c, 1.0);
        eye(g, pen, 0.19, 0.03, 0.012);
        return;
    }
    if k == 8 || k == 38 || k == 39 {
        ell(g, pen, 0.0, 0.0, 0.39, 0.10, 0.0, c, 1.0);
        ell(g, pen, 0.34, 0.015, 0.16, 0.075, 0.0, c, 1.0);
        poly(g, pen, &[(-0.35, 0.0), (-0.52, 0.12), (-0.46, 0.0), (-0.52, -0.12)], c2, 1.0);
        poly(g, pen, &[(0.12, -0.05), (0.03, -0.21), (0.24, -0.07)], c2, 1.0);
        seg(g, pen, 0.33, -0.015, 0.48, -0.015, 0.006, INK, 0.8);
        circ(g, pen, 0.39, 0.04, 0.009, INK, 1.0);
        if k == 38 { taper(g, pen, &[(0.38, -0.015), (0.35, -0.20), (0.28, -0.31)], 0.025, 0.003, WHITE, 1.0); }
        return;
    }
    if k == 9 {
        ell(g, pen, -0.02, 0.08, 0.10, 0.19, 0.0, c, 1.0);
        circ(g, pen, 0.01, 0.34, 0.075, c2, 1.0);
        for side in [-1.0f32, 1.0] {
            let stride = (t * 4.0).sin() * 0.07 * side;
            taper(g, pen, &[(side * 0.04, -0.06), (side * 0.05 + stride, -0.24), (side * 0.08 + stride, -0.42)], 0.045, 0.025, c, 1.0);
            taper(g, pen, &[(side * 0.08, 0.22), (side * 0.15 - stride, 0.02), (side * 0.15 - stride, -0.10)], 0.03, 0.018, c, 1.0);
        }
        circ(g, pen, 0.05, 0.35, 0.009, INK, 1.0);
        return;
    }
    if k == 28 {
        ell(g, pen, -0.10, 0.12, 0.14, 0.24, -0.2, c, 1.0);
        taper(g, pen, &[(-0.16, 0.02), (-0.39, -0.20), (-0.54, -0.30)], 0.10, 0.015, c, 1.0);
        taper(g, pen, &[(0.0, 0.23), (0.14, 0.36)], 0.08, 0.05, c, 1.0);
        ell(g, pen, 0.22, 0.38, 0.10, 0.06, 0.0, c, 1.0);
        for side in [-1.0f32, 1.0] {
            let stride = (t * 3.0).sin() * side * 0.03;
            taper(g, pen, &[(-0.04, -0.02), (0.10, -0.16), (stride, -0.32), (0.22 + stride, -0.34)], 0.06, 0.02, c2, 1.0);
            seg(g, pen, 0.03, 0.22, 0.17, 0.08, 0.025, c2, 1.0);
        }
        eye(g, pen, 0.25, 0.40, 0.012);
        return;
    }
    let tall = k == 5 || k == 25;
    let bulky = matches!(k, 6 | 7 | 11 | 13 | 18 | 20 | 26 | 27 | 30 | 31 | 33 | 34 | 40);
    let hip = if tall { 0.26 } else { 0.12 };
    let head = if tall { (0.32, 0.52, 0.11, 0.065) } else { (0.32, 0.22, 0.12, 0.08) };
    let w = Walker {
        body: (-0.06, hip + 0.11, 0.27, if bulky { 0.18 } else { 0.12 }),
        neck: vec![(0.12, hip + 0.16), (head.0 - 0.05, head.1)],
        neck_w: if bulky { 0.15 } else { 0.08 },
        head,
        tail: if k == 36 { vec![(-0.30, hip + 0.1), (-0.32, hip + 0.08)] } else { vec![(-0.30, hip + 0.1), (-0.44, hip + 0.03)] }, tail_w: 0.018,
        legs: vec![(-0.23, hip, 0.05), (0.13, hip, 0.05)],
    };
    walker(g, pen, &w, c, c2, t, 2.0);
    if matches!(k, 7 | 13 | 20 | 33 | 34) {
        // Trunk and curved ivory tusks.
        taper(g, pen, &[(0.42, 0.23), (0.46, 0.10), (0.41, 0.04), (0.48, 0.03)], 0.06, 0.022, c, 1.0);
        let tusk = if k == 13 { [(0.35, 0.18), (0.39, 0.12), (0.38, 0.05), (0.32, 0.04)] } else { [(0.37, 0.17), (0.48, 0.09), (0.55, 0.15), (0.54, 0.24)] };
        taper(g, pen, &tusk, 0.025, 0.002, WHITE, 1.0);
        if k == 7 {
            for i in 0..15 { let u = -0.29 + i as f32 * 0.035; seg(g, pen, u, 0.22, u - 0.025, 0.12, 0.009, c2, 0.65); }
        }
        ell(g, pen, 0.25, 0.24, 0.055, 0.08, 0.0, c2, 1.0);
        if k == 33 { taper(g, pen, &[(0.35, 0.15), (0.48, 0.12), (0.55, 0.13)], 0.022, 0.002, WHITE, 1.0); }
        if k == 34 { poly(g, pen, &[(0.31, 0.15), (0.54, 0.07), (0.57, 0.10), (0.39, 0.19)], c2, 1.0); }
    } else if k == 6 {
        poly(g, pen, &[(0.34, 0.28), (0.42, 0.47), (0.42, 0.27)], c2, 1.0);
        poly(g, pen, &[(0.27, 0.29), (0.30, 0.39), (0.33, 0.28)], c2, 1.0);
    } else if k == 10 {
        for u in [0.34, 0.40] { taper(g, pen, &[(u, 0.19), (u + 0.01, 0.06)], 0.018, 0.002, WHITE, 1.0); }
    } else if k == 12 {
        for side in [-1.0f32, 1.0] {
            let a = 0.25 + side * 0.04;
            taper(g, pen, &[(a, 0.28), (a + side * 0.09, 0.41), (a + side * 0.18, 0.49)], 0.022, 0.003, c2, 1.0);
            for j in 0..3 { let u = a + side * (0.06 + j as f32 * 0.04); seg(g, pen, u, 0.37 + j as f32 * 0.03, u - 0.025, 0.50 + j as f32 * 0.02, 0.012, c2, 1.0); }
        }
    }
    if k == 18 {
        dome(g, pen, -0.08, 0.16, 0.30, 0.29, c, 1.0);
        for row in 0..3 { for j in 0..7 {
            let u = -0.30 + j as f32 * 0.07;
            circ(g, pen, u, 0.20 + row as f32 * 0.065, 0.022, c2, 0.8);
        } }
    } else if k == 30 {
        for u in [0.24, 0.31, 0.38] { seg(g, pen, u, 0.28, u + 0.015, 0.37, 0.025, c2, 1.0); }
        for u in [0.34, 0.40] { taper(g, pen, &[(u, 0.19), (u, 0.07)], 0.02, 0.002, WHITE, 1.0); }
    } else if k == 31 {
        poly(g, pen, &[(0.25, 0.28), (0.30, 0.50), (0.42, 0.49), (0.42, 0.29)], c2, 1.0);
    } else if k == 40 {
        poly(g, pen, &[(0.25, 0.28), (0.30, 0.58), (0.40, 0.28)], c2, 1.0);
    } else if k == 37 {
        ell(g, pen, 0.44, 0.18, 0.13, 0.035, 0.0, c2, 1.0);
        ell(g, pen, -0.39, 0.18, 0.16, 0.055, 0.0, c2, 1.0);
    } else if k == 17 || k == 23 || k == 27 {
        poly(g, pen, &[(0.23, 0.28), (0.25, 0.39), (0.29, 0.28)], c2, 1.0);
        poly(g, pen, &[(0.31, 0.28), (0.33, 0.38), (0.36, 0.28)], c2, 1.0);
    }
}

fn penguin(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    ell(g, pen, 0.0, 0.0, 0.15, 0.29, 0.0, c, 1.0);
    ell(g, pen, 0.05, -0.03, 0.09, 0.23, 0.0, WHITE, 0.95);
    circ(g, pen, 0.05, 0.30, 0.09, c, 1.0);
    poly(g, pen, &[(0.10, 0.34), (0.25, 0.30), (0.12, 0.27)], c2, 1.0);
    let stroke = (t * 3.0).sin() * 0.06;
    taper(g, pen, &[(-0.10, 0.15), (-0.25, -0.08 + stroke), (-0.26, -0.20 + stroke)], 0.05, 0.01, c, 1.0);
    for u in [-0.05, 0.07] { seg(g, pen, u, -0.25, u + 0.09, -0.31, 0.025, c2, 1.0); }
    eye(g, pen, 0.09, 0.32, 0.01);
}

fn wading_bird(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    ell(g, pen, -0.07, 0.12, 0.18, 0.11, 0.0, c, 1.0);
    taper(g, pen, &[(0.04, 0.16), (0.10, 0.38), (0.18, 0.41)], 0.045, 0.03, c, 1.0);
    circ(g, pen, 0.20, 0.41, 0.06, c, 1.0);
    ell(g, pen, 0.30, 0.40, 0.10, 0.025, 0.0, c2, 1.0);
    for side in [-1.0f32, 1.0] { let stride = (t * 3.0).sin() * side * 0.03; seg(g, pen, side * 0.05, 0.06, side * 0.05 + stride, -0.33, 0.018, c2, 1.0); }
    eye(g, pen, 0.22, 0.43, 0.01);
}

/// ワニの仲間など 0=ラウイスクス類 1=アエトサウルス類（鎧） 2=ワニ 3=首の長いタニストロフェウス
pub(crate) fn archosaur(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32) {
    let sway = (t * 1.4).sin() * 0.015;
    let (hip, brv) = match k {
        2 => (0.05, 0.05),
        3 => (0.08, 0.05),
        1 => (0.12, 0.07),
        _ => (0.16, 0.08),
    };
    let head = match k {
        2 => (0.36, hip + 0.04, 0.14, 0.035),
        3 => (0.5, hip + 0.1, 0.035, 0.02),
        1 => (0.3, hip + 0.08, 0.06, 0.03),
        _ => (0.3, hip + 0.12, 0.1, 0.05),
    };
    let w = Walker {
        body: (if k == 3 { -0.22 } else { -0.04 }, hip + brv * 0.8, if k == 3 { 0.1 } else { 0.18 }, brv),
        neck: if k == 3 { curve((-0.13, hip + 0.06), (0.48, hip + 0.1), 0.05, 8) } else { vec![(0.12, hip + brv), (head.0 - head.2 * 0.6, head.1)] },
        neck_w: if k == 3 { 0.025 } else { brv * 1.1 },
        head,
        tail: curve((if k == 3 { -0.3 } else { -0.2 }, hip + brv * 0.7), (-0.5, hip + 0.01 + sway), 0.0, 6),
        tail_w: brv * 1.3,
        legs: if k == 3 { vec![(-0.28, hip, 0.03), (-0.16, hip, 0.03)] } else { vec![(-0.14, hip, 0.045), (0.08, hip, 0.04)] },
    };
    walker(g, pen, &w, c, c2, t, if k == 2 { 1.0 } else { 2.0 });
    if k == 1 || k == 2 {
        for i in 0..9 {
            let u = -0.36 + i as f32 * 0.07;
            let v = hip + brv * 1.6 - (u + 0.04).abs() * 0.1;
            if k == 1 && (i == 6 || i == 7) {
                poly(g, pen, &[(u - 0.02, v), (u + 0.05, v + 0.08), (u + 0.02, v)], WHITE, 0.9);
            }
            circ(g, pen, u, v, 0.016, c2, 0.9);
        }
    }
    if k == 0 || k == 2 {
        let (hu, hv, hr, _) = head;
        for i in 0..5 {
            let u = hu + hr * 0.1 + i as f32 * hr * 0.18;
            seg(g, pen, u, hv - 0.012, u, hv - 0.03, 0.005, WHITE, 0.9);
        }
    }
}

/// 中生代の植物 12=ソテツの仲間 13=イチョウ 14=花の咲く木
pub(crate) fn plant_meso(g: &mut Gfx, pen: &Pen, k: u8, c: [f32; 3], c2: [f32; 3], t: f32, seed: u32) {
    let p0 = Pen { fx: 1.0, ..*pen };
    let sway = (t * 0.6 + seed as f32 * 0.31).sin() * 0.015;
    match k {
        12 => {
            // 太くて短い幹と、かたい羽のような葉
            taper(g, &p0, &[(0.0, -0.5), (0.0, -0.1)], 0.2, 0.17, c, 1.0);
            for i in 0..6 {
                let v = -0.45 + i as f32 * 0.06;
                seg(g, &p0, -0.09, v, 0.09, v + 0.02, 0.01, mul(c, 0.75), 0.8);
            }
            for j in 0..11 {
                let a = -1.5 + j as f32 * 0.3 + sway;
                let end = (a.sin() * 0.42, -0.08 + a.cos().max(0.1) * 0.35);
                taper(g, &p0, &[(0.0, -0.1), ((end.0) * 0.5, -0.1 + (end.1 + 0.1) * 0.6 + 0.04), end], 0.03, 0.01, c2, 1.0);
                for q in 1..5 {
                    let f = q as f32 / 5.0;
                    let (bx, by) = (end.0 * f, -0.1 + (end.1 + 0.1) * f + 0.04 * (1.0 - (f - 0.5).abs() * 2.0));
                    seg(g, &p0, bx, by, bx + 0.03, by - 0.03, 0.006, mix(c2, WHITE, 0.1), 0.8);
                }
            }
        }
        18 => {
            for side in [-1.0f32, 1.0] { for i in 0..3 {
                let u = side * (0.17 + i as f32 * 0.07);
                let top = 0.38 - i as f32 * 0.11;
                taper(g, &p0, &[(0.0, -0.49), (u * 0.55, -0.12), (u + sway, top)], 0.018, 0.005, c, 1.0);
                for j in 1..7 {
                    let f = j as f32 / 7.0;
                    let (x,y) = (u * f + sway * f, -0.49 + (top + 0.49) * f);
                    let w = 0.065 * (1.0 - f * 0.5);
                    seg(g, &p0, x, y, x - w, y + 0.035, 0.016, c2, 1.0);
                    seg(g, &p0, x, y, x + w, y + 0.035, 0.016, c2, 1.0);
                }
            } }
        }
        15 => {
            for i in 0..17 {
                let u = (i as f32 - 8.0) * 0.025;
                let h = 0.35 + (i as f32 * 1.7).sin().abs() * 0.38;
                seg(g, &p0, u, -0.45, u + sway + (i as f32 * 2.1).sin() * 0.16, -0.45 + h, 0.013, c2, 1.0);
            }
        }
        13 => {
            // イチョウ：まっすぐな幹と扇形の葉の茂み
            taper(g, &p0, &[(0.0, -0.5), (sway * 2.0, 0.42)], 0.08, 0.04, c, 1.0);
            for i in 0..5 {
                let v = -0.15 + i as f32 * 0.13;
                for s in [-1.0f32, 1.0] {
                    let l = 0.22 - i as f32 * 0.03;
                    seg(g, &p0, 0.0, v, s * l, v + 0.05, 0.012, mul(c, 0.9), 1.0);
                    for q in 0..3 {
                        let bx = s * l * (0.4 + q as f32 * 0.3);
                        poly(g, &p0, &[(bx, v + 0.03), (bx - 0.04, v + 0.09), (bx + 0.04, v + 0.09)], c2, 0.95);
                    }
                }
            }
        }
        _ => {
            // 花の咲く木：丸い茂みと花
            taper(g, &p0, &[(0.0, -0.5), (sway * 2.0, 0.0)], 0.07, 0.05, c, 1.0);
            for (u, v, r) in [(0.0, 0.12, 0.28), (-0.18, 0.02, 0.18), (0.18, 0.04, 0.18), (0.0, 0.3, 0.18)] {
                ell(g, &p0, u + sway, v, r, r * 0.8, 0.0, c2, 1.0);
            }
            for i in 0..12 {
                let h1 = hash2(seed as i32, i, 41);
                let h2 = hash2(seed as i32, i, 43);
                let (u, v) = ((h1 - 0.5) * 0.5 + sway, -0.05 + h2 * 0.45);
                for q in 0..5 {
                    let a = q as f32 / 5.0 * TAU;
                    circ(g, &p0, u + a.cos() * 0.018, v + a.sin() * 0.018, 0.014, [1.0, 0.92, 0.94], 0.95);
                }
                circ(g, &p0, u, v, 0.01, [1.0, 0.8, 0.3], 1.0);
            }
        }
    }
}

fn flightless_bird(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    ell(g, pen, -0.08, 0.05, 0.21, 0.18, 0.0, c, 1.0);
    taper(g, pen, &[(0.03, 0.1), (0.12, 0.31)], 0.10, 0.06, c, 1.0);
    circ(g, pen, 0.17, 0.33, 0.08, c, 1.0);
    poly(g, pen, &[(0.22, 0.38), (0.38, 0.34), (0.29, 0.24), (0.22, 0.26)], c2, 1.0);
    circ(g, pen, 0.19, 0.35, 0.012, INK, 1.0);
    for side in [-1.0f32, 1.0] {
        let stride = (t * 4.0).sin() * 0.05 * side;
        taper(g, pen, &[(side * 0.05, -0.05), (side * 0.06 + stride, -0.24), (side * 0.08 + stride, -0.38)], 0.035, 0.02, c2, 1.0);
        seg(g, pen, side * 0.08 + stride, -0.38, 0.10 + side * 0.08 + stride, -0.39, 0.025, c2, 1.0);
    }
}

fn crab(g: &mut Gfx, pen: &Pen, c: [f32; 3], c2: [f32; 3], t: f32) {
    ell(g, pen, 0.0, 0.0, 0.22, 0.11, 0.0, c, 1.0);
    for side in [-1.0f32, 1.0] {
        for i in 0..4 {
            let x = -0.16 + i as f32 * 0.10;
            let wave = (t * 4.0 + i as f32).sin() * 0.015;
            taper(g, pen, &[(x, side * 0.06), (x - 0.09, side * 0.15), (x - 0.16, side * (0.22 + wave))], 0.025, 0.008, c2, 1.0);
        }
        taper(g, pen, &[(0.12, side * 0.08), (0.30, side * 0.18), (0.40, side * 0.12)], 0.035, 0.03, c, 1.0);
        seg(g, pen, 0.40, side * 0.12, 0.47, side * 0.15, 0.035, c2, 1.0);
        seg(g, pen, 0.40, side * 0.12, 0.47, side * 0.08, 0.03, c2, 1.0);
        circ(g, pen, 0.20, side * 0.06, 0.015, INK, 1.0);
    }
}
