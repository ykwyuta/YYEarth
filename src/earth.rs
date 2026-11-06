//! 地球：古生代（カンブリア紀〜ペルム紀）の大陸（プレート）の形・位置・移動、地形の高さ、化石産地（観察地点）、大きな出来事
//!
//! 大陸はそれぞれ「自分の座標（経緯度の差、度）」で輪郭と高さを持ち、時代ごとの中心位置・回転（キーフレーム）で
//! 地図の上を動く。地図は正距円筒図法（経度 -180〜180、緯度 90〜-90）で、1度＝10 単位。
//! 位置・形はおおよそ（Scotese・Torsvik などの古地理図を参考にした、ゲーム用の簡略化）。

use crate::lang::L;
use crate::rng::fbm;

pub const DEG: f32 = 10.0;
pub const MAP_W: f32 = 360.0 * DEG;
pub const MAP_H: f32 = 180.0 * DEG;

/// 古生代の始まり（カンブリア紀の始まり）と終わり（ペルム紀の終わり）（百万年前）
pub const T_START: f32 = 538.8;
pub const T_END: f32 = 251.9;

/// 紀（図鑑の巻）
pub struct Period {
    pub name: L,
    pub t0: f32,
    pub t1: f32,
    pub col: [f32; 3],
}

pub const CAMBRIAN: usize = 0;
pub const ORDOVICIAN: usize = 1;
pub const SILURIAN: usize = 2;
pub const DEVONIAN: usize = 3;
pub const CARBONIFEROUS: usize = 4;
pub const PERMIAN: usize = 5;

pub static PERIODS: &[Period] = &[
    Period { name: L("カンブリア紀", "Cambrian", "寒武纪"), t0: 538.8, t1: 485.4, col: [0.50, 0.72, 0.52] },
    Period { name: L("オルドビス紀", "Ordovician", "奥陶纪"), t0: 485.4, t1: 443.8, col: [0.10, 0.62, 0.50] },
    Period { name: L("シルル紀", "Silurian", "志留纪"), t0: 443.8, t1: 419.2, col: [0.62, 0.84, 0.74] },
    Period { name: L("デボン紀", "Devonian", "泥盆纪"), t0: 419.2, t1: 358.9, col: [0.80, 0.56, 0.26] },
    Period { name: L("石炭紀", "Carboniferous", "石炭纪"), t0: 358.9, t1: 298.9, col: [0.40, 0.64, 0.62] },
    Period { name: L("ペルム紀", "Permian", "二叠纪"), t0: 298.9, t1: 251.9, col: [0.90, 0.36, 0.26] },
];

/// その時代の紀の番号
pub fn period_at(t: f32) -> usize {
    PERIODS.iter().position(|p| t <= p.t0 + 1e-3 && t > p.t1 - 1e-3).unwrap_or(if t > T_START { 0 } else { PERIODS.len() - 1 })
}

/// 陸の植物の広がり（0：植物なし〜1：森）。シルル紀に岸辺から緑が広がり、デボン紀の後半に森ができる
pub fn veg_at(t: f32) -> f32 {
    let k = |a: f32, b: f32| ((a - t) / (a - b)).clamp(0.0, 1.0);
    if t >= 433.0 {
        0.0
    } else if t >= 410.0 {
        k(433.0, 410.0) * 0.25
    } else if t >= 385.0 {
        0.25 + k(410.0, 385.0) * 0.35
    } else {
        0.6 + k(385.0, 370.0) * 0.4
    }
}

/// 氷河時代（オルドビス紀末のヒルナンシアン氷期、石炭紀〜ペルム紀前期の後期古生代氷河時代）
pub fn glacial(t: f32) -> bool {
    (443.8..=445.4).contains(&t) || (285.0..=340.0).contains(&t)
}

/// 乾いた砂漠の広がる時代と場所（ペルム紀のパンゲアの内陸）
pub fn arid(t: f32, lat: f32) -> bool {
    t <= 292.0 && lat.abs() < 34.0
}

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
    /// (相手の大陸, 百万年前)：この時にぶつかり、それより新しい時代は相手と一つになって動く
    pub attach: Option<(usize, f32)>,
    /// 内陸が浅い海に沈む度合い（古生代の前半は海面が高く、大陸の内側まで海が入り込んだ）
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
pub const KAZAKH: usize = 7;
pub const TARIM: usize = 8;

