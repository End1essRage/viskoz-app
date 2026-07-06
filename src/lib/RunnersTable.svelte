<script>
  import { onMount, onDestroy } from "svelte";
  import { listRunners, stopContainer } from "./api.js";

  let runners = [];
  let error = "";
  let interval;

  async function refresh() {
    try {
      runners = await listRunners();
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
    <h2 style="font-size:14px; font-weight:500; margin:0;">Раннеры</h2>
    <button class="ghost-btn" on:click={refresh}>обновить</button>
  </div>

  {#if error}
    <p style="color: var(--danger)" class="mono">{error}</p>
  {:else if runners.length === 0}
    <p style="color: var(--text-dim)">Раннеры не найдены.</p>
  {:else}
    <table style="width:100%; border-collapse: collapse;" class="mono">
      <thead>
        <tr style="text-align:left; color: var(--text-dim); font-size:11px;">
          <th style="padding:6px 8px; font-weight:400;">имя</th>
          <th style="padding:6px 8px; font-weight:400;">tailscale ip</th>
          <th style="padding:6px 8px; font-weight:400;">cpu</th>
          <th style="padding:6px 8px; font-weight:400;">mem</th>
          <th style="padding:6px 8px; font-weight:400;">статус</th>
          <th style="padding:6px 8px; font-weight:400;"></th>
        </tr>
      </thead>
      <tbody>
        {#each runners as r}
          <tr style="border-top: 1px solid var(--border);">
            <td style="padding:6px 8px;">{r.name}</td>
            <td style="padding:6px 8px; color: var(--text-dim);">{r.tailscale_ip || "—"}</td>
            <td style="padding:6px 8px;">{r.cpu_cores}</td>
            <td style="padding:6px 8px;">{r.memory_mb} mb</td>
            <td style="padding:6px 8px;">
              <span class="pill">
                <span class="pill-dot" style="background:{statusColor(r.status)}"></span>
                {r.status}
              </span>
            </td>
            <td style="padding:6px 8px; text-align:right;">
              <button class="ghost-btn" on:click={() => handleStop(r.id)}>стоп</button>
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
