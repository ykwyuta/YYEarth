//! A deliberately pruned cladogram. Fossils are examples on branches, never
//! asserted to be direct ancestors. Branch points are schematic; specimen
//! ranges use the catalogue ages on a separate, piecewise time axis.
use crate::{earth::PERIODS, lang::L, life::SPECIES};

pub struct Node {
    pub parent: Option<usize>,
    pub name: L,
    pub examples: &'static [&'static str],
}

macro_rules! node {
    ($parent:expr, $ja:expr, $en:expr, $zh:expr, [$($slug:expr),*]) => {
        Node { parent: $parent, name: L($ja, $en, $zh), examples: &[$($slug),*] }
    };
}

// Preorder: every parent precedes its children. Omitted intermediate groups
// and unresolved relationships are described in docs/phylogeny.md.
pub static NODES: &[Node] = &[
    node!(None, "真核生物（抜粋）", "Eukaryotes (selection)", "真核生物（节选）", []), // 0
    node!(Some(0), "陸上植物", "Land plants", "陆生植物", ["early-land-plants"]), // 1
    node!(Some(1), "維管束植物", "Vascular plants", "维管植物", ["cooksonia", "rhynia", "lepidodendron", "calamites", "psaronius", "pleuromeia"]), // 2
    node!(Some(2), "種子植物", "Seed plants", "种子植物", ["glossopteris", "walchia", "ginkgo", "araucarioxylon"]), // 3
    node!(Some(3), "被子植物", "Flowering plants", "被子植物", ["archaefructus", "paleogene-trees", "neogene-grass", "holocene-oak"]), // 4
    node!(Some(0), "動物", "Animals", "动物", []), // 5
    node!(Some(5), "海綿動物", "Sponges", "海绵动物", ["archaeocyath", "vauxia"]), // 6
    node!(Some(5), "刺胞動物", "Cnidarians", "刺胞动物", ["burgessomedusa", "favosites", "halysites", "heliophyllum", "lithostrotion", "waagenophyllum"]), // 7
    node!(Some(5), "左右相称動物", "Bilaterians", "两侧对称动物", []), // 8
    node!(Some(8), "前口動物", "Protostomes", "原口动物", []), // 9
    node!(Some(9), "汎節足動物", "Panarthropods", "泛节肢动物", ["hallucigenia"]), // 10
    node!(Some(10), "節足動物と幹系統", "Arthropods + stem relatives", "节肢动物及干群", ["anomalocaris", "elrathia", "calymene", "walliserops", "eurypterus", "arthropleura", "ditomopyge", "cenozoic-crabs"]), // 11
    node!(Some(11), "昆虫", "Insects", "昆虫", ["meganeura", "meganeuropsis", "mesozoic-roachoids", "paleogene-insects", "neogene-insects", "quaternary-insects"]), // 12
    node!(Some(9), "軟体動物", "Molluscs", "软体动物", ["maclurites", "orthoceras", "dawsonoceras", "manticoceras", "goniatites", "waagenoceras", "ceratites", "dactylioceras", "cenozoic-bivalves"]), // 13
    node!(Some(8), "後口動物", "Deuterostomes", "后口动物", []), // 14
    node!(Some(14), "棘皮動物", "Echinoderms", "棘皮动物", ["gogia", "bothriocidaris", "helianthaster", "pentremites"]), // 15
    node!(Some(14), "脊索動物", "Chordates", "脊索动物", ["pikaia"]), // 16
    node!(Some(16), "脊椎動物", "Vertebrates", "脊椎动物", ["haikouichthys", "arandaspis", "birkenia", "dunkleosteus"]), // 17
    node!(Some(17), "軟骨魚類と近縁群", "Cartilaginous fish + relatives", "软骨鱼及近亲", ["cladoselache", "falcatus", "helicoprion", "megalodon"]), // 18
    node!(Some(17), "硬骨魚類", "Bony vertebrates", "硬骨鱼类", []), // 19
    node!(Some(19), "条鰭類", "Ray-finned fish", "辐鳍鱼类", ["cheirolepis", "elonichthys", "palaeoniscum", "dapedium", "leedsichthys", "xiphactinus", "cenozoic-fish"]), // 20
    node!(Some(19), "肉鰭類（四肢動物を含む）", "Lobe-fins incl. tetrapods", "肉鳍类（含四足动物）", ["guiyu", "eusthenopteron", "tiktaalik"]), // 21
    node!(Some(21), "四肢動物", "Tetrapods", "四足动物", ["acanthostega", "ichthyostega", "dendrerpeton", "eryops", "metoposaurus"]), // 22
    node!(Some(22), "羊膜類", "Amniotes", "羊膜动物", []), // 23
    node!(Some(23), "単弓類", "Synapsids", "合弓类", ["archaeothyris", "dimetrodon", "inostrancevia", "lystrosaurus", "thrinaxodon"]), // 24
    node!(Some(24), "哺乳類と近縁群", "Mammals + close relatives", "哺乳动物及近亲", ["megazostrodon", "castorocauda", "repenomamus", "purgatorius", "basilosaurus", "paraceratherium", "deinotherium", "hipparion", "australopithecus", "woolly-mammoth", "smilodon", "homo-sapiens"]), // 25
    node!(Some(23), "爬虫類（鳥類を含む）", "Reptiles incl. birds", "爬行动物（含鸟类）", ["hylonomus", "mesosaurus", "scutosaurus", "mixosaurus", "plesiosaurus", "tylosaurus", "cenozoic-lizards"]), // 26
    node!(Some(26), "主竜類と近縁群", "Archosaurs + close relatives", "主龙类及近亲", ["postosuchus"]), // 27
    node!(Some(27), "翼竜", "Pterosaurs", "翼龙", ["eudimorphodon", "rhamphorhynchus", "pteranodon"]), // 28
    node!(Some(27), "恐竜", "Dinosaurs", "恐龙", ["plateosaurus", "stegosaurus", "diplodocus", "triceratops"]), // 29
    node!(Some(29), "獣脚類", "Theropods", "兽脚类", ["coelophysis", "allosaurus", "tyrannosaurus", "microraptor"]), // 30
    node!(Some(30), "鳥類と初期の近縁群", "Birds + early relatives", "鸟类及早期近亲", ["archaeopteryx", "confuciusornis", "gastornis", "neogene-birds", "quaternary-birds"]), // 31
];

