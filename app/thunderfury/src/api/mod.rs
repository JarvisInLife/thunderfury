pub async fn health() -> &'static str {
    "I am working!"
}

pub fn app() -> Router {
    Router::new().route("/health", get(health))
}
