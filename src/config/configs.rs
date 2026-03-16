use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub daemon: DaemonConfig,
    pub aws: AWSConfig,
    pub metrics: MetricsConfig,
    pub system: SystemConfig,
    pub analysis: AnalysisConfig,
    pub alerting: AlertingConfig,
}

#[derive(Debug, Deserialize)]
pub struct DaemonConfig {
    pub poll_interval_secs: u64,
}

#[derive(Debug, Deserialize)]
pub struct AWSConfig {
    pub region: String,
    pub profile_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    #[serde(default = "default_instance_metrics")]
    pub instance_metrics: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SystemConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct AnalysisConfig {
    pub mode: String,
    pub thresholds: Vec<ThresholdRule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ThresholdRule {
    pub metric_name: String,
    pub warning: Option<f64>,
    pub critical: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AlertingConfig {
    pub enabled: bool,
    pub webhook_env: String,
    pub webhook_url: Option<String>,
}

fn default_instance_metrics() -> Vec<String> {
    vec![
        "CPUUtilization".into(),
        "NetworkIn".into(),
        "NetworkOut".into(),
    ]
}