pub fn node_for_entry(e: usize) -> Option<usize> {
    let sp = SPECIES.get(e)?;
    NODES.iter().position(|n| n.examples.contains(&sp.slug))
}

pub fn examples(node: usize, period: usize) -> Vec<usize> {
    let p = &PERIODS[period];
    NODES[node].examples.iter().filter_map(|slug| {
        SPECIES.iter().position(|sp| sp.slug == *slug && sp.t0 > p.t1 && sp.t1 < p.t0)
    }).collect()
}

pub fn on_path(ancestor: usize, mut node: usize) -> bool {
    loop {
        if ancestor == node { return true; }
        match NODES[node].parent { Some(parent) => node = parent, None => return false }
    }
}

pub const PRELUDE: f32 = 1100.0;
pub const PERIOD_HEIGHT: f32 = 460.0;
pub const CARD_W: f32 = 264.0;
pub const CARD_H: f32 = 112.0;

/// Equal-height periods keep short recent intervals navigable. Within each
/// band ages are linear. The header above the Cambrian is intentionally not dated.
pub fn age_y(age: f32) -> f32 {
    for (i, p) in PERIODS.iter().enumerate() {
        if age >= p.t1 {
            return PRELUDE + (i as f32 + ((p.t0 - age) / (p.t0 - p.t1)).clamp(0.0, 1.0)) * PERIOD_HEIGHT;
        }
    }
    PRELUDE + PERIODS.len() as f32 * PERIOD_HEIGHT
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub entry: usize,
    pub node: usize,
    pub x: f32,
    pub y: f32,
    pub start: f32,
    pub end: f32,
}

