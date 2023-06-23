use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

use super::{lang, EpisodeInfo, Title};

impl From<&str> for EpisodeInfo {
    fn from(filename: &str) -> Self {
        let mut info = EpisodeInfo::default();

        let mut filename = nomalize_filename(filename);
        info.parse_resolution(&mut filename);
        info.parse_special_season_number(&mut filename);

        let res = info.parse_season_and_episode_number(&filename);
        if res.is_none() {
            return info;
        }

        let (title_part, extra_part) = res.unwrap();
        info.parse_title(title_part);
        info.parse_subtitles(extra_part);

        info
    }
}

impl EpisodeInfo {
    fn parse_resolution(&mut self, filename: &mut String) {
        lazy_static! {
            static ref RESOLUTION_RE: Regex =
                Regex::new(r"(\d{3,4}x(?P<height>\d{3,4}))|(?i)(?P<resolution>\d{1,4}[pk])")
                    .unwrap();
        }

        if let Some(caps) = RESOLUTION_RE.captures(&filename) {
            if let Some(height) = caps.name("height") {
                self.resolution = Some(format!("{}p", height.as_str()));
            } else if let Some(resolution) = caps.name("resolution") {
                let mut resolution = resolution.as_str().to_lowercase();
                if resolution == "4k" {
                    resolution = "2160p".to_string();
                }
                self.resolution = Some(resolution);
            }

            filename.replace_range(caps.get(0).unwrap().range(), " ");
        }
    }

    fn parse_special_season_number(&mut self, filename: &mut String) {
        lazy_static! {
            static ref SPECIAL_SEASON_NUMBER_RE: Regex =
                Regex::new(r"第(?P<season_number>\d{1,3})季").unwrap();
        }

        if let Some(caps) = SPECIAL_SEASON_NUMBER_RE.captures(filename) {
            if let Some(season_number) = caps.name("season_number") {
                self.season_number = Some(season_number.as_str().parse().unwrap());
            }

            filename.replace_range(caps.get(0).unwrap().range(), "");
        }
    }

    fn parse_season_and_episode_number<'a>(
        &mut self,
        filename: &'a str,
    ) -> Option<(&'a str, &'a str)> {
        lazy_static! {
            static ref SEASON_AND_EPISODE_NUMBER_RE: Regex =
                Regex::new(r"(?i)(\[?S(eason)?\s*(?P<season_number>\d{1,2})\s*\]?\s*)?([\[|E]|(\-\s+))(?P<episode_number>\d{1,4})").unwrap();

            static ref SIMPLE_EPISODE_NUMBER_RE: Regex = Regex::new(r"(?P<episode_number>\d{1,4})").unwrap();
        }

        if let Some(caps) = SEASON_AND_EPISODE_NUMBER_RE.captures(filename) {
            if let Some(season_number) = caps.name("season_number") {
                self.season_number = Some(season_number.as_str().parse().unwrap());
            }
            if let Some(episode_number) = caps.name("episode_number") {
                self.episode_number = Some(episode_number.as_str().parse().unwrap());
            }

            let m = caps.get(0).unwrap();
            Some((filename[..m.start()].trim(), filename[m.end()..].trim()))
        } else {
            if let Some(caps) = SIMPLE_EPISODE_NUMBER_RE.captures(filename) {
                if let Some(episode_number) = caps.name("episode_number") {
                    self.episode_number = Some(episode_number.as_str().parse().unwrap());
                }

                let m = caps.get(0).unwrap();
                Some((filename[..m.start()].trim(), filename[m.end()..].trim()))
            } else {
                None
            }
        }
    }

    fn parse_title(&mut self, filename: &str) {
        lazy_static! {
            static ref TITLE_RE: Regex =
                Regex::new(r"(\[(?P<release_group>[^\]]+)\])?\[?(?P<title>[^\]\[]+)").unwrap();
        }

        if filename.is_empty() {
            return;
        }

        if let Some(caps) = TITLE_RE.captures(filename) {
            if let Some(release_group) = caps.name("release_group") {
                self.release_group = Some(release_group.as_str().trim().to_string());
            }
            if let Some(title) = caps.name("title") {
                let titles = Title::parse(title.as_str());
                if !titles.is_empty() {
                    self.titles = Some(titles);
                }
            }
        }
    }

    fn parse_subtitles(&mut self, filename: &str) {
        lazy_static! {
            static ref LANG_MAP: HashMap<&'static str, Vec<&'static str>> = HashMap::from([
                (lang::LANG_ZH_CN, vec!["简", "CHS"]),
                (lang::LANG_ZH_TW, vec!["繁", "CHT"]),
                (lang::LANG_JP, vec!["日"]),
            ]);
        }

        let mut subtitles: Vec<String> = Vec::new();

        LANG_MAP.iter().for_each(|(key, value)| {
            for lang in value {
                if filename.contains(lang) {
                    subtitles.push(key.to_string());
                    break;
                }
            }
        });

        if !subtitles.is_empty() {
            subtitles.sort();
            self.subtitles = Some(subtitles);
        }
    }
}

fn nomalize_filename(filename: &str) -> String {
    lazy_static! {
        static ref NORMALIZE_FILENAME_RE_LIST: Vec<Regex> = vec![
            Regex::new(r"(?i)@?\d{2,3}\s*fps").unwrap(),
            Regex::new(r"第[^\d]+季").unwrap(),
            Regex::new(r"\[(\S{1,4}年)?\S{1,2}月新番\]").unwrap(),
        ];
    }

    let mut n = filename
        .replace("【", "[")
        .replace("】", "]")
        .replace("(", "[")
        .replace(")", "]")
        .replace("_", " ")
        .replace("。", " ")
        .replace(".", " ");

    for re in NORMALIZE_FILENAME_RE_LIST.as_slice() {
        n = re.replace_all(&n, "").to_string();
    }

    n
}

#[cfg(test)]
mod test {
    use std::fs;

    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize)]
    struct TestCase {
        input: String,
        expected: EpisodeInfo,
    }

    #[test]
    fn test_parse_episode() {
        let content = fs::read_to_string(format!(
            "{}/resources/test/filename_parser/tv.yml",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();

        let cases: Vec<TestCase> = serde_yaml::from_str(&content).unwrap();

        for case in &cases {
            let episode = EpisodeInfo::from(case.input.as_str());
            assert_eq!(case.expected, episode, "input: {}", case.input);
        }
    }

    #[test]
    fn test_one() {
        println!("{:#?}", EpisodeInfo::from("[喵萌奶茶屋&LoliHouse] 与山田谈一场Lv999的恋爱 / Yamada-kun to Lv999 no Koi wo Suru - 12 [WebRip 1080p HEVC-10bit AAC][简繁日内封字幕]"));
    }
}
