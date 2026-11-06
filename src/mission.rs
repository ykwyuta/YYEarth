//! 学校の宿題（ミッション）：トキ先生から出される課題。撮影して図鑑に収録すると達成
use crate::earth::*;
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
    /// どの紀の宿題か
    pub period: usize,
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
pub const SP_AEGIROCASSIS: usize = 36;
pub const SP_CAMEROCERAS: usize = 37;
pub const SP_ASAPHUS: usize = 38;
pub const SP_ISOTELUS: usize = 39;
pub const SP_SACABAMBASPIS: usize = 40;
pub const SP_GRAPTOLITE: usize = 41;
pub const SP_STREPTELASMA: usize = 42;
pub const SP_CRINOID: usize = 43;
pub const SP_CONSTELLARIA: usize = 44;
pub const SP_RAFINESQUINA: usize = 45;
pub const SP_MEGALOGRAPTUS: usize = 46;
pub const SP_EARLY_LAND_PLANTS: usize = 47;
pub const SP_STROMATOPOROID: usize = 48;
pub const SP_COOKSONIA: usize = 49;
pub const SP_BARAGWANATHIA: usize = 50;
pub const SP_PNEUMODESMUS: usize = 51;
pub const SP_FAVOSITES: usize = 52;
pub const SP_CALYMENE: usize = 53;
pub const SP_GUIYU: usize = 54;
pub const SP_ENTELOGNATHUS: usize = 55;
pub const SP_BIRKENIA: usize = 56;
pub const SP_EURYPTERUS: usize = 57;
pub const SP_PROTOTAXITES: usize = 58;
pub const SP_RHYNIA: usize = 59;
pub const SP_PALAEOCHARINUS: usize = 60;
pub const SP_SCHINDERHANNES: usize = 61;
pub const SP_WALLISEROPS: usize = 62;
pub const SP_JAEKELOPTERUS: usize = 63;
pub const SP_EOSPERMATOPTERIS: usize = 64;
pub const SP_ARCHAEOPTERIS: usize = 65;
pub const SP_MATERPISCIS: usize = 66;
pub const SP_MANTICOCERAS: usize = 67;
pub const SP_BOTHRIOLEPIS: usize = 68;
pub const SP_EUSTHENOPTERON: usize = 69;
pub const SP_TIKTAALIK: usize = 70;
pub const SP_ACANTHOSTEGA: usize = 71;
pub const SP_ICHTHYOSTEGA: usize = 72;
pub const SP_DUNKLEOSTEUS: usize = 73;
pub const SP_CLADOSELACHE: usize = 74;
pub const SP_STETHACANTHUS: usize = 75;
pub const SP_GIGANTOPRODUCTUS: usize = 76;
pub const SP_LEPIDODENDRON: usize = 77;
pub const SP_CALAMITES: usize = 78;
pub const SP_PULMONOSCORPIUS: usize = 79;
pub const SP_ARTHROPLEURA: usize = 80;
pub const SP_HYLONOMUS: usize = 81;
pub const SP_DENDRERPETON: usize = 82;
pub const SP_TULLIMONSTRUM: usize = 83;
pub const SP_MEGANEURA: usize = 84;
pub const SP_ARCHIMYLACRIS: usize = 85;
pub const SP_PSARONIUS: usize = 86;
pub const SP_MESOSAURUS: usize = 87;
pub const SP_GLOSSOPTERIS: usize = 88;
pub const SP_WALCHIA: usize = 89;
pub const SP_ERYOPS: usize = 90;
pub const SP_DIPLOCAULUS: usize = 91;
pub const SP_DIMETRODON: usize = 92;
pub const SP_HELICOPRION: usize = 93;
pub const SP_PARAFUSULINA: usize = 94;
pub const SP_DITOMOPYGE: usize = 95;
pub const SP_INOSTRANCEVIA: usize = 96;
pub const SP_SCUTOSAURUS: usize = 97;
pub const SP_LYSTROSAURUS: usize = 98;

