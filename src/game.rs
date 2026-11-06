//! 画面と操作：タイトル、地図（上空）、観察（潜る・降りる）、図鑑、宿題、記録（両方の版で共通）
//!
//! 地図は正距円筒図法の地球。カプセルを動かして場所を選び、下の年表で時代を選ぶ（時間旅行）。
//! 観察の場面では生きものを撮影し、写りのよさ（★1〜3）とともに図鑑に収録する。

use crate::art;
use crate::dive::*;
use crate::earth::*;
use crate::gfx::{Font, Gfx, Texture};
use crate::lang::{self, L};
use crate::life::*;
use crate::mission::*;
use crate::platform;
use crate::progress::Progress;
use crate::Ev;

const VK_BACK: u32 = 0x08;
const VK_RETURN: u32 = 0x0d;
const VK_SHIFT: u32 = 0x10;
const VK_CONTROL: u32 = 0x11;
const VK_ESCAPE: u32 = 0x1b;
const VK_SPACE: u32 = 0x20;
const VK_LEFT: u32 = 0x25;
const VK_UP: u32 = 0x26;
const VK_RIGHT: u32 = 0x27;
const VK_DOWN: u32 = 0x28;
const VK_F1: u32 = 0x70;
const VK_F2: u32 = 0x71;
const VK_F3: u32 = 0x72;
const VK_F11: u32 = 0x7a;
const VK_PLUS: u32 = 0xbb;
const VK_MINUS: u32 = 0xbd;
const VK_ADD: u32 = 0x6b;
const VK_SUB: u32 = 0x6d;
const VK_COMMA: u32 = 0xbc;
const VK_PERIOD: u32 = 0xbe;

