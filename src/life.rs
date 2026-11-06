//! 古生代（カンブリア紀〜ペルム紀）の生きもの（図鑑の項目）と、上空・陸から撮る現象
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
    // ---- オルドビス紀〜ペルム紀
    /// 頭足類 0=まっすぐな殻（オウムガイの仲間） 1=巻いた殻（アンモノイド）
    Cephalopod(u8),
    /// ウミサソリ 0=オール形の脚 1=大きなはさみ 2=とげの並んだ脚
    Eurypterid(u8),
    Crinoid,
    /// サンゴ・コケムシ 0=角サンゴ 1=床板サンゴの群体 2=枝分かれするコケムシ
    Coral(u8),
    Graptolite,
    /// 顎のない甲冑魚 0=サカバンバスピス 1=ビルケニア
    Jawless(u8),
    /// 板皮類 0=エンテログナトゥス 1=ダンクルオステウス 2=ボスリオレピス 3=マテルピスキス
    Placoderm(u8),
    /// サメの仲間 0=クラドセラケ 1=ステタカントゥス 2=ヘリコプリオン
    Shark(u8),
    /// 肉鰭類 0=ふつうの魚の形 1=ティクターリク
    LobeFin(u8),
    /// 四肢動物 0=アカントステガ 1=イクチオステガ 2=ヒロノムス 3=デンドレルペトン 4=ディメトロドン 5=エリオプス
    /// 6=ディプロカウルス 7=メソサウルス 8=イノストランケビア 9=スクトサウルス 10=リストロサウルス
    Tetrapod(u8),
    /// 陸の植物など 0=クックソニア 1=ライニア 2=アーケオプテリス 3=エオスペルマトプテリス 4=鱗木 5=カラミテス
    /// 6=木生シダ 7=グロッソプテリス 8=ワルキア 9=苔類 10=プロトタキシーテス 11=バラグワナチア
    Plant(u8),
    /// 陸の節足動物 0=ヤスデ 1=アースロプレウラ 2=サソリ 3=ワレイタムシ 4=ゴキブリの仲間
    Arthro(u8),
    /// 空を飛ぶ大きな昆虫（メガネウラ）
    Griffinfly,
    Tully,
    Fusulinid,
}

impl Art {
    /// 海底に平たく置いて斜め上から見る描き方か
    pub fn dorsal(self) -> bool {
        matches!(self, Art::Trilobite { .. } | Art::Agnostus | Art::Wiwaxia | Art::Halkieria | Art::Fuxianhuia | Art::Sidneyia | Art::Slug | Art::Trace | Art::Mat)
    }
}

