use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunnerInfo {
    pub id: String,
    pub name: String,
    pub tailscale_ip: Option<String>,
    pub cpu_cores: u32,
    pub memory_mb: u64,
    pub status: String, // running | starting | stopped
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerInfo {
    pub id: String,
    pub name: String,
    pub runner_name: String,
    pub address: Option<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TailscaleStatusResp {
    pub connected: bool,
    pub tailscale_ip: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StartRunnerParams {
    pub host_data_path: String,
    pub host_data_bind: String,
    pub cpu_cores: u32,
    pub memory_mb: u64,
}

#[derive(Debug, Serialize)]
pub struct StartRunnerResult {
    pub runner_id: String,
    pub log_lines: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct StartServerParams {
    pub name: String,
    pub image: String,
    pub cpu_cores: u32,
    pub memory_mb: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartServerResult {
    pub server_id: String,
    pub address: Option<String>,
}
