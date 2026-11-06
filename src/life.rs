//! カンブリア紀の生きもの（図鑑の項目）と、上空から撮る現象
//!
//! 時代（百万年前）・すみか・産地はおおよそ。体の大きさは代表的な化石の長さ。

use crate::earth::*;
use crate::lang::L;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Region {
    /// 化石の産地の近く
    Site(usize),
    /// その大陸のまわり
    Plate(usize),
    /// 暖かい海（緯度 35 度まで）
    Tropics,
    /// 世界中
    Global,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Beh {
    /// 水の中を泳ぐ
    Swim,
    /// 海底の少し上を泳ぐ
    NearFloor,
    /// 海底を歩く・這う
    Crawl,
    /// 海底に固着して動かない
    Sessile,
    /// 海底の巣穴から顔を出す（近づくと引っ込む）
    Burrow,
    /// 水の中を漂う
    Drift,
    /// 干潟を這う
    FlatCrawl,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Art {
    /// 放射歯類 0=アノマロカリス 1=アンプレクトベルア 2=タミシオカリス 3=カンブロラスター
    Radiodont(u8),
    /// 三葉虫（幅、頬棘の長さ、尾の大きさ、胸の節の数）
    Trilobite { w: f32, spine: f32, pyg: f32, segs: u8 },
    Agnostus,
    Opabinia,
    Hallucigenia,
    Aysheaia,
    Kerygmachela,
    Wiwaxia,
    Halkieria,
    Marrella,
    Leanchoilia,
    Fuxianhuia,
    Sidneyia,
    Isoxys,
    Nectocaris,
    /// 魚・脊索動物 0=ハイコウイクチス 1=ピカイア 2=コノドント
    Fish(u8),
    Jelly,
    Radiolarian,
    /// 海綿 0=バウキシア（枝分かれ） 1=ガラス海綿（つぼ形）
    Sponge(u8),
    Archaeocyath,
    Stromatolite,
    Mat,
    Tubes,
    Brachiopod,
    Worm,
    Trace,
    Slug,
}

impl Art {
    /// 海底に平たく置いて斜め上から見る描き方か
    pub fn dorsal(self) -> bool {
        matches!(self, Art::Trilobite { .. } | Art::Agnostus | Art::Wiwaxia | Art::Halkieria | Art::Fuxianhuia | Art::Sidneyia | Art::Slug | Art::Trace | Art::Mat)
    }
}

pub struct Species {
    pub name: L,
    pub sci: &'static str,
    pub group: L,
    /// 体長（cm）
    pub len_cm: f32,
    /// 生きていた時代（百万年前、古い方から）
    pub t0: f32,
    pub t1: f32,
    pub hab: u8,
    pub regions: &'static [Region],
    pub beh: Beh,
    pub art: Art,
    pub col: [f32; 3],
    pub col2: [f32; 3],
    /// 1場面あたりの数の目安
    pub rarity: f32,
    /// この距離（m）より近づくと逃げる・隠れる
    pub shy: f32,
    /// 泳ぐ・歩く速さ（m/秒）
    pub speed: f32,
    pub desc: L,
    pub fact: L,
}

impl Species {
    /// 画面に描く長さ（m）。小さな生きものほど大きめに描いて見つけやすくする
    pub fn disp_len(&self) -> f32 {
        (0.40 * (self.len_cm / 25.0).powf(0.38)).max(0.11)
    }
    pub fn alive_at(&self, t: f32) -> bool {
        t <= self.t0 + 1e-3 && t >= self.t1 - 1e-3
    }
}

use Region::*;

pub static SPECIES: &[Species] = &[
    Species {
        name: L("ストロマトライト", "Stromatolite", "叠层石"),
        sci: "Stromatolite",
        group: L("シアノバクテリアがつくる岩", "Rock built by cyanobacteria", "蓝细菌建造的岩石"),
        len_cm: 90.0,
        t0: T_START,
        t1: T_END,
        hab: TIDAL | SHALLOW,
        regions: &[Global],
        beh: Beh::Sessile,
        art: Art::Stromatolite,
        col: [0.62, 0.56, 0.42],
        col2: [0.30, 0.48, 0.30],
        rarity: 5.0,
        shy: 0.0,
        speed: 0.0,
        desc: L(
            "光合成をするシアノバクテリアが、泥や砂を少しずつ貼り付けて層状に積み上げたドーム。35億年前から地球に酸素を送り続けてきた。",
            "Layered domes built as photosynthetic cyanobacteria trap mud and sand. They have been pumping oxygen into the air for 3.5 billion years.",
            "进行光合作用的蓝细菌一点点黏附泥沙、层层堆积成的圆丘。35亿年来一直为地球输送氧气。",
        ),
        fact: L(
            "カンブリア紀に動物がふえると、食べられたり掘り返されたりして数が減っていった。",
            "As animals multiplied in the Cambrian, grazing and burrowing made stromatolites rarer.",
            "寒武纪动物增多后，叠层石因被啃食和翻掘而逐渐减少。",
        ),
    },
    Species {
        name: L("微生物マット", "Microbial mat", "微生物席"),
        sci: "Microbial mat",
        group: L("微生物の群れ", "Microbial community", "微生物群落"),
        len_cm: 40.0,
        t0: T_START,
        t1: T_END,
        hab: LAND | TIDAL,
        regions: &[Global],
        beh: Beh::Sessile,
        art: Art::Mat,
        col: [0.38, 0.44, 0.26],
        col2: [0.52, 0.46, 0.30],
        rarity: 6.0,
        shy: 0.0,
        speed: 0.0,
        desc: L(
            "陸にはまだ植物がない。湿った岩や干潟の表面を、細菌や藻類が薄いじゅうたんのように覆っていた。",
            "There are no plants on land yet. Bacteria and algae coat damp rock and tidal flats like a thin carpet.",
            "陆地上还没有植物。细菌和藻类像一层薄地毯一样覆盖着潮湿的岩石和潮坪。",
        ),
        fact: L("陸の上の生命は、ほとんどこの薄い膜だけだった。", "Life on land was almost nothing but this thin film.", "陆地上的生命几乎只有这层薄膜。"),
    },
    Species {
        name: L("トレプティクヌス（生痕）", "Treptichnus (trace)", "旋迹（遗迹）"),
        sci: "Treptichnus pedum",
        group: L("生痕化石（巣穴の跡）", "Trace fossil (burrow)", "遗迹化石（潜穴）"),
        len_cm: 12.0,
        t0: T_START,
        t1: 520.0,
        hab: SHALLOW | SHELF,
        regions: &[Site(S_FORTUNE), Global],
        beh: Beh::Sessile,
        art: Art::Trace,
        col: [0.36, 0.30, 0.24],
        col2: [0.80, 0.56, 0.52],
        rarity: 3.0,
        shy: 0.0,
        speed: 0.0,
        desc: L(
            "鰓曳動物のような虫が、泥の中を枝分かれしながら掘り進んだ跡。この巣穴の化石が現れた時がカンブリア紀の始まりと決められている。",
            "Branching burrows dug through mud by a priapulid-like worm. The first appearance of this trace fossil defines the start of the Cambrian.",
            "类似鳃曳动物的蠕虫在泥中分叉挖掘留下的痕迹。这种潜穴化石的出现被定为寒武纪的开始。",
        ),
        fact: L(
            "基準の地層はカナダ・ニューファンドランド島のフォーチュン岬にある。",
            "The reference section for this boundary is at Fortune Head, Newfoundland, Canada.",
            "该界线的标准剖面位于加拿大纽芬兰岛的福琼角。",
        ),
    },
    Species {
        name: L("アナバリテス", "Anabarites", "阿纳巴管"),
        sci: "Anabarites trisulcatus",
        group: L("微小有殻化石", "Small shelly fossil", "小壳化石"),
        len_cm: 0.6,
        t0: T_START,
        t1: 520.0,
        hab: SHALLOW,
        regions: &[Global],
        beh: Beh::Sessile,
        art: Art::Tubes,
        col: [0.84, 0.80, 0.70],
        col2: [0.60, 0.52, 0.44],
        rarity: 3.0,
        shy: 0.0,
        speed: 0.0,
        desc: L(
            "断面が三つ葉の形をした、数ミリの小さな管。カンブリア紀のはじめ、硬い殻を作る動物が世界中で現れたことを示す。",
            "A tiny tube a few millimetres long with a three-lobed cross-section. It shows that animals building hard skeletons appeared worldwide in the earliest Cambrian.",
            "截面呈三叶形、仅几毫米长的小管。表明寒武纪初期能造硬壳的动物在全球出现。",
        ),
        fact: L("中に住んでいた動物の正体はまだ分かっていない。", "Nobody knows yet what animal lived inside.", "居住其中的动物至今不明。"),
    },
    Species {
        name: L("古杯類", "Archaeocyath", "古杯动物"),
        sci: "Archaeocyatha",
        group: L("海綿動物の仲間", "Sponge relative", "海绵动物"),
        len_cm: 10.0,
        t0: 525.0,
        t1: 509.0,
        hab: SHALLOW,
        regions: &[Tropics],
        beh: Beh::Sessile,
        art: Art::Archaeocyath,
        col: [0.86, 0.72, 0.56],
        col2: [0.70, 0.46, 0.40],
        rarity: 7.0,
        shy: 0.0,
        speed: 0.0,
        desc: L(
            "穴の開いた二重の壁を持つ、杯の形の海綿の仲間。動物として最初に「礁」をつくり、暖かい浅い海に群れていた。",
            "Cup-shaped sponge relatives with double, perforated walls. They built the first animal reefs in warm shallow seas.",
            "具有带孔双层壁的杯状海绵类。它们是最早建造“礁”的动物，成群生活在温暖浅海中。",
        ),
        fact: L(
            "シンスク事変のあと急に減り、カンブリア紀の中ごろにはほとんど姿を消した。",
            "They crashed after the Sinsk event and had nearly vanished by the middle Cambrian.",
            "辛斯克事件后急剧减少，到寒武纪中期几乎消失。",
        ),
    },
    Species {
        name: L("リンギュレラ", "Lingulella", "小舌形贝"),
        sci: "Lingulella",
        group: L("腕足動物", "Brachiopod", "腕足动物"),
        len_cm: 1.5,
        t0: 525.0,
        t1: T_END,
        hab: SHALLOW | SHELF,
        regions: &[Global],
        beh: Beh::Sessile,
        art: Art::Brachiopod,
        col: [0.42, 0.52, 0.48],
        col2: [0.90, 0.74, 0.56],
        rarity: 3.0,
        shy: 0.0,
        speed: 0.0,
        desc: L(
            "二枚の殻を持つが、貝ではなく腕足動物。肉の茎で海底に体を固定し、殻を開いて水の中の餌をこし取る。",
            "Two shells, but not a clam: a brachiopod. It anchors itself with a fleshy stalk and opens its shells to filter food.",
            "有两片壳，却不是贝类，而是腕足动物。用肉茎固定在海底，张开壳滤食水中的食物。",
        ),
        fact: L(
            "よく似た仲間のシャミセンガイは、いまも日本の干潟に暮らしている。",
            "Its close relative Lingula still lives on tidal flats today.",
            "与之相似的海豆芽至今仍生活在潮滩上。",
        ),
    },
    Species {
        name: L("レドリキア", "Redlichia", "莱得利基虫"),
        sci: "Redlichia",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 6.0,
        t0: 521.0,
        t1: 509.0,
        hab: SHALLOW | SHELF,
        regions: &[Site(S_EMU), Plate(GONDWANA), Plate(SOUTH_CHINA)],
        beh: Beh::Crawl,
        art: Art::Trilobite { w: 0.62, spine: 0.34, pyg: 0.05, segs: 13 },
        col: [0.56, 0.44, 0.34],
        col2: [0.34, 0.26, 0.20],
        rarity: 3.0,
        shy: 0.0,
        speed: 0.08,
        desc: L(
            "初期の三葉虫。半月形の頭に三日月形の大きな眼と長い頬の棘を持ち、尾はとても小さい。",
            "An early trilobite with a half-moon head, big crescent eyes, long cheek spines and a tiny tail.",
            "早期三叶虫。半月形的头部有新月形大眼和长长的颊刺，尾部很小。",
        ),
        fact: L(
            "オーストラリアのエミュー湾では、体長25cmにもなる大型種が見つかっている。",
            "At Emu Bay in Australia a giant species reached 25 cm.",
            "在澳大利亚鸸鹋湾发现了体长可达25厘米的大型种。",
        ),
    },
    Species {
        name: L("ハルキエリア", "Halkieria", "哈氏虫"),
        sci: "Halkieria evangelista",
        group: L("軟体動物に近い仲間", "Mollusc relative", "软体动物近亲"),
        len_cm: 6.0,
        t0: 525.0,
        t1: 513.0,
        hab: SHALLOW | SHELF,
        regions: &[Site(S_SIRIUS), Plate(SIBERIA), Plate(SOUTH_CHINA)],
        beh: Beh::Crawl,
        art: Art::Halkieria,
        col: [0.60, 0.54, 0.40],
        col2: [0.82, 0.76, 0.58],
        rarity: 1.6,
        shy: 0.0,
        speed: 0.04,
        desc: L(
            "背中を鎖かたびらのような小さな骨片で覆い、頭とおしりに一枚ずつ殻をのせたナメクジのような動物。",
            "A slug-like animal covered in chain-mail sclerites, with one shell at the front and one at the back.",
            "像鼻涕虫一样的动物，背上覆盖着锁子甲般的小骨片，头尾各有一块壳。",
        ),
        fact: L("グリーンランドのシリウス・パセットで、体全体の化石が見つかった。", "Complete specimens were found at Sirius Passet in Greenland.", "在格陵兰的天狼星帕塞特发现了完整的化石。"),
    },
    Species {
        name: L("ハイコウイクチス", "Haikouichthys", "海口鱼"),
        sci: "Haikouichthys ercaicunensis",
        group: L("脊椎動物（最古の魚の仲間）", "Vertebrate (early fish)", "脊椎动物（最古老的鱼类）"),
        len_cm: 2.5,
        t0: 520.0,
        t1: 515.0,
        hab: SHELF | SHALLOW,
        regions: &[Site(S_CHENGJIANG)],
        beh: Beh::Swim,
        art: Art::Fish(0),
        col: [0.72, 0.74, 0.80],
        col2: [0.46, 0.50, 0.62],
        rarity: 2.5,
        shy: 2.2,
        speed: 0.9,
        desc: L(
            "背骨の始まりと、頭・眼・えらを持つ、最も古い魚の仲間のひとつ。あごもひれの骨もまだない。",
            "One of the oldest fish-like vertebrates, with the beginnings of a backbone, a head, eyes and gills, but no jaws.",
            "最古老的鱼类之一，有原始脊柱、头、眼和鳃，但还没有颌。",
        ),
        fact: L("わたしたち人間につながる、遠いご先祖さまの仲間。", "A distant cousin of our own ancestors.", "与我们人类相连的远古祖先的亲戚。"),
    },
    Species {
        name: L("フーシャンフイア", "Fuxianhuia", "抚仙湖虫"),
        sci: "Fuxianhuia protensa",
        group: L("節足動物", "Arthropod", "节肢动物"),
        len_cm: 10.0,
        t0: 520.0,
        t1: 515.0,
        hab: SHELF | SHALLOW,
        regions: &[Site(S_CHENGJIANG)],
        beh: Beh::Crawl,
        art: Art::Fuxianhuia,
        col: [0.62, 0.48, 0.38],
        col2: [0.42, 0.30, 0.24],
        rarity: 2.0,
        shy: 0.0,
        speed: 0.07,
        desc: L(
            "昆虫やエビの遠い親戚。化石に脳や神経まで残っていて、節足動物の頭の進化を教えてくれる。",
            "A distant relative of insects and shrimp. Fossils even preserve its brain and nerves, revealing how arthropod heads evolved.",
            "昆虫和虾的远亲。化石中连脑和神经都保存了下来，揭示了节肢动物头部的演化。",
        ),
        fact: L("子どもの世話をしていた化石も見つかっている。", "A fossil shows an adult caring for its young.", "还发现了照顾幼体的化石。"),
    },
    Species {
        name: L("イソキシス", "Isoxys", "等刺虫"),
        sci: "Isoxys",
        group: L("節足動物", "Arthropod", "节肢动物"),
        len_cm: 4.0,
        t0: 520.0,
        t1: 510.0,
        hab: SHELF | SHALLOW,
        regions: &[Site(S_CHENGJIANG), Site(S_SIRIUS), Site(S_EMU)],
        beh: Beh::Swim,
        art: Art::Isoxys,
        col: [0.80, 0.70, 0.52],
        col2: [0.30, 0.24, 0.20],
        rarity: 2.5,
        shy: 0.8,
        speed: 0.45,
        desc: L(
            "二枚貝のような殻に体を包み、大きな眼で獲物を探して泳いだ節足動物。殻の前後に長い棘がある。",
            "An arthropod wrapped in a clam-like shell, swimming with large eyes to find prey. Long spines stick out at both ends.",
            "身体包在双壳般的甲壳中、用大眼睛寻找猎物游泳的节肢动物。壳的前后有长刺。",
        ),
        fact: L("中国・グリーンランド・オーストラリアと、世界の海で見つかる。", "Found in China, Greenland and Australia, all over the world's seas.", "在中国、格陵兰、澳大利亚等世界各地都有发现。"),
    },
    Species {
        name: L("レアンコイリア", "Leanchoilia", "林乔利虫"),
        sci: "Leanchoilia",
        group: L("節足動物（大付属肢類）", "Arthropod (great appendage)", "节肢动物（大附肢类）"),
        len_cm: 5.0,
        t0: 520.0,
        t1: 505.0,
        hab: SHELF,
        regions: &[Site(S_CHENGJIANG), Site(S_BURGESS)],
        beh: Beh::NearFloor,
        art: Art::Leanchoilia,
        col: [0.58, 0.64, 0.74],
        col2: [0.82, 0.86, 0.92],
        rarity: 1.6,
        shy: 0.6,
        speed: 0.3,
        desc: L(
            "頭の前に、先が長いムチになった大きな付属肢を一対持つ。ムチで周りを探りながら海底近くを泳いだ。",
            "It carries a pair of great appendages ending in long whips, feeling its way as it swims near the sea floor.",
            "头前有一对末端是长鞭的大附肢，一边用鞭子探索周围一边在海底附近游动。",
        ),
        fact: L("クモやサソリの遠い親戚かもしれない。", "It may be a distant relative of spiders and scorpions.", "它可能是蜘蛛和蝎子的远亲。"),
    },
    Species {
        name: L("ハルキゲニア", "Hallucigenia", "怪诞虫"),
        sci: "Hallucigenia",
        group: L("葉足動物", "Lobopodian", "叶足动物"),
        len_cm: 3.0,
        t0: 520.0,
        t1: 505.0,
        hab: SHELF,
        regions: &[Site(S_CHENGJIANG), Site(S_BURGESS)],
        beh: Beh::Crawl,
        art: Art::Hallucigenia,
        col: [0.84, 0.56, 0.44],
        col2: [0.94, 0.88, 0.76],
        rarity: 2.2,
        shy: 0.0,
        speed: 0.03,
        desc: L(
            "背中に7対の棘、おなかに細い脚を並べた不思議な動物。名前は「夢うつつの幻」という意味。",
            "A strange animal with seven pairs of spines on its back and slender legs beneath. Its name means 'dream-like hallucination'.",
            "背上长着7对刺、腹部排列着细腿的奇异动物。名字意为“如梦似幻”。",
        ),
        fact: L(
            "はじめは上下も前後も逆さまに復元されていた。カギムシの遠い親戚。",
            "It was first reconstructed upside down and back to front. A distant relative of velvet worms.",
            "最初被复原时上下前后都弄反了。是栉蚕的远亲。",
        ),
    },
    Species {
        name: L("アンプレクトベルア", "Amplectobelua", "抱怪虫"),
        sci: "Amplectobelua symbrachiata",
        group: L("放射歯類", "Radiodont", "放射齿类"),
        len_cm: 25.0,
        t0: 520.0,
        t1: 515.0,
        hab: SHELF | SHALLOW,
        regions: &[Site(S_CHENGJIANG)],
        beh: Beh::Swim,
        art: Art::Radiodont(1),
        col: [0.70, 0.40, 0.34],
        col2: [0.90, 0.62, 0.42],
        rarity: 1.0,
        shy: 0.0,
        speed: 0.6,
        desc: L(
            "アノマロカリスの仲間。頭のはさみのような付属肢で獲物をつかむ。体の横のひれを波打たせて泳ぐ。",
            "A relative of Anomalocaris that grasps prey with pincer-like head appendages, swimming by rippling its side flaps.",
            "奇虾的近亲。用头部钳子般的附肢抓住猎物，摆动身体两侧的桨叶游泳。",
        ),
        fact: L("澄江の海では最大級の捕食者だった。", "One of the largest predators in the Chengjiang sea.", "澄江海中最大的捕食者之一。"),
    },
    Species {
        name: L("ケリグマケラ", "Kerygmachela", "凯里格马虫"),
        sci: "Kerygmachela kierkegaardi",
        group: L("葉足動物（放射歯類に近い）", "Gilled lobopodian", "叶足动物（近放射齿类）"),
        len_cm: 17.0,
        t0: 520.0,
        t1: 514.0,
        hab: SHELF,
        regions: &[Site(S_SIRIUS)],
        beh: Beh::Swim,
        art: Art::Kerygmachela,
        col: [0.66, 0.60, 0.48],
        col2: [0.48, 0.40, 0.32],
        rarity: 1.2,
        shy: 0.0,
        speed: 0.4,
        desc: L(
            "脚とひれを両方持つ、葉足動物とアノマロカリスの間のような動物。長い2本の尾と、棘のある長い腕を持つ。",
            "Halfway between lobopodians and Anomalocaris, with both legs and flaps, two long tail spines and long spiny arms.",
            "兼具腿和桨叶，介于叶足动物和奇虾之间的动物。有两根长尾和带刺的长臂。",
        ),
        fact: L("学名は哲学者キルケゴールにちなむ。", "Its species name honours the philosopher Kierkegaard.", "种名取自哲学家克尔凯郭尔。"),
    },
    Species {
        name: L("タミシオカリス", "Tamisiocaris", "筛虾"),
        sci: "Tamisiocaris borealis",
        group: L("放射歯類", "Radiodont", "放射齿类"),
        len_cm: 30.0,
        t0: 520.0,
        t1: 514.0,
        hab: SHELF,
        regions: &[Site(S_SIRIUS)],
        beh: Beh::Swim,
        art: Art::Radiodont(2),
        col: [0.54, 0.56, 0.62],
        col2: [0.84, 0.86, 0.88],
        rarity: 0.9,
        shy: 0.0,
        speed: 0.35,
        desc: L(
            "アノマロカリスの仲間なのに、くしのような付属肢でプランクトンをこし取って食べた。のんびり泳ぐ「海の掃除機」。",
            "A radiodont that sieved plankton with comb-like appendages instead of hunting: a gentle filter feeder.",
            "虽是奇虾的近亲，却用梳子般的附肢滤食浮游生物，是悠闲游动的“海中吸尘器”。",
        ),
        fact: L("のちのクジラのように、大きな体でプランクトンを食べる戦略の先がけ。", "An early version of the big filter-feeder strategy later used by whales.", "就像后来的鲸一样，开创了大型滤食者的生存策略。"),
    },
    Species {
        name: L("アノマロカリス", "Anomalocaris", "奇虾"),
        sci: "Anomalocaris canadensis",
        group: L("放射歯類", "Radiodont", "放射齿类"),
        len_cm: 38.0,
        t0: 515.0,
        t1: 505.0,
        hab: SHELF | SHALLOW,
        regions: &[Site(S_EMU), Site(S_BURGESS)],
        beh: Beh::Swim,
        art: Art::Radiodont(0),
        col: [0.80, 0.38, 0.30],
        col2: [0.96, 0.66, 0.44],
        rarity: 1.0,
        shy: 0.0,
        speed: 0.8,
        desc: L(
            "カンブリア紀の海の王者。棘の並んだ一対の大きな腕で獲物を捕らえ、円い口へ運ぶ。体の横のひれを波打たせて泳いだ。",
            "King of the Cambrian seas. It seized prey with a pair of spiny arms and brought it to a round mouth, swimming with rippling side flaps.",
            "寒武纪海洋的霸主。用一对排满刺的大臂捕捉猎物送入圆形的口中，摆动身体两侧的桨叶游泳。",
        ),
        fact: L(
            "名前は「奇妙なエビ」という意味。最初は腕だけが見つかり、エビの体だと思われた。",
            "The name means 'odd shrimp': its arm was found first and mistaken for a shrimp's body.",
            "名字意为“奇怪的虾”。最初只发现了它的附肢，被误认为是虾的身体。",
        ),
    },
    Species {
        name: L("オパビニア", "Opabinia", "欧巴宾海蝎"),
        sci: "Opabinia regalis",
        group: L("放射歯類に近い仲間", "Radiodont relative", "放射齿类近亲"),
        len_cm: 7.0,
        t0: 510.0,
        t1: 505.0,
        hab: SHELF,
        regions: &[Site(S_BURGESS)],
        beh: Beh::NearFloor,
        art: Art::Opabinia,
        col: [0.74, 0.56, 0.48],
        col2: [0.86, 0.34, 0.30],
        rarity: 0.9,
        shy: 0.7,
        speed: 0.3,
        desc: L(
            "5つの眼と、先にはさみのついた長いホースのような口を持つ。学会で初めて復元図が示されたとき、会場は笑いに包まれた。",
            "Five eyes and a long, hose-like snout tipped with a claw. When first shown at a scientific meeting, the audience laughed.",
            "有5只眼睛和一根末端带钳的长管状吻。复原图首次在学会上展示时，全场哄堂大笑。",
        ),
        fact: L("バージェスでしか見つかっていない。", "It is known only from the Burgess Shale.", "只在伯吉斯页岩中发现过。"),
    },
    Species {
        name: L("ウィワクシア", "Wiwaxia", "威瓦亚虫"),
        sci: "Wiwaxia corrugata",
        group: L("軟体動物に近い仲間", "Mollusc relative", "软体动物近亲"),
        len_cm: 3.5,
        t0: 515.0,
        t1: 505.0,
        hab: SHELF,
        regions: &[Site(S_BURGESS)],
        beh: Beh::Crawl,
        art: Art::Wiwaxia,
        col: [0.50, 0.46, 0.40],
        col2: [0.76, 0.70, 0.54],
        rarity: 2.2,
        shy: 0.0,
        speed: 0.03,
        desc: L(
            "うろこ状の骨片で体を覆い、背中に2列の長い棘を立てて身を守った。海底の微生物をなめて食べていた。",
            "Covered in scale-like sclerites, with two rows of tall spines on its back for defence. It licked microbes off the sea floor.",
            "全身覆盖鳞片状骨片，背上竖着两排长刺保护自己。舔食海底的微生物。",
        ),
        fact: L("棘の多くに、かみ跡のような傷が残る化石もある。", "Some fossils have broken spines, maybe from attacks.", "有些化石的刺上留有像咬痕一样的伤。"),
    },
    Species {
        name: L("マーレラ", "Marrella", "马尔三叶形虫"),
        sci: "Marrella splendens",
        group: L("節足動物", "Arthropod", "节肢动物"),
        len_cm: 2.0,
        t0: 510.0,
        t1: 505.0,
        hab: SHELF,
        regions: &[Site(S_BURGESS)],
        beh: Beh::NearFloor,
        art: Art::Marrella,
        col: [0.80, 0.70, 0.86],
        col2: [0.40, 0.76, 0.86],
        rarity: 6.0,
        shy: 0.5,
        speed: 0.25,
        desc: L(
            "頭から後ろへ2対の大きな棘がのびる小さな節足動物。バージェスで最もたくさん見つかる動物で、「レースガニ」とも呼ばれる。",
            "A small arthropod with two pairs of big spines sweeping back from its head. The most common Burgess animal, nicknamed the 'lace crab'.",
            "头部向后伸出两对大刺的小型节肢动物。是伯吉斯最常见的动物，又称“蕾丝蟹”。",
        ),
        fact: L("棘の表面の細かな筋が光を虹色に反射していた可能性がある。", "Fine ridges on its spines may have shimmered in rainbow colours.", "刺表面的细纹可能会反射出彩虹般的光泽。"),
    },
    Species {
        name: L("ピカイア", "Pikaia", "皮卡虫"),
        sci: "Pikaia gracilens",
        group: L("脊索動物", "Chordate", "脊索动物"),
        len_cm: 4.0,
        t0: 510.0,
        t1: 505.0,
        hab: SHELF,
        regions: &[Site(S_BURGESS)],
        beh: Beh::Swim,
        art: Art::Fish(1),
        col: [0.86, 0.80, 0.70],
        col2: [0.66, 0.58, 0.50],
        rarity: 1.6,
        shy: 2.4,
        speed: 0.8,
        desc: L(
            "体の中に背骨のもとになる脊索を持つ、ナメクジウオに似た動物。体をくねらせて泳いだ。",
            "A lancelet-like animal with a notochord, the forerunner of a backbone. It swam by wriggling its body.",
            "体内有脊索（脊柱的前身）、类似文昌鱼的动物。扭动身体游泳。",
        ),
        fact: L("「人類の祖先かも」と有名になったが、本当の位置づけはまだ議論中。", "Famous as a possible ancestor of humans, though its exact place is still debated.", "曾因“可能是人类的祖先”而出名，但确切地位仍有争议。"),
    },
    Species {
        name: L("オットイア", "Ottoia", "奥托虫"),
        sci: "Ottoia prolifica",
        group: L("鰓曳動物", "Priapulid worm", "鳃曳动物"),
        len_cm: 8.0,
        t0: 510.0,
        t1: 505.0,
        hab: SHELF,
        regions: &[Site(S_BURGESS)],
        beh: Beh::Burrow,
        art: Art::Worm,
        col: [0.86, 0.62, 0.56],
        col2: [0.62, 0.40, 0.36],
        rarity: 3.0,
        shy: 2.0,
        speed: 0.0,
        desc: L(
            "U字形の巣穴に住み、棘の並んだ口を裏返して突き出し、獲物を捕らえた虫。近づくとすぐ巣穴に引っ込む。",
            "A worm living in a U-shaped burrow that shoots out a spiny, inside-out proboscis to catch prey. It ducks back in when approached.",
            "住在U形洞穴中、翻出带刺的吻部捕捉猎物的蠕虫。一靠近就缩回洞里。",
        ),
        fact: L("腸の中から、自分の仲間が見つかったことがある（共食い）。", "One was found with another of its kind in its gut: cannibalism.", "曾在其肠道中发现同类（同类相食）。"),
    },
    Species {
        name: L("アイシェアイア", "Aysheaia", "埃谢栉蚕"),
        sci: "Aysheaia pedunculata",
        group: L("葉足動物", "Lobopodian", "叶足动物"),
        len_cm: 4.0,
        t0: 510.0,
        t1: 505.0,
        hab: SHELF,
        regions: &[Site(S_BURGESS)],
        beh: Beh::Crawl,
        art: Art::Aysheaia,
        col: [0.88, 0.66, 0.60],
        col2: [0.66, 0.44, 0.40],
        rarity: 1.4,
        shy: 0.0,
        speed: 0.02,
        desc: L(
            "ずんぐりした体に10対のつめのある脚。海綿の上に乗って、それを食べていたと考えられている。",
            "A stubby body with ten pairs of clawed legs. It probably climbed on sponges and fed on them.",
            "粗短的身体上有10对带爪的腿。人们认为它爬在海绵上并以海绵为食。",
        ),
        fact: L("いまのカギムシやクマムシの遠い親戚。", "A distant relative of today's velvet worms and water bears.", "是今天的栉蚕和水熊虫的远亲。"),
    },
    Species {
        name: L("オレノイデス", "Olenoides", "奥伦虫"),
        sci: "Olenoides serratus",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 8.0,
        t0: 510.0,
        t1: 500.0,
        hab: SHELF | SHALLOW,
        regions: &[Site(S_BURGESS), Plate(LAURENTIA)],
        beh: Beh::Crawl,
        art: Art::Trilobite { w: 0.62, spine: 0.16, pyg: 0.17, segs: 7 },
        col: [0.48, 0.40, 0.34],
        col2: [0.30, 0.24, 0.20],
        rarity: 3.0,
        shy: 0.0,
        speed: 0.08,
        desc: L(
            "触角や脚まで化石に残っていた三葉虫。脚の付け根のとげで獲物をかみ砕いたらしい。",
            "A trilobite preserved with antennae and legs. Spiny leg bases probably crushed its food.",
            "连触角和腿都保存在化石中的三叶虫。似乎用腿基部的刺嚼碎食物。",
        ),
        fact: L("尾のふちにも棘が並び、後ろにも長い触角のような突起があった。", "Spines line its tail, and it also had antenna-like feelers at the back.", "尾部边缘也排列着刺，后方还有类似触角的突起。"),
    },
    Species {
        name: L("バウキシア", "Vauxia", "沃克斯海绵"),
        sci: "Vauxia gracilenta",
        group: L("海綿動物", "Sponge", "海绵动物"),
        len_cm: 8.0,
        t0: 510.0,
        t1: 500.0,
        hab: SHELF,
        regions: &[Site(S_BURGESS), Plate(LAURENTIA)],
        beh: Beh::Sessile,
        art: Art::Sponge(0),
        col: [0.84, 0.74, 0.52],
        col2: [0.66, 0.54, 0.36],
        rarity: 4.0,
        shy: 0.0,
        speed: 0.0,
        desc: L(
            "枝分かれした管の形の海綿。体の網目は、いまのお風呂のスポンジと同じ繊維（スポンジン）でできていた。",
            "A branching, tube-shaped sponge whose skeleton was made of spongin, the same fibre as natural bath sponges.",
            "分枝管状的海绵。它的网状骨架由海绵丝构成，和现在的浴用海绵一样。",
        ),
        fact: L("ハルキゲニアやアイシェアイアがこの上で暮らしていたかもしれない。", "Hallucigenia and Aysheaia may have lived on sponges like this.", "怪诞虫和埃谢栉蚕可能就生活在它上面。"),
    },
    Species {
        name: L("シドネイア", "Sidneyia", "西德尼虫"),
        sci: "Sidneyia inexpectans",
        group: L("節足動物", "Arthropod", "节肢动物"),
        len_cm: 13.0,
        t0: 510.0,
        t1: 505.0,
        hab: SHELF,
        regions: &[Site(S_BURGESS)],
        beh: Beh::Crawl,
        art: Art::Sidneyia,
        col: [0.54, 0.48, 0.42],
        col2: [0.36, 0.32, 0.28],
        rarity: 1.0,
        shy: 0.0,
        speed: 0.09,
        desc: L(
            "海底を歩き回った大型の節足動物。おなかの中から三葉虫や貝のかけらが見つかり、硬い獲物を砕いて食べたと分かった。",
            "A large arthropod that roamed the sea floor. Bits of trilobites and shells in its gut show it crushed hard prey.",
            "在海底四处行走的大型节肢动物。腹中发现了三叶虫和贝壳碎片，说明它能咬碎坚硬的猎物。",
        ),
        fact: L("バージェス頁岩で最初に名前がつけられた動物のひとつ。名前は発見者の息子シドニーにちなむ。", "One of the first Burgess animals to be named, after the discoverer's son Sidney.", "最早被命名的伯吉斯动物之一，以发现者的儿子西德尼命名。"),
    },
    Species {
        name: L("ネクトカリス", "Nectocaris", "内克虾"),
        sci: "Nectocaris pteryx",
        group: L("所属不明（頭足類の祖先説も）", "Uncertain (maybe early cephalopod)", "分类不明（或为头足类祖先）"),
        len_cm: 5.0,
        t0: 510.0,
        t1: 505.0,
        hab: SHELF,
        regions: &[Site(S_BURGESS)],
        beh: Beh::Swim,
        art: Art::Nectocaris,
        col: [0.80, 0.74, 0.66],
        col2: [0.94, 0.86, 0.78],
        rarity: 1.0,
        shy: 1.4,
        speed: 0.6,
        desc: L(
            "平たい体のまわりのひれを波打たせて泳ぎ、2本の触手と柄の先の眼を持つ。イカやタコの遠い祖先とする説がある。",
            "It swam by rippling the fin around its flat body, with two tentacles and stalked eyes. Some think it was an early relative of squid.",
            "摆动扁平身体周围的鳍游泳，有两根触手和长在柄上的眼睛。有学说认为它是乌贼和章鱼的远祖。",
        ),
        fact: L("長い間、たった1つの化石しか知られていなかった。", "For decades it was known from a single fossil.", "很长时间里只发现过一件化石。"),
    },
    Species {
        name: L("ブルゲッソメデューサ", "Burgessomedusa", "伯吉斯水母"),
        sci: "Burgessomedusa phasmiformis",
        group: L("刺胞動物（クラゲ）", "Cnidarian (jellyfish)", "刺胞动物（水母）"),
        len_cm: 20.0,
        t0: 510.0,
        t1: T_END,
        hab: SHELF | DEEP | SHALLOW,
        regions: &[Site(S_BURGESS), Global],
        beh: Beh::Drift,
        art: Art::Jelly,
        col: [0.80, 0.90, 1.00],
        col2: [0.90, 0.70, 0.96],
        rarity: 1.0,
        shy: 0.0,
        speed: 0.12,
        desc: L(
            "最古の泳ぐクラゲのひとつ。釣り鐘形のかさの縁に、90本ほどの短い触手が並んでいた。",
            "One of the oldest swimming jellyfish, with about 90 short tentacles around the rim of its bell.",
            "最古老的游泳水母之一。钟形伞缘排列着约90根短触手。",
        ),
        fact: L("体のやわらかいクラゲの化石はとても珍しい。", "Soft jellyfish fossils are extremely rare.", "身体柔软的水母化石非常罕见。"),
    },
    Species {
        name: L("カンブロラスター", "Cambroraster", "寒武耙虾"),
        sci: "Cambroraster falcatus",
        group: L("放射歯類", "Radiodont", "放射齿类"),
        len_cm: 30.0,
        t0: 508.0,
        t1: 505.0,
        hab: SHELF,
        regions: &[Site(S_BURGESS)],
        beh: Beh::NearFloor,
        art: Art::Radiodont(3),
        col: [0.62, 0.54, 0.46],
        col2: [0.82, 0.74, 0.62],
        rarity: 0.8,
        shy: 0.0,
        speed: 0.35,
        desc: L(
            "頭を大きな盾のような甲羅で覆い、くまでのような付属肢で海底の泥をかいて餌を探した放射歯類。",
            "A radiodont with a huge shield-like head carapace that raked the sea-floor mud for food with comb-like appendages.",
            "头部覆盖着巨大盾状甲壳、用耙子般的附肢在海底泥中搜寻食物的放射齿类。",
        ),
        fact: L("甲羅の形がSF映画の宇宙船に似ていると話題になった。", "Its carapace was compared to a sci-fi spaceship.", "它的甲壳形状因酷似科幻电影中的宇宙飞船而成为话题。"),
    },
    Species {
        name: L("パラドキシデス", "Paradoxides", "奇异虫"),
        sci: "Paradoxides",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 25.0,
        t0: 506.0,
        t1: 497.0,
        hab: SHELF | SHALLOW,
        regions: &[Site(S_AVALON), Plate(AVALONIA), Plate(BALTICA)],
        beh: Beh::Crawl,
        art: Art::Trilobite { w: 0.56, spine: 0.40, pyg: 0.05, segs: 17 },
        col: [0.52, 0.46, 0.40],
        col2: [0.32, 0.28, 0.24],
        rarity: 2.0,
        shy: 0.0,
        speed: 0.1,
        desc: L(
            "長い頬の棘を持つ大型の三葉虫。体長が数十センチになるものもいた。",
            "A large trilobite with long cheek spines; some grew to tens of centimetres.",
            "有长颊刺的大型三叶虫，有的体长可达数十厘米。",
        ),
        fact: L("同じ種類がイギリスとカナダ東部で見つかり、昔は同じ大陸（アヴァロニア）だった証拠になった。", "The same species in Britain and eastern Canada helped show both were once one land, Avalonia.", "英国和加拿大东部发现同种化石，成为两地曾属同一块大陆（阿瓦隆尼亚）的证据。"),
    },
    Species {
        name: L("オレヌス", "Olenus", "小油栉虫"),
        sci: "Olenus",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 3.0,
        t0: 497.0,
        t1: 486.0,
        hab: SHELF,
        regions: &[Site(S_ALUM), Plate(BALTICA)],
        beh: Beh::Crawl,
        art: Art::Trilobite { w: 0.70, spine: 0.20, pyg: 0.08, segs: 14 },
        col: [0.30, 0.30, 0.30],
        col2: [0.18, 0.18, 0.20],
        rarity: 3.5,
        shy: 0.0,
        speed: 0.06,
        desc: L(
            "酸素の少ない暗い海の底に暮らした三葉虫。薄く平たい体で、少ない酸素を取り込んでいたらしい。",
            "A trilobite living on dark, oxygen-poor sea floors. Its thin, flat body may have helped it take in scarce oxygen.",
            "生活在缺氧黑暗海底的三叶虫。扁薄的身体或许有助于摄取稀少的氧气。",
        ),
        fact: L("体の中に硫黄を食べる細菌を住まわせていたという説もある。", "It may have hosted sulphur-eating bacteria.", "也有学说认为它体内共生着以硫为食的细菌。"),
    },
    Species {
        name: L("アグノストゥス", "Agnostus", "球接子"),
        sci: "Agnostus pisiformis",
        group: L("三葉虫の仲間", "Agnostid", "球接子类"),
        len_cm: 0.8,
        t0: 500.0,
        t1: 486.0,
        hab: SHELF,
        regions: &[Site(S_ALUM), Plate(BALTICA)],
        beh: Beh::NearFloor,
        art: Art::Agnostus,
        col: [0.70, 0.62, 0.50],
        col2: [0.40, 0.34, 0.28],
        rarity: 5.0,
        shy: 0.4,
        speed: 0.15,
        desc: L(
            "頭と尾がそっくりな形の、1cmにも満たない小さな三葉虫の仲間。目がなく、群れで泳いだと考えられている。",
            "A tiny relative of trilobites, under a centimetre, with head and tail shields of the same shape. Eyeless, it may have swum in swarms.",
            "头尾形状几乎一样、不到1厘米的小型三叶虫类。没有眼睛，被认为成群游动。",
        ),
        fact: L("スウェーデンの石灰岩には、この化石が数え切れないほど詰まっている。", "Swedish limestones are packed with countless specimens.", "瑞典的石灰岩里密密麻麻地布满了这种化石。"),
    },
    Species {
        name: L("クリマクティクニテス", "Climactichnites", "梯迹"),
        sci: "Climactichnites (maker)",
        group: L("ナメクジのような動物（陸に上がった跡）", "Slug-like animal (trackmaker)", "似蛞蝓动物（造迹者）"),
        len_cm: 20.0,
        t0: 505.0,
        t1: 490.0,
        hab: TIDAL,
        regions: &[Site(S_TIDAL), Plate(LAURENTIA)],
        beh: Beh::FlatCrawl,
        art: Art::Slug,
        col: [0.70, 0.58, 0.48],
        col2: [0.52, 0.42, 0.34],
        rarity: 1.6,
        shy: 0.0,
        speed: 0.05,
        desc: L(
            "干潟の砂の上に、はしごのような幅の広い這い跡を残したナメクジのような動物。水から出て陸を這った、最古の動物のひとつ。",
            "A slug-like animal that left broad, ladder-like trails on tidal sand: one of the first animals to crawl out of the water onto land.",
            "在潮坪沙地上留下宽阔梯状爬痕的似蛞蝓动物，是最早离开水面在陆地上爬行的动物之一。",
        ),
        fact: L("微生物マットを食べながら、干潮のときに這い回ったらしい。", "It probably grazed microbial mats at low tide.", "它大概在退潮时一边啃食微生物席一边爬行。"),
    },
    Species {
        name: L("コノドント動物", "Conodont animal", "牙形动物"),
        sci: "Proconodontus",
        group: L("脊椎動物", "Vertebrate", "脊椎动物"),
        len_cm: 4.0,
        t0: 497.0,
        t1: T_END,
        hab: SHELF | SHALLOW,
        regions: &[Global],
        beh: Beh::Swim,
        art: Art::Fish(2),
        col: [0.74, 0.70, 0.62],
        col2: [0.52, 0.48, 0.42],
        rarity: 1.4,
        shy: 2.0,
        speed: 0.9,
        desc: L(
            "大きな眼を持つウナギのような動物。口の中に、とがった歯のような小さな骨（コノドント）が並んでいた。",
            "An eel-like animal with big eyes and rows of tiny, tooth-like elements (conodonts) in its mouth.",
            "长着大眼睛、像鳗鱼一样的动物。口中排列着细小的牙齿状骨片（牙形石）。",
        ),
        fact: L("歯の化石は地層の年代を決める「ものさし」として使われる。", "Their tooth fossils are used to date rocks.", "其牙齿化石被用作测定地层年代的“标尺”。"),
    },
    Species {
        name: L("放散虫", "Radiolarian", "放射虫"),
        sci: "Radiolaria",
        group: L("単細胞の生きもの（プランクトン）", "Single-celled plankton", "单细胞生物（浮游生物）"),
        len_cm: 0.02,
        t0: 515.0,
        t1: T_END,
        hab: DEEP | SHELF,
        regions: &[Global],
        beh: Beh::Drift,
        art: Art::Radiolarian,
        col: [0.80, 0.94, 1.00],
        col2: [0.60, 0.86, 0.96],
        rarity: 7.0,
        shy: 0.0,
        speed: 0.04,
        desc: L(
            "ガラス質の繊細な骨格を持つ、目に見えないほど小さな単細胞の生きもの。外洋をただよう。",
            "Microscopic single-celled organisms with delicate glassy skeletons, drifting in the open ocean.",
            "拥有玻璃质精美骨架、肉眼几乎看不见的单细胞生物，漂浮在远洋中。",
        ),
        fact: L("死んだ骨格が深海に積もると、チャートという硬い岩になる。", "Their skeletons pile up on the deep sea floor and become a hard rock called chert.", "其骨骼沉积在深海底，会形成名为燧石的坚硬岩石。"),
    },
    Species {
        name: L("ガラス海綿", "Glass sponge", "玻璃海绵"),
        sci: "Protospongia",
        group: L("海綿動物（六放海綿）", "Sponge (hexactinellid)", "海绵动物（六放海绵）"),
        len_cm: 20.0,
        t0: 535.0,
        t1: T_END,
        hab: DEEP | SHELF,
        regions: &[Global],
        beh: Beh::Sessile,
        art: Art::Sponge(1),
        col: [0.86, 0.88, 0.84],
        col2: [0.66, 0.70, 0.68],
        rarity: 1.3,
        shy: 0.0,
        speed: 0.0,
        desc: L(
            "ガラスの針（骨片）を組み合わせて体をつくる海綿。深い海の底に、つぼのような姿で立っていた。",
            "A sponge whose body is built from interlocking glass needles, standing like a vase on the deep sea floor.",
            "用玻璃骨针组合成身体的海绵，像花瓶一样立在深海底。",
        ),
        fact: L("仲間のカイロウドウケツはいまも深海に暮らしている。", "Its relative, the Venus' flower basket, still lives in the deep sea.", "其亲戚偕老同穴至今仍生活在深海中。"),
    },
];

/// 上空・陸上から撮る現象
pub struct Phenom {
    pub name: L,
    pub desc: L,
    pub hint: L,
}

pub const P_GONDWANA: usize = 0;
pub const P_LAURENTIA: usize = 1;
pub const P_ERUPTION: usize = 2;
pub const P_BARREN: usize = 3;

pub static PHENOMS: &[Phenom] = &[
    Phenom {
        name: L("ゴンドワナ大陸", "Gondwana", "冈瓦纳大陆"),
        desc: L(
            "南極のあたりから赤道まで広がる巨大な大陸。いまのアフリカ・南アメリカ・南極・オーストラリア・インドなどが一つにつながっていた。",
            "A giant continent stretching from near the South Pole to the equator, joining today's Africa, South America, Antarctica, Australia, India and more.",
            "从南极附近一直延伸到赤道的巨大陆地，今天的非洲、南美洲、南极洲、澳大利亚、印度等连成一片。",
        ),
        hint: L("上空から、大陸全体が写るように撮影する", "Photograph the whole continent from the sky", "从高空拍下整个大陆"),
    },
    Phenom {
        name: L("ローレンシア大陸", "Laurentia", "劳伦大陆"),
        desc: L(
            "赤道にまたがる大陸で、のちの北アメリカ。海面が高く、大陸の上まで浅い海が入り込んでいた。",
            "A continent straddling the equator that will become North America. High sea level floods much of it with shallow seas.",
            "横跨赤道的大陆，后来成为北美洲。海平面很高，浅海一直延伸到大陆内部。",
        ),
        hint: L("上空から、大陸全体が写るように撮影する", "Photograph the whole continent from the sky", "从高空拍下整个大陆"),
    },
    Phenom {
        name: L("カルカリンジの大噴火", "Kalkarindji eruption", "卡尔卡林吉大喷发"),
        desc: L(
            "約5億1100万年前、いまのオーストラリア北部で起きた巨大な溶岩の噴出（巨大火成岩岩石区）。広さは日本の5倍以上。噴き出したガスが地球を温め、海の酸素を減らし、生きものの絶滅を引き起こしたと考えられている。",
            "About 511 million years ago, vast floods of lava poured out over what is now northern Australia, covering more than five times the area of Japan. The gases probably warmed the planet, starved the seas of oxygen and caused extinctions.",
            "约5亿1100万年前发生在今澳大利亚北部的巨量熔岩喷发（大火成岩省），面积超过日本的5倍。喷出的气体使地球变暖、海洋缺氧，被认为引发了生物灭绝。",
        ),
        hint: L("噴火の時代に、上空から近づいて撮影する", "During the eruptions, fly close and photograph them from the sky", "在喷发时期，从高空靠近拍摄"),
    },
    Phenom {
        name: L("植物のない大地", "A land without plants", "没有植物的大地"),
        desc: L(
            "カンブリア紀の陸には、まだ植物も動物もいない。むき出しの岩と砂が広がり、雨が降ると土砂がそのまま海へ流れ込んだ。",
            "Cambrian land has no plants or animals yet. Bare rock and sand stretch everywhere, and rain washes sediment straight into the sea.",
            "寒武纪的陆地上还没有植物和动物。裸露的岩石和沙土一望无际，下雨时泥沙直接流入大海。",
        ),
        hint: L("陸に降りて、景色を撮影する", "Land on the continent and photograph the scenery", "降落到陆地上拍摄风景"),
    },
];

pub fn n_species() -> usize {
    SPECIES.len()
}

/// 図鑑の項目の数（生きもの＋現象）
pub fn n_entries() -> usize {
    SPECIES.len() + PHENOMS.len()
}

/// その生きものがその場所・その時代にいるか
pub fn lives_here(sp: &Species, earth: &Earth, spot: &Spot, t: f32) -> bool {
    if !sp.alive_at(t) || sp.hab & spot.hab.bit() == 0 {
        return false;
    }
    sp.regions.iter().any(|r| match *r {
        Region::Global => true,
        Region::Tropics => spot.lat.abs() < 35.0,
        Region::Plate(p) => spot.plate == Some(p),
        Region::Site(s) => {
            let site = &SITES[s];
            t <= site.t0 + 0.5 && t >= site.t1 - 0.5 && deg_dist(earth.site_lonlat(s, t), (spot.lon, spot.lat)) < 7.0
        }
    })
}

/// 産地の近くでは、ふつうより多く出会う
pub fn abundance(sp: &Species, earth: &Earth, spot: &Spot, t: f32) -> f32 {
    let at_site = sp.regions.iter().any(|r| match *r {
        Region::Site(s) => deg_dist(earth.site_lonlat(s, t), (spot.lon, spot.lat)) < 7.0,
        _ => false,
    });
    let has_site = sp.regions.iter().any(|r| matches!(r, Region::Site(_)));
    if has_site && !at_site {
        sp.rarity * 0.45
    } else {
        sp.rarity
    }
}

/// その生きものに会いに行く時と場所（経緯度）。
/// 産地のあるものはその産地へ。世界中にいるものは、いまの時代に生きていればその時代のまま、
/// いまの場所（near）からいちばん近い、すめる場所を探す
pub fn where_when(i: usize, earth: &Earth, now: f32, near: (f32, f32)) -> (f32, (f32, f32)) {
    let sp = &SPECIES[i];
    let has_site = sp.regions.iter().any(|r| matches!(r, Region::Site(_)));
    let only_site = sp.regions.iter().all(|r| matches!(r, Region::Site(_)));
    // 産地のあるものは、いまの時代に産地が開いていなければ産地へ
    if has_site && (only_site || !sp.alive_at(now)) {
        for r in sp.regions {
            if let Region::Site(s) = *r {
                let site = &SITES[s];
                let a = sp.t0.min(site.t0);
                let b = sp.t1.max(site.t1);
                if a >= b {
                    let t = if now <= a && now >= b { now } else { ((a + b) * 0.5 * 2.0).round() / 2.0 };
                    return (t, earth.site_lonlat(s, t));
                }
            }
        }
    }
    let t = if sp.alive_at(now) { now } else { ((sp.t0 + sp.t1) * 0.5 * 2.0).round() / 2.0 };
    let mut best: Option<(f32, (f32, f32))> = None;
    let mut lat = -80.0;
    while lat <= 80.0 {
        let mut lon = -179.0;
        while lon < 180.0 {
            let spot = earth.sample(lon, lat, t);
            if lives_here(sp, earth, &spot, t) {
                // 近いほどよい。深海のものは深いほど、浅い海のものは地点がはっきりした場所を選ぶ
                let mut score = deg_dist(near, (lon, lat));
                if spot.hab == Habitat::Deep {
                    score += (4000.0 - spot.depth) * 0.005;
                }
                // まわりも同じ環境の場所を選ぶ（端すぎて潜ると別の環境になるのを避ける）
                let ok = [(0.6, 0.0), (-0.6, 0.0), (0.0, 0.6), (0.0, -0.6)].iter().all(|&(dx, dy)| earth.sample(lon + dx, lat + dy, t).hab == spot.hab);
                if !ok {
                    score += 30.0;
                }
                if best.map_or(true, |b| score < b.0) {
                    best = Some((score, (lon, lat)));
                }
            }
            lon += 1.0;
        }
        lat += 1.0;
    }
    (t, best.map_or((0.0, 0.0), |b| b.1))
}

/// 産地の一覧から、いまの場所の名前を作る
pub fn place_text(sp: &Species) -> String {
    let mut v: Vec<&str> = Vec::new();
    for r in sp.regions {
        match *r {
            Region::Site(s) => v.push(SITES[s].now.get()),
            Region::Plate(p) => v.push(PLATES[p].name.get()),
            Region::Tropics => v.push(t!("世界の暖かい海", "Warm seas worldwide", "世界各地的温暖海域")),
            Region::Global => v.push(t!("世界中", "Worldwide", "世界各地")),
        }
    }
    v.dedup();
    v.join(crate::lang::sep())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_species_can_be_found() {
        let e = Earth::new();
        for now in [535.0, 518.0, 508.0, 490.0] {
            for (i, sp) in SPECIES.iter().enumerate() {
                let (t, (lon, lat)) = where_when(i, &e, now, (40.0, 5.0));
                let spot = e.sample(lon, lat, t);
                assert!(lives_here(sp, &e, &spot, t), "{} not found at {:.1},{:.1} t={} ({:?}) now={}", sp.name.1, lon, lat, t, spot.hab, now);
            }
        }
    }

    #[test]
    fn species_data_sane() {
        for sp in SPECIES {
            assert!(sp.t0 >= sp.t1, "{}", sp.name.1);
            assert!(sp.t0 <= T_START + 1e-3 && sp.t1 >= T_END - 1e-3, "{}", sp.name.1);
            assert!(sp.hab != 0 && !sp.regions.is_empty());
        }
    }
}
