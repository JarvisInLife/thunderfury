use std::{env, net::SocketAddr, str::FromStr};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Html,
    routing::get,
    Router, Server,
};
use sea_orm::{Database, DatabaseConnection};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DATABASE_URL: &str = "sqlite:./data/db/thunderfury.db?mode=rwc";
const RUST_LOG_KEY: &str = "RUST_LOG";

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    // env::set_var(
    //     RUST_LOG_KEY,
    //     env::var(RUST_LOG_KEY).unwrap_or("INFO".to_string()),
    // );

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let conn = Database::connect(DATABASE_URL)
        .await
        .expect("database connection failed");

    let state = AppState { conn };

    let app = Router::new()
        .route("/:id", get(edit_post))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::from_str("0.0.0.0:3000").unwrap();
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn edit_post(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    Ok(Html(format!("hello {}", id)))
}
