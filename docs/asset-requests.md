# 画像の注文書（古生代・中生代）

このファイルは `cargo test --lib write_asset_requests -- --ignored` でゲームのデータから作り直せます（手で書き換えない）。ツール用の同じ内容が `assets/requests.json` にあります。

## 置き方

- **図鑑の画像**：`assets/artwork/<slug>.jpg`（横3:2、960×640程度のJPEG。背景つき）。図鑑の一覧・詳細・撮影カードに使う。
- **観察用スプライト**：`assets/units/<slug>.png`（背景透過PNG、長い辺768px以内）。観察の場面で動かす。頭は右向き、植物は根元を下の端に。
- **遠景（任意）**：`assets/artwork/backdrop-*.jpg`（16:9、1536×1024程度）。観察の場面の奥に薄く敷く。無ければ共通の `shallow-sea` / `deep-sea` を使い、植物が広がった後の陸は手描きの空と山だけになる。
- 置いてビルドし直す（`bash build.sh` など）だけで使われます。表の登録は不要（`build.rs` がフォルダを見て自動で組み込む）。無い画像は手描きの絵で代わりに描きます。
- 生成元のPNGからの縮小・透過の確認は `tools/prepare_artwork.py` / `tools/prepare_units.py` で（`assets/requests.json` を読む）。

## 進み具合

- 図鑑の画像：302/302（生きもの283・現象19）
- 観察用スプライト：283/283
- 遠景（任意）：25/25

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
| `thrinaxodon` | トリナクソドン | *Thrinaxodon liorhinus* | 単弓類（キノドン類） | 歩く（横から） | ✅ | ✅ |
| `pleuromeia` | プレウロメイア | *Pleuromeia* | ヒカゲノカズラ類 | 立っている植物 | ✅ | ✅ |
| `triadobatrachus` | トリアドバトラクス | *Triadobatrachus massinoti* | 両生類（カエルの祖先） | 歩く（横から） | ✅ | ✅ |
| `cynognathus` | キノグナトゥス | *Cynognathus crateronotus* | 単弓類（キノドン類） | 歩く（横から） | ✅ | ✅ |
| `mixosaurus` | ミクソサウルス | *Mixosaurus* | 魚竜 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `nothosaurus` | ノトサウルス | *Nothosaurus* | 偽竜類（海の爬虫類） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `tanystropheus` | タニストロフェウス | *Tanystropheus* | 主竜形類（首の長い爬虫類） | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `placodus` | プラコドゥス | *Placodus gigas* | 板歯類（海の爬虫類） | 底の近くを泳ぐ（横から） | ✅ | ✅ |
| `ceratites` | セラタイト | *Ceratites nodosus* | 頭足類（アンモナイト） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `eoraptor` | エオラプトル | *Eoraptor lunensis* | 恐竜（初期の竜盤類） | 歩く（横から） | ✅ | ✅ |
| `herrerasaurus` | ヘレラサウルス | *Herrerasaurus ischigualastensis* | 恐竜（初期の肉食恐竜） | 歩く（横から） | ✅ | ✅ |
| `coelophysis` | コエロフィシス | *Coelophysis bauri* | 恐竜（肉食恐竜） | 歩く（横から） | ✅ | ✅ |
| `plateosaurus` | プラテオサウルス | *Plateosaurus trossingensis* | 恐竜（竜脚形類） | 歩く（横から） | ✅ | ✅ |
| `postosuchus` | ポストスクス | *Postosuchus kirkpatricki* | 偽鰐類（ワニの仲間） | 歩く（横から） | ✅ | ✅ |
| `desmatosuchus` | デスマトスクス | *Desmatosuchus spurensis* | 偽鰐類（アエトサウルス類） | 歩く（横から） | ✅ | ✅ |
| `eudimorphodon` | エウディモルフォドン | *Eudimorphodon ranzii* | 翼竜 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `shonisaurus` | ショニサウルス | *Shonisaurus popularis* | 魚竜 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `metoposaurus` | メトポサウルス | *Metoposaurus* | 両生類（分椎類） | 歩く（横から） | ✅ | ✅ |
| `dicroidium` | ディクロイディウム | *Dicroidium* | 種子シダ類（木） | 立っている植物 | ✅ | ✅ |
| `araucarioxylon` | アラウカリオキシロン（化石の森の木） | *Araucarioxylon arizonicum* | 針葉樹 | 立っている植物 | ✅ | ✅ |
| `camp-eruption` | 中央大西洋の大噴火（CAMP） | — | 現象 | 巨大噴火 | ✅ | 不要 |
| `gymnosperm-forest` | 恐竜時代の針葉樹とシダの森 | — | 現象 | 陸の景色 | ✅ | 不要 |

