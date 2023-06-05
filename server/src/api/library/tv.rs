use actix_web::{get, web};
use sea_orm::EntityTrait;

use crate::{
    api::{
        error::{ok, ApiResult},
        model::tv::TvDetail,
    },
    common::AppState,
    entity::tv,
};

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<TvDetail>),
    )
)]
#[get("/library/tvs")]
pub async fn list_tvs(state: web::Data<AppState>) -> ApiResult<Vec<TvDetail>> {
    ok(tv::Entity::find()
        .all(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

impl Into<TvDetail> for tv::Model {
    fn into(self) -> TvDetail {
        TvDetail {
            id: self.id,
            name: self.name,
            year: self.year,
            status: self.status,
            first_air_date: self.first_air_date,
            last_air_date: self.last_air_date,
            number_of_seasons: self.number_of_seasons,
            overview: self.overview,
            in_library: true,
            genres: vec![],
            tmdb_id: self.tmdb_id,
        }
    }
}
