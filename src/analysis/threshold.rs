use anyhow::Result;
use chrono::Utc;
use std::collections::HashMap;

use crate::{
    analysis::{
        AnomalyDetector,
        types::{Anomaly, Metric, Severity},
    },
    config::configs::{AnalysisConfig, ThresholdRule},
};

pub struct ThresholdDetector {
    pub thresholds: HashMap<String, ThresholdRule>,
}

impl ThresholdDetector {
    pub fn new(analysis_config: &AnalysisConfig) -> Result<Self> {
        let thresholds = analysis_config
            .thresholds
            .iter()
            .map(|threshold| (threshold.metric_name.clone(), threshold.clone()))
            .collect::<HashMap<_, _>>();
        Ok(Self { thresholds })
    }

    fn format_value(value: f64, unit: Option<&str>) -> String {
        match unit {
            Some("%") => format!("{value:.1}%"),
            Some(unit) => format!("{value:.1} {unit}"),
            None => format!("{value:.1}"),
        }
    }

    fn format_reason(metric: &Metric, severity: Severity, threshold: f64) -> String {
        let unit = metric.unit.as_deref();
        let value = Self::format_value(metric.value, unit);
        let threshold_value = Self::format_value(threshold, unit);
        let level = match severity {
            Severity::Warning => "warning",
            Severity::Critical => "critical",
        };
        format!(
            "{} ({}) exceeded {} threshold ({})",
            metric.name, value, level, threshold_value
        )
    }

    fn create_anomaly(metric: &Metric, threshold: f64, severity: Severity) -> Anomaly {
        Anomaly {
            reason: Self::format_reason(metric, severity, threshold),
            severity: Severity::Critical,
            detected_at: Utc::now(),
        }
    }
}

impl AnomalyDetector for ThresholdDetector {
    fn detect(&self, metrics: &[Metric]) -> Vec<Anomaly> {
        let mut anomalies: Vec<Anomaly> = Vec::new();
        for metric in metrics {
            let Some(rule) = self.thresholds.get(&metric.name) else {
                continue;
            };

            if let Some(critical) = rule.critical
                && metric.value >= critical
            {
                anomalies.push(Self::create_anomaly(metric, critical, Severity::Critical));
                continue;
            }
            if let Some(warning) = rule.warning
                && metric.value >= warning
            {
                anomalies.push(Self::create_anomaly(metric, warning, Severity::Warning));
            }
        }
        anomalies
    }
}
