mod pipeline;

use clap::{Parser, Subcommand};
use log::info;
use pipeline::scan::scan_images;

#[derive(Parser)]
#[command(name = "photosorter")]
#[command(version = "0.1")]
#[command(about = "Sort and analyse photo collections")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {
        /// Directory to scan
        path: String,
    },
}

fn main() {
    // Set default log level if user hasn't specified one
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path } => {
            let result = scan_images(&path);

            info!(
                "Found {} folders with {} images",
                result.folder_count,
                result.images.len()
            );
        }
    }
}