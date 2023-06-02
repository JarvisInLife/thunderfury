use serde::Serialize;
use utoipa::ToSchema;

use super::genre::Genre;

#[derive(Debug, Serialize, ToSchema)]
pub struct MovieDetail {
    pub id: i32,
    pub name: String,
    pub year: i32,
    pub status: String,
    pub release_date: String,
    pub tmdb_id: i32,
    pub overview: String,
    pub in_library: bool,
    pub genres: Vec<Genre>,
}
