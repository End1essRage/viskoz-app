import { invoke } from "@tauri-apps/api/core";

// --- Auth: команды идут в Rust core, который бьёт в REST control-plane ---
export function login(email, password) {
  return invoke("login", { params: { email, password } });
}

export function signUp(email, username, password) {
  return invoke("sign_up", { params: { email, username, password } });
}

export function restoreSession() {
  return invoke("restore_session");
}

export function logout() {
  return invoke("logout");
}

export function listServers() {
  return invoke("list_servers");
}

export function listRunners() {
  return invoke("list_runners");
}

export function tailscaleStatus() {
  return invoke("tailscale_status");
}

// --- Действия: команды идут в Rust core, который спавнит CLI как sidecar-процесс ---

export function startServer(runnerId, req) {
  return invoke("start_server", { runnerId, req });
}

export function startRunner(params) {
  return invoke("start_runner", { params });
}

export function joinMesh() {
  return invoke("join_mesh");
}

export function stopContainer(id) {
  return invoke("stop_container", { id });
}