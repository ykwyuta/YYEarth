//! 古生代・中生代（カンブリア紀〜白亜紀）の生きもの（図鑑の項目）と、上空・陸から撮る現象
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
    /// 貝 0=巻貝 1=二枚貝
    Mollusc(u8),
    Starfish,
    /// 条鰭類（ふつうの魚の形）
    RayFin,
    /// カブトガニの仲間
    Limulid,
    /// ウニ
    Urchin,
    // ---- 中生代
    /// 恐竜 0=小型の肉食 1=大型の肉食 2=帆のある肉食 3=竜脚類 4=古い竜脚形類 5=剣竜 6=鎧竜
    /// 7=大型の角竜 8=小型の角竜 9=鳥脚類 10=とさかのある鳥脚類 11=羽毛の小型肉食 13=首を上げた竜脚類
    Dino(u8),
    /// 海の爬虫類 0=魚竜 1=首長竜・偽竜 2=板歯類 3=首の短い首長竜 4=モササウルス 5=ウミガメ
    Marine(u8),
    /// 翼竜 0=尾の長い小型 1=とさかのある大型 2=地上を歩く大型
    Pterosaur(u8),
    /// 鳥 0=始祖鳥 1=四枚の翼のミクロラプトル 2=尾羽の長い鳥
    Bird(u8),
    /// 哺乳類など 0=トガリネズミ形 1=平たい尾 2=アナグマ形 3=キノドン類
    Mammal(u8),
    /// ワニの仲間など 0=ラウイスクス類 1=アエトサウルス類 2=ワニ 3=タニストロフェウス
    Archosaur(u8),
}

