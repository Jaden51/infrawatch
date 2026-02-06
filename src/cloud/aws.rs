use crate::{cloud::MetricsProvider, config::configs::AWSConfig};
use anyhow::Result;
use aws_config::BehaviorVersion;
use aws_sdk_cloudwatch::{self as cloudwatch};
use aws_sdk_costexplorer as costexplorer;
use aws_sdk_ec2 as ec2;

use super::types::{ConnectionStatus, PermissionsCheck};

pub struct AWSProvider {
    cloudwatch: cloudwatch::Client,
    costexplorer: costexplorer::Client,
    ec2: ec2::Client,
    region: String,
}

impl AWSProvider {
    pub async fn new(config: &AWSConfig) -> Result<Self> {
        let aws_config = aws_config::defaults(BehaviorVersion::latest())
            .region(aws_config::Region::new(config.region.clone()))
            .profile_name(config.profile_name.clone())
            .load()
            .await;

        let ec2_client = ec2::Client::new(&aws_config);
        let cloudwatch_client = cloudwatch::Client::new(&aws_config);
        let costexplorer_client = costexplorer::Client::new(&aws_config);

        Ok(Self {
            cloudwatch: cloudwatch_client,
            costexplorer: costexplorer_client,
            ec2: ec2_client,
            region: config.region.clone(),
        })
    }
}

impl MetricsProvider for AWSProvider {
    async fn verify_connection(&self) -> Result<ConnectionStatus> {
        let mut permissions = PermissionsCheck {
            cost_explorer_read: false,
            metrics_monitor_read: false,
            instance_describe: false,
        };

        // TODO: cost explorer check
        permissions.cost_explorer_read = false;

        permissions.metrics_monitor_read = self.cloudwatch.list_metrics().send().await.is_ok();

        permissions.instance_describe = self
            .ec2
            .describe_instances()
            .max_results(10)
            .send()
            .await
            .is_ok();

        Ok(ConnectionStatus {
            connected: permissions.metrics_monitor_read || permissions.instance_describe,
            region: self.region.clone(),
            permissions,
        })
    }
}
