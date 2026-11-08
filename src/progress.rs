//! 進み具合（図鑑の収録・宿題・時と場所）と、その記録の読み書き
use crate::life::{n_entries, n_species};
use crate::mission::{mission_done, MISSIONS};
use crate::save::{RResult, Reader, Writer};

const MAGIC: &[u8; 4] = b"YYE1";
/// 3：生きものの数を記録に書く版（生きものを足しても、現象の星がずれない）
/// 2：古生代の全体（オルドビス紀〜ペルム紀）を加えた版。1：カンブリア紀だけ
const VERSION: u32 = 3;
/// 版1・版2の記録の生きものの数。現象はこの後ろに並んでいた
const V1_SPECIES: usize = 36;
const V2_SPECIES: usize = 99;

#[derive(Clone)]
pub struct Progress {
    /// 図鑑の項目ごとの、収録済みか（1 が収録済み、0 が未収録。以前の版の星 1〜3 は 1 として読む）
    pub stars: Vec<u8>,
    /// まだ図鑑で見ていない新しい項目
    pub fresh: Vec<bool>,
    /// 選んでいる宿題（どの順番でも取り組める。合格したかは図鑑の収録から決まる）
    pub mission: usize,
    /// いまの時代（百万年前）
    pub t: f32,
    pub lon: f32,
    pub lat: f32,
    pub photos: u32,
    pub intro_done: bool,
    pub light: bool,
}

impl Progress {
    pub fn new() -> Progress {
        Progress { stars: vec![0; n_entries()], fresh: vec![false; n_entries()], mission: 0, t: 535.0, lon: 40.0, lat: 5.0, photos: 0, intro_done: false, light: false }
    }

    pub fn found(&self) -> usize {
        self.stars.iter().filter(|&&s| s > 0).count()
    }

    /// 写真を図鑑に収録する。(新しい項目か, 記録が変わったか)
    pub fn record(&mut self, entry: usize, stars: u8) -> (bool, bool) {
        let old = self.stars[entry];
        let stars = stars.min(1);
        if stars > old {
            self.stars[entry] = stars;
            if old == 0 {
                self.fresh[entry] = true;
            }
            (old == 0, true)
        } else {
            (false, false)
        }
    }

    /// その宿題に合格しているか（収録は消えないので、一度合格すればずっと合格）
    pub fn is_done(&self, i: usize) -> bool {
        mission_done(&MISSIONS[i], &self.stars)
    }

    /// 宿題ごとの合格（写真を撮る前に取っておき、`advance` に渡す）
    pub fn done_flags(&self) -> Vec<bool> {
        (0..MISSIONS.len()).map(|i| self.is_done(i)).collect()
    }

    pub fn done_count(&self) -> usize {
        (0..MISSIONS.len()).filter(|&i| self.is_done(i)).count()
    }

    /// 写真のあとで呼ぶ。新しく合格した宿題（どの順番でも）を返す。
    /// 選んでいた宿題が合格したら、次のまだ合格していない宿題を選ぶ
    pub fn advance(&mut self, before: &[bool]) -> Vec<usize> {
        let done: Vec<usize> = (0..MISSIONS.len()).filter(|&i| !before.get(i).copied().unwrap_or(false) && self.is_done(i)).collect();
        if self.is_done(self.mission) {
            self.select_next(1);
        }
        done
    }

    /// まだ合格していない宿題を、いまの宿題から d の向き（1 次、-1 前）に探して選ぶ。無ければそのまま
    pub fn select_next(&mut self, d: i32) {
        let n = MISSIONS.len() as i32;
        for k in 1..=n {
            let i = (self.mission as i32 + d.signum() * k).rem_euclid(n) as usize;
            if !self.is_done(i) {
                self.mission = i;
                return;
            }
        }
    }

    pub fn all_clear(&self) -> bool {
        self.done_count() >= MISSIONS.len()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut w = Writer::new();
        w.bytes(MAGIC);
        w.u32(VERSION);
        w.usize(n_species());
        w.usize(self.stars.len());
        for (i, &s) in self.stars.iter().enumerate() {
            w.u8(s);
            w.bool(self.fresh[i]);
        }
        w.usize(self.mission);
        w.f32(self.t);
        w.f32(self.lon);
        w.f32(self.lat);
        w.u32(self.photos);
        w.bool(self.intro_done);
        w.bool(self.light);
        w.buf
    }

