#[derive(Debug)]
pub struct ConnectionStatus {
    pub connected: bool,
    pub region: String,
    pub permissions: PermissionsCheck,
}

#[derive(Debug)]
pub struct PermissionsCheck {
    pub cost_explorer_read: bool,
    pub metrics_monitor_read: bool,
    pub instance_describe: bool,
}
