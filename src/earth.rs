//! 地球：カンブリア紀の大陸（プレート）の形・位置・移動、地形の高さ、化石産地（観察地点）、大きな出来事
//!
//! 大陸はそれぞれ「自分の座標（経緯度の差、度）」で輪郭と高さを持ち、時代ごとの中心位置・回転（キーフレーム）で
//! 地図の上を動く。地図は正距円筒図法（経度 -180〜180、緯度 90〜-90）で、1度＝10 単位。
//! 位置・形はおおよそ（Scotese・Torsvik などの古地理図を参考にした、ゲーム用の簡略化）。

use crate::lang::L;
use crate::rng::fbm;

pub const DEG: f32 = 10.0;
pub const MAP_W: f32 = 360.0 * DEG;
pub const MAP_H: f32 = 180.0 * DEG;

/// カンブリア紀の始まりと終わり（百万年前）
pub const T_START: f32 = 538.8;
pub const T_END: f32 = 485.4;

/// 大陸の高さのテクスチャの細かさ（1度あたりの画素）
const RES: f32 = 6.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Habitat {
    /// 陸（植物はまだない）
    Land,
    /// 干潟・潮だまり
    Tidal,
    /// 浅い海（〜30m）
    Shallow,
    /// 大陸棚（30〜200m）
    Shelf,
    /// 深海
    Deep,
}

impl Habitat {
    pub fn bit(self) -> u8 {
        1 << (self as u8)
    }
    pub fn name(self) -> &'static str {
        match self {
            Habitat::Land => t!("陸", "Land", "陆地"),
            Habitat::Tidal => t!("干潟", "Tidal flat", "潮坪"),
            Habitat::Shallow => t!("浅い海", "Shallow sea", "浅海"),
            Habitat::Shelf => t!("大陸棚の海", "Continental shelf", "大陆架"),
            Habitat::Deep => t!("外洋・深海", "Open ocean", "远洋·深海"),
        }
    }
    pub fn is_water(self) -> bool {
        !matches!(self, Habitat::Land)
    }
}

pub const LAND: u8 = 1;
pub const TIDAL: u8 = 2;
pub const SHALLOW: u8 = 4;
pub const SHELF: u8 = 8;
pub const DEEP: u8 = 16;

pub struct PlateDef {
    pub name: L,
    /// いまの地名（どの大陸・地域になるか）
    pub now: L,
    /// 輪郭（自分の座標、度。x=東、y=北）
    pub outline: &'static [(f32, f32)],
    /// (百万年前, 中心の経度, 緯度, 回転（度、反時計回り）)
    pub keys: &'static [(f32, f32, f32, f32)],
    /// 内陸が浅い海に沈む度合い（カンブリア紀は海面が高く、大陸の内側まで海が入り込んだ）
    pub flood: f32,
    /// 山の多さ
    pub relief: f32,
    pub seed: u32,
    /// 名前を出す大陸か（小さな島は出さない）
    pub label: bool,
}

pub const GONDWANA: usize = 0;
pub const LAURENTIA: usize = 1;
pub const BALTICA: usize = 2;
pub const AVALONIA: usize = 3;
pub const SIBERIA: usize = 4;
pub const SOUTH_CHINA: usize = 5;
#[allow(dead_code)]
pub const NORTH_CHINA: usize = 6;

