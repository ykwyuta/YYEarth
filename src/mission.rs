//! 学校の宿題（ミッション）：トキ先生から出される課題。撮影して図鑑に収録すると達成
use crate::lang::L;
use crate::life::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Target {
    Sp(usize),
    Ph(usize),
}

impl Target {
    /// 図鑑の項目番号
    pub fn entry(self) -> usize {
        match self {
            Target::Sp(i) => i,
            Target::Ph(i) => n_species() + i,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Target::Sp(i) => SPECIES[i].name.get(),
            Target::Ph(i) => PHENOMS[i].name.get(),
        }
    }
}

pub struct Mission {
    pub title: L,
    /// 先生からの課題の文
    pub brief: L,
    /// 達成したときの先生のことば
    pub done: L,
    /// (撮る対象, 必要な星の数)
    pub targets: &'static [(Target, u8)],
}

use Target::*;

pub const SP_STROMATOLITE: usize = 0;
pub const SP_MAT: usize = 1;
pub const SP_TREPTICHNUS: usize = 2;
pub const SP_ANABARITES: usize = 3;
pub const SP_ARCHAEOCYATH: usize = 4;
pub const SP_LINGULELLA: usize = 5;
pub const SP_REDLICHIA: usize = 6;
pub const SP_HALKIERIA: usize = 7;
pub const SP_HAIKOUICHTHYS: usize = 8;
pub const SP_FUXIANHUIA: usize = 9;
pub const SP_ISOXYS: usize = 10;
pub const SP_LEANCHOILIA: usize = 11;
pub const SP_HALLUCIGENIA: usize = 12;
pub const SP_AMPLECTOBELUA: usize = 13;
pub const SP_KERYGMACHELA: usize = 14;
pub const SP_TAMISIOCARIS: usize = 15;
pub const SP_ANOMALOCARIS: usize = 16;
pub const SP_OPABINIA: usize = 17;
pub const SP_WIWAXIA: usize = 18;
pub const SP_MARRELLA: usize = 19;
pub const SP_PIKAIA: usize = 20;
pub const SP_OTTOIA: usize = 21;
pub const SP_AYSHEAIA: usize = 22;
pub const SP_OLENOIDES: usize = 23;
pub const SP_VAUXIA: usize = 24;
pub const SP_SIDNEYIA: usize = 25;
pub const SP_NECTOCARIS: usize = 26;
pub const SP_JELLY: usize = 27;
pub const SP_CAMBRORASTER: usize = 28;
pub const SP_PARADOXIDES: usize = 29;
pub const SP_OLENUS: usize = 30;
pub const SP_AGNOSTUS: usize = 31;
pub const SP_CLIMACTICHNITES: usize = 32;
pub const SP_CONODONT: usize = 33;
pub const SP_RADIOLARIAN: usize = 34;
pub const SP_GLASS_SPONGE: usize = 35;

