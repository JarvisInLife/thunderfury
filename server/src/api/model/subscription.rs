use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SubscriptionDetail {
    pub id: i32,
    pub media_type: String,
    pub media_id: i32,
    pub status: String,
    pub rss_url: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct NewSubscriptionFromMikanRssRequest {
    pub tmdb_id: i32,
    pub rss_url: String,
}
