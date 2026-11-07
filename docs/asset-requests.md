# 画像の注文書（古生代・中生代）

このファイルは `cargo test --lib write_asset_requests -- --ignored` でゲームのデータから作り直せます（手で書き換えない）。ツール用の同じ内容が `assets/requests.json` にあります。

## 置き方

- **図鑑の画像**：`assets/artwork/<slug>.jpg`（横3:2、960×640程度のJPEG。背景つき）。図鑑の一覧・詳細・撮影カードに使う。
- **観察用スプライト**：`assets/units/<slug>.png`（背景透過PNG、長い辺768px以内）。観察の場面で動かす。頭は右向き、植物は根元を下の端に。
- **遠景（任意）**：`assets/artwork/backdrop-*.jpg`（16:9、1536×1024程度）。観察の場面の奥に薄く敷く。無ければ共通の `shallow-sea` / `deep-sea` を使い、植物が広がった後の陸は手描きの空と山だけになる。
- 置いてビルドし直す（`bash build.sh` など）だけで使われます。表の登録は不要（`build.rs` がフォルダを見て自動で組み込む）。無い画像は手描きの絵で代わりに描きます。
- 生成元のPNGからの縮小・透過の確認は `tools/prepare_artwork.py` / `tools/prepare_units.py` で（`assets/requests.json` を読む）。

## 進み具合

- 図鑑の画像：225/302（生きもの283・現象19）
- 観察用スプライト：212/283
- 遠景（任意）：16/25

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
| `canadaspis` | カナダスピス | *Canadaspis perfecta* | 節足動物 | 歩く（横から） | ✅ | ✅ |
| `waptia` | ワプティア | *Waptia fieldensis* | 節足動物 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `yohoia` | ヨホイア | *Yohoia tenuis* | 節足動物（大付属肢類） | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `eldonia` | エルドニア | *Eldonia ludwigi* | 所属不明（円盤形の動物） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `chancelloria` | チャンセロリア | *Chancelloria* | 所属不明（トゲの動物） | 海底に固着 | ✅ | ✅ |
| `elrathia` | エルラシア | *Elrathia kingii* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `gogia` | ゴギア | *Gogia* | 棘皮動物（始海百合類） | 海底に固着 | ✅ | ✅ |
| `myllokunmingia` | ミロクンミンギア | *Myllokunmingia fengjiaoa* | 脊椎動物（最古の魚の仲間） | 泳ぐ・漂う（横から） | ✅ | ✅ |
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
| `maclurites` | マクルリテス | *Maclurites* | 腹足類（巻貝） | 歩く（横から） | ✅ | ✅ |
| `ambonychia` | アンボニキア | *Ambonychia* | 二枚貝 | 海底に固着 | ✅ | ✅ |
| `orthis` | オルティス | *Orthis* | 腕足動物 | 海底に固着 | ✅ | ✅ |
| `triarthrus` | トリアルトルス | *Triarthrus eatoni* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `neseuretus` | ネセウレトゥス | *Neseuretus* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `orthoceras` | オルトセラス | *Orthoceras* | 頭足類（オウムガイの仲間） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `hudsonaster` | ハドソナスター（古いヒトデ） | *Hudsonaster* | 棘皮動物（ヒトデ） | 歩く（横から） | ✅ | ✅ |
| `arandaspis` | アランダスピス | *Arandaspis prionotolepis* | 無顎類（甲冑魚） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `promissum` | プロミッスム（巨大コノドント） | *Promissum pulchrum* | 脊椎動物（コノドント） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `pentecopterus` | ペンテコプテルス | *Pentecopterus decorahensis* | ウミサソリ | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `lunataspis` | ルナタスピス | *Lunataspis aurora* | カブトガニ類 | 歩く（横から） | ✅ | ✅ |
| `echinosphaerites` | エキノスファエリテス（ウミリンゴ） | *Echinosphaerites aurantium* | 棘皮動物（ウミリンゴ類） | 海底に固着 | ✅ | ✅ |
| `bothriocidaris` | ボスリオシダリス（古いウニ） | *Bothriocidaris* | 棘皮動物（ウニ） | 歩く（横から） | ✅ | ✅ |
| `lituites` | リツイテス | *Lituites* | 頭足類（オウムガイの仲間） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `selenopeltis` | セレノペルティス | *Selenopeltis* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `astraspis` | アストラスピス | *Astraspis desiderata* | 無顎類（甲冑魚） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `halysites` | ハリシテス（クサリサンゴ） | *Halysites* | 床板サンゴ | 海底に固着 | ✅ | ✅ |
| `receptaculites` | レセプタクリテス（ヒマワリサンゴ） | *Receptaculites* | 所属不明（緑藻の仲間か） | 海底に固着 | ✅ | ✅ |
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
| `pentamerus` | ペンタメルス | *Pentamerus oblongus* | 腕足動物 | 海底に固着 | ✅ | ✅ |
| `goniophyllum` | ゴニオフィルム | *Goniophyllum pyramidale* | 四放サンゴ（角サンゴ） | 海底に固着 | ✅ | ✅ |
| `dalmanites` | ダルマニテス | *Dalmanites* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `dawsonoceras` | ドーソノセラス | *Dawsonoceras* | 頭足類（オウムガイの仲間） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `platyceras` | プラティセラス | *Platyceras* | 腹足類（巻貝） | 海底に固着 | ✅ | ✅ |
| `climatius` | クリマティウス | *Climatius reticulatus* | 棘魚類（トゲのあるサメの仲間） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `loganellia` | ロガネリア | *Loganellia scotica* | 無顎類（テロドゥス類） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `slimonia` | スリモニア | *Slimonia acuminata* | ウミサソリ | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `zosterophyllum` | ゾステロフィルム | *Zosterophyllum* | 維管束植物 | 立っている植物 | ✅ | ✅ |
| `brontoscorpio` | ブロントスコルピオ | *Brontoscorpio anglicus* | サソリ | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `carcinosoma` | カルキノソーマ | *Carcinosoma* | ウミサソリ | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `jamoytius` | ジャモイティウス | *Jamoytius kerwoodi* | 無顎類 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `heliolites` | ヘリオリテス（日石サンゴ） | *Heliolites* | 床板サンゴ | 海底に固着 | ✅ | ✅ |
| `encrinurus` | エンクリヌルス（イチゴ頭の三葉虫） | *Encrinurus* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `ophioceras` | オフィオセラス | *Ophioceras* | 頭足類（オウムガイの仲間） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `atrypa` | アトリパ | *Atrypa* | 腕足動物 | 海底に固着 | ✅ | ✅ |
| `pterygotus` | プテリゴトゥス | *Pterygotus* | ウミサソリ | 底の近くを泳ぐ（横から） | ✅ | ✅ |
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
| `phacops` | ファコプス | *Phacops* | 三葉虫 | 歩く（横から） | ✅ | ✅ |
| `mucrospirifer` | ムクロスピリファー | *Mucrospirifer* | 腕足動物 | 海底に固着 | ✅ | ✅ |
| `heliophyllum` | ヘリオフィルム | *Heliophyllum* | 四放サンゴ（角サンゴ） | 海底に固着 | ✅ | ✅ |
| `cephalaspis` | ケファラスピス | *Cephalaspis* | 無顎類（骨甲類） | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `pteraspis` | プテラスピス | *Pteraspis* | 無顎類（異甲類） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `cheirolepis` | ケイロレピス | *Cheirolepis* | 条鰭類（硬骨魚） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `coccosteus` | コッコステウス | *Coccosteus* | 板皮類（節頸類） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `dipterus` | ディプテルス | *Dipterus* | 肺魚類 | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `hyneria` | ハイネリア | *Hyneria lindae* | 肉鰭類 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `psilophyton` | プシロフィトン | *Psilophyton* | 維管束植物 | 立っている植物 | ✅ | ✅ |
| `rhacophyton` | ラコフィトン | *Rhacophyton* | シダの仲間 | 立っている植物 | ✅ | ✅ |
| `drepanophycus` | ドレパノフィクス | *Drepanophycus spinaeformis* | ヒカゲノカズラ類 | 立っている植物 | ✅ | ✅ |
| `aglaophyton` | アグラオフィトン | *Aglaophyton majus* | 初期の陸上植物 | 立っている植物 | ✅ | ✅ |
| `rhyniognatha` | リニオグナタ | *Rhyniognatha hirsti* | 昆虫（最古の昆虫か） | 歩く（横から） | ✅ | ✅ |
| `drepanaspis` | ドレパナスピス | *Drepanaspis gemuendenensis* | 無顎類（異甲類） | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `holoptychius` | ホロプティキウス | *Holoptychius* | 肉鰭類 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `titanichthys` | ティタニクティス | *Titanichthys* | 板皮類（節頸類） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `panderichthys` | パンデリクティス | *Panderichthys rhombolepis* | 肉鰭類（魚と四肢動物の間） | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `clymenia` | クリメニア | *Clymenia* | 頭足類（アンモノイド） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `helianthaster` | ヘリアンタスター | *Helianthaster rhenanus* | 棘皮動物（ヒトデの仲間） | 歩く（横から） | ✅ | ✅ |
| `calamophyton` | カラモフィトン | *Calamophyton* | クラドキシロン類（初期の木） | 立っている植物 | ✅ | ✅ |
| `attercopus` | アッテルコプス | *Attercopus fimbriunguis* | クモ形類（クモの祖先の仲間） | 歩く（横から） | ✅ | ✅ |
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
| `spirifer` | スピリファー | *Spirifer* | 腕足動物 | 海底に固着 | ✅ | ✅ |
| `lithostrotion` | リソストロティオン | *Lithostrotion* | 四放サンゴ（群体） | 海底に固着 | ✅ | ✅ |
| `archimedes` | アルキメデス（らせんコケムシ） | *Archimedes* | コケムシ動物（群体） | 海底に固着 | ✅ | ✅ |
| `goniatites` | ゴニアタイト | *Goniatites* | 頭足類（アンモノイド） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `pentremites` | ペントレマイテス（ウミツボミ） | *Pentremites* | 棘皮動物（ウミツボミ） | 海底に固着 | ✅ | ✅ |
| `elonichthys` | エロニクティス | *Elonichthys* | 条鰭類（硬骨魚） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `euproops` | ユープロープス（カブトガニの仲間） | *Euproops* | カブトガニ類 | 歩く（横から） | ✅ | ✅ |
| `bellerophon` | ベレロフォン | *Bellerophon* | 軟体動物（巻貝の仲間） | 歩く（横から） | ✅ | ✅ |
| `sigillaria` | シギラリア（封印木） | *Sigillaria* | ヒカゲノカズラ類（木） | 立っている植物 | ✅ | ✅ |
| `cordaites` | コルダイテス | *Cordaites* | 裸子植物（木） | 立っている植物 | ✅ | ✅ |
| `medullosa` | メデュローサ（種子シダ） | *Medullosa* | 種子シダ類 | 立っている植物 | ✅ | ✅ |
| `stenodictya` | ステノディクティア | *Stenodictya* | 昆虫（ムカシアミバネムシ類） | 飛ぶ（はねを広げて） | ✅ | ✅ |
| `amphibamus` | アンフィバムス | *Amphibamus grandiceps* | 両生類（分椎類） | 歩く（横から） | ✅ | ✅ |
| `falcatus` | ファルカトゥス | *Falcatus falcatus* | 軟骨魚類（サメの仲間） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `edestus` | エデスタス | *Edestus* | 軟骨魚類（ギンザメの仲間） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `rhizodus` | リゾドゥス | *Rhizodus hibberti* | 肉鰭類 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `crassigyrinus` | クラッシギリヌス | *Crassigyrinus scoticus* | 四肢動物 | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `westlothiana` | ウェストロティアナ | *Westlothiana lizziae* | 爬形類 | 歩く（横から） | ✅ | ✅ |
| `balanerpeton` | バラネルペトン | *Balanerpeton woodi* | 両生類（分椎類） | 歩く（横から） | ✅ | ✅ |
| `megarachne` | メガラクネ | *Megarachne servinei* | ウミサソリ | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `petrolacosaurus` | ペトロラコサウルス | *Petrolacosaurus kansensis* | 爬虫類（双弓類） | 歩く（横から） | ✅ | ✅ |
| `archaeothyris` | アーケオシリス | *Archaeothyris florensis* | 単弓類 | 歩く（横から） | ✅ | ✅ |
| `essexella` | エセクセラ | *Essexella asherae* | 刺胞動物（クラゲ） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `lyginopteris` | リギノプテリス（種子シダ） | *Lyginopteris* | 種子シダ類 | 立っている植物 | ✅ | ✅ |
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
| `leptodus` | レプトドゥス | *Leptodus* | 腕足動物 | 海底に固着 | ✅ | ✅ |
| `waagenoceras` | ワーゲノセラス | *Waagenoceras* | 頭足類（アンモノイド） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `waagenophyllum` | ワーゲノフィルム | *Waagenophyllum* | 四放サンゴ（群体） | 海底に固着 | ✅ | ✅ |
| `eurydesma` | エウリデスマ | *Eurydesma* | 二枚貝 | 海底に固着 | ✅ | ✅ |
| `xenacanthus` | クセナカントゥス | *Xenacanthus* | 軟骨魚類（淡水のサメ） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `palaeoniscum` | パレオニスクム | *Palaeoniscum freieslebeni* | 条鰭類（硬骨魚） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `edaphosaurus` | エダフォサウルス | *Edaphosaurus* | 単弓類（盤竜類） | 歩く（横から） | ✅ | ✅ |
| `seymouria` | セイモウリア | *Seymouria baylorensis* | 爬形類（両生類と爬虫類の間） | 歩く（横から） | ✅ | ✅ |
| `captorhinus` | カプトリヌス | *Captorhinus aguti* | 爬虫類 | 歩く（横から） | ✅ | ✅ |
| `dicynodon` | ディキノドン | *Dicynodon* | 単弓類（ディキノドン類） | 歩く（横から） | ✅ | ✅ |
| `moschops` | モスコプス | *Moschops capensis* | 単弓類（恐頭類） | 歩く（横から） | ✅ | ✅ |
| `tshekardocoleus` | チェカルドコレウス（古い甲虫） | *Tshekardocoleus* | 昆虫（甲虫の祖先） | 歩く（横から） | ✅ | ✅ |
| `gigantopteris` | ギガントプテリス | *Gigantopteris* | 種子植物（大羽羊歯類） | 立っている植物 | ✅ | ✅ |
| `ophiacodon` | オフィアコドン | *Ophiacodon* | 単弓類（盤竜類） | 歩く（横から） | ✅ | ✅ |
| `cacops` | カコプス | *Cacops aspidephorus* | 両生類（分椎類） | 歩く（横から） | ✅ | ✅ |
| `varanops` | ヴァラノプス | *Varanops brevirostris* | 単弓類（盤竜類） | 歩く（横から） | ✅ | ✅ |
| `estemmenosuchus` | エステメノスクス | *Estemmenosuchus uralensis* | 単弓類（恐頭類） | 歩く（横から） | ✅ | ✅ |
| `titanophoneus` | ティタノフォネウス | *Titanophoneus potens* | 単弓類（恐頭類） | 歩く（横から） | ✅ | ✅ |
| `coelurosauravus` | コエルロサウラヴス | *Coelurosauravus* | 爬虫類（滑空する爬虫類） | 歩く（横から） | ✅ | ✅ |
| `prionosuchus` | プリオノスクス | *Prionosuchus plummeri* | 両生類（分椎類） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `dorypterus` | ドリプテルス | *Dorypterus hoffmanni* | 条鰭類（硬骨魚） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `meganeuropsis` | メガネウロプシス | *Meganeuropsis permiana* | 昆虫（オオトンボ類） | 飛ぶ（はねを広げて） | ✅ | ✅ |
| `euchambersia` | エウチャンベルシア | *Euchambersia mirabilis* | 単弓類（獣頭類） | 歩く（横から） | ✅ | ✅ |
| `voltzia` | ボルチア | *Voltzia* | 針葉樹 | 立っている植物 | ✅ | ✅ |
| `pangaea` | 超大陸パンゲア | — | 現象 | 上空から見た大陸 | ✅ | 不要 |
| `permian-desert` | パンゲアの砂漠 | — | 現象 | 陸の景色 | ✅ | 不要 |
| `siberian-traps` | シベリアの大噴火 | — | 現象 | 巨大噴火 | ✅ | 不要 |

