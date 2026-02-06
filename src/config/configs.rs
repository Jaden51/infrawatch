use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub aws: AWSConfig,
}

#[derive(Debug, Deserialize)]
pub struct AWSConfig {
    pub region: String,
    pub profile_name: String,
}
