#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    TvNotFound(String),

    #[error("{0}")]
    InternalError(String),
}