impl Art {
    /// 海底に平たく置いて斜め上から見る描き方か
    pub fn dorsal(self) -> bool {
        matches!(self, Art::Trilobite { .. } | Art::Agnostus | Art::Wiwaxia | Art::Halkieria | Art::Fuxianhuia | Art::Sidneyia | Art::Slug | Art::Trace | Art::Mat | Art::Starfish | Art::Limulid)
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
        len_cm: 4.0, t0: 460.0, t1: 420.0, hab: SHALLOW | SHELF, regions: &[Global],
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
        len_cm: 8.0, t0: 460.0, t1: 443.8, hab: SHALLOW, regions: &[Site(S_CINCINNATI), Tropics],
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
        len_cm: 2.0, t0: 470.0, t1: T_END, hab: TIDAL | LAND, regions: &[Global],
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
        len_cm: 30.0, t0: 445.0, t1: 300.0, hab: SHALLOW, regions: &[Tropics],
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
        len_cm: 1000.0, t0: 340.0, t1: 270.0, hab: LAND | TIDAL, regions: &[Tropics],
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
        len_cm: 3.0, t0: 315.0, t1: 260.0, hab: LAND, regions: &[Tropics],
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
        len_cm: 100.0, t0: 253.0, t1: 249.0, hab: LAND, regions: &[Site(S_KAROO), Plate(GONDWANA)],
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
    // ======================================================== どこにでもいた生きもの（各紀の海や陸に広く出会う）
    // ---- オルドビス紀
    Species {
        slug: "maclurites",
        name: L("マクルリテス", "Maclurites", "马克鲁螺"),
        sci: "Maclurites",
        group: L("腹足類（巻貝）", "Gastropod (sea snail)", "腹足类（海螺）"),
        len_cm: 8.0, t0: 465.0, t1: 445.0, hab: SHALLOW, regions: &[Tropics],
        beh: Beh::Crawl, art: Art::Mollusc(0), col: [0.74, 0.66, 0.54], col2: [0.50, 0.42, 0.34], rarity: 2.5, shy: 0.0, speed: 0.01,
        desc: L(
            "平たく巻いた殻を持つ大きな巻貝。殻の口には厚いふたがあり、暖かい浅い海の底をゆっくり這った。",
            "A large sea snail with a flat-coiled shell and a thick lid, creeping slowly over warm shallow sea floors.",
            "外壳扁平盘绕的大型海螺，壳口有厚盖，在温暖浅海底缓慢爬行。",
        ),
        fact: L("殻が左巻きに見える、めずらしい巻貝。", "Its shell appears to coil the 'wrong' way, which is unusual for snails.", "外壳看起来是左旋的，在海螺中很少见。"),
    },
    Species {
        slug: "ambonychia",
        name: L("アンボニキア", "Ambonychia", "钩翼蛤"),
        sci: "Ambonychia",
        group: L("二枚貝", "Bivalve", "双壳类"),
        len_cm: 5.0, t0: 470.0, t1: 443.8, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Sessile, art: Art::Mollusc(1), col: [0.62, 0.56, 0.48], col2: [0.80, 0.74, 0.62], rarity: 2.5, shy: 0.0, speed: 0.0,
        desc: L(
            "糸で海底に体を留めて暮らした二枚貝。殻のすき間から海水を吸って、餌をこし取った。",
            "A bivalve that anchored itself to the sea floor with threads and filtered food from the water.",
            "用足丝把身体固定在海底的双壳类，从壳缝中吸水滤食。",
        ),
        fact: L("二枚貝はオルドビス紀に大きくふえ、いまも海にたくさんいる。", "Bivalves spread widely in the Ordovician and are still abundant today.", "双壳类在奥陶纪大量增加，至今仍遍布海洋。"),
    },
    Species {
        slug: "orthis",
        name: L("オルティス", "Orthis", "正形贝"),
        sci: "Orthis",
        group: L("腕足動物", "Brachiopod", "腕足动物"),
        len_cm: 3.0, t0: 482.0, t1: 458.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Sessile, art: Art::Brachiopod, col: [0.72, 0.64, 0.52], col2: [0.88, 0.80, 0.66], rarity: 3.5, shy: 0.0, speed: 0.0,
        desc: L(
            "放射状のすじが並ぶ殻を持つ腕足動物。オルドビス紀の海底は、こうした殻でいっぱいだった。",
            "A brachiopod with strongly ribbed shells. Ordovician sea floors were covered with shells like these.",
            "外壳带有放射状肋纹的腕足动物。奥陶纪的海底布满了这样的壳。",
        ),
        fact: L("古生代の海では、二枚貝より腕足動物のほうがずっと多かった。", "In Paleozoic seas, brachiopods were far more common than bivalves.", "在古生代海洋中，腕足动物比双壳类多得多。"),
    },
    Species {
        slug: "triarthrus",
        name: L("トリアルトルス", "Triarthrus", "三分节虫"),
        sci: "Triarthrus eatoni",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 3.0, t0: 458.0, t1: 444.0, hab: SHELF | DEEP, regions: &[Plate(LAURENTIA), Plate(BALTICA), Plate(AVALONIA)],
        beh: Beh::Crawl, art: Art::Trilobite { w: 0.65, spine: 0.0, pyg: 0.15, segs: 14 }, col: [0.42, 0.40, 0.36], col2: [0.26, 0.24, 0.22], rarity: 3.0, shy: 0.0, speed: 0.03,
        desc: L(
            "酸素の少ない泥の海底で群れていた小さな三葉虫。体の細かい節が多い。",
            "A small trilobite that swarmed on oxygen-poor muddy sea floors, with many narrow body segments.",
            "成群生活在缺氧泥质海底的小型三叶虫，身体有许多细节。",
        ),
        fact: L("脚や触角まで黄鉄鉱に置きかわった、金色の化石が見つかっている。", "Some fossils preserve even the legs and antennae in golden pyrite.", "发现过连腿和触角都被黄铁矿置换的金色化石。"),
    },
    Species {
        slug: "neseuretus",
        name: L("ネセウレトゥス", "Neseuretus", "新绕虫"),
        sci: "Neseuretus",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 6.0, t0: 472.0, t1: 455.0, hab: SHALLOW | SHELF, regions: &[Plate(GONDWANA), Plate(AVALONIA)],
        beh: Beh::Crawl, art: Art::Trilobite { w: 0.8, spine: 0.15, pyg: 0.3, segs: 13 }, col: [0.58, 0.48, 0.38], col2: [0.36, 0.30, 0.24], rarity: 3.0, shy: 0.0, speed: 0.04,
        desc: L(
            "ゴンドワナの冷たい浅い海の砂地に多かった三葉虫。大きな頭で砂を掘りながら進んだ。",
            "A trilobite common on the sandy floors of Gondwana's cool shallow seas, ploughing through sand with its broad head.",
            "冈瓦纳寒冷浅海沙底常见的三叶虫，用宽大的头部推开沙子前进。",
        ),
        fact: L("同じ化石がどの大陸に出るかで、昔の大陸の位置を考える手がかりになる。", "Where its fossils turn up helps scientists work out where ancient continents lay.", "其化石出现在哪些大陆，是推断古大陆位置的线索。"),
    },
    Species {
        slug: "orthoceras",
        name: L("オルトセラス", "Orthoceras", "直角石"),
        sci: "Orthoceras",
        group: L("頭足類（オウムガイの仲間）", "Cephalopod (nautiloid)", "头足类（鹦鹉螺类）"),
        len_cm: 25.0, t0: 470.0, t1: 455.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Swim, art: Art::Cephalopod(0), col: [0.82, 0.72, 0.60], col2: [0.62, 0.36, 0.28], rarity: 2.5, shy: 1.5, speed: 0.25,
        desc: L(
            "まっすぐな殻を持つ、手ごろな大きさのオウムガイの仲間。群れで海をただよい、小さな動物を捕らえた。",
            "A modest-sized nautiloid with a straight shell that cruised the sea in groups, catching small animals.",
            "外壳笔直、体型适中的鹦鹉螺类，成群在海中游弋，捕食小动物。",
        ),
        fact: L("磨いた化石は、みやげ物の飾り石としていまも売られている。", "Polished fossils are still sold today as decorative stones.", "抛光的化石至今仍作为装饰石出售。"),
    },
    Species {
        slug: "hudsonaster",
        name: L("ハドソナスター（古いヒトデ）", "Hudsonaster (early starfish)", "哈德逊海星"),
        sci: "Hudsonaster",
        group: L("棘皮動物（ヒトデ）", "Echinoderm (starfish)", "棘皮动物（海星）"),
        len_cm: 5.0, t0: 465.0, t1: 443.8, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Crawl, art: Art::Starfish, col: [0.82, 0.56, 0.42], col2: [0.92, 0.76, 0.58], rarity: 2.0, shy: 0.0, speed: 0.01,
        desc: L(
            "5本の腕を持つ、古いヒトデの仲間。管足でゆっくり歩き、腕足動物などを食べたらしい。",
            "An early five-armed starfish that crept on tube feet and probably fed on brachiopods and other shellfish.",
            "长有五条腕的早期海星，用管足缓慢爬行，可能以腕足动物等为食。",
        ),
        fact: L("ヒトデの仲間は、オルドビス紀に初めてはっきり姿を見せる。", "Starfish first clearly appear in the Ordovician.", "海星类在奥陶纪首次明确出现。"),
    },
    Species {
        slug: "arandaspis",
        name: L("アランダスピス", "Arandaspis", "阿兰达鱼"),
        sci: "Arandaspis prionotolepis",
        group: L("無顎類（甲冑魚）", "Jawless armoured fish", "无颌类（甲胄鱼）"),
        len_cm: 15.0, t0: 480.0, t1: 465.0, hab: SHALLOW, regions: &[Plate(GONDWANA)],
        beh: Beh::Swim, art: Art::Jawless(0), col: [0.56, 0.56, 0.52], col2: [0.34, 0.34, 0.32], rarity: 2.5, shy: 1.4, speed: 0.3,
        desc: L(
            "骨の板で体の前を包んだ、あごのない魚。ひれはほとんどなく、海底近くでおだやかに泳いだ。",
            "A jawless fish with bony plates around the front of its body and almost no fins, swimming gently near the sea floor.",
            "身体前部包裹骨板的无颌鱼，几乎没有鳍，在海底附近缓缓游动。",
        ),
        fact: L("体の大部分がわかる、最も古い魚の一つ（オーストラリアで発見）。", "One of the oldest fishes known from much of its body, found in Australia.", "是已知身体大部分保存下来的最古老鱼类之一（发现于澳大利亚）。"),
    },
    Species {
        slug: "promissum",
        name: L("プロミッスム（巨大コノドント）", "Promissum (giant conodont)", "大牙形动物"),
        sci: "Promissum pulchrum",
        group: L("脊椎動物（コノドント）", "Vertebrate (conodont)", "脊椎动物（牙形动物）"),
        len_cm: 40.0, t0: 445.2, t1: 443.8, hab: SHALLOW | SHELF, regions: &[Plate(GONDWANA)],
        beh: Beh::Swim, art: Art::Fish(2), col: [0.56, 0.58, 0.60], col2: [0.80, 0.82, 0.80], rarity: 1.5, shy: 1.5, speed: 0.35,
        desc: L(
            "長さ40cmほどの大きなコノドント動物。氷河時代の冷たい海で、大きな眼で獲物を探した。",
            "A big conodont animal about 40 cm long that hunted with large eyes in the cold seas of the ice age.",
            "体长约40厘米的大型牙形动物，在冰期寒冷的海洋中用大眼睛寻找猎物。",
        ),
        fact: L("南アフリカの地層に、眼の筋肉まで残った化石がある。", "Fossils from South Africa preserve even its eye muscles.", "南非地层中的化石连眼部肌肉都保存了下来。"),
    },
    // ---- シルル紀
    Species {
        slug: "pentamerus",
        name: L("ペンタメルス", "Pentamerus", "五房贝"),
        sci: "Pentamerus oblongus",
        group: L("腕足動物", "Brachiopod", "腕足动物"),
        len_cm: 6.0, t0: 440.0, t1: 428.0, hab: SHALLOW, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Brachiopod, col: [0.80, 0.74, 0.62], col2: [0.92, 0.86, 0.72], rarity: 4.0, shy: 0.0, speed: 0.0,
        desc: L(
            "ぷっくりふくらんだ殻の腕足動物。浅い海の底に、びっしり並んで「貝の原っぱ」をつくった。",
            "A brachiopod with plump, swollen shells that packed together into dense 'shell meadows' on shallow sea floors.",
            "外壳鼓胀的腕足动物，在浅海底密密麻麻地排列成“贝壳草原”。",
        ),
        fact: L("同じ種類が集まった地層から、昔の海の深さを読み取れる。", "Beds of this shell help geologists work out how deep ancient seas were.", "由同种贝壳聚集而成的地层可用来推断古代海洋的深度。"),
    },
    Species {
        slug: "goniophyllum",
        name: L("ゴニオフィルム", "Goniophyllum", "角板珊瑚"),
        sci: "Goniophyllum pyramidale",
        group: L("四放サンゴ（角サンゴ）", "Rugose coral", "四射珊瑚"),
        len_cm: 5.0, t0: 432.0, t1: 420.0, hab: SHALLOW, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Coral(0), col: [0.82, 0.70, 0.56], col2: [0.96, 0.62, 0.48], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "四角いピラミッドの形をした単体サンゴ。口には4枚のふたがあり、閉じて身を守った。",
            "A solitary coral shaped like a square pyramid, with four lids it could close over its mouth.",
            "呈四棱锥形的单体珊瑚，口部有四片盖子，可以闭合保护自己。",
        ),
        fact: L("ふたを持つサンゴは、いまの海には一つもいない。", "No coral alive today has lids like these.", "现代海洋中没有一种珊瑚有这样的盖子。"),
    },
    Species {
        slug: "dalmanites",
        name: L("ダルマニテス", "Dalmanites", "达尔曼虫"),
        sci: "Dalmanites",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 6.0, t0: 440.0, t1: 412.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Crawl, art: Art::Trilobite { w: 0.75, spine: 0.35, pyg: 0.45, segs: 11 }, col: [0.50, 0.44, 0.38], col2: [0.32, 0.28, 0.24], rarity: 3.0, shy: 0.0, speed: 0.04,
        desc: L(
            "大きな複眼と、とがった尾を持つ三葉虫。シルル紀の海で最もよく見られた仲間の一つ。",
            "A trilobite with big compound eyes and a pointed tail, one of the commonest in Silurian seas.",
            "长着大复眼和尖尾的三叶虫，是志留纪海洋中最常见的类群之一。",
        ),
        fact: L("眼のレンズは方解石という鉱物でできていた。", "Its eye lenses were made of the mineral calcite.", "它的眼睛晶状体由方解石这种矿物构成。"),
    },
    Species {
        slug: "dawsonoceras",
        name: L("ドーソノセラス", "Dawsonoceras", "道森角石"),
        sci: "Dawsonoceras",
        group: L("頭足類（オウムガイの仲間）", "Cephalopod (nautiloid)", "头足类（鹦鹉螺类）"),
        len_cm: 40.0, t0: 435.0, t1: 420.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Swim, art: Art::Cephalopod(0), col: [0.76, 0.66, 0.52], col2: [0.52, 0.40, 0.30], rarity: 2.0, shy: 1.5, speed: 0.25,
        desc: L(
            "まっすぐな殻に、輪のようなでこぼこが並ぶオウムガイの仲間。",
            "A nautiloid whose straight shell was ringed with bands of ridges.",
            "笔直的外壳上排列着环状凸起的鹦鹉螺类。",
        ),
        fact: L("でこぼこは殻を強くし、浮き沈みにも役立ったと考えられる。", "The rings probably strengthened the shell.", "这些环状凸起可能加固了外壳。"),
    },
    Species {
        slug: "platyceras",
        name: L("プラティセラス", "Platyceras", "扁角螺"),
        sci: "Platyceras",
        group: L("腹足類（巻貝）", "Gastropod (sea snail)", "腹足类（海螺）"),
        len_cm: 3.0, t0: 440.0, t1: 260.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Sessile, art: Art::Mollusc(0), col: [0.80, 0.70, 0.58], col2: [0.60, 0.50, 0.40], rarity: 2.0, shy: 0.0, speed: 0.0,
        desc: L(
            "ウミユリの体にはりついて暮らした巻貝。ウミユリのふんを食べていたらしい。",
            "A sea snail that lived clinging to crinoids, apparently feeding on their waste.",
            "附着在海百合身上生活的海螺，似乎以海百合的排泄物为食。",
        ),
        fact: L("ウミユリにくっついたままの化石が見つかり、2億年近い「共生」が知られている。", "Fossils still attached to crinoids show a partnership that lasted nearly 200 million years.", "发现了仍附着在海百合上的化石，这种“共生”持续了近2亿年。"),
    },
    Species {
        slug: "climatius",
        name: L("クリマティウス", "Climatius", "栅鱼"),
        sci: "Climatius reticulatus",
        group: L("棘魚類（トゲのあるサメの仲間）", "Acanthodian ('spiny shark')", "棘鱼类"),
        len_cm: 8.0, t0: 425.0, t1: 405.0, hab: SHALLOW | TIDAL, regions: &[Global],
        beh: Beh::Swim, art: Art::Shark(0), col: [0.46, 0.46, 0.44], col2: [0.80, 0.78, 0.72], rarity: 3.0, shy: 1.2, speed: 0.45,
        desc: L(
            "すべてのひれの前に、するどいトゲを持つ小さな魚。トゲで大きな魚から身を守った。",
            "A small fish with a sharp spine in front of every fin, protecting it from bigger fish.",
            "每片鳍前都长着尖刺的小鱼，用刺防御大鱼。",
        ),
        fact: L("棘魚類は、サメの遠い親戚と考えられている。", "Acanthodians are thought to be distant relatives of sharks.", "棘鱼类被认为是鲨鱼的远亲。"),
    },
    Species {
        slug: "loganellia",
        name: L("ロガネリア", "Loganellia", "洛根鱼"),
        sci: "Loganellia scotica",
        group: L("無顎類（テロドゥス類）", "Jawless fish (thelodont)", "无颌类（花鳞鱼）"),
        len_cm: 12.0, t0: 435.0, t1: 420.0, hab: SHALLOW | TIDAL, regions: &[Plate(LAURENTIA), Plate(BALTICA), Plate(AVALONIA)],
        beh: Beh::Swim, art: Art::Jawless(1), col: [0.40, 0.42, 0.44], col2: [0.66, 0.68, 0.66], rarity: 3.0, shy: 1.2, speed: 0.4,
        desc: L(
            "骨の板ではなく、細かい歯のようなうろこで全身をおおった、あごのない魚。",
            "A jawless fish covered not in plates but in tiny tooth-like scales.",
            "全身覆盖着牙齿状细鳞而非骨板的无颌鱼。",
        ),
        fact: L("うろこがたくさん見つかるので、地層の年代を決めるのに使われる。", "Its scales are so common they are used to date rocks.", "其鳞片大量出现，被用于测定地层年代。"),
    },
    Species {
        slug: "slimonia",
        name: L("スリモニア", "Slimonia", "斯莱蒙鲎"),
        sci: "Slimonia acuminata",
        group: L("ウミサソリ", "Sea scorpion (eurypterid)", "板足鲎（海蝎）"),
        len_cm: 60.0, t0: 432.0, t1: 422.0, hab: SHALLOW | TIDAL, regions: &[Plate(LAURENTIA)],
        beh: Beh::NearFloor, art: Art::Eurypterid(0), col: [0.50, 0.40, 0.30], col2: [0.70, 0.58, 0.42], rarity: 1.5, shy: 2.0, speed: 0.25,
        desc: L(
            "尾の先が平たい剣のようになったウミサソリ。浅い入り江の底で獲物を探した。",
            "A sea scorpion with a flat, sword-like tail tip that hunted along the floors of shallow lagoons.",
            "尾端扁平如剑的海蝎，在浅潟湖底寻找猎物。",
        ),
        fact: L("スコットランドの地層から、全身の化石が見つかっている。", "Complete fossils have been found in Scotland.", "在苏格兰的地层中发现了完整的化石。"),
    },
    Species {
        slug: "zosterophyllum",
        name: L("ゾステロフィルム", "Zosterophyllum", "工蕨"),
        sci: "Zosterophyllum",
        group: L("維管束植物", "Vascular plant", "维管植物"),
        len_cm: 25.0, t0: 425.0, t1: 393.0, hab: TIDAL | LAND, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(1), col: [0.32, 0.50, 0.26], col2: [0.62, 0.56, 0.30], rarity: 3.5, shy: 0.0, speed: 0.0,
        desc: L(
            "水辺に群れて生えた、葉のない茎だけの植物。茎の横に胞子の袋が並んだ。",
            "A leafless plant growing in clumps at the water's edge, with spore capsules along the sides of its stems.",
            "成丛生长在水边、只有茎没有叶的植物，茎侧排列着孢子囊。",
        ),
        fact: L("中国・雲南省からオーストラリアまで、世界中で見つかる。", "It has been found worldwide, from Yunnan in China to Australia.", "从中国云南到澳大利亚，世界各地都有发现。"),
    },
    // ---- デボン紀
    Species {
        slug: "phacops",
        name: L("ファコプス", "Phacops", "镜眼虫"),
        sci: "Phacops",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 5.0, t0: 400.0, t1: 372.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Crawl, art: Art::Trilobite { w: 0.8, spine: 0.0, pyg: 0.3, segs: 11 }, col: [0.46, 0.40, 0.34], col2: [0.28, 0.24, 0.20], rarity: 3.0, shy: 0.0, speed: 0.04,
        desc: L(
            "大きなレンズが並ぶ、ぶどうのふさのような眼を持つ三葉虫。体を丸めて身を守った。",
            "A trilobite with grape-cluster eyes made of big separate lenses. It could roll up for protection.",
            "眼睛由一颗颗大晶状体组成、像葡萄串一样的三叶虫，能卷起身体自卫。",
        ),
        fact: L("デボン紀の三葉虫の代表で、丸まった化石がとても人気。", "A classic Devonian trilobite; rolled-up specimens are popular with collectors.", "泥盆纪三叶虫的代表，卷曲的化石很受收藏者欢迎。"),
    },
    Species {
        slug: "mucrospirifer",
        name: L("ムクロスピリファー", "Mucrospirifer", "尖石燕"),
        sci: "Mucrospirifer",
        group: L("腕足動物", "Brachiopod", "腕足动物"),
        len_cm: 5.0, t0: 393.0, t1: 382.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Sessile, art: Art::Brachiopod, col: [0.74, 0.68, 0.56], col2: [0.90, 0.82, 0.68], rarity: 4.0, shy: 0.0, speed: 0.0,
        desc: L(
            "鳥がつばさを広げたような、横に長い殻の腕足動物。殻の中にらせんの「腕」があった。",
            "A brachiopod with a wide shell like spread wings, holding a spiral feeding arm inside.",
            "外壳横向伸长、像鸟展开翅膀的腕足动物，壳内有螺旋状的“腕”。",
        ),
        fact: L("中国では、形から「石燕（いしつばめ）」と呼ばれてきた。", "In China these shells were called 'stone swallows' for their shape.", "在中国，因其形状被称为“石燕”。"),
    },
    Species {
        slug: "heliophyllum",
        name: L("ヘリオフィルム", "Heliophyllum", "日射珊瑚"),
        sci: "Heliophyllum",
        group: L("四放サンゴ（角サンゴ）", "Rugose coral", "四射珊瑚"),
        len_cm: 8.0, t0: 400.0, t1: 382.0, hab: SHALLOW, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Coral(0), col: [0.84, 0.72, 0.58], col2: [0.98, 0.58, 0.40], rarity: 3.5, shy: 0.0, speed: 0.0,
        desc: L(
            "太陽の光のような放射状の模様を持つ角サンゴ。デボン紀の礁に多かった。",
            "A horn coral with sunburst-like radiating partitions, common on Devonian reefs.",
            "具有太阳光芒般放射状纹路的角珊瑚，在泥盆纪礁中很常见。",
        ),
        fact: L("殻の細かな成長線から、当時の1年が約400日だったとわかった。", "Its fine growth lines revealed that a Devonian year had about 400 days.", "从它细密的生长线推算出，当时一年约有400天。"),
    },
    Species {
        slug: "cephalaspis",
        name: L("ケファラスピス", "Cephalaspis", "头甲鱼"),
        sci: "Cephalaspis",
        group: L("無顎類（骨甲類）", "Jawless fish (osteostracan)", "无颌类（骨甲鱼）"),
        len_cm: 22.0, t0: 418.0, t1: 395.0, hab: TIDAL | SHALLOW, regions: &[Plate(LAURENTIA), Plate(BALTICA), Plate(AVALONIA)],
        beh: Beh::NearFloor, art: Art::Jawless(0), col: [0.52, 0.48, 0.40], col2: [0.32, 0.30, 0.26], rarity: 3.0, shy: 1.2, speed: 0.25,
        desc: L(
            "馬のひづめのような形の骨の頭を持つ、あごのない魚。川や河口の底で泥の中の餌を吸った。",
            "A jawless fish with a horseshoe-shaped bony head that sucked food from the mud of rivers and estuaries.",
            "头部骨甲呈马蹄形的无颌鱼，在河流和河口底部吸食泥中的食物。",
        ),
        fact: L("頭の上に、まわりの動きを感じる器官があったと考えられている。", "Patches on its head may have sensed movements in the water.", "头顶上可能有感知周围水流变化的器官。"),
    },
    Species {
        slug: "pteraspis",
        name: L("プテラスピス", "Pteraspis", "鳍甲鱼"),
        sci: "Pteraspis",
        group: L("無顎類（異甲類）", "Jawless fish (heterostracan)", "无颌类（异甲鱼）"),
        len_cm: 20.0, t0: 418.0, t1: 400.0, hab: SHALLOW | TIDAL, regions: &[Plate(LAURENTIA), Plate(BALTICA), Plate(AVALONIA)],
        beh: Beh::Swim, art: Art::Jawless(0), col: [0.48, 0.48, 0.46], col2: [0.30, 0.30, 0.30], rarity: 2.5, shy: 1.3, speed: 0.4,
        desc: L(
            "とがった鼻先と、背中のトゲのある骨の鎧を持つ、あごのない魚。流線形で速く泳げた。",
            "A streamlined jawless fish in bony armour with a pointed snout and a spine on its back.",
            "长着尖吻、背上有刺的骨甲无颌鱼，身体呈流线型，游得很快。",
        ),
        fact: L("背中のトゲと横の角が、体を安定させる「つばさ」の役目をした。", "Its spine and side horns acted like stabilising wings.", "背刺和侧角起到了稳定身体的“翼”的作用。"),
    },
    Species {
        slug: "cheirolepis",
        name: L("ケイロレピス", "Cheirolepis", "鳕鳞鱼"),
        sci: "Cheirolepis",
        group: L("条鰭類（硬骨魚）", "Ray-finned fish", "辐鳍鱼类"),
        len_cm: 40.0, t0: 390.0, t1: 375.0, hab: TIDAL | SHALLOW, regions: &[Global],
        beh: Beh::Swim, art: Art::RayFin, col: [0.44, 0.44, 0.40], col2: [0.72, 0.66, 0.50], rarity: 3.0, shy: 1.6, speed: 0.5,
        desc: L(
            "細かいひし形のうろこを持つ、古い条鰭類。大きな口で、ほかの魚を丸のみにした。",
            "An early ray-finned fish with tiny diamond scales and a big mouth for swallowing other fish whole.",
            "长着细小菱形鳞片的早期辐鳍鱼，用大嘴把其他鱼整条吞下。",
        ),
        fact: L("いまの魚の大部分（条鰭類）の、古い親戚。", "An early relative of most fishes alive today.", "是现代大多数鱼类（辐鳍鱼类）的古老亲戚。"),
    },
    Species {
        slug: "coccosteus",
        name: L("コッコステウス", "Coccosteus", "尾骨鱼"),
        sci: "Coccosteus",
        group: L("板皮類（節頸類）", "Placoderm (arthrodire)", "盾皮鱼类（节颈鱼）"),
        len_cm: 40.0, t0: 392.0, t1: 382.0, hab: SHALLOW | TIDAL, regions: &[Global],
        beh: Beh::Swim, art: Art::Placoderm(1), col: [0.50, 0.48, 0.44], col2: [0.72, 0.68, 0.60], rarity: 2.5, shy: 1.6, speed: 0.4,
        desc: L(
            "ダンクルオステウスの小さな親戚。首の関節で頭を持ち上げ、口を大きく開けられた。",
            "A small relative of Dunkleosteus that could lift its head at a neck joint to open its mouth wide.",
            "邓氏鱼的小型亲戚，能通过颈部关节抬起头，把嘴张得很大。",
        ),
        fact: L("スコットランドの「オールド・レッド砂岩」から多く見つかる。", "It is common in Scotland's Old Red Sandstone.", "在苏格兰“老红砂岩”中大量发现。"),
    },
    Species {
        slug: "dipterus",
        name: L("ディプテルス", "Dipterus", "双鳍鱼"),
        sci: "Dipterus",
        group: L("肺魚類", "Lungfish", "肺鱼类"),
        len_cm: 35.0, t0: 390.0, t1: 380.0, hab: TIDAL | SHALLOW, regions: &[Global],
        beh: Beh::NearFloor, art: Art::LobeFin(0), col: [0.46, 0.44, 0.36], col2: [0.66, 0.60, 0.44], rarity: 3.0, shy: 1.4, speed: 0.3,
        desc: L(
            "えらのほかに肺も持ち、空気を吸えた古い肺魚。川や湖の底で貝などをかみくだいた。",
            "An early lungfish that could breathe air as well as use gills, crushing shellfish on river and lake beds.",
            "除了鳃还有肺、能呼吸空气的早期肺鱼，在河湖底部咬碎贝类。",
        ),
        fact: L("肺魚はいまもアフリカ・南アメリカ・オーストラリアに生きている。", "Lungfish still live today in Africa, South America and Australia.", "肺鱼至今仍生活在非洲、南美洲和澳大利亚。"),
    },
    Species {
        slug: "hyneria",
        name: L("ハイネリア", "Hyneria", "海纳鱼"),
        sci: "Hyneria lindae",
        group: L("肉鰭類", "Lobe-finned fish", "肉鳍鱼类"),
        len_cm: 300.0, t0: 366.0, t1: 360.0, hab: TIDAL | SHALLOW, regions: &[Plate(LAURENTIA)],
        beh: Beh::Swim, art: Art::LobeFin(0), col: [0.42, 0.40, 0.32], col2: [0.62, 0.56, 0.40], rarity: 0.8, shy: 3.0, speed: 0.45,
        desc: L(
            "長さ3mにもなる、川の大型の捕食者。大きな歯で魚や初期の四肢動物を襲った。",
            "A river predator up to 3 m long whose big teeth seized fish and even early tetrapods.",
            "体长可达3米的河流大型捕食者，用大牙捕食鱼类甚至早期四足动物。",
        ),
        fact: L("足を持ちはじめた動物たちにとって、水辺は危険な場所だった。", "For the first animals with legs, the water's edge was a dangerous place.", "对于刚长出四肢的动物来说，水边是危险的地方。"),
    },
    Species {
        slug: "psilophyton",
        name: L("プシロフィトン", "Psilophyton", "裸蕨"),
        sci: "Psilophyton",
        group: L("維管束植物", "Vascular plant", "维管植物"),
        len_cm: 60.0, t0: 405.0, t1: 385.0, hab: TIDAL | LAND, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(11), col: [0.34, 0.50, 0.26], col2: [0.48, 0.60, 0.30], rarity: 3.5, shy: 0.0, speed: 0.0,
        desc: L(
            "小さなトゲのある茎が何度も枝分かれする、ひざくらいの高さの植物。水辺に茂みをつくった。",
            "A knee-high plant with spiny, repeatedly branching stems that formed thickets near the water.",
            "茎上有小刺、反复分叉、高及膝盖的植物，在水边形成灌丛。",
        ),
        fact: L("のちの木やシダにつながる、枝分かれのしかたの変化が見られる。", "Its branching shows early steps toward trees and ferns.", "其分枝方式显示出通向树木和蕨类的早期变化。"),
    },
    Species {
        slug: "rhacophyton",
        name: L("ラコフィトン", "Rhacophyton", "破片蕨"),
        sci: "Rhacophyton",
        group: L("シダの仲間", "Fern-like plant", "蕨类"),
        len_cm: 100.0, t0: 368.0, t1: 359.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(6), col: [0.40, 0.34, 0.24], col2: [0.32, 0.52, 0.26], rarity: 3.5, shy: 0.0, speed: 0.0,
        desc: L(
            "デボン紀の終わりの湿地や森の下に茂った、シダに似た茂み。",
            "A fern-like shrub that grew thickly in swamps and under forest trees at the end of the Devonian.",
            "泥盆纪末在湿地和林下茂盛生长的类蕨灌木。",
        ),
        fact: L("山火事のあとに、真っ先に生えてくる植物だったらしい。", "It seems to have been among the first plants to regrow after wildfires.", "它似乎是山火之后最先重新长出的植物之一。"),
    },
    // ---- 石炭紀
    Species {
        slug: "spirifer",
        name: L("スピリファー", "Spirifer", "石燕贝"),
        sci: "Spirifer",
        group: L("腕足動物", "Brachiopod", "腕足动物"),
        len_cm: 6.0, t0: 355.0, t1: 320.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Sessile, art: Art::Brachiopod, col: [0.76, 0.70, 0.58], col2: [0.90, 0.84, 0.70], rarity: 4.0, shy: 0.0, speed: 0.0,
        desc: L(
            "すじのある翼形の殻を持つ腕足動物。石炭紀の浅い海の底に、どこにでもいた。",
            "A brachiopod with a ribbed, winged shell, found on shallow sea floors everywhere in the Carboniferous.",
            "外壳带肋纹、呈翼形的腕足动物，在石炭纪浅海底随处可见。",
        ),
        fact: L("殻の中のらせんは、触手を支える骨組みだった。", "The spiral inside its shell supported its feeding tentacles.", "壳内的螺旋是支撑摄食触手的骨架。"),
    },
    Species {
        slug: "lithostrotion",
        name: L("リソストロティオン", "Lithostrotion", "石柱珊瑚"),
        sci: "Lithostrotion",
        group: L("四放サンゴ（群体）", "Colonial rugose coral", "四射珊瑚（群体）"),
        len_cm: 40.0, t0: 350.0, t1: 320.0, hab: SHALLOW, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Coral(1), col: [0.82, 0.74, 0.60], col2: [0.62, 0.48, 0.40], rarity: 3.5, shy: 0.0, speed: 0.0,
        desc: L(
            "細い柱がたくさん集まった群体のサンゴ。暖かい浅い海に、大きなかたまりをつくった。",
            "A colonial coral of many slender columns that built big clumps in warm shallow seas.",
            "由许多细柱聚集而成的群体珊瑚，在温暖浅海中形成大团块。",
        ),
        fact: L("イギリスの石灰岩では、断面の模様がよく見られる。", "Its patterned cross-sections are common in British limestones.", "在英国的石灰岩中常能看到它的截面花纹。"),
    },
    Species {
        slug: "archimedes",
        name: L("アルキメデス（らせんコケムシ）", "Archimedes (screw bryozoan)", "阿基米德苔藓虫"),
        sci: "Archimedes",
        group: L("コケムシ動物（群体）", "Bryozoan colony", "苔藓动物（群体）"),
        len_cm: 15.0, t0: 345.0, t1: 300.0, hab: SHALLOW, regions: &[Global],
        beh: Beh::Sessile, art: Art::Coral(2), col: [0.80, 0.70, 0.56], col2: [0.92, 0.84, 0.68], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "ねじのような、らせんの軸を持つコケムシの群体。軸のまわりに網のような扇がついていた。",
            "A bryozoan colony built around a screw-shaped spiral axis, with lacy fans attached.",
            "围绕螺丝状螺旋轴生长的苔藓虫群体，轴周围附有网状扇面。",
        ),
        fact: L("名前は、らせんの水くみ機を考えた古代ギリシャのアルキメデスにちなむ。", "It is named after Archimedes, the Greek inventor of the screw pump.", "名字来自发明螺旋提水机的古希腊人阿基米德。"),
    },
    Species {
        slug: "goniatites",
        name: L("ゴニアタイト", "Goniatites", "棱角菊石"),
        sci: "Goniatites",
        group: L("頭足類（アンモノイド）", "Cephalopod (ammonoid)", "头足类（菊石类）"),
        len_cm: 6.0, t0: 340.0, t1: 323.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Swim, art: Art::Cephalopod(1), col: [0.74, 0.62, 0.48], col2: [0.48, 0.34, 0.24], rarity: 2.5, shy: 1.2, speed: 0.2,
        desc: L(
            "丸く巻いた殻のアンモノイド。殻の部屋の壁が、ジグザグの線を描く。",
            "A round-shelled ammonoid whose chamber walls trace zigzag lines.",
            "外壳圆卷的菊石类，壳内隔壁呈锯齿状线条。",
        ),
        fact: L("種類の移り変わりが速く、石炭紀の地層の年代を決めるのに使われる。", "It changed so quickly that it is used to date Carboniferous rocks.", "演化变化很快，被用来测定石炭纪地层的年代。"),
    },
    Species {
        slug: "pentremites",
        name: L("ペントレマイテス（ウミツボミ）", "Pentremites (blastoid)", "五角海蕾"),
        sci: "Pentremites",
        group: L("棘皮動物（ウミツボミ）", "Echinoderm (blastoid)", "棘皮动物（海蕾）"),
        len_cm: 20.0, t0: 345.0, t1: 318.0, hab: SHALLOW, regions: &[Global],
        beh: Beh::Sessile, art: Art::Crinoid, col: [0.82, 0.68, 0.54], col2: [0.94, 0.84, 0.70], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "茎の先に、花のつぼみのような頭を持つ棘皮動物。細い腕で海水の中の餌を集めた。",
            "An echinoderm with a flower-bud-shaped head on a stalk, gathering food with fine arms.",
            "茎顶端长着花蕾般头部的棘皮动物，用细腕收集水中的食物。",
        ),
        fact: L("ウミユリに近い仲間で、ペルム紀の終わりに絶滅した。", "A relative of crinoids that died out at the end of the Permian.", "是海百合的近亲，在二叠纪末灭绝。"),
    },
    Species {
        slug: "elonichthys",
        name: L("エロニクティス", "Elonichthys", "伊洛鱼"),
        sci: "Elonichthys",
        group: L("条鰭類（硬骨魚）", "Ray-finned fish", "辐鳍鱼类"),
        len_cm: 15.0, t0: 335.0, t1: 300.0, hab: TIDAL | SHALLOW, regions: &[Global],
        beh: Beh::Swim, art: Art::RayFin, col: [0.44, 0.46, 0.42], col2: [0.76, 0.72, 0.56], rarity: 4.0, shy: 1.2, speed: 0.5,
        desc: L(
            "光る厚いうろこを持つ条鰭類。石炭紀の川や入り江に群れで泳いでいた。",
            "A ray-finned fish with thick, shiny scales that swam in shoals in Carboniferous rivers and lagoons.",
            "长着发亮厚鳞的辐鳍鱼，成群游弋于石炭纪的河流和潟湖中。",
        ),
        fact: L("石炭紀にはこうした小魚が水辺にとても多かった。", "Small fish like this were very common in Carboniferous waters.", "石炭纪的水域中这类小鱼非常多。"),
    },
    Species {
        slug: "euproops",
        name: L("ユープロープス（カブトガニの仲間）", "Euproops (horseshoe crab)", "真节肢鲎"),
        sci: "Euproops",
        group: L("カブトガニ類", "Horseshoe crab", "鲎类"),
        len_cm: 4.0, t0: 312.0, t1: 300.0, hab: TIDAL | SHALLOW, regions: &[Tropics],
        beh: Beh::Crawl, art: Art::Limulid, col: [0.42, 0.36, 0.28], col2: [0.62, 0.54, 0.40], rarity: 3.0, shy: 0.4, speed: 0.04,
        desc: L(
            "トゲのある甲羅を持つ小さなカブトガニの仲間。湿地の水辺や、ときには陸にも上がったらしい。",
            "A small spiny horseshoe crab of swampy waters that may even have climbed onto land.",
            "甲壳带刺的小型鲎类，生活在沼泽水边，有时可能还会爬上陆地。",
        ),
        fact: L("カブトガニの仲間は4億年以上、ほとんど姿を変えずに生きている。", "Horseshoe crabs have changed little for over 400 million years.", "鲎类4亿多年来几乎没有改变模样。"),
    },
    Species {
        slug: "bellerophon",
        name: L("ベレロフォン", "Bellerophon", "神螺"),
        sci: "Bellerophon",
        group: L("軟体動物（巻貝の仲間）", "Mollusc (snail-like)", "软体动物（螺类）"),
        len_cm: 4.0, t0: 355.0, t1: 251.9, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Crawl, art: Art::Mollusc(0), col: [0.76, 0.66, 0.52], col2: [0.56, 0.44, 0.34], rarity: 2.5, shy: 0.0, speed: 0.01,
        desc: L(
            "左右対称に丸く巻いた殻を持つ、巻貝の仲間。海底を這って藻などを食べた。",
            "A snail-like mollusc with a symmetrical, rounded coiled shell that grazed as it crawled.",
            "外壳左右对称盘卷的螺类，在海底爬行啃食藻类。",
        ),
        fact: L("ペルム紀の終わりまで生き残ったが、大絶滅で姿を消した。", "It survived to the end of the Permian before vanishing in the great extinction.", "一直生存到二叠纪末，在大灭绝中消失。"),
    },
    Species {
        slug: "sigillaria",
        name: L("シギラリア（封印木）", "Sigillaria (seal tree)", "封印木"),
        sci: "Sigillaria",
        group: L("ヒカゲノカズラ類（木）", "Giant lycopsid tree", "石松类（树）"),
        len_cm: 2500.0, t0: 330.0, t1: 299.0, hab: LAND | TIDAL, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Plant(4), col: [0.44, 0.42, 0.30], col2: [0.32, 0.50, 0.24], rarity: 2.5, shy: 0.0, speed: 0.0,
        desc: L(
            "幹に、はんこを押したような葉の跡が縦に並ぶ木。てっぺんに長い葉の房をつけた。",
            "A tree whose trunk bore vertical rows of seal-like leaf scars, topped with tufts of long leaves.",
            "树干上纵向排列着印章般叶痕的树，顶端长着长叶簇。",
        ),
        fact: L("鱗木と並んで、石炭のもとになった木の代表。", "With Lepidodendron, it was one of the main coal-forming trees.", "与鳞木并列，是形成煤炭的代表性树木。"),
    },
    Species {
        slug: "cordaites",
        name: L("コルダイテス", "Cordaites", "科达树"),
        sci: "Cordaites",
        group: L("裸子植物（木）", "Gymnosperm tree", "裸子植物（树）"),
        len_cm: 3000.0, t0: 320.0, t1: 260.0, hab: LAND, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(8), col: [0.40, 0.32, 0.24], col2: [0.28, 0.44, 0.22], rarity: 2.5, shy: 0.0, speed: 0.0,
        desc: L(
            "細長い帯のような葉を持つ、種でふえる高い木。湿地よりやや乾いた土地に森をつくった。",
            "A tall seed-bearing tree with long strap-like leaves that formed forests on drier ground than the swamps.",
            "长着细长带状叶片、靠种子繁殖的高大树木，在比沼泽稍干的土地上形成森林。",
        ),
        fact: L("針葉樹に近い仲間と考えられている。", "It is thought to be related to conifers.", "被认为与针叶树亲缘相近。"),
    },
    Species {
        slug: "medullosa",
        name: L("メデュローサ（種子シダ）", "Medullosa (seed fern)", "髓木"),
        sci: "Medullosa",
        group: L("種子シダ類", "Seed fern", "种子蕨类"),
        len_cm: 500.0, t0: 320.0, t1: 295.0, hab: LAND | TIDAL, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Plant(6), col: [0.40, 0.30, 0.22], col2: [0.34, 0.54, 0.26], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "シダのような大きな葉をつけるが、種でふえた低い木。石炭紀の森の下に茂った。",
            "A small tree with huge fern-like fronds that nevertheless made seeds, crowding the understorey of coal forests.",
            "长着蕨类般大叶却靠种子繁殖的矮树，在石炭纪森林下层繁茂生长。",
        ),
        fact: L("種はクルミほどの大きさのものもあった。", "Some of its seeds were as big as walnuts.", "有些种子大如核桃。"),
    },
    Species {
        slug: "stenodictya",
        name: L("ステノディクティア", "Stenodictya", "窄网翅虫"),
        sci: "Stenodictya",
        group: L("昆虫（ムカシアミバネムシ類）", "Insect (palaeodictyopteran)", "昆虫（古网翅类）"),
        len_cm: 6.0, t0: 310.0, t1: 299.0, hab: LAND | TIDAL, regions: &[Tropics],
        beh: Beh::Swim, art: Art::Griffinfly, col: [0.34, 0.30, 0.26], col2: [0.86, 0.80, 0.70], rarity: 3.0, shy: 1.2, speed: 0.5,
        desc: L(
            "網目もようのはねを持つ昆虫。くちばしのような口で、植物の汁を吸った。",
            "An insect with net-veined wings and a beak-like mouth for sucking plant sap.",
            "长着网状翅脉的昆虫，用喙状口器吸食植物汁液。",
        ),
        fact: L("石炭紀の森は、空を飛ぶ昆虫が地球で初めてあふれた場所だった。", "Carboniferous forests were the first places on Earth full of flying insects.", "石炭纪森林是地球上第一次充满飞虫的地方。"),
    },
    Species {
        slug: "amphibamus",
        name: L("アンフィバムス", "Amphibamus", "两栖螈"),
        sci: "Amphibamus grandiceps",
        group: L("両生類（分椎類）", "Amphibian (temnospondyl)", "两栖类（离片椎类）"),
        len_cm: 12.0, t0: 312.0, t1: 300.0, hab: TIDAL | LAND, regions: &[Tropics],
        beh: Beh::Crawl, art: Art::Tetrapod(3), col: [0.44, 0.42, 0.32], col2: [0.62, 0.58, 0.42], rarity: 3.0, shy: 1.0, speed: 0.1,
        desc: L(
            "サンショウウオのような小さな両生類。水辺で虫を捕らえて暮らした。",
            "A small salamander-like amphibian that caught insects at the water's edge.",
            "形似蝾螈的小型两栖动物，在水边捕食昆虫。",
        ),
        fact: L("いまのカエルやサンショウウオの祖先に近いと考えられている。", "It may be close to the ancestors of modern frogs and salamanders.", "被认为接近现代青蛙和蝾螈的祖先。"),
    },
    // ---- ペルム紀
    Species {
        slug: "leptodus",
        name: L("レプトドゥス", "Leptodus", "细齿贝"),
        sci: "Leptodus",
        group: L("腕足動物", "Brachiopod", "腕足动物"),
        len_cm: 8.0, t0: 272.0, t1: 252.0, hab: SHALLOW, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Brachiopod, col: [0.80, 0.72, 0.60], col2: [0.92, 0.84, 0.70], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "牡蠣のような平たい殻の腕足動物。礁の上で、ふたのような殻を開け閉めした。",
            "An oyster-like brachiopod with a flat shell whose lid-like valve opened and closed on the reef.",
            "外壳扁平似牡蛎的腕足动物，在礁上开合盖状的壳。",
        ),
        fact: L("内側の葉のような模様は、ほかの腕足動物にはない変わったつくり。", "The leaf-like pattern inside its shell is unique among brachiopods.", "壳内叶状的花纹在腕足动物中独一无二。"),
    },
    Species {
        slug: "waagenoceras",
        name: L("ワーゲノセラス", "Waagenoceras", "瓦根菊石"),
        sci: "Waagenoceras",
        group: L("頭足類（アンモノイド）", "Cephalopod (ammonoid)", "头足类（菊石类）"),
        len_cm: 10.0, t0: 272.0, t1: 262.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Swim, art: Art::Cephalopod(1), col: [0.78, 0.66, 0.50], col2: [0.52, 0.38, 0.26], rarity: 2.5, shy: 1.2, speed: 0.2,
        desc: L(
            "まるい殻を持つペルム紀のアンモノイド。殻の部屋の壁が、とても細かく入りくんだ線を描く。",
            "A Permian ammonoid with a rounded shell whose chamber walls form very intricate lines.",
            "外壳圆润的二叠纪菊石类，壳内隔壁形成非常复杂的线条。",
        ),
        fact: L("アンモノイドはペルム紀末の大絶滅をわずかな種類だけが生きのびた。", "Only a handful of ammonoid groups survived the end-Permian extinction.", "菊石类只有极少数类群在二叠纪末大灭绝中幸存。"),
    },
    Species {
        slug: "waagenophyllum",
        name: L("ワーゲノフィルム", "Waagenophyllum", "瓦根珊瑚"),
        sci: "Waagenophyllum",
        group: L("四放サンゴ（群体）", "Colonial rugose coral", "四射珊瑚（群体）"),
        len_cm: 30.0, t0: 272.0, t1: 252.0, hab: SHALLOW, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Coral(1), col: [0.84, 0.74, 0.60], col2: [0.64, 0.50, 0.42], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "ペルム紀のテチス海の礁に多かった、柱が集まった群体のサンゴ。",
            "A colonial coral of clustered columns, common on the reefs of the Permian Tethys Sea.",
            "由柱体聚集而成的群体珊瑚，在二叠纪特提斯海的礁中很常见。",
        ),
        fact: L("日本の石灰岩からも見つかる。四放サンゴは大絶滅ですべて消えた。", "It is found in Japanese limestones too. All rugose corals died in the great extinction.", "在日本的石灰岩中也有发现。四射珊瑚在大灭绝中全部消失。"),
    },
    Species {
        slug: "eurydesma",
        name: L("エウリデスマ", "Eurydesma", "宽铰蛤"),
        sci: "Eurydesma",
        group: L("二枚貝", "Bivalve", "双壳类"),
        len_cm: 10.0, t0: 298.0, t1: 280.0, hab: SHALLOW | SHELF, regions: &[Plate(GONDWANA)],
        beh: Beh::Sessile, art: Art::Mollusc(1), col: [0.70, 0.64, 0.56], col2: [0.86, 0.80, 0.70], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "ゴンドワナの冷たい海に群れた、殻の厚い二枚貝。氷河の近くでも暮らせた。",
            "A thick-shelled bivalve that crowded the cold seas of Gondwana, even near glaciers.",
            "成群生活在冈瓦纳寒冷海洋中的厚壳双壳类，即使在冰川附近也能生存。",
        ),
        fact: L("冷たい海の目印の化石として使われる。", "Its fossils are used as a sign of cold ancient seas.", "其化石被用作古代寒冷海洋的标志。"),
    },
    Species {
        slug: "xenacanthus",
        name: L("クセナカントゥス", "Xenacanthus", "异刺鲨"),
        sci: "Xenacanthus",
        group: L("軟骨魚類（淡水のサメ）", "Freshwater shark", "软骨鱼类（淡水鲨）"),
        len_cm: 70.0, t0: 300.0, t1: 265.0, hab: TIDAL | SHALLOW, regions: &[Plate(LAURENTIA), Plate(BALTICA), Plate(AVALONIA)],
        beh: Beh::Swim, art: Art::Shark(0), col: [0.42, 0.42, 0.40], col2: [0.70, 0.68, 0.62], rarity: 2.0, shy: 2.0, speed: 0.45,
        desc: L(
            "川や湖に暮らした、ウナギのように細長いサメの仲間。頭の後ろに長いトゲがあった。",
            "An eel-like shark relative of rivers and lakes, with a long spine behind its head.",
            "生活在河湖中、身体如鳗鱼般细长的鲨类，头后有一根长刺。",
        ),
        fact: L("V字形の変わった歯の化石がたくさん見つかる。", "Its strange V-shaped teeth are common fossils.", "大量发现其奇特的V形牙齿化石。"),
    },
    Species {
        slug: "palaeoniscum",
        name: L("パレオニスクム", "Palaeoniscum", "古鳕"),
        sci: "Palaeoniscum freieslebeni",
        group: L("条鰭類（硬骨魚）", "Ray-finned fish", "辐鳍鱼类"),
        len_cm: 30.0, t0: 260.0, t1: 252.0, hab: SHALLOW | TIDAL, regions: &[Global],
        beh: Beh::Swim, art: Art::RayFin, col: [0.40, 0.40, 0.38], col2: [0.70, 0.66, 0.50], rarity: 4.0, shy: 1.2, speed: 0.5,
        desc: L(
            "細長い体に光るうろこの条鰭類。ペルム紀後期の浅い内海に群れで泳いだ。",
            "A slender, shiny-scaled ray-finned fish that swam in shoals in Late Permian inland seas.",
            "身体细长、鳞片发亮的辐鳍鱼，成群游弋于晚二叠世的浅内海。",
        ),
        fact: L("ドイツの銅を含む地層（銅頁岩）から、全身の化石が多く見つかる。", "Complete fossils are common in Germany's copper-bearing shale.", "在德国含铜页岩中发现了大量完整化石。"),
    },
    Species {
        slug: "edaphosaurus",
        name: L("エダフォサウルス", "Edaphosaurus", "基龙"),
        sci: "Edaphosaurus",
        group: L("単弓類（盤竜類）", "Synapsid (pelycosaur)", "合弓纲（盘龙类）"),
        len_cm: 300.0, t0: 300.0, t1: 280.0, hab: LAND, regions: &[Plate(LAURENTIA), Plate(BALTICA)],
        beh: Beh::Crawl, art: Art::Tetrapod(4), col: [0.50, 0.44, 0.34], col2: [0.76, 0.62, 0.44], rarity: 1.5, shy: 1.5, speed: 0.08,
        desc: L(
            "背中の帆に、小枝のような横向きの突起がついた植物食の動物。ディメトロドンの遠い親戚。",
            "A plant-eater whose back sail bore little crossbars like twigs — a distant relative of Dimetrodon.",
            "背帆上长着小枝般横向突起的植食动物，是异齿龙的远亲。",
        ),
        fact: L("口の中にもたくさんの歯の板があり、かたい植物をすりつぶした。", "Plates of teeth on its palate ground up tough plants.", "口腔上颚还有许多齿板，用来磨碎坚硬的植物。"),
    },
    Species {
        slug: "seymouria",
        name: L("セイモウリア", "Seymouria", "西蒙龙"),
        sci: "Seymouria baylorensis",
        group: L("爬形類（両生類と爬虫類の間）", "Reptiliomorph", "爬行形类"),
        len_cm: 60.0, t0: 290.0, t1: 272.0, hab: LAND, regions: &[Plate(LAURENTIA), Plate(BALTICA)],
        beh: Beh::Crawl, art: Art::Tetrapod(2), col: [0.50, 0.42, 0.32], col2: [0.68, 0.58, 0.42], rarity: 2.5, shy: 1.2, speed: 0.12,
        desc: L(
            "乾いた土地でも暮らせた、がっしりした四肢動物。両生類と爬虫類の特徴をあわせ持つ。",
            "A sturdy tetrapod at home on dry land, combining features of amphibians and reptiles.",
            "能在干燥土地上生活的粗壮四足动物，兼具两栖类和爬行类的特征。",
        ),
        fact: L("子どもにはオタマジャクシのような時期があったと考えられている。", "Its young probably had a tadpole-like stage.", "其幼体可能经历过类似蝌蚪的阶段。"),
    },
    Species {
        slug: "captorhinus",
        name: L("カプトリヌス", "Captorhinus", "大鼻龙"),
        sci: "Captorhinus aguti",
        group: L("爬虫類", "Early reptile", "爬行动物"),
        len_cm: 25.0, t0: 290.0, t1: 275.0, hab: LAND, regions: &[Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Tetrapod(2), col: [0.56, 0.46, 0.34], col2: [0.72, 0.62, 0.46], rarity: 3.0, shy: 1.0, speed: 0.14,
        desc: L(
            "トカゲに似た小さな爬虫類。何列にも並んだ歯で、虫や植物をかみくだいた。",
            "A small lizard-like reptile with several rows of teeth for crunching insects and plants.",
            "形似蜥蜴的小型爬行动物，用多排牙齿咬碎昆虫和植物。",
        ),
        fact: L("尾を自分で切って逃げることができた、最古の動物の一つ。", "One of the oldest animals known to drop its tail to escape predators.", "是已知最早能自断尾巴逃生的动物之一。"),
    },
    Species {
        slug: "dicynodon",
        name: L("ディキノドン", "Dicynodon", "二齿兽"),
        sci: "Dicynodon",
        group: L("単弓類（ディキノドン類）", "Synapsid (dicynodont)", "合弓纲（二齿兽类）"),
        len_cm: 120.0, t0: 260.0, t1: 252.0, hab: LAND, regions: &[Plate(GONDWANA), Plate(BALTICA)],
        beh: Beh::Crawl, art: Art::Tetrapod(10), col: [0.58, 0.52, 0.42], col2: [0.42, 0.38, 0.32], rarity: 3.0, shy: 1.2, speed: 0.08,
        desc: L(
            "くちばしと2本の牙を持つ植物食の動物。ペルム紀後期の陸で、いちばん数の多い動物の一つだった。",
            "A beaked, two-tusked plant-eater — one of the most numerous land animals of the Late Permian.",
            "长着喙和两根獠牙的植食动物，是晚二叠世陆地上数量最多的动物之一。",
        ),
        fact: L("名前は「2本の犬歯」という意味。", "Its name means 'two dog teeth'.", "名字意为“两颗犬齿”。"),
    },
    Species {
        slug: "moschops",
        name: L("モスコプス", "Moschops", "麝足兽"),
        sci: "Moschops capensis",
        group: L("単弓類（恐頭類）", "Synapsid (dinocephalian)", "合弓纲（恐头兽类）"),
        len_cm: 270.0, t0: 266.0, t1: 260.0, hab: LAND, regions: &[Plate(GONDWANA)],
        beh: Beh::Crawl, art: Art::Tetrapod(9), col: [0.58, 0.48, 0.38], col2: [0.44, 0.36, 0.28], rarity: 2.0, shy: 1.5, speed: 0.07,
        desc: L(
            "厚い頭の骨を持つ、どっしりした植物食の動物。頭どうしをぶつけ合って争ったらしい。",
            "A heavy plant-eater with a very thick skull, probably used for head-butting contests.",
            "头骨极厚、体态敦实的植食动物，可能用头互相撞击来争斗。",
        ),
        fact: L("南アフリカのカルー盆地を代表する動物の一つ。", "One of the signature animals of South Africa's Karoo Basin.", "是南非卡鲁盆地的代表动物之一。"),
    },
    Species {
        slug: "tshekardocoleus",
        name: L("チェカルドコレウス（古い甲虫）", "Tshekardocoleus (early beetle)", "切卡尔多甲虫"),
        sci: "Tshekardocoleus",
        group: L("昆虫（甲虫の祖先）", "Insect (early beetle)", "昆虫（早期甲虫）"),
        len_cm: 1.5, t0: 290.0, t1: 280.0, hab: LAND, regions: &[Plate(BALTICA), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Arthro(4), col: [0.30, 0.24, 0.18], col2: [0.56, 0.44, 0.30], rarity: 3.0, shy: 0.6, speed: 0.04,
        desc: L(
            "かたい前ばねを持つ、最も古い甲虫の仲間。木の皮の下などで暮らした。",
            "One of the oldest beetle relatives, with hardened forewings, living under bark.",
            "具有硬化前翅的最古老甲虫类之一，生活在树皮下等处。",
        ),
        fact: L("甲虫はいま、動物の中で最も種類が多いグループ。", "Beetles are now the most species-rich group of animals.", "甲虫如今是动物中种类最多的类群。"),
    },
    Species {
        slug: "gigantopteris",
        name: L("ギガントプテリス", "Gigantopteris", "大羽羊齿"),
        sci: "Gigantopteris",
        group: L("種子植物（大羽羊歯類）", "Seed plant (gigantopterid)", "种子植物（大羽羊齿类）"),
        len_cm: 300.0, t0: 275.0, t1: 252.0, hab: LAND | TIDAL, regions: &[Plate(SOUTH_CHINA), Plate(NORTH_CHINA)],
        beh: Beh::Sessile, art: Art::Plant(7), col: [0.40, 0.32, 0.24], col2: [0.30, 0.50, 0.24], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L(
            "広葉樹のような網目の葉脈を持つ大きな葉の植物。中国の暖かい島々の森に茂った。",
            "A plant with big leaves netted with veins like a broadleaf tree, thriving in the warm island forests of China.",
            "长着大叶、叶脉网状似阔叶树的植物，繁茂于中国温暖的岛屿森林中。",
        ),
        fact: L("花の咲く植物とは別に、同じような葉のつくりを独自に生み出した。", "It evolved broad netted leaves independently of flowering plants.", "它独立于开花植物，演化出了类似的叶片结构。"),
    },
    // ======================================================== さらに追加した生きもの
    // ---- カンブリア紀
    Species {
        slug: "canadaspis",
        name: L("カナダスピス", "Canadaspis", "加拿大盾虾"),
        sci: "Canadaspis perfecta",
        group: L("節足動物", "Arthropod", "节肢动物"),
        len_cm: 5.0, t0: 510.0, t1: 505.0, hab: SHELF, regions: &[Site(S_BURGESS)],
        beh: Beh::Crawl, art: Art::Isoxys, col: [0.56, 0.48, 0.36], col2: [0.80, 0.66, 0.48], rarity: 3.0, shy: 0.0, speed: 0.05,
        desc: L("2枚の殻で体をおおった節足動物。海底を歩き、泥の中の餌をかき集めた。", "An arthropod covered by a two-part shell that walked the sea floor, raking up food from the mud.", "身体被两片壳覆盖的节肢动物，在海底行走，从泥中搜集食物。"),
        fact: L("バージェスで2番目に多く見つかる節足動物。", "It is the second most common arthropod in the Burgess Shale.", "是伯吉斯页岩中数量第二多的节肢动物。"),
    },
    Species {
        slug: "waptia",
        name: L("ワプティア", "Waptia", "瓦普塔虾"),
        sci: "Waptia fieldensis",
        group: L("節足動物", "Arthropod", "节肢动物"),
        len_cm: 8.0, t0: 510.0, t1: 505.0, hab: SHELF, regions: &[Site(S_BURGESS)],
        beh: Beh::Swim, art: Art::Isoxys, col: [0.66, 0.58, 0.46], col2: [0.86, 0.74, 0.58], rarity: 2.5, shy: 1.0, speed: 0.3,
        desc: L("エビに似た姿で泳いだ節足動物。殻の下に卵を抱えた化石が見つかっている。", "A shrimp-like swimming arthropod; fossils have been found carrying eggs beneath the shell.", "外形似虾的游泳节肢动物，发现过在壳下抱卵的化石。"),
        fact: L("子育ての証拠が残る、最古の動物の一つ。", "One of the oldest animals with evidence of caring for its eggs.", "是留有育幼证据的最古老动物之一。"),
    },
    Species {
        slug: "yohoia",
        name: L("ヨホイア", "Yohoia", "约霍虾"),
        sci: "Yohoia tenuis",
        group: L("節足動物（大付属肢類）", "Arthropod (great-appendage)", "节肢动物（大附肢类）"),
        len_cm: 2.5, t0: 510.0, t1: 505.0, hab: SHELF, regions: &[Site(S_BURGESS)],
        beh: Beh::NearFloor, art: Art::Leanchoilia, col: [0.62, 0.52, 0.42], col2: [0.84, 0.70, 0.52], rarity: 3.0, shy: 0.8, speed: 0.2,
        desc: L("頭の前に、はさみのついた大きな腕を一対持つ小さな節足動物。腕で獲物をつかんだ。", "A small arthropod with one pair of large clawed arms at the front of its head for grabbing prey.", "头前长有一对带钳大附肢的小型节肢动物，用附肢抓捕猎物。"),
        fact: L("クモやサソリの祖先に近いとも考えられている。", "It may be close to the ancestors of spiders and scorpions.", "也有观点认为它接近蜘蛛和蝎子的祖先。"),
    },
    Species {
        slug: "eldonia",
        name: L("エルドニア", "Eldonia", "伊尔东钵"),
        sci: "Eldonia ludwigi",
        group: L("所属不明（円盤形の動物）", "Disc-shaped animal of uncertain affinity", "分类不明（盘状动物）"),
        len_cm: 10.0, t0: 520.0, t1: 505.0, hab: SHELF | SHALLOW, regions: &[Site(S_BURGESS), Site(S_CHENGJIANG)],
        beh: Beh::Drift, art: Art::Jelly, col: [0.74, 0.66, 0.56], col2: [0.56, 0.46, 0.38], rarity: 2.0, shy: 0.0, speed: 0.03,
        desc: L("円盤の形をした体に、うずまき形の腸が見える動物。クラゲのように見えるが、別の仲間らしい。", "A disc-shaped animal with a coiled gut visible inside. It looks like a jellyfish but probably was not one.", "身体呈圆盘状、可见螺旋形肠道的动物。外形像水母，但似乎属于别的类群。"),
        fact: L("海底に横たわっていたのか、ただよっていたのか、まだ議論がある。", "Scientists still argue whether it drifted or lay on the sea floor.", "它是漂浮还是躺在海底，至今仍有争论。"),
    },
    Species {
        slug: "chancelloria",
        name: L("チャンセロリア", "Chancelloria", "开腔骨"),
        sci: "Chancelloria",
        group: L("所属不明（トゲの動物）", "Spiny animal of uncertain affinity", "分类不明（多刺动物）"),
        len_cm: 15.0, t0: 520.0, t1: 490.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Sessile, art: Art::Sponge(0), col: [0.80, 0.74, 0.62], col2: [0.94, 0.88, 0.74], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L("星形のトゲがびっしり生えた、袋のような形の動物。海底に立って、海水から餌をこした。", "A sack-shaped animal studded with star-shaped spines, standing on the sea floor and filtering food.", "身体呈袋状、长满星形刺的动物，立在海底滤食。"),
        fact: L("トゲのかけらは、世界中のカンブリア紀の地層から見つかる。", "Its spine pieces are found in Cambrian rocks all over the world.", "其骨刺碎片在世界各地的寒武纪地层中都有发现。"),
    },
    Species {
        slug: "elrathia",
        name: L("エルラシア", "Elrathia", "耳材虫"),
        sci: "Elrathia kingii",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 3.0, t0: 507.0, t1: 500.0, hab: SHELF, regions: &[Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Trilobite { w: 0.75, spine: 0.08, pyg: 0.2, segs: 13 }, col: [0.40, 0.38, 0.34], col2: [0.26, 0.24, 0.22], rarity: 3.5, shy: 0.0, speed: 0.04,
        desc: L("酸素の少ない泥の海底にたくさんいた小さな三葉虫。", "A small trilobite that was abundant on oxygen-poor muddy sea floors.", "大量生活在缺氧泥质海底的小型三叶虫。"),
        fact: L("アメリカ・ユタ州で大量に見つかり、世界でいちばん有名な三葉虫の化石の一つ。", "Found by the thousands in Utah, USA — one of the world's best-known trilobite fossils.", "在美国犹他州大量发现，是世界上最有名的三叶虫化石之一。"),
    },
    Species {
        slug: "gogia",
        name: L("ゴギア", "Gogia", "古吉海林檎"),
        sci: "Gogia",
        group: L("棘皮動物（始海百合類）", "Echinoderm (eocrinoid)", "棘皮动物（始海百合类）"),
        len_cm: 6.0, t0: 510.0, t1: 500.0, hab: SHALLOW | SHELF, regions: &[Plate(LAURENTIA)],
        beh: Beh::Sessile, art: Art::Crinoid, col: [0.78, 0.66, 0.54], col2: [0.92, 0.82, 0.68], rarity: 2.5, shy: 0.0, speed: 0.0,
        desc: L("短い茎の上に、板でできた袋のような体と細い腕を持つ棘皮動物。", "An echinoderm with a sack-like body of plates on a short stalk, and slender feeding arms.", "短茎顶端有由骨板构成的袋状身体和细腕的棘皮动物。"),
        fact: L("のちのウミユリに似た暮らし方を、カンブリア紀にもうしていた。", "It already lived like the later crinoids during the Cambrian.", "早在寒武纪就过着类似后来海百合的生活。"),
    },
    Species {
        slug: "myllokunmingia",
        name: L("ミロクンミンギア", "Myllokunmingia", "昆明鱼"),
        sci: "Myllokunmingia fengjiaoa",
        group: L("脊椎動物（最古の魚の仲間）", "Vertebrate (early fish)", "脊椎动物（最早的鱼类）"),
        len_cm: 3.0, t0: 520.0, t1: 515.0, hab: SHELF | SHALLOW, regions: &[Site(S_CHENGJIANG)],
        beh: Beh::Swim, art: Art::Fish(0), col: [0.66, 0.66, 0.60], col2: [0.84, 0.84, 0.78], rarity: 2.0, shy: 1.5, speed: 0.4,
        desc: L("背びれとえらを持つ、小さな魚の仲間。ハイコウイクチスと並ぶ、最古の脊椎動物の一つ。", "A tiny fish-like animal with a dorsal fin and gills — with Haikouichthys, one of the oldest vertebrates.", "长有背鳍和鳃的小型鱼类，与海口鱼并列为最古老的脊椎动物之一。"),
        fact: L("名前は「昆明の魚」。雲南省の省都・昆明にちなむ。", "Its name means 'Kunming fish', after the capital of Yunnan.", "名字意为“昆明的鱼”，以云南省省会昆明命名。"),
    },
    // ---- オルドビス紀
    Species {
        slug: "pentecopterus",
        name: L("ペンテコプテルス", "Pentecopterus", "五十鲎"),
        sci: "Pentecopterus decorahensis",
        group: L("ウミサソリ", "Sea scorpion (eurypterid)", "板足鲎（海蝎）"),
        len_cm: 170.0, t0: 467.0, t1: 465.0, hab: SHALLOW, regions: &[Plate(LAURENTIA)],
        beh: Beh::NearFloor, art: Art::Eurypterid(2), col: [0.46, 0.34, 0.26], col2: [0.72, 0.54, 0.38], rarity: 0.8, shy: 2.5, speed: 0.3,
        desc: L("体長1.7mの、知られている中で最古のウミサソリ。とげだらけの前脚で獲物を捕らえた。", "At 1.7 m long, the oldest known sea scorpion, catching prey with spiky front legs.", "体长1.7米，是已知最古老的海蝎，用多刺的前肢捕捉猎物。"),
        fact: L("名前は、古代ギリシャの軍船「ペンテコンター」にちなむ。", "It is named after the pentekonter, an ancient Greek warship.", "名字来自古希腊战船“五十桨船”。"),
    },
    Species {
        slug: "lunataspis",
        name: L("ルナタスピス", "Lunataspis", "月盾鲎"),
        sci: "Lunataspis aurora",
        group: L("カブトガニ類", "Horseshoe crab", "鲎类"),
        len_cm: 4.0, t0: 446.0, t1: 443.8, hab: TIDAL | SHALLOW, regions: &[Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Limulid, col: [0.46, 0.40, 0.30], col2: [0.66, 0.58, 0.44], rarity: 2.5, shy: 0.4, speed: 0.04,
        desc: L("半月形の甲羅を持つ、知られている中で最古のカブトガニの仲間。", "The oldest known horseshoe crab, with a half-moon-shaped head shield.", "长着半月形头甲，是已知最古老的鲎类。"),
        fact: L("姿は、4億年以上たったいまのカブトガニとほとんど変わらない。", "It looks remarkably like horseshoe crabs alive 445 million years later.", "外形与4亿多年后的现代鲎几乎没有区别。"),
    },
    Species {
        slug: "echinosphaerites",
        name: L("エキノスファエリテス（ウミリンゴ）", "Echinosphaerites (cystoid)", "球海林檎"),
        sci: "Echinosphaerites aurantium",
        group: L("棘皮動物（ウミリンゴ類）", "Echinoderm (cystoid)", "棘皮动物（海林檎类）"),
        len_cm: 4.0, t0: 470.0, t1: 458.0, hab: SHALLOW | SHELF, regions: &[Site(S_STPETERSBURG), Plate(BALTICA)],
        beh: Beh::Sessile, art: Art::Crinoid, col: [0.84, 0.66, 0.48], col2: [0.94, 0.80, 0.62], rarity: 3.5, shy: 0.0, speed: 0.0,
        desc: L("オレンジのような丸い体の棘皮動物。小さな穴の開いた板で体ができていた。", "An orange-like round echinoderm built from plates pierced by tiny pores.", "身体圆如橙子的棘皮动物，由带小孔的骨板构成。"),
        fact: L("学名の aurantium は、ラテン語で「オレンジ」。", "Its species name aurantium is Latin for 'orange'.", "种名 aurantium 在拉丁语中意为“橙子”。"),
    },
    Species {
        slug: "bothriocidaris",
        name: L("ボスリオシダリス（古いウニ）", "Bothriocidaris (early sea urchin)", "穴头海胆"),
        sci: "Bothriocidaris",
        group: L("棘皮動物（ウニ）", "Echinoderm (sea urchin)", "棘皮动物（海胆）"),
        len_cm: 2.0, t0: 460.0, t1: 445.0, hab: SHALLOW, regions: &[Plate(BALTICA)],
        beh: Beh::Crawl, art: Art::Urchin, col: [0.64, 0.46, 0.40], col2: [0.84, 0.70, 0.60], rarity: 2.5, shy: 0.0, speed: 0.01,
        desc: L("小さなトゲにおおわれた、丸い体の古いウニ。海底の藻などをかじって食べた。", "An early round sea urchin covered with small spines that grazed algae on the sea floor.", "全身覆盖小刺的圆形早期海胆，啃食海底的藻类。"),
        fact: L("ウニの仲間の、知られている中でも古いものの一つ。", "One of the oldest known sea urchins.", "是已知最古老的海胆之一。"),
    },
    Species {
        slug: "lituites",
        name: L("リツイテス", "Lituites", "喇叭角石"),
        sci: "Lituites",
        group: L("頭足類（オウムガイの仲間）", "Cephalopod (nautiloid)", "头足类（鹦鹉螺类）"),
        len_cm: 30.0, t0: 470.0, t1: 458.0, hab: SHALLOW | SHELF, regions: &[Plate(BALTICA)],
        beh: Beh::Swim, art: Art::Cephalopod(1), col: [0.80, 0.68, 0.54], col2: [0.58, 0.40, 0.30], rarity: 2.0, shy: 1.5, speed: 0.2,
        desc: L("はじめはうずまきに巻き、途中からまっすぐにのびる殻を持つオウムガイの仲間。", "A nautiloid whose shell began as a tight coil and then grew out straight.", "外壳起初盘卷、后来笔直伸长的鹦鹉螺类。"),
        fact: L("司教の杖のような形から、スウェーデンでは昔から知られていた。", "Its shape, like a bishop's crook, made it familiar in Sweden for centuries.", "其形状如主教的权杖，在瑞典自古为人所知。"),
    },
    Species {
        slug: "selenopeltis",
        name: L("セレノペルティス", "Selenopeltis", "月盾虫"),
        sci: "Selenopeltis",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 10.0, t0: 470.0, t1: 455.0, hab: SHELF, regions: &[Plate(GONDWANA)],
        beh: Beh::Crawl, art: Art::Trilobite { w: 0.85, spine: 0.9, pyg: 0.2, segs: 10 }, col: [0.50, 0.42, 0.34], col2: [0.30, 0.24, 0.20], rarity: 2.0, shy: 0.0, speed: 0.04,
        desc: L("体の節ごとに、長いトゲが左右にのびた三葉虫。冷たいゴンドワナの海で暮らした。", "A trilobite with a long spine sticking out from each side of every segment, living in Gondwana's cold seas.", "身体每节两侧都伸出长刺的三叶虫，生活在冈瓦纳寒冷的海洋中。"),
        fact: L("長いトゲは、やわらかい泥に沈まないための「かんじき」だったのかもしれない。", "Its long spines may have worked like snowshoes on soft mud.", "长刺可能像雪鞋一样，防止它陷入软泥。"),
    },
    Species {
        slug: "astraspis",
        name: L("アストラスピス", "Astraspis", "星甲鱼"),
        sci: "Astraspis desiderata",
        group: L("無顎類（甲冑魚）", "Jawless armoured fish", "无颌类（甲胄鱼）"),
        len_cm: 13.0, t0: 458.0, t1: 450.0, hab: SHALLOW | TIDAL, regions: &[Plate(LAURENTIA)],
        beh: Beh::Swim, art: Art::Jawless(0), col: [0.56, 0.54, 0.48], col2: [0.80, 0.76, 0.66], rarity: 2.5, shy: 1.4, speed: 0.3,
        desc: L("星形の小さなこぶがならぶ骨の板で体をおおった、あごのない魚。", "A jawless fish armoured with bony plates covered in tiny star-shaped bumps.", "身披布满星形小瘤的骨板的无颌鱼。"),
        fact: L("名前は「星の盾」という意味。", "Its name means 'star shield'.", "名字意为“星之盾”。"),
    },
    Species {
        slug: "halysites",
        name: L("ハリシテス（クサリサンゴ）", "Halysites (chain coral)", "链珊瑚"),
        sci: "Halysites",
        group: L("床板サンゴ", "Tabulate coral", "床板珊瑚"),
        len_cm: 25.0, t0: 455.0, t1: 420.0, hab: SHALLOW, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Coral(1), col: [0.82, 0.74, 0.60], col2: [0.60, 0.48, 0.40], rarity: 3.5, shy: 0.0, speed: 0.0,
        desc: L("細い管が鎖のようにつながって、網目の模様をつくるサンゴの群体。", "A coral colony whose slender tubes link up like chains, forming a net-like pattern.", "细管如锁链般相连、形成网状花纹的珊瑚群体。"),
        fact: L("断面の模様が鎖そっくりなので「クサリサンゴ」と呼ばれる。", "It is called the chain coral because its cross-section looks like a chain.", "因截面花纹酷似锁链而被称为“链珊瑚”。"),
    },
    Species {
        slug: "receptaculites",
        name: L("レセプタクリテス（ヒマワリサンゴ）", "Receptaculites (sunflower coral)", "托盘类"),
        sci: "Receptaculites",
        group: L("所属不明（緑藻の仲間か）", "Uncertain (probably a green alga)", "分类不明（可能是绿藻）"),
        len_cm: 25.0, t0: 470.0, t1: 370.0, hab: SHALLOW, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Sponge(2), col: [0.66, 0.70, 0.52], col2: [0.84, 0.80, 0.62], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L("ひまわりの種のようならせん模様を持つ、皿のような生きもの。サンゴではなく、藻の仲間らしい。", "A dish-shaped organism with spirals like a sunflower's seeds. Despite its nickname it was probably an alga, not a coral.", "具有向日葵籽般螺旋花纹的盘状生物。虽俗称“向日葵珊瑚”，其实可能是藻类。"),
        fact: L("正体は100年以上なぞのままだった。", "What it was remained a mystery for over a century.", "它的真实身份一百多年来一直是个谜。"),
    },
    // ---- シルル紀
    Species {
        slug: "brontoscorpio",
        name: L("ブロントスコルピオ", "Brontoscorpio", "雷蝎"),
        sci: "Brontoscorpio anglicus",
        group: L("サソリ", "Scorpion", "蝎子"),
        len_cm: 90.0, t0: 421.0, t1: 419.2, hab: TIDAL | SHALLOW, regions: &[Plate(AVALONIA)],
        beh: Beh::NearFloor, art: Art::Arthro(2), col: [0.34, 0.28, 0.22], col2: [0.56, 0.44, 0.32], rarity: 1.0, shy: 2.0, speed: 0.15,
        desc: L("体長1mにもなったと考えられる、水の中で暮らした巨大なサソリ。", "A giant water-dwelling scorpion that may have reached 1 m long.", "体长可能达1米、生活在水中的巨型蝎子。"),
        fact: L("見つかっているのは、大きなはさみの一部だけ。", "Only part of its huge claw has been found.", "目前只发现了它巨大螯的一部分。"),
    },
    Species {
        slug: "carcinosoma",
        name: L("カルキノソーマ", "Carcinosoma", "蟹体鲎"),
        sci: "Carcinosoma",
        group: L("ウミサソリ", "Sea scorpion (eurypterid)", "板足鲎（海蝎）"),
        len_cm: 70.0, t0: 430.0, t1: 420.0, hab: SHALLOW | TIDAL, regions: &[Plate(LAURENTIA)],
        beh: Beh::NearFloor, art: Art::Eurypterid(2), col: [0.48, 0.38, 0.28], col2: [0.70, 0.56, 0.40], rarity: 1.5, shy: 2.0, speed: 0.25,
        desc: L("とげの並んだ前脚と、サソリのようにそり上がった尾を持つウミサソリ。", "A sea scorpion with spiny front legs and a scorpion-like upturned tail.", "前肢带刺、尾部如蝎子般上翘的海蝎。"),
        fact: L("名前は「カニの体」という意味。", "Its name means 'crab body'.", "名字意为“蟹的身体”。"),
    },
    Species {
        slug: "jamoytius",
        name: L("ジャモイティウス", "Jamoytius", "雅莫鱼"),
        sci: "Jamoytius kerwoodi",
        group: L("無顎類", "Jawless fish", "无颌类"),
        len_cm: 26.0, t0: 435.0, t1: 428.0, hab: SHALLOW, regions: &[Plate(LAURENTIA)],
        beh: Beh::Swim, art: Art::Jawless(1), col: [0.44, 0.46, 0.44], col2: [0.70, 0.72, 0.68], rarity: 2.5, shy: 1.2, speed: 0.4,
        desc: L("ヤツメウナギに似た細長い体の、あごのない魚。丸い口で吸いついて食べたらしい。", "A slender, lamprey-like jawless fish that probably fed by sucking with its round mouth.", "身体细长、形似七鳃鳗的无颌鱼，可能用圆口吸附进食。"),
        fact: L("いまのヤツメウナギの遠い親戚かもしれない。", "It may be a distant relative of today's lampreys.", "可能是现代七鳃鳗的远亲。"),
    },
    Species {
        slug: "heliolites",
        name: L("ヘリオリテス（日石サンゴ）", "Heliolites", "日射珊瑚群体"),
        sci: "Heliolites",
        group: L("床板サンゴ", "Tabulate coral", "床板珊瑚"),
        len_cm: 20.0, t0: 445.0, t1: 380.0, hab: SHALLOW, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Coral(1), col: [0.86, 0.78, 0.64], col2: [0.66, 0.52, 0.42], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L("小さな太陽のような花形の穴が並ぶ、こぶ形のサンゴの群体。", "A lumpy coral colony dotted with tiny sun-like star-shaped pits.", "表面排列着小太阳般星形孔的瘤状珊瑚群体。"),
        fact: L("シルル紀の礁の、ありふれた住人だった。", "It was a common resident of Silurian reefs.", "是志留纪礁中常见的居民。"),
    },
    Species {
        slug: "encrinurus",
        name: L("エンクリヌルス（イチゴ頭の三葉虫）", "Encrinurus ('strawberry-head')", "彗星虫"),
        sci: "Encrinurus",
        group: L("三葉虫", "Trilobite", "三叶虫"),
        len_cm: 5.0, t0: 440.0, t1: 420.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Crawl, art: Art::Trilobite { w: 0.75, spine: 0.1, pyg: 0.45, segs: 11 }, col: [0.54, 0.44, 0.36], col2: [0.34, 0.26, 0.22], rarity: 2.5, shy: 0.0, speed: 0.04,
        desc: L("頭にイチゴのようなつぶつぶがある三葉虫。眼は短い柄の上についていた。", "A trilobite with strawberry-like bumps on its head and eyes on short stalks.", "头部布满草莓般颗粒、眼睛长在短柄上的三叶虫。"),
        fact: L("英語では「イチゴ頭の三葉虫」と呼ばれる。", "In English it is nicknamed the 'strawberry-headed trilobite'.", "英文俗称“草莓头三叶虫”。"),
    },
    Species {
        slug: "ophioceras",
        name: L("オフィオセラス", "Ophioceras", "蛇角石"),
        sci: "Ophioceras",
        group: L("頭足類（オウムガイの仲間）", "Cephalopod (nautiloid)", "头足类（鹦鹉螺类）"),
        len_cm: 6.0, t0: 430.0, t1: 425.0, hab: SHALLOW, regions: &[Plate(BALTICA)],
        beh: Beh::Swim, art: Art::Cephalopod(1), col: [0.76, 0.66, 0.52], col2: [0.54, 0.40, 0.30], rarity: 2.5, shy: 1.2, speed: 0.2,
        desc: L("ゆるくうずまきに巻いた殻を持つ、小さなオウムガイの仲間。", "A small nautiloid with a loosely coiled shell.", "外壳松散盘卷的小型鹦鹉螺类。"),
        fact: L("ゴトランド島の礁のまわりで、たくさん見つかる。", "It is common around the reefs of Gotland.", "在哥得兰岛的礁周围大量发现。"),
    },
    Species {
        slug: "atrypa",
        name: L("アトリパ", "Atrypa", "无洞贝"),
        sci: "Atrypa",
        group: L("腕足動物", "Brachiopod", "腕足动物"),
        len_cm: 3.0, t0: 440.0, t1: 370.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Sessile, art: Art::Brachiopod, col: [0.74, 0.66, 0.54], col2: [0.88, 0.80, 0.66], rarity: 4.0, shy: 0.0, speed: 0.0,
        desc: L("細かいすじと成長の輪がある、丸い殻の腕足動物。とても長いあいだ世界中に栄えた。", "A round-shelled brachiopod with fine ribs and growth rings that thrived worldwide for a very long time.", "外壳圆润、带细肋和生长纹的腕足动物，长期遍布世界各地。"),
        fact: L("7000万年近く生きつづけた、とても長生きの属。", "The genus lasted for nearly 70 million years.", "这个属延续了近7000万年。"),
    },
    Species {
        slug: "pterygotus",
        name: L("プテリゴトゥス", "Pterygotus", "翼肢鲎"),
        sci: "Pterygotus",
        group: L("ウミサソリ", "Sea scorpion (eurypterid)", "板足鲎（海蝎）"),
        len_cm: 160.0, t0: 428.0, t1: 412.0, hab: SHALLOW | TIDAL, regions: &[Plate(LAURENTIA), Plate(AVALONIA)],
        beh: Beh::NearFloor, art: Art::Eurypterid(1), col: [0.50, 0.38, 0.28], col2: [0.74, 0.58, 0.40], rarity: 1.0, shy: 2.5, speed: 0.35,
        desc: L("長い腕の先に大きなはさみを持つ、巨大なウミサソリ。大きな眼で獲物を見つけて襲った。", "A huge sea scorpion with big claws on long arms, hunting with large eyes.", "长臂末端长着大螯的巨型海蝎，用大眼睛寻找并袭击猎物。"),
        fact: L("平たい尾は、泳ぐときのかじの役目をした。", "Its flat tail worked like a rudder when it swam.", "扁平的尾巴在游泳时起舵的作用。"),
    },
    // ---- デボン紀
    Species {
        slug: "drepanophycus",
        name: L("ドレパノフィクス", "Drepanophycus", "镰蕨"),
        sci: "Drepanophycus spinaeformis",
        group: L("ヒカゲノカズラ類", "Lycopsid (clubmoss)", "石松类"),
        len_cm: 40.0, t0: 420.0, t1: 385.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(11), col: [0.30, 0.48, 0.26], col2: [0.44, 0.58, 0.30], rarity: 3.5, shy: 0.0, speed: 0.0,
        desc: L("トゲのような小さな葉がついた茎が、地面をはうようにのびた植物。", "A clubmoss whose stems, covered in small thorn-like leaves, crept along the ground.", "长着刺状小叶的茎贴地蔓延的石松类植物。"),
        fact: L("世界中で見つかり、デボン紀前期の水辺の代表的な植物。", "Found worldwide, it was a typical waterside plant of the Early Devonian.", "在世界各地都有发现，是早泥盆世水边的代表植物。"),
    },
    Species {
        slug: "aglaophyton",
        name: L("アグラオフィトン", "Aglaophyton", "光蕨"),
        sci: "Aglaophyton majus",
        group: L("初期の陸上植物", "Early land plant", "早期陆生植物"),
        len_cm: 18.0, t0: 411.0, t1: 405.0, hab: LAND | TIDAL, regions: &[Site(S_RHYNIE)],
        beh: Beh::Sessile, art: Art::Plant(0), col: [0.36, 0.52, 0.28], col2: [0.62, 0.50, 0.30], rarity: 4.0, shy: 0.0, speed: 0.0,
        desc: L("ライニーの温泉湿地に茂った、葉のない茎だけの植物。根もとで菌類と助け合っていた。", "A leafless stemmed plant of the Rhynie hot springs that partnered with fungi in its roots.", "生长在莱尼温泉湿地、只有茎没有叶的植物，在根部与真菌互助共生。"),
        fact: L("植物と菌類の「共生」を示す、最古の証拠の一つ。", "One of the oldest pieces of evidence for plants and fungi living in partnership.", "是植物与真菌共生的最古老证据之一。"),
    },
    Species {
        slug: "rhyniognatha",
        name: L("リニオグナタ", "Rhyniognatha", "莱尼颚虫"),
        sci: "Rhyniognatha hirsti",
        group: L("昆虫（最古の昆虫か）", "Insect (perhaps the oldest)", "昆虫（可能是最古老的）"),
        len_cm: 0.5, t0: 411.0, t1: 405.0, hab: LAND, regions: &[Site(S_RHYNIE)],
        beh: Beh::Crawl, art: Art::Arthro(4), col: [0.34, 0.28, 0.22], col2: [0.56, 0.46, 0.34], rarity: 2.5, shy: 0.3, speed: 0.02,
        desc: L("あごの化石だけが見つかっている、とても小さな虫。最古の昆虫と考えられている。", "A tiny creature known only from its jaws, thought to be the oldest insect.", "只发现了颚部化石的微小虫子，被认为是最古老的昆虫。"),
        fact: L("あごの形から、はねがあったかもしれないとも言われる。", "The shape of its jaws hints that it may even have had wings.", "从颚的形状推测，它甚至可能有翅膀。"),
    },
    Species {
        slug: "drepanaspis",
        name: L("ドレパナスピス", "Drepanaspis", "镰甲鱼"),
        sci: "Drepanaspis gemuendenensis",
        group: L("無顎類（異甲類）", "Jawless fish (heterostracan)", "无颌类（异甲鱼）"),
        len_cm: 35.0, t0: 410.0, t1: 400.0, hab: SHALLOW | SHELF, regions: &[Site(S_HUNSRUCK), Plate(AVALONIA), Plate(BALTICA)],
        beh: Beh::NearFloor, art: Art::Jawless(0), col: [0.50, 0.48, 0.42], col2: [0.70, 0.66, 0.56], rarity: 2.0, shy: 1.2, speed: 0.2,
        desc: L("せんべいのように平たい、あごのない魚。口は上向きで、海底の泥の中の餌を食べた。", "A pancake-flat jawless fish with an upturned mouth that fed on mud on the sea floor.", "身体扁平如煎饼的无颌鱼，口朝上，以海底泥中的食物为食。"),
        fact: L("ドイツのフンスリュック粘板岩から、きれいな化石が見つかる。", "Beautiful fossils come from Germany's Hunsrück Slate.", "德国洪斯吕克板岩中出产精美化石。"),
    },
    Species {
        slug: "holoptychius",
        name: L("ホロプティキウス", "Holoptychius", "全褶鱼"),
        sci: "Holoptychius",
        group: L("肉鰭類", "Lobe-finned fish", "肉鳍鱼类"),
        len_cm: 70.0, t0: 380.0, t1: 360.0, hab: TIDAL | SHALLOW, regions: &[Global],
        beh: Beh::Swim, art: Art::LobeFin(0), col: [0.46, 0.44, 0.36], col2: [0.66, 0.60, 0.44], rarity: 2.5, shy: 2.0, speed: 0.4,
        desc: L("大きな波もようのうろこを持つ、川の捕食者の肉鰭類。", "A lobe-finned river predator with large, wavy-patterned scales.", "长着大型波纹鳞片的河流捕食性肉鳍鱼。"),
        fact: L("世界中のデボン紀後期の地層から見つかる。", "It is found in Late Devonian rocks worldwide.", "在世界各地的晚泥盆世地层中都有发现。"),
    },
    Species {
        slug: "titanichthys",
        name: L("ティタニクティス", "Titanichthys", "巨鱼"),
        sci: "Titanichthys",
        group: L("板皮類（節頸類）", "Placoderm (arthrodire)", "盾皮鱼类（节颈鱼）"),
        len_cm: 500.0, t0: 370.0, t1: 360.0, hab: SHELF | DEEP, regions: &[Plate(LAURENTIA), Plate(GONDWANA)],
        beh: Beh::Swim, art: Art::Placoderm(1), col: [0.44, 0.46, 0.48], col2: [0.68, 0.66, 0.60], rarity: 0.6, shy: 2.5, speed: 0.3,
        desc: L("ダンクルオステウスと同じくらい大きいが、歯のないあごで小さな生きものをこして食べた、おだやかな巨大魚。", "A giant as big as Dunkleosteus, but a gentle one: its toothless jaws filtered small creatures from the water.", "体型与邓氏鱼相当，却用无齿的颌滤食小生物的温和巨鱼。"),
        fact: L("いまのジンベエザメのような暮らし方をしていたらしい。", "It probably lived like a modern whale shark.", "它的生活方式可能类似现代的鲸鲨。"),
    },
    Species {
        slug: "panderichthys",
        name: L("パンデリクティス", "Panderichthys", "潘氏鱼"),
        sci: "Panderichthys rhombolepis",
        group: L("肉鰭類（魚と四肢動物の間）", "Lobe-finned 'fishapod'", "肉鳍鱼类（鱼与四足动物之间）"),
        len_cm: 100.0, t0: 385.0, t1: 380.0, hab: TIDAL | SHALLOW, regions: &[Plate(BALTICA)],
        beh: Beh::NearFloor, art: Art::LobeFin(1), col: [0.46, 0.44, 0.34], col2: [0.66, 0.58, 0.42], rarity: 1.5, shy: 2.0, speed: 0.25,
        desc: L("平たい頭と、背びれのない体を持つ魚。浅い水の中で、四肢動物に近い暮らしをしていた。", "A flat-headed fish without dorsal fins that lived in shallow water much like an early tetrapod.", "头部扁平、没有背鳍的鱼，在浅水中过着接近四足动物的生活。"),
        fact: L("胸びれの先に、指のもとになる骨が見つかっている。", "The tips of its pectoral fins contain bones like the beginnings of fingers.", "其胸鳍末端发现了类似手指雏形的骨头。"),
    },
    Species {
        slug: "clymenia",
        name: L("クリメニア", "Clymenia", "海神石"),
        sci: "Clymenia",
        group: L("頭足類（アンモノイド）", "Cephalopod (ammonoid)", "头足类（菊石类）"),
        len_cm: 6.0, t0: 368.0, t1: 359.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Swim, art: Art::Cephalopod(1), col: [0.74, 0.62, 0.48], col2: [0.50, 0.36, 0.26], rarity: 2.5, shy: 1.2, speed: 0.2,
        desc: L("デボン紀の終わりに栄えたアンモノイド。体の管が、ほかのアンモノイドと反対の側を通っていた。", "An ammonoid of the latest Devonian whose internal tube ran along the opposite side from other ammonoids.", "繁盛于泥盆纪末的菊石类，体管位于与其他菊石相反的一侧。"),
        fact: L("デボン紀末の絶滅で、ほとんどが姿を消した。", "Almost all of them vanished in the end-Devonian extinction.", "在泥盆纪末的灭绝中几乎全部消失。"),
    },
    Species {
        slug: "helianthaster",
        name: L("ヘリアンタスター", "Helianthaster", "日光海星"),
        sci: "Helianthaster rhenanus",
        group: L("棘皮動物（ヒトデの仲間）", "Echinoderm (starfish relative)", "棘皮动物（海星类）"),
        len_cm: 25.0, t0: 408.0, t1: 404.0, hab: SHELF, regions: &[Site(S_HUNSRUCK)],
        beh: Beh::Crawl, art: Art::Starfish, col: [0.72, 0.52, 0.44], col2: [0.88, 0.72, 0.60], rarity: 2.0, shy: 0.0, speed: 0.01,
        desc: L("16本もの長い腕を持つ、太陽のような形のヒトデの仲間。", "A sun-shaped starfish relative with as many as 16 long arms.", "长着多达16条长腕、形如太阳的海星类。"),
        fact: L("フンスリュック粘板岩の、黄鉄鉱に置きかわった美しい化石が有名。", "It is famous from beautiful pyrite-replaced fossils in the Hunsrück Slate.", "洪斯吕克板岩中被黄铁矿置换的精美化石很有名。"),
    },
    Species {
        slug: "calamophyton",
        name: L("カラモフィトン", "Calamophyton", "芦蕨"),
        sci: "Calamophyton",
        group: L("クラドキシロン類（初期の木）", "Cladoxylopsid (early tree)", "枝蕨类（早期树木）"),
        len_cm: 300.0, t0: 390.0, t1: 380.0, hab: LAND, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(3), col: [0.46, 0.38, 0.28], col2: [0.34, 0.50, 0.26], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L("高さ2〜3mの、ヤシに似た初期の木。枝を落としながら育ち、最古の森の一部をつくった。", "A palm-like early tree 2–3 m tall that shed its branches as it grew, part of the oldest forests.", "高2至3米、形似棕榈的早期树木，边生长边落枝，是最古老森林的组成部分。"),
        fact: L("落とした枝が積もって、森の土を豊かにした。", "Its fallen branches enriched the forest soil.", "落下的枝条堆积起来，使森林土壤变得肥沃。"),
    },
    Species {
        slug: "attercopus",
        name: L("アッテルコプス", "Attercopus", "毒蛛"),
        sci: "Attercopus fimbriunguis",
        group: L("クモ形類（クモの祖先の仲間）", "Arachnid (spider relative)", "蛛形类（蜘蛛祖先类）"),
        len_cm: 0.5, t0: 387.0, t1: 381.0, hab: LAND, regions: &[Site(S_GILBOA), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Arthro(3), col: [0.30, 0.24, 0.20], col2: [0.50, 0.40, 0.32], rarity: 2.5, shy: 0.3, speed: 0.02,
        desc: L("糸を出すことができた、クモの祖先に近い小さな生きもの。まだ網は張らなかったらしい。", "A tiny spider relative that could already make silk, though it probably did not spin webs.", "已能吐丝的微小蜘蛛近亲，但可能还不会结网。"),
        fact: L("名前は古い英語で「毒の頭」、つまりクモのこと。", "Its name comes from an Old English word for spider.", "名字来自古英语中“蜘蛛”一词。"),
    },
    // ---- 石炭紀
    Species {
        slug: "falcatus",
        name: L("ファルカトゥス", "Falcatus", "镰鳍鲨"),
        sci: "Falcatus falcatus",
        group: L("軟骨魚類（サメの仲間）", "Shark relative", "软骨鱼类（鲨类）"),
        len_cm: 30.0, t0: 325.0, t1: 318.0, hab: SHALLOW, regions: &[Site(S_BEARGULCH)],
        beh: Beh::Swim, art: Art::Shark(1), col: [0.42, 0.44, 0.46], col2: [0.20, 0.20, 0.22], rarity: 2.5, shy: 1.6, speed: 0.5,
        desc: L("オスの頭の上に、刀のような長いトゲが前向きについていた小さなサメの仲間。", "A small shark relative; males had a long sword-like spine curving forward over the head.", "小型鲨类，雄性头顶有一根向前弯曲的长刀状棘刺。"),
        fact: L("メスがオスのトゲをくわえた姿の化石があり、求愛のようすとも言われる。", "A fossil of a female holding a male's spine may show courtship.", "有雌性咬住雄性棘刺的化石，被认为可能是求偶的情景。"),
    },
    Species {
        slug: "edestus",
        name: L("エデスタス", "Edestus", "剪齿鲨"),
        sci: "Edestus",
        group: L("軟骨魚類（ギンザメの仲間）", "Ratfish relative", "软骨鱼类（银鲛类）"),
        len_cm: 600.0, t0: 315.0, t1: 300.0, hab: SHELF | DEEP, regions: &[Global],
        beh: Beh::Swim, art: Art::Shark(2), col: [0.42, 0.44, 0.48], col2: [0.84, 0.82, 0.76], rarity: 0.7, shy: 3.0, speed: 0.45,
        desc: L("上下のあごに、はさみのように並んだ歯の列を持つ大きな魚。獲物を切りさいた。", "A large fish with rows of teeth on both jaws that worked like shears to slice prey.", "上下颌都有剪刀般齿列的大型鱼类，用来切割猎物。"),
        fact: L("ヘリコプリオンの親戚で、「はさみの歯を持つサメ」と呼ばれる。", "A relative of Helicoprion, nicknamed the 'scissor-toothed shark'.", "是旋齿鲨的亲戚，被称为“剪刀齿鲨”。"),
    },
    Species {
        slug: "rhizodus",
        name: L("リゾドゥス", "Rhizodus", "根齿鱼"),
        sci: "Rhizodus hibberti",
        group: L("肉鰭類", "Lobe-finned fish", "肉鳍鱼类"),
        len_cm: 600.0, t0: 345.0, t1: 320.0, hab: TIDAL | SHALLOW, regions: &[Plate(LAURENTIA), Plate(BALTICA), Plate(AVALONIA)],
        beh: Beh::Swim, art: Art::LobeFin(0), col: [0.40, 0.38, 0.30], col2: [0.62, 0.56, 0.40], rarity: 0.7, shy: 3.0, speed: 0.45,
        desc: L("長さ6mにもなる、史上最大級の淡水魚。長さ22cmの牙で、大きな獲物も襲った。", "One of the largest freshwater fish ever, up to 6 m long, with fangs 22 cm long.", "体长可达6米，是史上最大的淡水鱼之一，长着22厘米长的獠牙。"),
        fact: L("初期の四肢動物にとって、最もおそろしい敵だった。", "It was the most fearsome enemy of early tetrapods.", "是早期四足动物最可怕的敌人。"),
    },
    Species {
        slug: "crassigyrinus",
        name: L("クラッシギリヌス", "Crassigyrinus", "厚蛙螈"),
        sci: "Crassigyrinus scoticus",
        group: L("四肢動物", "Early tetrapod", "四足动物"),
        len_cm: 200.0, t0: 335.0, t1: 325.0, hab: TIDAL | SHALLOW, regions: &[Plate(LAURENTIA)],
        beh: Beh::NearFloor, art: Art::Tetrapod(0), col: [0.38, 0.40, 0.32], col2: [0.58, 0.56, 0.42], rarity: 1.2, shy: 2.0, speed: 0.25,
        desc: L("小さな手足と大きな口を持つ、水の中で待ちぶせした四肢動物。", "A tetrapod with tiny limbs and a huge mouth that ambushed prey in the water.", "四肢短小、嘴巴巨大，在水中伏击猎物的四足动物。"),
        fact: L("一度陸に近づいた仲間が、また水の暮らしにもどったと考えられている。", "Its ancestors may have returned to water after moving toward land.", "人们认为它的祖先曾走向陆地，后来又回到了水中。"),
    },
    Species {
        slug: "westlothiana",
        name: L("ウェストロティアナ", "Westlothiana", "西洛锡安螈"),
        sci: "Westlothiana lizziae",
        group: L("爬形類", "Reptiliomorph", "爬行形类"),
        len_cm: 20.0, t0: 337.0, t1: 333.0, hab: LAND | TIDAL, regions: &[Site(S_EASTKIRKTON)],
        beh: Beh::Crawl, art: Art::Tetrapod(2), col: [0.48, 0.42, 0.32], col2: [0.66, 0.58, 0.42], rarity: 2.5, shy: 1.0, speed: 0.1,
        desc: L("トカゲに似た細長い体の、小さな四肢動物。爬虫類に近い仲間と考えられる。", "A small, slender, lizard-like tetrapod, thought to be close to the reptiles.", "身体细长、形似蜥蜴的小型四足动物，被认为接近爬行动物。"),
        fact: L("発見者は愛称で「リジー」と呼んだ。", "Its finder nicknamed it 'Lizzie'.", "发现者给它起了昵称“丽兹”。"),
    },
    Species {
        slug: "balanerpeton",
        name: L("バラネルペトン", "Balanerpeton", "巴兰螈"),
        sci: "Balanerpeton woodi",
        group: L("両生類（分椎類）", "Amphibian (temnospondyl)", "两栖类（离片椎类）"),
        len_cm: 50.0, t0: 337.0, t1: 333.0, hab: LAND | TIDAL, regions: &[Site(S_EASTKIRKTON)],
        beh: Beh::Crawl, art: Art::Tetrapod(3), col: [0.42, 0.42, 0.34], col2: [0.60, 0.58, 0.44], rarity: 2.5, shy: 1.2, speed: 0.08,
        desc: L("陸を歩くのに向いた手足と、大きな耳の膜を持つ両生類。", "An amphibian with limbs suited to walking on land and a large eardrum.", "四肢适合在陆地行走、长有大鼓膜的两栖动物。"),
        fact: L("空気中の音を聞くための耳を持った、最古の動物の一つ。", "One of the oldest animals with ears for hearing sounds in air.", "是拥有能听到空气中声音的耳朵的最古老动物之一。"),
    },
    Species {
        slug: "megarachne",
        name: L("メガラクネ", "Megarachne", "巨蛛鲎"),
        sci: "Megarachne servinei",
        group: L("ウミサソリ", "Sea scorpion (eurypterid)", "板足鲎（海蝎）"),
        len_cm: 54.0, t0: 300.0, t1: 299.0, hab: TIDAL | SHALLOW, regions: &[Plate(GONDWANA)],
        beh: Beh::NearFloor, art: Art::Eurypterid(0), col: [0.42, 0.34, 0.26], col2: [0.62, 0.52, 0.38], rarity: 1.5, shy: 1.5, speed: 0.15,
        desc: L("はじめは「史上最大のクモ」と思われた、川辺のウミサソリの仲間。", "A river-dwelling sea scorpion relative first mistaken for the largest spider ever.", "最初被误认为“史上最大蜘蛛”的河边海蝎类。"),
        fact: L("よい化石が見つかって、2005年にクモではないとわかった。", "A better fossil showed in 2005 that it was not a spider after all.", "2005年发现了更好的化石，才确认它并不是蜘蛛。"),
    },
    Species {
        slug: "petrolacosaurus",
        name: L("ペトロラコサウルス", "Petrolacosaurus", "林蜥龙"),
        sci: "Petrolacosaurus kansensis",
        group: L("爬虫類（双弓類）", "Reptile (diapsid)", "爬行动物（双孔类）"),
        len_cm: 40.0, t0: 302.0, t1: 299.0, hab: LAND, regions: &[Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Tetrapod(2), col: [0.52, 0.44, 0.32], col2: [0.70, 0.60, 0.44], rarity: 2.0, shy: 1.2, speed: 0.16,
        desc: L("首と尾の長い、すばしこい小型の爬虫類。頭の後ろに穴が2つずつあった。", "A quick little reptile with a long neck and tail, and two openings on each side of the skull.", "颈和尾都很长、行动敏捷的小型爬行动物，头骨两侧各有两个孔。"),
        fact: L("恐竜やトカゲ、鳥へとつながる「双弓類」の最古の仲間の一つ。", "One of the oldest diapsids — the group that later gave rise to dinosaurs, lizards and birds.", "是后来演化出恐龙、蜥蜴和鸟类的“双孔类”中最古老的成员之一。"),
    },
    Species {
        slug: "archaeothyris",
        name: L("アーケオシリス", "Archaeothyris", "始祖单弓兽"),
        sci: "Archaeothyris florensis",
        group: L("単弓類", "Synapsid", "合弓纲"),
        len_cm: 50.0, t0: 307.0, t1: 304.0, hab: LAND, regions: &[Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Tetrapod(2), col: [0.50, 0.42, 0.32], col2: [0.68, 0.58, 0.42], rarity: 2.0, shy: 1.2, speed: 0.12,
        desc: L("トカゲに似た姿だが、わたしたち哺乳類につながる単弓類の、最古の仲間の一つ。", "Lizard-like in shape, but one of the oldest synapsids — the line that leads to mammals like us.", "外形似蜥蜴，却是通向我们哺乳动物的合弓纲中最古老的成员之一。"),
        fact: L("頭の後ろの穴が1つずつ。この特徴は、いまのわたしたちの頭にも残っている。", "It had one opening behind each eye — a feature still found in our own skulls.", "眼后两侧各有一个孔，这一特征至今仍保留在我们的头骨中。"),
    },
    Species {
        slug: "essexella",
        name: L("エセクセラ", "Essexella", "埃塞克斯水母"),
        sci: "Essexella asherae",
        group: L("刺胞動物（クラゲ）", "Cnidarian (jellyfish)", "刺胞动物（水母）"),
        len_cm: 12.0, t0: 310.0, t1: 306.0, hab: SHALLOW | TIDAL, regions: &[Site(S_MAZON)],
        beh: Beh::Drift, art: Art::Jelly, col: [0.80, 0.86, 0.90], col2: [0.62, 0.72, 0.80], rarity: 3.0, shy: 0.0, speed: 0.03,
        desc: L("ふちから幕のようなスカートを垂らしたクラゲ。三角州の入り江にたくさんいた。", "A jellyfish with a curtain-like skirt hanging from its rim, abundant in the delta's bays.", "伞缘垂下帘幕般裙边的水母，在三角洲海湾中数量很多。"),
        fact: L("メゾンクリークで最も多く見つかる化石。", "It is the most common fossil at Mazon Creek.", "是梅森溪发现最多的化石。"),
    },
    Species {
        slug: "lyginopteris",
        name: L("リギノプテリス（種子シダ）", "Lyginopteris (seed fern)", "皱羊齿"),
        sci: "Lyginopteris",
        group: L("種子シダ類", "Seed fern", "种子蕨类"),
        len_cm: 500.0, t0: 330.0, t1: 310.0, hab: LAND, regions: &[Tropics],
        beh: Beh::Sessile, art: Art::Plant(6), col: [0.40, 0.32, 0.24], col2: [0.34, 0.52, 0.26], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L("ほかの木によりかかってのびる、つる性の種子シダ。シダのような葉に種をつけた。", "A scrambling seed fern that leaned on other trees, bearing seeds on fern-like fronds.", "倚靠其他树木攀援生长的种子蕨，在蕨状叶上结种子。"),
        fact: L("「シダなのに種がある」ことがわかった、記念すべき植物。", "It was the plant that showed some 'ferns' actually made seeds.", "是证明某些“蕨类”其实会结种子的里程碑式植物。"),
    },
    // ---- ペルム紀
    Species {
        slug: "ophiacodon",
        name: L("オフィアコドン", "Ophiacodon", "蛇齿龙"),
        sci: "Ophiacodon",
        group: L("単弓類（盤竜類）", "Synapsid (pelycosaur)", "合弓纲（盘龙类）"),
        len_cm: 250.0, t0: 300.0, t1: 280.0, hab: TIDAL | LAND, regions: &[Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Tetrapod(5), col: [0.46, 0.40, 0.32], col2: [0.66, 0.58, 0.44], rarity: 1.5, shy: 1.8, speed: 0.1,
        desc: L("長い頭を持つ、水辺の大型の単弓類。魚などを捕らえて食べたらしい。", "A large waterside synapsid with a long head that probably caught fish.", "头部很长的水边大型合弓纲动物，可能捕鱼为食。"),
        fact: L("ディメトロドンより前に栄えた、帆のない仲間。", "A sail-less relative that flourished before Dimetrodon.", "是比异齿龙更早繁盛、没有背帆的亲戚。"),
    },
    Species {
        slug: "cacops",
        name: L("カコプス", "Cacops", "蛙螈"),
        sci: "Cacops aspidephorus",
        group: L("両生類（分椎類）", "Amphibian (temnospondyl)", "两栖类（离片椎类）"),
        len_cm: 40.0, t0: 280.0, t1: 275.0, hab: LAND, regions: &[Site(S_TEXAS), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Tetrapod(3), col: [0.50, 0.40, 0.30], col2: [0.68, 0.56, 0.40], rarity: 2.0, shy: 1.2, speed: 0.1,
        desc: L("背中に骨の板が並んだ、がっしりした両生類。乾いた陸でも暮らせた。", "A sturdy amphibian with a row of bony plates along its back, able to live on dry land.", "背部排列着骨板的粗壮两栖动物，能在干燥的陆地上生活。"),
        fact: L("大きな耳の膜で、夜に音を聞いて獲物を探したのかもしれない。", "Its large eardrum may have helped it hunt by sound at night.", "大鼓膜或许能帮它在夜间凭声音寻找猎物。"),
    },
    Species {
        slug: "varanops",
        name: L("ヴァラノプス", "Varanops", "巨蜥龙"),
        sci: "Varanops brevirostris",
        group: L("単弓類（盤竜類）", "Synapsid (pelycosaur)", "合弓纲（盘龙类）"),
        len_cm: 120.0, t0: 279.0, t1: 272.0, hab: LAND, regions: &[Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Tetrapod(2), col: [0.52, 0.42, 0.32], col2: [0.72, 0.60, 0.44], rarity: 1.8, shy: 1.6, speed: 0.16,
        desc: L("オオトカゲに似た、すばしこい肉食の単弓類。", "A quick, meat-eating synapsid shaped like a monitor lizard.", "外形似巨蜥、行动敏捷的肉食性合弓纲动物。"),
        fact: L("ほかの動物にかじられた跡のある化石があり、食べ合いのようすがわかる。", "A fossil with bite marks from scavengers shows who ate whom.", "有带被啃咬痕迹的化石，可以看出动物间的捕食关系。"),
    },
    Species {
        slug: "estemmenosuchus",
        name: L("エステメノスクス", "Estemmenosuchus", "冠鳄兽"),
        sci: "Estemmenosuchus uralensis",
        group: L("単弓類（恐頭類）", "Synapsid (dinocephalian)", "合弓纲（恐头兽类）"),
        len_cm: 300.0, t0: 267.0, t1: 264.0, hab: LAND, regions: &[Plate(BALTICA)],
        beh: Beh::Crawl, art: Art::Tetrapod(9), col: [0.56, 0.46, 0.36], col2: [0.42, 0.34, 0.26], rarity: 1.5, shy: 1.5, speed: 0.08,
        desc: L("頭に角のような大きなこぶがいくつもついた、大型の単弓類。", "A big synapsid with several large horn-like bosses on its head.", "头上长着数个角状大瘤的大型合弓纲动物。"),
        fact: L("名前は「冠をかぶったワニ」という意味。", "Its name means 'crowned crocodile'.", "名字意为“戴冠的鳄鱼”。"),
    },
    Species {
        slug: "titanophoneus",
        name: L("ティタノフォネウス", "Titanophoneus", "巨型兽"),
        sci: "Titanophoneus potens",
        group: L("単弓類（恐頭類）", "Synapsid (dinocephalian)", "合弓纲（恐头兽类）"),
        len_cm: 250.0, t0: 266.0, t1: 262.0, hab: LAND, regions: &[Plate(BALTICA)],
        beh: Beh::Crawl, art: Art::Tetrapod(8), col: [0.50, 0.42, 0.32], col2: [0.78, 0.70, 0.56], rarity: 1.2, shy: 2.0, speed: 0.16,
        desc: L("長い犬歯を持つ、ペルム紀中ごろの大型の捕食者。", "A large predator of the Middle Permian with long canine teeth.", "长着长犬齿的中二叠世大型捕食者。"),
        fact: L("名前は「巨大な殺し屋」という意味。", "Its name means 'titanic murderer'.", "名字意为“巨大的杀手”。"),
    },
    Species {
        slug: "coelurosauravus",
        name: L("コエルロサウラヴス", "Coelurosauravus", "空尾蜥"),
        sci: "Coelurosauravus",
        group: L("爬虫類（滑空する爬虫類）", "Gliding reptile", "爬行动物（滑翔爬行动物）"),
        len_cm: 40.0, t0: 260.0, t1: 255.0, hab: LAND, regions: &[Plate(GONDWANA), Plate(BALTICA)],
        beh: Beh::Crawl, art: Art::Tetrapod(2), col: [0.48, 0.42, 0.32], col2: [0.80, 0.66, 0.46], rarity: 2.0, shy: 1.5, speed: 0.18,
        desc: L("体の横に広げられる膜の翼を持ち、木から木へ滑空した爬虫類。", "A reptile with membrane wings it could spread from its sides to glide between trees.", "身体两侧能展开膜翼、在树间滑翔的爬行动物。"),
        fact: L("空を飛んだ（滑空した）、最古の背骨のある動物。", "The oldest known vertebrate that could glide through the air.", "是已知最早能在空中滑翔的脊椎动物。"),
    },
    Species {
        slug: "prionosuchus",
        name: L("プリオノスクス", "Prionosuchus", "锯齿螈"),
        sci: "Prionosuchus plummeri",
        group: L("両生類（分椎類）", "Amphibian (temnospondyl)", "两栖类（离片椎类）"),
        len_cm: 550.0, t0: 272.0, t1: 268.0, hab: TIDAL | SHALLOW, regions: &[Plate(GONDWANA)],
        beh: Beh::Swim, art: Art::Tetrapod(7), col: [0.42, 0.42, 0.34], col2: [0.62, 0.60, 0.46], rarity: 0.8, shy: 2.5, speed: 0.35,
        desc: L("ワニのような細長いあごを持つ、最大級の両生類。川や湖で魚を捕らえた。", "One of the largest amphibians ever, with long crocodile-like jaws for catching fish in rivers and lakes.", "长着鳄鱼般细长颌部、史上最大的两栖动物之一，在河湖中捕鱼。"),
        fact: L("ブラジルで見つかり、長さは5m以上と考えられている。", "Found in Brazil, it may have been over 5 m long.", "发现于巴西，体长估计超过5米。"),
    },
    Species {
        slug: "dorypterus",
        name: L("ドリプテルス", "Dorypterus", "矛鳍鱼"),
        sci: "Dorypterus hoffmanni",
        group: L("条鰭類（硬骨魚）", "Ray-finned fish", "辐鳍鱼类"),
        len_cm: 15.0, t0: 259.0, t1: 256.0, hab: SHALLOW, regions: &[Plate(BALTICA)],
        beh: Beh::Swim, art: Art::RayFin, col: [0.50, 0.48, 0.42], col2: [0.80, 0.74, 0.58], rarity: 2.5, shy: 1.2, speed: 0.4,
        desc: L("背中に旗のような長いひれを持つ、体高の高い変わった魚。", "An odd, deep-bodied fish with a long, flag-like fin on its back.", "身体高耸、背上长着旗状长鳍的奇特鱼类。"),
        fact: L("いまのマンボウのような形の魚が、ペルム紀にもいた。", "Strangely shaped fish like this lived in the Permian, too.", "二叠纪也有这样形状奇特的鱼。"),
    },
    Species {
        slug: "meganeuropsis",
        name: L("メガネウロプシス", "Meganeuropsis", "二叠拟巨脉蜓"),
        sci: "Meganeuropsis permiana",
        group: L("昆虫（オオトンボ類）", "Insect (griffinfly)", "昆虫（原蜻蜓类）"),
        len_cm: 43.0, t0: 280.0, t1: 272.0, hab: LAND | TIDAL, regions: &[Plate(LAURENTIA)],
        beh: Beh::Swim, art: Art::Griffinfly, col: [0.30, 0.34, 0.30], col2: [0.82, 0.86, 0.90], rarity: 1.2, shy: 2.0, speed: 0.8,
        desc: L("はねを広げると71cm。知られている中で最大の昆虫。", "With a 71 cm wingspan, the largest insect known.", "翼展71厘米，是已知最大的昆虫。"),
        fact: L("メガネウラよりもわずかに大きかった。", "It was slightly bigger than Meganeura.", "比巨脉蜻蜓还要稍大一些。"),
    },
    Species {
        slug: "euchambersia",
        name: L("エウチャンベルシア", "Euchambersia", "真钱伯斯兽"),
        sci: "Euchambersia mirabilis",
        group: L("単弓類（獣頭類）", "Synapsid (therocephalian)", "合弓纲（兽头类）"),
        len_cm: 40.0, t0: 259.0, t1: 255.0, hab: LAND, regions: &[Site(S_KAROO), Plate(GONDWANA)],
        beh: Beh::Crawl, art: Art::Tetrapod(8), col: [0.54, 0.44, 0.34], col2: [0.78, 0.70, 0.56], rarity: 1.5, shy: 1.5, speed: 0.14,
        desc: L("顔のくぼみと溝のある牙から、毒を持っていたかもしれないと考えられる小型の捕食者。", "A small predator whose grooved fangs and facial pit suggest it may have been venomous.", "从脸部凹陷和带沟的獠牙推测，可能有毒的小型捕食者。"),
        fact: L("毒を持っていたとすれば、最古の毒を持つ単弓類になる。", "If venomous, it would be the oldest venomous synapsid.", "如果确实有毒，它将是最古老的有毒合弓纲动物。"),
    },
    Species {
        slug: "voltzia",
        name: L("ボルチア", "Voltzia", "伏脂杉"),
        sci: "Voltzia",
        group: L("針葉樹", "Conifer", "针叶树"),
        len_cm: 1000.0, t0: 260.0, t1: 252.0, hab: LAND, regions: &[Plate(BALTICA), Plate(LAURENTIA)],
        beh: Beh::Sessile, art: Art::Plant(8), col: [0.36, 0.30, 0.24], col2: [0.24, 0.40, 0.22], rarity: 2.5, shy: 0.0, speed: 0.0,
        desc: L("乾いた土地に育った、ペルム紀の終わりの針葉樹。", "A conifer of the latest Permian that grew on dry land.", "生长在干燥土地上的二叠纪末针叶树。"),
        fact: L("大絶滅を生きのび、三畳紀にも森をつくった。", "It survived the great extinction and formed forests in the Triassic too.", "在大灭绝中幸存，三叠纪也形成了森林。"),
    },
    // ======================================================== 三畳紀
    Species {
        slug: "thrinaxodon",
        name: L("トリナクソドン", "Thrinaxodon", "三尖叉齿兽"),
        sci: "Thrinaxodon liorhinus",
        group: L("単弓類（キノドン類）", "Synapsid (cynodont)", "合弓纲（犬齿兽类）"),
        len_cm: 40.0, t0: 251.0, t1: 247.0, hab: LAND, regions: &[Plate(GONDWANA)],
        beh: Beh::Crawl, art: Art::Mammal(3), col: [0.56, 0.48, 0.38], col2: [0.74, 0.66, 0.52], rarity: 2.5, shy: 1.2, speed: 0.12,
        desc: L("大絶滅のすぐあとに、巣穴を掘って暮らした小さな動物。哺乳類にとても近い仲間。", "A small burrowing animal living just after the great extinction, a very close relative of mammals.", "大灭绝后不久挖洞生活的小型动物，是哺乳动物的近亲。"),
        fact: L("けがをした両生類と一緒に、同じ巣穴で丸まった化石が見つかったことがある。", "One fossil was found curled up in the same burrow as an injured amphibian.", "曾发现它蜷缩在巢穴中的化石，旁边还有一只受伤的两栖动物。"),
    },
    Species {
        slug: "pleuromeia",
        name: L("プレウロメイア", "Pleuromeia", "肋木"),
        sci: "Pleuromeia",
        group: L("ヒカゲノカズラ類", "Lycopsid", "石松类"),
        len_cm: 200.0, t0: 251.5, t1: 245.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(11), col: [0.36, 0.40, 0.28], col2: [0.40, 0.56, 0.28], rarity: 4.0, shy: 0.0, speed: 0.0,
        desc: L("太い茎を一本だけのばした、高さ2mほどの植物。大絶滅のあと、荒れた大地に真っ先に広がった。", "A single-stemmed plant about 2 m tall that was among the first to spread across the barren land after the great extinction.", "只长一根粗茎、高约2米的植物，大灭绝后率先在荒芜大地上扩散。"),
        fact: L("日本（宮城県）からも化石が見つかっている。", "Its fossils have been found in Japan too.", "日本（宫城县）也发现过它的化石。"),
    },
    Species {
        slug: "triadobatrachus",
        name: L("トリアドバトラクス", "Triadobatrachus", "三叠蛙"),
        sci: "Triadobatrachus massinoti",
        group: L("両生類（カエルの祖先）", "Amphibian (frog ancestor)", "两栖类（蛙的祖先）"),
        len_cm: 10.0, t0: 250.0, t1: 248.0, hab: TIDAL | LAND, regions: &[Plate(GONDWANA)],
        beh: Beh::Crawl, art: Art::Tetrapod(3), col: [0.46, 0.50, 0.34], col2: [0.66, 0.70, 0.50], rarity: 2.0, shy: 1.0, speed: 0.08,
        desc: L("カエルの祖先に近い両生類。まだ短い尾があり、跳ねるより泳ぐほうが得意だった。", "A frog ancestor that still had a short tail and swam better than it hopped.", "接近青蛙祖先的两栖动物，还有短尾巴，比起跳跃更擅长游泳。"),
        fact: L("マダガスカルで見つかった。", "It was found in Madagascar.", "发现于马达加斯加。"),
    },
    Species {
        slug: "cynognathus",
        name: L("キノグナトゥス", "Cynognathus", "犬颌兽"),
        sci: "Cynognathus crateronotus",
        group: L("単弓類（キノドン類）", "Synapsid (cynodont)", "合弓纲（犬齿兽类）"),
        len_cm: 120.0, t0: 247.0, t1: 237.0, hab: LAND, regions: &[Plate(GONDWANA)],
        beh: Beh::Crawl, art: Art::Tetrapod(8), col: [0.54, 0.46, 0.36], col2: [0.78, 0.70, 0.56], rarity: 1.5, shy: 1.8, speed: 0.16,
        desc: L("オオカミほどの大きさの肉食動物。哺乳類のように、役割の違う歯を持っていた。", "A wolf-sized predator with different kinds of teeth, like a mammal.", "体型如狼的肉食动物，像哺乳动物一样有分工不同的牙齿。"),
        fact: L("南アメリカとアフリカの両方で見つかり、大陸がつながっていた証拠になった。", "Found in both South America and Africa, it helped show the continents were once joined.", "在南美洲和非洲都有发现，证明了大陆曾经相连。"),
    },
    Species {
        slug: "mixosaurus",
        name: L("ミクソサウルス", "Mixosaurus", "混鱼龙"),
        sci: "Mixosaurus",
        group: L("魚竜", "Ichthyosaur", "鱼龙"),
        len_cm: 100.0, t0: 245.0, t1: 235.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Swim, art: Art::Marine(0), col: [0.42, 0.46, 0.50], col2: [0.78, 0.80, 0.80], rarity: 2.5, shy: 2.0, speed: 0.6,
        desc: L("世界中の海にいた、小型の魚竜。細長いあごで魚やイカを捕らえた。", "A small ichthyosaur found in seas worldwide, catching fish and squid with its slender jaws.", "遍布全球海洋的小型鱼龙，用细长的颌捕捉鱼和鱿鱼。"),
        fact: L("トカゲのような祖先が、海で暮らすうちにイルカのような姿になった。", "Its lizard-like ancestors became dolphin-shaped after taking to the sea.", "它那蜥蜴般的祖先在海中生活后，变成了海豚般的模样。"),
    },
    Species {
        slug: "nothosaurus",
        name: L("ノトサウルス", "Nothosaurus", "幻龙"),
        sci: "Nothosaurus",
        group: L("偽竜類（海の爬虫類）", "Nothosaur (marine reptile)", "幻龙类（海生爬行动物）"),
        len_cm: 400.0, t0: 243.0, t1: 212.0, hab: SHALLOW | TIDAL, regions: &[Site(S_SANGIORGIO), Plate(BALTICA)],
        beh: Beh::Swim, art: Art::Marine(1), col: [0.42, 0.42, 0.38], col2: [0.72, 0.70, 0.60], rarity: 1.5, shy: 2.0, speed: 0.45,
        desc: L("長い首と、針のような歯の並んだあごを持つ海の爬虫類。浅い海で魚を捕らえ、岸で休んだ。", "A long-necked marine reptile with needle-toothed jaws that fished in shallow seas and rested on the shore.", "长颈、颌上排列针状牙齿的海生爬行动物，在浅海捕鱼，在岸上休息。"),
        fact: L("のちの首長竜の親戚と考えられている。", "It is thought to be a relative of the later plesiosaurs.", "被认为是后来蛇颈龙的亲戚。"),
    },
    Species {
        slug: "tanystropheus",
        name: L("タニストロフェウス", "Tanystropheus", "长颈龙"),
        sci: "Tanystropheus",
        group: L("主竜形類（首の長い爬虫類）", "Long-necked archosauromorph", "主龙形类（长颈爬行动物）"),
        len_cm: 600.0, t0: 243.0, t1: 239.0, hab: TIDAL | SHALLOW, regions: &[Site(S_SANGIORGIO)],
        beh: Beh::NearFloor, art: Art::Archosaur(3), col: [0.50, 0.46, 0.36], col2: [0.72, 0.66, 0.50], rarity: 1.2, shy: 1.5, speed: 0.2,
        desc: L("体長の半分が首という、とても首の長い爬虫類。浅い海で、長い首をのばして魚を捕らえた。", "A reptile whose neck was half its body length, used to snatch fish in shallow water.", "脖子占体长一半的长颈爬行动物，在浅海伸长脖子捕鱼。"),
        fact: L("首の骨は数が少なく、一つひとつがとても長かった。", "Its neck had few vertebrae, each one extremely long.", "颈椎数量不多，但每一节都非常长。"),
    },
    Species {
        slug: "placodus",
        name: L("プラコドゥス", "Placodus", "楯齿龙"),
        sci: "Placodus gigas",
        group: L("板歯類（海の爬虫類）", "Placodont (marine reptile)", "楯齿龙类（海生爬行动物）"),
        len_cm: 200.0, t0: 245.0, t1: 238.0, hab: SHALLOW, regions: &[Site(S_SANGIORGIO), Plate(BALTICA)],
        beh: Beh::NearFloor, art: Art::Marine(2), col: [0.48, 0.44, 0.38], col2: [0.70, 0.64, 0.52], rarity: 1.5, shy: 1.5, speed: 0.2,
        desc: L("平たい石うすのような歯で、海底の貝をかみくだいて食べた爬虫類。", "A reptile with flat, millstone-like teeth for crushing shellfish on the sea floor.", "用扁平如石磨的牙齿咬碎海底贝类的爬行动物。"),
        fact: L("前歯で貝をはがし、奥歯でくだいたと考えられる。", "It probably prised shellfish off rocks with its front teeth and crushed them with its back teeth.", "据推测它用门齿撬下贝类，再用臼齿咬碎。"),
    },
    Species {
        slug: "ceratites",
        name: L("セラタイト", "Ceratites", "齿菊石"),
        sci: "Ceratites nodosus",
        group: L("頭足類（アンモナイト）", "Cephalopod (ammonoid)", "头足类（菊石）"),
        len_cm: 10.0, t0: 242.0, t1: 237.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Swim, art: Art::Cephalopod(1), col: [0.76, 0.66, 0.52], col2: [0.52, 0.38, 0.28], rarity: 3.0, shy: 1.2, speed: 0.2,
        desc: L("こぶのある巻いた殻を持つアンモナイト。殻の部屋の壁が、のこぎりの歯のような線を描く。", "A ribbed, knobbly coiled ammonoid whose chamber walls trace saw-tooth lines.", "外壳带瘤的盘卷菊石，壳内隔壁形成锯齿状缝合线。"),
        fact: L("大絶滅をわずかに生きのびたアンモナイトの子孫が、三畳紀の海で再び大発展した。", "Descendants of the few ammonoids that survived the extinction boomed again in the Triassic.", "大灭绝中幸存的少数菊石的后代，在三叠纪海洋中再度繁盛。"),
    },
    Species {
        slug: "eoraptor",
        name: L("エオラプトル", "Eoraptor", "始盗龙"),
        sci: "Eoraptor lunensis",
        group: L("恐竜（初期の竜盤類）", "Dinosaur (early saurischian)", "恐龙（早期蜥臀类）"),
        len_cm: 100.0, t0: 231.0, t1: 229.0, hab: LAND, regions: &[Site(S_ISCHIGUALASTO), Plate(SOUTH_AMERICA)],
        beh: Beh::Crawl, art: Art::Dino(0), col: [0.56, 0.48, 0.36], col2: [0.74, 0.64, 0.46], rarity: 2.0, shy: 1.8, speed: 0.3,
        desc: L("長さ1mほどの、最も古い恐竜の一つ。二本足ですばやく走り、虫や小動物を食べた。", "One of the oldest dinosaurs, about 1 m long, running fast on two legs after insects and small animals.", "体长约1米，是最古老的恐龙之一，用两条腿快速奔跑，捕食昆虫和小动物。"),
        fact: L("名前は「夜明けの泥棒」。恐竜の時代の夜明けにいたから。", "Its name means 'dawn thief' — it lived at the dawn of the dinosaurs.", "名字意为“黎明的盗贼”，因为它生活在恐龙时代的黎明。"),
    },
    Species {
        slug: "herrerasaurus",
        name: L("ヘレラサウルス", "Herrerasaurus", "埃雷拉龙"),
        sci: "Herrerasaurus ischigualastensis",
        group: L("恐竜（初期の肉食恐竜）", "Dinosaur (early predator)", "恐龙（早期肉食恐龙）"),
        len_cm: 450.0, t0: 231.0, t1: 228.0, hab: LAND, regions: &[Site(S_ISCHIGUALASTO), Plate(SOUTH_AMERICA)],
        beh: Beh::Crawl, art: Art::Dino(1), col: [0.52, 0.42, 0.32], col2: [0.72, 0.60, 0.44], rarity: 1.2, shy: 2.5, speed: 0.35,
        desc: L("長さ4mほどの、初期の大型の肉食恐竜。するどい歯と爪で獲物を襲った。", "An early large meat-eating dinosaur about 4 m long, hunting with sharp teeth and claws.", "体长约4米的早期大型肉食恐龙，用锋利的牙齿和爪子捕猎。"),
        fact: L("このころの陸では、恐竜はまだ少数派で、ワニの仲間のほうが強かった。", "At this time dinosaurs were still a minority; crocodile relatives ruled the land.", "这时恐龙在陆地上还是少数派，鳄类亲戚更为强大。"),
    },
    Species {
        slug: "coelophysis",
        name: L("コエロフィシス", "Coelophysis", "腔骨龙"),
        sci: "Coelophysis bauri",
        group: L("恐竜（肉食恐竜）", "Dinosaur (theropod)", "恐龙（兽脚类）"),
        len_cm: 300.0, t0: 215.0, t1: 204.0, hab: LAND, regions: &[Site(S_GHOSTRANCH), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Dino(0), col: [0.58, 0.50, 0.38], col2: [0.80, 0.66, 0.46], rarity: 2.5, shy: 2.0, speed: 0.35,
        desc: L("細長い体で、群れで暮らしたと考えられる身軽な肉食恐竜。", "A slender, lightweight meat-eating dinosaur that may have lived in groups.", "身体修长、可能群居生活的轻巧肉食恐龙。"),
        fact: L("ゴーストランチでは何百もの化石がまとめて見つかった。1998年には化石が宇宙へ運ばれた。", "Hundreds of skeletons were found together at Ghost Ranch; a skull even flew into space in 1998.", "在幽灵牧场一次发现了数百具化石；1998年还有一块头骨被带上了太空。"),
    },
    Species {
        slug: "plateosaurus",
        name: L("プラテオサウルス", "Plateosaurus", "板龙"),
        sci: "Plateosaurus trossingensis",
        group: L("恐竜（竜脚形類）", "Dinosaur (sauropodomorph)", "恐龙（蜥脚形类）"),
        len_cm: 800.0, t0: 214.0, t1: 204.0, hab: LAND, regions: &[Site(S_TROSSINGEN), Plate(BALTICA)],
        beh: Beh::Crawl, art: Art::Dino(4), col: [0.54, 0.50, 0.38], col2: [0.72, 0.66, 0.48], rarity: 1.8, shy: 1.5, speed: 0.15,
        desc: L("長い首で高い木の葉を食べた、初期の大型の植物食恐竜。二本足で歩いた。", "An early large plant-eating dinosaur that browsed high leaves with its long neck, walking on two legs.", "用长颈吃高处树叶的早期大型植食恐龙，以两足行走。"),
        fact: L("のちの巨大な竜脚類（ディプロドクスなど）の親戚。", "A relative of the later giant sauropods such as Diplodocus.", "是后来巨型蜥脚类（如梁龙）的亲戚。"),
    },
    Species {
        slug: "postosuchus",
        name: L("ポストスクス", "Postosuchus", "后鳄"),
        sci: "Postosuchus kirkpatricki",
        group: L("偽鰐類（ワニの仲間）", "Pseudosuchian (crocodile-line)", "伪鳄类（鳄类亲戚）"),
        len_cm: 450.0, t0: 222.0, t1: 205.0, hab: LAND, regions: &[Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Archosaur(0), col: [0.50, 0.42, 0.34], col2: [0.70, 0.60, 0.46], rarity: 1.2, shy: 2.5, speed: 0.25,
        desc: L("大きな頭とするどい歯を持つ、三畳紀の陸の王者。ワニの遠い親戚。", "A big-headed, sharp-toothed top predator of Triassic land and a distant crocodile relative.", "头大齿利，是三叠纪陆地霸主，鳄鱼的远亲。"),
        fact: L("恐竜ではないが、恐竜より強い存在だった。", "It was not a dinosaur, but it out-ranked the dinosaurs of its day.", "它不是恐龙，却比当时的恐龙更强大。"),
    },
    Species {
        slug: "desmatosuchus",
        name: L("デスマトスクス", "Desmatosuchus", "链鳄"),
        sci: "Desmatosuchus spurensis",
        group: L("偽鰐類（アエトサウルス類）", "Aetosaur", "坚蜥类"),
        len_cm: 450.0, t0: 222.0, t1: 205.0, hab: LAND, regions: &[Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Archosaur(1), col: [0.54, 0.46, 0.36], col2: [0.72, 0.62, 0.46], rarity: 1.5, shy: 1.2, speed: 0.08,
        desc: L("骨の板とトゲでおおわれた、植物食のワニの仲間。肩から長いトゲがつき出ていた。", "A plant-eating crocodile relative covered in bony plates, with long spikes jutting from its shoulders.", "全身覆盖骨板和尖刺的植食性鳄类亲戚，肩部伸出长刺。"),
        fact: L("ブタのような鼻先で、地面を掘って根を食べたらしい。", "It probably rooted for food with a pig-like snout.", "它可能用猪一样的吻部拱土找根吃。"),
    },
    Species {
        slug: "eudimorphodon",
        name: L("エウディモルフォドン", "Eudimorphodon", "真双型齿翼龙"),
        sci: "Eudimorphodon ranzii",
        group: L("翼竜", "Pterosaur", "翼龙"),
        len_cm: 50.0, t0: 215.0, t1: 208.0, hab: LAND | TIDAL, regions: &[Plate(BALTICA)],
        beh: Beh::Swim, art: Art::Pterosaur(0), col: [0.46, 0.40, 0.34], col2: [0.70, 0.60, 0.50], rarity: 2.0, shy: 2.0, speed: 0.7,
        desc: L("翼を広げると1mほどの、最も古い翼竜の一つ。長い尾を持ち、魚を捕らえた。", "One of the oldest pterosaurs, with a 1 m wingspan and a long tail, catching fish.", "翼展约1米、是最古老的翼龙之一，长着长尾巴，以鱼为食。"),
        fact: L("翼竜は、背骨のある動物で初めて空を羽ばたいて飛んだ。", "Pterosaurs were the first animals with backbones to fly by flapping.", "翼龙是第一批能振翅飞行的脊椎动物。"),
    },
    Species {
        slug: "shonisaurus",
        name: L("ショニサウルス", "Shonisaurus", "肖尼鱼龙"),
        sci: "Shonisaurus popularis",
        group: L("魚竜", "Ichthyosaur", "鱼龙"),
        len_cm: 1500.0, t0: 217.0, t1: 210.0, hab: SHELF | DEEP, regions: &[Plate(LAURENTIA)],
        beh: Beh::Swim, art: Art::Marine(0), col: [0.40, 0.44, 0.48], col2: [0.74, 0.76, 0.76], rarity: 0.6, shy: 3.0, speed: 0.5,
        desc: L("長さ15mにもなる巨大な魚竜。歯の少ない長い口で、イカなどを吸いこんだらしい。", "A giant ichthyosaur up to 15 m long that may have sucked in squid with its long, nearly toothless snout.", "体长可达15米的巨型鱼龙，可能用几乎无齿的长嘴吸食鱿鱼。"),
        fact: L("アメリカ・ネバダ州の「州の化石」。", "It is the state fossil of Nevada, USA.", "是美国内华达州的“州化石”。"),
    },
    Species {
        slug: "metoposaurus",
        name: L("メトポサウルス", "Metoposaurus", "后蝾螈"),
        sci: "Metoposaurus",
        group: L("両生類（分椎類）", "Amphibian (temnospondyl)", "两栖类（离片椎类）"),
        len_cm: 200.0, t0: 230.0, t1: 210.0, hab: TIDAL | LAND, regions: &[Plate(LAURENTIA), Plate(BALTICA)],
        beh: Beh::Crawl, art: Art::Tetrapod(5), col: [0.44, 0.44, 0.36], col2: [0.62, 0.62, 0.48], rarity: 2.0, shy: 1.5, speed: 0.06,
        desc: L("平たく大きな頭を持つ両生類。浅い湖の底で、口を開けて魚を待ちぶせた。", "A big flat-headed amphibian that lay in wait for fish on the beds of shallow lakes.", "头部扁平宽大的两栖动物，在浅湖底张嘴等待鱼儿。"),
        fact: L("干上がった池で、何十匹もまとめて化石になっていた。", "Dozens were fossilised together in a dried-up pond.", "曾在一个干涸的池塘里发现了几十只一起形成的化石。"),
    },
    Species {
        slug: "dicroidium",
        name: L("ディクロイディウム", "Dicroidium", "二叉羊齿"),
        sci: "Dicroidium",
        group: L("種子シダ類（木）", "Seed-fern tree", "种子蕨类（树）"),
        len_cm: 1000.0, t0: 245.0, t1: 201.5, hab: LAND | TIDAL, regions: &[Plate(GONDWANA)],
        beh: Beh::Sessile, art: Art::Plant(7), col: [0.40, 0.32, 0.24], col2: [0.32, 0.50, 0.26], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L("Y字に分かれた葉を持つ、ゴンドワナの三畳紀の森の主役の木。", "A tree with forked fronds that dominated the Triassic forests of Gondwana.", "叶片呈Y形分叉，是冈瓦纳三叠纪森林的主角。"),
        fact: L("グロッソプテリスが大絶滅で消えたあと、その場所を受けついだ。", "It took Glossopteris's place after the great extinction.", "舌羊齿在大灭绝中消失后，它接替了那个位置。"),
    },
    Species {
        slug: "araucarioxylon",
        name: L("アラウカリオキシロン（化石の森の木）", "Araucarioxylon (petrified forest tree)", "南洋杉型木"),
        sci: "Araucarioxylon arizonicum",
        group: L("針葉樹", "Conifer", "针叶树"),
        len_cm: 3000.0, t0: 225.0, t1: 207.0, hab: LAND, regions: &[Plate(LAURENTIA)],
        beh: Beh::Sessile, art: Art::Plant(8), col: [0.42, 0.32, 0.26], col2: [0.24, 0.40, 0.22], rarity: 2.5, shy: 0.0, speed: 0.0,
        desc: L("高さ30mになる針葉樹。川に流されて埋もれ、虹色の石の木（珪化木）になった。", "A conifer up to 30 m tall whose trunks, buried by rivers, turned into rainbow-coloured stone.", "高达30米的针叶树，被河流冲埋后变成了彩虹色的硅化木。"),
        fact: L("アメリカ・アリゾナ州の「化石の森国立公園」で有名。", "It is famous from Petrified Forest National Park in Arizona.", "以美国亚利桑那州的“石化林国家公园”闻名。"),
    },
    // ======================================================== ジュラ紀
    Species {
        slug: "megazostrodon",
        name: L("メガゾストロドン", "Megazostrodon", "巨带齿兽"),
        sci: "Megazostrodon rudnerae",
        group: L("哺乳形類（初期の哺乳類）", "Early mammal", "哺乳形类（早期哺乳动物）"),
        len_cm: 10.0, t0: 201.0, t1: 195.0, hab: LAND, regions: &[Plate(GONDWANA)],
        beh: Beh::Crawl, art: Art::Mammal(0), col: [0.50, 0.42, 0.34], col2: [0.70, 0.62, 0.50], rarity: 2.5, shy: 1.0, speed: 0.15,
        desc: L("トガリネズミのような小さな動物。夜に活動して、虫を食べた。", "A tiny shrew-like animal that came out at night to eat insects.", "形似鼩鼱的小动物，夜间活动，以昆虫为食。"),
        fact: L("恐竜の時代の哺乳類の多くは、こうした夜行性の小さな動物だった。", "Most mammals in the age of dinosaurs were small night-time animals like this.", "恐龙时代的哺乳动物大多是这样的夜行性小动物。"),
    },
    Species {
        slug: "dilophosaurus",
        name: L("ディロフォサウルス", "Dilophosaurus", "双脊龙"),
        sci: "Dilophosaurus wetherilli",
        group: L("恐竜（肉食恐竜）", "Dinosaur (theropod)", "恐龙（兽脚类）"),
        len_cm: 700.0, t0: 196.0, t1: 183.0, hab: LAND, regions: &[Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Dino(1), col: [0.56, 0.46, 0.34], col2: [0.84, 0.56, 0.36], rarity: 1.2, shy: 2.5, speed: 0.35,
        desc: L("頭の上に2枚の薄いとさかを持つ、ジュラ紀前期の大型の肉食恐竜。", "A large Early Jurassic predator with two thin crests on its head.", "头顶长着两片薄冠的早侏罗世大型肉食恐龙。"),
        fact: L("映画のように毒を吐いたという証拠はない。", "There is no evidence it spat venom, as in the movies.", "没有证据表明它像电影里那样会喷毒。"),
    },
    Species {
        slug: "ichthyosaurus",
        name: L("イクチオサウルス", "Ichthyosaurus", "鱼龙"),
        sci: "Ichthyosaurus communis",
        group: L("魚竜", "Ichthyosaur", "鱼龙"),
        len_cm: 200.0, t0: 200.0, t1: 190.0, hab: SHELF | SHALLOW, regions: &[Site(S_LYME), Plate(AVALONIA), Plate(BALTICA)],
        beh: Beh::Swim, art: Art::Marine(0), col: [0.40, 0.44, 0.48], col2: [0.78, 0.80, 0.80], rarity: 2.0, shy: 2.0, speed: 0.6,
        desc: L("イルカのような体で速く泳いだ魚竜。大きな眼で、暗い海でも獲物を見つけた。", "A fast, dolphin-shaped ichthyosaur whose large eyes found prey even in dark water.", "身体如海豚、游速很快的鱼龙，大眼睛在昏暗的海中也能发现猎物。"),
        fact: L("19世紀に少女メアリー・アニングが化石を掘り出したことで有名。", "Fossils dug up by the young Mary Anning in the 1800s made it famous.", "因19世纪少女玛丽·安宁发掘其化石而闻名。"),
    },
    Species {
        slug: "plesiosaurus",
        name: L("プレシオサウルス", "Plesiosaurus", "蛇颈龙"),
        sci: "Plesiosaurus dolichodeirus",
        group: L("首長竜", "Plesiosaur", "蛇颈龙"),
        len_cm: 350.0, t0: 200.0, t1: 190.0, hab: SHELF | SHALLOW, regions: &[Site(S_LYME), Plate(AVALONIA), Plate(BALTICA)],
        beh: Beh::Swim, art: Art::Marine(1), col: [0.42, 0.42, 0.40], col2: [0.72, 0.70, 0.64], rarity: 1.5, shy: 2.0, speed: 0.4,
        desc: L("長い首と4枚のひれ足を持つ首長竜。ひれを鳥の翼のように動かして泳いだ。", "A long-necked plesiosaur that 'flew' through the water with four flippers like wings.", "长颈、四鳍的蛇颈龙，像鸟扇翅膀一样划动鳍游泳。"),
        fact: L("これもメアリー・アニングが見つけた。「首長竜」という名前のもとになった。", "Also found by Mary Anning, it gave its name to the whole plesiosaur group.", "同样由玛丽·安宁发现，整个蛇颈龙类都以它命名。"),
    },
    Species {
        slug: "dimorphodon",
        name: L("ディモルフォドン", "Dimorphodon", "双型齿翼龙"),
        sci: "Dimorphodon macronyx",
        group: L("翼竜", "Pterosaur", "翼龙"),
        len_cm: 100.0, t0: 200.0, t1: 190.0, hab: LAND | TIDAL, regions: &[Site(S_LYME), Plate(AVALONIA)],
        beh: Beh::Swim, art: Art::Pterosaur(0), col: [0.44, 0.38, 0.32], col2: [0.70, 0.58, 0.46], rarity: 1.8, shy: 2.0, speed: 0.6,
        desc: L("大きな頭と長い尾を持つ翼竜。翼を広げると1.4mほどあった。", "A big-headed, long-tailed pterosaur with a wingspan of about 1.4 m.", "头大尾长的翼龙，翼展约1.4米。"),
        fact: L("木や崖をよじ登るのも得意だったらしい。", "It was probably a good climber of trees and cliffs.", "它可能也很擅长攀爬树木和悬崖。"),
    },
    Species {
        slug: "scelidosaurus",
        name: L("スケリドサウルス", "Scelidosaurus", "肢龙"),
        sci: "Scelidosaurus harrisonii",
        group: L("恐竜（初期の装盾類）", "Dinosaur (early thyreophoran)", "恐龙（早期装甲类）"),
        len_cm: 400.0, t0: 196.0, t1: 190.0, hab: LAND, regions: &[Plate(AVALONIA)],
        beh: Beh::Crawl, art: Art::Dino(6), col: [0.48, 0.44, 0.34], col2: [0.68, 0.62, 0.48], rarity: 1.5, shy: 1.5, speed: 0.08,
        desc: L("背中に骨の板が並んだ、植物食の恐竜。のちの剣竜や鎧竜の祖先に近い。", "A plant-eating dinosaur with rows of bony plates, close to the ancestors of stegosaurs and ankylosaurs.", "背部排列骨板的植食恐龙，接近后来剑龙和甲龙的祖先。"),
        fact: L("全身がわかる恐竜として、最初に見つかったものの一つ。", "One of the first dinosaurs known from a nearly complete skeleton.", "是最早发现的近乎完整骨骼的恐龙之一。"),
    },
    Species {
        slug: "stenopterygius",
        name: L("ステノプテリギウス", "Stenopterygius", "狭翼鱼龙"),
        sci: "Stenopterygius quadriscissus",
        group: L("魚竜", "Ichthyosaur", "鱼龙"),
        len_cm: 300.0, t0: 183.0, t1: 178.0, hab: SHELF, regions: &[Site(S_HOLZMADEN), Plate(BALTICA)],
        beh: Beh::Swim, art: Art::Marine(0), col: [0.36, 0.40, 0.44], col2: [0.76, 0.78, 0.78], rarity: 2.0, shy: 2.0, speed: 0.6,
        desc: L("背びれと三日月形の尾びれを持つ、イルカそっくりの魚竜。", "A dolphin-like ichthyosaur with a dorsal fin and crescent-shaped tail.", "长着背鳍和新月形尾鳍、酷似海豚的鱼龙。"),
        fact: L("お腹に赤ちゃんがいる化石や、出産中の化石が見つかっている。", "Fossils show mothers carrying, and even giving birth to, babies.", "发现过腹中有幼崽、甚至正在分娩的化石。"),
    },
    Species {
        slug: "dapedium",
        name: L("ダペディウム", "Dapedium", "鳞齿鱼"),
        sci: "Dapedium",
        group: L("条鰭類（硬骨魚）", "Ray-finned fish", "辐鳍鱼类"),
        len_cm: 35.0, t0: 200.0, t1: 180.0, hab: SHALLOW | SHELF, regions: &[Global],
        beh: Beh::Swim, art: Art::RayFin, col: [0.40, 0.38, 0.34], col2: [0.78, 0.70, 0.50], rarity: 3.0, shy: 1.2, speed: 0.4,
        desc: L("ひし形の厚いうろこを持つ、円盤のような形の魚。かたい殻の生きものをかじった。", "A disc-shaped fish with thick diamond scales that nibbled hard-shelled animals.", "身体呈圆盘状、长着厚菱形鳞片的鱼，啃食硬壳生物。"),
        fact: L("ライム・リージスやホルツマーデンで、きれいな全身の化石が見つかる。", "Beautiful complete fossils come from Lyme Regis and Holzmaden.", "在莱姆里吉斯和霍尔茨马登都发现了精美的完整化石。"),
    },
    Species {
        slug: "passaloteuthis",
        name: L("パッサロテウティス（ベレムナイト）", "Passaloteuthis (belemnite)", "矛箭石"),
        sci: "Passaloteuthis",
        group: L("頭足類（ベレムナイト）", "Cephalopod (belemnite)", "头足类（箭石）"),
        len_cm: 30.0, t0: 190.0, t1: 175.0, hab: SHELF | SHALLOW, regions: &[Global],
        beh: Beh::Swim, art: Art::Cephalopod(0), col: [0.70, 0.60, 0.52], col2: [0.52, 0.36, 0.30], rarity: 3.0, shy: 1.5, speed: 0.4,
        desc: L("イカに似た体の中に、弾丸のような硬い骨（鞘）を持つ頭足類。群れで泳いだ。", "A squid-like cephalopod with a hard bullet-shaped guard inside its body, swimming in schools.", "身体似鱿鱼、体内有子弹状硬鞘的头足类，成群游动。"),
        fact: L("墨袋まで化石に残り、その墨で絵がかかれたこともある。", "Its ink sac can fossilise — and fossil ink has been used to draw pictures.", "连墨囊都能保存成化石，有人用这种化石墨汁画过画。"),
    },
    Species {
        slug: "dactylioceras",
        name: L("ダクチリオセラス", "Dactylioceras", "指菊石"),
        sci: "Dactylioceras commune",
        group: L("頭足類（アンモナイト）", "Cephalopod (ammonite)", "头足类（菊石）"),
        len_cm: 8.0, t0: 183.0, t1: 178.0, hab: SHELF | SHALLOW, regions: &[Global],
        beh: Beh::Swim, art: Art::Cephalopod(1), col: [0.70, 0.60, 0.48], col2: [0.48, 0.36, 0.28], rarity: 3.5, shy: 1.2, speed: 0.2,
        desc: L("細かいすじが並ぶ、平たく巻いた殻のアンモナイト。ジュラ紀の海にたくさんいた。", "A flat-coiled ammonite with fine ribs, abundant in Jurassic seas.", "外壳扁平盘卷、布满细肋的菊石，在侏罗纪海洋中数量众多。"),
        fact: L("イギリスでは昔「ヘビ石」と呼ばれ、ヘビの頭が彫りこまれた。", "In England it was once called a 'snakestone' and carved with snake heads.", "在英国曾被称为“蛇石”，人们还在上面刻上蛇头。"),
    },
    Species {
        slug: "liopleurodon",
        name: L("リオプレウロドン", "Liopleurodon", "滑齿龙"),
        sci: "Liopleurodon ferox",
        group: L("首長竜（首の短いプリオサウルス類）", "Pliosaur", "上龙"),
        len_cm: 650.0, t0: 166.0, t1: 155.0, hab: SHELF | DEEP, regions: &[Plate(BALTICA), Plate(AVALONIA)],
        beh: Beh::Swim, art: Art::Marine(3), col: [0.38, 0.40, 0.42], col2: [0.70, 0.70, 0.66], rarity: 0.8, shy: 3.0, speed: 0.5,
        desc: L("大きな頭と強いあごを持つ、首の短い首長竜。ジュラ紀の海の大型の捕食者。", "A short-necked plesiosaur with a huge head and powerful jaws — a top predator of Jurassic seas.", "头大颌强的短颈蛇颈龙，是侏罗纪海洋的大型捕食者。"),
        fact: L("においをかぎ分けて獲物を探したと考えられている。", "It may have tracked prey by smell underwater.", "据推测它能在水中嗅出猎物的气味。"),
    },
    Species {
        slug: "leedsichthys",
        name: L("リードシクティス", "Leedsichthys", "利兹鱼"),
        sci: "Leedsichthys problematicus",
        group: L("条鰭類（硬骨魚）", "Ray-finned fish", "辐鳍鱼类"),
        len_cm: 1600.0, t0: 166.0, t1: 155.0, hab: SHELF, regions: &[Plate(BALTICA), Plate(AVALONIA)],
        beh: Beh::Swim, art: Art::RayFin, col: [0.42, 0.44, 0.46], col2: [0.70, 0.72, 0.70], rarity: 0.6, shy: 2.0, speed: 0.3,
        desc: L("長さ16mにもなる、史上最大級の硬骨魚。大きな口でプランクトンをこして食べた。", "One of the largest bony fishes ever, up to 16 m, filtering plankton with its huge mouth.", "体长可达16米，是史上最大的硬骨鱼之一，用大嘴滤食浮游生物。"),
        fact: L("骨がもろく、全身の化石はまだ見つかっていない。", "Its bones were fragile, and no complete skeleton has been found.", "骨骼脆弱，至今未发现完整骨架。"),
    },
    Species {
        slug: "anchiornis",
        name: L("アンキオルニス", "Anchiornis", "近鸟龙"),
        sci: "Anchiornis huxleyi",
        group: L("恐竜（羽毛のある小型の肉食恐竜）", "Feathered dinosaur", "恐龙（长羽毛的小型兽脚类）"),
        len_cm: 34.0, t0: 161.0, t1: 158.0, hab: LAND, regions: &[Site(S_YANLIAO), Plate(SIBERIA)],
        beh: Beh::Crawl, art: Art::Dino(11), col: [0.30, 0.30, 0.32], col2: [0.90, 0.88, 0.86], rarity: 2.0, shy: 1.8, speed: 0.3,
        desc: L("手にも足にも羽を持つ、ハトほどの大きさの恐竜。白黒の羽に、頭には赤いとさかがあった。", "A pigeon-sized dinosaur with feathers on its arms and legs, black-and-white plumage and a reddish crest.", "鸽子大小、手脚都长羽毛的恐龙，羽毛黑白相间，头顶有红色羽冠。"),
        fact: L("化石に残った色素から、羽の色がわかった最初の恐竜の一つ。", "One of the first dinosaurs whose feather colours were worked out from fossil pigments.", "是最早通过化石色素复原羽毛颜色的恐龙之一。"),
    },
    Species {
        slug: "castorocauda",
        name: L("カストロカウダ", "Castorocauda", "獭形狸尾兽"),
        sci: "Castorocauda lutrasimilis",
        group: L("哺乳形類（泳ぐ哺乳類）", "Swimming mammaliaform", "哺乳形类（会游泳的哺乳动物）"),
        len_cm: 45.0, t0: 165.0, t1: 160.0, hab: TIDAL | LAND, regions: &[Site(S_YANLIAO)],
        beh: Beh::Crawl, art: Art::Mammal(1), col: [0.40, 0.32, 0.26], col2: [0.62, 0.52, 0.40], rarity: 2.0, shy: 1.2, speed: 0.15,
        desc: L("ビーバーのような平たい尾と、カワウソのような体で泳いだ、恐竜時代の哺乳類。", "A dinosaur-age mammal relative that swam with a beaver-like tail and an otter-like body.", "长着河狸般扁尾和水獭般身体的恐龙时代哺乳类，善于游泳。"),
        fact: L("恐竜の時代の哺乳類も、いろいろな暮らし方をしていたことを示した。", "It showed that mammals in the dinosaur age lived in many different ways.", "它证明了恐龙时代的哺乳类也有多种多样的生活方式。"),
    },
    Species {
        slug: "allosaurus",
        name: L("アロサウルス", "Allosaurus", "异特龙"),
        sci: "Allosaurus fragilis",
        group: L("恐竜（肉食恐竜）", "Dinosaur (theropod)", "恐龙（兽脚类）"),
        len_cm: 850.0, t0: 155.0, t1: 145.0, hab: LAND, regions: &[Site(S_MORRISON), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Dino(1), col: [0.54, 0.44, 0.34], col2: [0.80, 0.62, 0.42], rarity: 1.2, shy: 2.5, speed: 0.35,
        desc: L("ジュラ紀後期の北アメリカで最も強い肉食恐竜。目の上に小さな角があった。", "The top meat-eater of Late Jurassic North America, with small horns above its eyes.", "晚侏罗世北美洲最强的肉食恐龙，眼睛上方有小角。"),
        fact: L("ステゴサウルスの尾のトゲが刺さった跡のある骨が見つかっている。", "A bone has been found pierced by a Stegosaurus tail spike.", "曾发现被剑龙尾刺刺穿痕迹的骨头。"),
    },
    Species {
        slug: "stegosaurus",
        name: L("ステゴサウルス", "Stegosaurus", "剑龙"),
        sci: "Stegosaurus stenops",
        group: L("恐竜（剣竜）", "Dinosaur (stegosaur)", "恐龙（剑龙类）"),
        len_cm: 900.0, t0: 155.0, t1: 148.0, hab: LAND, regions: &[Site(S_MORRISON), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Dino(5), col: [0.48, 0.48, 0.36], col2: [0.80, 0.56, 0.36], rarity: 1.5, shy: 1.5, speed: 0.08,
        desc: L("背中に大きな骨の板が2列に並び、尾の先に4本のトゲを持つ植物食恐竜。", "A plant-eater with two rows of big bony plates on its back and four spikes on its tail.", "背上有两排大骨板、尾端有四根尖刺的植食恐龙。"),
        fact: L("体の大きさのわりに、脳はクルミほどしかなかった。", "For its size, its brain was only about as big as a walnut.", "相对于体型，它的脑只有核桃那么大。"),
    },
    Species {
        slug: "diplodocus",
        name: L("ディプロドクス", "Diplodocus", "梁龙"),
        sci: "Diplodocus carnegii",
        group: L("恐竜（竜脚類）", "Dinosaur (sauropod)", "恐龙（蜥脚类）"),
        len_cm: 2600.0, t0: 154.0, t1: 150.0, hab: LAND, regions: &[Site(S_MORRISON), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Dino(3), col: [0.52, 0.50, 0.42], col2: [0.70, 0.66, 0.54], rarity: 1.2, shy: 1.0, speed: 0.06,
        desc: L("長さ26m。長い首と、むちのような長い尾を持つ巨大な植物食恐竜。", "A 26 m giant plant-eater with a long neck and a whip-like tail.", "体长26米，有长颈和鞭子般长尾的巨型植食恐龙。"),
        fact: L("骨格の模型は世界中の博物館に送られ、「ディッピー」の愛称で親しまれた。", "Casts of its skeleton were sent to museums worldwide and nicknamed 'Dippy'.", "其骨架模型被送往世界各地的博物馆，昵称“迪皮”。"),
    },
    Species {
        slug: "brachiosaurus",
        name: L("ブラキオサウルス", "Brachiosaurus", "腕龙"),
        sci: "Brachiosaurus altithorax",
        group: L("恐竜（竜脚類）", "Dinosaur (sauropod)", "恐龙（蜥脚类）"),
        len_cm: 2200.0, t0: 154.0, t1: 150.0, hab: LAND, regions: &[Site(S_MORRISON), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Dino(13), col: [0.54, 0.52, 0.44], col2: [0.72, 0.68, 0.56], rarity: 1.0, shy: 1.0, speed: 0.06,
        desc: L("前足が長く、首を高く上げて、高い木の葉を食べた巨大な恐竜。", "A giant with long front legs that held its neck high to eat leaves from tall trees.", "前肢较长、高高抬起脖子吃高处树叶的巨型恐龙。"),
        fact: L("頭の高さは12mほどで、4階建ての建物くらい。", "Its head reached about 12 m high — as tall as a four-storey building.", "头部高约12米，相当于四层楼高。"),
    },
    Species {
        slug: "archaeopteryx",
        name: L("始祖鳥（アーケオプテリクス）", "Archaeopteryx", "始祖鸟"),
        sci: "Archaeopteryx lithographica",
        group: L("鳥類（最古の鳥の一つ）", "Early bird", "鸟类（最古老的鸟之一）"),
        len_cm: 50.0, t0: 151.0, t1: 149.0, hab: LAND | TIDAL, regions: &[Site(S_SOLNHOFEN)],
        beh: Beh::Swim, art: Art::Bird(0), col: [0.32, 0.30, 0.28], col2: [0.20, 0.20, 0.22], rarity: 1.5, shy: 2.0, speed: 0.6,
        desc: L("翼と羽を持ちながら、歯と長い骨の尾、指の爪もあった、恐竜と鳥の両方の特徴を持つ動物。", "An animal with wings and feathers but also teeth, a long bony tail and clawed fingers — half dinosaur, half bird.", "既有翅膀和羽毛，又有牙齿、长骨质尾和带爪手指，兼具恐龙和鸟类特征。"),
        fact: L("ダーウィンの『種の起源』が出た2年後に見つかり、進化の証拠として大きな話題になった。", "Found two years after Darwin's 'On the Origin of Species', it became famous evidence for evolution.", "在达尔文《物种起源》出版两年后被发现，成为轰动一时的演化证据。"),
    },
    Species {
        slug: "rhamphorhynchus",
        name: L("ランフォリンクス", "Rhamphorhynchus", "喙嘴翼龙"),
        sci: "Rhamphorhynchus muensteri",
        group: L("翼竜", "Pterosaur", "翼龙"),
        len_cm: 60.0, t0: 151.0, t1: 148.0, hab: TIDAL | LAND, regions: &[Site(S_SOLNHOFEN), Plate(BALTICA)],
        beh: Beh::Swim, art: Art::Pterosaur(0), col: [0.46, 0.40, 0.32], col2: [0.74, 0.62, 0.48], rarity: 2.0, shy: 2.0, speed: 0.7,
        desc: L("長い尾の先にひし形の帆を持つ翼竜。前向きの歯で魚をつかまえた。", "A long-tailed pterosaur with a diamond-shaped vane on its tail tip and forward-pointing teeth for catching fish.", "长尾末端有菱形尾翼的翼龙，用前伸的牙齿捕鱼。"),
        fact: L("魚を食べている最中に大きな魚に襲われた化石が残っている。", "One fossil shows it being grabbed by a large fish while eating a fish itself.", "有化石记录了它在吃鱼时被大鱼袭击的瞬间。"),
    },
    Species {
        slug: "compsognathus",
        name: L("コンプソグナトゥス", "Compsognathus", "美颌龙"),
        sci: "Compsognathus longipes",
        group: L("恐竜（小型の肉食恐竜）", "Dinosaur (small theropod)", "恐龙（小型兽脚类）"),
        len_cm: 100.0, t0: 151.0, t1: 149.0, hab: LAND | TIDAL, regions: &[Site(S_SOLNHOFEN)],
        beh: Beh::Crawl, art: Art::Dino(0), col: [0.52, 0.46, 0.34], col2: [0.74, 0.64, 0.46], rarity: 2.0, shy: 1.8, speed: 0.4,
        desc: L("ニワトリほどの大きさの、身軽な小型の肉食恐竜。トカゲを食べていた。", "A nimble chicken-sized meat-eater; one fossil has a lizard in its stomach.", "鸡一般大小的灵巧小型肉食恐龙，以蜥蜴为食。"),
        fact: L("お腹の中に、丸のみにしたトカゲの化石が残っていた。", "A swallowed lizard was found fossilised in its belly.", "其腹中保存着被整个吞下的蜥蜴化石。"),
    },
    Species {
        slug: "williamsonia",
        name: L("ウィリアムソニア（ベネチテス類）", "Williamsonia (bennettitalean)", "威廉姆逊苏铁"),
        sci: "Williamsonia",
        group: L("ベネチテス類（ソテツに似た植物）", "Bennettitalean (cycad-like)", "本内苏铁类（似苏铁植物）"),
        len_cm: 200.0, t0: 200.0, t1: 145.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(12), col: [0.46, 0.38, 0.28], col2: [0.36, 0.52, 0.26], rarity: 3.5, shy: 0.0, speed: 0.0,
        desc: L("ソテツに似た葉を持ち、花のような生殖器をつけた植物。恐竜時代の大地に多かった。", "A cycad-like plant with flower-like reproductive organs, common across dinosaur-age landscapes.", "叶似苏铁、长着花状生殖器官的植物，在恐龙时代的大地上很常见。"),
        fact: L("花を持つ植物ではないが、花に似たつくりを別に生み出した。", "Not a flowering plant, but it evolved flower-like structures on its own.", "它不是开花植物，却独立演化出了类似花的结构。"),
    },
    Species {
        slug: "ginkgo",
        name: L("イチョウの仲間", "Ginkgo", "银杏类"),
        sci: "Ginkgo",
        group: L("イチョウ類", "Ginkgophyte", "银杏类"),
        len_cm: 2000.0, t0: 200.0, t1: 66.0, hab: LAND, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(13), col: [0.40, 0.34, 0.26], col2: [0.50, 0.66, 0.30], rarity: 2.5, shy: 0.0, speed: 0.0,
        desc: L("扇形の葉を持つ木。ジュラ紀には世界中に広がり、たくさんの種類があった。", "A tree with fan-shaped leaves that spread worldwide in the Jurassic in many kinds.", "长着扇形叶片的树，侏罗纪时遍布全球，种类繁多。"),
        fact: L("いまはイチョウ1種だけが生き残る「生きている化石」。", "Today only one species, Ginkgo biloba, survives — a 'living fossil'.", "如今只剩银杏一种，是“活化石”。"),
    },
    // ======================================================== 白亜紀
    Species {
        slug: "iguanodon",
        name: L("イグアノドン", "Iguanodon", "禽龙"),
        sci: "Iguanodon bernissartensis",
        group: L("恐竜（鳥脚類）", "Dinosaur (ornithopod)", "恐龙（鸟脚类）"),
        len_cm: 1000.0, t0: 130.0, t1: 122.0, hab: LAND, regions: &[Site(S_WEALDEN), Plate(AVALONIA), Plate(BALTICA)],
        beh: Beh::Crawl, art: Art::Dino(9), col: [0.48, 0.50, 0.38], col2: [0.68, 0.70, 0.52], rarity: 1.8, shy: 1.5, speed: 0.1,
        desc: L("親指が大きなトゲになった、群れで暮らした植物食恐竜。", "A herding plant-eater whose thumbs were big spikes.", "拇指变成大尖刺、成群生活的植食恐龙。"),
        fact: L("世界で2番目に名前がついた恐竜。最初は親指のトゲが鼻の角だと思われた。", "The second dinosaur ever named; its thumb spike was first thought to be a nose horn.", "世界上第二个被命名的恐龙，拇指尖刺起初被误认为鼻角。"),
    },
    Species {
        slug: "sinosauropteryx",
        name: L("シノサウロプテリクス", "Sinosauropteryx", "中华龙鸟"),
        sci: "Sinosauropteryx prima",
        group: L("恐竜（羽毛のある小型の肉食恐竜）", "Feathered dinosaur", "恐龙（长羽毛的小型兽脚类）"),
        len_cm: 100.0, t0: 125.0, t1: 122.0, hab: LAND, regions: &[Site(S_JEHOL)],
        beh: Beh::Crawl, art: Art::Dino(11), col: [0.66, 0.46, 0.30], col2: [0.92, 0.84, 0.70], rarity: 2.0, shy: 1.8, speed: 0.35,
        desc: L("ふわふわの羽毛におおわれた小型の肉食恐竜。尾にはしま模様があった。", "A small meat-eater covered in fluffy feathers, with a striped tail.", "全身覆盖蓬松羽毛的小型肉食恐龙，尾巴上有条纹。"),
        fact: L("羽毛のある恐竜として、最初に見つかったもの（1996年）。", "In 1996 it became the first feathered dinosaur ever found.", "1996年发现，是第一种被发现的长羽毛恐龙。"),
    },
    Species {
        slug: "microraptor",
        name: L("ミクロラプトル", "Microraptor", "小盗龙"),
        sci: "Microraptor gui",
        group: L("恐竜（4枚の翼を持つ小型恐竜）", "Four-winged dinosaur", "恐龙（四翼小型恐龙）"),
        len_cm: 77.0, t0: 122.0, t1: 120.0, hab: LAND, regions: &[Site(S_JEHOL)],
        beh: Beh::Swim, art: Art::Bird(1), col: [0.12, 0.14, 0.20], col2: [0.24, 0.30, 0.42], rarity: 1.5, shy: 2.0, speed: 0.6,
        desc: L("手と足の両方に翼のような羽を持ち、木から木へ滑空した小型の恐竜。", "A small dinosaur with wing-like feathers on both arms and legs that glided between trees.", "手脚都长着翼状羽毛、在树间滑翔的小型恐龙。"),
        fact: L("羽は、カラスのように青みがかった黒い光沢をしていた。", "Its feathers had a glossy blue-black sheen, like a crow's.", "它的羽毛像乌鸦一样，泛着蓝黑色的光泽。"),
    },
    Species {
        slug: "psittacosaurus",
        name: L("プシッタコサウルス", "Psittacosaurus", "鹦鹉嘴龙"),
        sci: "Psittacosaurus",
        group: L("恐竜（初期の角竜）", "Dinosaur (early ceratopsian)", "恐龙（早期角龙类）"),
        len_cm: 150.0, t0: 125.0, t1: 100.0, hab: LAND, regions: &[Site(S_JEHOL), Plate(SIBERIA), Plate(NORTH_CHINA)],
        beh: Beh::Crawl, art: Art::Dino(8), col: [0.56, 0.48, 0.36], col2: [0.74, 0.66, 0.50], rarity: 2.5, shy: 1.2, speed: 0.15,
        desc: L("オウムのようなくちばしを持つ、小型の植物食恐竜。尾の上に長い毛のようなトゲがあった。", "A small plant-eater with a parrot-like beak and long bristles on its tail.", "长着鹦鹉般喙部的小型植食恐龙，尾巴上有长长的鬃毛状刺。"),
        fact: L("体の色のもようまでわかっていて、お腹が明るい「かくれ色」だった。", "Even its colour pattern is known: it was countershaded, with a pale belly.", "连体色图案都已复原：它有浅色腹部的“反荫蔽色”。"),
    },
    Species {
        slug: "confuciusornis",
        name: L("コンフキウソルニス（孔子鳥）", "Confuciusornis", "孔子鸟"),
        sci: "Confuciusornis sanctus",
        group: L("鳥類", "Early bird", "鸟类"),
        len_cm: 30.0, t0: 125.0, t1: 120.0, hab: LAND | TIDAL, regions: &[Site(S_JEHOL)],
        beh: Beh::Swim, art: Art::Bird(2), col: [0.40, 0.36, 0.32], col2: [0.62, 0.56, 0.48], rarity: 2.5, shy: 2.0, speed: 0.6,
        desc: L("歯のないくちばしと、長い2本の尾羽を持つ初期の鳥。群れで暮らした。", "An early bird with a toothless beak and two long tail streamers that lived in flocks.", "有无齿喙和两根长尾羽的早期鸟类，成群生活。"),
        fact: L("何千もの化石が見つかっていて、湖のほとりにとても多かった。", "Thousands of fossils are known; it was abundant around the lakes.", "已发现数千件化石，在湖畔数量非常多。"),
    },
    Species {
        slug: "repenomamus",
        name: L("レペノマムス", "Repenomamus", "爬兽"),
        sci: "Repenomamus giganticus",
        group: L("哺乳類", "Mammal", "哺乳动物"),
        len_cm: 100.0, t0: 125.0, t1: 123.0, hab: LAND, regions: &[Site(S_JEHOL)],
        beh: Beh::Crawl, art: Art::Mammal(2), col: [0.42, 0.34, 0.28], col2: [0.62, 0.52, 0.40], rarity: 1.5, shy: 1.5, speed: 0.15,
        desc: L("アナグマほどの大きさの、恐竜時代の大型の哺乳類。", "A badger-sized mammal, big for the age of dinosaurs.", "体型如獾的恐龙时代大型哺乳动物。"),
        fact: L("お腹の中から、小さな恐竜（プシッタコサウルスの子ども）の骨が見つかった。", "A baby Psittacosaurus was found in its stomach — mammals sometimes ate dinosaurs!", "其胃中发现了鹦鹉嘴龙幼崽的骨头——哺乳动物有时也吃恐龙！"),
    },
    Species {
        slug: "archaefructus",
        name: L("アルカエフルクトゥス（古果）", "Archaefructus", "古果"),
        sci: "Archaefructus sinensis",
        group: L("被子植物（初期の花の咲く植物）", "Early flowering plant", "被子植物（早期开花植物）"),
        len_cm: 50.0, t0: 125.0, t1: 122.0, hab: TIDAL, regions: &[Site(S_JEHOL)],
        beh: Beh::Sessile, art: Art::Plant(14), col: [0.36, 0.48, 0.30], col2: [0.42, 0.62, 0.32], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L("水辺に生えた、最も古い花の咲く植物の一つ。花びらはまだなかった。", "One of the oldest flowering plants, growing at the water's edge — it had no petals yet.", "生长在水边、最古老的开花植物之一，还没有花瓣。"),
        fact: L("名前は「古い果実」という意味。", "Its name means 'ancient fruit'.", "名字意为“古老的果实”。"),
    },
    Species {
        slug: "spinosaurus",
        name: L("スピノサウルス", "Spinosaurus", "棘龙"),
        sci: "Spinosaurus aegyptiacus",
        group: L("恐竜（水辺の大型肉食恐竜）", "Dinosaur (semi-aquatic theropod)", "恐龙（水栖大型兽脚类）"),
        len_cm: 1500.0, t0: 99.0, t1: 93.0, hab: TIDAL | LAND, regions: &[Site(S_KEMKEM), Plate(GONDWANA)],
        beh: Beh::Crawl, art: Art::Dino(2), col: [0.50, 0.42, 0.34], col2: [0.80, 0.52, 0.34], rarity: 0.8, shy: 2.5, speed: 0.3,
        desc: L("背中に大きな帆を持つ、長さ15mの肉食恐竜。ワニのような口で、川の魚を捕らえた。", "A 15 m sail-backed predator with crocodile-like jaws for catching river fish.", "背上有大帆、体长15米的肉食恐龙，用鳄鱼般的嘴捕食河鱼。"),
        fact: L("尾をひれのように使って、水の中を泳げたと考えられている。", "It may have swum by using its tail like a fin.", "据推测它能把尾巴当作鳍在水中游泳。"),
    },
    Species {
        slug: "argentinosaurus",
        name: L("アルゼンチノサウルス", "Argentinosaurus", "阿根廷龙"),
        sci: "Argentinosaurus huinculensis",
        group: L("恐竜（竜脚類）", "Dinosaur (titanosaur)", "恐龙（泰坦巨龙类）"),
        len_cm: 3500.0, t0: 96.0, t1: 92.0, hab: LAND, regions: &[Site(S_PATAGONIA), Plate(SOUTH_AMERICA)],
        beh: Beh::Crawl, art: Art::Dino(3), col: [0.54, 0.50, 0.42], col2: [0.70, 0.66, 0.54], rarity: 0.8, shy: 1.0, speed: 0.05,
        desc: L("長さ35m・重さ70トンにもなったと考えられる、史上最大級の陸の動物。", "Possibly 35 m long and 70 tonnes — one of the largest land animals ever.", "体长可能达35米、体重70吨，是史上最大的陆地动物之一。"),
        fact: L("背骨の一つだけで、人の背の高さほどもある。", "A single one of its vertebrae is as tall as a person.", "仅一块脊椎骨就有一个人那么高。"),
    },
    Species {
        slug: "giganotosaurus",
        name: L("ギガノトサウルス", "Giganotosaurus", "南方巨兽龙"),
        sci: "Giganotosaurus carolinii",
        group: L("恐竜（大型の肉食恐竜）", "Dinosaur (large theropod)", "恐龙（大型兽脚类）"),
        len_cm: 1250.0, t0: 99.0, t1: 95.0, hab: LAND, regions: &[Site(S_PATAGONIA), Plate(SOUTH_AMERICA)],
        beh: Beh::Crawl, art: Art::Dino(1), col: [0.54, 0.44, 0.34], col2: [0.76, 0.60, 0.42], rarity: 0.8, shy: 2.5, speed: 0.35,
        desc: L("ティラノサウルスに並ぶ大きさの、南アメリカの巨大な肉食恐竜。", "A giant South American meat-eater as big as Tyrannosaurus.", "体型与霸王龙相当的南美洲巨型肉食恐龙。"),
        fact: L("刃物のような薄い歯で、巨大な竜脚類の肉を切りさいた。", "Its blade-like teeth sliced flesh from giant sauropods.", "用刀刃般的薄齿切割巨型蜥脚类的肉。"),
    },
    Species {
        slug: "elasmosaurus",
        name: L("エラスモサウルス", "Elasmosaurus", "薄板龙"),
        sci: "Elasmosaurus platyurus",
        group: L("首長竜", "Plesiosaur (elasmosaur)", "蛇颈龙（薄板龙类）"),
        len_cm: 1000.0, t0: 81.0, t1: 79.0, hab: SHELF, regions: &[Site(S_NIOBRARA), Plate(LAURENTIA)],
        beh: Beh::Swim, art: Art::Marine(1), col: [0.40, 0.42, 0.42], col2: [0.72, 0.72, 0.68], rarity: 1.0, shy: 2.0, speed: 0.35,
        desc: L("体長の半分以上が首という、首のとても長い首長竜。首の骨は70個以上あった。", "A plesiosaur whose neck was over half its length, with more than 70 neck bones.", "颈部占体长一半以上的蛇颈龙，颈椎多达70多节。"),
        fact: L("発見した学者は、はじめ頭を尾の先につけて復元してしまった。", "Its first describer famously put the head on the end of the tail by mistake.", "最初研究它的学者曾错把头装在了尾巴末端。"),
    },
    Species {
        slug: "tylosaurus",
        name: L("ティロサウルス", "Tylosaurus", "海王龙"),
        sci: "Tylosaurus proriger",
        group: L("モササウルス類（海のトカゲ）", "Mosasaur", "沧龙类"),
        len_cm: 1200.0, t0: 88.0, t1: 78.0, hab: SHELF, regions: &[Site(S_NIOBRARA), Plate(LAURENTIA)],
        beh: Beh::Swim, art: Art::Marine(4), col: [0.36, 0.40, 0.42], col2: [0.70, 0.72, 0.70], rarity: 1.0, shy: 2.5, speed: 0.5,
        desc: L("長さ12mの、海のトカゲの仲間。とがった鼻先で獲物に体当たりしたらしい。", "A 12 m sea lizard that may have rammed prey with its pointed snout.", "体长12米的海生蜥蜴，可能用尖吻撞击猎物。"),
        fact: L("お腹から、首長竜や鳥、サメの化石が見つかっている。", "Plesiosaurs, birds and sharks have been found in its stomach.", "其腹中发现过蛇颈龙、鸟和鲨鱼的化石。"),
    },
    Species {
        slug: "pteranodon",
        name: L("プテラノドン", "Pteranodon", "无齿翼龙"),
        sci: "Pteranodon longiceps",
        group: L("翼竜", "Pterosaur", "翼龙"),
        len_cm: 180.0, t0: 88.0, t1: 80.0, hab: LAND | TIDAL, regions: &[Plate(LAURENTIA)],
        beh: Beh::Swim, art: Art::Pterosaur(1), col: [0.70, 0.68, 0.64], col2: [0.48, 0.42, 0.36], rarity: 1.5, shy: 2.5, speed: 0.8,
        desc: L("翼を広げると6m以上。歯のないくちばしと、後ろにのびるとさかを持つ大型の翼竜。", "A big pterosaur with a wingspan over 6 m, a toothless beak and a backward crest.", "翼展超过6米、无齿喙、头冠向后伸展的大型翼龙。"),
        fact: L("海の上をグライダーのように飛び、魚を捕らえていた。", "It soared over the sea like a glider, catching fish.", "它像滑翔机一样在海上翱翔，捕食鱼类。"),
    },
    Species {
        slug: "xiphactinus",
        name: L("シファクティヌス", "Xiphactinus", "剑射鱼"),
        sci: "Xiphactinus audax",
        group: L("条鰭類（硬骨魚）", "Ray-finned fish", "辐鳍鱼类"),
        len_cm: 500.0, t0: 88.0, t1: 75.0, hab: SHELF, regions: &[Site(S_NIOBRARA), Plate(LAURENTIA)],
        beh: Beh::Swim, art: Art::RayFin, col: [0.48, 0.52, 0.56], col2: [0.84, 0.84, 0.80], rarity: 1.2, shy: 2.0, speed: 0.6,
        desc: L("長さ5mの、ブルドッグのような顔の大きな魚。するどい牙で大きな魚を丸のみにした。", "A 5 m bulldog-faced fish with fangs that swallowed big fish whole.", "体长5米、长着斗牛犬般面孔的大鱼，用利牙整吞大鱼。"),
        fact: L("お腹に2mの魚が入ったままの化石が有名。", "A famous fossil has a 2 m fish inside its stomach.", "有一件著名化石的腹中还装着一条2米长的鱼。"),
    },
    Species {
        slug: "archelon",
        name: L("アーケロン", "Archelon", "古巨龟"),
        sci: "Archelon ischyros",
        group: L("カメ（ウミガメ）", "Sea turtle", "龟类（海龟）"),
        len_cm: 400.0, t0: 80.0, t1: 74.0, hab: SHELF | SHALLOW, regions: &[Plate(LAURENTIA)],
        beh: Beh::Swim, art: Art::Marine(5), col: [0.40, 0.42, 0.38], col2: [0.58, 0.60, 0.52], rarity: 1.2, shy: 2.0, speed: 0.3,
        desc: L("長さ4m、史上最大のウミガメ。かたい甲羅ではなく、皮でおおわれた骨組みの甲羅を持っていた。", "At 4 m long, the largest sea turtle ever, with a leathery, ribbed shell instead of solid armour.", "体长4米，是史上最大的海龟，龟壳不是坚硬的，而是覆盖皮肤的骨架。"),
        fact: L("いまのオサガメの遠い親戚。", "A distant relative of today's leatherback turtle.", "是现代棱皮龟的远亲。"),
    },
    Species {
        slug: "cretoxyrhina",
        name: L("クレトキシリナ", "Cretoxyrhina", "白垩刺甲鲨"),
        sci: "Cretoxyrhina mantelli",
        group: L("軟骨魚類（サメ）", "Shark", "软骨鱼类（鲨鱼）"),
        len_cm: 700.0, t0: 100.0, t1: 82.0, hab: SHELF | DEEP, regions: &[Global],
        beh: Beh::Swim, art: Art::Shark(0), col: [0.46, 0.50, 0.54], col2: [0.84, 0.86, 0.84], rarity: 1.0, shy: 2.5, speed: 0.6,
        desc: L("長さ7mの、ホホジロザメに似た大型のサメ。首長竜やモササウルスも襲った。", "A 7 m shark like a great white that attacked even plesiosaurs and mosasaurs.", "体长7米、形似大白鲨的大型鲨鱼，连蛇颈龙和沧龙都会攻击。"),
        fact: L("英語の愛称は「ジンスー・シャーク（よく切れる包丁のサメ）」。", "Its English nickname is the 'Ginsu shark', after a famous sharp knife.", "英文昵称是“金苏鲨”，取自一种锋利的刀具品牌。"),
    },
    Species {
        slug: "baculites",
        name: L("バキュリテス", "Baculites", "杆菊石"),
        sci: "Baculites",
        group: L("頭足類（まっすぐなアンモナイト）", "Cephalopod (straight ammonite)", "头足类（直壳菊石）"),
        len_cm: 100.0, t0: 85.0, t1: 66.0, hab: SHELF | SHALLOW, regions: &[Plate(LAURENTIA), Global],
        beh: Beh::Swim, art: Art::Cephalopod(0), col: [0.80, 0.70, 0.56], col2: [0.60, 0.44, 0.32], rarity: 3.0, shy: 1.2, speed: 0.2,
        desc: L("巻かずにまっすぐのびた殻を持つ、変わったアンモナイト。", "An unusual ammonite whose shell grew straight instead of coiling.", "外壳不盘卷、笔直生长的奇特菊石。"),
        fact: L("内陸の海にとても多く、殻の化石が地層にびっしり詰まっている。", "It was so common in the inland sea that its shells pack whole rock layers.", "在内陆海中数量极多，化石塞满了整层岩石。"),
    },
    Species {
        slug: "mosasaurus",
        name: L("モササウルス", "Mosasaurus", "沧龙"),
        sci: "Mosasaurus hoffmannii",
        group: L("モササウルス類（海のトカゲ）", "Mosasaur", "沧龙类"),
        len_cm: 1500.0, t0: 82.0, t1: 66.0, hab: SHELF | DEEP, regions: &[Global],
        beh: Beh::Swim, art: Art::Marine(4), col: [0.34, 0.38, 0.40], col2: [0.70, 0.72, 0.70], rarity: 0.8, shy: 3.0, speed: 0.5,
        desc: L("長さ15mにもなる、白亜紀の終わりの海の王者。オオトカゲやヘビの遠い親戚。", "Up to 15 m long, the ruler of the latest Cretaceous seas — a distant relative of monitor lizards and snakes.", "体长可达15米，是白垩纪末海洋的霸主，与巨蜥和蛇是远亲。"),
        fact: L("1764年にオランダの採石場で見つかり、のちにフランス軍が持ち去ったことで有名。", "Found in a Dutch quarry in 1764, it was later famously seized by French troops.", "1764年在荷兰采石场被发现，后来被法国军队夺走而闻名。"),
    },
    Species {
        slug: "velociraptor",
        name: L("ヴェロキラプトル", "Velociraptor", "伶盗龙"),
        sci: "Velociraptor mongoliensis",
        group: L("恐竜（羽毛のある小型の肉食恐竜）", "Dinosaur (dromaeosaur)", "恐龙（驰龙类）"),
        len_cm: 200.0, t0: 75.0, t1: 71.0, hab: LAND, regions: &[Site(S_GOBI), Plate(SIBERIA)],
        beh: Beh::Crawl, art: Art::Dino(11), col: [0.62, 0.50, 0.36], col2: [0.84, 0.72, 0.54], rarity: 1.5, shy: 2.0, speed: 0.45,
        desc: L("七面鳥ほどの大きさの、羽毛のある肉食恐竜。足に大きなかぎ爪を持っていた。", "A turkey-sized feathered predator with a big hooked claw on each foot.", "火鸡大小、长羽毛的肉食恐龙，脚上有巨大的钩爪。"),
        fact: L("プロトケラトプスと戦ったまま化石になった「格闘化石」が有名。", "It is famous from the 'Fighting Dinosaurs' fossil, locked in combat with a Protoceratops.", "以与原角龙缠斗中一起石化的“格斗化石”闻名。"),
    },
    Species {
        slug: "protoceratops",
        name: L("プロトケラトプス", "Protoceratops", "原角龙"),
        sci: "Protoceratops andrewsi",
        group: L("恐竜（角竜）", "Dinosaur (ceratopsian)", "恐龙（角龙类）"),
        len_cm: 200.0, t0: 75.0, t1: 71.0, hab: LAND, regions: &[Site(S_GOBI), Plate(SIBERIA)],
        beh: Beh::Crawl, art: Art::Dino(8), col: [0.66, 0.56, 0.42], col2: [0.80, 0.70, 0.54], rarity: 2.5, shy: 1.2, speed: 0.1,
        desc: L("ヒツジほどの大きさの、えりかざりを持つ植物食恐竜。砂漠のオアシスに群れた。", "A sheep-sized plant-eater with a neck frill that gathered around desert oases.", "体型如羊、长着颈盾的植食恐龙，聚集在沙漠绿洲附近。"),
        fact: L("グリフォン伝説のもとになったという説もある。", "Some think its fossils inspired the legend of the griffin.", "有人认为它的化石是狮鹫传说的来源。"),
    },
    Species {
        slug: "parasaurolophus",
        name: L("パラサウロロフス", "Parasaurolophus", "副栉龙"),
        sci: "Parasaurolophus walkeri",
        group: L("恐竜（鳥脚類・ハドロサウルス類）", "Dinosaur (hadrosaur)", "恐龙（鸭嘴龙类）"),
        len_cm: 950.0, t0: 77.0, t1: 73.0, hab: LAND, regions: &[Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Dino(10), col: [0.48, 0.50, 0.38], col2: [0.80, 0.56, 0.34], rarity: 1.5, shy: 1.5, speed: 0.12,
        desc: L("頭の後ろに長い管のようなとさかを持つ、カモノハシ竜の仲間。", "A duck-billed dinosaur with a long tube-like crest sweeping back from its head.", "头后有长管状头冠的鸭嘴龙类。"),
        fact: L("とさかの中は空洞で、トロンボーンのような音を鳴らせたと考えられている。", "Its hollow crest could probably make trombone-like calls.", "头冠内部中空，可能发出长号般的声音。"),
    },
    Species {
        slug: "deinosuchus",
        name: L("デイノスクス", "Deinosuchus", "恐鳄"),
        sci: "Deinosuchus",
        group: L("ワニの仲間", "Crocodylian relative", "鳄类"),
        len_cm: 1000.0, t0: 82.0, t1: 73.0, hab: TIDAL, regions: &[Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Archosaur(2), col: [0.36, 0.40, 0.32], col2: [0.52, 0.56, 0.44], rarity: 1.0, shy: 2.5, speed: 0.1,
        desc: L("長さ10mにもなる巨大なワニ。水辺で恐竜を待ちぶせした。", "A giant crocodile up to 10 m long that ambushed dinosaurs at the water's edge.", "体长可达10米的巨型鳄鱼，在水边伏击恐龙。"),
        fact: L("恐竜の骨に、このワニのかみ跡が残っている。", "Dinosaur bones have been found with its bite marks.", "恐龙骨骼上留有它的咬痕。"),
    },
    Species {
        slug: "tyrannosaurus",
        name: L("ティラノサウルス", "Tyrannosaurus", "霸王龙"),
        sci: "Tyrannosaurus rex",
        group: L("恐竜（大型の肉食恐竜）", "Dinosaur (tyrannosaur)", "恐龙（暴龙类）"),
        len_cm: 1200.0, t0: 68.0, t1: 66.0, hab: LAND, regions: &[Site(S_HELLCREEK), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Dino(1), col: [0.50, 0.42, 0.32], col2: [0.72, 0.58, 0.40], rarity: 0.8, shy: 3.0, speed: 0.35,
        desc: L("長さ12m、白亜紀の終わりの北アメリカの王者。骨までかみくだく強いあごを持っていた。", "The 12 m king of latest Cretaceous North America, with jaws strong enough to crush bone.", "体长12米，是白垩纪末北美洲的霸主，咬合力强到能咬碎骨头。"),
        fact: L("かむ力は、地上の動物で史上最強級と考えられている。", "Its bite may have been the strongest of any land animal ever.", "其咬合力可能是史上陆地动物中最强的。"),
    },
    Species {
        slug: "triceratops",
        name: L("トリケラトプス", "Triceratops", "三角龙"),
        sci: "Triceratops horridus",
        group: L("恐竜（角竜）", "Dinosaur (ceratopsian)", "恐龙（角龙类）"),
        len_cm: 900.0, t0: 68.0, t1: 66.0, hab: LAND, regions: &[Site(S_HELLCREEK), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Dino(7), col: [0.52, 0.48, 0.38], col2: [0.74, 0.62, 0.46], rarity: 1.5, shy: 1.5, speed: 0.1,
        desc: L("3本の角と大きなえりかざりを持つ、どっしりした植物食恐竜。", "A heavy plant-eater with three horns and a huge frill.", "长着三只角和巨大颈盾、体态敦实的植食恐龙。"),
        fact: L("ティラノサウルスにかまれて、治った跡のある骨が見つかっている。", "Bones with healed Tyrannosaurus bite marks show it sometimes survived attacks.", "发现过带有已愈合霸王龙咬痕的骨头。"),
    },
    Species {
        slug: "ankylosaurus",
        name: L("アンキロサウルス", "Ankylosaurus", "甲龙"),
        sci: "Ankylosaurus magniventris",
        group: L("恐竜（鎧竜）", "Dinosaur (ankylosaur)", "恐龙（甲龙类）"),
        len_cm: 700.0, t0: 68.0, t1: 66.0, hab: LAND, regions: &[Site(S_HELLCREEK), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Dino(6), col: [0.50, 0.46, 0.36], col2: [0.70, 0.64, 0.50], rarity: 1.2, shy: 1.2, speed: 0.06,
        desc: L("背中を骨の鎧でおおい、尾の先に大きなこん棒を持つ植物食恐竜。", "A plant-eater armoured with bony plates and armed with a heavy tail club.", "背部覆盖骨甲、尾端有大型尾锤的植食恐龙。"),
        fact: L("尾のこん棒のひと振りは、肉食恐竜の足の骨を折るほど強かったらしい。", "One swing of its tail club could probably break a predator's leg bone.", "尾锤一击可能足以打断肉食恐龙的腿骨。"),
    },
    Species {
        slug: "edmontosaurus",
        name: L("エドモントサウルス", "Edmontosaurus", "埃德蒙顿龙"),
        sci: "Edmontosaurus annectens",
        group: L("恐竜（鳥脚類・ハドロサウルス類）", "Dinosaur (hadrosaur)", "恐龙（鸭嘴龙类）"),
        len_cm: 1200.0, t0: 73.0, t1: 66.0, hab: LAND, regions: &[Site(S_HELLCREEK), Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Dino(9), col: [0.50, 0.50, 0.40], col2: [0.70, 0.68, 0.54], rarity: 2.0, shy: 1.5, speed: 0.12,
        desc: L("カモのような平たいくちばしの、大きな群れで暮らした植物食恐竜。", "A duck-billed plant-eater that lived in big herds.", "长着鸭子般扁喙、成大群生活的植食恐龙。"),
        fact: L("口の奥に、何百本もの歯がびっしり並んでいた。", "Its jaws held hundreds of tightly packed grinding teeth.", "嘴里密密排列着数百颗研磨齿。"),
    },
    Species {
        slug: "quetzalcoatlus",
        name: L("ケツァルコアトルス", "Quetzalcoatlus", "风神翼龙"),
        sci: "Quetzalcoatlus northropi",
        group: L("翼竜（アズダルコ類）", "Pterosaur (azhdarchid)", "翼龙（神龙翼龙类）"),
        len_cm: 500.0, t0: 68.0, t1: 66.0, hab: LAND, regions: &[Plate(LAURENTIA)],
        beh: Beh::Crawl, art: Art::Pterosaur(2), col: [0.66, 0.64, 0.60], col2: [0.84, 0.50, 0.40], rarity: 1.0, shy: 2.5, speed: 0.15,
        desc: L("翼を広げると10mにもなる、史上最大級の飛ぶ動物。地上では四つ足で歩き、小さな動物を食べた。", "One of the largest flying animals ever, with a 10 m wingspan; on the ground it walked on all fours and ate small animals.", "翼展可达10米，是史上最大的飞行动物之一；在地面上四足行走，捕食小动物。"),
        fact: L("立つとキリンほどの背の高さがあった。", "Standing, it was as tall as a giraffe.", "站立时和长颈鹿一样高。"),
    },
    Species {
        slug: "early-magnolia",
        name: L("初期のモクレンの仲間", "Early magnolia relative", "早期木兰类"),
        sci: "Magnoliaceae (early)",
        group: L("被子植物（花の咲く木）", "Flowering tree", "被子植物（开花树木）"),
        len_cm: 800.0, t0: 100.0, t1: 66.0, hab: LAND, regions: &[Global],
        beh: Beh::Sessile, art: Art::Plant(14), col: [0.40, 0.32, 0.26], col2: [0.30, 0.50, 0.26], rarity: 3.0, shy: 0.0, speed: 0.0,
        desc: L("大きな白い花を咲かせた、初期の花の咲く木。甲虫が花粉を運んだ。", "An early flowering tree with big white blossoms, pollinated by beetles.", "开出大白花的早期开花树木，靠甲虫传粉。"),
        fact: L("白亜紀の終わりには、花の咲く植物が陸の植物の主役になっていた。", "By the end of the Cretaceous, flowering plants had become the main plants on land.", "到白垩纪末，开花植物已成为陆地植物的主角。"),
    },
    // 広域の動物群。既存の種の番号を保つため、必ず末尾に追加する。
    Species {
        slug: "mesozoic-roachoids",
        name: L("中生代のゴキブリ型昆虫", "Mesozoic roach-like insects", "中生代蜚蠊型昆虫"),
        sci: "Blattodea and roach-like relatives",
        group: L("広域の動物群（代表像）", "Widespread animal group (representative)", "广布动物类群（代表形象）"),
        len_cm: 3.0, t0: 251.9, t1: 66.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Crawl, art: Art::Arthro(4), col: [0.48, 0.42, 0.30], col2: [0.72, 0.64, 0.46], rarity: 2.2, shy: 1.2, speed: 0.1,
        desc: L("落ち葉の間を歩き、植物のかけらなどを食べた小さな昆虫の仲間。", "Small insects that walked among leaf litter and fed on plant material.", "在落叶间行走、取食植物碎屑等的小型昆虫。"),
        fact: L("単一の種ではなく、各地の近縁な動物をまとめた項目。姿と大きさは代表例で、時代・分布はゲーム用の概略。", "This entry groups related animals, rather than one species. Appearance and size are representative; dates and distribution are simplified for the game.", "本条目汇总各地近缘动物，并非单一物种。形态和大小为代表例，年代及分布为游戏简化。"),
    },
    Species {
        slug: "mesozoic-millipedes",
        name: L("中生代のヤスデの仲間", "Mesozoic millipedes", "中生代马陆类"),
        sci: "Diplopoda",
        group: L("広域の動物群（代表像）", "Widespread animal group (representative)", "广布动物类群（代表形象）"),
        len_cm: 8.0, t0: 251.9, t1: 66.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Crawl, art: Art::Arthro(0), col: [0.48, 0.42, 0.30], col2: [0.72, 0.64, 0.46], rarity: 1.5, shy: 1.2, speed: 0.07,
        desc: L("たくさんの足で林床を歩き、落ち葉などの分解を助けた動物。", "Many-legged forest-floor animals that helped break down fallen leaves.", "用许多足在林地行走、帮助分解落叶的动物。"),
        fact: L("単一の種ではなく、各地の近縁な動物をまとめた項目。姿と大きさは代表例で、時代・分布はゲーム用の概略。", "This entry groups related animals, rather than one species. Appearance and size are representative; dates and distribution are simplified for the game.", "本条目汇总各地近缘动物，并非单一物种。形态和大小为代表例，年代及分布为游戏简化。"),
    },
    Species {
        slug: "mesozoic-scorpions",
        name: L("中生代のサソリの仲間", "Mesozoic scorpions", "中生代蝎类"),
        sci: "Scorpiones",
        group: L("広域の動物群（代表像）", "Widespread animal group (representative)", "广布动物类群（代表形象）"),
        len_cm: 10.0, t0: 251.9, t1: 66.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Crawl, art: Art::Arthro(2), col: [0.48, 0.42, 0.30], col2: [0.72, 0.64, 0.46], rarity: 0.8, shy: 1.2, speed: 0.12,
        desc: L("はさみと尾を持ち、地表で虫などを捕らえた小さな捕食者。", "Small ground predators with pincers and a tail, hunting insects and other prey.", "长着钳和尾巴、在地面捕食昆虫等的小型捕食者。"),
        fact: L("単一の種ではなく、各地の近縁な動物をまとめた項目。姿と大きさは代表例で、時代・分布はゲーム用の概略。", "This entry groups related animals, rather than one species. Appearance and size are representative; dates and distribution are simplified for the game.", "本条目汇总各地近缘动物，并非单一物种。形态和大小为代表例，年代及分布为游戏简化。"),
    },
    Species {
        slug: "mesozoic-small-reptiles",
        name: L("中生代の小型爬虫類", "Mesozoic small reptiles", "中生代小型爬行动物"),
        sci: "Small terrestrial Diapsida",
        group: L("広域の動物群（代表像）", "Widespread animal group (representative)", "广布动物类群（代表形象）"),
        len_cm: 25.0, t0: 251.9, t1: 66.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Crawl, art: Art::Tetrapod(2), col: [0.48, 0.42, 0.30], col2: [0.72, 0.64, 0.46], rarity: 1.6, shy: 1.2, speed: 0.18,
        desc: L("恐竜だけでなく、虫などを食べる小さな爬虫類も地表で暮らしていた。", "Alongside larger animals, small reptiles lived on the ground and ate insects and other prey.", "除了大型动物，地面上也生活着捕食昆虫等的小型爬行动物。"),
        fact: L("単一の種ではなく、各地の近縁な動物をまとめた項目。姿と大きさは代表例で、時代・分布はゲーム用の概略。", "This entry groups related animals, rather than one species. Appearance and size are representative; dates and distribution are simplified for the game.", "本条目汇总各地近缘动物，并非单一物种。形态和大小为代表例，年代及分布为游戏简化。"),
    },
    Species {
        slug: "triassic-cynodonts",
        name: L("三畳紀のキノドン類", "Triassic cynodonts", "三叠纪犬齿兽类"),
        sci: "Cynodontia",
        group: L("広域の動物群（代表像）", "Widespread animal group (representative)", "广布动物类群（代表形象）"),
        len_cm: 45.0, t0: 251.9, t1: 201.4, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Crawl, art: Art::Mammal(3), col: [0.48, 0.42, 0.30], col2: [0.72, 0.64, 0.46], rarity: 1.2, shy: 1.2, speed: 0.15,
        desc: L("哺乳類につながる仲間。三畳紀の陸には、小型のものから大きなものまでいた。", "Mammal relatives ranging from small to large forms on Triassic land.", "通向哺乳动物的亲缘类群，在三叠纪陆地上有大小不同的成员。"),
        fact: L("単一の種ではなく、各地の近縁な動物をまとめた項目。姿と大きさは代表例で、時代・分布はゲーム用の概略。", "This entry groups related animals, rather than one species. Appearance and size are representative; dates and distribution are simplified for the game.", "本条目汇总各地近缘动物，并非单一物种。形态和大小为代表例，年代及分布为游戏简化。"),
    },
    Species {
        slug: "mesozoic-mammaliaforms",
        name: L("中生代の小型哺乳形類", "Mesozoic small mammaliaforms", "中生代小型哺乳形类"),
        sci: "Mammaliaformes",
        group: L("広域の動物群（代表像）", "Widespread animal group (representative)", "广布动物类群（代表形象）"),
        len_cm: 15.0, t0: 201.4, t1: 66.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Crawl, art: Art::Mammal(0), col: [0.48, 0.42, 0.30], col2: [0.72, 0.64, 0.46], rarity: 1.5, shy: 1.2, speed: 0.18,
        desc: L("毛に覆われた小さな動物たち。恐竜の足もとで、虫などを探して暮らした。", "Small furry animals that searched for insects and other food beneath the dinosaurs.", "披着毛的小动物，在恐龙脚下寻找昆虫等食物。"),
        fact: L("単一の種ではなく、各地の近縁な動物をまとめた項目。姿と大きさは代表例で、時代・分布はゲーム用の概略。", "This entry groups related animals, rather than one species. Appearance and size are representative; dates and distribution are simplified for the game.", "本条目汇总各地近缘动物，并非单一物种。形态和大小为代表例，年代及分布为游戏简化。"),
    },
    Species {
        slug: "mesozoic-rhynchocephalians",
        name: L("中生代のムカシトカゲ類", "Mesozoic rhynchocephalians", "中生代喙头类"),
        sci: "Rhynchocephalia",
        group: L("広域の動物群（代表像）", "Widespread animal group (representative)", "广布动物类群（代表形象）"),
        len_cm: 30.0, t0: 242.0, t1: 66.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Crawl, art: Art::Tetrapod(2), col: [0.48, 0.42, 0.30], col2: [0.72, 0.64, 0.46], rarity: 0.9, shy: 1.2, speed: 0.15,
        desc: L("トカゲに似ているが別の仲間。中生代には今よりずっと多様だった。", "Reptiles resembling lizards but belonging to a different lineage, more diverse in the Mesozoic than today.", "外形像蜥蜴却属于不同支系，中生代时比今天更多样。"),
        fact: L("単一の種ではなく、各地の近縁な動物をまとめた項目。姿と大きさは代表例で、時代・分布はゲーム用の概略。", "This entry groups related animals, rather than one species. Appearance and size are representative; dates and distribution are simplified for the game.", "本条目汇总各地近缘动物，并非单一物种。形态和大小为代表例，年代及分布为游戏简化。"),
    },
    Species {
        slug: "mesozoic-small-theropods",
        name: L("中生代の小型獣脚類", "Mesozoic small theropods", "中生代小型兽脚类"),
        sci: "Small Theropoda",
        group: L("広域の動物群（代表像）", "Widespread animal group (representative)", "广布动物类群（代表形象）"),
        len_cm: 180.0, t0: 233.0, t1: 66.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Crawl, art: Art::Dino(0), col: [0.48, 0.42, 0.30], col2: [0.72, 0.64, 0.46], rarity: 1.1, shy: 1.2, speed: 0.32,
        desc: L("二本足ですばやく動く恐竜の仲間。時代と地域によって異なる種類が暮らした。", "Agile two-legged dinosaurs, represented by different kinds in different times and regions.", "灵活的两足恐龙，不同时代和地区生活着不同种类。"),
        fact: L("単一の種ではなく、各地の近縁な動物をまとめた項目。姿と大きさは代表例で、時代・分布はゲーム用の概略。", "This entry groups related animals, rather than one species. Appearance and size are representative; dates and distribution are simplified for the game.", "本条目汇总各地近缘动物，并非单一物种。形态和大小为代表例，年代及分布为游戏简化。"),
    },
    Species {
        slug: "mesozoic-sauropodomorphs",
        name: L("中生代の竜脚形類", "Mesozoic sauropodomorphs", "中生代蜥脚形类"),
        sci: "Sauropodomorpha",
        group: L("広域の動物群（代表像）", "Widespread animal group (representative)", "广布动物类群（代表形象）"),
        len_cm: 650.0, t0: 233.0, t1: 66.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Crawl, art: Art::Dino(4), col: [0.48, 0.42, 0.30], col2: [0.72, 0.64, 0.46], rarity: 0.7, shy: 1.2, speed: 0.13,
        desc: L("長い首を持つ植物食恐竜の大きな仲間。初期の二足歩行のものから、のちの巨大な竜脚類まで含む。", "A major group of long-necked plant eaters, from early two-legged forms to later giant sauropods.", "长颈植食恐龙的大类，包括早期两足成员和后来的巨型蜥脚类。"),
        fact: L("単一の種ではなく、各地の近縁な動物をまとめた項目。姿と大きさは代表例で、時代・分布はゲーム用の概略。", "This entry groups related animals, rather than one species. Appearance and size are representative; dates and distribution are simplified for the game.", "本条目汇总各地近缘动物，并非单一物种。形态和大小为代表例，年代及分布为游戏简化。"),
    },
    Species {
        slug: "mesozoic-ornithopods",
        name: L("中生代の小型鳥脚類", "Mesozoic small ornithopods", "中生代小型鸟脚类"),
        sci: "Small Ornithopoda",
        group: L("広域の動物群（代表像）", "Widespread animal group (representative)", "广布动物类群（代表形象）"),
        len_cm: 200.0, t0: 170.0, t1: 66.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Crawl, art: Art::Dino(9), col: [0.48, 0.42, 0.30], col2: [0.72, 0.64, 0.46], rarity: 1.2, shy: 1.2, speed: 0.25,
        desc: L("くちばしと奥歯で植物を食べる恐竜の仲間。小さな種類も各地にいた。", "Beaked dinosaurs with chewing teeth; small forms lived in many regions.", "用喙和后牙吃植物的恐龙，许多地区都有小型成员。"),
        fact: L("単一の種ではなく、各地の近縁な動物をまとめた項目。姿と大きさは代表例で、時代・分布はゲーム用の概略。", "This entry groups related animals, rather than one species. Appearance and size are representative; dates and distribution are simplified for the game.", "本条目汇总各地近缘动物，并非单一物种。形态和大小为代表例，年代及分布为游戏简化。"),
    },
    Species {
        slug: "mesozoic-pterosaurs",
        name: L("中生代の翼竜の仲間", "Mesozoic pterosaurs", "中生代翼龙类"),
        sci: "Pterosauria",
        group: L("広域の動物群（代表像）", "Widespread animal group (representative)", "广布动物类群（代表形象）"),
        len_cm: 120.0, t0: 228.0, t1: 66.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Swim, art: Art::Pterosaur(0), col: [0.48, 0.42, 0.30], col2: [0.72, 0.64, 0.46], rarity: 0.7, shy: 1.2, speed: 0.55,
        desc: L("膜の翼で飛ぶ爬虫類。空や水辺を利用し、時代ごとに違う仲間が広がった。", "Reptiles flying on membrane wings, with different forms using skies and shores through time.", "用膜翼飞行的爬行动物，不同时代的成员利用天空和水岸环境。"),
        fact: L("単一の種ではなく、各地の近縁な動物をまとめた項目。姿と大きさは代表例で、時代・分布はゲーム用の概略。", "This entry groups related animals, rather than one species. Appearance and size are representative; dates and distribution are simplified for the game.", "本条目汇总各地近缘动物，并非单一物种。形态和大小为代表例，年代及分布为游戏简化。"),
    },
    Species {
        slug: "mesozoic-crocodylomorphs",
        name: L("中生代のワニ形類", "Mesozoic crocodylomorphs", "中生代鳄形类"),
        sci: "Crocodylomorpha",
        group: L("広域の動物群（代表像）", "Widespread animal group (representative)", "广布动物类群（代表形象）"),
        len_cm: 160.0, t0: 230.0, t1: 66.0, hab: LAND | TIDAL, regions: &[Global],
        beh: Beh::Crawl, art: Art::Archosaur(2), col: [0.48, 0.42, 0.30], col2: [0.72, 0.64, 0.46], rarity: 0.7, shy: 1.2, speed: 0.17,
        desc: L("今のワニにつながる多様な仲間。水辺だけでなく、陸を歩いて暮らすものもいた。", "Diverse crocodile relatives, including forms that walked on land as well as shore dwellers.", "通向现代鳄类的多样类群，既有水岸成员也有陆地行走的成员。"),
        fact: L("単一の種ではなく、各地の近縁な動物をまとめた項目。姿と大きさは代表例で、時代・分布はゲーム用の概略。", "This entry groups related animals, rather than one species. Appearance and size are representative; dates and distribution are simplified for the game.", "本条目汇总各地近缘动物，并非单一物种。形态和大小为代表例，年代及分布为游戏简化。"),
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
    /// ペルム紀〜ジュラ紀前期の砂漠
    Desert,
    /// 恐竜時代の針葉樹とシダの森
    GymnoForest,
    /// 花の咲く森
    FlowerForest,
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
pub const P_CAMP: usize = 13;
pub const P_LAURASIA: usize = 14;
pub const P_GYMNO_FOREST: usize = 15;
pub const P_FLOWER_FOREST: usize = 16;
pub const P_DECCAN: usize = 17;
pub const P_CHICXULUB: usize = 18;

/// ローラシアをつくる大陸
pub const LAURASIA_PLATES: &[usize] = &[LAURENTIA, BALTICA, AVALONIA, SIBERIA, KAZAKH, TARIM, NORTH_CHINA, SOUTH_CHINA];

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
        t1: 200.0,
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
        t1: 195.0,
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
    Phenom {
        slug: "camp-eruption",
        kind: PhKind::Eruption(2),
        t0: 201.8,
        t1: 201.2,
        name: L("中央大西洋の大噴火（CAMP）", "Central Atlantic eruptions (CAMP)", "中大西洋岩浆省大喷发"),
        desc: L(
            "パンゲアが裂けはじめた場所で、地球史上最大級の溶岩が噴き出した。噴き出したガスで地球が温まり、三畳紀末の大絶滅が起こった。",
            "Where Pangaea began to split, one of the largest lava outpourings in history burst forth. Its gases warmed the planet and caused the end-Triassic extinction.",
            "在盘古大陆开始裂开的地方，喷发出地球史上规模最大的熔岩之一。喷出的气体使地球变暖，引发了三叠纪末大灭绝。",
        ),
        hint: L("2億150万年前ごろ、上空から噴火に近づいて撮影する", "Around 201.5 million years ago, fly close and photograph the eruption from the sky", "约2亿150万年前，从高空靠近拍摄喷发"),
    },
    Phenom {
        slug: "laurasia",
        kind: PhKind::Continent(LAURASIA_PLATES),
        t0: 180.0,
        t1: 66.0,
        name: L("ローラシア大陸", "Laurasia", "劳亚大陆"),
        desc: L(
            "パンゲアが南北に分かれてできた、北の大陸。いまの北アメリカ・ヨーロッパ・アジアがつながっていた。",
            "The northern continent left when Pangaea split in two, joining today's North America, Europe and Asia.",
            "盘古大陆南北分裂后形成的北方大陆，今天的北美洲、欧洲和亚洲连在一起。",
        ),
        hint: L("ジュラ紀〜白亜紀に、上空から大陸全体が写るように撮影する", "In the Jurassic or Cretaceous, photograph the whole continent from the sky", "在侏罗纪至白垩纪从高空拍下整个大陆"),
    },
    Phenom {
        slug: "gymnosperm-forest",
        kind: PhKind::Scenery(Scenery::GymnoForest),
        t0: 250.0,
        t1: 125.0,
        name: L("恐竜時代の針葉樹とシダの森", "Dinosaur-age conifer and fern forest", "恐龙时代的针叶树与蕨类森林"),
        desc: L(
            "三畳紀からジュラ紀の森は、針葉樹・ソテツの仲間・イチョウ・木生シダでできていた。花も草もまだなく、地面はシダがおおった。",
            "Triassic and Jurassic forests were made of conifers, cycad-like plants, ginkgos and tree ferns. There were no flowers or grass yet; ferns carpeted the ground.",
            "三叠纪至侏罗纪的森林由针叶树、苏铁类、银杏和树蕨组成，还没有花和草，地面覆盖着蕨类。",
        ),
        hint: L("三畳紀〜白亜紀前期に、陸に降りて森を撮影する", "From the Triassic to the Early Cretaceous, land and photograph the forest", "在三叠纪至早白垩世降落到陆地上拍摄森林"),
    },
    Phenom {
        slug: "flowering-forest",
        kind: PhKind::Scenery(Scenery::FlowerForest),
        t0: 125.0,
        t1: 66.0,
        name: L("花の咲く森", "The first flowering forests", "开花的森林"),
        desc: L(
            "白亜紀の後半、モクレンなどの花の咲く木が森に加わり、陸の景色がいろどり豊かになった。虫と花の助け合いもここから始まった。",
            "In the later Cretaceous, flowering trees such as magnolias joined the forests and colour came to the land. The partnership between insects and flowers began.",
            "白垩纪后期，木兰等开花树木加入森林，陆地景色变得多彩。昆虫与花朵的互助关系也从此开始。",
        ),
        hint: L("白亜紀に、陸に降りて景色を撮影する", "In the Cretaceous, land and photograph the scenery", "在白垩纪降落到陆地上拍摄风景"),
    },
    Phenom {
        slug: "deccan-traps",
        kind: PhKind::Eruption(3),
        t0: 66.6,
        t1: 66.05,
        name: L("デカン・トラップの噴火", "Deccan Traps eruption", "德干暗色岩喷发"),
        desc: L(
            "インドで何十万年も続いた巨大な溶岩の噴火。階段のような溶岩の大地（トラップ）がいまも残っている。",
            "Vast lava eruptions in India lasted hundreds of thousands of years, leaving step-like lava plateaus (traps) that remain today.",
            "印度持续数十万年的巨量熔岩喷发，留下的阶梯状熔岩高原（暗色岩）至今犹存。",
        ),
        hint: L("6650万年前ごろ、上空から噴火に近づいて撮影する", "Around 66.5 million years ago, fly close and photograph the eruption from the sky", "约6650万年前，从高空靠近拍摄喷发"),
    },
    Phenom {
        slug: "chicxulub-impact",
        kind: PhKind::Eruption(4),
        t0: 66.05,
        t1: 66.0,
        name: L("チクシュルーブの衝突", "The Chicxulub impact", "希克苏鲁伯撞击"),
        desc: L(
            "直径10kmの小惑星が、いまのメキシコ・ユカタン半島に落ちた。巨大な津波と山火事が起こり、空をおおったちりで太陽の光がさえぎられ、鳥以外の恐竜が絶滅した。",
            "A 10 km asteroid struck what is now Mexico's Yucatán. Giant tsunamis and wildfires followed, dust blotted out the Sun, and all dinosaurs except birds died out.",
            "直径10公里的小行星坠落在今墨西哥尤卡坦半岛，引发巨大海啸和山火，遮天蔽日的尘埃挡住了阳光，除鸟类以外的恐龙全部灭绝。",
        ),
        hint: L("6600万年前、上空からユカタン半島に近づいて撮影する", "66 million years ago, fly close to the Yucatán from the sky and photograph it", "6600万年前，从高空靠近尤卡坦半岛拍摄"),
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
        Region::Plate(p) => spot.plate.map_or(false, |sp| plate_matches(sp, p)),
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
                // 産地の環境（海・陸など）に暮らさないもの（海の産地の空を飛ぶ翼竜など）は、まわりを探す
                if sp.hab & site.want.bit() == 0 {
                    continue;
                }
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
    // 中生代：花の咲く植物が広がるまでは針葉樹とシダの森
    if t <= 251.9 {
        return Some(if t > 125.0 { Scenery::GymnoForest } else { Scenery::FlowerForest });
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
            for p in expand_plates(plates) {
                let (lon, lat) = earth.plate_center(p, t);
                let w = 1.0;
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

/// レーダー：その時代に、図鑑の項目（生きもの・現象）に会える場所（経緯度）の一覧。
/// 地球全体を3度ごとに調べ、産地のあるものは産地のまわりを細かく調べる。その時代にいなければ空
pub fn radar_spots(e: usize, earth: &Earth, t: f32) -> Vec<(f32, f32)> {
    let ns = SPECIES.len();
    let mut out = Vec::new();
    let mut fine: Vec<(f32, f32)> = Vec::new();
    if e < ns {
        let sp = &SPECIES[e];
        if !sp.alive_at(t) {
            return out;
        }
        for r in sp.regions {
            if let Region::Site(s) = *r {
                if t <= SITES[s].t0 + 0.5 && t >= SITES[s].t1 - 0.5 {
                    fine.push(earth.site_lonlat(s, t));
                }
            }
        }
    } else {
        let ph = &PHENOMS[e - ns];
        if !ph.at(t) {
            return out;
        }
        if let PhKind::Eruption(i) = ph.kind {
            out.push(earth.site_lonlat(ERUPTIONS[i].site, t));
            return out;
        }
    }
    let ok = |lon: f32, lat: f32| -> bool {
        let spot = earth.sample(lon, lat, t);
        if e < ns {
            return lives_here(&SPECIES[e], earth, &spot, t);
        }
        match PHENOMS[e - ns].kind {
            PhKind::Scenery(k) => scenery_at(&spot, t) == Some(k),
            PhKind::Continent(plates) => spot.hab == Habitat::Land && spot.plate.map_or(false, |p| plates.iter().any(|&q| plate_matches(p, q))),
            PhKind::Eruption(_) => false,
        }
    };
    let mut lat = -88.5;
    while lat < 89.0 {
        let mut lon = -178.5;
        while lon < 180.0 {
            if ok(lon, lat) {
                out.push((lon, lat));
            }
            lon += 3.0;
        }
        lat += 3.0;
    }
    // 産地のまわりは細かく（干潟や小さな入り江も見のがさない）
    for (cx, cy) in fine {
        let mut dy = -6.0;
        while dy <= 6.0 {
            let mut dx = -6.0;
            while dx <= 6.0 {
                let (lon, lat) = (cx + dx, cy + dy);
                if lat.abs() < 89.5 && ok(lon, lat) && !out.iter().any(|q| (q.0 - lon).abs() < 1.4 && (q.1 - lat).abs() < 1.4) {
                    out.push((lon, lat));
                }
                dx += 1.0;
            }
            dy += 1.0;
        }
    }
    out
}

/// 図鑑の項目がいた時代（百万年前、古い方から）
pub fn entry_span(e: usize) -> (f32, f32) {
    if e < SPECIES.len() {
        (SPECIES[e].t0, SPECIES[e].t1)
    } else {
        (PHENOMS[e - SPECIES.len()].t0, PHENOMS[e - SPECIES.len()].t1)
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

    /// Check the whole map at every Mesozoic stage boundary and midpoint,
    /// including the poles and continental interiors away from fossil sites.
    #[test]
    fn mesozoic_land_has_diverse_animals_throughout_every_stage() {
        let earth = Earth::new();
        for stage in STAGES.iter().filter(|s| s.t0 <= 251.9) {
            for t in [stage.t0, (stage.t0 + stage.t1) * 0.5, stage.t1] {
                let mut land = 0;
                let mut empty = 0;
                for lat in (-85..=85).step_by(5) {
                    for lon in (-180..180).step_by(5) {
                        let spot = earth.sample(lon as f32, lat as f32, t);
                        if spot.hab != Habitat::Land { continue; }
                        land += 1;
                        let animals = SPECIES.iter().filter(|sp| {
                            !matches!(sp.art, Art::Plant(_) | Art::Mat | Art::Stromatolite | Art::Trace)
                                && lives_here(sp, &earth, &spot, t)
                        }).count();
                        if animals == 0 { empty += 1; }
                        assert!(animals >= 4, "only {animals} animals at {lon},{lat}, t={t}");
                    }
                }
                assert!(land > 100, "insufficient land samples at t={t}");
                println!("Mesozoic land t={t:.2}: {land} samples, {empty} empty");
            }
        }
    }

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

    /// レーダーは、会いに行く時代にはかならず場所を示す。いない時代には何も示さない
    #[test]
    fn radar_finds_every_entry() {
        let e = Earth::new();
        for i in 0..n_entries() {
            let (t, _) = if i < SPECIES.len() { where_when(i, &e, 535.0, (40.0, 5.0)) } else { phenom_where_when(i - SPECIES.len(), &e, 535.0, (40.0, 5.0)) };
            let spots = radar_spots(i, &e, t);
            assert!(!spots.is_empty(), "{} at t={}", entry_slug(i), t);
            if i < SPECIES.len() {
                let (lon, lat) = spots[0];
                assert!(lives_here(&SPECIES[i], &e, &e.sample(lon, lat, t), t), "{}", entry_slug(i));
            }
            let (t0, _) = entry_span(i);
            if t0 + 5.0 < T_START {
                assert!(radar_spots(i, &e, t0 + 5.0).is_empty(), "{}", entry_slug(i));
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
