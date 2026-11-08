//! A deliberately pruned cladogram. Fossils are examples on branches, never
//! asserted to be direct ancestors. Branch lengths do not encode time.
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

#[cfg(test)]
mod tests {
    use super::*;
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