// キーフレームは古い方から。`attach` のある大陸は、その時からぶつかった相手と一緒に動く
// （アヴァロニア→バルティカ→ローレンシアで「ユーラメリカ（ローラシア）」、ゴンドワナがぶつかって「パンゲア」）。
pub static PLATES: &[PlateDef] = &[
    PlateDef {
        name: L("ゴンドワナ大陸", "Gondwana", "冈瓦纳大陆"),
        now: L("アフリカ・南アメリカ・南極・オーストラリア・インドなど", "Africa, South America, Antarctica, Australia, India and more", "非洲、南美洲、南极洲、澳大利亚、印度等"),
        outline: &[
            (-62.0, 20.0), (-50.0, 32.0), (-35.0, 38.0), (-20.0, 40.0), (-5.0, 46.0), (10.0, 50.0), (25.0, 52.0), (40.0, 55.0), (55.0, 54.0), (68.0, 48.0),
            (78.0, 38.0), (80.0, 25.0), (72.0, 12.0), (62.0, 0.0), (55.0, -12.0), (45.0, -25.0), (30.0, -38.0), (10.0, -44.0), (-10.0, -46.0), (-30.0, -42.0),
            (-48.0, -34.0), (-60.0, -22.0), (-68.0, -8.0), (-70.0, 6.0),
        ],
        keys: &[
            (540.0, 40.0, -44.0, -8.0), (510.0, 44.0, -40.0, 0.0), (485.0, 48.0, -37.0, 6.0), (470.0, 46.0, -40.0, 8.0), (445.0, 42.0, -44.0, 10.0),
            (420.0, 36.0, -46.0, 10.0), (400.0, 31.0, -46.0, 8.0), (380.0, 27.0, -46.0, 6.0), (360.0, 23.0, -45.0, 4.0), (340.0, 19.0, -44.0, 2.0),
            (320.0, 16.0, -42.0, 1.0), (305.0, 14.0, -41.0, 0.0),
        ],
        attach: Some((LAURENTIA, 305.0)),
        flood: 0.2,
        relief: 1.0,
        seed: 11,
        label: true,
    },
    PlateDef {
        name: L("ローレンシア大陸", "Laurentia", "劳伦大陆"),
        now: L("北アメリカ・グリーンランド・スコットランド", "North America, Greenland and Scotland", "北美洲、格陵兰和苏格兰"),
        outline: &[
            (-28.0, 14.0), (-15.0, 20.0), (0.0, 21.0), (14.0, 18.0), (26.0, 13.0), (31.0, 4.0), (27.0, -7.0), (18.0, -16.0), (5.0, -20.0), (-8.0, -19.0),
            (-20.0, -14.0), (-28.0, -4.0), (-31.0, 6.0),
        ],
        keys: &[
            (540.0, -104.0, -4.0, -10.0), (510.0, -98.0, 0.0, -2.0), (485.0, -92.0, 4.0, 6.0), (470.0, -88.0, 2.0, 10.0), (445.0, -80.0, 0.0, 14.0),
            (430.0, -72.0, 2.0, 14.0), (420.0, -66.0, 4.0, 12.0), (400.0, -58.0, 6.0, 11.0), (380.0, -50.0, 8.0, 9.0), (360.0, -42.0, 12.0, 7.0),
            (340.0, -34.0, 16.0, 5.0), (320.0, -28.0, 20.0, 3.0), (300.0, -24.0, 23.0, 1.0), (280.0, -23.0, 26.0, 0.0), (252.0, -22.0, 29.0, 0.0),
        ],
        attach: None,
        flood: 0.42,
        relief: 0.6,
        seed: 23,
        label: true,
    },
    PlateDef {
        name: L("バルティカ大陸", "Baltica", "波罗的大陆"),
        now: L("北ヨーロッパ・スカンディナヴィア・ロシア西部", "Northern Europe, Scandinavia and western Russia", "北欧、斯堪的纳维亚和俄罗斯西部"),
        outline: &[(-12.0, 9.0), (0.0, 11.0), (12.0, 7.0), (15.0, -2.0), (9.0, -10.0), (-3.0, -11.0), (-12.0, -6.0), (-15.0, 2.0)],
        keys: &[
            (540.0, -62.0, -60.0, -30.0), (510.0, -57.0, -54.0, -14.0), (485.0, -52.0, -48.0, 0.0), (470.0, -46.0, -36.0, 20.0), (445.0, -38.0, -10.0, 50.0),
            (430.0, -34.0, 14.0, 68.0), (420.0, -31.0, 28.0, 80.0),
        ],
        attach: Some((LAURENTIA, 420.0)),
        flood: 0.55,
        relief: 0.4,
        seed: 37,
        label: true,
    },
    PlateDef {
        name: L("アヴァロニア", "Avalonia", "阿瓦隆尼亚"),
        now: L("イギリス南部・ニューファンドランド東部・アメリカ北東部の一部", "Southern Britain, eastern Newfoundland and parts of New England", "英国南部、纽芬兰东部和美国东北部一部分"),
        outline: &[(-14.0, 3.0), (-4.0, 4.5), (6.0, 4.0), (14.0, 1.5), (12.0, -2.0), (2.0, -3.5), (-8.0, -3.0), (-15.0, -1.0)],
        keys: &[(540.0, -38.0, -40.0, 62.0), (510.0, -42.0, -38.0, 56.0), (485.0, -48.0, -36.0, 50.0), (470.0, -44.0, -34.0, 44.0), (445.0, -42.0, -25.0, 34.0)],
        attach: Some((BALTICA, 445.0)),
        flood: 0.35,
        relief: 0.5,
        seed: 41,
        label: true,
    },
    PlateDef {
        name: L("シベリア大陸", "Siberia", "西伯利亚大陆"),
        now: L("シベリア（ロシア）", "Siberia (Russia)", "西伯利亚（俄罗斯）"),
        outline: &[(-16.0, 10.0), (-4.0, 14.0), (10.0, 13.0), (18.0, 6.0), (17.0, -5.0), (8.0, -12.0), (-6.0, -11.0), (-15.0, -4.0)],
        keys: &[
            (540.0, 70.0, 30.0, 0.0), (510.0, 75.0, 31.0, 5.0), (485.0, 80.0, 33.0, 10.0), (445.0, 84.0, 28.0, 15.0), (400.0, 82.0, 34.0, 20.0),
            (360.0, 74.0, 44.0, 25.0), (320.0, 62.0, 52.0, 24.0), (280.0, 47.0, 55.0, 22.0),
        ],
        attach: Some((BALTICA, 280.0)),
        flood: 0.68,
        relief: 0.35,
        seed: 53,
        label: true,
    },
    PlateDef {
        name: L("南中国（揚子）地塊", "South China", "华南（扬子）板块"),
        now: L("中国南部", "Southern China", "中国南部"),
        outline: &[(-8.0, 5.0), (2.0, 7.0), (9.0, 4.0), (9.5, -3.0), (1.0, -6.0), (-7.0, -4.0)],
        keys: &[
            (540.0, 126.0, 8.0, 0.0), (510.0, 129.0, 12.0, 3.0), (485.0, 132.0, 16.0, 6.0), (445.0, 128.0, 8.0, 8.0), (400.0, 122.0, 4.0, 10.0),
            (360.0, 118.0, 4.0, 12.0), (320.0, 116.0, 8.0, 12.0), (252.0, 116.0, 14.0, 15.0),
        ],
        attach: None,
        flood: 0.5,
        relief: 0.5,
        seed: 67,
        label: true,
    },
    PlateDef {
        name: L("北中国地塊", "North China", "华北板块"),
        now: L("中国北部・朝鮮半島", "Northern China and Korea", "中国北部和朝鲜半岛"),
        outline: &[(-6.0, 4.0), (4.0, 5.0), (7.0, 0.0), (3.0, -4.0), (-5.0, -3.0)],
        keys: &[
            (540.0, 146.0, 24.0, 0.0), (510.0, 148.0, 27.0, 4.0), (485.0, 150.0, 30.0, 8.0), (445.0, 148.0, 22.0, 10.0), (400.0, 140.0, 18.0, 12.0),
            (360.0, 132.0, 20.0, 15.0), (320.0, 126.0, 24.0, 15.0), (252.0, 120.0, 30.0, 15.0),
        ],
        attach: None,
        flood: 0.5,
        relief: 0.4,
        seed: 71,
        label: false,
    },
    PlateDef {
        name: L("カザフの島々", "Kazakh islands", "哈萨克岛群"),
        now: L("カザフスタン", "Kazakhstan", "哈萨克斯坦"),
        outline: &[(-3.0, 2.0), (3.0, 3.0), (4.0, -1.0), (-2.0, -3.0)],
        keys: &[
            (540.0, 102.0, 24.0, 0.0), (485.0, 106.0, 26.0, 20.0), (445.0, 96.0, 20.0, 30.0), (400.0, 86.0, 26.0, 30.0), (360.0, 72.0, 34.0, 30.0),
            (320.0, 46.0, 36.0, 30.0), (300.0, 37.0, 31.0, 30.0),
        ],
        attach: Some((BALTICA, 300.0)),
        flood: 0.3,
        relief: 0.6,
        seed: 83,
        label: false,
    },
    PlateDef {
        name: L("タリム地塊", "Tarim", "塔里木板块"),
        now: L("中国・新疆", "Xinjiang, China", "中国新疆"),
        outline: &[(-4.0, 2.0), (3.0, 3.0), (5.0, -1.0), (-1.0, -3.0), (-5.0, -1.0)],
        keys: &[
            (540.0, 112.0, 2.0, 0.0), (485.0, 116.0, 8.0, 10.0), (445.0, 110.0, 10.0, 10.0), (400.0, 100.0, 18.0, 10.0), (360.0, 88.0, 26.0, 10.0),
            (320.0, 60.0, 28.0, 10.0), (290.0, 47.0, 23.0, 10.0),
        ],
        attach: Some((KAZAKH, 290.0)),
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
// オルドビス紀
pub const S_FEZOUATA: usize = 10;
pub const S_BOLIVIA: usize = 11;
pub const S_STPETERSBURG: usize = 12;
pub const S_CINCINNATI: usize = 13;
// シルル紀
pub const S_STONEHAVEN: usize = 14;
pub const S_GOTLAND: usize = 15;
pub const S_DUDLEY: usize = 16;
pub const S_QUJING: usize = 17;
pub const S_BERTIE: usize = 18;
// デボン紀
pub const S_RHYNIE: usize = 19;
pub const S_HUNSRUCK: usize = 20;
pub const S_GILBOA: usize = 21;
pub const S_GOGO: usize = 22;
pub const S_MIGUASHA: usize = 23;
pub const S_ELLESMERE: usize = 24;
pub const S_EGREENLAND: usize = 25;
pub const S_CLEVELAND: usize = 26;
pub const S_EIFEL: usize = 27;
// 石炭紀
pub const S_EASTKIRKTON: usize = 28;
pub const S_BEARGULCH: usize = 29;
pub const S_JOGGINS: usize = 30;
pub const S_MAZON: usize = 31;
pub const S_COMMENTRY: usize = 32;
// ペルム紀
pub const S_IRATI: usize = 33;
pub const S_TEXAS: usize = 34;
pub const S_CAPITAN: usize = 35;
pub const S_KAROO: usize = 36;
pub const S_VYATKA: usize = 37;
pub const S_SIBTRAPS: usize = 38;

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
    // ---- オルドビス紀
    SiteDef { name: L("フェズアタ", "Fezouata", "费祖阿塔"), now: L("モロッコ南東部", "South-eastern Morocco", "摩洛哥东南部"), plate: GONDWANA, at: (-62.0, -24.0), want: Habitat::Shelf, t0: 481.0, t1: 476.0 },
    SiteDef { name: L("アンデスの浅い海", "Andean shallow sea", "安第斯浅海"), now: L("ボリビア", "Bolivia", "玻利维亚"), plate: GONDWANA, at: (-68.0, 0.0), want: Habitat::Shallow, t0: 467.0, t1: 460.0 },
    SiteDef { name: L("バルト海の石灰岩", "Baltic limestone sea", "波罗的石灰岩海"), now: L("ロシア・サンクトペテルブルク", "St Petersburg, Russia", "俄罗斯圣彼得堡"), plate: BALTICA, at: (4.0, 6.0), want: Habitat::Shallow, t0: 470.0, t1: 460.0 },
    SiteDef { name: L("シンシナティの海", "Cincinnati sea", "辛辛那提海"), now: L("アメリカ・オハイオ州", "Ohio, USA", "美国俄亥俄州"), plate: LAURENTIA, at: (12.0, -8.0), want: Habitat::Shallow, t0: 453.0, t1: 444.5 },
    // ---- シルル紀
    SiteDef { name: L("ストーンヘイブン", "Stonehaven", "斯通黑文"), now: L("イギリス・スコットランド", "Scotland, UK", "英国苏格兰"), plate: LAURENTIA, at: (27.0, 10.0), want: Habitat::Land, t0: 429.0, t1: 424.0 },
    SiteDef { name: L("ゴトランドの礁", "Gotland reefs", "哥得兰礁"), now: L("スウェーデン・ゴトランド島", "Gotland, Sweden", "瑞典哥得兰岛"), plate: BALTICA, at: (-8.0, -5.0), want: Habitat::Shallow, t0: 433.0, t1: 425.0 },
    SiteDef { name: L("ダドリーの海", "Dudley sea", "达德利海"), now: L("イギリス・イングランド中部", "Midlands, England", "英国英格兰中部"), plate: AVALONIA, at: (8.0, 0.0), want: Habitat::Shallow, t0: 431.0, t1: 425.0 },
    SiteDef { name: L("曲靖（クーティ）", "Qujing (Kuanti)", "曲靖（关底）"), now: L("中国・雲南省", "Yunnan, China", "中国云南省"), plate: SOUTH_CHINA, at: (-4.0, -2.0), want: Habitat::Shallow, t0: 426.0, t1: 422.0 },
    SiteDef { name: L("バーティーの入り江", "Bertie lagoon", "伯蒂潟湖"), now: L("アメリカ・ニューヨーク州", "New York, USA", "美国纽约州"), plate: LAURENTIA, at: (20.0, -2.0), want: Habitat::Tidal, t0: 423.0, t1: 419.2 },
    // ---- デボン紀
    SiteDef { name: L("ライニーの温泉湿地", "Rhynie hot-spring wetland", "莱尼温泉湿地"), now: L("イギリス・スコットランド", "Scotland, UK", "英国苏格兰"), plate: LAURENTIA, at: (25.0, 13.0), want: Habitat::Land, t0: 411.0, t1: 405.0 },
    SiteDef { name: L("フンスリュックの海", "Hunsrück sea", "洪斯吕克海"), now: L("ドイツ西部", "Western Germany", "德国西部"), plate: AVALONIA, at: (12.0, 2.0), want: Habitat::Shelf, t0: 409.0, t1: 403.0 },
    SiteDef { name: L("ギルボアの森", "Gilboa forest", "吉尔博阿森林"), now: L("アメリカ・ニューヨーク州", "New York, USA", "美国纽约州"), plate: LAURENTIA, at: (18.0, -5.0), want: Habitat::Land, t0: 387.0, t1: 381.0 },
    SiteDef { name: L("ゴーゴーの礁", "Gogo reef", "戈戈礁"), now: L("オーストラリア西部", "Western Australia", "澳大利亚西部"), plate: GONDWANA, at: (76.0, 22.0), want: Habitat::Shallow, t0: 385.0, t1: 378.0 },
    SiteDef { name: L("ミグアシャの河口", "Miguasha estuary", "米瓜沙河口"), now: L("カナダ・ケベック州", "Quebec, Canada", "加拿大魁北克省"), plate: LAURENTIA, at: (24.0, 4.0), want: Habitat::Tidal, t0: 381.0, t1: 375.0 },
    SiteDef { name: L("エルズミーアの川辺", "Ellesmere riverbanks", "埃尔斯米尔河岸"), now: L("カナダ・ヌナブト準州", "Nunavut, Canada", "加拿大努纳武特"), plate: LAURENTIA, at: (2.0, 20.0), want: Habitat::Tidal, t0: 377.0, t1: 373.0 },
    SiteDef { name: L("東グリーンランドの川", "East Greenland rivers", "东格陵兰河流"), now: L("グリーンランド東部", "East Greenland", "格陵兰东部"), plate: LAURENTIA, at: (28.0, 15.0), want: Habitat::Tidal, t0: 366.0, t1: 359.0 },
    SiteDef { name: L("クリーブランドの海", "Cleveland sea", "克利夫兰海"), now: L("アメリカ・オハイオ州", "Ohio, USA", "美国俄亥俄州"), plate: LAURENTIA, at: (8.0, -10.0), want: Habitat::Shelf, t0: 372.0, t1: 359.0 },
    SiteDef { name: L("ラインの入り江", "Rhenish embayment", "莱茵海湾"), now: L("ドイツ・アイフェル地方", "Eifel, Germany", "德国艾费尔"), plate: AVALONIA, at: (2.0, 3.0), want: Habitat::Tidal, t0: 394.0, t1: 388.0 },
    // ---- 石炭紀
    SiteDef { name: L("イースト・カークトン", "East Kirkton", "东柯克顿"), now: L("イギリス・スコットランド", "Scotland, UK", "英国苏格兰"), plate: LAURENTIA, at: (26.0, 11.0), want: Habitat::Land, t0: 338.0, t1: 332.0 },
    SiteDef { name: L("ベア・ガルチの湾", "Bear Gulch bay", "熊峡湾"), now: L("アメリカ・モンタナ州", "Montana, USA", "美国蒙大拿州"), plate: LAURENTIA, at: (-16.0, 0.0), want: Habitat::Shallow, t0: 325.0, t1: 318.0 },
    SiteDef { name: L("ジョギンズの森", "Joggins forest", "乔金斯森林"), now: L("カナダ・ノバスコシア州", "Nova Scotia, Canada", "加拿大新斯科舍省"), plate: LAURENTIA, at: (24.0, -6.0), want: Habitat::Land, t0: 319.0, t1: 312.0 },
    SiteDef { name: L("メゾンクリークの三角州", "Mazon Creek delta", "梅森溪三角洲"), now: L("アメリカ・イリノイ州", "Illinois, USA", "美国伊利诺伊州"), plate: LAURENTIA, at: (2.0, -12.0), want: Habitat::Tidal, t0: 310.0, t1: 305.0 },
    SiteDef { name: L("コマントリーの沼", "Commentry swamp", "科芒特里沼泽"), now: L("フランス中部", "Central France", "法国中部"), plate: AVALONIA, at: (10.0, -2.0), want: Habitat::Land, t0: 303.0, t1: 298.9 },
    // ---- ペルム紀
    SiteDef { name: L("イラチの入り海", "Irati inland sea", "伊拉蒂内海"), now: L("ブラジル南部", "Southern Brazil", "巴西南部"), plate: GONDWANA, at: (-58.0, 8.0), want: Habitat::Shallow, t0: 299.0, t1: 292.0 },
    SiteDef { name: L("テキサスの赤い大地", "Texas red beds", "得克萨斯红层"), now: L("アメリカ・テキサス州", "Texas, USA", "美国得克萨斯州"), plate: LAURENTIA, at: (-10.0, -14.0), want: Habitat::Land, t0: 297.0, t1: 272.0 },
    SiteDef { name: L("キャピタン礁", "Capitan reef", "卡皮坦礁"), now: L("アメリカ・テキサス州西部", "West Texas, USA", "美国得克萨斯州西部"), plate: LAURENTIA, at: (-20.0, -14.0), want: Habitat::Shallow, t0: 268.0, t1: 259.0 },
    SiteDef { name: L("カルー盆地", "Karoo Basin", "卡鲁盆地"), now: L("南アフリカ", "South Africa", "南非"), plate: GONDWANA, at: (10.0, 0.0), want: Habitat::Land, t0: 266.0, t1: 251.9 },
    SiteDef { name: L("ヴャトカの川辺", "Vyatka riverlands", "维亚特卡河岸"), now: L("ロシア・キーロフ州", "Kirov region, Russia", "俄罗斯基洛夫州"), plate: BALTICA, at: (6.0, -2.0), want: Habitat::Land, t0: 262.0, t1: 252.0 },
    SiteDef { name: L("シベリアの大噴火", "Siberian Traps", "西伯利亚暗色岩"), now: L("ロシア・シベリア中部", "Central Siberia, Russia", "俄罗斯西伯利亚中部"), plate: SIBERIA, at: (0.0, 4.0), want: Habitat::Land, t0: 252.6, t1: 251.9 },
];

/// 大きな噴火（巨大火成岩岩石区）：(産地, 始まり, 終わり)
pub struct Eruption {
    pub site: usize,
    pub t0: f32,
    pub t1: f32,
}

pub static ERUPTIONS: &[Eruption] = &[
    Eruption { site: S_KALKARINDJI, t0: 511.6, t1: 509.9 },
    Eruption { site: S_SIBTRAPS, t0: 252.6, t1: 251.9 },
];

/// いま噴火しているもの
pub fn eruption_at(t: f32) -> Option<&'static Eruption> {
    ERUPTIONS.iter().find(|e| t <= e.t0 + 1e-3 && t >= e.t1 - 1e-3)
}

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
    Milestone {
        ma: 485.4,
        title: L("オルドビス紀のはじまり", "The Ordovician begins", "奥陶纪开始"),
        text: L("海はさらに広がり、大陸の多くが浅い海に沈む。ゴンドワナ大陸は南極の上へ移っていく。", "Seas spread even further, flooding much of the continents. Gondwana drifts over the South Pole.", "海洋进一步扩张，许多大陆被浅海淹没。冈瓦纳大陆向南极移动。"),
    },
    Milestone {
        ma: 470.0,
        title: L("オルドビス紀の生物大放散", "The Great Ordovician Biodiversification", "奥陶纪生物大辐射"),
        text: L("サンゴ・コケムシ・ウミユリ・オウムガイなどが一気にふえ、海の生きものの種類が数倍になる。陸には最初の小さな植物が現れた。", "Corals, bryozoans, crinoids and nautiloids boom, multiplying the variety of sea life. The first tiny plants appear on land.", "珊瑚、苔藓虫、海百合、鹦鹉螺等迅速增多，海洋生物种类增加数倍。陆地上出现了最早的小型植物。"),
    },
    Milestone {
        ma: 445.2,
        title: L("ヒルナンシアン氷期", "The Hirnantian ice age", "赫南特冰期"),
        text: L("南極のゴンドワナ大陸に氷床が広がり、海面が下がって海が冷える。", "Ice sheets spread over Gondwana at the South Pole; sea level falls and the oceans cool.", "南极的冈瓦纳大陆上冰盖扩张，海平面下降，海洋变冷。"),
    },
    Milestone {
        ma: 443.8,
        title: L("オルドビス紀末の大絶滅", "The end-Ordovician mass extinction", "奥陶纪末大灭绝"),
        text: L("寒さと海の変化で、海の生きものの種類のおよそ85%が絶滅した（五大絶滅の一つめ）。シルル紀へ。", "Cold and changing seas wipe out about 85% of marine species — the first of the 'Big Five' extinctions. The Silurian begins.", "寒冷和海洋变化导致约85%的海洋物种灭绝（五大灭绝之一）。志留纪开始。"),
    },
    Milestone {
        ma: 433.0,
        title: L("維管束植物の登場", "Vascular plants appear", "维管植物出现"),
        text: L("水を運ぶ管を持つクックソニアのような植物が、水辺に小さな緑を広げはじめる。", "Plants with water-carrying tubes, like Cooksonia, begin to spread small patches of green along the shores.", "具有输水管道的库克逊蕨等植物开始在水边铺展小片绿色。"),
    },
    Milestone {
        ma: 428.0,
        title: L("陸を歩く動物", "Animals walk on land", "动物登上陆地"),
        text: L("ヤスデの仲間が空気を吸って陸を歩いた。最古の陸上動物の一つ。", "A millipede breathes air and walks on land — one of the oldest land animals known.", "马陆类呼吸空气、在陆地上行走——已知最古老的陆生动物之一。"),
    },
    Milestone {
        ma: 425.0,
        title: L("顎を持つ魚がふえる", "Jawed fishes spread", "有颌鱼类增多"),
        text: L("顎を持つ板皮類や硬骨魚が現れ、海の食べる・食べられる関係が大きく変わる。", "Placoderms and bony fishes with jaws appear, transforming who eats whom in the sea.", "长着颌的盾皮鱼和硬骨鱼出现，海洋中的捕食关系发生巨变。"),
    },
    Milestone {
        ma: 420.0,
        title: L("ユーラメリカ大陸の誕生", "Euramerica is born", "欧美大陆诞生"),
        text: L("ローレンシアとバルティカがぶつかり、カレドニア山脈がそびえる大きな大陸になった。", "Laurentia and Baltica collide into one big continent, raising the Caledonian Mountains.", "劳伦大陆与波罗的大陆相撞，形成高耸着加里东山脉的大陆。"),
    },
    Milestone {
        ma: 419.2,
        title: L("デボン紀のはじまり：魚の時代", "The Devonian: the Age of Fishes", "泥盆纪开始：鱼类时代"),
        text: L("板皮類・サメ・肉鰭類など、さまざまな魚が海と川にあふれる。", "Placoderms, sharks and lobe-finned fishes fill the seas and rivers.", "盾皮鱼、鲨鱼、肉鳍鱼等各种鱼类遍布海洋和河流。"),
    },
    Milestone {
        ma: 385.0,
        title: L("最初の森", "The first forests", "最早的森林"),
        text: L("背の高い木が現れ、地球に初めて森ができた。根が岩を砕いて土をつくり、空気の二酸化炭素が減っていく。", "Tall trees appear and the first forests grow. Roots break rock into soil and draw carbon dioxide from the air.", "高大的树木出现，地球上第一次有了森林。树根把岩石破碎成土壤，空气中的二氧化碳减少。"),
    },
    Milestone {
        ma: 375.0,
        title: L("ひれから足へ", "From fins to feet", "从鳍到脚"),
        text: L("ティクターリクのような魚が浅瀬で体を持ち上げ、やがて足を持つ四肢動物が現れる。", "Fish like Tiktaalik prop themselves up in the shallows; soon four-legged tetrapods appear.", "提塔利克鱼等鱼类在浅滩中撑起身体，不久长着四肢的四足动物出现了。"),
    },
    Milestone {
        ma: 372.2,
        title: L("ケルワッサー事変", "The Kellwasser event", "凯尔瓦瑟事件"),
        text: L("海の酸素が減り、礁をつくる生きものなどが大きく絶滅した（デボン紀後期の大絶滅）。", "Oxygen drains from the seas and reef-builders and many others die out — the Late Devonian extinction.", "海洋缺氧，造礁生物等大量灭绝（晚泥盆世大灭绝）。"),
    },
    Milestone {
        ma: 358.9,
        title: L("石炭紀のはじまり", "The Carboniferous begins", "石炭纪开始"),
        text: L("デボン紀の終わりの絶滅（ハンゲンベルク事変）を越え、湿った森が大陸に広がっていく。", "After the Hangenberg extinction at the end of the Devonian, damp forests spread over the continents.", "经历泥盆纪末的汉根堡灭绝后，潮湿的森林在大陆上蔓延。"),
    },
    Milestone {
        ma: 340.0,
        title: L("後期古生代の氷河時代", "The Late Paleozoic Ice Age", "晚古生代冰期"),
        text: L("南のゴンドワナ大陸に大きな氷床ができ、海面が上がり下がりをくり返す。", "Huge ice sheets build up on southern Gondwana and sea level rises and falls again and again.", "南方的冈瓦纳大陆形成巨大冰盖，海平面反复升降。"),
    },
    Milestone {
        ma: 318.0,
        title: L("殻のある卵", "The shelled egg", "有壳的卵"),
        text: L("水辺を離れて卵を産める爬虫類の仲間（有羊膜類）が現れた。", "Reptile-like amniotes appear, able to lay their eggs away from water.", "能离开水边产卵的类爬行动物（羊膜动物）出现了。"),
    },
    Milestone {
        ma: 307.0,
        title: L("石炭紀の森の崩壊", "The Carboniferous rainforest collapse", "石炭纪雨林崩溃"),
        text: L("気候が乾いて、巨大なシダの森が小さく分かれていった。", "The climate dries and the great fern forests break up into islands.", "气候变干，巨大的蕨类森林分裂成小块。"),
    },
    Milestone {
        ma: 298.9,
        title: L("ペルム紀のはじまり：超大陸パンゲア", "The Permian: supercontinent Pangaea", "二叠纪开始：盘古超大陆"),
        text: L("大陸がぶつかり合い、ほとんどの陸が一つの超大陸になった。内陸には砂漠が広がる。", "The continents have collided into one supercontinent. Deserts spread across its interior.", "大陆相互碰撞，几乎所有陆地连成一个超大陆。内陆沙漠广布。"),
    },
    Milestone {
        ma: 259.5,
        title: L("キャピタニアンの絶滅", "The Capitanian extinction", "卡匹敦期灭绝"),
        text: L("南中国の峨眉山の大噴火のころ、礁の生きものなどが絶滅した。", "Around the Emeishan eruptions in South China, reef life and others die out.", "在华南峨眉山大喷发前后，礁生物等发生灭绝。"),
    },
    Milestone {
        ma: 252.6,
        title: L("シベリアの大噴火", "The Siberian Traps erupt", "西伯利亚暗色岩喷发"),
        text: L("シベリアで地球史上最大級の溶岩が噴き出す。ガスが地球を温め、海を酸性に、酸素の少ないものに変えていく。", "One of the largest lava outpourings in Earth's history begins in Siberia. Its gases heat the planet and turn the seas acidic and oxygen-poor.", "西伯利亚开始喷发地球史上规模最大的熔岩之一。气体使地球变暖，海洋酸化、缺氧。"),
    },
    Milestone {
        ma: 251.902,
        title: L("ペルム紀末の大絶滅", "The end-Permian mass extinction", "二叠纪末大灭绝"),
        text: L("海の生きものの約90%が消えた、史上最大の絶滅。古生代はここで終わり、中生代三畳紀へ続く。", "The greatest extinction ever: about 90% of marine species vanish. The Paleozoic ends here; the Mesozoic Triassic follows.", "史上最大的灭绝：约90%的海洋物种消失。古生代就此结束，进入中生代三叠纪。"),
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
    // オルドビス紀
    Stage { t0: 485.4, t1: 477.7, name: L("トレマドキアン期", "Tremadocian", "特马豆克期"), col: [0.10, 0.58, 0.46] },
    Stage { t0: 477.7, t1: 470.0, name: L("フロイアン期", "Floian", "弗洛期"), col: [0.16, 0.63, 0.50] },
    Stage { t0: 470.0, t1: 467.3, name: L("ダーピンジアン期", "Dapingian", "大坪期"), col: [0.26, 0.68, 0.55] },
    Stage { t0: 467.3, t1: 458.4, name: L("ダーリウィリアン期", "Darriwilian", "达瑞威尔期"), col: [0.34, 0.72, 0.58] },
    Stage { t0: 458.4, t1: 453.0, name: L("サンドビアン期", "Sandbian", "桑比期"), col: [0.44, 0.76, 0.62] },
    Stage { t0: 453.0, t1: 445.2, name: L("カティアン期", "Katian", "凯迪期"), col: [0.52, 0.80, 0.66] },
    Stage { t0: 445.2, t1: 443.8, name: L("ヒルナンシアン期", "Hirnantian", "赫南特期"), col: [0.60, 0.84, 0.72] },
    // シルル紀
    Stage { t0: 443.8, t1: 433.4, name: L("ランドベリ世", "Llandovery", "兰多维列世"), col: [0.56, 0.80, 0.70] },
    Stage { t0: 433.4, t1: 427.4, name: L("ウェンロック世", "Wenlock", "温洛克世"), col: [0.64, 0.85, 0.76] },
    Stage { t0: 427.4, t1: 423.0, name: L("ラドロー世", "Ludlow", "罗德洛世"), col: [0.72, 0.89, 0.82] },
    Stage { t0: 423.0, t1: 419.2, name: L("プリドリ世", "Pridoli", "普里道利世"), col: [0.82, 0.93, 0.88] },
    // デボン紀
    Stage { t0: 419.2, t1: 410.8, name: L("ロッコヴィアン期", "Lochkovian", "洛赫考夫期"), col: [0.78, 0.52, 0.24] },
    Stage { t0: 410.8, t1: 407.6, name: L("プラギアン期", "Pragian", "布拉格期"), col: [0.82, 0.58, 0.28] },
    Stage { t0: 407.6, t1: 393.3, name: L("エムシアン期", "Emsian", "埃姆斯期"), col: [0.85, 0.62, 0.32] },
    Stage { t0: 393.3, t1: 387.7, name: L("アイフェリアン期", "Eifelian", "艾菲尔期"), col: [0.88, 0.68, 0.40] },
    Stage { t0: 387.7, t1: 382.7, name: L("ジベティアン期", "Givetian", "吉维特期"), col: [0.90, 0.73, 0.46] },
    Stage { t0: 382.7, t1: 372.2, name: L("フラニアン期", "Frasnian", "弗拉斯期"), col: [0.92, 0.78, 0.54] },
    Stage { t0: 372.2, t1: 358.9, name: L("ファメニアン期", "Famennian", "法门期"), col: [0.94, 0.83, 0.62] },
    // 石炭紀
    Stage { t0: 358.9, t1: 346.7, name: L("トルネーシアン期", "Tournaisian", "杜内期"), col: [0.38, 0.60, 0.56] },
    Stage { t0: 346.7, t1: 330.9, name: L("ビゼーアン期", "Visean", "维宪期"), col: [0.44, 0.65, 0.60] },
    Stage { t0: 330.9, t1: 323.2, name: L("サープコビアン期", "Serpukhovian", "谢尔普霍夫期"), col: [0.50, 0.70, 0.64] },
    Stage { t0: 323.2, t1: 315.2, name: L("バシキーリアン期", "Bashkirian", "巴什基尔期"), col: [0.56, 0.74, 0.70] },
    Stage { t0: 315.2, t1: 307.0, name: L("モスコビアン期", "Moscovian", "莫斯科期"), col: [0.62, 0.78, 0.74] },
    Stage { t0: 307.0, t1: 303.7, name: L("カシモビアン期", "Kasimovian", "卡西莫夫期"), col: [0.68, 0.82, 0.78] },
    Stage { t0: 303.7, t1: 298.9, name: L("グゼーリアン期", "Gzhelian", "格舍尔期"), col: [0.74, 0.86, 0.82] },
    // ペルム紀
    Stage { t0: 298.9, t1: 290.1, name: L("アッセリアン〜サクマリアン期", "Asselian–Sakmarian", "阿瑟尔—萨克马尔期"), col: [0.88, 0.34, 0.24] },
    Stage { t0: 290.1, t1: 283.5, name: L("アーティンスキアン期", "Artinskian", "亚丁斯克期"), col: [0.90, 0.42, 0.30] },
    Stage { t0: 283.5, t1: 273.0, name: L("クングーリアン期", "Kungurian", "空谷期"), col: [0.91, 0.48, 0.36] },
    Stage { t0: 273.0, t1: 264.3, name: L("ローディアン〜ウォーディアン期", "Roadian–Wordian", "罗德—沃德期"), col: [0.93, 0.56, 0.42] },
    Stage { t0: 264.3, t1: 259.5, name: L("キャピタニアン期", "Capitanian", "卡匹敦期"), col: [0.94, 0.62, 0.50] },
    Stage { t0: 259.5, t1: 254.1, name: L("ウーチャーピンジアン期", "Wuchiapingian", "吴家坪期"), col: [0.95, 0.70, 0.60] },
    Stage { t0: 254.1, t1: 251.9, name: L("チャンシンジアン期", "Changhsingian", "长兴期"), col: [0.96, 0.76, 0.66] },
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

/// 時代区分の名前：古生代 カンブリア紀・第3期
pub fn era_text(t: f32) -> String {
    tf!("古生代 {}・{}", "Paleozoic · {} · {}", "古生代 {}·{}", PERIODS[period_at(t)].name.get(), stage_at(t).name.get())
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
    /// 植物におおわれた陸の色（地図で `veg_at` の濃さで重ねる）
    pub px_green: Vec<u32>,
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

/// 高さから色を決める（陸は植物のない赤茶けた岩、海岸は微生物マット、浅い海は明るい青緑）。
/// veg（0〜1）で低地を森の緑にする（デボン紀より後の陸）
pub fn color_of(e: f32, n: f32, shade: f32, veg: f32) -> ([f32; 3], f32) {
    if e > 0.0 {
        if e < 0.035 {
            // 干潟・微生物マット（植物の時代は水辺の湿地）
            let c = lerp3([0.46, 0.46, 0.30], [0.52, 0.50, 0.34], n);
            return (lerp3(c, lerp3([0.24, 0.40, 0.24], [0.32, 0.46, 0.28], n), veg * 0.8), 1.0);
        }
        let k = ((e - 0.035) / 0.9).clamp(0.0, 1.0);
        let low = lerp3([0.56, 0.38, 0.27], [0.78, 0.60, 0.42], n);
        let high = lerp3([0.46, 0.38, 0.35], [0.64, 0.58, 0.54], n);
        let mut c = lerp3(low, high, k);
        if veg > 0.0 {
            let green = lerp3([0.20, 0.36, 0.18], [0.36, 0.48, 0.24], n);
            c = lerp3(c, green, veg * (1.0 - k * 0.85));
        }
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
        let mut px_green = vec![0u32; w * h];
        for j in 0..h {
            for i in 0..w {
                let v = e[j * w + i];
                let vx = e[j * w + (i + 1).min(w - 1)] - e[j * w + i.saturating_sub(1)];
                let vy = e[(j + 1).min(h - 1) * w + i] - e[j.saturating_sub(1) * w + i];
                let shade = (0.5 - (vx - vy) * 3.0).clamp(0.0, 1.0);
                let lx = x0 + (i as f32 + 0.5) / RES;
                let ly = y1 - (j as f32 + 0.5) / RES;
                let n = ((fbm(lx * 0.25, ly * 0.25, sd + 61, 4) - 0.5) * 2.2 + 0.5).clamp(0.0, 1.0);
                let (c, a) = color_of(v, n, shade, 0.0);
                px[j * w + i] = rgba(c, a);
                // 緑の版は陸だけ（海は透明にして、重ねたときに海の色を変えない）
                let (c, a) = color_of(v, n, shade, 1.0);
                px_green[j * w + i] = rgba(c, if v > 0.0 { a } else { 0.0 });
            }
        }
        Plate { x0, y0, x1, y1, w, h, e, px, px_green }
    }

    /// 観察地点を、近くのふさわしい深さの場所に置き直す
    fn place_site(&self, s: &SiteDef) -> (f32, f32) {
        let p = &self.plates[s.plate];
        let want = s.want;
        let r = 18.0;
        let mut cand: Vec<(f32, (f32, f32))> = Vec::new();
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
                        cand.push((dx * dx + dy * dy, (lx, ly)));
                    }
                }
                dx += 0.25;
            }
            dy += 0.25;
        }
        cand.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        // ぶつかった大陸に隠れていない（その時代の地図の上でも、ふさわしい環境の）場所を選ぶ
        let times = [s.t0, (s.t0 + s.t1) * 0.5, s.t1];
        let visible = |l: (f32, f32)| {
            times.iter().all(|&t| {
                let (lon, lat) = self.to_world(s.plate, t, l);
                let sp = self.sample(lon, lat, t);
                sp.hab == want && sp.plate == Some(s.plate)
            })
        };
        cand.iter().find(|c| visible(c.1)).or(cand.first()).map_or(s.at, |c| c.1)
    }

    /// 大陸の中心位置と回転（度）
    pub fn pose(&self, i: usize, t: f32) -> (f32, f32, f32) {
        plate_pose(i, t)
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

    /// いくつかの大陸をまとめて囲む範囲（地図座標、地図の外にはみ出す分は切る）
    pub fn plates_bounds(&self, plates: &[usize], t: f32) -> (f32, f32, f32, f32) {
        let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for &i in plates {
            let b = self.plate_bounds(i, t);
            x0 = x0.min(b.0);
            y0 = y0.min(b.1);
            x1 = x1.max(b.2);
            y1 = y1.max(b.3);
        }
        (x0.max(0.0), y0.max(0.0), x1.min(MAP_W), y1.min(MAP_H))
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

/// キーフレームの間を補間した位置と回転
fn key_pose(i: usize, t: f32) -> (f32, f32, f32) {
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

/// 大陸の中心位置と回転（度）。ぶつかった後は相手と一緒に動く
pub fn plate_pose(i: usize, t: f32) -> (f32, f32, f32) {
    if let Some((p, ta)) = PLATES[i].attach {
        if t < ta {
            // ぶつかった時の、相手から見た位置と向きを保つ
            let (cx, cy, cr) = key_pose(i, ta);
            let (px, py, pr) = plate_pose(p, ta);
            let (s, c) = pr.to_radians().sin_cos();
            let (dx, dy) = (cx - px, cy - py);
            let (lx, ly) = (dx * c + dy * s, -dx * s + dy * c);
            let (qx, qy, qr) = plate_pose(p, t);
            let (s, c) = qr.to_radians().sin_cos();
            return (qx + lx * c - ly * s, qy + lx * s + ly * c, qr + (cr - pr));
        }
    }
    key_pose(i, t)
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
        assert_eq!(stage_at(444.0).name.1, "Hirnantian");
        assert_eq!(stage_at(252.0).name.1, "Changhsingian");
        assert_eq!(period_at(538.8), CAMBRIAN);
        assert_eq!(period_at(485.4), CAMBRIAN);
        assert_eq!(period_at(485.0), ORDOVICIAN);
        assert_eq!(period_at(375.0), DEVONIAN);
        assert_eq!(period_at(251.9), PERMIAN);
    }

    /// 期は紀の中にすき間なく並ぶ
    #[test]
    fn stages_tile_periods() {
        for w in STAGES.windows(2) {
            assert!((w[0].t1 - w[1].t0).abs() < 1e-3, "{} -> {}", w[0].name.1, w[1].name.1);
        }
        assert_eq!(STAGES[0].t0, T_START);
        assert!((STAGES[STAGES.len() - 1].t1 - T_END).abs() < 1e-3);
        for p in PERIODS {
            assert!(STAGES.iter().any(|s| (s.t0 - p.t0).abs() < 1e-3));
        }
        for m in MILESTONES {
            assert!(m.ma <= T_START + 1e-3 && m.ma >= T_END - 1e-3);
        }
    }

    /// ぶつかった大陸は、相手と一緒に動く
    #[test]
    fn attached_plates_move_together() {
        let e = Earth::new();
        let a = e.to_world(BALTICA, 400.0, (3.0, 2.0));
        let b = e.to_world(LAURENTIA, 400.0, e.to_local(LAURENTIA, 400.0, a.0, a.1));
        assert!((a.0 - b.0).abs() < 1e-3 && (a.1 - b.1).abs() < 1e-3);
        let near = |t: f32| {
            let (a, b) = (e.to_world(GONDWANA, t, (0.0, 50.0)), e.to_world(LAURENTIA, t, (0.0, -20.0)));
            ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
        };
        assert!((near(300.0) - near(255.0)).abs() < 0.01);
    }

    /// 大陸どうしが大きく重ならない
    #[test]
    fn plates_do_not_overlap_much() {
        let e = Earth::new();
        // カンブリア紀は大陸が離れている。ぶつかり合う後の時代は、山脈になる縁の重なりを少し許す
        for &(t, pct) in &[(538.8, 1), (520.0, 1), (505.0, 1), (485.4, 1), (460.0, 3), (430.0, 4), (400.0, 4), (370.0, 4), (330.0, 5), (300.0, 6), (270.0, 6), (252.0, 6)] {
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
            assert!(overlap * 100 < land * pct, "t={} overlap {} / land {}", t, overlap, land);
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

    /// 地図を画像に書き出す（目で確かめる用）：YY_DUMP=出力先 cargo test --lib dump_map -- --ignored
    /// YY_TIMES=485,445,... で時代を選べる。大陸ごとに色を変え、そのころの産地を赤で示す
    #[test]
    #[ignore]
    fn dump_map() {
        let e = Earth::new();
        let dir = std::env::var("YY_DUMP").unwrap_or("/tmp".into());
        let times: Vec<f32> = std::env::var("YY_TIMES")
            .map(|s| s.split(',').filter_map(|x| x.trim().parse().ok()).collect())
            .unwrap_or(vec![538.0, 508.0, 486.0, 460.0, 444.0, 425.0, 400.0, 375.0, 340.0, 310.0, 290.0, 260.0, 252.0]);
        let tint = [[1.0, 0.9, 0.6], [0.7, 1.0, 0.7], [0.7, 0.8, 1.0], [1.0, 0.7, 1.0], [0.6, 1.0, 1.0], [1.0, 0.7, 0.6], [0.9, 0.9, 0.9], [1.0, 1.0, 0.5], [0.8, 0.6, 1.0]];
        for &t in &times {
            let (w, h) = (720u32, 360u32);
            let mut img = image::RgbImage::new(w, h);
            for j in 0..h {
                for i in 0..w {
                    let lon = -180.0 + (i as f32 + 0.5) / 2.0;
                    let lat = 90.0 - (j as f32 + 0.5) / 2.0;
                    let s = e.sample(lon, lat, t);
                    let (mut c, a) = color_of(s.e, 0.5, 0.5, veg_at(t));
                    if let (Some(p), true) = (s.plate, s.e > 0.035) {
                        c = [c[0] * tint[p][0], c[1] * tint[p][1], c[2] * tint[p][2]];
                    }
                    let bg = [0.06, 0.18, 0.36];
                    let c = [bg[0] + (c[0] - bg[0]) * a, bg[1] + (c[1] - bg[1]) * a, bg[2] + (c[2] - bg[2]) * a];
                    let mut px = [(c[0] * 255.0) as u8, (c[1] * 255.0) as u8, (c[2] * 255.0) as u8];
                    if (lat.abs() - 30.0).abs() < 0.3 || lat.abs() < 0.3 {
                        px = [px[0] / 2 + 60, px[1] / 2 + 60, px[2] / 2 + 60];
                    }
                    for (k, st) in SITES.iter().enumerate() {
                        if t > st.t0 + 0.5 || t < st.t1 - 0.5 {
                            continue;
                        }
                        let (sl, sa) = e.site_lonlat(k, t);
                        if (sl - lon).abs() < 1.2 && (sa - lat).abs() < 1.2 {
                            px = [255, 40, 40];
                        }
                    }
                    img.put_pixel(i, j, image::Rgb(px));
                }
            }
            img.save(format!("{}/map_{}.png", dir, t as i32)).unwrap();
        }
        for (k, s) in SITES.iter().enumerate() {
            let t = (s.t0 + s.t1) * 0.5;
            let (lon, lat) = e.site_lonlat(k, t);
            let sp = e.sample(lon, lat, t);
            println!("site {} {} at {:.1},{:.1} local {:?} -> {:?} plate {:?} want {:?}", k, s.name.1, lon, lat, e.sites[k].at, sp.hab, sp.plate, s.want);
        }
    }
}
