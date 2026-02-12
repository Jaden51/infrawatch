use crate::{
    cloud::{MetricsProvider, types::CostDataPoint},
    config::configs::AWSConfig,
};
use anyhow::{Context, Result, anyhow};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_cloudwatch::{self as cloudwatch, primitives::DateTime as AwsDateTime};
use aws_sdk_costexplorer::{self as costexplorer};
use aws_sdk_ec2::{self as ec2, types::Filter};
use chrono::{DateTime, Utc};
use cloudwatch::types::{Dimension, Metric, MetricDataQuery, MetricStat};
use costexplorer::types::{DateInterval, Granularity, GroupDefinition, GroupDefinitionType};
use std::collections::HashMap;
use std::time::SystemTime;

use super::types::{ConnectionStatus, Instance, PermissionsCheck};

pub struct AWSProvider {
    cloudwatch: cloudwatch::Client,
    costexplorer: costexplorer::Client,
    ec2: ec2::Client,
    region: String,
}

impl AWSProvider {
    pub async fn new(config: &AWSConfig) -> Result<Self> {
        let mut builder = aws_config::defaults(BehaviorVersion::latest())
            .region(aws_config::Region::new(config.region.clone()));

        if let Some(profile) = &config.profile_name {
            builder = builder.profile_name(profile.clone());
        }

        let aws_config = builder.load().await;

        let ec2_client = ec2::Client::new(&aws_config);
        let cloudwatch_client = cloudwatch::Client::new(&aws_config);

        // Cost Explorer forced to us-east-1
        let ce_config = aws_config
            .into_builder()
            .region(Region::new("us-east-1"))
            .build();
        let costexplorer_client = costexplorer::Client::new(&ce_config);

        Ok(Self {
            cloudwatch: cloudwatch_client,
            costexplorer: costexplorer_client,
            ec2: ec2_client,
            region: config.region.clone(),
        })
    }

    pub async fn verify_cost_explorer_connection(&self) -> Result<()> {
        let start_date = Utc::now() - chrono::Duration::days(1);
        let end_date = Utc::now();

        let date_interval = DateInterval::builder()
            .start(start_date.format("%Y-%m-%d").to_string())
            .end(end_date.format("%Y-%m-%d").to_string())
            .build()?;

        self.costexplorer
            .get_cost_and_usage()
            .time_period(date_interval)
            .granularity(Granularity::Daily)
            .metrics("UnblendedCost")
            .send()
            .await?;

        Ok(())
    }

    pub fn chrono_to_aws(&self, dt: chrono::DateTime<Utc>) -> AwsDateTime {
        AwsDateTime::from_secs_and_nanos(dt.timestamp(), dt.timestamp_subsec_nanos())
    }
}

impl MetricsProvider for AWSProvider {
    async fn verify_connection(&self) -> Result<ConnectionStatus> {
        let mut permissions = PermissionsCheck {
            cost_explorer_read: false,
            metrics_monitor_read: false,
            instance_describe: false,
        };

        permissions.metrics_monitor_read = self.cloudwatch.list_metrics().send().await.is_ok();

        permissions.instance_describe = self
            .ec2
            .describe_instances()
            .max_results(10)
            .send()
            .await
            .is_ok();

        permissions.cost_explorer_read = self.verify_cost_explorer_connection().await.is_ok();

        Ok(ConnectionStatus {
            connected: permissions.metrics_monitor_read || permissions.instance_describe,
            region: self.region.clone(),
            permissions,
        })
    }

    async fn discover_instances(&self, tag_filters: &[(String, String)]) -> Result<Vec<Instance>> {
        let mut request = self.ec2.describe_instances();
        for (key, value) in tag_filters {
            let filter = Filter::builder()
                .name(format!("tag:{}", key))
                .values(value)
                .build();
            request = request.filters(filter)
        }

        let response = request
            .send()
            .await
            .context("Failed to describe EC2 instances")?;

        let mut instances = Vec::new();

        for reservation in response.reservations() {
            for instance in reservation.instances() {
                let instance_id = instance
                    .instance_id()
                    .ok_or_else(|| anyhow::anyhow!("Instance missing ID"))?
                    .to_string();

                let instance_type = instance
                    .instance_type()
                    .map(|t| t.as_str().to_string())
                    .ok_or_else(|| anyhow::anyhow!("Instance type missing"))?
                    .to_string();

                let state = instance
                    .state()
                    .and_then(|s| s.name())
                    .map(|n| n.as_str().to_string())
                    .unwrap_or_else(|| "unknown".to_string());

                let name = instance
                    .tags()
                    .iter()
                    .find(|t| t.key() == Some("Name"))
                    .and_then(|t| t.value())
                    .map(|t| t.to_string());

                let tags: Vec<_> = instance
                    .tags()
                    .iter()
                    .filter_map(|t| Some((t.key()?.to_string(), t.value()?.to_string())))
                    .collect();

                instances.push(Instance {
                    instance_id,
                    instance_type,
                    state,
                    name,
                    tags,
                });
            }
        }

        Ok(instances)
    }

