use std::env;

use anyhow::{Result, anyhow};

use crate::{alert::AlertSender, config::configs::AlertingConfig};

pub struct DiscordAlerter {
    client: reqwest::Client,
    webhook_url: String,
}

impl DiscordAlerter {
    pub fn new(config: &AlertingConfig) -> Result<Self> {
        let webhook_url = env::var(&config.webhook_env)
            .ok()
            .or_else(|| config.webhook_url.clone())
            .ok_or_else(|| anyhow!("No webhook URL found"))?;

        Ok(Self {
            client: reqwest::Client::new(),
            webhook_url,
        })
    }
}

impl AlertSender for DiscordAlerter {
    fn send(&self, anomalies: &[crate::analysis::types::Anomaly]) -> anyhow::Result<()> {
        Ok(())
    }
}
