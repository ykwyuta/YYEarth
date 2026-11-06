# 画像の注文書（古生代）

このファイルは `cargo test --lib write_asset_requests -- --ignored` でゲームのデータから作り直せます（手で書き換えない）。ツール用の同じ内容が `assets/requests.json` にあります。

## 置き方

- **図鑑の画像**：`assets/artwork/<slug>.jpg`（横3:2、960×640程度のJPEG。背景つき）。図鑑の一覧・詳細・撮影カードに使う。
- **観察用スプライト**：`assets/units/<slug>.png`（背景透過PNG、長い辺768px以内）。観察の場面で動かす。頭は右向き、植物は根元を下の端に。
- **遠景（任意）**：`assets/artwork/backdrop-*.jpg`（16:9、1536×1024程度）。観察の場面の奥に薄く敷く。無ければ共通の `shallow-sea` / `deep-sea` を使い、植物が広がった後の陸は手描きの空と山だけになる。
- 置いてビルドし直す（`bash build.sh` など）だけで使われます。表の登録は不要（`build.rs` がフォルダを見て自動で組み込む）。無い画像は手描きの絵で代わりに描きます。
- 生成元のPNGからの縮小・透過の確認は `tools/prepare_artwork.py` / `tools/prepare_units.py` で（`assets/requests.json` を読む）。

## 進み具合

- 図鑑の画像：112/112（生きもの99・現象13）
- 観察用スプライト：99/99
- 遠景（任意）：16/16

## カンブリア紀（Cambrian）

| slug | 名前 | 学名 | 種類 | 観察での姿 | 図鑑 jpg | スプライト png |
|---|---|---|---|---|---|---|
| `stromatolite` | ストロマトライト | *Stromatolite* | シアノバクテリアがつくる岩 | 海底に固着 | ✅ | ✅ |
| `microbial-mat` | 微生物マット | *Microbial mat* | 微生物の群れ | 海底に固着 | ✅ | ✅ |
| `treptichnus` | トレプティクヌス（生痕） | *Treptichnus pedum* | 生痕化石（巣穴の跡） | 海底に固着 | ✅ | ✅ |
| `anabarites` | アナバリテス | *Anabarites trisulcatus* | 微小有殻化石 | 海底に固着 | ✅ | ✅ |
| `archaeocyath` | 古杯類 | *Archaeocyatha* | 海綿動物の仲間 | 海底に固着 | ✅ | ✅ |
| `lingulella` | リンギュレラ | *Lingulella* | 腕足動物 | 海底に固着 | ✅ | ✅ |
| `redlichia` | レドリキア | *Redlichia* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `halkieria` | ハルキエリア | *Halkieria evangelista* | 軟体動物に近い仲間 | 歩く（横から） | ✅ | ✅ |
| `haikouichthys` | ハイコウイクチス | *Haikouichthys ercaicunensis* | 脊椎動物（最古の魚の仲間） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `fuxianhuia` | フーシャンフイア | *Fuxianhuia protensa* | 節足動物 | 歩く（横から） | ✅ | ✅ |
| `isoxys` | イソキシス | *Isoxys* | 節足動物 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `leanchoilia` | レアンコイリア | *Leanchoilia* | 節足動物（大付属肢類） | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `hallucigenia` | ハルキゲニア | *Hallucigenia* | 葉足動物 | 歩く（横から） | ✅ | ✅ |
| `amplectobelua` | アンプレクトベルア | *Amplectobelua symbrachiata* | 放射歯類 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `kerygmachela` | ケリグマケラ | *Kerygmachela kierkegaardi* | 葉足動物（放射歯類に近い） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `tamisiocaris` | タミシオカリス | *Tamisiocaris borealis* | 放射歯類 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `anomalocaris` | アノマロカリス | *Anomalocaris canadensis* | 放射歯類 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `opabinia` | オパビニア | *Opabinia regalis* | 放射歯類に近い仲間 | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `wiwaxia` | ウィワクシア | *Wiwaxia corrugata* | 軟体動物に近い仲間 | 歩く（横から） | ✅ | ✅ |
| `marrella` | マーレラ | *Marrella splendens* | 節足動物 | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `pikaia` | ピカイア | *Pikaia gracilens* | 脊索動物 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `ottoia` | オットイア | *Ottoia prolifica* | 鰓曳動物 | 巣穴から出る | ✅ | ✅ |
| `aysheaia` | アイシェアイア | *Aysheaia pedunculata* | 葉足動物 | 歩く（横から） | ✅ | ✅ |
| `olenoides` | オレノイデス | *Olenoides serratus* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `vauxia` | バウキシア | *Vauxia gracilenta* | 海綿動物 | 海底に固着 | ✅ | ✅ |
| `sidneyia` | シドネイア | *Sidneyia inexpectans* | 節足動物 | 歩く（横から） | ✅ | ✅ |
| `nectocaris` | ネクトカリス | *Nectocaris pteryx* | 所属不明（頭足類の祖先説も） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `burgessomedusa` | ブルゲッソメデューサ | *Burgessomedusa phasmiformis* | 刺胞動物（クラゲ） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `cambroraster` | カンブロラスター | *Cambroraster falcatus* | 放射歯類 | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `paradoxides` | パラドキシデス | *Paradoxides* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `olenus` | オレヌス | *Olenus* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `agnostus` | アグノストゥス | *Agnostus pisiformis* | 三葉虫の仲間 | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `climactichnites` | クリマクティクニテス | *Climactichnites (maker)* | ナメクジのような動物（陸に上がった跡） | 歩く（横から） | ✅ | ✅ |
| `conodont` | コノドント動物 | *Proconodontus* | 脊椎動物 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `radiolarian` | 放散虫 | *Radiolaria* | 単細胞の生きもの（プランクトン） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `glass-sponge` | ガラス海綿 | *Protospongia* | 海綿動物（六放海綿） | 海底に固着 | ✅ | ✅ |
| `gondwana` | ゴンドワナ大陸 | — | 現象 | 上空から見た大陸 | ✅ | 不要 |
| `laurentia` | ローレンシア大陸 | — | 現象 | 上空から見た大陸 | ✅ | 不要 |
| `eruption` | カルカリンジの大噴火 | — | 現象 | 巨大噴火 | ✅ | 不要 |
| `barren-land` | 植物のない大地 | — | 現象 | 陸の景色 | ✅ | 不要 |

