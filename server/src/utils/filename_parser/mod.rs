use serde::Deserialize;

mod episode;

#[derive(Debug, Default, PartialEq, Deserialize)]
pub struct EpisodeInfo {
    pub title: Option<String>,
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
