use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub source: MetricSource,
    pub unit: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct Anomaly {
    pub metric: Metric,
    pub reason: String,
    pub severity: Severity,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum MetricSource {
    System {
        hostname: String,
    },
    Cloud {
        provider: String,
        instance_id: String,
    },
}

#[derive(Debug)]
pub enum Severity {
    Warning,
    Critical,
}
