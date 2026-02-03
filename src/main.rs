mod config;

use std::path::Path;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version,
    about = "Cloud infrastructure anomaly detection platform",
    author,
    propagate_version = true,
    arg_required_else_help = true
)]
struct Cli {
    /// Path to the configuration file
    #[arg(short, long, global = true, value_name = "FILE")]
    config: Option<String>,

    /// Increase logging verbosity
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the daemon (poll metrics, detect anomalies, alerts)
    Run {},
    /// Validate config and cloud provider connectivity
    Check {},
    /// Query stored metrics (for debugging)
    Query {},
}

fn main() {
    let cli = Cli::parse();

    let config_path = cli.config.as_ref().map(Path::new);

    match &cli.command {
        Commands::Run {} => {
            println!("Starting the infrawatch daemon");
            // TODO: implement daemon
        }
        Commands::Check {} => {
            println!("Validating cloud provider connectivity");
            match config::check::verify_config(config_path) {
                Ok(_) => println!("Config valid"),
                Err(e) => {
                    eprintln!("Config check failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Query {} => {
            println!("Querying stored metrics");
            // TODO: implement querying metrics
        }
    }
}
