use crate::{cloud::MetricsProvider, config::configs::AWSConfig};
use anyhow::Result;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_cloudwatch::{self as cloudwatch};
use aws_sdk_costexplorer::{self as costexplorer, types::DateInterval, types::Granularity};
use aws_sdk_ec2 as ec2;
use chrono::Utc;

use super::types::{ConnectionStatus, PermissionsCheck};

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
}
