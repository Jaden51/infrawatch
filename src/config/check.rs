use crate::system::types::SystemSnapshot;
use crate::{
    cloud::{MetricsProvider, aws::AWSProvider},
    config::{configs::Config, load::load_config},
    system::{SystemCollector, collector::SysinfoCollector},
};
use anyhow::Result;
use chrono::Utc;
use std::path::Path;

pub async fn verify_config(config_path: Option<&Path>) -> Result<()> {
    let config: Config = load_config(config_path)?;

    println!("Configuration loaded successfully\n");

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

    println!("\nFetching System Information Metrics");
    let mut system_collector = SysinfoCollector::new()?;

    let _ = system_collector.collect_processes()?;

    std::thread::sleep(std::time::Duration::from_millis(500));

    let snapshot = system_collector.collect_all()?;

    let SystemSnapshot {
        memory: memory_metrics,
        disk: disk_metrics,
        processes: processes_metrics,
        hostname,
    } = snapshot;

    println!("\nHostname: {}", hostname);

    println!(
        "\nSystem Memory (timestamp: {:?}):",
        memory_metrics.timestamp
    );
    println!("  Total: {} bytes", memory_metrics.total_bytes);
    println!("  Used: {} bytes", memory_metrics.used_bytes);
    println!("  Available: {} bytes", memory_metrics.available_bytes);
    println!("  Usage: {:.2}%", memory_metrics.usage_percent);
    println!("  Swap Total: {} bytes", memory_metrics.swap_total_bytes);
    println!("  Swap Used: {} bytes", memory_metrics.swap_used_bytes);

    println!("\nDisks:");
    for d in disk_metrics.iter() {
        println!("  Mount: {} (timestamp: {:?})", d.mount_point, d.timestamp);
        println!("    FS: {}", d.filesystem_type);
        println!("    Total: {} bytes", d.total_bytes);
        println!("    Used: {} bytes", d.used_bytes);
        println!("    Available: {} bytes", d.available_bytes);
        println!("    Usage: {:.2}%", d.usage_percent);
    }

    println!(
        "\nTop Processes (timestamp: {:?}):",
        processes_metrics.timestamp
    );
    println!("  Count: {}", processes_metrics.process_count);
    for p in processes_metrics.process_info.iter() {
        println!(
            "    PID: {}  Name: {}  CPU: {:.2}%  Mem: {} Bytes",
            p.pid, p.name, p.cpu_usage, p.memory_bytes
        );
    }

    println!("\n Configuration and AWS connectivity verified");
    Ok(())
}
