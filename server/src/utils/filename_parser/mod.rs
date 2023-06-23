use serde::Deserialize;

mod episode;
pub mod lang;
pub mod title;

use title::Title;

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct EpisodeInfo {
    pub titles: Option<Vec<Title>>,
    pub release_group: Option<String>,
    pub season_number: Option<i32>,
    pub episode_number: Option<i32>,
    pub resolution: Option<String>,
    pub subtitles: Option<Vec<String>>,
}

impl EpisodeInfo {
    pub fn is_valid(&self) -> bool {
        self.episode_number.is_some()
    }
}
