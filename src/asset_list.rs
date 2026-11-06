//! 画像（アセット）の注文書を、ゲームのデータから書き出す。
//!
//! `cargo test --lib write_asset_requests -- --ignored` で
//! `docs/asset-requests.md`（人が読む一覧）と `assets/requests.json`（ツール用）を作り直す。
//! 生きもの・現象・背景を足したら、もう一度実行すれば一覧も追いつく。
//! 画像は無くてもゲームは手描きの絵で動く。置けば次のビルドから使われる。

use crate::artwork::{find, find_unit};
use crate::earth::*;
use crate::life::*;

/// 紀ごとの遠景（任意）：(slug, 紀, 種類, 内容)
pub const BACKDROPS: &[(&str, usize, &str, &str)] = &[
    ("backdrop-ordovician-land", ORDOVICIAN, "land", "Ordovician coastline: bare ochre rock and sand, damp dark-green films of tiny liverwort-like plants only at the water's edge, shallow turquoise sea"),
    ("backdrop-silurian-land", SILURIAN, "land", "Silurian shoreline: mostly bare rock inland, a low green fringe of tiny leafless Cooksonia-like plants along streams and the shore, no trees"),
    ("backdrop-devonian-land", DEVONIAN, "land", "Late Devonian riverside with the first forests of tall Archaeopteris trees and palm-like Eospermatopteris, low green undergrowth, meandering river, no flowers, no grass"),
    ("backdrop-carboniferous-land", CARBONIFEROUS, "land", "Carboniferous coal swamp forest: towering scale trees (Lepidodendron) with diamond-patterned trunks, giant horsetails (Calamites), tree ferns, dark still water, humid haze, no flowers, no grass"),
    ("backdrop-permian-land", PERMIAN, "land", "Permian Pangaea interior: red sandstone and dunes, scattered early conifers (Walchia) and seed ferns near a dry riverbed, hot hazy sky, no flowers, no grass"),
    ("backdrop-ice-land", ORDOVICIAN, "land", "Paleozoic polar Gondwana under an ice age: vast glacier and ice sheet meeting bare rock moraines, pale cold sky, no plants, no animals (used for both the Hirnantian and Late Paleozoic ice ages)"),
    ("backdrop-ordovician-sea", ORDOVICIAN, "sea", "Ordovician shallow tropical sea: sunbeams, crinoid meadows, bryozoan and tabulate coral patches, scattered shells on the sea floor, open centre, no fish with jaws"),
    ("backdrop-silurian-sea", SILURIAN, "sea", "Silurian reef: domed stromatoporoids, honeycomb and chain corals, crinoids, warm clear water, open centre"),
    ("backdrop-devonian-sea", DEVONIAN, "sea", "Devonian reef sea: large stromatoporoid and coral reef walls, crinoids, clear blue water, open centre for gameplay"),
    ("backdrop-carboniferous-sea", CARBONIFEROUS, "sea", "Carboniferous shallow sea: crinoid forests and big brachiopod beds on a muddy limestone floor, greenish water, open centre"),
    ("backdrop-permian-sea", PERMIAN, "sea", "Permian reef (Capitan-like): sponge and algal reef slope, fusulinid-rich sands, warm water, open centre"),
    ("backdrop-ordovician-deep", ORDOVICIAN, "deep", "Paleozoic deep open ocean: dark indigo water fading to black, drifting graptolite colonies far away, faint light above, distant muddy sea floor"),
    ("backdrop-silurian-deep", SILURIAN, "deep", "Paleozoic deep open ocean: dark indigo water fading to black, faint light above, distant muddy sea floor"),
    ("backdrop-devonian-deep", DEVONIAN, "deep", "Devonian deep oxygen-poor sea: dark petrol-blue water, black shale sea floor, faint light above"),
    ("backdrop-carboniferous-deep", CARBONIFEROUS, "deep", "Paleozoic deep open ocean: dark indigo water fading to black, faint light above, distant sea floor"),
    ("backdrop-permian-deep", PERMIAN, "deep", "Paleozoic deep open ocean: dark indigo water fading to black, faint light above, distant sea floor"),
];

