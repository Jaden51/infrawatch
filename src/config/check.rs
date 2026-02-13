use crate::{
    cloud::{MetricsProvider, aws::AWSProvider},
    config::{configs::Config, load::load_config},
};
use anyhow::Result;
use chrono::Utc;
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

    println!("\nFetching AWS Instance Information");

    let instances = provider.discover_instances(&[]).await?;

    for instance in &instances {
        println!("-------------");
        println!("    Id: {}", instance.instance_id);
        println!("    Type: {}", instance.instance_type);
        println!("    State: {}", instance.state);
        println!("    Name: {:?}", instance.name);
        println!("    Tags: {:?}", instance.tags);
    }

    println!("\nFetching AWS Instance Metrics");

    let instance_ids: Vec<String> = instances.iter().map(|i| i.instance_id.clone()).collect();
    let metrics = provider
        .fetch_instance_metrics(
            &instance_ids,
            &config.metrics.instance_metrics,
            Utc::now() - chrono::Duration::hours(1),
            Utc::now(),
        )
        .await?;

    for metric in &metrics {
        println!("-------------");
        println!("    Name: {}", metric.metric_name);
        println!("    Resource id: {:?}", metric.resource_id);
        println!("    Value: {}", metric.value);
        println!("    Unit: {:?}", metric.unit);
        println!("    Timestamp: {:?}", metric.timestamp);
    }

    println!("\n Configuration and AWS connectivity verified");
    Ok(())
}
