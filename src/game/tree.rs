//! Continuous, time-banded phylogeny atlas shared by desktop and WebAssembly.
use super::*;
use crate::phylogeny::{self, Card, CARD_H, CARD_W, NODES, PERIOD_HEIGHT, PRELUDE};

#[derive(Clone, Copy, Debug)]
pub(super) struct Rect { pub x0: f32, pub y0: f32, pub x1: f32, pub y1: f32 }

impl Rect {
    pub fn width(self) -> f32 { self.x1 - self.x0 }
    pub fn height(self) -> f32 { self.y1 - self.y0 }
    pub fn center(self) -> (f32, f32) { ((self.x0 + self.x1) * 0.5, (self.y0 + self.y1) * 0.5) }
    pub fn contains(self, x: f32, y: f32) -> bool { x >= self.x0 && x <= self.x1 && y >= self.y0 && y <= self.y1 }
    fn encloses(self, r: Rect) -> bool { self.contains(r.x0, r.y0) && self.contains(r.x1, r.y1) }
    fn intersection(self, r: Rect) -> Option<Self> {
        let r = Self { x0: self.x0.max(r.x0), y0: self.y0.max(r.y0), x1: self.x1.min(r.x1), y1: self.y1.min(r.y1) };
        (r.width() > 0.0 && r.height() > 0.0).then_some(r)
    }
}

pub(super) struct Drag {
    origin: (f32, f32),
    last: (f32, f32),
    moved: bool,
    minimap: bool,
    entry: Option<usize>,
}

impl Game {
    pub(super) fn tree_bounds(&self) -> Rect {
        Rect { x0: if self.compact() { 134.0 } else { 166.0 }, y0: self.top_h() + 82.0, x1: self.w() - 16.0, y1: self.h() - 58.0 }
    }

    pub(super) fn tree_minimap(&self) -> Rect {
        let area = self.tree_bounds();
        let width = if self.compact() { 132.0 } else { 180.0 };
        let height = (width * self.tree_map.height / self.tree_map.width).clamp(88.0, 170.0);
        Rect { x0: area.x1 - width - 12.0, y0: area.y0 + 36.0, x1: area.x1 - 12.0, y1: area.y0 + 36.0 + height }
    }

    pub(super) fn tree_minimap_frame(&self) -> Rect {
        let r = self.tree_minimap();
        Rect { x0: r.x0 - 8.0, y0: r.y0 - 28.0, x1: r.x1 + 8.0, y1: r.y1 + 8.0 }
    }

    pub(super) fn tree_clamp(&mut self) {
        let area = self.tree_bounds();
        if self.tree_view.zoom == 0.0 { self.tree_view.fit(&self.tree_map, area.width(), area.height()); }
        self.tree_view.clamp(&self.tree_map, area.width(), area.height());
    }

    fn tree_screen(&self, x: f32, y: f32) -> (f32, f32) {
        let c = self.tree_bounds().center();
        (c.0 + (x - self.tree_view.x) * self.tree_view.zoom, c.1 + (y - self.tree_view.y) * self.tree_view.zoom)
    }

    fn tree_card_rect(&self, card: &Card) -> Rect {
        let a = self.tree_screen(card.x, card.y);
        let b = self.tree_screen(card.x + CARD_W, card.y + CARD_H);
        Rect { x0: a.0, y0: a.1, x1: b.0, y1: b.1 }
    }

    pub(super) fn tree_pan(&mut self, dx: f32, dy: f32) {
        self.tree_view.x -= dx / self.tree_view.zoom;
        self.tree_view.y -= dy / self.tree_view.zoom;
        self.tree_clamp();
    }

    pub(super) fn tree_zoom(&mut self, factor: f32, anchor: (f32, f32)) {
        let area = self.tree_bounds();
        let c = area.center();
        let anchor = if area.contains(anchor.0, anchor.1) && !self.tree_minimap_frame().contains(anchor.0, anchor.1) { anchor } else { c };
        self.tree_view.zoom_at(factor, anchor.0 - c.0, anchor.1 - c.1, &self.tree_map, area.width(), area.height());
    }

    fn tree_mini_move(&mut self, x: f32, y: f32) {
        let mini = self.tree_minimap();
        self.tree_view.x = ((x - mini.x0) / mini.width()).clamp(0.0, 1.0) * self.tree_map.width;
        self.tree_view.y = ((y - mini.y0) / mini.height()).clamp(0.0, 1.0) * self.tree_map.height;
        self.tree_clamp();
    }

