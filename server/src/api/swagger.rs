use utoipa::OpenApi;

use super::{genre, library, model, subscription};

#[derive(OpenApi)]
#[openapi(
    paths(
        library::tv::list_tvs,
        library::movie::list_movies,
        genre::list_genres,
        subscription::list_subscriptions,
        subscription::new_subscription_from_mikan_rss
    ),
    components(schemas(
        model::tv::TvDetail,
        model::tv::NewTvRequest,
        model::movie::MovieDetail,
        model::genre::Genre,
        model::subscription::SubscriptionDetail,
        model::subscription::NewSubscriptionFromMikanRssRequest
    ))
)]
pub struct ApiDoc;
