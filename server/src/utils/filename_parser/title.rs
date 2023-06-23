use serde::Deserialize;

use lazy_static::lazy_static;
use lingua::Language::{Chinese, English, Japanese};
use lingua::{LanguageDetector, LanguageDetectorBuilder};
use unicode_segmentation::UnicodeSegmentation;

use super::lang::{self};

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct Title {
    pub language: String,
    pub title: String,
}

lazy_static! {
    static ref LANG_DETECTOR: LanguageDetector = {
        let languages = vec![English, Chinese, Japanese];
        LanguageDetectorBuilder::from_languages(&languages).build()
    };
}

impl Title {
    pub fn parse(s: &str) -> Vec<Self> {
        s.split("/")
            .flat_map(Self::split_titles_by_language)
            .collect()
    }

    fn split_titles_by_language(s: &str) -> Vec<Self> {
        let results = LANG_DETECTOR.detect_multiple_languages_of(s);

        match results.len() {
            0 => vec![Self {
                language: lang::LANG_UNKNOWN.to_string(),
                title: s.trim().to_string(),
            }],
            1 => vec![Self::normalized_title(lang::from(results[0].language()), s)],
            _ => {
                let first_result = &results[0];
                vec![
                    Self::normalized_title(
                        lang::from(first_result.language()),
                        &s[..first_result.end_index()],
                    ),
                    Self::normalized_title(
                        lang::from(results[1].language()),
                        &s[first_result.end_index()..],
                    ),
                ]
            }
        }
    }

    fn normalized_title(language: lang::LANG, title: &str) -> Self {
        let words = title.unicode_words().collect::<Vec<&str>>();
        let t = match language {
            lang::LANG_JP | lang::LANG_ZH => words.join(""),
            _ => words.join(" "),
        };
        Self {
            language: language.to_string(),
            title: t,
        }
    }
}
