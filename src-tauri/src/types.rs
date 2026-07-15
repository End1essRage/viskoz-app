use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RunnerInfo {
    pub runner_id: String,
    pub node_name: String,
    pub ip: String,
    pub ready: bool,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub running_servers: u32,
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
#[serde(rename_all = "camelCase")]
pub struct StartRunnerParams {
    pub host_data_path: String,
    pub runner_token: String,
    pub cp_address: String,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerIdWrapper {
    pub server_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameSpec {
    pub downloader_type: String,
    pub runner_image: String,
    pub game_saves_mount_path: String,
    pub supports_mods: bool,
    pub mods_mount_path: String,
    pub force_save_cmd: String,
    pub preinstaller_type: String,
    pub preinstaller_image: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub game_id: String,
    pub game_name: String,
    pub spec: GameSpec,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExecuteRequest {
    pub server_id: ServerIdWrapper,
    pub game: Game,
    pub start_script: String,
    pub env: Vec<String>,
    pub mod_set_id: String,
    pub mod_set_path: String,
    pub is_new_world: bool,
    pub s3_presigned_get_url: String,
    pub s3_presigned_put_url: String,
    #[serde(rename = "preinstaller_env")]
    pub preinstaller_env: Vec<String>,
}