pub static MISSIONS: &[Mission] = &[
    Mission {
        period: CAMBRIAN,
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
        period: CAMBRIAN,
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
        period: CAMBRIAN,
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
        period: CAMBRIAN,
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
        period: CAMBRIAN,
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
        period: CAMBRIAN,
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
        period: CAMBRIAN,
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
        period: CAMBRIAN,
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
        period: CAMBRIAN,
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
        period: CAMBRIAN,
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
    // ======================================================== オルドビス紀
    Mission {
        period: ORDOVICIAN,
        title: L("フェズアタの冷たい海", "The cold Fezouata sea", "费祖阿塔的寒冷海洋"),
        brief: L(
            "オルドビス紀のはじめ、4億7800万年前。南極に近い冷たい海（いまのモロッコ）に、アノマロカリスの子孫の巨大な生きものがいます。海をただよう筆石、海底のウミユリも記録しましょう。",
            "The early Ordovician, 478 million years ago. In a cold sea near the South Pole (now Morocco) lives a giant descendant of Anomalocaris. Record the drifting graptolites and the crinoids on the sea floor too.",
            "奥陶纪初期，4亿7800万年前。靠近南极的寒冷海洋（今摩洛哥）中生活着奇虾的巨大后代。也记录下漂浮的笔石和海底的海百合吧。",
        ),
        done: L(
            "あのアノマロカリスの仲間が、こんなに大きくおだやかな姿になっていたなんて。進化はおもしろいですね。",
            "Who would have thought Anomalocaris's relatives would become such huge, gentle giants? Evolution is full of surprises.",
            "奇虾的亲戚竟然变成了如此巨大而温和的模样。演化真有趣！",
        ),
        targets: &[(Sp(SP_AEGIROCASSIS), 1), (Sp(SP_GRAPTOLITE), 1), (Sp(SP_CRINOID), 1)],
    },
    Mission {
        period: ORDOVICIAN,
        title: L("ゴンドワナの浅い海", "Gondwana's shallow sea", "冈瓦纳的浅海"),
        brief: L(
            "4億6500万年前、ゴンドワナの西の浅い海（いまのボリビア）へ。とぼけた顔の魚サカバンバスピスを、顔がよく見えるように★2以上で撮ってください。それから岸辺に上がって、陸に生えはじめた最初の植物も探しましょう。",
            "Visit the shallow sea off western Gondwana (now Bolivia) 465 million years ago. Photograph the goofy-faced fish Sacabambaspis at ★2 or better so we can see its face. Then go ashore and look for the first plants on land.",
            "前往4亿6500万年前冈瓦纳西部的浅海（今玻利维亚）。请把呆萌的萨卡班巴鱼拍到★2以上，让它的脸清清楚楚。然后上岸寻找最早在陆地上生长的植物。",
        ),
        done: L(
            "あごのない魚と、地面にはりつく小さな植物。どちらも、のちの大きな進化の出発点です。",
            "A jawless fish and tiny plants hugging the ground — both are starting points for great things to come.",
            "没有颌的鱼和贴地生长的小植物，都是日后伟大演化的起点。",
        ),
        targets: &[(Sp(SP_SACABAMBASPIS), 2), (Sp(SP_EARLY_LAND_PLANTS), 1)],
    },
    Mission {
        period: ORDOVICIAN,
        title: L("バルティカの石灰の海", "Baltica's limestone sea", "波罗的的石灰岩海"),
        brief: L(
            "バルティカ大陸の浅い海（いまのロシア・サンクトペテルブルク）。眼が柄の先についた三葉虫と、まっすぐな長い殻のオウムガイの仲間を探してください。オウムガイは大きくて用心深いので、ゆっくり近づくこと。",
            "The shallow sea of Baltica (now St Petersburg, Russia). Find the trilobite with eyes on stalks and the nautiloid with a long straight shell. The nautiloid is big and wary, so approach slowly.",
            "波罗的大陆的浅海（今俄罗斯圣彼得堡）。寻找眼睛长在长柄上的三叶虫和拥有笔直长壳的鹦鹉螺类。鹦鹉螺体型大又警觉，要慢慢靠近。",
        ),
        done: L(
            "潜望鏡のような眼と、巨大な殻。オルドビス紀は、海の生きものの種類が一気にふえた時代でした。",
            "Periscope eyes and a gigantic shell! The Ordovician was a time when sea life suddenly became far more varied.",
            "潜望镜般的眼睛和巨大的外壳。奥陶纪是海洋生物种类迅速增多的时代。",
        ),
        targets: &[(Sp(SP_ASAPHUS), 1), (Sp(SP_CAMEROCERAS), 1)],
    },
    Mission {
        period: ORDOVICIAN,
        title: L("シンシナティの海", "The Cincinnati sea", "辛辛那提之海"),
        brief: L(
            "4億5000万年前、ローレンシア大陸の暖かい浅い海（いまのアメリカ・オハイオ州）。殻の敷き詰められた海底に、巨大な三葉虫やウミサソリが暮らしています。サンゴ・コケムシ・腕足動物と合わせて記録しましょう。",
            "The warm shallow sea of Laurentia 450 million years ago (now Ohio, USA). Giant trilobites and sea scorpions live on a floor paved with shells. Record them along with the corals, bryozoans and brachiopods.",
            "4亿5000万年前劳伦大陆温暖的浅海（今美国俄亥俄州）。在铺满贝壳的海底，生活着巨大的三叶虫和海蝎。连同珊瑚、苔藓虫和腕足动物一起记录吧。",
        ),
        done: L(
            "サンゴやコケムシがつくる礁に、たくさんの殻。オルドビス紀の生物大放散の、にぎやかな海でした。",
            "Reefs of corals and bryozoans and shells everywhere — the busy sea of the Great Ordovician Biodiversification.",
            "珊瑚和苔藓虫建造的礁，遍地的贝壳——这是奥陶纪生物大辐射时期热闹的海洋。",
        ),
        targets: &[(Sp(SP_ISOTELUS), 1), (Sp(SP_CONSTELLARIA), 1), (Sp(SP_RAFINESQUINA), 1), (Sp(SP_STREPTELASMA), 1), (Sp(SP_MEGALOGRAPTUS), 1)],
    },
    Mission {
        period: ORDOVICIAN,
        title: L("氷の時代へ", "Into the ice age", "走进冰期"),
        brief: L(
            "4億4500万年前、南極の上にあるゴンドワナ大陸に氷河が広がっています。陸に降りて、氷の世界を撮影してください。とても寒いので、カプセルの外には出ないこと！",
            "445 million years ago, glaciers spread over Gondwana at the South Pole. Land there and photograph the frozen world. It is bitterly cold, so stay inside the capsule!",
            "4亿4500万年前，位于南极的冈瓦纳大陆上冰川扩张。请降落并拍摄冰雪世界。外面非常寒冷，千万不要离开观测舱！",
        ),
        done: L(
            "この寒さで、海の生きものの85%が絶滅しました。でも生き残ったものたちが、次のシルル紀をつくっていきます。",
            "This cold wiped out 85% of marine species. But the survivors went on to build the Silurian world.",
            "这场严寒导致85%的海洋物种灭绝。但幸存者们开创了接下来的志留纪。",
        ),
        targets: &[(Ph(P_HIRNANTIAN_ICE), 1)],
    },
    // ======================================================== シルル紀
    Mission {
        period: SILURIAN,
        title: L("最初の緑と陸の動物", "First greenery, first land animals", "最早的绿色与陆地动物"),
        brief: L(
            "シルル紀、4億2700万年前のスコットランド。水辺に小さな植物クックソニアが生え、空気を吸う最初の動物、ヤスデが歩きはじめました。陸に降りて、緑の岸辺の景色も撮りましょう。",
            "Scotland in the Silurian, 427 million years ago. The tiny plant Cooksonia grows by the water and the first air-breathing animal, a millipede, has begun to walk. Land and photograph the green shoreline too.",
            "志留纪，4亿2700万年前的苏格兰。水边长出了小小的库克逊蕨，最早呼吸空气的动物马陆开始行走。降落到陆地上，也拍下绿色岸边的风景吧。",
        ),
        done: L(
            "小さな植物と小さなヤスデ。ここから、陸の生きものの長い歴史が始まります。",
            "A tiny plant and a tiny millipede: the long story of life on land begins here.",
            "小小的植物和小小的马陆，陆地生命的漫长历史从这里开始。",
        ),
        targets: &[(Sp(SP_COOKSONIA), 1), (Sp(SP_PNEUMODESMUS), 1), (Ph(P_GREEN_SHORE), 1)],
    },
    Mission {
        period: SILURIAN,
        title: L("シルル紀の礁", "Silurian reefs", "志留纪的礁"),
        brief: L(
            "暖かい浅い海に、サンゴと層孔虫の礁が広がっています（いまのスウェーデン・ゴトランド島やイギリス・ダドリー）。蜂の巣のようなサンゴ、ドーム形の層孔虫、体を丸める三葉虫カリメネを撮影してください。",
            "Reefs of corals and stromatoporoids fill the warm shallow seas (now Gotland in Sweden and Dudley in England). Photograph the honeycomb coral, the dome-shaped stromatoporoid and Calymene, the trilobite that rolls into a ball.",
            "温暖的浅海中遍布珊瑚和层孔虫建造的礁（今瑞典哥得兰岛和英国达德利）。请拍摄蜂巢般的珊瑚、穹顶状的层孔虫，以及会卷成一团的三叶虫隐头虫。",
        ),
        done: L(
            "礁は、海の生きもののアパートのようなもの。たくさんの生きものが集まっていましたね。",
            "A reef is like an apartment block for sea creatures — so many of them gathered there!",
            "礁就像海洋生物的公寓楼，聚集了许多生物。",
        ),
        targets: &[(Sp(SP_FAVOSITES), 1), (Sp(SP_STROMATOPOROID), 1), (Sp(SP_CALYMENE), 1)],
    },
    Mission {
        period: SILURIAN,
        title: L("あごの誕生", "The rise of jaws", "颌的诞生"),
        brief: L(
            "4億2400万年前、南中国の海（いまの雲南省）。骨のあごを持つ魚たちが現れました。わたしたちにつながる最古の硬骨魚グイユーと、鎧をまとったエンテログナトゥスを探してください。",
            "The South China sea 424 million years ago (now Yunnan). Fish with bony jaws have appeared. Find Guiyu, one of the oldest bony fishes on our own family line, and the armoured Entelognathus.",
            "4亿2400万年前的华南海域（今云南省）。长着骨质颌的鱼出现了。寻找通向我们的最古老硬骨鱼之一梦幻鬼鱼，以及披着盔甲的初始全颌鱼。",
        ),
        done: L(
            "あごを持ったことで、魚は大きな獲物をかみ、いろいろなものを食べられるようになりました。",
            "With jaws, fish could bite big prey and eat all sorts of new foods.",
            "有了颌，鱼类就能咬住大猎物，吃各种各样的食物了。",
        ),
        targets: &[(Sp(SP_GUIYU), 1), (Sp(SP_ENTELOGNATHUS), 1)],
    },
    Mission {
        period: SILURIAN,
        title: L("ウミサソリの入り江", "Lagoon of the sea scorpions", "海蝎的潟湖"),
        brief: L(
            "シルル紀の終わり、ローレンシアの浅い入り江（いまのニューヨーク州）。ウミサソリのユーリプテルスを★2以上で撮影してください。群れで泳ぐ小魚ビルケニア、南のゴンドワナに生えた葉のある植物バラグワナチアも記録しましょう。",
            "The end of the Silurian, in a shallow Laurentian lagoon (now New York). Photograph the sea scorpion Eurypterus at ★2 or better. Also record the shoaling little fish Birkenia and Baragwanathia, a leafy plant growing far south on Gondwana.",
            "志留纪末，劳伦大陆的浅潟湖（今纽约州）。请把海蝎广翅鲎拍到★2以上。也记录成群游动的小鱼伯肯鱼，以及生长在南方冈瓦纳的有叶植物巴拉格瓦纳蕨。",
        ),
        done: L(
            "シルル紀の図鑑も充実してきました。次はいよいよ「魚の時代」、デボン紀です！",
            "Your Silurian guide is looking great. Next comes the Devonian — the Age of Fishes!",
            "志留纪图鉴也充实起来了。接下来就是“鱼类时代”泥盆纪！",
        ),
        targets: &[(Sp(SP_EURYPTERUS), 2), (Sp(SP_BIRKENIA), 1), (Sp(SP_BARAGWANATHIA), 1)],
    },
    // ======================================================== デボン紀
    Mission {
        period: DEVONIAN,
        title: L("ライニーの温泉湿地", "The Rhynie hot springs", "莱尼温泉湿地"),
        brief: L(
            "デボン紀前期、4億800万年前のスコットランド。温泉の湧く湿地に、小さな植物と小さな動物が暮らしています。柱のような巨大なキノコの仲間、プロトタキシーテスも探してみましょう。",
            "Scotland in the Early Devonian, 408 million years ago. Small plants and tiny animals live in a hot-spring wetland. Look for Prototaxites, the pillar-like giant fungus, too.",
            "早泥盆世，4亿800万年前的苏格兰。温泉湿地中生活着小植物和小动物。也找找柱子般的巨型真菌原杉藻吧。",
        ),
        done: L(
            "ライニーでは、温泉のおかげで生きものが細胞まで化石になりました。まるで時間を止めた写真のようです。",
            "At Rhynie, the hot springs fossilised life right down to its cells — like a photograph of frozen time.",
            "在莱尼，温泉让生物连细胞都变成了化石，就像定格时间的照片。",
        ),
        targets: &[(Sp(SP_RHYNIA), 1), (Sp(SP_PALAEOCHARINUS), 1), (Sp(SP_PROTOTAXITES), 1)],
    },
    Mission {
        period: DEVONIAN,
        title: L("生き残りと三つまたの槍", "Survivors and tridents", "幸存者与三叉戟"),
        brief: L(
            "フンスリュックの海（いまのドイツ）には、アノマロカリスの仲間の生き残りがいます。ゴンドワナの海の三つまたの槍を持つ三葉虫、河口にひそむ史上最大級のウミサソリも探してください。",
            "In the Hunsrück sea (now Germany) lives a late survivor of the Anomalocaris family. Also find the trilobite with a trident in Gondwana's seas, and the record-breaking sea scorpion lurking in an estuary.",
            "洪斯吕克海（今德国）中生活着奇虾家族的幸存者。也去寻找冈瓦纳海中长着三叉戟的三叶虫，以及潜伏在河口的史上最大级海蝎。",
        ),
        done: L(
            "カンブリア紀から1億年も生き延びた仲間がいたとは、驚きですね。",
            "Imagine a relative surviving 100 million years after the Cambrian!",
            "竟然有亲戚从寒武纪之后又存活了1亿年，真令人惊讶。",
        ),
        targets: &[(Sp(SP_SCHINDERHANNES), 1), (Sp(SP_WALLISEROPS), 1), (Sp(SP_JAEKELOPTERUS), 1)],
    },
    Mission {
        period: DEVONIAN,
        title: L("ユーラメリカと最初の森", "Euramerica and the first forests", "欧美大陆与最早的森林"),
        brief: L(
            "ローレンシアとバルティカがぶつかってできたユーラメリカ大陸を、上空から撮影してください。それから陸に降りて、地球で最初の森の木を記録しましょう。",
            "Photograph Euramerica, born when Laurentia and Baltica collided, from the sky. Then land and record the trees of Earth's first forests.",
            "请从高空拍摄劳伦与波罗的碰撞形成的欧美大陆。然后降落到陆地上，记录地球上最早森林中的树木。",
        ),
        done: L(
            "森ができると、土ができ、川の流れや空気まで変わります。地球の景色を変えた大事件でした。",
            "Forests made soil and changed the rivers and even the air. It was an event that transformed the face of the Earth.",
            "森林形成后，土壤随之出现，连河流和空气都改变了。这是改变地球面貌的大事件。",
        ),
        targets: &[(Ph(P_EURAMERICA), 1), (Sp(SP_EOSPERMATOPTERIS), 1), (Sp(SP_ARCHAEOPTERIS), 1), (Ph(P_FIRST_FOREST), 1)],
    },
    Mission {
        period: DEVONIAN,
        title: L("ゴーゴーの礁", "The Gogo reef", "戈戈礁"),
        brief: L(
            "3億8200万年前、ゴンドワナの暖かい礁（いまのオーストラリア西部）。お腹に赤ちゃんを宿した魚マテルピスキスと、巻いた殻のアンモノイドを探してください。",
            "A warm Gondwanan reef 382 million years ago (now Western Australia). Find Materpiscis, the fish that carried a baby inside, and the coiled-shelled ammonoid.",
            "3亿8200万年前冈瓦纳温暖的礁（今澳大利亚西部）。寻找腹中怀着幼鱼的母鱼，以及拥有螺旋外壳的菊石类。",
        ),
        done: L(
            "魚が赤ちゃんを産んでいたなんて！ 化石は、親子の物語まで教えてくれます。",
            "A fish that gave birth! Fossils can even tell us stories of mothers and babies.",
            "鱼竟然直接生宝宝！化石甚至能讲述母子的故事。",
        ),
        targets: &[(Sp(SP_MATERPISCIS), 1), (Sp(SP_MANTICOCERAS), 1)],
    },
    Mission {
        period: DEVONIAN,
        title: L("水から陸へ", "From water to land", "从水到陆"),
        brief: L(
            "川の河口や浅瀬に、陸へ上がる準備をしている魚たちがいます。ユーステノプテロンとボスリオレピス、そして浅瀬から顔を出すティクターリクを★2以上で撮ってください。",
            "In estuaries and shallows, fish are getting ready to move onto land. Photograph Eusthenopteron and Bothriolepis, and Tiktaalik lifting its head from the shallows at ★2 or better.",
            "在河口和浅滩，鱼儿们正在为登上陆地做准备。请拍摄真掌鳍鱼和沟鳞鱼，并把从浅滩探出头的提塔利克鱼拍到★2以上。",
        ),
        done: L(
            "ひれの中の骨が、やがて腕と足になります。わたしたちの腕のルーツはここにあるのです。",
            "The bones inside these fins will become arms and legs. The roots of your own arms are right here.",
            "鳍中的骨头终将变成手臂和腿。我们手臂的根源就在这里。",
        ),
        targets: &[(Sp(SP_EUSTHENOPTERON), 1), (Sp(SP_BOTHRIOLEPIS), 1), (Sp(SP_TIKTAALIK), 2)],
    },
    Mission {
        period: DEVONIAN,
        title: L("四本足の誕生", "The first four-legged animals", "四足的诞生"),
        brief: L(
            "デボン紀の終わり、東グリーンランドの川。足を持つ最初の動物たち、アカントステガとイクチオステガを記録しましょう。",
            "The rivers of East Greenland at the end of the Devonian. Record Acanthostega and Ichthyostega, some of the first animals with legs.",
            "泥盆纪末东格陵兰的河流。记录最早长出四肢的动物——棘螈和鱼石螈。",
        ),
        done: L(
            "足は水の中で生まれ、やがて陸を歩くために使われるようになりました。",
            "Legs were born in the water, and only later used for walking on land.",
            "四肢诞生于水中，后来才被用来在陆地上行走。",
        ),
        targets: &[(Sp(SP_ACANTHOSTEGA), 1), (Sp(SP_ICHTHYOSTEGA), 1)],
    },
    Mission {
        period: DEVONIAN,
        title: L("デボン紀の海の王", "King of the Devonian seas", "泥盆纪的海洋之王"),
        brief: L(
            "クリーブランドの海（いまのオハイオ州）には、鎧の頭を持つ巨大魚ダンクルオステウスがいます。古いサメのクラドセラケも探し、最後にダンクルオステウスを画面いっぱいに★3で撮影してください。用心深いので、そっと！",
            "The Cleveland sea (now Ohio) is home to Dunkleosteus, the giant armour-headed fish. Find the early shark Cladoselache too, and finally fill the frame with Dunkleosteus for ★3. It is wary — go gently!",
            "克利夫兰海（今俄亥俄州）中生活着头披骨甲的巨鱼邓氏鱼。也去找找早期鲨鱼裂口鲨，最后把邓氏鱼拍满画面达到★3。它很警觉，要悄悄靠近！",
        ),
        done: L(
            "迫力満点の一枚です！ デボン紀の宿題、合格です。",
            "What a dramatic shot! Your Devonian assignments are complete.",
            "气势十足的一张照片！泥盆纪作业合格。",
        ),
        targets: &[(Sp(SP_CLADOSELACHE), 1), (Sp(SP_DUNKLEOSTEUS), 3)],
    },
    // ======================================================== 石炭紀
    Mission {
        period: CARBONIFEROUS,
        title: L("石炭紀の海", "Carboniferous seas", "石炭纪的海洋"),
        brief: L(
            "石炭紀の暖かい浅い海へ。アイロン台のような背びれのサメの仲間ステタカントゥスと、史上最大の腕足動物ギガントプロダクトゥスを探してください。",
            "Head into the warm, shallow seas of the Carboniferous. Find Stethacanthus, the shark relative with an ironing-board fin, and Gigantoproductus, the largest brachiopod ever.",
            "前往石炭纪温暖的浅海。寻找背鳍像熨衣板的鲨类胸脊鲨，以及史上最大的腕足动物大长身贝。",
        ),
        done: L(
            "石炭紀の海には、ふしぎな形のサメの仲間がたくさんいたのです。",
            "Carboniferous seas were full of strangely shaped shark relatives.",
            "石炭纪的海洋中有许多形状奇特的鲨类。",
        ),
        targets: &[(Sp(SP_STETHACANTHUS), 1), (Sp(SP_GIGANTOPRODUCTUS), 1)],
    },
    Mission {
        period: CARBONIFEROUS,
        title: L("巨大サソリの湖畔", "Lakeshore of the giant scorpion", "巨蝎的湖畔"),
        brief: L(
            "3億3500万年前、スコットランドの湖のほとり。体長70cmの巨大サソリが歩いています。鱗木やカラミテスの森も撮影しましょう。サソリには近づきすぎないこと！",
            "A Scottish lakeshore 335 million years ago, where a 70 cm scorpion roams. Photograph the scale trees and Calamites too. Don't get too close to that scorpion!",
            "3亿3500万年前苏格兰的湖畔，体长70厘米的巨蝎在行走。也拍下鳞木和芦木的森林吧。别靠蝎子太近！",
        ),
        done: L(
            "空気中の酸素が多かったこの時代、陸の節足動物はとても大きくなりました。",
            "With so much oxygen in the air, land arthropods grew huge in this age.",
            "在空气中氧气丰富的这个时代，陆地节肢动物长得非常巨大。",
        ),
        targets: &[(Sp(SP_PULMONOSCORPIUS), 1), (Sp(SP_LEPIDODENDRON), 1), (Sp(SP_CALAMITES), 1)],
    },
    Mission {
        period: CARBONIFEROUS,
        title: L("石炭の森と南の氷河", "Coal forests and southern ice", "煤炭森林与南方冰川"),
        brief: L(
            "赤道近くの湿地に広がる「石炭の森」を、陸に降りて撮影してください。森の床には巨大なヤスデのアースロプレウラや、ゴキブリの仲間がいます。同じころ、南のゴンドワナには大氷河がありました。そちらも撮ってきましょう。",
            "Land in the 'coal forests' of the equatorial swamps and photograph them. The giant millipede Arthropleura and early cockroach relatives roam the forest floor. Meanwhile great glaciers covered southern Gondwana — photograph those too.",
            "降落到赤道附近沼泽中的“煤炭森林”拍摄。森林地面上有巨型马陆节胸蜈蚣和蟑螂的祖先。同一时期，南方的冈瓦纳覆盖着大冰川，也去拍下来吧。",
        ),
        done: L(
            "暑い湿地林と、南の大氷河。同じ地球の同じ時代とは思えませんね。",
            "Steamy swamp forests and vast southern glaciers — hard to believe they share the same planet and time!",
            "炎热的沼泽森林和南方的大冰川，真难相信是同一个地球、同一个时代。",
        ),
        targets: &[(Ph(P_COAL_FOREST), 1), (Sp(SP_ARTHROPLEURA), 1), (Sp(SP_ARCHIMYLACRIS), 1), (Ph(P_GONDWANA_ICE), 1)],
    },
    Mission {
        period: CARBONIFEROUS,
        title: L("ジョギンズの化石の森", "The fossil forest of Joggins", "乔金斯化石森林"),
        brief: L(
            "3億1600万年前、いまのカナダ・ノバスコシア州。殻のある卵を産む最初の爬虫類ヒロノムスと、両生類デンドレルペトンを探してください。",
            "Nova Scotia, Canada, 316 million years ago. Find Hylonomus, one of the first reptiles to lay shelled eggs, and the amphibian Dendrerpeton.",
            "3亿1600万年前的加拿大新斯科舍省。寻找最早产有壳卵的爬行动物之一林蜥，以及两栖动物树螈。",
        ),
        done: L(
            "殻のある卵のおかげで、動物は水辺を離れて暮らせるようになりました。恐竜やわたしたち哺乳類も、この仲間の子孫です。",
            "The shelled egg let animals leave the water's edge behind. Dinosaurs — and mammals like us — descend from these pioneers.",
            "有壳的卵让动物能够离开水边生活。恐龙和我们哺乳动物都是它们的后代。",
        ),
        targets: &[(Sp(SP_HYLONOMUS), 1), (Sp(SP_DENDRERPETON), 1)],
    },
    Mission {
        period: CARBONIFEROUS,
        title: L("空の王者とふしぎな怪物", "Sky king and mystery monster", "天空霸主与神秘怪物"),
        brief: L(
            "石炭紀の終わり。メゾンクリークの三角州で正体不明の「トゥリーモンスター」を、フランスの沼地で巨大トンボのメガネウラを★2以上で撮影してください。木生シダの森も記録しましょう。",
            "The end of the Carboniferous. Photograph the mysterious 'Tully monster' in the Mazon Creek delta and the giant griffinfly Meganeura in a French swamp at ★2 or better. Record the tree-fern forest too.",
            "石炭纪末。在梅森溪三角洲拍摄身份不明的“塔利怪物”，在法国沼泽把巨型蜻蜓巨脉蜻蜓拍到★2以上。也记录树蕨森林吧。",
        ),
        done: L(
            "メガネウラは飛ぶのがとても速いので、★2は見事です！ 石炭紀の宿題、合格です。",
            "Meganeura is a very fast flier, so ★2 is superb! Your Carboniferous assignments are complete.",
            "巨脉蜻蜓飞得非常快，拍到★2真了不起！石炭纪作业合格。",
        ),
        targets: &[(Sp(SP_TULLIMONSTRUM), 1), (Sp(SP_MEGANEURA), 2), (Sp(SP_PSARONIUS), 1)],
    },
    // ======================================================== ペルム紀
    Mission {
        period: PERMIAN,
        title: L("超大陸パンゲア", "Supercontinent Pangaea", "盘古超大陆"),
        brief: L(
            "ペルム紀、ほとんどの大陸が一つになった超大陸パンゲアを上空から撮影してください。それからゴンドワナの入り海でメソサウルスを、陸でグロッソプテリスの森を探しましょう。",
            "In the Permian, photograph Pangaea — almost every continent joined into one — from the sky. Then find Mesosaurus in a Gondwanan inland sea and the Glossopteris forests on land.",
            "二叠纪，请从高空拍摄几乎所有大陆合而为一的盘古超大陆。然后在冈瓦纳的内海寻找中龙，在陆地上寻找舌羊齿森林。",
        ),
        done: L(
            "メソサウルスとグロッソプテリスの化石が、いまは海で隔てられた大陸で見つかることが、大陸が動いた証拠になったのです。",
            "Finding Mesosaurus and Glossopteris fossils on continents now separated by oceans became proof that the continents move.",
            "中龙和舌羊齿的化石在如今被海洋分隔的大陆上都有发现，这成为了大陆漂移的证据。",
        ),
        targets: &[(Ph(P_PANGAEA), 1), (Sp(SP_MESOSAURUS), 1), (Sp(SP_GLOSSOPTERIS), 1)],
    },
    Mission {
        period: PERMIAN,
        title: L("テキサスの赤い大地", "The red lands of Texas", "得克萨斯的红色大地"),
        brief: L(
            "2億9000万年前、パンゲアの赤い大地（いまのアメリカ・テキサス州）。背中に帆を持つディメトロドンを★2以上で撮ってください。水辺の両生類エリオプスとディプロカウルス、針葉樹ワルキアも記録しましょう。",
            "Pangaea's red lands 290 million years ago (now Texas, USA). Photograph sail-backed Dimetrodon at ★2 or better, and record the waterside amphibians Eryops and Diplocaulus and the conifer Walchia.",
            "2亿9000万年前盘古大陆的红色大地（今美国得克萨斯州）。请把背上长帆的异齿龙拍到★2以上，并记录水边的两栖动物引螈和笠头螈，以及针叶树瓦契杉。",
        ),
        done: L(
            "ディメトロドンは恐竜ではなく、わたしたち哺乳類の遠い親戚。ちょっと親しみがわきますね。",
            "Dimetrodon isn't a dinosaur but a distant relative of us mammals. That makes it feel a bit like family!",
            "异齿龙不是恐龙，而是我们哺乳动物的远亲，是不是有点亲切感？",
        ),
        targets: &[(Sp(SP_DIMETRODON), 2), (Sp(SP_ERYOPS), 1), (Sp(SP_DIPLOCAULUS), 1), (Sp(SP_WALCHIA), 1)],
    },
    Mission {
        period: PERMIAN,
        title: L("ペルム紀の海", "Permian seas", "二叠纪的海洋"),
        brief: L(
            "ペルム紀の海で、うずまきの歯を持つヘリコプリオン、米つぶのような紡錘虫、そして最後の三葉虫ディトモピゲを探してください。",
            "In the Permian seas, find Helicoprion with its spiral of teeth, the rice-grain-shaped fusulinids, and Ditomopyge, one of the last trilobites.",
            "在二叠纪的海洋中寻找长着螺旋齿的旋齿鲨、米粒般的纺锤蜓，以及最后的三叶虫之一双切尾虫。",
        ),
        done: L(
            "三葉虫は古生代のはじめから終わりまで生きた動物。最後の三葉虫まで撮れましたね。",
            "Trilobites lived from the very start of the Paleozoic to its end — and you've photographed one of the last.",
            "三叶虫从古生代开始一直生存到结束，你连最后的三叶虫都拍到了。",
        ),
        targets: &[(Sp(SP_HELICOPRION), 1), (Sp(SP_PARAFUSULINA), 1), (Sp(SP_DITOMOPYGE), 1)],
    },
    Mission {
        period: PERMIAN,
        title: L("砂漠と大型の獣", "Deserts and great beasts", "沙漠与大型兽类"),
        brief: L(
            "ペルム紀の後半、パンゲアの内陸には砂漠が広がりました。砂漠の景色を撮影したら、北の川辺（いまのロシア）で剣の歯を持つイノストランケビアと、骨のこぶのスクトサウルスを探しましょう。",
            "In the Late Permian, deserts spread across Pangaea's interior. Photograph the desert, then find sabre-toothed Inostrancevia and knobbly Scutosaurus on the northern riverlands (now Russia).",
            "二叠纪后期，盘古大陆内陆沙漠广布。拍下沙漠风景后，到北方的河岸（今俄罗斯）寻找长着军刀般牙齿的狼蜥兽和满身骨瘤的盾甲龙。",
        ),
        done: L(
            "大きな捕食者と大きな植物食の動物。ペルム紀の陸は、もう立派な生態系でした。",
            "Big predators and big plant-eaters: Permian land had become a full ecosystem.",
            "大型捕食者和大型植食动物——二叠纪的陆地已经是成熟的生态系统了。",
        ),
        targets: &[(Ph(P_DESERT), 1), (Sp(SP_INOSTRANCEVIA), 1), (Sp(SP_SCUTOSAURUS), 1)],
    },
    Mission {
        period: PERMIAN,
        title: L("古生代の終わり", "The end of the Paleozoic", "古生代的终结"),
        brief: L(
            "最後の課題です。2億5200万年前、シベリアで史上最大級の噴火が起こります。上空から記録し、大絶滅を生きのびたリストロサウルスを探して、古生代の図鑑を完成させましょう。",
            "Your final assignment. 252 million years ago, one of the greatest eruptions in history begins in Siberia. Record it from the sky, then find Lystrosaurus, which survived the great extinction, and complete your Paleozoic field guide.",
            "最后的课题。2亿5200万年前，西伯利亚发生史上规模最大的喷发之一。从高空记录下来，再找到在大灭绝中幸存的水龙兽，完成古生代图鉴吧。",
        ),
        done: L(
            "古生代の宿題、すべて合格です！ 3億年近い生命の歴史を、あなたは自分の目で見てきました。次の中生代への旅も、楽しみにしていてください。",
            "Every Paleozoic assignment passed! You have seen nearly 300 million years of life with your own eyes. Look forward to your next journey, into the Mesozoic.",
            "古生代作业全部合格！你亲眼见证了近3亿年的生命历史。敬请期待接下来的中生代之旅。",
        ),
        targets: &[(Ph(P_SIBERIAN_TRAPS), 1), (Sp(SP_LYSTROSAURUS), 1)],
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
        assert_eq!(SPECIES[SP_DUNKLEOSTEUS].slug, "dunkleosteus");
        assert_eq!(SPECIES[SP_LYSTROSAURUS].slug, "lystrosaurus");
        assert!(SPECIES.len() > SP_LYSTROSAURUS);
    }

    /// 宿題は紀の順に並び、対象はその紀に撮れる（時代の合うものがいる）
    #[test]
    fn missions_follow_periods() {
        for w in MISSIONS.windows(2) {
            assert!(w[0].period <= w[1].period);
        }
        for m in MISSIONS {
            for &(t, n) in m.targets {
                assert!((1..=3).contains(&n));
                let ok = match t {
                    Target::Sp(i) => SPECIES[i].t0 > PERIODS[m.period].t1 - 1e-3 && SPECIES[i].t1 < PERIODS[m.period].t0 + 1e-3,
                    Target::Ph(i) => PHENOMS[i].t0 > PERIODS[m.period].t1 - 1e-3 && PHENOMS[i].t1 < PERIODS[m.period].t0 + 1e-3,
                };
                assert!(ok, "{} in {}", t.name(), m.title.1);
            }
        }
        for p in 0..PERIODS.len() {
            assert!(MISSIONS.iter().filter(|m| m.period == p).count() >= 4);
        }
    }
}
