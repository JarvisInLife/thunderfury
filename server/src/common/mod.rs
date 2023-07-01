pub mod error;

use crate::third_party::{alist, tmdb};
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub db: DatabaseConnection,
    pub tmdb: tmdb::Client,
    pub alist: alist::Client,
}
