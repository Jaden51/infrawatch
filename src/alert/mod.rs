use anyhow::Result;

use crate::analysis::types::Anomaly;

pub mod discord;

pub trait AlertSender {
    async fn send(&self, anomalies: &[Anomaly]) -> Result<()>;
}
