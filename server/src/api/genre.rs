use actix_web::{get, web};
use sea_orm::EntityTrait;

use crate::{
    api::{
        error::{ok, ApiResult},
        model::genre::Genre,
    },
    common::AppState,
    entity::genre,
};

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<Genre>),
    )
)]
#[get("/genres")]
pub async fn list_genres(state: web::Data<AppState>) -> ApiResult<Vec<Genre>> {
    ok(genre::Entity::find()
        .all(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

impl Into<Genre> for genre::Model {
    fn into(self) -> Genre {
        Genre {
            id: self.id,
            name: self.name,
        }
    }
}