## オルドビス紀（Ordovician）

| slug | 名前 | 学名 | 種類 | 観察での姿 | 図鑑 jpg | スプライト png |
|---|---|---|---|---|---|---|
| `aegirocassis` | エーギロカシス | *Aegirocassis benmoulai* | 放射歯類（こしとり食） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `cameroceras` | カメロケラス | *Cameroceras* | 頭足類（オウムガイの仲間） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `asaphus` | アサフス・コワレフスキー | *Asaphus kowalewskii* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `isotelus` | イソテルス・レックス | *Isotelus rex* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `sacabambaspis` | サカバンバスピス | *Sacabambaspis janvieri* | 無顎類（甲冑魚） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `graptolite` | 筆石（グラプトライト） | *Graptolithina* | 半索動物（群体） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `streptelasma` | ストレプテラスマ | *Streptelasma* | 四放サンゴ（角サンゴ） | 海底に固着 | ✅ | ✅ |
| `crinoid` | ウミユリ | *Crinoidea* | 棘皮動物 | 海底に固着 | ✅ | ✅ |
| `constellaria` | コンステラリア（コケムシ） | *Constellaria* | コケムシ動物（群体） | 海底に固着 | ✅ | ✅ |
| `rafinesquina` | ラフィネスキナ | *Rafinesquina* | 腕足動物 | 海底に固着 | ✅ | ✅ |
| `megalograptus` | メガログラプトゥス | *Megalograptus ohioensis* | ウミサソリ | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `early-land-plants` | 最初の陸の植物（苔類のなかま） | *Early embryophytes* | コケ植物 | 立っている植物 | ✅ | ✅ |
| `stromatoporoid` | 層孔虫（ストロマトポロイド） | *Stromatoporoidea* | 海綿動物 | 海底に固着 | ✅ | ✅ |
| `hirnantian-ice` | ヒルナンシアンの氷床 | — | 現象 | 陸の景色 | ✅ | 不要 |

遠景（任意）：`backdrop-ordovician-land` ✅、`backdrop-ice-land` ✅、`backdrop-ordovician-sea` ✅、`backdrop-ordovician-deep` ✅

## シルル紀（Silurian）

