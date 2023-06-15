mod episode;

#[derive(Debug, Default)]
pub struct EpisodeInfo {
    pub name: Option<String>,
    pub detail: Option<String>,
    pub release_group: Option<String>,
    pub season_number: Option<i32>,
    pub episode_number: Option<i32>,
    pub resolution: Option<String>,
    pub video_format: Option<String>,
    pub audio_format: Option<String>,
    pub container_format: Option<String>,
    pub subtitles: Option<String>,
}