    pub fn from_bytes(data: &[u8]) -> RResult<Progress> {
        let mut r = Reader::new(data);
        if r.bytes(4)? != MAGIC {
            return Err(t!("地球史図鑑の記録ではない", "This is not a YYEarth save", "这不是地球史图鉴的存档").into());
        }
        let v = r.u32()?;
        if v > VERSION {
            return Err(t!("新しい版で作られた記録で、読み込めない", "This save was made by a newer version", "此存档由更新版本创建，无法读取").into());
        }
        let mut p = Progress::new();
        // 記録したときの生きものの数。現象の星は、いまの生きものの数の後ろへ移す
        let saved_species = match v {
            1 => V1_SPECIES,
            2 => V2_SPECIES,
            _ => r.len(4096)?,
        };
        let n = r.len(4096)?;
        for i in 0..n {
            let s = r.u8()?.min(1);
            let f = r.bool()?;
            let j = if i >= saved_species { n_species() + (i - saved_species) } else { i };
            if j < p.stars.len() {
                p.stars[j] = s;
                p.fresh[j] = f;
            }
        }
        // 前の版では「ここまで合格」の番号だった。合格済みなら、次のまだ合格していない宿題を選ぶ
        p.mission = r.usize()?.min(MISSIONS.len() - 1);
        p.t = r.f32()?.clamp(crate::earth::T_END, crate::earth::T_START);
        p.lon = r.f32()?.clamp(-180.0, 180.0);
        p.lat = r.f32()?.clamp(-89.0, 89.0);
        p.photos = r.u32()?;
        p.intro_done = r.bool()?;
        p.light = r.bool()?;
        if p.is_done(p.mission) {
            p.select_next(1);
        }
        if !p.t.is_finite() || !p.lon.is_finite() || !p.lat.is_finite() {
            return Err(t!("記録が壊れている", "The save is damaged", "存档已损坏").into());
        }
        Ok(p)
    }

    /// 記録の見出し
    pub fn meta(&self) -> String {
        let m = if self.all_clear() { t!("全課題クリア", "All clear", "全部完成").to_string() } else { tf!("宿題 {}/{}合格", "Tasks {}/{}", "作业 {}/{}合格", self.done_count(), MISSIONS.len()) };
        let era = crate::earth::PERIODS[crate::earth::period_at(self.t)].name.get();
        tf!("{}・{}・図鑑 {}/{}", "{} · {} · Guide {}/{}", "{}·{}·图鉴 {}/{}", era, m, self.found(), n_entries())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let mut p = Progress::new();
        p.record(3, 2);
        p.record(n_entries() - 1, 1);
        p.t = 508.0;
        p.lon = -100.5;
        p.lat = 12.25;
        p.photos = 42;
        p.intro_done = true;
        let b = p.to_bytes();
        let q = Progress::from_bytes(&b).unwrap();
        assert_eq!(q.stars, p.stars);
        assert_eq!(q.fresh, p.fresh);
        assert_eq!(q.t, 508.0);
        assert_eq!(q.lon, -100.5);
        assert_eq!(q.photos, 42);
        assert!(q.intro_done);
        assert!(Progress::from_bytes(&b[..b.len() - 3]).is_err());
        assert!(Progress::from_bytes(b"NOPE").is_err());
    }

    /// The Paleozoic release has 212 species followed by 13 phenomena.
    /// New species must never consume or displace those saved phenomenon records.
    #[test]
    fn paleozoic_v3_save_preserves_species_phenomena_and_completed_tasks() {
        let mut w = Writer::new();
        w.bytes(MAGIC);
        w.u32(3);
        w.usize(212);
        w.usize(225);
        for _ in 0..225 {
            w.u8(1);
            w.bool(true);
        }
        w.usize(35);
        w.f32(251.9);
        w.f32(10.0);
        w.f32(-30.0);
        w.u32(225);
        w.bool(true);
        w.bool(false);
        let p = Progress::from_bytes(&w.buf).unwrap();
        assert!(p.stars[..212].iter().all(|&s| s == 1));
        assert!(p.stars[212..n_species()].iter().all(|&s| s == 0));
        assert!(p.stars[n_species()..n_species()+13].iter().all(|&s| s == 1));
        assert!(p.stars[n_species()+13..].iter().all(|&s| s == 0));
        assert_eq!(p.found(), 225);
        assert_eq!(p.done_count(), 36);
        assert_eq!(p.mission, 36);
        assert!(!p.all_clear());
        let q = Progress::from_bytes(&p.to_bytes()).unwrap();
        assert_eq!(q.stars, p.stars);
        assert_eq!(q.fresh, p.fresh);
    }

