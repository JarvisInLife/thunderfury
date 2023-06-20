use lazy_static::lazy_static;
use regex::Regex;

use super::EpisodeInfo;

impl From<&str> for EpisodeInfo {
    fn from(filename: &str) -> Self {
        let mut info = EpisodeInfo::default();

        let mut filename = nomalize_filename(filename);
        filename = info.parse_resolution(filename);
        filename = info.parse_season_and_episode_number(filename);

        println!("{}", filename);

        info
    }
}

impl EpisodeInfo {
    fn parse_season_and_episode_number(&mut self, filename: String) -> String {
        lazy_static! {
            static ref SEASON_AND_EPISODE_NUMBER_RE: Regex =
                Regex::new(r"(?i)(\[?S(eason)?\s*(?P<season_number>\d{1,2})\s*\]?\s*)?[\[|E]?\s*(?P<episode_number>\d{1,4})[\]|\s]?").unwrap();
        }

        if let Some(caps) = SEASON_AND_EPISODE_NUMBER_RE.captures(&filename) {
            if let Some(season_number) = caps.name("season_number") {
                self.season_number = Some(season_number.as_str().parse().unwrap());
            }
            if let Some(episode_number) = caps.name("episode_number") {
                self.episode_number = Some(episode_number.as_str().parse().unwrap());
            }

            return filename.replace(caps.get(0).unwrap().as_str(), " ");
        } else {
            return filename;
        }
    }

    fn parse_resolution(&mut self, filename: String) -> String {
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

            return filename.replace(caps.get(0).unwrap().as_str(), " ");
        } else {
            return filename;
        }
    }
}

fn nomalize_filename(filename: &str) -> String {
    lazy_static! {
        static ref NORMALIZE_FILENAME_RE_LIST: Vec<Regex> = vec![
            Regex::new(r"(?i)\d{2,3}\s*fps").unwrap(),
            Regex::new(r"\[(\S{1,4}年)?\S{1,2}月新番\]").unwrap(),
        ];
    }

    let mut n = filename
        .replace("【", "[")
        .replace("】", "]")
        .replace("(", "[")
        .replace(")", "]")
        .replace("_", " ")
        .replace(".", " ");

    for re in NORMALIZE_FILENAME_RE_LIST.as_slice() {
        n = re.replace_all(&n, "").to_string();
    }

    n
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_parse_episode() {
        let cases = vec![
            "【智械尚未危机制作】Isekai One Turn Kill Neesan 异世界一击无双姐姐 ～和姐姐一起开始异世界生活～ - 01精校进化版 (1920x1080 AVC AAC MKV)【附外挂字幕】",
            "[桜都字幕组] 因为太怕痛就全点防御力了。 第2季 / Itai No Wa Iya Nano De Bougyoryoku Ni Kyokufuri Shitai To Omoimasu. S2 [10][1080P@60FPS][简繁内封]",
            "[桜都字幕组] 因为太怕痛就全点防御力了。 第2季 / Itai No Wa Iya Nano De Bougyoryoku Ni Kyokufuri Shitai To Omoimasu. Season 2 [10][1080p][简繁内封]",
            "[爱恋字幕社&猫恋汉化组][1月新番][因为太怕痛就全点防御力了。 第二季][Bofuri S2][10][1080P][MP4][GB][简中]",
            "【个人翻译】[因为太怕痛就全点防御力了][S02E01][Web先行版][1080P][简体内封][AVC AAC][mkv]【2023年1月新番】",
            "20.mp4",
            "A_20.mp4",
            "A_4K_20.mp4",
            "A_20_4K.mp4",
            "A.B.C.20.mp4",
            "A.B.C.20.4k.mp4",
        ];
        for c in cases {
            println!("{}", c);
            println!("{:#?}", EpisodeInfo::from(c));
        }
    }
}