遠景（任意）：`backdrop-triassic-land` ✅、`backdrop-triassic-sea` ✅、`backdrop-triassic-deep` ✅

## ジュラ紀（Jurassic）

| slug | 名前 | 学名 | 種類 | 観察での姿 | 図鑑 jpg | スプライト png |
|---|---|---|---|---|---|---|
| `megazostrodon` | メガゾストロドン | *Megazostrodon rudnerae* | 哺乳形類（初期の哺乳類） | 歩く（横から） | ✅ | ✅ |
| `dilophosaurus` | ディロフォサウルス | *Dilophosaurus wetherilli* | 恐竜（肉食恐竜） | 歩く（横から） | ✅ | ✅ |
| `ichthyosaurus` | イクチオサウルス | *Ichthyosaurus communis* | 魚竜 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `plesiosaurus` | プレシオサウルス | *Plesiosaurus dolichodeirus* | 首長竜 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `dimorphodon` | ディモルフォドン | *Dimorphodon macronyx* | 翼竜 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `scelidosaurus` | スケリドサウルス | *Scelidosaurus harrisonii* | 恐竜（初期の装盾類） | 歩く（横から） | ✅ | ✅ |
| `stenopterygius` | ステノプテリギウス | *Stenopterygius quadriscissus* | 魚竜 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `dapedium` | ダペディウム | *Dapedium* | 条鰭類（硬骨魚） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `passaloteuthis` | パッサロテウティス（ベレムナイト） | *Passaloteuthis* | 頭足類（ベレムナイト） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `dactylioceras` | ダクチリオセラス | *Dactylioceras commune* | 頭足類（アンモナイト） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `liopleurodon` | リオプレウロドン | *Liopleurodon ferox* | 首長竜（首の短いプリオサウルス類） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `leedsichthys` | リードシクティス | *Leedsichthys problematicus* | 条鰭類（硬骨魚） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `anchiornis` | アンキオルニス | *Anchiornis huxleyi* | 恐竜（羽毛のある小型の肉食恐竜） | 歩く（横から） | ✅ | ✅ |
| `castorocauda` | カストロカウダ | *Castorocauda lutrasimilis* | 哺乳形類（泳ぐ哺乳類） | 歩く（横から） | ✅ | ✅ |
| `allosaurus` | アロサウルス | *Allosaurus fragilis* | 恐竜（肉食恐竜） | 歩く（横から） | ✅ | ✅ |
| `stegosaurus` | ステゴサウルス | *Stegosaurus stenops* | 恐竜（剣竜） | 歩く（横から） | ✅ | ✅ |
| `diplodocus` | ディプロドクス | *Diplodocus carnegii* | 恐竜（竜脚類） | 歩く（横から） | ✅ | ✅ |
| `brachiosaurus` | ブラキオサウルス | *Brachiosaurus altithorax* | 恐竜（竜脚類） | 歩く（横から） | ✅ | ✅ |
| `archaeopteryx` | 始祖鳥（アーケオプテリクス） | *Archaeopteryx lithographica* | 鳥類（最古の鳥の一つ） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `rhamphorhynchus` | ランフォリンクス | *Rhamphorhynchus muensteri* | 翼竜 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `compsognathus` | コンプソグナトゥス | *Compsognathus longipes* | 恐竜（小型の肉食恐竜） | 歩く（横から） | ✅ | ✅ |
| `williamsonia` | ウィリアムソニア（ベネチテス類） | *Williamsonia* | ベネチテス類（ソテツに似た植物） | 立っている植物 | ✅ | ✅ |
| `ginkgo` | イチョウの仲間 | *Ginkgo* | イチョウ類 | 立っている植物 | ✅ | ✅ |
| `laurasia` | ローラシア大陸 | — | 現象 | 上空から見た大陸 | ✅ | 不要 |

遠景（任意）：`backdrop-jurassic-land` ✅、`backdrop-jurassic-sea` ✅、`backdrop-jurassic-deep` ✅

## 白亜紀（Cretaceous）