| slug | 名前 | 学名 | 種類 | 観察での姿 | 図鑑 jpg | スプライト png |
|---|---|---|---|---|---|---|
| `cooksonia` | クックソニア | *Cooksonia* | 維管束植物 | 立っている植物 | ✅ | ✅ |
| `baragwanathia` | バラグワナチア | *Baragwanathia longifolia* | ヒカゲノカズラ類 | 立っている植物 | ✅ | ✅ |
| `pneumodesmus` | プネウモデスムス | *Pneumodesmus newmani* | 多足類（ヤスデ） | 歩く（横から） | ✅ | ✅ |
| `favosites` | ファボシテス（ハチノスサンゴ） | *Favosites* | 床板サンゴ | 海底に固着 | ✅ | ✅ |
| `calymene` | カリメネ | *Calymene blumenbachii* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `guiyu` | グイユー | *Guiyu oneiros* | 肉鰭類（硬骨魚） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `entelognathus` | エンテログナトゥス | *Entelognathus primordialis* | 板皮類 | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `birkenia` | ビルケニア | *Birkenia elegans* | 無顎類（欠甲類） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `eurypterus` | ユーリプテルス | *Eurypterus remipes* | ウミサソリ | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `green-shore` | 最初の緑の岸辺 | — | 現象 | 陸の景色 | ✅ | 不要 |

遠景（任意）：`backdrop-silurian-land` ✅、`backdrop-silurian-sea` ✅、`backdrop-silurian-deep` ✅

## デボン紀（Devonian）

| slug | 名前 | 学名 | 種類 | 観察での姿 | 図鑑 jpg | スプライト png |
|---|---|---|---|---|---|---|
| `prototaxites` | プロトタキシーテス | *Prototaxites* | 菌類（キノコの仲間） | 立っている植物 | ✅ | ✅ |
| `rhynia` | ライニア | *Rhynia gwynne-vaughanii* | 維管束植物 | 立っている植物 | ✅ | ✅ |
| `palaeocharinus` | パレオカリヌス | *Palaeocharinus* | クモ形類（ワレイタムシ類） | 歩く（横から） | ✅ | ✅ |
| `schinderhannes` | シンダーハンネス | *Schinderhannes bartelsi* | 放射歯類 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `walliserops` | ワリセロプス | *Walliserops trifurcatus* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `jaekelopterus` | イェーケロプテルス | *Jaekelopterus rhenaniae* | ウミサソリ | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `eospermatopteris` | エオスペルマトプテリス | *Eospermatopteris (Wattieza)* | クラドキシロン類（初期の木） | 立っている植物 | ✅ | ✅ |
| `archaeopteris` | アーケオプテリス | *Archaeopteris* | 前裸子植物（木） | 立っている植物 | ✅ | ✅ |
| `materpiscis` | マテルピスキス | *Materpiscis attenboroughi* | 板皮類 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `manticoceras` | マンティコセラス | *Manticoceras* | 頭足類（アンモノイド） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `bothriolepis` | ボスリオレピス | *Bothriolepis canadensis* | 板皮類（胴甲類） | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `eusthenopteron` | ユーステノプテロン | *Eusthenopteron foordi* | 肉鰭類 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `tiktaalik` | ティクターリク | *Tiktaalik roseae* | 肉鰭類（魚と四肢動物の間） | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `acanthostega` | アカントステガ | *Acanthostega gunnari* | 四肢動物 | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `ichthyostega` | イクチオステガ | *Ichthyostega* | 四肢動物 | 歩く（横から） | ✅ | ✅ |
| `dunkleosteus` | ダンクルオステウス | *Dunkleosteus terrelli* | 板皮類（節頸類） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `cladoselache` | クラドセラケ | *Cladoselache* | 軟骨魚類（サメ） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `euramerica` | ユーラメリカ大陸 | — | 現象 | 上空から見た大陸 | ✅ | 不要 |
| `first-forest` | 最初の森 | — | 現象 | 陸の景色 | ✅ | 不要 |

遠景（任意）：`backdrop-devonian-land` ✅、`backdrop-devonian-sea` ✅、`backdrop-devonian-deep` ✅

## 石炭紀（Carboniferous）