遠景（任意）：`backdrop-permian-land` ✅、`backdrop-permian-sea` ✅、`backdrop-permian-deep` ✅

## 三畳紀（Triassic）

| slug | 名前 | 学名 | 種類 | 観察での姿 | 図鑑 jpg | スプライト png |
|---|---|---|---|---|---|---|
| `lystrosaurus` | リストロサウルス | *Lystrosaurus* | 単弓類（ディキノドン類） | 歩く（横から） | ✅ | ✅ |
| `thrinaxodon` | トリナクソドン | *Thrinaxodon liorhinus* | 単弓類（キノドン類） | 歩く（横から） | ⬜ | ⬜ |
| `pleuromeia` | プレウロメイア | *Pleuromeia* | ヒカゲノカズラ類 | 立っている植物 | ⬜ | ⬜ |
| `triadobatrachus` | トリアドバトラクス | *Triadobatrachus massinoti* | 両生類（カエルの祖先） | 歩く（横から） | ⬜ | ⬜ |
| `cynognathus` | キノグナトゥス | *Cynognathus crateronotus* | 単弓類（キノドン類） | 歩く（横から） | ⬜ | ⬜ |
| `mixosaurus` | ミクソサウルス | *Mixosaurus* | 魚竜 | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `nothosaurus` | ノトサウルス | *Nothosaurus* | 偽竜類（海の爬虫類） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `tanystropheus` | タニストロフェウス | *Tanystropheus* | 主竜形類（首の長い爬虫類） | 底の近くを泳ぐ（横から） | ⬜ | ⬜ |
| `placodus` | プラコドゥス | *Placodus gigas* | 板歯類（海の爬虫類） | 底の近くを泳ぐ（横から） | ⬜ | ⬜ |
| `ceratites` | セラタイト | *Ceratites nodosus* | 頭足類（アンモナイト） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `eoraptor` | エオラプトル | *Eoraptor lunensis* | 恐竜（初期の竜盤類） | 歩く（横から） | ⬜ | ⬜ |
| `herrerasaurus` | ヘレラサウルス | *Herrerasaurus ischigualastensis* | 恐竜（初期の肉食恐竜） | 歩く（横から） | ⬜ | ⬜ |
| `coelophysis` | コエロフィシス | *Coelophysis bauri* | 恐竜（肉食恐竜） | 歩く（横から） | ⬜ | ⬜ |
| `plateosaurus` | プラテオサウルス | *Plateosaurus trossingensis* | 恐竜（竜脚形類） | 歩く（横から） | ⬜ | ⬜ |
| `postosuchus` | ポストスクス | *Postosuchus kirkpatricki* | 偽鰐類（ワニの仲間） | 歩く（横から） | ⬜ | ⬜ |
| `desmatosuchus` | デスマトスクス | *Desmatosuchus spurensis* | 偽鰐類（アエトサウルス類） | 歩く（横から） | ⬜ | ⬜ |
| `eudimorphodon` | エウディモルフォドン | *Eudimorphodon ranzii* | 翼竜 | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `shonisaurus` | ショニサウルス | *Shonisaurus popularis* | 魚竜 | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `metoposaurus` | メトポサウルス | *Metoposaurus* | 両生類（分椎類） | 歩く（横から） | ⬜ | ⬜ |
| `dicroidium` | ディクロイディウム | *Dicroidium* | 種子シダ類（木） | 立っている植物 | ⬜ | ⬜ |
| `araucarioxylon` | アラウカリオキシロン（化石の森の木） | *Araucarioxylon arizonicum* | 針葉樹 | 立っている植物 | ⬜ | ⬜ |
| `camp-eruption` | 中央大西洋の大噴火（CAMP） | — | 現象 | 巨大噴火 | ⬜ | 不要 |
| `gymnosperm-forest` | 恐竜時代の針葉樹とシダの森 | — | 現象 | 陸の景色 | ⬜ | 不要 |