pub static PLATES: &[PlateDef] = &[
    PlateDef {
        name: L("ゴンドワナ大陸", "Gondwana", "冈瓦纳大陆"),
        now: L("アフリカ・南アメリカ・南極・オーストラリア・インドなど", "Africa, South America, Antarctica, Australia, India and more", "非洲、南美洲、南极洲、澳大利亚、印度等"),
        outline: &[
            (-62.0, 20.0), (-50.0, 32.0), (-35.0, 38.0), (-20.0, 40.0), (-5.0, 46.0), (10.0, 50.0), (25.0, 52.0), (40.0, 55.0), (55.0, 54.0), (68.0, 48.0),
            (78.0, 38.0), (80.0, 25.0), (72.0, 12.0), (62.0, 0.0), (55.0, -12.0), (45.0, -25.0), (30.0, -38.0), (10.0, -44.0), (-10.0, -46.0), (-30.0, -42.0),
            (-48.0, -34.0), (-60.0, -22.0), (-68.0, -8.0), (-70.0, 6.0),
        ],
        keys: &[(540.0, 40.0, -44.0, -8.0), (510.0, 44.0, -40.0, 0.0), (485.0, 48.0, -37.0, 6.0)],
        flood: 0.2,
        relief: 1.0,
        seed: 11,
        label: true,
    },
    PlateDef {
        name: L("ローレンシア大陸", "Laurentia", "劳伦大陆"),
        now: L("北アメリカ・グリーンランド", "North America and Greenland", "北美洲和格陵兰"),
        outline: &[
            (-28.0, 14.0), (-15.0, 20.0), (0.0, 21.0), (14.0, 18.0), (26.0, 13.0), (31.0, 4.0), (27.0, -7.0), (18.0, -16.0), (5.0, -20.0), (-8.0, -19.0),
            (-20.0, -14.0), (-28.0, -4.0), (-31.0, 6.0),
        ],
        keys: &[(540.0, -104.0, -4.0, -10.0), (510.0, -98.0, 0.0, -2.0), (485.0, -92.0, 4.0, 6.0)],
        flood: 0.42,
        relief: 0.6,
        seed: 23,
        label: true,
    },
    PlateDef {
        name: L("バルティカ大陸", "Baltica", "波罗的大陆"),
        now: L("北ヨーロッパ・スカンディナヴィア・ロシア西部", "Northern Europe, Scandinavia and western Russia", "北欧、斯堪的纳维亚和俄罗斯西部"),
        outline: &[(-12.0, 9.0), (0.0, 11.0), (12.0, 7.0), (15.0, -2.0), (9.0, -10.0), (-3.0, -11.0), (-12.0, -6.0), (-15.0, 2.0)],
        keys: &[(540.0, -62.0, -60.0, -30.0), (510.0, -57.0, -54.0, -14.0), (485.0, -52.0, -48.0, 0.0)],
        flood: 0.55,
        relief: 0.4,
        seed: 37,
        label: true,
    },
    PlateDef {
        name: L("アヴァロニア", "Avalonia", "阿瓦隆尼亚"),
        now: L("イギリス南部・ニューファンドランド東部・アメリカ北東部の一部", "Southern Britain, eastern Newfoundland and parts of New England", "英国南部、纽芬兰东部和美国东北部一部分"),
        outline: &[(-14.0, 3.0), (-4.0, 4.5), (6.0, 4.0), (14.0, 1.5), (12.0, -2.0), (2.0, -3.5), (-8.0, -3.0), (-15.0, -1.0)],
        keys: &[(540.0, -38.0, -40.0, 62.0), (510.0, -42.0, -38.0, 56.0), (485.0, -48.0, -36.0, 50.0)],
        flood: 0.35,
        relief: 0.5,
        seed: 41,
        label: true,
    },
    PlateDef {
        name: L("シベリア大陸", "Siberia", "西伯利亚大陆"),
        now: L("シベリア（ロシア）", "Siberia (Russia)", "西伯利亚（俄罗斯）"),
        outline: &[(-16.0, 10.0), (-4.0, 14.0), (10.0, 13.0), (18.0, 6.0), (17.0, -5.0), (8.0, -12.0), (-6.0, -11.0), (-15.0, -4.0)],
        keys: &[(540.0, 70.0, 30.0, 0.0), (510.0, 75.0, 31.0, 5.0), (485.0, 80.0, 33.0, 10.0)],
        flood: 0.68,
        relief: 0.35,
        seed: 53,
        label: true,
    },
    PlateDef {
        name: L("南中国（揚子）地塊", "South China", "华南（扬子）板块"),
        now: L("中国南部", "Southern China", "中国南部"),
        outline: &[(-8.0, 5.0), (2.0, 7.0), (9.0, 4.0), (9.5, -3.0), (1.0, -6.0), (-7.0, -4.0)],
        keys: &[(540.0, 126.0, 8.0, 0.0), (510.0, 129.0, 12.0, 3.0), (485.0, 132.0, 16.0, 6.0)],
        flood: 0.5,
        relief: 0.5,
        seed: 67,
        label: true,
    },
    PlateDef {
        name: L("北中国地塊", "North China", "华北板块"),
        now: L("中国北部・朝鮮半島", "Northern China and Korea", "中国北部和朝鲜半岛"),
        outline: &[(-6.0, 4.0), (4.0, 5.0), (7.0, 0.0), (3.0, -4.0), (-5.0, -3.0)],
        keys: &[(540.0, 146.0, 24.0, 0.0), (510.0, 148.0, 27.0, 4.0), (485.0, 150.0, 30.0, 8.0)],
        flood: 0.5,
        relief: 0.4,
        seed: 71,
        label: false,
    },
    PlateDef {
        name: L("カザフの島々", "Kazakh islands", "哈萨克岛群"),
        now: L("カザフスタン", "Kazakhstan", "哈萨克斯坦"),
        outline: &[(-3.0, 2.0), (3.0, 3.0), (4.0, -1.0), (-2.0, -3.0)],
        keys: &[(540.0, 102.0, 24.0, 0.0), (485.0, 106.0, 26.0, 20.0)],
        flood: 0.3,
        relief: 0.6,
        seed: 83,
        label: false,
    },
    PlateDef {
        name: L("タリム地塊", "Tarim", "塔里木板块"),
        now: L("中国・新疆", "Xinjiang, China", "中国新疆"),
        outline: &[(-4.0, 2.0), (3.0, 3.0), (5.0, -1.0), (-1.0, -3.0), (-5.0, -1.0)],
        keys: &[(540.0, 112.0, 2.0, 0.0), (485.0, 116.0, 8.0, 10.0)],
        flood: 0.4,
        relief: 0.6,
        seed: 97,
        label: false,
    },
];

/// 化石の産地（観察地点）
pub struct SiteDef {
    pub name: L,
    /// いまの場所
    pub now: L,
    pub plate: usize,
    /// 大陸の上の位置（自分の座標、度）。近くの、ふさわしい深さの場所に置き直す
    pub at: (f32, f32),
    pub want: Habitat,
    /// 生きものが暮らしていた時代（百万年前、古い方から）
    pub t0: f32,
    pub t1: f32,
}

