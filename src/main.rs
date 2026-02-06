mod cloud;
mod config;

use std::path::Path;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::cloud::{MetricsProvider, aws::AWSProvider};

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

    /// Generate default configuration
    Init {},
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = cli.config.as_ref().map(Path::new);

    match &cli.command {
        Commands::Run {} => {
            println!("Starting the infrawatch daemon");
            // TODO: implement daemon
        }
        Commands::Check {} => {
            println!("Validating cloud provider connectivity");
            let config = config::check::verify_config(config_path).await?;
            let provider = AWSProvider::new(&config.aws).await?;
            provider.verify_connection().await?;
            println!("Config valid and cloud provider connection successful");
        }
        Commands::Query {} => {
            println!("Querying stored metrics");
            // TODO: implement querying metrics
        }
        Commands::Init {} => {
            println!("Generating default configuration");
            config::load::init_config()?;
        }
    }
    Ok(())
}