    pub(super) fn tree_down(&mut self, button: u8, x: f32, y: f32) -> bool {
        if button != 0 && button != 2 { return false; }
        let minimap = self.tree_minimap_frame().contains(x, y);
        if !minimap && !self.tree_bounds().contains(x, y) { return false; }
        let entry = if minimap { None } else { self.tree_map.cards.iter().rev().find(|c| self.tree_card_rect(c).contains(x, y)).map(|c| c.entry) };
        self.tree_drag = Some(Drag { origin: (x, y), last: (x, y), moved: button == 2 || minimap, minimap, entry });
        if minimap { self.tree_mini_move(x, y); }
        true
    }

    pub(super) fn tree_move(&mut self, x: f32, y: f32) {
        let Some(mut drag) = self.tree_drag.take() else { return; };
        if drag.minimap { self.tree_mini_move(x, y); }
        else {
            if (x - drag.origin.0).hypot(y - drag.origin.1) > 8.0 { drag.moved = true; }
            if drag.moved { self.tree_pan(x - drag.last.0, y - drag.last.1); }
        }
        drag.last = (x, y);
        self.tree_drag = Some(drag);
    }

    pub(super) fn tree_up(&mut self, button: u8, x: f32, y: f32) {
        self.tree_move(x, y);
        let Some(drag) = self.tree_drag.take() else { return; };
        if button == 0 && !drag.moved {
            if let Some(e) = drag.entry {
                self.tree_entry = Some(e);
                self.tree_focus = phylogeny::node_for_entry(e);
                self.do_act(Act::Entry(e));
            }
        }
    }

    // Clip branch segments in screen coordinates, for both render backends.
    fn tree_line(&mut self, a: (f32, f32), b: (f32, f32), width: f32, color: [f32; 4]) {
        let r = self.tree_bounds();
        let (dx, dy) = (b.0 - a.0, b.1 - a.1);
        let (mut lo, mut hi) = (0.0_f32, 1.0_f32);
        for (p, q) in [(-dx, a.0 - r.x0), (dx, r.x1 - a.0), (-dy, a.1 - r.y0), (dy, r.y1 - a.1)] {
            if p.abs() < 0.0001 { if q < 0.0 { return; } }
            else if p < 0.0 { lo = lo.max(q / p); }
            else { hi = hi.min(q / p); }
        }
        if lo <= hi { self.gfx.line((a.0 + lo * dx, a.1 + lo * dy), (a.0 + hi * dx, a.1 + hi * dy), width, color); }
    }

    fn tree_branch(&mut self, a: (f32, f32), b: (f32, f32), color: [f32; 4]) {
        let mut prev = self.tree_screen(a.0, a.1);
        for step in 1..=20 {
            let t = step as f32 / 20.0;
            let ease = t * t * (3.0 - 2.0 * t);
            let next = self.tree_screen(a.0 + (b.0 - a.0) * ease, a.1 + (b.1 - a.1) * t);
            self.tree_line(prev, next, (self.tree_view.zoom * 6.0).clamp(1.3, 4.0), color);
            prev = next;
        }
    }

