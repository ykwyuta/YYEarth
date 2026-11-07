//! 読み上げ専用の読み辞書。画面の表記や保存データには適用しない。
use std::borrow::Cow;
use crate::lang::Lang;

// 「方」のような多義語は単独で置換せず、意味が決まる句で指定する。
// 専門用語の読みはブラウザとWindowsで共通に管理する。
const JA_READINGS: &[(&str, &str)] = &[
    ("行きたい方を", "いきたいほうを"),
    ("進みたい方を", "すすみたいほうを"),
    ("三葉虫", "さんようちゅう"),
    ("腕足動物", "わんそくどうぶつ"),
    ("頭足類", "とうそくるい"),
    ("条鰭類", "じょうきるい"),
    ("肉鰭類", "にくきるい"),
    ("生痕", "せいこん"),
    ("地球史", "ちきゅうし"),
    ("古生代", "こせいだい"),
    ("中生代", "ちゅうせいだい"),
    ("新生代", "しんせいだい"),
    ("三畳紀", "さんじょうき"),
    ("白亜紀", "はくあき"),
    ("石炭紀", "せきたんき"),
];

pub fn text_for_speech(text: &str, lang: Lang) -> Cow<'_, str> {
    let mut spoken = Cow::Borrowed(text);
    if lang == Lang::Ja {
        for &(written, reading) in JA_READINGS {
            if spoken.contains(written) {
                spoken = Cow::Owned(spoken.replace(written, reading));
            }
        }
    }
    spoken
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrects_reported_readings_in_names_and_sentences() {
        let written = "三葉虫。三葉虫を探して、行きたい方を押そう。進みたい方を押し続けると動くよ。";
        assert_eq!(text_for_speech(written, Lang::Ja), "さんようちゅう。さんようちゅうを探して、いきたいほうを押そう。すすみたいほうを押し続けると動くよ。");
        assert!(written.starts_with("三葉虫"));
    }

    #[test]
    fn preserves_other_meanings_of_kata_and_hou() {
        let text = "先生の方へ。お話を聞きたい方。暮らし方と使い方。行きたい方向へ。進みたい方向を押そう。";
        assert_eq!(text_for_speech(text, Lang::Ja), text);
    }

    #[test]
    fn preserves_other_languages_and_stop_requests() {
        for lang in [Lang::En, Lang::Zh] {
            let text = "三葉虫、白亜紀。Trilobite. 三叶虫。";
            assert_eq!(text_for_speech(text, lang), text);
        }
        assert_eq!(text_for_speech("", Lang::Ja), "");
        assert!(matches!(text_for_speech("こんにちは", Lang::Ja), Cow::Borrowed(_)));
    }

    #[test]
    fn dictionary_has_unique_keys_and_stable_output() {
        for (i, &(written, reading)) in JA_READINGS.iter().enumerate() {
            assert!(!JA_READINGS[..i].iter().any(|&(key, _)| key == written));
            assert_eq!(text_for_speech(written, Lang::Ja), reading);
            assert_eq!(text_for_speech(reading, Lang::Ja), reading);
        }
    }
}
