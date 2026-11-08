//! 読み上げ専用の読み辞書。画面の表記や保存データには適用しない。
use crate::lang::Lang;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::OnceLock;

// 「方」のような多義語は単独で置換せず、意味が決まる句で指定する。
// 専門用語の読みはブラウザとWindowsで共通に管理する。
const JA_READINGS: &str = include_str!("speech_ja.tsv");

fn readings() -> &'static HashMap<char, Vec<(&'static str, &'static str)>> {
    static INDEX: OnceLock<HashMap<char, Vec<(&str, &str)>>> = OnceLock::new();
    INDEX.get_or_init(|| {
        let mut index: HashMap<char, Vec<(&str, &str)>> = HashMap::new();
        for line in JA_READINGS
            .lines()
            .filter(|s| !s.is_empty() && !s.starts_with('#'))
        {
            let (written, reading) = line.split_once('\t').expect("読み辞書は表記と読みのTSV");
            index
                .entry(written.chars().next().expect("空の表記は禁止"))
                .or_default()
                .push((written, reading));
        }
        for entries in index.values_mut() {
            entries.sort_by_key(|(written, _)| std::cmp::Reverse(written.len()));
        }
        index
    })
}

pub fn text_for_speech(text: &str, lang: Lang) -> Cow<'_, str> {
    if lang != Lang::Ja || text.is_empty() {
        return Cow::Borrowed(text);
    }
    let mut spoken = String::new();
    let mut cursor = 0;
    let mut copied = 0;
    while cursor < text.len() {
        let rest = &text[cursor..];
        let first = rest.chars().next().unwrap();
        let matched = readings().get(&first).and_then(|entries| {
            entries
                .iter()
                .find(|(written, _)| rest.starts_with(written))
        });
        if let Some(&(written, reading)) = matched {
            spoken.push_str(&text[copied..cursor]);
            spoken.push_str(reading);
            cursor += written.len();
            copied = cursor;
        } else {
            cursor += first.len_utf8();
        }
    }
    if copied == 0 {
        Cow::Borrowed(text)
    } else {
        spoken.push_str(&text[copied..]);
        Cow::Owned(spoken)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrects_reported_readings_in_names_and_sentences() {
        let written =
            "三葉虫。三葉虫を探して、行きたい方を押そう。進みたい方を押し続けると動くよ。";
        assert_eq!(text_for_speech(written, Lang::Ja), "さんようちゅう。さんようちゅうを探して、いきたいほうを押そう。すすみたいほうを押し続けると動くよ。");
        assert!(written.starts_with("三葉虫"));
    }

    #[test]
    fn preserves_other_meanings_of_kata_and_hou() {
        let text = "先生の方へ。お話を聞きたい方。暮らし方と使い方。行きたい方向へ。進みたい方向を押そう。";
        assert_eq!(text_for_speech(text, Lang::Ja), "せんせいの方へ。お話を聞きたい方。暮らし方と使い方。行きたい方向へ。進みたい方向を押そう。");
    }

    #[test]
    fn preserves_other_languages_and_stop_requests() {
        for lang in [Lang::En, Lang::Zh] {
            let text = "三葉虫、白亜紀。Trilobite. 三叶虫。";
            assert_eq!(text_for_speech(text, lang), text);
        }
        assert_eq!(text_for_speech("", Lang::Ja), "");
        assert!(matches!(
            text_for_speech("こんにちは", Lang::Ja),
            Cow::Borrowed(_)
        ));
    }

    #[test]
    fn dictionary_has_unique_keys_and_stable_output() {
        let mut keys = std::collections::HashSet::new();
        for line in JA_READINGS
            .lines()
            .filter(|s| !s.is_empty() && !s.starts_with('#'))
        {
            let (written, reading) = line.split_once('\t').unwrap();
            assert!(keys.insert(written), "表記が重複: {written}");
            assert!(written.chars().count() >= 2, "一文字置換は禁止: {written}");
            assert!(
                !reading.is_empty()
                    && reading
                        .chars()
                        .all(|c| matches!(c, 'ぁ'..='ゖ' | 'ァ'..='ヶ' | 'ー')),
                "読みはかなのみ: {written}: {reading}"
            );
            assert_eq!(text_for_speech(written, Lang::Ja), reading);
            assert_eq!(text_for_speech(reading, Lang::Ja), reading);
        }
    }

    #[test]
    fn longest_match_handles_compounds_and_utf8_surroundings() {
        assert_eq!(text_for_speech("🔊「観察窓・小型哺乳形類・古第三紀・生痕化石・六放海綿」🌏", Lang::Ja),
            "🔊「かんさつまど・こがたほにゅうけいるい・こだいさんき・せいこんかせき・ろっぽうかいめん」🌏");
        assert_eq!(
            text_for_speech("更新世前期から完新世へ。5億1800万年前。", Lang::Ja),
            "こうしんせいぜんきからかんしんせいへ。5億1800まんねんまえ。"
        );
        assert!(matches!(
            text_for_speech("🌏 Hello! カタカナとひらがな", Lang::Ja),
            Cow::Borrowed(_)
        ));
    }

    #[test]
    fn context_readings_do_not_replace_ambiguous_characters_globally() {
        assert_eq!(
            text_for_speech(
                "泳ぎは上手ではなかった。大人にも歯がある。出産の最中に。",
                Lang::Ja
            ),
            "泳ぎはじょうずではなかった。おとなにも歯がある。出産のさいちゅうに。"
        );
        let ambiguous = "方、上手、最中、大人。";
        assert_eq!(text_for_speech(ambiguous, Lang::Ja), ambiguous);
    }

    #[test]
    fn catalog_and_mission_compounds_are_covered() {
        // 表示言語のグローバル設定に依存せず、日本語の実データを検査する。
        let mut texts = Vec::new();
        for s in crate::life::SPECIES {
            texts.extend([s.name.0, s.group.0, s.desc.0, s.fact.0]);
        }
        for p in crate::life::PHENOMS {
            texts.extend([p.name.0, p.desc.0]);
        }
        for m in crate::mission::MISSIONS {
            texts.extend([m.title.0, m.brief.0, m.done.0]);
        }
        for m in crate::earth::MILESTONES {
            texts.extend([m.title.0, m.text.0]);
        }
        for p in crate::earth::PERIODS {
            texts.push(p.name.0);
        }
        for s in crate::earth::STAGES {
            texts.push(s.name.0);
        }
        for s in crate::earth::SITES {
            texts.extend([s.name.0, s.now.0]);
        }
        for text in texts {
            let spoken = text_for_speech(text, Lang::Ja);
            let mut previous_kanji = false;
            for c in spoken.chars() {
                let kanji = matches!(c, '一'..='龯' | '々');
                assert!(
                    !(kanji && previous_kanji),
                    "未登録の熟語: {text} → {spoken}"
                );
                previous_kanji = kanji;
            }
            assert_eq!(
                text_for_speech(&spoken, Lang::Ja),
                spoken,
                "再変換で読みが変わる: {text}"
            );
        }
    }
}
