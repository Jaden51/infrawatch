pub mod collector;
pub mod types;

use crate::system::types::{DiskMetrics, MemoryMetrics, ProcessMetrics, SystemSnapshot};
use anyhow::Result;

pub trait SystemCollector {
    /// Collects the current memory usage metrics for the system.
    ///
    /// Implementations should query the underlying platform for memory-related
    /// statistics and return them as a `MemoryMetrics` value.
    fn collect_memory(&mut self) -> Result<MemoryMetrics>;

    /// Collects the current disk usage metrics for all relevant storage devices.
    ///
    /// Implementations should return a list of `DiskMetrics`, one for each disk
    /// or partition that is being monitored.
    fn collect_disk(&mut self) -> Result<Vec<DiskMetrics>>;

    /// Collects metrics describing the processes currently running on the system.
    ///
    /// Implementations should gather process-related statistics and return them
    /// in a single `ProcessesMetrics` structure.
    fn collect_processes(&mut self) -> Result<ProcessMetrics>;

    /// Collects a full system snapshot containing all supported metrics.
    ///
    /// This is a convenience method that should aggregate memory, disk, process,
    /// and any other available metrics into a `SystemSnapshot`.
    fn collect_all(&mut self) -> Result<SystemSnapshot>;
}
