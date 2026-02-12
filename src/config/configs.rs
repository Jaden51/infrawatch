use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub aws: AWSConfig,
    pub metrics: MetricsConfig,
}

#[derive(Debug, Deserialize)]
pub struct AWSConfig {
    pub region: String,
    pub profile_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MetricsConfig {
    pub instance_metrics: Vec<String>,
}