pub const S_FORTUNE: usize = 0;
#[allow(dead_code)]
pub const S_SIBERIA: usize = 1;
pub const S_CHENGJIANG: usize = 2;
pub const S_SIRIUS: usize = 3;
pub const S_EMU: usize = 4;
pub const S_KALKARINDJI: usize = 5;
pub const S_BURGESS: usize = 6;
pub const S_AVALON: usize = 7;
pub const S_ALUM: usize = 8;
pub const S_TIDAL: usize = 9;

pub static SITES: &[SiteDef] = &[
    SiteDef { name: L("フォーチュン岬", "Fortune Head", "福琼角"), now: L("カナダ・ニューファンドランド島", "Newfoundland, Canada", "加拿大纽芬兰岛"), plate: AVALONIA, at: (-9.0, 4.0), want: Habitat::Shallow, t0: 538.8, t1: 529.0 },
    SiteDef { name: L("シベリアの礁", "Siberian reefs", "西伯利亚礁"), now: L("ロシア・レナ川流域", "Lena River, Russia", "俄罗斯勒拿河流域"), plate: SIBERIA, at: (0.0, 0.0), want: Habitat::Shallow, t0: 525.0, t1: 510.0 },
    SiteDef { name: L("澄江（チェンジャン）", "Chengjiang", "澄江"), now: L("中国・雲南省", "Yunnan, China", "中国云南省"), plate: SOUTH_CHINA, at: (-2.0, -7.5), want: Habitat::Shelf, t0: 520.0, t1: 515.0 },
    SiteDef { name: L("シリウス・パセット", "Sirius Passet", "天狼星帕塞特"), now: L("グリーンランド北部", "North Greenland", "格陵兰北部"), plate: LAURENTIA, at: (31.0, 10.0), want: Habitat::Shelf, t0: 520.0, t1: 514.0 },
    SiteDef { name: L("エミュー湾", "Emu Bay", "鸸鹋湾"), now: L("オーストラリア・カンガルー島", "Kangaroo Island, Australia", "澳大利亚袋鼠岛"), plate: GONDWANA, at: (80.0, 30.0), want: Habitat::Shallow, t0: 515.0, t1: 510.0 },
    SiteDef { name: L("カルカリンジ火山地帯", "Kalkarindji volcanic province", "卡尔卡林吉火山区"), now: L("オーストラリア北部", "Northern Australia", "澳大利亚北部"), plate: GONDWANA, at: (56.0, 42.0), want: Habitat::Land, t0: 511.5, t1: 510.0 },
    SiteDef { name: L("バージェス", "Burgess", "伯吉斯"), now: L("カナダ・ブリティッシュコロンビア州", "British Columbia, Canada", "加拿大不列颠哥伦比亚省"), plate: LAURENTIA, at: (-6.0, 21.5), want: Habitat::Shelf, t0: 510.0, t1: 505.0 },
    SiteDef { name: L("アヴァロニアの海", "Avalonian sea", "阿瓦隆尼亚海"), now: L("イギリス・ウェールズ", "Wales, Britain", "英国威尔士"), plate: AVALONIA, at: (5.0, -4.0), want: Habitat::Shelf, t0: 506.0, t1: 497.0 },
    SiteDef { name: L("ミョウバン頁岩の海", "Alum Shale sea", "明矾页岩海"), now: L("スウェーデン南部", "Southern Sweden", "瑞典南部"), plate: BALTICA, at: (0.0, 0.0), want: Habitat::Shelf, t0: 500.0, t1: 486.0 },
    SiteDef { name: L("ウィスコンシンの干潟", "Wisconsin tidal flats", "威斯康星潮坪"), now: L("アメリカ・ウィスコンシン州", "Wisconsin, USA", "美国威斯康星州"), plate: LAURENTIA, at: (6.0, -19.5), want: Habitat::Tidal, t0: 505.0, t1: 490.0 },
];

/// 地球史の節目（時間旅行でこの時を越えると知らせる）
pub struct Milestone {
    pub ma: f32,
    pub title: L,
    pub text: L,
}