pub struct Species {
    /// 画像の名前（assets/artwork/<slug>.jpg・assets/units/<slug>.png）
    pub slug: &'static str,
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
        slug: "stromatolite",
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
        slug: "microbial-mat",
        name: L("微生物マット", "Microbial mat", "微生物席"),
        sci: "Microbial mat",
        group: L("微生物の群れ", "Microbial community", "微生物群落"),
        len_cm: 40.0,
        t0: T_START,
        t1: 433.0,
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
        slug: "treptichnus",
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
        slug: "anabarites",
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
        slug: "archaeocyath",
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
        slug: "lingulella",
        name: L("リンギュレラ", "Lingulella", "小舌形贝"),
        sci: "Lingulella",
        group: L("腕足動物", "Brachiopod", "腕足动物"),
        len_cm: 1.5,
        t0: 525.0,
        t1: 485.4,
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
        slug: "redlichia",
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
        slug: "halkieria",
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
        slug: "haikouichthys",
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
        slug: "fuxianhuia",
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
        slug: "isoxys",
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
        slug: "leanchoilia",
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
        slug: "hallucigenia",
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
        slug: "amplectobelua",
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
        slug: "kerygmachela",
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
        slug: "tamisiocaris",
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
        slug: "anomalocaris",
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
        slug: "opabinia",
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
        slug: "wiwaxia",
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
        slug: "marrella",
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
        slug: "pikaia",
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
        slug: "ottoia",
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
        slug: "aysheaia",
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
        slug: "olenoides",
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
        slug: "vauxia",
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
        slug: "sidneyia",
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
        slug: "nectocaris",
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
        slug: "burgessomedusa",
        name: L("ブルゲッソメデューサ", "Burgessomedusa", "伯吉斯水母"),
        sci: "Burgessomedusa phasmiformis",
        group: L("刺胞動物（クラゲ）", "Cnidarian (jellyfish)", "刺胞动物（水母）"),
        len_cm: 20.0,
        t0: 510.0,
        t1: 485.4,
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
        slug: "cambroraster",
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
        slug: "paradoxides",
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
        slug: "olenus",
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
        slug: "agnostus",
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
        slug: "climactichnites",
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
        slug: "conodont",
        name: L("コノドント動物", "Conodont animal", "牙形动物"),
        sci: "Proconodontus",
        group: L("脊椎動物", "Vertebrate", "脊椎动物"),
        len_cm: 4.0,
        t0: 497.0,
        t1: 485.4,
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
        slug: "radiolarian",
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
        slug: "glass-sponge",
        name: L("ガラス海綿", "Glass sponge", "玻璃海绵"),
        sci: "Protospongia",
        group: L("海綿動物（六放海綿）", "Sponge (hexactinellid)", "海绵动物（六放海绵）"),
        len_cm: 20.0,
        t0: 535.0,
        t1: 485.4,
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
    // ======================================================== オルドビス紀
    Species {
        slug: "aegirocassis",
        name: L("エーギロカシス", "Aegirocassis", "埃吉尔卡西斯虾"),
        sci: "Aegirocassis benmoulai",
        group: L("放射歯類（こしとり食）", "Radiodont (filter feeder)", "放射齿类（滤食）"),
        len_cm: 200.0, t0: 480.0, t1: 477.0, hab: SHELF, regions: &[Site(S_FEZOUATA)],
        beh: Beh::Swim, art: Art::Radiodont(2), col: [0.62, 0.46, 0.38], col2: [0.85, 0.70, 0.55], rarity: 0.9, shy: 2.5, speed: 0.25,
        desc: L(
            "長さ2mにもなる放射歯類。頭の前の腕にある細かい毛で、海水からプランクトンをこし取って食べた。体の横には、ひれが上下2列並んでいた。",
            "A radiodont up to 2 m long. Fine bristles on its head appendages sieved plankton from the water. Two rows of flaps ran along each side of its body.",
            "体长可达2米的放射齿类。用头前附肢上的细毛从海水中滤食浮游生物。身体两侧各有上下两排鳍。",
        ),
        fact: L(
            "カンブリア紀の王者アノマロカリスの仲間は、オルドビス紀には巨大な「こしとり屋」になっていた。",
            "Relatives of the Cambrian top predator Anomalocaris had become giant filter feeders by the Ordovician.",
            "寒武纪霸主奇虾的亲戚，到奥陶纪已演变成巨大的“滤食者”。",
        ),
    },
    Species {
        slug: "cameroceras",
        name: L("カメロケラス", "Cameroceras", "房角石"),
        sci: "Cameroceras",
        group: L("頭足類（オウムガイの仲間）", "Cephalopod (nautiloid)", "头足类（鹦鹉螺类）"),
        len_cm: 600.0, t0: 470.0, t1: 445.0, hab: SHALLOW | SHELF, regions: &[Plate(LAURENTIA), Plate(BALTICA)],
        beh: Beh::Swim, art: Art::Cephalopod(0), col: [0.80, 0.70, 0.56], col2: [0.55, 0.32, 0.24], rarity: 0.7, shy: 3.0, speed: 0.3,
        desc: L(
            "まっすぐな円すい形の殻を持つオウムガイの仲間。殻の中の部屋に入れた気体で浮き沈みし、触手で獲物を捕らえた。",
            "A nautiloid with a straight, cone-shaped shell. Gas in the shell's chambers let it rise and sink, and it seized prey with its tentacles.",
            "拥有笔直圆锥形外壳的鹦鹉螺类。靠壳内气室中的气体沉浮，用触手捕捉猎物。",
        ),
        fact: L(
            "殻は数メートルに達したと考えられ、オルドビス紀の海で最大級の動物だった。",
            "Its shell may have reached several metres, making it one of the largest animals in Ordovician seas.",
            "外壳估计可达数米长，是奥陶纪海洋中最大的动物之一。",
        ),
    },
    Species {
        slug: "asaphus",
        name: L("アサフス・コワレフスキー", "Asaphus kowalewskii", "科瓦列夫斯基栉虾"),
        sci: "Asaphus kowalewskii",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 11.0, t0: 468.0, t1: 460.0, hab: SHALLOW | SHELF, regions: &[Site(S_STPETERSBURG), Plate(BALTICA)],
        beh: Beh::Crawl, art: Art::Trilobite { w: 0.7, spine: 0.1, pyg: 0.45, segs: 8 }, col: [0.50, 0.40, 0.32], col2: [0.30, 0.22, 0.18], rarity: 2.0, shy: 0.0, speed: 0.04,
        desc: L(
            "眼が長い柄の先についた三葉虫。泥に体を埋めても、潜望鏡のように眼だけを出してまわりを見張れた。",
            "A trilobite with eyes on tall stalks. Buried in mud, it could still keep watch with its eyes raised like periscopes.",
            "眼睛长在长柄顶端的三叶虫。即使把身体埋在泥里，也能像潜望镜一样只露出眼睛观察四周。",
        ),
        fact: L(
            "ロシア・サンクトペテルブルクの近くの石灰岩から、美しい化石がたくさん見つかる。",
            "Beautiful fossils are found in limestone near St Petersburg, Russia.",
            "在俄罗斯圣彼得堡附近的石灰岩中发现了许多精美的化石。",
        ),
    },
    Species {
        slug: "isotelus",
        name: L("イソテルス・レックス", "Isotelus rex", "王等称虫"),
        sci: "Isotelus rex",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 72.0, t0: 453.0, t1: 445.0, hab: SHALLOW | SHELF, regions: &[Site(S_CINCINNATI), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Trilobite { w: 0.72, spine: 0.05, pyg: 0.5, segs: 8 }, col: [0.56, 0.48, 0.38], col2: [0.36, 0.28, 0.22], rarity: 1.0, shy: 0.0, speed: 0.05,
        desc: L(
            "長さ70cmをこえる、知られている中で最大級の三葉虫。なめらかな殻で、海底をゆっくり歩いた。",
            "At over 70 cm long, one of the largest trilobites known. It walked slowly over the sea floor in a smooth shell.",
            "体长超过70厘米，是已知最大的三叶虫之一。披着光滑的外壳在海底缓缓爬行。",
        ),
        fact: L(
            "イソテルスの仲間は、アメリカ・オハイオ州の「州の化石」になっている。",
            "Isotelus is the official state fossil of Ohio, USA.",
            "等称虫是美国俄亥俄州的“州化石”。",
        ),
    },
    Species {
        slug: "sacabambaspis",
        name: L("サカバンバスピス", "Sacabambaspis", "萨卡班巴鱼"),
        sci: "Sacabambaspis janvieri",
        group: L("無顎類（甲冑魚）", "Jawless armoured fish", "无颌类（甲胄鱼）"),
        len_cm: 25.0, t0: 467.0, t1: 460.0, hab: SHALLOW, regions: &[Site(S_BOLIVIA), Plate(GONDWANA)],
        beh: Beh::Swim, art: Art::Jawless(0), col: [0.62, 0.60, 0.54], col2: [0.36, 0.36, 0.34], rarity: 2.5, shy: 1.6, speed: 0.35,
        desc: L(
            "あごのない魚。頭は骨の板でおおわれ、眼は顔の前に寄っていて、口はいつも開いたまま。泳ぎはあまり上手ではなかった。",
            "A jawless fish. Its head was covered in bony plates, its eyes sat at the very front of its face and its mouth was always open. It was not a strong swimmer.",
            "没有颌的鱼。头部覆盖骨板，眼睛长在脸的最前方，嘴总是张着。不太擅长游泳。",
        ),
        fact: L(
            "とぼけた顔つきで、最近はインターネットでも人気者になった。",
            "Its goofy face has recently made it an internet star.",
            "呆萌的长相让它最近在网络上走红。",
        ),
    },
    Species {
        slug: "graptolite",
        name: L("筆石（グラプトライト）", "Graptolite", "笔石"),
        sci: "Graptolithina",
        group: L("半索動物（群体）", "Hemichordate colony", "半索动物（群体）"),
        len_cm: 8.0, t0: 485.0, t1: 400.0, hab: SHELF | DEEP, regions: &[Global],
        beh: Beh::Drift, art: Art::Graptolite, col: [0.80, 0.82, 0.78], col2: [0.40, 0.44, 0.46], rarity: 3.0, shy: 0.0, speed: 0.02,
        desc: L(
            "小さな虫のような個体が、細い枝の上に並んで暮らす群体。海をただよい、プランクトンを食べた。",
            "A colony of tiny worm-like animals lined up along slender branches, drifting through the sea and feeding on plankton.",
            "由细小的虫状个体排列在细枝上形成的群体。在海中漂浮，以浮游生物为食。",
        ),
        fact: L(
            "形が速く変わったので、化石から地層の年代をくわしく知る「ものさし」として使われる。",
            "Because their shapes changed quickly, their fossils are used as a precise 'ruler' for dating rocks.",
            "因为形态变化很快，其化石被用作精确测定地层年代的“标尺”。",
        ),
    },
    Species {
        slug: "streptelasma",
        name: L("ストレプテラスマ", "Streptelasma", "扭心珊瑚"),
        sci: "Streptelasma",
        group: L("四放サンゴ（角サンゴ）", "Rugose 'horn' coral", "四射珊瑚（角珊瑚）"),
        len_cm: 4.0, t0: 460.0, t1: 425.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Sessile, art: Art::Coral(0), col: [0.86, 0.72, 0.58], col2: [0.95, 0.55, 0.45], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "牛の角のような形の、一匹で暮らすサンゴ。角の口から触手を広げて、流れてくる小さな生きものを捕らえた。",
            "A solitary coral shaped like a cow's horn. It spread tentacles from the open end to catch tiny drifting creatures.",
            "形似牛角、单独生活的珊瑚。从角口伸出触手，捕捉漂来的小生物。",
        ),
        fact: L(
            "いまのサンゴとは別の仲間で、古生代の終わりにすべて絶滅した。",
            "Rugose corals are not related to modern corals, and all of them died out at the end of the Paleozoic.",
            "四射珊瑚与现代珊瑚不是同一类，在古生代末全部灭绝。",
        ),
    },
    Species {
        slug: "crinoid",
        name: L("ウミユリ", "Crinoid (sea lily)", "海百合"),
        sci: "Crinoidea",
        group: L("棘皮動物", "Echinoderm", "棘皮动物"),
        len_cm: 50.0, t0: 485.0, t1: T_END, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Sessile, art: Art::Crinoid, col: [0.78, 0.52, 0.48], col2: [0.92, 0.74, 0.62], rarity: 3.5, shy: 0.0, speed: 0.0,
        desc: L(
            "ウニやヒトデの仲間だが、長い茎で海底に立ち、花のように腕を広げて流れてくる餌を集めた。",
            "A relative of sea urchins and starfish that stood on a long stalk, spreading its arms like a flower to catch drifting food.",
            "与海胆、海星同属一类，却用长茎立在海底，像花一样张开腕收集漂来的食物。",
        ),
        fact: L(
            "古生代の海にはウミユリの「草原」が広がり、茎のかけらが積もってできた石灰岩もある。",
            "Crinoid 'meadows' covered Paleozoic sea floors; some limestones are made almost entirely of their stem pieces.",
            "古生代海底遍布海百合“草原”，有些石灰岩几乎全由其茎的碎片堆积而成。",
        ),
    },
    Species {
        slug: "constellaria",
        name: L("コンステラリア（コケムシ）", "Constellaria (bryozoan)", "星苔藓虫"),
        sci: "Constellaria",
        group: L("コケムシ動物（群体）", "Bryozoan colony", "苔藓动物（群体）"),
        len_cm: 8.0, t0: 460.0, t1: 444.0, hab: SHALLOW, regions: &[Site(S_CINCINNATI), Tropics],
        beh: Beh::Sessile, art: Art::Coral(2), col: [0.82, 0.66, 0.50], col2: [0.92, 0.82, 0.62], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "0.5mmほどの小さな個体が何千も集まって、枝の形の群体をつくる。表面には星形の模様が並ぶ。",
            "Thousands of tiny individuals, each about half a millimetre across, build a branching colony patterned with little stars.",
            "数千个约0.5毫米的小个体聚在一起形成枝状群体，表面排列着星形花纹。",
        ),
        fact: L(
            "コケムシはオルドビス紀に一気にふえ、ほかの生きもののすみかになる礁をつくった。",
            "Bryozoans boomed in the Ordovician and built reefs that sheltered other animals.",
            "苔藓虫在奥陶纪迅速增多，建造了庇护其他生物的礁。",
        ),
    },
    Species {
        slug: "rafinesquina",
        name: L("ラフィネスキナ", "Rafinesquina", "拉芬内斯贝"),
        sci: "Rafinesquina",
        group: L("腕足動物", "Brachiopod", "腕足动物"),
        len_cm: 4.0, t0: 458.0, t1: 444.0, hab: SHALLOW | SHELF, regions: &[Site(S_CINCINNATI), Plate(LAURENTIA)],
        beh: Beh::Sessile, art: Art::Brachiopod, col: [0.70, 0.62, 0.52], col2: [0.85, 0.75, 0.60], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "平たい2枚の殻を持つ腕足動物。泥の上に殻を置いたまま、すき間から海水を取り込んで餌をこした。",
            "A brachiopod with two flat shells. It lay on the mud and filtered food from water drawn in between them.",
            "具有两片扁平外壳的腕足动物。躺在泥上，从壳缝中吸入海水滤食。",
        ),
        fact: L(
            "シンシナティの地層では、この殻が敷き詰められたように見つかる。",
            "In the rocks around Cincinnati its shells are packed together like paving stones.",
            "在辛辛那提的地层中，它的壳像铺路石一样密密麻麻。",
        ),
    },
    Species {
        slug: "megalograptus",
        name: L("メガログラプトゥス", "Megalograptus", "巨笔鲎"),
        sci: "Megalograptus ohioensis",
        group: L("ウミサソリ", "Sea scorpion (eurypterid)", "板足鲎（海蝎）"),
        len_cm: 120.0, t0: 453.0, t1: 445.0, hab: SHALLOW | TIDAL, regions: &[Site(S_CINCINNATI)],
        beh: Beh::NearFloor, art: Art::Eurypterid(2), col: [0.48, 0.36, 0.26], col2: [0.72, 0.56, 0.40], rarity: 0.8, shy: 2.0, speed: 0.25,
        desc: L(
            "体長1mをこえる、オルドビス紀のウミサソリ。前の脚には長いとげが並び、獲物を逃さなかった。",
            "An Ordovician sea scorpion over 1 m long. Rows of long spines on its front legs kept prey from escaping.",
            "体长超过1米的奥陶纪海蝎。前肢上排列着长刺，猎物难以逃脱。",
        ),
        fact: L(
            "ウミサソリはサソリやクモに近い仲間で、のちに入り江や川にも進出した。",
            "Sea scorpions are close relatives of scorpions and spiders, and later spread into lagoons and rivers.",
            "海蝎是蝎子和蜘蛛的近亲，后来扩展到潟湖和河流中。",
        ),
    },
    Species {
        slug: "early-land-plants",
        name: L("最初の陸の植物（苔類のなかま）", "First land plants (liverwort-like)", "最早的陆生植物（苔类）"),
        sci: "Early embryophytes",
        group: L("コケ植物", "Bryophyte-grade plant", "苔藓植物"),
        len_cm: 2.0, t0: 470.0, t1: 420.0, hab: TIDAL | LAND, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(9), col: [0.30, 0.52, 0.26], col2: [0.52, 0.66, 0.36], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "水辺の湿った地面にへばりつく、平たい小さな植物。根も茎もなく、胞子でふえた。陸を緑にした最初の植物の一つ。",
            "Small flat plants hugging damp ground by the water, with no roots or stems, spreading by spores — among the first plants to turn the land green.",
            "贴在水边潮湿地面上的扁平小植物，没有根和茎，靠孢子繁殖。是最早让陆地变绿的植物之一。",
        ),
        fact: L(
            "体は残りにくく、見つかるのは丈夫な胞子の化石がほとんど。",
            "Their bodies rarely fossilised; mostly only their tough spores are found.",
            "植物体难以保存，发现的大多是坚韧的孢子化石。",
        ),
    },
    Species {
        slug: "stromatoporoid",
        name: L("層孔虫（ストロマトポロイド）", "Stromatoporoid", "层孔虫"),
        sci: "Stromatoporoidea",
        group: L("海綿動物", "Sponge", "海绵动物"),
        len_cm: 60.0, t0: 470.0, t1: 359.0, hab: SHALLOW, regions: &[Tropics, Site(S_GOTLAND)],
        beh: Beh::Sessile, art: Art::Sponge(2), col: [0.80, 0.74, 0.62], col2: [0.60, 0.54, 0.46], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "石灰の層を何枚も重ねて、こぶやドームの形に育つ海綿の仲間。サンゴと一緒に古生代の礁の骨組みをつくった。",
            "A sponge that grew into lumps and domes by laying down layer after layer of lime. With corals, it built the framework of Paleozoic reefs.",
            "层层堆积石灰质、长成瘤状或穹顶状的海绵类。与珊瑚一起构成古生代礁的骨架。",
        ),
        fact: L(
            "デボン紀の終わりの絶滅で、礁とともにほとんど姿を消した。",
            "They nearly vanished, along with the reefs, in the Late Devonian extinctions.",
            "在泥盆纪末的灭绝中，它们与礁一起几乎消失。",
        ),
    },
    // ======================================================== シルル紀
    Species {
        slug: "cooksonia",
        name: L("クックソニア", "Cooksonia", "库克逊蕨"),
        sci: "Cooksonia",
        group: L("維管束植物", "Vascular plant", "维管植物"),
        len_cm: 5.0, t0: 433.0, t1: 410.0, hab: TIDAL | LAND, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(0), col: [0.36, 0.56, 0.30], col2: [0.66, 0.52, 0.30], rarity: 4.0, shy: 0.0, speed: 0.0,
        desc: L(
            "高さ数cmの、Y字に枝分かれした茎だけの植物。茎の先の丸い袋（胞子のう）から胞子を飛ばした。葉も根もない。",
            "A plant just a few centimetres tall made only of Y-shaped branching stems, releasing spores from round capsules at the tips. No leaves, no roots.",
            "高仅几厘米、只有Y形分叉茎的植物。从茎顶端的圆形孢子囊散播孢子，没有叶也没有根。",
        ),
        fact: L(
            "茎の中に水を運ぶ管（維管束）を持つ、最も古い植物の一つ。",
            "One of the oldest plants with water-carrying tubes (vascular tissue) in its stems.",
            "是茎中具有输水管道（维管束）的最古老植物之一。",
        ),
    },
    Species {
        slug: "baragwanathia",
        name: L("バラグワナチア", "Baragwanathia", "巴拉格瓦纳蕨"),
        sci: "Baragwanathia longifolia",
        group: L("ヒカゲノカズラ類", "Lycopsid (clubmoss)", "石松类"),
        len_cm: 30.0, t0: 427.0, t1: 410.0, hab: TIDAL | LAND, regions: &[Plate(GONDWANA)],
        beh: Beh::Sessile, art: Art::Plant(11), col: [0.30, 0.48, 0.26], col2: [0.46, 0.60, 0.32], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "細い葉がびっしりついた茎を持つ、ヒカゲノカズラの仲間。当時としては大きな、本物の葉を持つ植物だった。",
            "A clubmoss with stems densely covered in slender leaves — a large plant with true leaves for its time.",
            "茎上密生细叶的石松类。在当时是拥有真正叶片的大型植物。",
        ),
        fact: L(
            "オーストラリアで見つかった。これほど古い時代にこんな植物があったことは、長く議論になった。",
            "Found in Australia, its great age was debated for decades.",
            "发现于澳大利亚。如此古老的时代竟有这样的植物，曾长期引发争论。",
        ),
    },
    Species {
        slug: "pneumodesmus",
        name: L("プネウモデスムス", "Pneumodesmus", "呼吸马陆"),
        sci: "Pneumodesmus newmani",
        group: L("多足類（ヤスデ）", "Myriapod (millipede)", "多足类（马陆）"),
        len_cm: 1.0, t0: 429.0, t1: 424.0, hab: LAND | TIDAL, regions: &[Site(S_STONEHAVEN)],
        beh: Beh::Crawl, art: Art::Arthro(0), col: [0.40, 0.30, 0.22], col2: [0.62, 0.48, 0.32], rarity: 3.0, shy: 0.4, speed: 0.02,
        desc: L(
            "長さ1cmほどのヤスデの仲間。体の横に、空気を吸うための小さな穴（気門）が並んでいた。",
            "A millipede about 1 cm long with tiny breathing holes (spiracles) along its sides.",
            "体长约1厘米的马陆类。身体两侧排列着呼吸空气用的小孔（气门）。",
        ),
        fact: L(
            "空気を吸って陸で暮らした証拠が残る、最古の動物の一つ。",
            "One of the oldest animals with clear evidence of breathing air on land.",
            "是留有呼吸空气、在陆地生活证据的最古老动物之一。",
        ),
    },
    Species {
        slug: "favosites",
        name: L("ファボシテス（ハチノスサンゴ）", "Favosites (honeycomb coral)", "蜂巢珊瑚"),
        sci: "Favosites",
        group: L("床板サンゴ", "Tabulate coral", "床板珊瑚"),
        len_cm: 30.0, t0: 445.0, t1: 380.0, hab: SHALLOW, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Coral(1), col: [0.84, 0.76, 0.60], col2: [0.66, 0.50, 0.40], rarity: 4.0, shy: 0.0, speed: 0.0,
        desc: L(
            "六角形の細い管が蜂の巣のように集まった、サンゴの群体。暖かく浅い海で、大きな礁をつくった。",
            "A coral colony of slender six-sided tubes packed together like a honeycomb, building large reefs in warm shallow seas.",
            "由六角形细管像蜂巢一样聚集而成的珊瑚群体，在温暖浅海中建造大型礁。",
        ),
        fact: L(
            "英語では「ハニカム・コーラル（蜂の巣サンゴ）」と呼ばれる。",
            "In English it is nicknamed the 'honeycomb coral'.",
            "英文俗称“蜂巢珊瑚”。",
        ),
    },
    Species {
        slug: "calymene",
        name: L("カリメネ", "Calymene", "隐头虫"),
        sci: "Calymene blumenbachii",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 6.0, t0: 431.0, t1: 425.0, hab: SHALLOW, regions: &[Site(S_DUDLEY), Plate(AVALONIA)],
        beh: Beh::Crawl, art: Art::Trilobite { w: 0.8, spine: 0.0, pyg: 0.3, segs: 13 }, col: [0.56, 0.50, 0.44], col2: [0.36, 0.32, 0.28], rarity: 2.5, shy: 0.0, speed: 0.04,
        desc: L(
            "体をくるりと丸めて身を守ることができた三葉虫。丸まったままの化石もよく見つかる。",
            "A trilobite that could roll itself into a ball for protection; many fossils are found curled up.",
            "能把身体卷成一团来保护自己的三叶虫，常发现卷曲状态的化石。",
        ),
        fact: L(
            "イギリスのダドリーの町の紋章にも描かれ、「ダドリーのイナゴ」と呼ばれた。",
            "It appears on the coat of arms of Dudley, England, where it was called the 'Dudley locust'.",
            "它出现在英国达德利镇的徽章上，被称为“达德利蝗虫”。",
        ),
    },
    Species {
        slug: "guiyu",
        name: L("グイユー", "Guiyu", "梦幻鬼鱼"),
        sci: "Guiyu oneiros",
        group: L("肉鰭類（硬骨魚）", "Lobe-finned bony fish", "肉鳍鱼类（硬骨鱼）"),
        len_cm: 33.0, t0: 425.0, t1: 423.0, hab: SHALLOW, regions: &[Site(S_QUJING)],
        beh: Beh::Swim, art: Art::LobeFin(0), col: [0.46, 0.44, 0.40], col2: [0.70, 0.62, 0.48], rarity: 1.5, shy: 1.8, speed: 0.4,
        desc: L(
            "全身がわかる最古の硬骨魚の一つ。厚いうろこと、肉のついたひれ、背中のとげを持っていた。",
            "One of the oldest bony fishes known from a complete body, with thick scales, fleshy fins and spines on its back.",
            "已知最古老的完整硬骨鱼之一，有厚鳞、肉质鳍和背棘。",
        ),
        fact: L(
            "名前は中国語で「幽霊の魚」。わたしたち陸の動物につながる肉鰭類の、はじめのほうにいる。",
            "Its name means 'ghost fish' in Chinese. It sits near the start of the lobe-finned line that leads to land animals like us.",
            "名字意为“鬼鱼”。它位于通向陆生动物（包括我们）的肉鳍鱼类演化之初。",
        ),
    },
    Species {
        slug: "entelognathus",
        name: L("エンテログナトゥス", "Entelognathus", "初始全颌鱼"),
        sci: "Entelognathus primordialis",
        group: L("板皮類", "Placoderm", "盾皮鱼类"),
        len_cm: 20.0, t0: 425.0, t1: 423.0, hab: SHALLOW, regions: &[Site(S_QUJING)],
        beh: Beh::NearFloor, art: Art::Placoderm(0), col: [0.52, 0.50, 0.44], col2: [0.30, 0.30, 0.28], rarity: 1.5, shy: 1.5, speed: 0.3,
        desc: L(
            "頭と胸を骨の板でおおった魚（板皮類）だが、わたしたちと同じような骨のあごを持っていた。",
            "An armoured placoderm fish with plated head and chest — yet with jaw bones much like ours.",
            "头和胸覆盖骨板的盾皮鱼，却拥有和我们相似的颌骨。",
        ),
        fact: L(
            "あごの骨のつくりが、顎を持つ動物の進化を考える手がかりになった。",
            "Its jaw bones changed our ideas about how jawed animals evolved.",
            "它的颌骨结构为研究有颌动物的演化提供了线索。",
        ),
    },
    Species {
        slug: "birkenia",
        name: L("ビルケニア", "Birkenia", "伯肯鱼"),
        sci: "Birkenia elegans",
        group: L("無顎類（欠甲類）", "Jawless fish (anaspid)", "无颌类（缺甲鱼）"),
        len_cm: 10.0, t0: 430.0, t1: 420.0, hab: SHALLOW | TIDAL, regions: &[Plate(LAURENTIA), Plate(BALTICA)],
        beh: Beh::Swim, art: Art::Jawless(1), col: [0.34, 0.36, 0.38], col2: [0.60, 0.62, 0.60], rarity: 3.0, shy: 1.2, speed: 0.4,
        desc: L(
            "細長い体を小さなうろこでおおった、あごのない小魚。尾びれは下側が長く、群れで泳いだ。",
            "A small jawless fish with a slender, finely scaled body and a tail fin longer below than above. It swam in shoals.",
            "身体细长、覆盖小鳞片的无颌小鱼。尾鳍下叶较长，成群游动。",
        ),
        fact: L("スコットランドの地層から、たくさん見つかる。", "Many have been found in the rocks of Scotland.", "在苏格兰的地层中大量发现。"),
    },
    Species {
        slug: "eurypterus",
        name: L("ユーリプテルス", "Eurypterus", "广翅鲎"),
        sci: "Eurypterus remipes",
        group: L("ウミサソリ", "Sea scorpion (eurypterid)", "板足鲎（海蝎）"),
        len_cm: 20.0, t0: 423.0, t1: 419.2, hab: TIDAL | SHALLOW, regions: &[Site(S_BERTIE), Plate(BALTICA)],
        beh: Beh::NearFloor, art: Art::Eurypterid(0), col: [0.46, 0.40, 0.30], col2: [0.66, 0.56, 0.40], rarity: 3.0, shy: 1.5, speed: 0.25,
        desc: L(
            "オール形の脚で泳ぎ、浅い入り江の底で小さな生きものを捕らえたウミサソリ。",
            "A sea scorpion that swam with oar-shaped legs and caught small creatures on the floor of shallow lagoons.",
            "用桨状附肢游泳、在浅潟湖底捕食小动物的海蝎。",
        ),
        fact: L("アメリカ・ニューヨーク州の「州の化石」。", "It is the state fossil of New York, USA.", "是美国纽约州的“州化石”。"),
    },
    Species {
        slug: "prototaxites",
        name: L("プロトタキシーテス", "Prototaxites", "原杉藻"),
        sci: "Prototaxites",
        group: L("菌類（キノコの仲間）", "Giant fungus", "真菌（蘑菇类）"),
        len_cm: 600.0, t0: 420.0, t1: 370.0, hab: LAND, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(10), col: [0.74, 0.68, 0.58], col2: [0.56, 0.50, 0.42], rarity: 1.0, shy: 0.0, speed: 0.0,
        desc: L(
            "高さ8mにもなる、柱のような巨大な生きもの。木ではなく、菌類（キノコの仲間）だったと考えられている。",
            "A pillar-like giant up to 8 m tall. It was not a tree but probably a fungus.",
            "高达8米、柱子般的巨大生物。它不是树，而被认为是真菌（蘑菇类）。",
        ),
        fact: L(
            "木がまだない時代、陸でいちばん背の高い生きものだった。",
            "Before trees existed, it was the tallest living thing on land.",
            "在还没有树木的时代，它是陆地上最高的生物。",
        ),
    },
    // ======================================================== デボン紀
    Species {
        slug: "rhynia",
        name: L("ライニア", "Rhynia", "莱尼蕨"),
        sci: "Rhynia gwynne-vaughanii",
        group: L("維管束植物", "Vascular plant", "维管植物"),
        len_cm: 18.0, t0: 411.0, t1: 405.0, hab: LAND | TIDAL, regions: &[Site(S_RHYNIE)],
        beh: Beh::Sessile, art: Art::Plant(1), col: [0.34, 0.52, 0.28], col2: [0.60, 0.48, 0.28], rarity: 5.0, shy: 0.0, speed: 0.0,
        desc: L(
            "温泉の湧く湿地に生えた、高さ20cmほどの葉のない植物。温泉の成分で一瞬で石になり、細胞まで残った。",
            "A leafless plant about 20 cm tall that grew in a hot-spring wetland. Minerals in the spring water turned it to stone so fast that even its cells survive.",
            "生长在温泉湿地、高约20厘米的无叶植物。温泉中的矿物让它瞬间石化，连细胞都保存了下来。",
        ),
        fact: L(
            "ライニーチャートには、植物・菌類・小さな動物が生きていたときのまま閉じ込められている。",
            "The Rhynie chert preserves plants, fungi and tiny animals just as they were in life.",
            "莱尼燧石把植物、真菌和小动物以生前的样子封存了下来。",
        ),
    },
    Species {
        slug: "palaeocharinus",
        name: L("パレオカリヌス", "Palaeocharinus", "古蛛"),
        sci: "Palaeocharinus",
        group: L("クモ形類（ワレイタムシ類）", "Arachnid (trigonotarbid)", "蛛形类（三角蛛）"),
        len_cm: 0.4, t0: 411.0, t1: 405.0, hab: LAND, regions: &[Site(S_RHYNIE)],
        beh: Beh::Crawl, art: Art::Arthro(3), col: [0.32, 0.26, 0.22], col2: [0.52, 0.42, 0.34], rarity: 3.0, shy: 0.3, speed: 0.02,
        desc: L(
            "クモに似た小さな捕食者（ワレイタムシ類）。糸は出さず、植物の間で小さな虫を待ちぶせた。",
            "A tiny spider-like hunter (a trigonotarbid). It spun no silk, but ambushed small creatures among the plants.",
            "类似蜘蛛的微小捕食者（三角蛛类）。不吐丝，在植物间伏击小动物。",
        ),
        fact: L(
            "呼吸する器官（書肺）のひだまで化石に残っている。",
            "Even the folds of its breathing organs (book lungs) are preserved.",
            "连呼吸器官（书肺）的褶皱都保存在化石中。",
        ),
    },
    Species {
        slug: "schinderhannes",
        name: L("シンダーハンネス", "Schinderhannes", "辛德汉斯虾"),
        sci: "Schinderhannes bartelsi",
        group: L("放射歯類", "Radiodont", "放射齿类"),
        len_cm: 10.0, t0: 408.0, t1: 404.0, hab: SHELF, regions: &[Site(S_HUNSRUCK)],
        beh: Beh::Swim, art: Art::Radiodont(0), col: [0.50, 0.42, 0.38], col2: [0.80, 0.62, 0.48], rarity: 0.8, shy: 1.2, speed: 0.3,
        desc: L(
            "大きな眼とトゲのある腕を持ち、背中には翼のような大きなひれが一対あった。",
            "It had big eyes, spiny grasping arms and a pair of large wing-like flaps on its back.",
            "长着大眼睛和带刺的附肢，背上有一对翅膀般的大鳍。",
        ),
        fact: L(
            "アノマロカリスの仲間が、カンブリア紀から1億年後も生き残っていたことを示した。",
            "It showed that relatives of Anomalocaris survived 100 million years after the Cambrian.",
            "它证明奇虾的亲戚在寒武纪之后又存活了1亿年。",
        ),
    },
    Species {
        slug: "walliserops",
        name: L("ワリセロプス", "Walliserops", "瓦利塞尔虫"),
        sci: "Walliserops trifurcatus",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 7.0, t0: 400.0, t1: 394.0, hab: SHALLOW | SHELF, regions: &[Plate(GONDWANA)],
        beh: Beh::Crawl, art: Art::Trilobite { w: 0.7, spine: 0.6, pyg: 0.3, segs: 11 }, col: [0.52, 0.42, 0.34], col2: [0.30, 0.24, 0.20], rarity: 1.5, shy: 0.0, speed: 0.04,
        desc: L(
            "頭の先に三つまたの槍のような突起を持つ三葉虫。体中にとげが生えていた。",
            "A spiny trilobite with a trident-like prong sticking out from the front of its head.",
            "头部前端长着三叉戟般突起、全身长刺的三叶虫。",
        ),
        fact: L(
            "三つまたの槍は、オスどうしが争うのに使ったのではないかと考えられている。",
            "The trident may have been used by males to fight one another.",
            "三叉戟可能是雄性之间争斗时使用的。",
        ),
    },
    Species {
        slug: "jaekelopterus",
        name: L("イェーケロプテルス", "Jaekelopterus", "耶克尔鲎"),
        sci: "Jaekelopterus rhenaniae",
        group: L("ウミサソリ", "Sea scorpion (eurypterid)", "板足鲎（海蝎）"),
        len_cm: 250.0, t0: 393.0, t1: 389.0, hab: TIDAL | SHALLOW, regions: &[Site(S_EIFEL)],
        beh: Beh::NearFloor, art: Art::Eurypterid(1), col: [0.52, 0.36, 0.26], col2: [0.76, 0.56, 0.38], rarity: 0.7, shy: 3.0, speed: 0.3,
        desc: L(
            "はさみの長さだけで46cm、体長は2.5mと考えられる、史上最大級の節足動物。",
            "Its claw alone was 46 cm long and its body perhaps 2.5 m — one of the biggest arthropods ever.",
            "仅螯就长46厘米，体长估计达2.5米，是史上最大的节肢动物之一。",
        ),
        fact: L(
            "海ではなく、川の河口や入り江で暮らしていたらしい。",
            "It seems to have lived in estuaries and lagoons rather than the open sea.",
            "它似乎生活在河口和潟湖，而不是大海中。",
        ),
    },
    Species {
        slug: "eospermatopteris",
        name: L("エオスペルマトプテリス", "Eospermatopteris", "始籽蕨"),
        sci: "Eospermatopteris (Wattieza)",
        group: L("クラドキシロン類（初期の木）", "Cladoxylopsid tree", "枝蕨类（早期树木）"),
        len_cm: 800.0, t0: 387.0, t1: 381.0, hab: LAND, regions: &[Site(S_GILBOA), Plate(LAURENTIA)],
        beh: Beh::Sessile, art: Art::Plant(3), col: [0.46, 0.38, 0.28], col2: [0.34, 0.52, 0.28], rarity: 2.5, shy: 0.0, speed: 0.0,
        desc: L(
            "高さ8mほど。幹のてっぺんに枝の束が生え、葉はなかった。ヤシの木に似た姿の、最古の森の木。",
            "About 8 m tall, with a crown of branchlets but no leaves — a palm-like tree from the oldest known forest.",
            "高约8米，树干顶端长着枝束而没有叶子，形似棕榈，是最古老森林中的树。",
        ),
        fact: L(
            "ニューヨーク州ギルボアでは、森の切り株の化石が並んで見つかった。",
            "At Gilboa, New York, its fossil stumps were found standing in rows.",
            "在纽约州吉尔博阿，发现了成排的树桩化石。",
        ),
    },
    Species {
        slug: "archaeopteris",
        name: L("アーケオプテリス", "Archaeopteris", "古羊齿"),
        sci: "Archaeopteris",
        group: L("前裸子植物（木）", "Progymnosperm tree", "前裸子植物（树）"),
        len_cm: 2000.0, t0: 385.0, t1: 359.0, hab: LAND, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(2), col: [0.36, 0.30, 0.24], col2: [0.26, 0.46, 0.22], rarity: 2.0, shy: 0.0, speed: 0.0,
        desc: L(
            "高さ20mをこえる、最初の本格的な木。太い幹と葉の茂った枝を持ち、世界中で森をつくった。",
            "The first true tree, over 20 m tall, with a woody trunk and leafy branches. It formed forests all over the world.",
            "高逾20米的第一种真正的树，有粗壮的树干和茂密的枝叶，在世界各地形成森林。",
        ),
        fact: L("まだ種はなく、シダのように胞子でふえた。", "It had no seeds yet and reproduced with spores, like a fern.", "它还没有种子，像蕨类一样用孢子繁殖。"),
    },
    Species {
        slug: "materpiscis",
        name: L("マテルピスキス", "Materpiscis", "母鱼"),
        sci: "Materpiscis attenboroughi",
        group: L("板皮類", "Placoderm", "盾皮鱼类"),
        len_cm: 25.0, t0: 384.0, t1: 379.0, hab: SHALLOW, regions: &[Site(S_GOGO)],
        beh: Beh::Swim, art: Art::Placoderm(3), col: [0.58, 0.54, 0.46], col2: [0.34, 0.32, 0.28], rarity: 1.5, shy: 1.5, speed: 0.35,
        desc: L(
            "お腹の中に赤ちゃんとへその緒が残っていた板皮類。卵ではなく、赤ちゃんを産んでいた。",
            "A placoderm found with an unborn baby and umbilical cord inside — it gave birth to live young instead of laying eggs.",
            "腹中保存着胎儿和脐带的盾皮鱼。它不是产卵，而是直接生下幼鱼。",
        ),
        fact: L(
            "名前は「母なる魚」。博物学者デイビッド・アッテンボローにちなむ。",
            "Its name means 'mother fish' and honours the naturalist David Attenborough.",
            "名字意为“母亲鱼”，以博物学家大卫·爱登堡命名。",
        ),
    },
    Species {
        slug: "manticoceras",
        name: L("マンティコセラス", "Manticoceras", "尖棱菊石"),
        sci: "Manticoceras",
        group: L("頭足類（アンモノイド）", "Cephalopod (ammonoid)", "头足类（菊石类）"),
        len_cm: 15.0, t0: 383.0, t1: 372.0, hab: SHALLOW | SHELF, regions: &[Global, Site(S_GOGO)],
        beh: Beh::Swim, art: Art::Cephalopod(1), col: [0.78, 0.66, 0.50], col2: [0.52, 0.36, 0.26], rarity: 2.0, shy: 1.2, speed: 0.2,
        desc: L(
            "ぐるぐる巻いた殻を持つアンモノイド（アンモナイトの古い仲間）。殻の部屋の壁が波うつ線を描く。",
            "An ammonoid — an early relative of ammonites — with a tightly coiled shell. The walls between its chambers trace wavy lines.",
            "拥有螺旋外壳的菊石类（菊石的早期亲戚）。壳内隔壁形成波浪状的缝合线。",
        ),
        fact: L("ケルワッサー事変で、多くの仲間とともに絶滅した。", "It died out with many others in the Kellwasser event.", "它与许多同类一起在凯尔瓦瑟事件中灭绝。"),
    },
    Species {
        slug: "bothriolepis",
        name: L("ボスリオレピス", "Bothriolepis", "沟鳞鱼"),
        sci: "Bothriolepis canadensis",
        group: L("板皮類（胴甲類）", "Placoderm (antiarch)", "盾皮鱼类（胴甲鱼）"),
        len_cm: 30.0, t0: 387.0, t1: 359.0, hab: TIDAL | SHALLOW, regions: &[Site(S_MIGUASHA), Global],
        beh: Beh::NearFloor, art: Art::Placoderm(2), col: [0.56, 0.50, 0.40], col2: [0.34, 0.30, 0.26], rarity: 2.0, shy: 1.2, speed: 0.2,
        desc: L(
            "箱のような骨の鎧と、カニの脚のような細い胸びれを持つ魚。川や河口の底で、泥の中の餌をあさった。",
            "A fish in a box of bony armour, with thin, crab-leg-like pectoral fins. It grubbed for food in the mud of rivers and estuaries.",
            "披着箱形骨甲、胸鳍细如蟹腿的鱼。在河流和河口底部的泥中觅食。",
        ),
        fact: L("南極を含む世界中から化石が見つかる、とても栄えた魚。", "A hugely successful fish, found worldwide — even in Antarctica.", "非常繁盛的鱼类，化石遍布全世界，连南极也有。"),
    },
    Species {
        slug: "eusthenopteron",
        name: L("ユーステノプテロン", "Eusthenopteron", "真掌鳍鱼"),
        sci: "Eusthenopteron foordi",
        group: L("肉鰭類", "Lobe-finned fish", "肉鳍鱼类"),
        len_cm: 120.0, t0: 381.0, t1: 375.0, hab: TIDAL | SHALLOW, regions: &[Site(S_MIGUASHA)],
        beh: Beh::Swim, art: Art::LobeFin(0), col: [0.50, 0.48, 0.36], col2: [0.74, 0.66, 0.46], rarity: 1.5, shy: 2.0, speed: 0.45,
        desc: L(
            "ひれの中に、わたしたちの腕と同じ並びの骨（上腕骨・橈骨・尺骨）を持っていた魚。",
            "A fish whose fins held bones in the same pattern as our arms: humerus, radius and ulna.",
            "鳍中骨骼的排列与我们手臂相同（肱骨、桡骨、尺骨）的鱼。",
        ),
        fact: L("ミグアシャは「魚の時代」の化石の宝庫で、世界遺産になっている。", "Miguasha is a treasure-house of 'Age of Fishes' fossils and a World Heritage Site.", "米瓜沙是“鱼类时代”化石的宝库，已被列为世界遗产。"),
    },
    Species {
        slug: "tiktaalik",
        name: L("ティクターリク", "Tiktaalik", "提塔利克鱼"),
        sci: "Tiktaalik roseae",
        group: L("肉鰭類（魚と四肢動物の間）", "Lobe-finned 'fishapod'", "肉鳍鱼类（鱼与四足动物之间）"),
        len_cm: 270.0, t0: 376.0, t1: 374.0, hab: TIDAL, regions: &[Site(S_ELLESMERE)],
        beh: Beh::NearFloor, art: Art::LobeFin(1), col: [0.48, 0.44, 0.34], col2: [0.66, 0.58, 0.42], rarity: 1.0, shy: 2.0, speed: 0.2,
        desc: L(
            "ワニのような平たい頭と首、体を支えられる丈夫な胸びれを持つ魚。浅瀬で体を持ち上げ、水面から顔を出せた。",
            "A fish with a flat, crocodile-like head, a neck and sturdy front fins strong enough to prop up its body and lift its head out of the shallows.",
            "长着鳄鱼般扁平的头和脖子、胸鳍粗壮足以支撑身体的鱼。能在浅滩撑起身体，把头探出水面。",
        ),
        fact: L(
            "「魚と四肢動物をつなぐ動物」を探して、北極の岩場をねらって掘り当てられた。",
            "Scientists predicted where a fish–tetrapod link should be found, searched Arctic rocks of the right age, and found it.",
            "科学家预测了“连接鱼和四足动物的动物”的所在，瞄准北极相应年代的岩层，终于把它挖了出来。",
        ),
    },
    Species {
        slug: "acanthostega",
        name: L("アカントステガ", "Acanthostega", "棘螈"),
        sci: "Acanthostega gunnari",
        group: L("四肢動物", "Early tetrapod", "四足动物"),
        len_cm: 60.0, t0: 365.0, t1: 360.0, hab: TIDAL, regions: &[Site(S_EGREENLAND)],
        beh: Beh::NearFloor, art: Art::Tetrapod(0), col: [0.42, 0.44, 0.36], col2: [0.62, 0.60, 0.46], rarity: 1.5, shy: 1.5, speed: 0.25,
        desc: L(
            "8本指の足を持つ、最初期の四肢動物。でも足首は弱く、えらも残っていて、ほとんど水の中で暮らした。",
            "One of the first tetrapods, with eight fingers on each hand. But its wrists were weak and it still had gills, so it lived mostly in water.",
            "最早期的四足动物之一，每只手有8根指头。但腕部无力，还保留着鳃，几乎都生活在水中。",
        ),
        fact: L("足は、陸を歩くためより先に、水の中で生まれたことを教えてくれた。", "It showed that legs evolved in the water before they were used for walking on land.", "它告诉我们：四肢先是在水中诞生，后来才用于在陆地上行走。"),
    },
    Species {
        slug: "ichthyostega",
        name: L("イクチオステガ", "Ichthyostega", "鱼石螈"),
        sci: "Ichthyostega",
        group: L("四肢動物", "Early tetrapod", "四足动物"),
        len_cm: 150.0, t0: 365.0, t1: 360.0, hab: TIDAL | LAND, regions: &[Site(S_EGREENLAND)],
        beh: Beh::Crawl, art: Art::Tetrapod(1), col: [0.40, 0.38, 0.30], col2: [0.60, 0.56, 0.40], rarity: 1.2, shy: 1.8, speed: 0.08,
        desc: L(
            "がっしりした肩と前足で、アザラシのように体を引きずって岸に上がれた四肢動物。尾には魚のようなひれがあった。",
            "A tetrapod with powerful shoulders and forelimbs that could haul itself ashore like a seal. Its tail still had a fish-like fin.",
            "肩膀和前肢强壮、能像海豹一样拖着身体爬上岸的四足动物。尾巴上还有鱼一样的鳍。",
        ),
        fact: L("後ろ足は7本指だった。", "Its hind feet had seven toes.", "它的后脚有7根趾头。"),
    },
    Species {
        slug: "dunkleosteus",
        name: L("ダンクルオステウス", "Dunkleosteus", "邓氏鱼"),
        sci: "Dunkleosteus terrelli",
        group: L("板皮類（節頸類）", "Placoderm (arthrodire)", "盾皮鱼类（节颈鱼）"),
        len_cm: 400.0, t0: 372.0, t1: 359.0, hab: SHELF | DEEP, regions: &[Site(S_CLEVELAND), Plate(LAURENTIA)],
        beh: Beh::Swim, art: Art::Placoderm(1), col: [0.42, 0.42, 0.42], col2: [0.66, 0.62, 0.54], rarity: 0.6, shy: 3.5, speed: 0.4,
        desc: L(
            "頭が骨の鎧でおおわれた、デボン紀の海の王者。歯はなく、あごの骨そのものが刃のように鋭かった。",
            "The armoured-headed ruler of the Devonian seas. It had no teeth; the edges of its jaw bones were sharp as blades.",
            "头部覆盖骨甲的泥盆纪海洋霸主。没有牙齿，颌骨本身像刀刃一样锋利。",
        ),
        fact: L(
            "口をすばやく開けて獲物を吸い込み、強い力でかみ切ったと考えられている。",
            "It probably snapped its mouth open to suck in prey, then sliced it with a powerful bite.",
            "据推测它能迅速张口吸入猎物，再用强大的咬合力将其切断。",
        ),
    },
    Species {
        slug: "cladoselache",
        name: L("クラドセラケ", "Cladoselache", "裂口鲨"),
        sci: "Cladoselache",
        group: L("軟骨魚類（サメ）", "Early shark", "软骨鱼类（鲨鱼）"),
        len_cm: 180.0, t0: 370.0, t1: 359.0, hab: SHELF, regions: &[Site(S_CLEVELAND), Plate(LAURENTIA)],
        beh: Beh::Swim, art: Art::Shark(0), col: [0.46, 0.50, 0.54], col2: [0.80, 0.82, 0.80], rarity: 1.0, shy: 2.5, speed: 0.6,
        desc: L(
            "古いサメの仲間。いまのサメに似た流線形の体で、速く泳いだ。うろこはほとんどなかった。",
            "An early shark with a streamlined body much like modern sharks. It was a fast swimmer and had almost no scales.",
            "早期的鲨鱼。身体呈流线型，与现代鲨鱼相似，游得很快，几乎没有鳞片。",
        ),
        fact: L("お腹の中から、丸のみにした小魚の化石が見つかっている。", "Fossils have been found with small fish swallowed whole, tail-first, in their stomachs.", "在它的胃中发现过被整条吞下的小鱼化石。"),
    },
    // ======================================================== 石炭紀
    Species {
        slug: "stethacanthus",
        name: L("ステタカントゥス", "Stethacanthus", "胸脊鲨"),
        sci: "Stethacanthus",
        group: L("軟骨魚類（サメの仲間）", "Shark relative", "软骨鱼类（鲨类）"),
        len_cm: 70.0, t0: 359.0, t1: 323.0, hab: SHALLOW | SHELF, regions: &[Global, Site(S_BEARGULCH)],
        beh: Beh::Swim, art: Art::Shark(1), col: [0.40, 0.42, 0.44], col2: [0.18, 0.18, 0.20], rarity: 1.5, shy: 2.0, speed: 0.5,
        desc: L(
            "背中に、上が平らな「アイロン台」のような変わったひれを持つサメの仲間。ひれの上面には小さな歯のようなとげが並んでいた。",
            "A shark relative with a strange flat-topped 'ironing board' fin on its back, its top covered in small tooth-like spines.",
            "背上长着奇特的平顶“熨衣板”状鳍的鲨类，鳍的顶面排列着牙齿般的小刺。",
        ),
        fact: L("オスだけにあったとも考えられ、なんに使ったのかはまだなぞ。", "It may have been found only in males; what it was for is still a mystery.", "可能只有雄性才有，用途至今成谜。"),
    },
    Species {
        slug: "gigantoproductus",
        name: L("ギガントプロダクトゥス", "Gigantoproductus", "大长身贝"),
        sci: "Gigantoproductus",
        group: L("腕足動物", "Brachiopod", "腕足动物"),
        len_cm: 30.0, t0: 340.0, t1: 325.0, hab: SHALLOW, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Brachiopod, col: [0.80, 0.74, 0.62], col2: [0.90, 0.82, 0.68], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "幅30cmをこえる、史上最大の腕足動物。厚い殻を海底に沈めて、群れで暮らした。",
            "The largest brachiopod ever, over 30 cm wide. It lived in groups, its heavy shell settled into the sea floor.",
            "宽逾30厘米，是史上最大的腕足动物。厚重的壳沉在海底，成群生活。",
        ),
        fact: L("殻の中に、光合成をする藻を共生させていたという説もある。", "Some think it hosted photosynthetic algae inside its shell.", "有学说认为它壳内共生着能进行光合作用的藻类。"),
    },
    Species {
        slug: "lepidodendron",
        name: L("レピドデンドロン（鱗木）", "Lepidodendron (scale tree)", "鳞木"),
        sci: "Lepidodendron",
        group: L("ヒカゲノカズラ類（木）", "Giant lycopsid tree", "石松类（树）"),
        len_cm: 3000.0, t0: 359.0, t1: 299.0, hab: LAND | TIDAL, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Plant(4), col: [0.40, 0.40, 0.30], col2: [0.30, 0.50, 0.24], rarity: 2.5, shy: 0.0, speed: 0.0,
        desc: L(
            "高さ30mをこえる、幹がうろこ模様の木（鱗木）。ヒカゲノカズラの仲間で、湿地に大森林をつくった。",
            "A clubmoss relative over 30 m tall with a trunk patterned like scales. It formed vast swamp forests.",
            "高逾30米、树干带鳞片花纹的树（鳞木），属石松类，在湿地形成大片森林。",
        ),
        fact: L("この森がうもれてできたのが石炭。産業革命を動かした燃料になった。", "Buried, these forests became coal — the fuel that powered the Industrial Revolution.", "这些森林被掩埋后变成煤炭，成为推动工业革命的燃料。"),
    },
    Species {
        slug: "calamites",
        name: L("カラミテス（ロボク）", "Calamites", "芦木"),
        sci: "Calamites",
        group: L("トクサ類", "Giant horsetail", "木贼类"),
        len_cm: 1000.0, t0: 340.0, t1: 299.0, hab: LAND | TIDAL, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Plant(5), col: [0.46, 0.50, 0.32], col2: [0.32, 0.52, 0.26], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "節のある竹のような幹を持つ、高さ10mにもなるトクサの仲間。川べりや湿地に茂った。",
            "A horsetail up to 10 m tall with a jointed, bamboo-like trunk, growing thickly along rivers and swamps.",
            "高达10米、树干有节似竹的木贼类，在河边和湿地繁茂生长。",
        ),
        fact: L("いまのスギナ（ツクシ）は、その小さな親戚。", "Today's little horsetails are its small relatives.", "现代的问荆（笔头草）是它的小亲戚。"),
    },
    Species {
        slug: "pulmonoscorpius",
        name: L("プルモノスコルピウス", "Pulmonoscorpius", "肺蝎"),
        sci: "Pulmonoscorpius kirktonensis",
        group: L("サソリ", "Scorpion", "蝎子"),
        len_cm: 70.0, t0: 337.0, t1: 333.0, hab: LAND, regions: &[Site(S_EASTKIRKTON)],
        beh: Beh::Crawl, art: Art::Arthro(2), col: [0.30, 0.24, 0.20], col2: [0.52, 0.40, 0.30], rarity: 1.5, shy: 1.2, speed: 0.06,
        desc: L(
            "体長70cmにもなる巨大なサソリ。陸で空気を吸って暮らし、小さな四肢動物も襲ったかもしれない。",
            "A giant scorpion up to 70 cm long that breathed air on land and may even have hunted small tetrapods.",
            "体长可达70厘米的巨型蝎子，在陆地上呼吸空气，可能还捕食小型四足动物。",
        ),
        fact: L("空気中の酸素が多かったことが、巨大化を助けたと考えられている。", "High oxygen levels in the air may have helped it grow so big.", "人们认为当时空气中氧气含量高，帮助它长得如此巨大。"),
    },
    Species {
        slug: "arthropleura",
        name: L("アースロプレウラ", "Arthropleura", "节胸蜈蚣"),
        sci: "Arthropleura",
        group: L("多足類（ヤスデの仲間）", "Giant myriapod", "多足类（马陆类）"),
        len_cm: 250.0, t0: 345.0, t1: 295.0, hab: LAND, regions: &[Plate(LAURENTIA), Plate(BALTICA), Plate(AVALONIA)],
        beh: Beh::Crawl, art: Art::Arthro(1), col: [0.36, 0.30, 0.26], col2: [0.56, 0.46, 0.36], rarity: 0.8, shy: 1.0, speed: 0.06,
        desc: L(
            "長さ2.5mにもなる、史上最大の陸の節足動物（ヤスデの仲間）。森の床を歩いて植物を食べた。",
            "At up to 2.5 m long, the largest land arthropod ever — a millipede relative that roamed the forest floor eating plants.",
            "体长可达2.5米，是史上最大的陆生节肢动物（马陆类），在森林地面上行走、以植物为食。",
        ),
        fact: L("頭まで残った化石が見つかり、顔つきがわかってきた。", "Recently found fossils with heads have finally revealed what its face looked like.", "最近发现了连头部都保存下来的化石，终于弄清了它的样貌。"),
    },
    Species {
        slug: "hylonomus",
        name: L("ヒロノムス", "Hylonomus", "林蜥"),
        sci: "Hylonomus lyelli",
        group: L("爬虫類（有羊膜類）", "Early reptile (amniote)", "爬行动物（羊膜动物）"),
        len_cm: 20.0, t0: 318.0, t1: 313.0, hab: LAND, regions: &[Site(S_JOGGINS)],
        beh: Beh::Crawl, art: Art::Tetrapod(2), col: [0.48, 0.40, 0.30], col2: [0.66, 0.56, 0.40], rarity: 2.0, shy: 1.0, speed: 0.12,
        desc: L(
            "トカゲのような姿の、最古の爬虫類の一つ。殻のある卵を陸に産めた。",
            "A lizard-like animal and one of the oldest reptiles, able to lay shelled eggs on land.",
            "外形像蜥蜴，是最古老的爬行动物之一，能在陆地上产有壳的卵。",
        ),
        fact: L("鱗木の切り株の空洞の中から化石が見つかった。", "Its fossils were found inside hollow scale-tree stumps.", "它的化石是在鳞木树桩的空洞里发现的。"),
    },
    Species {
        slug: "dendrerpeton",
        name: L("デンドレルペトン", "Dendrerpeton", "树螈"),
        sci: "Dendrerpeton",
        group: L("両生類（分椎類）", "Amphibian (temnospondyl)", "两栖类（离片椎类）"),
        len_cm: 90.0, t0: 318.0, t1: 313.0, hab: LAND | TIDAL, regions: &[Site(S_JOGGINS)],
        beh: Beh::Crawl, art: Art::Tetrapod(3), col: [0.40, 0.40, 0.32], col2: [0.58, 0.56, 0.42], rarity: 1.5, shy: 1.2, speed: 0.08,
        desc: L(
            "大きな頭を持つ両生類。水辺の森を歩き、虫や小さな動物を食べた。",
            "A big-headed amphibian that walked the waterside forests, eating insects and other small animals.",
            "头部很大的两栖动物，在水边森林中行走，以昆虫和小动物为食。",
        ),
        fact: L("ヒロノムスと同じく、木の切り株の中から化石が見つかった。", "Like Hylonomus, it was found fossilised inside tree stumps.", "和林蜥一样，它的化石也发现于树桩中。"),
    },
    Species {
        slug: "tullimonstrum",
        name: L("ツリモンストラム（トゥリーモンスター）", "Tullimonstrum (Tully monster)", "塔利怪物"),
        sci: "Tullimonstrum gregarium",
        group: L("正体不明の動物", "Animal of uncertain affinity", "分类不明的动物"),
        len_cm: 30.0, t0: 309.0, t1: 306.0, hab: SHALLOW | TIDAL, regions: &[Site(S_MAZON)],
        beh: Beh::Swim, art: Art::Tully, col: [0.56, 0.48, 0.44], col2: [0.34, 0.28, 0.26], rarity: 1.5, shy: 1.4, speed: 0.3,
        desc: L(
            "長い鼻の先にはさみのような口、横に突き出た棒の先に眼。どの動物の仲間なのか、いまも決着していないふしぎな動物。",
            "Claw-like jaws at the end of a long snout, and eyes on the ends of a sideways bar. Scientists still argue about what kind of animal it was.",
            "长鼻末端是钳子般的嘴，眼睛长在横向伸出的杆子末端。它属于哪类动物至今仍无定论。",
        ),
        fact: L("アメリカ・イリノイ州の「州の化石」。", "It is the state fossil of Illinois, USA.", "是美国伊利诺伊州的“州化石”。"),
    },
    Species {
        slug: "meganeura",
        name: L("メガネウラ", "Meganeura", "巨脉蜻蜓"),
        sci: "Meganeura monyi",
        group: L("昆虫（オオトンボ類）", "Insect (griffinfly)", "昆虫（原蜻蜓类）"),
        len_cm: 40.0, t0: 303.0, t1: 299.0, hab: LAND | TIDAL, regions: &[Site(S_COMMENTRY)],
        beh: Beh::Swim, art: Art::Griffinfly, col: [0.30, 0.34, 0.30], col2: [0.80, 0.86, 0.90], rarity: 1.2, shy: 2.0, speed: 0.8,
        desc: L(
            "はねを広げると70cm。トンボに似た巨大な昆虫で、空を飛ぶ虫を捕まえて食べた。",
            "A giant dragonfly-like insect with a 70 cm wingspan that caught other flying insects in mid-air.",
            "翼展达70厘米、形似蜻蜓的巨型昆虫，在空中捕食其他飞虫。",
        ),
        fact: L("鳥もコウモリもいない空で、いちばんのハンターだった。", "With no birds or bats yet, it was the top hunter of the skies.", "在还没有鸟和蝙蝠的天空中，它是最强的猎手。"),
    },
    Species {
        slug: "archimylacris",
        name: L("アーキミラクリス", "Archimylacris", "古蠊"),
        sci: "Archimylacris",
        group: L("昆虫（ゴキブリの祖先の仲間）", "Insect (roachoid)", "昆虫（蜚蠊类祖先）"),
        len_cm: 3.0, t0: 315.0, t1: 299.0, hab: LAND, regions: &[Tropics],
        beh: Beh::Crawl, art: Art::Arthro(4), col: [0.40, 0.28, 0.18], col2: [0.62, 0.48, 0.32], rarity: 2.5, shy: 0.6, speed: 0.06,
        desc: L(
            "森の床の落ち葉の中で暮らした、ゴキブリに似た昆虫。石炭紀の森にとても多かった。",
            "A cockroach-like insect that lived in the leaf litter of the forest floor. They were very common in Carboniferous forests.",
            "生活在森林地面落叶中、形似蟑螂的昆虫。在石炭纪森林中非常多。",
        ),
        fact: L("石炭紀は「ゴキブリの時代」と呼ばれるほど、その仲間の化石が多い。", "Their fossils are so common that the Carboniferous is sometimes called the 'Age of Cockroaches'.", "其化石多到石炭纪有时被称为“蟑螂时代”。"),
    },
    Species {
        slug: "psaronius",
        name: L("プサロニウス（木生シダ）", "Psaronius (tree fern)", "辉木（树蕨）"),
        sci: "Psaronius",
        group: L("シダ植物（木生シダ）", "Tree fern", "蕨类植物（树蕨）"),
        len_cm: 1000.0, t0: 310.0, t1: 260.0, hab: LAND | TIDAL, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Plant(6), col: [0.42, 0.32, 0.24], col2: [0.30, 0.52, 0.24], rarity: 2.0, shy: 0.0, speed: 0.0,
        desc: L(
            "高さ10mほどの木のようなシダ。幹のまわりを根がびっしりおおい、てっぺんに大きな葉を広げた。",
            "A tree-like fern about 10 m tall. A thick mantle of roots wrapped its trunk, and large fronds spread from the top.",
            "高约10米的树状蕨类。树干外包裹着厚厚的根，顶端展开巨大的叶片。",
        ),
        fact: L("森が乾いていくと、鱗木に代わって森の主役になった。", "As the forests dried out, it replaced the scale trees as the forest's main tree.", "随着森林变干，它取代鳞木成为森林的主角。"),
    },
    // ======================================================== ペルム紀
    Species {
        slug: "mesosaurus",
        name: L("メソサウルス", "Mesosaurus", "中龙"),
        sci: "Mesosaurus tenuidens",
        group: L("爬虫類", "Aquatic reptile", "爬行动物"),
        len_cm: 100.0, t0: 299.0, t1: 294.0, hab: SHALLOW | TIDAL, regions: &[Site(S_IRATI)],
        beh: Beh::Swim, art: Art::Tetrapod(7), col: [0.42, 0.40, 0.34], col2: [0.66, 0.60, 0.48], rarity: 2.0, shy: 1.5, speed: 0.4,
        desc: L(
            "細長いあごに針のような歯が並ぶ、水の中で暮らした爬虫類。長い尾で泳いだ。",
            "A water-dwelling reptile with long, slender jaws full of needle-like teeth, swimming with its long tail.",
            "生活在水中的爬行动物，细长的颌上排列着针状牙齿，靠长尾游泳。",
        ),
        fact: L(
            "南アメリカとアフリカの両方から見つかり、大陸がつながっていた証拠になった。",
            "It is found in both South America and Africa — evidence that the continents were once joined.",
            "在南美洲和非洲都有发现，成为大陆曾经相连的证据。",
        ),
    },
    Species {
        slug: "glossopteris",
        name: L("グロッソプテリス", "Glossopteris", "舌羊齿"),
        sci: "Glossopteris",
        group: L("種子シダ類（木）", "Seed-fern tree", "种子蕨类（树）"),
        len_cm: 600.0, t0: 299.0, t1: 252.0, hab: LAND | TIDAL, regions: &[Plate(GONDWANA)],
        beh: Beh::Sessile, art: Art::Plant(7), col: [0.40, 0.32, 0.24], col2: [0.30, 0.48, 0.24], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "舌の形の葉をつける、ゴンドワナの寒い土地に広がった木。種でふえた。",
            "A tree with tongue-shaped leaves that spread across the cool lands of Gondwana, reproducing with seeds.",
            "长着舌形叶片的树，遍布冈瓦纳寒冷的土地，靠种子繁殖。",
        ),
        fact: L(
            "南アメリカ・アフリカ・インド・南極・オーストラリアで見つかり、大陸移動説の証拠になった。",
            "Found in South America, Africa, India, Antarctica and Australia, it became key evidence for continental drift.",
            "在南美洲、非洲、印度、南极洲和澳大利亚都有发现，成为大陆漂移说的重要证据。",
        ),
    },
    Species {
        slug: "walchia",
        name: L("ワルキア", "Walchia", "瓦契杉"),
        sci: "Walchia",
        group: L("針葉樹", "Early conifer", "针叶树"),
        len_cm: 800.0, t0: 300.0, t1: 275.0, hab: LAND, regions: &[Plate(LAURENTIA), Plate(BALTICA), Plate(AVALONIA)],
        beh: Beh::Sessile, art: Art::Plant(8), col: [0.36, 0.30, 0.24], col2: [0.22, 0.38, 0.22], rarity: 2.5, shy: 0.0, speed: 0.0,
        desc: L(
            "枝が水平に並ぶ、いまのナンヨウスギに似た古い針葉樹。乾いた土地でも育つことができた。",
            "An early conifer with neat tiers of level branches, like a modern monkey-puzzle relative. It could grow on dry ground.",
            "枝条水平分层排列、类似现代南洋杉的古老针叶树，能在干燥的土地上生长。",
        ),
        fact: L("種が殻に守られているので、湿地を離れて乾いた土地にも森を広げられた。", "Its protected seeds let forests spread away from swamps onto dry land.", "种子受到保护，使森林得以离开湿地，扩展到干燥的土地上。"),
    },
    Species {
        slug: "eryops",
        name: L("エリオプス", "Eryops", "引螈"),
        sci: "Eryops megacephalus",
        group: L("両生類（分椎類）", "Amphibian (temnospondyl)", "两栖类（离片椎类）"),
        len_cm: 200.0, t0: 297.0, t1: 290.0, hab: TIDAL | LAND, regions: &[Site(S_TEXAS)],
        beh: Beh::Crawl, art: Art::Tetrapod(5), col: [0.44, 0.40, 0.32], col2: [0.62, 0.56, 0.42], rarity: 1.2, shy: 1.5, speed: 0.08,
        desc: L(
            "体長2mの、がっしりした両生類。大きな頭で、水辺の魚や小動物を食べた。",
            "A heavy-set amphibian about 2 m long whose big head snapped up fish and small animals at the water's edge.",
            "体长2米、体格粗壮的两栖动物，用大脑袋捕食水边的鱼和小动物。",
        ),
        fact: L("当時の陸で、最大級の捕食者の一つ。", "One of the largest land predators of its time.", "是当时陆地上最大的捕食者之一。"),
    },
    Species {
        slug: "diplocaulus",
        name: L("ディプロカウルス", "Diplocaulus", "笠头螈"),
        sci: "Diplocaulus magnicornis",
        group: L("両生類（空椎類）", "Amphibian (lepospondyl)", "两栖类（壳椎类）"),
        len_cm: 100.0, t0: 290.0, t1: 272.0, hab: TIDAL | LAND, regions: &[Site(S_TEXAS), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Tetrapod(6), col: [0.44, 0.46, 0.38], col2: [0.70, 0.66, 0.52], rarity: 1.5, shy: 1.2, speed: 0.06,
        desc: L(
            "ブーメランのような三角の頭を持つ両生類。川や池の底で暮らした。",
            "An amphibian with a boomerang-shaped head that lived on the beds of rivers and ponds.",
            "长着回旋镖状三角形脑袋的两栖动物，生活在河流和池塘底部。",
        ),
        fact: L("頭の形は、水の流れの中で体を持ち上げる翼の役目をしたのかもしれない。", "Its head may have worked like a wing, giving lift in flowing water.", "它的头可能起到翅膀的作用，在水流中提供升力。"),
    },
    Species {
        slug: "dimetrodon",
        name: L("ディメトロドン", "Dimetrodon", "异齿龙"),
        sci: "Dimetrodon grandis",
        group: L("単弓類（盤竜類）", "Synapsid (pelycosaur)", "合弓纲（盘龙类）"),
        len_cm: 300.0, t0: 295.0, t1: 272.0, hab: LAND, regions: &[Site(S_TEXAS), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Tetrapod(4), col: [0.50, 0.36, 0.28], col2: [0.82, 0.52, 0.36], rarity: 1.0, shy: 2.0, speed: 0.12,
        desc: L(
            "背中に大きな帆を持つ、ペルム紀前期の陸の王者。恐竜ではなく、わたしたち哺乳類につながる単弓類の仲間。",
            "The sail-backed top predator of the Early Permian. Not a dinosaur, but a synapsid — on the line leading to mammals like us.",
            "背上长着大帆的早二叠世陆地霸主。它不是恐龙，而是通向哺乳动物（包括我们）的合弓纲动物。",
        ),
        fact: L("名前は「2種類の歯」という意味。役割の違う歯を持っていた。", "Its name means 'two measures of teeth': it had teeth of different shapes for different jobs.", "名字意为“两种牙齿”，它拥有分工不同的牙齿。"),
    },
    Species {
        slug: "helicoprion",
        name: L("ヘリコプリオン", "Helicoprion", "旋齿鲨"),
        sci: "Helicoprion",
        group: L("軟骨魚類（ギンザメの仲間）", "Ratfish relative", "软骨鱼类（银鲛类）"),
        len_cm: 600.0, t0: 290.0, t1: 270.0, hab: SHELF | DEEP, regions: &[Global],
        beh: Beh::Swim, art: Art::Shark(2), col: [0.40, 0.44, 0.48], col2: [0.82, 0.82, 0.78], rarity: 0.6, shy: 3.0, speed: 0.45,
        desc: L(
            "下あごに、うずまき状の歯が並んだ魚。うずまきでイカなどのやわらかい獲物を切り裂いた。",
            "A fish with a spiral whorl of teeth in its lower jaw, used to slice soft prey such as squid.",
            "下颌长着螺旋状齿轮的鱼，用它切割鱿鱼等柔软的猎物。",
        ),
        fact: L("化石はうずまきの歯しか見つからず、どこについていたか100年近くなぞだった。", "Only the tooth whorls were found, and for nearly a century nobody knew where they went.", "化石只发现了螺旋齿，近百年来人们都不知道它长在哪里。"),
    },
    Species {
        slug: "parafusulina",
        name: L("パラフズリナ（紡錘虫）", "Parafusulina (fusulinid)", "拟纺锤蜓"),
        sci: "Parafusulina",
        group: L("有孔虫（紡錘虫）", "Foraminifer (fusulinid)", "有孔虫（蜓类）"),
        len_cm: 5.0, t0: 280.0, t1: 260.0, hab: SHALLOW, regions: &[Tropics, Site(S_CAPITAN)],
        beh: Beh::Sessile, art: Art::Fusulinid, col: [0.90, 0.84, 0.70], col2: [0.70, 0.60, 0.48], rarity: 4.0, shy: 0.0, speed: 0.0,
        desc: L(
            "米つぶ形の殻を持つ、ひとつの細胞の生きもの（有孔虫）。大きなものは10cmにもなった。",
            "A single-celled organism (a foraminifer) with a rice-grain-shaped shell. The biggest reached 10 cm.",
            "外壳呈米粒形的单细胞生物（有孔虫），大的可达10厘米。",
        ),
        fact: L("日本の石灰岩からもたくさん見つかり、地層の年代を決める手がかりになる。", "They are common in Japanese limestones too, and help date the rocks.", "在日本的石灰岩中也大量发现，是确定地层年代的线索。"),
    },
    Species {
        slug: "ditomopyge",
        name: L("ディトモピゲ", "Ditomopyge", "双切尾虫"),
        sci: "Ditomopyge",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 3.0, t0: 300.0, t1: 252.0, hab: SHELF | SHALLOW, regions: &[Global],
        beh: Beh::Crawl, art: Art::Trilobite { w: 0.75, spine: 0.15, pyg: 0.4, segs: 9 }, col: [0.54, 0.46, 0.38], col2: [0.34, 0.28, 0.24], rarity: 1.5, shy: 0.0, speed: 0.03,
        desc: L(
            "最後まで生き残った三葉虫の仲間の一つ。2億7000万年以上続いた三葉虫は、ペルム紀の終わりに姿を消した。",
            "One of the last trilobites. After more than 270 million years, trilobites vanished at the end of the Permian.",
            "最后幸存的三叶虫之一。延续了2.7亿多年的三叶虫在二叠纪末消失了。",
        ),
        fact: L("古生代の始まりから終わりまで生きた三葉虫は、古生代を代表する動物。", "Living from the start of the Paleozoic to its end, trilobites are the era's signature animals.", "从古生代开始一直生存到结束的三叶虫，是古生代的代表动物。"),
    },
    Species {
        slug: "inostrancevia",
        name: L("イノストランケビア", "Inostrancevia", "狼蜥兽"),
        sci: "Inostrancevia alexandri",
        group: L("単弓類（ゴルゴノプス類）", "Synapsid (gorgonopsian)", "合弓纲（丽齿兽类）"),
        len_cm: 350.0, t0: 259.0, t1: 252.0, hab: LAND, regions: &[Site(S_VYATKA), Plate(BALTICA)],
        beh: Beh::Crawl, art: Art::Tetrapod(8), col: [0.52, 0.42, 0.32], col2: [0.80, 0.72, 0.58], rarity: 0.8, shy: 2.5, speed: 0.18,
        desc: L(
            "長さ12cmもある剣のような犬歯を持つ、ペルム紀後期の大型の捕食者。",
            "A large Late Permian predator with sabre-like canine teeth up to 12 cm long.",
            "晚二叠世的大型捕食者，长着长达12厘米的军刀般犬齿。",
        ),
        fact: L("哺乳類の遠い親戚で、ゴルゴノプス類の中で最大級。", "A distant relative of mammals and one of the largest gorgonopsians.", "是哺乳动物的远亲，也是最大的丽齿兽类之一。"),
    },
    Species {
        slug: "scutosaurus",
        name: L("スクトサウルス", "Scutosaurus", "盾甲龙"),
        sci: "Scutosaurus karpinskii",
        group: L("爬虫類（パレイアサウルス類）", "Reptile (pareiasaur)", "爬行动物（锯齿龙类）"),
        len_cm: 250.0, t0: 264.0, t1: 252.0, hab: LAND, regions: &[Site(S_VYATKA), Plate(BALTICA)],
        beh: Beh::Crawl, art: Art::Tetrapod(9), col: [0.56, 0.48, 0.38], col2: [0.40, 0.34, 0.28], rarity: 1.2, shy: 1.5, speed: 0.06,
        desc: L(
            "体中を骨のこぶでおおった、どっしりした植物食の爬虫類。ほおには、とげのような突起があった。",
            "A bulky plant-eating reptile studded with bony knobs, with spiky projections on its cheeks.",
            "全身覆盖骨瘤、体态敦实的植食性爬行动物，脸颊上有刺状突起。",
        ),
        fact: L("イノストランケビアに狙われる獲物だったと考えられている。", "It was probably prey for Inostrancevia.", "据推测它是狼蜥兽的猎物。"),
    },
    Species {
        slug: "lystrosaurus",
        name: L("リストロサウルス", "Lystrosaurus", "水龙兽"),
        sci: "Lystrosaurus",
        group: L("単弓類（ディキノドン類）", "Synapsid (dicynodont)", "合弓纲（二齿兽类）"),
        len_cm: 100.0, t0: 253.0, t1: T_END, hab: LAND, regions: &[Site(S_KAROO), Plate(GONDWANA)],
        beh: Beh::Crawl, art: Art::Tetrapod(10), col: [0.56, 0.50, 0.40], col2: [0.40, 0.36, 0.30], rarity: 2.0, shy: 1.2, speed: 0.08,
        desc: L(
            "ブタくらいの大きさで、くちばしと2本の牙を持つ植物食の動物。穴を掘って暮らしたらしい。",
            "A pig-sized plant-eater with a beak and two tusks that probably lived in burrows.",
            "体型如猪、长着喙和两根獠牙的植食动物，可能挖洞而居。",
        ),
        fact: L(
            "史上最大の絶滅を生きのびて、三畳紀のはじめには陸の動物のほとんどがこの仲間になった。",
            "It survived the greatest extinction of all; in the earliest Triassic, most land animals were Lystrosaurus.",
            "它在史上最大的灭绝中幸存下来，三叠纪初期陆地动物大多是它的同类。",
        ),
    },
];

/// 陸に降りて撮る景色の種類
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Scenery {
    /// 植物のない大地
    Barren,
    /// 氷河
    Ice,
    /// 水辺にだけ小さな緑がある
    GreenShore,
    /// デボン紀の森
    Forest,
    /// 石炭紀の湿地林
    CoalForest,
    /// ペルム紀の砂漠
    Desert,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PhKind {
    /// 上空から大陸の全体を撮る（大陸の番号の組）
    Continent(&'static [usize]),
    /// 上空から大噴火を撮る（ERUPTIONS の番号）
    Eruption(usize),
    /// 陸に降りて景色を撮る
    Scenery(Scenery),
}

/// 上空・陸上から撮る現象
pub struct Phenom {
    pub slug: &'static str,
    pub name: L,
    pub desc: L,
    pub hint: L,
    pub kind: PhKind,
    /// 撮れる時代（百万年前、古い方から）
    pub t0: f32,
    pub t1: f32,
}

impl Phenom {
    pub fn at(&self, t: f32) -> bool {
        t <= self.t0 + 1e-3 && t >= self.t1 - 1e-3
    }
}

pub const P_GONDWANA: usize = 0;
pub const P_LAURENTIA: usize = 1;
pub const P_ERUPTION: usize = 2;
pub const P_BARREN: usize = 3;
pub const P_HIRNANTIAN_ICE: usize = 4;
pub const P_GREEN_SHORE: usize = 5;
pub const P_EURAMERICA: usize = 6;
pub const P_FIRST_FOREST: usize = 7;
pub const P_COAL_FOREST: usize = 8;
pub const P_GONDWANA_ICE: usize = 9;
pub const P_PANGAEA: usize = 10;
pub const P_DESERT: usize = 11;
pub const P_SIBERIAN_TRAPS: usize = 12;

/// パンゲアをつくる大陸
pub const PANGAEA_PLATES: &[usize] = &[GONDWANA, LAURENTIA, BALTICA, AVALONIA, SIBERIA, KAZAKH, TARIM];

pub static PHENOMS: &[Phenom] = &[
    Phenom {
        slug: "gondwana",
        kind: PhKind::Continent(&[GONDWANA]),
        t0: T_START,
        t1: 306.0,
        name: L("ゴンドワナ大陸", "Gondwana", "冈瓦纳大陆"),
        desc: L(
            "南極のあたりから赤道まで広がる巨大な大陸。いまのアフリカ・南アメリカ・南極・オーストラリア・インドなどが一つにつながっていた。",
            "A giant continent stretching from near the South Pole to the equator, joining today's Africa, South America, Antarctica, Australia, India and more.",
            "从南极附近一直延伸到赤道的巨大陆地，今天的非洲、南美洲、南极洲、澳大利亚、印度等连成一片。",
        ),
        hint: L("上空から、大陸全体が写るように撮影する", "Photograph the whole continent from the sky", "从高空拍下整个大陆"),
    },
    Phenom {
        slug: "laurentia",
        kind: PhKind::Continent(&[LAURENTIA]),
        t0: T_START,
        t1: 421.0,
        name: L("ローレンシア大陸", "Laurentia", "劳伦大陆"),
        desc: L(
            "赤道にまたがる大陸で、のちの北アメリカ。海面が高く、大陸の上まで浅い海が入り込んでいた。",
            "A continent straddling the equator that will become North America. High sea level floods much of it with shallow seas.",
            "横跨赤道的大陆，后来成为北美洲。海平面很高，浅海一直延伸到大陆内部。",
        ),
        hint: L("上空から、大陸全体が写るように撮影する", "Photograph the whole continent from the sky", "从高空拍下整个大陆"),
    },
    Phenom {
        slug: "eruption",
        kind: PhKind::Eruption(0),
        t0: 511.6,
        t1: 509.9,
        name: L("カルカリンジの大噴火", "Kalkarindji eruption", "卡尔卡林吉大喷发"),
        desc: L(
            "約5億1100万年前、いまのオーストラリア北部で起きた巨大な溶岩の噴出（巨大火成岩岩石区）。広さは日本の5倍以上。噴き出したガスが地球を温め、海の酸素を減らし、生きものの絶滅を引き起こしたと考えられている。",
            "About 511 million years ago, vast floods of lava poured out over what is now northern Australia, covering more than five times the area of Japan. The gases probably warmed the planet, starved the seas of oxygen and caused extinctions.",
            "约5亿1100万年前发生在今澳大利亚北部的巨量熔岩喷发（大火成岩省），面积超过日本的5倍。喷出的气体使地球变暖、海洋缺氧，被认为引发了生物灭绝。",
        ),
        hint: L("噴火の時代に、上空から近づいて撮影する", "During the eruptions, fly close and photograph them from the sky", "在喷发时期，从高空靠近拍摄"),
    },
    Phenom {
        slug: "barren-land",
        kind: PhKind::Scenery(Scenery::Barren),
        t0: T_START,
        t1: 433.0,
        name: L("植物のない大地", "A land without plants", "没有植物的大地"),
        desc: L(
            "カンブリア紀の陸には、まだ植物も動物もいない。むき出しの岩と砂が広がり、雨が降ると土砂がそのまま海へ流れ込んだ。",
            "Cambrian land has no plants or animals yet. Bare rock and sand stretch everywhere, and rain washes sediment straight into the sea.",
            "寒武纪的陆地上还没有植物和动物。裸露的岩石和沙土一望无际，下雨时泥沙直接流入大海。",
        ),
        hint: L("陸に降りて、景色を撮影する", "Land on the continent and photograph the scenery", "降落到陆地上拍摄风景"),
    },
    Phenom {
        slug: "hirnantian-ice",
        kind: PhKind::Scenery(Scenery::Ice),
        t0: 445.2,
        t1: 443.8,
        name: L("ヒルナンシアンの氷床", "Hirnantian ice sheet", "赫南特冰盖"),
        desc: L(
            "オルドビス紀の終わり、南極の上にあったゴンドワナ大陸に厚い氷床が広がった。海の水が氷になって海面が下がり、浅い海が干上がった。",
            "At the end of the Ordovician, thick ice spread over Gondwana at the South Pole. So much water was locked up as ice that sea level fell and shallow seas dried up.",
            "奥陶纪末，位于南极的冈瓦纳大陆上覆盖了厚厚的冰盖。大量海水变成冰，海平面下降，浅海干涸。",
        ),
        hint: L("4億4500万年前ごろ、南極に近いゴンドワナの陸に降りて撮影する", "Around 445 million years ago, land on Gondwana near the South Pole and photograph the ice", "约4亿4500万年前，降落在靠近南极的冈瓦纳大陆上拍摄"),
    },
    Phenom {
        slug: "green-shore",
        kind: PhKind::Scenery(Scenery::GreenShore),
        t0: 433.0,
        t1: 386.0,
        name: L("最初の緑の岸辺", "The first green shores", "最早的绿色岸边"),
        desc: L(
            "シルル紀からデボン紀のはじめ、背の低い植物が水辺にだけ緑のじゅうたんを広げた。少し内陸に入ると、まだ岩と砂の大地が続いていた。",
            "From the Silurian into the early Devonian, low plants carpeted only the water's edge in green. A little way inland, bare rock and sand still stretched on.",
            "从志留纪到泥盆纪初期，低矮的植物只在水边铺开绿色地毯。稍往内陆，仍是岩石和沙土的大地。",
        ),
        hint: L("シルル紀〜デボン紀前期に、陸に降りて景色を撮影する", "In the Silurian or Early Devonian, land and photograph the scenery", "在志留纪至早泥盆世降落到陆地上拍摄风景"),
    },
    Phenom {
        slug: "euramerica",
        kind: PhKind::Continent(&[LAURENTIA, BALTICA, AVALONIA]),
        t0: 420.0,
        t1: 306.0,
        name: L("ユーラメリカ大陸", "Euramerica", "欧美大陆"),
        desc: L(
            "ローレンシア・バルティカ・アヴァロニアがぶつかってできた大陸。ぶつかったところにカレドニア山脈がそびえ、山から流れた赤い砂が積もったので「オールド・レッド大陸」とも呼ばれる。",
            "The continent formed when Laurentia, Baltica and Avalonia collided, raising the Caledonian Mountains. Red sand washed from those mountains earned it the name 'Old Red Continent'.",
            "由劳伦、波罗的和阿瓦隆尼亚碰撞形成的大陆。碰撞处耸立着加里东山脉，山上冲下的红色砂层使它又名“老红砂岩大陆”。",
        ),
        hint: L("デボン紀に、上空から大陸全体が写るように撮影する", "In the Devonian, photograph the whole continent from the sky", "在泥盆纪从高空拍下整个大陆"),
    },
    Phenom {
        slug: "first-forest",
        kind: PhKind::Scenery(Scenery::Forest),
        t0: 385.0,
        t1: 358.9,
        name: L("最初の森", "The first forests", "最早的森林"),
        desc: L(
            "デボン紀の後半、アーケオプテリスなどの高い木が地球に初めて森をつくった。木の根が岩をくだいて土を生み、川や海の環境まで変えていった。",
            "In the Late Devonian, tall trees such as Archaeopteris formed the planet's first forests. Their roots broke rock into soil and changed even the rivers and seas.",
            "泥盆纪晚期，古羊齿等高大树木在地球上第一次形成森林。树根把岩石破碎成土壤，甚至改变了河流和海洋的环境。",
        ),
        hint: L("デボン紀の後半に、陸に降りて森を撮影する", "In the Late Devonian, land and photograph the forest", "在泥盆纪晚期降落到陆地上拍摄森林"),
    },
    Phenom {
        slug: "coal-forest",
        kind: PhKind::Scenery(Scenery::CoalForest),
        t0: 340.0,
        t1: 300.0,
        name: L("石炭紀の湿地林", "Carboniferous coal swamp", "石炭纪沼泽森林"),
        desc: L(
            "赤道近くの低地に、鱗木やカラミテスの大森林と沼地が広がった。倒れた木は腐りきらずに積み重なり、やがて石炭になった。",
            "Vast swamp forests of scale trees and Calamites spread over the lowlands near the equator. Fallen trees piled up without fully rotting and eventually became coal.",
            "赤道附近的低地上遍布鳞木和芦木组成的大片沼泽森林。倒下的树木未能完全腐烂而层层堆积，最终变成了煤。",
        ),
        hint: L("石炭紀に、赤道に近い陸に降りて撮影する", "In the Carboniferous, land near the equator and photograph the swamp forest", "在石炭纪降落到赤道附近的陆地上拍摄"),
    },
    Phenom {
        slug: "gondwana-ice",
        kind: PhKind::Scenery(Scenery::Ice),
        t0: 340.0,
        t1: 285.0,
        name: L("ゴンドワナの大氷河", "Gondwana's great glaciers", "冈瓦纳大冰川"),
        desc: L(
            "石炭紀からペルム紀のはじめ、南のゴンドワナ大陸を大きな氷床がおおった（後期古生代氷河時代）。氷が増えたり減ったりするたびに、海面も大きく上下した。",
            "From the Carboniferous into the early Permian, great ice sheets covered southern Gondwana (the Late Paleozoic Ice Age). As the ice grew and shrank, sea level rose and fell.",
            "从石炭纪到二叠纪初期，巨大的冰盖覆盖了南方的冈瓦纳大陆（晚古生代冰期）。冰量增减，海平面也随之大幅升降。",
        ),
        hint: L("石炭紀〜ペルム紀前期に、南極に近いゴンドワナの陸に降りて撮影する", "In the Carboniferous–Early Permian, land on Gondwana near the South Pole", "在石炭纪至早二叠世降落在靠近南极的冈瓦纳大陆上拍摄"),
    },
    Phenom {
        slug: "pangaea",
        kind: PhKind::Continent(PANGAEA_PLATES),
        t0: 300.0,
        t1: T_END,
        name: L("超大陸パンゲア", "Supercontinent Pangaea", "盘古超大陆"),
        desc: L(
            "ほとんどすべての大陸がぶつかり合ってできた一つの超大陸。北極から南極までつながり、東にはテチス海が大きな湾のように入り込んでいた。",
            "A single supercontinent formed as almost every landmass collided, stretching from pole to pole, with the Tethys Sea cutting in from the east like a huge bay.",
            "几乎所有大陆相互碰撞形成的单一超大陆，从北极延伸到南极，东边的特提斯海像巨大的海湾一样深入其中。",
        ),
        hint: L("ペルム紀に、上空から超大陸全体が写るように撮影する", "In the Permian, photograph the whole supercontinent from the sky", "在二叠纪从高空拍下整个超大陆"),
    },
    Phenom {
        slug: "permian-desert",
        kind: PhKind::Scenery(Scenery::Desert),
        t0: 292.0,
        t1: T_END,
        name: L("パンゲアの砂漠", "Deserts of Pangaea", "盘古大陆的沙漠"),
        desc: L(
            "海から遠いパンゲアの内陸は、雨がほとんど降らない砂漠になった。赤い砂丘が広がり、夏は焼けつくように暑かった。",
            "Far from the sea, Pangaea's interior became desert with almost no rain. Red dunes stretched away under scorching summers.",
            "远离海洋的盘古大陆内陆几乎不下雨，变成了沙漠。红色沙丘绵延，夏季酷热难当。",
        ),
        hint: L("ペルム紀に、赤道に近いパンゲアの陸に降りて撮影する", "In the Permian, land in Pangaea's tropical interior and photograph the desert", "在二叠纪降落到靠近赤道的盘古大陆上拍摄"),
    },
    Phenom {
        slug: "siberian-traps",
        kind: PhKind::Eruption(1),
        t0: 252.6,
        t1: 251.9,
        name: L("シベリアの大噴火", "Siberian Traps eruption", "西伯利亚暗色岩大喷发"),
        desc: L(
            "ペルム紀の終わり、シベリアで地球史上最大級の溶岩が噴き出した。噴き出したガスは地球を温め、海を酸性で酸素の少ないものに変え、生きものの大部分を絶滅させた。",
            "At the end of the Permian, one of the greatest lava outpourings in Earth's history burst out in Siberia. Its gases heated the planet and made the oceans acidic and oxygen-starved, wiping out most life.",
            "二叠纪末，西伯利亚喷发出地球史上规模最大的熔岩之一。喷出的气体使地球变暖，海洋酸化并严重缺氧，导致绝大部分生物灭绝。",
        ),
        hint: L("2億5200万年前ごろ、上空から噴火に近づいて撮影する", "Around 252 million years ago, fly close and photograph the eruption from the sky", "约2亿5200万年前，从高空靠近拍摄喷发"),
    },
];

/// 図鑑の項目の画像の名前
pub fn entry_slug(e: usize) -> &'static str {
    if e < SPECIES.len() {
        SPECIES[e].slug
    } else {
        PHENOMS[e - SPECIES.len()].slug
    }
}

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

/// その場所・その時代の陸の景色
pub fn scenery_at(spot: &Spot, t: f32) -> Option<Scenery> {
    if spot.hab != Habitat::Land {
        return None;
    }
    // 古生代の氷河は南極に近いゴンドワナ大陸にあった
    if glacial(t) && spot.lat < -55.0 {
        return Some(Scenery::Ice);
    }
    if t >= 433.0 {
        return Some(Scenery::Barren);
    }
    if arid(t, spot.lat) {
        return Some(Scenery::Desert);
    }
    if veg_at(t) < 0.6 {
        return Some(Scenery::GreenShore);
    }
    if (300.0..=340.0).contains(&t) && spot.lat.abs() < 30.0 {
        return Some(Scenery::CoalForest);
    }
    if (358.9..=385.0).contains(&t) {
        return Some(Scenery::Forest);
    }
    None
}

/// その景色の写真になる現象
pub fn scenery_phenoms(k: Scenery, t: f32) -> impl Iterator<Item = usize> {
    PHENOMS.iter().enumerate().filter(move |(_, p)| p.kind == PhKind::Scenery(k) && p.at(t)).map(|(i, _)| i)
}

/// 現象を見に行く時代と場所（経緯度）
pub fn phenom_where_when(i: usize, earth: &Earth, now: f32, near: (f32, f32)) -> (f32, (f32, f32)) {
    let ph = &PHENOMS[i];
    let mid = ((ph.t0 + ph.t1) * 0.5 * 2.0).round() / 2.0;
    let t = if ph.at(now) { now } else { mid };
    match ph.kind {
        PhKind::Continent(plates) => {
            let (mut x, mut y, mut n) = (0.0, 0.0, 0.0);
            for &p in plates {
                let (lon, lat, _) = earth.pose(p, t);
                let w = PLATES[p].outline.len() as f32;
                x += lon * w;
                y += lat * w;
                n += w;
            }
            (t, (x / n, y / n))
        }
        PhKind::Eruption(e) => {
            let er = &ERUPTIONS[e];
            let t = (er.t0 + er.t1) * 0.5;
            (t, earth.site_lonlat(er.site, t))
        }
        PhKind::Scenery(k) => {
            let mut best: Option<(f32, (f32, f32))> = None;
            let mut lat = -88.0;
            while lat <= 88.0 {
                let mut lon = -179.0;
                while lon < 180.0 {
                    let spot = earth.sample(lon, lat, t);
                    if scenery_at(&spot, t) == Some(k) {
                        let ok = [(0.6, 0.0), (-0.6, 0.0), (0.0, 0.6), (0.0, -0.6)].iter().all(|&(dx, dy)| earth.sample(lon + dx, lat + dy, t).hab == Habitat::Land);
                        let score = deg_dist(near, (lon, lat)) + if ok { 0.0 } else { 30.0 };
                        if best.map_or(true, |b| score < b.0) {
                            best = Some((score, (lon, lat)));
                        }
                    }
                    lon += 1.0;
                }
                lat += 1.0;
            }
            (t, best.map_or(near, |b| b.1))
        }
    }
}

/// 図鑑の項目が載る巻（紀）。現れてから少しのあいだ（最大1000万年）の真ん中の時代で決める
pub fn entry_period(e: usize) -> usize {
    let (t0, t1) = if e < SPECIES.len() { (SPECIES[e].t0, SPECIES[e].t1) } else { (PHENOMS[e - SPECIES.len()].t0, PHENOMS[e - SPECIES.len()].t1) };
    period_at((t0 + t1.max(t0 - 10.0)) * 0.5)
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
        for now in [535.0, 470.0, 400.0, 320.0, 260.0] {
            for (i, sp) in SPECIES.iter().enumerate() {
                let (t, (lon, lat)) = where_when(i, &e, now, (40.0, 5.0));
                let spot = e.sample(lon, lat, t);
                assert!(lives_here(sp, &e, &spot, t), "{} not found at {:.1},{:.1} t={} ({:?}) now={}", sp.name.1, lon, lat, t, spot.hab, now);
            }
        }
    }

    /// 現象はすべて、見に行けば撮れる
    #[test]
    fn every_phenom_can_be_found() {
        let e = Earth::new();
        for (i, ph) in PHENOMS.iter().enumerate() {
            let (t, (lon, lat)) = phenom_where_when(i, &e, 535.0, (40.0, 5.0));
            assert!(ph.at(t), "{} t={}", ph.name.1, t);
            if let PhKind::Scenery(k) = ph.kind {
                let spot = e.sample(lon, lat, t);
                assert_eq!(scenery_at(&spot, t), Some(k), "{} at {:.1},{:.1} t={}", ph.name.1, lon, lat, t);
                assert!(scenery_phenoms(k, t).any(|j| j == i));
            }
        }
    }

    /// 図鑑の各巻に、生きものと現象がある
    #[test]
    fn every_period_has_entries() {
        for p in 0..PERIODS.len() {
            let n = (0..n_entries()).filter(|&e| entry_period(e) == p).count();
            assert!(n >= 8, "period {} has {}", PERIODS[p].name.1, n);
        }
        assert_eq!(entry_period(0), CAMBRIAN);
        let find = |slug: &str| (0..n_entries()).find(|&e| entry_slug(e) == slug).unwrap();
        assert_eq!(entry_period(find("mesosaurus")), PERMIAN);
        assert_eq!(entry_period(find("pangaea")), PERMIAN);
        assert_eq!(entry_period(find("crinoid")), ORDOVICIAN);
        assert_eq!(entry_period(find("prototaxites")), DEVONIAN);
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
