<script>
  import { onMount, onDestroy } from "svelte";
  import { listServers, stopContainer } from "./api.js";

  let servers = [];
  let error = "";
  let interval;

  async function refresh() {
    try {
      servers = await listServers();
      error = "";
    } catch (e) {
      error = String(e);
    }
  }

  async function handleStop(id) {
    await stopContainer(id);
    refresh();
  }

  onMount(() => {
    refresh();
    interval = setInterval(refresh, 5000);
  });
  onDestroy(() => clearInterval(interval));

  const statusColor = (s) =>
    s === "running" ? "var(--success)" : s === "starting" ? "var(--warning)" : "var(--text-faint)";
</script>

<div class="panel" style="padding: 16px;">
  <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:12px;">
    <h2 style="font-size:14px; font-weight:500; margin:0;">Сервера</h2>
    <button class="ghost-btn" on:click={refresh}>обновить</button>
  </div>

  {#if error}
    <p style="color: var(--danger)" class="mono">{error}</p>
  {:else if servers.length === 0}
    <p style="color: var(--text-dim)">Сервера не найдены.</p>
  {:else}
    <table style="width:100%; border-collapse: collapse;" class="mono">
      <thead>
        <tr style="text-align:left; color: var(--text-dim); font-size:11px;">
          <th style="padding:6px 8px; font-weight:400;">имя</th>
          <th style="padding:6px 8px; font-weight:400;">раннер</th>
          <th style="padding:6px 8px; font-weight:400;">ip:порт</th>
          <th style="padding:6px 8px; font-weight:400;">статус</th>
          <th style="padding:6px 8px; font-weight:400;"></th>
        </tr>
      </thead>
      <tbody>
        {#each servers as s}
          <tr style="border-top: 1px solid var(--border);">
            <td style="padding:6px 8px;">{s.name}</td>
            <td style="padding:6px 8px; color: var(--text-dim);">{s.runner_name}</td>
            <td style="padding:6px 8px;">{s.address || "—"}</td>
            <td style="padding:6px 8px;">
              <span class="pill">
                <span class="pill-dot" style="background:{statusColor(s.status)}"></span>
                {s.status}
              </span>
            </td>
            <td style="padding:6px 8px; text-align:right;">
              <button class="ghost-btn" on:click={() => handleStop(s.id)}>стоп</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}
</div>

<style>
  .ghost-btn {
    background: transparent;
    color: var(--text-dim);
    border: 1px solid var(--border-strong);
    border-radius: var(--radius);
    padding: 4px 10px;
    font-size: 12px;
  }
  .ghost-btn:hover {
    color: var(--text);
    border-color: var(--accent);
  }
</style>
