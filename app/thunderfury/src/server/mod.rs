use std::{net::SocketAddr, str::FromStr};

use actix_web::{middleware, web, App, HttpServer};
use sea_orm::DatabaseConnection;
use tracing::info;

use crate::api;

pub async fn run(db: DatabaseConnection) -> std::io::Result<()> {
    let addr = SocketAddr::from_str("0.0.0.0:3000").unwrap();
    info!("server starting on {}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(middleware::Logger::default())
            .service(web::resource("/health").to(|| async { "I am working!" }))
            .configure(api::api)
    })
    .bind(addr)?
    .run()
    .await
}
