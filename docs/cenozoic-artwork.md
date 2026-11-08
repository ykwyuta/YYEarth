# 新生代の生成画像

内蔵ImageGenで未収録素材276点を個別に制作し、ゲーム内の名前（slug）に対応するファイルへ取り込みました。

| 用途 | 点数 | 保存先 |
| --- | ---: | --- |
| 生きものの図鑑画像 | 132 | `assets/artwork/*.jpg` |
| 生きものの透過スプライト | 132 | `assets/units/*.png` |
| 現象の図鑑画像 | 3 | `assets/artwork/*.jpg` |
| 古第三紀・新第三紀・第四紀の陸／浅海／深海遠景 | 9 | `assets/artwork/backdrop-*.jpg` |

豪州・南極の補強24項目も、図鑑と観察画面の両方に画像を用意しています。生成した体色・軟体部・景観の細部は復元上の想像を含みます。広域の観察項目は代表像であり、特定の一種が全地域に分布したことを意味しません。

プロンプトと生成元は `assets/artwork/manifest.json`、`assets/units/manifest.json` に保存しています。制作途中の記録は `output/cenozoic-generation/results/`、取り込み用の一覧は `output/cenozoic-generated.json` です。原画像を配信用サイズへ縮小し、PNGは生成時のアルファを保持しています。

`python tools/audit_cenozoic_art.py` は276点の欠落、デコード、寸法、スプライトの透過を検査し、`output/cenozoic-generation/review/` に一覧画像とJSONレポートを出力します。全時代の画像の組み込み・デコードはRustのテストでも検査します。

ウミユリは白い靄を除いた図鑑画像と透過スプライトへ差し替え、暗い背景でも輪郭を確認しました。

エリテリウムは現代のゾウに似すぎた初稿を修正し、小型の初期長鼻類として描き直しています。[原記載（Gheerbrant 2009）](https://pmc.ncbi.nlm.nih.gov/articles/PMC2705600/)は頭骨・歯の証拠によるもので、全身の姿や体色は推定です。
