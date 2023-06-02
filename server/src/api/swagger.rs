use utoipa::OpenApi;

use super::library;
use super::model;

#[derive(OpenApi)]
#[openapi(
    paths(library::tv::list_tv, library::tv::new_tv),
    components(schemas(
        model::tv::TvDetail,
        model::tv::NewTvRequest,
        model::movie::MovieDetail,
        model::genre::Genre
    ))
)]
pub struct ApiDoc;
