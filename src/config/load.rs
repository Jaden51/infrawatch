use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::config::configs::Config;

pub fn get_default_path() -> Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("", "", "infrawatch")
        .context("Could not determine config directory")?;
    let config_dir = dirs.config_dir().join("config.toml");
    Ok(config_dir)
}

pub fn load_config(path: Option<&Path>) -> Result<Config> {
    let config_path = match path {
        Some(p) => p.to_path_buf(),
        None => get_default_path()?,
    };

    let contents = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config from {}", config_path.display()))?;

    let config: Config = toml::from_str(&contents)
        .with_context(|| format!("Failed to parse config from {}", config_path.display()))?;

    Ok(config)
}