pub struct Map {
    pub nodes: Vec<(f32, f32)>,
    pub cards: Vec<Card>,
    pub width: f32,
    pub height: f32,
}

impl Map {
    pub fn new() -> Self {
        let mut result = Self { nodes: vec![(0.0, 0.0); NODES.len()], cards: Vec::new(), width: 0.0, height: PRELUDE + PERIODS.len() as f32 * PERIOD_HEIGHT + 100.0 };
        let mut columns = vec![0; NODES.len()];
        for (n, node) in NODES.iter().enumerate() {
            let mut entries: Vec<_> = node.examples.iter().map(|slug| SPECIES.iter().position(|s| s.slug == *slug).expect("tree example exists")).collect();
            entries.sort_by(|&a, &b| age_y((SPECIES[a].t0 + SPECIES[a].t1) * 0.5).total_cmp(&age_y((SPECIES[b].t0 + SPECIES[b].t1) * 0.5)));
            let mut ends = Vec::new();
            for entry in entries {
                let sp = &SPECIES[entry];
                let y = age_y((sp.t0 + sp.t1) * 0.5) - CARD_H * 0.5;
                let column = ends.iter().position(|&end| end + 26.0 <= y).unwrap_or(ends.len());
                if column == ends.len() { ends.push(y + CARD_H); } else { ends[column] = y + CARD_H; }
                result.cards.push(Card { entry, node: n, x: column as f32 * 300.0, y, start: age_y(sp.t0), end: age_y(sp.t1) });
            }
            columns[n] = ends.len();
        }
        fn place(n: usize, cursor: &mut f32, columns: &[usize], map: &mut Map) {
            let left = *cursor;
            for card in map.cards.iter_mut().filter(|c| c.node == n) { card.x += left; }
            *cursor += columns[n] as f32 * 300.0;
            for child in 0..NODES.len() {
                if NODES[child].parent == Some(n) { place(child, cursor, columns, map); }
            }
            map.nodes[n].0 = (left + *cursor - 36.0) * 0.5;
        }
        let mut cursor = 120.0;
        place(0, &mut cursor, &columns, &mut result);
        result.width = cursor + 120.0;
        for n in (0..NODES.len()).rev() {
            let earliest_card = result.cards.iter().filter(|c| c.node == n).map(|c| c.start).fold(f32::INFINITY, f32::min);
            let earliest_child = NODES.iter().enumerate().filter(|(_, child)| child.parent == Some(n)).map(|(child, _)| result.nodes[child].1).fold(f32::INFINITY, f32::min);
            result.nodes[n].1 = earliest_card.min(earliest_child) - 72.0;
        }
        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Self { Self { x: 0.0, y: 0.0, zoom: 0.0 } }

    pub fn fit(&mut self, map: &Map, width: f32, height: f32) {
        self.x = map.width * 0.5;
        self.y = map.height * 0.5;
        self.zoom = (width / map.width).min(height / map.height) * 0.94;
    }

    pub fn clamp(&mut self, map: &Map, width: f32, height: f32) {
        let min = (width / map.width).min(height / map.height) * 0.94;
        self.zoom = self.zoom.clamp(min, 2.4);
        fn bound(center: f32, size: f32, extent: f32) -> f32 {
            if size >= extent { extent * 0.5 } else { center.clamp(size * 0.5, extent - size * 0.5) }
        }
        self.x = bound(self.x, width / self.zoom, map.width);
        self.y = bound(self.y, height / self.zoom, map.height);
    }

    /// Anchor-preserving zoom. Coordinates are relative to the viewport centre.
    pub fn zoom_at(&mut self, factor: f32, x: f32, y: f32, map: &Map, width: f32, height: f32) {
        let before = (self.x + x / self.zoom, self.y + y / self.zoom);
        self.zoom *= factor;
        self.clamp(map, width, height);
        self.x = before.0 - x / self.zoom;
        self.y = before.1 - y / self.zoom;
        self.clamp(map, width, height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn atlas_layout_orders_time_and_branches_without_overlapping_cards() {
        let map = Map::new();
        assert!(map.width > 4000.0 && map.height > 6000.0);
        for (n, &(x, y)) in map.nodes.iter().enumerate() {
            assert!(x > 0.0 && x < map.width && y > 0.0 && y < map.height, "node {n}: {x},{y}");
            if let Some(parent) = NODES[n].parent { assert!(map.nodes[parent].1 < y); }
        }
        assert_eq!(map.cards.len(), NODES.iter().map(|n| n.examples.len()).sum::<usize>());
        for (i, a) in map.cards.iter().enumerate() {
            assert!(a.start <= a.y + CARD_H * 0.5 && a.y + CARD_H * 0.5 <= a.end);
            assert!(map.nodes[a.node].1 < a.start);
            for b in &map.cards[i + 1..] {
                assert!(a.x + CARD_W <= b.x || b.x + CARD_W <= a.x || a.y + CARD_H <= b.y || b.y + CARD_H <= a.y, "overlap {} / {}", a.entry, b.entry);
            }
        }
        for (i, p) in PERIODS.iter().enumerate() {
            assert!((age_y(p.t0) - (PRELUDE + i as f32 * PERIOD_HEIGHT)).abs() < 0.01);
            assert!((age_y(p.t1) - (PRELUDE + (i + 1) as f32 * PERIOD_HEIGHT)).abs() < 0.01);
        }
    }

    #[test]
    fn atlas_zoom_keeps_its_anchor_and_camera_stays_in_bounds() {
        let map = Map::new();
        let mut camera = Camera::new();
        camera.fit(&map, 1100.0, 600.0);
        let fit = camera;
        camera.zoom = 0.6;
        let anchor = (camera.x + 180.0 / camera.zoom, camera.y - 100.0 / camera.zoom);
        camera.zoom_at(1.7, 180.0, -100.0, &map, 1100.0, 600.0);
        assert!((camera.x + 180.0 / camera.zoom - anchor.0).abs() < 0.01);
        assert!((camera.y - 100.0 / camera.zoom - anchor.1).abs() < 0.01);
        camera.zoom_at(1000.0, 0.0, 0.0, &map, 1100.0, 600.0);
        assert_eq!(camera.zoom, 2.4);
        camera.x = -100000.0;
        camera.y = 100000.0;
        camera.clamp(&map, 1100.0, 600.0);
        assert!(camera.x - 550.0 / camera.zoom >= -0.01);
        assert!(camera.y + 300.0 / camera.zoom <= map.height + 0.01);
        camera.zoom_at(0.00001, 0.0, 0.0, &map, 1100.0, 600.0);
        assert_eq!(camera, fit);
    }

    #[test]
    fn tree_examples_are_unique_existing_entries_and_cover_every_period() {
        let mut seen = std::collections::HashSet::new();
        for (i, node) in NODES.iter().enumerate() {
            if let Some(parent) = node.parent { assert!(parent < i); }
            for slug in node.examples {
                assert!(seen.insert(slug), "duplicate {slug}");
                assert!(SPECIES.iter().any(|sp| sp.slug == *slug), "missing {slug}");
            }
        }
        for p in 0..PERIODS.len() {
            assert!(NODES.iter().enumerate().map(|(n, _)| examples(n, p).len()).sum::<usize>() >= 4);
        }
        let bird = node_for_entry(SPECIES.iter().position(|s| s.slug == "archaeopteryx").unwrap()).unwrap();
        assert!(on_path(29, bird));
        assert!(!on_path(28, bird));
        assert!(on_path(24, 25));
        assert!(!on_path(29, 25));
    }
}
