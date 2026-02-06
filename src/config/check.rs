use crate::config::{configs::Config, load::load_config};
use anyhow::Result;
use std::path::Path;

pub async fn verify_config(config_path: Option<&Path>) -> Result<Config> {
    let config: Config = load_config(config_path)?;

    Ok(config)
}
