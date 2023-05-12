mod error;

use crate::third_party::tmdb;
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub db: DatabaseConnection,
    pub tmdb: tmdb::Client,
}
