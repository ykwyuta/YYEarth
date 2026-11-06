//! 進み具合（図鑑の星・宿題・時と場所）と、その記録の読み書き
use crate::life::n_entries;
use crate::mission::{mission_done, MISSIONS};
use crate::save::{RResult, Reader, Writer};

const MAGIC: &[u8; 4] = b"YYE1";
const VERSION: u32 = 1;

#[derive(Clone)]
pub struct Progress {
    /// 図鑑の項目ごとの、いちばんよい写真の星（0 は未収録）
    pub stars: Vec<u8>,
    /// まだ図鑑で見ていない新しい項目
    pub fresh: Vec<bool>,
    /// いまの宿題（これより前はすべて達成）
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

    /// 写真を図鑑に収録する。新しい項目か、星が増えたら true
    pub fn record(&mut self, entry: usize, stars: u8) -> (bool, bool) {
        let old = self.stars[entry];
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

    /// 宿題を進める。達成した宿題の番号を返す
    pub fn advance(&mut self) -> Vec<usize> {
        let mut done = Vec::new();
        while self.mission < MISSIONS.len() && mission_done(&MISSIONS[self.mission], &self.stars) {
            done.push(self.mission);
            self.mission += 1;
        }
        done
    }

    pub fn all_clear(&self) -> bool {
        self.mission >= MISSIONS.len()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut w = Writer::new();
        w.bytes(MAGIC);
        w.u32(VERSION);
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
        let n = r.len(4096)?;
        for i in 0..n {
            let s = r.u8()?.min(3);
            let f = r.bool()?;
            if i < p.stars.len() {
                p.stars[i] = s;
                p.fresh[i] = f;
            }
        }
        p.mission = r.usize()?.min(MISSIONS.len());
        p.t = r.f32()?.clamp(crate::earth::T_END, crate::earth::T_START);
        p.lon = r.f32()?.clamp(-180.0, 180.0);
        p.lat = r.f32()?.clamp(-89.0, 89.0);
        p.photos = r.u32()?;
        p.intro_done = r.bool()?;
        p.light = r.bool()?;
        if !p.t.is_finite() || !p.lon.is_finite() || !p.lat.is_finite() {
            return Err(t!("記録が壊れている", "The save is damaged", "存档已损坏").into());
        }
        Ok(p)
    }

    /// 記録の見出し
    pub fn meta(&self) -> String {
        let m = if self.all_clear() { t!("全課題クリア", "All clear", "全部完成").to_string() } else { tf!("宿題{}", "Task {}", "作业{}", self.mission + 1) };
        tf!("カンブリア紀・{}・図鑑 {}/{}", "Cambrian · {} · Guide {}/{}", "寒武纪·{}·图鉴 {}/{}", m, self.found(), n_entries())
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

    #[test]
    fn missions_advance() {
        let mut p = Progress::new();
        assert!(p.advance().is_empty());
        for &(t, n) in MISSIONS[0].targets {
            p.record(t.entry(), n);
        }
        assert_eq!(p.advance(), vec![0]);
        assert_eq!(p.mission, 1);
    }
}