    #[test]
    fn original_mesozoic_save_preserves_records_after_widespread_fauna_are_added() {
        let mut w = Writer::new();
        w.bytes(MAGIC);
        w.u32(3);
        w.usize(283);
        w.usize(302);
        for _ in 0..302 { w.u8(1); w.bool(true); }
        w.usize(0);
        w.f32(150.0);
        w.f32(0.0);
        w.f32(0.0);
        w.u32(302);
        w.bool(true);
        w.bool(false);
        let p = Progress::from_bytes(&w.buf).unwrap();
        assert!(p.stars[..283].iter().all(|&s| s == 1));
        assert!(p.stars[283..n_species()].iter().all(|&s| s == 0));
        assert!(p.fresh[283..n_species()].iter().all(|&f| !f));
        assert!(p.stars[n_species()..n_species()+19].iter().all(|&s| s == 1));
        assert!(p.stars[n_species()+19..].iter().all(|&s| s == 0));
        assert_eq!(p.found(), 302);
        assert_eq!(p.done_count(), 51);
        assert!(!p.all_clear());
        let q = Progress::from_bytes(&p.to_bytes()).unwrap();
        assert_eq!(q.stars, p.stars);
        assert_eq!(q.fresh, p.fresh);
    }

    #[test]
    fn complete_295_species_save_keeps_old_tasks_and_adds_cenozoic_unfinished() {
        let mut w = Writer::new();
        w.bytes(MAGIC);
        w.u32(3);
        w.usize(295);
        w.usize(314);
        for _ in 0..314 { w.u8(1); w.bool(true); }
        w.usize(50);
        w.f32(66.0);
        w.f32(0.0);
        w.f32(0.0);
        w.u32(314);
        w.bool(true);
        w.bool(false);
        let mut p = Progress::from_bytes(&w.buf).unwrap();
        assert_eq!(p.found(), 314);
        assert_eq!(p.done_count(), 51);
        assert_eq!(p.mission, 51);
        assert!(p.stars[295..n_species()].iter().all(|&s| s == 0));
        assert!(p.stars[n_species()..n_species()+19].iter().all(|&s| s == 1));
        assert!(p.stars[n_species()+19..].iter().all(|&s| s == 0));
        p.t = crate::earth::T_END;
        let q = Progress::from_bytes(&p.to_bytes()).unwrap();
        assert_eq!(q.t, 0.01);
        assert_eq!(q.stars, p.stars);
    }

    #[test]
    fn initial_cenozoic_saves_preserve_all_records_after_regional_additions() {
        for saved_species in [331, 403] {
            let saved_entries = saved_species + 22;
            let mut w = Writer::new();
            w.bytes(MAGIC);
            w.u32(3);
            w.usize(saved_species);
            w.usize(saved_entries);
            for _ in 0..saved_entries { w.u8(1); w.bool(true); }
            w.usize(62);
            w.f32(0.01);
            w.f32(20.0);
            w.f32(-10.0);
            w.u32(saved_entries as u32);
            w.bool(true);
            w.bool(true);
            let p = Progress::from_bytes(&w.buf).unwrap();
            assert_eq!(p.found(), saved_entries);
            assert!(p.stars[..saved_species].iter().all(|&s| s == 1));
            assert!(p.stars[saved_species..n_species()].iter().all(|&s| s == 0));
            assert!(p.stars[n_species()..].iter().all(|&s| s == 1));
            assert_eq!(p.done_count(), 63);
            assert_eq!(p.t, 0.01);
            let q = Progress::from_bytes(&p.to_bytes()).unwrap();
            assert_eq!(q.stars, p.stars);
            assert_eq!(q.fresh, p.fresh);
        }
    }

