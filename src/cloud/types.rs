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

#[derive(Debug)]
pub struct Instance {
    pub instance_id: String,
    pub instance_type: String,
    pub state: String,
    pub name: Option<String>,
    pub tags: Vec<(String, String)>,
}
