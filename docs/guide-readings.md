# 図鑑の読みもの

日本語版の全449項目を、やさしい紹介から、しくみ・環境・証拠へ読み進められる構成にした。専門用語は「意味 → 用語」の順でも説明し、年齢や学年で読む内容を制限しない。個別の紹介は `src/life.rs`、追加の読みものは `src/guide.rs` にある。

生物・生痕427項目では、個別の紹介と豆知識に、分類群の共通解説、掲載巻の時代背景、化石を解釈する方法を加える。一部には個別の注目点もある。共通解説は個別種の未確認の生態を断定するためのものではない。大陸や噴火など22項目は、各現象の専用解説と、証拠の比べ方を収録する。全項目の日本語本文は600字以上。文字数には項目ごとの既存紹介も含み、表示用の基本データは含めない。

「大きいから強い」「魚から陸への一直線の進化」「花とともに虫の受粉が初めて始まった」などの単純化を避ける。観察・比較からの推論と直接の証拠を区別し、復元の不確実さと、その確かめ方も説明する。ストロマトライトの35億年前からの酸素供給を断定する旧文と、白亜紀に虫の受粉が始まるという旧文は、日本語・英語・中国語で修正した。

## 読むための表示

見出しごとに本文を計測し、名前・ページ操作・レーダーの領域を除いた高さに収まるよう自動で分割する。UTF-8の文字境界で区切り、可能な範囲で文末を優先する。ページをつなぐと元の全文に戻り、画面に収まらない豆知識を省く処理はない。計測結果は項目・言語・領域サイズごとにキャッシュする。画面サイズや言語を変えた場合はページを作り直し、先頭に戻る。

読み上げは現在のページの内容と一致させる。未撮影の項目では本文を表示・読み上げしない。日本語の追加文を `ja(...)` で明示し、`tools/audit_readings.py --check` とRustのテストで読み辞書を検査する。英語・中国語は既存の翻訳を維持し、追加の日本語本文は混在させない。

## 内容の確認資料

年代の境界はゲームの既存データを使う。以下の資料の古い年代値を、そのまま境界値として取り込んではいない。

- [Smithsonian Ocean: Cambrian Period](https://ocean.si.edu/through-time/ancient-seas/cambrian-period-542-488-million-years-ago) — カンブリア紀の生物と環境の概説。
- [NPS: What is a Fossil?](https://www.nps.gov/subjects/fossils/what-is-a-fossil.htm) — 体の化石と活動の記録の区別。
- [NPS: Impressions and Compressions](https://www.nps.gov/articles/000/impressions-and-compressions.htm) — 植物の化石と保存。
- [NPS: Florissant Climate Change](https://www.nps.gov/flfo/planyourvisit/upload/No11_climate_change_508_2023-0626.pdf) — 葉、年輪、現生植物など複数の資料から気候を考える方法。
- [Natural History Museum: What are dinosaurs?](https://www.nhm.ac.uk/discover/what-are-dinosaurs.html) — 鳥を含む恐竜の分類と、翼竜などとの区別。
- [AMNH: Pterosaurs — Unearthing Pterosaurs](https://www.amnh.org/content/download/342035/5312606/file/unearthing-pterosaurs.pdf) — 翼竜の化石と復元研究。
- [AMNH: Apex predator of the Cambrian likely sought soft over crunchy prey](https://www.amnh.org/content/download/415578/6019610/file/apex-predator-soft-prey) — アノマロカリスの食性をめぐる2023年の研究紹介。
- [Engelman 2023: A Devonian Fish Tale](https://doi.org/10.3390/d15030318) — ダンクルオステウスの全長推定と体の割合。
- [Engelman 2024: Reconstructing Dunkleosteus terrelli](https://www.palaeo-electronica.org/content/2024/5307-dunkleosteus-reconstruction) — 部分的な骨格からの復元と推定の制約。
- [Australian Museum: Insects in our gardens](https://australian.museum/learn/teachers/learning/bugwise/insects-in-our-gardens-stage-3/) — 被子植物以前の昆虫による受粉。
- [USGS: Subsurface Magma Triggers Earth's Most Severe Extinction](https://www.usgs.gov/news/national-news-release/subsurface-magma-triggers-earths-most-severe-extinction) — シベリアのマグマ活動とガス放出の研究。
- [NPS: Mass Extinctions Through Geologic Time](https://www.nps.gov/subjects/fossils/mass-extinctions-through-geologic-time.htm) — 大量絶滅と生き残った生物群。
