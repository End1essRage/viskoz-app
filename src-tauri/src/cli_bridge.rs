use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

/// Запускает встроенный (external binary) `viskoz-cli` sidecar с заданными аргументами
/// и ожидает от него JSON-строку на stdout (CLI должен поддерживать флаг `--json`
/// для машиночитаемого вывода команд состояния/запуска).
///
/// Каждая команда CLI, дергаемая отсюда, должна:
/// 1) писать прогресс/логи в stderr как обычно (tracing::info! и т.п.),
/// 2) писать РОВНО ОДНУ финальную JSON-строку в stdout при успешном завершении.
pub async fn run_cli_json<T: DeserializeOwned>(app: &AppHandle, args: &[&str]) -> Result<T> {
    let shell = app.shell();
    let sidecar = shell
        .sidecar("viskoz-cli")
        .map_err(|e| anyhow!("не удалось найти sidecar viskoz-cli: {e}"))?;

    let output = sidecar
        .args(args)
        .output()
        .await
        .map_err(|e| anyhow!("ошибка запуска viskoz-cli: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "viskoz-cli завершился с ошибкой (код {:?}): {}",
            output.status.code(),
            stderr.trim()
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let last_line = stdout
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .ok_or_else(|| anyhow!("viskoz-cli не вернул вывод"))?;

    serde_json::from_str(last_line)
        .map_err(|e| anyhow!("не удалось разобрать JSON от viskoz-cli: {e}. Вывод: {last_line}"))
}

/// То же самое, но также возвращает построчный лог (stdout) — удобно для форм,
/// где хочется показать процесс запуска в UI.
pub async fn run_cli_json_with_log<T: DeserializeOwned>(
    app: &AppHandle,
    args: &[&str],
) -> Result<(T, Vec<String>)> {
    let shell = app.shell();
    let sidecar = shell
        .sidecar("viskoz-cli")
        .map_err(|e| anyhow!("не удалось найти sidecar viskoz-cli: {e}"))?;

    let output = sidecar
        .args(args)
        .output()
        .await
        .map_err(|e| anyhow!("ошибка запуска viskoz-cli: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!(
            "viskoz-cli завершился с ошибкой (код {:?}): {}",
            output.status.code(),
            stderr.trim()
        ));
    }

    let last_line = lines
        .iter()
        .rev()
        .find(|l| !l.trim().is_empty())
        .ok_or_else(|| anyhow!("viskoz-cli не вернул вывод"))?;

    let parsed = serde_json::from_str(last_line)
        .map_err(|e| anyhow!("не удалось разобрать JSON от viskoz-cli: {e}"))?;

    Ok((parsed, lines))
}