遠景（任意）：`backdrop-triassic-land` ⬜、`backdrop-triassic-sea` ⬜、`backdrop-triassic-deep` ⬜

## ジュラ紀（Jurassic）

| slug | 名前 | 学名 | 種類 | 観察での姿 | 図鑑 jpg | スプライト png |
|---|---|---|---|---|---|---|
| `megazostrodon` | メガゾストロドン | *Megazostrodon rudnerae* | 哺乳形類（初期の哺乳類） | 歩く（横から） | ⬜ | ⬜ |
| `dilophosaurus` | ディロフォサウルス | *Dilophosaurus wetherilli* | 恐竜（肉食恐竜） | 歩く（横から） | ⬜ | ⬜ |
| `ichthyosaurus` | イクチオサウルス | *Ichthyosaurus communis* | 魚竜 | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `plesiosaurus` | プレシオサウルス | *Plesiosaurus dolichodeirus* | 首長竜 | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `dimorphodon` | ディモルフォドン | *Dimorphodon macronyx* | 翼竜 | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `scelidosaurus` | スケリドサウルス | *Scelidosaurus harrisonii* | 恐竜（初期の装盾類） | 歩く（横から） | ⬜ | ⬜ |
| `stenopterygius` | ステノプテリギウス | *Stenopterygius quadriscissus* | 魚竜 | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `dapedium` | ダペディウム | *Dapedium* | 条鰭類（硬骨魚） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `passaloteuthis` | パッサロテウティス（ベレムナイト） | *Passaloteuthis* | 頭足類（ベレムナイト） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `dactylioceras` | ダクチリオセラス | *Dactylioceras commune* | 頭足類（アンモナイト） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `liopleurodon` | リオプレウロドン | *Liopleurodon ferox* | 首長竜（首の短いプリオサウルス類） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `leedsichthys` | リードシクティス | *Leedsichthys problematicus* | 条鰭類（硬骨魚） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `anchiornis` | アンキオルニス | *Anchiornis huxleyi* | 恐竜（羽毛のある小型の肉食恐竜） | 歩く（横から） | ⬜ | ⬜ |
| `castorocauda` | カストロカウダ | *Castorocauda lutrasimilis* | 哺乳形類（泳ぐ哺乳類） | 歩く（横から） | ⬜ | ⬜ |
| `allosaurus` | アロサウルス | *Allosaurus fragilis* | 恐竜（肉食恐竜） | 歩く（横から） | ⬜ | ⬜ |
| `stegosaurus` | ステゴサウルス | *Stegosaurus stenops* | 恐竜（剣竜） | 歩く（横から） | ⬜ | ⬜ |
| `diplodocus` | ディプロドクス | *Diplodocus carnegii* | 恐竜（竜脚類） | 歩く（横から） | ⬜ | ⬜ |
| `brachiosaurus` | ブラキオサウルス | *Brachiosaurus altithorax* | 恐竜（竜脚類） | 歩く（横から） | ⬜ | ⬜ |
| `archaeopteryx` | 始祖鳥（アーケオプテリクス） | *Archaeopteryx lithographica* | 鳥類（最古の鳥の一つ） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `rhamphorhynchus` | ランフォリンクス | *Rhamphorhynchus muensteri* | 翼竜 | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `compsognathus` | コンプソグナトゥス | *Compsognathus longipes* | 恐竜（小型の肉食恐竜） | 歩く（横から） | ⬜ | ⬜ |
| `williamsonia` | ウィリアムソニア（ベネチテス類） | *Williamsonia* | ベネチテス類（ソテツに似た植物） | 立っている植物 | ⬜ | ⬜ |
| `ginkgo` | イチョウの仲間 | *Ginkgo* | イチョウ類 | 立っている植物 | ⬜ | ⬜ |
| `laurasia` | ローラシア大陸 | — | 現象 | 上空から見た大陸 | ⬜ | 不要 |

遠景（任意）：`backdrop-jurassic-land` ⬜、`backdrop-jurassic-sea` ⬜、`backdrop-jurassic-deep` ⬜

## 白亜紀（Cretaceous）

| slug | 名前 | 学名 | 種類 | 観察での姿 | 図鑑 jpg | スプライト png |
|---|---|---|---|---|---|---|
| `iguanodon` | イグアノドン | *Iguanodon bernissartensis* | 恐竜（鳥脚類） | 歩く（横から） | ⬜ | ⬜ |
| `sinosauropteryx` | シノサウロプテリクス | *Sinosauropteryx prima* | 恐竜（羽毛のある小型の肉食恐竜） | 歩く（横から） | ⬜ | ⬜ |
| `microraptor` | ミクロラプトル | *Microraptor gui* | 恐竜（4枚の翼を持つ小型恐竜） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `psittacosaurus` | プシッタコサウルス | *Psittacosaurus* | 恐竜（初期の角竜） | 歩く（横から） | ⬜ | ⬜ |
| `confuciusornis` | コンフキウソルニス（孔子鳥） | *Confuciusornis sanctus* | 鳥類 | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `repenomamus` | レペノマムス | *Repenomamus giganticus* | 哺乳類 | 歩く（横から） | ⬜ | ⬜ |
| `archaefructus` | アルカエフルクトゥス（古果） | *Archaefructus sinensis* | 被子植物（初期の花の咲く植物） | 立っている植物 | ⬜ | ⬜ |
| `spinosaurus` | スピノサウルス | *Spinosaurus aegyptiacus* | 恐竜（水辺の大型肉食恐竜） | 歩く（横から） | ⬜ | ⬜ |
| `argentinosaurus` | アルゼンチノサウルス | *Argentinosaurus huinculensis* | 恐竜（竜脚類） | 歩く（横から） | ⬜ | ⬜ |
| `giganotosaurus` | ギガノトサウルス | *Giganotosaurus carolinii* | 恐竜（大型の肉食恐竜） | 歩く（横から） | ⬜ | ⬜ |
| `elasmosaurus` | エラスモサウルス | *Elasmosaurus platyurus* | 首長竜 | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `tylosaurus` | ティロサウルス | *Tylosaurus proriger* | モササウルス類（海のトカゲ） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `pteranodon` | プテラノドン | *Pteranodon longiceps* | 翼竜 | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `xiphactinus` | シファクティヌス | *Xiphactinus audax* | 条鰭類（硬骨魚） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `archelon` | アーケロン | *Archelon ischyros* | カメ（ウミガメ） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `cretoxyrhina` | クレトキシリナ | *Cretoxyrhina mantelli* | 軟骨魚類（サメ） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `baculites` | バキュリテス | *Baculites* | 頭足類（まっすぐなアンモナイト） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `mosasaurus` | モササウルス | *Mosasaurus hoffmannii* | モササウルス類（海のトカゲ） | 泳ぐ・漂う（横から） | ⬜ | ⬜ |
| `velociraptor` | ヴェロキラプトル | *Velociraptor mongoliensis* | 恐竜（羽毛のある小型の肉食恐竜） | 歩く（横から） | ⬜ | ⬜ |
| `protoceratops` | プロトケラトプス | *Protoceratops andrewsi* | 恐竜（角竜） | 歩く（横から） | ⬜ | ⬜ |
| `parasaurolophus` | パラサウロロフス | *Parasaurolophus walkeri* | 恐竜（鳥脚類・ハドロサウルス類） | 歩く（横から） | ⬜ | ⬜ |
| `deinosuchus` | デイノスクス | *Deinosuchus* | ワニの仲間 | 歩く（横から） | ⬜ | ⬜ |
| `tyrannosaurus` | ティラノサウルス | *Tyrannosaurus rex* | 恐竜（大型の肉食恐竜） | 歩く（横から） | ⬜ | ⬜ |
| `triceratops` | トリケラトプス | *Triceratops horridus* | 恐竜（角竜） | 歩く（横から） | ⬜ | ⬜ |
| `ankylosaurus` | アンキロサウルス | *Ankylosaurus magniventris* | 恐竜（鎧竜） | 歩く（横から） | ⬜ | ⬜ |
| `edmontosaurus` | エドモントサウルス | *Edmontosaurus annectens* | 恐竜（鳥脚類・ハドロサウルス類） | 歩く（横から） | ⬜ | ⬜ |
| `quetzalcoatlus` | ケツァルコアトルス | *Quetzalcoatlus northropi* | 翼竜（アズダルコ類） | 歩く（横から） | ⬜ | ⬜ |
| `early-magnolia` | 初期のモクレンの仲間 | *Magnoliaceae (early)* | 被子植物（花の咲く木） | 立っている植物 | ⬜ | ⬜ |
| `flowering-forest` | 花の咲く森 | — | 現象 | 陸の景色 | ⬜ | 不要 |
| `deccan-traps` | デカン・トラップの噴火 | — | 現象 | 巨大噴火 | ⬜ | 不要 |
| `chicxulub-impact` | チクシュルーブの衝突 | — | 現象 | 巨大噴火 | ⬜ | 不要 |

