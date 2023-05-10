use crate::entity::tv;
use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use tracing::error;

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<tv::Model>),
    )
)]
#[get("/library/tv")]
pub async fn list_tv(db: web::Data<DatabaseConnection>) -> HttpResponse {
    match tv::Entity::find().all(db.get_ref()).await {
        Ok(tv) => HttpResponse::Ok().json(Json(tv)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[utoipa::path(
    post,
    context_path = "/api",
    responses(
        (status = 200, body = tv::Model),
    )
)]
#[post("/library/tv")]
pub async fn new_tv(db: web::Data<DatabaseConnection>) -> HttpResponse {
    let new_tv = tv::ActiveModel {
        title: Set("test".to_owned()),
        year: Set(2023),
        status: Set("test".to_owned()),
        tmdb_id: Set(1),
        original_language: Set("test".to_owned()),
        original_title: Set("test".to_owned()),
        overview: Set("test".to_owned()),
        ..Default::default()
    };
    match new_tv.insert(db.get_ref()).await {
        Ok(tv) => HttpResponse::Ok().json(Json(tv)),
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}
