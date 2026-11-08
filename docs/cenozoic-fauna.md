# 新生代（6600万年前〜1万年前）

第10巻を古第三紀、第11巻を新第三紀、第12巻を第四紀として追加。生きもの132項目・景観3項目・宿題12件を収録しています。

| 巻 | 年代（百万年前） | 主な生きもの |
|---|---|---|
| 古第三紀 | 66〜23.03 | プルガトリウス、パントラムダ、ガストルニス、ヒラコテリウム、バシロサウルス、アルシノイテリウム、パラケラテリウム |
| 新第三紀 | 23.03〜2.58 | デイノテリウム、ヒッパリオン、メガロドン、リヴィアタン、シヴァテリウム、アウストラロピテクス |
| 第四紀 | 2.58〜0.01 | ケナガマンモス、スミロドン、メガテリウム、ケブカサイ、メガロケロス、ホモ・サピエンス、完新世のオーク |

初期の36項目に、古第三紀・新第三紀・第四紀へ24項目ずつ、計72項目を追加しました。追加分は哺乳類44、鳥9、魚4、爬虫類2、節足動物3、二枚貝2、有孔虫1、ウニ1、植物6です。北アメリカへの集中を減らし、南アメリカの固有有蹄類、アフリカの初期長鼻類、オーストラリアの有袋類・単孔類、南極のペンギンなどを加えています。地域と世の集計および残る不足は [収録バランス](cenozoic-balance.md) に記録しています。

古第三紀の南方の空白を補うため、さらに地域指定の動物19項目・植物5項目を追加しました。詳しい年代・産地・化石の根拠は [豪州・南極の補強](paleogene-southern-coverage.md) にあります。

各巻には小型哺乳類や昆虫などの広域観察項目があり、新生代全体に鳥・トカゲ・魚・二枚貝・巻貝・カニを配置しています。広域項目は地域と年代によって入れ替わる近縁種をまとめた代表像で、同一種の世界分布を意味しません。

観察地点はメッセル、ファイユーム、シワリク、ハダール、ラ・ブレア、シベリアのマンモス草原に加え、マルゴン、エタダナ、セイモア島、クロス・ヴァレーを収録しています。産地の位置は既存の簡略化した大陸の上で、適切な環境に置き直します。

## 年代と古地理の扱い

- 1万年前は更新世ではなく完新世です。更新世後期の終わりを0.0117百万年前、旅行範囲の終わりを0.01百万年前としています。新生代の区分は既存ゲームの「期」の帯に、世または更新世の前・中・後期を単位として表示します。
- 年代表記、時間旅行、訪問時刻、レーダーのキャッシュは短い第四紀の区間を扱えます。上段は各紀を同じ幅で表示し、下段は選択中の紀を年代に比例して表示します。最後のクリック領域は正確に1万年前へ移動します。矢印で完新世の中間にも移動できます。
- 種の年代はゲームで観察できる範囲です。マンモス・メガロケロス・ホモ・サピエンスなどの終端0.01は絶滅年代を意味しません。
- 新生代の移動は6600万年前の既存地形から連続する簡略モデルです。北大西洋の拡大、インドの北上、オーストラリアの北東移動を示します。大陸の輪郭は既存の多角形を引き継ぎ、現代の海岸線・山脈・海面変動・陸橋を正確には再現しません。
- 南極の氷は約3400万年前以降、北半球の氷は更新世に高緯度で表示します。3400万〜1400万年前の南極では、植物化石に対応する沿岸避難地を設け、地域植物は氷のない陸に限定しています。氷期と間氷期の各周期は再現していません。完新世には北半球の氷の表示を外します。
- ホモ・サピエンスは30万年前から表示し、7万年前より古い時代ではアフリカに限ります。その後の分布拡大はゲーム用に簡略化しています。

## 描画と保存

新生代の生きもの132項目に図鑑JPEGと透過PNGを各1点、現象3項目に図鑑JPEG、3紀に陸・浅海・深海の専用遠景を各1点、計276素材を組み込みました。内蔵ImageGenで個別に制作し、生成プロンプトと原画像の参照を各素材フォルダの `manifest.json` に記録しています。Rustの手続き描画は画像がない場合の代替として維持しています。