const STYLE: &str = "Style: photorealistic museum-quality natural history reconstruction, highly detailed tactile surfaces, natural lighting, restrained teal, indigo and amber palette.";
const NEG: &str = "No text, labels, borders, watermarks, flowering plants, grass, modern animals, dinosaurs or fantasy ornament.";

fn ma(t: f32) -> String {
    format!("{:.0}", t.round())
}

fn setting(sp: &Species) -> &'static str {
    if sp.hab & LAND != 0 && sp.hab & (SHALLOW | SHELF | DEEP) == 0 {
        if sp.t0 > 385.0 {
            "softly blurred Paleozoic land: bare rock and sand with only low early plants near water"
        } else if sp.t0 > 300.0 {
            "softly blurred Paleozoic forest floor with early trees, ferns and horsetails, no flowers or grass"
        } else {
            "softly blurred Permian landscape: reddish ground, early conifers and seed ferns, no flowers or grass"
        }
    } else if sp.hab & DEEP != 0 && sp.hab & (SHALLOW | TIDAL) == 0 {
        "softly blurred dark deep-sea water and a muddy sea floor"
    } else if sp.hab & TIDAL != 0 && sp.hab & (SHELF | DEEP) == 0 {
        "softly blurred shallow water's edge, estuary or tidal flat"
    } else {
        "softly blurred teal water and a small patch of sea floor"
    }
}

fn pose(sp: &Species) -> &'static str {
    match (sp.beh, sp.art) {
        (_, Art::Plant(_)) => "Whole plant upright from ground level to its top, base at the bottom edge, nothing cropped.",
        (_, Art::Griffinfly) => "Flying, wings spread, three-quarter side view, head pointing RIGHT.",
        (_, Art::Starfish | Art::Limulid) => "Seen from above at a low angle, lying flat on its underside, front pointing RIGHT.",
        (Beh::Sessile, _) => "Upright natural pose attached to the bottom, base at the bottom edge.",
        (Beh::Crawl | Beh::FlatCrawl, _) => "Side view walking pose, head pointing RIGHT and tail LEFT, feet at the bottom of the silhouette.",
        _ => "Side view swimming pose, head pointing RIGHT and tail LEFT.",
    }
}

/// 図鑑の画像（JPEG、横3:2）の注文
fn species_art_prompt(sp: &Species) -> String {
    format!(
        "Use case: scientific-educational. Asset type: individual image for a Paleozoic natural history field guide. Primary request: photorealistic museum-quality life reconstruction of {} ({}), {}, {} period, about {} million years ago. {} Full subject centered, entirely within frame with generous 12% margins, landscape 3:2 composition. Backdrop: {}. {} {} This is an interpretive reconstruction, not a fossil specimen.",
        sp.name.1, sp.sci, sp.group.1, PERIODS[period_at((sp.t0 + sp.t1.max(sp.t0 - 10.0)) * 0.5)].name.1, ma(sp.t0), sp.desc.1, setting(sp), STYLE, NEG
    )
}

/// 観察用のスプライト（透過PNG）の注文
fn species_unit_prompt(sp: &Species) -> String {
    format!(
        "Use case: scientific-educational. Asset type: individual transparent-background game unit sprite. Subject: {} ({}), {}: {} Primary request: richly detailed photorealistic natural-history life reconstruction as a SINGLE isolated game unit, entire specimen visible with every appendage inside the frame. {} Landscape 3:2 canvas, tightly readable silhouette, centered subject within 10% clear margins. Realistic fine surface texture, soft neutral light from upper left, natural restrained colors, museum reconstruction quality. TRANSPARENT BACKGROUND with genuine alpha channel. Absolutely no water, scenery, ground plane, cast shadow, text, border, watermark or additional organisms. This sprite will be placed onto a moving scene.",
        sp.name.1, sp.sci, sp.group.1, sp.desc.1, pose(sp)
    )
}