    #[test]
    fn all_mesozoic_tasks_can_be_completed_and_saved_with_single_photographs() {
        let mut p = Progress::new();
        for (i, m) in MISSIONS.iter().enumerate().filter(|(_, m)| m.period >= crate::earth::TRIASSIC) {
            p.mission = i;
            for &(target, _) in m.targets {
                p.record(target.entry(), 1);
            }
            assert!(p.is_done(i), "{}", m.title.1);
        }
        let q = Progress::from_bytes(&p.to_bytes()).unwrap();
        assert_eq!(q.stars, p.stars);
        for (i, m) in MISSIONS.iter().enumerate().filter(|(_, m)| m.period >= crate::earth::TRIASSIC) {
            assert!(q.is_done(i), "{}", m.title.1);
        }
        for e in 0..n_entries() { p.record(e, 1); }
        assert!(p.all_clear());
        assert!(Progress::from_bytes(&p.to_bytes()).unwrap().all_clear());
    }

    /// 古生代99種の版（版2）の記録を読むと、現象の星が新しい並びに移る
    #[test]
    fn reads_v2_save() {
        let mut w = Writer::new();
        w.bytes(MAGIC);
        w.u32(2);
        w.usize(112);
        for i in 0..112 {
            w.u8(if i == 50 || i == 99 + crate::life::P_GONDWANA_ICE { 3 } else { 0 });
            w.bool(false);
        }
        w.usize(30);
        w.f32(300.0);
        w.f32(10.0);
        w.f32(-60.0);
        w.u32(9);
        w.bool(true);
        w.bool(false);
        let p = Progress::from_bytes(&w.buf).unwrap();
        assert_eq!(p.stars[50], 1);
        assert_eq!(p.stars[99], 0);
        assert_eq!(p.stars[n_species() + crate::life::P_GONDWANA_ICE], 1);
        assert_eq!(p.mission, 30);
    }

    /// カンブリア紀だけの版（版1）の記録を読むと、現象の星が新しい並びに移る
    #[test]
    fn reads_v1_save() {
        let mut w = Writer::new();
        w.bytes(MAGIC);
        w.u32(1);
        w.usize(40);
        for i in 0..40 {
            w.u8(if i == 16 || i == 36 || i == 39 { 2 } else { 0 });
            w.bool(false);
        }
        w.usize(10);
        w.f32(486.0);
        w.f32(10.0);
        w.f32(-5.0);
        w.u32(7);
        w.bool(true);
        w.bool(false);
        let p = Progress::from_bytes(&w.buf).unwrap();
        assert_eq!(p.stars[16], 1);
        assert_eq!(p.stars[36], 0);
        assert_eq!(p.stars[n_species() + crate::life::P_GONDWANA], 1);
        assert_eq!(p.stars[n_species() + crate::life::P_BARREN], 1);
        assert_eq!(p.mission, 10);
        assert_eq!(p.t, 486.0);
    }

    #[test]
    fn missions_advance() {
        let mut p = Progress::new();
        let before = p.done_flags();
        assert!(p.advance(&before).is_empty());
        for &(t, n) in MISSIONS[0].targets {
            p.record(t.entry(), n);
        }
        assert_eq!(p.advance(&before), vec![0]);
        assert_eq!(p.mission, 1);
    }

    /// 宿題はどの順番でも合格できる。選んでいない宿題に合格しても、選んでいる宿題はそのまま
    #[test]
    fn missions_in_any_order() {
        let mut p = Progress::new();
        let last = MISSIONS.len() - 1;
        let before = p.done_flags();
        for &(t, n) in MISSIONS[last].targets {
            p.record(t.entry(), n);
        }
        assert_eq!(p.advance(&before), vec![last]);
        assert_eq!(p.mission, 0);
        assert!(p.is_done(last) && !p.is_done(0));
        p.mission = 5;
        p.select_next(1);
        assert_eq!(p.mission, 6);
        p.mission = 0;
        p.select_next(-1);
        assert_eq!(p.mission, last - 1);
        // 記録を読み直しても、合格は図鑑の収録から決まる
        let q = Progress::from_bytes(&p.to_bytes()).unwrap();
        assert!(q.is_done(last) && !q.all_clear());
    }
}
