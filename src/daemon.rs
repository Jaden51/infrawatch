use crate::{
    analysis::AnomalyDetector,
    analysis::threshold::ThresholdDetector,
    analysis::types::Severity,
    cloud::MetricsProvider,
    cloud::aws::AWSProvider,
    config::configs::Config,
    system::{SystemCollector, collector::SysinfoCollector},
};
use anyhow::{Result, anyhow};
use chrono::{Duration as ChronoDuration, Utc};
use tokio::time::{self, Duration};

pub async fn run(config: Config) -> Result<()> {
    println!("infrawatch daemon starting\n");
    println!("daemon config information:");
    println!(
        "    polling interval (seconds): {:?}",
        config.daemon.poll_interval_secs
    );

    let threshold_detector = ThresholdDetector::new(&config.analysis)
        .map_err(|e| anyhow!("Error creating threshold detector: {e}"))?;
    let mut sys_collector: Option<SysinfoCollector> = None;

    let system_enabled = config.system.enabled;
    let cloud_enabled = config.metrics.enabled;

    if system_enabled {
        sys_collector = match SysinfoCollector::new() {
            Ok(sys_collector) => Some(sys_collector),
            Err(e) => return Err(anyhow!("Error creating system metrics collector: {e}")),
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

    let mut shutdown = std::pin::pin!(tokio::signal::ctrl_c());

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(err) = poll_cycle(&mut sys_collector, &mut aws_provider, &threshold_detector, &config).await {
                    eprintln!("poll cycle failed: {err}");
                }
            },
            _ = &mut shutdown => {
                println!("daemon shutdown gracefully");
                return Ok(());
            },
        }
    }
}

async fn poll_cycle(
    sys_collector: &mut Option<SysinfoCollector>,
    aws_provider: &mut Option<AWSProvider>,
    threshold_detector: &ThresholdDetector,
    config: &Config,
) -> Result<()> {
    let mut collected_metrics = Vec::new();

    if let Some(collector) = sys_collector.as_mut() {
        match collector.collect_all() {
            Ok(snapshot) => {
                let system_metrics: Vec<_> = snapshot.into();
                println!(
                    "system collection succeeded: {} metrics",
                    system_metrics.len()
                );
                collected_metrics.extend(system_metrics);
            }
            Err(err) => {
                eprintln!("system metric collection failed, continuing: {err}");
            }
        }
    }

    if let Some(provider) = aws_provider.as_ref() {
        let end_time = Utc::now();
        let start_time =
            end_time - ChronoDuration::seconds(config.daemon.poll_interval_secs as i64);

        match provider
            .fetch_instance_metrics(&[], &config.metrics.instance_metrics, start_time, end_time)
            .await
        {
            Ok(cloud_points) => {
                let cloud_metrics: Vec<_> = cloud_points.into_iter().map(Into::into).collect();
                println!(
                    "cloud collection succeeded: {} metrics",
                    cloud_metrics.len()
                );
                collected_metrics.extend(cloud_metrics);
            }
            Err(err) => {
                eprintln!("cloud metric collection failed, continuing: {err}");
            }
        }
    }

    if collected_metrics.is_empty() {
        println!("poll cycle complete: no metrics collected");
        return Ok(());
    }

    let anomalies = threshold_detector.detect(&collected_metrics);
    if anomalies.is_empty() {
        println!(
            "poll cycle complete: {} metrics collected, 0 anomalies detected",
            collected_metrics.len()
        );
        return Ok(());
    }

    println!(
        "poll cycle complete: {} metrics collected, {} anomalies detected",
        collected_metrics.len(),
        anomalies.len()
    );

    println!("anomaly detection mode: {}", config.analysis.mode);
    for anomaly in anomalies {
        let severity = match anomaly.severity {
            Severity::Warning => "WARNING",
            Severity::Critical => "CRITICAL",
        };
        println!(
            "[{}] {}. Detected at {}",
            severity, anomaly.reason, anomaly.detected_at
        );
    }

    Ok(())
}
