use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct TvDetail {
    pub id: i32,
    pub title: String,
    pub year: i32,
    pub status: String,
    pub tmdb_id: i32,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub in_library: bool,
}