pub static MILESTONES: &[Milestone] = &[
    Milestone {
        ma: 538.8,
        title: L("カンブリア紀のはじまり", "The Cambrian begins", "寒武纪开始"),
        text: L(
            "古生代の最初の時代。海の底に、動物が掘った複雑な巣穴（生痕）が現れはじめた。",
            "The first period of the Paleozoic. Complex burrows dug by animals begin to appear on the sea floor.",
            "古生代的第一个纪。海底开始出现动物挖掘的复杂潜穴（遗迹化石）。",
        ),
    },
    Milestone {
        ma: 529.0,
        title: L("第2期：殻を持つ動物がふえる", "Stage 2: shelled animals spread", "第二期：带壳动物增多"),
        text: L("小さな殻や管（微小有殻化石）が世界の浅い海に広がる。", "Tiny shells and tubes (small shelly fossils) spread through shallow seas worldwide.", "微小的壳和管（小壳化石）遍布全球浅海。"),
    },
    Milestone {
        ma: 521.0,
        title: L("第3期：三葉虫の登場とカンブリア爆発", "Stage 3: trilobites and the Cambrian explosion", "第三期：三叶虫登场与寒武纪大爆发"),
        text: L(
            "眼を持つ三葉虫が現れ、多くの動物のグループが短い間に一気に姿を見せる。",
            "Trilobites with eyes appear, and many animal groups show up in a geologically short time.",
            "长着眼睛的三叶虫出现，许多动物类群在短时间内纷纷登场。",
        ),
    },
    Milestone {
        ma: 514.0,
        title: L("第4期：シンスク事変", "Stage 4: the Sinsk event", "第四期：辛斯克事件"),
        text: L("海の酸素が減る出来事が起こり、古杯類の礁など浅い海の生きものが打撃を受けた。", "Oxygen in the sea drops, striking shallow-sea life such as archaeocyath reefs.", "海洋缺氧事件发生，古杯类礁等浅海生物遭受打击。"),
    },
    Milestone {
        ma: 511.5,
        title: L("カルカリンジの巨大噴火", "The Kalkarindji eruptions", "卡尔卡林吉大喷发"),
        text: L(
            "ゴンドワナ大陸の北（いまのオーストラリア）で、大量の溶岩が噴き出す。地球の環境が大きく揺さぶられた。",
            "Huge volumes of lava pour out in northern Gondwana (now Australia), shaking the global environment.",
            "冈瓦纳大陆北部（今澳大利亚）喷出大量熔岩，全球环境剧烈动荡。",
        ),
    },
    Milestone {
        ma: 509.0,
        title: L("ミャオリンギアン世：古杯類が姿を消す", "Miaolingian: archaeocyaths vanish", "苗岭世：古杯类消失"),
        text: L("礁をつくった古杯類はほとんど絶滅した。バージェスの海には不思議な動物たちが暮らす。", "The reef-building archaeocyaths are nearly gone. Strange animals live in the Burgess sea.", "造礁的古杯类几乎灭绝。伯吉斯海中生活着奇异的动物。"),
    },
    Milestone {
        ma: 497.0,
        title: L("芙蓉世：SPICE事変", "Furongian: the SPICE event", "芙蓉世：SPICE事件"),
        text: L("海の炭素の循環が大きく乱れ、三葉虫の仲間が入れ替わる。", "The ocean's carbon cycle is disrupted and trilobite groups turn over.", "海洋碳循环剧烈波动，三叶虫类群更替。"),
    },
    Milestone {
        ma: 486.0,
        title: L("カンブリア紀の終わり", "The end of the Cambrian", "寒武纪结束"),
        text: L("カンブリア紀の終わりにも絶滅があり、次のオルドビス紀へ続く。", "Extinctions mark the end of the Cambrian, leading into the Ordovician.", "寒武纪末期也发生了灭绝，随后进入奥陶纪。"),
    },
];

/// 時代の区分（世・期）
pub struct Stage {
    pub t0: f32,
    pub t1: f32,
    pub name: L,
    pub col: [f32; 3],
}

pub static STAGES: &[Stage] = &[
    Stage { t0: 538.8, t1: 529.0, name: L("フォーチュニアン期", "Fortunian", "幸运期"), col: [0.42, 0.64, 0.48] },
    Stage { t0: 529.0, t1: 521.0, name: L("第2期", "Stage 2", "第二期"), col: [0.48, 0.70, 0.52] },
    Stage { t0: 521.0, t1: 514.0, name: L("第3期", "Stage 3", "第三期"), col: [0.55, 0.74, 0.56] },
    Stage { t0: 514.0, t1: 509.0, name: L("第4期", "Stage 4", "第四期"), col: [0.61, 0.78, 0.60] },
    Stage { t0: 509.0, t1: 504.5, name: L("ウーリューアン期", "Wuliuan", "乌溜期"), col: [0.66, 0.80, 0.62] },
    Stage { t0: 504.5, t1: 500.5, name: L("ドラミアン期", "Drumian", "鼓山期"), col: [0.70, 0.82, 0.66] },
    Stage { t0: 500.5, t1: 497.0, name: L("ガズハンギアン期", "Guzhangian", "古丈期"), col: [0.74, 0.85, 0.70] },
    Stage { t0: 497.0, t1: 494.2, name: L("ペイビアン期", "Paibian", "排碧期"), col: [0.78, 0.87, 0.73] },
    Stage { t0: 494.2, t1: 489.5, name: L("ジャンシャニアン期", "Jiangshanian", "江山期"), col: [0.82, 0.89, 0.77] },
    Stage { t0: 489.5, t1: 485.4, name: L("第10期", "Stage 10", "第十期"), col: [0.86, 0.92, 0.81] },
];

pub fn stage_at(t: f32) -> &'static Stage {
    STAGES.iter().find(|s| t <= s.t0 + 1e-3 && t > s.t1 - 1e-3).unwrap_or(&STAGES[STAGES.len() - 1])
}

/// 「5億1800万年前」のような表記
pub fn ma_text(t: f32) -> String {
    let m = (t * 10.0).round() / 10.0;
    let oku = (m / 100.0).floor() as i32;
    let man = m - oku as f32 * 100.0;
    match crate::lang::lang() {
        crate::lang::Lang::En => format!("{:.1} million years ago", m),
        crate::lang::Lang::Ja => {
            if (man - man.round()).abs() < 0.05 {
                format!("{}億{}00万年前", oku, man.round() as i32)
            } else {
                format!("{}億{}0万年前", oku, (man * 10.0).round() as i32)
            }
        }
        crate::lang::Lang::Zh => {
            if (man - man.round()).abs() < 0.05 {
                format!("{}亿{}00万年前", oku, man.round() as i32)
            } else {
                format!("{}亿{}0万年前", oku, (man * 10.0).round() as i32)
            }
        }
    }
}

