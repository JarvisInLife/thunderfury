use actix_web::{get, post, web};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{
    api::{
        error::{ok, ApiResult},
        model::subscription::{NewSubscriptionRequest, SubscriptionDetail},
    },
    common::{
        error::{Error, NotFoundCode, Result},
        AppState,
    },
    entity::{subscription, tv},
    third_party::tmdb,
};

const MEDIA_TYPE_TV: &str = "tv";

#[utoipa::path(
    get,
    context_path = "/api",
    responses(
        (status = 200, body = Vec<SubscriptionDetail>),
    )
)]
#[get("/subscriptions")]
pub async fn list_subscriptions(state: web::Data<AppState>) -> ApiResult<Vec<SubscriptionDetail>> {
    ok(subscription::Entity::find()
        .all(&state.db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

#[utoipa::path(
    post,
    context_path = "/api",
    request_body = NewSubscriptionRequest,
    responses(
        (status = 200, body = SubscriptionDetail),
    )
)]
#[post("/subscriptions")]
pub async fn new_subscription_from_mikan_rss(
    state: web::Data<AppState>,
    request: web::Json<NewSubscriptionRequest>,
) -> ApiResult<SubscriptionDetail> {
    if let Some(exist_sub) = get_tv_subscription(&state, request.tmdb_id).await? {
        return ok(exist_sub.into());
    }

    let created_tv = create_tv(&state, request.tmdb_id).await?;

    let new_sub = subscription::ActiveModel {
        media_type: Set(MEDIA_TYPE_TV.to_string()),
        media_id: Set(created_tv.id),
        status: Set("".to_string()),
        resource_provider: Set(request.resource_provider.to_owned()),
        resource_url: Set(request.resource_url.to_owned()),
        ..Default::default()
    };

    ok(new_sub.insert(&state.db).await?.into())
}

impl Into<SubscriptionDetail> for subscription::Model {
    fn into(self) -> SubscriptionDetail {
        SubscriptionDetail {
            id: self.id,
            media_type: self.media_type,
            media_id: self.media_id,
            status: self.status,
            resource_provider: self.resource_provider,
            resource_url: self.resource_url,
        }
    }
}

async fn get_tv_subscription(
    state: &AppState,
    tmdb_id: i32,
) -> Result<Option<subscription::Model>> {
    if let Some(existing_tv) = tv::Entity::find()
        .filter(tv::Column::TmdbId.eq(tmdb_id))
        .one(&state.db)
        .await?
    {
        return Ok(subscription::Entity::find()
            .filter(subscription::Column::MediaType.eq(MEDIA_TYPE_TV))
            .filter(subscription::Column::MediaId.eq(existing_tv.id))
            .one(&state.db)
            .await?);
    }

    Ok(None)
}

async fn create_tv(state: &AppState, tmdb_id: i32) -> Result<tv::Model> {
    let detail = match state.tmdb.get_tv_detail(tmdb_id).await {
        Ok(detail) => detail,
        Err(tmdb::TmdbError::NotFound) => {
            return Err(Error::NotFound(
                NotFoundCode::TvNotFound,
                format!("can not find tv {} in tmdb", tmdb_id),
            ))
        }
        Err(err) => return Err(Error::Internal(err.to_string())),
    };

    let new_tv = tv::ActiveModel {
        name: Set(detail.name),
        year: Set(2023),
        status: Set(detail.status),
        first_air_date: Set(detail.first_air_date),
        last_air_date: Set(detail.last_air_date),
        number_of_seasons: Set(detail.number_of_seasons),
        tmdb_id: Set(detail.id),
        overview: Set(detail.overview),
        ..Default::default()
    };

    Ok(new_tv.insert(&state.db).await?)
}
