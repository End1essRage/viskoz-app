use crate::cli_bridge::{run_cli_json, run_cli_json_with_log};
use crate::app_state::AppState;
use crate::types::*;
use tauri::{AppHandle, State};

#[tauri::command]
pub async fn login(rest: State<'_, AppState>, params: LoginParams) -> Result<serde_json::Value, String> {
    rest.login(params.username, params.password).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sign_up(rest: State<'_, AppState>, params: SignUpParams) -> Result<(), String> {
    rest.sign_up(params.email, params.username, params.password).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn restore_session(rest: State<'_, AppState>) -> Result<serde_json::Value, String> {
    rest.restore_session().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn logout(rest: State<'_, AppState>) -> Result<(), String> {
    rest.logout().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_runners(rest: State<'_, AppState>) -> Result<Vec<RunnerInfo>, String> {
    rest.list_runners().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_servers(rest: State<'_, AppState>) -> Result<Vec<ServerInfo>, String> {
    rest.list_servers().await.map_err(|e| e.to_string())
}

/// Статус Tailscale получаем через CLI, а не REST: CLI управляет sidecar-контейнером
/// и знает его локальный API (127.0.0.1:41234), control-plane эту информацию не хранит.
#[tauri::command]
pub async fn tailscale_status(app: AppHandle) -> Result<TailscaleStatusResp, String> {
    run_cli_json(&app, &["tailscale", "status", "--json"])
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn join_mesh(app: AppHandle) -> Result<TailscaleStatusResp, String> {
    run_cli_json(
        &app,
        &["user", "connect", "--cp-address-user", "cp.viskoz.dev:9095", "--join-secret", "none"],
    )
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn start_runner(app: AppHandle, params: StartRunnerParams) -> Result<StartRunnerResult, String> {
    let cpu = params.cpu_cores.to_string();
    let mem = params.memory_mb.to_string();
    let args = vec![
        "runner", "start",
        "--cp-address-runner", &params.cp_address,
        "--join-secret", &params.runner_token,
        "--host-data-path", &params.host_data_path,
        "--cpu-cores", &cpu,
        "--memory-mb", &mem,
    ];
    let (result, log_lines): (StartRunnerResultRaw, Vec<String>) =
        run_cli_json_with_log(&app, &args).await.map_err(|e| e.to_string())?;

    Ok(StartRunnerResult { runner_id: result.runner_id, log_lines })
}

#[tauri::command]
pub async fn start_server(
    rest: State<'_, AppState>,
    runner_id: String,
    req: ExecuteRequest,
) -> Result<serde_json::Value, String> {
    rest.execute_on_runner(&runner_id, &req).await.map_err(|e| e.to_string())
}

//TODO - redo какая то шняга прямого управления контейнерами не планируется и в cli такого нет и не будет скорее всего
#[tauri::command]
pub async fn stop_container(app: AppHandle, id: String) -> Result<(), String> {
    run_cli_json::<serde_json::Value>(&app, &["container", "stop", "--id", &id, "--json"])
        .await
        .map(|_| ())
        .map_err(|e| e.to_string())
}

#[derive(serde::Deserialize)]
struct StartRunnerResultRaw {
    runner_id: String,
}