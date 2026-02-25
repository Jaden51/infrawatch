use crate::{
    analysis::threshold::ThresholdDetector, cloud::aws::AWSProvider, config::configs::Config,
    system::collector::SysinfoCollector,
};
use anyhow::{Result, anyhow};
use tokio::time::{self, Duration};

pub async fn run(config: Config) -> Result<()> {
    println!("infrawatch daemon starting\n");
    println!("daemon config information:");
    println!(
        "    polling interval (seconds): {:?}",
        config.daemon.poll_interval_secs
    );

    let threshold_detector = ThresholdDetector::new(&config.analysis);
    let mut sys_collector: Option<SysinfoCollector> = None;

    let system_enabled = config.system.enabled;
    let cloud_enabled = config.metrics.enabled;

    if system_enabled {
        sys_collector = match SysinfoCollector::new() {
            Ok(sys_collector) => Some(sys_collector),
            Err(e) => return Err(anyhow!("Error creating system metrics collector {e}")),
        }
    }
    let mut aws_provider: Option<AWSProvider> = None;
    if cloud_enabled {
        aws_provider = match AWSProvider::new(&config.aws).await {
            Ok(aws_provider) => Some(aws_provider),
            Err(e) => return Err(anyhow!("Error creating AWS Provider: {e}")),
        }
    }

    let mode = match (system_enabled, cloud_enabled) {
        (true, _) => "agent",
        (false, true) => "cloud",
        _ => return Err(anyhow!("Nothing to monitor: system and cloud disabled")),
    };

    println!("\ndaemon running in {mode} mode");

    let mut interval = time::interval(Duration::from_secs(config.daemon.poll_interval_secs));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(err) = poll_cycle().await {
                    eprintln!("poll cycle failed: {err}");
                }
            },
            _ = tokio::signal::ctrl_c() => {
                println!("daemon shutdown gracefully");
                return Ok(())
            },
        };
    }
}

async fn poll_cycle() -> Result<()> {
    println!("polling cycle");
    Ok(())
}
