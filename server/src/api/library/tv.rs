use crate::common::AppState;
use crate::entity::tv;
use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse,
};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::Deserialize;
use tracing::{error, info};
use utoipa::ToSchema;

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<tv::Model>),
    )
)]
#[get("/library/tv")]
pub async fn list_tv(state: web::Data<AppState>) -> HttpResponse {
    match tv::Entity::find().all(&state.db).await {
        Ok(tv) => HttpResponse::Ok().json(Json(tv)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
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
pub async fn new_tv(state: web::Data<AppState>, request: web::Json<NewTvRequest>) -> HttpResponse {
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
    match new_tv.insert(&state.db).await {
        Ok(tv) => HttpResponse::Ok().json(Json(tv)),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}

#[derive(Deserialize, ToSchema)]
#[schema(as = tv::NewTvRequest)]
pub struct NewTvRequest {
    tmdb_id: i32,
}