遠景（任意）：`backdrop-cretaceous-land` ⬜、`backdrop-cretaceous-sea` ⬜、`backdrop-cretaceous-deep` ⬜

## 生成プロンプト（例）

これまでの画像（`assets/artwork/manifest.json`・`assets/units/manifest.json`）と同じ書き方にそろえています。足りない項目だけ載せます。

### `thrinaxodon` トリナクソドン（*Thrinaxodon liorhinus*）

図鑑 `assets/artwork/thrinaxodon.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Thrinaxodon (Thrinaxodon liorhinus), Synapsid (cynodont), Triassic period, about 251 million years ago. A small burrowing animal living just after the great extinction, a very close relative of mammals. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/thrinaxodon.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Thrinaxodon (Thrinaxodon liorhinus), Synapsid (cynodont): A small burrowing animal living just after the great extinction, a very close relative of mammals. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `pleuromeia` プレウロメイア（*Pleuromeia*）

図鑑 `assets/artwork/pleuromeia.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Pleuromeia (Pleuromeia), Lycopsid, Triassic period, about 252 million years ago. A single-stemmed plant about 2 m tall that was among the first to spread across the barren land after the great extinction. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/pleuromeia.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Pleuromeia (Pleuromeia), Lycopsid: A single-stemmed plant about 2 m tall that was among the first to spread across the barren land after the great extinction. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Whole plant upright from ground level to its top, base at the bottom edge, nothing cropped. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `triadobatrachus` トリアドバトラクス（*Triadobatrachus massinoti*）

図鑑 `assets/artwork/triadobatrachus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Triadobatrachus (Triadobatrachus massinoti), Amphibian (frog ancestor), Triassic period, about 250 million years ago. A frog ancestor that still had a short tail and swam better than it hopped. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/triadobatrachus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Triadobatrachus (Triadobatrachus massinoti), Amphibian (frog ancestor): A frog ancestor that still had a short tail and swam better than it hopped. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `cynognathus` キノグナトゥス（*Cynognathus crateronotus*）

図鑑 `assets/artwork/cynognathus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Cynognathus (Cynognathus crateronotus), Synapsid (cynodont), Triassic period, about 247 million years ago. A wolf-sized predator with different kinds of teeth, like a mammal. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/cynognathus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Cynognathus (Cynognathus crateronotus), Synapsid (cynodont): A wolf-sized predator with different kinds of teeth, like a mammal. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `mixosaurus` ミクソサウルス（*Mixosaurus*）

図鑑 `assets/artwork/mixosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Mixosaurus (Mixosaurus), Ichthyosaur, Triassic period, about 245 million years ago. A small ichthyosaur found in seas worldwide, catching fish and squid with its slender jaws. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/mixosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Mixosaurus (Mixosaurus), Ichthyosaur: A small ichthyosaur found in seas worldwide, catching fish and squid with its slender jaws. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `nothosaurus` ノトサウルス（*Nothosaurus*）

図鑑 `assets/artwork/nothosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Nothosaurus (Nothosaurus), Nothosaur (marine reptile), Triassic period, about 243 million years ago. A long-necked marine reptile with needle-toothed jaws that fished in shallow seas and rested on the shore. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred shallow water's edge, estuary or tidal flat. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/nothosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Nothosaurus (Nothosaurus), Nothosaur (marine reptile): A long-necked marine reptile with needle-toothed jaws that fished in shallow seas and rested on the shore. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `tanystropheus` タニストロフェウス（*Tanystropheus*）

図鑑 `assets/artwork/tanystropheus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Tanystropheus (Tanystropheus), Long-necked archosauromorph, Triassic period, about 243 million years ago. A reptile whose neck was half its body length, used to snatch fish in shallow water. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred shallow water's edge, estuary or tidal flat. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/tanystropheus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Tanystropheus (Tanystropheus), Long-necked archosauromorph: A reptile whose neck was half its body length, used to snatch fish in shallow water. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `placodus` プラコドゥス（*Placodus gigas*）

図鑑 `assets/artwork/placodus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Placodus (Placodus gigas), Placodont (marine reptile), Triassic period, about 245 million years ago. A reptile with flat, millstone-like teeth for crushing shellfish on the sea floor. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/placodus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Placodus (Placodus gigas), Placodont (marine reptile): A reptile with flat, millstone-like teeth for crushing shellfish on the sea floor. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `ceratites` セラタイト（*Ceratites nodosus*）

図鑑 `assets/artwork/ceratites.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Ceratites (Ceratites nodosus), Cephalopod (ammonoid), Triassic period, about 242 million years ago. A ribbed, knobbly coiled ammonoid whose chamber walls trace saw-tooth lines. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/ceratites.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Ceratites (Ceratites nodosus), Cephalopod (ammonoid): A ribbed, knobbly coiled ammonoid whose chamber walls trace saw-tooth lines. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `eoraptor` エオラプトル（*Eoraptor lunensis*）

図鑑 `assets/artwork/eoraptor.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Eoraptor (Eoraptor lunensis), Dinosaur (early saurischian), Triassic period, about 231 million years ago. One of the oldest dinosaurs, about 1 m long, running fast on two legs after insects and small animals. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/eoraptor.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Eoraptor (Eoraptor lunensis), Dinosaur (early saurischian): One of the oldest dinosaurs, about 1 m long, running fast on two legs after insects and small animals. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `herrerasaurus` ヘレラサウルス（*Herrerasaurus ischigualastensis*）

図鑑 `assets/artwork/herrerasaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Herrerasaurus (Herrerasaurus ischigualastensis), Dinosaur (early predator), Triassic period, about 231 million years ago. An early large meat-eating dinosaur about 4 m long, hunting with sharp teeth and claws. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/herrerasaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Herrerasaurus (Herrerasaurus ischigualastensis), Dinosaur (early predator): An early large meat-eating dinosaur about 4 m long, hunting with sharp teeth and claws. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `coelophysis` コエロフィシス（*Coelophysis bauri*）

図鑑 `assets/artwork/coelophysis.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Coelophysis (Coelophysis bauri), Dinosaur (theropod), Triassic period, about 215 million years ago. A slender, lightweight meat-eating dinosaur that may have lived in groups. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/coelophysis.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Coelophysis (Coelophysis bauri), Dinosaur (theropod): A slender, lightweight meat-eating dinosaur that may have lived in groups. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `plateosaurus` プラテオサウルス（*Plateosaurus trossingensis*）

