#[derive(Debug, strum::Display)]
pub enum NotFoundCode {
    TvNotFound,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid argument, {0}")]
    InvalidArgument(String),

    #[error("not found, code: {0}, message: {1}")]
    NotFound(NotFoundCode, String),

    #[error("internal error, {0}")]
    Internal(String),
}
