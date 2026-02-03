use crate::config::{configs::Config, load::load_config};
use anyhow::Result;
use std::path::Path;

pub fn verify_config(config_path: Option<&Path>) -> Result<()> {
    let config: Config = load_config(config_path)?;

    println!("Printing AWS configuration");
    println!("AWS Region: {}", config.aws.region);
    Ok(())
}
