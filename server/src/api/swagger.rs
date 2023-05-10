use utoipa::OpenApi;

use super::library;
use crate::entity::tv;

#[derive(OpenApi)]
#[openapi(
    paths(library::tv::list_tv, library::tv::new_tv),
    components(schemas(tv::Model))
)]
pub struct ApiDoc;
