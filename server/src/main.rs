use std::{net::SocketAddr, str::FromStr};

use actix_web::{middleware, web, App, HttpServer};
use clap::{Args, Parser, Subcommand};
use config::{Config, File};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::info;

mod api;
mod common;
mod entity;
mod job;
mod logger;
mod migration;
mod service;
mod third_party;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start thunderfury server
    Server(DataDirArgs),

    /// Apply database changes, used for develop only
    #[command(hide = true)]
    Migrate(DataDirArgs),
}

#[derive(Args)]
struct DataDirArgs {
    /// Server data directory
    #[arg(short, long, default_value_t = String::from("./data"))]
    data_dir: String,
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}

async fn init_db(data_dir: &str) -> DatabaseConnection {
    if data_dir.trim().is_empty() {
        panic!("data dir is empty");
    }

    let db_dir = format!("{}/db", data_dir);

    // mkdir if db_dir not exists
    if !std::path::Path::new(&db_dir).exists() {
        std::fs::create_dir_all(&db_dir).unwrap();
    }

    let url = format!("sqlite:{}/thunderfury.db?mode=rwc", db_dir);

    let mut opt = ConnectOptions::new(url);
    opt.sqlx_logging(false);

    Database::connect(opt)
        .await
        .expect("database connection failed")
}

#[actix_web::main]
async fn main() {
    logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Server(args) => {
            let db = init_db(&args.data_dir).await;
            migration::up(&db).await.unwrap();
            run(db, &args.data_dir).await.unwrap();
        }
        Commands::Migrate(args) => {
            let db = init_db(args.data_dir.as_str()).await;
            migration::fresh(&db).await.unwrap();
        }
    }
}

async fn run(db: DatabaseConnection, data_dir: &str) -> std::io::Result<()> {
    let settings = Config::builder()
        .add_source(File::with_name(&format!("{}/config.toml", data_dir)))
        .build()
        .unwrap();

    let state = web::Data::new(common::AppState {
        db,
        tmdb: third_party::tmdb::Client::new(settings.get_string("tmdb_api_key").unwrap()),
        alist: third_party::alist::Client::new(
            settings.get_string("alist_host").unwrap(),
            settings.get_string("alist_token").unwrap(),
        ),
    });

    let addr = SocketAddr::from_str("0.0.0.0:3000").unwrap();
    info!("server starting on {}", addr);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/health").to(|| async { "I am working!" }))
            .configure(api::api)
    })
    .bind(addr)?
    .run()
    .await
}
