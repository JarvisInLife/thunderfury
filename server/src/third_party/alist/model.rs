use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub(super) struct ResponseModel<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize)]
pub struct File {
    pub name: String,
    pub size: i64,
    pub is_dir: bool,
    pub modified: String,

    #[serde(default)]
    pub raw_url: String,
}

#[derive(Debug, Serialize, Default)]
pub(super) struct ListRequest {
    pub path: String,
    pub refresh: bool,
    pub page: i32,
    pub per_page: i32,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct ListResponse {
    pub total: i32,
    pub readme: String,
    pub provider: String,
    pub write: bool,
    pub content: Vec<File>,
}

#[derive(Debug, Serialize, Default)]
pub(super) struct GetRequest {
    pub path: String,
    pub password: String,
}
