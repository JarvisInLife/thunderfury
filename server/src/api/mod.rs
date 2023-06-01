use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod error;
mod library;
mod model;
mod swagger;

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/swagger.json", swagger::ApiDoc::openapi()),
    )
    .service(
        web::scope("/api")
            .service(library::tv::list_tv)
            .service(library::tv::new_tv),
    );
}