    pub(super) fn render_tree_gl(&mut self) {
        self.tree_clamp();
        let area = self.tree_bounds();
        self.gfx.rect(0.0, 0.0, self.w(), self.h(), [0.02, 0.055, 0.085, 1.0]);
        for (i, p) in PERIODS.iter().enumerate() {
            let top = self.tree_screen(0.0, PRELUDE + i as f32 * PERIOD_HEIGHT).1.max(area.y0);
            let bottom = self.tree_screen(0.0, PRELUDE + (i + 1) as f32 * PERIOD_HEIGHT).1.min(area.y1);
            if bottom <= top { continue; }
            let c = p.col;
            self.gfx.rect(12.0, top, area.x1, bottom, [c[0] * 0.15 + 0.02, c[1] * 0.15 + 0.03, c[2] * 0.15 + 0.04, 1.0]);
            self.gfx.line((12.0, top), (area.x1, top), 1.0, [c[0], c[1], c[2], 0.24]);
        }
        for (n, node) in NODES.iter().enumerate() {
            if let Some(parent) = node.parent {
                let color = if self.tree_focus.is_some_and(|f| phylogeny::on_path(n, f)) { GOLD } else { [0.35, 0.77, 0.70, 0.8] };
                self.tree_branch(self.tree_map.nodes[parent], self.tree_map.nodes[n], color);
            }
        }
        let cards = self.tree_map.cards.clone();
        for card in &cards {
            // A separate stem attaches each catalogue example to its group;
            // the species card itself is never an ancestor of another card.
            let a = self.tree_map.nodes[card.node];
            let x = card.x + CARD_W * 0.5;
            self.tree_branch(a, (x, card.start), [0.40, 0.65, 0.65, 0.32]);
            let from = self.tree_screen(x, card.start);
            let to = self.tree_screen(x, card.end);
            self.tree_line(from, to, (self.tree_view.zoom * 8.0).clamp(1.8, 5.0), [0.58, 0.79, 0.67, 0.55]);
        }
        // All connections sit below every card, including connections for
        // specimens drawn later in the catalogue.
        for card in &cards {
            let r = self.tree_card_rect(card);
            let Some(vis) = area.intersection(r) else { continue; };
            let selected = self.tree_entry == Some(card.entry);
            self.gfx.rect(vis.x0, vis.y0, vis.x1, vis.y1, [0.065, 0.19, 0.24, 1.0]);
            self.gfx.rect_outline(vis.x0, vis.y0, vis.x1, vis.y1, if selected { 2.0 } else { 1.0 }, if selected { GOLD } else { EDGE });
            if r.width() >= 100.0 && area.encloses(r) && self.tree_minimap_frame().intersection(r).is_none() {
                let image_w = (r.width() * 0.36).min(r.height());
                self.draw_entry_art(card.entry, r.x0 + 3.0, r.y0 + 3.0, r.x0 + image_w, r.y1 - 3.0, if self.prog.stars[card.entry] > 0 { 1.0 } else { 0.38 });
                self.gfx.flush();
            }
        }
        for n in 0..NODES.len() {
            let p = self.tree_map.nodes[n];
            let p = self.tree_screen(p.0, p.1);
            if area.contains(p.0, p.1) { self.gfx.circle(p.0, p.1, (self.tree_view.zoom * 10.0).clamp(2.5, 6.0), 12, GOOD); }
        }
        self.render_tree_mini_gl();
    }

    fn render_tree_mini_gl(&mut self) {
        let r = self.tree_minimap();
        let outer = self.tree_minimap_frame();
        self.gfx.rect(outer.x0, outer.y0, outer.x1, outer.y1, [0.025, 0.075, 0.11, 1.0]);
        self.gfx.rect_outline(outer.x0, outer.y0, outer.x1, outer.y1, 1.0, EDGE);
        let project = |x: f32, y: f32| (r.x0 + x / self.tree_map.width * r.width(), r.y0 + y / self.tree_map.height * r.height());
        for (i, p) in PERIODS.iter().enumerate() {
            let top = project(0.0, PRELUDE + i as f32 * PERIOD_HEIGHT).1;
            let bottom = project(0.0, PRELUDE + (i + 1) as f32 * PERIOD_HEIGHT).1;
            self.gfx.rect(r.x0, top, r.x1, bottom, [p.col[0] * 0.3, p.col[1] * 0.3, p.col[2] * 0.3, 1.0]);
        }
        for (n, node) in NODES.iter().enumerate() {
            let b = self.tree_map.nodes[n];
            if let Some(parent) = node.parent {
                let a = self.tree_map.nodes[parent];
                self.gfx.line(project(a.0, a.1), project(b.0, b.1), 1.0, EDGE);
            }
        }
        for card in &self.tree_map.cards {
            let p = project(card.x + CARD_W * 0.5, card.y + CARD_H * 0.5);
            self.gfx.rect(p.0 - 1.0, p.1 - 1.0, p.0 + 1.0, p.1 + 1.0, DIM);
        }
        let area = self.tree_bounds();
        let half_w = area.width() / self.tree_view.zoom * 0.5;
        let half_h = area.height() / self.tree_view.zoom * 0.5;
        let a = project((self.tree_view.x - half_w).max(0.0), (self.tree_view.y - half_h).max(0.0));
        let b = project((self.tree_view.x + half_w).min(self.tree_map.width), (self.tree_view.y + half_h).min(self.tree_map.height));
        self.gfx.rect(a.0, a.1, b.0, b.1, [1.0, 0.85, 0.4, 0.12]);
        self.gfx.rect_outline(a.0, a.1, b.0, b.1, 1.5, GOLD);
    }