    async fn fetch_instance_metrics(
        &self,
        instance_ids: &[String],
        metric_names: &[String],
        start_time: chrono::DateTime<Utc>,
        end_time: chrono::DateTime<Utc>,
    ) -> Result<Vec<super::types::MetricDataPoint>> {
        let target_instance_ids = if instance_ids.is_empty() {
            self.discover_instances(&[])
                .await?
                .into_iter()
                .map(|instance| instance.instance_id)
                .collect()
        } else {
            instance_ids.to_vec()
        };

        if target_instance_ids.is_empty() || metric_names.is_empty() {
            return Ok(Vec::new());
        }

        let mut queries = Vec::new();
        let mut query_lookup = HashMap::new();
        let mut id_counter = 0;

        // Build a query for each instance + metric combination
        for instance_id in &target_instance_ids {
            for metric_name in metric_names {
                let dimension = Dimension::builder()
                    .name("InstanceId")
                    .value(instance_id)
                    .build();

                let metric = Metric::builder()
                    .namespace("AWS/EC2")
                    .metric_name(metric_name)
                    .dimensions(dimension)
                    .build();

                let metric_stat = MetricStat::builder()
                    .metric(metric)
                    .period(300) // 5 minutes
                    .stat("Average")
                    .build();

                let query_id = format!("m{}", id_counter);
                query_lookup.insert(query_id.clone(), (instance_id.clone(), metric_name.clone()));

                let query = MetricDataQuery::builder()
                    .id(query_id)
                    .metric_stat(metric_stat)
                    .return_data(true)
                    .build();

                queries.push(query);
                id_counter += 1;
            }
        }

        if queries.is_empty() {
            return Ok(Vec::new());
        }

        let response = self
            .cloudwatch
            .get_metric_data()
            .set_metric_data_queries(Some(queries))
            .start_time(self.chrono_to_aws(start_time))
            .end_time(self.chrono_to_aws(end_time))
            .send()
            .await
            .context("Failed to fetch CloudWatch metrics")?;

        let mut results = Vec::new();

        for result in response.metric_data_results() {
            let id = result.id().unwrap_or_default();
            let Some((instance_id, metric_name)) = query_lookup.get(id) else {
                continue;
            };

            let timestamps = result.timestamps();
            let values = result.values();
            for (timestamp, value) in timestamps.iter().zip(values.iter()) {
                let system_time = SystemTime::try_from(*timestamp)
                    .map_err(|err| anyhow!("Failed to convert metric timestamp: {err}"))?;
                let chrono_timestamp = DateTime::<Utc>::from(system_time);

                results.push(super::types::MetricDataPoint {
                    metric_name: metric_name.clone(),
                    resource_id: Some(instance_id.clone()),
                    value: *value,
                    unit: None,
                    timestamp: chrono_timestamp,
                });
            }
        }

        Ok(results)
    }

    async fn fetch_cost_data(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        granularity: Granularity,
    ) -> Result<Vec<CostDataPoint>> {
        let date_interval = DateInterval::builder()
            .start(start_date.format("%Y-%m-%d").to_string())
            .end(end_date.format("%Y-%m-%d").to_string())
            .build()?;

        let group_by = GroupDefinition::builder()
            .r#type(GroupDefinitionType::Dimension)
            .key("SERVICE")
            .build();

        let response = self
            .costexplorer
            .get_cost_and_usage()
            .time_period(date_interval)
            .granularity(granularity)
            .metrics("UnblendedCost")
            .group_by(group_by)
            .send()
            .await
            .context("Failed to fetch cost data")?;

        let mut results = Vec::new();

        for result_by_time in response.results_by_time() {
            let period = result_by_time.time_period();
            let period_start = period.map(|p| p.start().to_string());
            let period_end = period.map(|p| p.end().to_string());
            let period_start = match period_start {
                Some(value) => value,
                None => continue,
            };
            let period_end = match period_end {
                Some(value) => value,
                None => continue,
            };

            let parsed_start = DateTime::parse_from_rfc3339(&format!("{period_start}T00:00:00Z"))
                .map_err(|err| anyhow!("Invalid cost period end {period_end}: {err}"))?
                .with_timezone(&Utc);

            let parsed_end = DateTime::parse_from_rfc3339(&format!("{period_end}T00:00:00Z"))
                .map_err(|err| anyhow!("Invalid cost period end {period_end}: {err}"))?
                .with_timezone(&Utc);

            for group in result_by_time.groups() {
                let service = group.keys().first().cloned();

                let metric = group
                    .metrics()
                    .and_then(|metrics| metrics.get("UnblendedCost"));
                let Some(metric) = metric else {
                    continue;
                };

                let amount = metric
                    .amount()
                    .and_then(|value| value.parse::<f64>().ok())
                    .unwrap_or(0.0);
                let unit = metric.unit().unwrap_or("USD").to_string();

                results.push(CostDataPoint {
                    service,
                    amount,
                    unit,
                    period_start: parsed_start,
                    period_end: parsed_end,
                });
            }
        }

        Ok(results)
    }
}