図鑑 `assets/artwork/plateosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Plateosaurus (Plateosaurus trossingensis), Dinosaur (sauropodomorph), Triassic period, about 214 million years ago. An early large plant-eating dinosaur that browsed high leaves with its long neck, walking on two legs. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/plateosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Plateosaurus (Plateosaurus trossingensis), Dinosaur (sauropodomorph): An early large plant-eating dinosaur that browsed high leaves with its long neck, walking on two legs. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `postosuchus` ポストスクス（*Postosuchus kirkpatricki*）

図鑑 `assets/artwork/postosuchus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Postosuchus (Postosuchus kirkpatricki), Pseudosuchian (crocodile-line), Triassic period, about 222 million years ago. A big-headed, sharp-toothed top predator of Triassic land and a distant crocodile relative. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/postosuchus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Postosuchus (Postosuchus kirkpatricki), Pseudosuchian (crocodile-line): A big-headed, sharp-toothed top predator of Triassic land and a distant crocodile relative. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `desmatosuchus` デスマトスクス（*Desmatosuchus spurensis*）

図鑑 `assets/artwork/desmatosuchus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Desmatosuchus (Desmatosuchus spurensis), Aetosaur, Triassic period, about 222 million years ago. A plant-eating crocodile relative covered in bony plates, with long spikes jutting from its shoulders. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/desmatosuchus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Desmatosuchus (Desmatosuchus spurensis), Aetosaur: A plant-eating crocodile relative covered in bony plates, with long spikes jutting from its shoulders. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `eudimorphodon` エウディモルフォドン（*Eudimorphodon ranzii*）

図鑑 `assets/artwork/eudimorphodon.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Eudimorphodon (Eudimorphodon ranzii), Pterosaur, Triassic period, about 215 million years ago. One of the oldest pterosaurs, with a 1 m wingspan and a long tail, catching fish. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/eudimorphodon.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Eudimorphodon (Eudimorphodon ranzii), Pterosaur: One of the oldest pterosaurs, with a 1 m wingspan and a long tail, catching fish. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `shonisaurus` ショニサウルス（*Shonisaurus popularis*）

図鑑 `assets/artwork/shonisaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Shonisaurus (Shonisaurus popularis), Ichthyosaur, Triassic period, about 217 million years ago. A giant ichthyosaur up to 15 m long that may have sucked in squid with its long, nearly toothless snout. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred dark deep-sea water and a muddy sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/shonisaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Shonisaurus (Shonisaurus popularis), Ichthyosaur: A giant ichthyosaur up to 15 m long that may have sucked in squid with its long, nearly toothless snout. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `metoposaurus` メトポサウルス（*Metoposaurus*）

図鑑 `assets/artwork/metoposaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Metoposaurus (Metoposaurus), Amphibian (temnospondyl), Triassic period, about 230 million years ago. A big flat-headed amphibian that lay in wait for fish on the beds of shallow lakes. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/metoposaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Metoposaurus (Metoposaurus), Amphibian (temnospondyl): A big flat-headed amphibian that lay in wait for fish on the beds of shallow lakes. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `dicroidium` ディクロイディウム（*Dicroidium*）

図鑑 `assets/artwork/dicroidium.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Dicroidium (Dicroidium), Seed-fern tree, Triassic period, about 245 million years ago. A tree with forked fronds that dominated the Triassic forests of Gondwana. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/dicroidium.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Dicroidium (Dicroidium), Seed-fern tree: A tree with forked fronds that dominated the Triassic forests of Gondwana. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Whole plant upright from ground level to its top, base at the bottom edge, nothing cropped. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `araucarioxylon` アラウカリオキシロン（化石の森の木）（*Araucarioxylon arizonicum*）

図鑑 `assets/artwork/araucarioxylon.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Araucarioxylon (petrified forest tree) (Araucarioxylon arizonicum), Conifer, Triassic period, about 225 million years ago. A conifer up to 30 m tall whose trunks, buried by rivers, turned into rainbow-coloured stone. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/araucarioxylon.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Araucarioxylon (petrified forest tree) (Araucarioxylon arizonicum), Conifer: A conifer up to 30 m tall whose trunks, buried by rivers, turned into rainbow-coloured stone. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Whole plant upright from ground level to its top, base at the bottom edge, nothing cropped. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `megazostrodon` メガゾストロドン（*Megazostrodon rudnerae*）

図鑑 `assets/artwork/megazostrodon.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Megazostrodon (Megazostrodon rudnerae), Early mammal, Jurassic period, about 201 million years ago. A tiny shrew-like animal that came out at night to eat insects. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/megazostrodon.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Megazostrodon (Megazostrodon rudnerae), Early mammal: A tiny shrew-like animal that came out at night to eat insects. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `dilophosaurus` ディロフォサウルス（*Dilophosaurus wetherilli*）

図鑑 `assets/artwork/dilophosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Dilophosaurus (Dilophosaurus wetherilli), Dinosaur (theropod), Jurassic period, about 196 million years ago. A large Early Jurassic predator with two thin crests on its head. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/dilophosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Dilophosaurus (Dilophosaurus wetherilli), Dinosaur (theropod): A large Early Jurassic predator with two thin crests on its head. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `ichthyosaurus` イクチオサウルス（*Ichthyosaurus communis*）

図鑑 `assets/artwork/ichthyosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Ichthyosaurus (Ichthyosaurus communis), Ichthyosaur, Jurassic period, about 200 million years ago. A fast, dolphin-shaped ichthyosaur whose large eyes found prey even in dark water. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/ichthyosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Ichthyosaurus (Ichthyosaurus communis), Ichthyosaur: A fast, dolphin-shaped ichthyosaur whose large eyes found prey even in dark water. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `plesiosaurus` プレシオサウルス（*Plesiosaurus dolichodeirus*）

図鑑 `assets/artwork/plesiosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Plesiosaurus (Plesiosaurus dolichodeirus), Plesiosaur, Jurassic period, about 200 million years ago. A long-necked plesiosaur that 'flew' through the water with four flippers like wings. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/plesiosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Plesiosaurus (Plesiosaurus dolichodeirus), Plesiosaur: A long-necked plesiosaur that 'flew' through the water with four flippers like wings. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `dimorphodon` ディモルフォドン（*Dimorphodon macronyx*）

図鑑 `assets/artwork/dimorphodon.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Dimorphodon (Dimorphodon macronyx), Pterosaur, Jurassic period, about 200 million years ago. A big-headed, long-tailed pterosaur with a wingspan of about 1.4 m. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/dimorphodon.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Dimorphodon (Dimorphodon macronyx), Pterosaur: A big-headed, long-tailed pterosaur with a wingspan of about 1.4 m. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `scelidosaurus` スケリドサウルス（*Scelidosaurus harrisonii*）

図鑑 `assets/artwork/scelidosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Scelidosaurus (Scelidosaurus harrisonii), Dinosaur (early thyreophoran), Jurassic period, about 196 million years ago. A plant-eating dinosaur with rows of bony plates, close to the ancestors of stegosaurs and ankylosaurs. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/scelidosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Scelidosaurus (Scelidosaurus harrisonii), Dinosaur (early thyreophoran): A plant-eating dinosaur with rows of bony plates, close to the ancestors of stegosaurs and ankylosaurs. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `stenopterygius` ステノプテリギウス（*Stenopterygius quadriscissus*）

図鑑 `assets/artwork/stenopterygius.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Stenopterygius (Stenopterygius quadriscissus), Ichthyosaur, Jurassic period, about 183 million years ago. A dolphin-like ichthyosaur with a dorsal fin and crescent-shaped tail. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/stenopterygius.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Stenopterygius (Stenopterygius quadriscissus), Ichthyosaur: A dolphin-like ichthyosaur with a dorsal fin and crescent-shaped tail. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `dapedium` ダペディウム（*Dapedium*）

図鑑 `assets/artwork/dapedium.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Dapedium (Dapedium), Ray-finned fish, Jurassic period, about 200 million years ago. A disc-shaped fish with thick diamond scales that nibbled hard-shelled animals. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/dapedium.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Dapedium (Dapedium), Ray-finned fish: A disc-shaped fish with thick diamond scales that nibbled hard-shelled animals. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `passaloteuthis` パッサロテウティス（ベレムナイト）（*Passaloteuthis*）

図鑑 `assets/artwork/passaloteuthis.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Passaloteuthis (belemnite) (Passaloteuthis), Cephalopod (belemnite), Jurassic period, about 190 million years ago. A squid-like cephalopod with a hard bullet-shaped guard inside its body, swimming in schools. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/passaloteuthis.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Passaloteuthis (belemnite) (Passaloteuthis), Cephalopod (belemnite): A squid-like cephalopod with a hard bullet-shaped guard inside its body, swimming in schools. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `dactylioceras` ダクチリオセラス（*Dactylioceras commune*）

図鑑 `assets/artwork/dactylioceras.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Dactylioceras (Dactylioceras commune), Cephalopod (ammonite), Jurassic period, about 183 million years ago. A flat-coiled ammonite with fine ribs, abundant in Jurassic seas. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/dactylioceras.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Dactylioceras (Dactylioceras commune), Cephalopod (ammonite): A flat-coiled ammonite with fine ribs, abundant in Jurassic seas. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `liopleurodon` リオプレウロドン（*Liopleurodon ferox*）

