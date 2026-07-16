mod cli_bridge;
mod commands;
mod rest_client;
mod types;

use rest_client::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_store::init())
        .manage(AppState::new(
            std::env::var("CONTROL_PLANE_URL")
                .unwrap_or_else(|_| "https://cp.viskoz.dev/rest-cp".into()),
            "/rest-cp",
            "/rest-cp",
        ))
        .invoke_handler(tauri::generate_handler![
            //commands::login_user,
            //commands::logout_user,
            //commands::check_auth_status,
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