pub static MISSIONS: &[Mission] = &[
    Mission {
        title: L("はじめての観察", "First observations", "第一次观察"),
        brief: L(
            "まずはカプセルの使い方に慣れましょう。上空からゴンドワナ大陸の全体を撮影し、陸に降りて植物のない大地を、海に潜ってストロマトライトを撮ってきてください。",
            "First, get used to the capsule. Photograph the whole of Gondwana from the sky, land to shoot the plantless ground, then dive and photograph a stromatolite.",
            "先熟悉观测舱的用法吧。从高空拍下整个冈瓦纳大陆，降落到陆地上拍下没有植物的大地，再潜入海中拍摄叠层石。",
        ),
        done: L(
            "よくできました！ カンブリア紀の陸には草一本なく、生命のほとんどは海にいたのです。",
            "Well done! Not a single blade of grass grows on Cambrian land; nearly all life is in the sea.",
            "做得好！寒武纪的陆地上寸草不生，几乎所有生命都在海里。",
        ),
        targets: &[(Ph(P_GONDWANA), 1), (Ph(P_BARREN), 1), (Sp(SP_STROMATOLITE), 1)],
    },
    Mission {
        title: L("カンブリア紀の夜明け", "Dawn of the Cambrian", "寒武纪的黎明"),
        brief: L(
            "5億3500万年前ごろの浅い海へ。動物が泥に掘った巣穴の跡と、小さな殻を持つ動物、それに干潟の微生物マットを記録しましょう。",
            "Visit a shallow sea around 535 million years ago. Record animal burrows in the mud, a tiny shelled animal, and the microbial mats of the tidal flats.",
            "前往约5亿3500万年前的浅海。记录动物在泥中挖的潜穴、带小壳的动物，以及潮坪上的微生物席。",
        ),
        done: L(
            "この巣穴の跡こそが、カンブリア紀の始まりの目印なのです。",
            "These burrows are the very marker of the start of the Cambrian.",
            "这些潜穴痕迹正是寒武纪开始的标志。",
        ),
        targets: &[(Sp(SP_TREPTICHNUS), 1), (Sp(SP_ANABARITES), 1), (Sp(SP_MAT), 1)],
    },
    Mission {
        title: L("古杯類の礁", "The archaeocyath reefs", "古杯类礁"),
        brief: L(
            "5億2000万年前ごろ、シベリア大陸の暖かく浅い海には、動物がつくった最初の礁がありました。古杯類の礁と、そこに暮らす生きものを撮影してください。",
            "Around 520 million years ago, the warm shallow seas of Siberia hosted the first reefs built by animals. Photograph the archaeocyath reef and its residents.",
            "约5亿2000万年前，西伯利亚大陆温暖的浅海中有由动物建造的最早的礁。请拍摄古杯类礁和生活在那里的生物。",
        ),
        done: L(
            "見事な礁でしたね。でもこの礁は、やがて海の酸素が減る事件で姿を消してしまいます。",
            "A splendid reef! But it will vanish when the seas lose their oxygen.",
            "真是壮观的礁啊。但这片礁将因海洋缺氧事件而消失。",
        ),
        targets: &[(Sp(SP_ARCHAEOCYATH), 2), (Sp(SP_LINGULELLA), 1), (Sp(SP_HALKIERIA), 1)],
    },
    Mission {
        title: L("澄江の海", "The Chengjiang sea", "澄江之海"),
        brief: L(
            "カンブリア爆発のまっただ中、5億1800万年前の南中国の海へ。わたしたちの遠いご先祖さまの仲間、ハイコウイクチスも泳いでいるはずです。驚かせると逃げるので、そっと近づくこと。",
            "Into the heart of the Cambrian explosion: the South China sea 518 million years ago. Haikouichthys, a cousin of our own distant ancestors, swims here. It flees if startled, so approach quietly.",
            "深入寒武纪大爆发：5亿1800万年前的华南海域。我们远祖的亲戚海口鱼也在这里游动。受惊会逃跑，要悄悄靠近。",
        ),
        done: L(
            "すばらしい！ 澄江の化石は、カンブリア爆発のようすを教えてくれる宝物です。",
            "Wonderful! The Chengjiang fossils are a treasure that shows us the Cambrian explosion.",
            "太棒了！澄江化石是揭示寒武纪大爆发的宝藏。",
        ),
        targets: &[(Sp(SP_HAIKOUICHTHYS), 1), (Sp(SP_FUXIANHUIA), 1), (Sp(SP_ISOXYS), 1), (Sp(SP_HALLUCIGENIA), 1), (Sp(SP_AMPLECTOBELUA), 1)],
    },
    Mission {
        title: L("北の海 シリウス・パセット", "Sirius Passet in the north", "北方的天狼星帕塞特"),
        brief: L(
            "同じころのローレンシア大陸の縁、いまのグリーンランド北部の海です。大陸の全体を上空から撮ってから潜りましょう。",
            "The edge of Laurentia at the same time, now North Greenland. Photograph the whole continent from the sky, then dive.",
            "同一时期劳伦大陆的边缘，即今格陵兰北部的海域。先从高空拍下整个大陆，再潜入海中。",
        ),
        done: L(
            "腕のある奇妙な動物たちは、アノマロカリスの親戚。進化の途中のすがたです。",
            "Those strange armed animals are relatives of Anomalocaris, caught in the middle of evolving.",
            "那些长着臂的奇异动物是奇虾的亲戚，正处于演化的中途。",
        ),
        targets: &[(Ph(P_LAURENTIA), 1), (Sp(SP_KERYGMACHELA), 1), (Sp(SP_TAMISIOCARIS), 1)],
    },
    Mission {
        title: L("大噴火の時代", "The age of eruptions", "大喷发的时代"),
        brief: L(
            "5億1100万年前、ゴンドワナ大陸の北で巨大な噴火が起きます。上空から噴火を記録し、近くのエミュー湾の海で大きな三葉虫レドリキアとアノマロカリスを探してください。",
            "511 million years ago, a colossal eruption begins in northern Gondwana. Record it from the sky, then look for the big trilobite Redlichia and Anomalocaris in nearby Emu Bay.",
            "5亿1100万年前，冈瓦纳大陆北部发生巨大喷发。请从高空记录喷发，再到附近的鸸鹋湾寻找大型三叶虫莱得利基虫和奇虾。",
        ),
        done: L(
            "噴火のガスは地球を温め、海の酸素を奪いました。生きものの歴史は地球の出来事と深く結びついているのです。",
            "The gases warmed the planet and stole oxygen from the seas. The story of life is bound up with the story of the Earth.",
            "喷发的气体使地球变暖、夺走了海洋中的氧气。生命的历史与地球的事件紧密相连。",
        ),
        targets: &[(Ph(P_ERUPTION), 1), (Sp(SP_REDLICHIA), 1), (Sp(SP_ANOMALOCARIS), 1)],
    },
    Mission {
        title: L("バージェス頁岩の海", "The Burgess Shale sea", "伯吉斯页岩之海"),
        brief: L(
            "5億800万年前、ローレンシア大陸の海底の崖のふもと。ここは「バージェス動物群」で有名な場所です。たくさんの不思議な動物を記録しましょう。",
            "508 million years ago, at the foot of an undersea cliff on Laurentia: home of the famous Burgess Shale fauna. Record as many strange animals as you can.",
            "5亿800万年前，劳伦大陆海底悬崖脚下，这里因“伯吉斯动物群”而闻名。尽可能多地记录奇异的动物吧。",
        ),
        done: L(
            "見事な観察記録です！ 100年以上前から、科学者たちはこの化石に驚かされ続けてきました。",
            "A superb record! Scientists have been amazed by these fossils for more than a century.",
            "出色的观察记录！一百多年来，科学家们一直为这些化石惊叹。",
        ),
        targets: &[(Sp(SP_MARRELLA), 1), (Sp(SP_OPABINIA), 1), (Sp(SP_WIWAXIA), 1), (Sp(SP_PIKAIA), 1), (Sp(SP_OTTOIA), 1), (Sp(SP_VAUXIA), 1), (Sp(SP_OLENOIDES), 1)],
    },
    Mission {
        title: L("バージェスのもっと奥へ", "Deeper into the Burgess", "深入伯吉斯"),
        brief: L(
            "バージェスにはまだまだ仲間がいます。そして最後に、海の王者アノマロカリスを、画面いっぱいに大きく撮影してきてください（★3）。",
            "There are more Burgess animals to find. Finally, fill the frame with the king of the seas, Anomalocaris (★3).",
            "伯吉斯还有更多动物。最后，请把海洋霸主奇虾拍得满满一画面（★3）。",
        ),
        done: L(
            "大迫力の写真！ 学校のみんなもびっくりするでしょう。",
            "What a powerful photo! The whole class will be amazed.",
            "震撼的照片！学校的同学们一定会大吃一惊。",
        ),
        targets: &[
            (Sp(SP_AYSHEAIA), 1),
            (Sp(SP_SIDNEYIA), 1),
            (Sp(SP_NECTOCARIS), 1),
            (Sp(SP_LEANCHOILIA), 1),
            (Sp(SP_CAMBRORASTER), 1),
            (Sp(SP_JELLY), 1),
            (Sp(SP_ANOMALOCARIS), 3),
        ],
    },
    Mission {
        title: L("深い海と遠い海", "Deep and distant seas", "深海与远洋"),
        brief: L(
            "5億年前ごろのアヴァロニアの海で大きな三葉虫パラドキシデスを。それから外洋の深い海へ潜り、放散虫とガラス海綿を探しましょう。深い海は暗いので、ライトを点けて。",
            "Find the big trilobite Paradoxides in the Avalonian sea about 500 million years ago. Then dive into the deep open ocean for radiolarians and glass sponges. Turn on your light in the dark depths.",
            "在约5亿年前的阿瓦隆尼亚海域寻找大型三叶虫奇异虫。然后潜入远洋深海寻找放射虫和玻璃海绵。深海很暗，记得打开灯。",
        ),
        done: L(
            "深海の観察までできるなんて、立派な探検家ですね。",
            "Observing the deep sea too? You are a real explorer.",
            "连深海都观察到了，真是了不起的探险家。",
        ),
        targets: &[(Sp(SP_PARADOXIDES), 1), (Sp(SP_RADIOLARIAN), 1), (Sp(SP_GLASS_SPONGE), 1)],
    },
    Mission {
        title: L("カンブリア紀の終わりへ", "Towards the end of the Cambrian", "走向寒武纪的终结"),
        brief: L(
            "最後の課題です。酸素の少ないバルティカの海の三葉虫たち、干潟に上がってきた動物、そしてコノドント動物を記録して、カンブリア紀の図鑑を完成させましょう。",
            "Final assignment: the trilobites of Baltica's oxygen-poor sea, the animal that crawled onto the tidal flats, and the conodont animal. Complete your Cambrian field guide!",
            "最后的课题：记录缺氧的波罗的海中的三叶虫、爬上潮坪的动物和牙形动物，完成寒武纪图鉴吧。",
        ),
        done: L(
            "カンブリア紀の宿題、すべて合格です！ 次の時代、オルドビス紀への旅も楽しみにしていてください。",
            "Every Cambrian assignment passed! Look forward to your next journey, into the Ordovician.",
            "寒武纪的作业全部合格！敬请期待下一段奥陶纪之旅。",
        ),
        targets: &[(Sp(SP_OLENUS), 1), (Sp(SP_AGNOSTUS), 1), (Sp(SP_CLIMACTICHNITES), 1), (Sp(SP_CONODONT), 1)],
    },
];

/// 宿題を達成したか
pub fn mission_done(m: &Mission, stars: &[u8]) -> bool {
    m.targets.iter().all(|&(t, n)| stars.get(t.entry()).copied().unwrap_or(0) >= n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_match_names() {
        assert_eq!(SPECIES[SP_GLASS_SPONGE].sci, "Protospongia");
        assert_eq!(SPECIES[SP_ANOMALOCARIS].sci, "Anomalocaris canadensis");
        assert_eq!(SPECIES[SP_MARRELLA].sci, "Marrella splendens");
        assert_eq!(SPECIES[SP_CONODONT].sci, "Proconodontus");
        assert_eq!(SPECIES.len(), SP_GLASS_SPONGE + 1);
    }
}