/// 時代区分の名前：古生代 カンブリア紀 第3期
pub fn era_text(t: f32) -> String {
    tf!("古生代 カンブリア紀・{}", "Paleozoic · Cambrian · {}", "古生代 寒武纪·{}", stage_at(t).name.get())
}

pub struct Plate {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub w: usize,
    pub h: usize,
    /// 高さ（> 0 が陸。海は負で、-1 で大陸棚の縁、-1.6 より深いと外洋）
    pub e: Vec<f32>,
    /// 色（RGBA、非乗算アルファ）
    pub px: Vec<u32>,
}

pub struct Site {
    /// 置き直した位置（自分の座標）
    pub at: (f32, f32),
}

pub struct Earth {
    pub plates: Vec<Plate>,
    pub sites: Vec<Site>,
}

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct Spot {
    pub e: f32,
    pub hab: Habitat,
    /// 水深（m）。陸は 0
    pub depth: f32,
    pub plate: Option<usize>,
    pub lon: f32,
    pub lat: f32,
}

fn seg_dist(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l2 = dx * dx + dy * dy;
    let t = if l2 > 0.0 { (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / l2).clamp(0.0, 1.0) } else { 0.0 };
    let (qx, qy) = (a.0 + dx * t - p.0, a.1 + dy * t - p.1);
    (qx * qx + qy * qy).sqrt()
}

