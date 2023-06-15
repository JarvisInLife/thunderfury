use regex::Regex;

use super::EpisodeInfo;

pub fn parse_episode(filename: &str) -> EpisodeInfo {
    let mut info = EpisodeInfo::default();

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();

    info
}

fn nomalize_filename(filename: &str) -> String {
    filename
        .replace("【", "[")
        .replace("】", "]")
        .replace("(", "[")
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::parse_episode;

    #[test]
    fn test_parse_episode() {
        let cases = vec![
            "【智械尚未危机制作】Isekai One Turn Kill Neesan 异世界一击无双姐姐 ～和姐姐一起开始异世界生活～ - 01精校进化版 (1920x1080 AVC AAC MKV)【附外挂字幕】",
            "[桜都字幕组] 因为太怕痛就全点防御力了。 第2季 / Itai No Wa Iya Nano De Bougyoryoku Ni Kyokufuri Shitai To Omoimasu. S2 [10][1080P@60FPS][简繁内封]",
            "[桜都字幕组] 因为太怕痛就全点防御力了。 第2季 / Itai No Wa Iya Nano De Bougyoryoku Ni Kyokufuri Shitai To Omoimasu. Season 2 [10][1080p][简繁内封]",
            "[爱恋字幕社&猫恋汉化组][1月新番][因为太怕痛就全点防御力了。 第二季][Bofuri S2][10][1080P][MP4][GB][简中]",
            "【个人翻译】[因为太怕痛就全点防御力了][S02E01][Web先行版][1080P][简体内封][AVC AAC][mkv]【2023年1月新番】"
        ];
        for c in cases {
            println!("{:#?}", parse_episode(c));
        }
    }
}
