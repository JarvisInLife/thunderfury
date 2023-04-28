use clap::{Args, Parser, Subcommand};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

// mod api;
mod config;
mod entity;
mod logger;
mod migration;
mod server;

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
    #[clap(hide = true)]
    Migrate(DataDirArgs),
}

#[derive(Args)]
struct DataDirArgs {
    /// Server data directory
    #[arg(short, long, default_value_t = String::from("./data"))]
    data_dir: String,
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

#[tokio::main]
async fn main() {
    logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Server(args) => {
            let db = init_db(args.data_dir.as_str()).await;
            migration::up(&db).await.unwrap();
            server::run(db).await;
        }
        Commands::Migrate(args) => {
            let db = init_db(args.data_dir.as_str()).await;
            migration::fresh(&db).await.unwrap();
        }
    }
}
