use actix_web::{get, post, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{
    api::{
        error::{ok, ApiResult},
        model::tv::{NewTvRequest, TvDetail},
    },
    common::{
        error::{Error, NotFoundCode},
        AppState,
    },
    entity::tv,
    third_party::tmdb,
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

#[utoipa::path(
    post,
    context_path = "/api",
    request_body = NewTvRequest,
    responses(
        (status = 200, body = TvDetail),
    )
)]
#[post("/library/tv")]
pub async fn new_tv(
    state: web::Data<AppState>,
    request: web::Json<NewTvRequest>,
) -> ApiResult<TvDetail> {
    if let Some(exist_tv) = tv::Entity::find()
        .filter(tv::Column::TmdbId.eq(request.tmdb_id))
        .one(&state.db)
        .await?
    {
        return ok(exist_tv.into());
    }

    let detail = match state.tmdb.get_tv_detail(request.tmdb_id).await {
        Ok(detail) => detail,
        Err(tmdb::TmdbError::NotFound) => {
            return Err(Error::NotFound(
                NotFoundCode::TvNotFound,
                format!("can not find tv {} in tmdb", request.tmdb_id),
            ))
        }
        Err(err) => return Err(Error::Internal(err.to_string())),
    };

    let new_tv = tv::ActiveModel {
        name: Set(detail.name),
        year: Set(2023),
        status: Set(detail.status),
        first_air_date: Set(detail.first_air_date),
        last_air_date: Set(detail.last_air_date),
        number_of_seasons: Set(detail.number_of_seasons),
        tmdb_id: Set(detail.id),
        overview: Set(detail.overview),
        ..Default::default()
    };

    ok(new_tv.insert(&state.db).await?.into())
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