// 色
const BG: [f32; 4] = [0.03, 0.10, 0.17, 0.90];
const BG2: [f32; 4] = [0.06, 0.17, 0.26, 0.94];
const EDGE: [f32; 4] = [0.45, 0.85, 0.80, 0.55];
const TEXT: [f32; 4] = [0.93, 0.97, 0.96, 1.0];
const DIM: [f32; 4] = [0.66, 0.78, 0.80, 1.0];
const ACCENT: [f32; 4] = [0.98, 0.70, 0.30, 1.0];
const GOLD: [f32; 4] = [1.0, 0.84, 0.32, 1.0];
const GOOD: [f32; 4] = [0.55, 0.92, 0.62, 1.0];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Screen {
    Title,
    Map,
    Dive,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Panel {
    None,
    Guide,
    Entry(usize),
    Missions,
    Help,
    Intro(u8),
    Done(usize),
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Act {
    NewGame,
    Continue,
    Import,
    Export,
    Lang,
    Fullscreen,
    Dive,
    Surface,
    Shoot,
    Guide,
    Missions,
    Help,
    Close,
    Entry(usize),
    Goto(Target),
    StepStage(i32),
    TimeAt(f32),
    Light,
    Lens(i32),
    ToTitle,
    IntroNext,
    MissionCard,
    Boost,
    Jump(bool),
}

struct Button {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    act: Act,
    enabled: bool,
}

struct Travel {
    from: f32,
    to: f32,
    el: f32,
    dur: f32,
}

struct PhotoCard {
    /// (図鑑の項目, 星, 新しい, 星が増えた)
    lines: Vec<(usize, u8, bool, bool)>,
    /// 写りが悪かったときの助言
    advice: Option<&'static str>,
    left: f32,
    done_missions: Vec<usize>,
}

pub struct Game {
    gfx: Gfx,
    earth: Earth,
    prog: Progress,
    screen: Screen,
    panel: Panel,
    ocean_tex: Texture,
    cloud_tex: Texture,
    plate_tex: Vec<Texture>,
    // 地図
    cam: (f32, f32),
    zoom: f32,
    follow: bool,
    cap: (f32, f32),
    cap_target: Option<(f32, f32)>,
    travel: Option<Travel>,
    // 観察
    scene: Option<Scene>,
    lens: f32,
    dcam: (f32, f32),
    steer: Option<(f32, f32)>,
    dive_goto: Option<(f32, f32)>,
    boost_btn: bool,
    // 入力
    mouse: (f32, f32),
    keys: [bool; 256],
    ldown: Option<(f32, f32, bool)>,
    drag: Option<(f32, f32, (f32, f32))>,
    touch: bool,
    buttons: Vec<Button>,
    // 表示
    notices: Vec<(String, f32)>,
    banner: Vec<(usize, f32)>,
    photo: Option<PhotoCard>,
    flash: f32,
    clock: f32,
    guide_scroll: f32,
    guide_max: f32,
    mission_open: bool,
    have_save: bool,
    confirm_new: f32,
    save_cool: f32,
    save_dirty: bool,
    title_t: f32,
    tip: Option<String>,
    seed: u32,
}

fn c4(c: [f32; 3], a: f32) -> [f32; 4] {
    [c[0], c[1], c[2], a]
}

fn mix4(a: [f32; 4], b: [f32; 4], t: f32) -> [f32; 4] {
    [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t, a[2] + (b[2] - a[2]) * t, a[3] + (b[3] - a[3]) * t]
}

fn stars_text(n: u8) -> String {
    let mut s = String::new();
    for i in 0..3 {
        s.push(if i < n { '★' } else { '☆' });
    }
    s
}

fn ease(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn entry_name(e: usize) -> &'static str {
    if e < n_species() {
        SPECIES[e].name.get()
    } else {
        PHENOMS[e - n_species()].name.get()
    }
}

/// 時代の幅（「5億1500万〜5億500万年前」）
fn span_text(t0: f32, t1: f32) -> String {
    let a = ma_text(t0);
    let b = ma_text(t1);
    match lang::lang() {
        lang::Lang::En => format!("{:.1}–{:.1} million years ago", t0, t1),
        _ => format!("{}〜{}", a.trim_end_matches("年前"), b),
    }
}

fn size_text(cm: f32) -> String {
    if cm < 0.1 {
        tf!("約{:.1}mm", "about {:.1} mm", "约{:.1}毫米", cm * 10.0)
    } else if cm < 1.0 {
        tf!("約{:.0}mm", "about {:.0} mm", "约{:.0}毫米", cm * 10.0)
    } else if cm >= 100.0 {
        tf!("約{:.1}m", "about {:.1} m", "约{:.1}米", cm / 100.0)
    } else {
        tf!("約{:.0}cm", "about {:.0} cm", "约{:.0}厘米", cm)
    }
}

fn hab_list(bits: u8) -> String {
    let all = [Habitat::Land, Habitat::Tidal, Habitat::Shallow, Habitat::Shelf, Habitat::Deep];
    all.iter().filter(|h| bits & h.bit() != 0).map(|h| h.name()).collect::<Vec<_>>().join(lang::sep())
}

impl Game {
    pub fn new(gfx: Gfx) -> Game {
        let earth = Earth::new();
        let (ow, oh, opx) = ocean_texture();
        let ocean_tex = gfx.create_texture(ow, oh, &opx).expect("texture");
        let (cw, ch, cpx) = cloud_texture();
        let cloud_tex = gfx.create_texture(cw, ch, &cpx).expect("texture");
        let plate_tex = earth.plates.iter().map(|p| gfx.create_texture(p.w as u32, p.h as u32, &p.px).expect("texture")).collect();
        let prog = Progress::new();
        let cap = lonlat_to_map(prog.lon, prog.lat);
        let mut g = Game {
            gfx,
            earth,
            prog,
            screen: Screen::Title,
            panel: Panel::None,
            ocean_tex,
            cloud_tex,
            plate_tex,
            cam: (MAP_W * 0.5, MAP_H * 0.5),
            zoom: 0.4,
            follow: true,
            cap,
            cap_target: None,
            travel: None,
            scene: None,
            lens: 1.0,
            dcam: (0.0, 0.0),
            steer: None,
            dive_goto: None,
            boost_btn: false,
            mouse: (-100.0, -100.0),
            keys: [false; 256],
            ldown: None,
            drag: None,
            touch: false,
            buttons: Vec::new(),
            notices: Vec::new(),
            banner: Vec::new(),
            photo: None,
            flash: 0.0,
            clock: 0.0,
            guide_scroll: 0.0,
            guide_max: 0.0,
            mission_open: true,
            have_save: platform::slot_info(0).is_some(),
            confirm_new: 0.0,
            save_cool: 0.0,
            save_dirty: false,
            title_t: 0.0,
            tip: None,
            seed: platform::random_u32(),
        };
        g.fit_world();
        g
    }

    fn w(&self) -> f32 {
        self.gfx.width as f32
    }
    fn h(&self) -> f32 {
        self.gfx.height as f32
    }
    fn compact(&self) -> bool {
        self.gfx.compact
    }
    fn top_h(&self) -> f32 {
        if self.compact() {
            40.0
        } else {
            46.0
        }
    }

    fn min_zoom(&self) -> f32 {
        (self.w() / MAP_W).min(self.h() / MAP_H) * 0.98
    }

    fn fit_world(&mut self) {
        self.zoom = (self.w() / MAP_W).max(self.h() / MAP_H) * 1.0;
        self.cam = (MAP_W * 0.5, MAP_H * 0.5);
    }

    fn clamp_cam(&mut self) {
        let zmin = self.min_zoom();
        self.zoom = self.zoom.clamp(zmin, 3.5);
        let hw = self.w() * 0.5 / self.zoom;
        let hh = self.h() * 0.5 / self.zoom;
        self.cam.0 = if hw * 2.0 >= MAP_W { MAP_W * 0.5 } else { self.cam.0.clamp(hw, MAP_W - hw) };
        self.cam.1 = if hh * 2.0 >= MAP_H { MAP_H * 0.5 } else { self.cam.1.clamp(hh, MAP_H - hh) };
    }

    fn screen_to_map(&self, sx: f32, sy: f32) -> (f32, f32) {
        ((sx - self.w() * 0.5) / self.zoom + self.cam.0, (sy - self.h() * 0.5) / self.zoom + self.cam.1)
    }

    fn map_to_screen(&self, x: f32, y: f32) -> (f32, f32) {
        ((x - self.cam.0) * self.zoom + self.w() * 0.5, (y - self.cam.1) * self.zoom + self.h() * 0.5)
    }

    fn dive_view(&self) -> View {
        let ppm = self.h() / 4.0 * self.lens;
        View { cx: self.dcam.0, cy: self.dcam.1, ppm, w: self.w(), h: self.h() }
    }

    fn notify_text(&mut self, s: String) {
        self.notices.push((s, 5.0));
        if self.notices.len() > 4 {
            self.notices.remove(0);
        }
    }

    /// 保存・読み込みの結果など（ブラウザ側・デスクトップ版の記録の層から）
    pub fn notify(&mut self, s: &str) {
        self.notify_text(s.to_string());
    }

    /// 記録を読み込んだ
    pub fn apply_loaded(&mut self, data: &[u8]) {
        match Progress::from_bytes(data) {
            Ok(p) => {
                self.prog = p;
                self.cap = lonlat_to_map(self.prog.lon, self.prog.lat);
                self.cap_target = None;
                self.travel = None;
                self.scene = None;
                self.photo = None;
                self.screen = Screen::Map;
                self.panel = Panel::None;
                self.zoom = 0.9;
                self.cam = self.cap;
                self.follow = true;
                self.clamp_cam();
                self.notify_text(t!("記録を読み込んだ", "Save loaded", "已读取存档").to_string());
            }
            Err(e) => self.notify_text(tf!("読み込めなかった：{}", "Could not load: {}", "无法读取：{}", e)),
        }
    }

    /// 状況の説明（デスクトップ版の異常終了ログ用）
    #[allow(dead_code)]
    pub fn state_text(&self) -> String {
        format!("screen={:?} panel={:?} t={:.1} mission={} found={}", self.screen, self.panel, self.prog.t, self.prog.mission, self.prog.found())
    }

    /// 異常終了したときに残す記録
    #[allow(dead_code)]
    pub fn crash_bytes(&self) -> Option<Vec<u8>> {
        if self.screen == Screen::Title {
            None
        } else {
            Some(self.prog.to_bytes())
        }
    }

    fn autosave(&mut self, now: bool) {
        self.save_dirty = true;
        if now {
            self.save_cool = 0.0;
        }
    }

    fn save_tick(&mut self, dt: f32) {
        self.save_cool -= dt;
        if self.save_dirty && self.save_cool <= 0.0 && self.screen != Screen::Title {
            let (lon, lat) = map_to_lonlat(self.cap.0, self.cap.1);
            self.prog.lon = lon;
            self.prog.lat = lat;
            let data = self.prog.to_bytes();
            platform::save_slot(0, &data, &self.prog.meta());
            self.have_save = true;
            self.save_dirty = false;
            self.save_cool = 4.0;
        }
    }

    /// タッチした場所に何があるか：0=地図・場面 1=画面部品 3=なぞってスクロールできる一覧
    #[cfg_attr(windows, allow(dead_code))]
    pub fn hit_kind(&self, x: f32, y: f32) -> u32 {
        let (x, y) = (x / self.gfx.ui, y / self.gfx.ui);
        if self.button_at(x, y).is_some() {
            return 1;
        }
        if self.panel == Panel::Guide {
            return 3;
        }
        if self.panel != Panel::None || self.screen == Screen::Title || self.photo.is_some() {
            return 1;
        }
        if self.over_ui(x, y) {
            return 1;
        }
        0
    }

    fn button_at(&self, x: f32, y: f32) -> Option<Act> {
        self.buttons.iter().rev().find(|b| b.enabled && x >= b.x0 && x <= b.x1 && y >= b.y0 && y <= b.y1).map(|b| b.act)
    }

    /// 地図・場面の操作を受け付けない画面部品の上か
    fn over_ui(&self, x: f32, y: f32) -> bool {
        if y < self.top_h() {
            return true;
        }
        if self.screen == Screen::Map {
            let (tx0, ty0, _, _) = self.timeline_rect();
            if y > ty0 - 4.0 && x > tx0 - 60.0 {
                return true;
            }
            let (lx0, ly0, lx1, ly1) = self.loc_rect();
            if x >= lx0 && x <= lx1 && y >= ly0 && y <= ly1 {
                return true;
            }
        }
        if let Some((mx0, my0, mx1, my1)) = self.mission_rect() {
            if x >= mx0 && x <= mx1 && y >= my0 && y <= my1 {
                return true;
            }
        }
        false
    }

    // ------------------------------------------------------------ 1フレーム

    pub fn frame(&mut self, evs: &[Ev], dt: f32, focused: bool) -> bool {
        if let Some(data) = platform::take_loaded() {
            self.apply_loaded(&data);
        }
        for n in platform::take_notes() {
            self.notify(&n);
        }
        let _ = focused;
        self.clock += dt;
        let ui = self.gfx.ui;
        for e in evs {
            let e = match *e {
                Ev::MouseMove(x, y) => Ev::MouseMove(x / ui, y / ui),
                Ev::Down(b, x, y) => Ev::Down(b, x / ui, y / ui),
                Ev::Up(b, x, y) => Ev::Up(b, x / ui, y / ui),
                Ev::Scroll(d) => Ev::Scroll(d / ui),
                o => o,
            };
            if self.handle(e) {
                return true;
            }
        }
        self.update(dt);
        self.save_tick(dt);
        self.render();
        false
    }

    fn update(&mut self, dt: f32) {
        self.confirm_new = (self.confirm_new - dt).max(0.0);
        self.flash = (self.flash - dt * 2.2).max(0.0);
        for n in &mut self.notices {
            n.1 -= dt;
        }
        self.notices.retain(|n| n.1 > 0.0);
        if let Some(b) = self.banner.first_mut() {
            b.1 -= dt;
            if b.1 <= 0.0 {
                self.banner.remove(0);
            }
        }
        if let Some(p) = &mut self.photo {
            p.left -= dt;
            if p.left <= 0.0 {
                self.close_photo();
            }
        }
        match self.screen {
            Screen::Title => {
                self.title_t += dt;
                let (w, h) = (self.w(), self.h());
                self.zoom = (w / MAP_W).max(h / MAP_H) * 1.15;
                self.cam = (MAP_W * 0.5 + (self.title_t * 0.05).sin() * 120.0, MAP_H * 0.52);
                self.clamp_cam();
            }
            Screen::Map => self.update_map(dt),
            Screen::Dive => self.update_dive(dt),
        }
    }

    fn update_map(&mut self, dt: f32) {
        // 時間旅行
        if let Some(tr) = &mut self.travel {
            tr.el += dt;
            let k = ease(tr.el / tr.dur);
            let t = tr.from + (tr.to - tr.from) * k;
            let prev = self.prog.t;
            self.prog.t = t;
            // 節目を越えた
            for (i, m) in MILESTONES.iter().enumerate() {
                let crossed = (prev > m.ma && t <= m.ma) || (prev < m.ma && t >= m.ma);
                if crossed {
                    self.banner = vec![(i, 6.5)];
                }
            }
            if tr.el >= tr.dur {
                self.prog.t = tr.to;
                self.travel = None;
                self.autosave(false);
            }
        }
        // カプセルの移動
        if self.panel == Panel::None && self.photo.is_none() {
            let mut d = (0.0f32, 0.0f32);
            if self.keys[0x41] {
                d.0 -= 1.0;
            }
            if self.keys[0x44] {
                d.0 += 1.0;
            }
            if self.keys[0x57] {
                d.1 -= 1.0;
            }
            if self.keys[0x53] {
                d.1 += 1.0;
            }
            if d != (0.0, 0.0) {
                let sp = 240.0 / self.zoom.max(0.3) * dt;
                self.cap.0 = (self.cap.0 + d.0 * sp).clamp(0.0, MAP_W);
                self.cap.1 = (self.cap.1 + d.1 * sp).clamp(20.0, MAP_H - 20.0);
                self.cap_target = None;
                self.follow = true;
            }
            // 矢印キーで地図を動かす
            let mut p = (0.0f32, 0.0f32);
            if self.keys[VK_LEFT as usize] {
                p.0 -= 1.0;
            }
            if self.keys[VK_RIGHT as usize] {
                p.0 += 1.0;
            }
            if self.keys[VK_UP as usize] {
                p.1 -= 1.0;
            }
            if self.keys[VK_DOWN as usize] {
                p.1 += 1.0;
            }
            if p != (0.0, 0.0) {
                let sp = 700.0 / self.zoom * dt;
                self.cam.0 += p.0 * sp;
                self.cam.1 += p.1 * sp;
                self.follow = false;
            }
        }
        if let Some(tg) = self.cap_target {
            let (dx, dy) = (tg.0 - self.cap.0, tg.1 - self.cap.1);
            let d = (dx * dx + dy * dy).sqrt();
            let v = (d * 2.4).clamp(80.0, 1400.0) * dt;
            if d <= v {
                self.cap = tg;
                self.cap_target = None;
                self.autosave(false);
            } else {
                self.cap.0 += dx / d * v;
                self.cap.1 += dy / d * v;
            }
        }
        if self.follow {
            let k = (dt * 3.0).min(1.0);
            self.cam.0 += (self.cap.0 - self.cam.0) * k;
            self.cam.1 += (self.cap.1 - self.cam.1) * k;
        }
        self.clamp_cam();
        // 先生のことば（初めて）
        if !self.prog.intro_done && self.panel == Panel::None {
            self.panel = Panel::Intro(0);
        }
    }

    fn update_dive(&mut self, dt: f32) {
        let blocked = self.panel != Panel::None;
        let mut ax = 0.0;
        let mut ay = 0.0;
        if !blocked {
            if self.keys[0x41] || self.keys[VK_LEFT as usize] {
                ax -= 1.0;
            }
            if self.keys[0x44] || self.keys[VK_RIGHT as usize] {
                ax += 1.0;
            }
            if self.keys[0x57] || self.keys[VK_UP as usize] {
                ay -= 1.0;
            }
            if self.keys[0x53] || self.keys[VK_DOWN as usize] {
                ay += 1.0;
            }
            if self.keys[0x51] {
                self.lens = (self.lens * (1.0 - dt * 1.2)).max(1.0);
            }
            if self.keys[0x45] {
                self.lens = (self.lens * (1.0 + dt * 1.2)).min(8.0);
            }
        }
        if ax != 0.0 || ay != 0.0 {
            self.dive_goto = None;
        }
        let v = self.dive_view();
        let target = if blocked {
            None
        } else if let Some((sx, sy)) = self.steer {
            Some(v.to_world(sx, sy))
        } else {
            self.dive_goto
        };
        let boost = self.keys[VK_SHIFT as usize] || self.boost_btn;
        let lens = self.lens;
        let Some(sc) = &mut self.scene else { return };
        sc.update(dt, ax, ay, target, boost);
        if let Some(g) = self.dive_goto {
            let (dx, dy) = (g.0 - sc.cap.x, g.1 - sc.cap.y);
            if dx * dx + dy * dy < 0.09 {
                self.dive_goto = None;
            }
        }
        // カメラ：カプセルの前（レンズ）を追う
        let lead = if lens > 1.7 { 0.45 } else { 0.25 };
        let tx = sc.cap.x + sc.cap.facing * lead + sc.cap.vx * 0.25;
        let ty = sc.cap.y + sc.cap.vy * 0.2;
        let k = (dt * 5.0).min(1.0);
        self.dcam.0 += (tx - self.dcam.0) * k;
        self.dcam.1 += (ty - self.dcam.1) * k;
    }

    // ------------------------------------------------------------ 入力

    fn handle(&mut self, e: Ev) -> bool {
        match e {
            Ev::Resize(w, h) => {
                self.gfx.resize(w, h);
                self.clamp_cam();
            }
            Ev::Compact(c) => {
                self.gfx.set_compact(c);
                self.clamp_cam();
            }
            Ev::Touch(t) => self.touch = t,
            Ev::MouseMove(x, y) => {
                self.mouse = (x, y);
                if let Some((sx, sy, cam0)) = self.drag {
                    if self.screen == Screen::Map && self.panel == Panel::None {
                        self.cam = (cam0.0 - (x - sx) / self.zoom, cam0.1 - (y - sy) / self.zoom);
                        self.follow = false;
                        self.clamp_cam();
                    }
                }
                if let Some((lx, ly, moved)) = self.ldown {
                    let far = (x - lx).abs() + (y - ly).abs() > 8.0;
                    if self.screen == Screen::Map && !moved && far && self.panel == Panel::None && self.photo.is_none() {
                        self.ldown = Some((lx, ly, true));
                        self.drag = Some((lx, ly, self.cam));
                    }
                }
                if self.screen == Screen::Dive && self.steer.is_some() {
                    self.steer = Some((x, y));
                }
            }
            Ev::Down(b, x, y) => {
                self.mouse = (x, y);
                if b == 0 {
                    if let Some(a) = self.button_at(x, y) {
                        self.ldown = None;
                        self.do_act(a);
                        return false;
                    }
                    if self.photo.is_some() {
                        self.close_photo();
                        return false;
                    }
                    match self.panel {
                        Panel::Intro(_) => {
                            self.do_act(Act::IntroNext);
                            return false;
                        }
                        Panel::Done(_) => {
                            self.panel = Panel::None;
                            return false;
                        }
                        Panel::None => {}
                        _ => return false,
                    }
                    if self.screen == Screen::Map && !self.over_ui(x, y) {
                        self.ldown = Some((x, y, false));
                    } else if self.screen == Screen::Dive && !self.over_ui(x, y) {
                        self.steer = Some((x, y));
                        self.ldown = Some((x, y, false));
                    }
                } else if b == 2 || (b == 1 && self.screen == Screen::Map) {
                    if self.screen == Screen::Map && self.panel == Panel::None {
                        self.drag = Some((x, y, self.cam));
                    } else if self.screen == Screen::Dive && self.panel == Panel::None && self.photo.is_none() {
                        self.steer = Some((x, y));
                    }
                } else if b == 1 && self.screen == Screen::Dive && self.panel == Panel::None {
                    self.do_act(Act::Shoot);
                }
            }
            Ev::Up(b, x, y) => {
                self.mouse = (x, y);
                if b == 0 {
                    if let Some((lx, ly, moved)) = self.ldown.take() {
                        if self.screen == Screen::Map && !moved {
                            let p = self.screen_to_map(lx, ly);
                            self.cap_target = Some((p.0.clamp(0.0, MAP_W), p.1.clamp(20.0, MAP_H - 20.0)));
                            self.follow = true;
                        }
                        if self.screen == Screen::Dive {
                            let short = (x - lx).abs() + (y - ly).abs() < 12.0;
                            if short {
                                self.dive_goto = Some(self.dive_view().to_world(x, y));
                            }
                        }
                    }
                    self.drag = None;
                    self.steer = None;
                    self.boost_btn = false;
                } else {
                    self.drag = None;
                    if self.screen == Screen::Dive {
                        self.steer = None;
                    }
                }
            }
            Ev::Wheel(d) => {
                if self.panel == Panel::Guide {
                    self.guide_scroll = (self.guide_scroll - d * 60.0).clamp(0.0, self.guide_max);
                } else if self.panel == Panel::None {
                    match self.screen {
                        Screen::Map => {
                            let before = self.screen_to_map(self.mouse.0, self.mouse.1);
                            self.zoom *= 1.15f32.powf(d);
                            self.clamp_cam();
                            let after = self.screen_to_map(self.mouse.0, self.mouse.1);
                            self.cam.0 += before.0 - after.0;
                            self.cam.1 += before.1 - after.1;
                            self.follow = false;
                            self.clamp_cam();
                        }
                        Screen::Dive => self.lens = (self.lens * 1.15f32.powf(d)).clamp(1.0, 8.0),
                        _ => {}
                    }
                }
            }
            Ev::Scroll(d) => {
                if self.panel == Panel::Guide {
                    self.guide_scroll = (self.guide_scroll - d).clamp(0.0, self.guide_max);
                }
            }
            Ev::Key(k, down) => {
                if (k as usize) < 256 {
                    self.keys[k as usize] = down;
                }
                if down {
                    self.key_down(k);
                }
            }
            Ev::Char(_) => {}
        }
        false
    }

    fn key_down(&mut self, k: u32) {
        let ctrl = self.keys[VK_CONTROL as usize];
        if ctrl && (k == VK_PLUS || k == VK_ADD) {
            self.gfx.adjust_scale(1);
            return;
        }
        if ctrl && (k == VK_MINUS || k == VK_SUB) {
            self.gfx.adjust_scale(-1);
            return;
        }
        if k == VK_F11 {
            self.gfx.toggle_fullscreen();
            return;
        }
        if self.photo.is_some() {
            if k == VK_SPACE || k == VK_RETURN || k == VK_ESCAPE {
                self.close_photo();
            }
            return;
        }
        match self.panel {
            Panel::Intro(_) => {
                if k == VK_SPACE || k == VK_RETURN || k == VK_ESCAPE {
                    self.do_act(Act::IntroNext);
                }
                return;
            }
            Panel::Done(_) => {
                if k == VK_SPACE || k == VK_RETURN || k == VK_ESCAPE {
                    self.panel = Panel::None;
                }
                return;
            }
            Panel::Entry(i) => {
                if k == VK_LEFT || k == 0x41 {
                    self.panel = Panel::Entry((i + n_entries() - 1) % n_entries());
                } else if k == VK_RIGHT || k == 0x44 {
                    self.panel = Panel::Entry((i + 1) % n_entries());
                } else if k == VK_ESCAPE || k == VK_BACK || k == VK_F2 || k == 0x47 {
                    self.panel = Panel::Guide;
                }
                return;
            }
            Panel::Guide | Panel::Missions | Panel::Help => {
                if k == VK_ESCAPE || (k == VK_F2 && self.panel == Panel::Guide) || (k == VK_F1 && self.panel == Panel::Missions) || (k == VK_F3 && self.panel == Panel::Help) || k == 0x47 && self.panel == Panel::Guide || k == 0x4d && self.panel == Panel::Missions || k == 0x48 && self.panel == Panel::Help {
                    self.panel = Panel::None;
                }
                return;
            }
            Panel::None => {}
        }
        match self.screen {
            Screen::Title => {
                if k == VK_RETURN || k == VK_SPACE {
                    if self.have_save {
                        self.do_act(Act::Continue);
                    } else {
                        self.do_act(Act::NewGame);
                    }
                }
            }
            Screen::Map => match k {
                VK_RETURN => self.do_act(Act::Dive),
                VK_SPACE => self.do_act(Act::Shoot),
                VK_F1 | 0x4d => self.do_act(Act::Missions),
                VK_F2 | 0x47 => self.do_act(Act::Guide),
                VK_F3 | 0x48 => self.do_act(Act::Help),
                VK_COMMA => self.do_act(Act::StepStage(-1)),
                VK_PERIOD => self.do_act(Act::StepStage(1)),
                VK_PLUS | VK_ADD => {
                    self.zoom *= 1.25;
                    self.clamp_cam();
                }
                VK_MINUS | VK_SUB => {
                    self.zoom /= 1.25;
                    self.clamp_cam();
                }
                VK_ESCAPE => self.do_act(Act::Help),
                _ => {}
            },
            Screen::Dive => match k {
                VK_RETURN | VK_ESCAPE => self.do_act(Act::Surface),
                VK_SPACE => self.do_act(Act::Shoot),
                0x4c => self.do_act(Act::Light),
                0x46 => {
                    let near = self.scene.as_ref().map_or(true, |s| s.floor_at(s.cap.x) - s.cap.y < 3.0);
                    let deep = self.scene.as_ref().map_or(false, |s| s.hab == Habitat::Deep || s.depth > 120.0);
                    if deep {
                        self.do_act(Act::Jump(!near));
                    }
                }
                VK_F1 | 0x4d => self.do_act(Act::Missions),
                VK_F2 | 0x47 => self.do_act(Act::Guide),
                VK_F3 | 0x48 => self.do_act(Act::Help),
                VK_PLUS | VK_ADD => self.do_act(Act::Lens(1)),
                VK_MINUS | VK_SUB => self.do_act(Act::Lens(-1)),
                _ => {}
            },
        }
    }

    fn do_act(&mut self, a: Act) {
        platform::crumb(format!("act {:?}", a));
        match a {
            Act::NewGame => {
                if self.have_save && self.confirm_new <= 0.0 {
                    self.confirm_new = 4.0;
                    return;
                }
                self.prog = Progress::new();
                self.cap = lonlat_to_map(self.prog.lon, self.prog.lat);
                self.screen = Screen::Map;
                self.panel = Panel::Intro(0);
                self.zoom = self.min_zoom() * 1.6;
                self.cam = self.cap;
                self.follow = true;
                self.clamp_cam();
                self.autosave(true);
            }
            Act::Continue => platform::load_slot(0),
            Act::Import => platform::import_file(),
            Act::Export => platform::export_slot(0),
            Act::Lang => {
                let l = lang::lang().next();
                lang::set_lang(l);
                platform::save_lang(l);
                self.gfx.reload_fonts();
            }
            Act::Fullscreen => self.gfx.toggle_fullscreen(),
            Act::Dive => self.enter_dive(),
            Act::Surface => {
                if self.screen == Screen::Dive {
                    self.scene = None;
                    self.screen = Screen::Map;
                    self.steer = None;
                    self.dive_goto = None;
                    self.autosave(false);
                }
            }
            Act::Shoot => self.shoot(),
            Act::Guide => {
                self.panel = if self.panel == Panel::Guide { Panel::None } else { Panel::Guide };
            }
            Act::Missions => {
                self.panel = if self.panel == Panel::Missions { Panel::None } else { Panel::Missions };
            }
            Act::Help => {
                self.panel = if self.panel == Panel::Help { Panel::None } else { Panel::Help };
            }
            Act::Close => {
                self.panel = match self.panel {
                    Panel::Entry(_) => Panel::Guide,
                    _ => Panel::None,
                };
            }
            Act::Entry(i) => {
                if i < n_entries() {
                    self.prog.fresh[i] = false;
                    self.panel = Panel::Entry(i);
                }
            }
            Act::Goto(tg) => self.goto(tg),
            Act::StepStage(d) => {
                let t = self.travel.as_ref().map_or(self.prog.t, |tr| tr.to);
                let idx = STAGES.iter().position(|s| t <= s.t0 + 1e-3 && t > s.t1 + 1e-3).unwrap_or(STAGES.len() - 1) as i32;
                let ni = (idx + d).clamp(0, STAGES.len() as i32 - 1) as usize;
                let s = &STAGES[ni];
                self.travel_to(((s.t0 + s.t1) * 0.5 * 10.0).round() / 10.0);
            }
            Act::TimeAt(t) => self.travel_to(t),
            Act::Light => {
                self.prog.light = !self.prog.light;
            }
            Act::Lens(d) => {
                self.lens = (self.lens * 1.35f32.powi(d)).clamp(1.0, 8.0);
            }
            Act::ToTitle => {
                self.autosave(true);
                self.save_tick(0.0);
                self.screen = Screen::Title;
                self.scene = None;
                self.panel = Panel::None;
                self.photo = None;
            }
            Act::IntroNext => {
                if let Panel::Intro(n) = self.panel {
                    if n >= 3 {
                        self.panel = Panel::None;
                        self.prog.intro_done = true;
                        self.mission_open = true;
                        self.autosave(true);
                    } else {
                        self.panel = Panel::Intro(n + 1);
                    }
                }
            }
            Act::MissionCard => self.mission_open = !self.mission_open,
            Act::Boost => self.boost_btn = true,
            Act::Jump(f) => {
                if let Some(sc) = &mut self.scene {
                    sc.jump(f);
                    self.dcam = (sc.cap.x, sc.cap.y);
                    self.flash = 0.6;
                }
            }
        }
    }

    fn travel_to(&mut self, t: f32) {
        if self.screen != Screen::Map {
            return;
        }
        let t = t.clamp(T_END, T_START);
        let from = self.prog.t;
        if (t - from).abs() < 0.05 {
            return;
        }
        let dur = 1.4 + (t - from).abs() / (T_START - T_END) * 2.4;
        self.travel = Some(Travel { from, to: t, el: 0.0, dur });
    }

    /// 宿題の対象のところへ行く（時代と場所）
    fn goto(&mut self, tg: Target) {
        if self.screen == Screen::Dive {
            self.do_act(Act::Surface);
        }
        self.panel = Panel::None;
        match tg {
            Target::Sp(i) => {
                let near = map_to_lonlat(self.cap.0, self.cap.1);
                let (t, (lon, lat)) = where_when(i, &self.earth, self.prog.t, near);
                self.travel_to(t);
                self.cap_target = Some(lonlat_to_map(lon, lat));
                self.zoom = self.zoom.max(1.0);
            }
            Target::Ph(p) => {
                let (t, pos, fit): (f32, (f32, f32), Option<usize>) = match p {
                    P_GONDWANA => {
                        let (x, y, _) = self.earth.pose(GONDWANA, self.prog.t);
                        (self.prog.t, (x, y), Some(GONDWANA))
                    }
                    P_LAURENTIA => {
                        let (x, y, _) = self.earth.pose(LAURENTIA, self.prog.t);
                        (self.prog.t, (x, y), Some(LAURENTIA))
                    }
                    P_ERUPTION => (511.0, self.earth.site_lonlat(S_KALKARINDJI, 511.0), None),
                    _ => (self.prog.t, self.earth.to_world(GONDWANA, self.prog.t, (10.0, 0.0)), None),
                };
                let (lon, lat) = (pos.0, pos.1);
                self.travel_to(t);
                let m = lonlat_to_map(lon, lat);
                self.cap_target = Some(m);
                if let Some(pi) = fit {
                    let (x0, y0, x1, y1) = self.earth.plate_bounds(pi, t);
                    self.zoom = ((self.w() * 0.62) / (x1 - x0)).min((self.h() - 200.0) * 0.8 / (y1 - y0));
                    self.cam = ((x0 + x1) * 0.5, (y0 + y1) * 0.5);
                    self.follow = false;
                    self.clamp_cam();
                } else if p == P_ERUPTION {
                    self.zoom = self.zoom.max(1.4);
                }
            }
        }
        self.follow = self.follow || matches!(tg, Target::Sp(_));
    }

    fn enter_dive(&mut self) {
        if self.screen != Screen::Map || self.travel.is_some() {
            return;
        }
        let (lon, lat) = map_to_lonlat(self.cap.0, self.cap.1);
        let spot = self.earth.sample(lon, lat, self.prog.t);
        self.seed = self.seed.wrapping_mul(1664525).wrapping_add(1013904223);
        let sc = Scene::new(&self.earth, &spot, self.prog.t, self.seed);
        self.dcam = (sc.cap.x + 0.5, sc.cap.y);
        self.scene = Some(sc);
        self.cap_target = None;
        self.lens = 1.0;
        self.screen = Screen::Dive;
        self.steer = None;
        self.dive_goto = None;
        self.ldown = None;
        if spot.hab == Habitat::Deep && !self.prog.light {
            self.notify_text(t!("深い海は暗い。ライト（L）を点けよう", "The deep sea is dark. Turn on your light (L)", "深海很暗，打开灯（L）吧").to_string());
        }
    }

    // ------------------------------------------------------------ 撮影

    fn shoot(&mut self) {
        if self.photo.is_some() || self.panel != Panel::None {
            return;
        }
        let mut found: Vec<(usize, f32)> = Vec::new();
        let mut advice = None;
        match self.screen {
            Screen::Dive => {
                let v = self.dive_view();
                let Some(sc) = &self.scene else { return };
                let r = sc.shoot(&v, self.prog.light);
                for &(sp, s) in &r.subjects {
                    found.push((sp, s));
                }
                if r.landscape {
                    let s = if self.lens < 1.5 { 90.0 } else { 60.0 };
                    found.push((n_species() + P_BARREN, s));
                }
                if r.eruption {
                    found.push((n_species() + P_ERUPTION, 70.0));
                }
                let best = r.subjects.first().map_or(0.0, |b| b.1);
                if r.subjects.is_empty() && !r.landscape {
                    advice = Some(t!("何も写っていない。生きものに近づいて撮ろう", "Nothing in the shot. Get closer to a creature.", "什么也没拍到。靠近生物再拍吧。"));
                } else if best < 30.0 && !r.landscape {
                    let dark = sc.ambient_at(sc.cap.y) < 0.35 && !self.prog.light;
                    advice = Some(if dark {
                        t!("暗くてよく写らない。ライト（L）を点けよう", "Too dark. Turn on your light (L).", "太暗了，拍不清楚。打开灯（L）吧。")
                    } else {
                        t!("小さすぎる。近づくか、ズーム（ホイール・Q/E）で大きく撮ろう", "Too small. Move closer or zoom in (wheel, Q/E).", "太小了。靠近一些或用变焦（滚轮、Q/E）放大。")
                    });
                }
            }
            Screen::Map => {
                found = self.map_shoot();
                if found.iter().all(|f| stars_of(f.1) == 0) {
                    advice = Some(t!(
                        "上空からは大陸の全体や大噴火が撮れる。生きものは潜って（Enter）撮ろう",
                        "From the sky you can shoot whole continents or eruptions. Dive (Enter) to photograph life.",
                        "从高空可以拍下整个大陆或大喷发。要拍生物请潜入（Enter）。"
                    ));
                }
            }
            Screen::Title => return,
        }
        self.flash = 1.0;
        self.prog.photos += 1;
        let mut lines = Vec::new();
        for (e, score) in found {
            let st = stars_of(score);
            if st == 0 {
                continue;
            }
            let (new, better) = self.prog.record(e, st);
            lines.push((e, st, new, better));
        }
        lines.sort_by(|a, b| b.1.cmp(&a.1));
        lines.truncate(6);
        if lines.is_empty() && advice.is_none() {
            advice = Some(t!("写りが悪かった。もっと大きく、真ん中に", "Poor shot. Make the subject bigger and centred.", "拍得不好。拍大一点、放在中间。"));
        }
        let done = self.prog.advance();
        if !lines.is_empty() || !done.is_empty() {
            self.autosave(true);
        }
        self.photo = Some(PhotoCard { lines, advice, left: 4.5, done_missions: done });
    }

    fn close_photo(&mut self) {
        if let Some(p) = self.photo.take() {
            if let Some(&m) = p.done_missions.last() {
                self.panel = Panel::Done(m);
            }
        }
    }

    /// 上空からの写真：大陸の全体・大噴火
    fn map_shoot(&self) -> Vec<(usize, f32)> {
        let (x0, y0) = self.screen_to_map(0.0, self.top_h());
        let (x1, y1) = self.screen_to_map(self.w(), self.h());
        let va = (x1 - x0) * (y1 - y0);
        let t = self.prog.t;
        let mut out = Vec::new();
        for (p, pi) in [(P_GONDWANA, GONDWANA), (P_LAURENTIA, LAURENTIA)] {
            let (bx0, by0, bx1, by1) = self.earth.plate_bounds(pi, t);
            let ba = (bx1 - bx0) * (by1 - by0);
            let ix = ((bx1.min(x1)) - (bx0.max(x0))).max(0.0);
            let iy = ((by1.min(y1)) - (by0.max(y0))).max(0.0);
            let inside = ix * iy / ba;
            let fill = ba / va;
            if inside > 0.9 && fill > 0.12 {
                let s = 30.0 + ((fill - 0.12) / 0.38).clamp(0.0, 1.0) * 60.0 + (inside - 0.9) * 100.0;
                out.push((n_species() + p, s));
            }
        }
        if (509.9..=511.6).contains(&t) {
            let (lon, lat) = self.earth.site_lonlat(S_KALKARINDJI, t);
            let (mx, my) = lonlat_to_map(lon, lat);
            if mx > x0 && mx < x1 && my > y0 && my < y1 {
                let wdeg = (x1 - x0) / DEG;
                let cx = ((mx - (x0 + x1) * 0.5) / (x1 - x0) * 2.0).abs();
                let cy = ((my - (y0 + y1) * 0.5) / (y1 - y0) * 2.0).abs();
                let s = 32.0 + (1.0 - ((wdeg - 12.0) / 70.0).clamp(0.0, 1.0)) * 40.0 + (1.0 - cx.max(cy)) * 25.0;
                out.push((n_species() + P_ERUPTION, s));
            }
        }
        out
    }

    // ------------------------------------------------------------ 描画

    fn render(&mut self) {
        self.buttons.clear();
        self.tip = None;
        self.gfx.begin([0.02, 0.05, 0.09, 1.0]);
        match self.screen {
            Screen::Title | Screen::Map => self.render_map_world(),
            Screen::Dive => {
                let v = self.dive_view();
                let hide = self.lens >= 1.8;
                if let Some(sc) = &self.scene {
                    sc.render(&mut self.gfx, &v, self.prog.light, hide);
                }
            }
        }
        self.gfx.set_screen_xform();
        // 画面部品のうち、絵を含むもの（WebGL / Direct3D で描く）
        match self.panel {
            Panel::Guide => self.render_guide_gl(),
            Panel::Entry(i) => self.render_entry_gl(i),
            _ => {}
        }
        if self.flash > 0.0 {
            let (w, h) = (self.w(), self.h());
            self.gfx.rect(0.0, 0.0, w, h, [1.0, 1.0, 1.0, self.flash.min(1.0) * 0.85]);
        }
        self.gfx.flush();
        if self.gfx.d2d_begin() {
            let full = matches!(self.panel, Panel::Guide | Panel::Entry(_) | Panel::Missions | Panel::Help);
            if !full {
                match self.screen {
                    Screen::Title => self.render_title(),
                    Screen::Map => self.render_map_ui(),
                    Screen::Dive => self.render_dive_ui(),
                }
            }
            if self.screen != Screen::Title {
                if matches!(self.panel, Panel::Missions | Panel::Help) {
                    let (w, h) = (self.w(), self.h());
                    self.gfx.fill_rect(0.0, 0.0, w, h, [0.0, 0.03, 0.06, 0.5]);
                }
                self.render_top();
                if !full {
                    self.render_mission_card();
                }
            }
            match self.panel {
                Panel::Guide => self.render_guide_ui(),
                Panel::Entry(i) => self.render_entry_ui(i),
                Panel::Missions => self.render_missions(),
                Panel::Help => self.render_help(),
                Panel::Intro(n) => self.render_intro(n),
                Panel::Done(m) => self.render_done(m),
                Panel::None => {}
            }
            self.render_photo();
            self.render_notices();
            self.render_tip();
            self.gfx.d2d_end();
        }
        self.gfx.present();
    }

    fn add_button(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, label: &str, act: Act, enabled: bool, hot: bool) {
        let hover = self.mouse.0 >= x0 && self.mouse.0 <= x1 && self.mouse.1 >= y0 && self.mouse.1 <= y1 && !self.touch;
        let bg = if !enabled {
            [0.10, 0.14, 0.18, 0.85]
        } else if hot {
            if hover {
                [0.98, 0.76, 0.38, 1.0]
            } else {
                [0.92, 0.62, 0.24, 1.0]
            }
        } else if hover {
            [0.16, 0.36, 0.44, 0.98]
        } else {
            [0.08, 0.24, 0.32, 0.96]
        };
        let r = ((y1 - y0) * 0.3).min(10.0);
        self.gfx.fill_round(x0, y0, x1, y1, r, bg);
        self.gfx.stroke_round(x0, y0, x1, y1, r, 1.0, if hot { [1.0, 0.9, 0.7, 0.8] } else { EDGE });
        let tc = if !enabled {
            [0.5, 0.56, 0.6, 1.0]
        } else if hot {
            [0.12, 0.08, 0.04, 1.0]
        } else {
            TEXT
        };
        let f = if y1 - y0 >= 40.0 { Font::BigCenter } else { Font::NormalCenter };
        self.gfx.text(label, x0 + 4.0, y0, x1 - x0 - 8.0, y1 - y0, f, tc);
        self.buttons.push(Button { x0, y0, x1, y1, act, enabled });
    }

    fn panel_box(&mut self, x0: f32, y0: f32, x1: f32, y1: f32) {
        self.gfx.fill_round(x0, y0, x1, y1, 12.0, BG);
        self.gfx.stroke_round(x0, y0, x1, y1, 12.0, 1.5, EDGE);
    }

    // ------------------------------------------------------------ 地図

    fn render_map_world(&mut self) {
        let (w, h) = (self.w(), self.h());
        let z = self.zoom;
        let sx = 2.0 * z / w;
        let sy = -2.0 * z / h;
        self.gfx.set_xform(sx, sy, -self.cam.0 * sx, -self.cam.1 * sy);
        // タイトル画面では大陸がゆっくり動き続ける
        let t = if self.screen == Screen::Title { 512.0 - (self.title_t * 0.12).sin() * 26.0 } else { self.prog.t };
        let g = &mut self.gfx;
        g.draw_textured(&self.ocean_tex, 0.0, 0.0, MAP_W, MAP_H, [1.0, 1.0, 1.0, 1.0], true);
        // 経緯線
        let lw = (1.2 / z).max(0.5);
        for k in 1..12 {
            let x = k as f32 * 30.0 * DEG;
            g.line((x, 0.0), (x, MAP_H), lw, [0.6, 0.85, 1.0, 0.07]);
        }
        for k in 1..6 {
            let y = k as f32 * 30.0 * DEG;
            g.line((0.0, y), (MAP_W, y), lw, [0.6, 0.85, 1.0, if k == 3 { 0.16 } else { 0.07 }]);
        }
        g.flush();
        for i in 0..self.plate_tex.len() {
            let q = self.earth.plate_quad(i, t);
            self.gfx.draw_textured_quad(&self.plate_tex[i], q, [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], [1.0, 1.0, 1.0, 1.0], true);
        }
        // 大噴火
        let erupting = (509.9..=511.6).contains(&t);
        if erupting {
            let (lon, lat) = self.earth.site_lonlat(S_KALKARINDJI, t);
            let (mx, my) = lonlat_to_map(lon, lat);
            let k = 1.0 - ((t - 510.75).abs() / 0.85).clamp(0.0, 1.0) * 0.5;
            let g = &mut self.gfx;
            g.radial(mx, my, 90.0 * k, 32, [1.0, 0.35, 0.1, 0.55], [1.0, 0.2, 0.05, 0.0]);
            for i in 0..18 {
                let a = i as f32 * 2.39996;
                let r = 8.0 + (i as f32 * 3.7) % 55.0 * k;
                let fl = (self.clock * 3.0 + i as f32).sin() * 0.5 + 0.5;
                g.circle(mx + a.cos() * r, my + a.sin() * r * 0.7, 4.0 + fl * 5.0, 10, [1.0, 0.45 + fl * 0.4, 0.1, 0.85]);
            }
            for i in 0..10 {
                let f = ((self.clock * 0.25 + i as f32 * 0.1) % 1.0) as f32;
                let px = mx + (i as f32 * 1.7).sin() * 20.0 + f * 120.0;
                let py = my - f * 60.0;
                g.circle(px, py, 10.0 + f * 40.0, 16, [0.25, 0.22, 0.22, 0.4 * (1.0 - f)]);
            }
        }
        // 雲（横に流れる）
        let ca = if self.screen == Screen::Title { 0.45 } else { (0.4 - (z - 0.5) * 0.3).clamp(0.08, 0.36) };
        let off = (self.clock * 6.0) % MAP_W;
        let u = off / MAP_W;
        let cw = [1.0, 1.0, 1.0, ca];
        self.gfx.draw_textured_quad(&self.cloud_tex, [(0.0, 0.0), (MAP_W - off, 0.0), (MAP_W - off, MAP_H), (0.0, MAP_H)], [(u, 0.0), (1.0, 0.0), (1.0, 1.0), (u, 1.0)], cw, true);
        self.gfx.draw_textured_quad(&self.cloud_tex, [(MAP_W - off, 0.0), (MAP_W, 0.0), (MAP_W, MAP_H), (MAP_W - off, MAP_H)], [(0.0, 0.0), (u, 0.0), (u, 1.0), (0.0, 1.0)], cw, true);
        if self.screen == Screen::Title {
            return;
        }
        // 観察地点
        let g = &mut self.gfx;
        let ms = 1.0 / z;
        for (i, s) in SITES.iter().enumerate() {
            let (lon, lat) = self.earth.site_lonlat(i, t);
            let (mx, my) = lonlat_to_map(lon, lat);
            let active = t <= s.t0 + 0.5 && t >= s.t1 - 0.5;
            if active {
                let p = (self.clock * 2.0 + i as f32).sin() * 0.5 + 0.5;
                g.ring(mx, my, (9.0 + p * 4.0) * ms, 2.0 * ms, 24, [0.5, 1.0, 0.9, 0.85]);
                g.circle(mx, my, 3.5 * ms, 12, [0.7, 1.0, 0.95, 1.0]);
            } else {
                g.circle(mx, my, 3.0 * ms, 10, [0.8, 0.85, 0.9, 0.35]);
            }
        }
        // カプセル
        if let Some(tg) = self.cap_target {
            g.line(self.cap, tg, 1.5 * ms, [1.0, 0.8, 0.5, 0.5]);
            g.ring(tg.0, tg.1, 7.0 * ms, 1.5 * ms, 16, [1.0, 0.8, 0.5, 0.7]);
        }
        let (cx, cy) = self.cap;
        let p = (self.clock * 3.0).sin() * 0.5 + 0.5;
        g.radial(cx, cy, 26.0 * ms, 24, [1.0, 0.8, 0.4, 0.35], [1.0, 0.7, 0.3, 0.0]);
        g.ring(cx, cy, (14.0 + p * 6.0) * ms, 1.5 * ms, 24, [1.0, 0.85, 0.5, 0.8 - p * 0.5]);
        g.circle(cx, cy - 1.0 * ms, 7.0 * ms, 16, [0.88, 0.9, 0.92, 1.0]);
        g.circle(cx + 1.5 * ms, cy - 2.0 * ms, 4.0 * ms, 12, [0.3, 0.7, 0.85, 1.0]);
        g.line((cx - 7.0 * ms, cy), (cx + 7.0 * ms, cy), 1.6 * ms, [0.98, 0.62, 0.2, 1.0]);
    }

    fn timeline_rect(&self) -> (f32, f32, f32, f32) {
        let (w, h) = (self.w(), self.h());
        let th = if self.compact() { 50.0 } else { 58.0 };
        let x0 = if self.compact() { 64.0 } else { 80.0 };
        (x0, h - th - 8.0, w - x0, h - 8.0)
    }

    fn loc_rect(&self) -> (f32, f32, f32, f32) {
        let (_, ty0, _, _) = self.timeline_rect();
        let (lw, lh) = if self.compact() { (300.0, 104.0) } else { (350.0, 118.0) };
        (10.0, ty0 - lh - 10.0, 10.0 + lw, ty0 - 10.0)
    }

    fn render_map_ui(&mut self) {
        let t = self.prog.t;
        let z = self.zoom;
        // 大陸の名前
        if z < 1.6 {
            for (i, d) in PLATES.iter().enumerate() {
                if !d.label {
                    continue;
                }
                let (lon, lat, _) = self.earth.pose(i, t);
                let (mx, my) = lonlat_to_map(lon, lat);
                let (sx, sy) = self.map_to_screen(mx, my);
                let s = d.name.get();
                let a = (1.6 - z).clamp(0.0, 1.0) * 0.85;
                self.gfx.text_shadow(s, sx - 120.0, sy - 12.0, 240.0, 24.0, Font::BigCenter, [1.0, 0.96, 0.86, a]);
            }
        }
        // 観察地点の名前
        let (mx, my) = self.mouse;
        for (i, s) in SITES.iter().enumerate() {
            let (lon, lat) = self.earth.site_lonlat(i, t);
            let (px, py) = lonlat_to_map(lon, lat);
            let (sx, sy) = self.map_to_screen(px, py);
            let active = t <= s.t0 + 0.5 && t >= s.t1 - 0.5;
            let hover = (mx - sx).abs() < 14.0 && (my - sy).abs() < 14.0;
            if active && z > 0.55 || hover {
                let label = s.name.get();
                let wd = self.gfx.measure(label, Font::Small) + 12.0;
                self.gfx.fill_round(sx + 10.0, sy - 10.0, sx + 10.0 + wd, sy + 8.0, 6.0, [0.02, 0.1, 0.14, 0.75]);
                self.gfx.text(label, sx + 16.0, sy - 9.0, wd, 16.0, Font::Small, if active { [0.7, 1.0, 0.95, 1.0] } else { DIM });
            }
            if hover {
                self.tip = Some(format!("{}（{}）\n{}", s.name.get(), s.now.get(), span_text(s.t0, s.t1)));
            }
        }
        self.render_loc_card();
        self.render_timeline();
        if let Some((i, left)) = self.banner.first().copied() {
            let m = &MILESTONES[i];
            let a = (left / 0.6).min(1.0).min((6.5 - left) / 0.4).clamp(0.0, 1.0);
            let w = self.w();
            let bw = (w * 0.6).clamp(420.0, 760.0);
            let x0 = (w - bw) * 0.5;
            let y0 = self.top_h() + 70.0;
            self.gfx.fill_round(x0, y0, x0 + bw, y0 + 86.0, 12.0, [0.02, 0.08, 0.12, 0.85 * a]);
            self.gfx.stroke_round(x0, y0, x0 + bw, y0 + 86.0, 12.0, 1.5, [1.0, 0.8, 0.4, 0.7 * a]);
            self.gfx.text(&ma_text(m.ma), x0 + 16.0, y0 + 8.0, bw - 32.0, 18.0, Font::Small, [1.0, 0.85, 0.5, a]);
            self.gfx.text(m.title.get(), x0 + 16.0, y0 + 24.0, bw - 32.0, 24.0, Font::Big, [1.0, 1.0, 1.0, a]);
            self.gfx.text(m.text.get(), x0 + 16.0, y0 + 50.0, bw - 32.0, 36.0, Font::Small, [0.85, 0.92, 0.92, a]);
        }
    }

    fn render_loc_card(&mut self) {
        let (x0, y0, x1, y1) = self.loc_rect();
        let (lon, lat) = map_to_lonlat(self.cap.0, self.cap.1);
        let t = self.prog.t;
        let spot = self.earth.sample(lon, lat, t);
        self.panel_box(x0, y0, x1, y1);
        let plate = spot.plate.map(|p| PLATES[p].name.get()).unwrap_or(t!("パンサラサ海（外洋）", "Panthalassic Ocean", "泛大洋（远洋）"));
        let where_s = if spot.plate.is_some() && spot.hab == Habitat::Deep { tf!("{}の沖", "Off {}", "{}外海", plate) } else { plate.to_string() };
        self.gfx.text(&where_s, x0 + 12.0, y0 + 8.0, x1 - x0 - 24.0, 20.0, Font::Big, TEXT);
        if let Some(p) = spot.plate {
            let ww = self.gfx.measure(&where_s, Font::Big);
            let now = tf!("のちの{}", "later {}", "后来的{}", PLATES[p].now.get());
            self.gfx.text(&now, x0 + 20.0 + ww, y0 + 11.0, (x1 - x0 - 32.0 - ww).max(10.0), 16.0, Font::Small, DIM);
        }
        let ns = if lat >= 0.0 { t!("北緯", "N", "北纬") } else { t!("南緯", "S", "南纬") };
        let lat_s = match lang::lang() {
            lang::Lang::En => format!("{:.0}°{}", lat.abs(), ns),
            _ => format!("{}{:.0}°", ns, lat.abs()),
        };
        let depth = match spot.hab {
            Habitat::Land => t!("陸", "Land", "陆地").to_string(),
            Habitat::Tidal => t!("干潟", "Tidal flat", "潮坪").to_string(),
            h => tf!("{} 水深{:.0}m", "{} · {:.0} m deep", "{} 水深{:.0}米", h.name(), spot.depth),
        };
        self.gfx.text(&format!("{}　{}", lat_s, depth), x0 + 12.0, y0 + 32.0, x1 - x0 - 24.0, 18.0, Font::Normal, DIM);
        // 近くの観察地点
        let mut near = None;
        for (i, s) in SITES.iter().enumerate() {
            if t <= s.t0 + 0.5 && t >= s.t1 - 0.5 && deg_dist(self.earth.site_lonlat(i, t), (lon, lat)) < 7.0 {
                near = Some(i);
            }
        }
        if let Some(i) = near {
            self.gfx.text(&tf!("近くに化石の産地：{}", "Fossil site nearby: {}", "附近的化石产地：{}", SITES[i].name.get()), x0 + 12.0, y0 + 52.0, x1 - x0 - 24.0, 18.0, Font::Small, [0.7, 1.0, 0.95, 1.0]);
        }
        let bh = if self.compact() { 34.0 } else { 36.0 };
        let by = y1 - bh - 10.0;
        let label = if spot.hab.is_water() && spot.hab != Habitat::Tidal { t!("潜る (Enter)", "Dive (Enter)", "下潜 (Enter)") } else { t!("降りる (Enter)", "Land (Enter)", "降落 (Enter)") };
        let can = self.travel.is_none();
        let mid = x0 + (x1 - x0) * 0.58;
        self.add_button(x0 + 10.0, by, mid - 5.0, by + bh, label, Act::Dive, can, true);
        self.add_button(mid + 5.0, by, x1 - 10.0, by + bh, t!("撮影 (Space)", "Photo (Space)", "拍摄 (Space)"), Act::Shoot, true, false);
    }

    fn render_timeline(&mut self) {
        let (x0, y0, x1, y1) = self.timeline_rect();
        let t = self.prog.t;
        self.gfx.fill_round(x0 - 56.0, y0 - 4.0, x1 + 56.0, y1 + 4.0, 12.0, BG);
        let bar_y0 = y0 + 22.0;
        let bar_y1 = y1 - 6.0;
        let tx = |tt: f32| x0 + (T_START - tt) / (T_START - T_END) * (x1 - x0);
        for s in STAGES {
            let (a, b) = (tx(s.t0), tx(s.t1));
            self.gfx.fill_rect(a, bar_y0, b - 1.0, bar_y1, c4(s.col, 0.9));
            let nm = s.name.get();
            let wd = self.gfx.measure(nm, Font::Small);
            if wd < b - a - 4.0 {
                self.gfx.text(nm, a + 2.0, bar_y0 + 2.0, b - a - 4.0, bar_y1 - bar_y0, Font::Small, [0.08, 0.14, 0.12, 1.0]);
            }
        }
        for m in MILESTONES {
            let x = tx(m.ma);
            self.gfx.fill_rect(x - 1.0, bar_y0 - 5.0, x + 1.0, bar_y0, [1.0, 0.85, 0.5, 0.8]);
        }
        // 宿題の時代
        let title = tf!("{}　{}", "{}   {}", "{}　{}", era_text(t), ma_text(t));
        self.gfx.text(&title, x0, y0 + 1.0, x1 - x0, 18.0, Font::Normal, TEXT);
        let fut = t!("オルドビス紀〜現代：準備中", "Ordovician to today: coming soon", "奥陶纪至今：准备中");
        let fw = self.gfx.measure(fut, Font::Small);
        self.gfx.text(fut, x1 - fw, y0 + 3.0, fw + 4.0, 16.0, Font::Small, [0.6, 0.66, 0.7, 1.0]);
        // 現在の時代
        let cx = tx(t);
        self.gfx.fill_rect(cx - 2.0, bar_y0 - 4.0, cx + 2.0, bar_y1 + 3.0, [1.0, 1.0, 1.0, 1.0]);
        self.gfx.fill_circle(cx, bar_y0 - 5.0, 5.0, ACCENT);
        // 行き先
        if let Some(tr) = &self.travel {
            let dx = tx(tr.to);
            self.gfx.stroke_circle(dx, bar_y0 - 5.0, 6.0, 2.0, ACCENT);
        }
        // マウスを乗せた時代
        let (mx, my) = self.mouse;
        if mx >= x0 && mx <= x1 && my >= bar_y0 - 8.0 && my <= bar_y1 + 4.0 {
            let tt = T_START - (mx - x0) / (x1 - x0) * (T_START - T_END);
            let tt = (tt * 2.0).round() / 2.0;
            self.tip = Some(tf!("{}\nクリックでこの時代へ", "{}\nClick to travel here", "{}\n点击前往这个时代", ma_text(tt)));
            self.gfx.fill_rect(mx - 1.0, bar_y0, mx + 1.0, bar_y1, [1.0, 1.0, 1.0, 0.5]);
        }
        // 年表のクリック（ボタンとして扱う）
        let n = 107;
        for i in 0..n {
            let f0 = i as f32 / n as f32;
            let f1 = (i + 1) as f32 / n as f32;
            let tt = T_START - (f0 + f1) * 0.5 * (T_START - T_END);
            let tt = (tt * 2.0).round() / 2.0;
            self.buttons.push(Button { x0: x0 + f0 * (x1 - x0), y0: bar_y0 - 8.0, x1: x0 + f1 * (x1 - x0), y1: bar_y1 + 4.0, act: Act::TimeAt(tt), enabled: self.travel.is_none() });
        }
        let bw = 44.0;
        self.add_button(x0 - 52.0, y0 + 8.0, x0 - 52.0 + bw, y1 - 4.0, "◀", Act::StepStage(-1), self.travel.is_none(), false);
        self.add_button(x1 + 8.0, y0 + 8.0, x1 + 8.0 + bw, y1 - 4.0, "▶", Act::StepStage(1), self.travel.is_none(), false);
    }

    // ------------------------------------------------------------ 上の帯・宿題のカード

    fn render_top(&mut self) {
        let w = self.w();
        let th = self.top_h();
        self.gfx.fill_rect(0.0, 0.0, w, th, [0.02, 0.08, 0.13, 0.88]);
        self.gfx.fill_rect(0.0, th - 1.5, w, th, [0.45, 0.85, 0.80, 0.4]);
        let t = self.prog.t;
        let left = if self.compact() { stage_at(t).name.get().to_string() } else { era_text(t) };
        self.gfx.text(&left, 12.0, 4.0, 360.0, 18.0, Font::Small, DIM);
        self.gfx.text(&ma_text(t), 12.0, 18.0, 360.0, 24.0, Font::Big, TEXT);
        // 右のボタン
        let bh = th - 12.0;
        let mut x = w - 8.0;
        let items: Vec<(&str, Act, f32)> = vec![
            (t!("タイトル", "Title", "标题"), Act::ToTitle, 76.0),
            ("⛶", Act::Fullscreen, 38.0),
            (lang::lang().native(), Act::Lang, 74.0),
            (t!("？ 操作", "? Help", "？ 操作"), Act::Help, 72.0),
        ];
        for (label, act, bw) in items {
            self.add_button(x - bw, 6.0, x, 6.0 + bh, label, act, true, false);
            x -= bw + 6.0;
        }
        let fresh = self.prog.fresh.iter().filter(|&&f| f).count();
        let gl = if fresh > 0 { tf!("図鑑 {}/{} ●", "Guide {}/{} ●", "图鉴 {}/{} ●", self.prog.found(), n_entries()) } else { tf!("図鑑 {}/{}", "Guide {}/{}", "图鉴 {}/{}", self.prog.found(), n_entries()) };
        let gw = if self.compact() { 110.0 } else { 130.0 };
        self.add_button(x - gw, 6.0, x, 6.0 + bh, &gl, Act::Guide, true, fresh > 0);
        x -= gw + 6.0;
        self.add_button(x - 84.0, 6.0, x, 6.0 + bh, t!("宿題 F1", "Tasks F1", "作业 F1"), Act::Missions, true, false);
    }

    fn mission_rect(&self) -> Option<(f32, f32, f32, f32)> {
        if self.screen == Screen::Title || self.panel != Panel::None {
            return None;
        }
        let w = self.w();
        let cw = if self.compact() { 260.0 } else { 300.0 };
        let y0 = self.top_h() + 8.0;
        if !self.mission_open || self.prog.all_clear() {
            return Some((w - cw - 10.0, y0, w - 10.0, y0 + 34.0));
        }
        let n = MISSIONS[self.prog.mission].targets.len() as f32;
        let rh = if self.compact() { 26.0 } else { 28.0 };
        Some((w - cw - 10.0, y0, w - 10.0, y0 + 40.0 + n * rh + 8.0))
    }

    fn render_mission_card(&mut self) {
        let Some((x0, y0, x1, y1)) = self.mission_rect() else { return };
        self.panel_box(x0, y0, x1, y1);
        if self.prog.all_clear() {
            self.gfx.text(t!("カンブリア紀の宿題：すべて合格！", "Cambrian tasks: all passed!", "寒武纪作业：全部合格！"), x0 + 12.0, y0 + 7.0, x1 - x0 - 24.0, 20.0, Font::Normal, GOLD);
            return;
        }
        let mi = self.prog.mission;
        let m = &MISSIONS[mi];
        let done_n = m.targets.iter().filter(|&&(t, n)| self.prog.stars[t.entry()] >= n).count();
        let head = tf!("宿題{} {}（{}/{}）", "Task {} {} ({}/{})", "作业{} {}（{}/{}）", mi + 1, m.title.get(), done_n, m.targets.len());
        self.gfx.text(&head, x0 + 12.0, y0 + 8.0, x1 - x0 - 50.0, 20.0, Font::Normal, ACCENT);
        let open = self.mission_open;
        self.add_button(x1 - 34.0, y0 + 5.0, x1 - 8.0, y0 + 29.0, if open { "－" } else { "＋" }, Act::MissionCard, true, false);
        if !open {
            return;
        }
        let rh = if self.compact() { 26.0 } else { 28.0 };
        let mut y = y0 + 38.0;
        let present: Vec<usize> = self.scene.as_ref().map(|s| s.present.clone()).unwrap_or_default();
        for &(tg, need) in m.targets {
            let have = self.prog.stars[tg.entry()];
            let ok = have >= need;
            let mark = if ok { "✔" } else { "○" };
            let here = matches!(tg, Target::Sp(i) if present.contains(&i)) && !ok;
            let col = if ok { GOOD } else if here { GOLD } else { TEXT };
            let need_s = if need > 1 { format!(" {}", stars_text(need)) } else { String::new() };
            let label = format!("{} {}{}", mark, tg.name(), need_s);
            self.gfx.text(&label, x0 + 12.0, y + 3.0, x1 - x0 - 90.0, 20.0, Font::Normal, col);
            if here {
                self.gfx.text(t!("ここにいる！", "Here!", "就在这里！"), x1 - 120.0, y + 4.0, 70.0, 18.0, Font::Small, GOLD);
            }
            if !ok {
                self.add_button(x1 - 52.0, y, x1 - 10.0, y + rh - 4.0, t!("行く", "Go", "前往"), Act::Goto(tg), true, false);
            }
            y += rh;
        }
    }

    // ------------------------------------------------------------ 観察の画面

    fn render_dive_ui(&mut self) {
        let (w, h) = (self.w(), self.h());
        let Some(sc) = &self.scene else { return };
        let lens = self.lens;
        let depth_s = sc.depth_text(sc.cap.y);
        let hab = sc.hab;
        let present = sc.present.clone();
        let have = present.iter().filter(|&&i| self.prog.stars[i] > 0).count();
        // 宿題の対象が画面の外にいれば、端に印を出す
        let v = self.dive_view();
        let mut marks: Vec<(f32, f32)> = Vec::new();
        if !self.prog.all_clear() {
            let m = &MISSIONS[self.prog.mission];
            for &(tg, need) in m.targets {
                if let Target::Sp(i) = tg {
                    if self.prog.stars[i] >= need {
                        continue;
                    }
                    let mut best: Option<(f32, f32, f32)> = None;
                    for c in &sc.creatures {
                        if c.sp != i {
                            continue;
                        }
                        let d = (c.x - sc.cap.x).powi(2) + (c.y - sc.cap.y).powi(2);
                        if best.map_or(true, |b| d < b.0) {
                            best = Some((d, c.x, c.y));
                        }
                    }
                    if let Some((_, x, y)) = best {
                        marks.push(((x - v.cx) * v.ppm + w * 0.5, (y - v.cy) * v.ppm + h * 0.5));
                    }
                }
            }
        }
        // 情報のカード
        let cw = if self.compact() { 250.0 } else { 290.0 };
        let ch = if self.compact() { 138.0 } else { 132.0 };
        let (x0, y1) = (10.0, h - 10.0);
        let y0 = y1 - ch;
        self.panel_box(x0, y0, x0 + cw, y1);
        self.gfx.text(&format!("{}　{}", hab.name(), depth_s), x0 + 12.0, y0 + 8.0, cw - 24.0, 20.0, Font::Normal, TEXT);
        self.gfx.text(&tf!("ここの生きもの：{}/{}種 収録済み", "Life here: {}/{} logged", "此处生物：已收录 {}/{} 种", have, present.len()), x0 + 12.0, y0 + 30.0, cw - 24.0, 18.0, Font::Small, DIM);
        let bh = 34.0;
        let by = y1 - bh - 10.0;
        let half = (cw - 30.0) / 3.0;
        let up = if hab.is_water() && hab != Habitat::Tidal { t!("浮上 Esc", "Surface Esc", "上浮 Esc") } else { t!("離陸 Esc", "Lift off Esc", "起飞 Esc") };
        self.add_button(x0 + 10.0, by, x0 + 10.0 + half, by + bh, up, Act::Surface, true, false);
        let light_s = if self.prog.light { t!("ライト ON", "Light ON", "灯 开") } else { t!("ライト L", "Light L", "灯 L") };
        let lt = self.prog.light;
        self.add_button(x0 + 15.0 + half, by, x0 + 15.0 + half * 2.0, by + bh, light_s, Act::Light, true, lt);
        self.add_button(x0 + 20.0 + half * 2.0, by, x0 + 20.0 + half * 3.0, by + bh, t!("加速 Shift", "Boost Shift", "加速 Shift"), Act::Boost, true, false);
        let by2 = by - bh - 6.0;
        if hab == Habitat::Deep || (hab == Habitat::Shelf && self.scene.as_ref().map_or(false, |s| s.depth > 120.0)) {
            let near_floor = self.scene.as_ref().map_or(false, |s| s.floor_at(s.cap.x) - s.cap.y < 3.0);
            let label = if near_floor { t!("海面へ F", "To surface F", "去海面 F") } else { t!("海底へ F", "To sea floor F", "去海底 F") };
            self.add_button(x0 + cw + 10.0, by, x0 + cw + 130.0, by + bh, label, Act::Jump(!near_floor), true, false);
        }
        self.gfx.text(&tf!("ズーム ×{:.1}", "Zoom ×{:.1}", "变焦 ×{:.1}", lens), x0 + 12.0, by2 + 8.0, 120.0, 18.0, Font::Normal, TEXT);
        self.add_button(x0 + cw - 92.0, by2, x0 + cw - 54.0, by2 + bh, "－", Act::Lens(-1), lens > 1.01, false);
        self.add_button(x0 + cw - 48.0, by2, x0 + cw - 10.0, by2 + bh, "＋", Act::Lens(1), lens < 7.99, false);
        // シャッター
        let r = if self.compact() { 38.0 } else { 42.0 };
        let (sx, sy) = (w - r - 22.0, h - r - 22.0);
        let hover = (self.mouse.0 - sx).powi(2) + (self.mouse.1 - sy).powi(2) < r * r;
        self.gfx.fill_circle(sx, sy, r + 6.0, [0.02, 0.08, 0.12, 0.8]);
        self.gfx.fill_circle(sx, sy, r, if hover { [1.0, 0.8, 0.45, 1.0] } else { [0.95, 0.66, 0.28, 1.0] });
        self.gfx.stroke_circle(sx, sy, r - 7.0, 2.0, [1.0, 1.0, 1.0, 0.85]);
        self.gfx.text(t!("撮影", "Shoot", "拍摄"), sx - r, sy - 12.0, r * 2.0, 24.0, Font::BigCenter, [0.1, 0.06, 0.02, 1.0]);
        self.buttons.push(Button { x0: sx - r, y0: sy - r, x1: sx + r, y1: sy + r, act: Act::Shoot, enabled: true });
        // ファインダー
        if lens >= 1.8 {
            let m = 40.0;
            let l = 46.0;
            let (fx0, fy0, fx1, fy1) = (m, self.top_h() + m * 0.5, w - m, h - m);
            let c = [1.0, 1.0, 1.0, 0.85];
            for (x, y, dx, dy) in [(fx0, fy0, 1.0, 1.0), (fx1, fy0, -1.0, 1.0), (fx0, fy1, 1.0, -1.0), (fx1, fy1, -1.0, -1.0)] {
                self.gfx.fill_rect(x.min(x + dx * l), y - 1.5, x.max(x + dx * l), y + 1.5, c);
                self.gfx.fill_rect(x - 1.5, y.min(y + dy * l), x + 1.5, y.max(y + dy * l), c);
            }
            let (cx, cy) = (w * 0.5, h * 0.5);
            self.gfx.fill_rect(cx - 12.0, cy - 0.8, cx + 12.0, cy + 0.8, [1.0, 1.0, 1.0, 0.6]);
            self.gfx.fill_rect(cx - 0.8, cy - 12.0, cx + 0.8, cy + 12.0, [1.0, 1.0, 1.0, 0.6]);
            self.gfx.fill_circle(fx0 + 22.0, fy0 + 22.0, 6.0, [1.0, 0.3, 0.25, 0.5 + 0.5 * (self.clock * 3.0).sin().abs()]);
        }
        // 宿題の対象の印
        let top = self.top_h();
        for (mx, my) in marks {
            let inside = mx > 30.0 && mx < w - 30.0 && my > top + 30.0 && my < h - 30.0;
            if inside {
                self.gfx.stroke_circle(mx, my, 22.0 + (self.clock * 4.0).sin() * 3.0, 2.0, [1.0, 0.85, 0.35, 0.7]);
            } else {
                let ex = mx.clamp(26.0, w - 26.0);
                let ey = my.clamp(top + 26.0, h - 26.0);
                self.gfx.fill_circle(ex, ey, 14.0, [1.0, 0.82, 0.3, 0.9]);
                self.gfx.text("!", ex - 10.0, ey - 11.0, 20.0, 22.0, Font::BigCenter, [0.2, 0.1, 0.0, 1.0]);
            }
        }
    }

    // ------------------------------------------------------------ タイトル

    fn render_title(&mut self) {
        let (w, h) = (self.w(), self.h());
        self.gfx.fill_rect(0.0, 0.0, w, h, [0.0, 0.03, 0.06, 0.35]);
        let cw = if self.compact() { 420.0 } else { 520.0 };
        let x0 = (w - cw) * 0.5;
        let y0 = h * 0.16;
        self.gfx.fill_round(x0, y0, x0 + cw, y0 + 150.0, 18.0, [0.02, 0.08, 0.13, 0.72]);
        self.gfx.stroke_round(x0, y0, x0 + cw, y0 + 150.0, 18.0, 1.5, EDGE);
        self.gfx.text(t!("地球史図鑑", "YYEarth", "地球史图鉴"), x0, y0 + 14.0, cw, 56.0, Font::Title, [0.80, 1.0, 0.96, 1.0]);
        self.gfx.text(t!("時をめぐる観察カプセル", "A Field Guide Through Deep Time", "时光观测舱"), x0, y0 + 72.0, cw, 26.0, Font::BigCenter, TEXT);
        self.gfx.text(t!("第1巻　古生代カンブリア紀", "Volume 1: The Cambrian Period", "第1卷　古生代寒武纪"), x0, y0 + 104.0, cw, 24.0, Font::NormalCenter, ACCENT);
        let bw = if self.compact() { 300.0 } else { 340.0 };
        let bh = if self.compact() { 40.0 } else { 44.0 };
        let bx = (w - bw) * 0.5;
        let mut by = y0 + 172.0;
        if self.have_save {
            self.add_button(bx, by, bx + bw, by + bh, t!("つづきから", "Continue", "继续"), Act::Continue, true, true);
            by += bh + 10.0;
        }
        let new_label = if self.confirm_new > 0.0 { t!("もう一度押すと、はじめから（記録は上書き）", "Press again to restart (overwrites save)", "再按一次重新开始（覆盖存档）") } else { t!("はじめから", "New journey", "从头开始") };
        let hot = !self.have_save;
        self.add_button(bx, by, bx + bw, by + bh, new_label, Act::NewGame, true, hot);
        by += bh + 10.0;
        let half = (bw - 10.0) / 2.0;
        self.add_button(bx, by, bx + half, by + 36.0, lang::lang().native(), Act::Lang, true, false);
        self.add_button(bx + half + 10.0, by, bx + bw, by + 36.0, t!("全画面", "Fullscreen", "全屏"), Act::Fullscreen, true, false);
        by += 46.0;
        self.add_button(bx, by, bx + bw, by + 36.0, t!("記録ファイルを読み込む", "Load a save file", "读取存档文件"), Act::Import, true, false);
        let foot = t!(
            "西暦2650年。時間旅行学校の宿題で、5億年前の地球へ。",
            "The year 2650. A time-travel school assignment takes you 500 million years into the past.",
            "公元2650年。为了完成时间旅行学校的作业，你将前往5亿年前的地球。"
        );
        self.gfx.text_shadow(foot, 20.0, h - 34.0, w - 40.0, 24.0, Font::NormalCenter, [0.9, 0.96, 0.96, 0.9]);
    }

    // ------------------------------------------------------------ 図鑑

    fn guide_layout(&self) -> (f32, f32, f32, f32, usize, f32, f32) {
        let (w, h) = (self.w(), self.h());
        let x0 = 20.0;
        let y0 = self.top_h() + 56.0;
        let x1 = w - 20.0;
        let y1 = h - 16.0;
        let cell_w = if self.compact() { 112.0 } else { 132.0 };
        let cols = (((x1 - x0) + 10.0) / (cell_w + 10.0)).floor().max(2.0) as usize;
        let cw = ((x1 - x0) - (cols - 1) as f32 * 10.0) / cols as f32;
        let ch = cw * 0.86;
        (x0, y0, x1, y1, cols, cw, ch)
    }

    fn render_guide_gl(&mut self) {
        let (w, h) = (self.w(), self.h());
        self.gfx.rect(0.0, 0.0, w, h, [0.02, 0.07, 0.11, 0.97]);
        let (x0, y0, _x1, y1, cols, cw, ch) = self.guide_layout();
        let rows = (n_entries() + cols - 1) / cols;
        self.guide_max = ((rows as f32) * (ch + 10.0) - (y1 - y0)).max(0.0);
        self.guide_scroll = self.guide_scroll.clamp(0.0, self.guide_max);
        for e in 0..n_entries() {
            let (r, c) = (e / cols, e % cols);
            let cx0 = x0 + c as f32 * (cw + 10.0);
            let cy0 = y0 + r as f32 * (ch + 10.0) - self.guide_scroll;
            if cy0 + ch < y0 - 4.0 || cy0 > y1 {
                continue;
            }
            let found = self.prog.stars[e] > 0;
            let hover = self.mouse.0 >= cx0 && self.mouse.0 <= cx0 + cw && self.mouse.1 >= cy0 && self.mouse.1 <= cy0 + ch && !self.touch;
            let bg = if found { [0.10, 0.26, 0.34, 1.0] } else { [0.06, 0.12, 0.17, 1.0] };
            let bg = if hover { mix4(bg, [0.3, 0.5, 0.55, 1.0], 0.3) } else { bg };
            self.gfx.rect(cx0, cy0, cx0 + cw, cy0 + ch, bg);
            self.gfx.rect_outline(cx0, cy0, cx0 + cw, cy0 + ch, 1.0, if found { [0.45, 0.85, 0.80, 0.7] } else { [0.3, 0.4, 0.45, 0.6] });
            let a = if found { 1.0 } else { 0.16 };
            self.draw_entry_art(e, cx0 + 6.0, cy0 + 6.0, cx0 + cw - 6.0, cy0 + ch - 28.0, a);
            self.gfx.flush();
        }
        // 上下の余白を塗って、スクロールした項目を隠す
        self.gfx.rect(0.0, 0.0, w, y0 - 2.0, [0.02, 0.07, 0.11, 1.0]);
        self.gfx.rect(0.0, y1 + 2.0, w, h, [0.02, 0.07, 0.11, 1.0]);
    }

    /// 図鑑の絵（生きもの・現象）を枠の中に描く
    fn draw_entry_art(&mut self, e: usize, x0: f32, y0: f32, x1: f32, y1: f32, a: f32) {
        let (bw, bh) = (x1 - x0, y1 - y0);
        let ns = n_species();
        if e < ns {
            let sp = &SPECIES[e];
            let (ew, eh) = art::extent(e);
            let sq = if sp.art.dorsal() { 0.45 } else { 1.0 };
            let len = (bw * 0.82 / ew).min(bh * 0.8 / (eh * sq).max(0.3));
            let lift = art::base_lift(e);
            let ground = matches!(sp.beh, Beh::Sessile | Beh::Burrow);
            let y = if ground { y1 - 4.0 - lift * len } else { (y0 + y1) * 0.5 };
            let x = (x0 + x1) * 0.5 - if matches!(sp.art, Art::Radiodont(_)) { len * 0.08 } else { 0.0 };
            let state = if sp.beh == Beh::Burrow { 1.0 } else { 0.0 };
            art::draw(&mut self.gfx, e, x, y, len, 1.0, 0.0, self.clock, a, state, 7);
        } else {
            let p = e - ns;
            match p {
                P_GONDWANA | P_LAURENTIA => {
                    let pi = if p == P_GONDWANA { GONDWANA } else { LAURENTIA };
                    let pl = &self.earth.plates[pi];
                    let asp = pl.w as f32 / pl.h as f32;
                    let (dw, dh) = if bw / bh > asp { (bh * asp, bh) } else { (bw, bw / asp) };
                    let (cx, cy) = ((x0 + x1) * 0.5, (y0 + y1) * 0.5);
                    self.gfx.rect(x0, y0, x1, y1, [0.06, 0.18, 0.36, a]);
                    self.gfx.draw_textured(&self.plate_tex[pi], cx - dw * 0.5, cy - dh * 0.5, cx + dw * 0.5, cy + dh * 0.5, [1.0, 1.0, 1.0, a], true);
                }
                P_ERUPTION => {
                    self.gfx.rect_vgrad(x0, y0, x1, y1, [0.22, 0.12, 0.12, a], [0.7, 0.32, 0.14, a]);
                    let gy = y0 + bh * 0.7;
                    self.gfx.rect(x0, gy, x1, y1, [0.12, 0.10, 0.10, a]);
                    for i in 0..5 {
                        let fx = x0 + bw * (0.15 + i as f32 * 0.18);
                        let fl = (self.clock * 3.0 + i as f32).sin() * 0.5 + 0.5;
                        self.gfx.radial(fx, gy, bh * 0.25, 16, [1.0, 0.5, 0.1, 0.6 * a], [1.0, 0.3, 0.1, 0.0]);
                        self.gfx.tri((fx - bw * 0.03, gy), (fx + bw * 0.03, gy), (fx, gy - bh * (0.15 + fl * 0.2)), [1.0, 0.7, 0.2, a]);
                    }
                    self.gfx.rect(x0, gy - 2.0, x1, gy + 3.0, [1.0, 0.45, 0.1, a]);
                }
                _ => {
                    self.gfx.rect_vgrad(x0, y0, x1, y1, [0.36, 0.58, 0.82, a], [0.82, 0.86, 0.88, a]);
                    let gy = y0 + bh * 0.62;
                    let n = 8;
                    for i in 0..n {
                        let fa = i as f32 / n as f32;
                        let fb = (i + 1) as f32 / n as f32;
                        let ha = (fa * 9.0).sin() * 0.1 + 0.12;
                        let hb = (fb * 9.0).sin() * 0.1 + 0.12;
                        self.gfx.tri((x0 + bw * fa, gy), (x0 + bw * fb, gy), (x0 + bw * fb, gy - bh * hb), [0.62, 0.44, 0.34, a]);
                        self.gfx.tri((x0 + bw * fa, gy), (x0 + bw * fb, gy - bh * hb), (x0 + bw * fa, gy - bh * ha), [0.62, 0.44, 0.34, a]);
                    }
                    self.gfx.rect_vgrad(x0, gy, x1, y1, [0.70, 0.50, 0.36, a], [0.50, 0.34, 0.26, a]);
                    for i in 0..6 {
                        let rx = x0 + bw * (0.1 + i as f32 * 0.16);
                        self.gfx.circle(rx, gy + bh * (0.12 + (i % 3) as f32 * 0.06), bh * 0.04, 10, [0.44, 0.32, 0.28, a]);
                    }
                }
            }
        }
    }

    fn render_guide_ui(&mut self) {
        let w = self.w();
        let (x0, y0, _x1, y1, cols, cw, ch) = self.guide_layout();
        let title = tf!("カンブリア紀の図鑑　{}/{}", "Cambrian field guide  {}/{}", "寒武纪图鉴　{}/{}", self.prog.found(), n_entries());
        self.gfx.text(&title, 24.0, self.top_h() + 12.0, w - 200.0, 30.0, Font::Big, TEXT);
        self.add_button(w - 120.0, self.top_h() + 10.0, w - 20.0, self.top_h() + 44.0, t!("閉じる", "Close", "关闭"), Act::Close, true, false);
        for e in 0..n_entries() {
            let (r, c) = (e / cols, e % cols);
            let cx0 = x0 + c as f32 * (cw + 10.0);
            let cy0 = y0 + r as f32 * (ch + 10.0) - self.guide_scroll;
            if cy0 + ch < y0 || cy0 > y1 - 10.0 {
                continue;
            }
            let st = self.prog.stars[e];
            let name = if st > 0 { entry_name(e).to_string() } else { "？？？".to_string() };
            let vis_y0 = cy0.max(y0);
            let vis_y1 = (cy0 + ch).min(y1);
            if cy0 + ch - 26.0 >= y0 && cy0 + ch <= y1 + 1.0 {
                self.gfx.text(&name, cx0 + 4.0, cy0 + ch - 26.0, cw - 8.0, 16.0, Font::NormalCenter, if st > 0 { TEXT } else { DIM });
                if st > 0 {
                    self.gfx.text(&stars_text(st), cx0 + 4.0, cy0 + ch - 12.0, cw - 8.0, 12.0, Font::NormalCenter, GOLD);
                }
            }
            if self.prog.fresh[e] && cy0 >= y0 {
                self.gfx.fill_round(cx0 + 4.0, cy0 + 4.0, cx0 + 46.0, cy0 + 20.0, 6.0, ACCENT);
                self.gfx.text("NEW", cx0 + 4.0, cy0 + 4.0, 42.0, 16.0, Font::NormalCenter, [0.1, 0.05, 0.0, 1.0]);
            }
            if e >= n_species() && cy0 >= y0 {
                self.gfx.text(t!("現象", "Event", "现象"), cx0 + cw - 44.0, cy0 + 4.0, 40.0, 14.0, Font::Small, DIM);
            }
            if vis_y1 > vis_y0 + 4.0 {
                self.buttons.push(Button { x0: cx0, y0: vis_y0, x1: cx0 + cw, y1: vis_y1, act: Act::Entry(e), enabled: true });
            }
        }
    }

    fn entry_layout(&self) -> (f32, f32, f32, f32, f32) {
        let (w, h) = (self.w(), self.h());
        let x0 = 20.0;
        let y0 = self.top_h() + 56.0;
        let x1 = w - 20.0;
        let y1 = h - 16.0;
        let split = x0 + (x1 - x0) * 0.46;
        (x0, y0, x1, y1, split)
    }

    fn render_entry_gl(&mut self, e: usize) {
        let (w, h) = (self.w(), self.h());
        self.gfx.rect(0.0, 0.0, w, h, [0.02, 0.07, 0.11, 0.97]);
        let (x0, y0, _x1, y1, split) = self.entry_layout();
        let ns = n_species();
        let (ax0, ay0, ax1, ay1) = (x0, y0, split - 10.0, y1 - 50.0);
        // 背景（すみかの色）
        if e < ns {
            let sp = &SPECIES[e];
            if sp.hab & (SHALLOW | SHELF | DEEP) != 0 {
                let deep = sp.hab & (SHALLOW | TIDAL) == 0;
                let top = if deep { [0.06, 0.24, 0.36, 1.0] } else { [0.22, 0.56, 0.66, 1.0] };
                self.gfx.rect_vgrad(ax0, ay0, ax1, ay1, top, [0.03, 0.10, 0.18, 1.0]);
            } else {
                self.gfx.rect_vgrad(ax0, ay0, ax1, ay1, [0.36, 0.58, 0.82, 1.0], [0.80, 0.84, 0.86, 1.0]);
            }
            let ground = matches!(sp.beh, Beh::Sessile | Beh::Burrow | Beh::Crawl | Beh::FlatCrawl);
            if ground {
                let gy = ay1 - (ay1 - ay0) * 0.22;
                self.gfx.rect_vgrad(ax0, gy, ax1, ay1, [0.56, 0.52, 0.44, 1.0], [0.30, 0.28, 0.26, 1.0]);
            }
        }
        let found = self.prog.stars[e] > 0;
        let pad = 20.0;
        let (bx0, by0, bx1, by1) = if e < ns && matches!(SPECIES[e].beh, Beh::Sessile | Beh::Burrow | Beh::Crawl | Beh::FlatCrawl) {
            let gy = ay1 - (ay1 - ay0) * 0.22;
            (ax0 + pad, ay0 + pad, ax1 - pad, if matches!(SPECIES[e].beh, Beh::Crawl | Beh::FlatCrawl) { gy + (ay1 - gy) * 0.7 } else { gy + (ay1 - gy) * 0.6 })
        } else {
            (ax0 + pad, ay0 + pad, ax1 - pad, ay1 - pad)
        };
        self.draw_entry_art(e, bx0, by0, bx1, by1, if found { 1.0 } else { 0.14 });
        self.gfx.rect_outline(ax0, ay0, ax1, ay1, 1.5, [0.45, 0.85, 0.80, 0.6]);
    }

    fn render_entry_ui(&mut self, e: usize) {
        let w = self.w();
        let (_x0, y0, x1, y1, split) = self.entry_layout();
        let ns = n_species();
        let st = self.prog.stars[e];
        let found = st > 0;
        let top = self.top_h();
        self.gfx.text(&tf!("図鑑 No.{:02}", "Guide No.{:02}", "图鉴 No.{:02}", e + 1), 24.0, top + 14.0, 300.0, 24.0, Font::Big, DIM);
        self.add_button(w - 120.0, top + 10.0, w - 20.0, top + 44.0, t!("一覧へ", "Back", "返回"), Act::Close, true, false);
        let prev = (e + n_entries() - 1) % n_entries();
        let next = (e + 1) % n_entries();
        self.add_button(w - 230.0, top + 10.0, w - 180.0, top + 44.0, "◀", Act::Entry(prev), true, false);
        self.add_button(w - 174.0, top + 10.0, w - 124.0, top + 44.0, "▶", Act::Entry(next), true, false);
        let (tx, tw) = (split + 14.0, x1 - split - 14.0);
        let mut y = y0;
        if e < ns {
            let sp = &SPECIES[e];
            let name = if found { sp.name.get() } else { "？？？" };
            self.gfx.text(name, tx, y, tw, 34.0, Font::TitleMid, TEXT);
            y += 40.0;
            if found {
                self.gfx.text(&format!("{}　/　{}", sp.sci, sp.group.get()), tx, y, tw, 20.0, Font::Normal, DIM);
                y += 26.0;
                self.gfx.text(&tf!("写真 {}", "Photo {}", "照片 {}", stars_text(st)), tx, y, tw, 22.0, Font::Big, GOLD);
                y += 30.0;
            }
            let rows: Vec<(&str, String)> = vec![
                (t!("時代", "When", "时代"), span_text(sp.t0, sp.t1)),
                (t!("すみか", "Habitat", "栖息地"), hab_list(sp.hab)),
                (t!("化石の産地", "Fossils", "化石产地"), place_text(sp)),
                (t!("大きさ", "Size", "大小"), size_text(sp.len_cm)),
            ];
            for (k, v) in rows {
                if !found && k == t!("大きさ", "Size", "大小") {
                    continue;
                }
                self.gfx.text(k, tx, y, 96.0, 20.0, Font::Normal, ACCENT);
                let hh = self.gfx.measure_h(&v, Font::Normal, tw - 100.0).max(20.0);
                self.gfx.text(&v, tx + 100.0, y, tw - 100.0, hh, Font::Normal, TEXT);
                y += hh + 6.0;
            }
            y += 8.0;
            if found {
                let d = sp.desc.get();
                let hh = self.gfx.measure_h(d, Font::Normal, tw);
                self.gfx.text(d, tx, y, tw, hh, Font::Normal, TEXT);
                y += hh + 12.0;
                let f = format!("💡 {}", sp.fact.get());
                let hh = self.gfx.measure_h(&f, Font::Normal, tw);
                if y + hh < y1 - 50.0 {
                    self.gfx.text(&f, tx, y, tw, hh, Font::Normal, [0.75, 0.95, 0.9, 1.0]);
                }
            } else {
                self.gfx.text(t!("まだ撮影していない。上の時代と場所で探してみよう。", "Not photographed yet. Look for it at the time and place above.", "还没有拍到。按上面的时代和地点去找找吧。"), tx, y, tw, 40.0, Font::Normal, DIM);
                y += 48.0;
                self.add_button(tx, y, tx + 220.0, y + 38.0, t!("探しに行く", "Go look for it", "去寻找"), Act::Goto(Target::Sp(e)), true, true);
            }
        } else {
            let p = e - ns;
            let ph = &PHENOMS[p];
            self.gfx.text(if found { ph.name.get() } else { "？？？" }, tx, y, tw, 34.0, Font::TitleMid, TEXT);
            y += 40.0;
            if found {
                self.gfx.text(&tf!("写真 {}", "Photo {}", "照片 {}", stars_text(st)), tx, y, tw, 22.0, Font::Big, GOLD);
                y += 30.0;
                let d = ph.desc.get();
                let hh = self.gfx.measure_h(d, Font::Normal, tw);
                self.gfx.text(d, tx, y, tw, hh, Font::Normal, TEXT);
            } else {
                self.gfx.text(ph.hint.get(), tx, y, tw, 40.0, Font::Normal, DIM);
                y += 48.0;
                self.add_button(tx, y, tx + 220.0, y + 38.0, t!("見に行く", "Go and see", "去看看"), Act::Goto(Target::Ph(p)), true, true);
            }
        }
    }

    // ------------------------------------------------------------ 宿題・操作・先生のことば

    fn render_missions(&mut self) {
        let (w, h) = (self.w(), self.h());
        let pw = (w - 40.0).min(820.0);
        let x0 = (w - pw) * 0.5;
        let y0 = self.top_h() + 16.0;
        let y1 = h - 16.0;
        self.panel_box(x0, y0, x0 + pw, y1);
        self.gfx.text(t!("学校の宿題：カンブリア紀の図鑑づくり", "School assignment: a Cambrian field guide", "学校作业：制作寒武纪图鉴"), x0 + 20.0, y0 + 14.0, pw - 160.0, 26.0, Font::Big, TEXT);
        self.add_button(x0 + pw - 120.0, y0 + 12.0, x0 + pw - 16.0, y0 + 44.0, t!("閉じる", "Close", "关闭"), Act::Close, true, false);
        let mut y = y0 + 56.0;
        let cur = self.prog.mission;
        if cur < MISSIONS.len() {
            let m = &MISSIONS[cur];
            self.gfx.text(&tf!("いまの宿題　{}. {}", "Current task  {}. {}", "当前作业　{}. {}", cur + 1, m.title.get()), x0 + 20.0, y, pw - 40.0, 22.0, Font::Big, ACCENT);
            y += 28.0;
            let b = format!("{}{}{}", t!("トキ先生", "Ms. Toki", "时老师"), t!("：", ": ", "："), m.brief.get());
            let hh = self.gfx.measure_h(&b, Font::Normal, pw - 40.0);
            self.gfx.text(&b, x0 + 20.0, y, pw - 40.0, hh, Font::Normal, TEXT);
            y += hh + 10.0;
            for &(tg, need) in m.targets {
                let have = self.prog.stars[tg.entry()];
                let ok = have >= need;
                let s = format!("{} {}　{}", if ok { "✔" } else { "○" }, tg.name(), if need > 1 { tf!("（{}以上）", "({} or better)", "（{}以上）", stars_text(need)) } else { String::new() });
                self.gfx.text(&s, x0 + 32.0, y + 4.0, pw - 200.0, 20.0, Font::Normal, if ok { GOOD } else { TEXT });
                if !ok {
                    self.add_button(x0 + pw - 140.0, y, x0 + pw - 24.0, y + 28.0, t!("行く", "Go", "前往"), Act::Goto(tg), true, false);
                }
                y += 32.0;
            }
            y += 10.0;
        } else {
            self.gfx.text(t!("すべての宿題に合格！ 次はオルドビス紀（準備中）。", "All tasks passed! Next: the Ordovician (coming soon).", "全部作业合格！下一站：奥陶纪（准备中）。"), x0 + 20.0, y, pw - 40.0, 22.0, Font::Big, GOLD);
            y += 34.0;
        }
        self.gfx.text(t!("宿題の一覧", "All tasks", "作业一览"), x0 + 20.0, y, pw - 40.0, 20.0, Font::Normal, DIM);
        y += 24.0;
        let col2 = self.compact() || pw < 700.0;
        for (i, m) in MISSIONS.iter().enumerate() {
            let (cx, cy) = if col2 { (x0 + 20.0 + (i % 2) as f32 * (pw - 40.0) * 0.5, y + (i / 2) as f32 * 22.0) } else { (x0 + 20.0 + (i % 2) as f32 * (pw - 40.0) * 0.5, y + (i / 2) as f32 * 24.0) };
            let (mark, c) = if i < cur {
                ("✔", GOOD)
            } else if i == cur {
                ("▶", ACCENT)
            } else {
                ("・", DIM)
            };
            if cy + 20.0 < y1 {
                self.gfx.text(&format!("{} {}. {}", mark, i + 1, m.title.get()), cx, cy, (pw - 40.0) * 0.5 - 10.0, 20.0, Font::Normal, c);
            }
        }
    }

    fn render_help(&mut self) {
        let (w, h) = (self.w(), self.h());
        let pw = (w - 40.0).min(760.0);
        let x0 = (w - pw) * 0.5;
        let y0 = self.top_h() + 16.0;
        let y1 = h - 16.0;
        self.panel_box(x0, y0, x0 + pw, y1);
        self.gfx.text(t!("カプセルの操作", "Capsule controls", "观测舱操作"), x0 + 20.0, y0 + 14.0, pw - 160.0, 26.0, Font::Big, TEXT);
        self.add_button(x0 + pw - 120.0, y0 + 12.0, x0 + pw - 16.0, y0 + 44.0, t!("閉じる", "Close", "关闭"), Act::Close, true, false);
        let lines: Vec<(&str, &str)> = if self.touch {
            vec![
                (t!("地図", "Map", "地图"), t!("タップした場所へ飛ぶ・なぞって地図を動かす・2本指で拡大縮小", "Tap to fly there, drag to pan, pinch to zoom", "点击飞往该处·拖动移动地图·双指缩放")),
                (t!("時代", "Time", "时代"), t!("下の年表をタップ、または ◀ ▶ で時代を移る（時間旅行）", "Tap the timeline or ◀ ▶ to travel in time", "点击下方年表或 ◀ ▶ 穿越时代")),
                (t!("潜る", "Dive", "下潜"), t!("「潜る／降りる」で、その場所の海や陸を観察する", "Use Dive/Land to observe the sea or land there", "用“下潜／降落”观察该处的海洋或陆地")),
                (t!("観察", "Observe", "观察"), t!("触れた方へカプセルが進む。＋－でズーム、撮影ボタンで撮る", "The capsule moves where you touch. ＋－ to zoom, shutter to shoot", "观测舱朝触碰方向前进。＋－变焦，按快门拍摄")),
            ]
        } else {
            vec![
                (t!("地図", "Map", "地图"), t!("クリックした場所へ飛ぶ／WASD で移動・ドラッグや矢印キーで地図を動かす・ホイールで拡大縮小", "Click to fly there / WASD to move. Drag or arrow keys to pan, wheel to zoom", "点击飞往该处／WASD移动·拖动或方向键移动地图·滚轮缩放")),
                (t!("時代", "Time", "时代"), t!("下の年表をクリック、または ,（読点）と .（句点）キーで期を移る", "Click the timeline, or use , and . to change stage", "点击下方年表，或用 , 和 . 键切换期")),
                (t!("潜る", "Dive", "下潜"), t!("Enter：その場所の海に潜る（陸・干潟なら降りる）", "Enter: dive into the sea here (or land)", "Enter：潜入该处的海洋（陆地·潮坪则降落）")),
                (t!("観察", "Observe", "观察"), t!("WASD・矢印・マウスを押した方へ進む／Shift で加速／ホイール・Q/E でズーム／L でライト", "WASD, arrows or hold the mouse to move / Shift to boost / wheel or Q,E to zoom / L for light", "WASD·方向键·按住鼠标前进／Shift加速／滚轮·Q/E变焦／L开灯")),
                (t!("撮影", "Photo", "拍摄"), t!("Space（または右クリック）：大きく・真ん中に・明るく写すほど星が増える", "Space (or right-click): bigger, centred and brighter shots earn more stars", "Space（或右键）：拍得越大、越居中、越明亮，星越多")),
                (t!("画面", "Screens", "画面"), t!("F1 宿題　F2 図鑑　F3 この説明　Esc 浮上・閉じる　Ctrl＋＋/－ 文字の大きさ　F11 全画面", "F1 tasks  F2 guide  F3 help  Esc surface/close  Ctrl +/- text size  F11 fullscreen", "F1 作业　F2 图鉴　F3 说明　Esc 上浮·关闭　Ctrl＋＋/－ 字号　F11 全屏")),
            ]
        };
        let mut y = y0 + 58.0;
        for (k, v) in lines {
            self.gfx.text(k, x0 + 20.0, y, 90.0, 20.0, Font::Big, ACCENT);
            let hh = self.gfx.measure_h(v, Font::Normal, pw - 140.0).max(22.0);
            self.gfx.text(v, x0 + 118.0, y + 2.0, pw - 140.0, hh, Font::Normal, TEXT);
            y += hh + 12.0;
        }
        y += 6.0;
        let tips = t!(
            "ヒント：宿題のカードの「行く」で、その生きものがいた時代と場所へカプセルが向かう。観察中、宿題の生きものは ! の印で示される。逃げる生きものには、ゆっくり近づこう。",
            "Tip: 'Go' on the task card sends the capsule to the right time and place. While observing, task creatures are marked with '!'. Approach shy animals slowly.",
            "提示：点击作业卡上的“前往”，观测舱会前往该生物所在的时代和地点。观察时，作业中的生物会用“!”标出。对胆小的生物要慢慢靠近。"
        );
        let hh = self.gfx.measure_h(tips, Font::Normal, pw - 40.0);
        self.gfx.text(tips, x0 + 20.0, y, pw - 40.0, hh, Font::Normal, [0.75, 0.95, 0.9, 1.0]);
        y += hh + 16.0;
        self.gfx.text(platform::storage_note(), x0 + 20.0, y, pw - 40.0, 40.0, Font::Small, DIM);
        y += 44.0;
        if y + 36.0 < y1 {
            self.add_button(x0 + 20.0, y, x0 + 260.0, y + 34.0, t!("記録をファイルに書き出す", "Export save to a file", "导出存档文件"), Act::Export, self.have_save, false);
            self.add_button(x0 + 270.0, y, x0 + 510.0, y + 34.0, t!("記録ファイルを読み込む", "Load a save file", "读取存档文件"), Act::Import, true, false);
        }
    }

    fn teacher(&mut self, x: f32, y: f32, r: f32) {
        // トキ先生（未来の先生）
        self.gfx.fill_circle(x, y, r + 4.0, [0.45, 0.85, 0.80, 0.6]);
        self.gfx.fill_circle(x, y, r, [0.10, 0.22, 0.32, 1.0]);
        self.gfx.fill_circle(x, y + r * 0.12, r * 0.62, [0.98, 0.86, 0.74, 1.0]);
        self.gfx.fill_round(x - r * 0.7, y - r * 0.62, x + r * 0.7, y - r * 0.12, r * 0.3, [0.36, 0.30, 0.56, 1.0]);
        self.gfx.fill_circle(x - r * 0.22, y + r * 0.1, r * 0.08, [0.1, 0.1, 0.15, 1.0]);
        self.gfx.fill_circle(x + r * 0.22, y + r * 0.1, r * 0.08, [0.1, 0.1, 0.15, 1.0]);
        self.gfx.stroke_round(x - r * 0.42, y - r * 0.02, x - r * 0.04, y + r * 0.22, 4.0, 2.0, [0.3, 0.9, 1.0, 0.9]);
        self.gfx.stroke_round(x + r * 0.04, y - r * 0.02, x + r * 0.42, y + r * 0.22, 4.0, 2.0, [0.3, 0.9, 1.0, 0.9]);
        self.gfx.fill_round(x - r * 0.18, y + r * 0.38, x + r * 0.18, y + r * 0.46, 3.0, [0.8, 0.4, 0.4, 1.0]);
    }

    fn dialog(&mut self, speaker: &str, text: &str, foot: &str) {
        let (w, h) = (self.w(), self.h());
        let pw = (w - 40.0).min(820.0);
        let x0 = (w - pw) * 0.5;
        let ph = if self.compact() { 200.0 } else { 220.0 };
        let y0 = h - ph - 24.0;
        self.gfx.fill_rect(0.0, 0.0, w, h, [0.0, 0.0, 0.0, 0.35]);
        self.panel_box(x0, y0, x0 + pw, y0 + ph);
        let r = if self.compact() { 40.0 } else { 48.0 };
        self.teacher(x0 + 24.0 + r, y0 + 24.0 + r, r);
        let tx = x0 + 48.0 + r * 2.0;
        let tw = pw - (tx - x0) - 20.0;
        self.gfx.text(speaker, tx, y0 + 16.0, tw, 22.0, Font::Big, ACCENT);
        let hh = self.gfx.measure_h(text, Font::Normal, tw);
        self.gfx.text(text, tx, y0 + 46.0, tw, hh, Font::Normal, TEXT);
        self.gfx.text(foot, x0, y0 + ph - 28.0, pw - 20.0, 20.0, Font::Small, DIM);
        let fw = self.gfx.measure(foot, Font::Small);
        let _ = fw;
    }

    fn render_intro(&mut self, n: u8) {
        let name = t!("トキ先生", "Ms. Toki", "时老师");
        let pages: [L; 4] = [
            L(
                "こんにちは！ 時間旅行学校、地球史クラスのトキです。今学期の宿題は「地球史図鑑」づくり。時空観察カプセルに乗って昔の地球へ行き、生きものを撮影して図鑑に収録してもらいます。",
                "Hello! I'm Ms. Toki from the Earth History class at the Time-Travel School. This term's assignment is to build a 'Field Guide to Earth's History'. Ride the observation capsule into the past, photograph living things and add them to your guide.",
                "你好！我是时间旅行学校地球史班的时老师。本学期的作业是制作“地球史图鉴”。乘坐时空观测舱前往过去的地球，拍摄生物并收录进图鉴。",
            ),
            L(
                "最初の行き先は5億年以上前の古生代カンブリア紀。海の中に、眼や殻や脚を持つ動物たちが一気に現れた「カンブリア爆発」の時代です。陸にはまだ植物がありません。",
                "Your first destination is the Cambrian Period, over 500 million years ago: the 'Cambrian explosion', when animals with eyes, shells and legs suddenly filled the seas. There are no plants on land yet.",
                "第一站是5亿多年前的古生代寒武纪。那是“寒武纪大爆发”的时代，海中一下子出现了长着眼睛、外壳和腿的动物。陆地上还没有植物。",
            ),
            L(
                "地図の下の年表で時代を選び、地図で場所を選びます。大陸はゆっくり動いているので、時代によって形と位置が変わりますよ。「潜る」で海の中へ、撮影ボタンで写真を撮ります。",
                "Choose a time with the timeline under the map and a place on the map. The continents drift slowly, so their shapes and positions change with time. 'Dive' takes you under the sea; the shutter takes a photo.",
                "用地图下方的年表选择时代，在地图上选择地点。大陆在缓慢移动，所以不同时代的形状和位置都会变化。按“下潜”进入海中，按快门拍照。",
            ),
            L(
                "生きものは、大きく・真ん中に・明るく撮るほど★が増えます。右上の宿題のカードの「行く」を押せば、カプセルが目的の時代と場所へ向かいます。では、いってらっしゃい！",
                "The bigger, more centred and brighter your shot, the more ★ you earn. Press 'Go' on the task card at the top right and the capsule heads to the right time and place. Off you go!",
                "生物拍得越大、越居中、越明亮，★就越多。点击右上角作业卡上的“前往”，观测舱就会驶向目标时代和地点。出发吧！",
            ),
        ];
        let text = pages[(n as usize).min(3)].get();
        let foot = tf!("（{}/4）クリック・Enter で次へ", "({}/4) Click or press Enter", "（{}/4）点击或按 Enter 继续", n + 1);
        self.dialog(name, text, &foot);
    }

    fn render_done(&mut self, m: usize) {
        let name = t!("トキ先生", "Ms. Toki", "时老师");
        let mm = &MISSIONS[m];
        let mut text = format!("{}\n\n{}", tf!("宿題{}「{}」合格！", "Task {} '{}' passed!", "作业{}“{}”合格！", m + 1, mm.title.get()), mm.done.get());
        if m + 1 < MISSIONS.len() {
            text.push_str(&tf!("\n\n次の宿題：{}", "\n\nNext task: {}", "\n\n下一个作业：{}", MISSIONS[m + 1].title.get()));
        }
        self.dialog(name, &text, t!("クリック・Enter で閉じる", "Click or press Enter", "点击或按 Enter 关闭"));
    }

    // ------------------------------------------------------------ 写真・知らせ・説明

    fn render_photo(&mut self) {
        let Some(p) = &self.photo else { return };
        let lines = p.lines.clone();
        let advice = p.advice;
        let (w, h) = (self.w(), self.h());
        let pw = if self.compact() { 300.0 } else { 340.0 };
        let n = lines.len().max(1) as f32;
        let ph = 64.0 + n * 30.0 + if advice.is_some() { 48.0 } else { 0.0 };
        let x0 = (w - pw) * 0.5;
        let y0 = (h - ph) * 0.42;
        // ポラロイド風の白い枠
        self.gfx.fill_round(x0 - 6.0, y0 - 6.0, x0 + pw + 6.0, y0 + ph + 6.0, 8.0, [0.96, 0.95, 0.90, 0.97]);
        self.gfx.fill_round(x0 + 6.0, y0 + 6.0, x0 + pw - 6.0, y0 + ph - 6.0, 6.0, [0.06, 0.14, 0.20, 1.0]);
        let head = if lines.iter().any(|l| l.2) { t!("撮影成功！ 図鑑に収録", "Got it! Added to the guide", "拍摄成功！已收录图鉴") } else if !lines.is_empty() { t!("撮影した！", "Snap!", "拍到了！") } else { t!("うまく撮れなかった…", "Not a good shot…", "没拍好……") };
        self.gfx.text(head, x0 + 16.0, y0 + 16.0, pw - 32.0, 26.0, Font::Big, TEXT);
        let mut y = y0 + 52.0;
        for (e, st, new, better) in lines {
            self.gfx.text(entry_name(e), x0 + 18.0, y, pw - 150.0, 22.0, Font::Normal, TEXT);
            self.gfx.text(&stars_text(st), x0 + pw - 140.0, y, 60.0, 22.0, Font::Normal, GOLD);
            if new {
                self.gfx.fill_round(x0 + pw - 72.0, y + 1.0, x0 + pw - 18.0, y + 19.0, 6.0, ACCENT);
                self.gfx.text("NEW", x0 + pw - 72.0, y + 1.0, 54.0, 18.0, Font::NormalCenter, [0.1, 0.05, 0.0, 1.0]);
            } else if better {
                self.gfx.text(t!("★更新", "★ up", "★提升"), x0 + pw - 72.0, y + 1.0, 60.0, 18.0, Font::Small, GOOD);
            }
            y += 30.0;
        }
        if let Some(a) = advice {
            self.gfx.text(a, x0 + 18.0, y + 2.0, pw - 36.0, 44.0, Font::Small, [1.0, 0.86, 0.6, 1.0]);
        }
    }

    fn render_notices(&mut self) {
        let w = self.w();
        let mut y = self.top_h() + 10.0;
        let notes: Vec<(String, f32)> = self.notices.clone();
        for (s, left) in notes {
            let a = left.min(1.0);
            let tw = self.gfx.measure(&s, Font::Normal).min(w * 0.6) + 28.0;
            let x0 = (w - tw) * 0.5;
            self.gfx.fill_round(x0, y, x0 + tw, y + 30.0, 10.0, [0.02, 0.08, 0.12, 0.85 * a]);
            self.gfx.text(&s, x0 + 14.0, y + 6.0, tw - 28.0, 20.0, Font::Normal, [1.0, 0.95, 0.85, a]);
            y += 36.0;
        }
    }

    fn render_tip(&mut self) {
        if self.touch || self.panel != Panel::None {
            return;
        }
        let Some(s) = self.tip.clone() else { return };
        let (mx, my) = self.mouse;
        let mut wd: f32 = 0.0;
        for l in s.lines() {
            wd = wd.max(self.gfx.measure(l, Font::Normal));
        }
        let wd = wd + 20.0;
        let hh = s.lines().count() as f32 * 19.0 + 12.0;
        let x0 = (mx + 16.0).min(self.w() - wd - 6.0);
        let y0 = if my - hh - 10.0 > self.top_h() { my - hh - 10.0 } else { my + 20.0 };
        self.gfx.fill_round(x0, y0, x0 + wd, y0 + hh, 8.0, BG2);
        self.gfx.stroke_round(x0, y0, x0 + wd, y0 + hh, 8.0, 1.0, EDGE);
        self.gfx.text(&s, x0 + 10.0, y0 + 6.0, wd - 16.0, hh, Font::Normal, TEXT);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game() -> Game {
        Game::new(Gfx::new(1600, 900))
    }

    #[test]
    fn new_game_and_dive_and_shoot() {
        let mut g = game();
        g.do_act(Act::NewGame);
        assert_eq!(g.screen, Screen::Map);
        for _ in 0..4 {
            g.do_act(Act::IntroNext);
        }
        assert_eq!(g.panel, Panel::None);
        // バージェスへ
        g.goto(Target::Sp(SP_MARRELLA));
        for _ in 0..600 {
            g.frame(&[], 1.0 / 60.0, true);
        }
        assert!(g.travel.is_none());
        assert!((g.prog.t - 507.5).abs() < 3.0, "t={}", g.prog.t);
        g.do_act(Act::Dive);
        assert_eq!(g.screen, Screen::Dive);
        let present = g.scene.as_ref().unwrap().present.clone();
        assert!(present.contains(&SP_MARRELLA), "{:?}", present);
        for _ in 0..30 {
            g.frame(&[], 1.0 / 60.0, true);
        }
        g.do_act(Act::Shoot);
        assert!(g.photo.is_some());
        g.do_act(Act::Surface);
        assert_eq!(g.screen, Screen::Map);
    }

    #[test]
    fn map_photo_of_gondwana() {
        let mut g = game();
        g.do_act(Act::NewGame);
        g.prog.intro_done = true;
        g.panel = Panel::None;
        g.goto(Target::Ph(P_GONDWANA));
        for _ in 0..200 {
            g.frame(&[], 1.0 / 60.0, true);
        }
        g.follow = false;
        g.goto(Target::Ph(P_GONDWANA));
        let r = g.map_shoot();
        let s = r.iter().find(|x| x.0 == n_species() + P_GONDWANA).map(|x| x.1).unwrap_or(0.0);
        assert!(stars_of(s) >= 1, "score {}", s);
    }

    #[test]
    fn all_screens_render() {
        let mut g = game();
        g.frame(&[], 0.016, true);
        g.do_act(Act::NewGame);
        for p in [Panel::Intro(0), Panel::Guide, Panel::Entry(3), Panel::Entry(n_entries() - 1), Panel::Missions, Panel::Help, Panel::Done(0)] {
            g.panel = p;
            g.frame(&[], 0.016, true);
        }
        g.panel = Panel::None;
        g.prog.intro_done = true;
        g.do_act(Act::Dive);
        g.lens = 3.0;
        g.frame(&[], 0.016, true);
        g.do_act(Act::Shoot);
        g.frame(&[], 0.016, true);
    }
}
