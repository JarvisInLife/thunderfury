use utoipa::OpenApi;

use super::media;

#[derive(OpenApi)]
#[openapi(paths(media::list))]
pub struct ApiDoc;
