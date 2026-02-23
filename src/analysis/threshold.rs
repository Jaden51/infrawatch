use anyhow::Result;
use std::collections::HashMap;

use crate::{
    analysis::AnomalyDetector,
    config::configs::{AnalysisConfig, ThresholdRule},
};

struct ThresholdDetector {
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
}

impl AnomalyDetector for ThresholdDetector {
    fn detect(&self, metrics: &[super::types::Metric]) -> Vec<super::types::Anomaly> {
        unimplemented!()
    }
}
