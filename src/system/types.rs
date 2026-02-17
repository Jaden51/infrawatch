use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct MemoryMetrics {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct DiskMetrics {
    pub mount_point: String,
    pub filesystem_type: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ProcessesMetrics {
    pub process_count: usize,
    pub process_info: Vec<ProcessMetric>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct ProcessMetric {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
}

#[derive(Debug)]
pub struct SystemSnapshot {
    pub memory: MemoryMetrics,
    pub disk: Vec<DiskMetrics>, // one entry per mount point
    pub processes: ProcessesMetrics,
    pub hostname: String,
}
