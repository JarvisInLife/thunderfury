use serde::Deserialize;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Unauthorized(String),

    #[error("{self}")]
    NotFound,

    #[error("{0}")]
    Other(String),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Other(err.to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct TvDetail {
    pub id: i32,
    pub name: String,
    pub status: String,
    pub adult: bool,
    pub first_air_date: String,
    pub in_production: bool,
    pub languages: Vec<String>,
    pub last_air_date: String,
    pub number_of_episodes: u32,
    pub number_of_seasons: u32,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: String,
    pub backdrop_path: String,
    pub vote_average: f64,
    pub vote_count: i32,
    pub created_by: Vec<String>,
}