fn phenom_prompt(ph: &Phenom) -> String {
    let what = match ph.kind {
        PhKind::Continent(_) => "Cinematic orbital view of an ancient supercontinent/continent on a curved blue ocean planet, no modern recognizable continents, thin atmospheric rim.",
        PhKind::Eruption(_) => "Cinematic flood-basalt eruption: immense fissures pouring orange lava across dark basalt plains, towering ash clouds and red glow.",
        PhKind::Scenery(_) => "Cinematic wide landscape photograph, no animals.",
    };
    format!(
        "Use case: scientific-educational. Asset type: cinematic game environment. Subject: {} — {} {} Landscape 16:9 framing (delivered as 3:2 or 16:9). {} {}",
        ph.name.1, ph.desc.1, what, STYLE, NEG
    )
}

fn backdrop_prompt(subject: &str) -> String {
    format!(
        "Use case: scientific-educational. Asset type: cinematic game environment. Subject: wide cinematic empty {}, generous open centre for gameplay, dark foreground edges, realistic natural history film still. {} Landscape 16:9 framing. {}",
        subject, STYLE, NEG
    )
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn mark(b: bool) -> &'static str {
    if b {
        "✅"
    } else {
        "⬜"
    }
}

pub fn write() -> (String, String) {
    let mut md = String::new();
    let mut js = String::from("[\n");
    let ns = SPECIES.len();
    let (mut need_art, mut need_unit, mut have_art, mut have_unit) = (0, 0, 0, 0);
    for (i, sp) in SPECIES.iter().enumerate() {
        let (a, u) = (find(sp.slug).is_some(), find_unit(sp.slug).is_some());
        need_art += 1;
        need_unit += 1;
        have_art += a as usize;
        have_unit += u as usize;
        let _ = i;
    }
    for ph in PHENOMS {
        need_art += 1;
        have_art += find(ph.slug).is_some() as usize;
    }
    md.push_str("# 画像の注文書（古生代）\n\n");
    md.push_str("このファイルは `cargo test --lib write_asset_requests -- --ignored` でゲームのデータから作り直せます（手で書き換えない）。ツール用の同じ内容が `assets/requests.json` にあります。\n\n");
    md.push_str("## 置き方\n\n");
    md.push_str("- **図鑑の画像**：`assets/artwork/<slug>.jpg`（横3:2、960×640程度のJPEG。背景つき）。図鑑の一覧・詳細・撮影カードに使う。\n");
    md.push_str("- **観察用スプライト**：`assets/units/<slug>.png`（背景透過PNG、長い辺768px以内）。観察の場面で動かす。頭は右向き、植物は根元を下の端に。\n");
    md.push_str("- **遠景（任意）**：`assets/artwork/backdrop-*.jpg`（16:9、1536×1024程度）。観察の場面の奥に薄く敷く。無ければ共通の `shallow-sea` / `deep-sea` を使い、植物が広がった後の陸は手描きの空と山だけになる。\n");
    md.push_str("- 置いてビルドし直す（`bash build.sh` など）だけで使われます。表の登録は不要（`build.rs` がフォルダを見て自動で組み込む）。無い画像は手描きの絵で代わりに描きます。\n");
    md.push_str("- 生成元のPNGからの縮小・透過の確認は `tools/prepare_artwork.py` / `tools/prepare_units.py` で（`assets/requests.json` を読む）。\n\n");
    md.push_str(&format!("## 進み具合\n\n- 図鑑の画像：{}/{}（生きもの{}・現象{}）\n- 観察用スプライト：{}/{}\n- 遠景（任意）：{}/{}\n\n", have_art, need_art, ns, PHENOMS.len(), have_unit, need_unit, BACKDROPS.iter().filter(|b| find(b.0).is_some()).count(), BACKDROPS.len()));

    for (pi, per) in PERIODS.iter().enumerate() {
        md.push_str(&format!("## {}（{}）\n\n", per.name.0, per.name.1));
        md.push_str("| slug | 名前 | 学名 | 種類 | 観察での姿 | 図鑑 jpg | スプライト png |\n|---|---|---|---|---|---|---|\n");
        for (i, sp) in SPECIES.iter().enumerate().filter(|(i, _)| entry_period(*i) == pi) {
            let how = match sp.beh {
                Beh::Swim | Beh::Drift => "泳ぐ・漂う（横から）",
                Beh::NearFloor => "底の近くを泳ぐ（横から）",
                Beh::Crawl | Beh::FlatCrawl => "歩く（横から）",
                Beh::Burrow => "巣穴から出る",
                Beh::Sessile => if matches!(sp.art, Art::Plant(_)) { "立っている植物" } else { "海底に固着" },
            };
            let how = if sp.art == Art::Griffinfly { "飛ぶ（はねを広げて）" } else { how };
            md.push_str(&format!("| `{}` | {} | *{}* | {} | {} | {} | {} |\n", sp.slug, sp.name.0, sp.sci, sp.group.0, how, mark(find(sp.slug).is_some()), mark(find_unit(sp.slug).is_some())));
            let _ = i;
        }
        for (pj, ph) in PHENOMS.iter().enumerate().filter(|(j, _)| entry_period(ns + j) == pi) {
            let how = match ph.kind {
                PhKind::Continent(_) => "上空から見た大陸",
                PhKind::Eruption(_) => "巨大噴火",
                PhKind::Scenery(_) => "陸の景色",
            };
            md.push_str(&format!("| `{}` | {} | — | 現象 | {} | {} | 不要 |\n", ph.slug, ph.name.0, how, mark(find(ph.slug).is_some())));
            let _ = pj;
        }
        let bds: Vec<_> = BACKDROPS.iter().filter(|b| b.1 == pi).collect();
        if !bds.is_empty() {
            md.push_str("\n遠景（任意）：");
            md.push_str(&bds.iter().map(|b| format!("`{}` {}", b.0, mark(find(b.0).is_some()))).collect::<Vec<_>>().join("、"));
            md.push('\n');
        }
        md.push('\n');
    }

    md.push_str("## 生成プロンプト（例）\n\nこれまでの画像（`assets/artwork/manifest.json`・`assets/units/manifest.json`）と同じ書き方にそろえています。足りない項目だけ載せます。\n\n");
    for (i, sp) in SPECIES.iter().enumerate() {
        let (a, u) = (find(sp.slug).is_some(), find_unit(sp.slug).is_some());
        js.push_str(&format!(
            "  {{\"slug\": \"{}\", \"kind\": \"species\", \"period\": \"{}\", \"name_ja\": \"{}\", \"name_en\": \"{}\", \"sci\": \"{}\",\n   \"artwork\": {{\"path\": \"assets/artwork/{}.jpg\", \"exists\": {}, \"prompt\": \"{}\"}},\n   \"unit\": {{\"path\": \"assets/units/{}.png\", \"exists\": {}, \"prompt\": \"{}\"}}}},\n",
            sp.slug, PERIODS[entry_period(i)].name.1, esc(sp.name.0), esc(sp.name.1), esc(sp.sci), sp.slug, a, esc(&species_art_prompt(sp)), sp.slug, u, esc(&species_unit_prompt(sp))
        ));
        if a && u {
            continue;
        }
        md.push_str(&format!("### `{}` {}（*{}*）\n\n", sp.slug, sp.name.0, sp.sci));
        if !a {
            md.push_str(&format!("図鑑 `assets/artwork/{}.jpg`：\n\n> {}\n\n", sp.slug, species_art_prompt(sp)));
        }
        if !u {
            md.push_str(&format!("スプライト `assets/units/{}.png`：\n\n> {}\n\n", sp.slug, species_unit_prompt(sp)));
        }
    }
    for (j, ph) in PHENOMS.iter().enumerate() {
        let a = find(ph.slug).is_some();
        js.push_str(&format!(
            "  {{\"slug\": \"{}\", \"kind\": \"phenomenon\", \"period\": \"{}\", \"name_ja\": \"{}\", \"name_en\": \"{}\",\n   \"artwork\": {{\"path\": \"assets/artwork/{}.jpg\", \"exists\": {}, \"prompt\": \"{}\"}}}},\n",
            ph.slug, PERIODS[entry_period(ns + j)].name.1, esc(ph.name.0), esc(ph.name.1), ph.slug, a, esc(&phenom_prompt(ph))
        ));
        if !a {
            md.push_str(&format!("### `{}` {}（現象）\n\n図鑑 `assets/artwork/{}.jpg`：\n\n> {}\n\n", ph.slug, ph.name.0, ph.slug, phenom_prompt(ph)));
        }
    }
    for (k, b) in BACKDROPS.iter().enumerate() {
        let a = find(b.0).is_some();
        js.push_str(&format!(
            "  {{\"slug\": \"{}\", \"kind\": \"backdrop\", \"period\": \"{}\", \"scene\": \"{}\",\n   \"artwork\": {{\"path\": \"assets/artwork/{}.jpg\", \"exists\": {}, \"prompt\": \"{}\"}}}}{}\n",
            b.0, PERIODS[b.1].name.1, b.2, b.0, a, esc(&backdrop_prompt(b.3)), if k + 1 < BACKDROPS.len() { "," } else { "" }
        ));
        if !a {
            md.push_str(&format!("### `{}`（遠景・任意）\n\n> {}\n\n", b.0, backdrop_prompt(b.3)));
        }
    }
    js.push_str("]\n");
    (format!("{}\n", md.trim_end()), js)
}

