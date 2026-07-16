mod cli_bridge;
mod commands;
mod app_state;
mod types;

use app_state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::new(
            std::env::var("CONTROL_PLANE_URL").unwrap_or_else(|_| "https://cp.viskoz.dev".into()),
            "/rest-cp",
            "/auth",
        ))
        .invoke_handler(tauri::generate_handler![
            commands::login,
            commands::sign_up,
            commands::restore_session,
            commands::logout,
            commands::list_runners,
            commands::list_servers,
            commands::tailscale_status,
            commands::join_mesh,
            commands::start_runner,
            commands::start_server,
            commands::stop_container,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}