/// 多角形までの符号付き距離（内側が正）
fn signed_dist(p: (f32, f32), poly: &[(f32, f32)]) -> f32 {
    let mut d = f32::MAX;
    let mut inside = false;
    let n = poly.len();
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        d = d.min(seg_dist(p, a, b));
        if (a.1 > p.1) != (b.1 > p.1) && p.0 < (b.0 - a.0) * (p.1 - a.1) / (b.1 - a.1) + a.0 {
            inside = !inside;
        }
    }
    if inside {
        d
    } else {
        -d
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn lerp3(a: [f32; 3], b: [f32; 3], t: f32) -> [f32; 3] {
    [lerp(a[0], b[0], t), lerp(a[1], b[1], t), lerp(a[2], b[2], t)]
}

pub fn rgba(c: [f32; 3], a: f32) -> u32 {
    let r = (c[0].clamp(0.0, 1.0) * 255.0) as u32;
    let g = (c[1].clamp(0.0, 1.0) * 255.0) as u32;
    let b = (c[2].clamp(0.0, 1.0) * 255.0) as u32;
    let a = (a.clamp(0.0, 1.0) * 255.0) as u32;
    r | (g << 8) | (b << 16) | (a << 24)
}

/// 高さから色を決める（陸は植物のない赤茶けた岩、海岸は微生物マット、浅い海は明るい青緑）
pub fn color_of(e: f32, n: f32, shade: f32) -> ([f32; 3], f32) {
    if e > 0.0 {
        if e < 0.035 {
            // 干潟・微生物マット
            return (lerp3([0.46, 0.46, 0.30], [0.52, 0.50, 0.34], n), 1.0);
        }
        let k = ((e - 0.035) / 0.9).clamp(0.0, 1.0);
        let low = lerp3([0.56, 0.38, 0.27], [0.78, 0.60, 0.42], n);
        let high = lerp3([0.46, 0.38, 0.35], [0.64, 0.58, 0.54], n);
        let mut c = lerp3(low, high, k);
        if e > 1.0 {
            c = lerp3(c, [0.74, 0.70, 0.66], ((e - 1.0) * 1.5).clamp(0.0, 0.6));
        }
        let s = 0.72 + 0.5 * shade;
        return ([c[0] * s, c[1] * s, c[2] * s], 1.0);
    }
    if e > -0.2 {
        let k = (-e / 0.2).clamp(0.0, 1.0);
        return (lerp3([0.42, 0.80, 0.76], [0.20, 0.62, 0.70], k), 0.95);
    }
    if e > -1.0 {
        let k = ((-e - 0.2) / 0.8).clamp(0.0, 1.0);
        return (lerp3([0.20, 0.62, 0.70], [0.10, 0.36, 0.56], k), 0.92);
    }
    if e > -1.7 {
        let k = ((-e - 1.0) / 0.7).clamp(0.0, 1.0);
        return (lerp3([0.10, 0.36, 0.56], [0.06, 0.18, 0.36], k), 0.9 * (1.0 - k));
    }
    ([0.06, 0.18, 0.36], 0.0)
}

impl Earth {
    pub fn new() -> Earth {
        let mut plates = Vec::new();
        for d in PLATES {
            plates.push(Self::build_plate(d));
        }
        let mut e = Earth { plates, sites: Vec::new() };
        for s in SITES {
            let at = e.place_site(s);
            e.sites.push(Site { at });
        }
        e
    }

    fn build_plate(d: &PlateDef) -> Plate {
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for &(x, y) in d.outline {
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y);
        }
        let m = 15.0;
        let (x0, y0, x1, y1) = (x0 - m, y0 - m, x1 + m, y1 + m);
        let w = ((x1 - x0) * RES).ceil() as usize;
        let h = ((y1 - y0) * RES).ceil() as usize;
        let mut e = vec![0f32; w * h];
        let sd = d.seed;
        for j in 0..h {
            // 行 0 が北（y1）
            let ly = y1 - (j as f32 + 0.5) / RES;
            for i in 0..w {
                let lx = x0 + (i as f32 + 0.5) / RES;
                // 海岸線を不規則にする（大きな湾・半島から細かな出入りまで）
                let wob = (fbm(lx * 0.045, ly * 0.045, sd, 4) - 0.5) * 26.0 + (fbm(lx * 0.16, ly * 0.16, sd + 5, 4) - 0.5) * 9.0 + (fbm(lx * 0.6, ly * 0.6, sd + 9, 3) - 0.5) * 2.5;
                let raw = signed_dist((lx, ly), d.outline) + wob;
                let v = if raw > 0.0 {
                    let mut v = 0.03 + (raw.min(12.0) / 12.0) * 0.32;
                    // 山脈（尾根状のノイズを、山のある地域だけに）
                    let ridge = (1.0 - ((fbm(lx * 0.09, ly * 0.09, sd + 17, 4) - 0.5) * 5.0).abs()).max(0.0);
                    let mask = ((fbm(lx * 0.03, ly * 0.03, sd + 23, 3) - 0.45) * 4.0).clamp(0.0, 1.0);
                    v += ridge.powf(3.0) * mask * 1.1 * d.relief * (raw / 3.0).clamp(0.0, 1.0);
                    v += (fbm(lx * 0.5, ly * 0.5, sd + 29, 3) - 0.5) * 0.14;
                    // 内陸の浅い海（大陸の内側まで海が入り込む）
                    let f = (fbm(lx * 0.06, ly * 0.06, sd + 41, 4) - 0.5) * 2.6 + 0.5;
                    let lvl = d.flood;
                    if f < lvl && raw > 0.8 {
                        let k = ((lvl - f) * 12.0).clamp(0.0, 1.0);
                        v = v + (-0.04 - (lvl - f) * 0.9 - v) * k;
                    }
                    v.max(-0.7)
                } else {
                    let wid = 2.6 + fbm(lx * 0.08, ly * 0.08, sd + 51, 3) * 3.5;
                    (raw / wid).max(-3.0)
                };
                e[j * w + i] = v;
            }
        }
        // 陰影をつけて色を塗る
        let mut px = vec![0u32; w * h];
        for j in 0..h {
            for i in 0..w {
                let v = e[j * w + i];
                let vx = e[j * w + (i + 1).min(w - 1)] - e[j * w + i.saturating_sub(1)];
                let vy = e[(j + 1).min(h - 1) * w + i] - e[j.saturating_sub(1) * w + i];
                let shade = (0.5 - (vx - vy) * 3.0).clamp(0.0, 1.0);
                let lx = x0 + (i as f32 + 0.5) / RES;
                let ly = y1 - (j as f32 + 0.5) / RES;
                let n = ((fbm(lx * 0.25, ly * 0.25, sd + 61, 4) - 0.5) * 2.2 + 0.5).clamp(0.0, 1.0);
                let (c, a) = color_of(v, n, shade);
                px[j * w + i] = rgba(c, a);
            }
        }
        Plate { x0, y0, x1, y1, w, h, e, px }
    }

    /// 観察地点を、近くのふさわしい深さの場所に置き直す
    fn place_site(&self, s: &SiteDef) -> (f32, f32) {
        let p = &self.plates[s.plate];
        let mut best = (f32::MAX, s.at);
        let want = s.want;
        let r = 18.0;
        let mut dy = -r;
        while dy <= r {
            let mut dx = -r;
            while dx <= r {
                let (lx, ly) = (s.at.0 + dx, s.at.1 + dy);
                if let Some(e) = p.sample(lx, ly) {
                    let hab = hab_of(e).0;
                    // 周りも同じ環境か（端の1点だけ当たらないように）
                    let nb = if want == Habitat::Tidal { 0.0 } else { 0.5 };
                    let ok = hab == want && [(nb, 0.0), (-nb, 0.0), (0.0, nb), (0.0, -nb)].iter().all(|&(ox, oy)| p.sample(lx + ox, ly + oy).map_or(false, |e2| hab_of(e2).0 == want));
                    if ok {
                        let d = dx * dx + dy * dy;
                        if d < best.0 {
                            best = (d, (lx, ly));
                        }
                    }
                }
                dx += 0.25;
            }
            dy += 0.25;
        }
        best.1
    }

    /// 大陸の中心位置と回転（度）
    pub fn pose(&self, i: usize, t: f32) -> (f32, f32, f32) {
        let k = PLATES[i].keys;
        if t >= k[0].0 {
            return (k[0].1, k[0].2, k[0].3);
        }
        for w in k.windows(2) {
            let (a, b) = (w[0], w[1]);
            if t <= a.0 && t >= b.0 {
                let f = (a.0 - t) / (a.0 - b.0);
                return (lerp(a.1, b.1, f), lerp(a.2, b.2, f), lerp(a.3, b.3, f));
            }
        }
        let l = k[k.len() - 1];
        (l.1, l.2, l.3)
    }

    /// 大陸の座標 → 経緯度
    pub fn to_world(&self, i: usize, t: f32, l: (f32, f32)) -> (f32, f32) {
        let (cx, cy, r) = self.pose(i, t);
        let (s, c) = r.to_radians().sin_cos();
        (cx + l.0 * c - l.1 * s, cy + l.0 * s + l.1 * c)
    }

    /// 経緯度 → 大陸の座標
    pub fn to_local(&self, i: usize, t: f32, lon: f32, lat: f32) -> (f32, f32) {
        let (cx, cy, r) = self.pose(i, t);
        let (s, c) = r.to_radians().sin_cos();
        let (dx, dy) = (lon - cx, lat - cy);
        (dx * c + dy * s, -dx * s + dy * c)
    }

    /// その場所の高さと環境
    pub fn sample(&self, lon: f32, lat: f32, t: f32) -> Spot {
        let mut best = (-3.0f32, None);
        for (i, p) in self.plates.iter().enumerate() {
            let (lx, ly) = self.to_local(i, t, lon, lat);
            if let Some(e) = p.sample(lx, ly) {
                if e > best.0 {
                    best = (e, Some(i));
                }
            }
        }
        let (hab, depth) = hab_of(best.0);
        Spot { e: best.0, hab, depth, plate: best.1, lon, lat }
    }

    pub fn site_lonlat(&self, s: usize, t: f32) -> (f32, f32) {
        self.to_world(SITES[s].plate, t, self.sites[s].at)
    }

    /// 地図上の大陸の四隅（地図座標、左上・右上・右下・左下）
    pub fn plate_quad(&self, i: usize, t: f32) -> [(f32, f32); 4] {
        let p = &self.plates[i];
        let c = [(p.x0, p.y1), (p.x1, p.y1), (p.x1, p.y0), (p.x0, p.y0)];
        c.map(|l| {
            let (lon, lat) = self.to_world(i, t, l);
            lonlat_to_map(lon, lat)
        })
    }

    /// 大陸の陸地を囲む範囲（地図座標）
    pub fn plate_bounds(&self, i: usize, t: f32) -> (f32, f32, f32, f32) {
        let d = &PLATES[i];
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for &l in d.outline {
            let (lon, lat) = self.to_world(i, t, l);
            let (x, y) = lonlat_to_map(lon, lat);
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x);
            y1 = y1.max(y);
        }
        (x0, y0, x1, y1)
    }
}

