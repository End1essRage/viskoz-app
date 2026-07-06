import { invoke } from "@tauri-apps/api/core";

// --- Чтение состояния: команды идут в Rust core, который бьёт в REST control-plane ---

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

export function startServer(params) {
  // params: { name, image, cpuCores, memoryMb }
  return invoke("start_server", { params });
}

export function startRunner(params) {
  // params: { hostDataPath, hostDataBind, cpuCores, memoryMb }
  return invoke("start_runner", { params });
}

export function joinMesh() {
  return invoke("join_mesh");
}

export function stopContainer(id) {
  return invoke("stop_container", { id });
}
