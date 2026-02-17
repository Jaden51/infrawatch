pub mod collector;
pub mod types;

use crate::system::types::{DiskMetrics, MemoryMetrics, ProcessesMetrics, SystemSnapshot};
use anyhow::Result;

pub trait SystemCollector {
    fn collect_memory(&mut self) -> Result<MemoryMetrics>;
    fn collect_disk(&mut self) -> Result<Vec<DiskMetrics>>;
    fn collect_processes(&mut self) -> Result<ProcessesMetrics>;
    fn collect_all(&mut self) -> Result<SystemSnapshot>;
}