    pub(super) fn render_tree_ui(&mut self) {
        let area = self.tree_bounds();
        let header = self.top_h();
        let w = self.w();
        self.gfx.text(t!("生命の系統樹マップ", "Atlas of the tree of life", "生命系统树地图"), 24.0, header + 10.0, w - 460.0, 28.0, Font::Big, TEXT);
        self.add_button(w - 440.0, header + 8.0, w - 390.0, header + 42.0, "−", Act::TreeZoom(1.0 / 1.4), true, false);
        self.add_button(w - 384.0, header + 8.0, w - 334.0, header + 42.0, "＋", Act::TreeZoom(1.4), true, false);
        self.add_button(w - 326.0, header + 8.0, w - 220.0, header + 42.0, t!("全体を見る", "Fit map", "查看全图"), Act::TreeFit, true, false);
        self.add_button(w - 210.0, header + 8.0, w - 20.0, header + 42.0, t!("図鑑一覧へ", "Guide index", "返回图鉴"), Act::Close, true, false);
        self.gfx.text(t!("過去から今へ ↓　ドラッグで移動・2本指／ホイールで拡大縮小・生きものを押すと図鑑へ", "Past → present ↓  Drag to explore · pinch / wheel to zoom · choose a creature for its guide", "从古至今 ↓　拖动移动 · 双指／滚轮缩放 · 点击生物查看图鉴"), 24.0, header + 50.0, w - 48.0, 24.0, Font::Normal, DIM);
        let pre_bottom = self.tree_screen(0.0, PRELUDE).1.min(area.y1);
        if pre_bottom > area.y0 + 24.0 {
            self.gfx.text(t!("カンブリア紀より前\n模式的な分岐", "Before Cambrian\nSchematic splits", "寒武纪之前\n示意分支"), 20.0, area.y0 + 8.0, area.x0 - 28.0, 68.0, Font::Normal, DIM);
        }
        for (i, p) in PERIODS.iter().enumerate() {
            let top = self.tree_screen(0.0, PRELUDE + i as f32 * PERIOD_HEIGHT).1;
            let bottom = self.tree_screen(0.0, PRELUDE + (i + 1) as f32 * PERIOD_HEIGHT).1;
            if bottom <= area.y0 || top >= area.y1 { continue; }
            let y = top.max(area.y0) + 5.0;
            if y + 21.0 <= bottom.min(area.y1) {
                self.gfx.text(p.name.get(), 20.0, y, area.x0 - 30.0, 22.0, Font::Normal, TEXT);
                if y + 58.0 <= bottom.min(area.y1) {
                    self.gfx.text(&span_text(p.t0, p.t1), 20.0, y + 24.0, area.x0 - 30.0, 34.0, Font::Small, DIM);
                }
            }
        }
        // Readable labels appear as users zoom in; avoid overlapping labels in
        // the overview and avoid drawing through the minimap overlay.
        let mut labels: Vec<Rect> = Vec::new();
        for n in 0..NODES.len() {
            if self.tree_view.zoom < 0.25 && ![0, 1, 5, 11, 17, 25, 29, 31].contains(&n) { continue; }
            let p = self.tree_map.nodes[n];
            let p = self.tree_screen(p.0, p.1);
            let r = Rect { x0: p.0 - 80.0, y0: p.1 - 35.0, x1: p.0 + 80.0, y1: p.1 - 5.0 };
            if !area.encloses(r) || self.tree_minimap_frame().intersection(r).is_some() || labels.iter().any(|label| label.intersection(r).is_some()) { continue; }
            if self.tree_map.cards.iter().any(|card| self.tree_card_rect(card).intersection(r).is_some()) { continue; }
            labels.push(r);
            self.gfx.fill_round(r.x0, r.y0, r.x1, r.y1, 5.0, BG);
            self.gfx.text(NODES[n].name.get(), r.x0 + 4.0, r.y0 + 2.0, r.width() - 8.0, r.height() - 2.0, Font::Small, if self.tree_focus.is_some_and(|f| phylogeny::on_path(n, f)) { GOLD } else { TEXT });
        }
        for card in &self.tree_map.cards {
            let r = self.tree_card_rect(card);
            if r.width() < 100.0 || !area.encloses(r) || self.tree_minimap_frame().intersection(r).is_some() { continue; }
            let image_w = (r.width() * 0.36).min(r.height());
            self.gfx.text(entry_name(card.entry), r.x0 + image_w + 5.0, r.y0 + 5.0, r.width() - image_w - 10.0, (r.height() - 25.0).max(20.0), Font::Normal, TEXT);
            if r.height() >= 65.0 {
                self.gfx.text(if self.prog.stars[card.entry] > 0 { t!("収録済み · 図鑑へ", "Recorded · Open", "已收录 · 打开") } else { t!("未撮影 · 図鑑へ", "Unseen · Open", "未拍摄 · 打开") }, r.x0 + image_w + 5.0, r.y1 - 22.0, r.width() - image_w - 10.0, 18.0, Font::Small, DIM);
            }
        }
        let mini = self.tree_minimap();
        self.gfx.text(t!("全体図", "Overview", "总览"), mini.x0, mini.y0 - 23.0, mini.width(), 20.0, Font::Normal, TEXT);
        let zoom = self.tree_view.zoom;
        self.gfx.text(&tf!("拡大率 {}%", "Zoom {}%", "缩放 {}%", (zoom * 100.0).round() as u32), area.x0, area.y1 + 7.0, 150.0, 20.0, Font::Small, DIM);
        self.gfx.text(t!("帯は各紀を同じ高さで表示。枝分かれの年代は模式的。細い縦線は生存年代。", "Equal-height period bands; schematic split dates. Thin vertical bars show catalogue age ranges.", "各纪带状背景等高；分支时间为示意。细竖线表示生存年代。"), area.x0 + 154.0, area.y1 + 7.0, w - area.x0 - 170.0, 35.0, Font::Small, DIM);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phylogeny_atlas_fit_navigation_minimap_and_guide_round_trip() {
        for (width, height, compact) in [(1600, 900, false), (1170, 2532, true), (844, 390, true)] {
            let mut g = Game::new(Gfx::new(width, height));
            g.gfx.set_compact(compact);
            g.screen = Screen::Map;
            g.prog.intro_done = true;
            g.do_act(Act::Tree);
            g.frame(&[], 0.016, true);
            let area = g.tree_bounds();
            let mini = g.tree_minimap();
            assert!(area.width() > 500.0 && area.height() > 140.0);
            assert!(area.encloses(g.tree_minimap_frame()));
            let c = area.center();
            assert_eq!(g.hit_kind(c.0 * g.gfx.ui, c.1 * g.gfx.ui), 4);
            assert_eq!(g.hit_kind(mini.center().0 * g.gfx.ui, mini.center().1 * g.gfx.ui), 2);
            let fit = g.tree_view;
            let top = g.tree_screen(0.0, 0.0);
            let bottom = g.tree_screen(g.tree_map.width, g.tree_map.height);
            assert!(area.contains(top.0, top.1) && area.contains(bottom.0, bottom.1));
            let mammoth = SPECIES.iter().position(|s| s.slug == "woolly-mammoth").unwrap();
            g.do_act(Act::Entry(mammoth));
            g.do_act(Act::Tree);
            assert_eq!(g.tree_entry, Some(mammoth));
            assert!(g.tree_view.zoom > fit.zoom);
            let card = *g.tree_map.cards.iter().find(|c| c.entry == mammoth).unwrap();
            let r = g.tree_card_rect(&card);
            assert!(area.encloses(r));
            let p = r.center();
            let view = g.tree_view;
            g.handle(Ev::Down(0, p.0, p.1));
            assert_eq!(g.panel, Panel::Tree, "activation waits for release");
            g.handle(Ev::Up(0, p.0, p.1));
            assert_eq!(g.panel, Panel::Entry(mammoth));
            g.do_act(Act::Close);
            assert_eq!(g.panel, Panel::Tree);
            assert_eq!(g.tree_view, view);
            g.handle(Ev::Down(0, p.0, p.1));
            g.handle(Ev::MouseMove(p.0 + 48.0, p.1 - 45.0));
            g.handle(Ev::Up(0, p.0 + 48.0, p.1 - 45.0));
            assert_eq!(g.panel, Panel::Tree, "dragging a card never opens it");
            assert_ne!(g.tree_view, view);
            // Recentring from the minimap works while magnified, without
            // opening a specimen behind its overlay.
            g.handle(Ev::Down(0, mini.x0 + mini.width() * 0.3, mini.y0 + mini.height() * 0.3));
            g.handle(Ev::MouseMove(mini.x0 + mini.width() * 0.4, mini.y0 + mini.height() * 0.4));
            g.handle(Ev::Up(0, mini.x0 + mini.width() * 0.4, mini.y0 + mini.height() * 0.4));
            assert_eq!(g.panel, Panel::Tree);
            assert!((g.tree_view.x - g.tree_map.width * 0.4).abs() < 1.0);
            assert!((g.tree_view.y - g.tree_map.height * 0.4).abs() < 1.0);
            // Midpoint translation and zoom can interleave during a pinch.
            g.handle(Ev::Down(2, c.0, c.1));
            g.handle(Ev::MouseMove(c.0 + 20.0, c.1 + 16.0));
            g.handle(Ev::Wheel(2.0));
            let zoomed = g.tree_view;
            g.handle(Ev::MouseMove(c.0 + 22.0, c.1 + 18.0));
            assert!((g.tree_view.x - zoomed.x + 2.0 / zoomed.zoom).abs() < 0.1);
            g.handle(Ev::Up(2, c.0 + 22.0, c.1 + 18.0));
            g.do_act(Act::TreeFit);
            assert_eq!(g.tree_view, fit);
            g.frame(&[], 0.016, true);
        }
    }
}
