use crate::{
    analysis::types::{Metric, MetricSource},
    cloud::types::MetricDataPoint,
    system::types::SystemSnapshot,
};

impl From<SystemSnapshot> for Vec<Metric> {
    fn from(snapshot: SystemSnapshot) -> Self {
        let source = MetricSource::System {
            hostname: snapshot.hostname.clone(),
        };
        let mem = &snapshot.memory;
        let memory_metrics = [
            (
                "memory.total_bytes",
                mem.total_bytes as f64,
                Some("bytes"),
                mem.timestamp,
            ),
            (
                "memory.used_bytes",
                mem.used_bytes as f64,
                Some("bytes"),
                mem.timestamp,
            ),
            (
                "memory.available_bytes",
                mem.available_bytes as f64,
                Some("bytes"),
                mem.timestamp,
            ),
            (
                "memory.usage_percent",
                mem.usage_percent,
                Some("%"),
                mem.timestamp,
            ),
            (
                "memory.swap_total_bytes",
                mem.swap_total_bytes as f64,
                Some("bytes"),
                mem.timestamp,
            ),
            (
                "memory.swap_used_bytes",
                mem.swap_used_bytes as f64,
                Some("bytes"),
                mem.timestamp,
            ),
        ]
        .into_iter()
        .map(|(name, value, unit, ts)| Metric {
            name: name.to_string(),
            value,
            source: source.clone(),
            unit: unit.map(str::to_string),
            timestamp: ts,
        });

        let disk_metrics = snapshot.disk.iter().flat_map(|d| {
            let mount = d
                .mount_point
                .replace('/', "_")
                .replace(' ', "_")
                .replace('.', "_");
            [
                (
                    format!("disk.{mount}.total_bytes"),
                    d.total_bytes as f64,
                    Some("bytes"),
                    d.timestamp,
                ),
                (
                    format!("disk.{mount}.used_bytes"),
                    d.used_bytes as f64,
                    Some("bytes"),
                    d.timestamp,
                ),
                (
                    format!("disk.{mount}.available_bytes"),
                    d.available_bytes as f64,
                    Some("bytes"),
                    d.timestamp,
                ),
                (
                    format!("disk.{mount}.usage_percent"),
                    d.usage_percent,
                    Some("%"),
                    d.timestamp,
                ),
            ]
            .into_iter()
            .map(|(name, value, unit, ts)| Metric {
                name,
                value,
                source: source.clone(),
                unit: unit.map(str::to_string),
                timestamp: ts,
            })
        });

        let p = &snapshot.processes;
        let process_count = std::iter::once(Metric {
            name: "process.count".to_string(),
            value: p.process_count as f64,
            source: source.clone(),
            unit: None,
            timestamp: p.timestamp,
        });
        let per_process = p.process_info.iter().flat_map(|proc| {
            [
                (
                    format!("process.{}.cpu_percent", proc.pid),
                    proc.cpu_usage as f64,
                    Some("%"),
                ),
                (
                    format!("process.{}.memory_bytes", proc.pid),
                    proc.memory_bytes as f64,
                    Some("bytes"),
                ),
            ]
            .into_iter()
            .map(|(name, value, unit)| Metric {
                name,
                value,
                source: source.clone(),
                unit: unit.map(str::to_string),
                timestamp: p.timestamp,
            })
        });
        memory_metrics
            .chain(disk_metrics)
            .chain(process_count)
            .chain(per_process)
            .collect()
    }
}

impl From<MetricDataPoint> for Metric {
    fn from(value: MetricDataPoint) -> Self {
        let instance_id = value.resource_id.unwrap_or_else(|| "unknown".to_string());

        // current implementation assumes AWS provider for cloud metrics, will refactor
        // after adding more providers
        let source = MetricSource::Cloud {
            provider: "aws".to_string(),
            instance_id,
        };

        Metric {
            name: value.metric_name,
            value: value.value,
            source,
            unit: value.unit,
            timestamp: value.timestamp,
        }
    }
}