| slug | 名前 | 学名 | 種類 | 観察での姿 | 図鑑 jpg | スプライト png |
|---|---|---|---|---|---|---|
| `iguanodon` | イグアノドン | *Iguanodon bernissartensis* | 恐竜（鳥脚類） | 歩く（横から） | ✅ | ✅ |
| `sinosauropteryx` | シノサウロプテリクス | *Sinosauropteryx prima* | 恐竜（羽毛のある小型の肉食恐竜） | 歩く（横から） | ✅ | ✅ |
| `microraptor` | ミクロラプトル | *Microraptor gui* | 恐竜（4枚の翼を持つ小型恐竜） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `psittacosaurus` | プシッタコサウルス | *Psittacosaurus* | 恐竜（初期の角竜） | 歩く（横から） | ✅ | ✅ |
| `confuciusornis` | コンフキウソルニス（孔子鳥） | *Confuciusornis sanctus* | 鳥類 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `repenomamus` | レペノマムス | *Repenomamus giganticus* | 哺乳類 | 歩く（横から） | ✅ | ✅ |
| `archaefructus` | アルカエフルクトゥス（古果） | *Archaefructus sinensis* | 被子植物（初期の花の咲く植物） | 立っている植物 | ✅ | ✅ |
| `spinosaurus` | スピノサウルス | *Spinosaurus aegyptiacus* | 恐竜（水辺の大型肉食恐竜） | 歩く（横から） | ✅ | ✅ |
| `argentinosaurus` | アルゼンチノサウルス | *Argentinosaurus huinculensis* | 恐竜（竜脚類） | 歩く（横から） | ✅ | ✅ |
| `giganotosaurus` | ギガノトサウルス | *Giganotosaurus carolinii* | 恐竜（大型の肉食恐竜） | 歩く（横から） | ✅ | ✅ |
| `elasmosaurus` | エラスモサウルス | *Elasmosaurus platyurus* | 首長竜 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `tylosaurus` | ティロサウルス | *Tylosaurus proriger* | モササウルス類（海のトカゲ） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `pteranodon` | プテラノドン | *Pteranodon longiceps* | 翼竜 | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `xiphactinus` | シファクティヌス | *Xiphactinus audax* | 条鰭類（硬骨魚） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `archelon` | アーケロン | *Archelon ischyros* | カメ（ウミガメ） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `cretoxyrhina` | クレトキシリナ | *Cretoxyrhina mantelli* | 軟骨魚類（サメ） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `baculites` | バキュリテス | *Baculites* | 頭足類（まっすぐなアンモナイト） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `mosasaurus` | モササウルス | *Mosasaurus hoffmannii* | モササウルス類（海のトカゲ） | 泳ぐ・漂う（横から） | ✅ | ✅ |
| `velociraptor` | ヴェロキラプトル | *Velociraptor mongoliensis* | 恐竜（羽毛のある小型の肉食恐竜） | 歩く（横から） | ✅ | ✅ |
| `protoceratops` | プロトケラトプス | *Protoceratops andrewsi* | 恐竜（角竜） | 歩く（横から） | ✅ | ✅ |
| `parasaurolophus` | パラサウロロフス | *Parasaurolophus walkeri* | 恐竜（鳥脚類・ハドロサウルス類） | 歩く（横から） | ✅ | ✅ |
| `deinosuchus` | デイノスクス | *Deinosuchus* | ワニの仲間 | 歩く（横から） | ✅ | ✅ |
| `tyrannosaurus` | ティラノサウルス | *Tyrannosaurus rex* | 恐竜（大型の肉食恐竜） | 歩く（横から） | ✅ | ✅ |
| `triceratops` | トリケラトプス | *Triceratops horridus* | 恐竜（角竜） | 歩く（横から） | ✅ | ✅ |
| `ankylosaurus` | アンキロサウルス | *Ankylosaurus magniventris* | 恐竜（鎧竜） | 歩く（横から） | ✅ | ✅ |
| `edmontosaurus` | エドモントサウルス | *Edmontosaurus annectens* | 恐竜（鳥脚類・ハドロサウルス類） | 歩く（横から） | ✅ | ✅ |
| `quetzalcoatlus` | ケツァルコアトルス | *Quetzalcoatlus northropi* | 翼竜（アズダルコ類） | 歩く（横から） | ✅ | ✅ |
| `early-magnolia` | 初期のモクレンの仲間 | *Magnoliaceae (early)* | 被子植物（花の咲く木） | 立っている植物 | ✅ | ✅ |
| `flowering-forest` | 花の咲く森 | — | 現象 | 陸の景色 | ✅ | 不要 |
| `deccan-traps` | デカン・トラップの噴火 | — | 現象 | 巨大噴火 | ✅ | 不要 |
| `chicxulub-impact` | チクシュルーブの衝突 | — | 現象 | 巨大噴火 | ✅ | 不要 |

遠景（任意）：`backdrop-cretaceous-land` ✅、`backdrop-cretaceous-sea` ✅、`backdrop-cretaceous-deep` ✅

## 生成プロンプト（例）

これまでの画像（`assets/artwork/manifest.json`・`assets/units/manifest.json`）と同じ書き方にそろえています。足りない項目だけ載せます。
