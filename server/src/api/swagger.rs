use utoipa::OpenApi;

use super::{genre, library, model};

#[derive(OpenApi)]
#[openapi(
    paths(
        library::tv::list_tvs,
        library::tv::new_tv,
        library::movie::list_movies,
        genre::list_genres
    ),
    components(schemas(
        model::tv::TvDetail,
        model::tv::NewTvRequest,
        model::movie::MovieDetail,
        model::genre::Genre
    ))
)]
pub struct ApiDoc;
