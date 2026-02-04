use crate::config::{configs::Config, load::load_config};
use anyhow::Result;
use std::path::Path;

pub fn verify_config(config_path: Option<&Path>) -> Result<()> {
    let _config: Config = load_config(config_path)?;
    Ok(())
}