図鑑 `assets/artwork/liopleurodon.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Liopleurodon (Liopleurodon ferox), Pliosaur, Jurassic period, about 166 million years ago. A short-necked plesiosaur with a huge head and powerful jaws — a top predator of Jurassic seas. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred dark deep-sea water and a muddy sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/liopleurodon.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Liopleurodon (Liopleurodon ferox), Pliosaur: A short-necked plesiosaur with a huge head and powerful jaws — a top predator of Jurassic seas. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `leedsichthys` リードシクティス（*Leedsichthys problematicus*）

図鑑 `assets/artwork/leedsichthys.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Leedsichthys (Leedsichthys problematicus), Ray-finned fish, Jurassic period, about 166 million years ago. One of the largest bony fishes ever, up to 16 m, filtering plankton with its huge mouth. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/leedsichthys.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Leedsichthys (Leedsichthys problematicus), Ray-finned fish: One of the largest bony fishes ever, up to 16 m, filtering plankton with its huge mouth. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `anchiornis` アンキオルニス（*Anchiornis huxleyi*）

図鑑 `assets/artwork/anchiornis.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Anchiornis (Anchiornis huxleyi), Feathered dinosaur, Jurassic period, about 161 million years ago. A pigeon-sized dinosaur with feathers on its arms and legs, black-and-white plumage and a reddish crest. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/anchiornis.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Anchiornis (Anchiornis huxleyi), Feathered dinosaur: A pigeon-sized dinosaur with feathers on its arms and legs, black-and-white plumage and a reddish crest. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `castorocauda` カストロカウダ（*Castorocauda lutrasimilis*）

図鑑 `assets/artwork/castorocauda.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Castorocauda (Castorocauda lutrasimilis), Swimming mammaliaform, Jurassic period, about 165 million years ago. A dinosaur-age mammal relative that swam with a beaver-like tail and an otter-like body. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/castorocauda.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Castorocauda (Castorocauda lutrasimilis), Swimming mammaliaform: A dinosaur-age mammal relative that swam with a beaver-like tail and an otter-like body. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `allosaurus` アロサウルス（*Allosaurus fragilis*）

図鑑 `assets/artwork/allosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Allosaurus (Allosaurus fragilis), Dinosaur (theropod), Jurassic period, about 155 million years ago. The top meat-eater of Late Jurassic North America, with small horns above its eyes. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/allosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Allosaurus (Allosaurus fragilis), Dinosaur (theropod): The top meat-eater of Late Jurassic North America, with small horns above its eyes. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `stegosaurus` ステゴサウルス（*Stegosaurus stenops*）

図鑑 `assets/artwork/stegosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Stegosaurus (Stegosaurus stenops), Dinosaur (stegosaur), Jurassic period, about 155 million years ago. A plant-eater with two rows of big bony plates on its back and four spikes on its tail. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/stegosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Stegosaurus (Stegosaurus stenops), Dinosaur (stegosaur): A plant-eater with two rows of big bony plates on its back and four spikes on its tail. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `diplodocus` ディプロドクス（*Diplodocus carnegii*）

図鑑 `assets/artwork/diplodocus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Diplodocus (Diplodocus carnegii), Dinosaur (sauropod), Jurassic period, about 154 million years ago. A 26 m giant plant-eater with a long neck and a whip-like tail. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/diplodocus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Diplodocus (Diplodocus carnegii), Dinosaur (sauropod): A 26 m giant plant-eater with a long neck and a whip-like tail. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `brachiosaurus` ブラキオサウルス（*Brachiosaurus altithorax*）

図鑑 `assets/artwork/brachiosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Brachiosaurus (Brachiosaurus altithorax), Dinosaur (sauropod), Jurassic period, about 154 million years ago. A giant with long front legs that held its neck high to eat leaves from tall trees. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/brachiosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Brachiosaurus (Brachiosaurus altithorax), Dinosaur (sauropod): A giant with long front legs that held its neck high to eat leaves from tall trees. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `archaeopteryx` 始祖鳥（アーケオプテリクス）（*Archaeopteryx lithographica*）

図鑑 `assets/artwork/archaeopteryx.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Archaeopteryx (Archaeopteryx lithographica), Early bird, Jurassic period, about 151 million years ago. An animal with wings and feathers but also teeth, a long bony tail and clawed fingers — half dinosaur, half bird. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/archaeopteryx.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Archaeopteryx (Archaeopteryx lithographica), Early bird: An animal with wings and feathers but also teeth, a long bony tail and clawed fingers — half dinosaur, half bird. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `rhamphorhynchus` ランフォリンクス（*Rhamphorhynchus muensteri*）

図鑑 `assets/artwork/rhamphorhynchus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Rhamphorhynchus (Rhamphorhynchus muensteri), Pterosaur, Jurassic period, about 151 million years ago. A long-tailed pterosaur with a diamond-shaped vane on its tail tip and forward-pointing teeth for catching fish. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/rhamphorhynchus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Rhamphorhynchus (Rhamphorhynchus muensteri), Pterosaur: A long-tailed pterosaur with a diamond-shaped vane on its tail tip and forward-pointing teeth for catching fish. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `compsognathus` コンプソグナトゥス（*Compsognathus longipes*）

図鑑 `assets/artwork/compsognathus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Compsognathus (Compsognathus longipes), Dinosaur (small theropod), Jurassic period, about 151 million years ago. A nimble chicken-sized meat-eater; one fossil has a lizard in its stomach. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/compsognathus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Compsognathus (Compsognathus longipes), Dinosaur (small theropod): A nimble chicken-sized meat-eater; one fossil has a lizard in its stomach. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `williamsonia` ウィリアムソニア（ベネチテス類）（*Williamsonia*）

図鑑 `assets/artwork/williamsonia.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Williamsonia (bennettitalean) (Williamsonia), Bennettitalean (cycad-like), Jurassic period, about 200 million years ago. A cycad-like plant with flower-like reproductive organs, common across dinosaur-age landscapes. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/williamsonia.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Williamsonia (bennettitalean) (Williamsonia), Bennettitalean (cycad-like): A cycad-like plant with flower-like reproductive organs, common across dinosaur-age landscapes. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Whole plant upright from ground level to its top, base at the bottom edge, nothing cropped. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `ginkgo` イチョウの仲間（*Ginkgo*）

図鑑 `assets/artwork/ginkgo.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Ginkgo (Ginkgo), Ginkgophyte, Jurassic period, about 200 million years ago. A tree with fan-shaped leaves that spread worldwide in the Jurassic in many kinds. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/ginkgo.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Ginkgo (Ginkgo), Ginkgophyte: A tree with fan-shaped leaves that spread worldwide in the Jurassic in many kinds. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Whole plant upright from ground level to its top, base at the bottom edge, nothing cropped. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `iguanodon` イグアノドン（*Iguanodon bernissartensis*）

図鑑 `assets/artwork/iguanodon.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Iguanodon (Iguanodon bernissartensis), Dinosaur (ornithopod), Cretaceous period, about 130 million years ago. A herding plant-eater whose thumbs were big spikes. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Mesozoic landscape with conifers, cycads, ginkgos and ferns, no flowers or grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/iguanodon.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Iguanodon (Iguanodon bernissartensis), Dinosaur (ornithopod): A herding plant-eater whose thumbs were big spikes. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `sinosauropteryx` シノサウロプテリクス（*Sinosauropteryx prima*）

図鑑 `assets/artwork/sinosauropteryx.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Sinosauropteryx (Sinosauropteryx prima), Feathered dinosaur, Cretaceous period, about 125 million years ago. A small meat-eater covered in fluffy feathers, with a striped tail. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/sinosauropteryx.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Sinosauropteryx (Sinosauropteryx prima), Feathered dinosaur: A small meat-eater covered in fluffy feathers, with a striped tail. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `microraptor` ミクロラプトル（*Microraptor gui*）

図鑑 `assets/artwork/microraptor.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Microraptor (Microraptor gui), Four-winged dinosaur, Cretaceous period, about 122 million years ago. A small dinosaur with wing-like feathers on both arms and legs that glided between trees. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/microraptor.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Microraptor (Microraptor gui), Four-winged dinosaur: A small dinosaur with wing-like feathers on both arms and legs that glided between trees. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `psittacosaurus` プシッタコサウルス（*Psittacosaurus*）

図鑑 `assets/artwork/psittacosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Psittacosaurus (Psittacosaurus), Dinosaur (early ceratopsian), Cretaceous period, about 125 million years ago. A small plant-eater with a parrot-like beak and long bristles on its tail. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/psittacosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Psittacosaurus (Psittacosaurus), Dinosaur (early ceratopsian): A small plant-eater with a parrot-like beak and long bristles on its tail. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `confuciusornis` コンフキウソルニス（孔子鳥）（*Confuciusornis sanctus*）

図鑑 `assets/artwork/confuciusornis.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Confuciusornis (Confuciusornis sanctus), Early bird, Cretaceous period, about 125 million years ago. An early bird with a toothless beak and two long tail streamers that lived in flocks. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/confuciusornis.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Confuciusornis (Confuciusornis sanctus), Early bird: An early bird with a toothless beak and two long tail streamers that lived in flocks. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `repenomamus` レペノマムス（*Repenomamus giganticus*）

