use std::{net::SocketAddr, str::FromStr};

use axum::{
    Router, Server, Extension,
};
use sea_orm::{Database, DatabaseConnection};
use tower_http::trace::TraceLayer;
use tracing::{info};

const DATABASE_URL: &str = "sqlite:./data/db/thunderfury.db?mode=rwc";

async fn init_db() -> DatabaseConnection {
    let conn = Database::connect(DATABASE_URL)
        .await
        .expect("database connection failed");

    conn
}

pub async fn run(db: DatabaseConnection) {
    let app = Router::new()
        // .nest("/api", api::app())
        .layer(TraceLayer::new_for_http())
        .layer(Extension(db));

    let addr = SocketAddr::from_str("0.0.0.0:3000").unwrap();
    info!("server starting on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
