use crate::analysis::types::{Anomaly, Metric};

pub mod convert;
pub mod threshold;
pub mod types;

pub trait AnomalyDetector {
    fn detect(&self, metrics: &[Metric]) -> Vec<Anomaly>;
}
