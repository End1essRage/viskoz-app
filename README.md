# runner-manager (Tauri + Svelte)

Десктоп-обёртка над `runner-cli`: роли admin / user / runner, чтение списков
раннеров и серверов через REST control-plane, действия (запуск сервера,
запуск раннера, подключение к mesh) — через сам CLI как sidecar-процесс.

## Структура

```
src/                     Svelte фронт
  lib/api.js             все invoke() в одном месте
  lib/RunnersTable.svelte
  lib/ServersTable.svelte
  lib/RunnerForm.svelte  форма роли runner (ресурсы)
  lib/ServerForm.svelte  форма роли admin (запуск сервера)
  lib/MeshPanel.svelte   роль user (join mesh)
  lib/TailscaleBadge.svelte  индикатор статуса, поллинг раз в 4с

src-tauri/               Rust core
  src/commands.rs        tauri commands, вызываемые фронтом
  src/cli_bridge.rs      запуск sidecar runner-cli, парсинг JSON из stdout
  src/rest_client.rs     чтение списков у control-plane
  src/types.rs           общие структуры
  capabilities/default.json  разрешение на запуск sidecar
  tauri.conf.json
```

## Что нужно доделать в самом CLI

Rust-ядро дергает `runner-cli` со следующими под-командами и ожидает
**одну JSON-строку в конце stdout** (обычные логи туда же можно писать построчно
раньше — они используются для лога в форме запуска раннера):

| команда | пример вызова | ожидаемый JSON |
|---|---|---|
| статус tailscale | `runner-cli tailscale status --json` | `{"connected": true, "tailscale_ip": "100.x.x.x"}` |
| join mesh | `runner-cli mesh join --json` | `{"connected": true, "tailscale_ip": "100.x.x.x"}` |
| запуск раннера | `runner-cli runner start --host-data-path .. --host-data-bind .. --cpu-cores N --memory-mb M --json` | `{"runner_id": "..."}` |
| запуск сервера | `runner-cli server start --name .. --image .. --cpu-cores N --memory-mb M --json` | `{"server_id": "...", "address": "100.x.x.x:27015"}` |
| остановка контейнера | `runner-cli container stop --id .. --json` | `{}` |

В коде из первого сообщения ближе всего к этому уже `pub async fn start`
в `runner.rs` — его стоит обернуть в подкоманду `runner start` с флагом
`--json`, который в конце печатает `println!("{}", serde_json::json!({"runner_id": sidecar_id}))`.

Также стоит добавить эндпоинты на control-plane (или на сам bridge/CP-сервисе):

```
GET /api/v1/runners  -> [{ id, name, tailscale_ip, cpu_cores, memory_mb, status }]
GET /api/v1/servers  -> [{ id, name, runner_name, address, status }]
```

## Локация CLI-бинаря (sidecar)

Tauri требует, чтобы sidecar-бинарь лежал в `src-tauri/binaries/` с суффиксом
таргет-триплета:

```
src-tauri/binaries/runner-cli-x86_64-unknown-linux-gnu       # Arch Linux
src-tauri/binaries/runner-cli-x86_64-pc-windows-msvc.exe     # Windows
```

Узнать свой триплет: `rustc -Vv | grep host`. Собранный в GitLab CI бинарь CLI
нужно скопировать сюда под этим именем перед `tauri build` (можно сделать
отдельным CI-джобом, который копирует артефакт из job'ы CLI).

## Установка и запуск

```bash
npm install
npm run tauri dev      # dev-режим, hot reload фронта
npm run tauri build    # продовая сборка (.AppImage/.deb под Arch, .exe/.msi под Windows)
```

Системные зависимости под Arch Linux для сборки Tauri:

```bash
sudo pacman -S --needed webkit2gtk-4.1 base-devel curl wget file openssl appmenu-gtk-module \
  gtk3 libappindicator-gtk3 librsvg
```

Под Windows нужен установленный WebView2 Runtime (обычно уже стоит в Win 10/11)
и MSVC build tools.

## Дальше

- Добавить настройки (URL control-plane, путь до headscale) в отдельный
  конфиг-экран, читаемый/сохраняемый через `tauri-plugin-store`.
- Заменить поллинг (setInterval) на push-обновления, если backend умеет
  WebSocket/SSE — меньше нагрузки и мгновенный статус.
- Обернуть `stop_container`/`start_*` подтверждающими диалогами в UI.
