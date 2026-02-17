use crate::cloud::types::{ConnectionStatus, Instance, MetricDataPoint};
use anyhow::Result;
use chrono::{DateTime, Utc};

pub mod aws;
pub mod types;

pub trait MetricsProvider: Send + Sync {
    /// Verify provider connection and permissions
    ///
    /// Implementations should return a ConnectionStatus
    async fn verify_connection(&self) -> Result<ConnectionStatus>;

    /// Discover instances, optionally filtered by tags
    ///
    /// Implementations should return a list of instances on the hosts
    /// chosen cloud provider
    async fn discover_instances(&self, tag_filters: &[(String, String)]) -> Result<Vec<Instance>>;

    /// Fetch and compute instance metrics for the given instance IDs
    /// If instance_ids is empty, fetch and compute for all instances
    ///
    /// Implementations should return a list of MetricDataPoint that the
    /// host specifies in their config.
    async fn fetch_instance_metrics(
        &self,
        instance_ids: &[String],
        metric_names: &[String],
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<MetricDataPoint>>;
}
