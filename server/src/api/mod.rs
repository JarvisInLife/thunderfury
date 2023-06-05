use actix_web::web::{self};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod error;
mod genre;
mod library;
mod model;
mod subscription;
mod swagger;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/swagger.json", swagger::ApiDoc::openapi()),
    )
    .service(
        web::scope("/api")
            .service(library::tv::list_tvs)
            .service(library::movie::list_movies)
            .service(genre::list_genres)
            .service(subscription::list_subscriptions)
            .service(subscription::new_subscription_from_mikan_rss),
    );
}
