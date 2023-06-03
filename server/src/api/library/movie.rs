use actix_web::{get, web};
use sea_orm::EntityTrait;

use crate::{
    api::{
        error::{ok, ApiResult},
        model::movie::MovieDetail,
    },
    common::AppState,
    entity::movie,
};

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<MovieDetail>),
    )
)]
#[get("/library/movies")]
pub async fn list_movies(state: web::Data<AppState>) -> ApiResult<Vec<MovieDetail>> {
    ok(movie::Entity::find()
        .all(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

impl Into<MovieDetail> for movie::Model {
    fn into(self) -> MovieDetail {
        MovieDetail {
            id: self.id,
            name: self.name,
            year: self.year,
            status: self.status,
            release_date: self.release_data,
            tmdb_id: self.tmdb_id,
            overview: self.overview,
            in_library: true,
            genres: vec![],
        }
    }
}
