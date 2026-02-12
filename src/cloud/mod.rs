use crate::cloud::types::{ConnectionStatus, CostDataPoint, Instance, MetricDataPoint};
use anyhow::Result;
use aws_sdk_costexplorer::types::Granularity;
use chrono::{DateTime, Utc};

pub mod aws;
pub mod types;

pub trait MetricsProvider: Send + Sync {
    /// Verify provider connection and permissions
    async fn verify_connection(&self) -> Result<ConnectionStatus>;

    /// Discover instances, optionally filtered by tags
    async fn discover_instances(&self, tag_filters: &[(String, String)]) -> Result<Vec<Instance>>;

    /// Fetch and compute instance metrics for the given instance IDs
    /// If instance_ids is empty, fetch and compute for all instances
    async fn fetch_instance_metrics(
        &self,
        instance_ids: &[String],
        metric_names: &[String],
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Vec<MetricDataPoint>>;

    /// Fetch and compute cost metrics for the given instance IDs
    /// If instance_ids is empty, fetch and compute for all instances
    async fn fetch_cost_data(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        granularity: Granularity,
    ) -> Result<Vec<CostDataPoint>>;
}
