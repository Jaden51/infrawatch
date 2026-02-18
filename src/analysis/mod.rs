use crate::analysis::types::{Anomaly, Metric};

mod convert;
mod types;

pub trait AnomalyDetector {
    fn detect(&self, metrics: &[Metric]) -> Vec<Anomaly>;
}