図鑑 `assets/artwork/repenomamus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Repenomamus (Repenomamus giganticus), Mammal, Cretaceous period, about 125 million years ago. A badger-sized mammal, big for the age of dinosaurs. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/repenomamus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Repenomamus (Repenomamus giganticus), Mammal: A badger-sized mammal, big for the age of dinosaurs. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `archaefructus` アルカエフルクトゥス（古果）（*Archaefructus sinensis*）

図鑑 `assets/artwork/archaefructus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Archaefructus (Archaefructus sinensis), Early flowering plant, Cretaceous period, about 125 million years ago. One of the oldest flowering plants, growing at the water's edge — it had no petals yet. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred shallow water's edge, estuary or tidal flat. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/archaefructus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Archaefructus (Archaefructus sinensis), Early flowering plant: One of the oldest flowering plants, growing at the water's edge — it had no petals yet. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Whole plant upright from ground level to its top, base at the bottom edge, nothing cropped. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `spinosaurus` スピノサウルス（*Spinosaurus aegyptiacus*）

図鑑 `assets/artwork/spinosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Spinosaurus (Spinosaurus aegyptiacus), Dinosaur (semi-aquatic theropod), Cretaceous period, about 99 million years ago. A 15 m sail-backed predator with crocodile-like jaws for catching river fish. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/spinosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Spinosaurus (Spinosaurus aegyptiacus), Dinosaur (semi-aquatic theropod): A 15 m sail-backed predator with crocodile-like jaws for catching river fish. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `argentinosaurus` アルゼンチノサウルス（*Argentinosaurus huinculensis*）

図鑑 `assets/artwork/argentinosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Argentinosaurus (Argentinosaurus huinculensis), Dinosaur (titanosaur), Cretaceous period, about 96 million years ago. Possibly 35 m long and 70 tonnes — one of the largest land animals ever. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/argentinosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Argentinosaurus (Argentinosaurus huinculensis), Dinosaur (titanosaur): Possibly 35 m long and 70 tonnes — one of the largest land animals ever. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `giganotosaurus` ギガノトサウルス（*Giganotosaurus carolinii*）

図鑑 `assets/artwork/giganotosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Giganotosaurus (Giganotosaurus carolinii), Dinosaur (large theropod), Cretaceous period, about 99 million years ago. A giant South American meat-eater as big as Tyrannosaurus. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/giganotosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Giganotosaurus (Giganotosaurus carolinii), Dinosaur (large theropod): A giant South American meat-eater as big as Tyrannosaurus. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `elasmosaurus` エラスモサウルス（*Elasmosaurus platyurus*）

図鑑 `assets/artwork/elasmosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Elasmosaurus (Elasmosaurus platyurus), Plesiosaur (elasmosaur), Cretaceous period, about 81 million years ago. A plesiosaur whose neck was over half its length, with more than 70 neck bones. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/elasmosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Elasmosaurus (Elasmosaurus platyurus), Plesiosaur (elasmosaur): A plesiosaur whose neck was over half its length, with more than 70 neck bones. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `tylosaurus` ティロサウルス（*Tylosaurus proriger*）

図鑑 `assets/artwork/tylosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Tylosaurus (Tylosaurus proriger), Mosasaur, Cretaceous period, about 88 million years ago. A 12 m sea lizard that may have rammed prey with its pointed snout. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/tylosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Tylosaurus (Tylosaurus proriger), Mosasaur: A 12 m sea lizard that may have rammed prey with its pointed snout. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `pteranodon` プテラノドン（*Pteranodon longiceps*）

図鑑 `assets/artwork/pteranodon.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Pteranodon (Pteranodon longiceps), Pterosaur, Cretaceous period, about 88 million years ago. A big pterosaur with a wingspan over 6 m, a toothless beak and a backward crest. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/pteranodon.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Pteranodon (Pteranodon longiceps), Pterosaur: A big pterosaur with a wingspan over 6 m, a toothless beak and a backward crest. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `xiphactinus` シファクティヌス（*Xiphactinus audax*）

図鑑 `assets/artwork/xiphactinus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Xiphactinus (Xiphactinus audax), Ray-finned fish, Cretaceous period, about 88 million years ago. A 5 m bulldog-faced fish with fangs that swallowed big fish whole. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/xiphactinus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Xiphactinus (Xiphactinus audax), Ray-finned fish: A 5 m bulldog-faced fish with fangs that swallowed big fish whole. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `archelon` アーケロン（*Archelon ischyros*）

図鑑 `assets/artwork/archelon.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Archelon (Archelon ischyros), Sea turtle, Cretaceous period, about 80 million years ago. At 4 m long, the largest sea turtle ever, with a leathery, ribbed shell instead of solid armour. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/archelon.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Archelon (Archelon ischyros), Sea turtle: At 4 m long, the largest sea turtle ever, with a leathery, ribbed shell instead of solid armour. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `cretoxyrhina` クレトキシリナ（*Cretoxyrhina mantelli*）

図鑑 `assets/artwork/cretoxyrhina.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Cretoxyrhina (Cretoxyrhina mantelli), Shark, Cretaceous period, about 100 million years ago. A 7 m shark like a great white that attacked even plesiosaurs and mosasaurs. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred dark deep-sea water and a muddy sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/cretoxyrhina.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Cretoxyrhina (Cretoxyrhina mantelli), Shark: A 7 m shark like a great white that attacked even plesiosaurs and mosasaurs. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `baculites` バキュリテス（*Baculites*）

図鑑 `assets/artwork/baculites.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Baculites (Baculites), Cephalopod (straight ammonite), Cretaceous period, about 85 million years ago. An unusual ammonite whose shell grew straight instead of coiling. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred teal water and a small patch of sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/baculites.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Baculites (Baculites), Cephalopod (straight ammonite): An unusual ammonite whose shell grew straight instead of coiling. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `mosasaurus` モササウルス（*Mosasaurus hoffmannii*）

図鑑 `assets/artwork/mosasaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Mosasaurus (Mosasaurus hoffmannii), Mosasaur, Cretaceous period, about 82 million years ago. Up to 15 m long, the ruler of the latest Cretaceous seas — a distant relative of monitor lizards and snakes. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred dark deep-sea water and a muddy sea floor. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/mosasaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Mosasaurus (Mosasaurus hoffmannii), Mosasaur: Up to 15 m long, the ruler of the latest Cretaceous seas — a distant relative of monitor lizards and snakes. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view swimming pose, head pointing RIGHT and tail LEFT. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `velociraptor` ヴェロキラプトル（*Velociraptor mongoliensis*）

図鑑 `assets/artwork/velociraptor.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Velociraptor (Velociraptor mongoliensis), Dinosaur (dromaeosaur), Cretaceous period, about 75 million years ago. A turkey-sized feathered predator with a big hooked claw on each foot. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/velociraptor.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Velociraptor (Velociraptor mongoliensis), Dinosaur (dromaeosaur): A turkey-sized feathered predator with a big hooked claw on each foot. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `protoceratops` プロトケラトプス（*Protoceratops andrewsi*）

図鑑 `assets/artwork/protoceratops.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Protoceratops (Protoceratops andrewsi), Dinosaur (ceratopsian), Cretaceous period, about 75 million years ago. A sheep-sized plant-eater with a neck frill that gathered around desert oases. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/protoceratops.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Protoceratops (Protoceratops andrewsi), Dinosaur (ceratopsian): A sheep-sized plant-eater with a neck frill that gathered around desert oases. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `parasaurolophus` パラサウロロフス（*Parasaurolophus walkeri*）

図鑑 `assets/artwork/parasaurolophus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Parasaurolophus (Parasaurolophus walkeri), Dinosaur (hadrosaur), Cretaceous period, about 77 million years ago. A duck-billed dinosaur with a long tube-like crest sweeping back from its head. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/parasaurolophus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Parasaurolophus (Parasaurolophus walkeri), Dinosaur (hadrosaur): A duck-billed dinosaur with a long tube-like crest sweeping back from its head. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `deinosuchus` デイノスクス（*Deinosuchus*）

図鑑 `assets/artwork/deinosuchus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Deinosuchus (Deinosuchus), Crocodylian relative, Cretaceous period, about 82 million years ago. A giant crocodile up to 10 m long that ambushed dinosaurs at the water's edge. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred shallow water's edge, estuary or tidal flat. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/deinosuchus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Deinosuchus (Deinosuchus), Crocodylian relative: A giant crocodile up to 10 m long that ambushed dinosaurs at the water's edge. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `tyrannosaurus` ティラノサウルス（*Tyrannosaurus rex*）

図鑑 `assets/artwork/tyrannosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Tyrannosaurus (Tyrannosaurus rex), Dinosaur (tyrannosaur), Cretaceous period, about 68 million years ago. The 12 m king of latest Cretaceous North America, with jaws strong enough to crush bone. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/tyrannosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Tyrannosaurus (Tyrannosaurus rex), Dinosaur (tyrannosaur): The 12 m king of latest Cretaceous North America, with jaws strong enough to crush bone. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `triceratops` トリケラトプス（*Triceratops horridus*）