impl Plate {
    /// 自分の座標で高さを引く（範囲外なら None）
    pub fn sample(&self, lx: f32, ly: f32) -> Option<f32> {
        let fx = (lx - self.x0) * RES - 0.5;
        let fy = (self.y1 - ly) * RES - 0.5;
        if fx < 0.0 || fy < 0.0 || fx >= (self.w - 1) as f32 || fy >= (self.h - 1) as f32 {
            return None;
        }
        let (i, j) = (fx as usize, fy as usize);
        let (tx, ty) = (fx - i as f32, fy - j as f32);
        let w = self.w;
        let a = self.e[j * w + i];
        let b = self.e[j * w + i + 1];
        let c = self.e[(j + 1) * w + i];
        let d = self.e[(j + 1) * w + i + 1];
        Some(lerp(lerp(a, b, tx), lerp(c, d, tx), ty))
    }
}

/// 高さ → 環境と水深（m）
pub fn hab_of(e: f32) -> (Habitat, f32) {
    if e > 0.035 {
        (Habitat::Land, 0.0)
    } else if e > 0.0 {
        (Habitat::Tidal, 0.6)
    } else if e > -0.2 {
        (Habitat::Shallow, 2.0 + (-e / 0.2) * 28.0)
    } else if e > -1.0 {
        (Habitat::Shelf, 30.0 + (-e - 0.2) / 0.8 * 170.0)
    } else if e > -1.7 {
        (Habitat::Deep, 200.0 + (-e - 1.0) / 0.7 * 2800.0)
    } else {
        (Habitat::Deep, 3000.0 + ((-e - 1.7) * 800.0).min(1200.0))
    }
}

pub fn lonlat_to_map(lon: f32, lat: f32) -> (f32, f32) {
    ((lon + 180.0) * DEG, (90.0 - lat) * DEG)
}

pub fn map_to_lonlat(x: f32, y: f32) -> (f32, f32) {
    (x / DEG - 180.0, 90.0 - y / DEG)
}

/// 二点間のおおよその距離（度）
pub fn deg_dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    let mlat = ((a.1 + b.1) * 0.5).to_radians().cos().max(0.2);
    let dx = (a.0 - b.0) * mlat;
    let dy = a.1 - b.1;
    (dx * dx + dy * dy).sqrt()
}

/// 外洋の色（地図の背景）
pub fn ocean_texture() -> (u32, u32, Vec<u32>) {
    let (w, h) = (720u32, 360u32);
    let mut px = vec![0u32; (w * h) as usize];
    for j in 0..h {
        for i in 0..w {
            let n = fbm(i as f32 * 0.02, j as f32 * 0.02, 5, 5);
            let n2 = fbm(i as f32 * 0.08, j as f32 * 0.08, 9, 3);
            let lat = 90.0 - (j as f32 + 0.5) * 180.0 / h as f32;
            // 赤道ほど少し明るい（暖かい海）
            let warm = (1.0 - (lat.abs() / 70.0)).clamp(0.0, 1.0);
            let c = [0.05 + 0.03 * n + 0.015 * warm, 0.15 + 0.06 * n + 0.04 * warm + 0.02 * n2, 0.32 + 0.08 * n + 0.04 * warm];
            px[(j * w + i) as usize] = rgba(c, 1.0);
        }
    }
    (w, h, px)
}