72項目はID 331〜402、南方の補強24項目はID 403〜426へ追加しています。108項目版の図鑑・課題の記録も引き継ぎます。36項目版の現象22項目・宿題63件の記録も引き継ぎます。生きもの・現象・課題は既存の配列の末尾へ追加しています。保存形式v3の生きもの数に基づいて現象の記録位置を移し、過去の収録状況と51件の課題の合格を維持します。古生代・中生代だけの版から読み込む場合、新生代の12課題は未達成として追加します。36項目版で合格した新生代の課題は、そのまま維持します。

年代・地域は、現生種・属を含む復元上の観察窓です。古い端が正確な初出年代を表すとは限りません。ティンガマラの姿、属全体に含まれる複数種、現代的な動物の古い年代での分布には推定があり、図鑑とバランス表にも制約を記載しています。新生代の全地域・全世を均等な数で埋めてはいません。

## 参考資料

- [国際層序委員会の年代区分表](https://stratigraphy.org/chart/)：古第三紀・新第三紀・第四紀、更新世と完新世の境界。
- [Smithsonian：Homo sapiens](https://humanorigins.si.edu/evidence/human-fossils/species/homo-sapiens)：約30万年前のアフリカでの登場。
- [Smithsonian Ocean：Megalodon](https://ocean.si.edu/ocean-life/sharks-rays/megalodon)：新第三紀の大型サメと大きさ。
- [AMNH：Megalodon](https://www.amnh.org/explore/ology/ology-cards/379-megalodon)：約360万年前の絶滅。
- [Natural History Museum：The last of the mammoths](https://www.nhm.ac.uk/discover/last-mammoths.html)：氷期の分布と完新世まで残った集団。

- [Queensland Museum：Macropus](https://www.museum.qld.gov.au/learn-and-discover/queenslands-ancient-past/meet-the-megafauna/macropus)・[Australian Museum：Eastern Grey Kangaroo](https://australian.museum/learn/animals/mammals/eastern-grey-kangaroo/)：更新世から生き残ったカンガルーの仲間。

追加72項目の選定には、以下の地域別・生物群別の資料も参照しています。

- [NPS：Evolution on Land](https://www.nps.gov/subjects/fossils/evolution-on-land.htm)：北アメリカの新生代化石群と環境。
- [AMNH：Pakicetus](https://www.amnh.org/explore/news-blogs/first-whale-pakicetus)：歩いていた初期クジラと骨の特徴。
- [Australian Museum：Tingamarra](https://australian.museum/learn/australia-over-time/extinct-animals/tingamarra/)・[Murgon](https://australian.museum/learn/australia-over-time/fossils/sites/murgon/)：古第三紀のオーストラリアの化石と記録の少なさ。
- [SA Museum：Megafauna gallery](https://www.samuseum.sa.gov.au/megafauna-gallery)：オーストラリアの大型有袋類と飛べない鳥。
- [Australian Museum：Diprotodon](https://australian.museum/learn/australia-over-time/extinct-animals/diprotodon-optatum/)：大型草食性有袋類とティラコレオ。
- [AMNH：Plio-Pleistocene](https://research.amnh.org/paleontology/perissodactyl/concepts/deep-time/plio-pleistocene)：更新世の大型動物の代表例。
- [Florida Museum：Teleoceras](https://www.floridamuseum.ufl.edu/florida-vertebrate-fossils/species/teleoceras-proterum/)：短脚のサイと復元上の注意。
- [Smithsonian：Homo erectus](https://humanorigins.si.edu/evidence/human-fossils/species/homo-erectus)：年代・体格・アフリカとアジアの分布。
- [Natural History Museum：Elephant ancestors](https://www.nhm.ac.uk/discover/news/2021/july/the-rise-and-fall-of-elephant-ancestors.html)：アフリカの小型長鼻類から多様なゾウの仲間へ。
- [AMNH：Trigonostylopsの頭骨と系統](https://zenodo.org/records/5415024)：南アメリカ独自の始新世の有蹄類。
- [Andean Geology：Pygoscelisの化石](https://andeangeology.cl/index.php/revista1/article/view/V33n2-a06)：チリの新第三紀のペンギン。

## 検証

`cargo test --release --lib --features headless` で、Windowsでも新生代を含む対象への移動・撮影・保存、12巻のタブ、完新世の時刻選択、描画範囲を検証できます。全時代の画像の完全性・埋め込み画像のデコード・透過PNGを検査します。新生代の代替描画と実画像での訪問・撮影・保存も確認します。
