use crate::cloud::types::ConnectionStatus;
use anyhow::Result;

pub mod aws;
pub mod types;

pub trait MetricsProvider: Send + Sync {
    async fn verify_connection(&self) -> Result<ConnectionStatus>;
}