/// 雲（白、透明度で形を作る）。横につなげて流す
pub fn cloud_texture() -> (u32, u32, Vec<u32>) {
    let (w, h) = (512u32, 256u32);
    let mut px = vec![0u32; (w * h) as usize];
    for j in 0..h {
        for i in 0..w {
            // 横方向は周期的にする（つなぎ目を出さない）
            let a = i as f32 / w as f32 * std::f32::consts::TAU;
            let (sx, cx) = a.sin_cos();
            let y = j as f32 * 0.035;
            let n = fbm(cx * 2.6 + 10.0, y + sx * 2.6, 77, 5) * 0.65 + fbm(sx * 5.0 + 3.0, y * 2.0 + cx * 5.0, 91, 4) * 0.35;
            let lat = 90.0 - (j as f32 + 0.5) * 180.0 / h as f32;
            // 赤道付近と中緯度に雲が多い
            let band = 0.55 + 0.25 * (lat.to_radians() * 3.0).cos().abs();
            let v = ((n - 0.52) * 3.2 * band).clamp(0.0, 0.85);
            px[(j * w + i) as usize] = rgba([1.0, 1.0, 1.0], v);
        }
    }
    (w, h, px)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sites_land_in_wanted_habitat() {
        let e = Earth::new();
        for (i, s) in SITES.iter().enumerate() {
            let t = (s.t0 + s.t1) * 0.5;
            let (lon, lat) = e.site_lonlat(i, t);
            let sp = e.sample(lon, lat, t);
            assert_eq!(sp.hab, s.want, "site {} ({}) at {:.1},{:.1} is {:?}", i, s.name.1, lon, lat, sp.hab);
        }
    }

    #[test]
    fn stage_lookup() {
        assert_eq!(stage_at(538.8).name.1, "Fortunian");
        assert_eq!(stage_at(518.0).name.1, "Stage 3");
        assert_eq!(stage_at(485.4).name.1, "Stage 10");
    }

    /// 大陸どうしが大きく重ならない
    #[test]
    fn plates_do_not_overlap_much() {
        let e = Earth::new();
        for &t in &[538.8, 520.0, 505.0, 485.4] {
            let mut overlap = 0;
            let mut land = 0;
            let mut lat = -88.0;
            while lat < 88.0 {
                let mut lon = -179.0;
                while lon < 180.0 {
                    let mut n = 0;
                    for (i, p) in e.plates.iter().enumerate() {
                        let (lx, ly) = e.to_local(i, t, lon, lat);
                        if p.sample(lx, ly).map_or(false, |v| v > 0.035) {
                            n += 1;
                        }
                    }
                    if n > 0 {
                        land += 1;
                    }
                    if n > 1 {
                        overlap += 1;
                    }
                    lon += 1.0;
                }
                lat += 1.0;
            }
            assert!(overlap * 100 < land, "t={} overlap {} / land {}", t, overlap, land);
        }
    }

    #[test]
    fn ma_text_formats() {
        crate::lang::set_lang(crate::lang::Lang::Ja);
        assert_eq!(ma_text(518.0), "5億1800万年前");
        assert_eq!(ma_text(538.8), "5億3880万年前");
    }
}

#[cfg(test)]
mod dump {
    use super::*;

    /// 地図を画像に書き出す（目で確かめる用）：cargo test --lib dump_map -- --ignored
    #[test]
    #[ignore]
    fn dump_map() {
        let e = Earth::new();
        let dir = std::env::var("YY_DUMP").unwrap_or("/tmp".into());
        for &t in &[538.0, 518.0, 508.0, 486.0] {
            let (w, h) = (720usize, 360usize);
            let mut out = format!("P3\n{} {}\n255\n", w, h).into_bytes();
            for j in 0..h {
                for i in 0..w {
                    let lon = -180.0 + (i as f32 + 0.5) / 2.0;
                    let lat = 90.0 - (j as f32 + 0.5) / 2.0;
                    let s = e.sample(lon, lat, t);
                    let (c, a) = color_of(s.e, 0.5, 0.5);
                    let bg = [0.06, 0.18, 0.36];
                    let c = [bg[0] + (c[0] - bg[0]) * a, bg[1] + (c[1] - bg[1]) * a, bg[2] + (c[2] - bg[2]) * a];
                    let mut px = [(c[0] * 255.0) as u8, (c[1] * 255.0) as u8, (c[2] * 255.0) as u8];
                    for k in 0..SITES.len() {
                        let (sl, sa) = e.site_lonlat(k, t);
                        if (sl - lon).abs() < 0.8 && (sa - lat).abs() < 0.8 {
                            px = [255, 40, 40];
                        }
                    }
                    out.extend_from_slice(format!("{} {} {} ", px[0], px[1], px[2]).as_bytes());
                }
            }
            std::fs::write(format!("{}/map_{}.ppm", dir, t as i32), out).unwrap();
        }
        for (k, s) in SITES.iter().enumerate() {
            let t = (s.t0 + s.t1) * 0.5;
            let (lon, lat) = e.site_lonlat(k, t);
            let sp = e.sample(lon, lat, t);
            println!("site {} {} at {:.1},{:.1} local {:?} -> {:?} plate {:?} want {:?}", k, s.name.1, lon, lat, e.sites[k].at, sp.hab, sp.plate, s.want);
        }
    }
}
