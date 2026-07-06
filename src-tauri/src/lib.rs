mod cli_bridge;
mod commands;
mod rest_client;
mod types;

use rest_client::RestClient;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let control_plane_url =
        std::env::var("CONTROL_PLANE_URL").unwrap_or_else(|_| "https://cp.viskoz.dev/rest-cp".into());

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(RestClient::new(control_plane_url))
        .invoke_handler(tauri::generate_handler![
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