図鑑 `assets/artwork/triceratops.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Triceratops (Triceratops horridus), Dinosaur (ceratopsian), Cretaceous period, about 68 million years ago. A heavy plant-eater with three horns and a huge frill. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/triceratops.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Triceratops (Triceratops horridus), Dinosaur (ceratopsian): A heavy plant-eater with three horns and a huge frill. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `ankylosaurus` アンキロサウルス（*Ankylosaurus magniventris*）

図鑑 `assets/artwork/ankylosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Ankylosaurus (Ankylosaurus magniventris), Dinosaur (ankylosaur), Cretaceous period, about 68 million years ago. A plant-eater armoured with bony plates and armed with a heavy tail club. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/ankylosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Ankylosaurus (Ankylosaurus magniventris), Dinosaur (ankylosaur): A plant-eater armoured with bony plates and armed with a heavy tail club. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `edmontosaurus` エドモントサウルス（*Edmontosaurus annectens*）

図鑑 `assets/artwork/edmontosaurus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Edmontosaurus (Edmontosaurus annectens), Dinosaur (hadrosaur), Cretaceous period, about 73 million years ago. A duck-billed plant-eater that lived in big herds. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/edmontosaurus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Edmontosaurus (Edmontosaurus annectens), Dinosaur (hadrosaur): A duck-billed plant-eater that lived in big herds. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `quetzalcoatlus` ケツァルコアトルス（*Quetzalcoatlus northropi*）

図鑑 `assets/artwork/quetzalcoatlus.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Quetzalcoatlus (Quetzalcoatlus northropi), Pterosaur (azhdarchid), Cretaceous period, about 68 million years ago. One of the largest flying animals ever, with a 10 m wingspan; on the ground it walked on all fours and ate small animals. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/quetzalcoatlus.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Quetzalcoatlus (Quetzalcoatlus northropi), Pterosaur (azhdarchid): One of the largest flying animals ever, with a 10 m wingspan; on the ground it walked on all fours and ate small animals. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `early-magnolia` 初期のモクレンの仲間（*Magnoliaceae (early)*）

図鑑 `assets/artwork/early-magnolia.jpg`：

> Use case: scientific-educational. Asset type: individual image for an Earth-history natural history field guide. Primary request: photorealistic museum-quality life reconstruction of Early magnolia relative (Magnoliaceae (early)), Flowering tree, Cretaceous period, about 100 million years ago. An early flowering tree with big white blossoms, pollinated by beetles. Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: softly blurred Cretaceous landscape with conifers, cycads, ferns and early flowering shrubs, no grass. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament. This is an interpretive reconstruction, not a fossil specimen.

スプライト `assets/units/early-magnolia.png`：

> Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: Early magnolia relative (Magnoliaceae (early)), Flowering tree: An early flowering tree with big white blossoms, pollinated by beetles. Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. Whole plant upright from ground level to its top, base at the bottom edge, nothing cropped. Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.

### `camp-eruption` 中央大西洋の大噴火（CAMP）（現象）

図鑑 `assets/artwork/camp-eruption.jpg`：

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: Central Atlantic eruptions (CAMP) — Where Pangaea began to split, one of the largest lava outpourings in history burst forth. Its gases warmed the planet and caused the end-Triassic extinction. Cinematic flood-basalt eruption: immense fissures pouring orange lava across dark basalt plains, towering ash clouds and red glow. Landscape 16:9 framing (delivered as 3:2 or 16:9). Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament.

### `laurasia` ローラシア大陸（現象）

図鑑 `assets/artwork/laurasia.jpg`：

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: Laurasia — The northern continent left when Pangaea split in two, joining today's North America, Europe and Asia. Cinematic orbital view of an ancient supercontinent/continent on a curved blue ocean planet, no modern recognizable continents, thin atmospheric rim. Landscape 16:9 framing (delivered as 3:2 or 16:9). Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament.

### `gymnosperm-forest` 恐竜時代の針葉樹とシダの森（現象）

図鑑 `assets/artwork/gymnosperm-forest.jpg`：

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: Dinosaur-age conifer and fern forest — Triassic and Jurassic forests were made of conifers, cycad-like plants, ginkgos and tree ferns. There were no flowers or grass yet; ferns carpeted the ground. Cinematic wide landscape photograph, no animals. Landscape 16:9 framing (delivered as 3:2 or 16:9). Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament.

### `flowering-forest` 花の咲く森（現象）

図鑑 `assets/artwork/flowering-forest.jpg`：

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: The first flowering forests — In the later Cretaceous, flowering trees such as magnolias joined the forests and colour came to the land. The partnership between insects and flowers began. Cinematic wide landscape photograph, no animals. Landscape 16:9 framing (delivered as 3:2 or 16:9). Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament.

### `deccan-traps` デカン・トラップの噴火（現象）

図鑑 `assets/artwork/deccan-traps.jpg`：

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: Deccan Traps eruption — Vast lava eruptions in India lasted hundreds of thousands of years, leaving step-like lava plateaus (traps) that remain today. Cinematic flood-basalt eruption: immense fissures pouring orange lava across dark basalt plains, towering ash clouds and red glow. Landscape 16:9 framing (delivered as 3:2 or 16:9). Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament.

### `chicxulub-impact` チクシュルーブの衝突（現象）

図鑑 `assets/artwork/chicxulub-impact.jpg`：

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: The Chicxulub impact — A 10 km asteroid struck what is now Mexico's Yucatán. Giant tsunamis and wildfires followed, dust blotted out the Sun, and all dinosaurs except birds died out. Cinematic flood-basalt eruption: immense fissures pouring orange lava across dark basalt plains, towering ash clouds and red glow. Landscape 16:9 framing (delivered as 3:2 or 16:9). Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. No text, labels, borders, watermarks, grass, modern mammals, modern birds or fantasy ornament.

### `backdrop-triassic-land`（遠景・任意）

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: wide cinematic empty Triassic landscape: red sandy floodplain, conifers, cycad-like plants and seed-fern trees (Dicroidium), seasonal river, warm hazy sky, no flowers or grass, generous open centre for gameplay, dark foreground edges, realistic natural history film still. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. Landscape 16:9 framing. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament.

### `backdrop-jurassic-land`（遠景・任意）

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: wide cinematic empty Jurassic forest edge: tall conifers (araucarias), cycads, ginkgos and tree ferns over a fern-covered floor, river plain, no flowers or grass, generous open centre for gameplay, dark foreground edges, realistic natural history film still. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. Landscape 16:9 framing. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament.

### `backdrop-cretaceous-land`（遠景・任意）

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: wide cinematic empty Late Cretaceous floodplain forest: conifers and cycads mixed with early flowering trees such as magnolias, ferns on the ground, river, no grass, generous open centre for gameplay, dark foreground edges, realistic natural history film still. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. Landscape 16:9 framing. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament.

### `backdrop-triassic-sea`（遠景・任意）

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: wide cinematic empty Triassic warm lagoon of the Tethys: clear water, scattered ammonites shells and sponge reefs on the sea floor, open centre, generous open centre for gameplay, dark foreground edges, realistic natural history film still. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. Landscape 16:9 framing. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament.

### `backdrop-jurassic-sea`（遠景・任意）

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: wide cinematic empty Jurassic shallow sea: sponge and coral patches, crinoids and ammonite shells on a limestone floor, sunbeams, open centre, generous open centre for gameplay, dark foreground edges, realistic natural history film still. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. Landscape 16:9 framing. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament.

### `backdrop-cretaceous-sea`（遠景・任意）

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: wide cinematic empty Late Cretaceous shallow inland sea: chalky sea floor with oyster beds, sunbeams through greenish water, open centre, generous open centre for gameplay, dark foreground edges, realistic natural history film still. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. Landscape 16:9 framing. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament.

### `backdrop-triassic-deep`（遠景・任意）

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: wide cinematic empty Mesozoic deep open ocean: dark indigo water fading to black, faint light above, distant sea floor, generous open centre for gameplay, dark foreground edges, realistic natural history film still. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. Landscape 16:9 framing. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament.

### `backdrop-jurassic-deep`（遠景・任意）

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: wide cinematic empty Mesozoic deep open ocean: dark indigo water fading to black, faint light above, distant sea floor, generous open centre for gameplay, dark foreground edges, realistic natural history film still. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. Landscape 16:9 framing. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament.

### `backdrop-cretaceous-deep`（遠景・任意）

> Use case: scientific-educational. Asset type: cinematic game environment. Subject: wide cinematic empty Mesozoic deep open ocean: dark indigo water fading to black, faint light above, distant sea floor, generous open centre for gameplay, dark foreground edges, realistic natural history film still. Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette. Landscape 16:9 framing. No text, labels, borders, watermarks, flowering plants, grass, modern mammals, modern birds or fantasy ornament.
