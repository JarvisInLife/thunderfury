use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct Genre {
    pub id: i32,
    pub name: String,
}
