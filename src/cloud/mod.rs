use crate::cloud::types::{ConnectionStatus, Instance};
use anyhow::Result;

pub mod aws;
pub mod types;

pub trait MetricsProvider: Send + Sync {
    async fn verify_connection(&self) -> Result<ConnectionStatus>;
    async fn discover_instances(&self, tag_filters: &[(String, String)]) -> Result<Vec<Instance>>;
}