| slug | 名前 | 学名 | 種類 | 観察での姿 | 図鑑 jpg | スプライト png |
|---|---|---|---|---|---|---|
| `stethacanthus` | ステタカントゥス | *Stethacanthus* | 軟骨魚類（サメの仲間） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `gigantoproductus` | ギガントプロダクトゥス | *Gigantoproductus* | 腕足動物 | 海底に固着 | ✅ | ✅ |
| `lepidodendron` | レピドデンドロン（鱗木） | *Lepidodendron* | ヒカゲノカズラ類（木） | 立っている植物 | ✅ | ✅ |
| `calamites` | カラミテス（ロボク） | *Calamites* | トクサ類 | 立っている植物 | ✅ | ✅ |
| `pulmonoscorpius` | プルモノスコルピウス | *Pulmonoscorpius kirktonensis* | サソリ | 歩く（横から） | ✅ | ✅ |
| `arthropleura` | アースロプレウラ | *Arthropleura* | 多足類（ヤスデの仲間） | 歩く（横から） | ✅ | ✅ |
| `hylonomus` | ヒロノムス | *Hylonomus lyelli* | 爬虫類（有羊膜類） | 歩く（横から） | ✅ | ✅ |
| `dendrerpeton` | デンドレルペトン | *Dendrerpeton* | 両生類（分椎類） | 歩く（横から） | ✅ | ✅ |
| `tullimonstrum` | ツリモンストラム（トゥリーモンスター） | *Tullimonstrum gregarium* | 正体不明の動物 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `meganeura` | メガネウラ | *Meganeura monyi* | 昆虫（オオトンボ類） | 飛ぶ（はねを広げて） | ✅ | ✅ |
| `archimylacris` | アーキミラクリス | *Archimylacris* | 昆虫（ゴキブリの祖先の仲間） | 歩く（横から） | ✅ | ✅ |
| `psaronius` | プサロニウス（木生シダ） | *Psaronius* | シダ植物（木生シダ） | 立っている植物 | ✅ | ✅ |
| `coal-forest` | 石炭紀の湿地林 | — | 現象 | 陸の景色 | ✅ | 不要 |
| `gondwana-ice` | ゴンドワナの大氷河 | — | 現象 | 陸の景色 | ✅ | 不要 |

遠景（任意）：`backdrop-carboniferous-land` ✅、`backdrop-carboniferous-sea` ✅、`backdrop-carboniferous-deep` ✅

## ペルム紀（Permian）

| slug | 名前 | 学名 | 種類 | 観察での姿 | 図鑑 jpg | スプライト png |
|---|---|---|---|---|---|---|
| `mesosaurus` | メソサウルス | *Mesosaurus tenuidens* | 爬虫類 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `glossopteris` | グロッソプテリス | *Glossopteris* | 種子シダ類（木） | 立っている植物 | ✅ | ✅ |
| `walchia` | ワルキア | *Walchia* | 針葉樹 | 立っている植物 | ✅ | ✅ |
| `eryops` | エリオプス | *Eryops megacephalus* | 両生類（分椎類） | 歩く（横から） | ✅ | ✅ |
| `diplocaulus` | ディプロカウルス | *Diplocaulus magnicornis* | 両生類（空椎類） | 歩く（横から） | ✅ | ✅ |
| `dimetrodon` | ディメトロドン | *Dimetrodon grandis* | 単弓類（盤竜類） | 歩く（横から） | ✅ | ✅ |
| `helicoprion` | ヘリコプリオン | *Helicoprion* | 軟骨魚類（ギンザメの仲間） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `parafusulina` | パラフズリナ（紡錘虫） | *Parafusulina* | 有孔虫（紡錘虫） | 海底に固着 | ✅ | ✅ |
| `ditomopyge` | ディトモピゲ | *Ditomopyge* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `inostrancevia` | イノストランケビア | *Inostrancevia alexandri* | 単弓類（ゴルゴノプス類） | 歩く（横から） | ✅ | ✅ |
| `scutosaurus` | スクトサウルス | *Scutosaurus karpinskii* | 爬虫類（パレイアサウルス類） | 歩く（横から） | ✅ | ✅ |
| `lystrosaurus` | リストロサウルス | *Lystrosaurus* | 単弓類（ディキノドン類） | 歩く（横から） | ✅ | ✅ |
| `pangaea` | 超大陸パンゲア | — | 現象 | 上空から見た大陸 | ✅ | 不要 |
| `permian-desert` | パンゲアの砂漠 | — | 現象 | 陸の景色 | ✅ | 不要 |
| `siberian-traps` | シベリアの大噴火 | — | 現象 | 巨大噴火 | ✅ | 不要 |

遠景（任意）：`backdrop-permian-land` ✅、`backdrop-permian-sea` ✅、`backdrop-permian-deep` ✅

## 生成プロンプト（例）

これまでの画像（`assets/artwork/manifest.json`・`assets/units/manifest.json`）と同じ書き方にそろえています。足りない項目だけ載せます。
