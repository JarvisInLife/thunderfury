use clap::{Parser, Subcommand, Args};

// mod api;
mod entity;
mod migration;
mod server;
mod logger;
mod config;

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

#[tokio::main]
async fn main() {
    logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Server(args) => {
            println!("data dir: {}", args.data_dir);
        }
        Commands::Migrate(_) => {}
    }
}
