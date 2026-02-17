use anyhow::Result;
use chrono::Utc;
use sysinfo::{
    CpuRefreshKind, MemoryRefreshKind, ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System,
};

use crate::system::{
    SystemCollector,
    types::{DiskMetrics, MemoryMetrics, ProcessMetric, ProcessesMetrics, SystemSnapshot},
};

pub struct SysinfoCollector {
    pub system: System,
}

impl SysinfoCollector {
    pub fn new() -> Result<Self> {
        let refresh_kind = RefreshKind::nothing()
            .with_cpu(CpuRefreshKind::everything())
            .with_memory(MemoryRefreshKind::everything())
            .with_processes(ProcessRefreshKind::everything());

        let system = System::new_with_specifics(refresh_kind);
        Ok(SysinfoCollector { system })
    }
}

impl SystemCollector for SysinfoCollector {
    fn collect_all(&mut self) -> Result<SystemSnapshot> {
        // Run processes collection twice to ensure CPU deltas are populated
        let _ = self.collect_processes()?;
        std::thread::sleep(std::time::Duration::from_millis(500));

        let disk = self.collect_disk()?;
        let memory = self.collect_memory()?;
        let processes = self.collect_processes()?;
        let hostname = sysinfo::System::host_name().unwrap_or_else(|| "unknown".to_string());

        Ok(SystemSnapshot {
            memory,
            disk,
            processes,
            hostname,
        })
    }

    fn collect_memory(&mut self) -> Result<MemoryMetrics> {
        self.system.refresh_memory();

        let total_bytes = self.system.total_memory();
        let used_bytes = self.system.used_memory();
        let available_bytes = self.system.available_memory();
        let swap_total_bytes = self.system.total_swap();
        let swap_used_bytes = self.system.used_swap();
        let timestamp = Utc::now();
        let usage_percent = if total_bytes == 0 {
            0.0
        } else {
            used_bytes as f64 / total_bytes as f64 * 100.0
        };

        Ok(MemoryMetrics {
            total_bytes,
            used_bytes,
            available_bytes,
            usage_percent,
            swap_total_bytes,
            swap_used_bytes,
            timestamp,
        })
    }

    fn collect_disk(&mut self) -> Result<Vec<DiskMetrics>> {
        let disks = sysinfo::Disks::new_with_refreshed_list();
        let mut result: Vec<DiskMetrics> = Vec::new();
        for disk in disks.list() {
            let mount_point = disk.mount_point().to_string_lossy().into_owned();
            let filesystem_type = disk.file_system().to_string_lossy().into_owned();
            let total_bytes = disk.total_space();
            let available_bytes = disk.available_space();
            let used_bytes = total_bytes - available_bytes;
            let usage_percent = if total_bytes == 0 {
                0.0
            } else {
                used_bytes as f64 / total_bytes as f64 * 100.0
            };

            let metric = DiskMetrics {
                mount_point,
                filesystem_type,
                total_bytes,
                used_bytes,
                available_bytes,
                usage_percent,
                timestamp: Utc::now(),
            };
            result.push(metric);
        }

        Ok(result)
    }

    fn collect_processes(&mut self) -> Result<ProcessesMetrics> {
        self.system.refresh_processes(ProcessesToUpdate::All, true);
        let mut processes: Vec<_> = self.system.processes().values().collect();

        processes.sort_by(|a, b| {
            b.cpu_usage()
                .partial_cmp(&a.cpu_usage())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut process_info: Vec<ProcessMetric> = Vec::new();

        for process in processes.iter().take(5) {
            process_info.push(ProcessMetric {
                pid: process.pid().as_u32(),
                name: process.name().to_string_lossy().into_owned(),
                cpu_usage: process.cpu_usage(),
                memory: process.memory(),
            });
        }

        Ok(ProcessesMetrics {
            process_count: processes.len(),
            process_info,
            timestamp: Utc::now(),
        })
    }
}
