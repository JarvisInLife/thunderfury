use actix_web::{post, web};

use crate::{common::AppState, job};

#[utoipa::path(post, context_path = "/api")]
#[post("/test")]
pub async fn test(state: web::Data<AppState>) -> &'static str {
    job::do_subscription(&state).await;
    ""
}
