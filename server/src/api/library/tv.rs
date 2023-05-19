use actix_web::{get, post, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use tracing::info;
use utoipa::ToSchema;

use crate::{api::error::ok, common::AppState};
use crate::{api::error::ApiResult, entity::tv};

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<tv::Model>),
    )
)]
#[get("/library/tv")]
pub async fn list_tv(state: web::Data<AppState>) -> ApiResult<Vec<tv::Model>> {
    ok(tv::Entity::find().all(&state.db).await?)
}

#[utoipa::path(
    post,
    context_path = "/api",
    request_body = tv::NewTvRequest,
    responses(
        (status = 200, body = tv::Model),
    )
)]
#[post("/library/tv")]
pub async fn new_tv(
    state: web::Data<AppState>,
    request: web::Json<NewTvRequest>,
) -> ApiResult<tv::Model> {
    if let Some(exist_tv) = tv::Entity::find()
        .filter(tv::Column::TmdbId.eq(request.tmdb_id))
        .one(&state.db)
        .await?
    {
        return ok(exist_tv);
    }

    let detail = state.tmdb.get_tv_detail(request.tmdb_id).await.unwrap();

    info!("{:?}", detail);

    let new_tv = tv::ActiveModel {
        title: Set(detail.name),
        year: Set(2023),
        status: Set(detail.status),
        tmdb_id: Set(detail.id),
        original_language: Set(detail.original_language),
        original_title: Set(detail.original_name),
        overview: Set(detail.overview),
        ..Default::default()
    };
    ok(new_tv.insert(&state.db).await?)
}

#[derive(Deserialize, ToSchema)]
#[schema(as = tv::NewTvRequest)]
pub struct NewTvRequest {
    tmdb_id: i32,
}
