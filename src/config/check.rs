use crate::{
    cloud::{MetricsProvider, aws::AWSProvider},
    config::{configs::Config, load::load_config},
};
use anyhow::Result;
use std::path::Path;

pub async fn verify_config(config_path: Option<&Path>) -> Result<()> {
    let config: Config = load_config(config_path)?;

    println!("Configuration loaded successfully\n");

    // AWS Connectivity
    println!("Verifying AWS Connectivity...");

    let provider = AWSProvider::new(&config.aws).await?;
    let status = provider.verify_connection().await?;

    println!("  Region: {}", status.region);

    println!("\n  Permissions:");
    println!(
        "    CloudWatch Read: {}",
        if status.permissions.metrics_monitor_read {
            "OK"
        } else {
            "FAILED"
        }
    );
    println!(
        "    EC2 Describe: {}",
        if status.permissions.instance_describe {
            "OK"
        } else {
            "FAILED"
        }
    );

    if !status.connected {
        anyhow::bail!("Could not connect to AWS. Check your credentials and permissions.");
    }

    println!("\n Configuration and AWS connectivity verified");
    Ok(())
}
