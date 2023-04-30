use actix_web::{
    get,
    web::{self, Json},
    HttpResponse,
};
use sea_orm::{DatabaseConnection, EntityTrait};

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<String>),
    )
)]
#[get("/media")]
pub async fn list(_: web::Data<DatabaseConnection>) -> HttpResponse {
    HttpResponse::Ok().json(Json(vec!["a", "b"]))
}

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
}