#[cfg(test)]
mod tests {
    /// Every Paleozoic entry and period backdrop has its requested image.
    #[test]
    fn paleozoic_image_requests_are_complete() {
        for sp in crate::life::SPECIES {
            assert!(crate::artwork::find(sp.slug).is_some(), "missing guide image: {}", sp.slug);
            assert!(crate::artwork::find_unit(sp.slug).is_some(), "missing unit sprite: {}", sp.slug);
        }
        for ph in crate::life::PHENOMS {
            assert!(crate::artwork::find(ph.slug).is_some(), "missing phenomenon: {}", ph.slug);
        }
        for backdrop in super::BACKDROPS {
            assert!(crate::artwork::find(backdrop.0).is_some(), "missing backdrop: {}", backdrop.0);
        }
    }

    #[test]
    #[ignore]
    fn write_asset_requests() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let (md, js) = super::write();
        std::fs::create_dir_all(root.join("docs")).unwrap();
        std::fs::write(root.join("docs/asset-requests.md"), md).unwrap();
        std::fs::write(root.join("assets/requests.json"), js).unwrap();
    }

    /// 注文書は JSON として読めて、名前が重ならない
    #[test]
    fn requests_are_well_formed() {
        let (_, js) = super::write();
        assert!(js.starts_with("[\n") && js.trim_end().ends_with(']'));
        assert!(!js.contains(",\n]"));
        let mut slugs: Vec<&str> = super::BACKDROPS.iter().map(|b| b.0).collect();
        slugs.extend(crate::life::SPECIES.iter().map(|s| s.slug));
        slugs.extend(crate::life::PHENOMS.iter().map(|p| p.slug));
        let n = slugs.len();
        slugs.sort();
        slugs.dedup();
        assert_eq!(slugs.len(), n);
    }
}
