use actix_web::{
    body::BoxBody, error::JsonPayloadError, http::StatusCode, web::Json, HttpResponse,
    HttpResponseBuilder, ResponseError,
};
use sea_orm::DbErr;
use serde::Serialize;

use crate::common::error::Error;

pub type ApiResult<T> = Result<Json<T>, Error>;

pub fn ok<T>(t: T) -> ApiResult<T> {
    ApiResult::Ok(Json(t))
}

impl Error {
    #[inline]
    fn code(&self) -> String {
        match self {
            // Self::InvalidArgument(_) => "InvalidArgument".to_string(),
            Self::NotFound(c, _) => c.to_string(),
            _ => "Internal".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct ApiError {
    code: String,
    message: String,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let e = ApiError {
            code: self.code(),
            message: self.to_string(),
        };
        match serde_json::to_string(&e) {
            Ok(body) => match HttpResponseBuilder::new(self.status_code())
                .content_type("application/json")
                .message_body(BoxBody::new(body))
            {
                Ok(res) => res,
                Err(err) => HttpResponse::from_error(err),
            },

            Err(err) => HttpResponse::from_error(JsonPayloadError::Serialize(err)),
        }
    }
}

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal(err.to_string())
    }
}
