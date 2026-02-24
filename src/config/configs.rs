use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub aws: AWSConfig,
    pub metrics: MetricsConfig,
    pub system: SystemConfig,
    pub analysis: AnalysisConfig,
}

#[derive(Debug, Deserialize)]
pub struct AWSConfig {
    pub region: String,
    pub profile_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MetricsConfig {
    #[serde(default = "default_instance_metrics")]
    pub instance_metrics: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SystemConfig {
    pub enabled: bool,
    pub collect_memory: bool,
    pub collect_disk: bool,
    pub collect_processes: bool,
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

fn default_instance_metrics() -> Vec<String> {
    vec![
        "CPUUtilization".into(),
        "NetworkIn".into(),
        "NetworkOut".into(),
    ]
}
