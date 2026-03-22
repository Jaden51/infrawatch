use std::{collections::HashMap, env};

use anyhow::{Result, anyhow};

use crate::{
    alert::AlertSender,
    analysis::types::{Anomaly, Severity},
    config::configs::AlertingConfig,
};

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
    async fn send(&self, anomalies: &[Anomaly]) -> anyhow::Result<()> {
        for anomaly in anomalies {
            let mut body = HashMap::new();
            let severity = match anomaly.severity {
                Severity::Warning => "WARNING",
                Severity::Critical => "CRITICAL",
            };
            let message = format!(
                "[{}] {}. Detected at {}",
                severity, anomaly.reason, anomaly.detected_at
            );

            body.insert("content", message);
            self.client
                .post(self.webhook_url.clone())
                .json(&body)
                .send()
                .await?;
        }

        Ok(())
    }
}
